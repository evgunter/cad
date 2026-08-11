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
//! Two charts construct here: **cylinder** (M5 PR 11 — the tiltedcut
//! walls; every split cylinder wall) and, since M7's montage
//! skin-scenes unit, **described NURBS** (the loft/sweep
//! walls — the frontier this header used to call "the banked
//! trimmed-NURBS lane", now promoted; NURBS faces route here
//! unconditionally, iso-rectangle or not, because the swept-rectangle
//! walk has no NURBS chart; RATIONAL faces since M8-5, whose Hessian
//! bound is the quotient-rule assembly over the homogeneous nets).
//! The NURBS lane's certificate is the
//! hull-derived Hessian interpolation bound (`crate::nurbs_cert`:
//! derivation, covered-vs-refused inventory); its boundary pcurves
//! are the closed-form images (`Harmonic`, `IsoLine` — every
//! loft/sweep wall boundary stores `IsoLine`). Conic trims on
//! cone/sphere/torus charts refuse typed naming that frontier (their
//! pcurves mint since M6-3; the trimmed-lane geometry for them is
//! unwritten). A **fitted** (rung-3) chart image still refuses typed
//! on every chart: the chord pass has no certified UV chord-step
//! bound for a fitted image (the boundary-tightening contract in
//! `crate::chords`), and its first genuine consumer is the
//! edge×NURBS-face boolean layer (the cut-loft unit).
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
//! the guarantee; a BOUNDARY point as intermediate is a self-touching
//! trim loop, refused typed
//! ([`TessellateError::SelfTouchingTrimLoop`]). That refusal has no
//! at-rest fixture on purpose: split sections and boolean seams mint
//! simple loops, and hand-building a self-touching one at the mesh
//! layer would need a full body whose certified pcurve caches
//! describe a loop the mint pass itself refuses (`LoopNotClosed`/
//! continuity) — the arm stands as the backstop's tripwire (review
//! MIN-1), not a reachable lane.

use std::collections::{HashMap, HashSet};

use geom_brep::Pcurve;
use geom_core::Point3;
use geom_curves::Curve3;
use geom_surfaces::Surface;
use spade::{ConstrainedDelaunayTriangulation, Point2 as SpadePoint, Triangulation};
use topo::{Body, EdgeKey, FaceKey};

use crate::cert;
use crate::chords::{ceil_count, sagitta_angle};
use crate::curved::Tol;
use crate::nurbs_cert::{NurbsFaceBound, nurbs_face_bound};
use crate::planar::{classify_faces, edge_key, shoelace2};
use crate::types::TessellateError;

/// Retry budget for the grid-on-constraint rebuild (module docs).
const MAX_GRID_RETRIES: usize = 4;

