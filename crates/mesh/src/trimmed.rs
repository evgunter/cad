//! **Trimmed curved-face tessellation** (M5 PR 11, C12.6): faces that
//! are no swept UV rectangle — a conic or B-spline trim carrier, a
//! notch, a ring — cannot be traversed by the iso walk; instead the
//! face's **domain** provides the trim polygon in UV:
//! every loop's lift, the rings placed in the outer loop's sheet
//! (`topo::Body::face_lift`), each half-edge's image evaluated at exactly
//! the shared per-edge chord parameters; and the CDT + even-odd
//! machinery of the planar lane (#116: combinatorial flood fill, no
//! geometric classification) picks the interior, which is the outer
//! loop's region less each ring's. A face whose loops have no reading in
//! one sheet (a loop that does not close in the chart, a ring with no one
//! place) refuses with the door's cause
//! ([`TessellateError::DomainUnread`]).
//!
//! One class of face here is not triangulated: a cylinder or cone face
//! that is an iso rectangle of its chart but for a point on a straight
//! side (`walk::splits_a_straight_side`), whose loop is the walk's
//! ([`walkable`]), is **walked** (`tessellate::Lane::Walked`,
//! `crate::ruled_walk`): no lattice and no CDT, its triangles written along
//! its two rows in order of angle, a side shared with a neighbour that may
//! make it cut at its midpoint ([`may_make`]), and each triangle read by
//! the row's certificate. Its target is δ ([`face_target`]), by the
//! theorems at `sizing::sagitta_step` and `sizing::cone_grid_step`; every
//! other analytic face here sizes at [`UNPROVED_HEADROOM`] of δ.
//!
//! # Watertightness is inherited, not re-derived
//!
//! The polygon's mesh-vertex ids are the per-edge chord ids computed
//! once in 3-D (`crate::chords`) and consumed by both adjacent faces —
//! including the curved-curved shared case. UV coordinates only shape
//! THIS face's triangulation; the 3-D identification is by id.
//!
//! # Chart scope, stated
//!
//! Every chart constructs here, from the face's chart row
//! (`crate::chart_table`): the analytic charts — **cylinder** (M5 PR 11 —
//! the tiltedcut walls; a split cylinder wall is walked), **cone** (a frustum
//! cut by a tilted plane), **sphere** (a sphere face a circle tilted
//! against its chart bounds — the sphere pairs, a ball cut by a tilted
//! plane, the oblique fillet corners), **torus** (a hollowed partial
//! revolve's spiric-bounded wall) and **spindle sheet** (no door builds a
//! trimmed spindle face yet: a spindle operand refuses at the boolean and
//! split gates,
//! `work/tesslane/no-door-builds-a-trimmed-spindle-face.md`) — and, since M7's
//! montage skin-scenes unit, **described NURBS** (the loft/sweep walls;
//! NURBS faces route here unconditionally, iso-rectangle or not, because
//! the swept-rectangle walk has no NURBS chart; RATIONAL faces since
//! M8-5, whose Hessian bound is the second-order quotient statement over
//! the homogeneous nets). An analytic face routes here when it holds a
//! ring, stores an image of no iso form, or the curved lane's doors decide
//! it is no iso-rectangle of its chart; or when it is a ruled rectangle
//! whose straight side holds a point, which the route marks walked where
//! its loop is the walk's (`crate::tessellate::route_of`; module header).
//! The NURBS lane's certificate is the
//! hull-derived Hessian interpolation bound (`crate::nurbs_cert`:
//! derivation, covered-vs-refused inventory). Its boundary pcurves
//! are the four forms whose UV steps the chord pass can size, and
//! each has exactly one mint: `Harmonic` (the analytic charts' form,
//! reaching a NURBS chart only across a shared edge), `IsoLine`
//! (every loft/sweep wall boundary), `IsoArc` (the M8-3 arc rim's
//! rational-quadratic parameter) and `General` (the P-2 route's
//! interior-column `Intersection` image, and nothing else on this
//! head). The first three are closed forms of the image; `General` is
//! a spline, sized from its own differenced control net
//! (`geom_brep::Pcurve::speed_bound`) rather than a formula.
//!
//! An analytic arm's boundary images are the analytic mints' closed
//! forms, each read exactly at the chord pass's parameters: `Harmonic`
//! (rims, meridians, rulings), `Projected` (a circle tilted against the
//! chart, its inverse of the carrier), `FocalSection` (a cone's section
//! conic) and `Spiric` (a torus's plane section). The chord pass bounds
//! each image's UV step by the arm's lattice steps
//! (`geom_brep::Pcurve::speed_bound`) — except the cylinder arm's, which
//! is chorded at its target alone — and each triangle's certificate is
//! the row's. A singular row the loop passes (a sphere's pole, a cone's
//! apex, a spindle sheet's apex) is a side of the trim polygon
//! ([`singular_row_shape`]): the loop's lift (`topo::Body::loop_lift`)
//! runs along the row through the face's turn there, so each run of the
//! loop between two such points already sits where the face is. A face
//! holding a row's point inside it winds about it, and the door refuses
//! it ([`TessellateError::DomainUnread`]); a tilted circle passing so
//! near a sphere's pole that its chord schedule asks one edge
//! for more than the sphere arm's budget ([`MAX_SPHERE_EDGE_CHORDS`],
//! `chords::tighten`) refuses typed
//! ([`TessellateError::SphereTrimShape`]). On every analytic arm a trim polygon wound
//! against the face refuses typed
//! ([`TessellateError::TrimLoopOrientation`]), a triangle folded over
//! the chord beside it is dropped and rebuilt or refuses
//! ([`TessellateError::FoldedTriangle`]). On every arm, the spline's
//! included, a triangle with a side a neighbouring face may make too (a
//! side joining two polygon points that are not neighbours on their loop
//! and that one neighbour holds both of) is split by a point inside it
//! and rebuilt, or refuses ([`TessellateError::UnclearedSharedSide`]). On
//! the spline arm that is unwitnessed: no door yet trims a spline face
//! thin against such a neighbour
//! (`work/tesslie/the-shared-side-rule-on-the-spline-arm-has-no-witness.md`).
//! A **fitted** (rung-3) chart image still refuses typed on every chart:
//! the chord pass has no certified UV chord-step bound for a fitted image (the
//! boundary-tightening contract in `crate::chords`), and its first
//! genuine consumer is the edge×NURBS-face boolean layer (the cut-loft
//! unit).
//!
//! # The lattice metric
//!
//! No arm's CDT runs in raw UV: the cone arm's runs in its development
//! (below), every other arm's in the lattice metric. Delaunay is not
//! stretch-invariant, and a ruled wall's lattice is a strip whose
//! columns are many times narrower than its rows (a lofted circle's
//! wall: `s_u/s_v ≈ 0.03`; a cylinder of radius `r`, whose lattice is
//! square in 3-D, `≈ 1/r` in (θ, z)); there, any point off the lattice — a trim
//! chord point between two rows, a band interface's foreign column, a
//! refinement centroid — has an empty circle many columns wide and a
//! sliver across them. So points enter the CDT at `(u/k, v)`, with `k`
//! the lattice metric (`crate::nurbs_cert::lattice_metric`, a power of
//! two near the mean realized aspect: of the NURBS schedule's bands, or
//! of an analytic arm's one uniform lattice; on the sphere arm the
//! chart's own metric at the trim box's mid latitude, `1/cos v_mid`),
//! where the lattice is near square. One face-wide `k` cannot square
//! bands whose aspects
//! spread far apart, so the NURBS schedule also raises a band's rows
//! when its columns stay too narrow over `k`
//! ([`crate::nurbs_cert::NurbsCellGrid::band_schedule`], **Rows for
//! narrow-column bands**). The map is applied after the crate's
//! coordinate floor (`crate::cdt`), which merges the band
//! `(0, 2⁻¹⁴²)` into `0.0` and is neither affine nor injective there;
//! the metric itself scales the axis that grows by a power of two, so
//! it is affine (constraint segments stay straight), exact and
//! injective on floored points, and two points are one CDT vertex
//! exactly when they are one location after the floor. It is `(u/k, v)`
//! times a uniform power of two: spade's predicates are exact under
//! that scale, but its bulk loader's hull walk compares against an
//! absolute `f64::EPSILON`, so the triangulation is not guaranteed the
//! one `(u/k, v)` itself would give, and nothing here relies on it
//! being. Only the CDT sees it: `meta`, the creases, the polygon's
//! orientation and every certificate read UV.
//!
//! # The development
//!
//! A cone face's CDT runs in the cone's development
//! ([`crate::chart_table::AnalyticRow::development`]): the chart point
//! `(u, v)` at radius `σ·v` (the slant length) and angle `σ·sin α·u`,
//! `σ = ±1` the face's nappe ([`developed`]). It is an isometry of the
//! nappe onto a plane sector, so the CDT is the surface's own Delaunay
//! triangulation, and it keeps the chart's orientation on either nappe.
//! Its reason is the apex. In the chart the apex is a row, the loop
//! keeps it once at each azimuth it meets it at, and a Delaunay
//! triangulation of the chart can join one apex copy to two points of
//! one column: three points of one ruling, a straight line of the
//! surface, so a triangle of zero area. In the development every copy is
//! one point and every column a ray from it, cut so each point of a
//! column is an exact multiple of one direction ([`developed`]): the
//! exact predicates read any three points of one column (one `u`, bit
//! for bit), or two and the apex, as collinear, and no triangle lies
//! along one. A ruling is one column but for an end vertex the walk reads
//! through its rim's image, which can sit a rounding off it
//! (`work/tesslane/a-rulings-end-vertex-read-through-its-rims-image-is-off-the-rulings-exact-line.md`);
//! a triangle that made of a ruling would have no area, and the winding
//! rule refuses it. Three points of a cone on one line
//! lie on a ruling (a line meets the quadric in two points or lies in
//! it), so no other triangle of zero area exists to make. A THIN triangle
//! is another matter, and nothing bounds it: beside a ruling a triangle's
//! `2·area/L²` goes as `(h/L)²`, and a needle cone's face through its
//! apex meshes certified triangles near `1e-10`
//! (`work/tesslane/a-needle-cone-face-meshes-thin-triangles-beside-its-rulings.md`).
//!
//! Near a small end the development puts points far apart in azimuth
//! close together, and its Delaunay triangulation joins them. Two things
//! answer that. The certificate is the flat triangle's own deviation
//! ([`cert::cert_cone`]): each chord at its own two radii and its own
//! azimuth difference, no chart box, so such a triangle is charged what it
//! sags, which goes as its smaller radius. And the rows are graded to
//! their own radius ([`AnalyticRow::lattice_rows`]), so the row next to a
//! small end stands a chord of the lattice's target from it rather than
//! many: under rows uniform in `v` a face ending near its apex holds
//! triangles truly past δ.
//!
//! - **Which points are one CDT vertex.** Every copy of the apex is the
//!   origin. The development rounds, so it is not injective: the cut
//!   resolves a slant radius to `2⁻²⁷` of itself, a length that grows as
//!   the cone flattens toward a cylinder (`2⁻²⁷·r / sin α`). Consecutive
//!   points of the loop along one ruling that it puts at one point are a
//!   run ([`Run`]): the triangulation holds one vertex for them, and the
//!   emit pass gives each triangle there the run's point it stands on
//!   and adds the triangles between the run's points, each certified and
//!   wound on its true corners like any other ([`TrimmedFace::emit`]).
//!   A face whose polygon points would share a CDT vertex any other way
//!   (a point with the apex, points of two sides, a ruling side wholly
//!   inside the cut) refuses `SphereTrimShape { DevelopedMerge }` before
//!   its first attempt ([`developed_runs`]) rather than merge two mesh
//!   vertices. A lattice node is laid clear of every polygon point
//!   ([`lay`]), the apex among them, so none shares a vertex with one.
//!   [`crate::cdt::ids_at_two_uvs`] still reads UV locations, so it names
//!   the apex's id; the census of identified-vertex fan edges then counts
//!   the apex's one fan, whose edges a single vertex's fan uses twice at
//!   most.
//! - **The sector.** A face lies on one nappe (`singular_row_shape`
//!   refuses one on both, [`SphereTrimCause::BothNappes`]); a whole turn
//!   spans `2π·sin α < 2π`, so its seam ruling's two sides are two rays;
//!   a face through the apex meets it at the sector's corner; a loop
//!   meeting the apex twice is two wedges touching at the origin, which
//!   the even-odd classification reads as such.
//! - **What it moves.** The triangulation alone, and each kept triangle
//!   is still certified, so a different triangulation changes the
//!   triangles' shape, not what is guaranteed of them. Every other
//!   decision reads what it is about. The lattice's columns are the
//!   chart's, uniform in `u`, where the cell bound's azimuth step lives,
//!   and its rows stand a 3-D chord apart at their own radius. The
//!   certificate reads the flat triangle in space through its chart
//!   corners, which is neither the chart's box nor the development. The
//!   fold census reads the triangle's 3-D normal against the surface's.
//!   The shared-side test reads the polygon's order, and the polygon's
//!   orientation and the creases the chart. A point the shared-side cure
//!   inserts is the developed centroid read back to the chart
//!   ([`TrimmedFace::inside_point`]), strictly inside its triangle where
//!   the CDT runs.
//! - **Replay.** `sin` and `cos` are [`geom_core::Real`]'s, the pure-Rust
//!   `libm` every platform shares, and the read back is
//!   [`geom::Azimuth::near`], the angle door: what decides the
//!   triangulation replays bit for bit (D9).
//!
//! # C⁰ creases
//!
//! A NURBS face may carry interior knots of multiplicity equal to the
//! degree, where its first derivative may jump: a lofted circle's wall
//! has one per sub-arc joint. No triangle may cross one
//! (`NurbsCellGrid::cert` answers `+∞` for a box that does), so the
//! lane cuts its triangulation along each (`Creases`): boundary points
//! within rounding of a crease snap onto it, the lattice lays no node
//! within its clearance of one and gains a point on the crease at every
//! grid line across it, and the points on a crease are chained into
//! constraints. A boundary chord can only meet a
//! crease at a chord point, which an arc rim's schedule places
//! (`crate::chords`); any other boundary crossing one refuses typed, as
//! do creases in both directions.
//!
//! # The lattice keeps clear of the boundary
//!
//! A lattice node is laid only where the box [`BOUNDARY_CLEARANCE`] of
//! its own step each way, per chart axis, meets no side or point of the
//! trim polygon and no crease line ([`lay`]; `crates/mesh/README.md`
//! `[mesh.sizing]`). A node on a side would split it inside spade,
//! leaving this face triangulated against a sub-segment its neighbour
//! does not share (a T-junction), and a node within rounding of a side
//! or of a chord point makes a triangle with no area between them. The
//! lattice is laid without regard to the boundary, so both happen where
//! a boundary point sits on a lattice line; the clearance is what keeps
//! them from the triangulation, decided once, before the first attempt.
//! Nothing is merged or moved and nothing asks whether two points are
//! the same: the boundary point already there serves.
//!
//! - **The step is the node's own**: on an analytic chart the lattice's
//!   realized column step and its row's gap, the smaller of the two
//!   beside it where the rows are graded (the cone's); its band's on a
//!   spline chart, where a node on a cut line is laid by each band
//!   beside it at that band's steps.
//! - **Every loop is read**, a ring's sides as the outer loop's. A
//!   period seam the loop runs along and a singular row
//!   ([`singular_row_shape`]) are sides of the polygon like any other:
//!   the column nearest a seam ruling, or the row nearest a pole, is
//!   laid when it is a clearance away and not otherwise.
//! - **The plane is the one the CDT runs in** ([`Side`]): a flat triangle
//!   is a node on the line of a side where the triangulation holds that
//!   side straight. In the chart that is the chart segment against the
//!   node's chart rectangle, which the lattice metric scales together.
//!   In the cone's development it is the segment between the ends'
//!   developed points, which bows from the chart's chord toward the
//!   apex, against the image of the node's chart rectangle: the annular
//!   sector of its azimuths and slant lengths.
//! - **A face may lay no node.** A strip under two clearances wide loses
//!   every one; its triangles then stand on its boundary points, and the
//!   per-triangle certificate bounds each as it does any other.
//!
//! A point the lane places later (a shared side's or a refinement's,
//! below) is inside a kept triangle ([`TrimmedFace::inside_point`] says how far
//! down f64 holds that) and is not held to the clearance. A
//! point of the mesher's that still enters at an existing vertex, or
//! that a side or a crease link is realised through, refuses typed
//! ([`TessellateError::Triangulation`]): it is not dropped and the face
//! is not rebuilt around it.
//!
//! # The retry loop
//!
//! The lane rebuilds a face, at most [`MAX_GRID_RETRIES`] times, for
//! three causes: a triangle folded over the chord beside it drops its
//! lattice corners, a triangle with a side a neighbour may make too is
//! split by a point inside it, and (below) a triangle certified above δ
//! is refined. Lattice nodes are a fixed, (v, u)-sorted list computed
//! once, before the loop; a fold drops a node by its index, and the
//! shared-side and refinement arms append, so indices only grow.
//!
//! The same loop carries TESS-SPAN's second arm, **certificate-driven
//! refinement**: after the emit pass, any NURBS triangle whose
//! certificate exceeds δ — one that would otherwise refuse — queues
//! its UV centroid as a new candidate and the attempt rebuilds. This
//! is deliberately a LAST-RESORT backstop, not the sliver cure: in a
//! lattice that is anisotropic in the CDT's own coordinates, a
//! centroid is one more off-lattice point and re-admits the empty
//! circumcircle beside itself (measured on the #316 leaf, and on a
//! lofted circle's wall, where the ladder walked the sliver inward a
//! step per round: `work/tesslane/lofted-rational-walls-refuse-at-some-deltas`).
//! The cure is the lattice metric and the row raise (above), which
//! make the lattice near square where the CDT runs
//! ([`crate::nurbs_cert::NurbsCellGrid::band_schedule`]); an attempt
//! that still certifies above δ after the shared round budget refuses
//! typed ([`TessellateError::CertificateExceeded`]) — the
//! certificate, as everywhere, is the guarantee. An attempt's interior
//! positions belong to that attempt: the patch this lane returns is
//! the ACCEPTED attempt's, and a retried attempt's points are dropped
//! with it.
//!
//! A BOUNDARY point as intermediate is a
//! self-touching trim loop, refused typed
//! ([`TessellateError::SelfTouchingTrimLoop`]): a boundary point
//! inside another segment of the polygon, of its own loop or of another
//! (a ring touching its outer loop along a side). A loop through two vertices at
//! one point is not that shape; both are walk entries at one handle,
//! which [`Pinches`] reads. The refusal has no at-rest fixture:
//! hand-building a boundary point inside another segment at the mesh
//! layer would need a full body whose certified pcurve caches describe
//! a loop the mint pass itself refuses (`LoopNotClosed`/continuity) —
//! the arm stands as the backstop's tripwire (review MIN-1).

use std::collections::{HashMap, HashSet};

use geom::Surface;
use geom_brep::{OutwardNormal, Pcurve, PolygonOrientation};
use geom_core::period::Period;
use geom_core::{Band, Indeterminate, Point3};
use spade::handles::FixedVertexHandle;
use spade::{ConstrainedDelaunayTriangulation, Point2 as SpadePoint, Triangulation};
use topo::pcurves::{FaceLift, Lift};
use topo::{Body, EdgeKey, FaceKey};

use crate::cert;
use crate::chart_table::{
    Analytic, AnalyticRow, ChartRow, RowEnd, SingularPoint, at_a_singular_point,
};
use crate::chords::ChordPass;
use crate::nurbs_cert::{FaceBounds, NurbsCellGrid};
use crate::planar::{Pinches, Walk, shoelace2};
use crate::region::{Crossings, classify_faces, edge_key};
use crate::sizing::{SizingTols, ceil_count};
use crate::tessellate::{Lane, Patch, PatchVertex};
use crate::types::{SphereTrimCause, TessellateError};

/// Retry budget for the rebuild loop (module docs). **Termination.**
///
/// One round budget covers all three retry kinds — the
/// certificate-driven refinement, the fold drops and the shared-side
/// splits — and it is a typed-refusal backstop rather than a tuning
/// knob: exhausting it raises [`TessellateError`], it never coarsens or
/// accepts a mesh. The loop reads it through [`last_attempt`].
const MAX_GRID_RETRIES: usize = 6;

/// The retry loop's last attempt: [`MAX_GRID_RETRIES`], or under
/// `cfg(test)` an override a row sets to reach an exhaustion refusal
/// through the real call path (`tests::LAST_ATTEMPT`).
fn last_attempt() -> usize {
    #[cfg(test)]
    if let Some(k) = tests::LAST_ATTEMPT.with(std::cell::Cell::get) {
        return k;
    }
    MAX_GRID_RETRIES
}

/// **The trimmed lane's unproved headroom**: the cylinder arm sizes its
/// lattice, and the chord pass chords that face's edges
/// (`chords::edge_target`), at this share of δ rather than at δ
/// ([`face_target`]).
///
/// The lane has no worst-triangle theorem (`crates/mesh/README.md`
/// `[mesh.sizing]`). Its lattice cell-halves would certify at the
/// sagitta of the column step, but the trim polygon's points sit
/// anywhere in a cell, and a constrained Delaunay triangle beside a
/// trim chord can span more than one column — nothing yet bounds by
/// how much. The certificate refuses typed where it is exceeded; this
/// factor is what keeps that refusal rare, and it is a debt
/// (`work/tesslane/the-trimmed-cylinder-lane-has-no-worst-triangle-theorem.md`):
/// the fix is the theorem, not a retuned factor.
///
/// The sphere arm reads it the same way and for the same reason, at its
/// lattice steps ([`arm_steps`]) and its edges' chords: its
/// cells are the curved lane's sphere cells, whose theorem
/// (`sizing::sphere_grid_steps`) does not cover a CDT triangle beside a
/// trim chord the lattice did not place
/// (`work/tesslane/the-trimmed-sphere-arm-has-no-worst-triangle-theorem.md`).
/// The cone, torus and spindle arms read it for the same reason: each
/// chart's cell bound is proved (`crate::chart_table`'s steps), and what
/// is missing on every chart alike is the lemma that bounds a triangle
/// beside a boundary run crossing lattice lines
/// (`work/tesslane/one-containment-lemma-for-an-oblique-boundary.md`).
///
/// A face the lane walks (`tessellate::Lane::Walked`) does not read it: its triangles
/// are no CDT's, and the step functions' theorems hold them at δ
/// (`crate::ruled_walk`).
pub(crate) const UNPROVED_HEADROOM: f64 = 0.5;

/// **The sphere arm's per-edge chord budget**, `2^16`. **Work**: it
/// bounds what one face's CDT is asked to hold, as `sizing::MAX_COUNT`
/// bounds an allocation. A tilted circle passing a chart pole at polar
/// distance `d` asks about `ρ_c·Δt/(r·d·h_u)` chords of its edge
/// (`chords::tighten`), and every one enters the face's CDT.
/// Measured in release on the tilted pair of unit balls (PR 5920's
/// second fix pass): 3.7·10⁴ boundary points mesh in 0.2–0.3 s, and
/// 5.7·10⁵ take 18–62 s per attempt, ending folded either way. The
/// budget keeps one face's boundary under about 10⁵ points, about a
/// second; an edge asking more refuses `SphereTrimShape { ChordBudget }`
/// at once. It is a count, not a share of the lattice, so a coarser δ
/// (which asks fewer chords) never refuses where a finer one meshes. A
/// circle far from the poles reaches it only at a δ about `10⁻⁹·r`.
pub(crate) const MAX_SPHERE_EDGE_CHORDS: usize = 1 << 16;

/// **How far a lattice node keeps from the face's boundary**, in its own
/// steps: a node is laid only where the box this share of its step each
/// way, per chart axis, meets no side or point of the trim polygon and no
/// crease line ([`lay`]; `crates/mesh/README.md` `[mesh.sizing]`).
/// **Quality, and a debt.** The value is temporary: the worst triangle
/// beside the boundary grows with it, and this one is small enough that
/// a face which meshes with no clearance still does. The value nobody
/// has to choose is 1/2, a node not laid where the boundary crosses its
/// own cell, and that waits on the bound for a boundary run
/// (`work/tesslane/the-boundary-clearance-goes-to-half-a-step-with-the-boundary-run-bound.md`).
/// Read by `tessellate`, which hands it to [`tessellate_trimmed`]; the
/// lane takes its clearance as a value ([`TrimmedFace::clearance`]).
pub(crate) const BOUNDARY_CLEARANCE: f64 = 1.0 / 16.0;

/// **A face's deviation target**: [`UNPROVED_HEADROOM`]·δ for a face
/// this lane meshes on an analytic chart (one whose surface has no spline
/// chart), δ for every other face, a face it walks among them
/// (`tessellate::Lane::Walked`). The one statement those arms size
/// their lattices at and the chord pass chords the face's boundary at
/// (`chords::edge_target`), each reading the face's lane from the run's
/// route (`tessellate::Routes`).
pub(crate) fn face_target(surface: &Surface<f64>, lane: Lane, delta: f64) -> f64 {
    if lane == Lane::Trimmed && surface.spline_chart().is_none() {
        delta * UNPROVED_HEADROOM
    } else {
        delta
    }
}

/// **Whether the walk takes a face the route sends to this lane for a
/// split straight side** (`tessellate::route_of`'s fourth fact, which
/// asks it once per run and carries the answer as
/// `tessellate::Trim::Walked`): the face's outer loop is a cycle of rims
/// and lines in one of the walk's two shapes (`ruled_walk::rim_runs`),
/// two runs of rims with no vertex at a cone's apex, or one run whose
/// sides meet at the apex. A face whose sides pass through the apex onto
/// the other nappe is neither (no body built here makes one), and one
/// whose apex the band cannot decide is not answered: both keep the
/// lattice and the CDT (`Trim::Structural`), which refuse them typed. The
/// apex is read as the lane reads it (`chart_table::at_a_pole`, the
/// decision [`AnalyticRow::is_singular`] makes), with no row minted for a
/// window the route has not read.
pub(crate) fn walkable(
    body: &Body<f64>,
    fk: FaceKey,
    surface: &Surface<f64>,
    band: Band,
) -> Result<bool, TessellateError> {
    let face = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;
    let (mut line, mut apexes) = (Vec::new(), 0usize);
    for (hek, ek, _) in crate::walk::loop_half_edges(body, face.outer, fk)? {
        match edge_carrier(body, ek)? {
            geom::Curve3::Line(..) => line.push(true),
            geom::Curve3::Circle(..) => line.push(false),
            _ => return Ok(false),
        }
        // The vertex's point through topo's one read of it, the read behind
        // `Body::vertex_points`, whose copy the lane's mesh position of this
        // vertex is (`tessellate`): the lane reads the apex there
        // (`singular_vertices`), so both read one point.
        let he = body
            .get_half_edge(hek)
            .ok_or(TessellateError::MissingEntity { what: "half-edge" })?;
        let start = topo::readback::vertex_point(body, he.start).map_err(|_| {
            TessellateError::MissingEntity {
                what: "vertex position",
            }
        })?;
        let at_apex = crate::chart_table::at_a_pole(surface, start, band).ok_or(
            TessellateError::MissingEntity {
                what: "an analytic surface's chart",
            },
        )?;
        match at_apex {
            Ok(true) => apexes += 1,
            Ok(false) => {}
            Err(_) => return Ok(false),
        }
    }
    Ok(match crate::ruled_walk::rim_runs(&line) {
        1 => apexes > 0,
        2 => apexes == 0,
        _ => false,
    })
}

/// The carrier of edge `ek`.
fn edge_carrier(body: &Body<f64>, ek: EdgeKey) -> Result<&geom::Curve3<f64>, TessellateError> {
    crate::walk::edge_curve(body, ek).map(|c| c.carrier())
}

/// **The trimmed lane's steps on an analytic chart** `(h_u, h_v)`: the
/// row's steps ([`AnalyticRow::steps`]) at the face's target
/// ([`face_target`]). The chord pass reads the same steps to bound each
/// boundary chord's UV step (`chords::tighten`), so the lattice and the
/// trim polygon are sized from one value.
pub(crate) fn arm_steps(row: &AnalyticRow<'_>, target: f64) -> (f64, f64) {
    #[cfg(test)]
    let scale = tests::STEP_SCALE.with(std::cell::Cell::get);
    #[cfg(not(test))]
    let scale = 1.0;
    let (hu, hv) = row.steps(target);
    (hu * scale, hv * scale)
}

/// **A trimmed face's domain**: its outer loop's lift and each ring's,
/// placed in the outer loop's sheet by the door ([`topo::Body::face_lift`])
/// at the run's band. The tessellation reads it once per face and hands
/// the one value to the memo key and to [`tessellate_trimmed`].
///
/// # Errors
///
/// [`TessellateError::DomainUnread`] with the door's cause: the lane's one
/// refusal of a loop that does not close in the chart, a ring with no
/// one place, a ring that is one vertex, or a loop with no lift.
pub(crate) fn face_domain(
    body: &Body<f64>,
    fk: FaceKey,
    band: geom_core::Band,
) -> Result<FaceLift<f64>, TessellateError> {
    body.face_lift(fk, band)
        .map_err(|cause| TessellateError::DomainUnread {
            face: fk,
            cause: Box::new(cause),
        })
}

