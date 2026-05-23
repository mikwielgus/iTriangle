use crate::int::earcut::earcut_64::{Bit, EarcutStore};
use crate::int::triangulation::{IndexType, IntTriangulation};
use i_overlay::i_float::int::number::int::IntNumber;
use i_overlay::i_float::int::point::IntPoint;

pub(super) struct FlatEarcutStore<'a, I: IntNumber, N: IndexType> {
    triangulation: &'a mut IntTriangulation<I, N>,
}

impl<'a, I: IntNumber, N: IndexType> FlatEarcutStore<'a, I, N> {
    #[inline]
    pub(super) fn new(triangulation: &'a mut IntTriangulation<I, N>) -> Self {
        Self { triangulation }
    }
}

impl<I: IntNumber, N: IndexType> EarcutStore<I> for FlatEarcutStore<'_, I, N> {
    #[inline]
    fn collect_triangles(&mut self, _: &[IntPoint<I>], start: usize, bits: u64, count: u32) {
        let mut i = start;
        let a = unsafe { N::try_from(i).unwrap_unchecked() };
        i = bits.next_wrapped_index(i);
        let mut b = unsafe { N::try_from(i).unwrap_unchecked() };

        for _ in 0..count {
            i = bits.next_wrapped_index(i);
            let c = unsafe { N::try_from(i).unwrap_unchecked() };
            self.triangulation.indices.push(a);
            self.triangulation.indices.push(b);
            self.triangulation.indices.push(c);

            b = c;
        }
    }
}
