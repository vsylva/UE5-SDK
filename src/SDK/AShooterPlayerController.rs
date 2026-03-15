use super::{
    AActor::AActor, FItemNetID::FItemNetID, Structs::FString::FString,
    UPrimalInventoryComponent::UPrimalInventoryComponent,
};

#[struct_macro::inherit(super::ABasePlayerController::ABasePlayerController, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct AShooterPlayerController {
    #[offset(0x1048)]
    pub MaxUseDistance: f32,

    #[offset(0x1220)]
    pub PhotoModeMarkerActor: *mut AActor,

    #[offset(0x1619)]
    pub bPreventPaintingStreaming: bool,
    Pad_160A: [u8; 0x5E],
    pub CurrentTribeLog: super::Structs::TArray::TArray<super::Structs::FString::FString,>,
}

impl AShooterPlayerController {
    #[inline]
    pub unsafe fn GetPlayerCharacter(&self,) -> *mut super::AShooterCharacter::AShooterCharacter {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/ShooterGame.ShooterPlayerController.GetPlayerCharacter'"),
                false,
            );
        }

        #[repr(C)]
        struct Param {
            ReturnValue: *mut super::AShooterCharacter::AShooterCharacter,
        }

        let mut params = Param { ..::core::mem::zeroed() };

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.ReturnValue
    }

    #[inline]
    pub unsafe fn ServerRequestTribeLog(&self,) {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/ShooterGame.ShooterPlayerController.ServerRequestTribeLog'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            0 as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn ServerSendChatMessage(&self, ChatMessage: FString, SendMode: EChatSendMode, senderPlatform: i32,) {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/ShooterGame.ShooterPlayerController.ServerSendChatMessage'"),
                false,
            );
        }

        #[repr(C)]
        struct Param {
            ChatMessage:    FString,
            SendMode:       EChatSendMode,
            Pad_11:         [u8; 0x3],
            senderPlatform: i32,
        }

        let mut params = Param { ChatMessage, SendMode, Pad_11: [0u8; 0x3], senderPlatform, };

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn UnlockExplorerNote_V2(
        &self,
        ExplorerNoteIndex: i32,
        forceShowExplorerNoteUI: bool,
        SavePlayerData: bool,
        actuallyForceShowExplorerNoteEvenWhenAlreadyUnlockedLocally: bool,
    ) {
        #[repr(C)]
        struct Param {
            ExplorerNoteIndex: i32,
            forceShowExplorerNoteUI: bool,
            SavePlayerData: bool,
            actuallyForceShowExplorerNoteEvenWhenAlreadyUnlockedLocally: bool,
            Pad_7: [u8; 0x1],
        }

        let mut params = Param {
            ExplorerNoteIndex,
            forceShowExplorerNoteUI,
            SavePlayerData,
            actuallyForceShowExplorerNoteEvenWhenAlreadyUnlockedLocally,
            Pad_7: ::core::mem::zeroed(),
        };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/ShooterGame.ShooterPlayerController.UnlockExplorerNote_V2'"),
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
    pub unsafe fn ClientRunLocalConsoleCommand(&self, Command: FString, bWriteToLog: bool,) {
        #[repr(C)]
        struct Params {
            Command:     FString,
            bWriteToLog: bool,
            Pad_11:      [u8; 0x7],
        }

        let mut params = Params { Command, bWriteToLog, Pad_11: [0u8; 0x7], };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.ShooterPlayerController.ClientRunLocalConsoleCommand'"),
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
    pub unsafe fn ServerEquipPawnItem(&self, ItemID: FItemNetID,) {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.ShooterPlayerController.ServerEquipPawnItem'"),
                false,
            );
        }

        #[repr(C)]
        struct Param {
            ItemID: FItemNetID,
        }

        let mut params = Param { ItemID, };

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn ServerRequestActorItems(
        &self,
        ForInventory: *mut UPrimalInventoryComponent,
        bInventoryItems: bool,
        bWithFirstSpawn: bool,
    ) {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/ShooterGame.ShooterPlayerController.ServerRequestActorItems'"),
                false,
            );
        }

        #[repr(C)]
        struct Param {
            ForInventory:    *mut UPrimalInventoryComponent,
            bInventoryItems: bool,
            bWithFirstSpawn: bool,
            Pad_A:           [u8; 0x6],
        }

        let mut params = Param { ForInventory, bInventoryItems, bWithFirstSpawn, Pad_A: [0u8; 0x6], };

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn ServerTransferFromRemoteInventory(
        &self,
        ForInventory: *mut UPrimalInventoryComponent,
        ItemID: FItemNetID,
        requestedQuantity: i32,
        ToSlotIndex: i32,
        bEquipItem: bool,
    ) {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.ShooterPlayerController.ServerTransferFromRemoteInventory'"),
                false,
            );
        }

        #[repr(C)]
        struct Param {
            ForInventory:      *mut UPrimalInventoryComponent,
            ItemID:            FItemNetID,
            requestedQuantity: i32,
            ToSlotIndex:       i32,
            bEquipItem:        bool,
            Pad_19:            [u8; 0x7],
        }

        let mut params =
            Param { ForInventory, ItemID, requestedQuantity, ToSlotIndex, bEquipItem, Pad_19: [0u8; 0x7], };

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn ServerEquipToRemoteInventory(
        &self,
        ForInventory: *mut UPrimalInventoryComponent,
        ItemID: FItemNetID,
    ) {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.ShooterPlayerController.ServerEquipToRemoteInventory'"),
                false,
            );
        }

        #[repr(C)]
        struct Param {
            ForInventory: *mut UPrimalInventoryComponent,
            ItemID:       FItemNetID,
        }

        let mut params = Param { ForInventory, ItemID, };

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn ServerTransferAllFromRemoteInventory(
        &self,
        InventoryComp: *mut UPrimalInventoryComponent,
        CurrentCustomFolderFilter: FString,
        CurrentNameFilter: FString,
        CurrentDestinationFolder: FString,
        bNoFolderView: bool,
    ) {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.ShooterPlayerController.ServerTransferAllFromRemoteInventory'"),
                false,
            );
        }

        #[repr(C)]
        struct Param {
            InventoryComp:             *mut UPrimalInventoryComponent,
            CurrentCustomFolderFilter: FString,
            CurrentNameFilter:         FString,
            CurrentDestinationFolder:  FString,
            bNoFolderView:             bool,
            Pad_39:                    [u8; 0x7],
        }

        let mut params = Param {
            InventoryComp,
            CurrentCustomFolderFilter,
            CurrentNameFilter,
            CurrentDestinationFolder,
            bNoFolderView,
            Pad_39: [0u8; 0x7],
        };

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn ServerDropFromRemoteInventory(
        &self,
        ForInventory: *mut UPrimalInventoryComponent,
        ItemID: FItemNetID,
    ) {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.ShooterPlayerController.ServerDropFromRemoteInventory'"),
                false,
            );
        }

        #[repr(C)]
        struct Param {
            ForInventory: *mut UPrimalInventoryComponent,
            ItemID:       FItemNetID,
        }

        let mut params = Param { ForInventory, ItemID, };

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn ServerUnlockPerMapExplorerNote(
        &self,
        ExplorerNoteIndex: i32,
        bAvoidBuff: bool,
        SavePlayerData: bool,
    ) {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.ShooterPlayerController.ServerUnlockPerMapExplorerNote'"),
                false,
            );
        }

        #[repr(C)]
        struct Param {
            ExplorerNoteIndex: i32,
            bAvoidBuff:        bool,
            SavePlayerData:    bool,

            Pad_6: [u8; 0x2],
        }

        let mut params = Param { ExplorerNoteIndex, bAvoidBuff, SavePlayerData, Pad_6: [0u8; 0x2], };

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }
}

#[repr(u8)]
pub enum EChatSendMode {
    GlobalChat = 0,
    GlobalTribeChat = 1,
    LocalChat = 2,
    AllianceChat = 3,
    MAX = 4,
}
