use crate::generic::adapter::PointAdapter;
use crate::generic::triangulation::RawTriangulation;
use crate::int::triangulation::RawIntTriangulation;
use crate::int::unchecked::IntUncheckedTriangulatable;
use alloc::vec::Vec;
use i_key_sort::sort::key::SortKey;
use i_overlay::i_float::adapter::FloatPointAdapter;
use i_overlay::i_float::float::compatible::FloatPointCompatible;
use i_overlay::i_float::float::rect::FloatRect;
use i_overlay::i_float::int::number::int::IntNumber;
use i_overlay::i_shape::base::data::{Contour, Shape};
use i_overlay::i_shape::float::rect::RectInit;
use i_overlay::i_shape::int::shape::{IntContour, IntShape, IntShapes};
use i_tree::Expiration;

/// A trait for triangulating already valid float-based geometry.
///
/// Skips all validation for performance. Ideal when input is generated programmatically.
///
/// # Safety Requirements
/// - Outer contours must be counter-clockwise
/// - Holes must be clockwise
/// - Steiner points must lie strictly within the shape
pub trait UncheckedTriangulatable<P: FloatPointCompatible> {
    /// Triangulates float geometry without validation or simplification.
    fn unchecked_triangulate(&self) -> RawTriangulation<FloatPointAdapter<P, i32>> {
        self.unchecked_triangulate_as::<i32>()
    }

    /// Triangulates float geometry without validation using the requested integer coordinate type.
    fn unchecked_triangulate_as<I>(&self) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + SortKey;

    /// Same as `unchecked_triangulate`, but inserts user-defined Steiner points.
    fn unchecked_triangulate_with_steiner_points(
        &self,
        points: &[P],
    ) -> RawTriangulation<FloatPointAdapter<P, i32>> {
        self.unchecked_triangulate_with_steiner_points_as::<i32>(points)
    }

    /// Same as `unchecked_triangulate_as`, but inserts user-defined Steiner points.
    fn unchecked_triangulate_with_steiner_points_as<I>(
        &self,
        points: &[P],
    ) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + SortKey;
}

impl<P> UncheckedTriangulatable<P> for [P]
where
    P: FloatPointCompatible,
{
    fn unchecked_triangulate_as<I>(&self) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + SortKey,
    {
        if let Some(rect) = FloatRect::with_path(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_contour: IntContour<I> = adapter.points_to_int(self);
            let raw = int_contour.uncheck_triangulate();
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }

    fn unchecked_triangulate_with_steiner_points_as<I>(
        &self,
        points: &[P],
    ) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + SortKey,
    {
        if let Some(rect) = FloatRect::with_path(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_points = adapter.points_to_int(points);
            let int_contour: IntContour<I> = adapter.points_to_int(self);
            let raw = int_contour.uncheck_triangulate_with_steiner_points(&int_points);
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }
}

impl<P> UncheckedTriangulatable<P> for [Contour<P>]
where
    P: FloatPointCompatible,
{
    fn unchecked_triangulate_as<I>(&self) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + SortKey,
    {
        if let Some(rect) = FloatRect::with_paths(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_shape: IntShape<I> = self.iter().map(|c| adapter.points_to_int(c)).collect();
            let raw = int_shape.uncheck_triangulate();
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }

    fn unchecked_triangulate_with_steiner_points_as<I>(
        &self,
        points: &[P],
    ) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + SortKey,
    {
        if let Some(rect) = FloatRect::with_paths(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_points = adapter.points_to_int(points);
            let int_shape: IntShape<I> = self.iter().map(|c| adapter.points_to_int(c)).collect();
            let raw = int_shape.uncheck_triangulate_with_steiner_points(&int_points);
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }
}

impl<P> UncheckedTriangulatable<P> for [Shape<P>]
where
    P: FloatPointCompatible,
{
    fn unchecked_triangulate_as<I>(&self) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + SortKey,
    {
        if let Some(rect) = FloatRect::with_list_of_paths(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_shapes: IntShapes<I> = self
                .iter()
                .map(|shape| shape.iter().map(|c| adapter.points_to_int(c)).collect())
                .collect();
            let raw = int_shapes.uncheck_triangulate();
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }

    fn unchecked_triangulate_with_steiner_points_as<I>(
        &self,
        points: &[P],
    ) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + SortKey,
    {
        if let Some(rect) = FloatRect::with_list_of_paths(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_points = adapter.points_to_int(points);
            let int_shapes: IntShapes<I> = self
                .iter()
                .map(|shape| shape.iter().map(|c| adapter.points_to_int(c)).collect())
                .collect();
            let raw = int_shapes.uncheck_triangulate_with_steiner_points(&int_points);
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }
}
