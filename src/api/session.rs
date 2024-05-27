use super::session_info::SessionInfo;
use crate::{api::session_info::SessionHandle, onnx};
use std::ptr::NonNull;

pub struct Silero<'a> {
    ort_api: &'a onnx::OrtApi,
    ort_options: NonNull<onnx::OrtSessionOptions>,
    ort_session: NonNull<onnx::OrtSession>,
    info: SessionInfo,
}

impl<'a> Silero<'a> {
    pub fn new(ort_api: &'a onnx::OrtApi, ort_env: &onnx::OrtEnv) -> Option<Self> {
        let mut options = std::ptr::null_mut();
        unsafe {
            ort_api.CreateSessionOptions?(&mut options as _);
            ort_api.SetSessionGraphOptimizationLevel?(
                options as _,
                onnx::GraphOptimizationLevel_ORT_DISABLE_ALL,
            );
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
        let ort_options = NonNull::new(options)?;
        let ort_session = NonNull::new(session)?;
        let session_handle = SessionHandle::new(ort_api, &ort_session);
        Some(Self {
            ort_api,
            ort_options,
            ort_session,
            info: session_handle.info(),
        })
    }

    pub fn process(&self, frame: &[i16]) {
        let data: Vec<f32> = frame
            .iter()
            .map(|x| (*x as f32) / (i16::MAX as f32))
            .collect();
        let input_names = self
            .info
            .input_nodes
            .iter()
            .map(|node| node.name.as_ptr())
            .collect::<Vec<_>>()
            .as_ptr();
        println!("{:?}\n{:?}", data, input_names);
    }
}

impl<'a> Drop for Silero<'a> {
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

impl<'a> std::fmt::Debug for Silero<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Info={:#?}", self.info)
    }
}
