#[struct_macro::inherit(super::UActorComponent::UActorComponent, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct UPrimalCharacterStatusComponent {
    #[offset(0x00C8)]
    pub MaxStatusValues: MaxStatusValues,

    #[offset(0x0708)]
    pub BaseCharacterLevel:  i32,
    pub ExtraCharacterLevel: u16,

    #[offset(0x0858)]
    pub CurrentStatusValues: MaxStatusValues,
}

#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct MaxStatusValues {
    pub 生命: f32,
    pub 耐力: f32,
    pub 眩晕: f32,
    pub 氧气: f32,
    pub 食物: f32,
    pub 水:   f32,
    空7:      f32,
    pub 负重: f32,
    空8:      f32,
    空9:      f32,
    pub 抗性: f32,
    空11:     f32,
}
