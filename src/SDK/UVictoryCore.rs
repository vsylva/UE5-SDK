use super::{
    AActor::AActor,
    FOverlappedFoliageElement::FOverlappedFoliageElement,
    Structs::{FVector::FVector, TArray::TArray, TSubclassOf::TSubclassOf},
    UActorComponent::UActorComponent,
    UPrimalItem::UPrimalItem,
    UWorld::UWorld,
};

#[struct_macro::inherit(super::UBlueprintFunctionLibrary::UBlueprintFunctionLibrary, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct UVictoryCore {}

impl UVictoryCore {
    #[inline]
    pub unsafe fn get() -> *mut Self {
        static mut Instance: *mut super::UClass::UClass = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(0, -1, crate::w!("Class '/Script/ShooterGame.VictoryCore'"), false,);
        }

        Instance as *mut Self
    }

    #[inline]
    pub unsafe fn ServerSearchFoliageByResource(
        WorldContextObject: *mut UWorld,
        Origin: FVector,
        Radius: f32,
        OutFoliage: *mut TArray<FOverlappedFoliageElement,>,
        HarvestResources: TArray<TSubclassOf<UPrimalItem,>,>,
        bVisibleAndActiveOnly: bool,
        bIncludeUsableFoliage: bool,
        bIncludeMeshFoliage: bool,
        bSortByDistance: bool,
        bReverseSort: bool,
    ) {
        #[repr(C)]
        struct Params {
            WorldContextObject:    *mut UWorld,
            Origin:                FVector,
            Radius:                f32,
            Pad_24:                [u8; 0x4],
            OutFoliage:            TArray<FOverlappedFoliageElement,>,
            HarvestResources:      TArray<TSubclassOf<UPrimalItem,>,>,
            bVisibleAndActiveOnly: bool,
            bIncludeUsableFoliage: bool,
            bIncludeMeshFoliage:   bool,
            bSortByDistance:       bool,
            bReverseSort:          bool,
            Pad_4D:                [u8; 0x3],
        }

        let mut params = Params {
            WorldContextObject,
            Origin,
            Radius,
            HarvestResources,
            bVisibleAndActiveOnly,
            bIncludeUsableFoliage,
            bIncludeMeshFoliage,
            bSortByDistance,
            bReverseSort,
            ..::core::mem::zeroed()
        };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.VictoryCore.ServerSearchFoliageByResource'"),
                false,
            );
        }

        super::ProcessEvent(
            Self::get() as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        if !OutFoliage.is_null()
        {
            *OutFoliage = params.OutFoliage;
        }
    }

    #[inline]
    pub unsafe fn ServerSearchFoliage(
        WorldContextObject: *mut UWorld,
        Origin: FVector,
        Radius: f32,
        OutFoliage: *mut TArray<FOverlappedFoliageElement,>,
        bVisibleAndActiveOnly: bool,
        bIncludeUsableFoliage: bool,
        bIncludeMeshFoliage: bool,
        bSortByDistance: bool,
        bReverseSort: bool,
    ) {
        #[repr(C)]
        struct Params {
            WorldContextObject:    *mut UWorld,
            Origin:                FVector,
            Radius:                f32,
            Pad_24:                [u8; 0x4],
            OutFoliage:            TArray<FOverlappedFoliageElement,>,
            bVisibleAndActiveOnly: bool,
            bIncludeUsableFoliage: bool,
            bIncludeMeshFoliage:   bool,
            bSortByDistance:       bool,
            bReverseSort:          bool,
            Pad_4D:                [u8; 0x3],
        }

        let mut params = Params {
            WorldContextObject,
            Origin,
            Radius,
            bVisibleAndActiveOnly,
            bIncludeUsableFoliage,
            bIncludeMeshFoliage,
            bSortByDistance,
            bReverseSort,
            ..::core::mem::zeroed()
        };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.VictoryCore.ServerSearchFoliage'"),
                false,
            );
        }

        super::ProcessEvent(
            Self::get() as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        if !OutFoliage.is_null()
        {
            *OutFoliage = params.OutFoliage;
        }
    }

    #[allow(unused)]
    #[inline]
    pub unsafe fn GetOverlappedHarvestActors(
        WorldContextObject: *mut UWorld,
        AtLoc: FVector,
        AtRadius: f32,
        OutHarvestActors: *mut TArray<*mut AActor,>,
        OutHarvestComponents: *mut TArray<*mut UActorComponent,>,
        OutHarvestLocations: *mut TArray<FVector,>,
        OutHitBodyIndices: *mut TArray<i32,>,

        Origin: FVector,
        Radius: f32,
        OutFoliage: *mut TArray<FOverlappedFoliageElement,>,

        bVisibleAndActiveOnly: bool,
        bIncludeUsableFoliage: bool,
        bIncludeMeshFoliage: bool,
        bSortByDistance: bool,
        bReverseSort: bool,
    ) {
        #[repr(C)]
        struct Params {
            WorldContextObject:    *mut UWorld,
            Origin:                FVector,
            Radius:                f32,
            Pad_24:                [u8; 0x4],
            OutFoliage:            TArray<FOverlappedFoliageElement,>,
            bVisibleAndActiveOnly: bool,
            bIncludeUsableFoliage: bool,
            bIncludeMeshFoliage:   bool,
            bSortByDistance:       bool,
            bReverseSort:          bool,
            Pad_4D:                [u8; 0x3],
        }

        let mut params = Params {
            WorldContextObject,
            Origin,
            Radius,
            bVisibleAndActiveOnly,
            bIncludeUsableFoliage,
            bIncludeMeshFoliage,
            bSortByDistance,
            bReverseSort,
            ..::core::mem::zeroed()
        };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.VictoryCore.ServerSearchFoliage'"),
                false,
            );
        }

        super::ProcessEvent(
            Self::get() as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        if !OutFoliage.is_null()
        {
            *OutFoliage = params.OutFoliage;
        }
    }

    #[inline]
    pub unsafe fn GetSpecialActorList(
        WorldContextObject: *mut UWorld,
        ActorListType: EActorListsBP,
        OutActors: *mut TArray<*mut AActor,>,
    ) {
        #[repr(C)]
        struct Params {
            WorldContextObject: *mut UWorld,
            ActorListType:      EActorListsBP,
            Pad_9:              [u8; 0x7],
            OutActors:          TArray<*mut AActor,>,
        }

        let mut params = Params { WorldContextObject, ActorListType, Pad_9: [0u8; 0x7], OutActors: TArray::zero(), };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.VictoryCore.GetSpecialActorList'"),
                false,
            );
        }

        super::ProcessEvent(
            Self::get() as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        if !OutActors.is_null()
        {
            *OutActors = params.OutActors;
        }
    }

    #[inline]
    pub unsafe fn FindProjectileDirectionToLeadTarget(
        WorldContextObject: *mut UWorld,
        ProjectileStartLoc: FVector,
        TargetLoc: FVector,
        TargetVeloctity: FVector,
        ProjectileSpeed: f32,
        ProjectileGravityScale: f32,
        extraTimeAllowed: f32,
        Accuracy: f32,
    ) -> FVector {
        #[repr(C)]
        struct Params {
            WorldContextObject:     *mut UWorld,
            ProjectileStartLoc:     FVector,
            TargetLoc:              FVector,
            TargetVeloctity:        FVector,
            ProjectileSpeed:        f32,
            ProjectileGravityScale: f32,
            extraTimeAllowed:       f32,
            Accuracy:               f32,
            ReturnValue:            FVector,
        }

        let mut params = Params {
            WorldContextObject,
            ProjectileStartLoc,
            TargetLoc,
            TargetVeloctity,
            ProjectileSpeed,
            ProjectileGravityScale,
            extraTimeAllowed,
            Accuracy,
            ReturnValue: FVector::zero(),
        };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.VictoryCore.FindProjectileDirectionToLeadTarget'"),
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
}

