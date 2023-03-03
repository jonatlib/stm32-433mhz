use crate::vec_type::ColdVec;

pub(super) struct ColdVecIter<'a, T> {
    pub index: usize,
    pub vector: &'a ColdVec<'a, T>,
}

impl<'a, T> Iterator for ColdVecIter<'a, T>
where
    T: Sized,
    [(); core::mem::size_of::<T>()]:,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vector.len() {
            return None;
        }

        let value = self
            .vector
            .get(self.index)
            .expect("Memory error while iterating over vector.");

        self.index += 1;

        value
    }
}
