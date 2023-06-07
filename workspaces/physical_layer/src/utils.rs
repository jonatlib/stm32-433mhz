use core::cell::{Ref, RefCell, RefMut};
use static_cell::StaticCell;

pub struct Shared<T: 'static> {
    reference: &'static RefCell<T>,
}

impl<T: 'static> Shared<T> {
    pub fn new(value: T, memory: &'static StaticCell<RefCell<T>>) -> Self {
        let reference: &'static RefCell<T> = memory.init(RefCell::new(value));
        Self { reference }
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        RefCell::borrow(self.reference)
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        RefCell::borrow_mut(self.reference)
    }
}

impl<T: 'static> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self {
            reference: self.reference,
        }
    }
}
