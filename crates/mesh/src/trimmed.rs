//! **Trimmed curved-face tessellation** (M5 PR 11, C12.6): faces whose
//! boundary carries a conic or B-spline trim carrier, or that hold a
//! ring, are not swept UV rectangles, so the iso walk cannot traverse
//! them — instead the face's **domain** provides the trim polygon in UV:
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
//! the tiltedcut walls; every split cylinder wall), **cone** (a frustum
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
//! the homogeneous nets). An analytic face routes here by its outer
//! loop's carriers, or by holding a ring (`crate::tessellate::lane_of`).
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
//! ([`TessellateError::FoldedTriangle`]), and a triangle on three
//! consecutive boundary points (an ear of a chain the neighbour may cut
//! too) is split by its centroid and rebuilt, or refuses
//! ([`TessellateError::BoundaryEar`]). A **fitted** (rung-3) chart
//! image still refuses typed on every chart: the chord pass has no
//! certified UV chord-step bound for a fitted image (the
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
//! exact predicates read any three points of one ruling as collinear,
//! and no triangle lies along one. Three points of a cone on one line
//! lie on a ruling (a line meets the quadric in two points or lies in
//! it), so no other triangle of zero area exists to make. A THIN triangle
//! is another matter, and nothing bounds it: beside a ruling a triangle's
//! `2·area/L²` goes as `(h/L)²`, and a needle cone or a face ending near
//! its apex meshes certified triangles near `1e-8`
//! (`work/tesslane/a-needle-cone-face-meshes-thin-triangles-beside-its-rulings.md`).
//! A face ending close to its apex can also refuse at fine δ, because
//! the chart's certificate reads the span the development makes short
//! (`work/tesslane/a-cone-face-ending-near-its-apex-refuses-at-fine-delta.md`).
//!
//! - **Which points are one CDT vertex.** Every copy of the apex is the
//!   origin. The development rounds, so it is not injective: a face
//!   whose polygon point would share a CDT vertex with one at another UV
//!   location refuses `SphereTrimShape { DevelopedMerge }` before its
//!   first attempt ([`developed_merge`]) rather than merge two mesh
//!   vertices, and a grid candidate that would is dropped, as a candidate
//!   on a polygon point is in the chart ([`TrimmedFace::insert_points`]).
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
//! - **What it moves.** The triangulation alone: the lattice, the
//!   creases, the polygon's orientation, the fold census, the ear test
//!   and every certificate read UV, and each kept triangle is still
//!   certified, so a different triangulation changes the triangles'
//!   shape, not what is guaranteed of them. A point the ear cure inserts
//!   is the developed centroid read back to the chart
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
//! within rounding of a crease snap onto it, the grid gains a point on
//! the crease at every grid line across it, and the points on a crease
//! are chained into constraints. A boundary chord can only meet a
//! crease at a chord point, which an arc rim's schedule places
//! (`crate::chords`); any other boundary crossing one refuses typed, as
//! do creases in both directions.
//!
//! # The grid-on-constraint retry (the T-junction attack)
//!
//! An interior grid point landing EXACTLY on a boundary constraint
//! would split it inside spade, leaving this face triangulated against
//! a sub-segment its neighbour does not share — a T-junction. The
//! constraint pass therefore classifies every intermediate vertex of
//! a realised constraint: GRID points are dropped and the pass
//! rebuilds (deterministic, ≤ [`MAX_GRID_RETRIES`] rounds, then a
//! typed [`TessellateError::Triangulation`]) — dropping a grid point
//! only coarsens triangles, and the per-triangle certificates remain
//! the guarantee. Since TESS-SPAN the NURBS lane's grid is
//! NON-UNIFORM (per-knot-span-cell lines), and the ladder is argued
//! for it explicitly: candidates are generated, DEDUPLICATED by
//! location before the first attempt (the refinement arm may APPEND
//! later rounds' candidates — indices only grow), and a drop removes
//! a candidate INDEX — so a cell-boundary point minted by two
//! adjacent cells is one candidate, one drop actually removes the
//! offending location, every retry strictly shrinks the candidate
//! set, and the ≤ [`MAX_GRID_RETRIES`] bound terminates exactly as it
//! did for the uniform grid.
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
//! inside another segment of its loop. A loop through two vertices at
//! one point is not that shape; both are walk entries at one handle,
//! which [`Pinches`] reads. The refusal has no at-rest fixture:
//! hand-building a boundary point inside another segment at the mesh
//! layer would need a full body whose certified pcurve caches describe
//! a loop the mint pass itself refuses (`LoopNotClosed`/continuity) —
//! the arm stands as the backstop's tripwire (review MIN-1).

use std::collections::{HashMap, HashSet};

use geom::Curve3;
use geom::Surface;
use geom_brep::{OutwardNormal, Pcurve, PolygonOrientation};
use geom_core::Point3;
use spade::handles::FixedVertexHandle;
use spade::{ConstrainedDelaunayTriangulation, Point2 as SpadePoint, Triangulation};
use topo::pcurves::{FaceLift, Lift};
use topo::{Body, EdgeKey, FaceKey};

use crate::cert;
use crate::chart_table::{Analytic, AnalyticRow, ChartRow, RowEnd, SingularPoint};
use crate::chords::ChordPass;
use crate::nurbs_cert::{FaceBounds, NurbsCellGrid};
use crate::planar::{Crossings, Pinches, Walk, classify_faces, edge_key, shoelace2};
use crate::sizing::{SizingTols, ceil_count};
use crate::tessellate::{Patch, PatchVertex};
use crate::types::{SphereTrimCause, TessellateError};

/// Retry budget for the rebuild loop (module docs). **Termination.**
///
/// One round budget covers all four retry kinds — the grid-on-constraint
/// drops, the certificate-driven refinement, the fold drops and the ear
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

/// **A face's deviation target**: [`UNPROVED_HEADROOM`]·δ for a face
/// this lane meshes on an analytic chart (one whose surface has no spline
/// chart), δ for every other face. The one statement those arms size
/// their lattices at and the chord pass chords the face's boundary at
/// (`chords::edge_target`).
pub(crate) fn face_target(
    body: &Body<f64>,
    fk: FaceKey,
    delta: f64,
) -> Result<f64, TessellateError> {
    let surface = crate::tessellate::face_surface(body, fk)?;
    let headroom = surface.spline_chart().is_none()
        && crate::tessellate::lane_of(body, fk, surface)? == crate::tessellate::Lane::Trimmed;
    Ok(if headroom {
        delta * UNPROVED_HEADROOM
    } else {
        delta
    })
}

