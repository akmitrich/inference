use super::session_info::SessionInfo;
use crate::{
    api::{element_type::ElementType, session_info::SessionHandle},
    onnx,
};
use std::ptr::NonNull;

const AUX_DIMS: &[i64] = &[2, 1, 64];

pub struct Silero<'a> {
    ort_api: &'a onnx::OrtApi,
    ort_options: NonNull<onnx::OrtSessionOptions>,
    ort_session: NonNull<onnx::OrtSession>,
    ort_memory: NonNull<onnx::OrtMemoryInfo>,
    info: SessionInfo,
    h: Vec<f32>,
    c: Vec<f32>,
}

impl<'a> Silero<'a> {
    pub fn new(ort_api: &'a onnx::OrtApi, ort_env: &onnx::OrtEnv) -> Option<Self> {
        let mut options = std::ptr::null_mut();
        unsafe {
            ort_api.CreateSessionOptions?(&mut options as _);
            ort_api.SetSessionGraphOptimizationLevel?(
                options,
                onnx::GraphOptimizationLevel_ORT_ENABLE_ALL,
            );
        }

        let mut session = std::ptr::null_mut();
        unsafe {
            ort_api.CreateSession?(
                ort_env as _,
                b"/home/akalashnikov/silero_vad.onnx\0".as_ptr() as _,
                options as _,
                &mut session,
            );
        }
        let mut ort_memory = std::ptr::null();
        let ort_options = NonNull::new(options)?;
        let ort_session = NonNull::new(session)?;
        let session_handle = SessionHandle::new(ort_api, &ort_session);
        unsafe {
            ort_api.AllocatorGetInfo?(session_handle.allocator.as_ptr(), &mut ort_memory as _);
        }
        let ort_memory = NonNull::new(ort_memory as _)?;
        let info = session_handle.info();
        let aux_size = AUX_DIMS.iter().map(|x| *x as usize).product();
        let h = vec![0.0; aux_size];
        let c = vec![0.0; aux_size];
        Some(Self {
            ort_api,
            ort_options,
            ort_session,
            ort_memory,
            info,
            h,
            c,
        })
    }

    pub fn process(&mut self, frame: &[i16]) {
        println!("Start processing");
        let mut data: Vec<f32> = frame
            .iter()
            .map(|x| (*x as f32) / (i16::MAX as f32))
            .collect();
        let input_names = self
            .info
            .input_nodes
            .iter()
            .map(|node| dbg!(node.name.as_ptr()))
            .collect::<Vec<_>>();
        let output_names = self
            .info
            .output_nodes
            .iter()
            .map(|node| node.name.as_ptr())
            .collect::<Vec<_>>();
        let data_size = data.len() as i64;
        let input_tensor =
            unsafe { self.give_me_tensor(data.as_mut_ptr(), &[1, data_size], ElementType::Float) };
        let sr_tensor =
            unsafe { self.give_me_tensor([8000_i64].as_mut_ptr(), &[1], ElementType::Int64) };
        let h_ptr = self.h.as_mut_ptr();
        let h_tensor = unsafe { self.give_me_tensor(h_ptr, AUX_DIMS, ElementType::Float) };
        let c_ptr = self.c.as_mut_ptr();
        let c_tensor = unsafe { self.give_me_tensor(c_ptr, AUX_DIMS, ElementType::Float) };
        let mut output_tensors = [std::ptr::null_mut(); 3];
        println!("names: {:?}", input_names);
        unsafe {
            println!("Let's run");
            let status = self.ort_api.Run.unwrap()(
                self.ort_session.as_ptr(),
                std::ptr::null(),
                input_names.as_ptr(),
                [
                    input_tensor as _,
                    sr_tensor as _,
                    h_tensor as _,
                    c_tensor as _,
                ]
                .as_ptr(),
                dbg!(self.info.input_nodes.len()),
                output_names.as_ptr(),
                dbg!(self.info.output_nodes.len()),
                output_tensors.as_mut_ptr(),
            );
            if status.is_null() {
                println!("OK. {:?}", output_tensors)
            } else {
                println!(
                    "FAIL. {:?}",
                    std::ffi::CStr::from_ptr(self.ort_api.GetErrorMessage.unwrap()(status))
                )
            }
        }

        assert_eq!(data, unsafe { self.tensor_slice(input_tensor) });
        println!(
            "SR={:?}, h has {}, c has {}",
            unsafe { self.tensor_slice::<i64>(sr_tensor) },
            unsafe { self.tensor_slice::<f32>(h_tensor) }.len(),
            unsafe { self.tensor_slice::<f32>(c_tensor) }.len()
        );
    }
}

impl<'a> Silero<'a> {
    unsafe fn give_me_tensor<T: Sized>(
        &self,
        data: *mut T,
        dims: &[i64],
        element_type: ElementType,
    ) -> *mut onnx::OrtValue {
        let mut tensor = std::ptr::null_mut();
        let data_size = dims.iter().map(|x| *x as usize).product::<usize>();
        self.ort_api.CreateTensorWithDataAsOrtValue.unwrap()(
            self.ort_memory.as_ptr(),
            data.cast::<_>(),
            data_size * std::mem::size_of::<T>(),
            dims.as_ptr(),
            dims.len(),
            element_type.into(),
            &mut tensor,
        );
        tensor
    }

    unsafe fn tensor_slice<T: Sized>(&self, tensor: *mut onnx::OrtValue) -> &[T] {
        let mut dims_count = 0;
        let mut tensor_info = std::ptr::null_mut();
        self.ort_api.GetTensorTypeAndShape.unwrap()(tensor, &mut tensor_info);
        self.ort_api.GetDimensionsCount.unwrap()(tensor_info, &mut dims_count);
        let mut dims = vec![0; dims_count];
        self.ort_api.GetDimensions.unwrap()(tensor_info, dims.as_mut_ptr(), dims_count);
        let mut tensor_data = std::ptr::null_mut();
        self.ort_api.GetTensorMutableData.unwrap()(tensor, &mut tensor_data);
        std::slice::from_raw_parts(
            tensor_data as *mut T,
            dims.into_iter().product::<i64>() as _,
        )
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
