use super::UPrimalInventoryComponent::UPrimalInventoryComponent;

#[struct_macro::bitfields]
#[struct_macro::inherit(super::APrimalStructure::APrimalStructure, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct APrimalStructureItemContainer {
    #[offset(0x0B48)]
    pub MyInventoryComponent: *mut UPrimalInventoryComponent,

    #[offset(0x0B50)]
    #[bits(8, bContainerActivated)]
    _b48_0:                  bool,
    _b48_1:                  bool,
    _b48_2:                  bool,
    _b48_3:                  bool,
    _b48_4:                  bool,
    _b48_5:                  bool,
    _b48_6:                  bool,
    pub bContainerActivated: bool,

    #[offset(0x0BB8)]
    #[bits(1, bIsLocked)]
    pub bIsLocked: bool,

    #[offset(0x0BD5)]
    #[bits(7, bIsPowered)]
    _bcd_0:         bool,
    _bcd_1:         bool,
    _bcd_2:         bool,
    _bcd_3:         bool,
    _bcd_4:         bool,
    _bcd_5:         bool,
    pub bIsPowered: bool,

    #[offset(0x0C7C)]
    pub CurrentItemCount: i32,
}

impl APrimalStructureItemContainer {
    #[inline]
    pub unsafe fn StaticClass() -> *mut super::UClass::UClass {
        pub static mut Instance: *mut super::UClass::UClass = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                "Instance '/Script/ShooterGame.PrimalStructureItemContainer'"
                    .encode_utf16()
                    .chain(::core::iter::once(0,),)
                    .collect::<Vec<u16,>>()
                    .as_ptr(),
                false,
            );

            #[cfg(debug_assertions)]
            dbg!(Instance);
        }

        Instance
    }
}
