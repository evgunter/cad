//! **The walk's arc premise, verified at the door** (issue 1571).
//!
//! `walk.rs` needs every boundary edge to be an iso curve of the chart
//! traversed on ONE branch. `props::require_iso_rectangle` certifies
//! the CARRIER; `props::require_one_chart_branch` certifies the
//! traversed ARC, and `curved::require_iso_rectangle_face` cites both
//! in that order. These rows are the two π-rad witnesses the walk's
//! ledger names, from the outside, through the public `tessellate`.
//!
//! Each witness is a body that used to reach the walk: with debug
//! assertions on it panicked at the cross-face identification census
//! (issue 897) and with them off the walk returned a NON-watertight
//! mesh `Ok`; at finer δ it refused `CertificateExceeded`, a refusal
//! about the chord certificate rather than about the premise. What
//! these rows pin is that neither outcome is reachable any more: the
//! refusal is typed, names the premise, and comes before any mesh is
//! minted — at EVERY δ, which is the part `CertificateExceeded` never
//! gave.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::witness_bodies::{apex_crossing_bowtie, pole_crossing_half_cap};
use geom_brep::props::PropsError;
use geom_core::Tol;
use mesh::TessellateError;
use topo::{Body, FaceKey};

/// Every face of `body` refuses at the shape door with the branch
/// predicate's own error, at every δ — the refusal is a fact about the
/// body's boundary statement, not about a chord budget.
fn refuses_the_branch_premise(name: &str, body: &Body<f64>, faces: [FaceKey; 2]) {
    let tol = Tol::witness();
    for delta in [0.5, 0.3, 0.2, 0.1, 0.05, 0.02] {
        let got = mesh::tessellate(body, delta, tol);
        match got {
            Err(TessellateError::UnsupportedCurvedShape {
                face,
                source: PropsError::NotOneChartBranch { .. },
            }) => assert!(
                faces.contains(&face),
                "{name} at δ={delta}: refused a face that is not one of {faces:?}"
            ),
            other => {
                panic!("{name} at δ={delta}: expected the branch premise refusal, got {other:?}")
            }
        }
    }
}

/// **Witness 1, the Euler-door body** (issue 1571's own construction):
/// the unit sphere whose ONE meridian edge is a great-circle arc over
/// the north pole. Before this unit the door admitted both faces and
/// the walk ran; now the door names the premise and no mesh is minted.
#[test]
fn the_pole_crossing_half_cap_refuses_at_the_door() {
    let (body, cap, rest) = pole_crossing_half_cap();
    refuses_the_branch_premise("half-cap", &body, [cap, rest]);
}

/// **Witness 2's sibling on the other kind** — the cone bow tie, whose
/// generators run through the apex. Found by this unit's class sweep:
/// the shape door admitted it and the walk mis-read it exactly as it
/// mis-read the half-cap (a debug build panicked at the issue-897
/// census at δ = 0.5, `CertificateExceeded` below it).
#[test]
fn the_apex_crossing_bowtie_refuses_at_the_door() {
    let (body, f0, f1) = apex_crossing_bowtie();
    refuses_the_branch_premise("bow tie", &body, [f0, f1]);
}

/// **The props-side finding is NOT closed by this door, and this row
/// says so with the measurement** (issue 1598). `mass_properties` does
/// not cite the branch predicate — citing it there would retract
/// CERT-1, whose three rows measure pole-crossing arcs exactly — so
/// the half-cap body still answers, and what it answers is 0.0 for a
/// closed unit sphere: its two faces are bounded by the same two edges
/// traversed opposite ways, so one parse hands both the same levels
/// and their fluxes cancel. Tier 3 catches it only through check 6.
#[test]
fn mass_properties_still_answers_zero_on_the_half_cap() {
    let (body, _, _) = pole_crossing_half_cap();
    let mp = topo::mass_properties(&body, Tol::witness()).expect("props answers");
    assert_eq!(
        mp.volume, 0.0,
        "issue 1598: equal-and-opposite flux from one parse, on a closed sphere"
    );
}

