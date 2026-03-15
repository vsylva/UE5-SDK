use ::core::marker::PhantomData;

#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct TArray<T,> {
    pub Data:        *mut T,
    pub NumElements: i32,
    pub MaxElements: i32,
    _marker:         PhantomData<T,>,
}

unsafe impl<T: Send,> Send for TArray<T,> {}
unsafe impl<T: Sync,> Sync for TArray<T,> {}

impl<T,> TArray<T,> {
    #[inline]
    pub const fn zero() -> Self {
        Self { Data: ::core::ptr::null_mut(), NumElements: 0, MaxElements: 0, _marker: PhantomData, }
    }

    #[inline]
    pub unsafe fn from_raw_parts(data: *mut T, num: i32, max: i32,) -> Self {
        Self { Data: data, NumElements: num, MaxElements: max, _marker: PhantomData, }
    }

    #[inline]
    pub fn num(&self,) -> i32 {
        self.NumElements
    }
    #[inline]
    pub fn max(&self,) -> i32 {
        self.MaxElements
    }
    #[inline]
    pub fn slack(&self,) -> i32 {
        self.MaxElements - self.NumElements
    }

    #[inline]
    pub fn is_valid_index(&self, index: i32,) -> bool {
        !self.Data.is_null() && index >= 0 && index < self.NumElements
    }

    #[inline]
    pub fn is_valid(&self,) -> bool {
        !self.Data.is_null() && self.NumElements > 0 && self.MaxElements >= self.NumElements
    }

    #[inline]
    pub fn as_ptr(&self,) -> *mut T {
        self.Data
    }
    #[inline]
    pub fn as_mut_ptr(&mut self,) -> *mut T {
        self.Data
    }

    #[inline]
    pub unsafe fn as_slice(&self,) -> &[T] {
        if self.Data.is_null() || self.NumElements <= 0
        {
            return &[];
        }
        ::core::slice::from_raw_parts(self.Data, self.NumElements as usize,)
    }

    #[inline]
    pub unsafe fn as_slice_mut(&mut self,) -> &mut [T] {
        if self.Data.is_null() || self.NumElements <= 0
        {
            return &mut [];
        }
        ::core::slice::from_raw_parts_mut(self.Data, self.NumElements as usize,)
    }

    #[inline]
    pub fn get(&self, index: i32,) -> Option<&T,> {
        if self.is_valid_index(index,) { Some(unsafe { &*self.Data.add(index as usize,) },) } else { None }
    }

    #[inline]
    pub fn get_mut(&mut self, index: i32,) -> Option<&mut T,> {
        if self.is_valid_index(index,) { Some(unsafe { &mut *self.Data.add(index as usize,) },) } else { None }
    }

    #[inline]
    pub fn index(&self, index: i32,) -> &T {
        assert!(self.is_valid_index(index), "TArray index {index} out of range (num={})", self.NumElements);
        unsafe { &*self.Data.add(index as usize,) }
    }

    #[inline]
    pub fn index_mut(&mut self, index: i32,) -> &mut T {
        assert!(self.is_valid_index(index), "TArray index {index} out of range (num={})", self.NumElements);
        unsafe { &mut *self.Data.add(index as usize,) }
    }

    pub fn find_by<F,>(&self, mut predicate: F,) -> Option<&T,>
    where
        F: FnMut(&T,) -> bool,
    {
        for item in self.iter()
        {
            if predicate(item,)
            {
                return Some(item,);
            }
        }
        None
    }

    pub fn contains_by<F,>(&self, predicate: F,) -> bool
    where
        F: FnMut(&T,) -> bool,
    {
        self.find_by(predicate,).is_some()
    }

    #[inline]
    pub fn iter(&self,) -> TArrayIter<'_, T,> {
        TArrayIter { ptr: self.Data, index: 0, num: self.NumElements, _marker: PhantomData, }
    }

    #[inline]
    pub fn iter_mut(&mut self,) -> TArrayIterMut<'_, T,> {
        TArrayIterMut { ptr: self.Data, index: 0, num: self.NumElements, _marker: PhantomData, }
    }
}

impl<T: PartialEq,> TArray<T,> {
    pub fn find(&self, target: &T,) -> Option<&T,> {
        self.find_by(|e| e == target,)
    }

