#[struct_macro::inherit(super::USceneComponent::USceneComponent, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct UPrimitiveComponent {
    #[offset(0x028C)]
    pub DepthPriorityGroup: ESceneDepthPriorityGroup,
}

#[derive(Debug, Copy, Clone,)]
#[repr(u8)]
pub enum ESceneDepthPriorityGroup {
    SDPG_World = 0,
    SDPG_Foreground = 1,
    SDPG_MAX = 2,
}
