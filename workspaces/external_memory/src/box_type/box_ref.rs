use crate::box_type::ColdBox;

use core::ops::Deref;

pub struct ColdBoxRef<'a, T> {
    pub(super) value: T,
    pub(super) handle: &'a ColdBox<'a, T>,
}

pub struct ColdBoxRefMut<'a, T>
where
    [(); core::mem::size_of::<T>()]:,
{
    pub(super) value: T,
    pub(super) handle: &'a mut ColdBox<'a, T>,
}

pub struct ColdBoxArrayRef<'a, T, const SIZE: usize> {
    pub(super) handle: &'a ColdBox<'a, [T; SIZE]>,
}

impl<'a, T> Deref for ColdBoxRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, T> Drop for ColdBoxRefMut<'a, T>
where
    [(); core::mem::size_of::<T>()]:,
{
    fn drop(&mut self) {
        let value = unsafe { core::mem::replace(&mut self.value, core::mem::zeroed()) };
        self.handle
            .update(value)
            .expect("Memory could not be written");
    }
}
