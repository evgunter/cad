//! **A sphere face slit to its pole answers containment the same way at
//! both scalars.**
//!
//! The dome is a unit hemisphere revolved about `y`: a flat base and two
//! π-band sphere faces meeting along two meridians at the pole. Merging
//! the base and killing one meridian leaves ONE sphere face whose loop
//! runs the surviving meridian up into the pole and straight back down
//! it — a slit, the pole a valence-1 strut tip (tier 2 names it; tier 1
//! holds). Killed from its minus half, the surviving face's walk crosses
//! the slit mid-walk, where the azimuth walk's strictly-next pole branch
//! sits exactly on its jump: at `f64` it lands a whole period on, and at
//! `Interval` the enclosure straddles it and spans both branches. The
//! face's window is then a whole period wide in both lanes, which the
//! containment door's period gate declines before it reads the window as
//! a region — so the verdicts do not depend on which branch the pick
//! took. A pick that refused the straddle instead would turn every
//! verdict below into a partial-sphere refusal at `Interval`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Bounds, Decide, Interval, Point2, Point3, Real, Tol};
use sweep::Revolution;
use sweep::test_support::revolved_about_y_at;
use topo::{Body, SolidContainment, ValidationError, point_in_solid, validate, validate_closed};

fn f<T: Real>(x: f64) -> T {
    T::from_f64(x)
}

/// The unit dome, its base merged to one disc.
fn dome<T: Decide + topo::AtRestPolicy>() -> Body<T> {
    let bulge = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let mut body = revolved_about_y_at(
        vec![
            (Point2::new(f(0.0), f(0.0)), f(0.0)),
            (Point2::new(f(1.0), f(0.0)), f(bulge)),
            (Point2::new(f(0.0), f(1.0)), f(0.0)),
        ],
        Revolution::Full,
        Tol::witness(),
    );
    body.merge_coplanar_faces(Tol::witness())
        .expect("the pole-split base merges");
    body
}

/// Every slit of the dome: each meridian between the two sphere bands,
/// killed from each of its halves.
fn slits<T: Decide + topo::AtRestPolicy>() -> Vec<Body<T>> {
    let body = dome::<T>();
    let on_sphere = |he| {
        body.face_of_half_edge(he)
            .and_then(|face| body.get_face(face))
            .and_then(|face| body.get_surface(face.surface))
            .is_some_and(|s| matches!(s, geom::Surface::Sphere { .. }))
    };
    let meridians: Vec<_> = body
        .edges()
        .filter(|(_, e)| on_sphere(e.he_plus) && on_sphere(e.he_minus))
        .map(|(_, e)| (e.he_plus, e.he_minus))
        .collect();
    assert_eq!(meridians.len(), 2, "the dome's two band meridians");
    meridians
        .into_iter()
        .flat_map(|(plus, minus)| [plus, minus])
        .map(|he| {
            let mut slit = body.clone();
            slit.kef(he).expect("the meridian kills");
            assert_eq!(validate(&slit), Ok(()), "tier 1 holds on the slit");
            let tier2 = validate_closed(&slit).expect_err("the slit is scaffolding");
            assert!(
                matches!(
                    tier2.as_slice(),
                    [ValidationError::ScaffoldingStrutVertex { .. }]
                ),
                "the pole is the one strut tip: {tier2:?}"
            );
            slit
        })
        .collect()
}

/// Points against the unit dome `{ y ≥ 0, |p| ≤ 1 }`, with the answer.
const TABLE: [((f64, f64, f64), SolidContainment); 6] = [
    ((0.1, 0.5, 0.1), SolidContainment::In),
    ((0.1, 1.5, 0.1), SolidContainment::Out),
    ((0.5, 0.2, -0.3), SolidContainment::In),
    ((0.0, 0.99, 0.0), SolidContainment::In),
    ((0.7, 0.7, 0.0), SolidContainment::In),
    ((0.0, 0.5, 0.86), SolidContainment::In),
];

fn verdicts<T: Decide + topo::AtRestPolicy + Bounds>(lane: &str) {
    let band = Band::linear(Tol::witness()).expect("the witness band");
    for (i, slit) in slits::<T>().iter().enumerate() {
        for ((x, y, z), want) in TABLE {
            let got = point_in_solid(slit, Point3::new(f(x), f(y), f(z)), band, Tol::witness());
            assert!(
                matches!(got, Ok(v) if v == want),
                "[{lane}] slit {i}: point ({x}, {y}, {z}) against the slit dome: \
                 {got:?}, want {want:?}"
            );
        }
    }
}

#[test]
fn a_slit_dome_answers_containment_at_f64() {
    verdicts::<f64>("f64");
}

#[test]
fn a_slit_dome_answers_containment_at_interval() {
    verdicts::<Interval>("Interval");
}
