//! Slicing (ch. 14 §14.9): the **plane-section query** — the section
//! polygons a splitting plane cuts through a body, WITHOUT
//! constructing the result solids. Near-free from the join machinery:
//! the polygons exist as completed null faces the moment
//! `splitconnect` finishes; slicing reads them off the scratch clone,
//! takes out of each outline the slits where a piece on one side
//! touches itself along the plane (`drop_slits`; the section's region
//! runs on across them), groups each hole with the outline around it by
//! the rule the finish nests holes with, and discards the clone (our functional pipeline never mutates the operand,
//! so the book's "delete the inserted vertices to restore S" step
//! vanishes). The first real sectioning feature.
//!
//! # A multi-solid body
//!
//! `plane_section` stops after the join — it shares the section-loop
//! reading (`section_loops`) with the finish, not the finish
//! itself — so a multi-solid body is sliced whole: every solid the
//! plane crosses contributes regions, and they all land in one
//! `regions` vec (no per-solid attribution). This is deliberate for a
//! read-only query; [`super::split`] on the same body splits it whole
//! too and sorts each side into solids.

use geom_core::{Decide, Indeterminate, Point2, Point3, Real, Vec2, Vec3};

use super::join::SectionKind;
use super::section_loops::{self, NestFault, SenseFault};
use super::{KnifeEdge, PlaneSide, SplitError, SplitPlane, SplitReduceError, split_scratch};
use crate::body::Body;
use crate::entity::{HalfEdgeKey, LoopBoundary, LoopKey};
use geom_brep::props::chord_bulge;
use geom_core::Tol;

/// The lever every section refusal names: a section takes no
/// declarations, and its plane is the caller's.
const SECTION_PLANE_RECOURSE: &str = "move the section plane";

/// One section polygon: the closed boundary the plane cuts, by its
/// corners — as 3-D points and as in-plane `(u, v)` coordinates in the
/// section's frame — and by the edge from each corner to the next, on
/// its own carrier. An edge may be an arc (the bore through a brick
/// cuts a circle with two corners), so the corners alone need not
/// enclose the polygon's area; [`SectionPolygon::area`] reads it on the
/// edges. Only [`plane_section`] builds one, from one walk of the
/// section loop with its slits taken out (where a piece on one side
/// touches itself along an edge in the plane, straight or a conic
/// arc), so there is exactly one edge per corner and no corner is
/// visited twice.
#[derive(Clone, Debug)]
pub struct SectionPolygon<T: Real> {
    points: Vec<Point3<T>>,
    uv: Vec<Point2<T>>,
    edges: Vec<SectionEdge<T>>,
}

/// One edge of a [`SectionPolygon`], in the section's `(u, v)` frame.
/// Its ends are the polygon's corners.
#[derive(Clone, Copy, Debug)]
pub enum SectionEdge<T: Real> {
    /// A straight segment: the chord between its ends.
    Line,
    /// An arc of the conic `c(θ) = center + a·cos θ + b·sin θ`, run from
    /// `θ = start` to `θ = end` — `end < start` when it runs against its
    /// carrier's parameter. A circle's `a` and `b` are perpendicular,
    /// each of the radius's length; an ellipse's are its semi-axes. The
    /// conic is the section plane's own cut through a face, so it lies
    /// in the plane and its semi-axes chart without distortion.
    Arc {
        /// The conic's centre.
        center: Point2<T>,
        /// The conic's `θ = 0` semi-axis.
        a: Vec2<T>,
        /// The conic's `θ = π/2` semi-axis.
        b: Vec2<T>,
        /// The parameter at the edge's first end.
        start: T,
        /// The parameter at the edge's second end.
        end: T,
    },
}

impl<T: Real> SectionPolygon<T> {
    /// The corner points, in chain order (closed: the last connects to
    /// the first).
    pub fn points(&self) -> &[Point3<T>] {
        &self.points
    }

    /// The same corners in split-plane coordinates:
    /// `u = (p − origin)·u_ref`, `v = (p − origin)·v_ref`.
    pub fn uv(&self) -> &[Point2<T>] {
        &self.uv
    }

    /// The edges, one per corner: `edges()[i]` runs from corner `i` to
    /// corner `i + 1` (the last back to the first).
    pub fn edges(&self) -> &[SectionEdge<T>] {
        &self.edges
    }

