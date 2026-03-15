#[struct_macro::inherit(super::APrimalStructureItemContainer::APrimalStructureItemContainer, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct APrimalStructureItemContainer_SupplyCrate {
    #[offset(0x1238)]
    pub RequiredLevelToAccess: i32,
}

impl APrimalStructureItemContainer_SupplyCrate {
    #[inline]
    pub unsafe fn StaticClass() -> *mut super::UClass::UClass {
        pub static mut Instance: *mut super::UClass::UClass = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::UObject::UObject::find_object_fast_impl(
                "PrimalStructureItemContainer_SupplyCrate",
                super::UClass::EClassCastFlags::UClass,
            )
            .cast();

            #[cfg(debug_assertions)]
            dbg!(Instance);
        }

        Instance
    }
}
