use crate::int::meta::TrianglesCount;
use crate::int::monotone::chain::builder::ChainBuilder;
use crate::int::monotone::chain::vertex::ChainVertex;
use crate::int::monotone::flat::triangulator::FlatTriangulation;
use crate::int::monotone::net::triangulator::NetTriangulation;
use crate::int::triangulation::{IndexType, IntTriangulation, RawIntTriangulation};
use alloc::vec::Vec;
use i_key_sort::sort::key::SortKey;
use i_overlay::i_float::int::number::int::IntNumber;
use i_overlay::i_float::int::point::IntPoint;
use i_overlay::i_shape::flat::buffer::FlatContoursBuffer;
use i_overlay::i_shape::int::shape::{IntContour, IntShape};

pub(crate) struct MonotoneTriangulator<I: IntNumber + SortKey> {
    vertices: Option<Vec<ChainVertex<I>>>,
}

impl<I: IntNumber + SortKey> Default for MonotoneTriangulator<I> {
    fn default() -> Self {
        Self { vertices: None }
    }
}

impl<I: IntNumber + SortKey> MonotoneTriangulator<I> {
    #[inline]
    pub(crate) fn shape_into_net_triangulation(
        &mut self,
        shape: &IntShape<I>,
        points: Option<&[IntPoint<I>]>,
        triangulation: &mut RawIntTriangulation<I>,
    ) {
        let points_count = points.map(|points| points.len()).unwrap_or(0);

        let mut vertices = self.vertices.take().unwrap_or_default();
        ChainBuilder::shape_to_vertices(shape, points, &mut vertices);

        vertices.net_triangulate_into(shape.triangles_count(points_count), triangulation);

        self.vertices = Some(vertices);
    }

    #[inline]
    pub(crate) fn contour_into_net_triangulation(
        &mut self,
        contour: &IntContour<I>,
        points: Option<&[IntPoint<I>]>,
        triangulation: &mut RawIntTriangulation<I>,
    ) {
        let points_count = points.map(|points| points.len()).unwrap_or(0);

        let mut vertices = self.vertices.take().unwrap_or_default();
        ChainBuilder::contour_to_vertices(contour, points, &mut vertices);

        vertices.net_triangulate_into(contour.triangles_count(points_count), triangulation);

        self.vertices = Some(vertices);
    }

    #[inline]
    pub(crate) fn flat_into_net_triangulation(
        &mut self,
        flat: &FlatContoursBuffer<I>,
        triangulation: &mut RawIntTriangulation<I>,
    ) {
        let mut vertices = self.vertices.take().unwrap_or_default();
        ChainBuilder::flat_to_vertices(flat, &mut vertices);

        vertices.net_triangulate_into(flat.triangles_count(0), triangulation);

        self.vertices = Some(vertices);
    }

    #[inline]
    pub(crate) fn shape_into_flat_triangulation<N: IndexType>(
        &mut self,
        shape: &IntShape<I>,
        triangulation: &mut IntTriangulation<I, N>,
    ) {
        let mut vertices = self.vertices.take().unwrap_or_default();
        ChainBuilder::shape_to_vertices(shape, None, &mut vertices);

        vertices.flat_triangulate_into(shape.triangles_count(0), triangulation);

        self.vertices = Some(vertices);
    }

    #[inline]
    pub(crate) fn contour_into_flat_triangulation<N: IndexType>(
        &mut self,
        contour: &IntContour<I>,
        triangulation: &mut IntTriangulation<I, N>,
    ) {
        let mut vertices = self.vertices.take().unwrap_or_default();
        ChainBuilder::contour_to_vertices(contour, None, &mut vertices);

        vertices.flat_triangulate_into(contour.triangles_count(0), triangulation);

        self.vertices = Some(vertices);
    }

    #[inline]
    pub(crate) fn flat_into_flat_triangulation<N: IndexType>(
        &mut self,
        flat: &FlatContoursBuffer<I>,
        triangulation: &mut IntTriangulation<I, N>,
    ) {
        let mut vertices = self.vertices.take().unwrap_or_default();
        ChainBuilder::flat_to_vertices(flat, &mut vertices);

        vertices.flat_triangulate_into(flat.triangles_count(0), triangulation);

        self.vertices = Some(vertices);
    }
}
