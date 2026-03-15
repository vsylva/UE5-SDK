use super::{APlayerController::APlayerController, Structs::FString::FString, UWorld::UWorld};

#[struct_macro::inherit(super::UBlueprintFunctionLibrary::UBlueprintFunctionLibrary, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct UKismetSystemLibrary {}

impl UKismetSystemLibrary {
    #[inline]
    unsafe fn get() -> *mut Self {
        static mut Instance: *mut super::UClass::UClass = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(0, -1, crate::w!("Class '/Script/Engine.UKismetSystemLibrary'"), false,);
        }

        Instance as *mut Self
    }

    #[inline]
    pub unsafe fn ExecuteConsoleCommand(
        WorldContextObject: *mut UWorld,
        Command: FString,
        SpecificPlayer: *mut APlayerController,
    ) {
        #[repr(C)]
        struct Params {
            WorldContextObject: *mut UWorld,
            Command:            FString,
            SpecificPlayer:     *mut APlayerController,
        }

        let mut params = Params { WorldContextObject, Command, SpecificPlayer, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'Engine.KismetSystemLibrary.ExecuteConsoleCommand'"),
                false,
            );
        }

        super::ProcessEvent(
            Self::get() as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }
}
