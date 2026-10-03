//! **A swept cusp is legal at rest.** A `.cusp()` joint sweeps an edge
//! at material wedge 0 (2π on a hole loop). The profile is where that
//! tangency's intent is declared; at rest the edge is legal because
//! its tangency is jet-determinate — derived from the body exactly as
//! a π seam is (D1's second-order arm) — so `extrude` and `revolve`
//! hand back a body `topo::validate_geometric` passes with nothing
//! carried beside it.
//!
//! Each row reads the pass against check 4's own reading of the body:
//! the cusp edges must be marked `Tangent` (a jet-determinate
//! tangency the material arm judged), so a body that passed because
//! the arm skipped its cusp goes red here, and so does one whose cusp
//! refused.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::test_support::bulge_loop;
use profile::{Open, Profile, ProfileLoop, RawLoop, SketchPlane, Start, ValidatedProfile};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, loft_body, revolve};
use topo::{Body, ContactMark, EdgeKey};

/// The lune: the lip between the internally tangent circles (0,1) r 1
/// and (0,2) r 2, `.cusp()` at the kiss (canonical joint 2).
fn lune() -> ProfileLoop<f64> {
    let tol = Tol::witness();
    Open.at(Point2::new(0.0, 4.0))
        .angle(-std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .line(2.0, tol)
        .unwrap()
        .turn(std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .tangent_arc_to(Point2::new(0.0, 0.0), tol)
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, tol)
        .unwrap()
        .into()
}

/// A crescent between the unit circle (centre on the revolve axis
/// `x = 0`) and its tangent line at 45°, `.cusp()` at the tangency: the
/// revolve sweeps a sphere zone and a cone meeting at a cusp rim. Off
/// the axis, so its full revolve sweeps one full-period band with no
/// axis contact.
fn sphere_cone_crescent() -> ProfileLoop<f64> {
    let tol = Tol::witness();
    let h = std::f64::consts::FRAC_1_SQRT_2;
    Open.at(Point2::new(1.0, 0.0))
        .angle(std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .tangent_arc_to(Point2::new(h, h), tol)
        .unwrap()
        .cusp()
        .line(1.0, tol)
        .unwrap()
        .line_to(Start, tol)
        .unwrap()
        .into()
}

/// The same sphere–cone cusp closed ON the axis: up the axis from the
/// sphere's pole, down the tangent line, back along the sphere — the
/// full revolve's wire case, whose two π-bands each carry the cusp.
fn sphere_cone_on_axis() -> ProfileLoop<f64> {
    let tol = Tol::witness();
    Open.at(Point2::new(0.0, 1.0))
        .angle(std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .line(std::f64::consts::SQRT_2 - 1.0, tol)
        .unwrap()
        .turn(-3.0 * std::f64::consts::FRAC_PI_4, tol)
        .unwrap()
        .line(1.0, tol)
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, tol)
        .unwrap()
        .into()
}

fn validated(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .expect("the declared-cusp profile validates")
}

/// The row every analytic-walled verb answers: the body is tier-3
/// valid, and check 4 marked exactly `cusps` edges `Tangent` — a
/// jet-determinate tangency it JUDGED rather than exempted — every one
/// of them with both endpoints where the profile's cusp joint swept
/// to (`at`). The mark alone does not say "cusp" (a π seam is marked
/// the same); the location does, since the profile's other joints are
/// corners. Returns the marked edges.
fn tangent_marks_at_the_cusp(
    body: &Body<f64>,
    at: impl Fn(&Point3<f64>) -> bool,
    cusps: usize,
) -> Vec<EdgeKey> {
    let tol = Tol::witness();
    assert_eq!(
        topo::validate_geometric(body, tol),
        Ok(()),
        "the jet-determinate cusp is legal at rest"
    );
    let marks = topo::contact_marks(body, tol).expect("the body is valid");
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let tangent: Vec<EdgeKey> = marks
        .iter()
        .filter(|(_, m)| **m == ContactMark::Tangent)
        .map(|(e, _)| e)
        .collect();
    assert_eq!(tangent.len(), cusps, "{marks:?}");
    for &e in &tangent {
        let he = body.get_edge(e).unwrap().he_plus;
        let ends = [
            point(body.get_half_edge(he).unwrap().start),
            point(body.half_edge_end(he).unwrap()),
        ];
        assert!(
            ends.iter().all(&at),
            "a Tangent mark off the cusp: {ends:?}"
        );
    }
    tangent
}

/// On the lune's kiss line `x = y = 0`.
fn on_the_kiss(p: &Point3<f64>) -> bool {
    p.x.abs() < 1e-9 && p.y.abs() < 1e-9
}

/// On the circle the crescent's cusp joint `(h, h)` revolves to about
/// the sketch's y axis: height `h`, radius `h`.
fn on_the_rim(p: &Point3<f64>) -> bool {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    (p.y - h).abs() < 1e-9 && (p.x.hypot(p.z) - h).abs() < 1e-9
}

#[test]
fn a_cusp_extrude_is_legal_either_way_it_extrudes() {
    let profile = validated(vec![lune()]);
    for d in [1.0, -1.0] {
        let built = extrude(&profile, Extrusion::Distance(d), Tol::witness()).unwrap();
        let cusps = tangent_marks_at_the_cusp(&built.body, on_the_kiss, 1);
        assert!(
            built.strut_edges()[0].contains(&Some(cusps[0])),
            "the tangency is the strut the cusp joint swept"
        );
    }
}

#[test]
fn a_hole_cusp_extrudes_to_a_legal_slit() {
    let plate = bulge_loop(vec![
        (Point2::new(-1.0, -1.0), 0.0),
        (Point2::new(3.0, -1.0), 0.0),
        (Point2::new(3.0, 5.0), 0.0),
        (Point2::new(-1.0, 5.0), 0.0),
    ]);
    let profile = validated(vec![plate, lune()]);
    let built = extrude(&profile, Extrusion::Distance(1.0), Tol::witness()).unwrap();
    let cusps = tangent_marks_at_the_cusp(&built.body, on_the_kiss, 1);
    assert!(
        built.strut_edges()[1].contains(&Some(cusps[0])),
        "the hole's strut"
    );
}

#[test]
fn a_cusp_revolve_is_legal_partial_and_full() {
    let profile = validated(vec![sphere_cone_crescent()]);
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    for revolution in [Revolution::Partial(1.0), Revolution::Full] {
        let built = revolve(&profile, axis, revolution, Tol::witness()).unwrap();
        let cusps = tangent_marks_at_the_cusp(&built.body, on_the_rim, 1);
        assert!(
            built.rims[0].iter().flatten().any(|r| *r == cusps[0]),
            "the tangency is the rim the cusp joint swept"
        );
    }
}

/// The full revolve of an axis-touching profile sweeps two π-bands, so
/// its one cusp joint is two cusp rims — one per band — and both are
/// legal.
#[test]
fn a_revolve_wire_case_cusp_is_legal_on_both_bands() {
    let profile = validated(vec![sphere_cone_on_axis()]);
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let built = revolve(&profile, axis, Revolution::Full, Tol::witness()).unwrap();
    tangent_marks_at_the_cusp(&built.body, on_the_rim, 2);
}

/// **Not a validation of the cusp.** Loft walls are NURBS, whose edges
/// tier 3's material arm exempts BY KIND: the seam at the cusp joint is
/// unjudged — marked `Unmarked`, the escalation posture — and the body
/// passes because nothing about that seam was asked, not because the
/// cusp was found legal.
#[test]
fn a_cusp_loft_passes_with_its_nurbs_seam_unjudged_by_kind() {
    let places: Vec<Affine3<f64>> = [0.0, 1.0]
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    let built = loft_body::<f64>(&[vec![lune()], vec![lune()]], &places, 1, Tol::witness())
        .expect("the lune lofts");
    assert_eq!(
        topo::validate_geometric(&built.body, Tol::witness()),
        Ok(())
    );
    let marks = topo::contact_marks(&built.body, Tol::witness()).expect("valid");
    assert_eq!(
        marks.get(built.seam_edges[0][2]),
        Some(&ContactMark::Unmarked),
        "the cusp seam is unjudged by kind"
    );
}

/// The lune authored RAW (bulges + `with_tangent_joints`), not through
/// the `.cusp()` door: the joint sweeps the same legal cusp.
fn raw_lune() -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(0.0, 4.0), 0.0),
        (Point2::new(0.0, 2.0), -1.0),
        (Point2::new(0.0, 0.0), 1.0),
    ])
    .with_tangent_joints(vec![2])
}

