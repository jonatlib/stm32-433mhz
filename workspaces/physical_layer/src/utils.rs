use core::cell::{Ref, RefCell, RefMut};
use embassy_stm32::exti::ExtiInput;
use static_cell::StaticCell;

use embassy_stm32::gpio::Pin;

pub struct SharedExtiPin<'a, T> {
    reference: &'a RefCell<T>,
}

impl<'a, T> SharedExtiPin<'a, T> {
    pub fn new<'b>(value: T, memory: &'b StaticCell<RefCell<T>>) -> Self
    where
        'b: 'static,
        'a: 'b,
    {
        let reference: &RefCell<T> = memory.init(RefCell::new(value));
        Self { reference }
    }

    pub fn borrow(&self) -> Ref<'a, T> {
        RefCell::borrow(self.reference)
    }

    pub fn borrow_mut(&self) -> RefMut<'a, T> {
        RefCell::borrow_mut(self.reference)
    }
}

impl<'a, T> Clone for SharedExtiPin<'a, T> {
    fn clone(&self) -> Self {
        Self {
            reference: self.reference,
        }
    }
}

impl<'a, T> Copy for SharedExtiPin<'a, T> {}

impl<'a, T: Pin> SharedExtiPin<'a, ExtiInput<'a, T>> {
    pub async fn wait_for_falling_edge(&self) {
        self.borrow_mut().wait_for_falling_edge().await
    }

    pub async fn wait_for_rising_edge(&self) {
        self.borrow_mut().wait_for_rising_edge().await
    }
}
