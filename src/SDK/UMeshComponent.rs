use super::UMaterialInterface::UMaterialInterface;

#[struct_macro::inherit(super::UPrimitiveComponent::UPrimitiveComponent, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct UMeshComponent {
    #[offset(0x0618)]
    pub OverlayMaterial:                *mut UMaterialInterface,
    pub OverlayMaterialMaxDrawDistance: f32,
}

impl UMeshComponent {
    #[inline]
    pub unsafe fn SetOverlayMaterial(&self, NewOverlayMaterial: *mut UMaterialInterface,) {
        #[repr(C)]
        struct Params {
            NewOverlayMaterial: *mut UMaterialInterface,
        }

        let mut params = Params { NewOverlayMaterial, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.MeshComponent.SetOverlayMaterial'"),
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
    pub unsafe fn GetOverlayMaterial(&self,) -> *mut UMaterialInterface {
        #[repr(C)]
        struct Params {
            pub ReturnValue: *mut UMaterialInterface,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.MeshComponent.GetOverlayMaterial'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.ReturnValue
    }
}
