use super::{Structs::TArray::TArray, UPrimalItem::UPrimalItem};

#[struct_macro::bitfields]
#[struct_macro::inherit(super::UActorComponent::UActorComponent, Flags)]
#[derive(Debug, Clone,)]
#[repr(C)]
pub struct UPrimalInventoryComponent {
    #[offset(0x0120)]
    pub InventoryItems: TArray<*mut UPrimalItem,>,

    #[offset(0x0140)]
    pub EquippedItems: TArray<*mut UPrimalItem,>,
    pub ItemSlots:     TArray<*mut UPrimalItem,>,

    #[offset(0x01B4)]
    #[bits(3, bReceivingInventoryItems)]
    _pad_bit_0:                   bool,
    _pad_bit_1:                   bool,
    pub bReceivingInventoryItems: bool,

    #[offset(0x01B6)]
    #[bits(7, bForceInventoryBlueprints)]
    pub bForceInventoryBlueprints: bool,
    _pad_bit_1:                    bool,
    _pad_bit_2:                    bool,
    _pad_bit_3:                    bool,
    _pad_bit_4:                    bool,
    _pad_bit_5:                    bool,
    pub bIsTributeInventory:       bool,

    #[offset(0x03B8)]
    pub MaxRemoteInventoryViewingDistance: f32,

    #[offset(0x04AC)]
    pub MinItemSets: f32,
    pub MaxItemSets: f32,

    #[offset(0x0520)]
    pub MaxInventoryAccessDistance: f32,
}

impl UPrimalInventoryComponent {
    #[inline]
    pub unsafe fn GetEquippedItemOfType(&self, aType: EPrimalEquipmentType,) -> *mut UPrimalItem {
        #[repr(C)]
        struct Params {
            aType:       EPrimalEquipmentType,
            Pad_1:       [u8; 0x7],
            ReturnValue: *mut UPrimalItem,
        }

        let mut params: Params = Params { aType, Pad_1: [0u8; 0x7], ReturnValue: ::core::mem::zeroed(), };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.PrimalInventoryComponent.GetEquippedItemOfType'"),
                false,
            );
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
    pub unsafe fn TransferAllItemsToInventory(&self, ToInventory: *mut UPrimalInventoryComponent,) {
        #[repr(C)]
        struct Params {
            ToInventory: *mut UPrimalInventoryComponent,
        }

        let mut params: Params = Params { ToInventory, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.PrimalInventoryComponent.TransferAllItemsToInventory'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn TransferItemToInventory(&self, ToInventory: *mut UPrimalInventoryComponent, ItemIndex: i32,) {
        #[repr(C)]
        struct Params {
            ToInventory: *mut UPrimalInventoryComponent,
            ItemIndex:   i32,
            Pad_C:       [u8; 0x4],
        }

        let mut params: Params = Params { ToInventory, ItemIndex, Pad_C: [0u8; 0x4], };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.PrimalInventoryComponent.TransferItemToInventory'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd,)]
#[repr(u8)]
pub enum EPrimalEquipmentType {
    Hat = 0,
    Shirt = 1,
    Pants = 2,
    Boots = 3,
    Gloves = 4,
    DinoSaddle = 5,
    Trophy = 6,
    Costume = 7,
    Shield = 8,
    Weapon = 9,
    Snapshot = 10,
    Pet = 11,
    Cart = 12,
    Gear = 13,
    Accessory0 = 14,
    Accessory1 = 15,
    Accessory2 = 16,
    Accessory3 = 17,
    Accessory4 = 18,
    AccessoryMAX = 19,
    MAX = 20,
}
