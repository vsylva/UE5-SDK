use super::Structs::{FName::FName, FString::FString};

#[struct_macro::inherit(super::UBlueprintFunctionLibrary::UBlueprintFunctionLibrary, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct UKismetStringLibrary {}

impl UKismetStringLibrary {
    #[inline]
    pub unsafe fn get() -> *mut Self {
        static mut Instance: *mut super::UClass::UClass = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(0, -1, crate::w!("Class '/Script/Engine.KismetStringLibrary'"), false,);
        }

        Instance as *mut Self
    }

    #[inline]
    pub unsafe fn get_FName_OverlayStructurePreview() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("OverlayStructurePreview",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_ColorParam() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("ColorParam",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_MinAlpha() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("MinAlpha",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_MinTrans() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("MinTrans",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_MaxAlpha() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("MaxAlpha",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_Cnt_Head_JNT_SKL() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("Cnt_Head_JNT_SKL",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_c_head() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("c_head",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_HexTilingMult() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("HexTilingMult",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_FresnelExp() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("FresnelExp",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_Opacity() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("Opacity",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_GridInfluence() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("GridInfluence",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_BBContrast() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("BBContrast",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_Hex_Blend_Range() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("Hex Blend Range",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_BaseBrightness() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("BaseBrightness",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_DetailHexTiling() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("DetailHexTiling",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_MicroHexTiling() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("MicroHexTiling",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_LargeHexTiling() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("LargeHexTiling",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_RadiusSizes() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("RadiusSizes",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_SmallAndMediumTiling() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("SmallAndMediumTiling",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_LargeAndXLTiling() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("LargeAndXLTiling",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_DetailHexBlendRange() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("DetailHexBlendRange",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_LargeHexBlendRange() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("LargeHexBlendRange",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_StructureTurretBaseBP_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("StructureTurretBaseBP_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_StructureTurretBaseBP_Heavy_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("StructureTurretBaseBP_Heavy_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_StructureTurretTek_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("StructureTurretTek_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_SupplyCrate_Level15_Double_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("SupplyCrate_Level15_Double_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_SupplyCrate_Level15_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("SupplyCrate_Level15_Double_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_SupplyCrate_Level03_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("SupplyCrate_Level03_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_SupplyCrate_Level25_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("SupplyCrate_Level25_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_SupplyCrate_Level45_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("SupplyCrate_Level45_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_SupplyCrate_Level35_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("SupplyCrate_Level35_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_SupplyCrate_Level60_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("SupplyCrate_Level60_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_SimpleBed_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("SimpleBed_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_BeaverDam_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("BeaverDam_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_DeathItemCache_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("DeathItemCache_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_DeathItemCache_PlayerDeath_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("DeathItemCache_PlayerDeath_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_StorageBox_TekGenerator_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("StorageBox_TekGenerator_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_PrimalStructure_CityTerminal_BP_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("PrimalStructure_CityTerminal_BP_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_ModernBed_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("ModernBed_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_ElectricGenerator_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("ElectricGenerator_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_StorageBox_TekReplicator_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("StorageBox_TekReplicator_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_SupplyCrate_Cave_QualityTier1_EX_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("SupplyCrate_Cave_QualityTier1_EX_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_SupplyCrate_Cave_QualityTier2_EX_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("SupplyCrate_Cave_QualityTier2_EX_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn get_FName_SupplyCrate_Cave_QualityTier3_EX_C() -> FName {
        static mut Instance: FName = FName::zero();

        if ::core::intrinsics::unlikely(Instance.ComparisonIndex == 0,)
        {
            let mut buf = [0u16; 32];
            let mut InString = FString::from_ptr(buf.as_mut_ptr(), 0, 32,);
            InString.append_ascii("SupplyCrate_Cave_QualityTier3_EX_C",);

            Instance = Self::Conv_StringToName(InString,)
        }

        Instance
    }

    #[inline]
    pub unsafe fn Conv_StringToName(InString: FString,) -> FName {
        #[repr(C)]
        struct Params {
            InString:    FString,
            ReturnValue: FName,
        }

        let mut params = Params { InString, ReturnValue: ::core::mem::zeroed(), };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.KismetStringLibrary.Conv_StringToName'"),
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