/// Does this face's outer loop carry a circle tilted against the face's
/// chart — one whose stored image is the chart's inverse of the carrier
/// ([`Pcurve::Projected`]) rather than an iso line's closed form? On a
/// sphere that is every circle that is neither a parallel nor a
/// meridian, and it is the structural mark the dispatch routes on.
pub(crate) fn has_tilted_circle(body: &Body<f64>, fk: FaceKey) -> Result<bool, TessellateError> {
    let face = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;
    Ok(crate::walk::loop_half_edges(body, face.outer, fk)?
        .into_iter()
        .any(|(hek, _, _)| {
            body.pcurve(hek)
                .is_some_and(|c| matches!(c.pcurve(), Pcurve::Projected(_)))
        }))
}

/// **The trimmed lane's steps on an analytic chart** `(h_u, h_v)`: the
/// row's steps ([`AnalyticRow::steps`]) at the face's target
/// ([`face_target`]). The chord pass reads the same steps to bound each
/// boundary chord's UV step (`chords::tighten`), so the lattice and the
/// trim polygon are sized from one value.
pub(crate) fn arm_steps(
    body: &Body<f64>,
    fk: FaceKey,
    row: &AnalyticRow<'_>,
    delta: f64,
) -> Result<(f64, f64), TessellateError> {
    #[cfg(test)]
    let scale = tests::STEP_SCALE.with(std::cell::Cell::get);
    #[cfg(not(test))]
    let scale = 1.0;
    let (hu, hv) = row.steps(face_target(body, fk, delta)?);
    Ok((hu * scale, hv * scale))
}

