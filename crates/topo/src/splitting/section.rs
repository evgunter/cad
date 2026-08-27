//! Slicing (ch. 14 §14.9): the **plane-section query** — the section
//! polygons a splitting plane cuts through a body, WITHOUT
//! constructing the result solids. Near-free from the join machinery:
//! the polygons exist as completed null faces the moment
//! `splitconnect` finishes; slicing reads them off the scratch clone
//! and discards it (our functional pipeline never mutates the operand,
//! so the book's "delete the inserted vertices to restore S" step
//! vanishes). The first real sectioning feature.
//!
//! # Gate asymmetry vs `split`
//!
//! `plane_section` never reaches the finish stage, so it BYPASSES the
//! single-solid gate: a multi-solid body is sliced whole — every solid
//! the plane crosses contributes polygons, and they all land in one
//! `polygons` vec (no per-solid attribution). This is deliberate for a
//! read-only query; [`super::split`] on the same body refuses typed
//! with `NotSingleSolid`.

use geom_core::{Point2, Point3, Real, Vec3};

use super::join::loop_points_of;
use super::{SplitError, SplitPlane, split_scratch};
use crate::body::Body;
use geom_core::Tol;

/// One section polygon: the closed vertex chain the plane cuts, in
/// below-loop cycle order — as 3-D points and as in-plane `(u, v)`
/// coordinates in the section's frame.
#[derive(Clone, Debug)]
pub struct SectionPolygon<T: Real> {
    /// The corner points, in chain order (closed: last connects to
    /// first).
    pub points: Vec<Point3<T>>,
    /// The same corners in split-plane coordinates:
    /// `u = (p − origin)·u_ref`, `v = (p − origin)·v_ref`.
    pub uv: Vec<Point2<T>>,
}

/// A plane section: the frame and the polygons (empty when the plane
/// misses the body — a typed success, not an error).
#[derive(Clone, Debug)]
pub struct Section<T: Real> {
    /// The sectioning plane, as given.
    pub plane: SplitPlane<T>,
    /// The in-plane u axis (unit; derived from the first polygon's
    /// first chord — deterministic data; zero polygons ⇒ a default
    /// axis is impossible, so `u_ref`/`v_ref` are `None`).
    pub u_ref: Option<Vec3<T>>,
    /// The in-plane v axis (`normal × u_ref`).
    pub v_ref: Option<Vec3<T>>,
    /// The section polygons, in completion order.
    pub polygons: Vec<SectionPolygon<T>>,
}

/// Computes the section polygons of `operand` against `plane` without
/// building the result bodies (module docs).
///
/// # Winding contract
///
/// Polygons are consistently CCW in `(u, v)`: each polygon's signed
/// area (shoelace over `uv`) is positive. Consumers computing signed
/// areas or offsets may rely on this orientation.
///
/// # Frame semantics
///
/// `u_ref` is the normalized first chord of the first polygon's
/// below loop (deterministic data, not an arbitrary axis);
/// `v_ref = normal × u_ref`. Both are `None` iff `polygons` is empty
/// (the plane misses the body — a typed success). `uv` coordinates
/// are `((p − origin)·u_ref, (p − origin)·v_ref)`.
///
/// # Errors
///
/// [`SplitError`] — the reduce/join stages' typed refusals pass
/// through unchanged: in particular a pure-tangency section REFUSES
/// (`DegenerateSection`, exactly as [`super::split`] does) rather
/// than reporting a degenerate zero-area trace.
pub fn plane_section<T: geom_core::Decide>(
    operand: &Body<T>,
    plane: &SplitPlane<T>,
    tol: Tol,
) -> Result<Section<T>, SplitError> {
    let (red, completed, _fragments) = split_scratch(operand, plane, tol)?;

    let mut u_ref = None;
    let mut v_ref = None;
    let mut polygons = Vec::with_capacity(completed.len());
    for section in &completed {
        let points = loop_points_of(&red.body, section.below_loop).map_err(SplitError::Join)?;
        if u_ref.is_none() && points.len() >= 2 {
            let u = (points[1] - points[0]).normalize();
            v_ref = Some(plane.normal.cross(u));
            u_ref = Some(u);
        }
        let (u, v) = match (u_ref, v_ref) {
            (Some(u), Some(v)) => (u, v),
            _ => {
                return Err(SplitError::Join(
                    crate::chord_join::SplitJoinError::SectionInvariant {
                        face: section.face,
                        what: "the section polygon has fewer than two points, so the in-plane \
                               frame it is reported in was never established",
                    },
                ));
            }
        };
        let uv = points
            .iter()
            .map(|p| {
                let w = *p - plane.origin;
                Point2::new(w.dot(u), w.dot(v))
            })
            .collect();
        polygons.push(SectionPolygon { points, uv });
    }
    Ok(Section {
        plane: *plane,
        u_ref,
        v_ref,
        polygons,
    })
}
