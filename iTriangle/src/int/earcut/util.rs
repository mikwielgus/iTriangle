use core::cmp::Ordering;
use i_overlay::i_float::int::number::int::IntNumber;
use i_overlay::i_float::int::number::wide_int::WideIntNumber;
use i_overlay::i_float::int::point::IntPoint;
use i_overlay::i_float::int::vector::IntVector as FixVec;

#[derive(PartialEq, Eq)]
pub(super) enum ABCExcludeResult {
    Inside,
    Outside,
    OutsideEdge,
}

// a, b, c - counter clock wised points
pub(super) struct Abc<I: IntNumber> {
    a: IntPoint<I>,
    b: IntPoint<I>,
    c: IntPoint<I>,
    ab: FixVec<I>,
    bc: FixVec<I>,
    ca: FixVec<I>,
}

impl<I: IntNumber> Abc<I> {
    #[inline(always)]
    pub(super) fn new(a: IntPoint<I>, b: IntPoint<I>, c: IntPoint<I>) -> Self {
        let ab = b - a;
        let bc = c - b;
        let ca = a - c;
        Self {
            a,
            b,
            c,
            ab,
            bc,
            ca,
        }
    }

    #[inline(always)]
    pub(super) fn contains(&self, p: IntPoint<I>) -> bool {
        let ap = p - self.a;
        let a_cross = ap.cross_product(self.ab);
        if a_cross >= I::Wide::ZERO {
            return false;
        }

        let bp = p - self.b;
        let b_cross = bp.cross_product(self.bc);
        if b_cross >= I::Wide::ZERO {
            return false;
        }

        let cp = p - self.c;
        let c_cross = cp.cross_product(self.ca);

        c_cross < I::Wide::ZERO
    }

    #[inline(always)]
    pub(super) fn contains_exclude_ca(&self, p: IntPoint<I>) -> ABCExcludeResult {
        let ap = p - self.a;
        let a_cross = ap.cross_product(self.ab);
        if a_cross >= I::Wide::ZERO {
            return ABCExcludeResult::Outside;
        }

        let bp = p - self.b;
        let b_cross = bp.cross_product(self.bc);
        if b_cross >= I::Wide::ZERO {
            return ABCExcludeResult::Outside;
        }

        let cp = p - self.c;
        let c_cross = cp.cross_product(self.ca);

        match c_cross.cmp(&I::Wide::ZERO) {
            Ordering::Less => ABCExcludeResult::Inside,
            Ordering::Equal => {
                if AB::contains(self.a, self.c, p) {
                    ABCExcludeResult::Inside
                } else {
                    ABCExcludeResult::OutsideEdge
                }
            }
            Ordering::Greater => ABCExcludeResult::OutsideEdge,
        }
    }
}

pub(super) struct AB;

impl AB {
    #[inline(always)]
    pub(super) fn contains<I: IntNumber>(a: IntPoint<I>, b: IntPoint<I>, p: IntPoint<I>) -> bool {
        // a, b, p already on one line
        // not including ends
        let ap = a - p;
        let bp = b - p;

        // must have opposite direction
        ap.dot_product(bp) < I::Wide::ZERO
    }
}
