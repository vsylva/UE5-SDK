use super::{ProcessEvent, UPrimalInventoryComponent::UPrimalInventoryComponent};

#[struct_macro::inherit(super::AHUD::AHUD, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct AShooterHUD {}

impl AShooterHUD {
    #[inline]
    pub unsafe fn ShowInventory(&self, InventoryComp: *mut UPrimalInventoryComponent,) -> usize {
        #[repr(C)]
        struct Params {
            InventoryComp: *mut UPrimalInventoryComponent,
            ReturnValue:   usize,
        }

        let mut params = Params { InventoryComp, ..::core::mem::zeroed() };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                "Function 'ShooterGame.ShooterHUD.ShowInventory'"
                    .encode_utf16()
                    .chain(::core::iter::once(0,),)
                    .collect::<Vec<u16,>>()
                    .as_ptr(),
                false,
            );
        }

        ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.ReturnValue
    }
}
