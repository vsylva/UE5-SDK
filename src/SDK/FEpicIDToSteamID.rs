use super::Structs::FString::FString;

#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct FEpicIDToSteamID {
    pub Name:    FString,
    pub SteamID: FString,
    pub EpicID:  FString,
}