/// Tessellates one trimmed curved face (module docs) over its domain
/// `lift` ([`face_domain`]): pcurve-driven UV polygon, interior grid, CDT
/// with even-odd classification, and the chart row's certificate per
/// triangle ([`ChartRow::cert`]).
#[allow(clippy::too_many_arguments)] // the face, its domain, and what the run shares
pub(crate) fn tessellate_trimmed(
    body: &Body<f64>,
    fk: FaceKey,
    surface: &Surface<f64>,
    chords: &ChordPass,
    shared: &[Point3<f64>],
    tol: &SizingTols,
    bounds: &FaceBounds,
    lift: &FaceLift<f64>,
    clearance: f64,
    walked: bool,
) -> Result<Patch, TessellateError> {
    let face = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;
    // The face's chart row (`crate::chart_table`). A spline face's is the
    // cell table under this FACE's key, which the chord pass assembled
    // from the same `Surface::spline_chart()` this lane reads: the memo
    // holds it, and the row borrows it.
    let row = ChartRow::of(body, fk, surface, bounds)?;
    // A spline chart's C⁰ creases; an analytic chart has none.
    let creases = match row {
        ChartRow::Spline { grid } => match surface.spline_chart() {
            Some(chart) => Creases::of(grid, chart, fk)?,
            None => None,
        },
        ChartRow::Analytic(_) => None,
    };
    let spline = matches!(row, ChartRow::Spline { .. });
    let (singular, polygon) = face_polygon(&PolygonOf {
        body,
        fk,
        surface,
        row: &row,
        lift,
        creases: creases.as_ref(),
        chords,
        shared,
        band: tol.band,
        delta: tol.delta,
    })?;
    let orientation = trim_orientation(fk, face.sense, spline, &polygon)?;
    if walked {
        // `[failure.taxonomy]` row 0: could the route hand this lane a
        // type that only a cylinder or cone face's row inhabits, as
        // `curved::Rectangle` is the rectangle lane's? No, not locally:
        // whether a face's row is analytic or a spline's is `ChartRow::of`'s
        // run-time match on the surface's kind, the constructor every arm of
        // this lane shares, and a walked row by type would change it for
        // all of them. So row 4.
        let ChartRow::Analytic(ref a) = row else {
            unreachable!(
                "face {fk:?}: the route walks only a face whose split-side fact \
                 held (`walk::splits_a_straight_side`, which answers no off a \
                 cylinder or a cone, `chart_table::ruled`), and a cylinder's or a \
                 cone's chart row is analytic (`ChartRow::of`); a spline row here \
                 means the route and the row read two surfaces"
            );
        };
        return walk_face(body, fk, a, &polygon, &singular, chords, shared, tol);
    }
    // A CHART WITH SINGULAR POINTS keeps its polygon to the shape
    // `singular_row_shape` admits, so the duplicate-id skip in the emit
    // pass never collapses a fan at one (#678).
    if let ChartRow::Analytic(ref a) = row
        && !a.singular_points().is_empty()
    {
        singular_row_shape(
            fk,
            a.singular_points(),
            &singular,
            &polygon,
            a.development().is_some(),
        )?;
    }

    // The lattice, over the trim box: on an analytic chart a uniform
    // one at the row's steps at the face's target (`arm_steps`); on a
    // spline chart the Hessian-budget steps (`nurbs_cert`) at the lane's
    // lattice budget (`nurbs_cert::lattice_budget`), applied PER
    // KNOT-SPAN CELL from each cell's own certified bound (TESS-SPAN),
    // grid lines on the cell boundaries. The chord pass's boundary
    // tightening keeps the whole-patch steps (the reported D-2 choice —
    // `nurbs_cert` module docs).
    let (mut u0, mut u1, mut v0, mut v1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
    for &(u, v, _) in &polygon.points {
        u0 = u0.min(u);
        u1 = u1.max(u);
        v0 = v0.min(v);
        v1 = v1.max(v);
    }
    let (lattice, nurbs_stats, metric) = match row {
        ChartRow::Analytic(ref a) => {
            // The row's steps at the face's target, the steps the chord
            // pass bounded this face's boundary UV steps by (`arm_steps`),
            // and the lattice the row lays from them.
            let (hu, hv) =
                a.lattice_steps(arm_steps(a, face_target(surface, Lane::Trimmed, tol.delta)));
            let nu = pole_side_columns(&polygon, &singular, (u0, u1), ceil_count(u1 - u0, hu)?)?;
            let rows = a.lattice_rows((v0, v1), (hu, hv))?;
            // The other half of #678's shape on the cylinder, checked
            // where the count is: the fan needs ONE interior column
            // between two boundary entries that share a mesh vertex,
            // which is `nu == 2`. A cylinder has no singular point, and
            // `one_location_per_branch` holds each vertex at one UV per
            // branch, so a repeated id at two DISTINCT UVs is one vertex
            // on two branches, at least half a turn apart: `u1 - u0` is
            // at least π against an `hu` the sagitta cap holds at ≤ π/4,
            // so `nu >= 4` and the two never meet.
            debug_assert!(
                !matches!(a.kind(), Analytic::Cylinder { .. })
                    || nu != 2
                    || !id_repeats_apart(&polygon.points),
                "face {fk:?}: a trim polygon repeats a mesh id at two UV locations \
                 with a single interior column (nu = 2, {} rows) — the shape \
                 #678's non-manifold fan needs (curved::pole_columns)",
                rows.len()
            );
            (
                lattice_candidates((u0, u1), (v0, v1), nu, &rows),
                None,
                // The cone's CDT runs in its development and reads no
                // metric; every other arm's rows are uniform, one fewer
                // than their gaps.
                match a.development() {
                    Some(_) => 1.0,
                    None => a.trimmed_metric((u0, u1), (v0, v1), (nu, rows.len() + 1)),
                },
            )
        }
        ChartRow::Spline { grid } => {
            let (mut cand, stats) = per_cell_candidates(
                grid,
                (u0, u1),
                (v0, v1),
                crate::nurbs_cert::lattice_budget(tol.delta),
            )?;
            if let Some(c) = &creases {
                c.cut_candidates(&mut cand);
            }
            (cand, Some(stats), stats.metric)
        }
    };
    // The lattice nodes a fold has dropped, by index into the candidates
    // (module docs, "The retry loop"): mesh ids mint row-major on the
    // FINAL kept set.
    let across = faces_across(body, fk, chords)?;
    let mut dropped: HashSet<usize> = HashSet::new();
    // THE FALSIFICATION ACCUMULATOR, deliberately OUTSIDE the retry
    // loop and reset by nothing.
    //
    // Two obligations share the deviation samples and they want
    // different scopes. The CSV's `worst_dev` / `dev_samples` describe
    // the mesh that was KEPT, so they are per-attempt (below). The
    // falsification — was any triangle's measured deviation ever above
    // its own certificate — is about every triangle this face
    // TESSELLATED, discarded attempts included: a certificate that
    // fails on a triangle we then threw away has still failed, and a
    // falsifier that stops watching a case is the defect it exists to
    // catch. So `worst_ratio` accumulates across attempts and is
    // handed over once, at the end.
    let mut falsification = Falsification {
        worst_ratio: f64::NAN,
        ratio_samples: 0,
    };
    // The CSV's deviation columns and the face's worst certificate.
    // These describe ONE attempt — the one the face exits on — and are
    // reset at the top of each; `worst_ratio` above deliberately is
    // not. They live out here so the single hand-off below can read
    // them whichever way the loop ends.
    let mut measure = AttemptMeasure::fresh();
    // THE FACE'S ONE EXIT. Every path out of the retry loop lands
    // here, so the measurement hand-off below is on all of them —
    // including the two typed REFUSALS, which is the case the
    // falsification accumulator above exists for. It is never `None`
    // after the loop: an attempt asks for a rebuild only below
    // `MAX_GRID_RETRIES`, so the last attempt accepts or refuses.
    let mut outcome: Option<Result<Patch, TessellateError>> = None;
    let plane = CdtPlane::of(&row, metric, &polygon, &singular);
    let straight = |i: usize| carried_by_a_line(body, polygon.edges[i]);
    let runs = developed_runs(plane, &polygon, &singular, &straight).map_err(|i| {
        TessellateError::SphereTrimShape {
            face: fk,
            edge: polygon.edges[i],
            cause: SphereTrimCause::DevelopedMerge,
        }
    })?;
    let trimmed = TrimmedFace {
        fk,
        sense: face.sense,
        orientation,
        metric,
        surface,
        row: &row,
        singular: &singular,
        polygon: &polygon,
        creases: creases.as_ref(),
        shared,
        tol,
        across: &across,
        runs: &runs,
        clearance,
    };
    // The lattice the face lays: every node clear of the boundary
    // (`TrimmedFace::lay`), decided once, here. The retry loop appends
    // shared-side and refinement points, each strictly inside a kept
    // triangle.
    let mut candidates = trimmed.lay(&lattice);
    for attempt in 0..=last_attempt() {
        // THIS ATTEMPT's columns, cleared at its FIRST line: a
        // discarded attempt's triangles must not contribute to numbers
        // that describe the mesh the face exits on, and every path out
        // of this loop — including the ones that leave before the emit
        // pass — reaches the hand-off below. (`worst_ratio` is
        // deliberately not among them; see above.)
        measure = AttemptMeasure::fresh();
        match trimmed.attempt(
            attempt,
            &mut candidates,
            &mut dropped,
            &mut measure,
            &mut falsification,
        ) {
            Ok(Next::Rebuild) => {}
            Ok(Next::Accept(patch)) => {
                outcome = Some(Ok(patch));
                break;
            }
            Err(e) => {
                outcome = Some(Err(e));
                break;
            }
        }
    }

    // BUDGET METER (dead in normal runs): this face's measurements,
    // handed over ONCE, however the face ENDED — everything derivable
    // downstream is derived downstream (`crate::budget`, issue #320 /
    // TESS-SPAN). The certified bounds are the ones the sizing above
    // already read; nothing is recomputed for the meter.
    //
    // A refused face is measured too, and that is the point: the
    // falsification `worst_ratio` carries is about every triangle this
    // face TESSELLATED, and a face whose certificate was exceeded is
    // the likeliest carrier of a violating sample there is. The other
    // columns describe the attempt the face exited on — for a refusal
    // that is the attempt that lost, which is the only mesh there was.
    if crate::budget::armed()
        && let (Some(stats), ChartRow::Spline { grid }) = (nurbs_stats, &row)
    {
        let patch = grid.patch();
        let budget = crate::nurbs_cert::lattice_budget(tol.delta);
        crate::budget::note_face(crate::budget::FaceMeasure {
            face: fk,
            u: (u0, u1),
            v: (v0, v1),
            delta_s: budget,
            grid_cells: stats.grid_cells,
            bands: stats.bands,
            cap_bands: stats.cap_bands,
            // No band is projected; the column stays for the CSV's shape
            // until the budget lane's re-cut drops it
            // (`work/budget/snap-bands-column-reads-zero-since-the-snap-retired`).
            snap_bands: 0,
            realized_aspect: stats.realized_aspect,
            muu: patch.muu,
            muv: patch.muv,
            mvv: patch.mvv,
            mu1: patch.mu1,
            mv1: patch.mv1,
            patch_steps: patch.grid_steps(budget),
            cells: grid
                .cells()
                .map(|c| crate::budget::CellMeasure {
                    u: c.u,
                    v: c.v,
                    muu: c.bound.muu,
                    muv: c.bound.muv,
                    mvv: c.bound.mvv,
                    mu1: c.bound.mu1,
                    mv1: c.bound.mv1,
                    steps: c.bound.grid_steps(budget),
                })
                .collect(),
            worst_cert: measure.verdict.worst(),
            worst_dev: measure.worst_dev,
            worst_dev_cert: measure.worst_dev_cert,
            worst_ratio: falsification.worst_ratio,
            dev_samples: measure.dev_samples,
        });
    }
    outcome.unwrap_or_else(|| {
        unreachable!(
            "the last attempt accepts or refuses: `TrimmedFace::attempt` asks for a \
             rebuild only below MAX_GRID_RETRIES"
        )
    })
}

/// Handle-index → (mesh id or grid candidate index, uv).
#[derive(Clone, Copy)]
enum Slot {
    Boundary(u32),
    Grid(usize),
}

/// The falsification accumulator [`tessellate_trimmed`] carries across
/// attempts (its comment there says why it is never reset).
struct Falsification {
    worst_ratio: f64,
    ratio_samples: u64,
}

/// One attempt's measurements: the budget CSV's deviation columns and
/// the face's worst certificate, reset at the top of each attempt.
struct AttemptMeasure {
    /// The attempt's certificate verdict over its triangles.
    verdict: cert::FaceVerdict,
    worst_dev: f64,
    worst_dev_cert: f64,
    dev_samples: u64,
}

impl AttemptMeasure {
    fn fresh() -> Self {
        Self {
            verdict: cert::FaceVerdict::new(),
            worst_dev: f64::NAN,
            worst_dev_cert: f64::NAN,
            dev_samples: 0,
        }
    }
}

/// What an attempt that did not refuse asks of the retry loop.
enum Next {
    /// Rebuild: a fold's lattice corners dropped, or a shared side's or a
    /// refinement's points appended.
    Rebuild,
    /// The attempt's patch, accepted.
    Accept(Patch),
}

/// What every attempt on one face reads and none changes.
struct TrimmedFace<'a> {
    fk: FaceKey,
    /// How far the face's lattice keeps from its boundary, in a node's
    /// own steps ([`lay`]): [`BOUNDARY_CLEARANCE`] from `tessellate`.
    clearance: f64,
    /// The face's orientation sense: its outward normal is the
    /// surface's when true ([`Self::outward`]).
    sense: bool,
    /// The trim polygon's orientation in the chart
    /// ([`trim_orientation`]), which the emitted triangles take. It is
    /// the polygon's and not `sense` restated: on the spline arm
    /// nothing compares the two. `None` where a loop's signed area reads
    /// neither way, which refuses once the loops are known to bound a
    /// region (`attempt`).
    orientation: Option<PolygonOrientation>,
    /// The lattice metric `k`: in the chart, points enter the CDT at
    /// `(u/k, v)` times a uniform power of two, scaling the axis that
    /// grows ([`Self::plane`]).
    metric: f64,
    surface: &'a Surface<f64>,
    row: &'a ChartRow<'a>,
    /// The mesh ids of the loop's vertices that are the chart's singular
    /// points ([`singular_vertices`]).
    singular: &'a HashSet<u32>,
    /// Every loop of the face's domain ([`TrimPolygon`]).
    polygon: &'a TrimPolygon,
    creases: Option<&'a Creases>,
    shared: &'a [Point3<f64>],
    tol: &'a SizingTols,
    /// The faces across the loops' edges at each boundary mesh id
    /// ([`faces_across`]), which the shared-side rule reads
    /// ([`Self::attempt`]).
    across: &'a HashMap<u32, Vec<FaceKey>>,
    /// The runs of the polygon the development holds as one CDT vertex
    /// each ([`developed_runs`]); none in the chart.
    runs: &'a [Run],
}

/// One attempt's triangulation, between inserting its points and
/// emitting its triangles.
struct Cdt {
    /// The triangulation, in the plane the face's CDT runs in
    /// ([`cdt_point`]).
    cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>>,
    /// The first entry's slot at each handle; [`Pinches`] overrides
    /// a boundary slot per triangle at a pinch handle.
    meta: Vec<(f64, f64, Slot)>,
    /// The trim polygon's handles, in walk order.
    handles: Vec<FixedVertexHandle>,
    /// Every vertex exactly on a crease, with the crease and its
    /// position along it: the crease lines' constraint chain.
    on_crease: Vec<(f64, f64, FixedVertexHandle)>,
    /// The walk's pinch handles ([`Pinches`]).
    pinches: Pinches,
}

/// What the emit pass hands back: the attempt's patch, unless
/// refinement takes another round.
struct Emitted {
    interior: Vec<Point3<f64>>,
    triangles: Vec<[PatchVertex; 3]>,
    refine: Vec<(f64, f64)>,
    /// The grid candidates of triangles wound against the face
    /// ([`TrimmedFace::outward`]), to drop and rebuild; and whether a
    /// folded triangle had no grid corner to drop.
    folded: (Vec<usize>, bool),
    /// A point inside each kept triangle with a side a neighbouring face
    /// may make too ([`TrimmedFace::attempt`]), to insert and rebuild.
    shared_sides: Vec<(f64, f64)>,
}

impl TrimmedFace<'_> {
    /// **The plane the CDT runs in**: the cone's development on the
    /// face's nappe ([`nappe`]; `singular_row_shape` has refused a face
    /// on both), or the chart in the lattice metric.
    fn plane(&self) -> CdtPlane {
        CdtPlane::of(self.row, self.metric, self.polygon, self.singular)
    }

    /// `(u, v)` as the CDT sees it, off the chart's singular vertex
    /// ([`cdt_point`]).
    #[cfg(test)]
    fn cdt_point(&self, u: f64, v: f64) -> SpadePoint<f64> {
        cdt_point(self.plane(), u, v, false)
    }

    /// **The point a kept triangle's refinement or shared-side cure
    /// inserts**, inside the triangle where the CDT runs, in exact arithmetic: in
    /// the chart, the centroid of its corners. In the development, the centroid of its
    /// developed corners (`at`), read back to the chart: its angle by
    /// [`geom::Azimuth::near`] about the developed angle of a corner off
    /// the apex (a triangle has one, its angle at the apex under half a
    /// turn), its radius as its length. The chart centroid is no answer
    /// there: the development bends the chart's straight lines, and at
    /// an apex corner the chart holds the apex at one copy's `u`, which
    /// is any azimuth.
    ///
    /// **In f64 it is strictly inside down to a stated thinness**, held
    /// by `a_point_placed_inside_a_kept_triangle_is_strictly_inside_it`
    /// to spade's exact location: in the chart, a triangle at least four
    /// epsilons of its largest coordinate high; in the development, one
    /// whose height over its longest side is at least `2⁻²⁵` of its
    /// farthest radius, [`developed`]'s cut. A thinner developed triangle's
    /// point can land on its side, where the lane refuses
    /// (`work/tesslane/a-point-placed-inside-a-developed-needle-triangle-can-land-on-its-side.md`).
    fn inside_point(
        plane: CdtPlane,
        uv: &[[f64; 2]; 3],
        at: [SpadePoint<f64>; 3],
        apex: [bool; 3],
    ) -> (f64, f64) {
        match plane {
            CdtPlane::Lattice { .. } => uv_centroid(uv),
            CdtPlane::Development { sin_alpha, nappe } => {
                let g = SpadePoint::new(
                    (at[0].x + at[1].x + at[2].x) / 3.0,
                    (at[0].y + at[1].y + at[2].y) / 3.0,
                );
                let k = apex.iter().position(|&a| !a).unwrap_or(0);
                let turn = nappe * sin_alpha;
                // `Azimuth::of` takes `(y, x)`, atan2's order, and the
                // development puts a point at `r·(sin θ, cos θ)`: its first
                // coordinate is the angle's sine, so it goes first, and the
                // angle read is `θ` itself.
                let direction = geom::Azimuth::of(g.x, g.y);
                let theta = direction.near(turn * uv[k][0]);
                (theta / turn, nappe * direction.length())
            }
        }
    }

    /// The lattice the face lays ([`lay`]) at its clearance.
    fn lay(&self, nodes: &[Node]) -> Vec<(f64, f64)> {
        lay(
            nodes,
            self.clearance,
            self.plane(),
            self.polygon,
            self.singular,
            self.creases,
        )
    }

    /// The face's outward normal at the foot of `p` on its carrier, on
    /// an analytic arm (`geom_brep::implicit_outward_normal`); poison
    /// where the foot is not one point (the cylinder's axis, the
    /// sphere's centre). `None` on the NURBS arm, whose winding the
    /// census does not read. Its reader takes only the direction, the
    /// sign of a triangle normal's dot with it, so neither its unit
    /// length nor the raw gradient's magnitude is read.
    fn outward(&self, p: Point3<f64>) -> Option<OutwardNormal<f64>> {
        match self.row {
            ChartRow::Analytic(a) => Some(a.outward(self.sense, p)),
            ChartRow::Spline { .. } => None,
        }
    }

    /// One attempt of the retry loop (module docs): the points, the
    /// boundary constraints, the crease chain, the emit pass, then
    /// refinement or the verdict.
    fn attempt(
        &self,
        attempt: usize,
        candidates: &mut Vec<(f64, f64)>,
        dropped: &mut HashSet<usize>,
        measure: &mut AttemptMeasure,
        falsification: &mut Falsification,
    ) -> Result<Next, TessellateError> {
        let fk = self.fk;
        let mut at = self.insert_points(candidates, dropped)?;
        let crossings = self.constrain_boundary(&mut at)?;
        crate::region::require_a_region(fk, &crossings)?;
        let Some(orientation) = self.orientation else {
            return Err(TessellateError::TrimLoopOrientation { face: fk });
        };
        self.chain_crease_lines(&mut at)?;
        let Emitted {
            interior,
            triangles,
            refine,
            folded,
            shared_sides,
        } = self.emit(
            &at,
            &crossings,
            orientation,
            candidates,
            measure,
            falsification,
        )?;
        // THE SHARED-SIDE CURE (every arm). A triangle side joining two points
        // of the trim polygon is the same two mesh vertices whichever face makes
        // it. Where one neighbouring face holds both points and they are not
        // neighbours on the polygon, that face's triangulation may join them too
        // (a plane cap, whose triangulation has no interior point, joins its
        // boundary points freely): the side is then an interior edge of both
        // patches, an edge of four triangles. No patch sees that alone, since
        // each uses it twice, as any interior edge. So a face keeps no triangle
        // with such a side: a point strictly inside it where the CDT runs, and
        // so inside the polygon (`TrimmedFace::inside_point`), is inserted and
        // the face rebuilt. An ear of a chain (three consecutive points) is its
        // nearest case: its third side joins two points of one neighbour's
        // chain. A triangle the retry budget does not clear refuses typed. Where
        // a lane's side reaches four triangles anyway, the census where
        // `tessellate` joins the patches refuses it (`tessellate::joined`).
        if !shared_sides.is_empty() {
            if attempt == last_attempt() {
                return Err(TessellateError::UnclearedSharedSide { face: fk });
            }
            candidates.extend(shared_sides);
            return Ok(Next::Rebuild);
        }
        // THE WINDING CENSUS (every analytic arm): a triangle whose
        // normal points into the surface is folded over the chord beside
        // it — a lattice point inside the UV polygon but past the 3-D
        // chord, within the boundary arc's sagitta. Its grid corners are
        // dropped and the face rebuilt, which takes the fold's apex off
        // that side of the chord; a fold with no grid corner, or one
        // the retry budget does not clear, refuses typed.
        match folded {
            (_, true) => return Err(TessellateError::FoldedTriangle { face: fk }),
            (grid, false) if !grid.is_empty() => {
                if attempt == last_attempt() {
                    return Err(TessellateError::FoldedTriangle { face: fk });
                }
                dropped.extend(grid);
                return Ok(Next::Rebuild);
            }
            _ => {}
        }
        // Refinement retry (module docs): a centroid is strictly
        // interior to its kept triangle, hence inside the trim polygon;
        // splitting an offending triangle shrinks the empty region its
        // circumcircle spanned. Fires only for a triangle that would
        // otherwise REFUSE (bound > δ) — a RARE backstop: the lattice
        // metric (module docs) is the sliver cure, because an
        // insertion cannot converge against a strip
        // still anisotropic where the CDT runs (measured — anchors and
        // centroids each re-admit the empty circle beside themselves).
        // Appending keeps every existing candidate index (the `dropped`
        // set stays valid).
        if !refine.is_empty() && !measure.verdict.worst().is_nan() && attempt < last_attempt() {
            candidates.extend(refine);
            return Ok(Next::Rebuild);
        }
        measure.verdict.refusal(fk, self.tol.delta)?;
        // D2 addendum row 5, the same re-derivation `curved`'s emit
        // pass makes and for the same class (issue 897): an edge
        // incident to a vertex this walk placed at two distinct UV
        // locations, used more than twice inside ONE patch, is the
        // non-manifold state the duplicate-id drop in `emit` is claimed
        // never to produce. On an analytic chart this lane's sources of
        // such a vertex are one vertex on two branches, a seam's double
        // traversal (on one branch it is one location,
        // `one_location_per_branch`; a spline chart's entries are read
        // as walked), and a singular point's copies — the sphere's and the
        // spindle's pole sides, a spline chart's walked collapsed sides
        // ([`SidePoints`]), and the cone's apex, which its development holds as one CDT vertex, so
        // its fan's edges are used twice at most — so on every other face
        // the set is empty and the census returns on one branch.
        let patch = Patch {
            interior,
            triangles,
        };
        #[cfg(debug_assertions)]
        {
            let identified = crate::cdt::ids_at_two_uvs(self.polygon.points.iter().copied());
            // NOTHING IDENTIFIED, NOTHING MATERIALISED: `census_ids`
            // copies the whole patch, and this block runs in every
            // build with debug assertions on — the shipped release
            // profile included. On every face but a full-2π seam the
            // set is empty and this branch is the whole cost.
            if !identified.is_empty() {
                let over = crate::walk::overused_identified_edge_in(
                    &identified,
                    &patch.census_ids(self.shared),
                );
                debug_assert!(
                    over.is_none(),
                    "face {fk:?}: identified-vertex fan edge {:?} used {} times in one trimmed \
                     patch; the duplicate-id drop left something other than a fan — see \
                     curved::pole_columns, issue #678",
                    over.map(|(e, _)| e),
                    over.map_or(0, |(_, n)| n)
                );
            }
        }
        Ok(Next::Accept(patch))
    }

    /// The trim polygon's points, then the kept grid candidates, into a
    /// fresh triangulation, noting every point on a crease.
    fn insert_points(
        &self,
        candidates: &[(f64, f64)],
        dropped: &HashSet<usize>,
    ) -> Result<Cdt, TessellateError> {
        let fk = self.fk;
        // BULK-LOADED, not inserted one by one. Incremental insertion
        // along a dense boundary chain is quadratic in the flips it
        // cascades (a tilted circle near a sphere pole asks tens of
        // thousands of chord points, and their insertion took seconds per
        // attempt while constraining and emitting took milliseconds);
        // spade's bulk loader is `O(n log n)` and exact on the same
        // predicates. It drops duplicate positions keeping the first, as
        // `insert` answers the existing handle, so the duplicates are
        // dropped here first, by the same equality (`-0.0 == 0.0`), and
        // each input reads its handle off its unique position's index:
        // the loader keeps the input order.
        //
        // WHICH POINTS ARE ONE CDT VERTEX. The key is the point
        // `cdt_point` answers. In the chart that map is injective on
        // floored points, so two inputs share a vertex exactly when they
        // are one UV location ([`crate::cdt::ids_at_two_uvs`]'s reading):
        // a pinch's vertices, or a vertex the walk meets again on one
        // branch, which `one_location_per_branch` puts at its first
        // entry's UV on that branch. The
        // development rounds, and puts every copy of the apex at one
        // point on purpose: there the copies of the singular vertex share
        // one, and so do the points of a run ([`Run`]), which enters the
        // walk once, so its vertex is no pinch, and
        // which the emit pass tells apart again. A face whose polygon
        // points would share one any other way has refused before its
        // first attempt ([`developed_runs`]). A point of the mesher's
        // landing on an earlier point refuses below.
        let plane = self.plane();
        let inputs: Vec<(f64, f64, bool)> = self
            .polygon
            .points
            .iter()
            .map(|&(u, v, id)| (u, v, self.singular.contains(&id)))
            .chain(
                candidates
                    .iter()
                    .enumerate()
                    .filter(|(k, _)| !dropped.contains(k))
                    .map(|(_, &(u, v))| (u, v, false)),
            )
            .collect();
        let mut unique: Vec<SpadePoint<f64>> = Vec::with_capacity(inputs.len());
        let mut index_of: HashMap<(u64, u64), usize> = HashMap::with_capacity(inputs.len());
        let mut handle_of: Vec<FixedVertexHandle> = Vec::with_capacity(inputs.len());
        for &(u, v, singular) in &inputs {
            let p = cdt_point(plane, u, v, singular);
            let next = unique.len();
            let i = *index_of.entry(cdt_key(p)).or_insert(next);
            if i == next {
                unique.push(p);
            }
            handle_of.push(FixedVertexHandle::from_index(i));
        }
        let Ok(cdt) =
            ConstrainedDelaunayTriangulation::<SpadePoint<f64>>::bulk_load_cdt(unique, Vec::new())
        else {
            return Err(TessellateError::Triangulation { face: fk });
        };
        let mut meta: Vec<(f64, f64, Slot)> = Vec::new();
        let mut handles = Vec::with_capacity(self.polygon.points.len());
        let mut on_crease = Vec::new();
        let mut walk = Walk::new();
        let later_in_a_run: HashSet<usize> = self
            .runs
            .iter()
            .flat_map(|run| {
                let entry = run.entry();
                run.members.iter().copied().filter(move |&i| i != entry)
            })
            .collect();
        for (i, (&(u, v, id), &h)) in self.polygon.points.iter().zip(&handle_of).enumerate() {
            if later_in_a_run.contains(&i) {
                walk.skip();
            } else {
                walk.enter(&mut meta, h, (u, v, Slot::Boundary(id)), id);
            }
            handles.push(h);
            if let Some(c) = self.creases {
                c.note(&mut on_crease, (u, v), h);
            }
        }
        let pinches = Pinches::of(&walk, fk);
        let kept = candidates
            .iter()
            .enumerate()
            .filter(|(k, _)| !dropped.contains(k));
        for ((k, &(u, v)), &h) in kept.zip(&handle_of[self.polygon.points.len()..]) {
            // A point of the mesher's at an existing vertex. Nothing is
            // merged: the lattice keeps clear of the polygon (`lay`), and
            // a shared side's or a refinement's point is inside its triangle.
            if h.index() != meta.len() {
                return Err(TessellateError::Triangulation { face: fk });
            }
            meta.push((u, v, Slot::Grid(k)));
            if let Some(c) = self.creases {
                c.note(&mut on_crease, (u, v), h);
            }
        }
        Ok(Cdt {
            cdt,
            meta,
            handles,
            on_crease,
            pinches,
        })
    }

    /// The trim polygon's sides as constraints, with crossing counts
    /// (the planar #116 pattern).
    fn constrain_boundary(&self, at: &mut Cdt) -> Result<Crossings, TessellateError> {
        let fk = self.fk;
        let Cdt {
            cdt,
            meta,
            handles,
            pinches,
            ..
        } = at;
        let mut crossings: Crossings = HashMap::new();
        for i in 0..handles.len() {
            let j = self.polygon.next(i);
            let (a, b) = (handles[i], handles[j]);
            if a == b {
                continue;
            }
            let realised = cdt.try_add_constraint(a, b);
            if realised.is_empty() {
                return Err(TessellateError::Triangulation { face: fk });
            }
            if realised.len() > 1 {
                // An intermediate vertex (a vertex of a realised
                // sub-edge that is neither endpoint of THIS segment)
                // splits the side, and keeping it would emit a 3-D
                // T-junction: this face triangulated against a
                // sub-segment its neighbour does not share. A BOUNDARY
                // point there is the trim loop touching itself on this
                // segment. A point of the mesher's there is refused, not
                // moved: the lattice keeps clear of every side (`lay`),
                // and a shared side's or a refinement's point is inside its
                // triangle.
                for e in &realised {
                    let e = cdt.directed_edge(*e);
                    for vh in [e.from(), e.to()] {
                        let h = vh.fix();
                        if h == a || h == b {
                            continue;
                        }
                        let idx = h.index();
                        match meta[idx].2 {
                            Slot::Grid(_) => {
                                return Err(TessellateError::Triangulation { face: fk });
                            }
                            Slot::Boundary(_) => {
                                return Err(TessellateError::SelfTouchingTrimLoop { face: fk });
                            }
                        }
                    }
                }
            }
            for e in realised {
                let e = cdt.directed_edge(e);
                pinches.note_side(e, (a, i), (b, j))?;
                *crossings.entry(edge_key(e)).or_insert(0) += 1;
            }
        }
        Ok(crossings)
    }

    /// THE CREASE LINES (`Creases`): each crease is a chain of
    /// constraints through its vertices in order along it, so no
    /// triangle crosses it. Boundary crossings of a crease are
    /// vertices of the chain (`trim_polygon` refuses any other), so
    /// a link never crosses a boundary constraint, and the lattice lays
    /// no node within its clearance of a crease but the crease's own
    /// (`lay`), so a link runs through no vertex: one that does refuses.
    /// The links stay out of `crossings`: a crease is not a side of the
    /// face.
    fn chain_crease_lines(&self, at: &mut Cdt) -> Result<(), TessellateError> {
        match chain_creases(&mut at.cdt, &mut at.on_crease) {
            Some(through) if through.is_empty() => Ok(()),
            Some(_) | None => Err(TessellateError::Triangulation { face: self.fk }),
        }
    }

    /// The emit pass: even-odd interior flags, the kept grid points
    /// numbered, and every kept triangle certified, measured and
    /// queued for refinement when it certifies above δ.
    fn emit(
        &self,
        at: &Cdt,
        crossings: &Crossings,
        orientation: PolygonOrientation,
        candidates: &[(f64, f64)],
        measure: &mut AttemptMeasure,
        falsification: &mut Falsification,
    ) -> Result<Emitted, TessellateError> {
        let Cdt {
            cdt, meta, pinches, ..
        } = at;
        let (surface, shared, tol) = (self.surface, self.shared, self.tol);
        let plane = self.plane();
        // Even-odd interior flags, then emission with certificates.
        let inside = classify_faces(cdt, crossings);
        // The CDT's triangles are counter-clockwise in the chart; the
        // patch's take the polygon's orientation.
        let flip = orientation == PolygonOrientation::Clockwise;
        // Pass 1: which grid candidates the kept triangles use; number
        // them in (v, u) row-major order over the FINAL kept set —
        // sorted by coordinates, not by candidate index, so refinement
        // candidates appended by a later round (`attempt`) keep the same
        // determinism contract. The indices are this attempt's own: a
        // retry starts a fresh `interior` and only the accepted
        // attempt's becomes the patch.
        let mut used: Vec<usize> = Vec::new();
        for f in cdt.inner_faces() {
            if !inside[f.fix().index()] {
                continue;
            }
            for vtx in f.vertices() {
                if let (_, _, Slot::Grid(k)) = meta[vtx.fix().index()] {
                    used.push(k);
                }
            }
        }
        used.sort_unstable_by(|&a, &b| {
            let ((ua, va), (ub, vb)) = (candidates[a], candidates[b]);
            va.total_cmp(&vb).then(ua.total_cmp(&ub))
        });
        used.dedup();
        let mut interior: Vec<Point3<f64>> = Vec::with_capacity(used.len());
        let mut grid_ids: HashMap<usize, PatchVertex> = HashMap::new();
        for &k in &used {
            let (u, v) = candidates[k];
            #[allow(clippy::cast_possible_truncation)]
            let id = PatchVertex::Local(interior.len() as u32);
            interior.push(surface.eval(u, v));
            grid_ids.insert(k, id);
        }
        // Pass 2: emit and certify.
        //
        // The meter's arming is read ONCE per face, not per triangle:
        // this loop runs a quarter of a million times on #320's leaf,
        // and arming cannot change inside a face, so hoisting is
        // observationally identical. It is what makes the meter cost
        // exactly nothing per triangle when it is disarmed, which is
        // every normal run.
        // NURBS ONLY, and that is a coverage gap rather than a
        // subtlety: `cert::cert_cylinder` certifies every cylinder
        // triangle in BOTH lanes and no build samples one against it.
        // Left open rather than closed here because closing it changes
        // what a `FaceMeasure` means, which reaches the consumers of
        // `budget` in `tools/`.
        let dev_samples_per_edge = if matches!(self.row, ChartRow::Spline { .. }) {
            crate::budget::deviation_samples()
        } else {
            None
        };
        let mut triangles = Vec::new();
        // CERT-DRIVEN REFINEMENT (TESS-SPAN; module docs): NURBS
        // triangles certifying above the per-face sizing target get
        // their UV centroid queued as a new candidate for the next
        // round.
        let mut refine: Vec<(f64, f64)> = Vec::new();
        let refine_lane = matches!(self.row, ChartRow::Spline { .. });
        let (mut folded_grid, mut folded_boundary) = (Vec::new(), false);
        // Each boundary handle's places in the trim polygon (a pinch
        // handle holds several), for the shared-side test below.
        let n_poly = at.handles.len();
        let mut places: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, h) in at.handles.iter().enumerate() {
            places.entry(h.index()).or_default().push(i);
        }
        let mut shared_sides = Vec::new();
        // Each run's CDT vertex ([`Run`]).
        let run_at: HashMap<usize, &Run> = self
            .runs
            .iter()
            .map(|run| (at.handles[run.entry()].index(), run))
            .collect();
        for f in cdt.inner_faces() {
            if !inside[f.fix().index()] {
                continue;
            }
            let vs = f.vertices();
            let mut corners = [Corner {
                id: PatchVertex::Shared(0),
                uv: [0.0; 2],
                grid: None,
                places: None,
            }; 3];
            for (k, vtx) in vs.iter().enumerate() {
                let (u, v, slot) = meta[vtx.fix().index()];
                corners[k] = Corner {
                    id: match slot {
                        Slot::Boundary(id) => PatchVertex::Shared(id),
                        Slot::Grid(c) => grid_ids[&c],
                    },
                    uv: [u, v],
                    grid: match slot {
                        Slot::Grid(c) => Some(c),
                        Slot::Boundary(_) => None,
                    },
                    places: places.get(&vtx.fix().index()).map(Vec::as_slice),
                };
                match pinches.id_in(f, vtx.fix(), flip) {
                    Ok(Some(id)) => corners[k].id = PatchVertex::Shared(id),
                    Ok(None) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            let between = if run_at.is_empty() {
                Vec::new()
            } else {
                let hs = vs.map(|vtx| vtx.fix().index());
                cut_along_runs(self.polygon, &at.handles, &run_at, hs, &mut corners).map_err(
                    |i| TessellateError::SphereTrimShape {
                        face: self.fk,
                        edge: self.polygon.edges[i],
                        cause: SphereTrimCause::DevelopedMerge,
                    },
                )?
            };
            let developed = vs.map(|vtx| vtx.position());
            // The point the shared-side cure inserts for this CDT
            // triangle, whichever of the triangles cut from it asks: a
            // triangle between two points of a run has no inside where
            // the CDT runs, and the one it is cut from has. It is queued
            // once however many of them ask, since a location queued twice
            // enters the next attempt at an existing vertex, which
            // `insert_points` refuses `Triangulation`. No row has two of
            // one CDT triangle's asking: it takes a run whose anchored
            // triangle has, for its third corner, a polygon point that a
            // face across two of the run's points holds too.
            let inside_point = {
                let uv = corners.map(|c| c.uv);
                let apex = corners.map(|c| c.is_singular(self.singular));
                move || Self::inside_point(plane, &uv, developed, apex)
            };
            let mut a_shared_side = false;
            for corners in core::iter::once(corners).chain(between) {
                let ids = corners.map(|c| c.id);
                let uv = corners.map(|c| c.uv);
                // A triangle with two corners on one mesh vertex is
                // degenerate in 3-D. Dropping it is `curved`'s idiom, and
                // there it collapses a pole fan (#678). On the sphere's and
                // the spindle's arms it does the same at a pole side, and on
                // a spline chart at each piece of a walked collapsed side
                // ([`SidePoints`]); the cone's apex is one CDT vertex in its development, so no
                // triangle there has two corners on it; elsewhere the drop is
                // all it is, because neither ingredient of that fan reaches
                // the face — no singular point, and no repeated id at two UV
                // locations with a single interior column between them.
                //
                // WHAT IS ASSERTED, AND WHERE THE COVER STOPS. The singular
                // half is the row's: an analytic chart with singular points
                // holds its polygon to `singular_row_shape`'s shapes. A
                // spline chart's singular points are its collapsed sides, and
                // no shape check runs on that arm: what it relies on is the
                // census after this loop, which re-derives the fan about each
                // walked side's vertex. The INGREDIENT
                // half — a repeated id with a single interior column between
                // the entries — is checked at the cylinder arm's `nu`, kept
                // off at a singular point by `pole_side_columns`, and this
                // emit pass also runs for
                // NURBS, where there is no `nu`: the candidates come from
                // the cell grid rather than a uniform division, so the
                // "single interior column" has no analogue to test. That
                // arm's ingredient check is therefore still an ARGUMENT
                // and not a check.
                //
                // ITS EMISSION IS NOT (issue 897). The census after this
                // loop re-derives the CONCLUSION over the emitted patch,
                // and it needs no `nu` — it counts uses of the edges
                // incident to a repeated id, which every arm has. So the
                // NURBS arm, which had neither half, now has the half that
                // actually observes the defect.
                if ids[0] == ids[1] || ids[1] == ids[2] || ids[0] == ids[2] {
                    continue; // boundary-degenerate sliver
                }
                // A corner on a singular row has no azimuth
                // (`crate::chart_table`, the singular-row rule).
                let singular = corners.map(|c| c.is_singular(self.singular));
                let tri = [
                    ids[0].position(shared, &interior),
                    ids[1].position(shared, &interior),
                    ids[2].position(shared, &interior),
                ];
                // Wound as emitted below: reversed under `flip`.
                let wound = if flip { [tri[0], tri[2], tri[1]] } else { tri };
                let centroid = Point3::new(
                    (tri[0].x + tri[1].x + tri[2].x) / 3.0,
                    (tri[0].y + tri[1].y + tri[2].y) / 3.0,
                    (tri[0].z + tri[1].z + tri[2].z) / 3.0,
                );
                // A triangle a neighbouring face may make a side of too (the
                // cure is `attempt`'s): a side joining two polygon points
                // that are not neighbours on their loop and that one other
                // face holds both of.
                if n_poly >= 3 {
                    let on = corners.map(|c| c.places);
                    a_shared_side |= [(0, 1), (1, 2), (2, 0)].into_iter().any(|(a, b)| {
                        let (Some(pa), Some(pb)) = (on[a], on[b]) else {
                            return false;
                        };
                        let (PatchVertex::Shared(ia), PatchVertex::Shared(ib)) = (ids[a], ids[b])
                        else {
                            return false;
                        };
                        !self
                            .polygon
                            .faces_holding_side(self.across, (pa, ia), (pb, ib))
                            .is_empty()
                    });
                }
                // A triangle f64 cannot wind refuses the face at once
                // (`validate::resolved_normal`, `[mesh.winding]`): its normal
                // is noise, so it is neither outward nor a fold, and no retry
                // of this lane is built to resolve it (a fold's drop and a
                // shared side's split answer other causes). The join reads the same
                // definition for every lane; reading it here is what keeps
                // the sign below a resolved normal's.
                let normal = crate::validate::resolved_normal(wound).map_err(|unresolved| {
                    TessellateError::UnresolvedWinding {
                        face: self.fk,
                        unresolved: Box::new(unresolved),
                    }
                })?;
                // Folded unless wound outward: a zero dot, or NaN where
                // `outward` is poison, is a fold.
                if self.outward(centroid).is_some_and(|n| {
                    let wound_outward = normal.dot(n.vec()) > 0.0;
                    !wound_outward
                }) {
                    let grid: Vec<usize> = corners.iter().filter_map(|c| c.grid).collect();
                    folded_boundary |= grid.is_empty();
                    folded_grid.extend(grid);
                }
                let (bound, scale) = self.row.cert(tri, uv, singular);
                // The attempt's verdict takes the triangle in, its worst bound
                // a NaN kept once met (`cert::FaceVerdict`): what a refusal
                // reports is what refused.
                let verdict = measure.verdict.read(bound, tol.delta, scale, tol.eps);
                // THE DEVIATION PASS (`crate::budget`, issue #320):
                // barycentric samples of |S(w) − Π(w)| on this triangle,
                // reduced to the largest ratio against the triangle's OWN
                // certificate.
                //
                // INVARIANT: that maximum is the per-TRIANGLE claim, not a
                // per-face average — `worst_ratio ≤ 1` holds exactly when
                // every sample on every triangle was dominated by the
                // certificate of the triangle it was taken on. The suite
                // that drives the meter asserts it, and this lane does
                // not: no build of this crate may turn `tessellate`'s
                // typed-error contract into a panic.
                //
                // INVARIANT: this whole block is ABSENT from a default
                // build, and there is deliberately no `#[cfg]` here saying
                // so. `dev_samples_per_edge` comes from a module-gated
                // `deviation_samples()`; with the feature off that is a
                // `const fn` answering `None` in the module's inert half,
                // so the block is deleted outright. Do not convert this to
                // a `#[cfg]`: one lane in one version, shared by every
                // configuration, is the whole point.
                if let Some(m) = dev_samples_per_edge {
                    for a in 0..=m {
                        for b in 0..=(m - a) {
                            #[allow(clippy::cast_precision_loss)]
                            let (b0, b1) = (a as f64 / m as f64, b as f64 / m as f64);
                            let b2 = 1.0 - b0 - b1;
                            let (u, v) = (
                                b0 * uv[0][0] + b1 * uv[1][0] + b2 * uv[2][0],
                                b0 * uv[0][1] + b1 * uv[1][1] + b2 * uv[2][1],
                            );
                            let s = surface.eval(u, v);
                            let pi = Point3::new(
                                b0 * tri[0].x + b1 * tri[1].x + b2 * tri[2].x,
                                b0 * tri[0].y + b1 * tri[1].y + b2 * tri[2].y,
                                b0 * tri[0].z + b1 * tri[1].z + b2 * tri[2].z,
                            );
                            let d = ((s.x - pi.x).powi(2)
                                + (s.y - pi.y).powi(2)
                                + (s.z - pi.z).powi(2))
                            .sqrt();
                            // ε is the crate's documented promise: f64
                            // EVALUATION rounding sits outside the bound,
                            // and on a flat wall certifying at ~5e-17 a
                            // bare `d / cert` would read pure rounding
                            // dust as a violation.
                            let r = d / tol.eps.pad(bound);
                            // A first sample replaces the NaN seed; after
                            // that, max — sticky-NaN, the same rule the
                            // certificate accumulation below follows,
                            // because a `>` comparison silently DROPS a
                            // NaN and a NaN here is the loudest reading
                            // there is.
                            if measure.dev_samples == 0 || d.is_nan() || d > measure.worst_dev {
                                measure.worst_dev = d;
                                measure.worst_dev_cert = bound;
                            }
                            if falsification.ratio_samples == 0
                                || r.is_nan()
                                || r > falsification.worst_ratio
                            {
                                falsification.worst_ratio = r;
                            }
                            falsification.ratio_samples += 1;
                            measure.dev_samples += 1;
                        }
                    }
                }
                if refine_lane && verdict == cert::Verdict::Exceeded {
                    refine.push(inside_point());
                }
                triangles.push(if flip { [ids[0], ids[2], ids[1]] } else { ids });
            }
            if a_shared_side {
                shared_sides.push(inside_point());
            }
        }

        Ok(Emitted {
            interior,
            triangles,
            refine,
            folded: (folded_grid, folded_boundary),
            shared_sides,
        })
    }
}

