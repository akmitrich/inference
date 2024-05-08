mod onnx;

pub fn get_api() {
    let base = unsafe { onnx::OrtGetApiBase() };
    unsafe {
        (*base).GetApi.unwrap()(17);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn api() {
        get_api();
    }
}