#[test]
fn a_raw_authored_cusp_is_legal_like_the_door() {
    let profile = validated(vec![raw_lune()]);
    assert_eq!(profile.loops()[0].tangent_joints(), &[2]);
    for d in [1.0, -1.0] {
        let built = extrude(&profile, Extrusion::Distance(d), Tol::witness()).unwrap();
        tangent_marks_at_the_cusp(&built.body, on_the_kiss, 1);
    }
}

#[test]
fn a_hole_cusp_is_a_legal_slit_at_either_sign_and_either_winding() {
    let plate = bulge_loop(vec![
        (Point2::new(-1.0, -1.0), 0.0),
        (Point2::new(3.0, -1.0), 0.0),
        (Point2::new(3.0, 5.0), 0.0),
        (Point2::new(-1.0, 5.0), 0.0),
    ]);
    // Hole authored the "right" (CW) way AND the wrong way.
    for hole in [lune(), raw_lune()] {
        let profile = validated(vec![plate.clone(), hole]);
        for d in [1.0, -1.0] {
            let built = extrude(&profile, Extrusion::Distance(d), Tol::witness()).unwrap();
            tangent_marks_at_the_cusp(&built.body, on_the_kiss, 1);
        }
    }
}

fn crescent_raw(far: Point2<f64>, declared: bool) -> ProfileLoop<f64> {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let lp = bulge_loop(vec![
        (Point2::new(1.0, 0.0), (std::f64::consts::PI / 16.0).tan()),
        (Point2::new(h, h), 0.0),
        (far, 0.0),
    ]);
    if declared {
        lp.with_tangent_joints(vec![1])
    } else {
        lp
    }
}