#[repr(u8)]
pub enum EActorListsBP {
    AL_PLAYERS = 0,
    AL_PLAYERSTATES = 1,
    AL_FLOATINGHUD = 2,
    AL_UNSTASISEDACTORS = 3,
    AL_NPC = 4,
    AL_NPC_ACTIVE = 5,
    AL_FORCEDHUD = 6,
    AL_NPCZONEMANAGERS = 7,
    AL_PLAYERCONTROLLERS = 8,
    AL_BEDS = 9,
    AL_BIOMEZONEVOLUMES = 10,
    AL_NPC_DEAD = 11,
    AL_DAYCYCLEAMBIENTSOUNDS = 12,
    AL_STRUCTURESCLIENT = 13,
    AL_STRUCTUREPREVENTIONVOLUMES = 14,
    AL_TRANSPONDERS = 15,
    AL_CUSTOMACTORLISTS = 16,
    AL_BLOCKINGVOLUMES = 17,
    AL_AMBIENTSOUNDS = 18,
    AL_CONNECTEDSHOOTERCHARACTERS = 19,
    AL_EXPLORERNOTECHESTS = 20,
    AL_SUPPLYCRATEVOLUMES = 21,
    AL_TAMED_DINOS = 22,
    AL_NPCZONEMANAGERS_LEVELSTREAM = 23,
    AL_TILEVOLUMES = 24,
    AL_DIRECTIONAL_LIGHTS = 25,
    AL_MISSIONS = 26,
    AL_MISSION_DISPATCHERS = 27,
    AL_THROTTLEDTICKS = 28,
    AL_PLAYER_STARTS = 29,
    AL_MISSION_DISPATCHER_POINTS = 30,
    AL_UNDERMESH_VOLUMES = 31,
    AL_PERFORMANCETHROTTLEDTICKS = 32,
    AL_TAGGED = 33,
    AL_CLIENTSIDEDESTRUCTABLEACTORS = 34,
    AL_STRUCTURESTHATSHOULDINTERACTWITHVOLUMETRICDISPATCHERS = 35,
    AL_REALTIMETHROTTLEDTICKS = 36,
    AL_TELEPORTERS = 37,
    AL_ACTIVE_WILD_NONWATER_DINOS = 38,
    AL_ACTIVE_WILD_WATER_DINOS = 39,
    AL_POSSESSED_THRALLS = 40,
    MAX = 41,
}
