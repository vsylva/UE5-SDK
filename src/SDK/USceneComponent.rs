use super::Structs::{FName::FName, FTransform::FTransform, FVector::FVector};

#[doc = "ComponentToWorld与PhysicsVolumeChangedDelegate相差0x40"]
#[struct_macro::inherit(super::UActorComponent::UActorComponent, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct USceneComponent {
    #[offset(0x0148)]
    pub RelativeLocation: super::Structs::FVector::FVector,

    #[offset(0x01B8)]
    PhysicsVolumeChangedDelegate: usize,

    #[offset(0x01F8)]
    pub ComponentToWorld: super::Structs::FTransform::FTransform,
}

impl USceneComponent {
    #[inline]
    pub unsafe fn K2_GetComponentToWorld(&self,) -> FTransform {
        #[repr(C)]
        struct Params {
            return_value: FTransform,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        pub static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'Engine.SceneComponent.K2_GetComponentToWorld'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.return_value
    }

    #[inline]
    pub unsafe fn K2_GetComponentLocation(&self,) -> FVector {
        #[repr(C)]
        struct Params {
            return_value: FVector,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        pub static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'Engine.SceneComponent.K2_GetComponentLocation'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.return_value
    }

    #[inline]
    pub unsafe fn GetSocketLocation(&self, InSocketName: FName,) -> FVector {
        #[repr(C)]
        struct Params {
            InSocketName: FName,
            return_value: FVector,
        }

        let mut params = Params { InSocketName, return_value: FVector::zero(), };

        pub static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function 'Engine.SceneComponent.GetSocketLocation'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.return_value
    }
}
