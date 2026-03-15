use super::super::{
    Structs::{FName::FName, FString::FString},
    UKismetStringLibrary::UKismetStringLibrary,
};

#[repr(C)]
#[derive(Debug, Copy, Clone, Default,)]
pub struct FKey {
    pub KeyName: super::FName::FName,
    Pad_8:       [u8; 0x10],
}

impl FKey {
    #[inline]
    pub const fn zero() -> Self {
        unsafe { ::core::mem::zeroed() }
    }

    #[inline]
    pub unsafe fn get_End() -> Self {
        pub static mut Instance: FName = FName::zero();
        static mut Key: FKey = FKey::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("End",);

            Instance = UKismetStringLibrary::Conv_StringToName(InString,);

            Key.KeyName = Instance
        }

        Key
    }

    #[inline]
    pub unsafe fn get_LeftMouseButton() -> Self {
        pub static mut Instance: FName = FName::zero();
        static mut Key: FKey = FKey::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("LeftMouseButton",);

            Instance = UKismetStringLibrary::Conv_StringToName(InString,);

            Key.KeyName = Instance
        }

        Key
    }

    #[inline]
    pub unsafe fn get_RightMouseButton() -> Self {
        pub static mut Instance: FName = FName::zero();
        static mut Key: FKey = FKey::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("RightMouseButton",);

            Instance = UKismetStringLibrary::Conv_StringToName(InString,);

            Key.KeyName = Instance
        }

        Key
    }

    #[inline]
    pub unsafe fn get_H() -> Self {
        pub static mut Instance: FName = FName::zero();
        static mut Key: FKey = FKey::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("H",);

            Instance = UKismetStringLibrary::Conv_StringToName(InString,);

            Key.KeyName = Instance
        }

        Key
    }

    #[inline]
    pub unsafe fn get_G() -> Self {
        pub static mut Instance: FName = FName::zero();
        static mut Key: FKey = FKey::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("G",);

            Instance = UKismetStringLibrary::Conv_StringToName(InString,);

            Key.KeyName = Instance
        }

        Key
    }

    #[inline]
    pub unsafe fn get_F10() -> Self {
        pub static mut Instance: FName = FName::zero();
        static mut Key: FKey = FKey::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("F10",);

            Instance = UKismetStringLibrary::Conv_StringToName(InString,);

            Key.KeyName = Instance
        }

        Key
    }

    #[inline]
    pub unsafe fn get_F8() -> Self {
        pub static mut Instance: FName = FName::zero();
        static mut Key: FKey = FKey::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("F8",);

            Instance = UKismetStringLibrary::Conv_StringToName(InString,);

            Key.KeyName = Instance
        }

        Key
    }

    #[inline]
    pub unsafe fn get_V() -> Self {
        pub static mut Instance: FName = FName::zero();
        static mut Key: FKey = FKey::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("V",);

            Instance = UKismetStringLibrary::Conv_StringToName(InString,);

            Key.KeyName = Instance
        }

        Key
    }
}
