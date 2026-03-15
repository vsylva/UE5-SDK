use super::Structs::{FRotator::FRotator, FVector::FVector};

#[struct_macro::bitfields]
#[struct_macro::disinherit]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct FRepMovement {
    pub LinearVelocity: FVector,
    pub AngularVelocity: FVector,
    pub Location: FVector,
    pub Rotation: FRotator,
    pub Acceleration: FVector,
    #[bits(3, bSimulatedPhysicSleep)]
    bSimulatedPhysicSleep: bool,
    bRepPhysics: bool,
    bRepAcceleration: bool,
    Pad_79: [u8; 0x3],
    pub ServerFrame: i32,
    pub ServerPhysicsHandle: i32,
    pub LocationQuantizationLevel: EVectorQuantization,
    pub VelocityQuantizationLevel: EVectorQuantization,
    pub RotationQuantizationLevel: ERotatorQuantization,
    Pad_87: [u8; 0x1],
}

#[derive(Debug, Copy, Clone,)]
#[repr(u8)]
pub enum EVectorQuantization {
    RoundWholeNumber = 0,
    RoundOneDecimal = 1,
    RoundTwoDecimals = 2,
    EVectorQuantization_MAX = 3,
}

#[derive(Debug, Copy, Clone,)]
#[repr(u8)]
pub enum ERotatorQuantization {
    ByteComponents = 0,
    ShortComponents = 1,
    ERotatorQuantization_MAX = 2,
}
