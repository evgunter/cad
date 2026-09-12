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

use geom::Curve3;
use geom_core::{Point3, Real};

use crate::error::StepImportError;

/// The carrier parameters of `p_start` / `p_end` (module docs).
///
/// # Errors
///
/// [`StepImportError::Topology`] when the vertex data cannot sit on
/// the carrier in the subset's convention (a line traversed against
/// its own direction, a degenerate carrier), or when the parameter it
/// would derive is not finite. Both parameters this answers are
/// finite; every arm refuses rather than return one that is not.
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
            // Refused where it is derived, for the same reason as
            // the conic arm below: an ordering test cannot stand in
            // for a finiteness test. `+∞ > 0.0` is `Greater`, so a
            // projection that has run out of exponent passes the
            // comparison and escapes as a parameter.
            let project = |p: Point3<f64>| -> Result<f64, StepImportError> {
                let t = (p - *origin).dot(*dir);
                if t.is_finite() {
                    Ok(t)
                } else {
                    Err(StepImportError::Topology {
                        id,
                        what: "a LINE edge with a vertex whose projection onto the \
                               carrier is not finite (a coordinate, or its difference \
                               from the line's origin, that overflows)",
                    })
                }
            };
            let t0 = project(p_start)?;
            let t1 = project(p_end)?;
            // Both finite, so a total comparison says all of it.
            if t1 <= t0 {
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
            // Each angle is refused where it is derived, so the
            // self-loop arm's early return cannot outrun the check:
            // that arm has a start vertex and no end vertex, so a
            // refusal sited after both angles is unreachable from it.
            // `atan2` is total over finite arguments — the centre
            // answers 0, not NaN — so a non-finite angle means
            // non-finite vertex or placement data, or a semi-axis of
            // zero, reached it. Both returned parameters are therefore
            // finite, and the wrap loop below terminates.
            let angle = |p: Point3<f64>| -> Result<f64, StepImportError> {
                let w = p - *center;
                let t = (w.dot(v_ref) / min).atan2(w.dot(*u_ref) / maj);
                if t.is_finite() {
                    Ok(t)
                } else {
                    Err(StepImportError::Topology {
                        id,
                        what: "a conic edge with a vertex whose angle about the \
                               centre is not finite (a coordinate, or its difference \
                               from the centre, that overflows; or a semi-axis of \
                               zero under a zero offset)",
                    })
                }
            };
            let tau = f64::tau();
            let mut t0 = angle(p_start)?;
            if t0 < 0.0 {
                t0 += tau;
            }
            if self_loop {
                return Ok((t0, t0 + tau));
            }
            let mut t1 = angle(p_end)?;
            while t1 <= t0 {
                t1 += tau;
            }
            Ok((t0, t1))
        }
        Curve3::Nurbs(payload) => {
            // Clamped B-splines interpolate their end control points,
            // so the subset's B-spline edges span the whole knot
            // domain; the adoption gate pins the vertex positions to
            // the domain ends within ε (a trimmed B-spline edge would
            // need parameter inversion — M7-2 vocabulary).
            //
            // No finiteness refusal here, unlike the arms above: the
            // domain is knot values, and a knot vector refuses a
            // non-finite knot at construction, so there is nothing
            // left for this arm to catch.
            let (a, b) = payload.domain();
            Ok((a, b))
        }
    }
}

#[cfg(test)]
#[allow(clippy::panic)]
mod tests {
    use super::endpoint_params;
    use crate::error::StepImportError;
    use geom::Curve3;
    use geom_core::{Point3, Vec3};

    /// The refusals' text says "a coordinate, or its DIFFERENCE from
    /// the centre/origin, that overflows". The second half is what the
    /// door cannot demonstrate — a vertex far enough out to overflow a
    /// subtraction inverts the face it sits on, and the orientation
    /// gate refuses the file first — so it is pinned here, where the
    /// function can be called with the offending pair directly.
    ///
    /// Every coordinate below is finite. `HUGE - -HUGE` is not.
    const HUGE: f64 = 1.0e308;

    #[test]
    fn a_finite_pair_whose_difference_overflows_refuses_on_the_conic_arm() {
        let carrier = Curve3::Circle {
            center: Point3::new(0.0, 0.0, -HUGE),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let p = Point3::new(1.0, 0.0, HUGE);
        match endpoint_params(7, &carrier, p, p, true) {
            Err(StepImportError::Topology { id, what }) => {
                assert_eq!(id, 7);
                assert!(what.contains("not finite"), "{what}");
            }
            other => panic!("expected a finiteness refusal, got {other:?}"),
        }
    }

    #[test]
    fn a_finite_pair_whose_difference_overflows_refuses_on_the_line_arm() {
        // Chosen so the escape is an `Ok`, not a refusal with the
        // wrong sentence: the start projects to a finite `HUGE` and
        // the end to `+∞`, and `+∞ > HUGE` is `Greater`.
        let carrier = Curve3::Line {
            origin: Point3::new(0.0, 0.0, -HUGE),
            dir: Vec3::new(0.0, 0.0, 1.0),
        };
        let p_start = Point3::new(0.0, 0.0, 0.0);
        let p_end = Point3::new(0.0, 0.0, HUGE);
        match endpoint_params(9, &carrier, p_start, p_end, false) {
            Err(StepImportError::Topology { id, what }) => {
                assert_eq!(id, 9);
                assert!(what.contains("not finite"), "{what}");
            }
            other => panic!("expected a finiteness refusal, got {other:?}"),
        }
    }
}
