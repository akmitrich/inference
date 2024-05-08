pub mod api;
mod onnx;

pub fn get_api(version: u32) -> *mut onnx::OrtApi {
    let base = unsafe { onnx::OrtGetApiBase() };
    if !base.is_null() {
        unsafe { (*base).GetApi.unwrap()(version) as _ }
    } else {
        std::ptr::null_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn api() {
        assert!(!get_api(17).is_null());
    }
}
