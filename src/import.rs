use base64;
use gltf::buffer;
use gltf::image as gltf_image;
use gltf::{Document, Error, Gltf, Result};
use image::ImageFormat::{Jpeg, Png};
use image::{DynamicImage, ImageFormat};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use three_d::Loader;

#[cfg(not(target_arch = "wasm32"))]
use three_d::IOError;

pub struct GltfImporter {}

#[derive(Clone, Debug)]
pub struct ImportedGltfModel {
    pub images: HashMap<usize, gltf_image::Data>,
    pub buffers: HashMap<usize, buffer::Data>,
    pub document: Document,
}

enum ImageImport {
    Loaded {
        index: usize,
        data: gltf_image::Data,
    },
    NeedsLoading {
        index: usize,
        path: PathBuf,
        mime_type: Option<String>,
    },
}

enum BufferImport {
    Loaded {
        index: usize,
        data: Vec<u8>,
        length: usize,
    },
    NeedsLoading {
        index: usize,
        path: PathBuf,
        length: usize,
    },
}

impl GltfImporter {
    pub fn import<F>(Gltf { document, blob }: Gltf, base: Option<PathBuf>, on_done: F)
    where
        F: 'static + FnOnce(Result<ImportedGltfModel>),
    {
        Self::load_buffer_data(
            document,
            base.clone().as_deref(),
            blob,
            move |buffer_data, document| {
                let buffers = match buffer_data {
                    Ok(data) => data,
                    Err(e) => return on_done(Err(e)),
                };

                Self::load_image_data(
                    document,
                    base.clone().as_deref(),
                    buffers,
                    move |image_data, buffers, document| {
                        let images = match image_data {
                            Ok(data) => data,
                            Err(e) => return on_done(Err(e)),
                        };

                        on_done(Ok(ImportedGltfModel {
                            images,
                            buffers,
                            document,
                        }))
                    },
                );
            },
        );
    }

    fn load_buffer_data<F>(
        document: Document,
        base: Option<&Path>,
        mut blob: Option<Vec<u8>>,
        on_done: F,
    ) where
        F: 'static + FnOnce(Result<HashMap<usize, buffer::Data>>, Document),
    {
        let document_buffers = document.buffers();
        let mut imported_buffers = Vec::with_capacity(document_buffers.len());
        for buffer in document_buffers {
            let imported_buffer = match buffer.source() {
                buffer::Source::Uri(uri) => match Scheme::parse(uri) {
                    Scheme::Data(_, base64) => BufferImport::Loaded {
                        index: buffer.index(),
                        data: match Self::load_buffer_from_data_uri(base64) {
                            Ok(data) => data,
                            Err(e) => return on_done(Err(e), document),
                        },
                        length: buffer.length(),
                    },
                    #[cfg(not(target_arch = "wasm32"))]
                    Scheme::File(path) => BufferImport::NeedsLoading {
                        index: buffer.index(),
                        path: PathBuf::from(path),
                        length: buffer.length(),
                    },
                    Scheme::Relative if base.is_some() => {
                        let url = base.unwrap().join(uri);
                        BufferImport::NeedsLoading {
                            index: buffer.index(),
                            path: url,
                            length: buffer.length(),
                        }
                    }
                    Scheme::External(url) => BufferImport::NeedsLoading {
                        index: buffer.index(),
                        path: PathBuf::from(url),
                        length: buffer.length(),
                    },
                    Scheme::Unsupported => return on_done(Err(Error::UnsupportedScheme), document),
                    _ => return on_done(Err(Error::UnsupportedScheme), document),
                },
                buffer::Source::Bin => BufferImport::Loaded {
                    index: buffer.index(),
                    data: match blob.take() {
                        Some(data) => data,
                        None => return on_done(Err(Error::MissingBlob), document),
                    },
                    length: buffer.length(),
                },
            };

            imported_buffers.push(imported_buffer);
        }

        let paths: Vec<_> = imported_buffers
            .iter()
            .filter_map(|buffer| {
                if let BufferImport::NeedsLoading { path, .. } = buffer {
                    Some(path.clone())
                } else {
                    None
                }
            })
            .collect();

        Loader::load(paths.as_slice(), move |loaded| {
            let result: Result<HashMap<_, _>> = imported_buffers
                .into_iter()
                .map(|buffer| match buffer {
                    BufferImport::NeedsLoading {
                        index,
                        path,
                        length,
                    } => match loaded.bytes(path) {
                        Ok(bytes) => Ok((index, bytes.to_owned(), length)),
                        #[cfg(not(target_arch = "wasm32"))]
                        Err(IOError::IO(err)) => Err(Error::Io(err)),
                        _ => Err(Error::MissingBlob),
                    },
                    BufferImport::Loaded {
                        index,
                        data,
                        length,
                    } => Ok((index, data, length)),
                })
                .map(|data| {
                    let (index, mut buffer_data, length) = data?;
                    if buffer_data.len() < length {
                        return Err(Error::BufferLength {
                            buffer: index,
                            expected: length,
                            actual: buffer_data.len(),
                        });
                    }
                    while buffer_data.len() % 4 != 0 {
                        buffer_data.push(0);
                    }

                    Ok((index, buffer::Data(buffer_data)))
                })
                .collect();

            on_done(result, document);
        });
    }