/// Does this face's outer loop carry a non-iso trim carrier (conic or
/// B-spline)? Structural kind test — the C5 dispatch discipline, no
/// numeric fallback.
pub(crate) fn has_trim_carrier(body: &Body<f64>, fk: FaceKey) -> Result<bool, TessellateError> {
    let face = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;
    for (ek, _) in crate::walk::loop_edges(body, face.outer, fk)? {
        let curve = body
            .get_edge(ek)
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(|g| g.certified())
            .ok_or(TessellateError::MissingEntity { what: "edge curve" })?;
        if matches!(
            curve.carrier(),
            Curve3::Ellipse(..) | Curve3::Nurbs(_) | Curve3::Spiric { .. }
        ) {
            return Ok(true);
        }
    }
    Ok(false)
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
    tol: &SizingTols,
) -> Result<FaceLift<f64>, TessellateError> {
    body.face_lift(fk, tol.band)
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
) -> Result<Patch, TessellateError> {
    let face = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;
    let loops = domain_loops(lift);
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
    // The loop's vertices that are the chart's singular points, decided
    // where the kernel decides it, at the run's band (`chart_table`'s
    // module docs). Each vertex's mesh id is its edges' end chord id.
    let singular = match row {
        ChartRow::Analytic(ref a) if !a.singular_points().is_empty() => {
            singular_vertices(body, fk, &loops, chords, shared, a, tol.band)?
        }
        ChartRow::Analytic(_) | ChartRow::Spline { .. } => HashSet::new(),
    };
    // The UV polygon: per half-edge, the pcurve evaluated at the
    // shared chord parameters (each traversal contributes all but its
    // last point; ids stay the shared 3-D chord ids), every loop.
    let polygon = trim_polygon(body, fk, lift, chords, spline, creases.as_ref(), &singular)?;
    if polygon.loops().any(|l| l.len() < 3) {
        return Err(TessellateError::MissingEntity {
            what: "degenerate trimmed boundary",
        });
    }
    let orientation = trim_orientation(fk, face.sense, spline, &polygon)?;
    // A CHART WITH SINGULAR POINTS keeps its polygon to the shape
    // `singular_row_shape` admits, so the duplicate-id skip in the emit
    // pass never collapses a fan at one (#678).
    if let ChartRow::Analytic(ref a) = row
        && !a.singular_points().is_empty()
    {
        singular_row_shape(fk, a.singular_points(), &singular, &polygon)?;
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
    let (mut candidates, nurbs_stats, metric) = match row {
        ChartRow::Analytic(ref a) => {
            // The row's steps at the face's target, the steps the chord
            // pass bounded this face's boundary UV steps by (`arm_steps`),
            // and the lattice the row lays from them.
            let (hu, hv) = a.lattice_steps(arm_steps(body, fk, a, tol.delta)?);
            let nu = pole_side_columns(&polygon, &singular, (u0, u1), ceil_count(u1 - u0, hu)?)?;
            let nv = ceil_count(v1 - v0, hv)?;
            // The other half of #678's shape on the cylinder, checked
            // where the count is: the fan needs ONE interior column
            // between two boundary entries that share a mesh vertex,
            // which is `nu == 2`. The only repeated id at two DISTINCT UVs
            // on a cylinder is the full-2π seam traversed twice, and there
            // `u1 - u0` is 2π against an `hu` the sagitta cap holds at
            // ≤ π/4 — so the two never meet. Arithmetic, not an argument,
            // and this is the line that says so. At a singular point
            // `pole_side_columns` keeps the count off the shape.
            //
            // VACUOUS TODAY, and the search that says so was
            // adversarial rather than incidental: across the suite's
            // 19 trimmed-cylinder faces `nu` is 5, 16 or 50 and no
            // polygon repeats an id apart, and the two conditions are
            // arithmetically exclusive — `nu == 2` needs
            // `u1 - u0 <= π/2`, a repeat-apart needs the full `2π`
            // seam. Recorded as *searched for by a reviewer told what
            // to look for and not found*, which survives someone
            // adding a fixture in a way that *nothing in tree does
            // this* does not.
            debug_assert!(
                !matches!(a.kind(), Analytic::Cylinder { .. })
                    || nu != 2
                    || !id_repeats_apart(&polygon.points),
                "face {fk:?}: a trim polygon repeats a mesh id at two UV locations \
                 with a single interior column (nu = 2, nv = {nv}) — the shape \
                 #678's non-manifold fan needs (curved::pole_columns)"
            );
            (
                uniform_candidates((u0, u1), (v0, v1), nu, nv),
                None,
                a.metric((u0, u1), (v0, v1), (nu, nv)),
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
    // Grid candidates are a fixed, deduplicated, (v, u)-sorted list
    // computed ONCE (above the retry loop): mesh ids mint row-major on
    // the FINAL kept set, and a retry drops candidates by INDEX — with
    // per-cell generation a cell-boundary line is shared by adjacent
    // cells, and dropping a location rather than one cell's copy of it
    // is what keeps the retry ladder strictly shrinking (module docs).
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
    };
    if let Some(i) = developed_merge(trimmed.plane(), &polygon.points, &singular) {
        return Err(TessellateError::SphereTrimShape {
            face: fk,
            edge: polygon.edges[i],
            cause: SphereTrimCause::DevelopedMerge,
        });
    }
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
            worst_cert: measure.worst,
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
    worst: f64,
    /// Some triangle's bound is [`cert::Verdict::Exceeded`] at its own
    /// rounding magnitude: the attempt refuses.
    exceeded: bool,
    /// The largest allowance a [`cert::Verdict::Unresolved`] triangle
    /// would have had to absorb, if any.
    unresolved: Option<f64>,
    worst_dev: f64,
    worst_dev_cert: f64,
    dev_samples: u64,
}

impl AttemptMeasure {
    fn fresh() -> Self {
        Self {
            worst: 0.0,
            exceeded: false,
            unresolved: None,
            worst_dev: f64::NAN,
            worst_dev_cert: f64::NAN,
            dev_samples: 0,
        }
    }
}

/// What an attempt that did not refuse asks of the retry loop.
enum Next {
    /// Rebuild: a grid point dropped, or refinement candidates were
    /// appended.
    Rebuild,
    /// The attempt's patch, accepted.
    Accept(Patch),
}

/// What every attempt on one face reads and none changes.
struct TrimmedFace<'a> {
    fk: FaceKey,
    /// The face's orientation sense: its outward normal is the
    /// surface's when true ([`Self::outward`]).
    sense: bool,
    /// The trim polygon's orientation in the chart
    /// ([`trim_orientation`]), which the emitted triangles take. It is
    /// the polygon's and not `sense` restated: on the spline arm
    /// nothing compares the two.
    orientation: PolygonOrientation,
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
    /// The UV centroids of kept triangles on three consecutive points of
    /// the trim polygon (an ear of a boundary chain), to insert and
    /// rebuild — analytic arms only.
    ears: Vec<(f64, f64)>,
}

impl TrimmedFace<'_> {
    /// **The plane the CDT runs in**: the cone's development on the
    /// face's nappe ([`nappe`]; `singular_row_shape` has refused a face
    /// on both), or the chart in the lattice metric.
    fn plane(&self) -> CdtPlane {
        match self.row {
            ChartRow::Analytic(a) => match a.development() {
                Some(sin_alpha) => CdtPlane::Development {
                    sin_alpha,
                    nappe: nappe(&self.polygon.points, self.singular),
                },
                None => CdtPlane::Lattice {
                    metric: self.metric,
                },
            },
            ChartRow::Spline { .. } => CdtPlane::Lattice {
                metric: self.metric,
            },
        }
    }

    /// `(u, v)` as the CDT sees it, off the chart's singular vertex
    /// ([`cdt_point`]).
    #[cfg(test)]
    fn cdt_point(&self, u: f64, v: f64) -> SpadePoint<f64> {
        cdt_point(self.plane(), u, v, false)
    }

    /// **The point a kept triangle's refinement or ear cure inserts**,
    /// strictly inside the triangle where the CDT runs: in the chart, the
    /// centroid of its corners. In the development, the centroid of its
    /// developed corners (`at`), read back to the chart: its angle by
    /// [`geom::Azimuth::near`] about the developed angle of a corner off
    /// the apex (a triangle has one, its angle at the apex under half a
    /// turn), its radius as its length. The chart centroid is no answer
    /// there: the development bends the chart's straight lines, and at
    /// an apex corner the chart holds the apex at one copy's `u`, which
    /// is any azimuth.
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
        let (crossings, mut split_offender) = self.constrain_boundary(&mut at, dropped)?;
        split_offender |= self.chain_crease_lines(&mut at, dropped)?;
        if split_offender {
            if attempt == last_attempt() {
                return Err(TessellateError::Triangulation { face: fk });
            }
            return Ok(Next::Rebuild);
        }
        let Emitted {
            interior,
            triangles,
            refine,
            folded,
            ears,
        } = self.emit(&at, &crossings, candidates, measure, falsification)?;
        // THE EAR CURE (every analytic arm). A kept triangle on three
        // consecutive points of the trim polygon is an ear of a boundary
        // chain, and it is the same three mesh vertices whichever face
        // cuts it: a neighbour across that chain that cuts it too
        // (a plane cap, whose polygon is convex, cuts ears freely) makes
        // the triangle twice, wound both ways, and their diagonal an edge
        // of four. No patch sees that alone — each uses the diagonal
        // twice, as any interior edge — so this face never keeps an ear:
        // a point strictly inside it where the CDT runs, and so inside the
        // polygon (`TrimmedFace::inside_point`), is inserted and the face
        // rebuilt, which splits it.
        // An ear the retry budget does not clear refuses typed
        // (`work/tesslie/two-faces-cutting-one-ear-of-a-shared-chord-chain-leave-a-four-use-edge.md`).
        if !ears.is_empty() {
            if attempt == last_attempt() {
                return Err(TessellateError::BoundaryEar { face: fk });
            }
            candidates.extend(ears);
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
        // set stays valid); a refinement point landing exactly on a
        // constraint is caught by the same intermediate-vertex
        // classification as any candidate.
        if !refine.is_empty() && !measure.worst.is_nan() && attempt < last_attempt() {
            candidates.extend(refine);
            return Ok(Next::Rebuild);
        }
        if measure.exceeded {
            return Err(TessellateError::CertificateExceeded {
                face: fk,
                bound: measure.worst,
                requested: self.tol.delta,
            });
        }
        if let Some(allowance) = measure.unresolved {
            return Err(TessellateError::RoundingUnresolved {
                face: fk,
                bound: measure.worst,
                requested: self.tol.delta,
                allowance,
            });
        }
        // D2 addendum row 5, the same re-derivation `curved`'s emit
        // pass makes and for the same class (issue 897): an edge
        // incident to a vertex this walk placed at two distinct UV
        // locations, used more than twice inside ONE patch, is the
        // non-manifold state the duplicate-id drop in `emit` is claimed
        // never to produce. This lane's sources of such a vertex are the
        // cylinder's full-2π seam double-traversal and a singular point's
        // copies — the sphere's and the spindle's pole sides, and the
        // cone's apex, which its development holds as one CDT vertex, so
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
        // a pinch's vertices, or a vertex the walk meets again. The
        // development rounds, and puts every copy of the apex at one
        // point on purpose: there the copies of the singular vertex share
        // one, and a face whose polygon points would share one apart from
        // those has refused before its first attempt ([`developed_merge`]),
        // so no two mesh vertices merge that the chart keeps apart. A grid
        // candidate landing on an earlier point is dropped, as in the
        // chart: it is no mesh vertex yet.
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
        let key = |x: f64| if x == 0.0 { 0u64 } else { x.to_bits() };
        let mut unique: Vec<SpadePoint<f64>> = Vec::with_capacity(inputs.len());
        let mut index_of: HashMap<(u64, u64), usize> = HashMap::with_capacity(inputs.len());
        let mut handle_of: Vec<FixedVertexHandle> = Vec::with_capacity(inputs.len());
        for &(u, v, singular) in &inputs {
            let p = cdt_point(plane, u, v, singular);
            let next = unique.len();
            let i = *index_of.entry((key(p.x), key(p.y))).or_insert(next);
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
        for (&(u, v, id), &h) in self.polygon.points.iter().zip(&handle_of) {
            walk.enter(&mut meta, h, (u, v, Slot::Boundary(id)), id);
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
            if h.index() == meta.len() {
                meta.push((u, v, Slot::Grid(k)));
            }
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
    /// (the planar #116 pattern); answers the counts and whether a grid
    /// point dropped.
    fn constrain_boundary(
        &self,
        at: &mut Cdt,
        dropped: &mut HashSet<usize>,
    ) -> Result<(Crossings, bool), TessellateError> {
        let fk = self.fk;
        let Cdt {
            cdt,
            meta,
            handles,
            pinches,
            ..
        } = at;
        // Constraints with crossing counts (the planar #116 pattern);
        // a constraint realised through an intermediate vertex means a
        // grid point sits exactly on it — drop and rebuild.
        let mut crossings: Crossings = HashMap::new();
        let mut split_offender = false;
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
                // Classify every intermediate vertex (a vertex of a
                // realised sub-edge that is neither endpoint of THIS
                // segment): a grid point drops and the pass rebuilds;
                // a BOUNDARY point means the trim loop touches itself
                // exactly on this segment — no rebuild can repair
                // that, and silently keeping it would emit a 3-D
                // T-junction (this face triangulated against a
                // sub-segment its neighbour does not share). Refused
                // TYPED (review MIN-1: the advertised watertightness
                // backstop must not have a silent arm).
                for e in &realised {
                    let e = cdt.directed_edge(*e);
                    for vh in [e.from(), e.to()] {
                        let h = vh.fix();
                        if h == a || h == b {
                            continue;
                        }
                        let idx = h.index();
                        match meta[idx].2 {
                            Slot::Grid(k) => {
                                dropped.insert(k);
                                split_offender = true;
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
        Ok((crossings, split_offender))
    }

    /// THE CREASE LINES (`Creases`): each crease is a chain of
    /// constraints through its vertices in order along it, so no
    /// triangle crosses it. Boundary crossings of a crease are
    /// vertices of the chain (`trim_polygon` refuses any other), so
    /// a link never crosses a boundary constraint. A link through an
    /// intermediate vertex has a grid point exactly on it, which
    /// drops as on a boundary segment. The links stay out of
    /// `crossings`: a crease is not a side of the face. Answers whether
    /// a grid point dropped.
    fn chain_crease_lines(
        &self,
        at: &mut Cdt,
        dropped: &mut HashSet<usize>,
    ) -> Result<bool, TessellateError> {
        let fk = self.fk;
        let mut split_offender = false;
        let Some(through) = chain_creases(&mut at.cdt, &mut at.on_crease) else {
            return Err(TessellateError::Triangulation { face: fk });
        };
        for h in through {
            match at.meta[h.index()].2 {
                Slot::Grid(k) => {
                    dropped.insert(k);
                    split_offender = true;
                }
                Slot::Boundary(_) => {
                    return Err(TessellateError::Triangulation { face: fk });
                }
            }
        }
        Ok(split_offender)
    }

    /// The emit pass: even-odd interior flags, the kept grid points
    /// numbered, and every kept triangle certified, measured and
    /// queued for refinement when it certifies above δ.
    fn emit(
        &self,
        at: &Cdt,
        crossings: &Crossings,
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
        let flip = self.orientation == PolygonOrientation::Clockwise;
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
        // handle holds several), for the ear test below.
        let n_poly = at.handles.len();
        let mut places: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, h) in at.handles.iter().enumerate() {
            places.entry(h.index()).or_default().push(i);
        }
        let mut ears = Vec::new();
        for f in cdt.inner_faces() {
            if !inside[f.fix().index()] {
                continue;
            }
            let vs = f.vertices();
            let mut ids = [PatchVertex::Shared(0); 3];
            let mut uv = [[0.0f64; 2]; 3];
            for (k, vtx) in vs.iter().enumerate() {
                let (u, v, slot) = meta[vtx.fix().index()];
                uv[k] = [u, v];
                ids[k] = match slot {
                    Slot::Boundary(id) => PatchVertex::Shared(id),
                    Slot::Grid(c) => grid_ids[&c],
                };
                match pinches.id_in(f, vtx.fix(), flip) {
                    Ok(Some(id)) => ids[k] = PatchVertex::Shared(id),
                    Ok(None) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            // A triangle with two corners on one mesh vertex is
            // degenerate in 3-D. Dropping it is `curved`'s idiom, and
            // there it collapses a pole fan (#678). On the sphere's and
            // the spindle's arms it does the same at a pole side; the
            // cone's apex is one CDT vertex in its development, so no
            // triangle there has two corners on it; elsewhere the drop is
            // all it is, because neither ingredient of that fan reaches
            // the face — no singular point, and no repeated id at two UV
            // locations with a single interior column between them.
            //
            // WHAT IS ASSERTED, AND WHERE THE COVER STOPS. The singular
            // half is the row's: a chart with singular points holds its
            // polygon to `singular_row_shape`'s shapes. The INGREDIENT
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
            let singular =
                ids.map(|id| matches!(id, PatchVertex::Shared(id) if self.singular.contains(&id)));
            let developed = vs.map(|vtx| vtx.position());
            let tri = [
                ids[0].position(shared, &interior),
                ids[1].position(shared, &interior),
                ids[2].position(shared, &interior),
            ];
            // Wound as emitted below: reversed under `flip`.
            let normal = (tri[1] - tri[0]).cross(tri[2] - tri[0]);
            let normal = if flip { -normal } else { normal };
            let centroid = Point3::new(
                (tri[0].x + tri[1].x + tri[2].x) / 3.0,
                (tri[0].y + tri[1].y + tri[2].y) / 3.0,
                (tri[0].z + tri[1].z + tri[2].z) / 3.0,
            );
            // An ear: three corners at consecutive places of one loop of
            // the polygon (cyclically), read on the analytic arms.
            if !refine_lane && n_poly >= 3 {
                let at_places: Vec<&Vec<usize>> = vs
                    .iter()
                    .filter_map(|vtx| places.get(&vtx.fix().index()))
                    .collect();
                let next = |i| self.polygon.next(i);
                let consecutive = |a: usize, b: usize, c: usize| {
                    let set = [a, b, c];
                    set.iter()
                        .any(|&i| set.contains(&next(i)) && set.contains(&next(next(i))))
                };
                if at_places.len() == 3
                    && at_places[0].iter().any(|&a| {
                        at_places[1]
                            .iter()
                            .any(|&b| at_places[2].iter().any(|&c| consecutive(a, b, c)))
                    })
                {
                    ears.push(Self::inside_point(plane, &uv, developed, singular));
                }
            }
            // Folded unless wound outward: a zero dot, or NaN where
            // `outward` is poison, is a fold.
            if self.outward(centroid).is_some_and(|n| {
                let wound_outward = normal.dot(n.vec()) > 0.0;
                !wound_outward
            }) {
                let grid: Vec<usize> = vs
                    .iter()
                    .filter_map(|vtx| match meta[vtx.fix().index()].2 {
                        Slot::Grid(k) => Some(k),
                        Slot::Boundary(_) => None,
                    })
                    .collect();
                folded_boundary |= grid.is_empty();
                folded_grid.extend(grid);
            }
            let (bound, scale) = self.row.cert(tri, uv, singular);
            let verdict = cert::verdict(bound, tol.delta, scale, tol.eps);
            match verdict {
                cert::Verdict::Certified => {}
                cert::Verdict::Exceeded => measure.exceeded = true,
                cert::Verdict::Unresolved { allowance } => {
                    measure.unresolved = Some(
                        measure
                            .unresolved
                            .map_or(allowance, |a: f64| a.max(allowance)),
                    );
                }
            }
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
                        let d =
                            ((s.x - pi.x).powi(2) + (s.y - pi.y).powi(2) + (s.z - pi.z).powi(2))
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
            // Sticky-NaN accumulation (the curved lane's rule).
            if bound.is_nan() || measure.worst.is_nan() || bound > measure.worst {
                measure.worst = bound;
            }
            if refine_lane && verdict == cert::Verdict::Exceeded {
                refine.push(Self::inside_point(plane, &uv, developed, singular));
            }
            triangles.push(if flip { [ids[0], ids[2], ids[1]] } else { ids });
        }

        Ok(Emitted {
            interior,
            triangles,
            refine,
            folded: (folded_grid, folded_boundary),
            ears,
        })
    }
}

/// `(u, v)` as the CDT sees it on `plane`, `singular` where the point
/// is a copy of the chart's singular vertex.
///
/// - **The chart** ([`CdtPlane::Lattice`]): floored
///   ([`crate::cdt::spade_point`]), then in the lattice metric, `(u/k,
///   v)` times a uniform power of two (module docs, "The lattice
///   metric", on what that scale does and does not leave unchanged).
///   The axis that GROWS is scaled, `v` by `k` when `k > 1` and `u` by
///   `1/k` when `k < 1`: a power of two of at least one, which rounds
///   nothing and cannot carry a value spade accepts into the floor's
///   band, so the scale is injective on floored points and two chart
///   points are one CDT vertex exactly when
///   [`crate::cdt::ids_at_two_uvs`] reads them as one location.
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
        CdtPlane::Lattice { metric } => {
            let p = crate::cdt::spade_point(u, v);
            if metric > 1.0 {
                SpadePoint::new(p.x, p.y * metric)
            } else {
                SpadePoint::new(p.x / metric, p.y)
            }
        }
        CdtPlane::Development { .. } if singular => crate::cdt::spade_point(0.0, 0.0),
        CdtPlane::Development { sin_alpha, nappe } => {
            let (x, y) = developed(sin_alpha, nappe, u, v);
            crate::cdt::spade_point(x, y)
        }
    }
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

/// The nappe a cone face lies on, `+1` or `−1`: the sign of `v` at its
/// points off the apex (`singular_row_shape` has refused a face with
/// points on both).
fn nappe(polygon: &[(f64, f64, u32)], singular: &HashSet<u32>) -> f64 {
    let below = polygon
        .iter()
        .any(|&(_, v, id)| !singular.contains(&id) && v < 0.0);
    if below { -1.0 } else { 1.0 }
}

/// **The first polygon point the development would merge with one at
/// another UV location** (module docs, "The development", which points are
/// one CDT vertex), as its index; `None` in the chart, whose map is
/// injective on floored points. The copies of the singular vertex share
/// the origin, and points at one UV location (a pinch's vertices, or a
/// vertex the walk meets again) share a point as in the chart; any other
/// two polygon points that the development's rounding or spade's floor
/// put at one point are two mesh vertices the CDT would hold as one, and
/// the face refuses `SphereTrimShape { DevelopedMerge }` at the second.
fn developed_merge(
    plane: CdtPlane,
    polygon: &[(f64, f64, u32)],
    singular: &HashSet<u32>,
) -> Option<usize> {
    if let CdtPlane::Lattice { .. } = plane {
        return None;
    }
    let key = |x: f64| if x == 0.0 { 0u64 } else { x.to_bits() };
    let mut first: HashMap<(u64, u64), (SpadePoint<f64>, bool)> = HashMap::new();
    polygon.iter().position(|&(u, v, id)| {
        let apex = singular.contains(&id);
        let p = cdt_point(plane, u, v, apex);
        let at = crate::cdt::spade_point(u, v);
        let (was, was_apex) = *first.entry((key(p.x), key(p.y))).or_insert((at, apex));
        !(was == at || apex && was_apex)
    })
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
/// points, so no triangle of the face lies along a ruling. The cut moves
/// a point by under `2⁻²⁵` of its radius, which the CDT alone sees.
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

/// A triangle's centroid in UV: the point both the ear cure and the
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

/// **The loops' vertices that are the chart's singular points**, as mesh
/// ids, a ring's as an outer loop's: each loop edge's end chord ids (the
/// vertices' own points, bitwise, `crate::chords`), kept where
/// [`AnalyticRow::is_singular`] decides the point is one, at `band`. It
/// is the decision the loop's lift takes to turn the face there
/// (`chart_table`'s module docs), read on the same points, so the
/// polygon's sides at a singular point are the lift's.
///
/// # Errors
///
/// `SphereTrimShape { SingularUndecided }` at a vertex the band cannot
/// place on or off a singular point, read before the polygon is walked.
fn singular_vertices(
    body: &Body<f64>,
    fk: FaceKey,
    loops: &[&Lift<f64>],
    chords: &ChordPass,
    shared: &[Point3<f64>],
    row: &AnalyticRow<'_>,
    band: geom_core::predicate::Band,
) -> Result<HashSet<u32>, TessellateError> {
    let mut out = HashSet::new();
    for lifted in loops.iter().flat_map(|l| l.rows()) {
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
            match row.is_singular(p, band) {
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
/// - a mesh id at two chart locations that is no singular vertex: a seam
///   traversed twice.
///
/// A loop that winds about a singular point inside the face does not
/// close in the chart, and the door refuses it before any polygon is
/// drawn ([`TessellateError::DomainUnread`]). So no singular point lies
/// strictly inside a polygon read here: a row that ends the chart's `v`
/// domain bounds every loop, so the region's open inside stops short of
/// it, and a row inside the domain (a cone's apex) has the face's points
/// on one side of it only.
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

/// The uniform interior grid candidates of every analytic arm: the trim
/// box divided `nu × nv`, interior points only (the box boundary is at
/// or outside the trim polygon), in the historical arithmetic —
/// positions are bit-identical to the pre-TESS-SPAN grid. Already
/// generated in (v, u) row-major order; sorted anyway so every arm
/// hands the retry loop the same invariant.
///
/// At `nu == 1` (or `nv == 1`) a range below is empty and the other
/// axis' computed count is dropped — a member of the sibling class
/// `curved::grid_counts`' doc records (issue 685 decided only the
/// cone's `nu == 1` case; here the candidates are heuristic seeds and
/// the per-triangle certificate is the stated guarantee).
fn uniform_candidates(u: (f64, f64), v: (f64, f64), nu: usize, nv: usize) -> Vec<(f64, f64)> {
    let mut cand = Vec::new();
    for j in 1..nv {
        for i in 1..nu {
            #[allow(clippy::cast_precision_loss)]
            let uu = u.0 + (u.1 - u.0) * (i as f64 / nu as f64);
            #[allow(clippy::cast_precision_loss)]
            let vv = v.0 + (v.1 - v.0) * (j as f64 / nv as f64);
            cand.push((uu, vv));
        }
    }
    sort_candidates(&mut cand);
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
) -> Result<(Vec<(f64, f64)>, ScheduleStats), TessellateError> {
    let mut cand: Vec<(f64, f64)> = Vec::new();
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
        for j in 0..=b.nvc {
            let vv = at(b.va, b.vb, j, b.nvc);
            if !(vv > v.0 && vv < v.1) {
                continue;
            }
            for i in 1..b.nuc {
                #[allow(clippy::cast_precision_loss)]
                let uu = u.0 + (u.1 - u.0) * (i as f64 / b.nuc as f64);
                cand.push((uu, vv));
            }
        }
    }
    sort_candidates(&mut cand);
    cand.dedup();
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

/// Row-major candidate order: by `v`, then `u` (total order — the
/// values are finite by construction, and `total_cmp` keeps the sort
/// deterministic without a partial-compare unwrap).
fn sort_candidates(cand: &mut [(f64, f64)]) {
    cand.sort_unstable_by(|a, b| a.1.total_cmp(&b.1).then(a.0.total_cmp(&b.0)));
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
/// # Errors
///
/// [`TessellateError::MissingEntity`] for a loop of no signed area,
/// or of a poisoned one: it bounds no region, and an orientation
/// assumed for it would hand the CDT a boundary that crosses itself or
/// lies on one line as though it were a face.
/// [`TessellateError::TrimLoopOrientation`] on an analytic arm whose
/// outer loop is oriented against its face, and on any arm for a ring
/// oriented with its outer loop.
fn trim_orientation(
    fk: FaceKey,
    sense: bool,
    spline: bool,
    trim: &TrimPolygon,
) -> Result<PolygonOrientation, TessellateError> {
    let of = |points: &[(f64, f64, u32)]| {
        let poly2: Vec<[f64; 2]> = points.iter().map(|&(u, v, _)| [u, v]).collect();
        PolygonOrientation::of_signed_area(shoelace2(&poly2)).ok_or(
            TessellateError::MissingEntity {
                what: "degenerate trimmed boundary",
            },
        )
    };
    let orientation = of(trim.outer())?;
    if !spline && orientation != PolygonOrientation::of_face(sense) {
        return Err(TessellateError::TrimLoopOrientation { face: fk });
    }
    for ring in trim.rings() {
        if of(ring)? == orientation {
            return Err(TessellateError::TrimLoopOrientation { face: fk });
        }
    }
    Ok(orientation)
}

/// The loops of a face's domain, outer loop first: the outer loop's lift
/// and each ring's as the door placed it ([`topo::Body::face_lift`]).
pub(crate) fn domain_loops(lift: &FaceLift<f64>) -> Vec<&Lift<f64>> {
    let mut loops = vec![lift.outer()];
    loops.extend(lift.rings().iter().map(topo::pcurves::PlacedRing::lift));
    loops
}

/// The pcurve-driven UV polygon of the face's domain, every loop of it,
/// each as the door places it (`lift`, [`topo::Body::face_lift`]): per
/// half-edge, `pcurve.eval` at the shared chord parameters (module
/// docs; each traversal contributes all but its last point, and its last
/// point too where that is a `singular` vertex the next traversal leaves
/// from another `u`, so the singular point's row is a side of the polygon
/// — [`singular_row_shape`]).
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
    singular: &HashSet<u32>,
) -> Result<TrimPolygon, TessellateError> {
    let read =
        |lift: &Lift<f64>| loop_polygon(body, fk, lift, chords, nurbs_chart, creases, singular);
    let mut trim = TrimPolygon::new(read(lift.outer())?);
    for ring in lift.rings() {
        trim.push_ring(read(ring.lift())?);
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
/// vertex the next traversal leaves from another `u`, so the singular
/// point's row is a side of the polygon ([`singular_row_shape`]).
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

/// One loop's points, each its chart `(u, v)` and mesh id in walk order,
/// and beside them the edge each point's traversal walks.
type LoopPoints = (Vec<(f64, f64, u32)>, Vec<EdgeKey>);

/// **A face's trim polygon**: every loop of its domain, outer loop first
/// and then the rings in the face's order, each in walk order. The CDT
/// takes every loop's sides as constraints, and the even-odd reading
/// (`planar::classify_faces`) is what makes a ring's inside a hole.
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
        // Two entries at one handle (a boundary point a grid point
        // landed on) are one vertex, and a vertex is no link.
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
/// - the grid candidates within rounding of a crease are moved onto
///   it, and each grid row gains a point where it meets the crease
///   ([`Self::cut_candidates`]);
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
/// crease; a grid point any nearer than this would only cut a sliver
/// beside the crease line.
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

    /// The grid candidates cut at the creases: each within rounding of
    /// a crease moved onto it, and a point on each crease at every
    /// grid line across it (a row for a `u` crease, a column for a
    /// `v` one). Sorted and deduplicated again, as the lane takes them.
    fn cut_candidates(&self, cand: &mut Vec<(f64, f64)>) {
        for p in cand.iter_mut() {
            *p = self.snap(*p);
        }
        let mut lines: Vec<f64> = cand
            .iter()
            .map(|p| if self.axis == 0 { p.1 } else { p.0 })
            .collect();
        lines.sort_unstable_by(f64::total_cmp);
        lines.dedup();
        for &c in &self.at {
            for &l in &lines {
                cand.push(if self.axis == 0 { (c, l) } else { (l, c) });
            }
        }
        sort_candidates(cand);
        cand.dedup();
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
pub(super) mod tests {
    use super::{
        ChordPass, Creases, FixedVertexHandle, SpadePoint, TessellateError, chain_creases,
        id_repeats_apart, trim_polygon,
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
    fn triangulate(c: &Creases, mut cand: Vec<(f64, f64)>, cut: bool) -> Vec<(f64, f64)> {
        let boundary = square(c, 7);
        if cut {
            c.cut_candidates(&mut cand);
        }
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

    /// A grid point and a boundary point one ulp off a crease merge
    /// onto it, on either side: neither survives beside the crease line
    /// to cut a sliver, and the triangulation still keeps every
    /// triangle on one side.
    #[test]
    fn a_point_an_ulp_off_a_crease_merges_onto_it() {
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
            c.cut_candidates(&mut cand);
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
        let chords: ChordPass =
            crate::chords::compute_chords(&body, 5e-3, &vids, &mut positions, &mut bounds).unwrap();
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
            let polygon =
                trim_polygon(&body, fk, &lift, &chords, true, c.as_ref(), &HashSet::new())
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
        let chords: ChordPass =
            crate::chords::compute_chords(&body, delta, &vids, &mut positions, &mut bounds)
                .unwrap();
        let (mut faces, mut worst) = (0, 0.0f64);
        for (fk, face) in body.faces() {
            let surface = body.face_surface(fk).unwrap();
            if !matches!(surface, geom::Surface::Sphere { .. })
                || crate::tessellate::lane_of(&body, fk, surface).unwrap()
                    != crate::tessellate::Lane::Trimmed
            {
                continue;
            }
            faces += 1;
            let row = crate::chart_table::AnalyticRow::of(&body, fk, surface).unwrap();
            let (hu, hv) = super::arm_steps(&body, fk, &row, delta).unwrap();
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
            for (fk, face) in body.faces() {
                let surface = body.face_surface(fk).unwrap();
                let geom::Surface::Cone {
                    apex,
                    axis,
                    half_angle,
                    ..
                } = *surface
                else {
                    continue;
                };
                if crate::tessellate::lane_of(&body, fk, surface).unwrap()
                    != crate::tessellate::Lane::Trimmed
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
                let chords: ChordPass =
                    crate::chords::compute_chords(&body, delta, &vids, &mut positions, &mut bounds)
                        .unwrap();
                for (fk, face) in body.faces() {
                    let surface = body.face_surface(fk).unwrap();
                    if !matches!(
                        surface,
                        geom::Surface::Cone { .. } | geom::Surface::Torus { .. }
                    ) || crate::tessellate::lane_of(&body, fk, surface).unwrap()
                        != crate::tessellate::Lane::Trimmed
                    {
                        continue;
                    }
                    faces += 1;
                    let row = crate::chart_table::AnalyticRow::of(&body, fk, surface).unwrap();
                    let (hu, hv) = super::arm_steps(&body, fk, &row, delta).unwrap();
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
                        let target = super::face_target(&body, fk, delta).unwrap();
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
                        apex: far,
                        axis,
                        half_angle: 0.5,
                        u_ref,
                    },
                    Analytic::Cone {
                        half_angle: 0.5,
                        rho_max: scale,
                    },
                ),
                (
                    Surface::Sphere {
                        center: far,
                        radius: scale,
                        axis,
                        u_ref,
                    },
                    Analytic::Sphere {
                        radius: scale,
                        widest: 1.0,
                    },
                ),
                (
                    Surface::Spindle {
                        center: far,
                        axis,
                        major_radius: -0.5 * scale,
                        minor_radius: scale,
                        u_ref,
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
                        origin: far,
                        axis,
                        radius: scale,
                        u_ref,
                    },
                    Analytic::Cylinder { radius: scale },
                ),
                (
                    Surface::Torus {
                        center: far,
                        axis,
                        major_radius: 2.0 * scale,
                        minor_radius: 0.5 * scale,
                        u_ref,
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
        let chords: ChordPass =
            crate::chords::compute_chords(&body, 1e-2, &vids, &mut positions, &mut bounds).unwrap();
        let mut with_poles = 0;
        for (fk, face) in body.faces() {
            let surface = body.face_surface(fk).unwrap();
            if !matches!(surface, geom::Surface::Sphere { .. })
                || crate::tessellate::lane_of(&body, fk, surface).unwrap()
                    != crate::tessellate::Lane::Trimmed
            {
                continue;
            }
            let crate::chart_table::ChartRow::Analytic(row) =
                crate::chart_table::ChartRow::of(&body, fk, surface, &bounds).unwrap()
            else {
                panic!("a sphere face has an analytic row");
            };
            let lift = body.face_lift(fk, band()).unwrap();
            let loops = super::domain_loops(&lift);
            let singular =
                super::singular_vertices(&body, fk, &loops, &chords, &positions, &row, band())
                    .unwrap();
            let polygon = super::trim_polygon(&body, fk, &lift, &chords, false, None, &singular)
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
    /// a refusal on every arm for a polygon of no signed area (a
    /// figure-eight of equal lobes, three points on one line), which has
    /// no orientation to assume.
    #[test]
    fn a_trim_polygon_is_oriented_as_read_and_one_of_no_area_refuses() {
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
                    Some(has),
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
                Some(has),
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
                Some(CounterClockwise),
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
            assert!(
                matches!(
                    super::trim_orientation(
                        fk,
                        true,
                        nurbs_chart,
                        &super::TrimPolygon::of_loops(&outer, &[&flat])
                    ),
                    Err(TessellateError::MissingEntity {
                        what: "degenerate trimmed boundary"
                    })
                ),
                "spline arm {nurbs_chart}: a ring of no area refuses"
            );
        }
        for (name, polygon) in [("figure-eight", &eight), ("collinear", &line)] {
            for (sense, nurbs_chart) in [(true, false), (false, false), (true, true), (false, true)]
            {
                assert!(
                    matches!(
                        super::trim_orientation(fk, sense, nurbs_chart, polygon),
                        Err(TessellateError::MissingEntity {
                            what: "degenerate trimmed boundary"
                        })
                    ),
                    "{name}, sense {sense}, spline arm {nurbs_chart}: no area, no orientation"
                );
            }
        }
    }

    /// The tilted cylinder halves of `m5_pr11_trimmed` at radius `r`: a
    /// cylinder of height 2.5 split by a plane tilted 0.3 rad, whose
    /// trimmed walls take the cylinder arm.
    fn tilted_cylinder_above(r: f64) -> topo::Body<f64> {
        use geom_core::{Point2, Point3, Tol, Vec3};
        let lp = profile::test_support::bulge_loop(vec![
            (Point2::new(-r, 0.0), 1.0),
            (Point2::new(r, 0.0), 1.0),
        ]);
        let disc = profile::Profile::new(profile::SketchPlane::xy(), vec![lp])
            .validate(Tol::witness())
            .unwrap();
        let cylinder = sweep::extrude(
            &disc,
            sweep::Extrusion::Distance {
                depth: 2.5,
                side: sweep::ExtrudeSide::Along,
            },
            Tol::witness(),
        )
        .unwrap()
        .body;
        let cylinder = topo::test_support::finished("the cylinder", cylinder, Tol::witness());
        let plane = topo::test_support::split_plane(
            Point3::new(0.0, 0.0, 1.25),
            Vec3::new(0.3f64.sin(), 0.0, 0.3f64.cos()),
            Tol::witness(),
        );
        let result = topo::splitting::split(&cylinder, &plane, Tol::witness()).unwrap();
        let topo::splitting::SplitPart::Body(above) = &result.above else {
            panic!("material above the cut");
        };
        topo::Body::clone(above)
    }

    /// **An ear the retry budget leaves refuses `BoundaryEar`.** The
    /// wide tilted cylinder's trimmed wall keeps ears on its ellipse
    /// chain at some δ of the review's sweep, which the ear cure splits
    /// on the next attempt. With the loop's last attempt set to the first
    /// (`LAST_ATTEMPT`, test-only) there is no next attempt, so the
    /// face refuses typed; with the budget as shipped every δ meshes.
    /// Set on a one-thread pool's worker, which runs the faces.
    #[test]
    fn an_ear_the_budget_leaves_refuses_typed() {
        use geom_core::Tol;
        let body = tilted_cylinder_above(5.0);
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .expect("a one-thread pool");
        let (refused, meshed) = pool.install(|| {
            let (mut refused, mut meshed) = (0, 0);
            for k in 0..20 {
                let delta = 5.0 * 5e-2 * (3e-4f64 / 5e-2).powf(f64::from(k) / 19.0);
                LAST_ATTEMPT.with(|a| a.set(Some(0)));
                let first = crate::tessellate(&body, delta, Tol::witness());
                LAST_ATTEMPT.with(|a| a.set(None));
                if matches!(first, Err(TessellateError::BoundaryEar { .. })) {
                    refused += 1;
                    let m = crate::tessellate(&body, delta, Tol::witness())
                        .unwrap_or_else(|e| panic!("δ {delta:e}: at the shipped budget, {e:?}"));
                    crate::validate::check_mesh(&m).expect("and it checks");
                    meshed += 1;
                }
            }
            (refused, meshed)
        });
        assert!(refused > 0, "some δ keeps an ear on its first attempt");
        assert_eq!(refused, meshed);
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
            match super::singular_row_shape(fk, &poles, &HashSet::new(), &trim) {
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
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
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
                orientation: PolygonOrientation::of_face(true),
                metric: k,
                surface: &sphere,
                row: &row,
                singular: &HashSet::new(),
                polygon: &super::TrimPolygon::of_loops(&[], &[]),
                creases: None,
                shared: &[],
                tol: &tol,
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
                apex: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                half_angle,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
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
        ///   first attempt (`developed_merge`), where the face refuses typed
        ///   rather than merge two mesh vertices the chart keeps apart;
        /// - a grid candidate landing there is dropped: it is no mesh vertex.
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
            let face = |polygon| TrimmedFace {
                fk: FaceKey::default(),
                sense: true,
                orientation: PolygonOrientation::of_face(true),
                metric: 1.0,
                surface: &surface,
                row: &row,
                singular: &singular,
                polygon,
                creases: None,
                shared: &[],
                tol: &tol,
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
            let at = wedge
                .insert_points(&[(0.5, 0.0), (0.5, 0.5)], &HashSet::new())
                .expect("the wedge's points enter the CDT");
            assert_eq!(
                at.meta.len(),
                3 + 1,
                "the apex and the two polygon points off it, and the one grid candidate off the apex"
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
            assert_eq!(
                super::super::developed_merge(crushed.plane(), &crushed.polygon.points, &singular),
                Some(1),
                "a polygon point off the apex whose development lands on the apex's CDT point \
                 merges two mesh vertices; the face refuses there"
            );
            assert_eq!(
                super::super::developed_merge(wedge.plane(), &wedge.polygon.points, &singular),
                None,
                "the apex copies alone share a point"
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
