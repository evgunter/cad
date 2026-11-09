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
use geom_core::period::Period;
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
        Curve3::Line(line) => {
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
                let t = line.param(p);
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
        Curve3::Circle(_) | Curve3::Ellipse(_) => {
            // The start angle in [0, τ), the end angle in (t0, t0 + τ]
            // — the increasing-parameter arc, its angle the conic's
            // principal one (an ellipse's eccentric anomaly).
            let Some(conic) = carrier.conic() else {
                unreachable!("a circle or an ellipse is a conic")
            };
            // Each angle is refused where it is derived, so the
            // self-loop arm's early return cannot outrun the check:
            // that arm has a start vertex and no end vertex, so a
            // refusal sited after both angles is unreachable from it.
            // The read is total over a finite direction (the centre
            // answers the reference, 0) and NaN over a non-finite one,
            // so a non-finite angle means non-finite vertex or
            // placement data reached it, and both returned parameters
            // are finite.
            let angle = |p: Point3<f64>| -> Result<f64, StepImportError> {
                let t = conic.azimuth(p).near(0.0);
                if t.is_finite() {
                    Ok(t)
                } else {
                    Err(StepImportError::Topology {
                        id,
                        what: "a conic edge with a vertex whose angle about the \
                               centre is not finite (a coordinate, or its difference \
                               from the centre, that overflows)",
                    })
                }
            };
            let turn = Period::new(f64::tau());
            let t0 = turn.at_or_above(angle(p_start)?, 0.0);
            if self_loop {
                return Ok((t0, t0 + turn.length()));
            }
            Ok((t0, turn.above(angle(p_end)?, t0)))
        }
        // No STEP entity is recognized as a spiric: the import mints
        // `Line`, `Circle`, `Ellipse` and `Nurbs` carriers only
        // (`entities.rs`), so this arm names a carrier this crate cannot
        // have built. Written for exhaustiveness; reaching it is a
        // kernel bug, not an input's.
        Curve3::Spiric { .. } => unreachable!(
            "endpoint_params: the STEP import mints no spiric carrier (no entity maps to one)"
        ),
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
        let carrier = Curve3::circle(
            Point3::new(0.0, 0.0, -HUGE),
            Vec3::new(0.0, 0.0, 1.0),
            1.0,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let p = Point3::new(1.0, 0.0, HUGE);
        match endpoint_params(7, &carrier, p, p, true) {
            Err(StepImportError::Topology { id, what }) => {
                assert_eq!(id, 7);
                assert!(what.contains("not finite"), "{what}");
            }
            other => panic!("expected a finiteness refusal, got {other:?}"),
        }
    }

    /// **A conic edge's interval is `[0, τ)` at its start and `(t0, t0 + τ]`
    /// at its end, exactly at both open ends.** A seam start reads `0`,
    /// not `τ`; an end at the start's own point, on an edge whose two
    /// vertices are distinct, reads a whole turn on, never a zero-length
    /// arc, at a start where `Period::near` about `t0 + π` would keep it.
    #[test]
    fn a_conic_intervals_open_ends_hold_exactly() {
        let carrier = Curve3::circle(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            1.0,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let seam = Point3::new(1.0, 0.0, 0.0);
        let below = Point3::new(0.0, -1.0, 0.0);
        let Ok((t0, t1)) = endpoint_params(1, &carrier, seam, below, false) else {
            panic!("an arc")
        };
        assert_eq!(t0.to_bits(), 0.0_f64.to_bits(), "a seam start reads 0");
        assert!(
            (t1 - 1.5 * core::f64::consts::PI).abs() < 1e-12,
            "the end three quarters on: {t1}"
        );
        let p = Point3::new(0.89_f64.cos(), 0.89_f64.sin(), 0.0);
        let Ok((t0, t1)) = endpoint_params(2, &carrier, p, p, false) else {
            panic!("an arc")
        };
        let turn = geom_core::period::Period::new(core::f64::consts::TAU);
        assert_eq!(
            turn.near(t0, t0 + core::f64::consts::PI).to_bits(),
            t0.to_bits(),
            "this start is one near about t0 + π keeps: {t0}"
        );
        assert_eq!(
            t1.to_bits(),
            (t0 + core::f64::consts::TAU).to_bits(),
            "an end at the start's point is a whole turn on"
        );
    }

    #[test]
    fn a_finite_pair_whose_difference_overflows_refuses_on_the_line_arm() {
        // Chosen so the escape is an `Ok`, not a refusal with the
        // wrong sentence: the start projects to a finite `HUGE` and
        // the end to `+∞`, and `+∞ > HUGE` is `Greater`.
        let carrier = Curve3::line(Point3::new(0.0, 0.0, -HUGE), Vec3::new(0.0, 0.0, 1.0));
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
