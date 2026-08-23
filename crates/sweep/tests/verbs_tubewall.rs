//! **VERBS-TUBEWALL acceptance: `tube_along_arc_hollow`** — the tube
//! door grown a wall, so a hollow tube keeps the door's exact-intent
//! storage instead of being re-said as a partial revolve of an
//! annulus.
//!
//! The suite pins:
//! - the **windowed** hollow tube: an open elbow of annular section,
//!   tier-3 valid, its mass properties against the closed forms
//!   (Pappus on the annulus for the volume and the two walls, the
//!   two annular caps added by hand);
//! - the **full-period** hollow tube: a torus shell — two shells in
//!   one solid, the inner one a CAVITY born through the shared
//!   void-insertion door by the revolve's own holed path — against
//!   the torus closed forms;
//! - **intent recoverability**: the outer wall's `minor_radius` and
//!   the frame are the caller's numbers bit for bit, exactly as the
//!   solid door; the inner wall's is `minor_radius - wall`, the one
//!   subtraction the caller can repeat, also bit for bit;
//! - the **wall's three request-fact refusals**, and that everything
//!   the solid door refuses the hollow door refuses identically.
//!
//! The solid door's own suite (`m6_tube`) is untouched: the hollow
//! form is one more loop through the same machinery, not a fork.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::Surface;
use geom_core::{Point3, Tol, Vec3};
use sweep::{Revolved, TubeError, TubeWindow, tube_along_arc, tube_along_arc_hollow};
use topo::Body;

const R: f64 = 2.0;
const OUTER: f64 = 0.5;
const WALL: f64 = 0.125;
const INNER: f64 = OUTER - WALL;
/// The window of `m6_tube`'s wedge, so the hollow elbow and the solid
/// one are the same swept arc.
const T0: f64 = 0.25;
const T1: f64 = 1.75;

fn hollow(window: TubeWindow<f64>) -> Revolved<f64> {
    tube_along_arc_hollow::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        R,
        window,
        OUTER,
        WALL,
        Tol::witness(),
    )
    .expect("the hollow tube builds")
}

fn tiers(body: &Body<f64>, what: &str) {
    assert_eq!(topo::validate(body), Ok(()), "{what} tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "{what} tier 2");
    assert_eq!(
        topo::validate_geometric(body, Tol::witness()),
        Ok(()),
        "{what} tier 3"
    );
}

/// The relative agreement the closed forms are pinned at. The torus
/// walls take the closed-form area/volume lane (both quadrature pads
/// are 0 on these bodies), so what is left is the arithmetic's own
/// rounding — the ring suite's pin for the same shapes.
const REL: f64 = 1e-12;

fn assert_close(got: f64, want: f64, what: &str) {
    assert!(
        ((got - want) / want).abs() < REL,
        "{what}: got {got}, closed form {want} (relative {})",
        ((got - want) / want).abs()
    );
}

/// The windowed hollow tube: an ordinary open elbow whose section is
/// an annulus. Four faces (two torus walls + two annular caps), one
/// shell, no cavity — a window has open ends, so nothing is enclosed.
/// Mass properties by Pappus on the annulus.
#[test]
fn hollow_elbow_is_valid_with_pappus_mass_properties() {
    let t = hollow(TubeWindow::Arc { t0: T0, t1: T1 });
    tiers(&t.body, "hollow elbow");

    assert_eq!(t.body.solids().count(), 1);
    assert_eq!(t.body.shells().count(), 1, "an open elbow encloses nothing");
    assert!(t.cavities.is_empty(), "a window has no cavity");
    // Each circle is two half-circle arcs, so each wall is two torus
    // faces — the solid door's own shape (`m6_tube`'s wedge is 4
    // faces: 2 walls + 2 caps), one more circle.
    assert_eq!(
        t.body.faces().count(),
        6,
        "two half-walls per circle + two annular caps"
    );

    // Pappus on the annulus, section area A = π(ro² − ri²) with its
    // centroid on the spine (distance R from the axis), swept θ:
    //   V = θ·R·A
    // the two walls by the curve form (θ·R·2πr each), the two caps
    // flat annuli.
    let theta = T1 - T0;
    let area = PI * (OUTER * OUTER - INNER * INNER);
    let v_expect = theta * R * area;
    let a_expect = theta * R * 2.0 * PI * (OUTER + INNER) + 2.0 * area;

    let props = topo::props::mass_properties(&t.body, Tol::witness()).expect("mass properties");
    assert_close(props.volume, v_expect, "elbow volume");
    assert_close(props.surface_area, a_expect, "elbow area");

    // The elbow really is hollow: the solid tube of the same window
    // is heavier by exactly the bore, which is the same Pappus form
    // on the inner disc.
    let solid = tube_along_arc::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        R,
        TubeWindow::Arc { t0: T0, t1: T1 },
        OUTER,
        Tol::witness(),
    )
    .expect("the solid elbow builds");
    let solid_props =
        topo::props::mass_properties(&solid.body, Tol::witness()).expect("mass properties");
    assert_close(
        solid_props.volume - props.volume,
        theta * R * PI * INNER * INNER,
        "the bore",
    );
}

