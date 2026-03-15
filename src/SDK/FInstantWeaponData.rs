#[struct_macro::disinherit]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct FInstantWeaponData {
    pub WeaponSpread: f32,

    #[offset(0x0008)]
    pub FinalWeaponSpreadMultiplier: f32,

    #[offset(0x0010)]
    pub FiringSpreadMax: f32,

    #[offset(0x0014)]
    pub WeaponRange: f32,
}
