#[struct_macro::inherit(super::UField::UField, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct UStruct {
    #[offset(0x0030)]
    pub BaseChain: FStructBaseChain,

    #[offset(0x40)]
    pub SuperStruct: *mut UStruct,

    #[offset(0x0050)]
    pub ChildProperties: *mut super::FField::FField,
}

impl UStruct {
    #[inline]
    pub unsafe fn is_subclass_of(&self, base_ptr: *mut UStruct,) -> bool {
        if base_ptr.is_null()
        {
            return false;
        }

        let base = base_ptr.as_mut_unchecked();
        let num_parent = base.BaseChain.NumStructBasesInChainMinusOne;

        if num_parent <= self.BaseChain.NumStructBasesInChainMinusOne
        {
            let target_chain_ptr = *self.BaseChain.StructBaseChainArray.add(num_parent as usize,);

            return target_chain_ptr == &base.BaseChain as *const FStructBaseChain as *mut _;
        }

        false
    }
}

#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct FStructBaseChain {
    pub StructBaseChainArray:          *mut *mut FStructBaseChain,
    pub NumStructBasesInChainMinusOne: i32,
}