    /// The signed area the polygon encloses in `(u, v)`: positive for
    /// an outline (counter-clockwise), negative for a hole. Exact on
    /// the carriers: the corners' shoelace, plus for each arc the
    /// segment between it and its chord, `(a × b)·(Δ − sin Δ)/2` for
    /// the signed span `Δ = end − start`.
    pub fn area(&self) -> T {
        let n = self.uv.len();
        let mut twice = T::zero();
        for (i, edge) in self.edges.iter().enumerate() {
            let (p, q) = (self.uv[i], self.uv[(i + 1) % n]);
            twice = twice + (p.x * q.y - q.x * p.y);
            if let SectionEdge::Arc {
                a, b, start, end, ..
            } = *edge
            {
                twice = twice + a.perp_dot(b) * chord_bulge(end - start);
            }
        }
        twice / (T::one() + T::one())
    }
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

impl<T: Real> SectionRegion<T> {
    /// The region's area: its outline's, less its holes'.
    pub fn area(&self) -> T {
        self.holes
            .iter()
            .fold(self.outline.area(), |area, hole| area + hole.area())
    }
}

/// A plane section: the frame and the regions (empty when the plane
/// misses the body — a typed success, not an error).
#[derive(Clone, Debug)]
pub struct Section<T: Real> {
    /// The sectioning plane, as given.
    pub plane: SplitPlane<T>,
    /// The in-plane u axis (unit; derived from the first polygon's
    /// below loop by `section_loops::chord_u_ref` — deterministic data;
    /// zero polygons ⇒ `u_ref`/`v_ref` are `None`).
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
    /// [`super::split`].
    Split(SplitError),
    /// The plane is tangent to a curved wall that bends away from its
    /// material ([`KnifeEdge`]): the wall touches the section along a
    /// line, a contact of zero width that a region list cannot state.
    /// The split refuses the same contact as the knife edge it would
    /// mint.
    KnifeEdge(KnifeEdge),
    /// Whether a section polygon is an outline or a hole cannot be
    /// read: its winding is in the band (`diag`), or (`None`) zero, or
    /// unread because the polygon carries an edge the winding does not
    /// read (`crate::loop_winding`).
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
        match e.knife_edge() {
            Some(&knife) => Self::KnifeEdge(knife),
            None => Self::Split(e),
        }
    }
}

