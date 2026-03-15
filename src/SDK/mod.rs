pub mod AActor;
pub mod ABaseFuelBurner_C;
pub mod ABasePlayerController;
pub mod ABasePrimalWorldSettings;
pub mod ACharacter;
pub mod AController;
pub mod ADroppedItem;
pub mod AExplorerChest_Base_C;
pub mod AGameState;
pub mod AGameStateBase;
pub mod AHUD;
pub mod AInfo;
pub mod APawn;
pub mod APlayerCameraManager;
pub mod APlayerController;
pub mod APlayerControllerBlueprint_C;
pub mod APlayerState;
pub mod APrimalCharacter;
pub mod APrimalController;
pub mod APrimalDinoCharacter;
pub mod APrimalPawn;
pub mod APrimalPlayerController;
pub mod APrimalStructure;
pub mod APrimalStructureItemContainer;
pub mod APrimalStructureItemContainer_SupplyCrate;
pub mod APrimalStructureSeating;
pub mod APrimalStructureTurret;
pub mod APrimalTargetableActor;
pub mod AShooterCharacter;
pub mod AShooterGameState;
pub mod AShooterHUD;
pub mod AShooterPlayerController;
pub mod AShooterPlayerState;
pub mod AShooterProjectile;
pub mod AShooterWeapon;
pub mod AStandingMountedTurret;
pub mod AStructureBase_PowerGenerator_C;
pub mod AStructureItemContainerBaseBP_C;
pub mod AStructureTurretBaseBP_BaseHeavy_C;
pub mod AStructureTurretBaseBP_C;
pub mod AStructureTurretTek_C;
pub mod AWorldSettings;
pub mod DeathItemCache_C;
pub mod FActorInstanceHandle;
pub mod FCameraCacheEntry;
pub mod FEpicIDToSteamID;
pub mod FField;
pub mod FFieldClass;
pub mod FHitResult;
pub mod FInstantWeaponData;
pub mod FItemNetID;
pub mod FMinimalViewInfo;
pub mod FMountedDLCInfo;
pub mod FOverlappedFoliageElement;
pub mod FProperty;
pub mod FRepMovement;
pub mod FSavedMove_Character;
pub mod FTribeData;
pub mod FWeaponData;
pub mod Structs;
pub mod UActorComponent;
pub mod UBlueprintFunctionLibrary;
pub mod UCameraShakeBase;
pub mod UCanvas;
pub mod UCapsuleComponent;
pub mod UCharacterMovementComponent;
pub mod UClass;
pub mod UEngine;
pub mod UFcuntion;
pub mod UField;
pub mod UFont;
pub mod UGameInstance;
pub mod UGameViewportClient;
pub mod UGameplayStatics;
pub mod UInstancedStaticMeshComponent;
pub mod UKismetMaterialLibrary;
pub mod UKismetMathLibrary;
pub mod UKismetStringLibrary;
pub mod UKismetSystemLibrary;
pub mod ULegacyCameraShake;
pub mod ULevel;
pub mod ULocalPlayer;
pub mod UMaterial;
pub mod UMaterialInstance;
pub mod UMaterialInstanceDynamic;
pub mod UMaterialInterface;
pub mod UMeshComponent;
pub mod UMountedDLCManager;
pub mod UMovementComponent;
pub mod UNavMovementComponent;
pub mod UNetDriver;
pub mod UObject;
pub mod UPawnMovementComponent;
pub mod UPlayer;
pub mod UPrimalActor;
pub mod UPrimalCharacterStatusComponent;
pub mod UPrimalHarvestingComponent;
pub mod UPrimalInventoryComponent;
pub mod UPrimalItem;
pub mod UPrimalWorld;
pub mod UPrimitiveComponent;
pub mod USceneComponent;
pub mod UScriptViewportClient;
pub mod UShapeComponent;
pub mod USkeletalMeshComponent;
pub mod USkinnedMeshComponent;
pub mod UStaticMeshComponent;
pub mod UStreamableRenderAsset;
pub mod UStruct;
pub mod UTexture;
pub mod UTexture2D;
pub mod UVictoryCore;
pub mod UWorld;
pub mod mod_impls;

