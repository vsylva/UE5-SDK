#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash,)]
pub struct FColor {
    pub B: u8,
    pub G: u8,
    pub R: u8,
    pub A: u8,
}

impl FColor {
    pub const BLUE: Self = Self { R: 0, G: 0, B: 255, A: 255, };
    pub const GREEN: Self = Self { R: 0, G: 255, B: 0, A: 255, };
    pub const RED: Self = Self { R: 255, G: 0, B: 0, A: 255, };
    pub const WHITE: Self = Self { R: 255, G: 255, B: 255, A: 255, };

    #[inline]
    pub const fn zero() -> Self {
        unsafe { ::core::mem::zeroed() }
    }

    #[inline]
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8,) -> Self {
        Self { R: r, G: g, B: b, A: a, }
    }
}
