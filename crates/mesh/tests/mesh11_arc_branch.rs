//! **The walk's arc premise, verified at the door** (issue 1571).
//!
//! `walk.rs` needs every boundary edge to be an iso curve of the chart
//! traversed on ONE branch. `props::require_iso_rectangle` certifies
//! the CARRIER; `props::require_one_chart_branch` certifies the
//! traversed ARC, and `curved::iso_rectangle_shape` asks both in that
//! order and routes the face by their answer. These rows are the two
//! π-rad witnesses the walk's ledger names, from the outside, through
//! the public `tessellate`.
//!
//! Each witness is a body that used to reach the walk: with debug
//! assertions on it panicked at the cross-face identification census
//! (issue 897) and with them off the walk returned a NON-watertight
//! mesh `Ok`; at finer δ it refused `CertificateExceeded`, a refusal
//! about the chord certificate rather than about the premise. What
//! these rows pin is that neither outcome is reachable any more: the
//! doors decide each face is no rectangle, so the route sends it to the
//! trimmed lane, never the walk — at EVERY δ, which is the part
//! `CertificateExceeded` never gave. Both bodies are assembled through
//! the Euler doors, which store no chart image, so the trimmed lane's
//! chord pass refuses them typed before any mesh is minted.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::witness_bodies::{apex_crossing_bowtie, pole_crossing_half_cap};
use geom_brep::props::PropsError;
use geom_core::Tol;
use mesh::TessellateError;
use topo::{Body, FaceKey};

/// What the two doors, in order, answer on `face`'s outer loop.
fn doors(body: &Body<f64>, face: FaceKey) -> Result<(), PropsError> {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let (outer, _) = topo::props::loop_edges(body, body.get_face(face).unwrap().outer).unwrap();
    let surface = body.face_surface(face).unwrap();
    geom_brep::props::require_iso_rectangle(surface, &outer, band)
        .and_then(|()| geom_brep::props::require_one_chart_branch(surface, &outer, band))
}

/// `tessellate` refuses `body` at every δ before any mesh is minted, in
/// the trimmed lane's chord pass: the route took its faces away from the
/// walk, and the Euler-built body stores no chart image to chord by.
fn routed_away_from_the_walk(name: &str, body: &Body<f64>) {
    let tol = Tol::witness();
    for delta in [0.5, 0.3, 0.2, 0.1, 0.05, 0.02] {
        match mesh::tessellate(body, delta, tol) {
            Err(TessellateError::UnsupportedCurve { note, .. })
                if note.contains("carries no stored pcurve cache") => {}
            other => panic!(
                "{name} at δ={delta}: expected the trimmed lane's chord pass to refuse, got \
                 {other:?}"
            ),
        }
    }
}

/// **Witness 1, the Euler-door body** (issue 1571's own construction):
/// the unit sphere whose ONE meridian edge is a great-circle arc over
/// the north pole. The shape door admits both faces and the branch door
/// decides the arc leaves its branch, so neither face reaches the walk.
#[test]
fn the_pole_crossing_half_cap_is_decided_off_one_branch_at_the_door() {
    let (body, cap, rest) = pole_crossing_half_cap();
    for face in [cap, rest] {
        assert!(
            matches!(
                doors(&body, face),
                Err(PropsError::NotOneChartBranch { .. })
            ),
            "half-cap face {face:?}: {:?}",
            doors(&body, face)
        );
    }
    routed_away_from_the_walk("half-cap", &body);
}

/// **Witness 2's sibling on the other kind** — the cone bow tie, whose
/// generators run through the apex. Found by this unit's class sweep:
/// the shape door admitted it and the walk mis-read it exactly as it
/// mis-read the half-cap (a debug build panicked at the issue-897
/// census at δ = 0.5, `CertificateExceeded` below it).
///
/// **The shape door reaches this body first, and by its own name.**
/// `require_iso_rectangle` requires every rim to encode the same
/// material side, and the bow tie's faces state two rims that do not:
/// its verdict is `props_rim_side`, decided before
/// `require_one_chart_branch` is asked. The branch premise's own cone
/// witness is the props-level row
/// `an_apex_crossing_generator_is_not_one_chart_branch` (`geom-brep`),
/// on a hand-built bow whose rims do not contradict; this body does not
/// reach the branch door through `tessellate`, and the row for a body
/// that does is
/// `work/tesslane/apex-crossing-branch-premise-has-no-body-level-witness.md`.
#[test]
fn the_apex_crossing_bowtie_is_decided_no_rectangle_at_the_shape_door() {
    let (body, f0, f1) = apex_crossing_bowtie();
    for face in [f0, f1] {
        assert_eq!(
            doors(&body, face),
            Err(PropsError::NotIsoRectangle {
                what: "props_rim_side"
            }),
            "bow tie face {face:?}"
        );
    }
    routed_away_from_the_walk("bow tie", &body);
}