    fn load_buffer_from_data_uri(base64: &str) -> Result<Vec<u8>> {
        base64::decode(&base64).map_err(Error::Base64)
    }

    fn load_image_data<F>(
        document: Document,
        base: Option<&Path>,
        buffer_data: HashMap<usize, buffer::Data>,
        on_done: F,
    ) where
        F: 'static
            + FnOnce(Result<HashMap<usize, gltf_image::Data>>, HashMap<usize, buffer::Data>, Document),
    {
        let document_images = document.images();
        let mut imported_images = Vec::with_capacity(document_images.len());
        for image in document_images {
            let imported_image = match image.source() {
                gltf_image::Source::Uri { uri, mime_type } if base.is_some() => {
                    match Scheme::parse(uri) {
                        Scheme::Data(media_type, base64) => ImageImport::Loaded {
                            index: image.index(),
                            // todo type missmatch (result)
                            data: match Self::load_image_from_data_uri(
                                media_type.or(mime_type),
                                base64,
                            ) {
                                Ok(data) => data,
                                Err(e) => return on_done(Err(e), buffer_data, document),
                            },
                        },
                        #[cfg(not(target_arch = "wasm32"))]
                        Scheme::File(path) => ImageImport::NeedsLoading {
                            index: image.index(),
                            path: PathBuf::from(path),
                            mime_type: mime_type.map(|mime| mime.to_owned()),
                        },
                        Scheme::Relative if base.is_some() => {
                            let url = base.unwrap().join(uri);
                            ImageImport::NeedsLoading {
                                index: image.index(),
                                path: url,
                                mime_type: mime_type.map(|mime| mime.to_owned()),
                            }
                        }
                        Scheme::External(url) => ImageImport::NeedsLoading {
                            index: image.index(),
                            path: PathBuf::from(url),
                            mime_type: mime_type.map(|mime| mime.to_owned()),
                        },
                        Scheme::Unsupported => {
                            return on_done(Err(Error::UnsupportedScheme), buffer_data, document)
                        }
                        _ => return on_done(Err(Error::UnsupportedScheme), buffer_data, document),
                    }
                }
                gltf_image::Source::View { view, mime_type } => {
                    let parent_buffer_data = match buffer_data.get(&view.buffer().index()) {
                        Some(data) => data,
                        None => return on_done(Err(Error::MissingBlob), buffer_data, document),
                    };
                    let begin = view.offset();
                    let end = begin + view.length();
                    let encoded_image = &parent_buffer_data[begin..end];

                    let image_data = Self::load_image_from_buffer(encoded_image, Some(mime_type));

                    match image_data {
                        Ok(data) => ImageImport::Loaded {
                            index: image.index(),
                            data,
                        },
                        Err(err) => return on_done(Err(err), buffer_data, document),
                    }
                }
                _ => {
                    return on_done(
                        Err(Error::ExternalReferenceInSliceImport),
                        buffer_data,
                        document,
                    )
                }
            };

            imported_images.push(imported_image);
        }

        let paths: Vec<_> = imported_images
            .iter()
            .filter_map(|buffer| {
                if let ImageImport::NeedsLoading { path, .. } = buffer {
                    Some(path.clone())
                } else {
                    None
                }
            })
            .collect();

        Loader::load(paths.as_slice(), move |loaded| {
            let result: Result<HashMap<_, _>> = imported_images
                .into_iter()
                .map(|image| match image {
                    ImageImport::NeedsLoading {
                        index,
                        path,
                        mime_type,
                    } => match loaded.bytes(path) {
                        Ok(bytes) => {
                            let image_data =
                                Self::load_image_from_buffer(bytes, mime_type.as_deref())?;

                            Ok((index, image_data))
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        Err(IOError::IO(err)) => Err(Error::Io(err)),
                        _ => Err(Error::MissingBlob),
                    },
                    ImageImport::Loaded { index, data } => Ok((index, data)),
                })
                .collect();

            on_done(result, buffer_data, document);
        });
    }

    fn guess_format(encoded_image: &[u8]) -> Option<ImageFormat> {
        match image::guess_format(encoded_image) {
            Ok(Png) => Some(Png),
            Ok(Jpeg) => Some(Jpeg),
            _ => None,
        }
    }

    fn mime_type_to_image_format(
        encoded_image: &[u8],
        mime_type: Option<&str>,
    ) -> Result<ImageFormat> {
        match mime_type {
            Some(t) => match t.as_ref() {
                "image/png" => Ok(Png),
                "image/jpeg" => Ok(Jpeg),
                _ => match Self::guess_format(&encoded_image) {
                    Some(format) => Ok(format),
                    None => Err(Error::UnsupportedImageEncoding),
                },
            },
            None => match Self::guess_format(&encoded_image) {
                Some(format) => Ok(format),
                None => Err(Error::UnsupportedImageEncoding),
            },
        }
    }

    fn load_image_from_data_uri(mime_type: Option<&str>, base64: &str) -> Result<gltf_image::Data> {
        let encoded_image = base64::decode(&base64).map_err(Error::Base64)?;
        let encoded_format = Self::mime_type_to_image_format(&encoded_image, mime_type)?;
        let decoded_image = image::load_from_memory_with_format(&encoded_image, encoded_format)?;
        Ok(data_from_dynamic_image(decoded_image))
    }

    fn load_image_from_buffer(buffer: &[u8], mime_type: Option<&str>) -> Result<gltf_image::Data> {
        let encoded_format = Self::mime_type_to_image_format(buffer, mime_type)?;
        let decoded_image = image::load_from_memory_with_format(buffer, encoded_format)?;

        Ok(data_from_dynamic_image(decoded_image))
    }
}

fn data_from_dynamic_image(image: DynamicImage) -> gltf_image::Data {
    use gltf::image::Format;
    use image::GenericImageView;

    let format = match image {
        DynamicImage::ImageLuma8(_) => Format::R8,
        DynamicImage::ImageLumaA8(_) => Format::R8G8,
        DynamicImage::ImageRgb8(_) => Format::R8G8B8,
        DynamicImage::ImageRgba8(_) => Format::R8G8B8A8,
        DynamicImage::ImageBgr8(_) => Format::B8G8R8,
        DynamicImage::ImageBgra8(_) => Format::B8G8R8A8,
        DynamicImage::ImageLuma16(_) => Format::R16,
        DynamicImage::ImageLumaA16(_) => Format::R16G16,
        DynamicImage::ImageRgb16(_) => Format::R16G16B16,
        DynamicImage::ImageRgba16(_) => Format::R16G16B16A16,
    };
    let (width, height) = image.dimensions();
    let pixels = image.to_bytes();
    gltf_image::Data {
        format,
        width,
        height,
        pixels,
    }
}

/// Represents the set of URI schemes the importer supports.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Scheme<'a> {
    /// `data:[<media type>];base64,<data>`.
    Data(Option<&'a str>, &'a str),

