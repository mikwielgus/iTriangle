use crate::generic::adapter::PointAdapter;
use crate::generic::triangulation::RawTriangulation;
use crate::int::triangulatable::IntTriangulatable;
use crate::int::triangulation::RawIntTriangulation;
use alloc::vec::Vec;
use i_key_sort::sort::key::SortKey;
use i_overlay::i_float::adapter::FloatPointAdapter;
use i_overlay::i_float::float::compatible::FloatPointCompatible;
use i_overlay::i_float::float::rect::FloatRect;
use i_overlay::i_float::int::number::int::IntNumber;
use i_overlay::i_shape::base::data::{Contour, Shape};
use i_overlay::i_shape::float::rect::RectInit;
use i_overlay::i_shape::int::shape::{IntContour, IntShape, IntShapes};
use i_tree::{Expiration, LayoutNumber};

/// A trait for triangulating float-based geometry with default validation.
///
/// Automatically converts the input to integer space, applies validation,
/// and returns a float-mapped result.
///
/// # Implemented For
/// - `Contour<P>`
/// - `[Contour<P>]`
/// - `[Shape<P>]`
pub trait Triangulatable<P: FloatPointCompatible> {
    /// Triangulates the shape(s) using the default [`Triangulator`] configuration.
    ///
    /// Validation includes contour simplification, direction correction, and area filtering.
    fn triangulate(&self) -> RawTriangulation<FloatPointAdapter<P, i32>> {
        self.triangulate_as::<i32>()
    }

    /// Triangulates the shape(s) using the requested integer coordinate type.
    fn triangulate_as<I>(&self) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + LayoutNumber + SortKey;

    /// Triangulates the shape(s) and inserts the given Steiner points.
    ///
    /// Points must lie strictly within the interior of the geometry.
    fn triangulate_with_steiner_points(
        &self,
        points: &[P],
    ) -> RawTriangulation<FloatPointAdapter<P, i32>> {
        self.triangulate_with_steiner_points_as::<i32>(points)
    }

    /// Triangulates the shape(s) with Steiner points using the requested integer coordinate type.
    fn triangulate_with_steiner_points_as<I>(
        &self,
        points: &[P],
    ) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + LayoutNumber + SortKey;
}

impl<P> Triangulatable<P> for [P]
where
    P: FloatPointCompatible,
{
    fn triangulate_as<I>(&self) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + LayoutNumber + SortKey,
    {
        if let Some(rect) = FloatRect::with_path(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_contour: IntContour<I> = adapter.points_to_int(self);
            let raw = int_contour.triangulate();
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }

    fn triangulate_with_steiner_points_as<I>(
        &self,
        points: &[P],
    ) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + LayoutNumber + SortKey,
    {
        if let Some(rect) = FloatRect::with_path(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_points = adapter.points_to_int(points);
            let int_contour: IntContour<I> = adapter.points_to_int(self);
            let raw = int_contour.triangulate_with_steiner_points(&int_points);
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }
}

impl<P> Triangulatable<P> for [Contour<P>]
where
    P: FloatPointCompatible,
{
    fn triangulate_as<I>(&self) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + LayoutNumber + SortKey,
    {
        if let Some(rect) = FloatRect::with_paths(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_shape: IntShape<I> = self.iter().map(|c| adapter.points_to_int(c)).collect();
            let raw = int_shape.triangulate();
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }

    fn triangulate_with_steiner_points_as<I>(
        &self,
        points: &[P],
    ) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + LayoutNumber + SortKey,
    {
        if let Some(rect) = FloatRect::with_paths(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_points = adapter.points_to_int(points);
            let int_shape: IntShape<I> = self.iter().map(|c| adapter.points_to_int(c)).collect();
            let raw = int_shape.triangulate_with_steiner_points(&int_points);
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }
}

impl<P> Triangulatable<P> for [Shape<P>]
where
    P: FloatPointCompatible,
{
    fn triangulate_as<I>(&self) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + LayoutNumber + SortKey,
    {
        if let Some(rect) = FloatRect::with_list_of_paths(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_shapes: IntShapes<I> = self
                .iter()
                .map(|shape| shape.iter().map(|c| adapter.points_to_int(c)).collect())
                .collect();
            let raw = int_shapes.triangulate();
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }

    fn triangulate_with_steiner_points_as<I>(
        &self,
        points: &[P],
    ) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + LayoutNumber + SortKey,
    {
        if let Some(rect) = FloatRect::with_list_of_paths(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_points = adapter.points_to_int(points);
            let int_shapes: IntShapes<I> = self
                .iter()
                .map(|shape| shape.iter().map(|c| adapter.points_to_int(c)).collect())
                .collect();
            let raw = int_shapes.triangulate_with_steiner_points(&int_points);
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }
}
