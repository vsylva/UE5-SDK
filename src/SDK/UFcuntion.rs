#[struct_macro::inherit(super::UStruct::UStruct, Flags)]
#[repr(C)]
pub struct UFunction {
    #[offset(0x00D8)]
    pub ExecFunction: *mut (),
}