/// **One CDT triangle with a run's vertex for a corner, cut into the
/// triangles the patch holds** ([`Run`]): `corners` are the triangle's, at
/// the CDT vertices `hs`, and `run_at` is each run by its vertex. A corner
/// at a run's vertex becomes the run's point this triangle stands on: in
/// the triangle on the run's anchored side the point next to the
/// anchoring neighbour, in every other the run's far end. Answered are
/// the triangles added between the run's points, which only the triangle
/// on the anchored side has: each on two consecutive points of the run
/// and that triangle's third corner, wound as the triangle they are cut
/// from. The run's fan about its vertex becomes a fan about each of its
/// points, and the side between two of them is a side of one triangle,
/// as a boundary side is.
///
/// **Two runs anchored by one neighbour** are each the other's third
/// corner: both beside the apex, the one point on two rulings. The
/// triangle is then cut along both. One run's triangles fan about the
/// other run's far end, which leaves the other's points and the first
/// run's point next to the apex, and the other's fan about that point.
/// Which run cuts first is free, and the triangle's corner order (the
/// CDT's, which replays) fixes it.
///
/// # Errors
///
/// The first polygon index of a run whose anchored triangle has for its
/// third corner a run anchored there by the first run's own vertex: a
/// neighbour, a run and the next run of one ruling in one triangle, where
/// the face refuses `SphereTrimShape { DevelopedMerge }`. Three points of
/// one chart column are exactly collinear where the CDT runs and make no
/// triangle, so no body measured reaches this; a neighbour that sits a
/// rounding off the column could make one
/// (`work/tesslane/a-rulings-end-vertex-read-through-its-rims-image-is-off-the-rulings-exact-line.md`),
/// and the two fans have no one reading of it.
fn cut_along_runs<'c>(
    polygon: &TrimPolygon,
    handles: &[FixedVertexHandle],
    run_at: &HashMap<usize, &'c Run>,
    hs: [usize; 3],
    corners: &mut [Corner<'c>; 3],
) -> Result<Vec<[Corner<'c>; 3]>, usize> {
    // The run at each corner, and the corner anchoring it when this is
    // the triangle on its anchored side.
    let runs: [Option<(&'c Run, Option<usize>)>; 3] = core::array::from_fn(|k| {
        let run = *run_at.get(&hs[k])?;
        let by = handles[run.neighbour(polygon)].index();
        Some((run, (0..3).find(|&j| j != k && hs[j] == by)))
    });
    for (corner, at) in corners.iter_mut().zip(runs) {
        if let Some((run, by)) = at {
            *corner = run.point(polygon, run.corner(by.is_some()));
        }
    }
    let mut between = Vec::new();
    for k in 0..3 {
        let Some((run, Some(by))) = runs[k] else {
            continue;
        };
        let third = 3 - k - by;
        let mut fan = *corners;
        match runs[third] {
            Some((other, Some(common))) if common == by => {
                let cuts_first = k < third;
                if cuts_first {
                    fan[third] = other.point(polygon, other.corner(false));
                }
            }
            Some((_, Some(_))) => return Err(run.members[0]),
            Some((_, None)) | None => {}
        }
        let (first, second) = match run.anchor {
            Anchor::Before => (by, k),
            Anchor::After => (k, by),
        };
        for m in 0..run.members.len() - 1 {
            let mut t = fan;
            t[first] = run.point(polygon, m);
            t[second] = run.point(polygon, m + 1);
            between.push(t);
        }
    }
    Ok(between)
}

/// One corner of a triangle the emit pass makes.
#[derive(Clone, Copy)]
struct Corner<'a> {
    id: PatchVertex,
    /// Its chart point.
    uv: [f64; 2],
    /// The grid candidate it is, if one.
    grid: Option<usize>,
    /// Its places in the trim polygon, if a boundary point: a pinch
    /// handle's several, a run's point's own.
    places: Option<&'a [usize]>,
}

impl Corner<'_> {
    /// Whether the corner is a copy of the chart's singular vertex.
    fn is_singular(&self, singular: &HashSet<u32>) -> bool {
        matches!(self.id, PatchVertex::Shared(id) if singular.contains(&id))
    }
}

/// **The faces across the edges of face `fk`'s loops (its outer loop and
/// its rings) at each boundary mesh id**: for every edge of a loop, the
/// face across it ([`ChordPass::across`], a view over the chord pass's
/// [`ChordPass::holders`]), at each of the edge's chord ids
/// (`crate::chords`), ends included, so a vertex's id holds the faces
/// across both edges meeting there. Two polygon points that one face here
/// holds both of are two points that face's own triangulation may join.
/// A face meeting `fk` at a corner alone is not here; a side it and `fk`
/// both make is the census's (`tessellate::overused_side`).
///
/// An edge a loop uses twice (a seam) has `fk` across it, and `fk` is
/// left out: a side between two of its points is this face's own, made
/// once by this one triangulation. A one-face full-turn cone wall
/// reaches that arm: its development holds the seam's two copies on the
/// sector's two rays ([`singular_row_shape`]).
///
/// This is an input of the face, and the memo key folds it: each edge's
/// face across, renumbered by first appearance along the walk
/// (`memo::EdgeInputs::across`), read off the same holders.
fn faces_across(
    body: &Body<f64>,
    fk: FaceKey,
    chords: &ChordPass,
) -> Result<HashMap<u32, Vec<FaceKey>>, TessellateError> {
    let mut out: HashMap<u32, Vec<FaceKey>> = HashMap::new();
    let face = body
        .face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;
    let mut sides = Vec::new();
    for (lk, _, _) in face.loops() {
        sides.extend(crate::walk::loop_half_edges(body, lk, fk)?);
    }
    for (_, ek, _) in sides {
        let Some(other) = chords.across(ek, fk)? else {
            continue;
        };
        let ids = chords.ids.get(&ek).ok_or(TessellateError::MissingEntity {
            what: "edge chords",
        })?;
        for &id in ids {
            let at = out.entry(id).or_default();
            if !at.contains(&other) {
                at.push(other);
            }
        }
    }
    Ok(out)
}

/// `(u, v)` as the CDT sees it on `plane`, `singular` where the point
/// is a copy of the chart's singular vertex.
///
/// - **The chart** ([`CdtPlane::Lattice`]): in the lattice metric
///   ([`crate::cdt::lattice_point`]; module docs, "The lattice metric",
///   on what that scale does and does not leave unchanged).
/// - **The cone's development** ([`CdtPlane::Development`], module
///   docs, "The development"): every copy of the apex at the origin, any
///   other point at [`developed`], then floored. The development rounds,
///   so it is not injective: [`TrimmedFace::insert_points`] says which
///   points it lets share a vertex.
///
/// Only the CDT sees these: `meta`, the creases, the polygon's
/// orientation and every certificate read UV.
fn cdt_point(plane: CdtPlane, u: f64, v: f64, singular: bool) -> SpadePoint<f64> {
    match plane {
        CdtPlane::Lattice { metric } => crate::cdt::lattice_point(u, v, metric),
        CdtPlane::Development { .. } if singular => crate::cdt::spade_point(0.0, 0.0),
        CdtPlane::Development { sin_alpha, nappe } => {
            let (x, y) = developed(sin_alpha, nappe, u, v);
            crate::cdt::spade_point(x, y)
        }
    }
}

/// **Which CDT vertex a point is**: two points are one vertex exactly
/// when their keys are equal, spade's own equality (`-0.0 == 0.0`).
/// [`TrimmedFace::insert_points`] builds the triangulation's vertices by
/// it and [`developed_runs`] finds the points that share one.
fn cdt_key(p: SpadePoint<f64>) -> (u64, u64) {
    let key = |x: f64| if x == 0.0 { 0u64 } else { x.to_bits() };
    (key(p.x), key(p.y))
}

/// **The lattice the face lays**: the nodes of `nodes` (row-major,
/// [`sort_nodes`]) whose clearance box meets no side of the trim
/// polygon and no crease line, as chart points, row-major, each
/// location once (`crates/mesh/README.md` `[mesh.sizing]`, "The
/// lattice keeps clear of the boundary"). A node's box is `c`
/// ([`BOUNDARY_CLEARANCE`]) of its own step each way, per chart axis
/// ([`Side::meets`] says in which plane it is read). Every loop's
/// sides are read, a ring's, a period seam's and a singular row's as
/// any other. A crease's own point on a grid line
/// ([`Creases::cut_candidates`], [`Node::on_crease`]) is held clear of
/// the polygon alone. One location laid more than once is laid where
/// each copy clears ([`Node`], "its own step").
///
/// Nothing is merged or moved: a node is laid or it is not, and the
/// decision is made before the first attempt.
fn lay(
    nodes: &[Node],
    c: f64,
    plane: CdtPlane,
    polygon: &TrimPolygon,
    singular: &HashSet<u32>,
    creases: Option<&Creases>,
) -> Vec<(f64, f64)> {
    let mut near = vec![false; nodes.len()];
    // Each row's run of `nodes`, and the widest window a side's
    // search has to open either way.
    let mut rows: Vec<(usize, usize)> = Vec::new();
    let (mut reach_u, mut reach_v) = (0.0f64, 0.0f64);
    for (k, n) in nodes.iter().enumerate() {
        match rows.last_mut() {
            Some((start, end)) if nodes[*start].v == n.v => *end = k + 1,
            _ => rows.push((k, k + 1)),
        }
        reach_u = reach_u.max(c * n.step.0);
        reach_v = reach_v.max(c * n.step.1);
    }
    let points = &polygon.points;
    for i in 0..points.len() {
        let side = Side::of(plane, points[i], points[polygon.next(i)], singular);
        let ((u_lo, u_hi), (v_lo, v_hi)) = side.window();
        let first = rows.partition_point(|&(start, _)| nodes[start].v < v_lo - reach_v);
        for &(start, end) in &rows[first..] {
            if nodes[start].v > v_hi + reach_v {
                break;
            }
            let from = start + nodes[start..end].partition_point(|n| n.u < u_lo - reach_u);
            for k in from..end {
                if nodes[k].u > u_hi + reach_u {
                    break;
                }
                near[k] = near[k] || side.meets(&nodes[k], c);
            }
        }
    }
    if let Some(creases) = creases {
        for (n, near) in nodes.iter().zip(&mut near) {
            let (x, h) = if creases.axis == 0 {
                (n.u, n.step.0)
            } else {
                (n.v, n.step.1)
            };
            *near = *near || !n.on_crease && creases.at.iter().any(|&at| (x - at).abs() <= c * h);
        }
    }
    // One location, every copy of it ([`Node`], "its own step"): a
    // crease's own copies where it has them, else all, and the location
    // is laid when each of those clears.
    let mut laid: Vec<(f64, f64)> = Vec::new();
    let mut start = 0;
    while start < nodes.len() {
        let at = (nodes[start].u, nodes[start].v);
        let end = start + nodes[start..].partition_point(|n| (n.u, n.v) == at);
        let copies = start..end;
        let crease = copies.clone().any(|k| nodes[k].on_crease);
        if copies
            .filter(|&k| nodes[k].on_crease == crease)
            .all(|k| !near[k])
        {
            laid.push(at);
        }
        start = end;
    }
    laid
}

/// **One side of the trim polygon, where the triangulation holds it
/// straight**, for [`TrimmedFace::lay`]'s clearance: a flat triangle is a
/// node on the line of a side in the plane the CDT runs in, so that is
/// the plane a node's box is read in.
///
/// - **In the chart** ([`CdtPlane::Lattice`]) the side is the chart
///   segment and the box the chart rectangle about the node. The lattice
///   metric scales one axis, and a box against a segment reads the same
///   under it.
/// - **In the cone's development** ([`CdtPlane::Development`]) the side
///   is the segment between its ends' developed points, which bows from
///   the chart's toward the apex, and the box is the image of the node's
///   chart rectangle: the annular sector of its azimuths and slant
///   lengths. Reading the cone's box against the chart's chord instead
///   differs by that bow, on either side of it, so neither reading is
///   the permissive one; no measured body tells them apart (no mesh or
///   digest of the suite moves at this clearance), and
///   `in_the_development_a_nodes_box_is_its_sector` is what holds the
///   choice.
#[derive(Clone, Copy, Debug)]
enum Side {
    Chart {
        a: (f64, f64),
        b: (f64, f64),
    },
    Developed {
        a: SpadePoint<f64>,
        b: SpadePoint<f64>,
        /// The developed angle per chart `u`, `σ·sin α`, and the nappe
        /// `σ` ([`developed`]).
        turn: f64,
        nappe: f64,
        /// The side's extent in the chart ([`Self::window`]).
        window: ((f64, f64), (f64, f64)),
    },
}

impl Side {
    /// The side from polygon point `a` to `b` on `plane`.
    fn of(
        plane: CdtPlane,
        a: (f64, f64, u32),
        b: (f64, f64, u32),
        singular: &HashSet<u32>,
    ) -> Self {
        match plane {
            CdtPlane::Lattice { .. } => Self::Chart {
                a: (a.0, a.1),
                b: (b.0, b.1),
            },
            CdtPlane::Development { sin_alpha, nappe } => {
                let apex = (singular.contains(&a.2), singular.contains(&b.2));
                let (pa, pb) = (
                    cdt_point(plane, a.0, a.1, apex.0),
                    cdt_point(plane, b.0, b.1, apex.1),
                );
                // A developed segment's azimuth runs monotonically between
                // its ends'. The apex has none, and every sector that
                // reaches the origin holds it, whatever its azimuths: a
                // side with an end there is searched at every `u`.
                let u = if apex.0 || apex.1 {
                    (f64::NEG_INFINITY, f64::INFINITY)
                } else {
                    (a.0.min(b.0), a.0.max(b.0))
                };
                let (near, far) = radii_sq(pa, pb, (0.0, 1.0));
                let (near, far) = (near.sqrt(), far.sqrt());
                let v = if nappe > 0.0 {
                    (near, far)
                } else {
                    (-far, -near)
                };
                Self::Developed {
                    a: pa,
                    b: pb,
                    turn: nappe * sin_alpha,
                    nappe,
                    window: (u, v),
                }
            }
        }
    }

    /// The side's extent in the chart, `((u_lo, u_hi), (v_lo, v_hi))`: no
    /// node whose box lies wholly outside it meets the side. Unbounded in
    /// `u` for a side with an end at the cone's apex.
    fn window(&self) -> ((f64, f64), (f64, f64)) {
        match *self {
            Self::Chart { a, b } => ((a.0.min(b.0), a.0.max(b.0)), (a.1.min(b.1), a.1.max(b.1))),
            Self::Developed { window, .. } => window,
        }
    }

    /// Whether the box `c` of `node`'s own step each way, per chart axis,
    /// meets the side (both closed).
    fn meets(&self, node: &Node, c: f64) -> bool {
        let half = (c * node.step.0, c * node.step.1);
        match *self {
            Self::Chart { a, b } => {
                // Separating axes of a segment and a rectangle: the two
                // chart axes, and the segment's normal.
                let ((u_lo, u_hi), (v_lo, v_hi)) = self.window();
                if u_lo > node.u + half.0
                    || u_hi < node.u - half.0
                    || v_lo > node.v + half.1
                    || v_hi < node.v - half.1
                {
                    return false;
                }
                let d = (b.0 - a.0, b.1 - a.1);
                let cross = d.0 * (node.v - a.1) - d.1 * (node.u - a.0);
                cross.abs() <= half.0 * d.1.abs() + half.1 * d.0.abs()
            }
            Self::Developed {
                a, b, turn, nappe, ..
            } => {
                // The sector: azimuths between the box's two, a wedge
                // narrower than a half turn and so two half-planes; slant
                // lengths between the box's two.
                let (t0, t1) = (turn * (node.u - half.0), turn * (node.u + half.0));
                let (lo, hi) = (t0.min(t1), t0.max(t1));
                let rho = nappe * node.v;
                let (r0, r1) = ((rho - half.1).max(0.0), rho + half.1);
                // The developed point at angle `θ` is `r·(sin θ, cos θ)`,
                // so `(cos θ, −sin θ)` points the way `θ` grows.
                let past = |p: SpadePoint<f64>, theta: f64| {
                    let (sin, cos) = <f64 as geom_core::Real>::sin_cos(theta);
                    p.x * cos - p.y * sin
                };
                let Some(span) = clip(past(a, lo), past(b, lo), (0.0, 1.0))
                    .and_then(|span| clip(-past(a, hi), -past(b, hi), span))
                else {
                    return false;
                };
                let (near, far) = radii_sq(a, b, span);
                near <= r1.powi(2) && far >= r0.powi(2)
            }
        }
    }
}

/// The part of `(t0, t1)` where the linear function running from `ga` at
/// `t = 0` to `gb` at `t = 1` is not negative.
fn clip(ga: f64, gb: f64, (t0, t1): (f64, f64)) -> Option<(f64, f64)> {
    let (lo, hi) = match (ga >= 0.0, gb >= 0.0) {
        (true, true) => (t0, t1),
        (false, false) => return None,
        (true, false) => (t0, t1.min(ga / (ga - gb))),
        (false, true) => (t0.max(ga / (ga - gb)), t1),
    };
    (lo <= hi).then_some((lo, hi))
}

/// The least and greatest squared distance from the origin of the
/// segment `a`–`b` over its parameters `(t0, t1)`.
fn radii_sq(a: SpadePoint<f64>, b: SpadePoint<f64>, (t0, t1): (f64, f64)) -> (f64, f64) {
    let d = (b.x - a.x, b.y - a.y);
    let at = |t: f64| {
        let (x, y) = (a.x + t * d.0, a.y + t * d.1);
        x.powi(2) + y.powi(2)
    };
    let len = d.0.powi(2) + d.1.powi(2);
    let foot = if len > 0.0 {
        (-(a.x * d.0 + a.y * d.1) / len).clamp(t0, t1)
    } else {
        t0
    };
    (at(foot), at(t0).max(at(t1)))
}

/// The plane a trimmed face's CDT runs in ([`cdt_point`]).
#[derive(Clone, Copy, Debug)]
enum CdtPlane {
    /// The chart in the lattice metric `k` (module docs, "The lattice
    /// metric").
    Lattice { metric: f64 },
    /// The cone's development (module docs, "The development"):
    /// `sin α`, and the face's nappe `σ = ±1` ([`nappe`]).
    Development { sin_alpha: f64, nappe: f64 },
}

impl CdtPlane {
    /// The plane of a face on chart `row` with lattice metric `metric`
    /// and trim polygon `polygon` ([`TrimmedFace::plane`]).
    fn of(row: &ChartRow<'_>, metric: f64, polygon: &TrimPolygon, singular: &HashSet<u32>) -> Self {
        match row {
            ChartRow::Analytic(a) => match a.development() {
                Some(sin_alpha) => Self::Development {
                    sin_alpha,
                    nappe: nappe(&polygon.points, singular),
                },
                None => Self::Lattice { metric },
            },
            ChartRow::Spline { .. } => Self::Lattice { metric },
        }
    }
}

/// The nappe a cone face lies on, `+1` or `−1`: the sign of `v` at its
/// points off the apex (`singular_row_shape` has refused a face with
/// points on both).
fn nappe(polygon: &[(f64, f64, u32)], singular: &HashSet<u32>) -> f64 {
    let below = polygon
        .iter()
        .any(|&(_, v, id)| !singular.contains(&id) && v < 0.0);
    if below { -1.0 } else { 1.0 }
}

/// **A run of the trim polygon the development holds as one CDT
/// vertex**: consecutive points of one loop along one ruling that
/// [`developed`]'s cut puts at one point. The CDT
/// triangulates with the one vertex, and the emit pass gives each of its
/// triangles the run's point it stands on and adds the triangles between
/// the run's points ([`TrimmedFace::emit`]).
#[derive(Debug, PartialEq, Eq)]
struct Run {
    /// The run's points, as indices into the polygon, in walk order.
    members: Vec<usize>,
    anchor: Anchor,
}

/// **Which neighbour of a [`Run`] on its loop the run's triangles are
/// added beside**: the one the polygon's side to it is carried by a line,
/// as the sides between the run's points are, so the one on the run's
/// ruling. A run at a corner of the polygon has one such neighbour; a run
/// inside a ruling side has two and takes the first, a choice between two
/// valid triangulations.
///
/// **What stands behind the triangles added**, as behind every triangle,
/// is the certificate and the winding rule, read on the triangle made.
/// The anchor is what makes them have area: the CDT triangle on the side
/// to it has two corners on the ruling, so its third, which the added
/// triangles fan about, is off it. That is exact where the neighbour's
/// chart column is the run's bit for bit ([`developed`]), and to a
/// rounding where the neighbour is an end vertex read through its rim's
/// image.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Anchor {
    /// The point before the run's first.
    Before,
    /// The point after the run's last.
    After,
}

impl Run {
    /// **The run's point that stands for it**, as a polygon index: the
    /// one that enters the boundary walk, whose handle is the run's CDT
    /// vertex. It is the run's lowest index, since handles are numbered as
    /// their points first appear and `Cdt::meta` holds a row per handle in
    /// that order.
    fn entry(&self) -> usize {
        self.members
            .iter()
            .copied()
            .min()
            .unwrap_or_else(|| unreachable!("a run holds two points at least"))
    }

    /// The polygon point the run is anchored by ([`Anchor`]).
    fn neighbour(&self, polygon: &TrimPolygon) -> usize {
        match self.anchor {
            Anchor::Before => polygon.prev(self.members[0]),
            Anchor::After => polygon.next(self.members[self.members.len() - 1]),
        }
    }

    /// The run's point `m`, as a triangle's corner.
    fn point(&self, polygon: &TrimPolygon, m: usize) -> Corner<'_> {
        let (u, v, id) = polygon.points[self.members[m]];
        Corner {
            id: PatchVertex::Shared(id),
            uv: [u, v],
            grid: None,
            places: Some(&self.members[m..=m]),
        }
    }

    /// The run's point a triangle at its vertex stands on, as an index
    /// into `members`: the one next to the anchoring neighbour in the
    /// triangle on the anchored side, and the run's other end in every
    /// other triangle.
    fn corner(&self, anchored: bool) -> usize {
        match (self.anchor, anchored) {
            (Anchor::Before, true) | (Anchor::After, false) => 0,
            (Anchor::Before, false) | (Anchor::After, true) => self.members.len() - 1,
        }
    }
}

/// **The runs of the polygon the development holds as one CDT vertex
/// each** (module docs, "The development", which points are one CDT
/// vertex); none in the chart, whose map is injective on floored points.
/// The copies of the singular vertex share the origin, and points at one
/// UV location (a pinch's vertices, or a vertex the walk meets again)
/// share a point as in the chart. Any other polygon points that the
/// development's rounding or spade's floor put at one point are mesh
/// vertices the CDT holds as one, and they are a [`Run`] when, read from
/// the polygon's own order and never from a distance:
///
/// - they are every polygon point at that CDT point, consecutive on one
///   loop, and not the whole of it but two points at least, which lie at
///   two CDT points;
/// - none is the singular vertex, and no two are one mesh vertex;
/// - the polygon's side from each to the next is carried by a line
///   (`straight`, by the side's first point), which on a cone is a
///   ruling, and they run along it one way (`v` strictly monotone);
/// - the side to a neighbour of the run on its loop is carried by a line
///   too ([`Anchor`]).
///
/// # Errors
///
/// The index of the first polygon point merged with one at another UV
/// location in a group that is no run, where the face refuses
/// `SphereTrimShape { DevelopedMerge }`: a point merged with the apex,
/// points of two sides or of two loops, a ruling side wholly within the
/// cut.
fn developed_runs(
    plane: CdtPlane,
    trim: &TrimPolygon,
    singular: &HashSet<u32>,
    straight: &dyn Fn(usize) -> bool,
) -> Result<Vec<Run>, usize> {
    if let CdtPlane::Lattice { .. } = plane {
        return Ok(Vec::new());
    }
    let polygon = &trim.points;
    let keys: Vec<(u64, u64)> = polygon
        .iter()
        .map(|&(u, v, id)| cdt_key(cdt_point(plane, u, v, singular.contains(&id))))
        .collect();
    let mut first: HashMap<(u64, u64), usize> = HashMap::new();
    let mut merged: Vec<usize> = Vec::new();
    for (i, &(u, v, id)) in polygon.iter().enumerate() {
        let was = *first.entry(keys[i]).or_insert(i);
        let (wu, wv, wid) = polygon[was];
        let one_location = crate::cdt::spade_point(wu, wv) == crate::cdt::spade_point(u, v);
        if !(one_location || singular.contains(&id) && singular.contains(&wid)) {
            merged.push(i);
        }
    }
    let mut runs: Vec<Run> = Vec::new();
    let mut seen: HashSet<(u64, u64)> = HashSet::new();
    for i in merged {
        if !seen.insert(keys[i]) {
            continue;
        }
        let group: Vec<usize> = (0..polygon.len()).filter(|&j| keys[j] == keys[i]).collect();
        runs.push(run_of(trim, &keys, &group, singular, straight).ok_or(i)?);
    }
    Ok(runs)
}

/// Whether edge `ek` is carried by a line, which on a cone is a ruling:
/// what [`developed_runs`] reads of the polygon side an edge carries.
fn carried_by_a_line(body: &Body<f64>, ek: EdgeKey) -> bool {
    edge_carrier(body, ek).is_ok_and(|carrier| !carrier.is_curved())
}

/// The [`Run`] that `group`, every polygon point at one CDT point, is;
/// `None` where it is not one ([`developed_runs`] states the conditions).
fn run_of(
    trim: &TrimPolygon,
    keys: &[(u64, u64)],
    group: &[usize],
    singular: &HashSet<u32>,
    straight: &dyn Fn(usize) -> bool,
) -> Option<Run> {
    let polygon = &trim.points;
    let inside: HashSet<usize> = group.iter().copied().collect();
    let mut starts = group.iter().filter(|&&i| !inside.contains(&trim.prev(i)));
    let (Some(&start), None) = (starts.next(), starts.next()) else {
        return None;
    };
    let mut members = vec![start];
    while members.len() < group.len() {
        let next = trim.next(members[members.len() - 1]);
        if !inside.contains(&next) {
            return None;
        }
        members.push(next);
    }
    let ids: HashSet<u32> = members.iter().map(|&i| polygon[i].2).collect();
    if ids.len() != members.len() || ids.iter().any(|id| singular.contains(id)) {
        return None;
    }
    let last = members[members.len() - 1];
    let up = polygon[members[1]].1 > polygon[start].1;
    let along = members.windows(2).all(|w| {
        let (va, vb) = (polygon[w[0]].1, polygon[w[1]].1);
        straight(w[0]) && if up { vb > va } else { vb < va }
    });
    let (before, after) = (trim.prev(start), trim.next(last));
    if !along || keys[before] == keys[after] {
        return None;
    }
    let anchor = if straight(before) {
        Anchor::Before
    } else if straight(last) {
        Anchor::After
    } else {
        return None;
    };
    Some(Run { members, anchor })
}

/// **The developed point of chart point `(u, v)` on the cone of
/// half-angle `α` (`sin_alpha`), on nappe `σ`**: radius `r = σ·v` (the
/// slant length, `geom::Surface::Cone`), angle `θ = σ·sin α·u`, at
/// `r·(sin θ, cos θ)`. It is an isometry of the nappe onto a plane
/// sector, and it keeps the chart's orientation on either nappe
/// (Jacobian `r·sin α > 0`), so the polygon's orientation is the CDT's.
///
/// **ONE RULING, ONE LINE, EXACTLY.** The direction's components are
/// cut to 26 significant bits and the radius to 27, so each product is
/// exact: every point of one chart column (one `u`, bit for bit) is an
/// exact multiple of one direction, on one line through the origin, and
/// spade's exact orientation predicate reads any three of them, or two
/// and the apex, as collinear. A CDT has no triangle on three collinear
/// points, so no triangle of the face lies along a column. That is a
/// ruling wherever its points share their `u` bit for bit, which an end
/// vertex read through its rim's image need not (module docs, "The
/// development"). The cut moves a point by under `2⁻²⁵` of its radius,
/// which the CDT alone sees.
///
/// `sin` and `cos` are [`geom_core::Real`]'s, the pure-Rust `libm`
/// every platform shares (D9), so the triangulation they decide replays
/// bit for bit.
fn developed(sin_alpha: f64, nappe: f64, u: f64, v: f64) -> (f64, f64) {
    let (sin, cos) = <f64 as geom_core::Real>::sin_cos(nappe * sin_alpha * u);
    let r = significant(nappe * v, 27);
    (r * significant(sin, 26), r * significant(cos, 26))
}

/// `x` cut toward zero to its leading `bits` significant bits (of the
/// 53 a normal `f64` holds).
fn significant(x: f64, bits: u32) -> f64 {
    f64::from_bits(x.to_bits() & !((1u64 << (53 - bits)) - 1))
}

/// A triangle's centroid in UV: the point both the shared-side cure and the
/// certificate refinement insert in the chart, strictly inside the
/// triangle and so inside the trim polygon.
fn uv_centroid(uv: &[[f64; 2]; 3]) -> (f64, f64) {
    (
        (uv[0][0] + uv[1][0] + uv[2][0]) / 3.0,
        (uv[0][1] + uv[1][1] + uv[2][1]) / 3.0,
    )
}

/// Whether any mesh id appears at two DIFFERENT UV locations in the
/// trim polygon — the second ingredient of #678's non-manifold fan
/// (`curved::pole_columns` carries the argument; the first ingredient
/// is a single interior column between them).
///
/// The rule for "different" is spade's and is stated once, at
/// [`crate::cdt::ids_at_two_uvs`], which both lanes call. This
/// function is that set's emptiness, named for the question the
/// cylinder arm's `debug_assert` asks.
fn id_repeats_apart(polygon: &[(f64, f64, u32)]) -> bool {
    !crate::cdt::ids_at_two_uvs(polygon.iter().copied()).is_empty()
}

/// **A vertex the walk meets again on one chart branch is one UV
/// location**: each entry of a mesh id takes the `(u, v)` of that id's
/// first entry on its branch, bit for bit. A later entry is on an
/// earlier entry's branch where [`Period::near`] renames it onto that
/// entry's copy by no whole period, in every angle channel (`near`'s
/// half-period marks decide it, and it answers the entry itself, bit for
/// bit, on its own copy); a length channel has one branch. An entry on
/// no branch met yet starts one. The polygon reads each entry off its
/// own traversal's pcurve, and two images of one vertex agree only to
/// rounding; pinned, the CDT holds the vertex as one handle per branch,
/// the boundary pinch [`Pinches`] meshes. So a mesh id at two UV
/// locations is a singular vertex's copies or one vertex on two
/// branches, at least half a period apart.
///
/// A spline chart (`periods` `None`) is read as walked: its periods are
/// its knot domains' and are decided at a band, which this pass has not.
fn one_location_per_branch(
    mut trim: TrimPolygon,
    periods: Option<geom::AnglePeriods<f64>>,
    singular: &HashSet<u32>,
) -> TrimPolygon {
    let Some((pu, pv)) = periods else {
        return trim;
    };
    #[allow(clippy::float_cmp)] // `near` answers `t` itself, bit for bit, on its own copy
    let on_copy =
        |p: Option<Period<f64>>, t: f64, first: f64| p.is_none_or(|p| p.near(t, first) == t);
    let mut branches: HashMap<u32, Vec<(f64, f64)>> = HashMap::new();
    for point in &mut trim.points {
        if singular.contains(&point.2) {
            continue;
        }
        let firsts = branches.entry(point.2).or_default();
        match firsts
            .iter()
            .find(|&&(u, v)| on_copy(pu, point.0, u) && on_copy(pv, point.1, v))
        {
            Some(&(u, v)) => (point.0, point.1) = (u, v),
            None => firsts.push((point.0, point.1)),
        }
    }
    trim
}

