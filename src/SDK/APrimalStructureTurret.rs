#[struct_macro::inherit(super::APrimalStructureItemContainer::APrimalStructureItemContainer, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct APrimalStructureTurret {
    #[offset(0x12E0)]
    pub AISetting: TurretTargetingMode,

    #[offset(0x1300)]
    pub NumBullets: i32,
}

impl APrimalStructureTurret {
    #[inline]
    pub unsafe fn StaticClass() -> *mut super::UClass::UClass {
        pub static mut Instance: *mut super::UClass::UClass = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                "Instance '/Script/ShooterGame.PrimalStructureTurret'"
                    .encode_utf16()
                    .chain(::core::iter::once(0,),)
                    .collect::<Vec<u16,>>()
                    .as_ptr(),
                false,
            );

            #[cfg(debug_assertions)]
            dbg!(Instance);
        }

        Instance
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq,)]
pub enum TurretTargetingMode {
    AllTargets = 0,
    PlayersAndTamed = 1,
    OnlyPlayers = 2,
    OnlyWild = 3,
    OnlyTamed = 4,
    OnlyPlayersAndMountedDinos = 5,
    Unknown = 255,
}

impl From<u8,> for TurretTargetingMode {
    fn from(value: u8,) -> Self {
        match value
        {
            0 => Self::AllTargets,
            1 => Self::PlayersAndTamed,
            2 => Self::OnlyPlayers,
            3 => Self::OnlyWild,
            4 => Self::OnlyTamed,
            5 => Self::OnlyPlayersAndMountedDinos,
            _ => Self::Unknown,
        }
    }
}

impl TurretTargetingMode {
    #[inline]
    pub const fn to_string(&self,) -> &'static str {
        match self
        {
            Self::AllTargets => "全",

            Self::PlayersAndTamed => "人|驯",

            Self::OnlyPlayers => "人",

            Self::OnlyWild => "野",

            Self::OnlyTamed => "驯",

            Self::OnlyPlayersAndMountedDinos => "人|骑",
            Self::Unknown => "?",
        }
    }
}
