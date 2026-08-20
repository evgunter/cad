//! Carrier-parameter recovery (Leg B's derived half): the exported
//! subset carries **no trim parameters** — an `EDGE_CURVE` is a
//! carrier plus two vertices — so the parameter interval is derived
//! from the vertex positions against the exact carrier fields.
//!
//! Convention (ISO 10303-42's, and the writer's): the edge runs from
//! `start` to `end` in the carrier's **increasing parameter**
//! direction; on periodic carriers the arc is the increasing-parameter
//! arc from the start angle, and a self-loop (start == end vertex) is
//! the full period. The derived endpoints are then *pinned* by the
//! kernel's own certification gates when the edge is adopted — a wrong
//! derivation cannot survive to rest.

use geom_core::{Point3, Real, Vec3};
use geom_curves::Curve3;

use crate::error::StepImportError;

/// Negative zero normalized to `+0.0`, componentwise — the same
/// hygiene the record reader applies to every literal it parses
/// (`entities::as_real`), applied to values the importer DERIVES.
///
/// It matters for one reason beyond tidiness: a minted direction like
/// `−(axis × u_ref)` picks up `−0.0` components from negating exact
/// zeros, the writer prints them as `-0.0`, and a re-import normalizes
/// them back to `0.0` — so the adoption pass would not be a fixed
/// point of the writer over a single printed sign. Normalizing at the
/// mint keeps one representative everywhere and moves no value
/// (`-0.0 == 0.0`).
///
/// The same argument covers every frame RECOGNITION derives —
/// `center`/`axis`/`u_ref`/`origin` on a promoted analytic surface or
/// curve are minted, not read, so a promoted body's one-cycle
/// re-export fixed point misses by exactly those sign bits unless they
/// are normalized here too.
pub(crate) fn plus_zero(v: Vec3<f64>) -> Vec3<f64> {
    Vec3::new(
        plus_zero_scalar(v.x),
        plus_zero_scalar(v.y),
        plus_zero_scalar(v.z),
    )
}

/// [`plus_zero`] for a point.
pub(crate) fn plus_zero_point(p: Point3<f64>) -> Point3<f64> {
    Point3::new(
        plus_zero_scalar(p.x),
        plus_zero_scalar(p.y),
        plus_zero_scalar(p.z),
    )
}

/// [`plus_zero`] on one component. NaN passes through unchanged: this
/// states a representative, it never certifies one.
fn plus_zero_scalar(x: f64) -> f64 {
    if x == 0.0 { 0.0 } else { x }
}

/// The carrier parameters of `p_start` / `p_end` (module docs).
///
/// # Errors
///
/// [`StepImportError::Topology`] when the vertex data cannot sit on
/// the carrier in the subset's convention (a line traversed against
/// its own direction, a degenerate carrier).
pub(crate) fn endpoint_params(
    id: u64,
    carrier: &Curve3<f64>,
    p_start: Point3<f64>,
    p_end: Point3<f64>,
    self_loop: bool,
) -> Result<(f64, f64), StepImportError> {
    match carrier {
        Curve3::Line { origin, dir } => {
            if self_loop {
                return Err(StepImportError::Topology {
                    id,
                    what: "a LINE edge with one vertex at both ends (a closed line \
                           has no interval)",
                });
            }
            let t0 = (p_start - *origin).dot(*dir);
            let t1 = (p_end - *origin).dot(*dir);
            // partial_cmp: a NaN projection must refuse too.
            if t1.partial_cmp(&t0) != Some(std::cmp::Ordering::Greater) {
                return Err(StepImportError::Topology {
                    id,
                    what: "a LINE edge whose end projects at or before its start — \
                           the subset's carriers run start → end in increasing \
                           parameter",
                });
            }
            Ok((t0, t1))
        }
        Curve3::Circle {
            center,
            axis,
            u_ref,
            ..
        }
        | Curve3::Ellipse {
            center,
            axis,
            u_ref,
            ..
        } => {
            // The start angle in [0, τ), the end angle in (t0, t0 + τ]
            // — the increasing-parameter arc. For the ellipse the
            // angle is the eccentric anomaly: dividing the in-plane
            // components by the semi-axes reduces it to the circle
            // case (atan2 is scale-invariant per axis).
            let v_ref = axis.cross(*u_ref);
            let (maj, min) = match carrier {
                Curve3::Ellipse { major, minor, .. } => (*major, *minor),
                _ => (1.0, 1.0),
            };
            let angle = |p: Point3<f64>| -> f64 {
                let w = p - *center;
                (w.dot(v_ref) / min).atan2(w.dot(*u_ref) / maj)
            };
            let tau = f64::tau();
            let mut t0 = angle(p_start);
            if t0 < 0.0 {
                t0 += tau;
            }
            if self_loop {
                return Ok((t0, t0 + tau));
            }
            let mut t1 = angle(p_end);
            while t1 <= t0 {
                t1 += tau;
            }
            if !t1.is_finite() || !t0.is_finite() {
                return Err(StepImportError::Topology {
                    id,
                    what: "a conic edge whose vertex angles are not finite (vertex \
                           at the conic's centre, or non-finite data)",
                });
            }
            Ok((t0, t1))
        }
        Curve3::Nurbs(payload) => {
            // Clamped B-splines interpolate their end control points,
            // so the subset's B-spline edges span the whole knot
            // domain; the adoption gate pins the vertex positions to
            // the domain ends within ε (a trimmed B-spline edge would
            // need parameter inversion — M7-2 vocabulary).
            let (a, b) = payload.domain();
            Ok((a, b))
        }
    }
}
