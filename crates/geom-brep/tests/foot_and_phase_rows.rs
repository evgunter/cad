//! **A closed-form foot and a harmonic's phase decide no escalation of
//! their own**: the pose's phase read keeps its cut-side choice on the
//! least band, under its own row, and the chart's circle foot reads its
//! angle onto the turn at `from`.

#![allow(clippy::unwrap_used)]

use core::f64::consts::TAU;

use geom::Curve3;
use geom_brep::curve_pair::{Pose, Span, pose};
use geom_core::k_stats::Bracket;
use geom_core::{Band, Interval, Point3, Real, Vec3};

/// A line crossing a unit circle twice, its normal's angle within the
/// band's gap of the angle window's cut at `−1`, read at `Interval`: the
/// phase's formula choice is a heuristic's, so it records nothing and
/// the pose reads its two crossings.
#[test]
fn a_phase_on_the_window_cut_records_no_escalation() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let z = Vec3::new(0.0, 0.0, 1.0);
    let circle = Curve3::circle(
        Point3::new(0.0, 0.0, 0.0),
        z,
        1.0,
        Vec3::new(1.0, 0.0, 0.0),
    );
    let lift = |c: &Curve3<f64>| c.map_scalar(Interval::from_f64);
    let x = Interval::from_f64;
    for delta in [-5e-9, 5e-9] {
        let phi = -1.0f64 + delta;
        let m = Vec3::new(phi.cos(), phi.sin(), 0.0);
        let dir = m.cross(z);
        let origin = Point3::new(0.0, 0.0, 0.0) + m * -0.5 + dir * -3.0;
        let (li, ci) = (
            lift(&Curve3::Line(geom::Line { origin, dir })),
            lift(&circle),
        );
        let bracket = Bracket::open();
        let got = pose(
            &Span::new(&li, (x(0.0), x(6.0))),
            &Span::new(&ci, (x(-3.0), x(3.0))),
            band,
        );
        let rec = bracket.finish();
        assert!(
            matches!(got, Ok(Pose::Generic(ref cs)) if cs.len() == 2),
            "{delta:e}: {got:?}"
        );
        let esc: Vec<_> = rec.escalations.iter().map(|e| e.predicate()).collect();
        assert!(esc.is_empty(), "{delta:e}: {esc:?}");
    }
}

/// The chart's circle foot is the point's angle, on `[from, from + τ]`,
/// and a point on the axis reads `from`.
#[test]
fn a_circle_foot_is_its_angle_on_the_turn_at_from() {
    let c = Point3::new(0.3, -0.2, 0.1);
    let k = Curve3::circle(c, Vec3::new(0.0, 0.0, 1.0), 2.0, Vec3::new(1.0, 0.0, 0.0));
    for from in [-7.0, -1.0, 0.0, 2.5, 9.0] {
        for ang in [0.2, 1.0, 3.1, 4.0, 6.0] {
            let p = k.eval(ang) + Vec3::new(0.0, 0.0, 0.7);
            let t = geom_brep::chart_singularity::foot(&k, p, from).unwrap();
            assert!(from <= t && t <= from + TAU, "{from} {ang}: {t}");
            let r = (t - ang).rem_euclid(TAU);
            assert!(r < 1e-12 || TAU - r < 1e-12, "{from} {ang}: {t}");
        }
        let on_axis = Point3::new(0.3, -0.2, 5.0);
        assert_eq!(
            geom_brep::chart_singularity::foot(&k, on_axis, from),
            Some(from),
            "{from}: on the axis"
        );
    }
}
