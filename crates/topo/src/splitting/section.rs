//! Slicing (ch. 14 §14.9): the **plane-section query** — the section
//! polygons a splitting plane cuts through a body, WITHOUT
//! constructing the result solids. Near-free from the join machinery:
//! the polygons exist as completed null faces the moment
//! `splitconnect` finishes; slicing reads them off the scratch clone,
//! groups each hole with the outline around it by the rule the finish
//! nests holes with, and discards the clone (our functional pipeline never mutates the operand,
//! so the book's "delete the inserted vertices to restore S" step
//! vanishes). The first real sectioning feature.
//!
//! # Gate asymmetry vs `split`
//!
//! `plane_section` never reaches the finish stage, so it BYPASSES the
//! single-solid gate: a multi-solid body is sliced whole — every solid
//! the plane crosses contributes regions, and they all land in one
//! `regions` vec (no per-solid attribution). This is deliberate for a
//! read-only query; [`super::split`] on the same body refuses typed
//! with `NotSingleSolid`.

use geom_core::{Point2, Point3, Real, Vec3};

use super::finish::{nest_holes, section_sense};
use super::join::loop_points_of;
use super::{SplitError, SplitPlane, split_scratch};
use crate::body::Body;
use geom_core::Tol;

/// One section polygon: the closed vertex chain the plane cuts — as
/// 3-D points and as in-plane `(u, v)` coordinates in the section's
/// frame. An edge between two corners may be an arc (the bore through
/// a brick cuts a circle with two corners), so the corners alone need
/// not enclose the polygon's area.
#[derive(Clone, Debug)]
pub struct SectionPolygon<T: Real> {
    /// The corner points, in chain order (closed: last connects to
    /// first).
    pub points: Vec<Point3<T>>,
    /// The same corners in split-plane coordinates:
    /// `u = (p − origin)·u_ref`, `v = (p − origin)·v_ref`.
    pub uv: Vec<Point2<T>>,
}

/// One connected piece of a section: an outline and the holes inside
/// it. The outline winds counter-clockwise in `(u, v)`, each hole
/// clockwise, so the material lies to the left of every polygon.
#[derive(Clone, Debug)]
pub struct SectionRegion<T: Real> {
    /// The region's outer boundary.
    pub outline: SectionPolygon<T>,
    /// The holes the outline immediately encloses (an island inside a
    /// hole is a region of its own).
    pub holes: Vec<SectionPolygon<T>>,
}

/// A plane section: the frame and the regions (empty when the plane
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
    /// The section's regions, each outline and each region's holes in
    /// completion order.
    pub regions: Vec<SectionRegion<T>>,
}

/// Computes the section regions of `operand` against `plane` without
/// building the result bodies (module docs).
///
/// # Winding contract
///
/// Each polygon's role is read from its winding about the plane normal,
/// decided on its edges' own carriers (the reading `split` gives its
/// section faces' senses): an outline winds counter-clockwise in
/// `(u, v)`, a hole clockwise. Each hole is placed in the outline that
/// immediately encloses it by the nesting rule `split` nests its holed
/// section faces with.
///
/// # Frame semantics
///
/// `u_ref` is the normalized first chord of the first polygon's
/// below loop (deterministic data, not an arbitrary axis);
/// `v_ref = normal × u_ref`. Both are `None` iff `regions` is empty
/// (the plane misses the body — a typed success). `uv` coordinates
/// are `((p − origin)·u_ref, (p − origin)·v_ref)`.
///
/// # Errors
///
/// [`SplitError`] — the reduce/join stages' typed refusals pass
/// through unchanged: in particular a pure-tangency section REFUSES
/// (`DegenerateSection`, exactly as [`super::split`] does) rather
/// than reporting a degenerate zero-area trace. A polygon whose winding
/// has no sign, or a hole the nesting rule cannot place, refuses
/// ([`SplitError::Finish`], [`SplitError::UnplacedHole`]).
pub fn plane_section<T: geom_core::Decide>(
    operand: &Body<T>,
    plane: &SplitPlane<T>,
    tol: Tol,
) -> Result<Section<T>, SplitError> {
    let (red, completed, _fragments) = split_scratch(operand, plane, tol)?;
    let band = geom_core::Band::linear(tol).map_err(super::SplitFinishError::Band)?;

    let mut u_ref = None;
    let mut v_ref = None;
    let mut outlines = Vec::new();
    let mut holes = Vec::new();
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
        let polygon = SectionPolygon { points, uv };
        // The below side's section face wears `+normal` (the split's
        // finish), and `u_ref × v_ref = normal`.
        let entry = ((section.face, section.below_loop), polygon);
        if section_sense(
            &red.body,
            section.face,
            section.below_loop,
            plane.normal,
            band,
        )? {
            outlines.push(entry);
        } else {
            holes.push(entry);
        }
    }
    let outline_loops: Vec<_> = outlines.iter().map(|&((_, l), _)| l).collect();
    let hole_loops: Vec<_> = holes.iter().map(|&(k, _)| k).collect();
    let parents = nest_holes(&red.body, &outline_loops, &hole_loops, plane.normal, band)?;
    let mut regions: Vec<_> = outlines
        .into_iter()
        .map(|(_, outline)| SectionRegion {
            outline,
            holes: Vec::new(),
        })
        .collect();
    for (((face, _), hole), parent) in holes.into_iter().zip(parents) {
        let parent = parent.ok_or(SplitError::UnplacedHole { face })?;
        regions[parent].holes.push(hole);
    }
    Ok(Section {
        plane: *plane,
        u_ref,
        v_ref,
        regions,
    })
}
