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

    pub fn get_insertion_order_ascending(
        &self,
        sequence: &[Self],
        first_element: Option<&Self>,
    ) -> Option<usize> {
        if sequence.is_empty() {
            return Some(0);
        }

        for (index, item) in sequence.iter().enumerate() {
            if self == item {
                return None;
            }

            if let Some(cmp) = self.partial_compare(item, first_element) {
                if cmp.is_le() {
                    return Some(index);
                }
            }
        }

        Some(sequence.len())
    }

    pub fn partial_compare(&self, other: &Self, first_element: Option<&Self>) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        if let Some(base) = first_element {
            let self_from_base = base.positive_distance(self);
            let other_from_base = base.positive_distance(other);

            if self_from_base > other_from_base {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        } else {
            // FIXME i don't know about this branch?
            // FIXME It will compare only numbers mod/2 apart
            let positive = self.positive_distance(other);
            let negative = self.negative_distance(other);

            // Without any fix point we can't compare these two values
            if positive > MODULO / 2 {
                return None;
            }

            if positive > negative {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        }
    }

    pub fn compare(&self, other: &Self, first_element: &Self) -> Ordering {
        self.partial_compare(other, Some(first_element))
            .expect("When first element is passed this should never happen")
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

// FIXME is this good idea to implement it with some base? Or None base? Or?
impl<const MODULO: u8> PartialOrd for SequenceNumber<MODULO> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.partial_compare(other, None)
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

    /// Since there is asymmetry in cmp we have to use bubble sort to get correct order
    fn bubble_sort(arr: &mut [SN], base_element: Option<u8>) {
        for i in 0..arr.len() {
            for j in 0..arr.len() - 1 - i {
                if let Some(cmp) =
                    arr[j].partial_compare(&arr[j + 1], base_element.map(|v| v.into()).as_ref())
                {
                    if cmp.is_gt() {
                        arr.swap(j, j + 1);
                    }
                }
            }
        }
    }

    #[test]
    fn test_simple_comparison() {
        let a = SN::new(4);
        let b = SN::new(5);

        assert!(a < b);
        // assert!(b > a); // This can't be tested without `base`

        let a = SN::new(8);
        let b = SN::new(0);

        assert_eq!(a, b);
    }

    #[test]
    fn test_complex_comparison() {
        let a_0 = SN::new(0);
        let a_1 = SN::new(1);
        let a_6 = SN::new(6);
        let a_7 = SN::new(7);

        assert_eq!(
            a_7.positive_distance(&a_0),
            1,
            "From 7mod8 to 0mod8 it is one step forward"
        );
        assert_eq!(
            a_7.negative_distance(&a_0),
            7,
            "From 7mod8 to 0mod8 backwards it is 7steps back."
        );

        assert!(a_0.compare(&a_7, &5u8.into()).is_gt());
        assert!(a_6 < a_7);
        assert!(a_0 < a_1);
    }

    #[test]
    fn test_ordering_simple() {
        let mut numbers: Vec<_> = (0..8u8).into_iter().map(SN::new).collect();
        bubble_sort(&mut numbers, None);
        let result: Vec<u8> = numbers.iter().map(|v| v.value()).collect();
        assert_eq!(result, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_ordering_random() {
        let mut numbers: Vec<_> = vec![2, 5, 7, 3, 1, 4, 6, 0]
            .into_iter()
            .map(SN::new)
            .collect();
        bubble_sort(&mut numbers, Some(0)); // We use bubble sort to go through each element
        let result: Vec<u8> = numbers.iter().map(|v| v.value()).collect();

        assert_eq!(result, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_ordering_complex() {
        let mut numbers: Vec<_> = vec![5, 6, 3, 4, 7, 0, 2, 1]
            .into_iter()
            .map(SN::new)
            .collect();
        bubble_sort(&mut numbers, Some(5));
        let result: Vec<u8> = numbers.iter().map(|v| v.value()).collect();
        assert_eq!(result, vec![5, 6, 7, 0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_insertion_ordering() {
        let insertion_order: Vec<_> = vec![5, 6, 3, 4, 7, 0, 2, 1]
            .into_iter()
            .map(SN::new)
            .collect();
        let expected = vec![5, 6, 7, 0, 1, 2, 3, 4];

        let mut data: Vec<SN> = Vec::new();
        for insert in insertion_order.into_iter() {
            let index = insert.get_insertion_order_ascending(&data[..], Some(&5u8.into()));
            data.insert(
                index.expect("This should be always a value in this test"),
                insert,
            );
        }

        let result: Vec<u8> = data.iter().map(|v| v.value()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_insertion_ordering_random() {
        let base = SN::new(5);
        let insertion_order: Vec<_> = vec![6, 3, 5, 4, 7, 0, 2, 1]
            .into_iter()
            .map(SN::new)
            .collect();
        let expected = vec![5, 6, 7, 0, 1, 2, 3, 4];

        let mut data: Vec<SN> = Vec::new();
        let mut seen_base = false;

        for insert in insertion_order.into_iter() {
            if insert == base {
                seen_base = true;
            }

            let index = insert.get_insertion_order_ascending(
                &data[..],
                if seen_base { Some(&base) } else { None },
            );
            data.insert(
                index.expect("This should be always a value in this test"),
                insert,
            );
        }

        let result: Vec<u8> = data.iter().map(|v| v.value()).collect();
        assert_ne!(result, expected);

        // FIXME is this the correct way? With sorting?
        data.sort_by(|a, b| a.compare(b, &base));

        let result: Vec<u8> = data.iter().map(|v| v.value()).collect();
        assert_eq!(result, expected);
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
