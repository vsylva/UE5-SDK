use super::{APawn::APawn, Structs::FString::FString};

#[struct_macro::inherit(super::AInfo::AInfo, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct APlayerState {
    #[offset(0x04D8)]
    pub SavedNetworkAddress: FString,

    #[offset(0x0500)]
    pub PawnPrivate: *mut APawn,
}

impl APlayerState {
    #[inline]
    pub unsafe fn GetExactPing(&self,) -> f32 {
        #[repr(C)]
        struct Params {
            ReturnValue: f32,
        }

        let mut params = Params { ReturnValue: 0.0, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(0, -1, crate::w!("Function 'Engine.PlayerState.GetExactPing'"), false,);
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
