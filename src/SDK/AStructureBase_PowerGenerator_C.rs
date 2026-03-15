#[struct_macro::inherit(super::ABaseFuelBurner_C::ABaseFuelBurner_C, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct AStructureBase_PowerGenerator_C {}

impl AStructureBase_PowerGenerator_C {
    #[inline]
    pub unsafe fn StaticClass() -> *mut super::UClass::UClass {
        pub static mut Instance: *mut super::UClass::UClass = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::UObject::UObject::find_object_fast_impl(
                "StructureBase_PowerGenerator_C",
                super::UClass::EClassCastFlags::UClass,
            )
            .cast();

            #[cfg(debug_assertions)]
            dbg!(Instance);
        }

        Instance
    }
}
