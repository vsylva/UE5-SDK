use ::core::ffi::c_void;

use super::{FFieldClass::FFieldClass, Structs::FName::FName};

#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct FField {
    pub VTable: *mut c_void,

    pub ClassPrivate: *mut FFieldClass,

    pub Owner: [u8; 0x8],

    pub Next: *mut FField,

    pub Name: FName,

    pub ObjFlags: i32,
}