/// **What a face's polygon is read from** ([`face_polygon`]): the face
/// and its chart, its lift, and what the run shares (the chord pass and
/// its positions, the band, δ).
struct PolygonOf<'a> {
    body: &'a Body<f64>,
    fk: FaceKey,
    surface: &'a Surface<f64>,
    row: &'a ChartRow<'a>,
    lift: &'a FaceLift<f64>,
    creases: Option<&'a Creases>,
    chords: &'a ChordPass,
    shared: &'a [Point3<f64>],
    band: Band,
    delta: f64,
}

/// **The face's singular vertices and its UV polygon** ([`tessellate_trimmed`]).
///
/// The singular vertices are the loop's vertices that are the chart's
/// singular points, decided at the run's band by the one read of that
/// incidence (`chart_table`'s `at_a_singular_point`, through
/// `geom_brep::chart_singularity::vertex_at_singular_point`): an analytic
/// chart's poles and apexes, a spline chart's collapsed sides
/// ([`CollapsedSides`]). It is the read topo's lift makes where it turns the
/// face at a vertex (`pcurves` `singular_at`), on the same points at the
/// same band. The lift does not store that decision: a joint at a singular
/// point stores an ordinary deck element, as every joint does, so the mesh
/// reads the incidence again rather than reading it off the lift. Each
/// vertex's mesh id is its edges' end chord id.
///
/// The polygon: per half-edge, the pcurve evaluated at the shared chord
/// parameters (each traversal contributes all but its last point; ids
/// stay the shared 3-D chord ids), every loop, a spline chart's
/// collapsed sides walked at the face's chord steps at `delta`
/// ([`SidePoints`]), the steps its rims are chorded at. Every loop has at
/// least three points: the chord pass's short-loop floor (`chords`).
fn face_polygon(of: &PolygonOf<'_>) -> Result<(HashSet<u32>, TrimPolygon), TessellateError> {
    let PolygonOf {
        body,
        fk,
        surface,
        row,
        lift,
        creases,
        chords,
        shared,
        band,
        delta,
    } = *of;
    let sides = CollapsedSides::of(surface);
    let singular = match row {
        ChartRow::Analytic(a) if !a.singular_points().is_empty() => {
            singular_vertices(body, fk, lift, chords, shared, |p| a.is_singular(p, band))?
        }
        ChartRow::Spline { .. } if !sides.is_empty() => {
            singular_vertices(body, fk, lift, chords, shared, |p| sides.holds(p, band))?
        }
        ChartRow::Analytic(_) | ChartRow::Spline { .. } => HashSet::new(),
    };
    let walk = match row {
        ChartRow::Spline { .. } if !singular.is_empty() => sides.points(
            shared,
            &singular,
            row.steps(face_target(surface, Lane::Trimmed, delta)),
            band,
        ),
        ChartRow::Analytic(_) | ChartRow::Spline { .. } => SidePoints::default(),
    };
    let spline = matches!(row, ChartRow::Spline { .. });
    let polygon = one_location_per_branch(
        trim_polygon(body, fk, lift, chords, spline, creases, (&singular, &walk))?,
        surface.angle_periods(),
        &singular,
    );
    Ok((singular, polygon))
}

/// **The loops' vertices that are the chart's singular points**, as mesh
/// ids, a ring's as an outer loop's: each loop edge's end chord ids (the
/// vertices' own points, bitwise, `crate::chords`), kept where
/// `is_singular` decides the point is one: [`AnalyticRow::is_singular`]
/// or [`CollapsedSides::holds`], each at the run's band, the decision the
/// loop's lift takes to turn the face there (`chart_table`'s module
/// docs), read on the same points, so the polygon's sides at a singular
/// point are the lift's.
///
/// # Errors
///
/// `SphereTrimShape { SingularUndecided }` at a vertex `is_singular`
/// cannot place on or off a singular point, read before the polygon is
/// walked.
fn singular_vertices(
    body: &Body<f64>,
    fk: FaceKey,
    lift: &FaceLift<f64>,
    chords: &ChordPass,
    shared: &[Point3<f64>],
    is_singular: impl Fn(Point3<f64>) -> Result<bool, geom_core::Indeterminate>,
) -> Result<HashSet<u32>, TessellateError> {
    let mut out = HashSet::new();
    for lifted in every_loop(lift).flat_map(Lift::rows) {
        let ek = body
            .get_half_edge(lifted.half_edge)
            .ok_or(TessellateError::MissingEntity { what: "half-edge" })?
            .edge;
        let ids = chords.ids.get(&ek).ok_or(TessellateError::MissingEntity {
            what: "edge chords",
        })?;
        for id in [ids[0], ids[ids.len() - 1]] {
            let p = *shared
                .get(id as usize)
                .ok_or(TessellateError::MissingEntity {
                    what: "vertex position",
                })?;
            match is_singular(p) {
                Ok(true) => {
                    out.insert(id);
                }
                Ok(false) => {}
                Err(_) => {
                    return Err(TessellateError::SphereTrimShape {
                        face: fk,
                        edge: ek,
                        cause: SphereTrimCause::SingularUndecided,
                    });
                }
            }
        }
    }
    Ok(out)
}

/// **A polygon on a chart with singular points** (`crate::chart_table`):
/// a singular point the loop passes through is a vertex where two of its
/// edges meet from different `u` (one of the `singular` ids), and the
/// polygon holds it twice, once at each ([`flat_points`]), so the point's
/// row is a side of the polygon, as it is of the curved lane's walk. A CDT
/// triangle on two of that side's points has two corners on the one vertex
/// and the emit pass drops it. The triangles left there are manifold about
/// the vertex unless exactly one lattice column lies between the side's
/// ends (#678's shape), which [`pole_side_columns`] keeps the lattice off.
///
/// Refused typed, each a shape the lane does not mesh:
/// - a point past a row that ends the chart's `v` domain
///   ([`SingularPoint::passed_by`]): an edge whose image runs through the
///   singular point mid-edge;
/// - a mesh id at two chart locations that is no singular vertex (a seam
///   traversed twice), where the CDT runs in the lattice (`developed`
///   false): there the two copies are two columns of one lattice, and a
///   triangle across them is the collapse #678 is about. In the cone's
///   development (`developed` true) the seam's copies lie on the sector's
///   two bounding rays, which the face's interior keeps apart. A
///   triangle with a corner at each copy of one point lies outside the
///   face when the sector is wider than a half turn; inside a narrower
///   one it has two corners on one vertex and is dropped, as the apex
///   row's are, and the emit pass's census of identified edges re-derives
///   that what is left about the vertex is a fan.
///
/// A loop that winds about a singular point inside the face does not
/// close in the chart, and the door refuses it before any polygon is
/// drawn ([`TessellateError::DomainUnread`]). What is left is held by
/// the refusals here, not asserted beside them: past a row that ends the
/// chart's `v` domain no point is admitted (`PastPole`), so the open
/// region a closed loop bounds stops short of the row; and on both sides
/// of a row inside the domain no face is admitted (`BothNappes`), so a
/// cone's apex is at most a corner. A polygon these admit holds no
/// singular point strictly inside it, and a check of that would re-run
/// these two.
///
/// Each refusal is [`TessellateError::SphereTrimShape`] naming its
/// [`SphereTrimCause`] and the edge the offending point or side comes
/// from (`edges[i]` is point `i`'s, and side `i` runs from point `i` to
/// its successor on its loop). Every loop is read, a ring as the outer
/// loop.
fn singular_row_shape(
    fk: FaceKey,
    points: &[SingularPoint],
    singular: &HashSet<u32>,
    trim: &TrimPolygon,
    developed: bool,
) -> Result<(), TessellateError> {
    let (polygon, edges) = (trim.points.as_slice(), trim.edges.as_slice());
    let refuse = |i: usize, cause| -> Result<(), TessellateError> {
        Err(TessellateError::SphereTrimShape {
            face: fk,
            edge: edges[i],
            cause,
        })
    };
    if let Some(i) = polygon
        .iter()
        .position(|&(_, v, _)| points.iter().any(|r| r.passed_by(v)))
    {
        return refuse(i, SphereTrimCause::PastPole);
    }
    // A point inside the chart (the cone's apex) has a side either way
    // of its row, and a face keeps to one: its development is one
    // nappe's ([`nappe`]).
    for r in points.iter().filter(|r| r.end == RowEnd::Inside) {
        let off = |(_, v, id): (f64, f64, u32)| !singular.contains(&id) && v != r.v;
        let above = polygon.iter().position(|&p| off(p) && p.1 > r.v);
        let below = polygon.iter().position(|&p| off(p) && p.1 < r.v);
        if let (Some(a), Some(b)) = (above, below) {
            return refuse(a.max(b), SphereTrimCause::BothNappes);
        }
    }
    if developed {
        return Ok(());
    }
    let apart = crate::cdt::ids_at_two_uvs(polygon.iter().copied());
    if let Some(i) = polygon
        .iter()
        .position(|&(_, _, id)| apart.contains(&id) && !singular.contains(&id))
    {
        return refuse(i, SphereTrimCause::SeamTwice);
    }
    Ok(())
}

/// The lattice's column count over the trim box `(u0, u1)`: `nu`, doubled
/// until no side of `trim` at a `singular` vertex (a pole side,
/// [`singular_row_shape`]) has exactly one lattice
/// column strictly between its ends — the shape #678's non-manifold fan
/// needs (`curved::pole_columns` carries the argument). Doubling keeps
/// every column it had, so a side's count never falls, but it need not
/// rise past one at the first doubling (the new columns can land
/// outside a short side); the loop reads the count again after each, and
/// ends once the spacing is under half the shortest pole side, where
/// every side holds at least two. [`ceil_count`] caps the doubling.
fn pole_side_columns(
    trim: &TrimPolygon,
    singular: &HashSet<u32>,
    (u0, u1): (f64, f64),
    mut nu: usize,
) -> Result<usize, TessellateError> {
    let polygon = &trim.points;
    let sides: Vec<(f64, f64)> = (0..polygon.len())
        .map(|i| (polygon[i], polygon[trim.next(i)]))
        .filter(|&(a, b)| a.2 == b.2 && singular.contains(&a.2) && a.0 != b.0)
        .map(|(a, b)| (a.0.min(b.0), a.0.max(b.0)))
        .collect();
    let one_between = |nu: usize| {
        sides.iter().any(|&(a, b)| {
            (1..nu)
                .filter(|&i| {
                    #[allow(clippy::cast_precision_loss)]
                    let u = u0 + (u1 - u0) * (i as f64 / nu as f64);
                    u > a && u < b
                })
                .count()
                == 1
        })
    };
    while one_between(nu) {
        #[allow(clippy::cast_precision_loss)]
        let doubled = 2.0 * nu as f64;
        nu = ceil_count(doubled, 1.0)?;
    }
    Ok(nu)
}

/// The interior grid candidates of every analytic arm: the trim box's
/// `u` window divided into `nu` columns, at each of `rows`
/// ([`AnalyticRow::lattice_rows`]), interior points only (the box
/// boundary is at or outside the trim polygon). Generated in (v, u)
/// row-major order; sorted anyway so every arm hands the retry loop the
/// same invariant.
///
/// At `nu == 1` (or no row) a range below is empty and the other axis'
/// computed count is dropped — a member of the sibling class
/// `curved::grid_counts`' doc records (issue 685 decided only the
/// cone's `nu == 1` case; here the candidates are heuristic seeds and
/// the per-triangle certificate is the stated guarantee).
fn lattice_candidates(u: (f64, f64), v: (f64, f64), nu: usize, rows: &[f64]) -> Vec<Node> {
    #[allow(clippy::cast_precision_loss)]
    let hu = (u.1 - u.0) / nu as f64;
    let mut cand = Vec::new();
    for (k, &vv) in rows.iter().enumerate() {
        // A row's own step: the smaller of its gaps to the lines beside
        // it, the box's ends among them. Uniform rows have one gap; the
        // cone's graded rows have two that differ, and the smaller is the
        // permissive choice (a smaller box, more nodes laid). No measured
        // body tells it from the larger: no mesh or digest of the suite
        // moves between them at this clearance.
        let below = if k == 0 { v.0 } else { rows[k - 1] };
        let above = rows.get(k + 1).copied().unwrap_or(v.1);
        let step = (hu, (vv - below).abs().min((above - vv).abs()));
        for i in 1..nu {
            #[allow(clippy::cast_precision_loss)]
            let uu = u.0 + (u.1 - u.0) * (i as f64 / nu as f64);
            cand.push(Node {
                u: uu,
                v: vv,
                step,
                on_crease: false,
            });
        }
    }
    sort_nodes(&mut cand);
    cand
}

/// The NURBS lane's interior grid candidates: a **per-v-band
/// tensor** from the ONE shipped schedule derivation,
/// [`crate::nurbs_cert::NurbsCellGrid::band_schedule`] — per-band
/// `nuc × nvc` through the aspect-capped split selection
/// (TESS-SPLIT), each band at its own counts.
///
/// * Row lines land exactly on the band cuts (never through the lerp,
///   whose rounding would put two almost-coincident lines an ulp
///   apart on a shared boundary), so a grid triangle sits inside one
///   band and its certificate is that band's cells' (one or several
///   in `u`: the columns are uniform over the trim box, not on the `u`
///   cuts). A cut line gets
///   points from BOTH adjacent bands' column schedules (their union —
///   the shared cut value is the same f64 either side, so the dedup
///   merges the line itself).
/// * Points on the trim box boundary are excluded exactly as the
///   uniform grid excluded its `i = 0, nu` indices.
///
/// Also returns the grid-cell count `Σ nuc·nvc` over the bands — the
/// budget meter's `grid_cells` column, read off the sizing that
/// actually ran. A STATED approximation (dual-review MIN): a shared
/// cut line carries the union of both bands' column points, so the
/// point set is slightly denser than any one band's `nuc` accounts
/// for — `grid_cells` counts schedule CELLS, the triangle count
/// carries the realised total, and the gate compares both.
fn per_cell_candidates(
    grid: &NurbsCellGrid,
    u: (f64, f64),
    v: (f64, f64),
    budget: f64,
) -> Result<(Vec<Node>, ScheduleStats), TessellateError> {
    let mut cand: Vec<Node> = Vec::new();
    let mut stats = ScheduleStats {
        grid_cells: 0,
        bands: 0,
        cap_bands: 0,
        realized_aspect: f64::NAN,
        metric: 1.0,
    };
    let du = u.1 - u.0;
    let schedule = grid.band_schedule(u, v, budget)?;
    stats.metric = schedule.metric;
    for b in schedule.bands {
        stats.grid_cells += b.nuc * b.nvc;
        stats.bands += 1;
        stats.cap_bands += usize::from(b.cap);
        // The emitted lattice's post-`ceil` parameter aspect `s_u/s_v`
        // in raw UV, max over bands. The NaN
        // seed means "no bands yet" and the first band overwrites it;
        // every band's aspect is a finite positive ratio (nonempty
        // extents over counts floored at one).
        let aspect = b.aspect(du);
        stats.realized_aspect = if stats.realized_aspect.is_nan() {
            aspect
        } else {
            stats.realized_aspect.max(aspect)
        };
        // Band-boundary indices reproduce the cut values EXACTLY.
        let at = |lo: f64, hi: f64, i: usize, n: usize| -> f64 {
            if i == 0 {
                lo
            } else if i == n {
                hi
            } else {
                #[allow(clippy::cast_precision_loss)]
                {
                    lo + (hi - lo) * (i as f64 / n as f64)
                }
            }
        };
        // The band's own steps: a node on a cut line is laid by each
        // band beside it, at that band's.
        #[allow(clippy::cast_precision_loss)]
        let step = (du / b.nuc as f64, (b.vb - b.va) / b.nvc as f64);
        for j in 0..=b.nvc {
            let vv = at(b.va, b.vb, j, b.nvc);
            if !(vv > v.0 && vv < v.1) {
                continue;
            }
            for i in 1..b.nuc {
                #[allow(clippy::cast_precision_loss)]
                let uu = u.0 + (u.1 - u.0) * (i as f64 / b.nuc as f64);
                cand.push(Node {
                    u: uu,
                    v: vv,
                    step,
                    on_crease: false,
                });
            }
        }
    }
    sort_nodes(&mut cand);
    Ok((cand, stats))
}

/// What one face's band schedule did, summarized for the budget meter
/// (`crate::budget`): the cell count the lane sized, and the
/// constraint-activity indicator TESS-SPLIT's spec demands — which
/// bands the aspect cap clamped, and the
/// worst realized lattice aspect the emitted schedule carries. Nothing
/// downstream can recover any of these (bands are not in the CSV, and
/// the selection rule lives only in `nurbs_cert`).
#[derive(Clone, Copy, Debug)]
struct ScheduleStats {
    /// `Σ nuc·nvc` over the clipped bands.
    grid_cells: usize,
    /// Bands in the schedule.
    bands: usize,
    /// Bands whose step selection the A cap clamped.
    cap_bands: usize,
    /// Max over bands of the post-`ceil` spacing ratio `s_u/s_v`.
    realized_aspect: f64,
    /// The schedule's lattice metric
    /// ([`crate::nurbs_cert::lattice_metric`]).
    metric: f64,
}

/// **A lattice node, with the steps of the lattice that lays it**
/// `(h_u, h_v)`, which [`lay`] measures the node's clearance in.
///
/// **"Its own step"**, said here once:
/// - on an analytic chart, the lattice's realized column step and the
///   row's own gap ([`lattice_candidates`]);
/// - on a spline chart, its band's steps. A location on a cut line that
///   two bands lay is two nodes, one at each band's steps, and it is laid
///   only where both clear ([`lay`]): the box that counts is the larger
///   on each axis;
/// - a crease's own node on a grid line is laid by each band that lays
///   that line, at that band's steps, under the same rule
///   ([`Creases::cut_candidates`]).
#[derive(Clone, Copy, Debug, PartialEq)]
struct Node {
    u: f64,
    v: f64,
    step: (f64, f64),
    /// Laid on a crease line, as that crease's own point on a grid line
    /// ([`Creases::cut_candidates`]): held clear of the polygon, and not
    /// of the crease lines.
    on_crease: bool,
}

/// Row-major node order: by `v`, then `u`, then by step where two bands
/// lay one location. A total order (`total_cmp`), so the sort is
/// deterministic.
fn sort_nodes(nodes: &mut [Node]) {
    nodes.sort_unstable_by(|a, b| {
        a.v.total_cmp(&b.v)
            .then(a.u.total_cmp(&b.u))
            .then(a.step.0.total_cmp(&b.step.0))
            .then(a.step.1.total_cmp(&b.step.1))
            .then(a.on_crease.cmp(&b.on_crease))
    });
}

/// The orientation the outer loop's polygon HAS in the chart, which the
/// emit pass gives its triangles. On an analytic arm it is also CHECKED
/// against the one the face promises ([`PolygonOrientation::of_face`]):
/// a polygon that contradicts the sense bounds the face's complement,
/// and emitting by it would ship that complement as a closed manifold
/// mesh. The spline arm (`spline`) takes the polygon's unchecked:
/// its triangles follow the loop's stored traversal whatever the bit
/// says. Every ring is checked against the outer loop on every arm: a
/// ring bounds the face from inside and winds the other way
/// (`geom_brep::PolygonOrientation`'s rule). That is a validity check on
/// the body's loops and nothing the triangulation needs: the even-odd
/// reading is blind to a side's direction, so a ring wound either way
/// meshes the same, and the check refuses a face whose loops contradict
/// each other rather than mesh it as though they did not.
///
/// `None` where a loop's signed area reads neither way (exactly zero,
/// or poisoned): it has no orientation. Whether such loops bound a
/// region at all is not this area's to say; the constraint pass decides
/// it on the boundary's parities (`region::require_a_region`), and a face
/// whose loops do bound one refuses for the orientation it lacks
/// (`TrimmedFace::attempt`).
///
/// # Errors
///
/// [`TessellateError::TrimLoopOrientation`] on an analytic arm whose
/// outer loop is oriented against its face, and on any arm for a ring
/// oriented with its outer loop.
fn trim_orientation(
    fk: FaceKey,
    sense: bool,
    spline: bool,
    trim: &TrimPolygon,
) -> Result<Option<PolygonOrientation>, TessellateError> {
    let of = |points: &[(f64, f64, u32)]| {
        let poly2: Vec<[f64; 2]> = points.iter().map(|&(u, v, _)| [u, v]).collect();
        PolygonOrientation::of_signed_area(shoelace2(&poly2))
    };
    let Some(orientation) = of(trim.outer()) else {
        return Ok(None);
    };
    if !spline && orientation != PolygonOrientation::of_face(sense) {
        return Err(TessellateError::TrimLoopOrientation { face: fk });
    }
    for ring in trim.rings() {
        match of(ring) {
            None => return Ok(None),
            Some(r) if r == orientation => {
                return Err(TessellateError::TrimLoopOrientation { face: fk });
            }
            Some(_) => {}
        }
    }
    Ok(Some(orientation))
}

/// **The one way a face's domain is walked**: the outer loop's lift, and
/// each ring's as the door placed it ([`topo::Body::face_lift`]), in the
/// face's order.
pub(crate) fn domain_loops(
    lift: &FaceLift<f64>,
) -> (&Lift<f64>, impl Iterator<Item = &Lift<f64>> + '_) {
    (
        lift.outer(),
        lift.rings().iter().map(topo::pcurves::PlacedRing::lift),
    )
}

/// Every loop of a face's domain, outer loop first ([`domain_loops`]).
pub(crate) fn every_loop(lift: &FaceLift<f64>) -> impl Iterator<Item = &Lift<f64>> + '_ {
    let (outer, rings) = domain_loops(lift);
    core::iter::once(outer).chain(rings)
}

/// The pcurve-driven UV polygon of the face's domain, every loop of it,
/// each as the door places it (`lift`, [`topo::Body::face_lift`]): per
/// half-edge, `pcurve.eval` at the shared chord parameters (module
/// docs; each traversal contributes all but its last point, and its last
/// point too where that is a `singular` vertex the next traversal leaves
/// from another chart location, so the singular point's row (a pole's
/// latitude, a collapsed side) is a side of the polygon
/// — [`singular_row_shape`]; a spline chart's collapsed side is then
/// walked through its vertex, [`SidePoints::along`]).
/// `nurbs_chart` widens the accepted image forms to `IsoLine`,
/// `IsoArc` and `General`; `Fitted` refuses typed on every chart
/// (module docs). It is the SPLINE-chart flag, not a `Surface::Nurbs`
/// test: the caller sets it from [`ChartRow::Spline`], which an
/// approximating surface takes too (it meshes on its fit, and the fit
/// is a NURBS chart), so every "NURBS chart" below means "NURBS or
/// approximating chart".
///
/// Beside each point, the edge its traversal walks: a refusal of the
/// polygon's shape names the edge that shows it.
fn trim_polygon(
    body: &Body<f64>,
    fk: FaceKey,
    lift: &FaceLift<f64>,
    chords: &ChordPass,
    nurbs_chart: bool,
    creases: Option<&Creases>,
    (singular, walk): (&HashSet<u32>, &SidePoints),
) -> Result<TrimPolygon, TessellateError> {
    let read = |lift: &Lift<f64>| {
        walk.along(loop_polygon(
            body,
            fk,
            lift,
            chords,
            nurbs_chart,
            creases,
            singular,
        )?)
    };
    let (outer, rings) = domain_loops(lift);
    let mut trim = TrimPolygon::new(read(outer)?);
    for ring in rings {
        trim.push_ring(read(ring)?);
    }
    Ok(trim)
}

/// One loop's points and their edges ([`trim_polygon`]).
fn loop_polygon(
    body: &Body<f64>,
    fk: FaceKey,
    lift: &Lift<f64>,
    chords: &ChordPass,
    nurbs_chart: bool,
    creases: Option<&Creases>,
    singular: &HashSet<u32>,
) -> Result<LoopPoints, TessellateError> {
    if lift.is_empty() {
        return Err(TessellateError::EmptyLoop { face: fk });
    }
    let mut travs: Vec<Vec<(f64, f64, u32)>> = Vec::with_capacity(lift.rows().len());
    let mut trav_edges: Vec<EdgeKey> = Vec::with_capacity(lift.rows().len());
    for row in lift.rows() {
        let (hek, image) = (row.half_edge, &row.pcurve);
        let he = body
            .get_half_edge(hek)
            .ok_or(TessellateError::MissingEntity { what: "half-edge" })?;
        let edge = body
            .get_edge(he.edge)
            .ok_or(TessellateError::MissingEntity { what: "edge" })?;
        let forward = edge.he_plus == hek;
        // Trim-loop tessellation walks the forms whose UV steps the
        // chord pass can size (module docs): `Harmonic` on every
        // chart; `IsoLine`, `IsoArc` and `General` on a spline chart
        // (NURBS or approximating — see the flag's doc). Each is one
        // mint's form: an `IsoLine` on a cylinder chart is not minted
        // at rest, the harmonic form with zero trigonometric channels
        // owns that image. A fitted (rung-3) image refuses typed on
        // every chart rather than silently approximating a spline
        // boundary the chord pass could not have sized
        // (`crate::chords`' boundary-tightening contract).
        match image {
            Pcurve::Harmonic { .. } => {}
            // The spiric's two images are closed forms of the
            // carrier's own parameter on every chart it lives on,
            // so the walk below — which samples the image at the
            // chord pass's parameters — reads them exactly as it reads
            // a harmonic one.
            Pcurve::Spiric { .. } => {}
            // A focal section's image is likewise a closed form of the
            // carrier's own parameter, read exactly at the chord pass's
            // parameters.
            Pcurve::FocalSection(_) => {}
            // A projected image is the chart's inverse of the carrier,
            // read exactly at the chord pass's parameters too.
            Pcurve::Projected(_) => {}
            Pcurve::IsoLine { .. } if nurbs_chart => {}
            // The arc rim is the NURBS chart's other minted closed
            // form (M8-3) — same boundary line, rational-quadratic
            // parameter. It has no meaning on an analytic chart.
            Pcurve::IsoArc { .. } if nurbs_chart => {}
            Pcurve::IsoArc { .. } => {
                return Err(TessellateError::UnsupportedCurve {
                    edge: he.edge,
                    note: "trimmed face half-edge carries an ARC-RIM pcurve on an \
                           analytic chart — the class is a NURBS chart's rational \
                           quadratic parameterization and no mint produces it \
                           elsewhere",
                });
            }
            Pcurve::IsoLine { .. } => {
                return Err(TessellateError::UnsupportedCurve {
                    edge: he.edge,
                    note: "trimmed face half-edge carries an ISO-LINE pcurve on an \
                           analytic chart — analytic charts mint the harmonic form \
                           for exactly this image, so this cache is not one the \
                           at-rest mint pass produces",
                });
            }
            Pcurve::Fitted(_) => {
                return Err(TessellateError::UnsupportedCurve {
                    edge: he.edge,
                    note: "trimmed face half-edge carries a FITTED pcurve — \
                           the trim walk and the chord pass's boundary tightening \
                           read closed-form chart images; the fitted image's first \
                           tessellation consumer is the edge×NURBS-face boolean \
                           layer (the cut-loft unit)",
                });
            }
            // The general curve-in-UV arm (U2) on a spline chart: the
            // image is a spline rather than a closed form, and the
            // walk needs neither — it reads `eval` at the SHARED chord
            // parameters, which `Pcurve::eval` answers for this
            // variant, and the chord pass sizes those parameters from
            // the image's own certified UV speed
            // (`geom_brep::Pcurve::speed_bound`).
            //
            // **Watertightness does not move.** The 3-D positions are
            // the carrier's chord points, minted once per edge and
            // consumed by both adjacent faces by ID; what this arm
            // adds is only THIS face's UV shape for those same ids, so
            // the neighbour's polyline still names the same vertices
            // in the same order. The corner's certificate is the
            // crate's documented `δ + ε`: the triangle corner sits at
            // `C(t_i)`, within the image's certified `envelope ≤ ε` of
            // `S(P(t_i))`.
            Pcurve::General(_) if nurbs_chart => {}
            // Off a spline chart the class has no meaning: no mint
            // produces a spline chart image for an analytic carrier.
            // `EdgeDescriptionSpec::chart_image` is public and will
            // ACCEPT one, so this arm is what meets a hand-declared
            // cache rather than a shape the at-rest mint pass yields.
            Pcurve::General(_) => {
                return Err(TessellateError::UnsupportedCurve {
                    edge: he.edge,
                    note: "trimmed face half-edge carries a GENERAL curve-in-UV pcurve \
                           on an analytic chart — the class is a spline chart's image \
                           and no mint produces it elsewhere",
                });
            }
        }
        let ids = chords
            .ids
            .get(&he.edge)
            .ok_or(TessellateError::MissingEntity {
                what: "edge chords",
            })?;
        let ts = chords
            .params
            .get(&he.edge)
            .ok_or(TessellateError::MissingEntity {
                what: "edge chord params",
            })?;
        // Traversal order: forward keeps he_plus order, reversed walks
        // backwards; drop the traversal's LAST entry (the next
        // traversal owns the junction).
        let n = ids.len();
        let points: Vec<(f64, f64)> = (0..n)
            .map(|k| {
                let idx = if forward { k } else { n - 1 - k };
                let uv = image.eval(ts[idx]);
                creases.map_or((uv.x, uv.y), |c| c.snap((uv.x, uv.y)))
            })
            .collect();
        // A boundary chord crossing a crease between its points leaves
        // a triangle across it. The arc rim's schedule lands on its
        // joints (`crate::chords`' `IsoArc` arm), which the snap above
        // puts exactly on the crease; no other image places a point
        // there, and refuses here, typed, if it crosses one.
        if let Some(c) = creases
            && points.windows(2).any(|w| c.straddled(w[0], w[1]))
        {
            return Err(TessellateError::UnsupportedCurve {
                edge: he.edge,
                note: "trimmed NURBS face boundary crosses a C⁰ crease of the face \
                       between two chord points — only an arc rim's chord schedule \
                       lands on the crease it crosses",
            });
        }
        travs.push(
            points
                .iter()
                .enumerate()
                .map(|(k, &(u, v))| (u, v, ids[if forward { k } else { n - 1 - k }]))
                .collect(),
        );
        trav_edges.push(he.edge);
    }
    flatten_traversals(&travs, &trav_edges, singular)
}

/// The traversals joined into one loop's polygon: each contributes all but
/// its last point, and its last point too where that is a `singular`
/// vertex the next traversal leaves from another chart location, so the
/// singular point's row (a pole's latitude, a collapsed side) is a side of
/// the polygon ([`singular_row_shape`], [`SidePoints`]).
/// Beside it, the edge each point's traversal walks.
fn flatten_traversals(
    travs: &[Vec<(f64, f64, u32)>],
    edges: &[EdgeKey],
    singular: &HashSet<u32>,
) -> Result<LoopPoints, TessellateError> {
    if travs.iter().any(Vec::is_empty) {
        return Err(TessellateError::MissingEntity {
            what: "edge chords",
        });
    }
    Ok(flat_points(travs, singular)
        .map(|(k, p)| (p, edges[k]))
        .unzip())
}

/// [`flatten_traversals`]' points, each with its traversal's index.
/// Every traversal is non-empty (the caller's check).
fn flat_points<'t>(
    travs: &'t [Vec<(f64, f64, u32)>],
    singular: &'t HashSet<u32>,
) -> impl Iterator<Item = (usize, (f64, f64, u32))> + 't {
    let m = travs.len();
    (0..m).flat_map(move |k| {
        let trav = &travs[k];
        let last = trav[trav.len() - 1];
        let next = travs[(k + 1) % m][0];
        #[allow(clippy::float_cmp)]
        let side =
            last.2 == next.2 && singular.contains(&last.2) && (last.0, last.1) != (next.0, next.1);
        trav[..trav.len() - 1]
            .iter()
            .map(move |&p| (k, p))
            .chain(side.then_some((k, last)))
    })
}

/// **A spline chart's collapsed sides**: each side of the knot rectangle
/// whose boundary iso is one stored point, read from the one home
/// (`geom_brep::chart_singularity::singular_points`). Empty on an analytic
/// chart, whose singular points its row reads.
struct CollapsedSides(Vec<CollapsedSide>);

/// One collapsed side: which side, its one point, and the chart value
/// its fixed axis holds there (the knot rectangle's end).
#[derive(Clone, Copy, Debug)]
struct CollapsedSide {
    side: geom_brep::ChartSide,
    point: Point3<f64>,
    held: f64,
}

impl CollapsedSides {
    fn of(surface: &Surface<f64>) -> Self {
        use geom_brep::chart_singularity::{Singularity, singular_points};
        use geom_brep::{ChartAxis, ChartEnd};
        let Some(net) = surface.spline_chart() else {
            return Self(Vec::new());
        };
        Self(
            singular_points(surface)
                .into_iter()
                .filter_map(|(which, point)| match which {
                    Singularity::Side(side) => {
                        let (lo, hi) = match side.fixed {
                            ChartAxis::U => net.knot_values_u().domain(),
                            ChartAxis::V => net.knot_values_v().domain(),
                        };
                        let held = match side.end {
                            ChartEnd::Low => lo,
                            ChartEnd::High => hi,
                        };
                        Some(CollapsedSide { side, point, held })
                    }
                    _ => None,
                })
                .collect(),
        )
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Whether vertex position `p` is a collapsed side's point, decided at
    /// `band` by the one read of a vertex on a singular point
    /// (`crate::chart_table`'s `at_a_singular_point`, [`face_polygon`]).
    fn holds(&self, p: Point3<f64>, band: Band) -> Result<bool, Indeterminate> {
        at_a_singular_point(self.0.iter().map(|s| s.point), p, band)
    }

    /// The points along each side ([`SidePoints`]) at the face's chord
    /// steps `(h_u, h_v)`: every `singular` vertex decided on exactly one
    /// side, and clear of every other, walks it at the step of the axis the
    /// side runs along.
    fn points(
        &self,
        shared: &[Point3<f64>],
        singular: &HashSet<u32>,
        (hu, hv): (f64, f64),
        band: Band,
    ) -> SidePoints {
        use geom_brep::ChartAxis;
        let mut along = HashMap::new();
        for &id in singular {
            let p = *shared.get(id as usize).unwrap_or_else(|| {
                unreachable!(
                    "singular id {id} is a loop vertex's end chord id, which the chord \
                     pass mints into the shared positions"
                )
            });
            let reads: Vec<_> = self
                .0
                .iter()
                .map(|s| (s, at_a_singular_point(core::iter::once(s.point), p, band)))
                .collect();
            // A vertex the band leaves undecided against one side, or decides
            // on two, may stand at the sides' shared corner, where the loop's
            // path between its copies turns: it keeps its two copies and no
            // point between, a shape that is sound and coarse
            // (`work/tesslane/a-spline-vertex-on-two-collapsed-sides-keeps-its-polygons-copies-unwalked.md`).
            if reads.iter().any(|(_, r)| r.is_err()) {
                continue;
            }
            let mut on = reads
                .iter()
                .filter(|(_, r)| *r == Ok(true))
                .map(|(s, _)| *s);
            let (Some(s), None) = (on.next(), on.next()) else {
                continue;
            };
            let step = match s.side.fixed {
                ChartAxis::U => hv,
                ChartAxis::V => hu,
            };
            along.insert(
                id,
                Along {
                    fixed: s.side.fixed,
                    held: s.held,
                    step,
                },
            );
        }
        SidePoints { along }
    }
}

/// How one vertex's collapsed side is walked: the axis the side holds
/// fixed, the chart value it holds it at, and the face's chord step along
/// the other axis.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Along {
    fixed: geom_brep::ChartAxis,
    held: f64,
    step: f64,
}

