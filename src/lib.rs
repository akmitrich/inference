mod onnx;

pub fn get_api() -> *const onnx::OrtApi {
    let base = unsafe { onnx::OrtGetApiBase() };
    let api = unsafe { (*base).GetApi.unwrap()(17) };
    api
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn api() {
        println!("{:?}", get_api());
    }
}
