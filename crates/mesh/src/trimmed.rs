//! **Trimmed curved-face tessellation** (M5 PR 11, C12.6): faces whose
//! boundary carries a conic or B-spline trim carrier are not swept UV
//! rectangles, so the iso walk cannot traverse them — instead the
//! face's **stored pcurves** (PR 6 caches; C4's consumers begin here)
//! provide the trim polyline in UV, evaluated at exactly the shared
//! per-edge chord parameters, and the CDT + even-odd machinery of the
//! planar lane (#116: combinatorial flood fill, no geometric
//! classification) picks the interior.
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
//! Three charts construct here: **cylinder** (M5 PR 11 — the tiltedcut
//! walls; every split cylinder wall), **sphere** (a sphere face a
//! circle tilted against its chart bounds — the sphere pairs, a ball
//! cut by a tilted plane, the oblique fillet corners) and, since M7's montage
//! skin-scenes unit, **described NURBS** (the loft/sweep
//! walls — the frontier this header used to call "the banked
//! trimmed-NURBS lane", now promoted; NURBS faces route here
//! unconditionally, iso-rectangle or not, because the swept-rectangle
//! walk has no NURBS chart; RATIONAL faces since M8-5, whose Hessian
//! bound is the second-order quotient statement over the homogeneous
//! nets).
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
//! (`crate::chords::general_uv_speeds`) rather than a formula.
//!
//! The sphere arm's faces are bounded by circles: parallels and
//! meridians carry their `Harmonic` image, and a tilted circle its
//! `Projected` one, the chart's inverse of the carrier, which the walk
//! reads exactly. The chord pass bounds each image's UV step by the
//! arm's lattice steps from a closed form for the circle's
//! `(u′(t), v′(t))` (`crate::chords::sphere_uv_speeds`), and its
//! certificate is the ball-convexity one (`cert::cert_sphere`), in 3-D.
//! A pole the loop passes is a side of the trim polygon
//! ([`sphere_polygon_shape`]); a face holding a pole inside it, and a
//! tilted circle passing a pole nearer than the arm's lattice resolves
//! ([`NEAR_POLE_ROW_SHARE`]), refuse typed. Conic and B-spline trims
//! on cone/sphere/torus charts refuse typed naming that frontier (their
//! pcurves mint since M6-3; the trimmed-lane geometry for them is
//! unwritten). A **fitted** (rung-3) chart image still refuses typed
//! on every chart: the chord pass has no certified UV chord-step
//! bound for a fitted image (the boundary-tightening contract in
//! `crate::chords`), and its first genuine consumer is the
//! edge×NURBS-face boolean layer (the cut-loft unit).
//!
//! # The lattice metric
//!
//! Neither lane's CDT runs in raw UV. Delaunay is not
//! stretch-invariant, and a ruled wall's lattice is a strip whose
//! columns are many times narrower than its rows (a lofted circle's
//! wall: `s_u/s_v ≈ 0.03`; a cylinder of radius `r`, whose lattice is
//! square in 3-D, `≈ 1/r` in (θ, z)); there, any point off the lattice — a trim
//! chord point between two rows, a band interface's foreign column, a
//! refinement centroid — has an empty circle many columns wide and a
//! sliver across them. So points enter the CDT at `(u/k, v)`, with `k`
//! the lattice metric (`crate::nurbs_cert::lattice_metric`, a power of
//! two near the mean realized aspect: of the NURBS schedule's bands, or
//! of the cylinder lane's one uniform lattice), where the lattice is
//! near square. One face-wide `k` cannot square bands whose aspects
//! spread far apart, so the NURBS schedule also raises a band's rows
//! when its columns stay too narrow over `k`
//! ([`crate::nurbs_cert::NurbsCellGrid::band_schedule`], **Rows for
//! narrow-column bands**). The map is affine, so constraint segments
//! stay straight; and exact, so every coincidence in UV is one in the
//! CDT. Only the CDT sees it: `meta`, the creases, the polygon's
//! orientation and every certificate read UV.
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
//! step per round: `work/tess/lofted-rational-walls-refuse-at-some-deltas`).
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
use geom_brep::Pcurve;
use geom_core::Point3;
use spade::handles::FixedVertexHandle;
use spade::{ConstrainedDelaunayTriangulation, Point2 as SpadePoint, Triangulation};
use topo::{Body, FaceKey};

use crate::cert;
use crate::chords::ChordPass;
use crate::nurbs_cert::{FaceBounds, NurbsCellGrid, NurbsFaceBound, face_cells, lattice_metric};
use crate::planar::{Crossings, Pinches, Walk, classify_faces, edge_key, shoelace2};
use crate::sizing::{SizingTols, ceil_count, sagitta_step};
use crate::tessellate::{Patch, PatchVertex};
use crate::types::TessellateError;

/// Retry budget for the rebuild loop (module docs). **Termination.**
///
/// One round budget covers both retry kinds — the grid-on-constraint
/// drops and the certificate-driven refinement — and it is a
/// typed-refusal backstop rather than a tuning knob: exhausting it
/// raises [`TessellateError`], it never coarsens or accepts a mesh.
const MAX_GRID_RETRIES: usize = 6;

