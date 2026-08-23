//! **Independent review probes for VERBS-TUBEWALL** — authored against
//! the public doors (`tube_along_arc_hollow` / `tube_along_arc`) and
//! the PR's own claims, not against the diff. Geometry is deliberately
//! NOT the unit suite's fixture: varied major radii, walls and windows,
//! including a thin wall, a small part, a large part and a negative
//! window start.
//!
//! The rows, and what makes each go RED:
//! - **Closed forms at varied geometry**: Pappus on the annulus for
//!   three different elbows and the torus-shell forms for three
//!   different full periods, re-derived independently — red the moment
//!   the hollow construction stores or sweeps anything but the two
//!   concentric intent circles.
//! - **The bore differences out**: solid minus hollow of the same
//!   window equals the inner tube's own Pappus volume — red if the
//!   hollow door quietly changes the OUTER geometry too.
//! - **Intent bits at adversarial values**: radii/walls with
//!   non-terminating binary fractions; the stored minors are exactly
//!   `{outer, outer − wall}` and the caller RECOVERS the inner radius
//!   by repeating the one IEEE subtraction — red if any arithmetic
//!   (normalize, round-trip, reconstruction) lands on the path.
//! - **The `u_ref` caveat is honest in both directions**: a `t0 = 0`
//!   window stores the caller's `u_ref` bit-exactly, a `t0 ≠ 0` window
//!   stores a DIFFERENT vector (the derived start direction) — red if
//!   the door starts claiming verbatim `u_ref` where it does not hold,
//!   or silently stops storing it where it does.
//! - **The metered wall is really metered**: at the run's ε a wall of
//!   `1e-20` m refuses `NonpositiveWall` (a bracket read would accept
//!   it and build a sliver), an in-band wall and an in-band bore
//!   ESCALATE rather than pass — red exactly when the two decides are
//!   replaced by raw comparisons, which is the PR's flagged deviation.
//! - **The cavity is a real void at varied geometry**: two shells, one
//!   cavity, both two-face tori, cavity minors at the inner-radius
//!   bits — red if the full-period hole loop stops riding the
//!   void-insertion route.
//! - **(interval) enclosures stay TIGHT**: the closed forms are inside
//!   the enclosure AND the pads are small relative to the quantity —
//!   red when certification degrades, not merely when it breaks; the
//!   unit suite's own containment rows get easier as pads grow.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::Surface;
use geom_core::{Point3, Tol, Vec3};
use sweep::{Revolved, TubeError, TubeWindow, tube_along_arc, tube_along_arc_hollow};
use topo::Body;

/// (major, outer minor, wall, window) — none of them the unit
/// fixture's, spanning ~5 decades of scale and a thin wall.
const CASES: [(f64, f64, f64, Option<(f64, f64)>); 6] = [
    (3.7, 0.9, 0.31, Some((-0.4, 2.9))),
    (0.02, 0.004, 0.0015, Some((0.1, 0.35))),
    (1300.0, 2.5, 0.001, Some((1.0, 5.0))),
    (3.7, 0.9, 0.31, None),
    (0.02, 0.004, 0.0015, None),
    (1300.0, 2.5, 0.001, None),
];

fn hollow(major: f64, outer: f64, wall: f64, window: Option<(f64, f64)>) -> Revolved<f64> {
    let window = match window {
        None => TubeWindow::Full,
        Some((t0, t1)) => TubeWindow::Arc { t0, t1 },
    };
    tube_along_arc_hollow::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_z(),
        Vec3::unit_x(),
        major,
        window,
        outer,
        wall,
        Tol::witness(),
    )
    .expect("the hollow tube builds")
}

fn rel_close(got: f64, want: f64, rel: f64, what: &str) {
    let e = ((got - want) / want).abs();
    assert!(e < rel, "{what}: got {got}, want {want}, relative {e}");
}

fn tori(body: &Body<f64>) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for (_, face) in body.faces() {
        if let Some(Surface::Torus {
            major_radius,
            minor_radius,
            ..
        }) = body.get_surface(face.surface)
        {
            out.push((*major_radius, *minor_radius));
        }
    }
    out
}

