//! **M6-3 Leg F acceptance: `tube_along_arc`** — the world-coordinate
//! tube/torus door (the Evan-ratified rider, #175 thread).
//!
//! The pins, per the binding spec (§6): a tube-door donut against the
//! revolve-door donut — same census, same tier-3 outcome; volume
//! exact by Pappus (`2π²Rr²`, bracket-pinned against the certified
//! enclosure); and **`minor_radius` stored bit-exact** — the 56-ulp
//! reconstruction drift is retired FOR THIS DOOR, and the retirement
//! is pinned with `==`, not a tolerance. No cross-door bit-identity
//! contract exists (module docs): the census and certified statements
//! match; the bits need not.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use profile::RawLoop;

use geom::Surface;
use geom_core::{Point2, Point3, Tolerance, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Revolution, RevolveAxis, TubeError, TubeWindow, revolve, tube_along_arc};
use topo::Body;

const R: f64 = 2.0;
const MINOR: f64 = 0.5;

fn tube_donut() -> Body<f64> {
    tube_along_arc::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        R,
        TubeWindow::Full,
        MINOR,
    )
    .expect("the tube-door donut builds")
    .body
}

/// The revolve-door donut of the same torus (the corpus `donut()`
/// construction: a bulge-encoded circle profile about the y-axis).
fn revolve_donut() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(R - MINOR, 0.0), 1.0),
        ProfileVertex::new(Point2::new(R + MINOR, 0.0), 1.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve::<f64>(&vp, axis, Revolution::Full)
        .expect("the revolve donut builds")
        .body
}

fn census(body: &Body<f64>) -> (usize, usize, usize, usize) {
    (
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
        body.solids().count(),
    )
}

/// Census + tier parity with the revolve door, and both tier-3 green.
#[test]
fn tube_donut_matches_the_revolve_donut() {
    let tube = tube_donut();
    let rev = revolve_donut();
    assert_eq!(
        census(&tube),
        census(&rev),
        "same census through both doors"
    );
    for (name, body) in [("tube", &tube), ("revolve", &rev)] {
        assert_eq!(topo::validate(body), Ok(()), "{name} tier 1");
        assert_eq!(topo::validate_closed(body), Ok(()), "{name} tier 2");
        assert_eq!(topo::validate_geometric(body), Ok(()), "{name} tier 3");
    }
}

/// The 56-ulp drift, retired for this door: the stored torus surface's
/// `minor_radius` (and major, centre, axis, u_ref) IS the input, bit
/// for bit.
#[test]
fn tube_minor_radius_is_stored_bit_exact() {
    let tube = tube_donut();
    let mut tori = 0usize;
    for (_, face) in tube.faces() {
        let Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } = tube.get_surface(face.surface).expect("surface resolves")
        else {
            continue;
        };
        tori += 1;
        assert!(minor_radius.to_bits() == MINOR.to_bits(), "minor bit-exact");
        assert!(major_radius.to_bits() == R.to_bits(), "major bit-exact");
        assert_eq!(
            (center.x, center.y, center.z),
            (0.0, 0.0, 0.0),
            "centre verbatim"
        );
        assert_eq!((axis.x, axis.y, axis.z), (0.0, 1.0, 0.0), "axis verbatim");
        assert_eq!(
            (u_ref.x, u_ref.y, u_ref.z),
            (1.0, 0.0, 0.0),
            "u_ref verbatim"
        );
    }
    assert!(tori >= 1, "the tube body stores torus walls");
    // And the revolve door's reconstruction really is the thing this
    // door retires: its minor MAY drift by ulps (the 56-ulp lily
    // finding's class). Measured on THIS fixture's parameters
    // (R = 2, r = 0.5) the bulge-1 reconstruction happens to be
    // exact — 0 ulps (the review's measurement, adopted) — which is
    // precisely why the retirement is a CONTRACT pin on the tube
    // door (`==` above), not an inequality against the revolve's
    // value: the revolve's drift is input-dependent and its size is
    // the revolve's own business.
    let rev = revolve_donut();
    let rev_minor = rev
        .faces()
        .find_map(|(_, f)| match rev.get_surface(f.surface) {
            Some(Surface::Torus { minor_radius, .. }) => Some(*minor_radius),
            _ => None,
        })
        .expect("the revolve donut stores a torus");
    // Not asserting drift != 0 (a future exact profile lane may close
    // it); asserting the TUBE door cannot drift is the pin above.
    let _ = rev_minor;
}