/// **The trimmed cylinder's unproved headroom**: the lane sizes its
/// lattice, and the chord pass chords this face's edges
/// (`chords::edge_target`), at this share of δ rather than at δ.
///
/// The lane has no worst-triangle theorem (`crates/mesh/README.md`
/// `[mesh.sizing]`). Its lattice cell-halves would certify at the
/// sagitta of the column step, but the trim polygon's points sit
/// anywhere in a cell, and a constrained Delaunay triangle beside a
/// trim chord can span more than one column — nothing yet bounds by
/// how much. The certificate refuses typed where it is exceeded; this
/// factor is what keeps that refusal rare, and it is a debt
/// (`work/tess/the-trimmed-cylinder-lane-has-no-worst-triangle-theorem.md`):
/// the fix is the theorem, not a retuned factor.
///
/// The sphere arm reads it the same way and for the same reason, at its
/// lattice steps ([`sphere_lattice_steps`]) and its edges' chords: its
/// cells are the curved lane's sphere cells, whose theorem
/// (`sizing::sphere_grid_steps`) does not cover a CDT triangle beside a
/// trim chord the lattice did not place
/// (`work/tess/the-trimmed-sphere-arm-has-no-worst-triangle-theorem.md`).
pub(crate) const UNPROVED_HEADROOM: f64 = 0.5;

/// **How near a pole a tilted circle may pass on the sphere arm**, as a
/// share of the lattice's row step `h_v`. **Structural** (it caps
/// allocation, as `sizing::MAX_COUNT` does).
///
/// A circle's image turns in azimuth at up to `ρ_c/(r·cos v)`, so the
/// uniform chord schedule that keeps its UV steps within the lattice's
/// (`chords::sphere_arm_tighten`) grows as the inverse of the arc's
/// closest approach to a pole. An arc one row from the pole asks about
/// as many chords as the lattice has columns per row; this share lets
/// it ask sixteen times that, and an arc nearer still refuses typed
/// rather than allocate a boundary finer than the face's whole lattice
/// (`work/tess/a-tilted-circle-near-a-sphere-pole-refuses-at-the-sphere-arm.md`).
pub(crate) const NEAR_POLE_ROW_SHARE: f64 = 1.0 / 16.0;

/// Whether face `fk` is meshed by one of this lane's two analytic arms,
/// the trimmed cylinder or the trimmed sphere: the faces whose lattice
/// and boundary chords size at [`UNPROVED_HEADROOM`]
/// (`chords::edge_target`).
pub(crate) fn sizes_at_headroom(body: &Body<f64>, fk: FaceKey) -> Result<bool, TessellateError> {
    let surface = crate::tessellate::face_surface(body, fk)?;
    Ok(match surface {
        Surface::Cylinder { .. } => {
            crate::tessellate::lane_of(body, fk, surface)? == crate::tessellate::Lane::Trimmed
        }
        Surface::Sphere { .. } => takes_sphere_arm(body, fk, surface)?,
        _ => false,
    })
}

