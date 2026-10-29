//! **Two tori decided one surface once** ([`geom_brep::tori_coincide`]):
//! the pose reads the decision before any other question, on poses the
//! coaxial reading alone used to decide otherwise, and the decision
//! answers a sampled oracle of the two surfaces' distance.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/pair_section/torus_coincidence.rs",
    "crates/geom-brep/src/pair_section/torus.rs",
    "crates/geom-brep/tests/shared/"
];

use crate::shared::tol::band;
use core::f64::consts::TAU;
use geom::Surface;
use geom_brep::{
    TorusCoincidence, TorusData, TorusDisplacement, TorusPoseRefusal, TorusQuestion,
    TorusTorusPose, tori_coincide,
};
use geom_core::Band;
use geom_core::{Point3, Vec3};
use test_utils::fuzz;

fn data(center: Point3<f64>, axis: Vec3<f64>, big: f64, small: f64) -> TorusData<f64> {
    TorusData {
        center,
        axis: axis.normalize(),
        major_radius: big,
        minor_radius: small,
    }
}

fn surface(t: TorusData<f64>) -> Surface<f64> {
    Surface::Torus {
        center: t.center,
        axis: t.axis,
        major_radius: t.major_radius,
        minor_radius: t.minor_radius,
        u_ref: t.axis.orthonormal_basis().0,
    }
}

fn pose(a: TorusData<f64>, b: TorusData<f64>) -> Result<TorusTorusPose<f64>, TorusPoseRefusal> {
    TorusTorusPose::decide(&surface(a), &surface(b), band()).expect("two tori")
}

/// **The pose reads the pair's one decision, in every stance.** About
/// a ring of `R = 5`, `r = 0.06`: tilted so the far ring swings `0.74ε`
/// (a tilt the pose's own lever, twice the ball, reads in the band, so
/// its gap read the pair tangent), slid `0.6ε` along and across the axis
/// (`0.85ε` apart, which the coaxial reading summed to `1.2ε` across its
/// slack and escalated), and the tube `0.95ε` wider: one surface at the
/// decision and the pose alike. The tube in the middle of the band is
/// undecided at both, on its own row; a thousand escalation widths out
/// it is apart, and the pose goes on to the tube rows.
#[test]
fn the_pose_reads_two_tori_one_where_the_decision_does() {
    let b = band();
    let e = b.zero();
    let z = Vec3::unit_z();
    let ring = data(Point3::origin(), z, 5.0, 0.06);
    for (what, other) in [
        (
            "tilted",
            data(Point3::origin(), Vec3::new(0.148 * e, 0.0, 1.0), 5.0, 0.06),
        ),
        (
            "slid",
            data(Point3::new(0.6 * e, 0.0, 0.6 * e), z, 5.0, 0.06),
        ),
        ("wider", data(Point3::origin(), z, 5.0, 0.06 + 0.95 * e)),
    ] {
        assert!(
            matches!(tori_coincide(ring, other, b), TorusCoincidence::One(_)),
            "{what}: one surface"
        );
        assert!(
            matches!(pose(ring, other), Err(TorusPoseRefusal::Coincident)),
            "{what}: the pose refuses the coincidence"
        );
    }
    let in_band = data(Point3::origin(), z, 5.0, 0.06 + 0.5 * (e + b.escalate()));
    match (tori_coincide(ring, in_band, b), pose(ring, in_band)) {
        (
            TorusCoincidence::Undecided(diag),
            Err(TorusPoseRefusal::Undecided {
                question: TorusQuestion::Symmetry,
                diag: at_pose,
            }),
        ) => {
            assert_eq!(diag.predicate, Some("carrier_torus_minor_radius"));
            assert_eq!(
                at_pose.predicate, diag.predicate,
                "the pose quotes the decision"
            );
        }
        other => panic!("the tube in the band: {other:?}"),
    }
    let apart = data(Point3::origin(), z, 5.0, 0.06 + 1000.0 * b.escalate());
    assert!(matches!(
        tori_coincide(ring, apart, b),
        TorusCoincidence::Apart
    ));
    assert!(pose(ring, apart).is_ok(), "apart, the pose is decided");
}

