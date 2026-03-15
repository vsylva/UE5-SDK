#[struct_macro::inherit(super::AActor::AActor, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct AShooterProjectile {}

impl AShooterProjectile {
    #[inline]
    pub unsafe fn StaticClass() -> *mut super::UClass::UClass {
        pub static mut Instance: *mut super::UClass::UClass = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                "Instance '/Script/ShooterGame.ShooterProjectile'"
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
