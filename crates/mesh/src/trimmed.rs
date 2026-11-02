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
//! ([`sphere_polygon_shape`]), and each run of the loop between two
//! poles is placed a whole turn where its equal-area shoelace says the
//! face is ([`choose_pole_azimuths`]). A face holding a pole inside it,
//! a loop whose runs between poles cannot be placed, and a tilted circle
//! passing so near a pole that its chord schedule asks one edge for more
//! than the arm's budget ([`MAX_SPHERE_EDGE_CHORDS`],
//! `chords::sphere_arm_tighten`), refuse typed
//! ([`TessellateError::SphereTrimShape`]). On both analytic arms a trim
//! polygon wound against the face refuses typed
//! ([`TessellateError::TrimLoopOrientation`]), a triangle folded over
//! the chord beside it is dropped and rebuilt or refuses
//! ([`TessellateError::FoldedTriangle`]), and a triangle on three
//! consecutive boundary points (an ear of a chain the neighbour may cut
//! too) is split by its centroid and rebuilt, or refuses
//! ([`TessellateError::BoundaryEar`]). Conic and B-spline trims
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
//! No arm's CDT runs in raw UV. Delaunay is not
//! stretch-invariant, and a ruled wall's lattice is a strip whose
//! columns are many times narrower than its rows (a lofted circle's
//! wall: `s_u/s_v ≈ 0.03`; a cylinder of radius `r`, whose lattice is
//! square in 3-D, `≈ 1/r` in (θ, z)); there, any point off the lattice — a trim
//! chord point between two rows, a band interface's foreign column, a
//! refinement centroid — has an empty circle many columns wide and a
//! sliver across them. So points enter the CDT at `(u/k, v)`, with `k`
//! the lattice metric (`crate::nurbs_cert::lattice_metric`, a power of
//! two near the mean realized aspect: of the NURBS schedule's bands, or
//! of the cylinder arm's one uniform lattice; on the sphere arm the
//! chart's own metric at the trim box's mid latitude, `1/cos v_mid`),
//! where the lattice is near square. One face-wide `k` cannot square
//! bands whose aspects
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
use topo::{Body, EdgeKey, FaceKey};

use crate::cert;
use crate::chords::ChordPass;
use crate::nurbs_cert::{FaceBounds, NurbsCellGrid, NurbsFaceBound, face_cells, lattice_metric};
use crate::planar::{Crossings, Pinches, Walk, classify_faces, edge_key, shoelace2};
use crate::sizing::{SizingTols, ceil_count, sagitta_step};
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
/// (`work/tesssize/the-trimmed-sphere-arm-has-no-worst-triangle-theorem.md`).
pub(crate) const UNPROVED_HEADROOM: f64 = 0.5;

/// The most pole junctions a sphere arm loop may pass for
/// [`choose_pole_azimuths`] to try every `2π` placement of the runs
/// between them: `3^(J−1)` placements, 27 at four. A half-band remnant
/// passes two; a loop past four refuses typed rather than search.
/// **Structural** (it bounds a search, as `sizing::MAX_COUNT` bounds an
/// allocation).
const MAX_POLE_JUNCTIONS: usize = 4;