/// The per-chart data of the two constructing lanes (module docs):
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
    /// The M7 trimmed-NURBS lane (closed-form pcurve images, Hessian
    /// interpolation certificate — `crate::nurbs_cert`).
    Nurbs { bound: NurbsFaceBound },
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
        if matches!(curve.carrier(), Curve3::Ellipse { .. } | Curve3::Nurbs(_)) {
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
    chords: &HashMap<EdgeKey, Vec<u32>>,
    chord_ts: &HashMap<EdgeKey, Vec<f64>>,
    positions: &mut Vec<Point3<f64>>,
    tol: &Tol,
) -> Result<Vec<[u32; 3]>, TessellateError> {
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
        Surface::Nurbs(ref payload) => {
            if payload.is_placeholder() {
                // The mvfs "no description yet" state — the historical
                // refusal, kept for exactly this class (types docs).
                return Err(TessellateError::UnsupportedSurface { face: fk });
            }
            Lane::Nurbs {
                bound: nurbs_face_bound(payload, fk)?,
            }
        }
        _ => return Err(trim_frontier(body, fk, face.outer)?),
    };

    // The UV polygon: per half-edge, the pcurve evaluated at the
    // shared chord parameters (each traversal contributes all but its
    // last point; ids stay the shared 3-D chord ids).
    let nurbs_chart = matches!(lane, Lane::Nurbs { .. });
    let polygon = trim_polygon(body, fk, face.outer, chords, chord_ts, nurbs_chart)?;
    if polygon.len() < 3 {
        return Err(TessellateError::MissingEntity {
            what: "degenerate trimmed boundary",
        });
    }

    // Grid sizing (heuristic; the certificates are the guarantee —
    // crate docs): cylinder — sagitta-tight in u, rows every r·hu
    // metres in v; NURBS — the Hessian-budget steps (`nurbs_cert`),
    // shared with the chord pass's boundary tightening.
    let (mut u0, mut u1, mut v0, mut v1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
    for &(u, v, _) in &polygon {
        u0 = u0.min(u);
        u1 = u1.max(u);
        v0 = v0.min(v);
        v1 = v1.max(v);
    }
    let (nu, nv) = match lane {
        Lane::Cylinder { radius, .. } => {
            let hu = sagitta_angle(tol.delta_s, radius);
            (ceil_count(u1 - u0, hu)?, ceil_count(v1 - v0, radius * hu)?)
        }
        Lane::Nurbs { ref bound } => {
            let (hu, hv) = bound.grid_steps(tol.delta_s);
            (ceil_count(u1 - u0, hu)?, ceil_count(v1 - v0, hv)?)
        }
    };

    // Grid candidates (row-major, deterministic ids assigned on the
    // FINAL kept set so retries stay deterministic).
    let mut dropped: HashSet<(usize, usize)> = HashSet::new();
    'retry: for attempt in 0..=MAX_GRID_RETRIES {
        let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
            ConstrainedDelaunayTriangulation::new();
        // Handle-index → (mesh id or grid slot, uv).
        #[derive(Clone, Copy)]
        enum Slot {
            Boundary(u32),
            Grid { i: usize, j: usize },
        }
        let mut meta: Vec<(f64, f64, Slot)> = Vec::new();
        let mut handles = Vec::with_capacity(polygon.len());
        for &(u, v, id) in &polygon {
            let h = cdt
                .insert(SpadePoint::new(u, v))
                .map_err(|_| TessellateError::Triangulation { face: fk })?;
            if h.index() == meta.len() {
                meta.push((u, v, Slot::Boundary(id)));
            }
            handles.push(h);
        }
        for j in 1..nv {
            for i in 1..nu {
                if dropped.contains(&(i, j)) {
                    continue;
                }
                #[allow(clippy::cast_precision_loss)]
                let u = u0 + (u1 - u0) * (i as f64 / nu as f64);
                #[allow(clippy::cast_precision_loss)]
                let v = v0 + (v1 - v0) * (j as f64 / nv as f64);
                let h = cdt
                    .insert(SpadePoint::new(u, v))
                    .map_err(|_| TessellateError::Triangulation { face: fk })?;
                if h.index() == meta.len() {
                    meta.push((u, v, Slot::Grid { i, j }));
                }
            }
        }

        // Constraints with crossing counts (the planar #116 pattern);
        // a constraint realised through an intermediate vertex means a
        // grid point sits exactly on it — drop and rebuild.
        let mut crossings: HashMap<(usize, usize), u32> = HashMap::new();
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
                            Slot::Grid { i, j } => {
                                dropped.insert((i, j));
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
                *crossings.entry(edge_key(e)).or_insert(0) += 1;
            }
        }
        if split_offender {
            if attempt == MAX_GRID_RETRIES {
                return Err(TessellateError::Triangulation { face: fk });
            }
            continue 'retry;
        }

        // Even-odd interior flags, then emission with certificates.
        let inside = classify_faces(&cdt, &crossings);
        let flip = {
            let poly2: Vec<[f64; 2]> = polygon.iter().map(|&(u, v, _)| [u, v]).collect();
            shoelace2(&poly2) < 0.0
        };
        // Pass 1: which grid slots the kept triangles use; mint their
        // mesh ids in row-major (j, i) order (the crate's determinism
        // contract for interior grid points).
        let mut used: Vec<(usize, usize)> = Vec::new();
        for f in cdt.inner_faces() {
            if !inside[f.fix().index()] {
                continue;
            }
            for vtx in f.vertices() {
                if let (_, _, Slot::Grid { i, j }) = meta[vtx.fix().index()] {
                    used.push((j, i));
                }
            }
        }
        used.sort_unstable();
        used.dedup();
        let mut grid_ids: HashMap<(usize, usize), u32> = HashMap::new();
        for &(j, i) in &used {
            #[allow(clippy::cast_precision_loss)]
            let u = u0 + (u1 - u0) * (i as f64 / nu as f64);
            #[allow(clippy::cast_precision_loss)]
            let v = v0 + (v1 - v0) * (j as f64 / nv as f64);
            #[allow(clippy::cast_possible_truncation)]
            let id = positions.len() as u32;
            positions.push(surface.eval(u, v));
            grid_ids.insert((i, j), id);
        }
        // Pass 2: emit and certify.
        let mut triangles = Vec::new();
        let mut worst: f64 = 0.0;
        for f in cdt.inner_faces() {
            if !inside[f.fix().index()] {
                continue;
            }
            let vs = f.vertices();
            let mut ids = [0u32; 3];
            let mut uv = [[0.0f64; 2]; 3];
            for (k, vtx) in vs.iter().enumerate() {
                let (u, v, slot) = meta[vtx.fix().index()];
                uv[k] = [u, v];
                ids[k] = match slot {
                    Slot::Boundary(id) => id,
                    Slot::Grid { i, j } => grid_ids[&(i, j)],
                };
            }
            if ids[0] == ids[1] || ids[1] == ids[2] || ids[0] == ids[2] {
                continue; // boundary-degenerate sliver
            }
            let tri = [
                positions[ids[0] as usize],
                positions[ids[1] as usize],
                positions[ids[2] as usize],
            ];
            let bound = match lane {
                Lane::Cylinder {
                    origin,
                    axis,
                    radius,
                } => cert::cert_cylinder(origin, axis, radius, tri),
                Lane::Nurbs { ref bound } => bound.cert(uv),
            };
            // REVIEW PROBE (env-gated): per-triangle falsification of
            // the NURBS certificate — dense barycentric samples of
            // |S(w) − Π(w)| must be dominated by cert + ε on EVERY
            // triangle, not in aggregate.
            if matches!(lane, Lane::Nurbs { .. }) && crate::probe_stats::armed() {
                let m = 12usize;
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
                        assert!(
                            d <= bound + tol.eps,
                            "PROBE per-triangle violation: |S-Pi| {d} > cert {bound} + eps {} \
                             at uv=({u},{v}) tri uv {uv:?}",
                            tol.eps
                        );
                        crate::probe_stats::record(d, bound + tol.eps);
                    }
                }
            }
            // Sticky-NaN accumulation (the curved lane's rule).
            if bound.is_nan() || worst.is_nan() || bound > worst {
                worst = bound;
            }
            triangles.push(if flip { [ids[0], ids[2], ids[1]] } else { ids });
        }
        if worst.is_nan() || worst > tol.delta {
            return Err(TessellateError::CertificateExceeded {
                face: fk,
                bound: worst,
                requested: tol.delta,
            });
        }
        return Ok(triangles);
    }
    Err(TessellateError::Triangulation { face: fk })
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
        if matches!(curve.carrier(), Curve3::Ellipse { .. } | Curve3::Nurbs(_)) {
            return Ok(TessellateError::UnsupportedCurve {
                edge: ek,
                note: "conic/B-spline trim on a cone/sphere/torus chart — those charts \
                       mint stored pcurves since M6-3, but the trimmed-face \
                       tessellation lanes written are the cylinder chart's (M5 PR 11) \
                       and the NURBS chart's (M7); the remaining analytic charts' \
                       trimmed lanes are banked with their first construction",
            });
        }
    }
    Ok(TessellateError::MissingEntity {
        what: "trimmed router fired without a trim carrier",
    })
}

