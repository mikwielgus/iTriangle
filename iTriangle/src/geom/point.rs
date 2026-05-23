use i_overlay::i_float::int::number::int::IntNumber;
use i_overlay::i_float::int::point::IntPoint;

#[derive(Debug, Clone, Copy)]
pub struct IndexPoint<I: IntNumber> {
    pub index: usize,
    pub point: IntPoint<I>,
}

impl<I: IntNumber> IndexPoint<I> {
    #[inline]
    pub fn new(index: usize, point: IntPoint<I>) -> Self {
        Self { index, point }
    }

    #[inline]
    pub const fn empty() -> Self {
        Self {
            index: usize::MAX,
            point: IntPoint::ZERO,
        }
    }
}

impl<I: IntNumber> Default for IndexPoint<I> {
    #[inline]
    fn default() -> Self {
        IndexPoint::empty()
    }
}
