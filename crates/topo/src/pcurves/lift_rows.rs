//! [`super::decide_joint`]'s decisions, each on a joint built to sit
//! where one of them is the only thing deciding. Every row runs at
//! `f64` and at `Interval`, under a band pinned here (`zero = ε`),
//! so the digits mean the same at every ε row.
#![allow(clippy::unwrap_used, clippy::panic)]

use super::{DescribedChart, PinMiss, Singular, decide_plain_joint, singular_at};
use crate::joint::{Deck, JointElement};
use geom::Surface;
use geom_brep::Pcurve;
use geom_core::{Band, Decide, Interval, Point2, Point3, Vec3};

const EPS: f64 = 1e-9;

fn band(k: f64) -> Band {
    Band::new(EPS, k * EPS).unwrap()
}

fn unit_sphere<T: Decide>() -> Surface<T> {
    let f = T::from_f64;
    Surface::Sphere {
        center: Point3::new(f(0.0), f(0.0), f(0.0)),
        radius: f(1.0),
        axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
        u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
    }
}

fn torus<T: Decide>() -> Surface<T> {
    let f = T::from_f64;
    Surface::Torus {
        center: Point3::new(f(0.0), f(0.0), f(0.0)),
        axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
        major_radius: f(2.0),
        minor_radius: f(1.0),
        u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
    }
}

/// A row that sits at one chart point of a sphere or torus chart,
/// both of whose channels are angles: what a joint reads of it is
/// its entry, and its twin.
fn at<T: Decide>(u: f64, v: f64) -> Pcurve<T> {
    let f = T::from_f64;
    Pcurve::Harmonic {
        u: geom_brep::Channel::fixed_angle(f(u)),
        v: geom_brep::Channel::fixed_angle(f(v)),
    }
}

fn on_sphere<T: Decide>(u: f64, v: f64) -> Point3<T> {
    let f = T::from_f64;
    Point3::new(f(v.cos() * u.cos()), f(v.cos() * u.sin()), f(v.sin()))
}

/// The joint at `(u, v)` against a predecessor exit `prev`, on
/// `surface`, with `vertex` the joint's point.
fn lift<T: Decide>(
    surface: &Surface<T>,
    (u, v): (f64, f64),
    prev: (f64, f64),
    vertex: Point3<T>,
    k: f64,
) -> Result<JointElement, PinMiss> {
    let f = T::from_f64;
    decide_plain_joint(
        DescribedChart::of(surface).unwrap(),
        &at(u, v),
        f(0.0),
        Point2::new(f(prev.0), f(prev.1)),
        vertex,
        super::ChartPeriods::given_u(Some(geom_brep::Period::angle()), surface, band(k)),
        band(k),
    )
}

/// **A joint whose predecessor sits on the image's TWIN is the
/// twin, at every K** — the reproduction both reviews of the first
/// cut made. The predecessor's exit is the twin of `(0.3, 0.4)`,
/// `(0.3 + π, π − 0.4)`, moved `δ = ±3.5ε` in metres along the
/// azimuth: inside the 4ε joint bound, and past `K·ε` at `K = 3` and
/// `K = 2`. Read as a whole number of periods, the image's own
/// azimuth then decides a branch half a period off and never
/// reaches the twin; read as half periods, the twin has a quarter
/// period of room. A twin is never the identity.
fn twin_at_every_k<T: Decide>(lane: &str) {
    let s = unit_sphere::<T>();
    let (u, v): (f64, f64) = (0.3, 0.4);
    // The twin either side: half a period over (`m = 1`, no period)
    // or half a period back (`m = −1`: the twin, one period back).
    let sides = [(core::f64::consts::PI, 0), (-core::f64::consts::PI, -1)];
    for k in [10.0, 3.0, 2.0] {
        for delta in [-3.5 * EPS, 3.5 * EPS] {
            for (side, ku) in sides {
                let du = delta / v.cos();
                let prev = (u + side + du, core::f64::consts::PI - v);
                let Ok(element) = lift(&s, (u, v), prev, on_sphere(u, v), k) else {
                    panic!("{lane} K = {k} δ = {delta:e} side {side}: the joint lifts");
                };
                let want = Deck {
                    u: ku,
                    v: 0,
                    twin: true,
                };
                assert_eq!(
                    element,
                    JointElement(want),
                    "{lane} K = {k} δ = {delta:e} side {side}: the twin, {ku} periods"
                );
                // Where the element puts the entry, read on the f64
                // lane: the element is integers, the same at both.
                let lifted = want
                    .apply(
                        &at::<f64>(u, v),
                        &unit_sphere(),
                        super::ChartPeriods::stated(&unit_sphere()),
                    )
                    .unwrap();
                assert!(
                    (lifted.eval(0.0).x - prev.0).abs() < 1e-6,
                    "{lane} K = {k} side {side}: the lifted entry lands on the predecessor"
                );
                assert_ne!(
                    element,
                    JointElement::IDENTITY,
                    "{lane} K = {k}: a twin is never the identity"
                );
            }
        }
    }
}

