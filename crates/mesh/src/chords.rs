//! Per-edge chord points: computed **once** from each edge's certified
//! carrier and consumed by both adjacent faces (the watertightness
//! half of the pure-function invariant, crate docs).
//!
//! Chord counts are deterministic ceil arithmetic from δ_s = δ/2 (the
//! documented sizing safety factor):
//!
//! - Line carriers: 1 chord (a segment of the exact locus).
//! - Circle carriers (radius ρ, forward span Δt): per-chord angle
//!   φ = 2·acos(1 − δ_s/ρ) (the closed-form sagitta bound
//!   ρ(1 − cos(φ/2)) ≤ δ_s), capped at π/4 (so periodic unwrapping is
//!   branch-unambiguous and full-period rims polygonalize with ≥ 8
//!   chords); n = ceil(Δt/φ).
//! - Adjacent-torus tightening: a face on a torus certifies through
//!   the UV interpolation bound (crate docs), which needs boundary UV
//!   steps ≤ its grid step h = √(δ_s/(3(R+2r))); a circle edge's
//!   carrier parameter *is* the torus chart coordinate along it
//!   (azimuth for rims, minor angle for meridians), so each adjacent
//!   torus face adds n ≥ ceil(Δt/h).
//! - Adjacent-NURBS tightening (M7, the trimmed-NURBS lane): the same
//!   shape with a hull-derived Hessian — a described NURBS face
//!   certifies through `crate::nurbs_cert`'s anisotropic bound, which
//!   needs boundary UV steps within the face's own (h_u, h_v); the
//!   half-edge's stored CLOSED-FORM pcurve (`IsoLine`/`Harmonic`)
//!   gives per-axis UV speed bounds (s_u, s_v) — exact `|pl|`
//!   components for the iso line, the amplitude sum for the harmonic
//!   form — so each adjacent NURBS face adds n ≥ ⌈s_u·Δt/h_u⌉ and
//!   ⌈s_v·Δt/h_v⌉, on EVERY carrier kind (a straight wall edge is one
//!   3-D chord but many UV steps). A `Fitted` image has no certified
//!   speed bound here and refuses typed (the trimmed lane's module
//!   docs name its consumer).
//!
//! These tightenings are the only places adjacent surfaces enter
//! chord counts — chord points remain a pure function of (carrier +
//! interval, endpoint points, adjacent surface parameters, δ).
//!
//! Polyline endpoints are the topology vertices' points **bitwise**
//! (never `carrier(t₀)`, which is only within ε of them) so every
//! polyline meeting at a vertex shares its mesh vertex id; interior
//! points are `carrier(t₀ + (t₁−t₀)·i/n)` in `he_plus`-forward order.

use std::collections::HashMap;

use geom_brep::Pcurve;
use geom_core::ring_interval::RingInterval;
use geom_core::spline::KnotVector;
use geom_core::spline::hull::derivative_coeffs;
use geom_curves::Curve3;
use geom_surfaces::Surface;
use topo::{Body, EdgeKey};

use crate::nurbs_cert::nurbs_face_bound;
use crate::types::TessellateError;

/// Sanity cap on any single count (δ small enough to exceed this would
/// allocate gigabytes before failing anywhere else).
const MAX_STEPS: f64 = 16_777_216.0; // 2^24

/// The per-chord azimuth angle for sagitta ≤ `delta_s` on a circle of
/// radius `rho`, capped at π/4. Total (poison-free for positive
/// inputs): if δ_s ≥ ρ the sagitta constraint is vacuous and the π/4
/// cap rules.
pub fn sagitta_angle(delta_s: f64, rho: f64) -> f64 {
    let cap = core::f64::consts::FRAC_PI_4;
    if delta_s < rho {
        let phi = 2.0 * (1.0 - delta_s / rho).acos();
        if phi < cap { phi } else { cap }
    } else {
        cap
    }
}

