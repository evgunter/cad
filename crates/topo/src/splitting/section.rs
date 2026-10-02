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
//! `plane_section` stops after the join — it shares the section-loop
//! reading (`section_loops`) with the finish, not the finish
//! itself — so it BYPASSES the single-solid gate: a multi-solid body is sliced whole — every solid
//! the plane crosses contributes regions, and they all land in one
//! `regions` vec (no per-solid attribution). This is deliberate for a
//! read-only query; [`super::split`] on the same body refuses typed
//! with `NotSingleSolid`.

use geom_core::{Indeterminate, Point2, Point3, Real, Vec3};

use super::join::loop_points_of;
use super::section_loops::{self, NestFault, SenseFault};
use super::{PlaneSide, SplitError, SplitPlane, SplitReduceError, split_scratch};
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

/// Typed failure of [`plane_section`].
#[derive(Debug)]
pub enum SectionError<T: Real> {
    /// The reduce or join stage refused, exactly as it does for
    /// [`super::split`] (a curved face's zero-area graze included).
    Split(SplitError),
    /// Whether a section polygon is an outline or a hole cannot be
    /// read: its winding is in the band (`diag`), or (`None`) zero, or
    /// unread because the polygon carries a spiric or NURBS edge.
    WindingUndecided {
        /// A corner of the polygon.
        corner: Point3<T>,
        /// The winding's diagnostic, when it escalated.
        diag: Option<Indeterminate>,
    },
    /// Nothing decides which outline encloses a hole — an outline edge
    /// on a spiric or NURBS carrier, or a contact or containment in the
    /// band ([`super::section_loops`]'s `nest` says what else could).
    /// The split keeps such a hole as a face of its own; a region list
    /// cannot state it.
    UnplacedHole {
        /// A corner of the hole.
        corner: Point3<T>,
    },
    /// Two outlines each read as enclosing the other around a hole
    /// (a kernel bug).
    NestingContradiction {
        /// A corner of the hole.
        corner: Point3<T>,
    },
    /// A traversal of the scratch body met a dangling key (a kernel
    /// bug).
    Corrupt,
}

impl<T: Real> From<SplitError> for SectionError<T> {
    fn from(e: SplitError) -> Self {
        Self::Split(e)
    }
}

impl<T: Real> core::fmt::Display for SectionError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Split(e) => write!(f, "{e}"),
            Self::WindingUndecided {
                diag: Some(diag), ..
            } => write!(
                f,
                "whether a polygon of the section is an outline or a hole is too close to \
                 call ({}). Recourse: {}",
                diag.payload(),
                super::SPLIT_COINCIDENCE_RECOURSE
            ),
            Self::WindingUndecided { diag: None, .. } => write!(
                f,
                "whether a polygon of the section is an outline or a hole cannot be read: it \
                 encloses no area, or has a spiric or NURBS edge, whose winding the kernel \
                 does not read. Recourse: move the section plane"
            ),
            Self::UnplacedHole { .. } => write!(
                f,
                "no outline of the section can be shown to enclose one of its holes: an \
                 outline edge the nesting cannot read (spiric or NURBS), a contact too close \
                 to call, or a sliver the section join mints across a curved face. Recourse: \
                 move the section plane"
            ),
            Self::NestingContradiction { .. } => write!(
                f,
                "two outlines of the section each read as enclosing the other around a hole. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::Corrupt => write!(
                f,
                "the section traversal failed (corrupt body). {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
        }
    }
}

impl<T: Real> std::error::Error for SectionError<T> {}

/// Computes the section regions of `operand` against `plane` without
/// building the result bodies (module docs).
///
/// # Winding contract
///
/// Each polygon's role is read from its winding about the plane normal,
/// decided on its edges' own carriers (`section_loops::loop_sense`,
/// the reading `split` gives its section faces' senses): an outline
/// winds counter-clockwise in `(u, v)`, a hole clockwise. Each hole is
/// placed in the outline that immediately encloses it by the section
/// nesting rule (`section_loops::nest`), which `split` nests its
/// holed section faces with.
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
/// [`SectionError`]: [`SectionError::Split`] passes the reduce and join
/// stages' refusals through unchanged — in particular a zero-area
/// section (a curved face's graze) REFUSES (`DegenerateSection`,
/// exactly as [`super::split`] does) rather than reporting a
/// degenerate trace.
pub fn plane_section<T: geom_core::Decide>(
    operand: &Body<T>,
    plane: &SplitPlane<T>,
    tol: Tol,
) -> Result<Section<T>, SectionError<T>> {
    let (red, completed, _fragments) = split_scratch(operand, plane, tol)?;
    let band = geom_core::Band::linear(tol)
        .map_err(|e| SectionError::Split(SplitError::Reduce(SplitReduceError::from(e))))?;
    // The below loops are read, so the frame is the below section
    // face's: its outward normal, and `u_ref × v_ref` equals it.
    let normal = section_loops::section_normal(plane.normal.get(), PlaneSide::Below);

    let mut u_ref = None;
    let mut v_ref = None;
    let mut outlines = Vec::new();
    let mut holes = Vec::new();
    for section in &completed {
        let points = loop_points_of(&red.body, section.below_loop).map_err(SplitError::Join)?;
        if u_ref.is_none() {
            u_ref = section_loops::chord_u_ref(&points);
            v_ref = u_ref.map(|u| normal.cross(u));
        }
        let (Some(u), Some(v)) = (u_ref, v_ref) else {
            return Err(
                SplitError::Join(crate::chord_join::SplitJoinError::SectionInvariant {
                    face: section.face,
                    what: "the section polygon has fewer than two points, so the in-plane \
                           frame it is reported in was never established",
                })
                .into(),
            );
        };
        let uv = points
            .iter()
            .map(|p| {
                let w = *p - plane.origin;
                Point2::new(w.dot(u), w.dot(v))
            })
            .collect();
        let polygon = SectionPolygon { points, uv };
        let corner = polygon.points[0];
        match section_loops::loop_sense(&red.body, section.below_loop, normal, band) {
            Ok(true) => outlines.push((polygon, section.below_loop)),
            Ok(false) => holes.push((polygon, section.below_loop)),
            Err(SenseFault::Torn) => return Err(SectionError::Corrupt),
            Err(SenseFault::Undecided(diag)) => {
                return Err(SectionError::WindingUndecided { corner, diag });
            }
        }
    }
    let nesting = section_loops::nest(&red.body, outlines, holes, normal, band).map_err(
        |fault| match fault {
            NestFault::Torn => SectionError::Corrupt,
            NestFault::Contradiction(hole) => SectionError::NestingContradiction {
                corner: hole.points[0],
            },
        },
    )?;
    if let Some(hole) = nesting.unplaced.first() {
        return Err(SectionError::UnplacedHole {
            corner: hole.points[0],
        });
    }
    Ok(Section {
        plane: *plane,
        u_ref,
        v_ref,
        regions: nesting
            .regions
            .into_iter()
            .map(|(outline, holes)| SectionRegion { outline, holes })
            .collect(),
    })
}
