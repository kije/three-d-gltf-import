//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use gltf::Gltf;
use js_sys::{Function, Object};
use std::path::PathBuf;
use three_d::Loader;
use three_d_gltf_import::GltfImporter;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_import_triangle_model() {
    let promise = js_sys::Promise::new(&mut |resolve: Function, reject: Function| {
        let base = PathBuf::from(format!("{}/{}", "..", "sample_models/2.0/Triangle/glTF"));

        Loader::load(&[base.join("Triangle.gltf")], move |loaded| {
            let b = loaded.bytes(base.join("Triangle.gltf")).unwrap();

            let gltf = Gltf::from_slice(b).unwrap();
            GltfImporter::import(gltf, Some(base), move |imported| {
                let result = imported.unwrap();
                if result.buffers.len() != 1 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.buffers.len(),
                            1
                        )),
                        &Object::new(),
                    );
                } else if result.images.len() != 0 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.images.len(),
                            0
                        )),
                        &Object::new(),
                    );
                } else {
                    resolve.call0(&Object::new());
                }
            })
        })
    });

    JsFuture::from(promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_import_triangle_model_with_embedded_data() {
    let promise = js_sys::Promise::new(&mut |resolve: Function, reject: Function| {
        let base = PathBuf::from(format!(
            "{}/{}",
            "..", "sample_models/2.0/Triangle/glTF-Embedded"
        ));

        Loader::load(&[base.join("Triangle.gltf")], move |loaded| {
            let b = loaded.bytes(base.join("Triangle.gltf")).unwrap();

            let gltf = Gltf::from_slice(b).unwrap();
            GltfImporter::import(gltf, Some(base), move |imported| {
                let result = imported.unwrap();
                if result.buffers.len() != 1 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.buffers.len(),
                            1
                        )),
                        &Object::new(),
                    );
                } else if result.images.len() != 0 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.images.len(),
                            0
                        )),
                        &Object::new(),
                    );
                } else {
                    resolve.call0(&Object::new());
                }
            })
        })
    });

    JsFuture::from(promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_import_cube_model() {
    let promise = js_sys::Promise::new(&mut |resolve: Function, reject: Function| {
        let base = PathBuf::from(format!("{}/{}", "..", "sample_models/2.0/Cube/glTF"));

        Loader::load(&[base.join("Cube.gltf")], move |loaded| {
            let b = loaded.bytes(base.join("Cube.gltf")).unwrap();

            let gltf = Gltf::from_slice(b).unwrap();
            GltfImporter::import(gltf, Some(base), move |imported| {
                let result = imported.unwrap();
                if result.buffers.len() != 1 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.buffers.len(),
                            1
                        )),
                        &Object::new(),
                    );
                } else if result.images.len() != 2 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.images.len(),
                            2
                        )),
                        &Object::new(),
                    );
                } else {
                    resolve.call0(&Object::new());
                }
            })
        })
    });

    JsFuture::from(promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_import_simple_meshes_model() {
    let promise = js_sys::Promise::new(&mut |resolve: Function, reject: Function| {
        let base = PathBuf::from(format!(
            "{}/{}",
            "..", "sample_models/2.0/SimpleMeshes/glTF"
        ));

        Loader::load(&[base.join("SimpleMeshes.gltf")], move |loaded| {
            let b = loaded.bytes(base.join("SimpleMeshes.gltf")).unwrap();

            let gltf = Gltf::from_slice(b).unwrap();
            GltfImporter::import(gltf, Some(base), move |imported| {
                let result = imported.unwrap();
                if result.buffers.len() != 1 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.buffers.len(),
                            1
                        )),
                        &Object::new(),
                    );
                } else if result.images.len() != 0 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.images.len(),
                            0
                        )),
                        &Object::new(),
                    );
                } else {
                    resolve.call0(&Object::new());
                }
            })
        })
    });

    JsFuture::from(promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_import_simple_meshes_model_with_embedded_data() {
    let promise = js_sys::Promise::new(&mut |resolve: Function, reject: Function| {
        let base = PathBuf::from(format!(
            "{}/{}",
            "..", "sample_models/2.0/SimpleMeshes/glTF-Embedded"
        ));

        Loader::load(&[base.join("SimpleMeshes.gltf")], move |loaded| {
            let b = loaded.bytes(base.join("SimpleMeshes.gltf")).unwrap();

            let gltf = Gltf::from_slice(b).unwrap();
            GltfImporter::import(gltf, Some(base), move |imported| {
                let result = imported.unwrap();
                if result.buffers.len() != 1 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.buffers.len(),
                            1
                        )),
                        &Object::new(),
                    );
                } else if result.images.len() != 0 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.images.len(),
                            0
                        )),
                        &Object::new(),
                    );
                } else {
                    resolve.call0(&Object::new());
                }
            })
        })
    });

    JsFuture::from(promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_import_fox_model() {
    let promise = js_sys::Promise::new(&mut |resolve: Function, reject: Function| {
        let base = PathBuf::from(format!("{}/{}", "..", "sample_models/2.0/Fox/glTF"));

        Loader::load(&[base.join("Fox.gltf")], move |loaded| {
            let b = loaded.bytes(base.join("Fox.gltf")).unwrap();

            let gltf = Gltf::from_slice(b).unwrap();
            GltfImporter::import(gltf, Some(base), move |imported| {
                let result = imported.unwrap();
                if result.buffers.len() != 1 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.buffers.len(),
                            1
                        )),
                        &Object::new(),
                    );
                } else if result.images.len() != 1 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.images.len(),
                            1
                        )),
                        &Object::new(),
                    );
                } else {
                    resolve.call0(&Object::new());
                }
            })
        })
    });

    JsFuture::from(promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_import_fox_model_with_embedded_data() {
    let promise = js_sys::Promise::new(&mut |resolve: Function, reject: Function| {
        let base = PathBuf::from(format!(
            "{}/{}",
            "..", "sample_models/2.0/Fox/glTF-Embedded"
        ));

        Loader::load(&[base.join("Fox.gltf")], move |loaded| {
            let b = loaded.bytes(base.join("Fox.gltf")).unwrap();

            let gltf = Gltf::from_slice(b).unwrap();
            GltfImporter::import(gltf, Some(base), move |imported| {
                let result = imported.unwrap();
                if result.buffers.len() != 1 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.buffers.len(),
                            1
                        )),
                        &Object::new(),
                    );
                } else if result.images.len() != 1 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.images.len(),
                            1
                        )),
                        &Object::new(),
                    );
                } else {
                    resolve.call0(&Object::new());
                }
            })
        })
    });

    JsFuture::from(promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_import_fox_model_binary() {
    let promise = js_sys::Promise::new(&mut |resolve: Function, reject: Function| {
        let base = PathBuf::from(format!("{}/{}", "..", "sample_models/2.0/Fox/glTF-Binary"));

        Loader::load(&[base.join("Fox.glb")], move |loaded| {
            let b = loaded.bytes(base.join("Fox.glb")).unwrap();

            let gltf = Gltf::from_slice(b).unwrap();
            GltfImporter::import(gltf, Some(base), move |imported| {
                let result = imported.unwrap();
                if result.buffers.len() != 1 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.buffers.len(),
                            1
                        )),
                        &Object::new(),
                    );
                } else if result.images.len() != 1 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.images.len(),
                            1
                        )),
                        &Object::new(),
                    );
                } else {
                    resolve.call0(&Object::new());
                }
            })
        })
    });

    JsFuture::from(promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_import_toy_car_model() {
    let promise = js_sys::Promise::new(&mut |resolve: Function, reject: Function| {
        let base = PathBuf::from(format!("{}/{}", "..", "sample_models/2.0/ToyCar/glTF"));

        Loader::load(&[base.join("ToyCar.gltf")], move |loaded| {
            let b = loaded.bytes(base.join("ToyCar.gltf")).unwrap();

            let gltf = Gltf::from_slice(b).unwrap();
            GltfImporter::import(gltf, Some(base), move |imported| {
                let result = imported.unwrap();
                if result.buffers.len() != 1 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.buffers.len(),
                            1
                        )),
                        &Object::new(),
                    );
                } else if result.images.len() != 8 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.images.len(),
                            8
                        )),
                        &Object::new(),
                    );
                } else {
                    resolve.call0(&Object::new());
                }
            })
        })
    });

    JsFuture::from(promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_import_toy_car_model_binary() {
    let promise = js_sys::Promise::new(&mut |resolve: Function, reject: Function| {
        let base = PathBuf::from(format!(
            "{}/{}",
            "..", "sample_models/2.0/ToyCar/glTF-Binary"
        ));

        Loader::load(&[base.join("ToyCar.glb")], move |loaded| {
            let b = loaded.bytes(base.join("ToyCar.glb")).unwrap();

            let gltf = Gltf::from_slice(b).unwrap();
            GltfImporter::import(gltf, Some(base), move |imported| {
                let result = imported.unwrap();
                if result.buffers.len() != 1 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.buffers.len(),
                            1
                        )),
                        &Object::new(),
                    );
                } else if result.images.len() != 8 {
                    reject.call1(
                        &JsValue::from_str(&format!(
                            "{:?} not equals {:?}",
                            result.images.len(),
                            8
                        )),
                        &Object::new(),
                    );
                } else {
                    resolve.call0(&Object::new());
                }
            })
        })
    });

    JsFuture::from(promise).await.unwrap();
}
