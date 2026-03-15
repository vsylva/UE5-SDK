use super::Structs::FName::FName;

#[repr(C)]
pub struct FFieldClass {
    pub Name:       FName,
    pub Id:         u64,
    pub CastFlags:  u64,
    pub ClassFlags: EClassFlags,
    Pad_1C:         [u8; 0x4],
    pub SuperClass: *mut FFieldClass,
}

#[repr(u32)]
pub enum EClassFlags {
    CLASS_None = 0x00000000,
    Abstract = 0x00000001,
    DefaultConfig = 0x00000002,
    Config = 0x00000004,
    Transient = 0x00000008,
    Parsed = 0x00000010,
    MatchedSerializers = 0x00000020,
    ProjectUserConfig = 0x00000040,
    Native = 0x00000080,
    NoExport = 0x00000100,
    NotPlaceable = 0x00000200,
    PerObjectConfig = 0x00000400,
    ReplicationDataIsSetUp = 0x00000800,
    EditInlineNew = 0x00001000,
    CollapseCategories = 0x00002000,
    Interface = 0x00004000,
    CustomConstructor = 0x00008000,
    Const = 0x00010000,
    LayoutChanging = 0x00020000,
    CompiledFromBlueprint = 0x00040000,
    MinimalAPI = 0x00080000,
    RequiredAPI = 0x00100000,
    DefaultToInstanced = 0x00200000,
    TokenStreamAssembled = 0x00400000,
    HasInstancedReference = 0x00800000,
    Hidden = 0x01000000,
    Deprecated = 0x02000000,
    HideDropDown = 0x04000000,
    GlobalUserConfig = 0x08000000,
    Intrinsic = 0x10000000,
    Constructed = 0x20000000,
    ConfigDoNotCheckDefaults = 0x40000000,
    NewerVersionExists = 0x80000000,
}