/// The exact distance from `q` to the torus `t`.
fn distance(q: Point3<f64>, t: TorusData<f64>) -> f64 {
    let d = q - t.center;
    let h = d.dot(t.axis);
    let rho = (d - t.axis * h).norm();
    ((rho - t.major_radius).hypot(h) - t.minor_radius).abs()
}

/// The farthest any of a grid of points on `a` stands from `b`: a lower
/// bound on how far `a` stands from `b`.
fn sampled_reach(a: TorusData<f64>, b: TorusData<f64>) -> f64 {
    let (e1, e2) = a.axis.orthonormal_basis();
    let (nu, nv) = (96, 48);
    let mut far = 0.0f64;
    for i in 0..nu {
        let u = TAU * f64::from(i) / f64::from(nu);
        let out = e1 * u.cos() + e2 * u.sin();
        for j in 0..nv {
            let v = TAU * f64::from(j) / f64::from(nv);
            let q = a.center
                + out * (a.major_radius + a.minor_radius * v.cos())
                + a.axis * (a.minor_radius * v.sin());
            far = far.max(distance(q, b));
        }
    }
    far
}

fn unit_vector(g: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0));
        let n = v.norm();
        if n > 0.2 && n < 1.0 {
            break v * (1.0 / n);
        }
    }
}

/// A perturbation's size: none, across the zero band's edge, across the
/// escalation width's, or past it.
fn size(g: &mut fuzz::Rng) -> f64 {
    let b = band();
    match g.below(5) {
        0 | 1 => 0.0,
        2 => g.range(0.0, 2.0) * b.zero(),
        3 => g.range(0.5, 1.5) * b.escalate(),
        _ => g.range(2.0, 20.0) * b.escalate(),
    }
}

/// **The counterexample search against the surfaces' distance.** A ring
/// torus in a generic frame, up to a few metres across, against a copy
/// moved by up to a few escalation widths in each datum: tilted about a
/// random diameter so its ring swings that far, slid, and each radius
/// changed. The oracle is the exact distance from a grid of points on
/// each torus to the other, which bounds how far the two stand apart
/// from below. The displacement bounds that distance from above; a pair
/// decided one stands within the zero band everywhere; a pair decided
/// apart stands past it somewhere. The pose refuses exactly the pairs
/// decided one as a coincidence. Mutant: the tilt left out of the bound
/// (a tilted pair stands past it).
#[test]
fn a_pair_decided_one_or_apart_stands_where_the_decision_says() {
    let mut g = fuzz::start("tori_coincide::oracle");
    let b = band();
    let mut tally = [0usize; 3];
    for _ in 0..fuzz::scaled(60) {
        let scale = g.range(0.5, 3.0);
        let axis = unit_vector(&mut g);
        let a = data(
            Point3::origin() + unit_vector(&mut g) * g.range(0.0, scale),
            axis,
            scale,
            scale * g.range(0.05, 0.5),
        );
        let swing = size(&mut g);
        let diameter = {
            let v = unit_vector(&mut g);
            (v - axis * v.dot(axis)).normalize()
        };
        let turn = swing / a.major_radius;
        let tilted = axis * turn.cos() + axis.cross(diameter) * turn.sin();
        let other = data(
            a.center + unit_vector(&mut g) * size(&mut g),
            tilted,
            a.major_radius + size(&mut g) * if g.unit() < 0.5 { -1.0 } else { 1.0 },
            a.minor_radius + size(&mut g) * if g.unit() < 0.5 { -1.0 } else { 1.0 },
        );
        let reach = sampled_reach(a, other).max(sampled_reach(other, a));
        let upper = TorusDisplacement::of(a, other).upper;
        // The oracle's own rounding: coordinates of the scale's size.
        let rounding = 64.0 * f64::EPSILON * 4.0 * scale;
        let what = format!("{a:?} against {other:?}: {}", fuzz::replay());
        assert!(
            reach <= upper + rounding,
            "the bound {upper:e} under the sampled distance {reach:e}: {what}"
        );
        let decided = tori_coincide(a, other, b);
        match decided {
            TorusCoincidence::One(_) => {
                tally[0] += 1;
                assert!(
                    reach <= b.zero() + rounding,
                    "decided one at a sampled distance {reach:e}: {what}"
                );
            }
            TorusCoincidence::Apart => {
                tally[1] += 1;
                assert!(
                    reach > b.zero(),
                    "decided apart at a sampled distance {reach:e}: {what}"
                );
            }
            TorusCoincidence::Undecided(_) => tally[2] += 1,
            TorusCoincidence::OffRing(refusal) => {
                panic!("a ring torus refused {refusal:?}: {what}")
            }
        }
        assert_eq!(
            matches!(pose(a, other), Err(TorusPoseRefusal::Coincident)),
            matches!(decided, TorusCoincidence::One(_)),
            "the pose refuses a coincidence exactly where the pair is decided one: {what}"
        );
    }
    eprintln!("[fuzz] tori_coincide::oracle: one/apart/undecided = {tally:?}");
}