#[test]
fn a_twin_joint_is_the_twin_at_every_k() {
    twin_at_every_k::<f64>("f64");
    twin_at_every_k::<Interval>("Interval");
}

/// **A joint whose vertex is within the band of a pole, at a gap no
/// orbit point meets, escalates.** The vertex is `5ε` from the north
/// pole (incidence undecided), so its lever is `5ε`; the gap is one
/// radian, so no mark decides, and none of the orbit points the
/// near-pole reading may take (the image, the twin either way, the
/// image a period either way) is within ε of it at that lever. Taking the image there would be a
/// branch no margin saw.
fn near_pole_off_orbit_escalates<T: Decide>(lane: &str) {
    let s = unit_sphere::<T>();
    let theta: f64 = 5e-9;
    let v = core::f64::consts::FRAC_PI_2 - theta;
    let out = lift(&s, (0.0, v), (1.0, v), on_sphere(0.0, v), 10.0);
    assert!(
        matches!(out, Err(PinMiss::Escalated(_))),
        "{lane}: the incidence's escalation, not a lift: {:?}",
        out.as_ref().ok()
    );
}

/// **A joint whose vertex is on a pole takes the turn on the face's
/// side.** On the north pole of an outward face the azimuth returns,
/// so the entry is placed in `(exit − τ, exit)`: an entry one radian
/// or half a turn past the exit goes one period back. An entry at
/// the exit's own azimuth, or a whole turn from it, is two edges
/// leaving the pole along one direction, and refuses.
fn on_the_pole_turns_on_the_faces_side<T: Decide>(lane: &str) {
    let s = unit_sphere::<T>();
    let v = core::f64::consts::FRAC_PI_2;
    let f = T::from_f64;
    let pole = Point3::new(f(0.0), f(0.0), f(1.0));
    for gap in [1.0, core::f64::consts::PI] {
        let out = lift(&s, (gap, v), (0.0, v), pole, 10.0);
        assert!(
            matches!(
                out,
                Ok(JointElement(Deck {
                    u: -1,
                    v: 0,
                    twin: false
                }))
            ),
            "{lane} gap {gap}: one period back, on the principal sheet: {:?}",
            out.as_ref().ok()
        );
    }
    for gap in [0.0, core::f64::consts::TAU] {
        let out = lift(&s, (gap, v), (0.0, v), pole, 10.0);
        assert!(
            matches!(
                out,
                Err(PinMiss::Singular(super::SingularMiss::EdgesCoincide))
            ),
            "{lane} gap {gap}: one direction out of the pole refuses: {:?}",
            out.as_ref().ok()
        );
    }
}

#[test]
fn a_joint_on_a_pole_takes_the_faces_turn() {
    on_the_pole_turns_on_the_faces_side::<f64>("f64");
    on_the_pole_turns_on_the_faces_side::<Interval>("Interval");
}

/// **Near a pole the fallback takes the orbit point the gap meets.**
/// The vertex is `5ε` from the north pole (incidence undecided), so
/// its lever is `5ε` and the half-period marks at it are undecided
/// too. Against a predecessor on the image's own point the fallback
/// takes `m = 0`, the identity; against one on the twin either way
/// round it takes `m = ±1`, the twin. Each orbit point's gap at the
/// vertex's lever decides Zero, and every other one's does not.
fn near_pole_takes_the_orbit_point<T: Decide>(lane: &str) {
    let s = unit_sphere::<T>();
    let v = core::f64::consts::FRAC_PI_2 - 5e-9;
    let u = 0.3;
    let vertex = on_sphere(u, v);
    let twin_v = core::f64::consts::PI - v;
    let rows = [
        ((u, v), Deck::IDENTITY),
        (
            (u + core::f64::consts::PI, twin_v),
            Deck {
                u: 0,
                v: 0,
                twin: true,
            },
        ),
        (
            (u - core::f64::consts::PI, twin_v),
            Deck {
                u: -1,
                v: 0,
                twin: true,
            },
        ),
    ];
    for (prev, want) in rows {
        let out = lift(&s, (u, v), prev, vertex, 10.0);
        assert!(
            matches!(out, Ok(JointElement(deck)) if deck == want),
            "{lane} prev {prev:?}: the fallback takes {want:?}: {:?}",
            out.as_ref().ok()
        );
    }
}

