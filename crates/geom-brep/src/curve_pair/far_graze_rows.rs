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
/// its arc (`Ellipse::anchored`, its seam then turned under the anchor so
/// the anchor sits at any of the four vertices' phases: no public door
/// anchors one at a general phase), `stored` the placement it stores.
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
    let conic = Curve3::Ellipse(if arc_held {
        // Anchored at its seam vertex, then the seam turned a random
        // quarter or half turn under the anchor, so the anchor's phase is
        // any of the four vertices'.
        let turn = rng.range(0.0, TAU);
        geom::Ellipse::anchored(centre + u * a, axis, u, a, b).seamed_toward(turn.cos(), turn.sin())
    } else {
        geom::Ellipse::new(geom::Frame::new(centre, axis, u), a, b)
    });
    Graze {
        stored: stored_placement(&conic),
        conic,
        crest,
        outward,
        along: axis.cross(outward),
        axis,
    }
}

/// **The placement `conic` stores**, read through its datum walk
/// ([`Curve3::data`]): the stored point (centre or anchor), axis, seam,
/// semi-axes, and an anchor's phase.
fn stored_placement(conic: &Curve3<f64>) -> StoredHarmonic<f64> {
    use geom::{CurveDatum as D, DatumValue as V};
    let point = |d| match conic.data().named(d) {
        Some(V::Point(p)) => Some([p.x, p.y, p.z]),
        Some(V::Direction(v)) => Some([v.x, v.y, v.z]),
        _ => None,
    };
    let scalar = |d| match conic.data().named(d) {
        Some(V::Scalar(x)) => x,
        other => panic!("{d:?} is no scalar: {other:?}"),
    };
    let (at, phase) = match (point(D::Center), point(D::Anchor), point(D::Phase)) {
        (Some(c), None, None) => (c, (0.0, 0.0)),
        (None, Some(x), Some(p)) => (x, (p[0], p[1])),
        other => panic!("no stored point: {other:?}"),
    };
    StoredHarmonic {
        at,
        axis: point(D::Axis).unwrap(),
        u_ref: point(D::URef).unwrap(),
        semi: (scalar(D::SemiU), scalar(D::SemiV)),
        phase,
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

/// `(wrong, beyond, definite, placed)` of 1000 poses read at `far`: a
/// line in the plane of an ellipse held at its centre, along its tangent
/// at the graze, set off it by a gap within twice the band's escalation,
/// its origin by the arc. Judged across the plane through the line's
/// origin along the pose's own in-plane normal `m`, the question's
/// direction: the belly's change with `m` is levered by the graze's
/// distance from the line's origin, metres, not `far`. A meeting counts
/// as definite only where it is decided with no window.
///
/// `placed` counts apart, out of `wrong`, the poses answered `Apart`
/// whose exact belly lies within the band's zero. There the belly reads
/// a touch, and the touch is placed on the spans by the carrier walk's
/// `clear_by` on points evaluated about the far centre. That walk reads
/// the conic clear of its own point at `u·far`, so the candidate is
/// dropped. It is not the belly's reading
/// (`work/conic/a-centre-held-far-conics-touch-is-dropped-by-a-carrier-walk-on-points-evaluated-about-its-centre.md`).
fn far_line_grazes(far: f64) -> (usize, usize, usize, usize) {
    let band = band();
    let mut rng = test_utils::fuzz::start("curve_pair::far_line_grazes");
    let (mut wrong, mut beyond, mut definite, mut placed) = (0, 0, 0, 0);
    for _ in 0..test_utils::fuzz::scaled(1000) {
        let g = graze(&mut rng, far, false);
        let gap = rng.range(-2.0, 2.0) * band.escalate();
        let origin = g.crest + g.outward * gap + g.along * rng.range(-3.0, 3.0);
        let line = Curve3::line(origin, geom::stated_direction(g.along));
        let Curve3::Line(stored) = line else {
            unreachable!("a line")
        };
        let axis = Vec3::from_array(g.stored.axis);
        let m = axis.cross(stored.dir.get()).normalize();
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
            let (o, n) = ([origin.x, origin.y, origin.z], [m.x, m.y, m.z]);
            let zero = band.zero() * (1.0 + 64.0 * f64::EPSILON);
            let touch = !stored_belly::exceeds(g.stored, o, n, zero)
                && !stored_belly::below(g.stored, o, n, -zero);
            placed += usize::from(w && !meets && touch);
        }
    }
    (wrong, beyond, definite, placed)
}

/// **A line grazing a far-centred conic is read wrong nowhere past the
/// touch's placement**, at any angle round the turn. The pose charges no
/// rounding (`[simple.conditioned]`), so no reading may be definite where
/// the exact belly is not, except an `Apart` on a belly within the band's
/// zero, which [`far_line_grazes`] counts apart as `placed`. That one is
/// the carrier walk's, not the belly's. The belly's own reading of a
/// touch is held without that exception by
/// `curve_pair::pose::far_rows::the_harmonics_near_extreme_reads_a_far_centred_conics_belly`.
/// Held at its centre only: for an ellipse held on its arc,
/// `line_conic`'s router height and slant still read the derived centre,
/// and they are ONEROUND's
/// (`work/oneround/an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre.md`).
#[test]
fn a_line_grazing_a_far_centred_conic_is_read_wrong_nowhere_past_the_touchs_placement() {
    let got: Vec<_> = [1e4, 1e5, 1e6]
        .map(|far| (far, far_line_grazes(far)))
        .to_vec();
    println!("(far, (wrong, beyond the escalation, definite, placed)): {got:?}");
    assert!(
        got.iter().all(|g| g.1.0 == g.1.3),
        "read wrong past the touch's placement: {got:?}; {}",
        test_utils::fuzz::replay()
    );
}