/// **The branch door's arms mirror the chart's own singularity
/// enumeration, executed.** `require_one_chart_branch` decides on the
/// sphere and the cone and answers `Ok(())` unconditionally on the
/// cylinder and the torus. That asymmetry is not a choice — it is
/// `Chart::poles()`, the kernel's own list of where a chart is
/// singular: sphere 2, cone 1, cylinder 0, torus 0. The props-side
/// rows cannot say so (`topo` sits above `geom-brep`, so that crate's
/// tests cannot name `Chart`), and a row that only asserts `Ok(())`
/// against an unconditional arm cannot go red at all. This one can, in
/// both directions: give a chart a singularity the door does not
/// decide, or take one away that it does, and the equivalence breaks.
///
/// Each shape below runs its kind's own "span through the axis region"
/// — the arc that WOULD leave the branch if the chart had one there.
#[test]
fn the_branch_doors_arms_mirror_the_charts_own_singularities() {
    use geom::{Curve3, Surface};
    use geom_brep::props::{LoopEdge, require_one_chart_branch};
    use geom_core::{Band, Point3, Vec3};
    use topo::Chart;

    let p3 = |x, y, z| Point3::<f64>::new(x, y, z);
    let v3 = |x, y, z| Vec3::<f64>::new(x, y, z);
    let band = Band::linear(Tol::witness()).unwrap();
    let edge = |c: Curve3<f64>, a: f64, b: f64, s: u32, e: u32| {
        let (t0, t1, fwd) = if a < b { (a, b, true) } else { (b, a, false) };
        LoopEdge::hand_built(c, t0, t1, fwd, s, e)
    };
    let s2 = core::f64::consts::FRAC_1_SQRT_2;
    let pi = core::f64::consts::PI;

    let sphere = Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let cone = Surface::Cone {
        apex: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: core::f64::consts::FRAC_PI_4,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let cylinder = Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let torus = Surface::Torus {
        center: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        major_radius: 1.0,
        minor_radius: 0.25,
        u_ref: v3(1.0, 0.0, 0.0),
    };

    // A great-circle arc from latitude 0.5 OVER the north pole.
    let sphere_face = vec![edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, -1.0, 0.0),
            radius: 1.0,
            u_ref: v3(1.0, 0.0, 0.0),
        },
        0.5,
        pi - 0.5,
        0,
        1,
    )];
    // A generator segment THROUGH the apex.
    let cone_face = vec![edge(
        Curve3::Line {
            origin: p3(0.0, 0.0, 0.0),
            dir: v3(s2, 0.0, s2),
        },
        -1.0,
        1.0,
        0,
        1,
    )];
    // A generator segment spanning the axis LEVEL — the cylinder's
    // nearest analogue, and no singularity is there to cross.
    let cylinder_face = vec![edge(
        Curve3::Line {
            origin: p3(1.0, 0.0, 0.0),
            dir: v3(0.0, 0.0, 1.0),
        },
        -5.0,
        5.0,
        0,
        1,
    )];
    // A minor circle wrapping one and a half turns.
    let torus_face = vec![edge(
        Curve3::Circle {
            center: p3(1.0, 0.0, 0.0),
            axis: v3(0.0, 1.0, 0.0),
            radius: 0.25,
            u_ref: v3(1.0, 0.0, 0.0),
        },
        0.0,
        1.5 * core::f64::consts::TAU,
        0,
        1,
    )];

    for (name, surface, face, poles) in [
        ("sphere", &sphere, &sphere_face, 2),
        ("cone", &cone, &cone_face, 1),
        ("cylinder", &cylinder, &cylinder_face, 0),
        ("torus", &torus, &torus_face, 0),
    ] {
        let chart = Chart::of(surface).expect("an analytic chart");
        assert_eq!(
            chart.poles().len(),
            poles,
            "{name}: the chart's own singularity count moved"
        );
        let refused = require_one_chart_branch(surface, face, band).is_err();
        assert_eq!(
            refused,
            poles > 0,
            "{name}: the branch door decides iff the chart has a singularity to cross \
             ({poles} pole(s), door refused = {refused})"
        );
    }
}