/// **A small torus tangent inside the first's tube along a whole tube
/// circle**: centred on the first's core at `(2, 0, 0)`, its axis the
/// core's tangent there, with `R + r = 0.5`, so its outer equator is the
/// first's tube circle there. The pose refuses on the gap in both orders,
/// never on a coincidence.
#[test]
fn a_torus_tangent_inside_the_tube_along_a_tube_circle_refuses_at_the_gap() {
    let b = band();
    let first = surface(data(Point3::origin(), Vec3::unit_z(), 2.0, 0.5));
    let small = surface(data(
        Point3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.3,
        0.2,
    ));
    for (x, y) in [(&first, &small), (&small, &first)] {
        assert!(matches!(
            TorusTorusPose::decide(x, y, b).expect("tori"),
            Err(TorusPoseRefusal::Undecided {
                question: TorusQuestion::Gap,
                ..
            })
        ));
    }
}

/// **Every datum within the zero band beside a bound past the band
/// straddles**, which only an escalation factor under four reaches (the
/// bound is at most the four data, the tilt's chord within `√2` of its
/// sine): at a band of `(ε, 2ε)` three data of `0.9ε` each decide `Zero`
/// and sum to `2.7ε`. The band is pinned here, not the run's, because
/// the run's factor never reaches the arm.
#[test]
fn data_each_zero_beside_a_bound_past_the_band_straddle() {
    let b = Band::new(1e-9, 2e-9).unwrap();
    let e = b.zero();
    let ring = data(Point3::origin(), Vec3::unit_z(), 5.0, 0.5);
    let other = data(
        Point3::new(0.0, 0.0, 0.9 * e),
        Vec3::unit_z(),
        5.0 + 0.9 * e,
        0.5 + 0.9 * e,
    );
    match tori_coincide(ring, other, b) {
        TorusCoincidence::Undecided(diag) => {
            assert_eq!(diag.predicate, Some("tt_coincident_straddle"));
        }
        other => panic!("the straddle: {other:?}"),
    }
}

/// **A torus off its ring convention is refused before any datum**, and
/// a datum that is not finite reads as unreadable: a spindle (`r > R`)
/// refuses on `ring_torus_convention` at the decision and the pose
/// alike; a NaN centre leaves the pair undecided on an invalid margin.
#[test]
fn an_off_ring_or_poisoned_pair_is_refused_not_decided() {
    let b = band();
    let ring = data(Point3::origin(), Vec3::unit_z(), 2.0, 0.5);
    let spindle = data(Point3::origin(), Vec3::unit_z(), 1.0, 1.5);
    assert!(matches!(
        tori_coincide(ring, spindle, b),
        TorusCoincidence::OffRing(_)
    ));
    assert!(matches!(
        TorusTorusPose::decide(&surface(ring), &surface(spindle), b).expect("tori"),
        Err(TorusPoseRefusal::Torus(_))
    ));
    let poisoned = data(Point3::new(f64::NAN, 0.0, 0.0), Vec3::unit_z(), 2.0, 0.5);
    match tori_coincide(ring, poisoned, b) {
        TorusCoincidence::Undecided(diag) => assert!(diag.margin.is_invalid(), "{diag:?}"),
        other => panic!("the poisoned centre: {other:?}"),
    }
}
