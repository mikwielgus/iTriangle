use crate::int::binder::SteinerInference;
use crate::int::solver::{ContourSolver, ShapeSolver, ShapesSolver};
use crate::int::triangulation::RawIntTriangulation;
use i_key_sort::sort::key::SortKey;
use i_overlay::i_float::int::number::int::IntNumber;
use i_overlay::i_float::int::point::IntPoint;
use i_overlay::i_shape::int::shape::{IntContour, IntShape, IntShapes};
use i_tree::Expiration;

/// A trait for performing triangulation on already validated geometry.
///
/// Skips all shape simplification and orientation checks for maximum performance.
/// Useful when the input is known to be valid (e.g., preprocessed or generated data).
///
/// # ⚠️ Safety Requirements
/// - Outer contours must be in **counter-clockwise** order
/// - Holes must be in **clockwise** order
/// - Shapes must not have self-intersections
/// - Steiner points must be **strictly inside** the shape
///
/// # Implemented For
/// - [`IntContour`]
/// - [`IntShape`]
/// - [`IntShapes`]
pub trait IntUncheckedTriangulatable<I: IntNumber> {
    /// Performs triangulation without applying any shape simplification or validation.
    fn uncheck_triangulate(&self) -> RawIntTriangulation<I>;

    /// Performs triangulation without validation, inserting the given Steiner points.
    ///
    /// Points are grouped and applied based on their target shape.
    fn uncheck_triangulate_with_steiner_points(
        &self,
        points: &[IntPoint<I>],
    ) -> RawIntTriangulation<I>;
}

impl<I: IntNumber + SortKey> IntUncheckedTriangulatable<I> for IntContour<I> {
    #[inline]
    fn uncheck_triangulate(&self) -> RawIntTriangulation<I> {
        ContourSolver::uncheck_triangulate(self)
    }

    #[inline]
    fn uncheck_triangulate_with_steiner_points(
        &self,
        points: &[IntPoint<I>],
    ) -> RawIntTriangulation<I> {
        ContourSolver::uncheck_triangulate_with_steiner_points(self, points)
    }
}

impl<I: IntNumber + SortKey> IntUncheckedTriangulatable<I> for IntShape<I> {
    #[inline]
    fn uncheck_triangulate(&self) -> RawIntTriangulation<I> {
        ShapeSolver::uncheck_triangulate(self)
    }

    #[inline]
    fn uncheck_triangulate_with_steiner_points(
        &self,
        points: &[IntPoint<I>],
    ) -> RawIntTriangulation<I> {
        ShapeSolver::uncheck_triangulate_with_steiner_points(self, points)
    }
}

impl<I: IntNumber + Expiration + SortKey> IntUncheckedTriangulatable<I> for IntShapes<I> {
    #[inline]
    fn uncheck_triangulate(&self) -> RawIntTriangulation<I> {
        ShapesSolver::uncheck_triangulate(self)
    }

    #[inline]
    fn uncheck_triangulate_with_steiner_points(
        &self,
        points: &[IntPoint<I>],
    ) -> RawIntTriangulation<I> {
        let group = self.group_by_shapes(points);
        ShapesSolver::uncheck_triangulate_with_steiner_points(self, &group)
    }
}