/// Whether the sphere face `fk` is the sphere arm's: the dispatch routes
/// it here for a tilted circle ([`has_tilted_circle`]), and no conic or
/// B-spline carrier bounds it (those still refuse at the chart roster,
/// [`trim_frontier`]).
pub(crate) fn takes_sphere_arm(
    body: &Body<f64>,
    fk: FaceKey,
    surface: &Surface<f64>,
) -> Result<bool, TessellateError> {
    Ok(matches!(surface, Surface::Sphere { .. })
        && !has_trim_carrier(body, fk)?
        && has_tilted_circle(body, fk)?)
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

/// **The trimmed sphere's lattice steps** `(h_u, h_v)`, in radians of
/// azimuth and latitude: the curved lane's sphere steps
/// ([`crate::sizing::sphere_grid_steps`]) at [`UNPROVED_HEADROOM`]·δ,
/// over the face's own latitude window ([`sphere_widest_cos`]). The
/// chord pass reads the same steps to bound each boundary chord's UV
/// step (`chords::sphere_arm_tighten`), so the lattice and the trim
/// polygon are sized from one value.
pub(crate) fn sphere_lattice_steps(
    body: &Body<f64>,
    fk: FaceKey,
    surface: &Surface<f64>,
    delta: f64,
) -> Result<(f64, f64), TessellateError> {
    let Surface::Sphere {
        center,
        radius,
        axis,
        ..
    } = *surface
    else {
        return Err(TessellateError::MissingEntity {
            what: "sphere arm on a non-sphere surface",
        });
    };
    let widest = sphere_widest_cos(body, fk, center, axis, radius)?;
    #[cfg(test)]
    let scale = tests::SPHERE_STEP_SCALE.with(std::cell::Cell::get);
    #[cfg(not(test))]
    let scale = 1.0;
    let (hu, hv) = crate::sizing::sphere_grid_steps(delta * UNPROVED_HEADROOM, radius, widest);
    Ok((hu * scale, hv * scale))
}

/// The cosine of a trimmed sphere face's latitude nearest the equator,
/// from its boundary circles' own latitude ranges: each circle's height
/// along the axis is `c + a·cos t + b·sin t` in its parameter, whose
/// range over the edge's interval is closed form ([`harmonic_range`]).
/// A tilted circle's latitude is not monotone between its ends, so the
/// vertices alone (the curved lane's `sizing::sphere_face_widest_cos`)
/// do not bound it.
fn sphere_widest_cos(
    body: &Body<f64>,
    fk: FaceKey,
    center: Point3<f64>,
    axis: geom_core::Vec3<f64>,
    radius: f64,
) -> Result<f64, TessellateError> {
    let face = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for (ek, _) in crate::walk::loop_edges(body, face.outer, fk)? {
        let curve = body
            .get_edge(ek)
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(|g| g.certified())
            .ok_or(TessellateError::MissingEntity { what: "edge curve" })?;
        let Curve3::Circle { center: cc, .. } = *curve.carrier() else {
            return Err(TessellateError::UnsupportedCurve {
                edge: ek,
                note: "a trimmed sphere face bounded by an edge that is not a circle — \
                       the sphere arm sizes its lattice from its circles' latitude ranges",
            });
        };
        let height = |p: Point3<f64>| (p - center).dot(axis);
        let c = height(cc);
        let a = height(curve.carrier().eval(0.0)) - c;
        let b = height(curve.carrier().eval(core::f64::consts::FRAC_PI_2)) - c;
        let (z0, z1) = harmonic_range(c, a, b, curve.params());
        lo = lo.min(z0);
        hi = hi.max(z1);
    }
    if lo.is_nan() || hi.is_nan() || lo > hi {
        return Err(TessellateError::MissingEntity {
            what: "trimmed sphere face latitude window",
        });
    }
    Ok(if lo <= 0.0 && hi >= 0.0 {
        1.0
    } else {
        let s = (lo.abs().min(hi.abs()) / radius).min(1.0);
        (1.0 - s.powi(2)).sqrt()
    })
}

/// The range of `c + a·cos t + b·sin t` over `[t0, t1]`: the ends, and
/// `c ± √(a² + b²)` where the interval may hold the phase of either
/// extremum ([`geom::periodic_window_may_hold`]).
pub(crate) fn harmonic_range(c: f64, a: f64, b: f64, (t0, t1): (f64, f64)) -> (f64, f64) {
    use core::f64::consts::{PI, TAU};
    let at = |t: f64| c + a * t.cos() + b * t.sin();
    let (e0, e1) = (at(t0), at(t1));
    let (mut lo, mut hi) = (e0.min(e1), e0.max(e1));
    let (m, tau) = (a.hypot(b), b.atan2(a));
    if geom::periodic_window_may_hold((tau, tau), (t0, t1), TAU) {
        hi = c + m;
    }
    if geom::periodic_window_may_hold((tau + PI, tau + PI), (t0, t1), TAU) {
        lo = c - m;
    }
    (lo, hi)
}

/// The per-chart data of the constructing arms (module docs):
/// which certificate each emitted triangle checks, and which pcurve
/// forms the trim walk accepts.
enum Lane {
    /// The M5 PR 11 cylinder lane (harmonic pcurves, radial-convexity
    /// certificate).
    Cylinder {
        origin: Point3<f64>,
        axis: geom_core::Vec3<f64>,
        radius: f64,
    },
    /// The sphere arm (harmonic and projected pcurves, the ball-convexity
    /// certificate `cert::cert_sphere`), with its lattice steps
    /// ([`sphere_lattice_steps`]).
    Sphere {
        center: Point3<f64>,
        radius: f64,
        steps: (f64, f64),
    },
    /// The M7 trimmed-NURBS lane (closed-form pcurve images, Hessian
    /// interpolation certificate — `crate::nurbs_cert`; since
    /// TESS-SPAN certified per knot-span cell and sized per v-band,
    /// the whole-patch bound kept beside the cell grid for the budget
    /// meter's columns).
    Nurbs {
        grid: NurbsCellGrid,
        patch: NurbsFaceBound,
    },
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
            Curve3::Ellipse { .. } | Curve3::Nurbs(_) | Curve3::Spiric { .. }
        ) {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Tessellates one trimmed curved face (module docs): pcurve-driven UV
/// polygon, interior grid, CDT with even-odd classification, cylinder
/// certificates.
pub(crate) fn tessellate_trimmed(
    body: &Body<f64>,
    fk: FaceKey,
    surface: &Surface<f64>,
    chords: &ChordPass,
    shared: &[Point3<f64>],
    tol: &SizingTols,
    bounds: &FaceBounds,
) -> Result<Patch, TessellateError> {
    let face = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;
    if !face.rings.is_empty() {
        return Err(TessellateError::RingOnCurvedFace { face: fk });
    }
    let lane = match *surface {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => Lane::Cylinder {
            origin,
            axis,
            radius,
        },
        // The mvfs "no description yet" state — the historical
        // refusal, kept for exactly this class (types docs). An
        // approximating surface has no such state to screen: its fit is
        // certified by construction.
        Surface::Nurbs(ref payload) if payload.is_placeholder() => {
            return Err(TessellateError::UnsupportedSurface { face: fk });
        }
        // ONE spline arm past that screen, for the described NURBS face
        // and for the approximating surface meshed on its fit: the lane
        // reads the cell table under this FACE's key, not off the
        // surface, and `chords::nurbs_tighten` assembled it from the
        // same `Surface::spline_chart()` this dispatch is testing.
        //
        // Both readings off ONE assembly: the memo holds the cell table
        // (`nurbs_cert::face_cells`), and the whole-patch bound is a
        // reading of it. The chord pass filled the entry, so the cost
        // here is a clone of the table, never an assembly.
        Surface::Nurbs(_) | Surface::Approx(_) => {
            let grid = face_cells(bounds, fk)?.clone();
            let patch = grid.patch();
            Lane::Nurbs { grid, patch }
        }
        Surface::Sphere { center, radius, .. } if takes_sphere_arm(body, fk, surface)? => {
            Lane::Sphere {
                center,
                radius,
                steps: sphere_lattice_steps(body, fk, surface, tol.delta)?,
            }
        }
        _ => return Err(trim_frontier(body, fk, face.outer)?),
    };
    let creases = match (&lane, surface.spline_chart()) {
        (Lane::Nurbs { grid, .. }, Some(chart)) => Creases::of(grid, chart, fk)?,
        _ => None,
    };
    // The UV polygon: per half-edge, the pcurve evaluated at the
    // shared chord parameters (each traversal contributes all but its
    // last point; ids stay the shared 3-D chord ids).
    let nurbs_chart = matches!(lane, Lane::Nurbs { .. });
    let poles = if matches!(lane, Lane::Sphere { .. }) {
        ChartPoles::Kept
    } else {
        ChartPoles::None
    };
    let polygon = trim_polygon(
        body,
        fk,
        face.outer,
        chords,
        nurbs_chart,
        creases.as_ref(),
        poles,
    )?;
    if polygon.len() < 3 {
        return Err(TessellateError::MissingEntity {
            what: "degenerate trimmed boundary",
        });
    }
    // THE ONE CHART WITH POLES is the sphere's, and its polygon is the
    // shape `sphere_polygon_shape` admits, so the duplicate-id skip in
    // the emit pass never collapses a pole fan (#678). Every other arm's
    // chart has none — `Chart::poles()` is empty for a cylinder and a
    // NURBS face has no `Chart` at all — and that fact is `walk`'s, so
    // it is asserted here rather than written down (D2 addendum row 5).
    if matches!(lane, Lane::Sphere { .. }) {
        sphere_polygon_shape(body, fk, face.outer, &polygon)?;
    } else {
        debug_assert!(
            crate::walk::Chart::of(surface).is_none_or(|c| c.poles().is_empty()),
            "face {fk:?}: the trimmed lane took a chart with a pole outside its sphere \
             arm, so the duplicate-id skip in the emit pass may be collapsing a pole fan \
             — see curved::pole_columns, issue #678"
        );
    }

    // Grid sizing: cylinder — sagitta-tight in u at
    // `UNPROVED_HEADROOM`·δ, rows every r·hu metres in v, a uniform
    // grid over the trim box; NURBS — the Hessian-budget steps
    // (`nurbs_cert`) at the lane's lattice budget
    // (`nurbs_cert::lattice_budget`), applied PER KNOT-SPAN CELL from
    // each cell's own certified bound (TESS-SPAN), grid lines on the
    // cell boundaries. The chord pass's boundary tightening keeps
    // the whole-patch steps (the reported D-2 choice — `nurbs_cert`
    // module docs).
    let (mut u0, mut u1, mut v0, mut v1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
    for &(u, v, _) in &polygon {
        u0 = u0.min(u);
        u1 = u1.max(u);
        v0 = v0.min(v);
        v1 = v1.max(v);
    }
    let (mut candidates, nurbs_stats, metric) = match lane {
        Lane::Cylinder { radius, .. } => {
            let hu = sagitta_step(tol.delta * UNPROVED_HEADROOM, radius);
            let nu = ceil_count(u1 - u0, hu)?;
            let nv = ceil_count(v1 - v0, radius * hu)?;
            // The other half of #678's shape, checked where the count
            // is: the fan needs ONE interior column between two
            // boundary entries that share a mesh vertex, which is
            // `nu == 2`. The only repeated id at two DISTINCT UVs in
            // this lane is the full-2π seam double-traversal, and
            // there `u1 - u0` is 2π against an `hu` the sagitta cap
            // holds at ≤ π/4 — so the two never meet. Arithmetic, not
            // an argument, and this is the line that says so.
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
                nu != 2 || !id_repeats_apart(&polygon),
                "face {fk:?}: a trim polygon repeats a mesh id at two UV locations \
                 with a single interior column (nu = 2, nv = {nv}) — the shape \
                 #678's non-manifold fan needs (curved::pole_columns)"
            );
            // The lattice is square in 3-D (`r·hu` by `hu` radians), so
            // in (θ, v) it is a strip of aspect ~1/r: its CDT runs in the
            // same lattice metric as the NURBS lane's, from its one
            // lattice's realized spacing.
            #[allow(clippy::cast_precision_loss)]
            let aspect = ((u1 - u0) / nu as f64) / ((v1 - v0) / nv as f64);
            (
                uniform_candidates((u0, u1), (v0, v1), nu, nv),
                None,
                lattice_metric(std::iter::once(aspect)),
            )
        }
        // The sphere: the steps the chord pass bounded this face's
        // boundary UV steps by (`sphere_lattice_steps`), over the trim
        // box. The CDT runs in the chart's own metric at the box's mid
        // latitude, `(u·cos v, v)` to the nearest power of two: there a
        // small face's boundary circle is near round, so every ear on it
        // has the face's interior points in its circumcircle and the CDT
        // cuts none. An ear on a boundary chain is the triangle of three
        // consecutive chord points, which the neighbour across that chain
        // may cut too: the two would be one triangle twice, and their
        // diagonal an edge of four.
        Lane::Sphere {
            steps: (hu, hv), ..
        } => (
            uniform_candidates(
                (u0, u1),
                (v0, v1),
                pole_side_columns(&polygon, (u0, u1), ceil_count(u1 - u0, hu)?)?,
                ceil_count(v1 - v0, hv)?,
            ),
            None,
            lattice_metric(std::iter::once(1.0 / (0.5 * (v0 + v1)).cos())),
        ),
        Lane::Nurbs { ref grid, .. } => {
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
        metric,
        surface,
        lane: &lane,
        polygon: &polygon,
        creases: creases.as_ref(),
        shared,
        tol,
    };
    for attempt in 0..=MAX_GRID_RETRIES {
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
        && let (Some(stats), Lane::Nurbs { grid, patch }) = (nurbs_stats, &lane)
    {
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
    /// The lattice metric `k`: points enter the CDT at `(u/k, v)`,
    /// scaling the axis that shrinks ([`Self::cdt_point`]).
    metric: f64,
    surface: &'a Surface<f64>,
    lane: &'a Lane,
    polygon: &'a [(f64, f64, u32)],
    creases: Option<&'a Creases>,
    shared: &'a [Point3<f64>],
    tol: &'a SizingTols,
}

/// One attempt's triangulation, between inserting its points and
/// emitting its triangles.
struct Cdt {
    /// The triangulation, in the lattice metric
    /// ([`TrimmedFace::cdt_point`]).
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
}

impl TrimmedFace<'_> {
    /// `(u, v)` in the lattice metric, `(u/k, v)` up to a uniform scale
    /// (which Delaunay ignores): the axis that SHRINKS is scaled, `u`
    /// by `1/k` when `k > 1`, `v` by `k` when `k < 1`, so the
    /// coordinates stay within the chart's own magnitude. `k` is a power
    /// of two, so the product is exact and every coincidence in UV is
    /// one in the CDT. Only the CDT sees these: `meta`, the creases,
    /// the polygon's orientation and every certificate read UV.
    fn cdt_point(&self, u: f64, v: f64) -> SpadePoint<f64> {
        if self.metric > 1.0 {
            SpadePoint::new(u / self.metric, v)
        } else {
            SpadePoint::new(u, v * self.metric)
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
            if attempt == MAX_GRID_RETRIES {
                return Err(TessellateError::Triangulation { face: fk });
            }
            return Ok(Next::Rebuild);
        }
        let Emitted {
            interior,
            triangles,
            refine,
        } = self.emit(&at, &crossings, candidates, measure, falsification)?;
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
        if !refine.is_empty() && !measure.worst.is_nan() && attempt < MAX_GRID_RETRIES {
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
        // cylinder's full-2π seam double-traversal and the sphere arm's
        // pole sides, so on every other face the set is empty and the
        // census returns on one branch.
        let patch = Patch {
            interior,
            triangles,
        };
        #[cfg(debug_assertions)]
        {
            let identified =
                crate::walk::ids_at_two_uvs(self.polygon.iter().map(|&(u, v, id)| (u, v, id)));
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
        let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
            ConstrainedDelaunayTriangulation::new();
        let mut meta: Vec<(f64, f64, Slot)> = Vec::new();
        let mut handles = Vec::with_capacity(self.polygon.len());
        let mut on_crease = Vec::new();
        let mut walk = Walk::new();
        for &(u, v, id) in self.polygon {
            let Ok(h) = cdt.insert(self.cdt_point(u, v)) else {
                return Err(TessellateError::Triangulation { face: fk });
            };
            walk.enter(&mut meta, h, (u, v, Slot::Boundary(id)), id);
            handles.push(h);
            if let Some(c) = self.creases {
                c.note(&mut on_crease, (u, v), h);
            }
        }
        let pinches = Pinches::of(&walk, fk);
        for (k, &(u, v)) in candidates.iter().enumerate() {
            if dropped.contains(&k) {
                continue;
            }
            let Ok(h) = cdt.insert(self.cdt_point(u, v)) else {
                return Err(TessellateError::Triangulation { face: fk });
            };
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
            let (a, b) = (handles[i], handles[(i + 1) % handles.len()]);
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
                pinches.note_side(e, (a, i), (b, (i + 1) % handles.len()))?;
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
        // Even-odd interior flags, then emission with certificates.
        let inside = classify_faces(cdt, crossings);
        let flip = {
            let poly2: Vec<[f64; 2]> = self.polygon.iter().map(|&(u, v, _)| [u, v]).collect();
            shoelace2(&poly2) < 0.0
        };
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
        let dev_samples_per_edge = if matches!(self.lane, Lane::Nurbs { .. }) {
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
        let refine_lane = matches!(self.lane, Lane::Nurbs { .. });
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
            // there it collapses a pole fan (#678). On the sphere arm
            // it does the same at a pole side; elsewhere the drop is all
            // it is, because neither ingredient of that fan reaches the
            // lane — no chart singularity, and no repeated id at two UV
            // locations with a single interior column between them.
            //
            // WHAT IS ASSERTED, AND WHERE THE COVER STOPS. The chart
            // half is checked at the lane choice, which runs for every
            // arm. The INGREDIENT half — a repeated id with a single
            // interior column between the entries — is checked at the
            // cylinder arm's `nu`, kept off by the sphere arm's
            // `pole_side_columns`, and this emit pass also runs for
            // NURBS, where there is no `nu`: the candidates come from
            // the cell grid rather than a uniform division, so the
            // "single interior column" has no analogue to test. That
            // arm's ingredient check is therefore still an ARGUMENT
            // and not a check.
            //
            // ITS EMISSION IS NOT (issue 897). The census after this
            // loop re-derives the CONCLUSION over the emitted patch,
            // and it needs no `nu` — it counts uses of the edges
            // incident to a repeated id, which both arms have. So the
            // NURBS arm, which had neither half, now has the half that
            // actually observes the defect.
            if ids[0] == ids[1] || ids[1] == ids[2] || ids[0] == ids[2] {
                continue; // boundary-degenerate sliver
            }
            let tri = [
                ids[0].position(shared, &interior),
                ids[1].position(shared, &interior),
                ids[2].position(shared, &interior),
            ];
            let (bound, scale) = match *self.lane {
                Lane::Cylinder {
                    origin,
                    axis,
                    radius,
                } => (
                    cert::cert_cylinder(origin, axis, radius, tri),
                    cert::position_scale(origin, tri),
                ),
                Lane::Sphere { center, radius, .. } => (
                    cert::cert_sphere(center, radius, tri),
                    cert::position_scale(center, tri),
                ),
                // TESS-SPAN: the certificate of the cell(s) covering
                // the triangle's UV box — one band's cells in `v`, since
                // rows land on the `v` cuts, but possibly several in `u`,
                // whose columns do not (`NurbsCellGrid::cert` argues the
                // straddling and half-open boundary cases).
                Lane::Nurbs { ref grid, .. } => {
                    let bound = grid.cert(uv);
                    (bound, cert::uv_scale(bound, uv))
                }
            };
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
                refine.push((
                    (uv[0][0] + uv[1][0] + uv[2][0]) / 3.0,
                    (uv[0][1] + uv[1][1] + uv[2][1]) / 3.0,
                ));
            }
            triangles.push(if flip { [ids[0], ids[2], ids[1]] } else { ids });
        }

        Ok(Emitted {
            interior,
            triangles,
            refine,
        })
    }
}

/// Whether any mesh id appears at two DIFFERENT UV locations in the
/// trim polygon — the second ingredient of #678's non-manifold fan
/// (`curved::pole_columns` carries the argument; the first ingredient
/// is a single interior column between them).
///
/// The rule for "different" is spade's and is stated once, at
/// [`crate::walk::ids_at_two_uvs`], which both lanes call. This
/// function is that set's emptiness, named for the question the
/// cylinder arm's `debug_assert` asks.
fn id_repeats_apart(polygon: &[(f64, f64, u32)]) -> bool {
    !crate::walk::ids_at_two_uvs(polygon.iter().copied()).is_empty()
}

/// **The sphere arm's poles.** A pole the loop passes through is a
/// vertex where two meridians meet, and the polygon holds it twice, at
/// the azimuth of each ([`trim_polygon`]'s [`ChartPoles::Kept`]): the
/// pole is a side `v = ±π/2` of the polygon, as it is of the curved
/// lane's walk. A CDT triangle on two of that side's points has two
/// corners on the one pole vertex and the emit pass drops it, so the
/// triangles left at the pole are a fan about it, each certified in 3-D
/// by `cert::cert_sphere`. The fan is manifold unless exactly one
/// lattice column lies between the side's ends (#678's shape), which
/// [`pole_side_columns`] keeps the lattice off.
///
/// Refused typed, each a shape this arm does not mesh:
/// - a point past a pole (`|v| > π/2` beyond rounding): an edge whose
///   image runs through a pole mid-edge;
/// - a side other than a pole side turning more than 3π/2 in azimuth:
///   the loop winds about a pole inside the face (its image does not
///   close in the chart);
/// - a mesh id at two UV locations off a pole: a seam traversed twice.
fn sphere_polygon_shape(
    body: &Body<f64>,
    fk: FaceKey,
    lk: topo::LoopKey,
    polygon: &[(f64, f64, u32)],
) -> Result<(), TessellateError> {
    use core::f64::consts::{FRAC_PI_2, PI};
    let refuse = |note| -> Result<(), TessellateError> {
        let edge = crate::walk::loop_half_edges(body, lk, fk)?
            .into_iter()
            .find(|&(hek, _, _)| {
                body.pcurve(hek)
                    .is_some_and(|c| matches!(c.pcurve(), Pcurve::Projected(_)))
            })
            .map(|(_, ek, _)| ek)
            .ok_or(TessellateError::MissingEntity {
                what: "sphere arm face without a tilted circle",
            })?;
        Err(TessellateError::UnsupportedCurve { edge, note })
    };
    if polygon
        .iter()
        .any(|&(_, v, _)| v.abs() > FRAC_PI_2 * (1.0 + 4.0 * f64::EPSILON))
    {
        return refuse(
            "trimmed sphere face whose boundary image runs past a chart pole mid-edge — \
             the sphere arm meets a pole only at a loop vertex",
        );
    }
    let n = polygon.len();
    if (0..n).any(|i| {
        let (a, b) = (polygon[i], polygon[(i + 1) % n]);
        !(a.2 == b.2 && at_pole(a.1)) && (b.0 - a.0).abs() > 1.5 * PI
    }) {
        return refuse(
            "trimmed sphere face whose loop winds about a chart pole — the sphere arm \
             meshes a face that holds no pole inside it \
             (`work/tess/a-trimmed-sphere-face-holding-a-pole-inside-it-refuses.md`)",
        );
    }
    let apart = crate::walk::ids_at_two_uvs(polygon.iter().copied());
    if polygon
        .iter()
        .any(|&(_, v, id)| apart.contains(&id) && !at_pole(v))
    {
        return refuse(
            "trimmed sphere face whose loop visits one mesh vertex at two chart \
             locations off a pole (a seam traversed twice), which the sphere arm \
             does not mesh",
        );
    }
    Ok(())
}

/// The sphere arm's column count over the trim box `(u0, u1)`: `nu`,
/// doubled until no pole side of `polygon` has exactly one lattice
/// column strictly between its ends — the shape #678's non-manifold fan
/// needs (`curved::pole_columns` carries the argument). Doubling keeps
/// every column it had, so a side's count only grows, and one past one
/// is two.
fn pole_side_columns(
    polygon: &[(f64, f64, u32)],
    (u0, u1): (f64, f64),
    mut nu: usize,
) -> Result<usize, TessellateError> {
    let n = polygon.len();
    let sides: Vec<(f64, f64)> = (0..n)
        .map(|i| (polygon[i], polygon[(i + 1) % n]))
        .filter(|&(a, b)| a.2 == b.2 && at_pole(a.1) && a.0 != b.0)
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

/// The uniform interior grid candidates of the cylinder lane: the trim
/// box divided `nu × nv`, interior points only (the box boundary is at
/// or outside the trim polygon), in the historical arithmetic —
/// positions are bit-identical to the pre-TESS-SPAN grid. Already
/// generated in (v, u) row-major order; sorted anyway so both lanes
/// hand the retry loop the same invariant.
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

/// The typed frontier refusal for a trimmed face on a chart whose
/// pcurves do not mint: names the offending edge and the REAL blocker.
fn trim_frontier(
    body: &Body<f64>,
    fk: FaceKey,
    lk: topo::LoopKey,
) -> Result<TessellateError, TessellateError> {
    for (ek, _) in crate::walk::loop_edges(body, lk, fk)? {
        let curve = body
            .get_edge(ek)
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(|g| g.certified())
            .ok_or(TessellateError::MissingEntity { what: "edge curve" })?;
        if matches!(
            curve.carrier(),
            Curve3::Ellipse { .. } | Curve3::Nurbs(_) | Curve3::Spiric { .. }
        ) {
            return Ok(TessellateError::UnsupportedCurve {
                edge: ek,
                note: "conic/B-spline/spiric trim on a plane/cone/sphere/torus chart — \
                       the only trimmed-face tessellation lanes written are the \
                       cylinder chart's, the NURBS chart's and the sphere chart's for \
                       circle-bounded faces; the remaining analytic \
                       charts have no trimmed lane (the spiric-bounded torus wall and \
                       plane cap of a hollowed partial revolve land here: \
                       `work/issues/trimmed-tessellation-lacks-torus-and-plane-arms.md`)",
            });
        }
    }
    Ok(TessellateError::MissingEntity {
        what: "trimmed router fired without a trim carrier",
    })
}

/// The pcurve-driven UV polygon of the face's outer loop: per
/// half-edge, `pcurve.eval` at the shared chord parameters (module
/// docs; each traversal contributes all but its last point, and on a
/// chart with poles ([`ChartPoles::Kept`]) its last point too where that
/// is a pole the next traversal leaves from another azimuth, so the pole
/// is a side of the polygon — [`sphere_polygon_shape`]).
/// `nurbs_chart` widens the accepted image forms to `IsoLine`,
/// `IsoArc` and `General`; `Fitted` refuses typed on every chart
/// (module docs). It is the SPLINE-chart flag, not a `Surface::Nurbs`
/// test: the caller sets it from `Lane::Nurbs`, which an
/// approximating surface takes too (it meshes on its fit, and the fit
/// is a NURBS chart), so every "NURBS chart" below means "NURBS or
/// approximating chart".
fn trim_polygon(
    body: &Body<f64>,
    fk: FaceKey,
    lk: topo::LoopKey,
    chords: &ChordPass,
    nurbs_chart: bool,
    creases: Option<&Creases>,
    poles: ChartPoles,
) -> Result<Vec<(f64, f64, u32)>, TessellateError> {
    let lp = body
        .get_loop(lk)
        .ok_or(TessellateError::MissingEntity { what: "loop" })?;
    let topo::LoopBoundary::Cycle { first } = lp.boundary else {
        return Err(TessellateError::EmptyLoop { face: fk });
    };
    let cycle = body
        .loop_cycle(first)
        .ok_or(TessellateError::MissingEntity { what: "loop cycle" })?;
    // The loop's lift (`topo::Body::loop_lift`), by half-edge: each
    // image where the loop's chain places it. A loop with no lift reads
    // as rowless at its every half-edge, and the walk refuses at the
    // first.
    let mut lifted: HashMap<topo::HalfEdgeKey, Pcurve<f64>> = body
        .loop_lift(lk)
        .map(|rows| {
            rows.into_iter()
                .map(|row| (row.half_edge, row.pcurve))
                .collect()
        })
        .unwrap_or_default();
    let mut travs: Vec<Vec<(f64, f64, u32)>> = Vec::with_capacity(cycle.len());
    for hek in cycle {
        let image = lifted.remove(&hek);
        let he = body
            .get_half_edge(hek)
            .ok_or(TessellateError::MissingEntity { what: "half-edge" })?;
        let edge = body
            .get_edge(he.edge)
            .ok_or(TessellateError::MissingEntity { what: "edge" })?;
        let forward = edge.he_plus == hek;
        let Some(image) = image else {
            return Err(TessellateError::UnsupportedCurve {
                edge: he.edge,
                note: "trimmed face half-edge carries no stored pcurve cache — caches \
                       mint in the split/boolean pipelines",
            });
        };
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
        match &image {
            Pcurve::Harmonic { .. } => {}
            // The spiric's two images are closed forms of the
            // carrier's own parameter on every chart it lives on,
            // so the walk below — which samples the image at the
            // chord pass's parameters — reads them exactly as it reads
            // a harmonic one. Unreachable at this head for a different
            // reason: `tessellate_trimmed`'s chart roster has no torus
            // or plane arm, so a spiric-bounded face refuses before
            // this walk (`work/issues/trimmed-tessellation-lacks-torus-and-plane-arms.md`).
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
            // (`crate::chords::general_uv_speeds`).
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
    }
    let mut out = Vec::new();
    for (k, trav) in travs.iter().enumerate() {
        let (&last, rest) = trav.split_last().ok_or(TessellateError::MissingEntity {
            what: "edge chords",
        })?;
        out.extend_from_slice(rest);
        let next = travs[(k + 1) % travs.len()][0];
        #[allow(clippy::float_cmp)]
        if poles == ChartPoles::Kept
            && last.2 == next.2
            && at_pole(last.1)
            && (last.0, last.1) != (next.0, next.1)
        {
            out.push(last);
        }
    }
    Ok(out)
}

/// Whether [`trim_polygon`] keeps a pole the loop passes as a side of
/// the polygon: the sphere arm's chart has poles, and no other arm's
/// does.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ChartPoles {
    Kept,
    None,
}

/// Whether a sphere chart latitude is a pole's, to the rounding of the
/// chart's own evaluation: a meridian's image reaches `±π/2` at the
/// pole vertex within a few ulps.
fn at_pole(v: f64) -> bool {
    core::f64::consts::FRAC_PI_2 - v.abs() <= 64.0 * f64::EPSILON
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
/// along** (`work/tess/lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`).
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
mod tests {
    use super::{
        ChartPoles, ChordPass, Creases, FixedVertexHandle, SpadePoint, TessellateError,
        chain_creases, id_repeats_apart, trim_polygon,
    };
    use spade::{ConstrainedDelaunayTriangulation, Triangulation};
    use topo::FaceKey;

    thread_local! {
        /// A factor on the sphere arm's lattice steps, 1 in every
        /// shipped build: the end-to-end refusal row coarsens the arm
        /// with it to reach `CertificateExceeded` through the real call
        /// path. `cfg(test)` only.
        pub(crate) static SPHERE_STEP_SCALE: std::cell::Cell<f64> =
            const { std::cell::Cell::new(1.0) };
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
            if !matches!(curve.carrier(), geom::Curve3::Circle { .. }) {
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
        for (fk, face) in body.faces() {
            let surface = body.get_surface(face.surface).unwrap();
            let Some(chart) = surface.spline_chart() else {
                continue;
            };
            let grid = crate::nurbs_cert::face_cells(&bounds, fk).unwrap();
            assert_eq!(grid.creases(), (&[0.5][..], &[][..]), "face {fk:?}");
            let c = Creases::of(grid, chart, fk).unwrap();
            let polygon = trim_polygon(
                &body,
                fk,
                face.outer,
                &chords,
                true,
                c.as_ref(),
                ChartPoles::None,
            )
            .unwrap();
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
    /// spelling gets backwards, and the reason this compares `f64`s.
    #[test]
    fn a_repeat_at_one_uv_is_not_a_repeat_apart() {
        let same = [(0.0, 0.0, 7u32), (1.0, 0.5, 8), (0.0, 0.0, 7)];
        assert!(!id_repeats_apart(&same), "one location, one CDT vertex");

        let apart = [(0.0, 0.0, 7u32), (1.0, 0.5, 8), (2.0, 0.0, 7)];
        assert!(id_repeats_apart(&apart), "the seam double-traversal shape");

        // An ulp at SEAM magnitude (2π), not at zero: an ulp of 0.0 is
        // 5e-324, a subnormal no chart coordinate reaches, and pinning
        // there would pin the wrong scale.
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
    /// the arm's lattice steps `(h_u, h_v)` (`chords::sphere_arm_tighten`
    /// on `chords::sphere_uv_speeds`), and on the worst chord it is more
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
            let surface = body.get_surface(face.surface).unwrap();
            if !super::takes_sphere_arm(&body, fk, surface).unwrap() {
                continue;
            }
            faces += 1;
            let (hu, hv) = super::sphere_lattice_steps(&body, fk, surface, delta).unwrap();
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

    /// **An over-δ triangle on the sphere arm refuses typed through
    /// `crate::tessellate`.** The arm's lattice and boundary steps are
    /// scaled four-fold (`SPHERE_STEP_SCALE`, test-only), so its cells
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
            SPHERE_STEP_SCALE.with(|s| s.set(4.0));
            let coarse = crate::tessellate(&body, 1e-2, Tol::witness());
            SPHERE_STEP_SCALE.with(|s| s.set(1.0));
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
        crate::validate::check_volume(&mesh, volume, 1e-2).expect("and it encloses the body");
    }
}
