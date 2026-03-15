use core::ops::{Deref, DerefMut};

use super::super::UClass::UClass;

#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct TSubclassOf<T,> {
    pub ClassPtr: *mut UClass,
    _marker:      ::core::marker::PhantomData<T,>,
}

impl<T,> TSubclassOf<T,> {
    #[inline]
    pub const fn zero() -> Self {
        Self { ClassPtr: ::core::ptr::null_mut(), _marker: ::core::marker::PhantomData, }
    }

    #[inline]
    pub const fn from_ptr(ptr: *mut UClass,) -> Self {
        Self { ClassPtr: ptr, _marker: ::core::marker::PhantomData, }
    }

    #[inline]
    pub fn is_null(&self,) -> bool {
        self.ClassPtr.is_null()
    }
}

impl<T,> Deref for TSubclassOf<T,> {
    type Target = *mut UClass;

    #[inline]
    fn deref(&self,) -> &Self::Target {
        &self.ClassPtr
    }
}

impl<T,> DerefMut for TSubclassOf<T,> {
    #[inline]
    fn deref_mut(&mut self,) -> &mut Self::Target {
        &mut self.ClassPtr
    }
}
