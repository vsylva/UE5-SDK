use super::Structs::{FString::FString, TArray::TArray};

#[struct_macro::disinherit]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct FTribeData {
    #[offset(0x00B8)]
    pub TribeLog: TArray<FString,>,
    pub LogIndex: i32,
}
