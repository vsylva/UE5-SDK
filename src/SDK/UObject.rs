use super::{Globals::GobjectsP, UClass::EClassCastFlags};

#[struct_macro::disinherit]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct UObject {
    VTable: *mut *mut ::core::ffi::c_void,

    #[offset(0x0008)]
    pub Flags: i32,

    #[offset(0x000C)]
    pub Index: i32,

    #[offset(0x0010)]
    pub Class: *mut super::UClass::UClass,

    #[offset(0x0018)]
    pub Name: super::Structs::FName::FName,

    #[offset(0x0020)]
    pub Outer: *mut UObject,
}

impl UObject {
    #[inline]
    pub unsafe fn is_a(&self, class: *mut super::UClass::UClass,) -> bool {
        (*self.Class).is_subclass_of(class.cast(),)
    }

    #[inline]
    pub unsafe fn is_ac(&self, flags: EClassCastFlags,) -> bool {
        ((*self.Class).CastFlags as u64 & flags as u64) != 0
    }

    #[inline]
    pub unsafe fn get_full_name(&self,) -> String {
        if self.Class.is_null()
        {
            return "None".to_owned();
        }

        let mut temp = String::new();
        let mut next_outer = self.Outer;

        while let Some(obj,) = next_outer.as_ref()
        {
            temp = format!("{}.{}", obj.Name.to_string(), temp);
            next_outer = obj.Outer;
        }

        let class_name = (*self.Class).Name.to_string();
        let self_name = self.Name.to_string();

        format!("{} {}{}", class_name, temp, self_name)
    }

    #[inline]
    pub unsafe fn has_type_flag(&self, TypeFlags: EClassCastFlags,) -> bool {
        (self.Class.as_ref_unchecked().CastFlags as u64 & TypeFlags as u64) != 0
    }

    #[inline]
    pub unsafe fn find_object_fast_impl(name: &str, required_type: EClassCastFlags,) -> *mut UObject {
        let gobjects = GobjectsP;

        if gobjects.is_null()
        {
            return ::core::ptr::null_mut();
        }

        let arr = &*gobjects;

        for i in 0..arr.NumElements
        {
            let obj = arr.get_object_by_index(i,);

            if obj.is_null()
            {
                continue;
            }

            if !(*obj).has_type_flag(required_type,)
            {
                continue;
            }

            if (*obj).Name.to_string() == name
            {
                return obj;
            }
        }

        ::core::ptr::null_mut()
    }

    #[inline]
    pub unsafe fn find_object_impl(full_name: &str, required_type: EClassCastFlags,) -> *mut UObject {
        let gobjects = GobjectsP;

        if gobjects.is_null()
        {
            return ::core::ptr::null_mut();
        }

        let arr = &*gobjects;

        for i in 0..arr.NumElements
        {
            let obj = arr.get_object_by_index(i,);

            if obj.is_null()
            {
                continue;
            }

            if (*obj).has_type_flag(required_type,) && (*obj).get_full_name() == full_name
            {
                return obj;
            }
        }

        ::core::ptr::null_mut()
    }
}