pub mod Globals {
    use super::Structs::FVector2D::FVector2D;

    pub static mut ModuleBase: usize = 0;
    pub static mut ModuleSize: usize = 0;

    pub static mut GworldPP: *mut *mut super::UWorld::UWorld = ::core::ptr::null_mut();
    pub static mut GnamesP: usize = 0;
    pub static mut GobjectsP: *mut super::Structs::TUObjectArray::TUObjectArray = ::core::ptr::null_mut();

    pub static mut AppendStringFnAddr: usize = 0;
    pub static mut GetNmaeEntryFnAddr: usize = 0;

    pub static mut ProcessEventFnAddr: usize = 0;
    pub static mut StaticFindObjectFnAddr: usize = 0;

    pub static mut JmpRbxRegGadGet: usize = 0;

    pub static mut SCREEN_SIZE: super::Structs::FVector2D::FVector2D =
        super::Structs::FVector2D::FVector2D { X: 1920.0, Y: 1080.0, };
    pub static mut SCREEN_CENTER: FVector2D = super::Structs::FVector2D::FVector2D::zero();
    pub static mut SCREEN_SCALE: f64 = 1.0;
}

#[deny(dead_code)]
#[inline]
pub(crate) unsafe fn setup() -> bool {
    Globals::ModuleBase = todo!();

    if Globals::ModuleBase == 0
    {
        return false;
    }

    Globals::ModuleSize = todo!();

    if Globals::ModuleSize == 0
    {
        return false;
    }

    Globals::JmpRbxRegGadGet = todo!();

    if Globals::JmpRbxRegGadGet == 0
    {
        return false;
    }

    Globals::GworldPP = todo!();
    if Globals::GworldPP.is_null()
    {
        return false;
    }

    Globals::GnamesP = todo!();
    if Globals::GnamesP == 0
    {
        return false;
    }

    Globals::GobjectsP = todo!();

    if Globals::GobjectsP.is_null()
    {
        return false;
    }

    Globals::GetNmaeEntryFnAddr = todo!();

    if Globals::GetNmaeEntryFnAddr == 0
    {
        return false;
    }

    Globals::AppendStringFnAddr = todo!();

    if Globals::AppendStringFnAddr == 0
    {
        return false;
    }

    Globals::StaticFindObjectFnAddr = todo!();

    if Globals::StaticFindObjectFnAddr == 0
    {
        return false;
    }

    Globals::ProcessEventFnAddr = todo!();

    if Globals::ProcessEventFnAddr == 0
    {
        return false;
    }

    true
}

#[inline(always)]
pub unsafe extern "system" fn StaticFindObject<T,>(a: isize, b: isize, c: *const u16, d: bool,) -> *mut T {
    crate::SpoofCall!(Globals::StaticFindObjectFnAddr,a, b, c, d,-> *mut T)
}

#[inline]
pub unsafe extern "system" fn ProcessEvent(a: usize, b: usize, c: usize, d: usize,) {
    crate::SpoofCall!(Globals::ProcessEventFnAddr, a, b, c, d)
}

#[inline]
pub unsafe fn SpoofCallInternal<Ret, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11,>(
    fn_ptr: usize,
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
    a5: A5,
    a6: A6,
    a7: A7,
    a8: A8,
    a9: A9,
    a10: A10,
    a11: A11,
) -> Ret {
    todo!()
}

#[doc = "You MUST add `use <SDK>::spoof_call;` in your project root (e.g., lib.rs) to make this macro available globally"]
#[macro_export]
macro_rules! SpoofCall {
    ($fn_ptr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, -> $ret:ty) => {
        unsafe {
            $crate::SpoofCallInternal::<$ret, _, _, _, _, _, _, _, _, _, _, _>(
                $fn_ptr, $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8, $a9, $a10, $a11,
            )
        }
    };

    ($fn_ptr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr) => {
        $crate::SpoofCall!($fn_ptr, $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8, $a9, $a10, $a11, => ())
    };


    ($fn_ptr:expr $(, $args:expr)*, -> $ret:ty) => {
        $crate::SpoofCall!($fn_ptr $(, $args)*, 0usize, -> $ret)
    };


    ($fn_ptr:expr $(, $args:expr)*) => {
        $crate::SpoofCall!($fn_ptr $(, $args)*, 0usize, -> ())
    };
}

