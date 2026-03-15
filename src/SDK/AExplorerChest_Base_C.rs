#[struct_macro::inherit(super::AActor::AActor, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct AExplorerChest_Base_C {
    #[offset(0x04A8)]
    pub bIsUnlocked: bool,

    #[offset(0x04AC)]
    pub ExplorerNoteIndex: i32,
}
