#[struct_macro::inherit(super::APrimalStructureSeating::APrimalStructureSeating, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct AStandingMountedTurret {}

impl AStandingMountedTurret {
    #[inline]
    pub unsafe fn StaticClass() -> *mut super::UClass::UClass {
        static mut Class: *mut super::UClass::UClass = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Class.is_null(),)
        {
            Class = super::StaticFindObject(
                0,
                -1,
                "Class '/Script/ShooterGame.StandingMountedTurret'"
                    .encode_utf16()
                    .chain(::core::iter::once(0,),)
                    .collect::<Vec<u16,>>()
                    .as_ptr(),
                false,
            );

            #[cfg(debug_assertions)]
            dbg!(Class);
        }

        Class
    }
}
