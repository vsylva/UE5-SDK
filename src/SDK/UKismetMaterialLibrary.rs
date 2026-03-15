use super::{
    Structs::FName::FName, UMaterialInstanceDynamic::UMaterialInstanceDynamic, UMaterialInterface::UMaterialInterface,
    UWorld::UWorld,
};

#[struct_macro::inherit(super::UBlueprintFunctionLibrary::UBlueprintFunctionLibrary, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct UKismetMaterialLibrary {}

impl UKismetMaterialLibrary {
    #[inline]
    unsafe fn get() -> *mut Self {
        static mut Instance: *mut super::UClass::UClass = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Class '/Script/Engine.KismetMaterialLibrary'"), false,);
        }

        Instance as *mut Self
    }

    #[inline]
    pub unsafe fn CreateDynamicMaterialInstance(
        WorldContextObject: *mut UWorld,
        Parent: *mut UMaterialInterface,
        OptionalName: FName,
        CreationFlags: EMIDCreationFlags,
    ) -> *mut UMaterialInstanceDynamic {
        #[repr(C)]
        struct Params {
            WorldContextObject: *mut UWorld,
            Parent:             *mut UMaterialInterface,
            OptionalName:       FName,
            CreationFlags:      EMIDCreationFlags,
            _padding:           [u8; 7],
            ReturnValue:        *mut UMaterialInstanceDynamic,
        }

        let mut params = Params {
            WorldContextObject,
            Parent,
            OptionalName,
            CreationFlags,
            _padding: [0; 7],
            ReturnValue: ::core::ptr::null_mut(),
        };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.KismetMaterialLibrary.CreateDynamicMaterialInstance'"),
                false,
            );
        }

        super::ProcessEvent(
            Self::get().addr(),
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.ReturnValue
    }
}

#[repr(u8)]
pub enum EMIDCreationFlags {
    None = 0,
    Transient = 1,
    EMIDCreationFlags_MAX = 2,
}
