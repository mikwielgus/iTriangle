use crate::advanced::delaunay::IntDelaunay;
use crate::int::triangulation::{IndexType, IntTriangulation};
use alloc::vec::Vec;
use i_overlay::i_float::int::number::int::IntNumber;
use i_overlay::i_float::int::point::IntPoint;

impl<I: IntNumber> IntDelaunay<I> {
    /// Returns the vertex positions in the triangulation.
    #[inline]
    pub fn points(&self) -> &Vec<IntPoint<I>> {
        &self.points
    }

    /// Returns indices forming counter-clockwise triangles.
    #[inline]
    pub fn triangle_indices<N: IndexType>(&self) -> Vec<N> {
        let mut result = Vec::with_capacity(3 * self.triangles.len());
        for t in &self.triangles {
            let v = &t.vertices;
            let i0 = N::try_from(v[0].index).unwrap_or(N::ZERO);
            let i1 = N::try_from(v[1].index).unwrap_or(N::ZERO);
            let i2 = N::try_from(v[2].index).unwrap_or(N::ZERO);

            result.extend_from_slice(&[i0, i1, i2]);
        }
        result
    }

    /// Returns the indices of each triangle's neighboring triangles.
    #[inline]
    pub fn triangle_neighbors(&self) -> Vec<[usize; 3]> {
        self.triangles
            .iter()
            .map(|triangle| triangle.neighbors)
            .collect()
    }

    #[inline]
    pub fn into_triangulation<N: IndexType>(self) -> IntTriangulation<I, N> {
        IntTriangulation {
            indices: self.triangle_indices(),
            points: self.points,
        }
    }
}
