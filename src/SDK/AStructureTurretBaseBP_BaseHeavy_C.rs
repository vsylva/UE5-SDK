#[struct_macro::inherit(super::APrimalStructureTurret::APrimalStructureTurret, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct AStructureTurretBaseBP_BaseHeavy_C {
    #[offset(0x1470)]
    pub SkeletalMesh1: *mut super::USkeletalMeshComponent::USkeletalMeshComponent,
}

impl AStructureTurretBaseBP_BaseHeavy_C {
    #[inline]
    pub unsafe fn StaticClass() -> *mut super::UClass::UClass {
        pub static mut Instance: *mut super::UClass::UClass = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::UObject::UObject::find_object_fast_impl(
                "StructureTurretBaseBP_BaseHeavy_C",
                super::UClass::EClassCastFlags::UClass,
            )
            .cast();

            #[cfg(debug_assertions)]
            dbg!(Instance);
        }

        Instance
    }
}