    /// `file:[//]<absolute file path>`.
    ///
    /// Note: The file scheme does not implement authority.
    #[cfg(not(target_arch = "wasm32"))]
    File(&'a str),

    /// `../foo`, etc.
    Relative,

    // http[s]://<host>/<path>
    External(&'a str),

    /// Placeholder for an unsupported URI scheme identifier.
    Unsupported,
}

impl<'a> Scheme<'a> {
    fn parse(uri: &str) -> Scheme {
        if uri.contains(":") {
            if uri.starts_with("data:") {
                let match0 = &uri["data:".len()..].split(";base64,").nth(0);
                let match1 = &uri["data:".len()..].split(";base64,").nth(1);
                if match1.is_some() {
                    Scheme::Data(Some(match0.unwrap()), match1.unwrap())
                } else if match0.is_some() {
                    Scheme::Data(None, match0.unwrap())
                } else {
                    Scheme::Unsupported
                }
            } else if uri.starts_with("file://") {
                #[cfg(not(target_arch = "wasm32"))]
                return Scheme::File(&uri["file://".len()..]);
                #[cfg(target_arch = "wasm32")]
                return Scheme::Unsupported;
            } else if uri.starts_with("file:") {
                #[cfg(not(target_arch = "wasm32"))]
                return Scheme::File(&uri["file:".len()..]);
                #[cfg(target_arch = "wasm32")]
                return Scheme::Unsupported;
            } else if uri.starts_with("http:") || uri.starts_with("https:") {
                Scheme::External(&uri)
            } else {
                Scheme::Unsupported
            }
        } else {
            Scheme::Relative
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_triangle_model() {
        let base = PathBuf::from(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "sample_models/2.0/Triangle/glTF"
        ));
        let gltf = Gltf::open(base.join("Triangle.gltf")).unwrap();
        GltfImporter::import(gltf, Some(base), |imported| {
            let result = imported.unwrap();
            assert_eq!(result.buffers.len(), 1);
            assert_eq!(result.images.len(), 0);
        })
    }

    #[test]
    fn test_import_triangle_model_with_embedded_data() {
        let base = PathBuf::from(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "sample_models/2.0/Triangle/glTF-Embedded"
        ));
        let gltf = Gltf::open(base.join("Triangle.gltf")).unwrap();
        GltfImporter::import(gltf, Some(base), |imported| {
            let result = imported.unwrap();
            assert_eq!(result.buffers.len(), 1);
            assert_eq!(result.images.len(), 0);
        })
    }

    #[test]
    fn test_import_cube_model() {
        let base = PathBuf::from(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "sample_models/2.0/Cube/glTF"
        ));
        let gltf = Gltf::open(base.join("Cube.gltf")).unwrap();
        GltfImporter::import(gltf, Some(base), |imported| {
            let result = imported.unwrap();
            assert_eq!(result.buffers.len(), 1);
            assert_eq!(result.images.len(), 2);
        })
    }

    #[test]
    fn test_import_simple_meshes_model() {
        let base = PathBuf::from(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "sample_models/2.0/SimpleMeshes/glTF"
        ));
        let gltf = Gltf::open(base.join("SimpleMeshes.gltf")).unwrap();
        GltfImporter::import(gltf, Some(base), |imported| {
            let result = imported.unwrap();
            assert_eq!(result.buffers.len(), 1);
            assert_eq!(result.images.len(), 0);
        })
    }

