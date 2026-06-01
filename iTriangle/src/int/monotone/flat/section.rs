use crate::geom::point::IndexPoint;
use crate::int::monotone::v_segment::VSegment;
use alloc::vec::Vec;
use i_overlay::i_float::int::number::int::IntNumber;
use i_tree::set::sort::KeyValue;

#[derive(Debug, Clone)]
pub(super) struct FlatSection<I: IntNumber> {
    pub(super) sort: VSegment<I>,
    pub(super) points: Vec<IndexPoint<I>>,
}

impl<I: IntNumber> Default for FlatSection<I> {
    #[inline]
    fn default() -> Self {
        Self {
            sort: Default::default(),
            points: Vec::new(),
        }
    }
}

impl<I: IntNumber> KeyValue<VSegment<I>> for FlatSection<I> {
    #[inline]
    fn key(&self) -> &VSegment<I> {
        &self.sort
    }
}
