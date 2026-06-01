use core::cmp::Ordering;
use i_overlay::i_float::int::number::int::IntNumber;
use i_overlay::i_float::int::point::IntPoint;
use i_overlay::i_float::triangle::Triangle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VSegment<I: IntNumber> {
    pub(crate) a: IntPoint<I>,
    pub(crate) b: IntPoint<I>,
}

impl<I: IntNumber> VSegment<I> {
    #[inline]
    fn is_under_segment_order(&self, other: &VSegment<I>) -> Ordering {
        match self.b.cmp(&other.b) {
            Ordering::Less => Triangle::clock_order(self.b, other.a, other.b),
            Ordering::Equal => Triangle::clock_order(self.b, self.a, other.a),
            Ordering::Greater => Triangle::clock_order(other.b, self.b, self.a),
        }
    }

    #[inline]
    pub(crate) fn is_under_point_order(&self, p: IntPoint<I>) -> Ordering {
        debug_assert!(self.a.x <= p.x && p.x <= self.b.x);
        Triangle::clock_order(self.a, p, self.b)
    }
}

impl<I: IntNumber> PartialOrd<Self> for VSegment<I> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I: IntNumber> Ord for VSegment<I> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.is_under_segment_order(other)
    }
}

impl<I: IntNumber> Default for VSegment<I> {
    #[inline]
    fn default() -> Self {
        Self {
            a: IntPoint::ZERO,
            b: IntPoint::ZERO,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::int::monotone::v_segment::VSegment;
    use core::cmp::Ordering;
    use i_overlay::i_float::int::point::IntPoint;

    #[test]
    fn test_0() {
        let v0 = VSegment {
            a: IntPoint::new(0, 0),
            b: IntPoint::new(5, 0),
        };
        let v1 = VSegment {
            a: IntPoint::new(0, 0),
            b: IntPoint::new(5, 5),
        };

        assert_eq!(v0.is_under_segment_order(&v1), Ordering::Less);
    }

    #[test]
    fn test_1() {
        let v0 = VSegment {
            a: IntPoint::new(-2, -2),
            b: IntPoint::new(5, -2),
        };
        let v1 = VSegment {
            a: IntPoint::new(0, 0),
            b: IntPoint::new(5, 0),
        };

        assert_eq!(v0.is_under_segment_order(&v1), Ordering::Less);
    }

    #[test]
    fn test_2() {
        let v0 = VSegment {
            a: IntPoint::new(-2, -5),
            b: IntPoint::new(5, 0),
        };
        let v1 = VSegment {
            a: IntPoint::new(0, 0),
            b: IntPoint::new(5, 0),
        };

        assert_eq!(v0.is_under_segment_order(&v1), Ordering::Less);
    }

    #[test]
    fn test_3() {
        let v0 = VSegment {
            a: IntPoint::new(0, -5),
            b: IntPoint::new(5, 5),
        };
        let v1 = VSegment {
            a: IntPoint::new(0, 0),
            b: IntPoint::new(5, 0),
        };

        assert_eq!(v0.is_under_segment_order(&v1), Ordering::Greater);
    }
}