impl<T: Real> core::fmt::Display for SectionError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Split(e) => write!(f, "{e}"),
            Self::KnifeEdge(_) => write!(
                f,
                "the section plane is tangent to a curved face from inside the material, so \
                 the face only touches the section along a line. Recourse: move the section \
                 plane off the tangency"
            ),
            Self::WindingUndecided {
                diag: Some(diag), ..
            } => write!(
                f,
                "{}",
                diag.undecided(
                    "whether a polygon of the section is an outline or a hole",
                    diag.lever_ending(SECTION_PLANE_RECOURSE),
                )
            ),
            Self::WindingUndecided { diag: None, .. } => write!(
                f,
                "whether a polygon of the section is an outline or a hole cannot be read: it \
                 encloses no area, or has an edge whose winding the kernel does not read (a \
                 NURBS edge, or spiric edges pinched near a torus's inner equator). Recourse: \
                 {SECTION_PLANE_RECOURSE}"
            ),
            Self::UnplacedHole { .. } => write!(
                f,
                "no outline of the section can be shown to enclose one of its holes: an \
                 outline edge the nesting cannot read (spiric or NURBS), a contact too close \
                 to call, or a sliver the section join mints across a curved face. Recourse: \
                 {SECTION_PLANE_RECOURSE}"
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
/// holed section faces with. A slit trace bounds no region and is not
/// read: its slit lies inside an outline.
///
/// # Frame semantics
///
/// `u_ref` is the normalized first chord of the first polygon's
/// below loop, or, when that loop has one corner (a whole section
/// conic), the first axis of the plane normal's own basis
/// (deterministic data either way); `v_ref = normal × u_ref`. Both are `None` iff `regions` is empty
/// (the plane misses the body — a typed success). `uv` coordinates
/// are `((p − origin)·u_ref, (p − origin)·v_ref)`.
///
/// # Errors
///
/// [`SectionError`]: [`SectionError::Split`] passes the reduce and join
/// stages' refusals through unchanged, save a curved wall's knife-edge
/// contact, which refuses as [`SectionError::KnifeEdge`] rather than
/// reporting a trace with the contact in it.
pub fn plane_section<F: crate::FinishedBody>(
    operand: &F,
    plane: &SplitPlane<F::Scalar>,
    tol: Tol,
) -> Result<Section<F::Scalar>, SectionError<F::Scalar>>
where
    F::Scalar: geom_core::Decide,
{
    let band = geom_core::Band::linear(tol)
        .map_err(|e| SectionError::Split(SplitError::Reduce(SplitReduceError::from(e))))?;
    let (red, completed, _fragments) = split_scratch(operand, plane, tol)?;
    // The below loops are read, so the frame is the below section
    // face's: its outward normal, and `u_ref × v_ref` equals it.
    let normal = section_loops::section_normal(plane.normal.get(), PlaneSide::Below);

    let mut u_ref = None;
    let mut v_ref = None;
    let mut outlines = Vec::new();
    let mut holes = Vec::new();
    for section in &completed {
        // A slit trace bounds no region: its slit lies inside an outline.
        let SectionKind::Area { slit } = &section.kind else {
            continue;
        };
        let halves = section_walk(&red.body, section.below_loop)?;
        // The caller's plane, which the frame is reported on: the below
        // loop's first chord, as the split's section face reads it.
        let u = *u_ref.get_or_insert_with(|| {
            let corners: Vec<_> = halves.iter().map(|h| h.corner).collect();
            section_loops::chord_u_ref(&red.body, section.below_loop, &corners, plane.normal)
        });
        let halves = drop_slits(halves, slit);
        let points: Vec<_> = halves.iter().map(|h| h.corner).collect();
        let v = *v_ref.get_or_insert_with(|| normal.cross(u));
        let corner = points[0];
        let outline = match section_loops::loop_sense(&red.body, section.below_loop, normal, band) {
            Ok(outline) => outline,
            Err(SenseFault::Undecided(diag)) => {
                return Err(SectionError::WindingUndecided { corner, diag });
            }
        };
        let chart = |w: Vec3<F::Scalar>| Vec2::new(w.dot(u), w.dot(v));
        let mut uv = Vec::with_capacity(halves.len());
        let mut edges = Vec::with_capacity(halves.len());
        for half in &halves {
            let c = chart(half.corner - plane.origin);
            uv.push(Point2::new(c.x, c.y));
            edges.push(match &half.carrier {
                HalfCarrier::Chord => SectionEdge::Line,
                HalfCarrier::Conic {
                    carrier,
                    start,
                    end,
                } => {
                    let conic = carrier
                        .conic()
                        .unwrap_or_else(|| unreachable!("a conic half's carrier is a conic"));
                    let c = chart(conic.frame().center - plane.origin);
                    let (a, b) = conic.semi_axis_vectors();
                    SectionEdge::Arc {
                        center: Point2::new(c.x, c.y),
                        a: chart(a),
                        b: chart(b),
                        start: *start,
                        end: *end,
                    }
                }
                // The split's carrier gate admits no spiric or NURBS edge,
                // and its nesting reads none.
                HalfCarrier::Unread => {
                    return Err(SectionError::WindingUndecided { corner, diag: None });
                }
            });
        }
        let polygon = SectionPolygon { points, uv, edges };
        if outline {
            outlines.push((polygon, section.below_loop));
        } else {
            holes.push((polygon, section.below_loop));
        }
    }
    let nesting = section_loops::nest(&red.body, outlines, holes, normal, band).map_err(
        |fault| match fault {
            NestFault::Torn => SectionError::Corrupt,
            NestFault::Contradiction(hole) => SectionError::NestingContradiction {
                corner: hole.points()[0],
            },
        },
    )?;
    if let Some(hole) = nesting.unplaced.first() {
        return Err(SectionError::UnplacedHole {
            corner: hole.points()[0],
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

/// Takes the slit out of a walked section loop: the halves `slit` names,
/// which run over a tree of the other loop ([`super::join::SectionKind::Area`])
/// out to its tips and back to a second copy of the vertex they left, where
/// the side whose loop this is touches itself. The slit lies inside the
/// section's region, so the outline goes on from the first copy along the
/// second copy's run: the first half of each run of slit halves keeps its
/// corner and takes the run of the half after it, and the rest of the run
/// and that half leave the walk.
fn drop_slits<T: Real>(mut halves: Vec<Half<T>>, slit: &[HalfEdgeKey]) -> Vec<Half<T>> {
    let n = halves.len();
    let in_slit: Vec<bool> = halves.iter().map(|h| slit.contains(&h.he)).collect();
    let mut gone = vec![false; n];
    for first in 0..n {
        if !in_slit[first] || in_slit[(first + n - 1) % n] {
            continue;
        }
        let mut copy = (first + 1) % n;
        while in_slit[copy] {
            gone[copy] = true;
            copy = (copy + 1) % n;
        }
        halves[first].carrier = core::mem::replace(&mut halves[copy].carrier, HalfCarrier::Chord);
        gone[copy] = true;
    }
    halves
        .into_iter()
        .zip(gone)
        .filter_map(|(half, gone)| (!gone).then_some(half))
        .collect()
}

/// One traversed half of a section loop: the corner it starts at, and
/// what it runs along to the next.
struct Half<T: Real> {
    he: HalfEdgeKey,
    corner: Point3<T>,
    carrier: HalfCarrier<T>,
}

/// What a half runs along.
enum HalfCarrier<T: Real> {
    /// Its chord: a line, or a null-edge scaffold, which states no
    /// carrier (the winding reads it the same way).
    Chord,
    /// An arc of `conic` from parameter `start` to `end`, in traversal
    /// order.
    Conic {
        carrier: geom::Curve3<T>,
        start: T,
        end: T,
    },
    /// A spiric or NURBS carrier, which nothing here reads.
    Unread,
}

/// The halves of cycle loop `l` in `loop_cycle` order — the corners and
/// the edges between them from one walk, so each edge runs from its
/// own corner to the next.
///
/// # Errors
///
/// [`SectionError::Corrupt`] on a dangling key or an unclaimed half;
/// a lone-vertex loop is a broken section invariant
/// ([`crate::chord_join::SplitJoinError::SectionInvariant`]).
fn section_walk<T: Decide>(body: &Body<T>, l: LoopKey) -> Result<Vec<Half<T>>, SectionError<T>> {
    let loop_data = body.get_loop(l).ok_or(SectionError::Corrupt)?;
    let LoopBoundary::Cycle { first } = loop_data.boundary else {
        return Err(
            SplitError::Join(crate::chord_join::SplitJoinError::SectionInvariant {
                face: loop_data.face,
                what: "a completed section polygon's below loop holds a lone vertex instead of a \
                   cycle",
            })
            .into(),
        );
    };
    let mut halves = Vec::new();
    for he in body.loop_cycle(first).ok_or(SectionError::Corrupt)? {
        let h = body.get_half_edge(he).ok_or(SectionError::Corrupt)?;
        let corner = body
            .get_vertex(h.start)
            .and_then(|vx| body.get_point(vx.point))
            .copied()
            .ok_or(SectionError::Corrupt)?;
        let edge = body.get_edge(h.edge).ok_or(SectionError::Corrupt)?;
        let forward = edge.claim(he).ok_or(SectionError::Corrupt)?.plus;
        let curve = body
            .get_curve_geom(edge.curve)
            .ok_or(SectionError::Corrupt)?
            .certified();
        let carrier = match curve {
            None => HalfCarrier::Chord,
            Some(curve) => match curve.carrier() {
                carrier @ (geom::Curve3::Circle(_) | geom::Curve3::Ellipse(_)) => {
                    let (t0, t1) = curve.params();
                    let (start, end) = if forward { (t0, t1) } else { (t1, t0) };
                    HalfCarrier::Conic {
                        carrier: carrier.clone(),
                        start,
                        end,
                    }
                }
                geom::Curve3::Line(_) => HalfCarrier::Chord,
                geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => HalfCarrier::Unread,
            },
        };
        halves.push(Half {
            he,
            corner,
            carrier,
        });
    }
    Ok(halves)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod winding_ending {
    use geom_core::{Band, Decide, Indeterminate, MarginDiag, Point3};

    use super::SectionError;

    /// **An undecided section winding ends in the section's lever
    /// alone**, as its unreadable sibling does: no tolerance on a read
    /// margin, the unreadable-margin note on poison.
    #[test]
    fn an_undecided_winding_ends_in_the_section_lever() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let read = 3e-9_f64.sign_within(band).unwrap_err();
        let text = SectionError::<f64>::WindingUndecided {
            corner: Point3::origin(),
            diag: Some(read),
        }
        .to_string();
        assert_eq!(
            text,
            format!(
                "whether a polygon of the section is an outline or a hole is undecided: {}. \
                 Recourse: move the section plane",
                read.payload()
            )
        );
        let poisoned = Indeterminate {
            margin: MarginDiag::INVALID,
            ..read
        };
        let text = SectionError::<f64>::WindingUndecided {
            corner: Point3::origin(),
            diag: Some(poisoned),
        }
        .to_string();
        assert!(
            text.contains(". Recourse: move the section plane; "),
            "the lever and the note, got {text}"
        );
    }
}
