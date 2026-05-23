use crate::float::delaunay::Delaunay;
use alloc::vec::Vec;
use i_overlay::i_float::float::compatible::FloatPointCompatible;
use i_overlay::i_float::int::number::int::IntNumber;
use i_overlay::i_float::int::number::wide_int::WideIntNumber;
use i_overlay::i_shape::base::data::Contour;
use i_overlay::i_shape::float::adapter::ShapeToFloat;

impl<P: FloatPointCompatible, I: IntNumber> Delaunay<P, I> {
    #[inline]
    pub fn to_centroid_net(&self, min_area: P::Scalar) -> Vec<Contour<P>> {
        let int_area = self.adapter.round_sqr_len_to_int(min_area);
        self.delaunay
            .centroid_net(int_area.to_uint())
            .to_float(&self.adapter)
    }
}
