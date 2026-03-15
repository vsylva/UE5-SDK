#![feature(arbitrary_self_types_pointers)]
#![feature(core_intrinsics)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(static_mut_refs)]
#![allow(nonstandard_style)]
#![allow(dangerous_implicit_autorefs)]
#![allow(integer_to_ptr_transmutes)]
#![allow(internal_features)]
#![allow(ptr_to_integer_transmute_in_consts)]
#![allow(unused_doc_comments)]
#![allow(function_casts_as_integer)]

pub mod SDK;

#[doc = "MUST use this for SpoofCall Macro"]
use SDK::SpoofCallInternal;

#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllMain(_handle: usize, _ul_reason_for_call: u32, _: *mut ::core::ffi::c_void,) -> i32 {
    if _ul_reason_for_call == 1
    {
        SDK::setup();

        let engine = SDK::UEngine::UEngine::get();

        if engine.is_null()
        {
            return 1;
        }

        let viewport = (*engine).GameViewport;

        if viewport.is_null()
        {
            return 1;
        }

        let vtable_entry_ptr = viewport.vtable_entry(122,);

        PostRender_O = *vtable_entry_ptr;

        let mut protect_addr = vtable_entry_ptr as usize;
        let mut protect_size = 8usize;
        let mut old_prot: u32 = 0;

        let status = NtProtectVirtualMemory(-1isize, &mut protect_addr, &mut protect_size, 0x04, &mut old_prot,);

        if status == 0
        {
            *vtable_entry_ptr = PostRender as usize;

            let mut trash_prot: u32 = 0;
            NtProtectVirtualMemory(-1isize, &mut protect_addr, &mut protect_size, old_prot, &mut trash_prot,);
        }
    }

    1
}

pub static mut PostRender_O: usize = 0;
pub unsafe extern "system" fn PostRender(this: usize, canvas_ptr: *mut SDK::UCanvas::UCanvas,) {
    crate::SpoofCall!(PostRender_O, this, canvas_ptr);

    if canvas_ptr.is_null()
    {
        return;
    }

    let canvas = canvas_ptr.as_ref_unchecked();

    // Should set windows scale
    SDK::Globals::SCREEN_SCALE = 1.25;
    let inv_dpi = 1.0 / SDK::Globals::SCREEN_SCALE;
    let sw = canvas.SizeX as f64 * inv_dpi;
    let sh = canvas.SizeY as f64 * inv_dpi;
    SDK::Globals::SCREEN_SIZE = SDK::Structs::FVector2D::FVector2D { X: sw, Y: sh, };
    SDK::Globals::SCREEN_CENTER = SDK::Globals::SCREEN_SIZE * 0.5;

    if !World::update()
    {
        return;
    }

    for actor_ptr in World::PersistentLevel.as_ref_unchecked().Actors
    {
        if !actor_ptr.is_valid()
        {
            continue;
        }

        let actor = actor_ptr.as_ref_unchecked();
        let actor_loc = actor.K2_GetActorLocation();
        let player_loc = World::Character.as_ref_unchecked().K2_GetActorLocation();

        if SDK::mod_impls::distance_m(actor_loc, player_loc,) > 1000.0
        {
            continue;
        }

        let Some(screen_pos,) = SDK::mod_impls::world_to_screen(World::POV, actor_loc,)
        else
        {
            continue;
        };

        let mut fs = SDK::Structs::FString::FString::tiny_static();
        fs.append_str(&actor.Name.to_string(),);

        canvas.K2_DrawText(
            SDK::UFont::UFont::get_SansationBold18(),
            fs,
            screen_pos,
            SDK::Structs::FVector2D::FVector2D::half(),
            SDK::Structs::FLinearColor::FLinearColor::发光_纯绿,
            1.0,
            SDK::Structs::FLinearColor::FLinearColor::zero(),
            SDK::Structs::FVector2D::FVector2D::zero(),
            true,
            true,
            true,
            SDK::Structs::FLinearColor::FLinearColor::zero(),
        );
    }
}

pub mod World {
    use std::ptr::null_mut;

