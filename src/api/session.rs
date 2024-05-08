use crate::onnx;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Session<'a> {
    ort_api: &'a onnx::OrtApi,
    ort_options: NonNull<onnx::OrtSessionOptions>,
    ort_session: NonNull<onnx::OrtSession>,
}

impl<'a> Session<'a> {
    pub fn new(ort_api: &'a onnx::OrtApi, ort_env: &onnx::OrtEnv) -> Option<Self> {
        let mut options = std::ptr::null_mut();
        unsafe {
            ort_api.CreateSessionOptions?(&mut options as _);
        }
        let mut session = std::ptr::null_mut();
        unsafe {
            ort_api.CreateSession?(
                ort_env as _,
                b"/home/akalashnikov/silero_vad.onnx\0".as_ptr() as _,
                options as _,
                &mut session as _,
            );
        }
        Some(Self {
            ort_api,
            ort_options: NonNull::new(options)?,
            ort_session: NonNull::new(session)?,
        })
    }
}

impl<'a> Drop for Session<'a> {
    fn drop(&mut self) {
        unsafe {
            if let Some(release_session_options) = self.ort_api.ReleaseSessionOptions {
                release_session_options(self.ort_options.as_ptr());
            }
            if let Some(release_session) = self.ort_api.ReleaseSession {
                release_session(self.ort_session.as_ptr());
            }
        }
    }
}
