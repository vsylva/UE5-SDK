#[struct_macro::inherit(super::AShooterPlayerController::AShooterPlayerController, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct APlayerControllerBlueprint_C {}

impl APlayerControllerBlueprint_C {
    #[inline]
    pub unsafe fn TogglePhotoMode(&self,) {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'PlayerControllerBlueprint.PlayerControllerBlueprint_C.TogglePhotoMode'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            0 as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }
}