/// The full-period hollow tube: a torus shell. The inner wall closes
/// into a CAVITY — the second shell of one solid, inserted through
/// the shared void-insertion door by the revolve's holed-profile
/// path, which is the policy the solid door's full period mirrors
/// (it builds the closed torus; the hollow one builds the closed
/// torus with its void).
#[test]
fn hollow_torus_carries_its_cavity() {
    let t = hollow(TubeWindow::Full);
    tiers(&t.body, "hollow torus");

    assert_eq!(t.body.solids().count(), 1);
    assert_eq!(t.body.shells().count(), 2, "outer boundary + the cavity");
    assert_eq!(t.cavities.len(), 1);
    assert_ne!(t.cavities[0], t.shell);
    let cavity = t.body.get_shell(t.cavities[0]).unwrap();
    let outer = t.body.get_shell(t.shell).unwrap();
    assert_eq!(cavity.solid, t.solid);
    assert_eq!(outer.solid, t.solid);
    assert_eq!(cavity.faces.len(), 2, "the cavity is a two-wall torus");
    assert_eq!(outer.faces.len(), 2);

    // The handle bundle names the inner loop with result-body keys.
    assert_eq!(t.walls.len(), 2);
    assert!(t.walls[1].iter().all(Option::is_some));
    assert!(t.rims[1].iter().all(Option::is_some));
    assert!(t.poles[1].iter().all(Option::is_none));

    // Torus closed forms: V = 2π²R(ro² − ri²), A = 4π²R(ro + ri).
    let props = topo::props::mass_properties(&t.body, Tol::witness()).expect("mass properties");
    assert_close(
        props.volume,
        2.0 * PI * PI * R * (OUTER * OUTER - INNER * INNER),
        "torus-shell volume",
    );
    assert_close(
        props.surface_area,
        4.0 * PI * PI * R * (OUTER + INNER),
        "torus-shell area",
    );
}

/// Both walls' stored tori, keyed by minor radius.
fn tori(body: &Body<f64>) -> Vec<(f64, f64, Point3<f64>, Vec3<f64>, Vec3<f64>)> {
    let mut out = Vec::new();
    for (_, face) in body.faces() {
        if let Some(Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        }) = body.get_surface(face.surface)
        {
            out.push((*minor_radius, *major_radius, *center, *axis, *u_ref));
        }
    }
    out
}