/// The per-chord parameter step for chord deviation ≤ `delta_s` on an
/// ellipse with semi-axes `major > minor` (M5 PR 5), capped at π/4.
///
/// Certified-conservative from the standard C² chord bound
/// `deviation ≤ κ_max·L²/8`: over a parameter span φ the arc length is
/// `L ≤ major·φ` (`|dP/dθ| ≤ major`) and the ellipse's maximum
/// curvature is `κ_max = major/minor²` (at the major vertices), so
/// `deviation ≤ R_eff·φ²/8` with `R_eff = major·(major/minor)²` and
/// `φ = √(8·δ_s/R_eff)` guarantees the bound. Coarser than the
/// circle's exact sagitta near `major = minor` — conservative is the
/// promised direction.
pub fn ellipse_step(delta_s: f64, major: f64, minor: f64) -> f64 {
    let cap = core::f64::consts::FRAC_PI_4;
    let r_eff = major * (major / minor) * (major / minor);
    let phi = (8.0 * delta_s / r_eff).sqrt();
    if phi < cap { phi } else { cap }
}

/// `ceil(span/step)` as a chord/grid count, with the `MAX_STEPS` (2^24)
/// sanity cap surfaced as a typed error and a floor of 1.
pub fn ceil_count(span: f64, step: f64) -> Result<usize, TessellateError> {
    let raw = (span / step).ceil();
    if !(raw.is_finite() && raw < MAX_STEPS) {
        return Err(TessellateError::ResolutionOverflow { count: raw });
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Ok(if raw < 1.0 { 1 } else { raw as usize })
}

/// The torus boundary-step requirement `h` (crate docs) for a face's
/// surface, if that surface is a torus.
fn torus_step(surface: &geom_surfaces::Surface<f64>, delta_s: f64) -> Option<f64> {
    match *surface {
        geom_surfaces::Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => Some(torus_grid_step(delta_s, major_radius, minor_radius)),
        _ => None,
    }
}

/// The torus UV grid step `h = √(δ_s/(3(R+2r)))` — shared with the
/// curved-face grid sizing so boundary and interior steps agree.
pub(crate) fn torus_grid_step(delta_s: f64, major: f64, minor: f64) -> f64 {
    (delta_s / (3.0 * (major + 2.0 * minor))).sqrt()
}

/// Computes every edge's chord-point ids (minting interior points into
/// `positions`), in edge-arena order. `vids` maps topology vertices to
/// their already-minted mesh ids.
#[allow(clippy::type_complexity)] // an (ids, params) pair of per-edge maps
pub(crate) fn compute_chords(
    body: &Body<f64>,
    delta_s: f64,
    vids: &HashMap<topo::VertexKey, u32>,
    positions: &mut Vec<geom_core::Point3<f64>>,
) -> Result<(HashMap<EdgeKey, Vec<u32>>, HashMap<EdgeKey, Vec<f64>>), TessellateError> {
    let mut chords = HashMap::new();
    // Chord PARAMETERS per edge (`he_plus`-forward, endpoints
    // included) — the trimmed-face lane evaluates pcurves at exactly
    // the chord schedule, so both stay one derivation (M5 PR 11).
    let mut params = HashMap::new();
    // Per-NURBS-face (h_u, h_v) memo for the adjacent-NURBS
    // tightening (module docs) — the Hessian hull is a per-face fact,
    // computed once however many edges the face bounds.
    let mut nurbs_steps: HashMap<topo::FaceKey, (f64, f64)> = HashMap::new();
    for (ek, edge) in body.edges() {
        let curve = body
            .get_curve_geom(edge.curve)
            .ok_or(TessellateError::MissingEntity { what: "edge curve" })?
            .certified()
            .ok_or(TessellateError::NullScaffoldEdge { edge: ek })?;
        let (t0, t1) = curve.params();
        let span = t1 - t0;
        let n = match *curve.carrier() {
            Curve3::Line { .. } => 1,
            Curve3::Circle { .. } => {
                let mut n =
                    ceil_count(span, sagitta_angle(delta_s, circle_radius(curve.carrier())))?;
                for fk in adjacent_faces(body, ek)? {
                    let face = body
                        .get_face(fk)
                        .ok_or(TessellateError::MissingEntity { what: "face" })?;
                    let surface =
                        body.get_surface(face.surface)
                            .ok_or(TessellateError::MissingEntity {
                                what: "face surface",
                            })?;
                    if let Some(h) = torus_step(surface, delta_s) {
                        n = n.max(ceil_count(span, h)?);
                    }
                }
                n
            }
            // Ellipse arcs (curved-cut boundaries, M5 PR 5): the
            // certified-conservative curvature-bound step; no torus
            // tightening applies (an ellipse never lies on a torus
            // chart of this kernel's constructions).
            Curve3::Ellipse { major, minor, .. } => {
                ceil_count(span, ellipse_step(delta_s, major, minor))?
            }
            // B-spline carriers (rung-3 edges at rest, M5 PR 9):
            // the hull-bounded sagitta generalization (C9/PR 11) —
            // secant deviation on a parameter step h is ≤ h²·sup|C″|/8
            // (Taylor with integral remainder; needs C¹, i.e. interior
            // multiplicities ≤ p − 1), and sup|C″| is a control-
            // coefficient convexity fact via iterated derivative
            // hulls. Rational carriers refuse typed (a rational
            // derivative is not a hull fact; none is minted at rest).
            Curve3::Nurbs(ref n) => nurbs_chord_count(n, span, delta_s, ek)?,
        };
        // Adjacent-NURBS tightening (module docs), on every carrier
        // kind — a straight wall edge is one 3-D chord but many UV
        // steps of the wall's certificate budget.
        let n = nurbs_tighten(body, ek, span, delta_s, &mut nurbs_steps, n)?;
        let (vs, ve) = edge_vertices(body, ek)?;
        let start_id = *vids.get(&vs).ok_or(TessellateError::MissingEntity {
            what: "start vertex",
        })?;
        let end_id = *vids
            .get(&ve)
            .ok_or(TessellateError::MissingEntity { what: "end vertex" })?;
        let mut ids = Vec::with_capacity(n + 1);
        let mut ts = Vec::with_capacity(n + 1);
        ids.push(start_id);
        ts.push(t0);
        for i in 1..n {
            #[allow(clippy::cast_precision_loss)]
            let t = t0 + span * (i as f64 / n as f64);
            #[allow(clippy::cast_possible_truncation)]
            let id = positions.len() as u32;
            positions.push(curve.carrier().eval(t));
            ids.push(id);
            ts.push(t);
        }
        ids.push(end_id);
        ts.push(t1);
        chords.insert(ek, ids);
        params.insert(ek, ts);
    }
    Ok((chords, params))
}

/// Chord count for a nonrational B-spline carrier from the
/// hull-bounded sagitta (module note at the call site): componentwise
/// `sup|C″|` hulls give `|C″| ≤ √(Σ sup²)`, and the per-step bound
/// `h²·M/8 ≤ δ_s` sizes `h`.
fn nurbs_chord_count(
    n: &geom_curves::NurbsCurve3<f64>,
    span: f64,
    delta_s: f64,
    ek: EdgeKey,
) -> Result<usize, TessellateError> {
    if n.weights().iter().any(|w| *w != 1.0) {
        return Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "rational B-spline carrier — a rational second derivative is not a \
                   control-hull convexity fact, and no at-rest construction mints one \
                   (conics stay exact kinds; the M6-3 loft's carriers are non-rational \
                   boundary rows, and arc-bearing profiles refuse at the rational-wall \
                   gate)",
        });
    }
    let kv = n.knots();
    let p = kv.degree();
    if p < 2 {
        // A single-segment degree-1 carrier is an exact chord; one
        // with interior knots is a C⁰ polyline whose kinks a uniform
        // parameter schedule would miss — refused, not guessed at.
        return if kv.knots().len() == 4 {
            Ok(1)
        } else {
            Err(TessellateError::UnsupportedCurve {
                edge: ek,
                note: "degree-1 B-spline carrier with interior knots (a C⁰ polyline) — \
                       the uniform chord schedule cannot pin its kinks; split the edge",
            })
        };
    }
    // C¹ needed for the secant bound: interior multiplicities ≤ p − 1.
    let interior = &kv.knots()[p + 1..kv.knots().len() - p - 1];
    let mut i = 0;
    while i < interior.len() {
        let mut j = i + 1;
        while j < interior.len() && interior[j] == interior[i] {
            j += 1;
        }
        if j - i > p - 1 {
            return Err(TessellateError::UnsupportedCurve {
                edge: ek,
                note: "B-spline carrier with a C⁰ kink (interior multiplicity = degree) — \
                       the hull sagitta bound needs C¹; split the edge at the kink",
            });
        }
        i = j;
    }
    let m_bound = {
        let mut sum_sq = RingInterval::zero();
        for comp in 0..3 {
            let coeffs: Vec<RingInterval> = n
                .control()
                .iter()
                .map(|pt| {
                    RingInterval::point(match comp {
                        0 => pt.x,
                        1 => pt.y,
                        _ => pt.z,
                    })
                })
                .collect();
            let q1 = derivative_coeffs(kv, &coeffs);
            let inner = kv.knots()[1..kv.knots().len() - 1].to_vec();
            let Ok(kv1) = KnotVector::clamped(inner, p - 1) else {
                return Err(TessellateError::UnsupportedCurve {
                    edge: ek,
                    note: "B-spline carrier whose derivative knot vector fails to \
                           materialise — outside the certified chord inventory",
                });
            };
            let q2 = derivative_coeffs(&kv1, &q1);
            let mut hull = RingInterval::poison();
            for (k, q) in q2.iter().enumerate() {
                hull = if k == 0 {
                    *q
                } else {
                    RingInterval::hull(hull, *q)
                };
            }
            sum_sq = sum_sq + hull.sqr();
        }
        sum_sq.hi().sqrt().next_up()
    };
    if !m_bound.is_finite() {
        return Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "B-spline carrier second-derivative hull is unbounded/poisoned — \
                   outside the certified chord inventory",
        });
    }
    if m_bound == 0.0 {
        return Ok(1);
    }
    let h = (8.0 * delta_s / m_bound).sqrt();
    ceil_count(span, h)
}