    #[test]
    fn test_import_simple_meshes_model_with_embedded_data() {
        let base = PathBuf::from(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "sample_models/2.0/SimpleMeshes/glTF-Embedded"
        ));
        let gltf = Gltf::open(base.join("SimpleMeshes.gltf")).unwrap();
        GltfImporter::import(gltf, Some(base), |imported| {
            let result = imported.unwrap();
            assert_eq!(result.buffers.len(), 1);
            assert_eq!(result.images.len(), 0);
        })
    }

    #[test]
    fn test_import_fox_model() {
        let base = PathBuf::from(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "sample_models/2.0/Fox/glTF"
        ));
        let gltf = Gltf::open(base.join("Fox.gltf")).unwrap();
        GltfImporter::import(gltf, Some(base), |imported| {
            let result = imported.unwrap();
            assert_eq!(result.buffers.len(), 1);
            assert_eq!(result.images.len(), 1);
        })
    }

    #[test]
    fn test_import_fox_model_with_embedded_data() {
        let base = PathBuf::from(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "sample_models/2.0/Fox/glTF-Embedded"
        ));
        let gltf = Gltf::open(base.join("Fox.gltf")).unwrap();
        GltfImporter::import(gltf, Some(base), |imported| {
            let result = imported.unwrap();
            assert_eq!(result.buffers.len(), 1);
            assert_eq!(result.images.len(), 1);
        })
    }

    #[test]
    fn test_import_fox_model_binary() {
        let base = PathBuf::from(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "sample_models/2.0/Fox/glTF-Binary"
        ));
        let gltf = Gltf::open(base.join("Fox.glb")).unwrap();
        GltfImporter::import(gltf, Some(base), |imported| {
            let result = imported.unwrap();
            assert_eq!(result.buffers.len(), 1);
            assert_eq!(result.images.len(), 1);
        })
    }

    #[test]
    fn test_import_toy_car_model() {
        let base = PathBuf::from(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "sample_models/2.0/ToyCar/glTF"
        ));
        let gltf = Gltf::open(base.join("ToyCar.gltf")).unwrap();
        GltfImporter::import(gltf, Some(base), |imported| {
            let result = imported.unwrap();
            assert_eq!(result.buffers.len(), 1);
            assert_eq!(result.images.len(), 8);
        })
    }

    #[test]
    fn test_import_toy_car_model_binary() {
        let base = PathBuf::from(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "sample_models/2.0/ToyCar/glTF-Binary"
        ));
        let gltf = Gltf::open(base.join("ToyCar.glb")).unwrap();
        GltfImporter::import(gltf, Some(base), |imported| {
            let result = imported.unwrap();
            assert_eq!(result.buffers.len(), 1);
            assert_eq!(result.images.len(), 8);
        })
    }
}