/// Intent recoverability. The outer wall is the solid door's contract
/// verbatim — the caller's `minor_radius`, major radius and frame,
/// bit for bit. The inner wall stores `minor_radius - wall`, the one
/// IEEE subtraction of the caller's own two numbers: a caller
/// recovers it by writing that subtraction, which is what this pins
/// with `==` on the bits rather than a tolerance.
#[test]
fn hollow_intent_parameters_are_stored_bit_exact() {
    for window in [TubeWindow::Full, TubeWindow::Arc { t0: T0, t1: T1 }] {
        // A window's start direction is the door's ONE derived
        // quantity (`u_ref` rotated by `t0`); the full period stores
        // `u_ref` itself.
        let full = matches!(window, TubeWindow::Full);
        let t = hollow(window);
        let walls = tori(&t.body);
        assert_eq!(walls.len(), 4, "two half-circle torus faces per circle");
        for (minor, major, center, axis, u_ref) in &walls {
            assert!(major.to_bits() == R.to_bits(), "major bit-exact");
            assert_eq!((center.x, center.y, center.z), (0.0, 0.0, 0.0));
            assert_eq!((axis.x, axis.y, axis.z), (0.0, 1.0, 0.0));
            if full {
                assert_eq!((u_ref.x, u_ref.y, u_ref.z), (1.0, 0.0, 0.0));
            }
            assert!(
                minor.to_bits() == OUTER.to_bits() || minor.to_bits() == (OUTER - WALL).to_bits(),
                "minor radius {minor} is neither the given outer radius nor \
                 minor_radius - wall, bit for bit"
            );
        }
        let mut minors: Vec<u64> = walls.iter().map(|w| w.0.to_bits()).collect();
        minors.sort_unstable();
        let mut want = vec![
            (OUTER - WALL).to_bits(),
            (OUTER - WALL).to_bits(),
            OUTER.to_bits(),
            OUTER.to_bits(),
        ];
        want.sort_unstable();
        assert_eq!(minors, want, "each radius on both of its half-walls");
    }
}

/// The wall's three refusals — plain facts about the REQUEST, decided
/// before anything is minted — and the shared refusals the solid door
/// already owns, raised identically by the hollow door.
#[test]
fn hollow_wall_and_shared_refusal_doors() {
    let build = |minor: f64, wall: f64| {
        tube_along_arc_hollow::<f64>(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::unit_y(),
            Vec3::unit_x(),
            R,
            TubeWindow::Full,
            minor,
            wall,
            Tol::witness(),
        )
    };
    assert!(matches!(
        build(OUTER, 0.0),
        Err(TubeError::NonpositiveWall { .. })
    ));
    assert!(matches!(
        build(OUTER, -0.1),
        Err(TubeError::NonpositiveWall { .. })
    ));
    // A poisoned thickness is not definitely positive either.
    assert!(matches!(
        build(OUTER, f64::NAN),
        Err(TubeError::NonpositiveWall { .. })
    ));
    assert!(matches!(
        build(OUTER, OUTER),
        Err(TubeError::WallExceedsRadius { .. })
    ));
    assert!(matches!(
        build(OUTER, OUTER * 2.0),
        Err(TubeError::WallExceedsRadius { .. })
    ));
    // Positive, far below the outer radius, and still no annulus:
    // `0.5 - 1e-300` rounds back to `0.5`, so the two circles would be
    // stored as one.
    assert!(matches!(
        build(OUTER, 1e-300),
        Err(TubeError::WallBelowResolution { .. })
    ));

    // Every message names the door and the number that failed.
    for e in [
        TubeError::NonpositiveWall { wall: 0.0 },
        TubeError::WallExceedsRadius {
            wall: 1.0,
            minor_radius: 0.5,
        },
        TubeError::WallBelowResolution {
            wall: 1e-300,
            minor_radius: 0.5,
        },
    ] {
        let msg = e.to_string();
        assert!(msg.starts_with("tube_along_arc_hollow: "), "{msg}");
    }

    // The solid door's own doors, unchanged through the hollow one.
    let frame = |axis: Vec3<f64>, u_ref: Vec3<f64>, window, major: f64| {
        tube_along_arc_hollow::<f64>(
            Point3::new(0.0, 0.0, 0.0),
            axis,
            u_ref,
            major,
            window,
            OUTER,
            WALL,
            Tol::witness(),
        )
    };
    assert!(matches!(
        frame(Vec3::unit_y() * 1.5, Vec3::unit_x(), TubeWindow::Full, R),
        Err(TubeError::NonUnitAxis)
    ));
    assert!(matches!(
        frame(Vec3::unit_y(), Vec3::unit_y(), TubeWindow::Full, R),
        Err(TubeError::NonUnitURef | TubeError::FrameNotOrthogonal)
    ));
    assert!(matches!(
        frame(
            Vec3::unit_y(),
            Vec3::unit_x(),
            TubeWindow::Arc { t0: 1.0, t1: 1.0 },
            R
        ),
        Err(TubeError::DegenerateWindow)
    ));
    assert!(matches!(
        frame(
            Vec3::unit_y(),
            Vec3::unit_x(),
            TubeWindow::Arc {
                t0: 0.0,
                t1: core::f64::consts::TAU
            },
            R
        ),
        Err(TubeError::FullRangeWindow)
    ));
    // The ring-torus convention, still through the SHARED revolve
    // decide — the no-fork claim, executed on the hollow door.
    assert!(matches!(
        frame(Vec3::unit_y(), Vec3::unit_x(), TubeWindow::Full, 0.4),
        Err(TubeError::Revolve(_))
    ));
}