/// The adjacent-NURBS chord tightening (module docs): for each
/// adjacent described NURBS face, raise `n` until the edge's UV image
/// steps fit inside the face's certificate-budget grid steps. The
/// per-face `(h_u, h_v)` memo (`steps`) keeps the Hessian hull a
/// once-per-face computation.
fn nurbs_tighten(
    body: &Body<f64>,
    ek: EdgeKey,
    span: f64,
    delta_s: f64,
    steps: &mut HashMap<topo::FaceKey, (f64, f64)>,
    mut n: usize,
) -> Result<usize, TessellateError> {
    let edge = body
        .get_edge(ek)
        .ok_or(TessellateError::MissingEntity { what: "edge" })?;
    for hek in [edge.he_plus, edge.he_minus] {
        let he = body
            .get_half_edge(hek)
            .ok_or(TessellateError::MissingEntity { what: "half-edge" })?;
        let lp = body
            .get_loop(he.parent_loop)
            .ok_or(TessellateError::MissingEntity {
                what: "parent loop",
            })?;
        let fk = lp.face;
        let face = body
            .get_face(fk)
            .ok_or(TessellateError::MissingEntity { what: "face" })?;
        let surface = body
            .get_surface(face.surface)
            .ok_or(TessellateError::MissingEntity {
                what: "face surface",
            })?;
        let Surface::Nurbs(ref payload) = *surface else {
            continue;
        };
        if payload.is_placeholder() {
            return Err(TessellateError::UnsupportedSurface { face: fk });
        }
        let (hu, hv) = match steps.get(&fk) {
            Some(&s) => s,
            None => {
                let s = nurbs_face_bound(payload, fk)?.grid_steps(delta_s);
                steps.insert(fk, s);
                s
            }
        };
        let Some(cache) = body.pcurve(hek) else {
            return Err(TessellateError::UnsupportedCurve {
                edge: ek,
                note: "NURBS-face half-edge carries no stored pcurve cache — caches \
                       mint at loft/sweep assembly and STEP adoption; without one \
                       the chord schedule has no certified UV step bound",
            });
        };
        // Per-axis UV speed bounds (module docs): exact for the iso
        // line, the amplitude sum |pa|+|pb|+|pl| componentwise for the
        // harmonic form (|P′| = |−pa·sin t + pb·cos t + pl|).
        let (su, sv) = match cache.pcurve() {
            Pcurve::IsoLine { pl, .. } => (pl.x.abs(), pl.y.abs()),
            // The ARC rim (M8-3). With `s = ½ + tan(φ/2)/(2·tan(h/4))`
            // and `g = (k + s)/m`,
            //   `dg/dt = sec²(φ/2) / (4·m·tan(h/4))`,
            // maximal at the sub-arc ends (`|φ| = h/2`), where
            // `sec²(h/4)/(4·m·tan(h/4)) = 1/(2·m·sin(h/2))`.
            // A closed-form sup over the whole span, like the other
            // two arms — no sampling.
            Pcurve::IsoArc {
                pd, angle, breaks, ..
            } => {
                let spans = breaks.control_count().saturating_sub(1);
                if spans == 0 {
                    return Err(TessellateError::UnsupportedCurve {
                        edge: ek,
                        note: "an arc-rim pcurve with no sub-arc structure — a                                malformed cache, not a chord-schedule question",
                    });
                }
                #[allow(clippy::cast_precision_loss)]
                let m = spans as f64;
                let rate = 1.0 / (2.0 * m * (angle / (2.0 * m)).sin());
                (pd.x.abs() * rate, pd.y.abs() * rate)
            }
            Pcurve::Harmonic { pa, pb, pl, .. } => (
                pa.x.abs() + pb.x.abs() + pl.x.abs(),
                pa.y.abs() + pb.y.abs() + pl.y.abs(),
            ),
            Pcurve::Fitted(_) => {
                return Err(TessellateError::UnsupportedCurve {
                    edge: ek,
                    note: "NURBS-face half-edge carries a FITTED (rung-3) pcurve — no \
                           certified UV speed bound is wired for a fitted image's \
                           chord schedule; its first tessellation consumer is the \
                           edge×NURBS-face boolean layer (the cut-loft unit)",
                });
            }
        };
        n = n
            .max(ceil_count(su * span, hu)?)
            .max(ceil_count(sv * span, hv)?);
    }
    Ok(n)
}