/// **The sphere arm's per-edge chord budget**, `2^16`. **Work**: it
/// bounds what one face's CDT is asked to hold, as `sizing::MAX_COUNT`
/// bounds an allocation. A tilted circle passing a chart pole at polar
/// distance `d` asks about `ρ_c·Δt/(r·d·h_u)` chords of its edge
/// (`chords::sphere_arm_tighten`), and every one enters the face's CDT.
/// Measured in release on the tilted pair of unit balls (PR 5920's
/// second fix pass): 3.7·10⁴ boundary points mesh in 0.2–0.3 s, and
/// 5.7·10⁵ take 18–62 s per attempt, ending folded either way. The
/// budget keeps one face's boundary under about 10⁵ points, about a
/// second; an edge asking more refuses `SphereTrimShape { ChordBudget }`
/// at once. It is a count, not a share of the lattice, so a coarser δ
/// (which asks fewer chords) never refuses where a finer one meshes. A
/// circle far from the poles reaches it only at a δ about `10⁻⁹·r`.
pub(crate) const MAX_SPHERE_EDGE_CHORDS: usize = 1 << 16;

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
/// over the face's own latitude window
/// ([`crate::sizing::sphere_face_widest_cos`]). The
/// chord pass reads the same steps to bound each boundary chord's UV
/// step (`chords::sphere_arm_tighten`), so the lattice and the trim
/// polygon are sized from one value.
pub(crate) fn sphere_lattice_steps(
    body: &Body<f64>,
    fk: FaceKey,
    surface: &Surface<f64>,
    delta: f64,
) -> Result<(f64, f64), TessellateError> {
    let Surface::Sphere { radius, .. } = *surface else {
        return Err(TessellateError::MissingEntity {
            what: "sphere arm on a non-sphere surface",
        });
    };
    let widest = crate::sizing::sphere_face_widest_cos(body, fk, surface)?;
    #[cfg(test)]
    let scale = tests::SPHERE_STEP_SCALE.with(std::cell::Cell::get);
    #[cfg(not(test))]
    let scale = 1.0;
    let (hu, hv) = crate::sizing::sphere_grid_steps(delta * UNPROVED_HEADROOM, radius, widest);
    Ok((hu * scale, hv * scale))
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
/// polygon, interior grid, CDT with even-odd classification, and the
/// arm's certificate per triangle (`cert::cert_cylinder`,
/// `cert::cert_sphere`, or the NURBS cell bound).
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
        ChartPoles::Kept { sense: face.sense }
    } else {
        ChartPoles::None
    };
    let (polygon, polygon_edges) = trim_polygon(
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
    // THE ORIENTATION THE LOOP PROMISES, checked rather than read. The
    // loop runs interior-left about the face's outward normal, and an
    // analytic chart's normal `∂u × ∂v` is the surface's outward one,
    // so the polygon runs counter-clockwise in the chart exactly when
    // `sense` is true. The emit pass winds its triangles by the
    // polygon's own shoelace; a polygon that contradicts the sense
    // bounds the face's complement, and winding by it would ship that
    // complement as a closed manifold mesh — so it refuses here. A
    // spline chart's winding is not checked: the NURBS arm keeps the
    // emit pass's reading.
    if !nurbs_chart {
        let poly2: Vec<[f64; 2]> = polygon.iter().map(|&(u, v, _)| [u, v]).collect();
        if (shoelace2(&poly2) > 0.0) != face.sense {
            return Err(TessellateError::TrimLoopOrientation { face: fk });
        }
    }
    // THE ONE CHART WITH POLES is the sphere's, and its polygon is the
    // shape `sphere_polygon_shape` admits, so the duplicate-id skip in
    // the emit pass never collapses a pole fan (#678). Every other arm's
    // chart has none — `Chart::poles()` is empty for a cylinder and a
    // NURBS face has no `Chart` at all — and that fact is `walk`'s, so
    // it is asserted here rather than written down (D2 addendum row 5).
    if matches!(lane, Lane::Sphere { .. }) {
        sphere_polygon_shape(fk, &polygon, &polygon_edges)?;
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
        // small face's boundary circle is near round, so an ear on it
        // usually has the face's interior points in its circumcircle and
        // the CDT does not cut it. An ear on a boundary chain is the
        // triangle of three consecutive chord points, which the
        // neighbour across that chain may cut too. That is a steer, not
        // a guarantee: what holds the line is the ear cure in `attempt`,
        // which splits any ear the CDT keeps by its centroid.
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
        sense: face.sense,
        metric,
        surface,
        lane: &lane,
        polygon: &polygon,
        creases: creases.as_ref(),
        shared,
        tol,
    };
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
    /// The face's orientation sense: its outward normal is the
    /// surface's when true ([`Self::outward`]).
    sense: bool,
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

    /// The face's outward normal direction at `p`, on an analytic arm:
    /// the surface's radial direction (off the cylinder's axis, off the
    /// sphere's centre), reversed on a reversed face. `None` on the
    /// NURBS arm, whose winding the census does not read.
    fn outward(&self, p: Point3<f64>) -> Option<geom_core::Vec3<f64>> {
        let radial = match *self.lane {
            Lane::Cylinder { origin, axis, .. } => {
                let d = p - origin;
                d - axis * d.dot(axis)
            }
            Lane::Sphere { center, .. } => p - center,
            Lane::Nurbs { .. } => return None,
        };
        Some(if self.sense { radial } else { -radial })
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
        // THE EAR CURE (both analytic arms). A kept triangle on three
        // consecutive points of the trim polygon is an ear of a boundary
        // chain, and it is the same three mesh vertices whichever face
        // cuts it: a neighbour across that chain that cuts it too
        // (a plane cap, whose polygon is convex, cuts ears freely) makes
        // the triangle twice, wound both ways, and their diagonal an edge
        // of four. No patch sees that alone — each uses the diagonal
        // twice, as any interior edge — so this face never keeps an ear:
        // its centroid, strictly inside the ear and so inside the
        // polygon, is inserted and the face rebuilt, which splits it.
        // An ear the retry budget does not clear refuses typed
        // (`work/tesslie/two-faces-cutting-one-ear-of-a-shared-chord-chain-leave-a-four-use-edge.md`).
        if !ears.is_empty() {
            if attempt == last_attempt() {
                return Err(TessellateError::BoundaryEar { face: fk });
            }
            candidates.extend(ears);
            return Ok(Next::Rebuild);
        }
        // THE WINDING CENSUS (both analytic arms): a triangle whose
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
        let inputs: Vec<(f64, f64)> = self
            .polygon
            .iter()
            .map(|&(u, v, _)| (u, v))
            .chain(
                candidates
                    .iter()
                    .enumerate()
                    .filter(|(k, _)| !dropped.contains(k))
                    .map(|(_, &p)| p),
            )
            .collect();
        let key = |x: f64| if x == 0.0 { 0u64 } else { x.to_bits() };
        let mut unique: Vec<SpadePoint<f64>> = Vec::with_capacity(inputs.len());
        let mut index_of: HashMap<(u64, u64), usize> = HashMap::with_capacity(inputs.len());
        let mut handle_of: Vec<FixedVertexHandle> = Vec::with_capacity(inputs.len());
        for &(u, v) in &inputs {
            let p = self.cdt_point(u, v);
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
        let mut handles = Vec::with_capacity(self.polygon.len());
        let mut on_crease = Vec::new();
        let mut walk = Walk::new();
        for (&(u, v, id), &h) in self.polygon.iter().zip(&handle_of) {
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
        for ((k, &(u, v)), &h) in kept.zip(&handle_of[self.polygon.len()..]) {
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
            // Wound as emitted below: reversed under `flip`.
            let normal = (tri[1] - tri[0]).cross(tri[2] - tri[0]);
            let normal = if flip { -normal } else { normal };
            let centroid = Point3::new(
                (tri[0].x + tri[1].x + tri[2].x) / 3.0,
                (tri[0].y + tri[1].y + tri[2].y) / 3.0,
                (tri[0].z + tri[1].z + tri[2].z) / 3.0,
            );
            // An ear: three corners at consecutive places `i, i+1, i+2`
            // of the polygon (cyclically), read on the analytic arms.
            if !refine_lane && n_poly >= 3 {
                let at_places: Vec<&Vec<usize>> = vs
                    .iter()
                    .filter_map(|vtx| places.get(&vtx.fix().index()))
                    .collect();
                let consecutive = |a: usize, b: usize, c: usize| {
                    let set = [a, b, c];
                    set.iter().any(|&i| {
                        set.contains(&((i + 1) % n_poly)) && set.contains(&((i + 2) % n_poly))
                    })
                };
                if at_places.len() == 3
                    && at_places[0].iter().any(|&a| {
                        at_places[1]
                            .iter()
                            .any(|&b| at_places[2].iter().any(|&c| consecutive(a, b, c)))
                    })
                {
                    ears.push(uv_centroid(&uv));
                }
            }
            if self.outward(centroid).is_some_and(|n| normal.dot(n) <= 0.0) {
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
                refine.push(uv_centroid(&uv));
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

/// A triangle's centroid in UV: the point both the ear cure and the
/// certificate refinement insert, strictly inside the triangle and so
/// inside the trim polygon.
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
///
/// Each refusal is [`TessellateError::SphereTrimShape`] naming its
/// [`SphereTrimCause`] and the edge the offending point or side comes
/// from (`edges[i]` is point `i`'s, and side `i` runs from point `i`).
///
/// The constants are argued where they stand: `4ε` relative past `π/2`
/// is the rounding of an `atan2` at a pole (a few ulps), so a meridian's
/// endpoint does not read as past it; `3π/2` is past any side a chord
/// pass can make between two lattice columns (`sphere_arm_tighten`
/// holds every chord to one `h_u ≤ π/4`), and short of the `2π` a
/// winding side spans, so it separates the two with room either way.
fn sphere_polygon_shape(
    fk: FaceKey,
    polygon: &[(f64, f64, u32)],
    edges: &[EdgeKey],
) -> Result<(), TessellateError> {
    use core::f64::consts::{FRAC_PI_2, PI};
    let refuse = |i: usize, cause| -> Result<(), TessellateError> {
        Err(TessellateError::SphereTrimShape {
            face: fk,
            edge: edges[i],
            cause,
        })
    };
    if let Some(i) = polygon
        .iter()
        .position(|&(_, v, _)| v.abs() > FRAC_PI_2 * (1.0 + 4.0 * f64::EPSILON))
    {
        return refuse(i, SphereTrimCause::PastPole);
    }
    let n = polygon.len();
    if let Some(i) = (0..n).find(|&i| {
        let (a, b) = (polygon[i], polygon[(i + 1) % n]);
        !(a.2 == b.2 && at_pole(a.1)) && (b.0 - a.0).abs() > 1.5 * PI
    }) {
        return refuse(i, SphereTrimCause::PoleInside);
    }
    let apart = crate::walk::ids_at_two_uvs(polygon.iter().copied());
    if let Some(i) = polygon
        .iter()
        .position(|&(_, v, id)| apart.contains(&id) && !at_pole(v))
    {
        return refuse(i, SphereTrimCause::SeamTwice);
    }
    Ok(())
}

/// The sphere arm's column count over the trim box `(u0, u1)`: `nu`,
/// doubled until no pole side of `polygon` has exactly one lattice
/// column strictly between its ends — the shape #678's non-manifold fan
/// needs (`curved::pole_columns` carries the argument). Doubling keeps
/// every column it had, so a side's count never falls, but it need not
/// rise past one at the first doubling (the new columns can land
/// outside a short side); the loop reads the count again after each, and
/// ends once the spacing is under half the shortest pole side, where
/// every side holds at least two. [`ceil_count`] caps the doubling.
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

/// The uniform interior grid candidates of the cylinder and sphere arms: the trim
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
///
/// Beside the polygon, the edge each point's traversal walks: a refusal
/// of the polygon's shape names the edge that shows it.
fn trim_polygon(
    body: &Body<f64>,
    fk: FaceKey,
    lk: topo::LoopKey,
    chords: &ChordPass,
    nurbs_chart: bool,
    creases: Option<&Creases>,
    poles: ChartPoles,
) -> Result<(TrimPolygon, Vec<EdgeKey>), TessellateError> {
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
    let mut trav_edges: Vec<EdgeKey> = Vec::with_capacity(cycle.len());
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
        trav_edges.push(he.edge);
    }
    if let ChartPoles::Kept { sense } = poles {
        choose_pole_azimuths(&mut travs, &trav_edges, sense, fk)?;
    }
    flatten_traversals(&travs, &trav_edges, poles != ChartPoles::None)
}

/// The traversals joined into one polygon: each contributes all but its
/// last point, and with `keep_poles` its last point too where that is a
/// pole the next traversal leaves from another azimuth, so the pole is a
/// side of the polygon ([`sphere_polygon_shape`]). Beside it, the edge
/// each point's traversal walks.
fn flatten_traversals(
    travs: &[Vec<(f64, f64, u32)>],
    edges: &[EdgeKey],
    keep_poles: bool,
) -> Result<(TrimPolygon, Vec<EdgeKey>), TessellateError> {
    if travs.iter().any(Vec::is_empty) {
        return Err(TessellateError::MissingEntity {
            what: "edge chords",
        });
    }
    Ok(flat_points(travs, keep_poles, |_| 0.0)
        .map(|(k, p)| (p, edges[k]))
        .unzip())
}

/// [`flatten_traversals`]' points, each with its traversal's index, with
/// traversal `k`'s azimuths moved by `du(k)`: the one walk both the
/// polygon and [`choose_pole_azimuths`]' placements read. Every
/// traversal is non-empty (the caller's check).
fn flat_points<'a>(
    travs: &'a [Vec<(f64, f64, u32)>],
    keep_poles: bool,
    du: impl Fn(usize) -> f64 + Copy + 'a,
) -> impl Iterator<Item = (usize, (f64, f64, u32))> + 'a {
    let m = travs.len();
    (0..m).flat_map(move |k| {
        let shift = move |q: usize, (u, v, id): (f64, f64, u32)| (u + du(q), v, id);
        let trav = &travs[k];
        let last = shift(k, trav[trav.len() - 1]);
        let next = shift((k + 1) % m, travs[(k + 1) % m][0]);
        #[allow(clippy::float_cmp)]
        let pole = keep_poles
            && last.2 == next.2
            && at_pole(last.1)
            && (last.0, last.1) != (next.0, next.1);
        trav[..trav.len() - 1]
            .iter()
            .map(move |&p| (k, shift(k, p)))
            .chain(pole.then_some((k, last)))
    })
}

/// **The azimuth of each run of a sphere arm loop between its poles.**
///
/// The loop's lift (`topo::Body::loop_lift`) carries each image where
/// the chain places it, and the chain is continuous in the chart at
/// every vertex but a pole: every azimuth is the pole, so the run that
/// leaves it may sit `2πk` from where the region is. A run placed one
/// turn off makes the polygon bound the face's complement plus the cap
/// past the pole (a half-band remnant whose far seam meridian lands at
/// `+π` for `−π`), and nothing in the chart says so locally.
///
/// What says so is the AREA. A polygon on the chart bounds the region
/// whose area is its equal-area shoelace `A = ½ Σ (u_i w_{i+1} −
/// u_{i+1} w_i)` with `w = sin v` (times `r²`), and a face holding no
/// pole has an area in `(0, 4π)` with the sign the loop's orientation
/// gives it — positive exactly when `sense`, the chart's normal being
/// the sphere's outward one. Moving a run from one pole to the other by
/// `2π` moves `A` by `±2π·(w_N − w_S) = ±4π`, so exactly one placement
/// of each such run lands in the window. A run from a pole back to the
/// same pole moves `A` by nothing; among placements that tie on area the
/// one with the shortest pole sides is taken (a longer one doubles back
/// across the pole), and a tie there too refuses typed.
///
/// The first run keeps its lift; each other run is tried at `−2π`, `0`
/// and `+2π`. A loop with fewer than two pole junctions has no run to
/// place: its one run closes through a vertex the lift is continuous
/// at. These are SCOPE limits on valid bodies, not defects, and refuse
/// [`TessellateError::SphereTrimShape`]: more than
/// [`MAX_POLE_JUNCTIONS`] junctions (`PoleJunctions`), and no placement
/// fitting the window or two tying (`PoleAzimuth`), each naming the
/// first traversal of the run that could not be placed.
fn choose_pole_azimuths(
    travs: &mut [Vec<(f64, f64, u32)>],
    edges: &[EdgeKey],
    sense: bool,
    fk: FaceKey,
) -> Result<(), TessellateError> {
    use core::f64::consts::{PI, TAU};
    let m = travs.len();
    if travs.iter().any(Vec::is_empty) {
        return Err(TessellateError::MissingEntity {
            what: "edge chords",
        });
    }
    let junctions: Vec<usize> = (0..m)
        .filter(|&k| {
            let (last, next) = (travs[k][travs[k].len() - 1], travs[(k + 1) % m][0]);
            last.2 == next.2 && at_pole(last.1)
        })
        .collect();
    let j = junctions.len();
    if j < 2 {
        return Ok(());
    }
    let refuse = |k: usize, cause| TessellateError::SphereTrimShape {
        face: fk,
        edge: edges[k],
        cause,
    };
    if j > MAX_POLE_JUNCTIONS {
        return Err(refuse(
            junctions[MAX_POLE_JUNCTIONS],
            SphereTrimCause::PoleJunctions,
        ));
    }
    // Run `s` of traversal `k`: the junctions before it, cyclically, so
    // the traversals after the last junction close onto run 0.
    let run: Vec<usize> = (0..m)
        .map(|k| junctions.iter().filter(|&&p| p < k).count() % j)
        .collect();
    #[allow(clippy::cast_possible_truncation)]
    let placements = 3usize.pow((j - 1) as u32);
    let shift = |placement: usize, s: usize| -> f64 {
        if s == 0 {
            return 0.0;
        }
        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        let digit = (placement / 3usize.pow((s - 1) as u32)) % 3;
        #[allow(clippy::cast_precision_loss)]
        let turns = digit as f64 - 1.0;
        turns * TAU
    };
    let want = if sense { 1.0 } else { -1.0 };
    let mut best: Option<(f64, usize)> = None;
    let mut tied = false;
    for placement in 0..placements {
        let run = &run;
        let mut points =
            flat_points(travs, true, move |k| shift(placement, run[k])).map(|(_, p)| p);
        let Some(first) = points.next() else {
            return Err(TessellateError::MissingEntity {
                what: "edge chords",
            });
        };
        let (mut area2, mut pole_len, mut a) = (0.0, 0.0, first);
        for b in points.chain(core::iter::once(first)) {
            area2 += a.0 * b.1.sin() - b.0 * a.1.sin();
            if a.2 == b.2 && at_pole(a.1) {
                pole_len += (b.0 - a.0).abs();
            }
            a = b;
        }
        let area = want * 0.5 * area2;
        if !(area > 0.0 && area < 2.0 * TAU) {
            continue;
        }
        // THE TIE TOLERANCE IS HALF A TURN. Two placements that both fit
        // differ by whole turns on some run, so their pole sides' total
        // lengths differ by a multiple of `2π` or not at all; rounding
        // moves them by ulps of `π`. Anything between separates a tie
        // from a difference, and `π` sits midway.
        match best {
            Some((len, _)) if (pole_len - len).abs() < PI => tied = true,
            Some((len, _)) if pole_len > len => {}
            _ => {
                best = Some((pole_len, placement));
                tied = false;
            }
        }
    }
    let unplaced = refuse((junctions[0] + 1) % m, SphereTrimCause::PoleAzimuth);
    let Some((_, placement)) = best else {
        return Err(unplaced);
    };
    if tied {
        return Err(unplaced);
    }
    for (k, trav) in travs.iter_mut().enumerate() {
        let du = shift(placement, run[k]);
        for p in trav.iter_mut() {
            p.0 += du;
        }
    }
    Ok(())
}

/// A trim polygon: each point's chart `(u, v)` and mesh id, in walk
/// order.
type TrimPolygon = Vec<(f64, f64, u32)>;

/// Whether [`trim_polygon`] keeps a pole the loop passes as a side of
/// the polygon: the sphere arm's chart has poles, and no other arm's
/// does. `sense` is the face's, which places the runs between the
/// poles ([`choose_pole_azimuths`]).
#[derive(Clone, Copy, PartialEq, Eq)]
enum ChartPoles {
    Kept { sense: bool },
    None,
}

/// Whether a sphere chart latitude is a pole's, to the rounding of the
/// chart's own evaluation: a meridian's image reaches `±π/2` at the
/// pole vertex within a few ulps. `64ε` of latitude is a polar distance
/// under `1.5e-14` rad, far inside the nearest a chord point off the
/// pole can sit at any count `sizing::MAX_COUNT` admits, so no point off
/// the pole reads as on it.
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
            .unwrap()
            .0;
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
        // Its area: two spheres less the cap each loses, `2πrh` at
        // `h = 1 − 0.7`.
        let area = 8.0 * pi - 2.0 * 2.0 * pi * 0.3;
        crate::validate::check_volume(&mesh, volume, area, 1e-2).expect("and it encloses the body");
    }

    /// **`harmonic_range` reads an extremum inside the interval.** A
    /// tilted circle's height turns inside its edge's interval, so the
    /// ends alone under-read its range. Each case's range must hold a
    /// dense sampling of the function and reach the extremum the
    /// interval holds: `cos t` over `[−1, 1]` peaks at 1 with both ends at
    /// `cos 1`, and the shifted forms put the turn off zero.
    #[test]
    fn harmonic_range_holds_a_turn_inside_the_interval() {
        let (lo, hi) = super::harmonic_range(0.0, 1.0, 0.0, (-1.0, 1.0));
        assert!((hi - 1.0).abs() < 1e-15, "the peak at t = 0: {hi}");
        assert!((lo - 1f64.cos()).abs() < 1e-15, "the ends: {lo}");
        for (c, a, b, t) in [
            (0.3, 0.4, -0.7, (0.5, 3.0)),
            (-0.2, -0.6, 0.1, (2.0, 4.5)),
            (0.0, 0.5, 0.5, (-2.5, 2.5)),
            (1.0, -0.3, -0.9, (4.0, 7.5)),
        ] {
            let (lo, hi) = super::harmonic_range(c, a, b, t);
            let f = |x: f64| c + a * x.cos() + b * x.sin();
            let (mut smin, mut smax) = (f64::INFINITY, f64::NEG_INFINITY);
            for i in 0..=4096 {
                let x = t.0 + (t.1 - t.0) * f64::from(i) / 4096.0;
                smin = smin.min(f(x));
                smax = smax.max(f(x));
            }
            assert!(
                lo <= smin + 1e-12 && hi >= smax - 1e-12,
                "{c} {a} {b} {t:?}"
            );
            assert!(
                smin - lo < 1e-6 && hi - smax < 1e-6,
                "{c} {a} {b} {t:?}: the range is the function's, not wider: \
                 [{lo}, {hi}] against [{smin}, {smax}]"
            );
            let ends = (f(t.0).min(f(t.1)), f(t.0).max(f(t.1)));
            assert!(
                lo < ends.0 - 1e-3 || hi > ends.1 + 1e-3,
                "{c} {a} {b} {t:?}: the case turns inside its interval"
            );
        }
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
        let nu = super::pole_side_columns(&polygon, (0.0, 1.0), 2).unwrap();
        assert!(between(nu) >= 2, "nu {nu} leaves {} between", between(nu));
        assert_eq!(
            super::pole_side_columns(&polygon, (0.0, 1.0), 3).unwrap(),
            3
        );
    }

    /// **The tilted pair's faces meet their poles as polygon sides, and
    /// the runs between the poles sit where the region is.** Each ball's
    /// half-band remnant passes both chart poles at loop vertices
    /// (`y`-poled balls, seam meridians pole to pole), so its polygon
    /// carries two pole sides ([`super::ChartPoles::Kept`]) and the run
    /// between them is placed by area ([`super::choose_pole_azimuths`]):
    /// every such face's equal-area shoelace is positive and under `4π`.
    /// Without the kept pole the polygon cuts the cap off the pole, and
    /// with the lift's own azimuth the far meridian lands a turn away
    /// (the whole-face inversion PR 5920's review found).
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
            let surface = body.get_surface(face.surface).unwrap();
            if !super::takes_sphere_arm(&body, fk, surface).unwrap() {
                continue;
            }
            let (polygon, _) = super::trim_polygon(
                &body,
                fk,
                face.outer,
                &chords,
                false,
                None,
                ChartPoles::Kept { sense: face.sense },
            )
            .unwrap();
            let n = polygon.len();
            let pole_sides = (0..n)
                .filter(|&i| {
                    let (a, b) = (polygon[i], polygon[(i + 1) % n]);
                    a.2 == b.2 && super::at_pole(a.1)
                })
                .count();
            let area = 0.5
                * (0..n)
                    .map(|i| {
                        let (a, b) = (polygon[i], polygon[(i + 1) % n]);
                        a.0 * b.1.sin() - b.0 * a.1.sin()
                    })
                    .sum::<f64>();
            let area = if face.sense { area } else { -area };
            assert!(
                area > 0.0 && area < 4.0 * core::f64::consts::PI,
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
    fn twelve_edge_keys() -> Vec<topo::EdgeKey> {
        let brick: topo::Body<f64> = sweep::test_support::brick(
            (0.0, 1.0),
            (0.0, 1.0),
            (0.0, 1.0),
            geom_core::Tol::witness(),
        );
        brick.edges().map(|(k, _)| k).collect()
    }

    /// **`sphere_polygon_shape` refuses each shape it names, at the edge
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
            match super::sphere_polygon_shape(fk, poly, &edges) {
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
        let around = [(0.0, 0.0, 1), (1.0, 0.0, 2), (6.0, 0.5, 3), (0.0, 1.0, 4)];
        assert_eq!(cause(&around), Some((e(1), SphereTrimCause::PoleInside)));
        let seam = [
            (0.0, 0.0, 1),
            (1.0, 0.0, 2),
            (1.0, 1.0, 3),
            (0.5, 0.5, 1),
            (0.0, 1.0, 4),
        ];
        assert_eq!(cause(&seam), Some((e(0), SphereTrimCause::SeamTwice)));
    }

    /// **A projected circle through the chart's pole has no azimuth
    /// bound** (`chords::projected_circle_speeds` answers `None`, which
    /// `sphere_uv_speeds` refuses as `SphereTrimShape { ThroughPole }`):
    /// the unit great circle in the `xz` plane passes `z = ±1`, where
    /// `ρ = 0`. Tilted off the pole it answers finite bounds.
    #[test]
    fn a_circle_through_the_pole_has_no_azimuth_bound() {
        use geom_core::Vec3;
        let (o, x, z) = (
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );
        assert_eq!(
            crate::chords::projected_circle_speeds((o, x, z), (0.0, core::f64::consts::TAU)),
            None
        );
        let tilted = Vec3::new(0.0, 0.6, 0.8);
        let (su, sv) =
            crate::chords::projected_circle_speeds((o, x, tilted), (0.0, core::f64::consts::TAU))
                .expect("a circle clear of the pole is bounded");
        assert!(su.is_finite() && sv.is_finite() && su >= 1.0 && sv > 0.0);
    }

    /// **`choose_pole_azimuths` refuses its scope limits by cause.** Runs
    /// between pole junctions: two traversals each from pole to pole
    /// place by area; six junctions are past the search; and a loop whose
    /// every placement's area misses the window refuses `PoleAzimuth`.
    #[test]
    fn the_pole_placement_refuses_its_scope_limits_by_cause() {
        use crate::types::SphereTrimCause;
        let half_pi = core::f64::consts::FRAC_PI_2;
        let fk = FaceKey::default();
        let edges = twelve_edge_keys();
        // A half band: up the meridian u = 0 from the south pole (id 1)
        // to the north (id 2), down u = −π, the far run lifted a turn
        // off at +π. The placement moves it back.
        let meridian = |u: f64, up: bool| -> Vec<(f64, f64, u32)> {
            let mut v: Vec<_> = (0..=8)
                .map(|k| (u, -half_pi + core::f64::consts::PI * f64::from(k) / 8.0, 0))
                .collect();
            if !up {
                v.reverse();
            }
            let n = v.len();
            v[0].2 = if up { 1 } else { 2 };
            v[n - 1].2 = if up { 2 } else { 1 };
            for (k, p) in v.iter_mut().enumerate().take(n - 1).skip(1) {
                #[allow(clippy::cast_possible_truncation)]
                let id = 10 + k as u32 + if up { 0 } else { 100 };
                p.2 = id;
            }
            v
        };
        let mut band = vec![meridian(0.0, true), meridian(core::f64::consts::PI, false)];
        super::choose_pole_azimuths(&mut band, &edges, false, fk)
            .expect("the band places, wound clockwise for `sense: false`");
        assert!(
            band[1].iter().all(|p| p.0 == core::f64::consts::PI),
            "already in the window"
        );
        let mut lifted = vec![meridian(0.0, true), meridian(core::f64::consts::PI, false)];
        super::choose_pole_azimuths(&mut lifted, &edges, true, fk)
            .expect("the band places, wound counter-clockwise for `sense: true`");
        assert!(
            lifted[1].iter().all(|p| p.0 == -core::f64::consts::PI),
            "the far run moves a turn"
        );
        let mut many: Vec<_> = (0..6)
            .map(|k| meridian(f64::from(k) * 0.5, k % 2 == 0))
            .collect();
        assert!(matches!(
            super::choose_pole_azimuths(&mut many, &edges, true, fk),
            Err(TessellateError::SphereTrimShape {
                cause: SphereTrimCause::PoleJunctions,
                ..
            })
        ));
        // Runs four turns apart: every placement's area is past 4π.
        let mut wide = vec![
            meridian(0.0, true),
            meridian(-8.0 * core::f64::consts::PI, false),
        ];
        assert!(matches!(
            super::choose_pole_azimuths(&mut wide, &edges, true, fk),
            Err(TessellateError::SphereTrimShape {
                cause: SphereTrimCause::PoleAzimuth,
                ..
            })
        ));
    }
}
