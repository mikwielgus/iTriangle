use crate::generic::delaunay::Delaunay;
use i_overlay::i_float::adapter::FloatPointAdapter;
use i_overlay::i_float::float::compatible::FloatPointCompatible;
use i_overlay::i_float::int::number::int::IntNumber;
use i_overlay::i_float::int::number::wide_int::WideIntNumber;

impl<P: FloatPointCompatible, I: IntNumber> Delaunay<FloatPointAdapter<P, I>> {
    #[inline]
    pub fn refine_with_circumcenters(mut self, min_area: P::Scalar) -> Self {
        self.refine_with_circumcenters_mut(min_area);
        self
    }

    #[inline]
    pub fn refine_with_circumcenters_by_obtuse_angle(mut self, min_area: P::Scalar) -> Self {
        self.refine_with_circumcenters_by_obtuse_angle_mut(min_area);
        self
    }

    #[inline]
    pub fn refine_with_circumcenters_mut(&mut self, min_area: P::Scalar) {
        let int_area = self.adapter.round_sqr_len_to_int(min_area);
        self.delaunay
            .refine_with_circumcenters_mut(int_area.to_uint());
    }

    #[inline]
    pub fn refine_with_circumcenters_by_obtuse_angle_mut(&mut self, min_area: P::Scalar) {
        let int_area = self.adapter.round_sqr_len_to_int(min_area);
        self.delaunay
            .refine_with_circumcenters_by_obtuse_angle_mut(int_area.to_uint());
    }
}
