//! **A far-centred conic's graze is read to the belly's own digits**
//! ([`geom::Conic::belly_along`]): an ellipse of semi-axes of the order
//! `far`, centred `far` from the point of it a plane or a line grazes,
//! the graze at any angle round the turn and the ellipse held at its
//! centre or on its arc. Every row judges its readings exactly on the
//! stored data ([`geom_core::test_support::stored_belly`]), never through
//! the code's own terms.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::TAU;

use geom::Curve3;
use geom_core::test_support::stored_belly;
use geom_core::{Band, Point3, StoredHarmonic, Vec3};

use super::{Pose, Span, pose};

/// The band the rows read at: ε 1e-12.
pub(crate) fn band() -> Band {
    Band::linear_at(geom_core::Tol::witness(), 1e-12).unwrap()
}

/// **One graze**: an ellipse of semi-axes `far` and `far/3` in any
/// orientation, its point `crest` at the angle `θ*` round the turn lying
/// in the box, and its centre `far` off; `outward` the in-plane unit
/// normal there and `along` the unit tangent. Held at its centre, or on
/// its arc at its seam's vertex (`Ellipse::anchored`), `stored` the
/// placement it stores.
pub(crate) struct Graze {
    pub(crate) conic: Curve3<f64>,
    pub(crate) stored: StoredHarmonic<f64>,
    pub(crate) crest: Point3<f64>,
    pub(crate) outward: Vec3<f64>,
    pub(crate) along: Vec3<f64>,
    pub(crate) axis: Vec3<f64>,
}

pub(crate) fn graze(rng: &mut test_utils::fuzz::Rng, far: f64, arc_held: bool) -> Graze {
    let mut unit = || {
        Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        )
        .normalize()
    };
    let axis = unit();
    let u = unit().cross(axis).normalize();
    let v = axis.cross(u);
    let (a, b) = (far, far / 3.0);
    let theta = rng.range(0.0, TAU);
    let crest = Point3::new(
        rng.range(-500.0, 500.0),
        rng.range(-500.0, 500.0),
        rng.range(-500.0, 500.0),
    );
    let centre = crest - (u * (a * theta.cos()) + v * (b * theta.sin()));
    let outward = (u * (b * theta.cos()) + v * (a * theta.sin())).normalize();
    let (frame, phase, conic) = if arc_held {
        let anchor = centre + u * a;
        let conic = geom::Ellipse::anchored(anchor, axis, u, a, b);
        (geom::Frame::new(anchor, axis, u), (1.0, 0.0), conic)
    } else {
        let frame = geom::Frame::new(centre, axis, u);
        (frame, (0.0, 0.0), geom::Ellipse::new(frame, a, b))
    };
    let arr = |p: Vec3<f64>| [p.x, p.y, p.z];
    let o = frame.origin();
    Graze {
        conic: Curve3::Ellipse(conic),
        stored: StoredHarmonic {
            at: [o.x, o.y, o.z],
            axis: arr(frame.axis()),
            u_ref: arr(frame.u_ref()),
            semi: (a, b),
            phase,
        },
        crest,
        outward,
        along: axis.cross(outward),
        axis,
    }
}

/// **A definite reading judged** against the exact belly of `stored`
/// across the plane through `o` with normal `n`: `(wrong, beyond)`,
/// wrong where it is definite on a belly the band does not decide
/// (within `Kε`, less the reading's own rounding at the threshold) or
/// against the exact sign, and beyond where it is against the sign past
/// `Kε`.
pub(crate) fn judge(
    stored: StoredHarmonic<f64>,
    (o, n): (Point3<f64>, Vec3<f64>),
    meets: bool,
    band: Band,
) -> (bool, bool) {
    let (o, n) = ([o.x, o.y, o.z], [n.x, n.y, n.z]);
    let e = band.escalate();
    let threshold = e * (1.0 - 64.0 * f64::EPSILON);
    let decided = stored_belly::exceeds(stored, o, n, threshold)
        || stored_belly::below(stored, o, n, -threshold);
    let against = meets != stored_belly::exceeds(stored, o, n, 0.0);
    let past = stored_belly::exceeds(stored, o, n, e) || stored_belly::below(stored, o, n, -e);
    (!decided || against, against && past)
}

/// `(wrong, beyond, definite)` of 600 poses read at `far`: a line in the
/// ellipse's plane along its tangent at the graze, set off it by a gap
/// within twice the band's escalation, its origin by the arc. Judged
/// across the plane through the line's origin along the pose's own in-plane
/// normal `m`, the question's direction: the belly's change with `m` is
/// levered by the graze's distance from the line's origin, metres, not
/// `far`. A meeting counts as definite only where it is decided with no
/// window.
fn far_line_grazes(far: f64, arc_held: bool) -> (usize, usize, usize) {
    let band = band();
    let mut rng = test_utils::fuzz::start("curve_pair::far_line_grazes");
    let (mut wrong, mut beyond, mut definite) = (0, 0, 0);
    for _ in 0..test_utils::fuzz::scaled(600) {
        let g = graze(&mut rng, far, arc_held);
        let gap = rng.range(-2.0, 2.0) * band.escalate();
        let origin = g.crest + g.outward * gap + g.along * rng.range(-3.0, 3.0);
        let line = Curve3::line(origin, g.along);
        let Curve3::Line(stored) = line else {
            unreachable!("a line")
        };
        let axis = Vec3::from_array(g.stored.axis);
        let m = axis.cross(stored.dir.normalize()).normalize();
        let read = match pose(
            &Span::new(&line, (-10.0, 10.0)),
            &Span::new(&g.conic, (0.0, TAU)),
            band,
        ) {
            Ok(Pose::Apart) => Some(false),
            Ok(Pose::Generic(found)) if found.iter().any(|c| c.bound.is_none()) => Some(true),
            _ => None,
        };
        if let Some(meets) = read {
            definite += 1;
            let (w, b) = judge(g.stored, (stored.origin, m), meets, band);
            wrong += usize::from(w);
            beyond += usize::from(b);
        }
    }
    (wrong, beyond, definite)
}

/// **A line grazing a far-centred conic is read wrong nowhere**, at any
/// angle round the turn, the conic held at its centre or on its arc: the
/// pose charges no rounding (`[simple.conditioned]`), so no reading may
/// be definite where the exact belly is not.
#[test]
fn a_line_grazing_a_far_centred_conic_is_read_wrong_nowhere() {
    let got: Vec<_> = [false, true]
        .into_iter()
        .flat_map(|held| [1e4, 1e5, 1e6].map(|far| (held, far, far_line_grazes(far, held))))
        .collect();
    println!("(arc-held, far, (wrong, beyond the escalation, definite)): {got:?}");
    assert!(
        got.iter().all(|g| g.2.0 == 0),
        "read wrong: {got:?}; {}",
        test_utils::fuzz::replay()
    );
}
