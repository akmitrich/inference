use super::element_type::ElementType;
use crate::onnx;
use std::{ffi::CString, ptr::NonNull};

#[derive(Debug)]
pub struct SessionInfo {
    pub input_nodes: Vec<ModelNode>,
}

#[derive(Debug)]
pub struct ModelNode {
    pub name: String,
    pub element_type: ElementType,
    pub dims: Vec<i64>,
}

pub struct SessionHandle<'a> {
    ort_api: &'a onnx::OrtApi,
    session: &'a NonNull<onnx::OrtSession>,
    allocator: NonNull<onnx::OrtAllocator>,
}

impl<'a> SessionHandle<'a> {
    pub fn new(ort_api: &'a onnx::OrtApi, session: &'a NonNull<onnx::OrtSession>) -> Self {
        let mut allocator: *mut onnx::OrtAllocator = std::ptr::null_mut();
        unsafe {
            ort_api.GetAllocatorWithDefaultOptions.unwrap()(&mut allocator as _);
        }
        Self {
            ort_api,
            session,
            allocator: NonNull::new(allocator).unwrap(),
        }
    }

    pub fn info(&self) -> SessionInfo {
        let mut input_nodes_count = 0;
        unsafe {
            self.ort_api.SessionGetInputCount.unwrap()(
                self.session.as_ptr(),
                &mut input_nodes_count,
            );
        }
        let input_nodes = (0..input_nodes_count)
            .map(|node| {
                let mut name_raw = std::ptr::null_mut();
                unsafe {
                    self.ort_api.SessionGetInputName.unwrap()(
                        self.session.as_ptr(),
                        node,
                        self.allocator.as_ptr(),
                        &mut name_raw,
                    );
                }
                let name = raw_to_string(name_raw);
                let mut typeinfo_ptr: *mut onnx::OrtTypeInfo = std::ptr::null_mut();
                unsafe {
                    self.ort_api.SessionGetInputTypeInfo.unwrap()(
                        self.session.as_ptr(),
                        node,
                        &mut typeinfo_ptr,
                    );
                }
                let mut tensor_info_ptr: *const onnx::OrtTensorTypeAndShapeInfo =
                    std::ptr::null_mut();
                unsafe {
                    self.ort_api.CastTypeInfoToTensorInfo.unwrap()(
                        typeinfo_ptr,
                        &mut tensor_info_ptr,
                    );
                }
                let mut element_type: onnx::ONNXTensorElementDataType =
                    onnx::ONNXTensorElementDataType_ONNX_TENSOR_ELEMENT_DATA_TYPE_UNDEFINED;
                unsafe {
                    self.ort_api.GetTensorElementType.unwrap()(tensor_info_ptr, &mut element_type);
                }
                let mut dims_count = 0;
                unsafe {
                    self.ort_api.GetDimensionsCount.unwrap()(tensor_info_ptr, &mut dims_count);
                }
                let mut dims = vec![Default::default(); dims_count];
                unsafe {
                    self.ort_api.GetDimensions.unwrap()(
                        tensor_info_ptr,
                        dims.as_mut_ptr(),
                        dims_count,
                    );
                }
                unsafe {
                    self.ort_api.ReleaseTypeInfo.unwrap()(typeinfo_ptr);
                }
                ModelNode {
                    name,
                    element_type: element_type.into(),
                    dims,
                }
            })
            .collect();

        SessionInfo { input_nodes }
    }
}

fn raw_to_string(raw: *mut i8) -> String {
    unsafe { CString::from_raw(raw).to_str().unwrap().to_owned() }
}
