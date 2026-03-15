#[derive(Debug, Copy, Clone, Default,)]
#[repr(C)]
pub struct FName {
    pub ComparisonIndex: i32,
    pub Number:          i32,
}

impl FName {
    #[inline]
    pub const fn zero() -> Self {
        unsafe { ::core::mem::zeroed() }
    }

    #[inline]
    pub unsafe fn to_string(&self,) -> String {
        let mut buffer = [0u16; 256];
        let mut temp_fstring = super::FString::FString::from_ptr(buffer.as_mut_ptr(), 0, 256,);

        let get_name_entry = crate::SpoofCall!(super::super::Globals::GetNmaeEntryFnAddr,self.ComparisonIndex,-> *mut ::core::ffi::c_void);

        crate::SpoofCall!(super::super::Globals::AppendStringFnAddr, get_name_entry, &mut temp_fstring);

        let mut output = temp_fstring.to_string();

        if self.Number > 0
        {
            output.push_str(&format!("_{}", self.Number - 1),);
        }

        output
    }

    #[inline]
    pub unsafe fn to_string_by_chunk(&self,) -> String {
        let chunk_idx = (self.ComparisonIndex >> 16) as usize;
        let name_idx = (self.ComparisonIndex as u16) as usize;

        let pool_addr = super::super::Globals::GnamesP;
        let blocks_ptr = (pool_addr + 0x10) as *mut usize;

        let chunk_ptr = *blocks_ptr.add(chunk_idx,);
        if chunk_ptr == 0
        {
            return "None".to_owned();
        }

        let entry_addr = chunk_ptr + (name_idx * 2);

        let header = *(entry_addr as *const u16);

        let is_wide = (header & 0x1) != 0;
        let len = (header >> 6) as usize;

        if len == 0 || len > 256
        {
            return "None".to_owned();
        }

        let str_ptr = (entry_addr + 2) as *mut u8;

        if is_wide
        {
            let utf16_slice = ::core::slice::from_raw_parts(str_ptr as *const u16, len,);
            String::from_utf16_lossy(utf16_slice,)
        }
        else
        {
            let slice = ::core::slice::from_raw_parts(str_ptr, len,);
            String::from_utf8_lossy(slice,).into()
        }
    }
}

impl PartialEq for FName {
    fn eq(&self, other: &Self,) -> bool {
        self.ComparisonIndex == other.ComparisonIndex
    }
}