/// Volume exact by Pappus: `V = 2π²Rr²`, inside the certified
/// enclosure of the quadrature lane (bracket pin, two-sided).
#[test]
fn tube_donut_volume_is_pappus_exact() {
    let tube = tube_donut();
    let props = topo::props::mass_properties(&tube).expect("mass properties");
    let exact = 2.0 * PI * PI * R * MINOR * MINOR;
    let pad = props.volume_pad;
    assert!(
        (props.volume - exact).abs() <= pad,
        "V = {} ± {pad} must bracket Pappus {exact}",
        props.volume
    );
}

/// A window builds the wedge (caps present), and the degenerate /
/// full-range / non-exact-frame doors refuse typed.
#[test]
fn tube_window_and_refusal_doors() {
    let wedge = tube_along_arc::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        R,
        TubeWindow::Arc { t0: 0.25, t1: 1.75 },
        MINOR,
    )
    .expect("the wedge builds")
    .body;
    assert_eq!(topo::validate_closed(&wedge), Ok(()), "wedge tier 2");
    assert_eq!(topo::validate_geometric(&wedge), Ok(()), "wedge tier 3");
    assert_eq!(wedge.faces().count(), 4, "two walls + two caps");

    let build = |axis: Vec3<f64>, u_ref: Vec3<f64>, window| {
        tube_along_arc::<f64>(Point3::new(0.0, 0.0, 0.0), axis, u_ref, R, window, MINOR)
    };
    assert!(matches!(
        build(Vec3::unit_y() * 1.5, Vec3::unit_x(), TubeWindow::Full),
        Err(TubeError::NonUnitAxis)
    ));
    assert!(matches!(
        build(Vec3::unit_y(), Vec3::unit_y(), TubeWindow::Full),
        Err(TubeError::NonUnitURef | TubeError::FrameNotOrthogonal)
    ));
    assert!(matches!(
        build(
            Vec3::unit_y(),
            Vec3::unit_x(),
            TubeWindow::Arc { t0: 1.0, t1: 1.0 }
        ),
        Err(TubeError::DegenerateWindow)
    ));
    assert!(matches!(
        build(
            Vec3::unit_y(),
            Vec3::unit_x(),
            TubeWindow::Arc {
                t0: 0.0,
                t1: core::f64::consts::TAU
            }
        ),
        Err(TubeError::FullRangeWindow)
    ));
    // The ring-torus convention (R > r > 0) refuses through the
    // SHARED revolve decide — the no-fork claim, executed.
    assert!(matches!(
        tube_along_arc::<f64>(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::unit_y(),
            Vec3::unit_x(),
            0.4,
            TubeWindow::Full,
            MINOR,
        ),
        Err(TubeError::Revolve(_))
    ));
}

/// **§9.3 interval row:** the tube door at the interval scalar —
/// build, tier 3, and the Pappus volume bracketed enclosure-style.
#[cfg(feature = "interval")]
mod certified {
    use geom_core::Real;
    use geom_core::interval::Interval;

    use super::*;

    fn iv(x: f64) -> Interval {
        <Interval as Real>::from_f64(x)
    }

    #[test]
    fn the_tube_donut_certifies_and_encloses_pappus_at_interval() {
        let t = tube_along_arc::<Interval>(
            Point3::new(iv(0.0), iv(0.0), iv(0.0)),
            Vec3::new(iv(0.0), iv(1.0), iv(0.0)),
            Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
            iv(R),
            TubeWindow::Full,
            iv(MINOR),
        )
        .expect("the tube builds at Interval");
        assert_eq!(topo::validate_geometric(&t.body), Ok(()), "tier 3");
        let m = topo::props::mass_properties(&t.body).expect("mass properties");
        let exact = 2.0 * PI * PI * R * MINOR * MINOR;
        let (lo, hi) = (
            geom_core::Bounds::lo(m.volume) - geom_core::Bounds::hi(m.volume_pad),
            geom_core::Bounds::hi(m.volume) + geom_core::Bounds::hi(m.volume_pad),
        );
        assert!(lo <= exact && exact <= hi, "Pappus {exact} ∈ [{lo}, {hi}]");
    }
}
