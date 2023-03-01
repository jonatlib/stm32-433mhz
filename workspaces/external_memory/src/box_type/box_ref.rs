use crate::box_type::ColdBox;

use core::ops::Deref;

pub struct SuperBoxRef<'a, T> {
    pub(super) value: T,
    pub(super) handle: &'a ColdBox<'a, T>,
}

pub struct SuperBoxRefMut<'a, T>
where
    [(); core::mem::size_of::<T>()]:,
{
    pub(super) value: T,
    pub(super) handle: &'a mut ColdBox<'a, T>,
}

impl<'a, T> Deref for SuperBoxRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, T> Drop for SuperBoxRefMut<'a, T>
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
