#[struct_macro::inherit(super::UShapeComponent::UShapeComponent, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct UCapsuleComponent {
    #[offset(0x0628)]
    pub CapsuleHalfHeight: f32,
    pub CapsuleRadius:     f32,
}