#[test]
fn near_a_pole_the_fallback_takes_the_orbit_point_the_gap_meets() {
    near_pole_takes_the_orbit_point::<f64>("f64");
    near_pole_takes_the_orbit_point::<Interval>("Interval");
}

#[test]
fn a_near_pole_joint_off_every_orbit_point_escalates() {
    near_pole_off_orbit_escalates::<f64>("f64");
    near_pole_off_orbit_escalates::<Interval>("Interval");
}

/// **An undecided incidence still decides the branch where its marks
/// decide.** The vertex is `9ε` from the pole (undecided), and the
/// gap is a whole period: every mark it meets is past the band at
/// that lever, so the joint lifts one period over. Skipping the
/// azimuth there, as on the singular set, would read it as the
/// identity.
fn undecided_incidence_still_lifts<T: Decide>(lane: &str) {
    let s = unit_sphere::<T>();
    let v = core::f64::consts::FRAC_PI_2 - 9e-9;
    let out = lift(
        &s,
        (0.0, v),
        (core::f64::consts::TAU, v),
        on_sphere(0.0, v),
        10.0,
    );
    let Ok(element) = out else {
        panic!("{lane}: the marks decide")
    };
    let chart = DescribedChart::of(&s).unwrap();
    assert!(
        matches!(
            singular_at(chart, on_sphere(0.0, v), band(10.0)),
            Singular::Undecided(_)
        ),
        "{lane}"
    );
    assert_eq!(
        element,
        JointElement(Deck {
            u: 1,
            v: 0,
            twin: false
        }),
        "{lane}: one period over, the marks' own decision"
    );
}

#[test]
fn an_undecided_incidence_still_decides_the_period() {
    undecided_incidence_still_lifts::<f64>("f64");
    undecided_incidence_still_lifts::<Interval>("Interval");
}

/// **The azimuth's marks are metered at the vertex's own lever.** At
/// colatitude `1e-3` the lever is `1e-3` m per radian, not the
/// sphere's radius: a gap `5e-6` rad past the quarter-period mark is
/// `5e-9` m there, inside the band, so the joint escalates. Metered
/// at the radius it would read `5e-6` m and decide.
fn marks_at_the_vertex_lever<T: Decide>(lane: &str) {
    let s = unit_sphere::<T>();
    let v = core::f64::consts::FRAC_PI_2 - 1e-3;
    let gap = core::f64::consts::FRAC_PI_2 + 5e-6;
    let out = lift(&s, (0.0, v), (gap, v), on_sphere(0.0, v), 10.0);
    assert!(
        matches!(out, Err(PinMiss::Escalated(_))),
        "{lane}: the mark is undecided at the vertex's lever"
    );
}

#[test]
fn the_marks_are_metered_at_the_vertex_lever() {
    marks_at_the_vertex_lever::<f64>("f64");
    marks_at_the_vertex_lever::<Interval>("Interval");
}

/// **The second channel's period is part of the deck element.** On a
/// torus a predecessor one meridian period over is the same point,
/// and the joint between them is not the identity.
fn meridian_period<T: Decide>(lane: &str) {
    let s = torus::<T>();
    let (u, v): (f64, f64) = (0.3, 0.4);
    let f = T::from_f64;
    let r = 2.0 + v.cos();
    let vertex = Point3::new(f(r * u.cos()), f(r * u.sin()), f(v.sin()));
    let Ok(element) = lift(&s, (u, v), (u, v + core::f64::consts::TAU), vertex, 10.0) else {
        panic!("{lane}: the joint lifts")
    };
    assert_eq!(
        element,
        JointElement(Deck {
            u: 0,
            v: 1,
            twin: false
        }),
        "{lane}: one meridian period"
    );
}

#[test]
fn a_meridian_period_is_not_the_identity() {
    meridian_period::<f64>("f64");
    meridian_period::<Interval>("Interval");
}
