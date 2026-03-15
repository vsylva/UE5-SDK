use super::Structs::{FLinearColor::FLinearColor, FName::FName};

#[struct_macro::inherit(super::UMaterialInstance::UMaterialInstance, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct UMaterialInstanceDynamic {}

unsafe impl Send for UMaterialInstanceDynamic {}
unsafe impl Sync for UMaterialInstanceDynamic {}

impl UMaterialInstanceDynamic {
    #[inline]
    pub unsafe fn SetVectorParameterValue(&self, ParameterName: FName, Value: FLinearColor,) {
        #[repr(C)]
        struct Params {
            ParameterName: FName,
            Value:         FLinearColor,
        }

        let mut params = Params { ParameterName, Value, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.MaterialInstanceDynamic.SetVectorParameterValue'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn SetScalarParameterValue(&self, ParameterName: FName, Value: f32,) {
        #[repr(C)]
        struct Params {
            ParameterName: FName,
            Value:         f32,
        }

        let mut params = Params { ParameterName, Value, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.MaterialInstanceDynamic.SetScalarParameterValue'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }
}
