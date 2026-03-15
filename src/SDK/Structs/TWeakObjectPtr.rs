#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct TWeakObjectPtr<T,> {
    pub ObjectIndex:        i32,
    pub ObjectSerialNumber: i32,
    _marker:                ::core::marker::PhantomData<T,>,
}

impl<T,> TWeakObjectPtr<T,> {
    #[inline]
    pub unsafe fn get(&self,) -> *mut T {
        super::super::Globals::GobjectsP.as_ref_unchecked().get_object_by_index(self.ObjectIndex,).cast()
    }
}
