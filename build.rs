fn main() {
    let builder = bindgen::builder();
    let bindings = builder
        .header("/home/akalashnikov/onnxruntime/include/onnxruntime_c_api.h")
        .generate()
        .unwrap();
    // let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings.write_to_file("bindings.rs").unwrap();
}