/// Sections that DISAGREE at one joint: a cusp in one, a corner in the
/// other, same arc (so the loft's pcurve lane admits it). The seam is
/// NURBS-adjacent, so it stays unjudged by kind either way — this row
/// pins that the pass is that exemption, not a verdict.
#[test]
fn a_loft_whose_sections_disagree_passes_with_the_seam_unjudged_by_kind() {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let a = crescent_raw(Point2::new(2.0 * h, 0.0), true);
    let b = crescent_raw(Point2::new(1.5, 0.2), false);
    let va = validated(vec![a.clone()]);
    let vb = validated(vec![b.clone()]);
    assert_eq!(
        vb.loops()[0].tangent_joints(),
        &[] as &[usize],
        "B's joint is a corner"
    );
    let j = va.loops()[0].tangent_joints()[0];
    let places: Vec<Affine3<f64>> = [0.0, 1.0]
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    for sections in [
        [vec![a.clone()], vec![b.clone()]],
        [vec![b.clone()], vec![a.clone()]],
    ] {
        let built = loft_body::<f64>(&sections, &places, 1, Tol::witness())
            .expect("mixed sections loft (same arc, different far vertex)");
        assert_eq!(
            topo::validate_geometric(&built.body, Tol::witness()),
            Ok(())
        );
        let marks = topo::contact_marks(&built.body, Tol::witness()).expect("valid");
        assert_eq!(
            marks.get(built.seam_edges[0][j]),
            Some(&ContactMark::Unmarked)
        );
    }
}
