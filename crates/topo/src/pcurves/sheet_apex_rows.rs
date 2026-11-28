//! [`super::decide_joint`] at a spindle sheet's apex: which side of it a
//! face lies on is read off the joint's two chart ends
//! (`decision::face_above`), never off the point. Every row runs at
//! `f64` and at `Interval`, under a band pinned here (`zero = ε`).
#![allow(clippy::unwrap_used, clippy::panic)]

use super::{DescribedChart, PinMiss, SingularMiss, decide_plain_joint};
use crate::joint::{Deck, JointElement};
use core::f64::consts::PI;
use geom::Surface;
use geom_brep::Pcurve;
use geom_core::{Band, Decide, Interval, Point2, Point3, Vec3};

const EPS: f64 = 1e-9;

fn band() -> Band {
    Band::new(EPS, 10.0 * EPS).unwrap()
}

/// The sheet `(R, r = 1)` about `z` through the origin: the horn at
/// `R = 1`, its one cusp the origin; a lemon at `R < 0`.
fn sheet<T: Decide>(major: f64) -> Surface<T> {
    let f = T::from_f64;
    Surface::Spindle {
        frame: geom::Frame::new(
            Point3::new(f(0.0), f(0.0), f(0.0)),
            Vec3::new(f(0.0), f(0.0), f(1.0)),
            Vec3::new(f(1.0), f(0.0), f(0.0)),
        ),
        major_radius: f(major),
        minor_radius: f(1.0),
    }
}

/// The joint from an exit at `prev` into an image at `entry`, both chart
/// points, at the vertex `apex`, on a face of sense `true`.
fn joint<T: Decide>(
    surface: &Surface<T>,
    prev: (f64, f64),
    entry: (f64, f64),
    apex: Point3<T>,
) -> Result<JointElement, PinMiss> {
    let f = T::from_f64;
    decide_plain_joint(
        DescribedChart::of(surface),
        &Pcurve::Harmonic {
            u: geom_brep::Channel::fixed_angle(f(entry.0)),
            v: geom_brep::Channel::fixed_angle(f(entry.1)),
        },
        f(0.0),
        Point2::new(f(prev.0), f(prev.1)),
        apex,
        super::ChartPeriods::given_u(Some(geom_brep::Period::angle()), surface, band()),
        band(),
    )
}

fn origin<T: Decide>() -> Point3<T> {
    let f = T::from_f64;
    Point3::new(f(0.0), f(0.0), f(0.0))
}

const BACK: Deck = Deck {
    u: -1,
    v: 0,
    twin: false,
};

/// **A horn's cusp turns on the side its ends reach it from**, and a
/// lemon's apexes by the same rule. The exit at `u = 0` and the entry at
/// `u = 1` both reach the point: from below in `v` (the horn's `v = −π`,
/// a lemon's lower apex `−v*`) the face lies above it, the azimuth
/// advances, and the entry is the one branch in `(0, τ)`, the image
/// itself; from above (`v = π`, `v*`) it returns, the branch in
/// `(−τ, 0)`, a period back. Read off the point, a horn's cusp would
/// read one side for both.
fn turns_on_the_side_its_ends_reach<T: Decide>(lane: &str) {
    let lemon_major = -0.5_f64;
    let lemon_v = (1.0 - lemon_major * lemon_major).sqrt().atan2(-lemon_major);
    let f = T::from_f64;
    let h = (1.0 - lemon_major * lemon_major).sqrt();
    for (what, major, v, apex, want) in [
        (
            "the horn from below",
            1.0,
            -PI,
            origin::<T>(),
            Deck::IDENTITY,
        ),
        ("the horn from above", 1.0, PI, origin::<T>(), BACK),
        (
            "a lemon's lower apex",
            lemon_major,
            -lemon_v,
            Point3::new(f(0.0), f(0.0), f(-h)),
            Deck::IDENTITY,
        ),
        (
            "a lemon's upper apex",
            lemon_major,
            lemon_v,
            Point3::new(f(0.0), f(0.0), f(h)),
            BACK,
        ),
    ] {
        let got = joint(&sheet::<T>(major), (0.0, v), (1.0, v), apex);
        assert!(
            matches!(got, Ok(JointElement(deck)) if deck == want),
            "{lane} {what}: {want:?}, got {:?}",
            got.as_ref().ok()
        );
    }
}

#[test]
fn a_sheet_apex_turns_on_the_side_its_ends_reach() {
    turns_on_the_side_its_ends_reach::<f64>("f64");
    turns_on_the_side_its_ends_reach::<Interval>("Interval");
}

/// **Ends on either side of a horn's cusp refuse**: an exit at `v = −π`
/// and an entry at `v = π`, either way round, is a face crossing the
/// cusp from one side to the other ([`SingularMiss::CuspCrossed`]).
fn ends_either_side_refuse<T: Decide>(lane: &str) {
    for (prev, entry) in [((0.0, -PI), (1.0, PI)), ((0.0, PI), (1.0, -PI))] {
        let got = joint(&sheet::<T>(1.0), prev, entry, origin::<T>());
        assert!(
            matches!(got, Err(PinMiss::Singular(SingularMiss::CuspCrossed))),
            "{lane} {prev:?} → {entry:?}: {:?}",
            got.as_ref().ok()
        );
    }
}

#[test]
fn ends_either_side_of_a_horns_cusp_refuse() {
    ends_either_side_refuse::<f64>("f64");
    ends_either_side_refuse::<Interval>("Interval");
}

/// **An end whose `v` reads no side refuses**: on the lemon of zero
/// height (`R = −r`), whose two apexes are its centre, an end at `v = 0`
/// decides zero ([`SingularMiss::CuspUnsided`]), and one `3ε` off it, in
/// the band at the polar radius `1`, escalates. Either end alone is
/// enough: the arriving end's reading is the first.
fn an_end_with_no_side<T: Decide>(lane: &str) {
    let flat = sheet::<T>(-1.0);
    for (what, prev, entry) in [
        ("arriving", (0.0, 0.0), (1.0, 0.5)),
        ("leaving", (0.0, 0.5), (1.0, 0.0)),
    ] {
        let got = joint(&flat, prev, entry, origin::<T>());
        assert!(
            matches!(got, Err(PinMiss::Singular(SingularMiss::CuspUnsided))),
            "{lane} {what} at v = 0: {:?}",
            got.as_ref().ok()
        );
    }
    let near = 3.0 * EPS;
    for (what, prev, entry) in [
        ("arriving", (0.0, near), (1.0, 0.5)),
        ("leaving", (0.0, 0.5), (1.0, near)),
    ] {
        let got = joint(&flat, prev, entry, origin::<T>());
        assert!(
            matches!(got, Err(PinMiss::Escalated(_))),
            "{lane} {what} at v = 3ε: {:?}",
            got.as_ref().ok()
        );
    }
}

#[test]
fn a_sheet_apex_end_that_reads_no_side_refuses() {
    an_end_with_no_side::<f64>("f64");
    an_end_with_no_side::<Interval>("Interval");
}
