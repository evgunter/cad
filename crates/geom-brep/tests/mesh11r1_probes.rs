//! **Reviewer probes for the arc-branch door** (independent review of
//! the MESH-11 head). Nothing here is a fix; every row either attacks
//! the door's band from the outside or records a measurement the unit's
//! own suite does not take.
//!
//! Offsets are expressed in the run's OWN `Band` (`zero`, `escalate`),
//! never as an eps literal, so the rows read the same at every ε row of
//! the matrix.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
use geom_brep::props::{
    LoopEdge, PropsError, curved_face, require_iso_rectangle, require_one_chart_branch,
};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};

fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}
fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}
fn edge(carrier: Curve3<f64>, a: f64, b: f64, start: u32, end: u32) -> LoopEdge<f64> {
    let (t0, t1, forward) = if a < b { (a, b, true) } else { (b, a, false) };
    LoopEdge::hand_built(carrier, t0, t1, forward, start, end)
}

const RS: f64 = 0.010;

fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: RS,
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

fn rim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, RS * v.sin()),
            axis: v3(0.0, 0.0, 1.0),
            radius: RS * v.cos(),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        a,
        b,
    )
}

/// The meridian great circle at azimuth `u`; its parameter IS the
/// latitude on the `u` side, so `π/2` is the north pole and `-π/2`
/// (equivalently `3π/2`) the south pole.
fn great(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, 0.0),
            axis: v3(u.sin(), -u.cos(), 0.0),
            radius: RS,
            u_ref: v3(u.cos(), u.sin(), 0.0),
        },
        t0,
        t1,
        a,
        b,
    )
}

/// The half-cap split by a vertex `d` radians BEYOND the north pole:
/// edge 1's span `[π/2 + d, π − b]` ends `d` past the pole going one
/// way, so the pole sits `d` INSIDE edge 2's span `[b, π/2 + d]`.
fn split_past_the_pole(d: f64) -> Vec<LoopEdge<f64>> {
    let b = 0.5;
    let split = core::f64::consts::FRAC_PI_2 + d;
    vec![
        rim(b, 0.0, core::f64::consts::PI, 0, 1),
        great(0.0, core::f64::consts::PI - b, split, 1, 2),
        great(0.0, split, b, 2, 0),
    ]
}

/// **The band attack.** An arc ending 0.25, 1 and 4 band-widths PAST a
/// pole, at whatever ε this run carries. The unit's own rows only
/// exercise `Zero` (0.5× and 0.125× the coincidence threshold) and a
/// decisive `Positive` (10× the escalation threshold); these are the
/// three rungs between, including the INDETERMINATE band, which no row
/// in the unit reaches.
#[test]
fn r1_arcs_ending_past_a_pole_across_the_whole_band() {
    let (zero, esc) = (band().zero(), band().escalate());
    println!(
        "R1 band: zero={zero:e} escalate={esc:e} eps={:e}",
        Tol::witness().get().eps
    );
    // Deviations in METRES, then converted to the arc's own angular
    // offset by the lever R — the door's own dimensional story.
    let rungs: [(&str, f64); 5] = [
        ("0.25 * zero", 0.25 * zero),
        ("1.00 * zero", zero),
        ("0.5*(zero+esc) [indeterminate]", 0.5 * (zero + esc)),
        ("1.00 * escalate", esc),
        ("4.00 * escalate", 4.0 * esc),
    ];
    for (name, dev) in rungs {
        let face = split_past_the_pole(dev / RS);
        let shape = require_iso_rectangle(&sphere(), &face, band());
        let branch = require_one_chart_branch(&sphere(), &face, band());
        println!("R1 rung {name}: dev={dev:e} m  shape={shape:?}  branch={branch:?}");
    }
    // The two ends of the ladder are the claims worth pinning: below
    // the coincidence threshold the door MUST admit (CERT-1's split
    // vertex row is the same shape), and decisively above the
    // escalation threshold it MUST refuse.
    assert_eq!(
        require_one_chart_branch(&sphere(), &split_past_the_pole(0.25 * zero / RS), band()),
        Ok(()),
        "a quarter of the coincidence threshold past the pole is an arc that ENDS at it"
    );
    assert!(
        matches!(
            require_one_chart_branch(&sphere(), &split_past_the_pole(4.0 * esc / RS), band()),
            Err(PropsError::NotOneChartBranch { edge: 2, .. })
        ),
        "four escalation widths past the pole is a definite crossing"
    );
    // The indeterminate rung is the one the unit argues about and does
    // not execute: a margin between the two thresholds ADMITS.
    assert_eq!(
        require_one_chart_branch(
            &sphere(),
            &split_past_the_pole(0.5 * (zero + esc) / RS),
            band()
        ),
        Ok(()),
        "an indeterminate membership margin admits, as the doc claims"
    );
}

