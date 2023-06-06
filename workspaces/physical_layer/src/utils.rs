use core::borrow::Borrow;
use core::cell::{Ref, RefCell};
use core::ops::Deref;
use static_cell::StaticCell;

pub struct Shared<T: 'static> {
    reference: &'static T,
}

impl<T: 'static> Shared<T> {
    pub fn new(value: T, memory: &'static StaticCell<T>) -> Self {
        let reference: &'static T = memory.init(value);
        Self { reference }
    }
}

impl<T: 'static> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self {
            reference: self.reference,
        }
    }
}
