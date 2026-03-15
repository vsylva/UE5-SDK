use super::{
    Structs::{FVector::FVector, TSubclassOf::TSubclassOf},
    ULegacyCameraShake::ULegacyCameraShake,
    UMaterialInterface::UMaterialInterface,
};

#[struct_macro::bitfields]
#[struct_macro::inherit(super::AActor::AActor, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct AShooterWeapon {
    #[offset(0x0708)]
    pub AssociatedPrimalItem: *mut super::UPrimalItem::UPrimalItem,

    #[offset(0x0950)]
    pub WeaponConfig: super::FWeaponData::FWeaponData,

    #[offset(0x0AF8)]
    pub FireCameraShake: TSubclassOf<ULegacyCameraShake,>,

    #[offset(0x0CC8)]
    #[bits(2, bIsSpyglass)]
    _bit_pad_c8_0:   bool,
    pub bIsSpyglass: bool,

    #[offset(0x0CD0)]
    pub WeaponUnequipDelay: f32,

    #[offset(0x0CF8)]
    pub LastFireTime: f32,

    #[offset(0x0D0E)]
    #[bits(8, bForceKeepEquippedWhileInInventory)]
    _bit_pad_0e_0: bool,
    _bit_pad_0e_1: bool,
    _bit_pad_0e_2: bool,
    _bit_pad_0e_3: bool,
    _bit_pad_0e_4: bool,
    _bit_pad_0e_5: bool,
    _bit_pad_0e_6: bool,
    pub bForceKeepEquippedWhileInInventory: bool,

    #[offset(0x0D40)]
    pub LastNotifyShotTime: f32,

    #[offset(0x0D98)]
    pub ScopeOverlayMI: *mut UMaterialInterface,

    #[offset(0x0DBC)]
    #[bits(7, bUseScopeOverlay)]
    _bit_pad_bc_0:                   bool,
    _bit_pad_bc_1:                   bool,
    _bit_pad_bc_2:                   bool,
    _bit_pad_bc_3:                   bool,
    _bit_pad_bc_4:                   bool,
    pub bApplyAimDriftWhenTargeting: bool,
    pub bUseScopeOverlay:            bool,

    #[offset(0x0DC5)]
    #[bits(5, bAllowRunning)]
    _bit_pad_c5_0:     bool,
    _bit_pad_c5_1:     bool,
    _bit_pad_c5_2:     bool,
    _bit_pad_c5_3:     bool,
    pub bAllowRunning: bool,

    #[offset(0x0DC6)]
    #[bits(8, bAllowRunningWhileFiring)]
    _bit_pad_c6_0:                bool,
    _bit_pad_c6_1:                bool,
    _bit_pad_c6_2:                bool,
    _bit_pad_c6_3:                bool,
    _bit_pad_c6_4:                bool,
    _bit_pad_c6_5:                bool,
    _bit_pad_c6_6:                bool,
    pub bAllowRunningWhileFiring: bool,

    #[offset(0x0DC7)]
    #[bits(1, bAllowRunningWhileReloading)]
    pub bAllowRunningWhileReloading: bool,

    #[offset(0x0DD4)]
    #[bits(1, bForceFirstPersonWhileTargeting)]
    pub bForceFirstPersonWhileTargeting: bool,

    #[offset(0x0DDC)]
    pub TargetingDelayTime: f32,

    #[offset(0x0DEC)]
    pub AimDriftYawFrequency: f32,

    #[offset(0x0DF0)]
    pub AimDriftPitchFrequency: f32,

    #[offset(0x0E24)]
    pub GlobalFireCameraShakeScaleTargeting: f32,

    #[offset(0x0E2C)]
    pub ReloadCameraShakeSpeedScale: f32,

    #[offset(0x0F0D)]
    #[bits(1, bForceAllowPassengerTPV)]
    pub bForceAllowPassengerTPV: bool,

    #[offset(0x0F50)]
    pub InstantConfig: super::FInstantWeaponData::FInstantWeaponData,

    #[offset(0x0F80)]
    pub CurrentFiringSpread: f32,

    #[offset(0x1000)]
    pub bForceTPV_EquippedWhileRiding: bool,

    #[offset(0x1034)]
    pub bForceTPVCameraOffset: bool,
}

impl AShooterWeapon {
    #[inline]
    pub unsafe fn GetMuzzleLocation(&self,) -> FVector {
        #[repr(C)]
        struct Params {
            return_value: FVector,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.ShooterWeapon.GetMuzzleLocation'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.return_value
    }
}
