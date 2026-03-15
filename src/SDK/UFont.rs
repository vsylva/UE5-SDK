#[struct_macro::inherit(super::UObject::UObject, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct UFont {}

impl UFont {
    #[inline]
    pub unsafe fn get_SansationBold18() -> *mut Self {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(0, -1, crate::w!("Font 'SansationBold18.SansationBold18'"), false,);
        }

        Instance as *mut Self
    }
}
