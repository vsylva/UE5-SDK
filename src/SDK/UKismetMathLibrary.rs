use super::Structs::{FQuat::FQuat, FRotator::FRotator, FVector::FVector};

#[struct_macro::inherit(super::UBlueprintFunctionLibrary::UBlueprintFunctionLibrary, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct UKismetMathLibrary {}

impl UKismetMathLibrary {
    #[inline]
    unsafe fn get() -> *mut Self {
        static mut Instance: *mut super::UClass::UClass = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(0, -1, crate::w!("Class '/Script/Engine.KismetMathLibrary'"), false,);
        }

        Instance as *mut Self
    }

    #[inline]
    pub unsafe fn Quat_RotateVector(Q: FQuat, V: FVector,) -> FVector {
        #[repr(C)]
        struct Params {
            Q:           FQuat,
            V:           FVector,
            ReturnValue: FVector,
        }

        let mut params = Params { Q, V, ReturnValue: FVector::zero(), };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'Engine.KismetMathLibrary.Quat_RotateVector'"),
                false,
            );
        }

        super::ProcessEvent(
            Self::get() as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.ReturnValue
    }

    #[inline]
    pub unsafe fn Conv_RotatorToQuaternion(InRot: FRotator,) -> FQuat {
        #[repr(C)]
        struct Params {
            InRot:       FRotator,
            Pad_18:      [u8; 0x8],
            ReturnValue: FQuat,
        }

        let mut params = Params { InRot, Pad_18: [0u8; 0x8], ReturnValue: FQuat::zero(), };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'Engine.KismetMathLibrary.Conv_RotatorToQuaternion'"),
                false,
            );
        }

        super::ProcessEvent(
            Self::get() as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.ReturnValue
    }

    #[inline]
    pub unsafe fn FindLookAtRotation(Start: FVector, Target: FVector,) -> FRotator {
        #[repr(C)]
        struct Params {
            Start:       FVector,
            Target:      FVector,
            ReturnValue: FRotator,
        }

        let mut params = Params { Start, Target, ReturnValue: FRotator::zero(), };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'Engine.KismetMathLibrary.FindLookAtRotation'"),
                false,
            );
        }

        super::ProcessEvent(
            Self::get() as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.ReturnValue
    }

    #[inline]
    pub unsafe fn RInterpTo(Current: FRotator, Target: FRotator, DeltaTime: f32, InterpSpeed: f32,) -> FRotator {
        #[repr(C)]
        struct Params {
            Current:     FRotator,
            Target:      FRotator,
            DeltaTime:   f32,
            InterpSpeed: f32,
            ReturnValue: FRotator,
        }

        let mut params = Params { Current, Target, DeltaTime, InterpSpeed, ReturnValue: FRotator::zero(), };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function 'Engine.KismetMathLibrary.RInterpTo'"), false,);
        }

        super::ProcessEvent(
            Self::get() as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.ReturnValue
    }
}
