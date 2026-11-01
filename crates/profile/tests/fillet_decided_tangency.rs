//! **A fillet on a decided offset tangency registers nothing it did
//! not prove.** Where a fillet's two offset carriers are decided
//! tangent (`fillet_offset_circles_*`, `fillet_offset_line_circle`
//! Zero), the one candidate centre is the point they would touch at,
//! on them only to that decision: its rims are the radius to ε, not
//! over the reals. Each corner here is built so its offset carriers
//! miss tangency by `δ` inside the band (`δ = ε/10`) and by nothing
//! (`δ = 0`), at `f64` and at `Interval`. A fillet that registered its
//! endpoint facts on the `δ > 0` corner would abort the `Interval`
//! build, the exact witness separating the rim at its start.
//!
//! Both corners are lens fillets: the fillet circle fills the region
//! between the two carriers at its widest, from `t1` to `t2` through a
//! half turn.

use geom_core::{Interval, Point2};
use profile::{ArcCarrierScalar, ArcSweep, Center, Open, PathError, Start};

use crate::common::tol;

fn pt<T: ArcCarrierScalar>(x: f64, y: f64) -> Point2<T> {
    Point2::new(T::from_f64(x), T::from_f64(y))
}

/// Two unit circles `1.5 + δ` apart, travelled counterclockwise from
/// 20° below the first's axis point to 20° below the second's, the
/// fillet of radius `0.25` in their lens at the upper corner: the
/// offset circles of radius `0.75` are `δ` short of touching.
fn circle_circle<T: ArcCarrierScalar>(delta: f64) -> Result<(), PathError<T>> {
    let w = tol();
    let (s, c) = 20f64.to_radians().sin_cos();
    Open.arc_fillet_arc(
        Center {
            c: pt::<T>(0.0, 0.0),
            winding: ArcSweep::Ccw,
            p: pt(c, -s),
        },
        T::from_f64(0.25),
        Center {
            c: pt(1.5 + delta, 0.0),
            winding: ArcSweep::Ccw,
            p: pt(1.5 + delta - c, -s),
        },
        w,
    )?
    .line_to(Start, w)
    .map(|_| ())
}

/// The vertical line `x = 0.5 + δ` travelled up, into the unit circle
/// travelled clockwise, the fillet of radius `0.25` in the circular
/// segment between them: the offset line `x = 0.75 + δ` is `δ` short of
/// the offset circle of radius `0.75`.
fn line_circle<T: ArcCarrierScalar>(delta: f64) -> Result<(), PathError<T>> {
    let w = tol();
    let (s, c) = (0.5, 30f64.to_radians().cos());
    Open.at(pt::<T>(0.5 + delta, -0.5))
        .toward(T::zero(), T::one(), w)?
        .fillet_arc(
            T::from_f64(0.25),
            Center {
                c: pt(0.0, 0.0),
                winding: ArcSweep::Cw,
                p: pt(c, -s),
            },
            w,
        )?
        .line_to(Start, w)
        .map(|_| ())
}

/// Both lanes build at `δ` = 0 and ε/10.
fn builds_at_every_scalar(
    corner: &str,
    at_f64: impl Fn(f64) -> Result<(), PathError<f64>>,
    at_interval: impl Fn(f64) -> Result<(), PathError<Interval>>,
) {
    for delta in [0.0, tol().eps() / 10.0] {
        let f = at_f64(delta).map_err(|e| format!("{e:?}"));
        assert_eq!(f, Ok(()), "{corner} at δ = {delta} on f64");
        let i = at_interval(delta).map_err(|e| format!("{e:?}"));
        assert_eq!(i, Ok(()), "{corner} at δ = {delta} on Interval");
    }
}

#[test]
fn a_circle_circle_fillet_on_a_decided_offset_tangency_builds_at_every_scalar() {
    builds_at_every_scalar(
        "circle×circle",
        circle_circle::<f64>,
        circle_circle::<Interval>,
    );
}

#[test]
fn a_line_circle_fillet_on_a_decided_offset_tangency_builds_at_every_scalar() {
    builds_at_every_scalar("line×circle", line_circle::<f64>, line_circle::<Interval>);
}
