use super::UGameViewportClient::UGameViewportClient;

#[struct_macro::inherit(super::UObject::UObject, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct UEngine {
    #[offset(0x0BE0)]
    pub GameViewport: *mut UGameViewportClient,
}

impl UEngine {
    #[inline]
    pub unsafe fn get() -> *mut Self {
        let gobject_array_ptr = super::Globals::GobjectsP;

        let total_count = (*gobject_array_ptr).NumElements;

        for i in 0..total_count
        {
            let uobject_ptr = (*gobject_array_ptr).get_object_by_index(i,);

            if uobject_ptr.is_null()
            {
                continue;
            }

            let full_name = uobject_ptr.as_ref_unchecked().get_full_name();

            if full_name.starts_with("ShooterEngine /Engine/Transient.ShooterEngine",)
            {
                return uobject_ptr.cast();
            }
        }

        ::core::ptr::null_mut()
    }
}