/// **The points a spline chart's polygon walks a collapsed side through**,
/// by vertex id ([`Along`]).
///
/// A singular vertex the loop arrives at from one place on its side and
/// leaves from another is held twice ([`flat_points`]), and the polygon's
/// side between the copies is the collapsed side. The spline certificate
/// bounds a triangle by its chart box (`NurbsCellGrid::cert`), so a
/// triangle with a corner at one copy spans, in the chart, as far along
/// the side as that copy is from its other corners, however small it is
/// in space. The side is therefore walked as a rim is chorded
/// (`crates/mesh/README.md` `[mesh.sizing]`, "Rim chords use the face's
/// grid step"): its image moves at unit speed along its free axis, so it
/// takes `⌈span/h⌉` pieces, each point a copy of the vertex. That is as
/// fine as a strut on that column would be chorded, which is finer than
/// the certificate alone needs. Each point holds the side's fixed axis at
/// the side's own value, so it lies on the side, where the surface is the
/// vertex, and the corner a triangle holds there is the surface's point
/// at its chart corner: every kept triangle's certificate bounds its
/// points' distance from the surface.
///
/// **The triangles the walk leaves out.** A triangle on two copies has two
/// corners on one mesh vertex, and the emit pass drops it before any
/// certificate, as it drops a sphere's or a spindle's at a pole side: its
/// 3-D image is the segment from the vertex to its third corner, an edge
/// of kept, certified triangles, so it adds no point to the mesh and the
/// promise (every point of the mesh within δ of the surface) is not
/// reached by it. What the surface does over the dropped chart region,
/// against that segment, is measured and not proved: within 0.061 δ over
/// the 2,184 such triangles of the loft's class at four δ (the review of
/// PR 7517). The triangles left about the vertex are a fan, which the emit
/// pass's census of identified vertices re-derives.
///
/// Empty on every analytic chart: the sphere's and spindle's certificates
/// read the triangle in space and the cone's reads no azimuth at its
/// apex, so their pole sides hold no point between the copies.
#[derive(Default)]
struct SidePoints {
    along: HashMap<u32, Along>,
}

impl SidePoints {
    /// One loop's points with each walked side's points inserted between
    /// the copies that end it, each beside the edge of the copy it
    /// follows.
    ///
    /// # Errors
    ///
    /// [`ceil_count`]'s, on a side's count.
    fn along(&self, (points, edges): LoopPoints) -> Result<LoopPoints, TessellateError> {
        use geom_brep::ChartAxis;
        if self.along.is_empty() {
            return Ok((points, edges));
        }
        let n = points.len();
        let (mut out, mut out_edges) = (Vec::with_capacity(n), Vec::with_capacity(n));
        for i in 0..n {
            let (a, b) = (points[i], points[(i + 1) % n]);
            out.push(a);
            out_edges.push(edges[i]);
            let Some(&Along { fixed, held, step }) = self.along.get(&a.2) else {
                continue;
            };
            let (fa, fb) = match fixed {
                ChartAxis::U => (a.1, b.1),
                ChartAxis::V => (a.0, b.0),
            };
            #[allow(clippy::float_cmp)] // two copies at one place on the side bound no piece
            if b.2 != a.2 || fa == fb {
                continue;
            }
            let pieces = ceil_count((fb - fa).abs(), step)?;
            for k in 1..pieces {
                #[allow(clippy::cast_precision_loss)]
                let free = fa + (fb - fa) * (k as f64 / pieces as f64);
                out.push(match fixed {
                    ChartAxis::U => (held, free, a.2),
                    ChartAxis::V => (free, held, a.2),
                });
                out_edges.push(edges[i]);
            }
        }
        Ok((out, out_edges))
    }
}

/// One loop's points, each its chart `(u, v)` and mesh id in walk order,
/// and beside them the edge each point's traversal walks.
type LoopPoints = (Vec<(f64, f64, u32)>, Vec<EdgeKey>);

/// **A face's trim polygon**: every loop of its domain, outer loop first
/// and then the rings in the face's order, each in walk order. The CDT
/// takes every loop's sides as constraints, and the even-odd reading
/// (`region::classify_faces`) is what makes a ring's inside a hole.
///
/// Built only by [`TrimPolygon::new`] and [`TrimPolygon::push_ring`], so
/// a point and its edge enter together and the loops are one list of
/// where each ends.
struct TrimPolygon {
    /// Each point's chart `(u, v)` and mesh id.
    points: Vec<(f64, f64, u32)>,
    /// The edge each point's traversal walks.
    edges: Vec<EdgeKey>,
    /// Where each loop ends in `points`, outer loop first: loop `k` is
    /// `ends[k − 1]..ends[k]`, the first from `0`.
    ends: Vec<usize>,
}

impl TrimPolygon {
    /// The polygon of the outer loop alone.
    fn new((points, edges): LoopPoints) -> Self {
        let ends = vec![points.len()];
        Self {
            points,
            edges,
            ends,
        }
    }

    /// Appends a ring.
    fn push_ring(&mut self, (points, edges): LoopPoints) {
        self.points.extend(points);
        self.edges.extend(edges);
        self.ends.push(self.points.len());
    }

    /// The polygon of `outer` and then `rings`, every point's edge the
    /// default key: for the unit rows, whose shapes name no edge.
    #[cfg(test)]
    fn of_loops(outer: &[(f64, f64, u32)], rings: &[&[(f64, f64, u32)]]) -> Self {
        let edges = |l: &[(f64, f64, u32)]| vec![EdgeKey::default(); l.len()];
        let mut out = Self::new((outer.to_vec(), edges(outer)));
        for l in rings {
            out.push_ring((l.to_vec(), edges(l)));
        }
        out
    }

    /// Point `i`'s successor along its own loop: the next point, or its
    /// loop's first after its last.
    fn next(&self, i: usize) -> usize {
        let k = self.ends.partition_point(|&e| e <= i);
        let start = if k == 0 { 0 } else { self.ends[k - 1] };
        if i + 1 == self.ends[k] { start } else { i + 1 }
    }

    /// **The one test of a shared side**: the faces across this
    /// polygon's loops ([`faces_across`]) that hold both ends of a side
    /// joining boundary ids `a` and `b`, which sit at polygon places `at_a`
    /// and `at_b` (a pinch or a singular point's copies sit at several).
    /// None where the two are neighbours on their loop (the side is a
    /// boundary chord) or no one face holds both. A face listed may make
    /// the side in its own triangulation, so a triangle of this face that
    /// has it can be an edge of four: the CDT arms cure it by inserting a
    /// point (`TrimmedFace::emit`), the walk by cutting the side
    /// ([`walk_face`]), which first asks whether the face can reach it
    /// ([`may_make`]).
    fn faces_holding_side(
        &self,
        across: &HashMap<u32, Vec<FaceKey>>,
        (at_a, a): (&[usize], u32),
        (at_b, b): (&[usize], u32),
    ) -> Vec<FaceKey> {
        let neighbours = at_a
            .iter()
            .any(|&i| at_b.iter().any(|&j| self.next(i) == j || self.next(j) == i));
        if neighbours {
            return Vec::new();
        }
        match (across.get(&a), across.get(&b)) {
            (Some(fa), Some(fb)) => fa.iter().filter(|f| fb.contains(f)).copied().collect(),
            _ => Vec::new(),
        }
    }

    /// Point `i`'s predecessor along its own loop.
    fn prev(&self, i: usize) -> usize {
        let k = self.ends.partition_point(|&e| e <= i);
        let start = if k == 0 { 0 } else { self.ends[k - 1] };
        if i == start { self.ends[k] - 1 } else { i - 1 }
    }

    /// The outer loop's points.
    fn outer(&self) -> &[(f64, f64, u32)] {
        &self.points[..self.ends[0]]
    }

    /// Each loop's points, outer loop first.
    fn loops(&self) -> impl Iterator<Item = &[(f64, f64, u32)]> + '_ {
        let starts = core::iter::once(0).chain(self.ends.iter().copied());
        starts.zip(&self.ends).map(|(a, &b)| &self.points[a..b])
    }

    /// Each ring's points.
    fn rings(&self) -> impl Iterator<Item = &[(f64, f64, u32)]> + '_ {
        self.loops().skip(1)
    }
}

/// Chains the vertices `on` each crease into constraints, in order
/// along it ([`Creases::note`] records them), and answers the vertices
/// a link was realised THROUGH: each sits exactly on a link without
/// being one of its ends. `None` when spade refuses a link, which
/// crosses an existing constraint.
fn chain_creases(
    cdt: &mut ConstrainedDelaunayTriangulation<SpadePoint<f64>>,
    on: &mut [(f64, f64, FixedVertexHandle)],
) -> Option<Vec<FixedVertexHandle>> {
    on.sort_unstable_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
    let mut through = Vec::new();
    for w in on.windows(2) {
        let ((ca, _, a), (cb, _, b)) = (w[0], w[1]);
        // Two entries at one handle (a vertex the walk meets twice) are
        // one vertex, and a vertex is no link.
        if ca != cb || a == b {
            continue;
        }
        let realised = cdt.try_add_constraint(a, b);
        if realised.is_empty() {
            return None;
        }
        for e in realised {
            let e = cdt.directed_edge(e);
            for vh in [e.from(), e.to()] {
                let h = vh.fix();
                if h != a && h != b {
                    through.push(h);
                }
            }
        }
    }
    Some(through)
}

/// **A NURBS face's C⁰ creases, as lines the triangulation is cut
/// along** (`work/tesslane/lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`).
///
/// At an interior knot of multiplicity at least the degree the
/// surface's first derivative jumps, and no second-derivative bound
/// certifies a triangle across it ([`NurbsCellGrid::cert`] answers
/// `+∞`). A lofted circle's wall has one in `u` per sub-arc joint.
/// This lane keeps every triangle on one side:
///
/// - a boundary point within rounding of a crease is snapped onto it
///   ([`Self::snap`]); an arc rim has one at each joint, by its chord
///   schedule, and a boundary chord crossing a crease anywhere else
///   refuses typed ([`Self::straddled`]);
/// - no lattice node is laid within its clearance of a crease
///   ([`lay`]), and each grid line across a crease gains a point where
///   it meets it ([`Self::cut_candidates`]);
/// - the points exactly on a crease are chained into constraints in
///   order along it.
///
/// Creases in both directions refuse typed: their lines would cross
/// at a point no chord or grid schedule is built to place.
struct Creases {
    /// The chart coordinate the creases are values of: `0` for `u`,
    /// `1` for `v`.
    axis: usize,
    /// The crease values, ascending.
    at: Vec<f64>,
    /// Within this of a crease is within rounding of it:
    /// [`CREASE_MERGE`] of the chart's extent in `axis`.
    merge: f64,
}

/// The merge band, as a fraction of the chart's extent across the
/// creases: `2⁻⁴⁰`, about 9e-13, or 4096 ulps of an extent. An arc
/// rim's joint point evaluates its chart image to a few ulps of the
/// crease, and is the one point this band moves ([`Creases::snap`]).
const CREASE_MERGE: f64 = 1.0 / 1_099_511_627_776.0;

impl Creases {
    /// The creases of `grid`, the cell table of `chart`; `None` for a
    /// face with none.
    ///
    /// # Errors
    ///
    /// [`TessellateError::UnsupportedNurbsFace`] — creases in both
    /// directions.
    fn of(
        grid: &NurbsCellGrid,
        chart: &geom::NurbsSurface<f64>,
        fk: FaceKey,
    ) -> Result<Option<Self>, TessellateError> {
        Self::from_parts(
            grid.creases(),
            (
                chart.knot_values_u().domain(),
                chart.knot_values_v().domain(),
            ),
            fk,
        )
    }

    /// [`Self::of`] from the crease values and the chart's domain in
    /// each direction.
    ///
    /// # Errors
    ///
    /// As [`Self::of`].
    fn from_parts(
        (cu, cv): (&[f64], &[f64]),
        (du, dv): ((f64, f64), (f64, f64)),
        fk: FaceKey,
    ) -> Result<Option<Self>, TessellateError> {
        let (axis, at, (lo, hi)) = match (cu.is_empty(), cv.is_empty()) {
            (true, true) => return Ok(None),
            (false, true) => (0, cu, du),
            (true, false) => (1, cv, dv),
            (false, false) => {
                return Err(TessellateError::UnsupportedNurbsFace {
                    face: fk,
                    note: "NURBS face with sharp creases in both directions; the trimmed \
                           lane cuts its triangles along creases in one direction only. \
                           Recourse: split the face at the creases of one direction",
                });
            }
        };
        Ok(Some(Self {
            axis,
            at: at.to_vec(),
            merge: (hi - lo).abs() * CREASE_MERGE,
        }))
    }

    /// The crease within rounding of `x`, if any.
    fn near(&self, x: f64) -> Option<f64> {
        self.at
            .iter()
            .copied()
            .find(|c| (x - c).abs() <= self.merge)
    }

    /// `p` with its crease coordinate moved onto a crease within
    /// rounding of it.
    fn snap(&self, p: (f64, f64)) -> (f64, f64) {
        let x = if self.axis == 0 { p.0 } else { p.1 };
        match (self.near(x), self.axis) {
            (Some(c), 0) => (c, p.1),
            (Some(c), _) => (p.0, c),
            (None, _) => p,
        }
    }

    /// Whether the segment `a`–`b` crosses a crease strictly between
    /// its ends.
    fn straddled(&self, a: (f64, f64), b: (f64, f64)) -> bool {
        let (xa, xb) = if self.axis == 0 {
            (a.0, b.0)
        } else {
            (a.1, b.1)
        };
        let (lo, hi) = (xa.min(xb), xa.max(xb));
        self.at.iter().any(|&c| lo < c && c < hi)
    }

    /// The lattice with its crease points: on each crease, a node at
    /// every grid line across it (a row for a `u` crease, a column for a
    /// `v` one), once for each band that lays that line, at that band's
    /// steps ([`Node`], "its own step"). No node is moved: one near a
    /// crease is not laid ([`lay`]), and the crease's own point on its
    /// line serves.
    fn cut_candidates(&self, cand: &mut Vec<Node>) {
        let line = |n: &Node| if self.axis == 0 { n.v } else { n.u };
        let mut lines: Vec<Node> = cand.clone();
        lines.sort_by(|a, b| {
            line(a)
                .total_cmp(&line(b))
                .then(a.step.0.total_cmp(&b.step.0))
                .then(a.step.1.total_cmp(&b.step.1))
        });
        lines.dedup_by(|b, a| line(a) == line(b) && a.step == b.step);
        for &c in &self.at {
            for n in &lines {
                cand.push(if self.axis == 0 {
                    Node {
                        u: c,
                        on_crease: true,
                        ..*n
                    }
                } else {
                    Node {
                        v: c,
                        on_crease: true,
                        ..*n
                    }
                });
            }
        }
        sort_nodes(cand);
    }

    /// Records `h` at `p` in `on`, as `(crease, position along it,
    /// handle)`, when `p` is exactly on a crease.
    fn note<H>(&self, on: &mut Vec<(f64, f64, H)>, p: (f64, f64), h: H) {
        let (x, along) = if self.axis == 0 {
            (p.0, p.1)
        } else {
            (p.1, p.0)
        };
        if self.at.contains(&x) {
            on.push((x, along, h));
        }
    }
}

/// **Whether a neighbouring face on `neighbour` may make the side `a–b`**
/// in a mesh at `delta`, read where it provably cannot, as a function of
/// the body and δ alone: no ε is read (DESIGN `[tolerance.chordal]`, an ε
/// flip never moves emitted coordinates with δ held fixed; `[mesh.winding]`).
///
/// A triangle the neighbour emits passes its certificate
/// (`cert::verdict`): its computed bound is within δ, or past δ by no more
/// than the certificate's own rounding allowance
/// (`cert::rounding_allowance`, 8 ulps of the coordinates' magnitude) while
/// that allowance is small beside δ (`ALLOWANCE_SHARE`). The true bound is
/// within that allowance of the computed one, so a passed triangle lies
/// within `δ·(1 + ALLOWANCE_SHARE)` plus one allowance of its surface. This
/// test computes the side's midpoint's distance from the neighbour's
/// surface (`geom::Surface::foot`, in closed form on every analytic kind)
/// and keeps the side uncut only where that distance exceeds
/// `δ·(1 + ALLOWANCE_SHARE)` by more than TWO allowances at the magnitude
/// of the points it reads: one for the neighbour's bound, one for this
/// distance's own rounding (a midpoint, a closed-form foot, a difference
/// and a norm: a chain of the certificate's kind, which its 8 ulps bound
/// with room, `cert::ROUNDING_ULPS`). Past that, no triangle the neighbour
/// passes holds the side. Everything else cuts: a distance within it, a
/// surface with no foot, a point where the foot is not one point.
///
/// **A planar neighbour** has no certificate: its triangles lie in its
/// plane, so it holds a side only where the side's midpoint is in the
/// plane, and there the distance is within rounding of zero, never past
/// the reach; the walk cuts such a side.
///
/// **The magnitude read.** The allowance is taken at the side's two ends
/// and the midpoint's foot. The neighbour's verdict reads its own
/// triangle's third corner and its chart's anchor too
/// (`cert::position_scale`); where those stand farther from the origin,
/// its allowance is larger than the one read here (by at most
/// `δ·ALLOWANCE_SHARE`, past which the verdict does not pass the
/// triangle). A side excluded on that difference, which no body built here
/// reaches, is the census's (`tessellate::overused_side`) to refuse.
///
/// Every face of one run is meshed at its one δ (the run's `SizingTols`),
/// and a face sized finer than δ is still within it.
fn may_make(neighbour: &Surface<f64>, a: Point3<f64>, b: Point3<f64>, delta: f64) -> bool {
    let mid = Point3::new(0.5 * (a.x + b.x), 0.5 * (a.y + b.y), 0.5 * (a.z + b.z));
    let Some(foot) = neighbour.foot(mid) else {
        return true;
    };
    let allowance = cert::rounding_allowance(cert::coordinate_scale([a, b, foot]));
    (mid - foot).norm() <= delta * (1.0 + cert::ALLOWANCE_SHARE) + 2.0 * allowance
}

/// **A walked face's patch** (`crate::ruled_walk`): the walk of its two
/// rows, each side a neighbouring face may also make cut at a point of the
/// surface between its ends, then each triangle read as every triangle of
/// this lane is: its winding resolved and outward, its certificate within
/// δ.
///
/// **The shared side**, the polygon's one test
/// ([`TrimPolygon::faces_holding_side`], which the CDT arms read too): a
/// side joining two polygon points that are not neighbours on their loop
/// and that one face across the loop's edges holds both of is a side that
/// face's own triangulation may make, an edge of four triangles. A
/// neighbour holding both of this face's rims (a sphere or a torus face
/// round a bore, at a δ past the shell's thickness) makes rim-to-rim sides
/// exactly as the walk does. The walk has no triangulation to rebuild
/// round an inserted point, as the CDT arms do: it cuts the side, where
/// the face across may reach it ([`may_make`]), at the surface point at
/// its chart midpoint (the apex's `u` left out), and each triangle on it
/// becomes two. That point lies between the same two rulings, so the two
/// keep the walk's bound, and each cut adds two triangles to the face's
/// count.
///
/// # Errors
///
/// The lane's per-triangle refusals: [`TessellateError::UnresolvedWinding`],
/// [`TessellateError::FoldedTriangle`],
/// [`TessellateError::CertificateExceeded`],
/// [`TessellateError::RoundingUnresolved`].
#[allow(clippy::too_many_arguments)] // the face, its domain, and what the run shares
fn walk_face(
    body: &Body<f64>,
    fk: FaceKey,
    row: &AnalyticRow<'_>,
    polygon: &TrimPolygon,
    singular: &HashSet<u32>,
    chords: &ChordPass,
    shared: &[Point3<f64>],
    tol: &SizingTols,
) -> Result<Patch, TessellateError> {
    let sense = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?
        .sense;
    let points = &polygon.points;
    let line: Vec<bool> = polygon
        .edges
        .iter()
        .map(|&ek| carried_by_a_line(body, ek))
        .collect();
    // `[failure.taxonomy]` row 0: no. The two rows are read again here off
    // the polygon the chord pass built, after the route read the loop's
    // carriers (`walkable`); the route cannot hand the lane the polygon,
    // which does not exist when it runs. A re-derivation that a correct
    // kernel always passes is row 4, and it is a re-derivation of one
    // decision, not a second decision that should agree: the apex flags
    // here (`singular_vertices`) and the route's are one function on the
    // same arguments, `chart_table::at_a_singular_point` over
    // `topo::chart::Chart::of(surface).poles()` (the route through
    // `chart_table::at_a_pole`, the lane through `AnalyticRow::is_singular`,
    // its row minted on the same surface), at the vertex's point by topo's
    // one read of it (`readback::vertex_point`; the lane's mesh position is
    // `Body::vertex_points`' copy of it), at the run's one band
    // (`tessellate` mints it once and hands it to the route and to
    // `SizingTols`). A vertex in the band between them cannot exist: where
    // the route's reading is undecided it does not walk the face.
    let Some(rows) = (polygon.ends.len() == 1)
        .then(|| crate::ruled_walk::Rows::of(points, &line, singular))
        .flatten()
    else {
        unreachable!(
            "face {fk:?}: the route walks a face only where its outer loop, with no \
             ring, is lines and rims in the walk's shape (`walkable`, deciding its \
             vertices on the apex at the run's band), and the trim polygon is that \
             loop in order with chord points added along its edges and the apex \
             decided at the same band (`singular_vertices`), so it reads as the same \
             two rows: here it has {} loop(s) and {} point(s)",
            polygon.ends.len(),
            points.len()
        );
    };
    let across = faces_across(body, fk, chords)?;
    // Each boundary id's places on the polygon: the apex's copies are one
    // point, and so are a full turn's seam ends.
    let mut places: HashMap<u32, Vec<usize>> = HashMap::new();
    for (i, point) in points.iter().enumerate() {
        places.entry(point.2).or_default().push(i);
    }
    // A side one face across holds both ends of (the polygon's one test,
    // `TrimPolygon::faces_holding_side`), which that face may also make
    // ([`may_make`]).
    let shares = |p: u32, q: u32| {
        let at = |id: u32| places.get(&id).map_or(&[][..], Vec::as_slice);
        for f in polygon.faces_holding_side(&across, (at(p), p), (at(q), q)) {
            let neighbour = crate::tessellate::face_surface(body, f)?;
            if may_make(neighbour, shared[p as usize], shared[q as usize], tol.delta) {
                return Ok(true);
            }
        }
        Ok::<bool, TessellateError>(false)
    };
    // Each corner: its mesh vertex, its chart point, and whether it is the
    // apex.
    let mut corners: Vec<[(PatchVertex, [f64; 2], bool); 3]> = rows
        .walk(points)
        .into_iter()
        .map(|t| {
            t.map(|k| {
                let (u, v, id) = points[k];
                (PatchVertex::Shared(id), [u, v], singular.contains(&id))
            })
        })
        .collect();
    // The shared sides, in the order the walk meets them.
    let mut split: Vec<(u32, u32)> = Vec::new();
    for c in &corners {
        for q in 0..3 {
            if let (PatchVertex::Shared(p), PatchVertex::Shared(r)) = (c[q].0, c[(q + 1) % 3].0)
                && !split.contains(&(p.min(r), p.max(r)))
                && shares(p, r)?
            {
                split.push((p.min(r), p.max(r)));
            }
        }
    }
    // Each shared side is cut at the surface point at its chart midpoint
    // (the apex's `u` left out), and each triangle on it made two.
    let mut interior: Vec<Point3<f64>> = Vec::new();
    for (p, r) in split {
        let mut cut = None;
        let mut next = Vec::with_capacity(corners.len() + 2);
        for c in corners {
            let at = (0..3).find(|&q| {
                let (x, y) = (c[q].0, c[(q + 1) % 3].0);
                (x, y) == (PatchVertex::Shared(p), PatchVertex::Shared(r))
                    || (x, y) == (PatchVertex::Shared(r), PatchVertex::Shared(p))
            });
            let Some(q) = at else {
                next.push(c);
                continue;
            };
            let (x, y, z) = (c[q], c[(q + 1) % 3], c[(q + 2) % 3]);
            let mid = *cut.get_or_insert_with(|| {
                let u = match (x.2, y.2) {
                    (true, _) => y.1[0],
                    (_, true) => x.1[0],
                    _ => 0.5 * (x.1[0] + y.1[0]),
                };
                let v = 0.5 * (x.1[1] + y.1[1]);
                interior.push(row.surface().eval(u, v));
                #[allow(clippy::cast_possible_truncation)]
                let id = PatchVertex::Local(interior.len() as u32 - 1);
                (id, [u, v], false)
            });
            next.push([x, mid, z]);
            next.push([mid, y, z]);
        }
        corners = next;
    }
    let mut triangles = Vec::with_capacity(corners.len());
    let mut verdict = cert::FaceVerdict::new();
    for c in corners {
        let ids = c.map(|c| c.0);
        let tri = ids.map(|id| id.position(shared, &interior));
        let normal = crate::validate::resolved_normal(tri).map_err(|unresolved| {
            TessellateError::UnresolvedWinding {
                face: fk,
                unresolved: Box::new(unresolved),
            }
        })?;
        let centroid = Point3::new(
            (tri[0].x + tri[1].x + tri[2].x) / 3.0,
            (tri[0].y + tri[1].y + tri[2].y) / 3.0,
            (tri[0].z + tri[1].z + tri[2].z) / 3.0,
        );
        // Folded unless wound outward: a zero dot is a fold.
        if normal
            .dot(row.outward(sense, centroid).vec())
            .partial_cmp(&0.0)
            != Some(core::cmp::Ordering::Greater)
        {
            return Err(TessellateError::FoldedTriangle { face: fk });
        }
        // Which corners are the apex: an apex corner's chart `u` is a copy's
        // (the polygon places the apex at the `u` of each ruling meeting
        // it), and the certificate reads such a corner as the apex itself,
        // a point at every `u`, not as `S(u, v)` at that copy's `u`
        // (`cert::cert_cone`'s `pole`), as the CDT arms read theirs. A cut
        // point is never the apex.
        let (bound, scale) = row.cert(tri, c.map(|c| c.1), c.map(|c| c.2));
        verdict.read(bound, tol.delta, scale, tol.eps);
        triangles.push(ids);
    }
    verdict.refusal(fk, tol.delta)?;
    Ok(Patch {
        interior,
        triangles,
    })
}

#[cfg(test)]
#[path = "trimmed_collapsed_rows.rs"]
mod collapsed_rows;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
pub(super) mod tests {
    use super::{
        ChordPass, Creases, FixedVertexHandle, Node, SpadePoint, TessellateError, TrimPolygon,
        chain_creases, id_repeats_apart, one_location_per_branch, trim_polygon,
    };
    use crate::chart_table::SingularPoint;
    use geom_brep::PolygonOrientation;
    use spade::{ConstrainedDelaunayTriangulation, Triangulation};
    use std::collections::HashSet;
    use topo::FaceKey;

    /// The witness tolerance's band, the one the unit rows read lifts at.
    fn band() -> geom_core::predicate::Band {
        geom_core::predicate::Band::linear(geom_core::Tol::witness()).unwrap()
    }

    thread_local! {
        /// A factor on the trimmed lane's analytic steps
        /// (`super::arm_steps`), 1 in every shipped build: the end-to-end
        /// refusal row coarsens the sphere arm with it to reach
        /// `CertificateExceeded` through the real call path. `cfg(test)`
        /// only.
        pub(crate) static STEP_SCALE: std::cell::Cell<f64> =
            const { std::cell::Cell::new(1.0) };
        /// The retry loop's last attempt, when a row sets it
        /// (`super::last_attempt`). `cfg(test)` only.
        pub(crate) static LAST_ATTEMPT: std::cell::Cell<Option<usize>> =
            const { std::cell::Cell::new(None) };
    }

    /// **The MAJOR-1 backstop's end-to-end guard**: a face left with a
    /// sliver above δ refuses typed [`TessellateError::CertificateExceeded`]
    /// through `crate::tessellate`, not only at `cert::verdict`. No natural
    /// body refuses once narrow-column bands take rows, so this row turns
    /// the raise off (`nurbs_cert::tests::ROWS_RAISED`, test-only) and meshes the
    /// uneven-station cylinder at the δ where it refused before the raise:
    /// a lane that stopped consulting the verdict would mesh it silently.
    ///
    /// The face map runs on rayon, so the switch is set inside a one-thread
    /// pool, on the worker that runs the face.
    #[test]
    fn a_face_left_slivered_refuses_typed_through_tessellate() {
        use geom_core::Tol;
        let tau = core::f64::consts::TAU;
        let bulge = (tau / 16.0).tan();
        let section = || {
            vec![profile::test_support::bulge_loop(
                (0..4)
                    .map(|k| {
                        let a = tau * f64::from(k) / 4.0;
                        (geom_core::Point2::new(a.cos(), a.sin()), bulge)
                    })
                    .collect(),
            )]
        };
        let body = sweep::loft_body::<f64>(
            &[section(), section(), section(), section(), section()],
            &sweep::test_support::stacked_at(&[0.0, 0.01, 0.02, 0.5, 1.0]),
            2,
            Tol::witness(),
        )
        .expect("the uneven-station cylinder lofts")
        .body;
        let delta = 2.586_047_619_477_458e-3;
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .expect("a one-thread pool");
        let (slivered, raised) = pool.install(|| {
            crate::nurbs_cert::tests::ROWS_RAISED.with(|r| r.set(false));
            let slivered = crate::tessellate(&body, delta, Tol::witness());
            crate::nurbs_cert::tests::ROWS_RAISED.with(|r| r.set(true));
            (slivered, crate::tessellate(&body, delta, Tol::witness()))
        });
        assert!(
            matches!(
                slivered,
                Err(TessellateError::CertificateExceeded { bound, requested, .. })
                    if bound > requested
            ),
            "rows not raised: the slivered face refuses typed, got {:?}",
            slivered.map(|m| m.patches.len())
        );
        assert!(raised.is_ok(), "rows raised: the same face meshes");
    }

    /// Creases at `at` along `axis` on the unit square.
    fn creases(axis: usize, at: &[f64]) -> Creases {
        let (u, v) = if axis == 0 {
            (at, &[][..])
        } else {
            (&[][..], at)
        };
        Creases::from_parts((u, v), ((0.0, 1.0), (0.0, 1.0)), FaceKey::default())
            .unwrap()
            .unwrap()
    }

    /// The unit square's boundary with `k` points per side, plus a
    /// point on each side the creases cross, snapped as the lane snaps.
    fn square(c: &Creases, k: usize) -> Vec<(f64, f64)> {
        #[allow(clippy::cast_precision_loss)]
        let mut side: Vec<f64> = (0..k).map(|i| i as f64 / k as f64).collect();
        side.extend(c.at.iter().copied());
        side.sort_unstable_by(f64::total_cmp);
        side.dedup();
        let mut out = Vec::new();
        side.push(1.0);
        out.extend(side.iter().map(|&t| (t, 0.0)));
        out.extend(side.iter().skip(1).map(|&t| (1.0, t)));
        out.extend(side.iter().rev().skip(1).map(|&t| (t, 1.0)));
        out.extend(side.iter().rev().skip(1).map(|&t| (0.0, t)));
        out.pop();
        out.into_iter().map(|p| c.snap(p)).collect()
    }

    /// The lane's CDT steps on a square: boundary constraints and,
    /// when `cut`, the candidates cut at the creases and the crease
    /// lines chained. Answers the inner triangles' boxes along
    /// `c.axis`.
    fn triangulate(c: &Creases, cand: Vec<(f64, f64)>, cut: bool) -> Vec<(f64, f64)> {
        let boundary = square(c, 7);
        let cand = if cut {
            cut_and_laid(c, &boundary, &cand)
        } else {
            cand
        };
        let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
            ConstrainedDelaunayTriangulation::new();
        let mut on: Vec<(f64, f64, FixedVertexHandle)> = Vec::new();
        let mut handles = Vec::new();
        for &p in boundary.iter().chain(&cand) {
            let h = cdt.insert(SpadePoint::new(p.0, p.1)).unwrap();
            c.note(&mut on, p, h);
            handles.push(h);
        }
        for i in 0..boundary.len() {
            let (a, b) = (handles[i], handles[(i + 1) % boundary.len()]);
            assert_eq!(
                cdt.try_add_constraint(a, b).len(),
                1,
                "a boundary segment splits"
            );
        }
        if cut {
            let through = chain_creases(&mut cdt, &mut on).expect("every link realises");
            assert!(through.is_empty(), "a link ran through {through:?}");
        }
        cdt.inner_faces()
            .map(|f| {
                let xs = f.positions().map(|p| if c.axis == 0 { p.x } else { p.y });
                (xs[0].min(xs[1]).min(xs[2]), xs[0].max(xs[1]).max(xs[2]))
            })
            .collect()
    }

    /// `cand` as the lane lays it inside `boundary`: nodes of the step
    /// `STEP`, cut at the creases (`Creases::cut_candidates`) and kept
    /// clear of the boundary and the crease lines (`super::lay`).
    fn cut_and_laid(c: &Creases, boundary: &[(f64, f64)], cand: &[(f64, f64)]) -> Vec<(f64, f64)> {
        let mut nodes: Vec<Node> = cand
            .iter()
            .map(|&(u, v)| Node {
                u,
                v,
                step: STEP,
                on_crease: false,
            })
            .collect();
        super::sort_nodes(&mut nodes);
        c.cut_candidates(&mut nodes);
        #[allow(clippy::cast_possible_truncation)]
        let outer: Vec<(f64, f64, u32)> = boundary
            .iter()
            .enumerate()
            .map(|(i, &(u, v))| (u, v, i as u32))
            .collect();
        super::lay(
            &nodes,
            super::BOUNDARY_CLEARANCE,
            super::CdtPlane::Lattice { metric: 1.0 },
            &super::TrimPolygon::of_loops(&outer, &[]),
            &HashSet::new(),
            Some(c),
        )
    }

    /// The step the crease rows give their nodes: finer than any grid
    /// they lay, so a node is dropped only where it crowds a crease.
    const STEP: (f64, f64) = (1e-3, 1e-3);

    /// A uniform interior grid `nu × nv`, offset from the creases.
    fn grid(nu: usize, nv: usize) -> Vec<(f64, f64)> {
        #[allow(clippy::cast_precision_loss)]
        (1..nv)
            .flat_map(|j| (1..nu).map(move |i| (i as f64 / nu as f64, j as f64 / nv as f64)))
            .collect()
    }

    /// [`grid`] with `n_across` divisions across the crease direction
    /// `axis` and `n_along` along it.
    fn grid_on(axis: usize, n_across: usize, n_along: usize) -> Vec<(f64, f64)> {
        if axis == 0 {
            grid(n_across, n_along)
        } else {
            grid(n_along, n_across)
        }
    }

    /// The CDT row, in each direction alone: with the grid cut at the
    /// creases and the crease lines chained, no triangle's box strictly
    /// crosses a crease, on a square grid and on one dense across the
    /// creases and sparse along them; the uncut grid does cross.
    #[test]
    fn crease_lines_keep_every_triangle_on_one_side() {
        for axis in [0, 1] {
            let c = creases(axis, &[0.37, 0.61]);
            let crossing = |boxes: &[(f64, f64)]| {
                boxes
                    .iter()
                    .filter(|(lo, hi)| c.at.iter().any(|&x| *lo < x && x < *hi))
                    .count()
            };
            for (n_across, n_along) in [(9, 6), (40, 2)] {
                let cut = triangulate(&c, grid_on(axis, n_across, n_along), true);
                assert_eq!(
                    crossing(&cut),
                    0,
                    "axis {axis}: a triangle crosses a crease"
                );
            }
            // The lane before the crease lines, on the same grid, does
            // cross: the row reads the cut.
            let uncut = triangulate(&c, grid_on(axis, 9, 6), false);
            assert!(
                crossing(&uncut) > 0,
                "axis {axis}: the row cannot see a missing cut"
            );
        }
    }