/// The pcurve-driven UV polygon of the face's outer loop: per
/// half-edge, `pcurve.eval` at the shared chord parameters (module
/// docs; each traversal contributes all but its last point).
/// `nurbs_chart` widens the accepted image forms to `IsoLine` (the
/// NURBS chart's minted form); `Fitted` refuses typed on every chart
/// (module docs).
fn trim_polygon(
    body: &Body<f64>,
    fk: FaceKey,
    lk: topo::LoopKey,
    chords: &HashMap<EdgeKey, Vec<u32>>,
    chord_ts: &HashMap<EdgeKey, Vec<f64>>,
    nurbs_chart: bool,
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
    let mut out = Vec::new();
    for hek in cycle {
        let he = body
            .get_half_edge(hek)
            .ok_or(TessellateError::MissingEntity { what: "half-edge" })?;
        let edge = body
            .get_edge(he.edge)
            .ok_or(TessellateError::MissingEntity { what: "edge" })?;
        let forward = edge.he_plus == hek;
        let Some(cache) = body.pcurve(hek) else {
            return Err(TessellateError::UnsupportedCurve {
                edge: he.edge,
                note: "trimmed face half-edge carries no stored pcurve cache — caches \
                       mint in the split/boolean pipelines",
            });
        };
        // Trim-loop tessellation walks a chart image's CLOSED FORM
        // (module docs): `Harmonic` on every chart, `IsoLine` on the
        // NURBS chart (its minted form; an `IsoLine` on a cylinder
        // chart is not minted at rest — the harmonic form with zero
        // trigonometric channels owns that image). A fitted (rung-3)
        // image refuses typed on every chart rather than silently
        // approximating a spline boundary the chord pass could not
        // have sized (`crate::chords`' boundary-tightening contract).
        match cache.pcurve() {
            Pcurve::Harmonic { .. } => {}
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
                    note: "trimmed face half-edge carries a FITTED (rung-3) pcurve — \
                           the trim walk and the chord pass's boundary tightening \
                           read closed-form chart images; the fitted image's first \
                           tessellation consumer is the edge×NURBS-face boolean \
                           layer (the cut-loft unit)",
                });
            }
        }
        let ids = chords.get(&he.edge).ok_or(TessellateError::MissingEntity {
            what: "edge chords",
        })?;
        let ts = chord_ts
            .get(&he.edge)
            .ok_or(TessellateError::MissingEntity {
                what: "edge chord params",
            })?;
        // Traversal order: forward keeps he_plus order, reversed walks
        // backwards; drop the traversal's LAST entry (the next
        // traversal owns the junction).
        let n = ids.len();
        for k in 0..n - 1 {
            let idx = if forward { k } else { n - 1 - k };
            let uv = cache.pcurve().eval(ts[idx]);
            out.push((uv.x, uv.y, ids[idx]));
        }
    }
    Ok(out)
}
