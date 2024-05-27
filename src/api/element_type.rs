// const TYPE_UNDEFINED: u32 = 0;
const TYPE_FLOAT: u32 = 1;
const TYPE_UINT8: u32 = 2;
const TYPE_INT8: u32 = 3;
const TYPE_UINT16: u32 = 4;
const TYPE_INT16: u32 = 5;
const TYPE_INT32: u32 = 6;
const TYPE_INT64: u32 = 7;
const TYPE_STRING: u32 = 8;
const TYPE_BOOL: u32 = 9;
const TYPE_FLOAT16: u32 = 10;
const TYPE_DOUBLE: u32 = 11;
const TYPE_UINT32: u32 = 12;
const TYPE_UINT64: u32 = 13;
const TYPE_COMPLEX64: u32 = 14;
const TYPE_COMPLEX128: u32 = 15;
const TYPE_BFLOAT16: u32 = 16;
const TYPE_FLOAT8E4M3FN: u32 = 17;
const TYPE_FLOAT8E4M3FNUZ: u32 = 18;
const TYPE_FLOAT8E5M2: u32 = 19;
const TYPE_FLOAT8E5M2FNUZ: u32 = 20;

#[derive(Debug)]
pub enum ElementType {
    Undefined,
    Float,
    Uint8,
    Int8,
    Uint16,
    Int16,
    Int32,
    Int64,
    String,
    Bool,
    Float16,
    Double,
    Uint32,
    Uint64,
    Complex64,
    Complex128,
    Bfloat16,
    Float8e4m3fn,
    Float8e4m3fnuz,
    Float8e5m2,
    Float8e5m2fnuz,
}

impl From<u32> for ElementType {
    fn from(value: u32) -> Self {
        match value {
            TYPE_FLOAT => Self::Float,
            TYPE_UINT8 => Self::Uint8,
            TYPE_INT8 => Self::Int8,
            TYPE_UINT16 => Self::Uint16,
            TYPE_INT16 => Self::Int16,
            TYPE_INT32 => Self::Int32,
            TYPE_INT64 => Self::Int64,
            TYPE_STRING => Self::String,
            TYPE_BOOL => Self::Bool,
            TYPE_FLOAT16 => Self::Float16,
            TYPE_DOUBLE => Self::Double,
            TYPE_UINT32 => Self::Uint32,
            TYPE_UINT64 => Self::Uint64,
            TYPE_COMPLEX64 => Self::Complex64,
            TYPE_COMPLEX128 => Self::Complex128,
            TYPE_BFLOAT16 => Self::Bfloat16,
            TYPE_FLOAT8E4M3FN => Self::Float8e4m3fn,
            TYPE_FLOAT8E4M3FNUZ => Self::Float8e4m3fnuz,
            TYPE_FLOAT8E5M2 => Self::Float8e5m2,
            TYPE_FLOAT8E5M2FNUZ => Self::Float8e5m2fnuz,
            _ => Self::Undefined,
        }
    }
}
