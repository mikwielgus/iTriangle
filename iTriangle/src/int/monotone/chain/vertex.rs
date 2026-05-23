use crate::geom::point::IndexPoint;
use i_overlay::i_float::int::number::int::IntNumber;
use i_overlay::i_float::int::point::IntPoint;
use i_overlay::i_float::triangle::Triangle;

#[derive(Debug, Clone, Copy)]
pub(crate) enum VertexType {
    Start,
    End,
    Merge,
    Split,
    Join,
    Steiner,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ChainVertex<I: IntNumber> {
    pub(crate) index: usize,
    pub(crate) this: IntPoint<I>,
    pub(crate) next: IntPoint<I>,
    pub(crate) prev: IntPoint<I>,
}

impl<I: IntNumber> ChainVertex<I> {
    #[inline]
    pub(super) fn new(this: IntPoint<I>, next: IntPoint<I>, prev: IntPoint<I>) -> Self {
        Self {
            index: 0,
            this,
            next,
            prev,
        }
    }

    #[inline]
    pub(crate) fn implant(this: IntPoint<I>) -> Self {
        Self {
            index: 0,
            this,
            next: IntPoint::EMPTY,
            prev: IntPoint::EMPTY,
        }
    }

    #[inline]
    pub(crate) fn get_type(&self) -> VertexType {
        let clock_wise = Triangle::is_clockwise(self.prev, self.this, self.next);
        if self.prev == IntPoint::EMPTY && self.next == IntPoint::EMPTY {
            VertexType::Steiner
        } else if self.prev < self.this && self.next < self.this {
            if clock_wise {
                VertexType::Merge
            } else {
                VertexType::End
            }
        } else if self.this < self.next && self.this < self.prev {
            if clock_wise {
                VertexType::Split
            } else {
                VertexType::Start
            }
        } else {
            VertexType::Join
        }
    }

    #[inline]
    pub(crate) fn index_point(&self) -> IndexPoint<I> {
        IndexPoint::new(self.index, self.this)
    }
}
