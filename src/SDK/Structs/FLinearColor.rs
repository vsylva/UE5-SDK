#[repr(C)]
#[derive(Debug, Clone, Copy,)]
pub struct FLinearColor {
    pub R: f32,
    pub G: f32,
    pub B: f32,
    pub A: f32,
}

impl FLinearColor {
    pub const 纯粉: Self = Self::from_rgba(1.0, 0.4, 0.7, 1.0,);
    pub const 浅蓝: Self = Self::from_rgba(0.6, 0.8, 1.0, 1.0,);
    pub const 纯红: Self = Self::from_rgba(1.0, 0.0, 0.0, 1.0,);
    pub const 浅红色: Self = Self::from_rgba(1.0, 0.4, 0.4, 1.0,);
    pub const 纯绿: Self = Self::from_rgba(0.0, 1.0, 0.0, 1.0,);
    pub const 纯蓝: Self = Self::from_rgba(0.0, 0.0, 1.0, 1.0,);
    pub const 纯黄: Self = Self::from_rgba(1.0, 1.0, 0.0, 1.0,);
    pub const 纯紫: Self = Self::from_rgba(1.0, 0.0, 1.0, 1.0,);
    pub const 纯青: Self = Self::from_rgba(0.0, 1.0, 1.0, 1.0,);
    pub const 纯橙: Self = Self::from_rgba(1.0, 0.5, 0.0, 1.0,);
    pub const 纯白: Self = Self::from_rgba(1.0, 1.0, 1.0, 1.0,);
    pub const 深褐色: Self = Self::from_rgba(0.2, 0.1, 0.05, 1.0,);

    pub const 荧光绿: Self = Self::from_rgba(0.5, 1.0, 0.0, 1.0,);
    pub const 电光蓝: Self = Self::from_rgba(0.0, 0.5, 1.0, 1.0,);
    pub const 霓虹粉: Self = Self::from_rgba(1.0, 0.0, 0.5, 1.0,);
    pub const 柠檬黄: Self = Self::from_rgba(0.8, 1.0, 0.0, 1.0,);
    pub const 冰晶蓝: Self = Self::from_rgba(0.0, 1.0, 0.8, 1.0,);
    pub const 亮紫色: Self = Self::from_rgba(0.6, 0.2, 1.0, 1.0,);

    pub const 传说金: Self = Self::from_rgba(1.0, 0.8, 0.0, 1.0,);
    pub const 史诗紫: Self = Self::from_rgba(0.5, 0.0, 1.0, 1.0,);
    pub const 稀有蓝: Self = Self::from_rgba(0.0, 0.3, 1.0, 1.0,);
    pub const 罕见绿: Self = Self::from_rgba(0.2, 0.8, 0.2, 1.0,);
    pub const 垃圾灰: Self = Self::from_rgba(0.4, 0.4, 0.4, 1.0,);

    pub const 烈焰橙: Self = Self::from_rgba(1.0, 0.2, 0.0, 1.0,);
    pub const 蒂芙尼蓝: Self = Self::from_rgba(0.1, 0.9, 0.7, 1.0,);
    pub const 胭脂红: Self = Self::from_rgba(0.9, 0.1, 0.3, 1.0,);
    pub const 深海蓝: Self = Self::from_rgba(0.0, 0.1, 0.6, 1.0,);
    pub const 橄榄绿: Self = Self::from_rgba(0.4, 0.5, 0.1, 1.0,);

    pub const 发光_纯红: Self = Self::from_rgba(15.0, 0.0, 0.0, 1.0,);
    pub const 发光_纯绿: Self = Self::from_rgba(0.0, 15.0, 0.0, 1.0,);
    pub const 发光_纯蓝: Self = Self::from_rgba(0.0, 0.0, 15.0, 1.0,);
    pub const 发光_纯黄: Self = Self::from_rgba(15.0, 15.0, 0.0, 1.0,);
    pub const 发光_纯紫: Self = Self::from_rgba(15.0, 0.0, 15.0, 1.0,);
    pub const 发光_纯青: Self = Self::from_rgba(0.0, 15.0, 15.0, 1.0,);
    pub const 发光_纯橙: Self = Self::from_rgba(15.0, 7.5, 0.0, 1.0,);
    pub const 发光_纯白: Self = Self::from_rgba(15.0, 15.0, 15.0, 1.0,);

    pub const 发光_荧光绿: Self = Self::from_rgba(7.5, 15.0, 0.0, 1.0,);
    pub const 发光_电光蓝: Self = Self::from_rgba(0.0, 7.5, 15.0, 1.0,);
    pub const 发光_霓虹粉: Self = Self::from_rgba(15.0, 0.0, 7.5, 1.0,);
    pub const 发光_柠檬黄: Self = Self::from_rgba(12.0, 15.0, 0.0, 1.0,);
    pub const 发光_冰晶蓝: Self = Self::from_rgba(0.0, 15.0, 12.0, 1.0,);
    pub const 发光_亮紫色: Self = Self::from_rgba(9.0, 3.0, 15.0, 1.0,);

    pub const 发光_传说金: Self = Self::from_rgba(15.0, 12.0, 0.0, 1.0,);
    pub const 发光_史诗紫: Self = Self::from_rgba(7.5, 0.0, 15.0, 1.0,);
    pub const 发光_稀有蓝: Self = Self::from_rgba(0.0, 4.5, 15.0, 1.0,);
    pub const 发光_烈焰橙: Self = Self::from_rgba(15.0, 3.0, 0.0, 1.0,);
    pub const 发光_蒂芙尼蓝: Self = Self::from_rgba(1.5, 13.5, 10.5, 1.0,);
    pub const 发光_胭脂红: Self = Self::from_rgba(13.5, 1.5, 4.5, 1.0,);

    #[inline]
    pub const fn zero() -> Self {
        unsafe { ::core::mem::zeroed() }
    }

    #[inline]
    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32,) -> Self {
        Self { R: r, G: g, B: b, A: a, }
    }
}