/// The radius of a circle carrier (caller guarantees the variant).
fn circle_radius(carrier: &Curve3<f64>) -> f64 {
    match *carrier {
        Curve3::Circle { radius, .. } => radius,
        _ => f64::NAN,
    }
}

/// The (start, end) vertices of the edge's `he_plus`.
pub(crate) fn edge_vertices(
    body: &Body<f64>,
    ek: EdgeKey,
) -> Result<(topo::VertexKey, topo::VertexKey), TessellateError> {
    let edge = body
        .get_edge(ek)
        .ok_or(TessellateError::MissingEntity { what: "edge" })?;
    let he = body
        .get_half_edge(edge.he_plus)
        .ok_or(TessellateError::MissingEntity { what: "he_plus" })?;
    let end = body
        .half_edge_end(edge.he_plus)
        .ok_or(TessellateError::MissingEntity {
            what: "he_plus end",
        })?;
    Ok((he.start, end))
}

/// The (≤ 2 distinct) faces adjacent to an edge.
fn adjacent_faces(body: &Body<f64>, ek: EdgeKey) -> Result<Vec<topo::FaceKey>, TessellateError> {
    let edge = body
        .get_edge(ek)
        .ok_or(TessellateError::MissingEntity { what: "edge" })?;
    let mut out = Vec::with_capacity(2);
    for hek in [edge.he_plus, edge.he_minus] {
        let he = body
            .get_half_edge(hek)
            .ok_or(TessellateError::MissingEntity { what: "half-edge" })?;
        let lp = body
            .get_loop(he.parent_loop)
            .ok_or(TessellateError::MissingEntity {
                what: "parent loop",
            })?;
        if !out.contains(&lp.face) {
            out.push(lp.face);
        }
    }
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Point3;
    use geom_curves::NurbsCurve3;
    use topo::EdgeKey;

    fn wiggle() -> NurbsCurve3<f64> {
        let pts: Vec<Point3<f64>> = (0..7)
            .map(|i| {
                let t = f64::from(i) / 6.0;
                Point3::new(t, (3.0 * t * (1.0 - t)).powi(2), 0.3 * t.powi(2))
            })
            .collect();
        NurbsCurve3::interpolate(&pts, 3).unwrap()
    }

    /// The hull-bounded sagitta generalization: the chord count sizes
    /// the secant deviation under δ_s — verified against a dense
    /// per-segment sampling oracle.
    #[test]
    fn nurbs_chords_bound_the_secant_deviation() {
        let n = wiggle();
        let (d0, d1) = n.knots().domain();
        for delta_s in [1e-2, 1e-3, 1e-4] {
            let count =
                nurbs_chord_count(&n, d1 - d0, delta_s, EdgeKey::default()).expect("in inventory");
            assert!(count >= 1);
            #[allow(clippy::cast_precision_loss)]
            let h = (d1 - d0) / count as f64;
            let mut worst = 0.0f64;
            for seg in 0..count {
                #[allow(clippy::cast_precision_loss)]
                let a = d0 + h * seg as f64;
                let b = a + h;
                let (pa, pb) = (n.eval(a), n.eval(b));
                for k in 1..32 {
                    let t = a + h * f64::from(k) / 32.0;
                    let p = n.eval(t);
                    let lam = f64::from(k) / 32.0;
                    let chord = Point3::new(
                        pa.x + (pb.x - pa.x) * lam,
                        pa.y + (pb.y - pa.y) * lam,
                        pa.z + (pb.z - pa.z) * lam,
                    );
                    worst = worst.max((p - chord).norm());
                }
            }
            assert!(
                worst <= delta_s * 1.0000001,
                "secant deviation {worst} exceeds the certified budget {delta_s}"
            );
        }
    }

    /// Review probe (adopted): the SPIKE carrier — an interpolated
    /// cubic with one control point far off the line concentrates
    /// |C″| in one span, driving the measured secant deviation to
    /// 0.990–0.999 of the certified budget (the review's
    /// falsification sweep). The pin is two-sided: never OVER budget
    /// (soundness), and at least half of it (the fixture stays
    /// adversarial — a slack rewrite of the bound fails here too).
    #[test]
    fn adversarial_spike_stays_inside_but_near_the_budget() {
        let pts: Vec<Point3<f64>> = [
            (0.0, 0.0),
            (0.2, 0.01),
            (0.4, 0.02),
            (0.5, 0.9),
            (0.6, 0.02),
            (0.8, 0.01),
            (1.0, 0.0),
        ]
        .iter()
        .map(|&(x, y)| Point3::new(x, y, 0.0))
        .collect();
        let n = NurbsCurve3::interpolate(&pts, 3).unwrap();
        let (d0, d1) = n.knots().domain();
        for delta_s in [1e-2, 1e-3, 1e-4] {
            let count =
                nurbs_chord_count(&n, d1 - d0, delta_s, EdgeKey::default()).expect("in inventory");
            #[allow(clippy::cast_precision_loss)]
            let h = (d1 - d0) / count as f64;
            let mut worst = 0.0f64;
            for seg in 0..count {
                #[allow(clippy::cast_precision_loss)]
                let a = d0 + h * seg as f64;
                let b = a + h;
                let (pa, pb) = (n.eval(a), n.eval(b));
                for k in 1..64 {
                    let t = a + h * f64::from(k) / 64.0;
                    let p = n.eval(t);
                    let lam = f64::from(k) / 64.0;
                    let chord = Point3::new(
                        pa.x + (pb.x - pa.x) * lam,
                        pa.y + (pb.y - pa.y) * lam,
                        pa.z + (pb.z - pa.z) * lam,
                    );
                    worst = worst.max((p - chord).norm());
                }
            }
            assert!(
                worst <= delta_s * 1.0000001,
                "spike deviation {worst} exceeds the certified budget {delta_s}"
            );
            assert!(
                worst >= delta_s * 0.5,
                "spike deviation {worst} fell below half the budget {delta_s} — the \
                 adversarial fixture went slack (review measured 0.990-0.999)"
            );
        }
    }

    /// Rational carriers refuse typed, naming the honest blocker.
    #[test]
    fn rational_nurbs_carrier_refuses_typed() {
        let n = wiggle();
        let kv = n.knots().clone();
        let weights = vec![0.9; n.control().len()];
        let rational = NurbsCurve3::new(kv, n.control().to_vec(), weights).unwrap();
        match nurbs_chord_count(&rational, 1.0, 1e-3, EdgeKey::default()) {
            Err(TessellateError::UnsupportedCurve { note, .. }) => {
                assert!(note.contains("rational"), "{note}");
            }
            other => panic!("expected the rational refusal, got {other:?}"),
        }
    }
}