/// Closed forms, re-derived: Pappus on the annulus (section area
/// `π(ro²−ri²)`, centroid on the spine at distance R) for a window;
/// the torus-shell forms for the full period. The thin-wall case
/// keeps the differenced form honest at ~1e-3 relative wall.
#[test]
fn closed_forms_at_varied_geometry() {
    for (major, outer, wall, window) in CASES {
        let inner = outer - wall;
        let t = hollow(major, outer, wall, window);
        let props = topo::props::mass_properties(&t.body, Tol::witness()).expect("props");
        let ring = outer * outer - inner * inner;
        let (v, a) = match window {
            Some((t0, t1)) => {
                let th = t1 - t0;
                (
                    th * major * PI * ring,
                    th * major * 2.0 * PI * (outer + inner) + 2.0 * PI * ring,
                )
            }
            None => (
                2.0 * PI * PI * major * ring,
                4.0 * PI * PI * major * (outer + inner),
            ),
        };
        let what = format!("R={major} ro={outer} w={wall} win={window:?}");
        rel_close(props.volume, v, 1e-9, &format!("volume {what}"));
        rel_close(props.surface_area, a, 1e-9, &format!("area {what}"));
    }
}

/// Solid minus hollow of the same window is exactly the bore — the
/// hollow door leaves the OUTER geometry alone.
#[test]
fn the_bore_differences_out_against_the_solid_door() {
    for (major, outer, wall, window) in CASES {
        let Some((t0, t1)) = window else { continue };
        let inner = outer - wall;
        let h = hollow(major, outer, wall, window);
        let s = tube_along_arc::<f64>(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::unit_z(),
            Vec3::unit_x(),
            major,
            TubeWindow::Arc { t0, t1 },
            outer,
            Tol::witness(),
        )
        .expect("the solid tube builds");
        let hp = topo::props::mass_properties(&h.body, Tol::witness()).expect("props");
        let sp = topo::props::mass_properties(&s.body, Tol::witness()).expect("props");
        rel_close(
            sp.volume - hp.volume,
            (t1 - t0) * major * PI * inner * inner,
            1e-9,
            "bore",
        );
    }
}

/// Adversarial bit patterns: every value a non-terminating binary
/// fraction. The stored minors are `{outer, outer, inner, inner}` on
/// the bits, and the CALLER's recovery — repeating the one IEEE
/// subtraction — reproduces the stored inner radius exactly.
#[test]
fn intent_bits_survive_adversarial_values() {
    let major = 2.000_000_000_000_000_4;
    let outer = 0.300_000_000_000_000_04;
    let wall = 0.1;
    for window in [None, Some((0.7, 3.3))] {
        let t = hollow(major, outer, wall, window);
        let walls = tori(&t.body);
        assert_eq!(walls.len(), 4);
        let recovered = outer - wall; // the caller's own recovery
        let mut minors: Vec<u64> = walls.iter().map(|(_, m)| m.to_bits()).collect();
        minors.sort_unstable();
        let mut want = vec![
            outer.to_bits(),
            outer.to_bits(),
            recovered.to_bits(),
            recovered.to_bits(),
        ];
        want.sort_unstable();
        assert_eq!(minors, want, "stored minors are the caller's numbers");
        for (maj, _) in &walls {
            assert_eq!(maj.to_bits(), major.to_bits(), "major verbatim");
        }
    }
}

/// The `u_ref` caveat, both directions: `t0 = 0` stores the caller's
/// `u_ref` bit-exactly (the rotation by zero is exact), `t0 ≠ 0`
/// stores the derived start direction, which is NOT the caller's
/// vector — the "full-period-only" wording under-promises but never
/// lies.
#[test]
fn u_ref_caveat_is_honest_in_both_directions() {
    let get_u_refs = |t0: f64| -> Vec<Vec3<f64>> {
        let t = tube_along_arc_hollow::<f64>(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::unit_z(),
            Vec3::unit_x(),
            2.5,
            TubeWindow::Arc { t0, t1: t0 + 1.5 },
            0.6,
            0.2,
            Tol::witness(),
        )
        .expect("builds");
        let mut out = Vec::new();
        for (_, face) in t.body.faces() {
            if let Some(Surface::Torus { u_ref, .. }) = t.body.get_surface(face.surface) {
                out.push(*u_ref);
            }
        }
        out
    };
    for u in get_u_refs(0.0) {
        assert_eq!(
            (u.x.to_bits(), u.y.to_bits(), u.z.to_bits()),
            (1.0f64.to_bits(), 0.0f64.to_bits(), 0.0f64.to_bits()),
            "t0 = 0 stores the caller's u_ref"
        );
    }
    for u in get_u_refs(0.9) {
        assert!(
            (u.x, u.y, u.z) != (1.0, 0.0, 0.0),
            "t0 ≠ 0 stores the DERIVED start direction, as documented"
        );
    }
}

