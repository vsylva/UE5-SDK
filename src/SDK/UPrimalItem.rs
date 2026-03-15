use super::{
    AShooterPlayerController::AShooterPlayerController, FItemNetID::FItemNetID, Structs::FString::FString,
    UMaterialInterface::UMaterialInterface, UTexture2D::UTexture2D,
};

#[struct_macro::bitfields]
#[struct_macro::inherit(super::UObject::UObject, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct UPrimalItem {
    #[offset(0x0570)]
    pub ItemID: FItemNetID,

    #[offset(0x0588)]
    pub DescriptiveNameBase: FString,

    #[offset(0x08F8)]
    pub ItemIconMaterialParent: *mut UMaterialInterface,

    #[offset(0x09F4)]
    pub ItemDurability: f32,

    #[offset(0x0A2C)]
    pub MaxDurabiltiyOverride: f32,

    #[offset(0x0A48)]
    pub ItemQuantity: i32,

    #[offset(0x0AE8)]
    pub MyItemType: EPrimalItemType,

    #[offset(0x0AEA)]
    pub MyEquipmentType: super::UPrimalInventoryComponent::EPrimalEquipmentType,

    #[offset(0x0B15)]
    #[bits(2, bIsEngram)]
    _pad_bit_0:    bool,
    pub bIsEngram: bool,

    #[offset(0x0B1A)]
    #[bits(1, bIsItemSkin)]
    pub bIsItemSkin: bool,
}

impl UPrimalItem {
    #[inline]
    pub unsafe fn BPGetItemIcon(&self, ForPC: *mut AShooterPlayerController,) -> *mut UTexture2D {
        #[repr(C)]
        struct Params {
            ForPC:           *mut AShooterPlayerController,
            pub ReturnValue: *mut UTexture2D,
        }

        let mut params: Params = Params { ForPC, ReturnValue: ::core::ptr::null_mut(), };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function 'ShooterGame.PrimalItem.BPGetItemIcon'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.ReturnValue
    }

    #[inline]
    pub unsafe fn CanDrop(&self,) -> bool {
        #[repr(C)]
        struct Params {
            pub ReturnValue: bool,
        }

        let mut params: Params = Params { ReturnValue: false, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(0, -1, crate::w!("Function 'ShooterGame.PrimalItem.CanDrop'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.ReturnValue
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd,)]
#[repr(u8)]
pub enum EPrimalItemType {
    MiscConsumable = 0,
    Equipment = 1,
    Weapon = 2,
    Ammo = 3,
    Structure = 4,
    Resource = 5,
    Skin = 6,
    WeaponAttachment = 7,
    Artifact = 8,
    MAX = 9,
}
