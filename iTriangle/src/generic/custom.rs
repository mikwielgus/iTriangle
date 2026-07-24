use crate::generic::adapter::PointAdapter;
use crate::generic::triangulation::RawTriangulation;
use crate::int::custom::IntCustomTriangulatable;
use crate::int::triangulation::RawIntTriangulation;
use crate::int::validation::Validation;
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

/// A trait for triangulating float geometry with user-defined validation rules.
///
/// Accepts a custom [`Validation`] object for tuning fill rule, min area, etc.
pub trait CustomTriangulatable<P: FloatPointCompatible> {
    /// Performs triangulation using the specified [`Validation`] settings.
    fn custom_triangulate(
        &self,
        validation: Validation<i32>,
    ) -> RawTriangulation<FloatPointAdapter<P, i32>> {
        self.custom_triangulate_as(validation)
    }

    /// Performs triangulation using the requested integer coordinate type.
    fn custom_triangulate_as<I>(
        &self,
        validation: Validation<I>,
    ) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + LayoutNumber + SortKey;

    /// Performs triangulation with Steiner points and a custom [`Validation`] config.
    fn custom_triangulate_with_steiner_points(
        &self,
        points: &[P],
        validation: Validation<i32>,
    ) -> RawTriangulation<FloatPointAdapter<P, i32>> {
        self.custom_triangulate_with_steiner_points_as(points, validation)
    }

    /// Performs triangulation with Steiner points using the requested integer coordinate type.
    fn custom_triangulate_with_steiner_points_as<I>(
        &self,
        points: &[P],
        validation: Validation<I>,
    ) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + LayoutNumber + SortKey;
}

impl<P> CustomTriangulatable<P> for Contour<P>
where
    P: FloatPointCompatible,
{
    fn custom_triangulate_as<I>(
        &self,
        validation: Validation<I>,
    ) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + LayoutNumber + SortKey,
    {
        if let Some(rect) = FloatRect::with_path(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_contour: IntContour<I> = adapter.points_to_int(self);
            let raw = int_contour.custom_triangulate(validation);
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }

    fn custom_triangulate_with_steiner_points_as<I>(
        &self,
        points: &[P],
        validation: Validation<I>,
    ) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + LayoutNumber + SortKey,
    {
        if let Some(rect) = FloatRect::with_path(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_points = adapter.points_to_int(points);
            let int_contour: IntContour<I> = adapter.points_to_int(self);
            let raw = int_contour.custom_triangulate_with_steiner_points(&int_points, validation);
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }
}

impl<P> CustomTriangulatable<P> for [Contour<P>]
where
    P: FloatPointCompatible,
{
    fn custom_triangulate_as<I>(
        &self,
        validation: Validation<I>,
    ) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + LayoutNumber + SortKey,
    {
        if let Some(rect) = FloatRect::with_paths(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_shape: IntShape<I> = self.iter().map(|c| adapter.points_to_int(c)).collect();
            let raw = int_shape.custom_triangulate(validation);
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }

    fn custom_triangulate_with_steiner_points_as<I>(
        &self,
        points: &[P],
        validation: Validation<I>,
    ) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + LayoutNumber + SortKey,
    {
        if let Some(rect) = FloatRect::with_paths(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_points = adapter.points_to_int(points);
            let int_shape: IntShape<I> = self.iter().map(|c| adapter.points_to_int(c)).collect();
            let raw = int_shape.custom_triangulate_with_steiner_points(&int_points, validation);
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }
}

impl<P> CustomTriangulatable<P> for [Shape<P>]
where
    P: FloatPointCompatible,
{
    fn custom_triangulate_as<I>(
        &self,
        validation: Validation<I>,
    ) -> RawTriangulation<FloatPointAdapter<P, I>>
    where
        I: IntNumber + Expiration + LayoutNumber + SortKey,
    {
        if let Some(rect) = FloatRect::with_list_of_paths(self) {
            let adapter = FloatPointAdapter::<P, I>::new(rect);
            let int_shapes: IntShapes<I> = self
                .iter()
                .map(|shape| shape.iter().map(|c| adapter.points_to_int(c)).collect())
                .collect();
            let raw = int_shapes.custom_triangulate(validation);
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }

    fn custom_triangulate_with_steiner_points_as<I>(
        &self,
        points: &[P],
        validation: Validation<I>,
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
            let raw = int_shapes.custom_triangulate_with_steiner_points(&int_points, validation);
            RawTriangulation { raw, adapter }
        } else {
            RawTriangulation {
                raw: RawIntTriangulation::default(),
                adapter: FloatPointAdapter::<P, I>::new(FloatRect::zero()),
            }
        }
    }
}
