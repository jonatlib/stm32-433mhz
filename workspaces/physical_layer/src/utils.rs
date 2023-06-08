use core::cell::{Ref, RefCell, RefMut};
use embassy_stm32::exti::ExtiInput;
use static_cell::StaticCell;

use embassy_stm32::gpio::{Input, Output, Pin};

pub struct SharedPin<'a, T> {
    reference: &'a RefCell<T>,
}

impl<'a, T> SharedPin<'a, T> {
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

impl<'a, T> Clone for SharedPin<'a, T> {
    fn clone(&self) -> Self {
        Self {
            reference: self.reference,
        }
    }
}

impl<'a, T> Copy for SharedPin<'a, T> {}

impl<'a, T: Pin> SharedPin<'a, ExtiInput<'a, T>> {
    pub async fn wait_for_falling_edge(&self) {
        self.borrow_mut().wait_for_falling_edge().await
    }

    pub async fn wait_for_rising_edge(&self) {
        self.borrow_mut().wait_for_rising_edge().await
    }
}

impl<'a, T: Pin> SharedPin<'a, Output<'a, T>> {
    pub fn set_high(&self) {
        self.borrow_mut().set_high()
    }

    pub fn set_low(&self) {
        self.borrow_mut().set_low()
    }
}

impl<'a, T: Pin> SharedPin<'a, Input<'a, T>> {
    pub fn is_high(&self) -> bool {
        self.borrow().is_high()
    }

    pub fn is_low(&self) -> bool {
        self.borrow().is_low()
    }
}