/// **The props-side finding is closed by the side reader, not by this
/// door** (issue 1598). `mass_properties` still does not cite the
/// branch predicate — citing it there would retract CERT-1, whose rows
/// measure pole-crossing arcs exactly — and the half-cap A still
/// measures, exactly as before. What changed is the complement: the
/// north pole lies on its loop and the south pole inside it, and a
/// chart pole inside a face is a vertex of it, so the face's bit
/// contradicts the side its loop encodes and the closed form refuses
/// `SenseContradicted`. The body answered `0.0` for a closed unit
/// sphere — two faces bounded by the same two edges traversed opposite
/// ways, equal and opposite fluxes — and now it answers the refusal,
/// naming the face it came from.
#[test]
fn mass_properties_reports_the_side_refusal_on_the_half_cap() {
    let (body, cap, rest) = pole_crossing_half_cap();
    // Raw door: the measured body is a body still carrying construction scaffolding, which tier 3 refuses (ScaffoldAtRest).
    let got = topo::test_support::measure_unvalidated(
        &body,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    );
    let Err(topo::MassPropsError::Face { face, source }) = got else {
        panic!("issue 1598: the closed sphere must refuse, not measure; got {got:?}");
    };
    assert_eq!(face, rest, "the L-shaped complement is the refusing face");
    assert_eq!(source, PropsError::SenseContradicted);
    // Face A is untouched: the half-cap's own closed form measures it
    // exactly, which is what kept this defect out of reach of a branch
    // door (CERT-1's rows admit the pole-crossing arc on purpose).
    let face_a = body.get_face(cap).expect("the half-cap");
    let surface = body.face_surface(cap).expect("its sphere");
    let (outer, _) = topo::props::loop_edges(&body, face_a.outer).expect("its loop");
    let band = geom_core::Band::linear(Tol::witness()).expect("band");
    let fc = geom_brep::props::curved_face(surface, &outer, face_a.sense, band)
        .expect("the half-cap still measures");
    let exact = core::f64::consts::PI * (1.0 - 0.5);
    assert!(
        (fc.area - exact).abs() < 1e-12 * exact,
        "half-cap area {} vs exact {exact}",
        fc.area
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

    let band = Band::linear(Tol::witness()).unwrap();
    let edge = |c: Curve3<f64>, a: f64, b: f64, s: u32, e: u32| {
        let (t0, t1, fwd) = if a < b { (a, b, true) } else { (b, a, false) };
        LoopEdge::hand_built(c, t0, t1, fwd, s, e)
    };
    let s2 = core::f64::consts::FRAC_1_SQRT_2;
    let pi = core::f64::consts::PI;

    let sphere = Surface::Sphere {
        center: Point3::<f64>::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::<f64>::new(0.0, 0.0, 1.0),
        u_ref: Vec3::<f64>::new(1.0, 0.0, 0.0),
    };
    let cone = Surface::Cone {
        apex: Point3::<f64>::new(0.0, 0.0, 0.0),
        axis: Vec3::<f64>::new(0.0, 0.0, 1.0),
        half_angle: core::f64::consts::FRAC_PI_4,
        u_ref: Vec3::<f64>::new(1.0, 0.0, 0.0),
    };
    let cylinder = Surface::Cylinder {
        origin: Point3::<f64>::new(0.0, 0.0, 0.0),
        axis: Vec3::<f64>::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::<f64>::new(1.0, 0.0, 0.0),
    };
    let torus = Surface::Torus {
        center: Point3::<f64>::new(0.0, 0.0, 0.0),
        axis: Vec3::<f64>::new(0.0, 0.0, 1.0),
        major_radius: 1.0,
        minor_radius: 0.25,
        u_ref: Vec3::<f64>::new(1.0, 0.0, 0.0),
    };

    // A great-circle arc from latitude 0.5 OVER the north pole.
    let sphere_face = vec![edge(
        Curve3::circle(
            Point3::<f64>::new(0.0, 0.0, 0.0),
            Vec3::<f64>::new(0.0, -1.0, 0.0),
            1.0,
            Vec3::<f64>::new(1.0, 0.0, 0.0),
        ),
        0.5,
        pi - 0.5,
        0,
        1,
    )];
    // A generator segment THROUGH the apex.
    let cone_face = vec![edge(
        Curve3::line(
            Point3::<f64>::new(0.0, 0.0, 0.0),
            Vec3::<f64>::new(s2, 0.0, s2),
        ),
        -1.0,
        1.0,
        0,
        1,
    )];
    // A generator segment spanning the axis LEVEL — the cylinder's
    // nearest analogue, and no singularity is there to cross.
    let cylinder_face = vec![edge(
        Curve3::line(
            Point3::<f64>::new(1.0, 0.0, 0.0),
            Vec3::<f64>::new(0.0, 0.0, 1.0),
        ),
        -5.0,
        5.0,
        0,
        1,
    )];
    // A minor circle wrapping one and a half turns.
    let torus_face = vec![edge(
        Curve3::circle(
            Point3::<f64>::new(1.0, 0.0, 0.0),
            Vec3::<f64>::new(0.0, 1.0, 0.0),
            0.25,
            Vec3::<f64>::new(1.0, 0.0, 0.0),
        ),
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