#[macro_export]
macro_rules! w {
    ($s:literal) => {{
        const INPUT: &[u8] = $s.as_bytes();

        macro_rules! decode_utf8 {
            ($bytes: expr,$pos: expr) => {{
                let bytes = $bytes;
                let mut pos = $pos;
                if bytes.len() == pos
                {
                    None
                }
                else
                {
                    let ch = bytes[pos] as u32;
                    pos += 1;
                    if ch <= 0x7f
                    {
                        Some((ch, pos,),)
                    }
                    else if (ch & 0xe0) == 0xc0
                    {
                        if bytes.len() - pos < 1
                        {
                            None
                        }
                        else
                        {
                            let ch2 = bytes[pos] as u32;
                            pos += 1;
                            if (ch2 & 0xc0) != 0x80
                            {
                                None
                            }
                            else
                            {
                                let result = ((ch & 0x1f) << 6) | (ch2 & 0x3f);
                                if result <= 0x7f { None } else { Some((result, pos,),) }
                            }
                        }
                    }
                    else if (ch & 0xf0) == 0xe0
                    {
                        if bytes.len() - pos < 2
                        {
                            None
                        }
                        else
                        {
                            let ch2 = bytes[pos] as u32;
                            pos += 1;
                            let ch3 = bytes[pos] as u32;
                            pos += 1;
                            if (ch2 & 0xc0) != 0x80 || (ch3 & 0xc0) != 0x80
                            {
                                None
                            }
                            else
                            {
                                let result = ((ch & 0x0f) << 12) | ((ch2 & 0x3f) << 6) | (ch3 & 0x3f);
                                if result <= 0x7ff || (0xd800 <= result && result <= 0xdfff)
                                {
                                    None
                                }
                                else
                                {
                                    Some((result, pos,),)
                                }
                            }
                        }
                    }
                    else if (ch & 0xf8) == 0xf0
                    {
                        if bytes.len() - pos < 3
                        {
                            None
                        }
                        else
                        {
                            let ch2 = bytes[pos] as u32;
                            pos += 1;
                            let ch3 = bytes[pos] as u32;
                            pos += 1;
                            let ch4 = bytes[pos] as u32;
                            pos += 1;
                            if (ch2 & 0xc0) != 0x80 || (ch3 & 0xc0) != 0x80 || (ch4 & 0xc0) != 0x80
                            {
                                None
                            }
                            else
                            {
                                let result =
                                    ((ch & 0x07) << 18) | ((ch2 & 0x3f) << 12) | ((ch3 & 0x3f) << 6) | (ch4 & 0x3f);
                                if result <= 0xffff || 0x10ffff < result { None } else { Some((result, pos,),) }
                            }
                        }
                    }
                    else
                    {
                        None
                    }
                }
            }};
        }

        const OUTPUT_LEN: usize = {
            let mut pos = 0usize;
            let mut len = 0usize;
            while let Some((code_point, new_pos,),) = decode_utf8!(INPUT, pos)
            {
                pos = new_pos;
                len += if code_point <= 0xffff { 1 } else { 2 };
            }
            len + 1
        };

        const OUTPUT: &[u16; OUTPUT_LEN] = {
            let mut buffer = [0u16; OUTPUT_LEN];
            let mut input_pos = 0usize;
            let mut output_pos = 0usize;
            while let Some((mut code_point, new_pos,),) = decode_utf8!(INPUT, input_pos)
            {
                input_pos = new_pos;
                if code_point <= 0xffff
                {
                    buffer[output_pos] = code_point as u16;
                    output_pos += 1;
                }
                else
                {
                    code_point -= 0x10000;
                    buffer[output_pos] = 0xd800 + (code_point >> 10) as u16;
                    output_pos += 1;
                    buffer[output_pos] = 0xdc00 + (code_point & 0x3ff) as u16;
                    output_pos += 1;
                }
            }
            &{ buffer }
        };

        OUTPUT.as_ptr()
    }};
}
