# three-d-gltf-import
This crate implements a GLTF loader for the [three-d](https://crates.io/crates/three-d) crate. It supports both desktop and wasm target.

It supports loading `.gltf` as well as `.glb` (binary) files, and can handle embedded (e.g. via base64) or external references to buffers and images (relative paths to the GLTF-File, `file://` on desktop, `http[s]://` for wasm).

Internally ist ueses the [`gltf`](https://crates.io/crates/gltf) crate to parse the GLTF files.

## THIS CRATE IS STILL UNDER DEVELOPMENT

## Usage
TODO

## Testing
For testing purposes, some sample models from [https://github.com/KhronosGroup/glTF-Sample-Models]() are used (stored in `/sample_models`.

The repository with sample models is added via a git submodule and can be initialized via:
```bash
git submodule update --init --recursive
```

### WASM/Browser
In order to run WASM tests, the steps outlined [here](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html#appendix-using-wasm-bindgen-test-without-wasm-pack) need to be performed.
After that, the tests can be run with this command:
```bash 
WASM_BINDGEN_TEST_TIMEOUT=600 cargo test --target wasm32-unknown-unknown
```

Note: Test-Timout needs to be set pretty high since some test that load bigger/complex models can take a long time.