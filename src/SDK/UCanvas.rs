use super::{
    Structs::{FLinearColor::FLinearColor, FString::FString, FVector2D::FVector2D},
    UFont::UFont,
    UMaterialInterface::UMaterialInterface,
    UTexture::UTexture,
};

#[struct_macro::inherit(super::UObject::UObject, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct UCanvas {
    #[offset(0x0040)]
    pub SizeX: i32,
    pub SizeY: i32,
}

impl UCanvas {
    #[inline]
    pub unsafe fn K2_DrawLine(
        &self,
        screen_position_a: FVector2D,
        screen_position_b: FVector2D,
        thickness: f32,
        render_color: FLinearColor,
    ) {
        #[repr(C)]
        pub struct Params {
            pub screen_position_a: FVector2D,
            pub screen_position_b: FVector2D,
            pub thickness:         f32,
            pub render_color:      FLinearColor,
            _pad_34:               [u8; 4],
        }

        let mut params = Params {
            screen_position_a: screen_position_a,
            screen_position_b: screen_position_b,
            thickness,
            render_color: render_color,
            _pad_34: [0; 4],
        };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function '/Script/Engine.Canvas.K2_DrawLine'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn K2_DrawText(
        &self,
        render_font: *mut UFont,
        render_text: FString,
        screen_position: FVector2D,
        scale: FVector2D,
        render_color: FLinearColor,
        kerning: f32,
        shadow_color: FLinearColor,
        shadow_offset: FVector2D,
        b_centre_x: bool,
        b_centre_y: bool,
        b_outlined: bool,
        outline_color: FLinearColor,
    ) {
        #[repr(C)]
        pub struct Params {
            pub render_font:     *mut UFont,
            pub render_text:     FString,
            pub screen_position: FVector2D,
            pub scale:           FVector2D,
            pub render_color:    FLinearColor,
            pub kerning:         f32,
            pub shadow_color:    FLinearColor,
            pub shadow_offset:   FVector2D,
            pub b_centre_x:      bool,
            pub b_centre_y:      bool,
            pub b_outlined:      bool,
            pub outline_color:   FLinearColor,
        }

        let mut params = Params {
            render_font,
            render_text,
            screen_position,
            scale,
            render_color,
            kerning,
            shadow_color,
            shadow_offset,
            b_centre_x,
            b_centre_y,
            b_outlined,
            outline_color,
        };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function '/Script/Engine.Canvas.K2_DrawText'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn K2_TextSize(&self, render_font: *mut UFont, render_text: FString, scale: FVector2D,) -> FVector2D {
        #[repr(C)]
        pub struct Params {
            pub render_font: *mut UFont,
            pub render_text: FString,
            pub scale:       FVector2D,
            pub ReturnValue: FVector2D,
        }

        let mut params = Params { render_font, render_text, scale, ReturnValue: FVector2D::zero(), };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function '/Script/Engine.Canvas.K2_TextSize'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.ReturnValue
    }

    #[inline]
    pub unsafe fn SetDrawColor(&self, C: super::Structs::FColor::FColor,) {
        #[repr(C)]
        pub struct Params {
            pub C: super::Structs::FColor::FColor,
        }

        let mut params = Params { C, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function '/Script/Engine.Canvas.SetDrawColor'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn K2_DrawBox(
        &self,
        screen_position: FVector2D,
        ScreenSize: FVector2D,
        thickness: f32,
        render_color: FLinearColor,
    ) {
        #[repr(C)]
        pub struct Params {
            pub screen_position: FVector2D,
            pub ScreenSize:      FVector2D,
            thickness:           f32,
            pub render_color:    FLinearColor,
            Pad_34:              [u8; 4],
        }

        let mut params = Params { screen_position, ScreenSize, thickness, render_color, Pad_34: [0; 4], };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(0, -1, crate::w!("Function '/Script/Engine.Canvas.K2_DrawBox'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn K2_DrawPolygon(
        &self,
        RenderTexture: *mut usize,
        ScreenPosition: FVector2D,
        Radius: FVector2D,
        NumberOfSides: i32,
        RenderColor: FLinearColor,
    ) {
        #[repr(C)]
        pub struct Params {
            RenderTexture:  *mut usize,
            ScreenPosition: FVector2D,
            Radius:         FVector2D,
            NumberOfSides:  i32,
            RenderColor:    FLinearColor,
            Pad_34:         [u8; 4],
        }

        let mut params = Params { RenderTexture, ScreenPosition, Radius, NumberOfSides, RenderColor, Pad_34: [0; 4], };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function '/Script/Engine.Canvas.K2_DrawPolygon'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn K2_DrawTexture(
        &self,
        RenderTexture: *mut UTexture,
        ScreenPosition: FVector2D,
        ScreenSize: FVector2D,
        CoordinatePosition: FVector2D,
        CoordinateSize: FVector2D,
        RenderColor: FLinearColor,
        BlendMode: EBlendMode,
        Rotation: f32,
        PivotPoint: FVector2D,
    ) {
        #[repr(C)]
        pub struct Params {
            RenderTexture:      *mut UTexture,
            ScreenPosition:     FVector2D,
            ScreenSize:         FVector2D,
            CoordinatePosition: FVector2D,
            CoordinateSize:     FVector2D,
            RenderColor:        FLinearColor,
            BlendMode:          EBlendMode,
            Pad_59:             [u8; 0x3],
            Rotation:           f32,
            PivotPoint:         FVector2D,
        }

        let mut params = Params {
            RenderTexture,
            ScreenPosition,
            ScreenSize,
            CoordinatePosition,
            CoordinateSize,
            RenderColor,
            BlendMode,
            Pad_59: [0u8; 0x3],
            Rotation,
            PivotPoint,
        };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function '/Script/Engine.Canvas.K2_DrawTexture'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn K2_DrawBorder(
        &self,
        BorderTexture: *mut usize,
        BackgroundTexture: *mut usize,
        LeftBorderTexture: *mut usize,
        RightBorderTexture: *mut usize,
        TopBorderTexture: *mut usize,
        BottomBorderTexture: *mut usize,
        ScreenPosition: FVector2D,
        ScreenSize: FVector2D,
        CoordinatePosition: FVector2D,
        CoordinateSize: FVector2D,
        RenderColor: FLinearColor,
        BorderScale: FVector2D,
        BackgroundScale: FVector2D,
        Rotation: f32,
        PivotPoint: FVector2D,
        CornerSize: FVector2D,
    ) {
        #[repr(C)]
        pub struct Params {
            BorderTexture:      *mut usize,
            BackgroundTexture:  *mut usize,
            LeftBorderTexture:  *mut usize,
            RightBorderTexture: *mut usize,

            TopBorderTexture:    *mut usize,
            BottomBorderTexture: *mut usize,

            ScreenPosition:     FVector2D,
            ScreenSize:         FVector2D,
            CoordinatePosition: FVector2D,
            CoordinateSize:     FVector2D,

            RenderColor: FLinearColor,

            BorderScale:     FVector2D,
            BackgroundScale: FVector2D,

            Rotation: f32,
            Pad_A4:   [u8; 0x4],

            PivotPoint: FVector2D,
            CornerSize: FVector2D,
        }

        let mut params = Params {
            BorderTexture,
            BackgroundTexture,
            LeftBorderTexture,
            RightBorderTexture,
            TopBorderTexture,
            BottomBorderTexture,
            ScreenPosition,
            ScreenSize,
            CoordinatePosition,
            CoordinateSize,
            RenderColor,
            BorderScale,
            BackgroundScale,
            Rotation,
            Pad_A4: [0u8; 0x4],
            PivotPoint,
            CornerSize,
        };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function '/Script/Engine.Canvas.K2_DrawBorder'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn K2_DrawMaterial(
        &self,
        RenderMaterial: *mut UMaterialInterface,
        ScreenPosition: FVector2D,
        ScreenSize: FVector2D,
        CoordinatePosition: FVector2D,
        CoordinateSize: FVector2D,
        Rotation: f32,
        PivotPoint: FVector2D,
        bApplyGammaCorrection: bool,
    ) {
        #[repr(C)]
        pub struct Params {
            RenderMaterial: *mut UMaterialInterface,

            ScreenPosition:     FVector2D,
            ScreenSize:         FVector2D,
            CoordinatePosition: FVector2D,
            CoordinateSize:     FVector2D,

            Rotation: f32,
            Pad_4C:   [u8; 0x4],

            PivotPoint:            FVector2D,
            bApplyGammaCorrection: bool,
            Pad_61:                [u8; 0x7],
        }

        let mut params = Params {
            RenderMaterial,
            ScreenPosition,
            ScreenSize,
            CoordinatePosition,
            CoordinateSize,
            Rotation,
            Pad_4C: [0u8; 0x4],
            PivotPoint,
            bApplyGammaCorrection,
            Pad_61: [0u8; 0x7],
        };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function '/Script/Engine.Canvas.K2_DrawMaterial'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }
}

#[repr(u8)]
pub enum EBlendMode {
    BLEND_Opaque = 0,
    BLEND_Masked = 1,
    BLEND_Translucent = 2,
    BLEND_Additive = 3,
    BLEND_Modulate = 4,
    BLEND_AlphaComposite = 5,
    BLEND_AlphaHoldout = 6,
    BLEND_TranslucentColoredTransmittance = 7,
    BLEND_MAX = 8,
}