    /// **The chain row: the crease line holds where Delaunay alone
    /// would cross it.** On grid rows the chain is redundant: every
    /// point beside a crease has a crease point on its own row
    /// ([`Creases::cut_candidates`]), so each link is a Gabriel edge
    /// and Delaunay realises it unconstrained. A trim loop's points
    /// have no rows: here two holes, one each side of the crease and
    /// `5e-3` off it, so the crease points are only the outer square's
    /// two, and the circle on that link holds every hole corner.
    /// Chained, no triangle crosses the crease; unchained, one does, so
    /// the row goes red if [`chain_creases`] stops adding its links.
    #[test]
    fn the_crease_chain_holds_beside_off_row_trim_points() {
        for axis in [0, 1] {
            let c = creases(axis, &[0.5]);
            let swap = |(x, y): (f64, f64)| if axis == 0 { (x, y) } else { (y, x) };
            let hole = |lo: f64, hi: f64| {
                [(lo, 0.4), (hi, 0.4), (hi, 0.6), (lo, 0.6)]
                    .map(swap)
                    .to_vec()
            };
            let loops = [square(&c, 1), hole(0.48, 0.495), hole(0.505, 0.52)];
            let crossing = |chained: bool| {
                let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
                    ConstrainedDelaunayTriangulation::new();
                let mut on: Vec<(f64, f64, FixedVertexHandle)> = Vec::new();
                for l in &loops {
                    let handles: Vec<_> = l
                        .iter()
                        .map(|&p| {
                            let h = cdt.insert(SpadePoint::new(p.0, p.1)).unwrap();
                            c.note(&mut on, p, h);
                            h
                        })
                        .collect();
                    for i in 0..handles.len() {
                        let (a, b) = (handles[i], handles[(i + 1) % handles.len()]);
                        assert_eq!(cdt.try_add_constraint(a, b).len(), 1);
                    }
                }
                assert_eq!(on.len(), 2, "axis {axis}: the square's two crease points");
                if chained {
                    let through = chain_creases(&mut cdt, &mut on).expect("the link realises");
                    assert!(through.is_empty(), "a link ran through {through:?}");
                }
                cdt.inner_faces()
                    .filter(|f| {
                        let xs = f.positions().map(|p| if axis == 0 { p.x } else { p.y });
                        let (lo, hi) = (xs[0].min(xs[1]).min(xs[2]), xs[0].max(xs[1]).max(xs[2]));
                        lo < 0.5 && 0.5 < hi
                    })
                    .count()
            };
            assert_eq!(
                crossing(true),
                0,
                "axis {axis}: a triangle crosses the chained crease"
            );
            assert!(
                crossing(false) > 0,
                "axis {axis}: the row cannot see a missing chain"
            );
        }
    }

    /// A boundary point one ulp off a crease snaps onto it, on either
    /// side, and a node one ulp off it is not laid: neither survives
    /// beside the crease line to cut a sliver, and the triangulation
    /// still keeps every triangle on one side.
    #[test]
    fn a_node_beside_a_crease_is_not_laid_and_a_boundary_point_snaps() {
        let x = 0.37f64;
        let (below, above) = (
            f64::from_bits(x.to_bits() - 1),
            f64::from_bits(x.to_bits() + 1),
        );
        for axis in [0, 1] {
            let c = creases(axis, &[x]);
            let along = |p: (f64, f64)| if axis == 0 { p.0 } else { p.1 };
            let at = |w: f64| if axis == 0 { (w, 0.5) } else { (0.5, w) };
            assert_eq!(along(c.snap(at(below))), x, "axis {axis}: below");
            assert_eq!(along(c.snap(at(above))), x, "axis {axis}: above");
            let mut cand = grid(9, 6);
            cand.extend([at(below), at(above)]);
            let cand = cut_and_laid(&c, &square(&c, 7), &cand);
            assert!(
                cand.iter()
                    .all(|&p| along(p) == x || (along(p) - x).abs() > 1e-6),
                "axis {axis}: a candidate survives beside the crease"
            );
            let boxes = triangulate(&c, cand, true);
            assert!(
                boxes.iter().all(|(lo, hi)| !(*lo < x && x < *hi)),
                "axis {axis}"
            );
        }
    }

    /// Creases in both directions refuse typed, naming the face.
    #[test]
    fn creases_in_both_directions_refuse() {
        let face = FaceKey::default();
        let r = Creases::from_parts((&[0.5], &[0.5]), ((0.0, 1.0), (0.0, 1.0)), face);
        assert!(
            matches!(r, Err(crate::TessellateError::UnsupportedNurbsFace { face: f, .. }) if f == face),
            "{:?}",
            r.err()
        );
    }

    /// The chord row, on the lofted cylinder: each rim's chord schedule
    /// holds its joint angle `t0 + span·½`, the same expression
    /// `segment_curve` places its joint knot with
    /// (`geom_brep::sub_arc_joint`), and the wall's trim polygon puts
    /// that point's UV exactly on the crease `u = ½`.
    #[test]
    fn the_rim_carries_the_joint_with_its_uv_on_the_crease() {
        use std::collections::HashMap;
        let body = sweep::test_support::lofted_cylinder(1.0, geom_core::Tol::witness());
        let mut positions = Vec::new();
        let mut vids = HashMap::new();
        for (vk, p) in body.vertex_points() {
            #[allow(clippy::cast_possible_truncation)]
            vids.insert(vk, positions.len() as u32);
            positions.push(p);
        }
        let mut bounds = crate::nurbs_cert::FaceBounds::new();
        let routes = crate::tessellate::Routes::witness(&body);
        let chords: ChordPass =
            crate::chords::compute_chords(&body, 5e-3, &routes, &vids, &mut positions, &mut bounds)
                .unwrap();
        let mut rims = 0;
        for (ek, edge) in body.edges() {
            let curve = body
                .get_curve_geom(edge.curve)
                .unwrap()
                .certified()
                .unwrap();
            if !matches!(curve.carrier(), geom::Curve3::Circle(..)) {
                continue;
            }
            rims += 1;
            let (t0, t1) = curve.params();
            let joint = t0 + (t1 - t0) * geom_brep::sub_arc_joint(1, 2);
            assert!(
                chords.params[&ek].contains(&joint),
                "rim {ek:?}: no chord point at {joint}"
            );
        }
        assert_eq!(rims, 4, "two semicircles per section");
        let mut on = 0;
        for (fk, _) in body.faces() {
            let surface = body.face_surface(fk).unwrap();
            let Some(chart) = surface.spline_chart() else {
                continue;
            };
            let grid = crate::nurbs_cert::face_cells(&bounds, fk).unwrap();
            assert_eq!(grid.creases(), (&[0.5][..], &[][..]), "face {fk:?}");
            let c = Creases::of(grid, chart, fk).unwrap();
            let lift = body.face_lift(fk, band()).unwrap();
            let polygon = trim_polygon(
                &body,
                fk,
                &lift,
                &chords,
                true,
                c.as_ref(),
                (&HashSet::new(), &super::SidePoints::default()),
            )
            .unwrap()
            .points;
            let (_, v0, v1) = polygon
                .iter()
                .fold((0, f64::MAX, f64::MIN), |(n, lo, hi), p| {
                    (n, lo.min(p.1), hi.max(p.1))
                });
            let hits: Vec<f64> = polygon.iter().filter(|p| p.0 == 0.5).map(|p| p.1).collect();
            assert!(
                hits.contains(&v0) && hits.contains(&v1),
                "face {fk:?}: the crease meets both rims at chord points, got v = {hits:?}"
            );
            on += 1;
        }
        assert_eq!(on, 2, "two creased walls");
    }

    /// The ingredient test #678's fan needs, pinned on its own: a mesh
    /// id at two DIFFERENT UV locations is the thing that can be
    /// fanned apart; the same id twice at the SAME location is one CDT
    /// vertex after spade's dedup and cannot be.
    ///
    /// **Both edges of "different" are here**, because the helper's
    /// only way to go quietly wrong is to disagree with spade about
    /// which pairs are one vertex: an ulp apart at seam magnitude is
    /// two spade vertices and must read APART, and `-0.0` against
    /// `0.0` is one spade vertex and must not — the case a `to_bits`
    /// spelling gets backwards.
    #[test]
    fn a_repeat_at_one_uv_is_not_a_repeat_apart() {
        let same = [(0.0, 0.0, 7u32), (1.0, 0.5, 8), (0.0, 0.0, 7)];
        assert!(!id_repeats_apart(&same), "one location, one CDT vertex");

        let apart = [(0.0, 0.0, 7u32), (1.0, 0.5, 8), (2.0, 0.0, 7)];
        assert!(id_repeats_apart(&apart), "the seam double-traversal shape");

        // An ulp at SEAM magnitude (2π), not at zero: near zero the
        // floor merges everything below 2⁻¹⁴² (`crate::cdt`'s rows).
        let seam = std::f64::consts::TAU;
        let ulp = [
            (0.0, 0.0, 7u32),
            (1.0, 0.5, 8),
            (f64::from_bits(seam.to_bits() + 1), 0.0, 7),
            (seam, 0.0, 7),
        ];
        assert!(
            id_repeats_apart(&ulp),
            "an ulp apart at 2pi is two CDT vertices, so it is apart"
        );

        // The other edge: spade compares positions with `==`, and
        // `-0.0 == 0.0`. One vertex, so NOT apart — a bitwise compare
        // would say the opposite.
        let signed_zero = [(0.0, 0.0, 7u32), (1.0, 0.5, 8), (-0.0, 0.0, 7)];
        assert!(
            !id_repeats_apart(&signed_zero),
            "-0.0 and 0.0 are one spade vertex"
        );

        // No repeat at all, and distinct ids at one location (a
        // self-touch) is not this shape either.
        let none = [(0.0, 0.0, 7u32), (0.0, 0.0, 8), (2.0, 0.0, 9)];
        assert!(!id_repeats_apart(&none), "distinct ids are not a repeat");
    }

    /// A vertex met again on one branch takes its first entry's UV, bit
    /// for bit; on another branch, or at a singular point, it keeps its
    /// own. The rounding is the measured shape: one cylinder vertex's two
    /// pcurve images at `u = -1.7e-18` and `u = 0`, and `0.9` an ulp
    /// apart.
    #[test]
    fn a_vertex_met_again_on_one_branch_is_one_uv_location() {
        let tau = std::f64::consts::TAU;
        let up = |x: f64| f64::from_bits(x.to_bits() + 1);
        let cylinder = Some((Some(geom_core::period::Period::angle()), None));
        let read = |outer: &[(f64, f64, u32)], periods, singular: &[u32]| {
            let singular = singular.iter().copied().collect();
            one_location_per_branch(TrimPolygon::of_loops(outer, &[]), periods, &singular).points
        };

        let rounded = [
            (-1.7e-18, 1.0, 7u32),
            (0.1, 0.5, 8),
            (0.0, up(1.0), 7),
            (0.9, 4.0, 9),
            (0.5, 0.0, 10),
            (up(0.9), 4.0, 9),
        ];
        let pinned = read(&rounded, cylinder, &[]);
        assert_eq!(
            (pinned[2], pinned[5]),
            ((-1.7e-18, 1.0, 7), (0.9, 4.0, 9)),
            "one branch: the first entry's UV"
        );
        assert!(!id_repeats_apart(&pinned), "pinned, no id is apart");
        assert!(id_repeats_apart(&rounded), "as walked, both are apart");

        let seam = [(0.0, 0.0, 7u32), (1.0, 0.5, 8), (tau, up(0.0), 7)];
        assert_eq!(
            read(&seam, cylinder, &[]),
            seam,
            "a whole turn away is the other branch"
        );
        assert_eq!(
            read(&rounded, cylinder, &[7, 9]),
            rounded,
            "a singular vertex keeps its copies"
        );
        assert_eq!(
            read(&rounded, None, &[]),
            rounded,
            "a spline chart is read as walked"
        );
        let torus = Some((
            Some(geom_core::period::Period::angle()),
            Some(geom_core::period::Period::angle()),
        ));
        let v_seam = [(0.0, 0.0, 7u32), (1.0, 0.5, 8), (up(0.0), tau, 7)];
        assert_eq!(
            read(&v_seam, torus, &[]),
            v_seam,
            "a whole turn away in v is the other branch"
        );
    }

    /// Each branch has its own first entry. A vertex met at `u = 0` and
    /// then twice a turn on, the two later entries rounding apart, is
    /// two UV locations, not three, in either walk order.
    #[test]
    fn a_vertex_met_twice_on_its_second_branch_is_one_location_there() {
        let tau = std::f64::consts::TAU;
        let up = |x: f64| f64::from_bits(x.to_bits() + 1);
        let cylinder = Some((Some(geom_core::period::Period::angle()), None));
        let read = |outer: &[(f64, f64, u32)]| {
            one_location_per_branch(TrimPolygon::of_loops(outer, &[]), cylinder, &HashSet::new())
                .points
        };
        let zero_first = [
            (0.0, 1.0, 7u32),
            (1.0, 0.5, 8),
            (tau, 1.0, 7),
            (5.0, 2.0, 9),
            (up(tau), up(1.0), 7),
            (3.0, 3.0, 10),
        ];
        let pinned = read(&zero_first);
        assert_eq!(
            (pinned[0], pinned[4]),
            ((0.0, 1.0, 7), (tau, 1.0, 7)),
            "the first branch keeps its entry; the third entry takes the second's UV"
        );
        let tau_first = [
            (tau, 1.0, 7u32),
            (1.0, 0.5, 8),
            (0.0, 1.0, 7),
            (5.0, 2.0, 9),
            (up(tau), up(1.0), 7),
            (3.0, 3.0, 10),
        ];
        let pinned = read(&tau_first);
        assert_eq!(
            (pinned[2], pinned[4]),
            ((0.0, 1.0, 7), (tau, 1.0, 7)),
            "walked from the second branch: the same two locations"
        );
    }

    /// Two unit balls 1.4 apart along `x`, poles on `y`, unioned: the
    /// radical circle is tilted against both charts, so each ball keeps
    /// sphere faces the sphere arm meshes.
    fn tilted_sphere_pair() -> topo::Body<f64> {
        use geom_core::{Tol, Vec3};
        let tol = Tol::witness();
        let ball = |c: Vec3<f64>| {
            sweep::test_support::finished(
                "the ball",
                sweep::test_support::ball_poled_y(1.0, c, tol),
                tol,
            )
        };
        topo::boolean::union(
            &ball(Vec3::new(0.0, 0.0, 0.0)),
            &ball(Vec3::new(1.4, 0.0, 0.0)),
            tol,
        )
        .expect("the pair unions")
        .body()
        .expect("a body")
        .body
        .clone()
        .into_body()
    }

    /// **The sphere arm's chord-step bound, pinned.** On every boundary
    /// chord of every face the arm meshes, the image's UV step is within
    /// the arm's lattice steps `(h_u, h_v)` (`chords::tighten` on
    /// `geom_brep::Pcurve::speed_bound`), and on the worst chord it is more
    /// than half of them: the radical circle of this pair turns in
    /// latitude at `cos v = 0.7`, where its azimuth speed attains the
    /// bound `ρ_c/(r·cos v)`, so the bound is what sets the schedule
    /// there. A schedule at twice the step breaks the first assertion.
    #[test]
    fn the_sphere_arms_boundary_chords_step_within_its_lattice() {
        use std::collections::HashMap;
        let body = tilted_sphere_pair();
        let delta = 1e-2;
        let mut positions = Vec::new();
        let mut vids = HashMap::new();
        for (vk, p) in body.vertex_points() {
            #[allow(clippy::cast_possible_truncation)]
            vids.insert(vk, positions.len() as u32);
            positions.push(p);
        }
        let mut bounds = crate::nurbs_cert::FaceBounds::new();
        let routes = crate::tessellate::Routes::witness(&body);
        let chords: ChordPass = crate::chords::compute_chords(
            &body,
            delta,
            &routes,
            &vids,
            &mut positions,
            &mut bounds,
        )
        .unwrap();
        let (mut faces, mut worst) = (0, 0.0f64);
        for (fk, face) in body.faces() {
            let surface = body.face_surface(fk).unwrap();
            if !matches!(surface, geom::Surface::Sphere { .. })
                || routes.lane(fk).unwrap()
                    != crate::tessellate::Routed::To(crate::tessellate::Lane::Trimmed)
            {
                continue;
            }
            faces += 1;
            let row = crate::chart_table::AnalyticRow::of(&body, fk, surface).unwrap();
            let (hu, hv) = super::arm_steps(
                &row,
                super::face_target(surface, crate::tessellate::Lane::Trimmed, delta),
            );
            for (hek, ek, _) in crate::walk::loop_half_edges(&body, face.outer, fk).unwrap() {
                let image = body.pcurve(hek).unwrap().pcurve();
                let uv: Vec<_> = chords.params[&ek].iter().map(|&t| image.eval(t)).collect();
                for w in uv.windows(2) {
                    let (du, dv) = ((w[1].x - w[0].x).abs(), (w[1].y - w[0].y).abs());
                    assert!(
                        du <= hu * (1.0 + 1e-9) && dv <= hv * (1.0 + 1e-9),
                        "face {fk:?} edge {ek:?}: a chord steps ({du}, {dv}) in UV, past the \
                         lattice's ({hu}, {hv})"
                    );
                    worst = worst.max(du / hu).max(dv / hv);
                }
            }
        }
        assert!(faces > 0, "the pair has sphere-arm faces");
        assert!(
            worst > 0.5,
            "the worst boundary chord steps {worst} of a lattice step: the bound does not \
             set the schedule, so this row cannot see it loosened"
        );
    }

    /// The bodies the reach probes close this lane's cone and torus arms
    /// on (`tests/reach_probes.rs`): the tour's torus-walled vessel and the
    /// klein elbow, each revolved a quarter turn and hollowed, whose cavity
    /// walls are torus faces bounded by spirics; and both frusta split by
    /// the tilted plane through `(0, 0.5, 0)`, whose cone faces are bounded
    /// by the section ellipse.
    fn cone_and_torus_bodies() -> Vec<(String, topo::Body<f64>)> {
        use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
        use profile::path::{Open, Start};
        use profile::{ArcSweep, Center, Profile, ProfileLoop, RawLoop as _, SketchPlane};
        use sweep::{Revolution, RevolveAxis, revolve};
        let tol = Tol::witness();
        let revolved = |lp: ProfileLoop<f64>, axis: RevolveAxis<f64>, turn| {
            let profile = Profile::new(SketchPlane::xy(), vec![lp])
                .validate(tol)
                .unwrap();
            revolve(&profile, axis, turn, tol).unwrap().body.into_body()
        };
        let y_axis = |x: f64, dir: f64| RevolveAxis {
            origin: Point2::new(x, 0.0),
            dir: Vec2::new(0.0, dir),
        };
        let at = |x: f64, y: f64| Point2::new(x / 64.0, y / 64.0);
        let vessel: ProfileLoop<f64> = Open
            .at(at(0.0, 0.0))
            .line_to(at(5.0, 0.0), tol)
            .unwrap()
            .line_to(at(5.0, 4.0), tol)
            .unwrap()
            .line_to(at(9.0, 4.0), tol)
            .unwrap()
            .arc_to(
                Center {
                    c: at(6.0, 8.0),
                    winding: ArcSweep::Ccw,
                    p: at(9.0, 12.0),
                },
                tol,
            )
            .unwrap()
            .line_to(at(7.0, 12.0), tol)
            .unwrap()
            .line_to(at(7.0, 24.0), tol)
            .unwrap()
            .line_to(at(0.0, 24.0), tol)
            .unwrap()
            .line_to(Start, tol)
            .unwrap()
            .into();
        let disc = profile::test_support::bulge_loop(vec![
            (Point2::new(-0.275, 0.0), 1.0),
            (Point2::new(0.275, 0.0), 1.0),
        ]);
        let hollow = |body: topo::Body<f64>, t: f64| -> topo::Body<f64> {
            let at_rest = topo::test_support::finished("the operand", body, tol);
            topo::shell(&at_rest, t, tol).unwrap().body.into_body()
        };
        let mut out = vec![
            (
                "the sectioned vessel".to_owned(),
                hollow(
                    revolved(
                        vessel,
                        y_axis(0.0, 1.0),
                        Revolution::Partial(core::f64::consts::FRAC_PI_2),
                    ),
                    1.0 / 128.0,
                ),
            ),
            (
                "the klein elbow".to_owned(),
                hollow(
                    revolved(
                        disc,
                        y_axis(1.2, -1.0),
                        Revolution::Partial(-core::f64::consts::FRAC_PI_2),
                    ),
                    0.02,
                ),
            ),
        ];
        let plane = topo::test_support::split_plane(
            Point3::new(0.0, 0.5, 0.0),
            Vec3::new(0.3f64.sin(), 0.3f64.cos(), 0.0),
            tol,
        );
        for (name, (r0, r1)) in [("narrowing", (1.0, 0.5)), ("widening", (0.5, 1.0))] {
            let frustum = revolved(
                profile::ProfileLoop::polygon(
                    [(0.0, 0.0), (r0, 0.0), (r1, 1.0), (0.0, 1.0)].map(|(x, y)| Point2::new(x, y)),
                ),
                y_axis(0.0, 1.0),
                Revolution::Full,
            );
            let frustum = topo::test_support::finished("the frustum", frustum, tol);
            let split = topo::splitting::split(&frustum, &plane, tol).unwrap();
            for (side, part) in [("above", &split.above), ("below", &split.below)] {
                let topo::splitting::SplitPart::Body(half) = part else {
                    panic!("the {name} frustum has material {side} the cut");
                };
                out.push((
                    format!("the {name} frustum, {side}"),
                    topo::Body::clone(half),
                ));
            }
        }
        out
    }

    /// **A cone face's window reads its widest edge, a circle or not**
    /// (`crate::sizing::cone_face_rho_max`, the radius its row's lattice
    /// steps are read at). On every cone face the trimmed lane meshes in
    /// [`cone_and_torus_bodies`], the window holds the largest radius
    /// sampled on the face's edges, and stays within a fifth of it, the
    /// looseness of the stored image's box. On the narrowing frustum's
    /// upper halves the section ellipse is wider than the top rim, so the
    /// window is the non-circle arm's, and an arm that skipped it would
    /// read the rim's narrower radius and red here.
    #[test]
    fn a_cone_faces_window_reads_its_widest_edge() {
        let mut ellipse_widest = 0;
        for (name, body) in cone_and_torus_bodies() {
            let routes = crate::tessellate::Routes::witness(&body);
            for (fk, face) in body.faces() {
                let surface = body.face_surface(fk).unwrap();
                let geom::Surface::Cone {
                    frame, half_angle, ..
                } = *surface
                else {
                    continue;
                };
                let (apex, axis) = (frame.origin(), frame.axis());
                if routes.lane(fk).unwrap()
                    != crate::tessellate::Routed::To(crate::tessellate::Lane::Trimmed)
                {
                    continue;
                }
                let window = crate::sizing::cone_face_rho_max(&body, fk, half_angle).unwrap();
                let (mut widest, mut rim) = (0.0f64, 0.0f64);
                for (_, ek, _) in crate::walk::loop_half_edges(&body, face.outer, fk).unwrap() {
                    let curve = body
                        .get_curve_geom(body.get_edge(ek).unwrap().curve)
                        .unwrap()
                        .certified()
                        .unwrap();
                    let (t0, t1) = curve.params();
                    for k in 0..=256 {
                        let w = curve.carrier().eval(t0 + (t1 - t0) * f64::from(k) / 256.0) - apex;
                        let rho = (w - axis * w.dot(axis)).norm();
                        widest = widest.max(rho);
                        if matches!(curve.carrier(), geom::Curve3::Circle(..)) {
                            rim = rim.max(rho);
                        }
                    }
                }
                assert!(
                    widest * (1.0 - 1e-12) <= window && window <= widest * 1.2,
                    "{name}, face {fk:?}: the window {window} holds the widest sampled \
                     radius {widest}, within a fifth"
                );
                ellipse_widest += usize::from(widest > rim * (1.0 + 1e-9));
            }
        }
        assert!(
            ellipse_widest > 0,
            "some face's section is wider than its rim, so the non-circle arm decides"
        );
    }

    /// **The cone and torus arms' boundary chords step within their
    /// lattice.** On every boundary chord of every cone and torus face the
    /// trimmed lane meshes in [`cone_and_torus_bodies`], the image's UV step
    /// is within the arm's steps (`chords::tighten` on
    /// `geom_brep::Pcurve::speed_bound`, the spiric's and the focal
    /// section's), at the three δ the reach probes mesh at. Beside each
    /// edge it prints the chord count the edge is asked for, the count its
    /// speed bound asks of this face's lattice, and the count the image's
    /// sampled speed would ask (`--nocapture`), so an edge chorded past
    /// what its image needs is seen.
    #[test]
    fn the_cone_and_torus_arms_boundary_chords_step_within_their_lattice() {
        use std::collections::HashMap;
        let mut faces = 0;
        for (name, body) in cone_and_torus_bodies() {
            for delta in [5e-3, 1e-3, 2e-4] {
                let mut positions = Vec::new();
                let mut vids = HashMap::new();
                for (vk, p) in body.vertex_points() {
                    #[allow(clippy::cast_possible_truncation)]
                    vids.insert(vk, positions.len() as u32);
                    positions.push(p);
                }
                let mut bounds = crate::nurbs_cert::FaceBounds::new();
                let routes = crate::tessellate::Routes::witness(&body);
                let chords: ChordPass = crate::chords::compute_chords(
                    &body,
                    delta,
                    &routes,
                    &vids,
                    &mut positions,
                    &mut bounds,
                )
                .unwrap();
                for (fk, face) in body.faces() {
                    let surface = body.face_surface(fk).unwrap();
                    if !matches!(
                        surface,
                        geom::Surface::Cone { .. } | geom::Surface::Torus { .. }
                    ) || routes.lane(fk).unwrap()
                        != crate::tessellate::Routed::To(crate::tessellate::Lane::Trimmed)
                    {
                        continue;
                    }
                    faces += 1;
                    let row = crate::chart_table::AnalyticRow::of(&body, fk, surface).unwrap();
                    let (hu, hv) = super::arm_steps(
                        &row,
                        super::face_target(surface, crate::tessellate::Lane::Trimmed, delta),
                    );
                    for (hek, ek, _) in crate::walk::loop_half_edges(&body, face.outer, fk).unwrap()
                    {
                        let image = body.pcurve(hek).unwrap().pcurve();
                        let ts = &chords.params[&ek];
                        let (t0, t1) = (ts[0], ts[ts.len() - 1]);
                        let span = t1 - t0;
                        let ask = |(su, sv): (f64, f64)| {
                            let n =
                                |s: f64, h: f64| if s == 0.0 { 1.0 } else { (s * span / h).ceil() };
                            n(su, hu).max(n(sv, hv))
                        };
                        let bound = ask(image.speed_bound(t0, t1).unwrap());
                        let samples = 4096;
                        let mut sampled = (0.0f64, 0.0f64);
                        #[allow(clippy::cast_precision_loss)]
                        let at = |k: usize| image.eval(t0 + span * (k as f64 / samples as f64));
                        for k in 0..samples {
                            let (a, b) = (at(k), at(k + 1));
                            #[allow(clippy::cast_precision_loss)]
                            let dt = span / samples as f64;
                            sampled.0 = sampled.0.max((b.x - a.x).abs() / dt);
                            sampled.1 = sampled.1.max((b.y - a.y).abs() / dt);
                        }
                        // The carrier's own count at the edge's target, and
                        // the one its sampled second derivative would ask
                        // (`h²·sup|C″|/8 ≤ target`).
                        let carrier = body
                            .get_curve_geom(body.get_edge(ek).unwrap().curve)
                            .unwrap()
                            .certified()
                            .unwrap()
                            .carrier();
                        let target =
                            super::face_target(surface, crate::tessellate::Lane::Trimmed, delta);
                        // The new arms size at the lane's headroom, as the
                        // cylinder and sphere arms do: it stands in for
                        // the containment lemma no arm has yet.
                        assert_eq!(
                            target.to_bits(),
                            (delta * super::UNPROVED_HEADROOM).to_bits(),
                            "{name}, δ {delta:e}, face {fk:?}: a trimmed {:?} face sizes at the \
                             lane's headroom",
                            surface.kind()
                        );
                        let own = match *carrier {
                            geom::Curve3::Spiric {
                                major_radius,
                                minor_radius,
                                offset,
                                ..
                            } => (span
                                / crate::sizing::spiric_step(
                                    target,
                                    major_radius,
                                    minor_radius,
                                    offset,
                                ))
                            .ceil(),
                            geom::Curve3::Ellipse(ref e) => {
                                (span / crate::sizing::ellipse_step(target, e)).ceil()
                            }
                            _ => f64::NAN,
                        };
                        let mut curvature = 0.0f64;
                        #[allow(clippy::cast_precision_loss)]
                        let c = |k: usize| carrier.eval(t0 + span * (k as f64 / samples as f64));
                        #[allow(clippy::cast_precision_loss)]
                        let dt = span / samples as f64;
                        for k in 1..samples {
                            let d2 = (c(k + 1) - c(k)) - (c(k) - c(k - 1));
                            curvature = curvature.max(d2.norm() / (dt * dt));
                        }
                        let own_sampled = (span * (curvature / (8.0 * target)).sqrt()).ceil();
                        eprintln!(
                            "{name}, δ {delta:e}, {:?} face {fk:?}, edge {ek:?} ({}): asked {}, \
                             its bound asks {bound}, its sampled speed {}; its carrier asks \
                             {own}, its sampled curvature {own_sampled}",
                            surface.kind(),
                            image_form(image),
                            ts.len() - 1,
                            ask(sampled)
                        );
                        let uv: Vec<_> = ts.iter().map(|&t| image.eval(t)).collect();
                        for w in uv.windows(2) {
                            let (du, dv) = ((w[1].x - w[0].x).abs(), (w[1].y - w[0].y).abs());
                            assert!(
                                du <= hu * (1.0 + 1e-9) && dv <= hv * (1.0 + 1e-9),
                                "{name}, δ {delta:e}, face {fk:?} edge {ek:?}: a chord steps \
                                 ({du}, {dv}) in UV, past the lattice's ({hu}, {hv})"
                            );
                        }
                    }
                }
            }
        }
        // Three walls (one on the vessel, two on the elbow) and two cone
        // faces a half, at three δ.
        assert_eq!(faces, 3 * (3 + 2 * 4), "the bodies' cone and torus faces");
    }

