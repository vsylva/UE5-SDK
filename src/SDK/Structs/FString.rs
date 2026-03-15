use ::core::arch::x86_64::*;

#[derive(PartialEq, Debug, Clone, Copy,)]
#[repr(C)]
pub struct FString {
    pub Data:        *mut u16,
    pub NumElements: i32,
    pub MaxElements: i32,
}

const FSTRING_BUFFER_SIZE: usize = 512;
static mut STRING_BUFFER: [u16; FSTRING_BUFFER_SIZE] = [0; FSTRING_BUFFER_SIZE];

impl FString {
    #[inline]
    pub const fn is_valid(&self,) -> bool {
        !self.Data.is_null() && self.NumElements > 1
    }

    #[inline]
    pub unsafe fn tiny_static() -> Self {
        STRING_BUFFER[0] = 0;
        Self { Data: STRING_BUFFER.as_mut_ptr(), NumElements: 1, MaxElements: FSTRING_BUFFER_SIZE as i32, }
    }

    #[inline]
    pub const fn from_ptr(ptr: *mut u16, count: i32, max: i32,) -> Self {
        Self { Data: ptr, NumElements: count, MaxElements: max, }
    }

    #[inline]
    pub const fn zero() -> Self {
        unsafe { ::core::mem::zeroed() }
    }

    #[inline]
    pub unsafe fn append_str(&mut self, s: &str,) -> &mut Self {
        let bytes = s.as_bytes();
        let src_len = bytes.len();
        let write_pos = (self.NumElements - 1).max(0,) as usize;
        let mut dst = self.Data.add(write_pos,);
        let mut i = 0usize;

        while i + 16 <= src_len
        {
            let chunk = _mm_loadu_si128(bytes.as_ptr().add(i,) as *mut __m128i,);
            if _mm_movemask_epi8(chunk,) == 0
            {
                let zero = _mm_setzero_si128();
                _mm_storeu_si128(dst as *mut __m128i, _mm_unpacklo_epi8(chunk, zero,),);
                _mm_storeu_si128(dst.add(8,) as *mut __m128i, _mm_unpackhi_epi8(chunk, zero,),);
                dst = dst.add(16,);
                i += 16;
            }
            else
            {
                break;
            }
        }

        while i < src_len
        {
            let b0 = *bytes.get_unchecked(i,);

            if b0 < 0x80
            {
                *dst = b0 as u16;
                dst = dst.add(1,);
                i += 1;
            }
            else if b0 < 0xE0
            {
                let b1 = *bytes.get_unchecked(i + 1,);
                *dst = ((b0 as u16 & 0x1F) << 6) | (b1 as u16 & 0x3F);
                dst = dst.add(1,);
                i += 2;
            }
            else if b0 < 0xF0
            {
                let b1 = *bytes.get_unchecked(i + 1,);
                let b2 = *bytes.get_unchecked(i + 2,);
                *dst = ((b0 as u16 & 0x0F) << 12) | ((b1 as u16 & 0x3F) << 6) | (b2 as u16 & 0x3F);
                dst = dst.add(1,);
                i += 3;
            }
            else
            {
                let b1 = *bytes.get_unchecked(i + 1,);
                let b2 = *bytes.get_unchecked(i + 2,);
                let b3 = *bytes.get_unchecked(i + 3,);
                let cp = ((b0 as u32 & 0x07) << 18)
                    | ((b1 as u32 & 0x3F) << 12)
                    | ((b2 as u32 & 0x3F) << 6)
                    | (b3 as u32 & 0x3F) - 0x10000;
                *dst = ((cp >> 10) as u16) | 0xD800;
                *dst.add(1,) = ((cp & 0x3FF) as u16) | 0xDC00;
                dst = dst.add(2,);
                i += 4;
            }
        }

        *dst = 0;
        self.NumElements = dst.offset_from(self.Data,) as i32 + 1;
        self
    }

    #[inline]
    pub unsafe fn append_ascii<'a,>(&mut self, s: &str,) {
        let bytes = s.as_bytes();
        let src_len = bytes.len();
        let write_pos = (self.NumElements - 1).max(0,) as usize;
        let mut dst = self.Data.add(write_pos,);
        let mut i = 0usize;

        while i + 16 <= src_len
        {
            let chunk = _mm_loadu_si128(bytes.as_ptr().add(i,) as *mut __m128i,);
            let zero = _mm_setzero_si128();
            _mm_storeu_si128(dst as *mut __m128i, _mm_unpacklo_epi8(chunk, zero,),);
            _mm_storeu_si128(dst.add(8,) as *mut __m128i, _mm_unpackhi_epi8(chunk, zero,),);
            dst = dst.add(16,);
            i += 16;
        }
        while i < src_len
        {
            *dst = *bytes.get_unchecked(i,) as u16;
            dst = dst.add(1,);
            i += 1;
        }
        *dst = 0;
        self.NumElements = dst.offset_from(self.Data,) as i32 + 1;
    }

    #[inline]
    pub unsafe fn to_string(&self,) -> String {
        if !self.is_valid()
        {
            return String::new();
        }

        String::from_utf16_lossy(::core::slice::from_raw_parts(self.Data, (self.NumElements - 1) as usize,),)
    }
}

impl ::core::ops::Index<usize,> for FString {
    type Output = u16;

    fn index(&self, index: usize,) -> &Self::Output {
        unsafe { &*self.Data.add(index,) }
    }
}
