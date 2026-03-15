use super::{
    AActor::AActor,
    Structs::{FLinearColor::FLinearColor, FVector::FVector},
    UWorld::UWorld,
};

#[struct_macro::inherit(super::UBlueprintFunctionLibrary::UBlueprintFunctionLibrary, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct UGameplayStatics {}

impl UGameplayStatics {
    #[inline]
    unsafe fn get() -> *mut Self {
        static mut Instance: *mut super::UClass::UClass = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(0, -1, crate::w!("Class '/Script/Engine.GameplayStatics'"), false,);
        }

        Instance as *mut Self
    }

    #[inline]
    pub unsafe fn SuggestProjectileVelocity_MovingTarget(
        WorldContextObject: *mut UWorld,
        OutLaunchVelocity: *mut FVector,
        ProjectileStartLocation: FVector,
        TargetActor: *mut AActor,
        TargetLocationOffset: FVector,
        GravityZOverride: f64,
        TimeToTarget: f64,
        DrawDebugType: u8,
        DrawDebugTime: f32,
        DrawDebugColor: FLinearColor,
    ) -> bool {
        #[repr(C)]
        struct Params {
            WorldContextObject:      *mut UWorld,
            OutLaunchVelocity:       FVector,
            ProjectileStartLocation: FVector,
            TargetActor:             *mut AActor,
            TargetLocationOffset:    FVector,
            GravityZOverride:        f64,
            TimeToTarget:            f64,
            DrawDebugType:           u8,
            Pad_69:                  [u8; 0x3],
            DrawDebugTime:           f32,
            DrawDebugColor:          FLinearColor,
            ReturnValue:             bool,
            Pad_81:                  [u8; 0x7],
        }

        let mut params = Params {
            WorldContextObject,
            OutLaunchVelocity: FVector::zero(),
            ProjectileStartLocation,
            TargetActor,
            TargetLocationOffset,
            GravityZOverride,
            TimeToTarget,
            DrawDebugType,
            Pad_69: [0u8; 0x3],
            DrawDebugTime,
            DrawDebugColor,
            ReturnValue: false,
            Pad_81: [0u8; 0x7],
        };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'Engine.GameplayStatics.SuggestProjectileVelocity_MovingTarget'"),
                false,
            );
        }

        super::ProcessEvent(
            Self::get().addr(),
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        if !OutLaunchVelocity.is_null()
        {
            *OutLaunchVelocity = params.OutLaunchVelocity;
        }

        params.ReturnValue
    }
}
