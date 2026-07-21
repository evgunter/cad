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
//!   torus face adds n ≥ ceil(Δt/h). This is the one place adjacent
//!   surfaces enter chord counts — chord points remain a pure function
//!   of (carrier + interval, endpoint points, adjacent surface
//!   parameters, δ).
//!
//! Polyline endpoints are the topology vertices' points **bitwise**
//! (never `carrier(t₀)`, which is only within ε of them) so every
//! polyline meeting at a vertex shares its mesh vertex id; interior
//! points are `carrier(t₀ + (t₁−t₀)·i/n)` in `he_plus`-forward order.

use std::collections::HashMap;

use geom_curves::Curve3;
use topo::{Body, EdgeKey};

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
pub(crate) fn compute_chords(
    body: &Body<f64>,
    delta_s: f64,
    vids: &HashMap<topo::VertexKey, u32>,
    positions: &mut Vec<geom_core::Point3<f64>>,
) -> Result<HashMap<EdgeKey, Vec<u32>>, TessellateError> {
    let mut chords = HashMap::new();
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
            Curve3::Nurbs => return Err(TessellateError::UnsupportedCurve { edge: ek }),
        };
        let (vs, ve) = edge_vertices(body, ek)?;
        let start_id = *vids.get(&vs).ok_or(TessellateError::MissingEntity {
            what: "start vertex",
        })?;
        let end_id = *vids
            .get(&ve)
            .ok_or(TessellateError::MissingEntity { what: "end vertex" })?;
        let mut ids = Vec::with_capacity(n + 1);
        ids.push(start_id);
        for i in 1..n {
            #[allow(clippy::cast_precision_loss)]
            let t = t0 + span * (i as f64 / n as f64);
            #[allow(clippy::cast_possible_truncation)]
            let id = positions.len() as u32;
            positions.push(curve.carrier().eval(t));
            ids.push(id);
        }
        ids.push(end_id);
        chords.insert(ek, ids);
    }
    Ok(chords)
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
