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
        other.positive_distance(self)
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

impl<const MODULO: u8> Eq for SequenceNumber<MODULO> {}

impl<const MODULO: u8> PartialOrd for SequenceNumber<MODULO> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        let positive = self.positive_distance(other);
        let negative = self.negative_distance(other);

        // FIXME is this ok?
        if positive == 1 && negative > 1 {
            return Some(Ordering::Less);
        }

        if positive > negative {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl<const MODULO: u8> Ord for SequenceNumber<MODULO> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("Ordering should always exists.")
    }
}

#[cfg(test)]
#[macro_use]
extern crate std;
#[cfg(test)]
mod test {
    use std::vec::Vec;

    use super::SequenceNumber;
    type SN = SequenceNumber<8>;

    #[test]
    fn test_simple_comparison() {
        let a = SN::new(4);
        let b = SN::new(5);

        assert!(a < b);
        assert!(b > a);

        let a = SN::new(8);
        let b = SN::new(0);

        assert_eq!(a, b);
    }

    #[test]
    fn test_complex_comparison() {
        let a = SN::new(7);
        let b = SN::new(0);

        assert_eq!(
            a.positive_distance(&b),
            1,
            "From 7mod8 to 0mod8 it is one step forward"
        );
        assert_eq!(
            a.negative_distance(&b),
            7,
            "From 7mod8 to 0mod8 backwards it is 7steps back."
        );

        // FIXME
        println!("{}", a.positive_distance(&b)); // This should be 1
        println!("{}", a.negative_distance(&b)); //

        // assert!(a > b);
        assert!(b > a);
    }

    #[test]
    fn test_ordering_simple() {
        let mut numbers: Vec<_> = (0..8u8).into_iter().map(SN::new).collect();
        numbers.sort();
        let result: Vec<u8> = numbers.iter().map(|v| v.value()).collect();
        assert_eq!(result, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_ordering_random() {
        let mut numbers: Vec<_> = vec![2, 5, 7, 3, 1, 4, 6, 0]
            .into_iter()
            .map(SN::new)
            .collect();
        numbers.sort();
        let result: Vec<u8> = numbers.iter().map(|v| v.value()).collect();

        assert_eq!(result, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_ordering_complex() {
        let mut numbers: Vec<_> = vec![5, 6, 3, 4, 7, 0, 2, 1]
            .into_iter()
            .map(SN::new)
            .collect();
        numbers.sort();
        let result: Vec<u8> = numbers.iter().map(|v| v.value()).collect();
        assert_eq!(result, vec![3, 4, 5, 6, 7, 0, 1, 2]);
    }

    #[test]
    fn test_positive_distance() {
        let base = SN::new(0);

        let positive_distances: Vec<_> = (0..8u8)
            .into_iter()
            .map(SN::new)
            .map(|v| v.positive_distance(&base))
            .collect();

        assert_eq!(positive_distances, vec![0, 7, 6, 5, 4, 3, 2, 1])
    }

    #[test]
    fn test_positive_distance_reverse() {
        let base = SN::new(0);

        let positive_distances: Vec<_> = (0..8u8)
            .into_iter()
            .map(SN::new)
            .map(|v| base.positive_distance(&v))
            .collect();

        assert_eq!(positive_distances, vec![0, 1, 2, 3, 4, 5, 6, 7])
    }

    #[test]
    fn test_negative_distance() {
        let base = SN::new(0);

        let negative_distances: Vec<_> = (0..8u8)
            .into_iter()
            .map(SN::new)
            .map(|v| v.negative_distance(&base))
            .collect();

        assert_eq!(negative_distances, vec![0, 1, 2, 3, 4, 5, 6, 7])
    }

    #[test]
    fn test_negative_distance_reverse() {
        let base = SN::new(0);

        let negative_distances: Vec<_> = (0..8u8)
            .into_iter()
            .map(SN::new)
            .map(|v| base.negative_distance(&v))
            .collect();

        assert_eq!(negative_distances, vec![0, 7, 6, 5, 4, 3, 2, 1])
    }
}