/// **The SOUTH pole is the second margin, and nothing in the unit's
/// suite crosses it.** Every refusing sphere row in
/// `mesh11_arc_branch.rs` crosses the north pole, i.e. entry 0 of
/// `sphere_meridian_pole_margins`' pair. This crosses the south pole
/// only.
#[test]
fn r1_a_south_pole_crossing_arc_is_refused_too() {
    let b = -0.5;
    let face = vec![
        rim(b, 0.0, core::f64::consts::PI, 0, 1),
        great(0.0, b, -core::f64::consts::PI - b, 1, 0),
    ];
    let branch = require_one_chart_branch(&sphere(), &face, band());
    println!(
        "R1 south-pole face: shape={:?} branch={branch:?}",
        require_iso_rectangle(&sphere(), &face, band())
    );
    assert!(
        matches!(branch, Err(PropsError::NotOneChartBranch { .. })),
        "a span containing the SOUTH pole leaves the branch too; got {branch:?}"
    );
}

/// **CERT-1's multi-wrap face: the flux lane measures it, the branch
/// door refuses it.** `cert1_sphere_polar::a_multi_wrap_span_covers_
/// both_poles` asserts the closed form on a 3π span; the claim under
/// review is that the branch door refuses exactly there. Both halves,
/// on one face.
#[test]
fn r1_the_multi_wrap_span_splits_the_two_doors() {
    let pi = core::f64::consts::PI;
    let pair = vec![
        great(0.0, 0.0, 3.0 * pi, 0, 1),
        great(0.0, 3.0 * pi, 4.0 * pi, 1, 0),
    ];
    assert_eq!(require_iso_rectangle(&sphere(), &pair, band()), Ok(()));
    let fc = curved_face(&sphere(), &pair, 1.0, band());
    let exact = 2.0 * pi * RS * RS;
    let branch = require_one_chart_branch(&sphere(), &pair, band());
    println!(
        "R1 multi-wrap: flux={:?} exact={exact} branch={branch:?}",
        fc.map(|c| c.area)
    );
    assert!(
        matches!(branch, Err(PropsError::NotOneChartBranch { .. })),
        "a 3pi span contains both poles in its interior; got {branch:?}"
    );
}

/// **The cone arm has no rim/meridian guard of its own — it reads
/// EVERY `Line` carrier as a generator.** The sphere arm filters on
/// `props_circle_axis_class` first; the cone arm does not filter at
/// all, so what keeps it honest is entirely the layering claim (the
/// shape door ran first). Recorded, called alone, on a line that lies
/// on no cone at all.
#[test]
fn r1_the_cone_arm_reads_any_line_as_a_generator() {
    let cone = Surface::Cone {
        apex: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: core::f64::consts::FRAC_PI_4,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    // A line NOWHERE near the cone, whose foot-point parameter happens
    // to fall inside its stored span.
    let stray = vec![edge(
        Curve3::Line {
            origin: p3(5.0, 5.0, 5.0),
            dir: v3(1.0, 0.0, 0.0),
        },
        -10.0,
        10.0,
        0,
        1,
    )];
    let shape = require_iso_rectangle(&cone, &stray, band());
    let branch = require_one_chart_branch(&cone, &stray, band());
    println!("R1 stray line on a cone: shape={shape:?} branch={branch:?}");
    assert!(
        shape.is_err(),
        "the SHAPE door is what refuses a line that is no generator; got {shape:?}"
    );
    assert!(
        matches!(branch, Err(PropsError::NotOneChartBranch { .. })),
        "recorded: the branch door answers on a carrier it never certified; got {branch:?}"
    );
}
