//! **The carry rows**: a spline wall's section against a plane and a
//! sphere, read whole and carried. A branch between two rim ends is
//! carried as the reading fitted it, its pad in metres; a loop on the
//! chart carries none, and a classification that reaches is not the
//! listing the carry takes. Each row names the mutant that turns it red,
//! and runs at the run's ε.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};

use crate::pair_section::{
    CarriedComponent, CarryError, CarryRefusal, ChartReach, ComponentShape, NoCarrier,
    PairSection, SectionRead, SsiLane, pair_section,
};

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the run's tolerance forms a band")
}

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// **A lofted wall, scaled by `s`**: linear in `u` from `x = −1` to
/// `x = 1`, quadratic in `v` through the heights `z = 0, 1, 2` with the
/// middle row bulged out to `y = 1.3` and the end rows at `y = 1`, every
/// coordinate times `s`. Its height is `z = 2v` exactly, so the plane
/// `z = s` cuts it along the iso-line `v = ½`, from the side `u = 0` to
/// the side `u = 1`.
fn wall(s: f64) -> Surface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = [-1.0, 1.0]
        .into_iter()
        .flat_map(|x| [(1.0, 0.0), (1.3, 1.0), (1.0, 2.0)].map(|(y, z)| pt(s * x, s * y, s * z)))
        .collect();
    Surface::Nurbs(std::sync::Arc::new(
        NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap(),
    ))
}

