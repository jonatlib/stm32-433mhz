#![no_std]

use core::cmp::Ordering;
use core::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct SequenceNumber<const MODULO: u8> {
    value: u8,
}

impl<const MODULO: u8> SequenceNumber<MODULO> {
    pub fn new(value: u8) -> Self {
        Self {
            value: value % MODULO,
        }
    }

    #[inline]
    pub fn value(&self) -> u8 {
        self.value
    }

    #[inline]
    pub fn advance(&mut self) -> Self {
        let old_value = self.clone();
        self.value = (self.value + 1) % MODULO;
        old_value
    }

    /// This method computes how far away are those two
    /// sequence numbers if they were ordered the way that
    /// `self` comes first and `other` comes later.
    pub fn positive_distance(&self, other: &Self) -> u8 {
        if self == other {
            return 0;
        }

        if self.value() < other.value() {
            other.value() - self.value()
        } else {
            (MODULO - self.value()) + other.value()
        }
    }

    /// This method computes how far away are those two
    /// sequence numbers if they were ordered the way that
    /// `other` comes first and `self` comes later.
    pub fn negative_distance(&self, other: &Self) -> u8 {
        if self == other {
            return 0;
        }

        if other.value() < self.value() {
            self.value() - other.value()
        } else {
            (MODULO - other.value()) + self.value()
        }
    }

    pub fn min_distance(&self, other: &Self) -> u8 {
        self.positive_distance(other)
            .min(self.negative_distance(other))
    }
}

impl<const MODULO: u8> From<SequenceNumber<MODULO>> for u32 {
    fn from(value: SequenceNumber<MODULO>) -> Self {
        value.value() as u32
    }
}

impl<const MODULO: u8> From<u32> for SequenceNumber<MODULO> {
    fn from(value: u32) -> Self {
        Self::new((value % (MODULO as u32)) as u8)
    }
}

impl<const MODULO: u8> From<u8> for SequenceNumber<MODULO> {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl<const MODULO: u8> Debug for SequenceNumber<MODULO> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "SequenceNumber<{}>[{}]", MODULO, self.value())
    }
}

impl<const MODULO: u8> defmt::Format for SequenceNumber<MODULO> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "SequenceNumber<{}>[{}]", MODULO, self.value())
    }
}

impl<const MODULO: u8> PartialEq for SequenceNumber<MODULO> {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl<const MODULO: u8> PartialOrd for SequenceNumber<MODULO> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.positive_distance(other) > self.negative_distance(other) {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}