/// **The interval row**: the hollow tube at the certified scalar —
/// build, tier 3, and both closed forms inside the enclosure.
#[cfg(feature = "interval")]
mod certified {
    use geom_core::Real;
    use geom_core::interval::Interval;

    use super::*;

    fn iv(x: f64) -> Interval {
        <Interval as Real>::from_f64(x)
    }

    fn encloses(value: Interval, pad: f64, exact: f64, what: &str) {
        let lo = geom_core::Bounds::lo(value) - pad;
        let hi = geom_core::Bounds::hi(value) + pad;
        assert!(lo <= exact && exact <= hi, "{what}: {exact} ∈ [{lo}, {hi}]");
    }

    #[test]
    fn the_hollow_torus_certifies_and_encloses_its_closed_forms() {
        let t = tube_along_arc_hollow::<Interval>(
            Point3::new(iv(0.0), iv(0.0), iv(0.0)),
            Vec3::new(iv(0.0), iv(1.0), iv(0.0)),
            Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
            iv(R),
            TubeWindow::Full,
            iv(OUTER),
            iv(WALL),
            Tol::witness(),
        )
        .expect("the hollow tube builds at Interval");
        assert_eq!(
            topo::validate_geometric(&t.body, Tol::witness()),
            Ok(()),
            "tier 3"
        );
        assert_eq!(t.body.shells().count(), 2);
        let m = topo::props::mass_properties(&t.body, Tol::witness()).expect("mass properties");
        encloses(
            m.volume,
            m.volume_pad,
            2.0 * PI * PI * R * (OUTER * OUTER - INNER * INNER),
            "volume",
        );
        encloses(
            m.surface_area,
            m.area_pad,
            4.0 * PI * PI * R * (OUTER + INNER),
            "area",
        );
    }

    #[test]
    fn the_hollow_elbow_certifies_at_interval() {
        let t = tube_along_arc_hollow::<Interval>(
            Point3::new(iv(0.0), iv(0.0), iv(0.0)),
            Vec3::new(iv(0.0), iv(1.0), iv(0.0)),
            Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
            iv(R),
            TubeWindow::Arc {
                t0: iv(T0),
                t1: iv(T1),
            },
            iv(OUTER),
            iv(WALL),
            Tol::witness(),
        )
        .expect("the hollow elbow builds at Interval");
        assert_eq!(
            topo::validate_geometric(&t.body, Tol::witness()),
            Ok(()),
            "tier 3"
        );
        let m = topo::props::mass_properties(&t.body, Tol::witness()).expect("mass properties");
        encloses(
            m.volume,
            m.volume_pad,
            (T1 - T0) * R * PI * (OUTER * OUTER - INNER * INNER),
            "volume",
        );
    }
}
