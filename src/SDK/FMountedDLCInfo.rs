use super::Structs::FString::FString;

#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct FMountedDLCInfo {
    pub Name: FString,
    pub Path: FString,
    pub Map:  FString,
    pub ID:   FString,
}
