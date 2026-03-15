use super::{Structs::TWeakObjectPtr::TWeakObjectPtr, UObject::UObject};

#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct FActorInstanceHandle {
    pub ReferenceObject: TWeakObjectPtr<UObject,>,
    Pad_8:               [u8; 0x18],
}