/// The plane `z = s`.
fn level(s: f64) -> Surface<f64> {
    Surface::Plane {
        origin: pt(0.0, 0.0, s),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// The sphere about `s·c` of radius `s·r`.
fn ball(s: f64, c: (f64, f64, f64), r: f64) -> Surface<f64> {
    Surface::Sphere {
        center: pt(s * c.0, s * c.1, s * c.2),
        radius: s * r,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// `a` against `b` classified through the march, then read whole: the
/// listing the carry takes.
fn listed(a: &Surface<f64>, b: &Surface<f64>) -> PairSection<f64> {
    let lane = Some(SsiLane::marched());
    match pair_section(a, b, None, lane, band()).expect("the pair classifies") {
        SectionRead::Built(s) => s.read_whole(lane, band()).expect("the pair reads whole"),
        SectionRead::Elsewhere(_) => panic!("a spline × implicit pair is built here"),
    }
}

/// The section's one carried component.
fn one(section: &PairSection<f64>) -> CarriedComponent<f64> {
    let carried = section
        .carry(Some(SsiLane::marched()), band())
        .expect("the section carries");
    let [c] = carried.components() else {
        panic!("one component: {:?}", carried.components().len());
    };
    c.clone()
}

/// The wall's point at `(u, v)`.
fn on_wall(w: &Surface<f64>, u: f64, v: f64) -> Point3<f64> {
    let Surface::Nurbs(net) = w else {
        unreachable!("the fixture's wall is a spline")
    };
    net.eval(u, v)
}

/// **A branch between two rim ends is carried as the reading fitted it,
/// its pad in metres**, at scales far from 1 either way, against a plane
/// (the iso-line `v = ½`) and a sphere (an arc from the bottom side back
/// to it). The component is open on the wall's rim, holds the reading's
/// certificate, and its pad holds the certificate's limb-2 bound; every
/// point of the section read off the wall lies on it within its pad, and
/// the pad is the tube's region in metres, about its rung.
///
/// Red against: a carry that refuses every spline × implicit component
/// (the listing's `Unminted`); a pad left in chart units (the tube's
/// `pad_u`, `pad_v`), far past the rung at `s = 0.01` and far inside it
/// at `s = 1000`; a pad read off whole windows, past four rungs.
#[test]
fn a_rim_arc_is_carried_as_its_reading_fitted_it_with_its_pad_in_metres() {
    for s in [1e-2, 1.0, 1e2, 1e3] {
        let w = wall(s);
        // The plane: the section is `S(u, ½)`.
        let c = one(&listed(&level(s), &w));
        // The sphere about the bottom side's middle, pushed out past the
        // wall's rim line: an arc from the side `v = 0` back to it.
        let b = ball(s, (0.0, 1.1, 0.0), 0.4);
        let sphere_point = |u: f64| {
            // The section on the iso-line `u`: the root in `v` of the
            // sphere's residual, bisected from the rim (inside the ball)
            // out to `v = ½` (outside it).
            let f = |v: f64| {
                let p = on_wall(&w, u, v);
                let c = pt(0.0, 1.1 * s, 0.0);
                (p - c).norm() - 0.4 * s
            };
            let (mut lo, mut hi) = (0.0, 0.5);
            assert!(f(lo) < 0.0 && f(hi) > 0.0, "s = {s}: the iso-line {u} crosses the ball");
            for _ in 0..80 {
                let mid = 0.5 * (lo + hi);
                if f(mid) < 0.0 {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            on_wall(&w, u, 0.5 * (lo + hi))
        };
        let mut cases = vec![(
            "the plane",
            c,
            (0..=20)
                .map(|i| on_wall(&w, f64::from(i) / 20.0, 0.5))
                .collect::<Vec<_>>(),
        )];
        // The sphere's whole reading refuses at `s = 1000` before any
        // carry (`work/sectarms/a-sphere-against-a-spline-wall-a-kilometre-across-settles-no-foot.md`).
        if s < 1e3 {
            cases.push((
                "the sphere",
                one(&listed(&w, &b)),
                (0..=10)
                    .map(|i| sphere_point(0.32 + 0.36 * f64::from(i) / 10.0))
                    .collect(),
            ));
        }
        for (what, comp, points) in &cases {
            let ComponentShape::Open { ends } = comp.shape else {
                panic!("s = {s}, {what}: a rim arc is open: {:?}", comp.shape);
            };
            assert_ne!(
                ends.rims,
                [ChartReach::default(); 2],
                "s = {s}, {what}: the arc reaches the wall's rim"
            );
            let cert = comp
                .certificate
                .as_ref()
                .expect("the reading's certificate is carried");
            assert!(
                comp.pad.is_finite() && comp.pad > 0.0 && comp.pad >= cert.hull_sup,
                "s = {s}, {what}: the pad {} holds the limb-2 bound {}",
                comp.pad,
                cert.hull_sup
            );
            // The tube's windows are its rung over each axis's chart
            // speed: imaged, they stand about a rung from the carrier.
            let crate::ssi::SsiTube::Chart { rung, .. } = cert.tube else {
                panic!("s = {s}, {what}: a chart tube certifies the arc");
            };
            assert!(
                0.5 * rung <= comp.pad && comp.pad <= 4.0 * rung,
                "s = {s}, {what}: the pad {} is the tube's region in metres, about its rung {rung}",
                comp.pad
            );
            assert!(
                comp.reach.is_finite(),
                "s = {s}, {what}: the witness's reach reads"
            );
            for &p in points {
                assert!(
                    comp.reads(p, band()).expect("a section point reads").is_some(),
                    "s = {s}, {what}: the section's point {p:?} reads on its component \
                     within the pad {}",
                    comp.pad
                );
            }
        }
    }
}

/// **A loop on the wall's chart carries none, by name**: the reading
/// lists it and fits its carrier, and no seat opens a loop certified on a
/// chart, so the carry refuses it as [`NoCarrier::ChartLoop`] rather than
/// hand the join a component it cannot open.
///
/// Red against: a loop carried as a rim arc is.
#[test]
fn a_loop_on_the_wall_carries_none_by_name() {
    let w = wall(1.0);
    // A ball through the wall's middle, clear of every side: a loop.
    let section = listed(&w, &ball(1.0, (0.0, 1.3, 1.0), 0.3));
    let refused = section.carry(Some(SsiLane::marched()), band());
    assert!(
        matches!(
            refused,
            Err(CarryError::Refused(CarryRefusal::Unminted(NoCarrier::ChartLoop)))
        ),
        "{refused:?}"
    );
}

/// **A classification that reaches is not the listing the carry takes**:
/// it names the sides the section ends on and no component, so carrying
/// it is a desync; its whole reading is what carries.
///
/// Red against: a carry that reads a reaching classification as empty.
#[test]
fn a_reaching_classification_is_carried_from_its_whole_reading() {
    let lane = Some(SsiLane::marched());
    let SectionRead::Built(section) =
        pair_section(&level(1.0), &wall(1.0), None, lane, band()).expect("the pair classifies")
    else {
        panic!("a spline × implicit pair is built here");
    };
    assert!(
        matches!(section.carry(lane, band()), Err(CarryError::Desync(_))),
        "a reaching classification carries nothing"
    );
    assert_eq!(one(&listed(&level(1.0), &wall(1.0))).winds, [false, false]);
}
