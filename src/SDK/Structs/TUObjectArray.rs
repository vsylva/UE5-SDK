use super::super::{Structs, Structs::FUObjectItem::FUObjectItem, UObject::UObject};

#[struct_macro::disinherit]
#[repr(C)]
pub struct TUObjectArray {
    pub Objects: *mut *mut Structs::FUObjectItem::FUObjectItem,

    #[offset(0x0010)]
    pub MaxElements: i32,
    pub NumElements: i32,
    pub MaxChunks:   i32,
    pub NumChunks:   i32,
}

impl TUObjectArray {
    const ELEMENTS_PER_CHUNK: i32 = 0x10000;

    #[inline]
    pub unsafe fn get_object_by_index(&self, index: i32,) -> *mut UObject {
        let item_ptr = self.get_object_item_by_index(index,);

        if item_ptr.is_null()
        {
            return ::core::ptr::null_mut();
        }

        item_ptr.as_ref_unchecked().Object.cast()
    }

    #[inline]
    pub unsafe fn get_object_item_by_index(&self, index: i32,) -> *mut FUObjectItem {
        if index < 0 || index >= self.NumElements
        {
            return ::core::ptr::null_mut();
        }

        let chunk_index = index / Self::ELEMENTS_PER_CHUNK;
        let in_chunk_idx = index % Self::ELEMENTS_PER_CHUNK;

        if chunk_index >= self.NumChunks
        {
            return ::core::ptr::null_mut();
        }

        let chunk_ptr = *self.Objects.offset(chunk_index as isize,);
        if chunk_ptr.is_null()
        {
            return ::core::ptr::null_mut();
        }

        chunk_ptr.offset(in_chunk_idx as isize,)
    }

    #[inline]
    pub unsafe fn find_object<T,>(&self, name: &str,) -> *mut T {
        for i in 0..self.NumElements
        {
            let object = self.get_object_by_index(i,);

            if object.is_null()
            {
                return ::core::ptr::null_mut();
            }

            let full_name = object.as_ref_unchecked().get_full_name();
            if full_name == name
            {
                return object.cast();
            }
        }

        ::core::ptr::null_mut()
    }
}