    pub fn contains(&self, target: &T,) -> bool {
        self.contains_by(|e| e == target,)
    }
}

impl<T,> ::core::ops::Index<i32,> for TArray<T,> {
    type Output = T;
    #[inline]
    fn index(&self, index: i32,) -> &T {
        self.index(index,)
    }
}

impl<T,> ::core::ops::IndexMut<i32,> for TArray<T,> {
    #[inline]
    fn index_mut(&mut self, index: i32,) -> &mut T {
        self.index_mut(index,)
    }
}

impl<T,> ::core::ops::Index<usize,> for TArray<T,> {
    type Output = T;
    #[inline]
    fn index(&self, index: usize,) -> &T {
        self.index(index as i32,)
    }
}

impl<T,> ::core::ops::IndexMut<usize,> for TArray<T,> {
    #[inline]
    fn index_mut(&mut self, index: usize,) -> &mut T {
        self.index_mut(index as i32,)
    }
}

impl<T,> From<&TArray<T,>,> for bool {
    #[inline]
    fn from(a: &TArray<T,>,) -> bool {
        a.is_valid()
    }
}

pub struct TArrayIter<'a, T,> {
    ptr:     *mut T,
    index:   i32,
    num:     i32,
    _marker: PhantomData<&'a T,>,
}

impl<'a, T,> Iterator for TArrayIter<'a, T,> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self,) -> Option<Self::Item,> {
        if self.index < self.num
        {
            let item = unsafe { &*self.ptr.add(self.index as usize,) };
            self.index += 1;
            Some(item,)
        }
        else
        {
            None
        }
    }

    #[inline]
    fn size_hint(&self,) -> (usize, Option<usize,>,) {
        let remaining = (self.num - self.index).max(0,) as usize;
        (remaining, Some(remaining,),)
    }
}

impl<'a, T,> ExactSizeIterator for TArrayIter<'a, T,> {}

pub struct TArrayIterMut<'a, T,> {
    ptr:     *mut T,
    index:   i32,
    num:     i32,
    _marker: PhantomData<&'a mut T,>,
}

impl<'a, T,> Iterator for TArrayIterMut<'a, T,> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self,) -> Option<Self::Item,> {
        if self.index < self.num
        {
            let item = unsafe { &mut *self.ptr.add(self.index as usize,) };
            self.index += 1;
            Some(item,)
        }
        else
        {
            None
        }
    }

    #[inline]
    fn size_hint(&self,) -> (usize, Option<usize,>,) {
        let remaining = (self.num - self.index).max(0,) as usize;
        (remaining, Some(remaining,),)
    }
}

impl<'a, T,> ExactSizeIterator for TArrayIterMut<'a, T,> {}

impl<'a, T,> IntoIterator for &'a TArray<T,> {
    type Item = &'a T;
    type IntoIter = TArrayIter<'a, T,>;
    fn into_iter(self,) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T,> IntoIterator for &'a mut TArray<T,> {
    type Item = &'a mut T;
    type IntoIter = TArrayIterMut<'a, T,>;
    fn into_iter(self,) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: Copy,> IntoIterator for TArray<T,> {
    type Item = T;
    type IntoIter = TArrayIntoIter<T,>;
    fn into_iter(self,) -> Self::IntoIter {
        TArrayIntoIter { ptr: self.Data, index: 0, num: self.NumElements, _marker: PhantomData, }
    }
}

pub struct TArrayIntoIter<T,> {
    ptr:     *mut T,
    index:   i32,
    num:     i32,
    _marker: PhantomData<T,>,
}

impl<T: Copy,> Iterator for TArrayIntoIter<T,> {
    type Item = T;

    #[inline]
    fn next(&mut self,) -> Option<Self::Item,> {
        if self.index < self.num
        {
            let item = unsafe { *self.ptr.add(self.index as usize,) };
            self.index += 1;
            Some(item,)
        }
        else
        {
            None
        }
    }

    #[inline]
    fn size_hint(&self,) -> (usize, Option<usize,>,) {
        let remaining = (self.num - self.index).max(0,) as usize;
        (remaining, Some(remaining,),)
    }
}

impl<T: Copy,> ExactSizeIterator for TArrayIntoIter<T,> {}