    use crate::SDK::{
        APawn::APawn, APlayerCameraManager::APlayerCameraManager, APrimalDinoCharacter::APrimalDinoCharacter,
        AShooterCharacter::AShooterCharacter, AShooterGameState::AShooterGameState, AShooterHUD::AShooterHUD,
        AShooterPlayerController::AShooterPlayerController, FMinimalViewInfo::FMinimalViewInfo,
        Structs::TArray::TArray, UGameInstance::UGameInstance, ULevel::ULevel, ULocalPlayer::ULocalPlayer,
        UNetDriver::UNetDriver, UWorld::UWorld,
    };

    pub static mut GWorld: *mut UWorld = null_mut();
    pub static mut PersistentLevel: *mut ULevel = null_mut();
    pub static mut NetDriver: *mut UNetDriver = null_mut();
    pub static mut GameState: *mut AShooterGameState = null_mut();
    pub static mut Levels: TArray<*mut ULevel,> = TArray::zero(); // 假设有new
    pub static mut OwningGameInstance: *mut UGameInstance = null_mut();
    pub static mut LocalPlayer: *mut ULocalPlayer = null_mut();
    pub static mut PlayerController: *mut AShooterPlayerController = null_mut();
    pub static mut AcknowledgedPawn: *mut APawn = null_mut();
    pub static mut Character: *mut AShooterCharacter = null_mut();
    pub static mut DinoCharacter: *mut APrimalDinoCharacter = null_mut();
    pub static mut MyHUD: *mut AShooterHUD = null_mut();
    pub static mut PlayerCameraManager: *mut APlayerCameraManager = null_mut();
    pub static mut POV: FMinimalViewInfo = FMinimalViewInfo::zero();

    #[inline]
    pub unsafe fn update() -> bool {
        let gworld_ptr = crate::SDK::UWorld::UWorld::get();
        if !gworld_ptr.is_valid()
        {
            return false;
        }
        GWorld = gworld_ptr;

        let gworld = GWorld.as_ref_unchecked();

        if !gworld.PersistentLevel.is_valid()
        {
            return false;
        }
        PersistentLevel = gworld.PersistentLevel;

        NetDriver = gworld.NetDriver;

        let game_state = gworld.GameState as *mut AShooterGameState;
        if !game_state.is_valid()
        {
            return false;
        }
        GameState = game_state;

        if !gworld.Levels.is_valid()
        {
            return false;
        }
        Levels = gworld.Levels;

        let game_instance = gworld.OwningGameInstance;
        if !game_instance.is_valid()
        {
            return false;
        }
        OwningGameInstance = game_instance;

        let lp_array = OwningGameInstance.as_ref_unchecked().LocalPlayers;
        if !lp_array.is_valid()
        {
            return false;
        }

        let local_player = *lp_array.Data;
        if !local_player.is_valid()
        {
            return false;
        }
        LocalPlayer = local_player;

        // PlayerController
        let pc = LocalPlayer.as_ref_unchecked().PlayerController as *mut AShooterPlayerController;
        if !pc.is_valid()
        {
            return false;
        }
        PlayerController = pc;

        // Pawn & HUD
        let pawn = PlayerController.as_ref_unchecked().AcknowledgedPawn;
        if !pawn.is_valid()
        {
            return false;
        }
        AcknowledgedPawn = pawn;

        let hud = PlayerController.as_ref_unchecked().MyHUD as *mut AShooterHUD;
        if !hud.is_valid()
        {
            return false;
        }
        MyHUD = hud;

        let character = PlayerController.as_ref_unchecked().GetPlayerCharacter();
        if !character.is_valid()
        {
            return false;
        }
        Character = character;

        DinoCharacter = PlayerController
            .as_ref_unchecked()
            .AcknowledgedPawn
            .as_ref()
            .filter(|p| p.bit_get_IsPrimalDinoCharacter(),)
            .map(|p| p as *const APawn as *mut APrimalDinoCharacter,)
            .filter(|ptr| ptr.is_valid(),)
            .unwrap_or(std::ptr::null_mut(),);

        let camera_manager = PlayerController.as_ref_unchecked().PlayerCameraManager;
        if !camera_manager.is_valid()
        {
            return false;
        }
        PlayerCameraManager = camera_manager;

        POV = PlayerCameraManager.as_ref_unchecked().CameraCachePrivate.POV;

        true
    }
}

#[link(name = "ntdll")]
unsafe extern "C" {
    fn NtProtectVirtualMemory(
        ProcessHandle: isize,
        BaseAddress: *mut usize,
        RegionSize: *mut usize,
        NewProtect: u32,
        OldProtect: *mut u32,
    ) -> u32;
}