/// The flagged deviation, exercised where a bracket read and the
/// meter disagree. `1e-20` m is positive to a comparison and NOT a
/// wall at any run band; an in-band wall (and an in-band bore) must
/// ESCALATE, not pass. These rows go red exactly when the two decides
/// are replaced by the brief's plain bracket read.
#[test]
fn the_metered_wall_refuses_what_a_bracket_read_would_pass() {
    let eps = Tol::witness().eps();
    let k = Tol::witness().k();
    // Geometric mean of the band's two edges: strictly inside
    // (ε, Kε) for every K > 1, so the row does not assume a K.
    let in_band = eps * k.sqrt();
    let build = |outer: f64, wall: f64| {
        tube_along_arc_hollow::<f64>(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::unit_z(),
            Vec3::unit_x(),
            2.0,
            TubeWindow::Full,
            outer,
            wall,
            Tol::witness(),
        )
    };
    // Sub-ε: a sliver a comparison would build.
    assert!(matches!(build(0.5, 1e-20), Err(TubeError::NonpositiveWall)));
    assert!(matches!(
        build(0.5, eps * 0.5),
        Err(TubeError::NonpositiveWall)
    ));
    // In the ambiguity band: escalates with its margin.
    assert!(matches!(
        build(0.5, in_band),
        Err(TubeError::Escalated { .. })
    ));
    // The bore's own band: outer − wall lands in (ε, Kε).
    let outer = 0.5;
    let wall = outer - in_band;
    assert!(matches!(
        build(outer, wall),
        Err(TubeError::Escalated { .. })
    ));
    // And a bore of half an ε is no bore at all.
    assert!(matches!(
        build(outer, outer - eps * 0.5),
        Err(TubeError::WallExceedsRadius)
    ));
}

/// The full period's cavity at varied geometry: a second shell that
/// is a real void — two torus faces at the inner radius, distinct
/// from the outer shell, one per solid.
#[test]
fn the_cavity_is_a_two_wall_torus_at_the_inner_bits() {
    for (major, outer, wall, window) in CASES {
        if window.is_some() {
            continue;
        }
        let t = hollow(major, outer, wall, window);
        assert_eq!(t.body.shells().count(), 2);
        assert_eq!(t.cavities.len(), 1);
        assert_ne!(t.cavities[0], t.shell);
        let cavity = t.body.get_shell(t.cavities[0]).unwrap();
        assert_eq!(cavity.faces.len(), 2);
        let inner = outer - wall;
        for f in &cavity.faces {
            let face = t.body.get_face(*f).unwrap();
            match t.body.get_surface(face.surface) {
                Some(Surface::Torus { minor_radius, .. }) => {
                    assert_eq!(
                        minor_radius.to_bits(),
                        inner.to_bits(),
                        "cavity wall stores minor_radius - wall verbatim"
                    );
                }
                other => panic!("cavity face is not a torus: {other:?}"),
            }
        }
    }
}

/// **The interval rows**: certified builds at varied geometry, closed
/// forms inside the enclosure — and the enclosure held TIGHT (pads
/// bounded relative to the quantity), which is the direction the unit
/// suite's own containment rows cannot pin.
#[cfg(feature = "interval")]
mod certified {
    use geom_core::interval::Interval;
    use geom_core::{Bounds, Real};

    use super::*;

    fn iv(x: f64) -> Interval {
        <Interval as Real>::from_f64(x)
    }

    #[test]
    fn enclosures_contain_and_stay_tight() {
        for (major, outer, wall, window) in CASES {
            let win = match window {
                None => TubeWindow::Full,
                Some((t0, t1)) => TubeWindow::Arc {
                    t0: iv(t0),
                    t1: iv(t1),
                },
            };
            let t = tube_along_arc_hollow::<Interval>(
                Point3::new(iv(0.0), iv(0.0), iv(0.0)),
                Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
                Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
                iv(major),
                win,
                iv(outer),
                iv(wall),
                Tol::witness(),
            )
            .expect("builds at Interval");
            let m = topo::props::mass_properties(&t.body, Tol::witness()).expect("props");
            let inner = outer - wall;
            let ring = outer * outer - inner * inner;
            let v = match window {
                Some((t0, t1)) => (t1 - t0) * major * PI * ring,
                None => 2.0 * PI * PI * major * ring,
            };
            let what = format!("R={major} ro={outer} w={wall} win={window:?}");
            let (lo, hi) = (
                Bounds::lo(m.volume) - m.volume_pad,
                Bounds::hi(m.volume) + m.volume_pad,
            );
            assert!(lo <= v && v <= hi, "{what}: volume {v} ∉ [{lo}, {hi}]");
            // Tightness: the pad and the interval width both small
            // against the quantity — red when certification DEGRADES.
            assert!(
                m.volume_pad <= 1e-6 * v.abs(),
                "{what}: volume pad {} degraded past 1e-6 relative",
                m.volume_pad
            );
            assert!(
                (Bounds::hi(m.volume) - Bounds::lo(m.volume)) <= 1e-6 * v.abs(),
                "{what}: volume interval width degraded past 1e-6 relative"
            );
        }
    }
}