    /// The form of a stored chart image, for a row's report.
    fn image_form(image: &geom_brep::Pcurve<f64>) -> &'static str {
        match image {
            geom_brep::Pcurve::Harmonic { .. } => "harmonic",
            geom_brep::Pcurve::Spiric { .. } => "spiric",
            geom_brep::Pcurve::FocalSection(_) => "focal section",
            geom_brep::Pcurve::Projected(_) => "projected",
            geom_brep::Pcurve::IsoLine { .. } => "iso line",
            geom_brep::Pcurve::IsoArc { .. } => "iso arc",
            geom_brep::Pcurve::Fitted(_) => "fitted",
            geom_brep::Pcurve::General(_) => "general",
        }
    }

    /// **An over-δ triangle on the sphere arm refuses typed through
    /// `crate::tessellate`.** The arm's lattice and boundary steps are
    /// scaled four-fold (`STEP_SCALE`, test-only), so its cells
    /// deviate by about sixteen times the share of δ they were sized at,
    /// and the per-triangle certificate refuses the face
    /// `CertificateExceeded`; at scale one the same body meshes. The face
    /// map runs on rayon, so the switch is set inside a one-thread pool,
    /// on the worker that runs the face.
    #[test]
    fn an_over_delta_sphere_arm_triangle_refuses_typed() {
        use geom_core::Tol;
        let body = tilted_sphere_pair();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .expect("a one-thread pool");
        let (coarse, sized) = pool.install(|| {
            STEP_SCALE.with(|s| s.set(4.0));
            let coarse = crate::tessellate(&body, 1e-2, Tol::witness());
            STEP_SCALE.with(|s| s.set(1.0));
            (coarse, crate::tessellate(&body, 1e-2, Tol::witness()))
        });
        assert!(
            matches!(
                coarse,
                Err(TessellateError::CertificateExceeded { bound, requested, .. })
                    if bound > requested
            ),
            "steps scaled four-fold: the face refuses typed, got {:?}",
            coarse.map(|m| m.patches.len())
        );
        let mesh = sized.expect("at the arm's own steps the body meshes");
        crate::validate::check_mesh(&mesh).expect("and the mesh checks");
        // Two unit balls 1.4 apart: twice the ball less their lens,
        // `π(4r + d)(2r − d)²/12`.
        let pi = core::f64::consts::PI;
        let volume = 2.0 * 4.0 / 3.0 * pi - pi * 5.4 * 0.6f64.powi(2) / 12.0;
        // Its area: two spheres less the cap each loses, `2πrh` at
        // `h = 1 − 0.7`.
        let area = 8.0 * pi - 2.0 * 2.0 * pi * 0.3;
        crate::validate::check_volume(&mesh, volume, area, 1e-2).expect("and it encloses the body");
    }

    /// **A singular vertex is a side of the polygon** (`crate::chart_table`,
    /// [`super::flat_points`]): a loop arriving at a singular vertex along
    /// one `u` and leaving along another keeps the vertex twice, once at
    /// each `u`; leaving along the same `u` it keeps it once; and the same
    /// turn at a vertex that is no singular point keeps it once. Keeping a
    /// point the next traversal starts from, or reading the turn alone,
    /// reds here.
    #[test]
    fn a_singular_vertex_is_kept_once_at_each_u() {
        let kept = |leave_u: f64, singular: &HashSet<u32>| {
            let travs = vec![
                vec![(0.3, 0.5, 1), (0.3, 0.0, 9)],
                vec![(leave_u, 0.0, 9), (leave_u, 0.5, 2)],
                vec![(leave_u, 0.5, 2), (0.3, 0.5, 1)],
            ];
            super::flat_points(&travs, singular)
                .filter(|(_, p)| p.2 == 9)
                .map(|(_, p)| (p.0, p.1))
                .collect::<Vec<_>>()
        };
        let at_apex: HashSet<u32> = [9].into_iter().collect();
        assert_eq!(
            kept(1.2, &at_apex),
            [(0.3, 0.0), (1.2, 0.0)],
            "a singular vertex at the arriving and the leaving u"
        );
        assert_eq!(
            kept(0.3, &at_apex),
            [(0.3, 0.0)],
            "leaving along the arriving u, the vertex once"
        );
        assert_eq!(
            kept(1.2, &HashSet::new()).len(),
            1,
            "a turn at a vertex that is no singular point is no side"
        );
    }

    /// **Whether a vertex is a singular point is decided at the band, in
    /// model space** ([`crate::chart_table::AnalyticRow::is_singular`]), on
    /// charts posed far from the origin and at scales from `1e-2` to `1e3`:
    /// the singular point itself and a point half the band's zero from it
    /// are it, and a point twice the band's escalation from it is not. A
    /// reading in the chart's `v`, or a threshold of its own, reds here at
    /// one scale or another (the cone's `v` is slant metres: a fixed
    /// threshold there is a size).
    #[test]
    fn a_singular_point_is_decided_at_the_band_at_every_scale_and_pose() {
        use crate::chart_table::Analytic;
        use geom::Surface;
        use geom_core::{Point3, Tol, Vec3, predicate::Band};
        let band = Band::linear(Tol::witness()).unwrap();
        let far = Point3::new(700.3, -577.7, 123.4);
        let axis = Vec3::new(1.0, 2.0, 3.0).normalize();
        let u_ref = Vec3::new(2.0, -1.0, 0.0).normalize();
        let off = Vec3::new(0.0, 3.0, -2.0).normalize();
        let mut seen = 0;
        for scale in [1e-2, 1.0, 1e3] {
            let surfaces = [
                (
                    Surface::Cone {
                        frame: geom::Frame::new(far, axis, u_ref),
                        half_angle: 0.5,
                    },
                    Analytic::Cone {
                        half_angle: 0.5,
                        rho_max: scale,
                    },
                ),
                (
                    Surface::Sphere {
                        frame: geom::Frame::new(far, axis, u_ref),
                        radius: scale,
                    },
                    Analytic::Sphere {
                        radius: scale,
                        widest: 1.0,
                    },
                ),
                (
                    Surface::Spindle {
                        frame: geom::Frame::new(far, axis, u_ref),
                        major_radius: -0.5 * scale,
                        minor_radius: scale,
                    },
                    Analytic::Spindle {
                        major: -0.5 * scale,
                        minor: scale,
                    },
                ),
            ];
            for (surface, kind) in &surfaces {
                let row = crate::chart_table::AnalyticRow::new(surface, 1.0).unwrap();
                assert!(
                    !row.singular_points().is_empty(),
                    "{kind:?}: a chart with a singular point"
                );
                for s in row.singular_points() {
                    let p = s.point;
                    assert_eq!(
                        row.is_singular(p, band),
                        Ok(true),
                        "{kind:?}: the point itself"
                    );
                    assert_eq!(
                        row.is_singular(p + off * (0.5 * band.zero()), band),
                        Ok(true),
                        "{kind:?}: half the band's zero off it"
                    );
                    assert_eq!(
                        row.is_singular(p + off * (2.0 * band.escalate()), band),
                        Ok(false),
                        "{kind:?}: twice the band's escalation off it"
                    );
                    assert!(
                        row.is_singular(p + off * (0.5 * (band.zero() + band.escalate())), band)
                            .is_err(),
                        "{kind:?}: between the band's zero and its escalation, undecided, which \
                         `singular_vertices` refuses typed"
                    );
                    seen += 1;
                }
            }
            for (surface, kind) in [
                (
                    Surface::Cylinder {
                        frame: geom::Frame::new(far, axis, u_ref),
                        radius: scale,
                    },
                    Analytic::Cylinder { radius: scale },
                ),
                (
                    Surface::Torus {
                        frame: geom::Frame::new(far, axis, u_ref),
                        major_radius: 2.0 * scale,
                        minor_radius: 0.5 * scale,
                    },
                    Analytic::Torus {
                        major: 2.0 * scale,
                        minor: 0.5 * scale,
                    },
                ),
            ] {
                assert!(
                    crate::chart_table::AnalyticRow::new(&surface, 1.0)
                        .unwrap()
                        .singular_points()
                        .is_empty(),
                    "{kind:?}: no singular point"
                );
            }
        }
        assert_eq!(
            seen,
            3 * (1 + 2 + 2),
            "the cone's apex, the sphere's poles, the sheet's apexes"
        );
    }

    /// **`pole_side_columns` takes the lattice off #678's shape.** A
    /// pole side over `u ∈ [0.45, 0.55]` on a box `[0, 1]` has exactly
    /// one column strictly between its ends at `nu = 2` (at ½), and the
    /// first doubling's new columns (¼, ¾) land outside it; the count
    /// must keep doubling until two columns stand between, and a side
    /// with none or two between is left as it is.
    #[test]
    fn pole_side_columns_doubles_past_one_column_between() {
        let half_pi = core::f64::consts::FRAC_PI_2;
        let polygon = [
            (0.0, 0.0, 1u32),
            (0.45, half_pi, 7),
            (0.55, half_pi, 7),
            (1.0, 0.0, 2),
        ];
        let between = |nu: usize| {
            (1..nu)
                .filter(|&i| {
                    #[allow(clippy::cast_precision_loss)]
                    let u = i as f64 / nu as f64;
                    u > 0.45 && u < 0.55
                })
                .count()
        };
        assert_eq!(between(2), 1);
        assert_eq!(between(4), 1, "the first doubling alone does not clear it");
        let pole: HashSet<u32> = [7].into_iter().collect();
        let polygon = super::TrimPolygon::of_loops(&polygon, &[]);
        let nu = super::pole_side_columns(&polygon, &pole, (0.0, 1.0), 2).unwrap();
        assert!(between(nu) >= 2, "nu {nu} leaves {} between", between(nu));
        assert_eq!(
            super::pole_side_columns(&polygon, &pole, (0.0, 1.0), 3).unwrap(),
            3
        );
    }

    /// **The tilted pair's faces meet their poles as polygon sides, and
    /// the runs between the poles sit where the region is.** Each ball's
    /// half-band remnant passes both chart poles at loop vertices
    /// (`y`-poled balls, seam meridians pole to pole), so its polygon
    /// carries two pole sides ([`super::flat_points`]) and the run
    /// between them sits where the loop's lift puts it, through the
    /// face's turn at each pole: every such face's equal-area shoelace
    /// is positive and under `4π`. Without the kept pole the polygon
    /// cuts the cap off the pole, and a run a whole turn off would bound
    /// the face's complement plus the cap past the pole.
    #[test]
    fn the_tilted_pairs_faces_keep_their_poles_as_sides() {
        use std::collections::HashMap;
        let body = tilted_sphere_pair();
        let mut positions = Vec::new();
        let mut vids = HashMap::new();
        for (vk, p) in body.vertex_points() {
            #[allow(clippy::cast_possible_truncation)]
            vids.insert(vk, positions.len() as u32);
            positions.push(p);
        }
        let mut bounds = crate::nurbs_cert::FaceBounds::new();
        let routes = crate::tessellate::Routes::witness(&body);
        let chords: ChordPass =
            crate::chords::compute_chords(&body, 1e-2, &routes, &vids, &mut positions, &mut bounds)
                .unwrap();
        let mut with_poles = 0;
        for (fk, face) in body.faces() {
            let surface = body.face_surface(fk).unwrap();
            if !matches!(surface, geom::Surface::Sphere { .. })
                || routes.lane(fk).unwrap()
                    != crate::tessellate::Routed::To(crate::tessellate::Lane::Trimmed)
            {
                continue;
            }
            let crate::chart_table::ChartRow::Analytic(row) =
                crate::chart_table::ChartRow::of(&body, fk, surface, &bounds).unwrap()
            else {
                panic!("a sphere face has an analytic row");
            };
            let lift = body.face_lift(fk, band()).unwrap();
            let singular = super::singular_vertices(&body, fk, &lift, &chords, &positions, |p| {
                row.is_singular(p, band())
            })
            .unwrap();
            let polygon = super::trim_polygon(
                &body,
                fk,
                &lift,
                &chords,
                false,
                None,
                (&singular, &super::SidePoints::default()),
            )
            .unwrap()
            .points;
            let n = polygon.len();
            let pole_sides = (0..n)
                .filter(|&i| {
                    let (a, b) = (polygon[i], polygon[(i + 1) % n]);
                    a.2 == b.2 && singular.contains(&a.2)
                })
                .count();
            let area = 0.5
                * (0..n)
                    .map(|i| {
                        let (a, b) = (polygon[i], polygon[(i + 1) % n]);
                        a.0 * b.1.sin() - b.0 * a.1.sin()
                    })
                    .sum::<f64>();
            assert_eq!(
                PolygonOrientation::of_signed_area(area),
                Some(PolygonOrientation::of_face(face.sense)),
                "face {fk:?}: the polygon is oriented as its face's sense says ({area})"
            );
            assert!(
                area.abs() < 4.0 * core::f64::consts::PI,
                "face {fk:?}: equal-area {area}"
            );
            if pole_sides == 2 {
                with_poles += 1;
            }
        }
        assert!(
            with_poles >= 2,
            "both balls' remnants pass both poles: {with_poles}"
        );
    }

    /// [`super::trim_orientation`]'s three answers, on polygons written
    /// out: the polygon's own orientation where it is the face's; a
    /// typed refusal where an analytic arm's contradicts its face, and
    /// the polygon's own on the spline arm, which does not compare; and
    /// no orientation on every arm for a polygon of no signed area (a
    /// figure-eight of equal lobes, three points on one line), which the
    /// lane then refuses by what its constraint pass reads.
    #[test]
    fn a_trim_polygon_is_oriented_as_read_and_one_of_no_area_has_none() {
        use PolygonOrientation::{Clockwise, CounterClockwise};
        let fk = FaceKey::default();
        let points = |uv: &[(f64, f64)]| -> Vec<(f64, f64, u32)> {
            uv.iter().map(|&(u, v)| (u, v, 0)).collect()
        };
        let poly = |uv: &[(f64, f64)]| super::TrimPolygon::of_loops(&points(uv), &[]);
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let ccw = poly(&square);
        let cw = poly(&square.iter().rev().copied().collect::<Vec<_>>());
        for (polygon, has) in [(&ccw, CounterClockwise), (&cw, Clockwise)] {
            let promised = has == CounterClockwise;
            for nurbs_chart in [false, true] {
                assert_eq!(
                    super::trim_orientation(fk, promised, nurbs_chart, polygon).ok(),
                    Some(Some(has)),
                    "{has:?}, spline arm {nurbs_chart}: oriented as its face is"
                );
            }
            assert!(
                matches!(
                    super::trim_orientation(fk, !promised, false, polygon),
                    Err(TessellateError::TrimLoopOrientation { face }) if face == fk
                ),
                "{has:?}: an analytic arm refuses a polygon against its face"
            );
            assert_eq!(
                super::trim_orientation(fk, !promised, true, polygon).ok(),
                Some(Some(has)),
                "{has:?}: the spline arm takes the polygon's, whatever the bit"
            );
        }
        let eight = poly(&[(0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 1.0)]);
        let line = poly(&[(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)]);
        // A ring winds against its outer loop on every arm: one wound with
        // it refuses, and one of no area has no orientation either.
        let outer = points(&[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]);
        let hole = points(&[(1.0, 1.0), (1.0, 2.0), (2.0, 2.0), (2.0, 1.0)]);
        let with = points(&[(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0)]);
        let flat = points(&[(1.0, 1.0), (2.0, 2.0), (3.0, 3.0)]);
        for nurbs_chart in [false, true] {
            assert_eq!(
                super::trim_orientation(
                    fk,
                    true,
                    nurbs_chart,
                    &super::TrimPolygon::of_loops(&outer, &[&hole])
                )
                .ok(),
                Some(Some(CounterClockwise)),
                "spline arm {nurbs_chart}: a ring wound against its outer loop is the face's"
            );
            assert!(
                matches!(
                    super::trim_orientation(
                        fk,
                        true,
                        nurbs_chart,
                        &super::TrimPolygon::of_loops(&outer, &[&hole, &with])
                    ),
                    Err(TessellateError::TrimLoopOrientation { face }) if face == fk
                ),
                "spline arm {nurbs_chart}: a ring wound with its outer loop refuses"
            );
            assert_eq!(
                super::trim_orientation(
                    fk,
                    true,
                    nurbs_chart,
                    &super::TrimPolygon::of_loops(&outer, &[&flat])
                ),
                Ok(None),
                "spline arm {nurbs_chart}: a ring of no area has no orientation"
            );
        }
        for (name, polygon) in [("figure-eight", &eight), ("collinear", &line)] {
            for (sense, nurbs_chart) in [(true, false), (false, false), (true, true), (false, true)]
            {
                assert_eq!(
                    super::trim_orientation(fk, sense, nurbs_chart, polygon),
                    Ok(None),
                    "{name}, sense {sense}, spline arm {nurbs_chart}: no area, no orientation"
                );
            }
        }
    }

    /// A sliver of a rod: the unit disc extruded 2 along `z`, met by a
    /// slab turned 0.05 about `y` whose edge sits `0.01` inside the wall
    /// at mid-height. The intersection's cylinder face is about two
    /// lattice cells across, between two section-ellipse arcs whose plane
    /// faces have no interior point, so each joins the points of its arc
    /// freely.
    fn a_rod_sliver() -> topo::Body<f64> {
        use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
        let tol = Tol::witness();
        let disc = profile::Profile::new(
            profile::SketchPlane::xy(),
            vec![
                profile::circle(Point2::new(0.0, 0.0), 1.0, tol)
                    .unwrap()
                    .into(),
            ],
        )
        .validate(tol)
        .unwrap();
        let depth = sweep::Extrusion::Distance {
            depth: 2.0,
            side: sweep::ExtrudeSide::Along,
        };
        let rod = sweep::test_support::finished(
            "the rod",
            sweep::extrude(&disc, depth, tol).unwrap().body.into_body(),
            tol,
        );
        let pose = Affine3::translation(Vec3::new(0.99, 0.0, 1.0))
            * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), 0.05);
        let slab = sweep::test_support::finished(
            "the slab",
            topo::transform_rigid(
                &sweep::test_support::brick((0.0, 6.0), (-3.0, 3.0), (-0.4, 0.4), tol),
                &geom_core::test_support::rigid(&pose, tol),
            )
            .unwrap(),
            tol,
        );
        let met = topo::intersect(&rod, &slab, tol).unwrap();
        topo::Body::clone(&met.body().expect("a sliver").body)
    }

    /// **A side the retry budget leaves refuses `UnclearedSharedSide`.** The rod
    /// sliver's cylinder face keeps, on its first attempt, a side between
    /// two points of one ellipse arc that are not neighbours on it, which
    /// the plane face across that arc joins too; the shared-side cure
    /// splits it on the next attempt. With the loop's last attempt set to
    /// the first (`LAST_ATTEMPT`, test-only) there is no next attempt, so
    /// the face refuses typed; with the budget as shipped it meshes. Set
    /// on a one-thread pool's worker, which runs the faces.
    #[test]
    fn a_shared_side_the_budget_leaves_refuses_typed() {
        use geom_core::Tol;
        let body = a_rod_sliver();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .expect("a one-thread pool");
        pool.install(|| {
            for delta in [2e-2, 5e-3] {
                LAST_ATTEMPT.with(|a| a.set(Some(0)));
                let first = crate::tessellate(&body, delta, Tol::witness());
                LAST_ATTEMPT.with(|a| a.set(None));
                assert!(
                    matches!(first, Err(TessellateError::UnclearedSharedSide { .. })),
                    "δ {delta:e}: the first attempt keeps the side, got {first:?}"
                );
                let m = crate::tessellate(&body, delta, Tol::witness())
                    .unwrap_or_else(|e| panic!("δ {delta:e}: at the shipped budget, {e:?}"));
                crate::validate::check_mesh(&m)
                    .unwrap_or_else(|e| panic!("δ {delta:e}: and it checks, got {e:?}"));
            }
        });
    }

    /// Whether the closed rectangle about `c`, `half` each way, meets the
    /// segment `a`–`b`, by clipping the segment to the rectangle's four
    /// sides in turn: the reference [`super::Side::meets`] is held to.
    fn clipped(c: (f64, f64), half: (f64, f64), a: (f64, f64), b: (f64, f64)) -> bool {
        let (mut t0, mut t1) = (0.0f64, 1.0f64);
        for (p, d, lo, hi) in [
            (a.0, b.0 - a.0, c.0 - half.0, c.0 + half.0),
            (a.1, b.1 - a.1, c.1 - half.1, c.1 + half.1),
        ] {
            if d == 0.0 {
                if p < lo || p > hi {
                    return false;
                }
                continue;
            }
            let (ta, tb) = ((lo - p) / d, (hi - p) / d);
            t0 = t0.max(ta.min(tb));
            t1 = t1.min(ta.max(tb));
        }
        t0 <= t1
    }

    /// **A node is laid exactly where its box meets no side of any loop**
    /// ([`super::lay`], `crates/mesh/README.md` `[mesh.sizing]`): on an
    /// oblique quadrilateral holding a triangular ring, under a lattice
    /// whose steps differ by axis, the nodes laid are those whose box of
    /// `c` own steps each way no side clips, at the shipped clearance, at
    /// twice it and at four times. Every loop is read (a ring's sides drop nodes),
    /// the box is per axis (the steps differ), and a wider clearance lays
    /// no node a narrower one does not.
    #[test]
    fn a_node_is_laid_exactly_where_its_box_meets_no_side() {
        let outer = [(0.0, 0.0, 0), (4.0, 0.6, 1), (3.1, 3.0, 2), (0.4, 2.2, 3)];
        let ring = [(1.5, 1.0, 4), (2.5, 1.9, 5), (1.4, 1.7, 6)];
        let trim = super::TrimPolygon::of_loops(&outer, &[&ring]);
        let sides: Vec<((f64, f64), (f64, f64))> = (0..trim.points.len())
            .map(|i| (trim.points[i], trim.points[trim.next(i)]))
            .map(|(a, b)| ((a.0, a.1), (b.0, b.1)))
            .collect();
        #[allow(clippy::cast_precision_loss)]
        let rows: Vec<f64> = (1..23).map(|j| 3.0 * (j as f64 / 23.0)).collect();
        let nodes = super::lattice_candidates((0.0, 4.0), (0.0, 3.0), 37, &rows);
        let plane = super::CdtPlane::Lattice { metric: 1.0 };
        let mut before: Option<Vec<(f64, f64)>> = None;
        let shipped = super::BOUNDARY_CLEARANCE;
        for c in [4.0 * shipped, 2.0 * shipped, shipped] {
            let laid = super::lay(&nodes, c, plane, &trim, &HashSet::new(), None);
            let (mut near_outer, mut near_ring) = (0, 0);
            for n in &nodes {
                let half = (c * n.step.0, c * n.step.1);
                let near: Vec<bool> = sides
                    .iter()
                    .map(|&(a, b)| clipped((n.u, n.v), half, a, b))
                    .collect();
                near_outer += usize::from(near[..4].iter().any(|&x| x));
                near_ring += usize::from(near[4..].iter().any(|&x| x));
                assert_eq!(
                    laid.contains(&(n.u, n.v)),
                    !near.iter().any(|&x| x),
                    "clearance {c}: the node ({}, {}) is laid exactly when no side clips its box",
                    n.u,
                    n.v
                );
            }
            assert!(
                near_outer > 0 && near_ring > 0 && laid.len() > nodes.len() / 2,
                "clearance {c}: nodes are dropped beside the outer loop ({near_outer}) and \
                 beside the ring ({near_ring}), and most are laid ({})",
                laid.len()
            );
            if let Some(before) = &before {
                assert!(
                    before.iter().all(|p| laid.contains(p)) && before.len() < laid.len(),
                    "clearance {c}: a narrower clearance lays every node a wider one does, and more"
                );
            }
            before = Some(laid);
        }
    }

    /// **A crease's clearance is `c` of the node's own step across it, to
    /// the node** ([`super::lay`]): of four columns laid just inside and
    /// just outside `c·h` either side of a crease, the two inside are not
    /// laid and the two outside are, in each direction; and the crease's
    /// own points on those rows are laid on the line itself.
    #[test]
    fn a_node_is_laid_just_outside_a_creases_clearance_and_not_just_inside() {
        let (x, c) = (0.37f64, super::BOUNDARY_CLEARANCE);
        let h = 0.04;
        for axis in [0, 1] {
            let creases = creases(axis, &[x]);
            let offsets = [-1.01, -0.99, 0.99, 1.01].map(|k| k * c * h);
            let nodes: Vec<Node> = [0.3, 0.6]
                .into_iter()
                .flat_map(|along| offsets.map(|d| (x + d, along)))
                .map(|(across, along)| {
                    if axis == 0 {
                        (across, along)
                    } else {
                        (along, across)
                    }
                })
                .map(|(u, v)| Node {
                    u,
                    v,
                    step: (h, h),
                    on_crease: false,
                })
                .collect();
            let mut nodes = nodes;
            super::sort_nodes(&mut nodes);
            creases.cut_candidates(&mut nodes);
            let outer = [(0.0, 0.0, 0), (1.0, 0.0, 1), (1.0, 1.0, 2), (0.0, 1.0, 3)];
            let laid = super::lay(
                &nodes,
                c,
                super::CdtPlane::Lattice { metric: 1.0 },
                &super::TrimPolygon::of_loops(&outer, &[]),
                &HashSet::new(),
                Some(&creases),
            );
            let across = |p: &(f64, f64)| if axis == 0 { p.0 } else { p.1 };
            let mut got: Vec<f64> = laid.iter().map(|p| (across(p) - x) / (c * h)).collect();
            got.sort_by(f64::total_cmp);
            let want = [-1.01, -1.01, 0.0, 0.0, 1.01, 1.01];
            assert!(
                got.len() == want.len() && got.iter().zip(want).all(|(g, w)| (g - w).abs() < 1e-9),
                "axis {axis}: laid at {got:?} clearances from the crease, want {want:?}"
            );
        }
    }

    /// **A location two bands lay is laid only where both clear**
    /// ([`super::Node`], "its own step"; [`super::lay`]): one location at
    /// a small step and at a large one, a side passing inside the large
    /// step's box and outside the small one's, is not laid; with the side
    /// outside both it is. A crease's own point follows the same rule,
    /// once for each band on its line ([`Creases::cut_candidates`]).
    #[test]
    fn a_location_two_bands_lay_is_laid_only_where_both_clear() {
        let c = super::BOUNDARY_CLEARANCE;
        let at = (0.5, 0.5);
        let copies = |crease| {
            let mut nodes: Vec<Node> = [0.1, 0.4]
                .into_iter()
                .map(|h| Node {
                    u: at.0,
                    v: at.1,
                    step: (h, h),
                    on_crease: crease,
                })
                .collect();
            super::sort_nodes(&mut nodes);
            nodes
        };
        // A triangle whose side `v = 0.5 + d` runs past the location.
        let polygon = |d: f64| {
            super::TrimPolygon::of_loops(
                &[(0.0, at.1 + d, 0), (1.0, at.1 + d, 1), (0.5, 3.0, 2)],
                &[],
            )
        };
        let lay = |nodes: &[Node], d: f64| {
            super::lay(
                nodes,
                c,
                super::CdtPlane::Lattice { metric: 1.0 },
                &polygon(d),
                &HashSet::new(),
                None,
            )
        };
        for crease in [false, true] {
            let nodes = copies(crease);
            let (between, outside) = (0.5 * (0.1 + 0.4) * c, 2.0 * 0.4 * c);
            assert_eq!(
                lay(&nodes, between),
                [],
                "crease {crease}: a side inside the larger box alone: not laid"
            );
            assert_eq!(
                lay(&nodes[..1], between),
                [at],
                "crease {crease}: the smaller band alone clears that side"
            );
            assert_eq!(
                lay(&nodes, outside),
                [at],
                "crease {crease}: a side outside both boxes: laid, once"
            );
        }
        // On a row two bands lay, the crease gains a point for each.
        let creases = creases(0, &[0.37]);
        let mut row = copies(false);
        creases.cut_candidates(&mut row);
        assert_eq!(
            row.iter().filter(|n| n.on_crease && n.u == 0.37).count(),
            2,
            "a crease point at each band's steps: {row:?}"
        );
    }

    /// **On a cone face through its apex, a node is laid exactly where
    /// its sector meets no side** ([`super::lay`] against
    /// [`super::Side::meets`] read side by side with no window): a loop
    /// that keeps the apex once, up one ruling and back down another. A
    /// side with an end at the apex is met by every sector that reaches
    /// the origin, at any azimuth, so its search has no window in `u`.
    #[test]
    fn a_side_from_the_apex_is_searched_at_every_azimuth() {
        let plane = super::CdtPlane::Development {
            sin_alpha: 0.5f64.sin(),
            nappe: 1.0,
        };
        let singular: HashSet<u32> = [0].into_iter().collect();
        let outer = [(1.0, 0.0, 0), (1.0, 2.0, 1), (2.0, 2.0, 2)];
        let trim = super::TrimPolygon::of_loops(&outer, &[]);
        // Rows down to the apex, the first within its own clearance of
        // it. Written out: the lane's own rows keep a gap from the box's
        // end, and no sector of theirs reaches the apex.
        let mut nodes: Vec<Node> = [0.05, 0.4, 0.8, 1.2, 1.6]
            .into_iter()
            .flat_map(|v| {
                (1..30).map(move |i| Node {
                    u: 0.1 * f64::from(i),
                    v,
                    step: (0.1, 0.4),
                    on_crease: false,
                })
            })
            .collect();
        super::sort_nodes(&mut nodes);
        let c = 0.25;
        let laid = super::lay(&nodes, c, plane, &trim, &singular, None);
        let mut beside_the_apex = 0;
        for n in &nodes {
            let meets = (0..3).any(|i| {
                super::Side::of(plane, outer[i], outer[(i + 1) % 3], &singular).meets(n, c)
            });
            assert_eq!(
                laid.contains(&(n.u, n.v)),
                !meets,
                "the node ({}, {}) is laid exactly when no side meets its sector",
                n.u,
                n.v
            );
            beside_the_apex += usize::from(meets && !(0.9..=2.1).contains(&n.u));
        }
        assert!(
            beside_the_apex > 0,
            "some node off the loop's azimuths holds the apex in its sector"
        );
    }

    /// Whether `p` is strictly inside the triangle `at`, by the predicate
    /// the lane's triangulation itself reads: spade's exact point location
    /// in the triangulation of the three corners. `None` where spade
    /// holds the corners as one line (no triangle to be inside).
    fn strictly_inside(at: [SpadePoint<f64>; 3], p: SpadePoint<f64>) -> Option<bool> {
        use spade::PositionInTriangulation as P;
        let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
            ConstrainedDelaunayTriangulation::new();
        for q in at {
            cdt.insert(q).ok()?;
        }
        (cdt.num_inner_faces() == 1).then(|| match cdt.locate(p) {
            P::OnFace(_) => true,
            P::OnVertex(_) | P::OnEdge(_) | P::OutsideOfConvexHull(_) | P::NoTriangulation => false,
        })
    }

    /// **A point the lane places inside a kept triangle is strictly inside
    /// it, down to a stated thinness** ([`super::TrimmedFace::inside_point`],
    /// held to spade's exact location, [`strictly_inside`]). An
    /// enumeration of needle triangles, each read where the CDT runs:
    ///
    /// - **In the chart**, at three lattice metrics, three places (the
    ///   last a thousand units out), two lengths, five directions and
    ///   three apex positions, heights from a tenth of the length down to
    ///   `10⁻¹⁵` of it: the point is strictly inside every triangle whose
    ///   height is at least four f64 epsilons of its largest coordinate.
    ///   (The misses of the table are all under half of one: triangles f64
    ///   does not hold as triangles.)
    /// - **In the cone's development**, at three half-angles on both
    ///   nappes, with two corners on one ruling, with an apex corner and
    ///   with neither: the point is strictly inside every triangle whose
    ///   developed height over its longest side is at least `2⁻²⁵` of its
    ///   farthest corner's radius, the bound [`super::developed`] states
    ///   for its own cut. **Under that it is not**: the point is read back
    ///   to the chart and developed again, and the cut can carry it onto a
    ///   side. The last assertion pins one such triangle, an apex corner
    ///   and a ruling `10⁻⁸` rad from its third corner, whose point lands
    ///   on an edge, where the lane refuses `Triangulation`
    ///   (`work/tesslane/a-point-placed-inside-a-developed-needle-triangle-can-land-on-its-side.md`).
    #[test]
    fn a_point_placed_inside_a_kept_triangle_is_strictly_inside_it() {
        let mut held = 0;
        for metric in [1.0, 1.0 / 32.0, 64.0] {
            let plane = super::CdtPlane::Lattice { metric };
            for (ax, ay) in [(0.1, 0.2), (3.0, -2.0), (1000.3, 500.1)] {
                for len in [1.0, 1e-3] {
                    for phi in [0.0f64, 0.3, 0.8, 1.2, core::f64::consts::FRAC_PI_2] {
                        for t in [0.03, 0.5, 0.97] {
                            for k in 1..=15 {
                                let h = len * 10f64.powi(-k);
                                let (c, sn) = (phi.cos(), phi.sin());
                                let uv = [
                                    [ax, ay],
                                    [ax + len * c, ay + len * sn],
                                    [ax + t * len * c - h * sn, ay + t * len * sn + h * c],
                                ];
                                let scale = uv.iter().flatten().fold(0.0f64, |m, x| m.max(x.abs()));
                                if h < 4.0 * f64::EPSILON * scale {
                                    continue;
                                }
                                let at = uv.map(|p| super::cdt_point(plane, p[0], p[1], false));
                                let (u, v) =
                                    super::TrimmedFace::inside_point(plane, &uv, at, [false; 3]);
                                assert_eq!(
                                    strictly_inside(at, super::cdt_point(plane, u, v, false)),
                                    Some(true),
                                    "the chart at metric {metric}: the point ({u}, {v}) of {uv:?}"
                                );
                                held += 1;
                            }
                        }
                    }
                }
            }
        }
        assert!(held > 3000, "the chart's table holds {held} triangles");
        let developed_at = |plane, uv: [[f64; 2]; 3], apex: [bool; 3]| {
            let at = [0, 1, 2].map(|i| super::cdt_point(plane, uv[i][0], uv[i][1], apex[i]));
            let (u, v) = super::TrimmedFace::inside_point(plane, &uv, at, apex);
            (at, (u, v))
        };
        let mut held = 0;
        for alpha in [0.02f64, 0.5, 1.4] {
            for nappe in [1.0, -1.0] {
                let plane = super::CdtPlane::Development {
                    sin_alpha: alpha.sin(),
                    nappe,
                };
                for u0 in [0.1, 3.0, 6.0] {
                    for v0 in [1e-3, 0.4, 50.0] {
                        for (du, dv) in [(0.3, 1.0), (1.0, 0.3), (0.05, 0.05)] {
                            for t in [0.03, 0.5, 0.97] {
                                for k in 1..=15 {
                                    let f = 10f64.powi(-k);
                                    let far = v0 * (1.0 + t * dv);
                                    for (uv, apex) in [
                                        (
                                            [
                                                [u0, v0],
                                                [u0 + du, v0 * (1.0 + dv)],
                                                [u0 + t * du, far * (1.0 + f)],
                                            ],
                                            [false; 3],
                                        ),
                                        (
                                            [[u0, v0], [u0, v0 * (1.0 + dv)], [u0 + du * f, far]],
                                            [false; 3],
                                        ),
                                        (
                                            [[u0, 0.0], [u0, v0], [u0 + du * f, far]],
                                            [true, false, false],
                                        ),
                                    ] {
                                        let uv = uv.map(|p| [p[0], nappe * p[1]]);
                                        let (at, (u, v)) = developed_at(plane, uv, apex);
                                        let side = |a: SpadePoint<f64>, b: SpadePoint<f64>| {
                                            (a.x - b.x).hypot(a.y - b.y)
                                        };
                                        let longest = side(at[0], at[1])
                                            .max(side(at[1], at[2]))
                                            .max(side(at[2], at[0]));
                                        let twice_area = ((at[1].x - at[0].x)
                                            * (at[2].y - at[0].y)
                                            - (at[1].y - at[0].y) * (at[2].x - at[0].x))
                                            .abs();
                                        let radius = at
                                            .iter()
                                            .map(|q| q.x.hypot(q.y))
                                            .fold(0.0f64, f64::max);
                                        if twice_area / longest < radius / f64::from(1u32 << 25) {
                                            continue;
                                        }
                                        assert_eq!(
                                            strictly_inside(
                                                at,
                                                super::cdt_point(plane, u, v, false)
                                            ),
                                            Some(true),
                                            "the development at α {alpha}, nappe {nappe}: the point \
                                             ({u}, {v}) of {uv:?}"
                                        );
                                        held += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        assert!(
            held > 5000,
            "the development's table holds {held} triangles"
        );
        let plane = super::CdtPlane::Development {
            sin_alpha: 0.5f64.sin(),
            nappe: 1.0,
        };
        let needle = [
            [6.0, 0.0],
            [6.0, 50.0],
            [6.000_000_01, 50.449_999_999_999_996],
        ];
        let (at, (u, v)) = developed_at(plane, needle, [true, false, false]);
        assert_eq!(
            strictly_inside(at, super::cdt_point(plane, u, v, false)),
            Some(false),
            "the needle {needle:?}: its point ({u}, {v}) is held strictly inside now; the \
             row's gap has closed, so widen the table and close its issue"
        );
    }

    /// **In the cone's development a node's box is its sector**
    /// ([`super::Side::meets`]): the side is the straight segment between
    /// its ends' developed points, and the box the image of the node's
    /// chart rectangle. Held to the side sampled along its length and read
    /// back to the chart, over a lattice about it: a node whose rectangle
    /// holds a sample is not laid, and one not laid has a sample in its
    /// rectangle grown by a thousandth: the samples are `1.2·10⁻⁵` apart
    /// along the side, so a rectangle the side only clips at a corner can
    /// hold none of them, by under `6·10⁻⁶` against a half-height of
    /// `7.5·10⁻³`. The chart's own chord between the
    /// same ends is a different line (the developed side bows toward the
    /// apex), and a node between the two is read against the developed one.
    #[test]
    fn in_the_development_a_nodes_box_is_its_sector() {
        let (sin_alpha, nappe) = (0.5f64.sin(), 1.0);
        let plane = super::CdtPlane::Development { sin_alpha, nappe };
        let (a, b) = ((0.2, 1.0, 0u32), (2.6, 1.4, 1u32));
        let side = super::Side::of(plane, a, b, &HashSet::new());
        let (pa, pb) = (
            super::cdt_point(plane, a.0, a.1, false),
            super::cdt_point(plane, b.0, b.1, false),
        );
        const SAMPLES: usize = 200_000;
        #[allow(clippy::cast_precision_loss)]
        let samples: Vec<(f64, f64)> = (0..=SAMPLES)
            .map(|k| k as f64 / SAMPLES as f64)
            .map(|t| (pa.x + t * (pb.x - pa.x), pa.y + t * (pb.y - pa.y)))
            .map(|(x, y)| (x.atan2(y) / sin_alpha, x.hypot(y)))
            .collect();
        // A quarter of a step, whatever the lane ships: the row is about
        // which plane the box is read in.
        let c = 0.25;
        let step = (0.11, 0.03);
        let holds = |n: &Node, grow: f64| {
            let half = (c * step.0 * grow, c * step.1 * grow);
            samples
                .iter()
                .any(|&(u, v)| (u - n.u).abs() <= half.0 && (v - n.v).abs() <= half.1)
        };
        let (mut met, mut bowed) = (0, 0);
        for j in 0..40 {
            for i in 0..30 {
                let n = Node {
                    u: 0.0 + f64::from(i) * step.0,
                    v: 0.8 + f64::from(j) * step.1 * 0.7,
                    step,
                    on_crease: false,
                };
                let meets = side.meets(&n, c);
                assert!(
                    !holds(&n, 1.0) || meets,
                    "the node ({}, {}) holds a point of the developed side and is laid",
                    n.u,
                    n.v
                );
                assert!(
                    !meets || holds(&n, 1.001),
                    "the node ({}, {}) is not laid and holds no point of the developed side",
                    n.u,
                    n.v
                );
                met += usize::from(meets);
                let chart = clipped((n.u, n.v), (c * step.0, c * step.1), (a.0, a.1), (b.0, b.1));
                bowed += usize::from(chart != meets);
            }
        }
        assert!(
            met > 10,
            "the side passes through the lattice ({met} nodes)"
        );
        assert!(
            bowed > 0,
            "some node tells the developed side from the chart's chord, so the row holds which \
             one is read"
        );
    }

    /// The `y`-poled unit ball united with the box over `x ∈ (0.5, 3)`,
    /// `z ∈ (0, 2)`: at `δ = 0.04` the section circle meets the equator
    /// at azimuth `π/3`, a lattice column of the sphere face, and a node
    /// of an unconditioned lattice lies `3·10⁻¹⁶` from the loop's vertex
    /// there
    /// (`work/tesslane/a-lattice-point-within-rounding-of-a-trim-chord-point-cuts-a-triangle-of-no-area.md`).
    fn ball_under_a_box_face() -> topo::Body<f64> {
        use geom_core::{Tol, Vec3};
        let tol = Tol::witness();
        let finished = sweep::test_support::finished;
        let ball = sweep::test_support::ball_poled_y(1.0, Vec3::new(0.0, 0.0, 0.0), tol);
        let brick = sweep::test_support::brick::<f64>((0.5, 3.0), (-2.0, 2.0), (0.0, 2.0), tol);
        topo::boolean::union(
            &finished("the box", brick, tol),
            &finished("the ball", ball, tol),
            tol,
        )
        .expect("the pair unions")
        .body()
        .expect("a body")
        .body
        .clone()
        .into_body()
    }

    /// What the lane answers on each face of `body` it takes, at `delta`
    /// and the clearance `clearance`: the chord pass and the face's domain
    /// as `tessellate` hands them over, the lane called as it calls it.
    fn lane_answers(
        body: &topo::Body<f64>,
        delta: f64,
        clearance: f64,
    ) -> Vec<(FaceKey, Result<usize, TessellateError>)> {
        use std::collections::HashMap;
        let witness = geom_core::Tol::witness();
        let mut positions = Vec::new();
        let mut vids = HashMap::new();
        for (vk, p) in body.vertex_points() {
            #[allow(clippy::cast_possible_truncation)]
            vids.insert(vk, positions.len() as u32);
            positions.push(p);
        }
        let mut bounds = crate::nurbs_cert::FaceBounds::new();
        let routes = crate::tessellate::Routes::witness(body);
        let chords: ChordPass =
            crate::chords::compute_chords(body, delta, &routes, &vids, &mut positions, &mut bounds)
                .unwrap();
        let tol = crate::sizing::SizingTols {
            delta,
            eps: crate::sizing::Eps::at(witness),
            band: geom_core::Band::linear(witness).unwrap(),
        };
        body.faces()
            .filter_map(|(fk, _)| {
                let surface = body.face_surface(fk).unwrap();
                (routes.lane(fk).unwrap()
                    == crate::tessellate::Routed::To(crate::tessellate::Lane::Trimmed))
                .then(|| {
                    let lift = super::face_domain(body, fk, tol.band).unwrap();
                    let answer = super::tessellate_trimmed(
                        body, fk, surface, &chords, &positions, &tol, &bounds, &lift, clearance,
                        false,
                    );
                    (fk, answer.map(|patch| patch.triangles.len()))
                })
            })
            .collect()
    }

    /// **The clearance is what keeps a node off the boundary, and the
    /// lane refuses a triangle f64 cannot wind as soon as it holds one.**
    /// The ball under a box face at `δ = 0.04`: at the shipped clearance
    /// the lane meshes its sphere face. Handed a clearance of zero, a
    /// node is kept off a side only where it lies on it exactly, the node
    /// within rounding of the loop's vertex is laid, and the triangle
    /// between the two has a height far inside its allowance: the lane
    /// answers `UnresolvedWinding` itself, naming that height, and does
    /// not read the noise as a fold to cure or as outward.
    #[test]
    fn a_lattice_laid_without_clearance_cuts_a_triangle_the_lane_refuses() {
        let body = ball_under_a_box_face();
        let shipped = lane_answers(&body, 0.04, super::BOUNDARY_CLEARANCE);
        assert!(
            !shipped.is_empty() && shipped.iter().all(|(_, answer)| answer.is_ok()),
            "at the shipped clearance every face the lane takes meshes: {shipped:?}"
        );
        let bare = lane_answers(&body, 0.04, 0.0);
        let unresolved: Vec<_> = bare
            .iter()
            .filter_map(|(_, answer)| match answer {
                Err(TessellateError::UnresolvedWinding { unresolved, .. }) => Some(**unresolved),
                _ => None,
            })
            .collect();
        assert!(
            !unresolved.is_empty()
                && unresolved
                    .iter()
                    .all(|u| u.height < 0.5 * u.allowance && u.allowance > 0.0),
            "clearance zero: the sphere face refuses on a triangle under half its \
             allowance high, got {bare:?}"
        );
    }

    /// Twelve distinct edge keys (a brick's), for rows that check which
    /// edge a refusal names.
    pub(super) fn twelve_edge_keys() -> Vec<topo::EdgeKey> {
        let brick: topo::Body<f64> = sweep::test_support::brick(
            (0.0, 1.0),
            (0.0, 1.0),
            (0.0, 1.0),
            geom_core::Tol::witness(),
        );
        brick.edges().map(|(k, _)| k).collect()
    }

    /// **`singular_row_shape` refuses each shape it names, at the edge
    /// that shows it.** Unit rows on the trim polygon: no body of the
    /// suite reaches these (the kernel's balls meet their poles at
    /// loop vertices), so the polygons are written out.
    #[test]
    fn the_sphere_shapes_refuse_by_cause_at_their_edge() {
        use crate::types::SphereTrimCause;
        let half_pi = core::f64::consts::FRAC_PI_2;
        let fk = FaceKey::default();
        let keys = twelve_edge_keys();
        #[allow(clippy::cast_possible_truncation)]
        let e = |i: u64| keys[i as usize];
        let cause = |poly: &[(f64, f64, u32)]| {
            let edges: Vec<_> = (0..poly.len() as u64).map(e).collect();
            let poles = [
                SingularPoint {
                    point: geom_core::Point3::new(0.0, 0.0, 1.0),
                    v: half_pi,
                    end: crate::chart_table::RowEnd::Top,
                },
                SingularPoint {
                    point: geom_core::Point3::new(0.0, 0.0, -1.0),
                    v: -half_pi,
                    end: crate::chart_table::RowEnd::Bottom,
                },
            ];
            let trim = super::TrimPolygon::new((poly.to_vec(), edges));
            match super::singular_row_shape(fk, &poles, &HashSet::new(), &trim, false) {
                Err(TessellateError::SphereTrimShape { edge, cause, .. }) => Some((edge, cause)),
                Ok(()) => None,
                Err(other) => panic!("not a shape refusal: {other:?}"),
            }
        };
        let square = [(0.0, 0.0, 1), (1.0, 0.0, 2), (1.0, 1.0, 3), (0.0, 1.0, 4)];
        assert_eq!(cause(&square), None, "a plain quad is the arm's");
        let past = [
            (0.0, 0.0, 1),
            (1.0, 0.0, 2),
            (1.0, half_pi + 1e-6, 3),
            (0.0, 1.0, 4),
        ];
        assert_eq!(cause(&past), Some((e(2), SphereTrimCause::PastPole)));
        let seam = [
            (0.0, 0.0, 1),
            (1.0, 0.0, 2),
            (1.0, 1.0, 3),
            (0.5, 0.5, 1),
            (0.0, 1.0, 4),
        ];
        assert_eq!(cause(&seam), Some((e(0), SphereTrimCause::SeamTwice)));
    }

    /// **`cdt_point` floors before it scales, and agrees with
    /// `ids_at_two_uvs` about which points are one.** At a metric above
    /// one (`v` grows) and below one (`u` grows): a sub-floor coordinate
    /// of either sign enters as `+0.0` on either axis, a legal value on
    /// the growing axis is multiplied exactly, the other axis passes
    /// bit-identical, and two chart points are equal after `cdt_point`
    /// exactly when [`crate::cdt::ids_at_two_uvs`] reads them as one
    /// location.
    #[test]
    fn cdt_point_floors_then_scales_the_growing_axis() {
        use core::f64::consts::TAU;
        use geom::Surface;
        use geom_core::{Band, Point3, Tol, Vec3};
        use spade::MIN_ALLOWED_VALUE as FLOOR;
        let sphere = Surface::Sphere {
            frame: geom::Frame::new(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            radius: 1.0,
        };
        let row = crate::chart_table::ChartRow::Analytic(
            crate::chart_table::AnalyticRow::new(&sphere, 1.0).unwrap(),
        );
        let tol = crate::sizing::SizingTols {
            delta: 1e-2,
            eps: crate::sizing::Eps::at(Tol::witness()),
            band: Band::linear(Tol::witness()).unwrap(),
        };
        let dust = 4.838_220_930_617_058e-49;
        for k in [4.0, 0.25] {
            let face = super::TrimmedFace {
                fk: FaceKey::default(),
                sense: true,
                orientation: Some(PolygonOrientation::of_face(true)),
                metric: k,
                surface: &sphere,
                row: &row,
                singular: &HashSet::new(),
                polygon: &super::TrimPolygon::of_loops(&[], &[]),
                creases: None,
                shared: &[],
                tol: &tol,
                across: &std::collections::HashMap::new(),
                runs: &[],
                clearance: super::BOUNDARY_CLEARANCE,
            };
            let bits = |p: SpadePoint<f64>| (p.x.to_bits(), p.y.to_bits());
            for c in [dust, -dust] {
                assert_eq!(
                    bits(face.cdt_point(c, 0.5)).0,
                    0.0f64.to_bits(),
                    "k = {k}: u = {c:e} enters as +0.0"
                );
                assert_eq!(
                    bits(face.cdt_point(0.5, c)).1,
                    0.0f64.to_bits(),
                    "k = {k}: v = {c:e} enters as +0.0"
                );
            }
            let p = face.cdt_point(FLOOR, FLOOR);
            let (grown, kept) = if k > 1.0 { (p.y, p.x) } else { (p.x, p.y) };
            assert_eq!(
                grown.to_bits(),
                (FLOOR * k.max(1.0 / k)).to_bits(),
                "k = {k}: the growing axis is multiplied exactly"
            );
            assert_eq!(
                kept.to_bits(),
                FLOOR.to_bits(),
                "k = {k}: the other axis passes bit-identical"
            );
            let pairs = [
                ((dust, 0.5), (0.0, 0.5)),
                ((0.5, -dust), (0.5, dust)),
                ((-0.0, 0.5), (0.0, 0.5)),
                ((FLOOR, 0.5), (0.0, 0.5)),
                ((0.5, FLOOR), (0.5, 0.0)),
                ((TAU, 0.5), (TAU.next_up(), 0.5)),
                ((0.3, 0.2), (0.3, 0.2)),
            ];
            for ((au, av), (bu, bv)) in pairs {
                let one_vertex = face.cdt_point(au, av) == face.cdt_point(bu, bv);
                let one_location =
                    crate::cdt::ids_at_two_uvs([(au, av, 7), (bu, bv, 7)]).is_empty();
                assert_eq!(
                    one_vertex, one_location,
                    "k = {k}: ({au:e}, {av:e}) and ({bu:e}, {bv:e}) — the CDT and \
                     ids_at_two_uvs must agree about whether they are one point"
                );
            }
        }
    }

    /// The cone row's development (`developed`).
    mod development_tests {
        use std::collections::HashSet;

        use geom::Surface;
        use geom_brep::PolygonOrientation;
        use geom_core::predicate::Band;
        use geom_core::{Point3, Tol, Vec3};
        use topo::FaceKey;

        use super::super::{CdtPlane, TrimmedFace, cdt_point, developed, significant};
        use crate::chart_table::{ChartRow, RowEnd, SingularPoint};
        use crate::types::{SphereTrimCause, TessellateError};

        fn cone(half_angle: f64) -> Surface<f64> {
            Surface::Cone {
                frame: geom::Frame::new(
                    Point3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(1.0, 0.0, 0.0),
                ),
                half_angle,
            }
        }

        /// **Which points are one CDT vertex in the development**
        /// (`TrimmedFace::insert_points`, condition 1 of the cone's map): on a
        /// face whose loop keeps the apex at two azimuths,
        ///
        /// - the two apex copies are one CDT point, and the face's plane is the
        ///   development;
        /// - every point of one chart column is an exact multiple of one
        ///   direction (each product is exact, `fma` reads no remainder), so
        ///   any two of them and the apex are collinear to spade's exact
        ///   predicate, and no triangle lies along a ruling;
        /// - a polygon point off the apex that would land on the apex's CDT
        ///   point (its development under spade's floor) is found before the
        ///   first attempt (`developed_runs`), where the face refuses typed
        ///   rather than merge two mesh vertices the chart keeps apart;
        /// - a lattice node there is not laid (`lay`: the apex is a point of
        ///   the polygon, and the node's sector holds it), and a point of the
        ///   mesher's entering at an existing vertex refuses: nothing is
        ///   merged.
        ///
        /// **The exactness cut is pinned here alone.** `developed` returning
        /// the plain `r·sin θ, r·cos θ` reds this row and no mesh row: the
        /// rounding it would leave makes three points of a ruling a sliver
        /// of about `1e-16` only where the triangulation is forced to join
        /// them (a ruling side chorded into several chords, a grid point on
        /// a ruling side), and no fixture here makes one.
        #[test]
        fn the_development_keeps_apex_copies_one_and_mesh_vertices_apart() {
            let surface = cone(0.3);
            let row =
                ChartRow::Analytic(crate::chart_table::AnalyticRow::new(&surface, 1.0).unwrap());
            let tol = crate::sizing::SizingTols {
                delta: 1e-2,
                eps: crate::sizing::Eps::at(Tol::witness()),
                band: Band::linear(Tol::witness()).unwrap(),
            };
            let singular: HashSet<u32> = [1].into_iter().collect();
            let across = std::collections::HashMap::new();
            let face = |polygon| TrimmedFace {
                fk: FaceKey::default(),
                sense: true,
                orientation: Some(PolygonOrientation::of_face(true)),
                metric: 1.0,
                surface: &surface,
                row: &row,
                singular: &singular,
                polygon,
                creases: None,
                shared: &[],
                tol: &tol,
                across: &across,
                runs: &[],
                clearance: super::super::BOUNDARY_CLEARANCE,
            };
            let wedge = super::super::TrimPolygon::of_loops(
                &[(0.2, 0.0, 1), (0.9, 0.0, 1), (0.9, 1.0, 3), (0.2, 1.0, 2)],
                &[],
            );
            let wedge = face(&wedge);
            let plane = wedge.plane();
            assert!(
                matches!(plane, CdtPlane::Development { nappe, .. } if nappe == 1.0),
                "a cone face on the v > 0 nappe triangulates in its development: {plane:?}"
            );
            assert_eq!(
                cdt_point(plane, 0.2, 0.0, true),
                cdt_point(plane, 0.9, 0.0, true),
                "the apex copies are one CDT point"
            );
            let s = 0.3f64.sin();
            for u in [0.2, 0.55, 0.9] {
                let (sin, cos) = <f64 as geom_core::Real>::sin_cos(s * u);
                let (dx, dy) = (significant(sin, 26), significant(cos, 26));
                for v in [1e-3, 0.37, 1.0, 9.1e2] {
                    let r = significant(v, 27);
                    let (x, y) = developed(s, 1.0, u, v);
                    assert!(
                        r.mul_add(dx, -x) == 0.0 && r.mul_add(dy, -y) == 0.0,
                        "u = {u}, v = {v}: the developed point is an exact multiple of its column's \
                     direction"
                    );
                }
            }
            let step = (0.1, 0.1);
            let nodes = [(0.5, 0.0), (0.5, 0.5)].map(|(u, v)| super::super::Node {
                u,
                v,
                step,
                on_crease: false,
            });
            assert_eq!(
                wedge.lay(&nodes),
                [(0.5, 0.5)],
                "a node on the apex row is not laid: its sector holds the apex"
            );
            let at = wedge
                .insert_points(&[(0.5, 0.5)], &HashSet::new())
                .expect("the wedge's points enter the CDT");
            assert_eq!(
                at.meta.len(),
                3 + 1,
                "the apex and the two polygon points off it, and the one node off the apex"
            );
            assert!(
                matches!(
                    wedge.insert_points(&[(0.5, 0.0), (0.5, 0.5)], &HashSet::new()),
                    Err(TessellateError::Triangulation { .. })
                ),
                "a point of the mesher's at the apex's CDT point refuses: it is not merged"
            );
            let crushed = super::super::TrimPolygon::of_loops(
                &[
                    (0.2, 0.0, 1),
                    (0.5, 1e-60, 4),
                    (0.9, 0.0, 1),
                    (0.9, 1.0, 3),
                    (0.2, 1.0, 2),
                ],
                &[],
            );
            let crushed = face(&crushed);
            let no_line = |_: usize| false;
            assert_eq!(
                super::super::developed_runs(crushed.plane(), crushed.polygon, &singular, &no_line),
                Err(1),
                "a polygon point off the apex whose development lands on the apex's CDT point \
                 merges two mesh vertices; the face refuses there"
            );
            assert_eq!(
                super::super::developed_runs(wedge.plane(), wedge.polygon, &singular, &no_line),
                Ok(Vec::new()),
                "the apex copies alone share a point"
            );
        }

        /// **A run is read from the polygon's own order**
        /// ([`super::super::developed_runs`]). On a cone of half-angle 0.3,
        /// `(u, 1)` and `(u, 1 + 2⁻⁴⁰)` develop to one point (the slant
        /// radius is cut to 27 bits). Polygon points there are a run when
        /// they are consecutive on one loop along sides a line carries, with
        /// a neighbour reached along one too; the run is anchored by the
        /// first such neighbour. Every other merge refuses at its second
        /// point: a point merged with the apex, points of two sides or of
        /// two loops, a side wholly within the cut, points out of order
        /// along the ruling, one mesh vertex twice, sides no line carries,
        /// and a run whose two neighbours are one CDT point.
        #[test]
        fn a_run_is_read_from_the_polygons_order() {
            use super::super::{Anchor, Run, TrimPolygon, developed_runs};
            let plane = CdtPlane::Development {
                sin_alpha: 0.3f64.sin(),
                nappe: 1.0,
            };
            let e = (0.5f64).powi(40);
            assert_eq!(
                cdt_point(plane, 0.2, 1.0, false),
                cdt_point(plane, 0.2, 1.0 + 2.0 * e, false),
                "the two radii are one after the cut"
            );
            let apex: HashSet<u32> = [9].into_iter().collect();
            let runs =
                |outer: &[(f64, f64, u32)], rings: &[&[(f64, f64, u32)]], lines: &[usize]| {
                    developed_runs(plane, &TrimPolygon::of_loops(outer, rings), &apex, &|i| {
                        lines.contains(&i)
                    })
                };
            let run = |members: &[usize], anchor| Run {
                members: members.to_vec(),
                anchor,
            };
            // A wall's quad, its ruling at u = 0.2 walked upward.
            let mid = [
                (0.2, 0.5, 1),
                (0.2, 1.0, 2),
                (0.2, 1.0 + e, 3),
                (0.2, 1.5, 4),
                (0.9, 1.5, 5),
                (0.9, 0.5, 6),
            ];
            assert_eq!(
                runs(&mid, &[], &[0, 1, 2]),
                Ok(vec![run(&[1, 2], Anchor::Before)]),
                "two points inside a ruling side: anchored by the first neighbour"
            );
            let three = [
                (0.2, 0.5, 1),
                (0.2, 1.0, 2),
                (0.2, 1.0 + e, 3),
                (0.2, 1.0 + 2.0 * e, 7),
                (0.2, 1.5, 4),
                (0.9, 1.5, 5),
                (0.9, 0.5, 6),
            ];
            assert_eq!(
                runs(&three, &[], &[0, 1, 2, 3]),
                Ok(vec![run(&[1, 2, 3], Anchor::Before)]),
                "a run of three"
            );
            let wrapped = [
                (0.2, 1.0 + e, 3),
                (0.2, 1.5, 4),
                (0.9, 1.5, 5),
                (0.9, 0.5, 6),
                (0.2, 0.5, 1),
                (0.2, 1.0, 2),
            ];
            assert_eq!(
                runs(&wrapped, &[], &[4, 5, 0]),
                Ok(vec![run(&[5, 0], Anchor::Before)]),
                "a run across the polygon's first point, in walk order"
            );
            // The ruling's lower end is the corner the rim arrives at.
            let corner_first = [
                (0.9, 1.0, 6),
                (0.2, 1.0, 2),
                (0.2, 1.0 + e, 3),
                (0.2, 1.5, 4),
                (0.9, 1.5, 5),
            ];
            assert_eq!(
                runs(&corner_first, &[], &[1, 2]),
                Ok(vec![run(&[1, 2], Anchor::After)]),
                "a run starting at a corner: anchored by the neighbour after it"
            );
            let corner_last = [
                (0.2, 0.5, 1),
                (0.2, 1.0, 2),
                (0.2, 1.0 + e, 3),
                (0.9, 1.0, 5),
                (0.9, 0.5, 6),
            ];
            assert_eq!(
                runs(&corner_last, &[], &[0, 1]),
                Ok(vec![run(&[1, 2], Anchor::Before)]),
                "a run ending at a corner"
            );
            let from_the_apex = [
                (0.2, 0.0, 9),
                (0.2, 1.0, 2),
                (0.2, 1.0 + e, 3),
                (0.9, 1.0, 5),
            ];
            assert_eq!(
                runs(&from_the_apex, &[], &[0, 1]),
                Ok(vec![run(&[1, 2], Anchor::Before)]),
                "the apex is a neighbour like another: the side from it is a ruling"
            );

            let with_the_apex = [(0.5, 0.0, 9), (0.2, 1e-60, 2), (0.2, 1.0, 3), (0.9, 1.0, 5)];
            assert_eq!(
                runs(&with_the_apex, &[], &[0, 1]),
                Err(1),
                "a point merged with the apex is no run, though consecutive and on a ruling"
            );
            let two_sides = [
                (0.2, 1.0, 1),
                (0.9, 1.0, 2),
                (0.2, 1.0 + e, 3),
                (0.9, 2.0, 4),
                (0.5, 2.0, 5),
            ];
            assert_eq!(
                runs(&two_sides, &[], &[0, 1, 2, 3, 4]),
                Err(2),
                "two points that are not consecutive: two sides"
            );
            let ring = [(0.2, 1.0 + e, 7), (0.4, 1.2, 8), (0.3, 1.3, 10)];
            assert_eq!(
                runs(&mid[..2], &[&ring], &[0, 1, 2, 3, 4]).err(),
                Some(2),
                "points of two loops"
            );
            let whole_side = [
                (0.9, 1.0, 6),
                (0.2, 1.0, 2),
                (0.2, 1.0 + e, 3),
                (0.9, 1.5, 5),
            ];
            assert_eq!(
                runs(&whole_side, &[], &[1]),
                Err(2),
                "a ruling side wholly within the cut: no neighbour is on the ruling"
            );
            let out_of_order = [
                (0.2, 0.5, 1),
                (0.2, 1.0, 2),
                (0.2, 1.0 + 2.0 * e, 7),
                (0.2, 1.0 + e, 3),
                (0.2, 1.5, 4),
                (0.9, 1.5, 5),
                (0.9, 0.5, 6),
            ];
            assert_eq!(
                runs(&out_of_order, &[], &[0, 1, 2, 3]),
                Err(2),
                "points out of order along the ruling"
            );
            let mut twice = mid;
            twice[2].2 = 2;
            assert_eq!(
                runs(&twice, &[], &[0, 1, 2]),
                Err(2),
                "one mesh vertex at two points"
            );
            assert_eq!(
                runs(&mid, &[], &[0, 2]),
                Err(2),
                "a side between the two that no line carries"
            );
            let closed = [(0.2, 1.0, 2), (0.2, 1.0 + e, 3), (0.9, 1.0, 5)];
            assert_eq!(
                runs(&closed[..2], &[], &[0, 1]).err(),
                Some(1),
                "the whole loop at one point"
            );
            let pinched = [
                (0.9, 1.0, 5),
                (0.2, 1.0, 2),
                (0.2, 1.0 + e, 3),
                (0.9, 1.0, 6),
                (0.5, 2.0, 7),
            ];
            assert_eq!(
                runs(&pinched, &[], &[0, 1, 2]).err(),
                Some(2),
                "a run whose two neighbours are one point"
            );
        }

        /// **A CDT triangle at a run's vertex is cut by the run's anchor**
        /// ([`super::super::cut_along_runs`]). Two runs of one ruling side
        /// in a row, `R` then `T`, each anchored by the point before it, at
        /// CDT vertices 1 and 2 beside their neighbour at 0:
        ///
        /// - the triangle on `R`'s anchored side stands on `R`'s first
        ///   point, and the triangle added is `R`'s two points and the third
        ///   corner, in the slots of the neighbour and the run;
        /// - any other triangle at `R`'s vertex stands on its last point and
        ///   adds nothing;
        /// - the triangle of the neighbour, `R` and `T` (three vertices of
        ///   one ruling, which the CDT makes of no three points of one
        ///   column) has `T` anchored by `R`'s own vertex for `R`'s third
        ///   corner, and refuses at `R`'s first point.
        #[test]
        fn a_triangle_at_a_runs_vertex_is_cut_by_its_anchor() {
            use std::collections::HashMap;

            use spade::handles::FixedVertexHandle;

            use super::super::{Corner, Run, TrimPolygon, cut_along_runs, developed_runs};
            use crate::tessellate::PatchVertex;

            let plane = CdtPlane::Development {
                sin_alpha: 0.3f64.sin(),
                nappe: 1.0,
            };
            let e = (0.5f64).powi(40);
            let polygon = TrimPolygon::of_loops(
                &[
                    (0.2, 0.5, 1),
                    (0.2, 1.0, 2),
                    (0.2, 1.0 + e, 3),
                    (0.2, 1.25, 4),
                    (0.2, 1.25 + e, 5),
                    (0.2, 1.5, 6),
                    (0.9, 1.5, 7),
                    (0.9, 0.5, 8),
                ],
                &[],
            );
            let runs = developed_runs(plane, &polygon, &HashSet::new(), &|i| i < 5).unwrap();
            let [r, t] = runs.as_slice() else {
                panic!("two runs, got {runs:?}");
            };
            assert_eq!((&r.members[..], &t.members[..]), (&[1, 2][..], &[3, 4][..]));
            let handles: Vec<FixedVertexHandle> = [0, 1, 1, 2, 2, 3, 4, 5]
                .map(FixedVertexHandle::from_index)
                .to_vec();
            let run_at: HashMap<usize, &Run> = [(1, r), (2, t)].into_iter().collect();
            let cut = |hs: [usize; 3]| {
                let mut corners = hs.map(|h| Corner {
                    #[allow(clippy::cast_possible_truncation)]
                    id: PatchVertex::Shared(100 + h as u32),
                    uv: [0.0; 2],
                    grid: None,
                    places: None,
                });
                let ids = |t: [Corner<'_>; 3]| t.map(|c| c.id);
                cut_along_runs(&polygon, &handles, &run_at, hs, &mut corners).map(|between| {
                    (
                        ids(corners),
                        between.into_iter().map(ids).collect::<Vec<_>>(),
                    )
                })
            };
            let s = PatchVertex::Shared;
            assert_eq!(
                cut([0, 1, 5]),
                Ok(([s(100), s(2), s(105)], vec![[s(2), s(3), s(105)]])),
                "on the anchored side: the run's first point, and one triangle added"
            );
            assert_eq!(
                cut([1, 5, 4]),
                Ok(([s(3), s(105), s(104)], vec![])),
                "any other triangle: the run's last point, nothing added"
            );
            assert_eq!(
                cut([0, 1, 2]),
                Err(1),
                "the neighbour and two runs of its ruling in one triangle"
            );
        }

        /// **A split seam's close points are a run on each of its two rays.**
        /// A wall whose outer face is a draft cone, one face a full turn, so
        /// its loop walks the seam ruling twice, once on each bounding ray
        /// of its development. The seam split three times a gap apart at its
        /// middle: the three vertices are one run of three on each ray, the
        /// same mesh vertices in opposite orders, each inside a ruling side.
        /// Split `10⁻⁵` of its length from its start: the split vertex and
        /// the seam's end are a run of two on each ray, at a corner of the
        /// polygon, so one is anchored by the neighbour before it and the
        /// other by the one after.
        ///
        /// **By ε.** A cone of 0.1° and a gap of `10⁻⁶` at the default ε
        /// and `10⁻¹²`. At `10⁻⁶` a split is a split from `10⁻⁵`, three of
        /// which that cone does not merge: the cone is 0.01° and the gap
        /// `10⁻⁵`.
        #[test]
        fn a_split_seams_close_points_are_a_run_on_each_ray() {
            use std::collections::HashMap;

            use geom_core::{Point2, Vec2};
            use profile::{Profile, ProfileLoop, RawLoop as _, SketchPlane};
            use sweep::{Revolution, RevolveAxis, revolve};

            use topo::construction::Construct as _;

            use super::super::{Anchor, carried_by_a_line, developed_runs};

            let tol = Tol::witness();
            let band = Band::linear(tol).unwrap();
            let (alpha, gap) = if tol.eps() > geom_core::tolerance::DEFAULT_EPS {
                (0.01f64, 1e-5)
            } else {
                (0.1f64, 1e-6)
            };
            let t = alpha.to_radians().tan();
            let lp = ProfileLoop::polygon(
                [(1.0, 0.0), (2.0 + t, 0.0), (2.0, 1.0), (1.0, 1.0)]
                    .map(|(x, y)| Point2::new(x, y)),
            );
            let profile = Profile::new(SketchPlane::xy(), vec![lp])
                .validate(tol)
                .unwrap();
            let axis = RevolveAxis {
                origin: Point2::new(0.0, 0.0),
                dir: Vec2::new(0.0, 1.0),
            };
            let wall = revolve(&profile, axis, Revolution::Full, tol)
                .unwrap()
                .body
                .into_body();
            let ends = |body: &topo::Body<f64>, ek| {
                let c = body
                    .get_curve_geom(body.get_edge(ek).unwrap().curve)
                    .unwrap()
                    .certified()
                    .unwrap();
                let (t0, t1) = c.params();
                (c.carrier().eval(t0), c.carrier().eval(t1), (t0, t1))
            };
            // The cone face's seam: the line edge whose ends differ in radius.
            let seam = wall
                .edges()
                .map(|(ek, _)| ek)
                .find(|&ek| {
                    let (a, b, _) = ends(&wall, ek);
                    carried_by_a_line(&wall, ek) && (a.x.hypot(a.z) - b.x.hypot(b.z)).abs() > 1e-6
                })
                .expect("the cone face's seam");
            let runs_of = |fracs: &[f64]| {
                let mut body = wall.clone();
                let (_, _, (t0, t1)) = ends(&body, seam);
                for &f in fracs {
                    body.split_edge(seam, t0 + (t1 - t0) * f, tol).unwrap();
                }
                let mut positions = Vec::new();
                let mut vids = HashMap::new();
                for (vk, p) in body.vertex_points() {
                    #[allow(clippy::cast_possible_truncation)]
                    vids.insert(vk, positions.len() as u32);
                    positions.push(p);
                }
                let mut bounds = crate::nurbs_cert::FaceBounds::new();
                let routes = crate::tessellate::Routes::witness(&body);
                let chords = crate::chords::compute_chords(
                    &body,
                    1e-2,
                    &routes,
                    &vids,
                    &mut positions,
                    &mut bounds,
                )
                .unwrap();
                let (fk, surface) = body
                    .faces()
                    .map(|(fk, _)| (fk, body.face_surface(fk).unwrap()))
                    .find(|(_, surface)| matches!(surface, Surface::Cone { .. }))
                    .expect("the cone face");
                let row = ChartRow::of(&body, fk, surface, &bounds).unwrap();
                let ChartRow::Analytic(ref analytic) = row else {
                    panic!("a cone face has an analytic row");
                };
                let lift = body.face_lift(fk, band).unwrap();
                let singular =
                    super::super::singular_vertices(&body, fk, &lift, &chords, &positions, |p| {
                        analytic.is_singular(p, band)
                    })
                    .unwrap();
                let polygon = super::super::trim_polygon(
                    &body,
                    fk,
                    &lift,
                    &chords,
                    false,
                    None,
                    (&singular, &super::super::SidePoints::default()),
                )
                .unwrap();
                let plane = CdtPlane::of(&row, 1.0, &polygon, &singular);
                let runs = developed_runs(plane, &polygon, &singular, &|i| {
                    carried_by_a_line(&body, polygon.edges[i])
                })
                .expect("every merge of the face is a run");
                runs.into_iter()
                    .map(|run| {
                        let ids: Vec<u32> =
                            run.members.iter().map(|&i| polygon.points[i].2).collect();
                        (ids, run.anchor)
                    })
                    .collect::<Vec<_>>()
            };
            let mid = runs_of(&[0.5 + 2.0 * gap, 0.5 + gap, 0.5]);
            let [(one, Anchor::Before), (other, Anchor::Before)] = mid.as_slice() else {
                panic!("three close splits: a run inside a ruling side on each ray, got {mid:?}");
            };
            assert_eq!(one.len(), 3, "a run of three");
            assert_eq!(
                one.iter().rev().collect::<Vec<_>>(),
                other.iter().collect::<Vec<_>>(),
                "the two rays hold the same mesh vertices, walked opposite ways"
            );
            let end = runs_of(&[1e-5]);
            let [(one, a), (other, b)] = end.as_slice() else {
                panic!("a split beside the seam's end: a run of two on each ray, got {end:?}");
            };
            assert_eq!((one.len(), other.len()), (2, 2));
            assert_eq!(
                one.iter().rev().collect::<Vec<_>>(),
                other.iter().collect::<Vec<_>>()
            );
            assert!(
                matches!(
                    (a, b),
                    (Anchor::Before, Anchor::After) | (Anchor::After, Anchor::Before)
                ),
                "at a corner one run is reached along the ruling and the other leaves along it: \
                 {a:?}, {b:?}"
            );
        }

        /// **The development keeps the chart's orientation on either nappe, and
        /// a whole turn's seam is two rays.** A small chart square's developed
        /// signed area has the chart's sign at `σ = ±1`, and the seam ruling at
        /// `u = 0` and at `u = 2π` develops to two distinct directions at every
        /// half-angle from a needle's to a near-flat cone's (`sin α < 1`).
        #[test]
        fn the_development_keeps_orientation_and_a_whole_turns_seam_apart() {
            use core::f64::consts::TAU;
            for alpha in [0.02, 0.3, 1.0, 1.5] {
                let s = <f64 as geom_core::Real>::sin(alpha);
                for nappe in [1.0, -1.0] {
                    let chart = [(0.4, 0.5), (0.5, 0.5), (0.5, 0.6), (0.4, 0.6)]
                        .map(|(u, v): (f64, f64)| (u, nappe * v));
                    let area = |p: [(f64, f64); 4]| -> f64 {
                        (0..4)
                            .map(|i| {
                                let (a, b) = (p[i], p[(i + 1) % 4]);
                                a.0 * b.1 - b.0 * a.1
                            })
                            .sum()
                    };
                    let developed_area = area(chart.map(|(u, v)| developed(s, nappe, u, v)));
                    assert!(
                        developed_area.signum() == area(chart).signum(),
                        "α = {alpha}, σ = {nappe}: a chart square develops with its orientation, \
                     got {developed_area:e} for {:e}",
                        area(chart)
                    );
                }
                assert_ne!(
                    developed(s, 1.0, 0.0, 1.0),
                    developed(s, 1.0, TAU, 1.0),
                    "α = {alpha}: the seam's two sides develop to two rays"
                );
            }
        }

        /// **A cone face on both nappes refuses typed**: no door builds one
        /// (a profile crossing the axis refuses at validation, and the union
        /// of two cones apex to apex keeps one face a nappe), and the
        /// development is one nappe's, so `singular_row_shape` refuses it
        /// `BothNappes` before the plane is chosen.
        #[test]
        fn a_cone_face_on_both_nappes_refuses_typed() {
            let apex = [SingularPoint {
                point: Point3::new(0.0, 0.0, 0.0),
                v: 0.0,
                end: RowEnd::Inside,
            }];
            let singular: HashSet<u32> = [1].into_iter().collect();
            let edges: Vec<topo::EdgeKey> = super::twelve_edge_keys();
            let cause = |poly: &[(f64, f64, u32)]| match super::super::singular_row_shape(
                FaceKey::default(),
                &apex,
                &singular,
                &super::super::TrimPolygon::new((poly.to_vec(), edges[..poly.len()].to_vec())),
                true,
            ) {
                Err(TessellateError::SphereTrimShape { cause, .. }) => Some(cause),
                Ok(()) => None,
                Err(other) => panic!("not a shape refusal: {other:?}"),
            };
            let one = [(0.2, 0.0, 1), (0.9, 0.0, 1), (0.9, 1.0, 3), (0.2, 1.0, 2)];
            assert_eq!(cause(&one), None, "a wedge on one nappe is the arm's");
            let below = [(0.2, 0.0, 1), (0.9, 0.0, 1), (0.9, -1.0, 3), (0.2, -1.0, 2)];
            assert_eq!(cause(&below), None, "and so is one on the other");
            let both = [(0.2, 1.0, 2), (0.5, 0.0, 1), (0.2, -1.0, 3), (0.1, 0.0, 4)];
            assert_eq!(cause(&both), Some(SphereTrimCause::BothNappes));
        }
    }
}
