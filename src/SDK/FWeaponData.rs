#[struct_macro::disinherit]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct FWeaponData {
    #[offset(0x0014)]
    pub TimeBetweenShots: f32,
}
