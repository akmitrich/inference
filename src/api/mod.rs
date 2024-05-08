mod session;

pub use session::*;

use crate::{get_api, onnx};
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Api {
    ort_api: NonNull<onnx::OrtApi>,
    ort_env: NonNull<onnx::OrtEnv>,
}

impl Api {
    pub fn new() -> Option<Self> {
        let ort_api = NonNull::new(get_api(17))?;
        let mut env = std::ptr::null_mut();
        unsafe {
            ort_api.as_ref().CreateEnv?(
                onnx::OrtLoggingLevel_ORT_LOGGING_LEVEL_VERBOSE,
                b"inference\0".as_ptr() as _,
                &mut env as _,
            );
        }
        Some(Self {
            ort_api,
            ort_env: NonNull::new(env)?,
        })
    }

    pub fn silero_vad(&self) -> Option<Silero> {
        unsafe { Silero::new(self.ort_api.as_ref(), self.ort_env.as_ref()) }
    }
}

impl Drop for Api {
    fn drop(&mut self) {
        unsafe {
            if let Some(release_env) = self.ort_api.as_ref().ReleaseEnv {
                release_env(self.ort_env.as_ptr());
            }
        }
    }
}
