#[struct_macro::bitfields]
#[struct_macro::inherit(super::UMaterialInterface::UMaterialInterface, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct UMaterial {
    #[offset(0x0178)]
    pub ShadingModel: EMaterialShadingModel,

    #[offset(0x01BC)]
    #[bits(1, bDisableDepthTest)]
    pub bDisableDepthTest: bool,

    #[offset(0x01CC)]
    #[bits(7, Wireframe)]
    _pad_bit_0:    bool,
    _pad_bit_1:    bool,
    _pad_bit_2:    bool,
    _pad_bit_3:    bool,
    _pad_bit_4:    bool,
    _pad_bit_5:    bool,
    pub Wireframe: bool,
}

impl UMaterial {
    #[inline]
    pub unsafe fn get_StructurePreviewMat() -> &'static mut Self {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Material 'StructurePreviewMat.StructurePreviewMat'"), false,);
        }

        ::core::mem::transmute(Instance,)
    }
}

#[derive(Debug, Clone, Copy,)]
#[repr(u8)]
pub enum EMaterialShadingModel {
    MSM_Unlit = 0,
    MSM_DefaultLit = 1,
    MSM_Subsurface = 2,
    MSM_PreintegratedSkin = 3,
    MSM_ClearCoat = 4,
    MSM_SubsurfaceProfile = 5,
    MSM_TwoSidedFoliage = 6,
    MSM_Hair = 7,
    MSM_Cloth = 8,
    MSM_Eye = 9,
    MSM_SingleLayerWater = 10,
    MSM_ThinTranslucent = 11,
    MSM_Strata = 12,
    MSM_Toon = 13,
    MSM_LitReactive = 14,
    MSM_NUM = 15,
    MSM_FromMaterialExpression = 16,
    MSM_MAX = 17,
}
