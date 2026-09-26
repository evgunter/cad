//! **A sweep verb carries the contacts its profile declared.** A
//! `.cusp()` joint sweeps an edge at material wedge 0 (2π on a hole
//! loop), which tier 3 holds legal exactly where the edge's wall pair
//! is declared in `Tangent` contact. The profile made that
//! declaration, so the verb's output carries it: `extrude`, `revolve`
//! and `loft_body` each return `declared_contacts`, and the body
//! validates through `topo::validate_geometric_declared` with nothing
//! rebuilt by the caller.
//!
//! Each row reads the carried record against the body's OWN refusal:
//! the undeclared gate names the cusp edges, and the record must be
//! exactly those edges' face pairs — so a verb that stops carrying,
//! carries the wrong pair, or carries a pair for a smooth joint goes
//! red here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::MaterialWedge;
use geom_core::{Affine3, Point2, Tol, Vec2, Vec3};
use profile::test_support::bulge_loop;
use profile::{Open, Profile, ProfileLoop, RawLoop, SketchPlane, Start, ValidatedProfile};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, loft_body, revolve};
use topo::{Body, ContactClass, DeclaredContact, EdgeKey, FaceKey, ValidationError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The lune: the lip between the internally tangent circles (0,1) r 1
/// and (0,2) r 2, `.cusp()` at the kiss (canonical joint 2).
fn lune() -> ProfileLoop<f64> {
    let tol = Tol::witness();
    Open.at(p2(0.0, 4.0))
        .angle(-std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .line(2.0, tol)
        .unwrap()
        .turn(std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .tangent_arc_to(p2(0.0, 0.0), tol)
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, tol)
        .unwrap()
        .into()
}

/// A crescent between the unit circle (centre on the revolve axis
/// `x = 0`) and its tangent line at 45°, `.cusp()` at the tangency: the
/// revolve sweeps a sphere zone and a cone meeting at a cusp rim. Off
/// the axis, so a revolve of it is the lamina case.
fn sphere_cone_crescent() -> ProfileLoop<f64> {
    let tol = Tol::witness();
    let h = std::f64::consts::FRAC_1_SQRT_2;
    Open.at(p2(1.0, 0.0))
        .angle(std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .tangent_arc_to(p2(h, h), tol)
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
    Open.at(p2(0.0, 1.0))
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

fn edge_faces(body: &Body<f64>, edge: EdgeKey) -> [FaceKey; 2] {
    let e = body.get_edge(edge).expect("a live edge");
    let face_of = |he| {
        let l = body.get_half_edge(he).expect("live half-edge").parent_loop;
        body.get_loop(l).expect("live loop").face
    };
    let mut pair = [face_of(e.he_plus), face_of(e.he_minus)];
    pair.sort();
    pair
}

fn carried_pairs(contacts: &[DeclaredContact]) -> Vec<[FaceKey; 2]> {
    let mut pairs: Vec<[FaceKey; 2]> = contacts
        .iter()
        .map(|d| {
            assert_eq!(d.class, ContactClass::Tangent, "{d:?}");
            let mut p = [d.a, d.b];
            p.sort();
            p
        })
        .collect();
    pairs.sort();
    pairs
}

/// The row every analytic-walled verb answers: undeclared, the body
/// refuses with `cusps` wedge-`wedge` edges and nothing else; the
/// carried record is exactly those edges' face pairs; with it, the
/// body is tier-3 valid.
fn carries_exactly_its_cusps(
    body: &Body<f64>,
    contacts: &[DeclaredContact],
    wedge: MaterialWedge,
    cusps: usize,
) {
    let tol = Tol::witness();
    let errs = topo::validate_geometric(body, tol).expect_err("undeclared, the cusp refuses");
    let mut refused: Vec<[FaceKey; 2]> = errs
        .iter()
        .map(|e| match e {
            ValidationError::UndeclaredCusp { edge, wedge: w } if *w == wedge => {
                edge_faces(body, *edge)
            }
            other => panic!("only the undeclared {wedge:?} may refuse: {other:?}"),
        })
        .collect();
    refused.sort();
    assert_eq!(refused.len(), cusps, "{errs:?}");
    assert_eq!(
        carried_pairs(contacts),
        refused,
        "the verb carries exactly the refused edges' wall pairs"
    );
    assert_eq!(
        topo::validate_geometric_declared(body, contacts, tol),
        Ok(()),
        "with the carried declarations the body is tier-3 valid"
    );
}

#[test]
fn extrude_carries_the_outer_cusp_either_way_it_extrudes() {
    let profile = validated(vec![lune()]);
    for d in [1.0, -1.0] {
        let built = extrude(&profile, Extrusion::Distance(d), Tol::witness()).unwrap();
        carries_exactly_its_cusps(
            &built.body,
            &built.declared_contacts,
            MaterialWedge::Cusp,
            1,
        );
    }
}

#[test]
fn extrude_carries_a_hole_cusp_as_the_slit_it_sweeps() {
    let plate = bulge_loop(vec![
        (p2(-1.0, -1.0), 0.0),
        (p2(3.0, -1.0), 0.0),
        (p2(3.0, 5.0), 0.0),
        (p2(-1.0, 5.0), 0.0),
    ]);
    let profile = validated(vec![plate, lune()]);
    let built = extrude(&profile, Extrusion::Distance(1.0), Tol::witness()).unwrap();
    carries_exactly_its_cusps(
        &built.body,
        &built.declared_contacts,
        MaterialWedge::Slit,
        1,
    );
}

/// A declared SMOOTH joint is wedge π, legal undeclared: it carries
/// nothing, so the record is the cusps and not every declared joint.
#[test]
fn extrude_carries_nothing_for_declared_smooth_joints() {
    let q = 0.25;
    let b = core::f64::consts::FRAC_PI_8.tan();
    let lp = bulge_loop(vec![
        (p2(q, 0.0), 0.0),
        (p2(1.0 - q, 0.0), b),
        (p2(1.0, q), 0.0),
        (p2(1.0, 1.0 - q), b),
        (p2(1.0 - q, 1.0), 0.0),
        (p2(q, 1.0), b),
        (p2(0.0, 1.0 - q), 0.0),
        (p2(0.0, q), b),
    ])
    .with_tangent_joints(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    let built = extrude(
        &validated(vec![lp]),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap();
    assert_eq!(built.declared_contacts, []);
    assert_eq!(
        topo::validate_geometric(&built.body, Tol::witness()),
        Ok(())
    );
}

#[test]
fn revolve_carries_the_cusp_rim_partial_and_full() {
    let profile = validated(vec![sphere_cone_crescent()]);
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    for revolution in [Revolution::Partial(1.0), Revolution::Full] {
        let built = revolve(&profile, axis, revolution, Tol::witness()).unwrap();
        carries_exactly_its_cusps(
            &built.body,
            &built.declared_contacts,
            MaterialWedge::Cusp,
            1,
        );
    }
}

/// The full revolve of an axis-touching profile sweeps two π-bands, so
/// its one cusp joint is two cusp rims — one per band — and both
/// wall pairs are carried.
#[test]
fn revolve_wire_case_carries_the_cusp_on_both_bands() {
    let profile = validated(vec![sphere_cone_on_axis()]);
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let built = revolve(&profile, axis, Revolution::Full, Tol::witness()).unwrap();
    carries_exactly_its_cusps(
        &built.body,
        &built.declared_contacts,
        MaterialWedge::Cusp,
        2,
    );
}

/// Loft walls are NURBS, whose edges tier 3's material arm exempts by
/// kind, so the body validates either way; the record is still the
/// sections' declaration, on the seam at the cusp joint.
#[test]
fn loft_carries_the_cusp_seam() {
    let places: Vec<Affine3<f64>> = [0.0, 1.0]
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    let built = loft_body::<f64>(&[vec![lune()], vec![lune()]], &places, 1, Tol::witness())
        .expect("the lune lofts");
    assert_eq!(
        carried_pairs(&built.declared_contacts),
        [edge_faces(&built.body, built.seam_edges[0][2])],
        "the carried pair is the seam at the lune's cusp joint"
    );
    assert_eq!(
        topo::validate_geometric_declared(&built.body, &built.declared_contacts, Tol::witness()),
        Ok(())
    );
}

/// The lune authored RAW (bulges + `with_tangent_joints`), not through
/// the `.cusp()` door: the joint carries the same record.
fn raw_lune() -> ProfileLoop<f64> {
    bulge_loop(vec![
        (p2(0.0, 4.0), 0.0),
        (p2(0.0, 2.0), -1.0),
        (p2(0.0, 0.0), 1.0),
    ])
    .with_tangent_joints(vec![2])
}

fn filleted_block(x0: f64) -> ProfileLoop<f64> {
    let q = 0.25;
    let b = core::f64::consts::FRAC_PI_8.tan();
    bulge_loop(vec![
        (p2(x0 + q, 0.0), 0.0),
        (p2(x0 + 1.0 - q, 0.0), b),
        (p2(x0 + 1.0, q), 0.0),
        (p2(x0 + 1.0, 1.0 - q), b),
        (p2(x0 + 1.0 - q, 1.0), 0.0),
        (p2(x0 + q, 1.0), b),
        (p2(x0, 1.0 - q), 0.0),
        (p2(x0, q), b),
    ])
    .with_tangent_joints(vec![0, 1, 2, 3, 4, 5, 6, 7])
}

#[test]
fn a_raw_authored_cusp_carries_like_the_door() {
    let profile = validated(vec![raw_lune()]);
    assert_eq!(profile.loops()[0].tangent_joints(), &[2]);
    for d in [1.0, -1.0] {
        let built = extrude(&profile, Extrusion::Distance(d), Tol::witness()).unwrap();
        carries_exactly_its_cusps(
            &built.body,
            &built.declared_contacts,
            MaterialWedge::Cusp,
            1,
        );
    }
}

#[test]
fn a_hole_cusp_carries_its_slit_at_either_sign_and_either_winding() {
    let plate = bulge_loop(vec![
        (p2(-1.0, -1.0), 0.0),
        (p2(3.0, -1.0), 0.0),
        (p2(3.0, 5.0), 0.0),
        (p2(-1.0, 5.0), 0.0),
    ]);
    // Hole authored the "right" (CW) way AND the wrong way.
    for hole in [lune(), raw_lune()] {
        let profile = validated(vec![plate.clone(), hole]);
        for d in [1.0, -1.0] {
            let built = extrude(&profile, Extrusion::Distance(d), Tol::witness()).unwrap();
            carries_exactly_its_cusps(
                &built.body,
                &built.declared_contacts,
                MaterialWedge::Slit,
                1,
            );
        }
    }
}

#[test]
fn revolve_and_loft_carry_nothing_for_smooth_joints() {
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let profile = validated(vec![filleted_block(2.0)]);
    for revolution in [Revolution::Partial(1.0), Revolution::Full] {
        let built = revolve(&profile, axis, revolution, Tol::witness()).unwrap();
        assert_eq!(built.declared_contacts, []);
        assert_eq!(
            topo::validate_geometric(&built.body, Tol::witness()),
            Ok(())
        );
    }
    let places: Vec<Affine3<f64>> = [0.0, 1.0]
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    let built = loft_body::<f64>(
        &[vec![filleted_block(0.0)], vec![filleted_block(0.0)]],
        &places,
        1,
        Tol::witness(),
    )
    .expect("the block lofts");
    assert_eq!(built.declared_contacts, []);
}

fn crescent_raw(far: Point2<f64>, declared: bool) -> ProfileLoop<f64> {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let lp = bulge_loop(vec![
        (p2(1.0, 0.0), (std::f64::consts::PI / 16.0).tan()),
        (p2(h, h), 0.0),
        (far, 0.0),
    ]);
    if declared {
        lp.with_tangent_joints(vec![1])
    } else {
        lp
    }
}

/// Sections that DISAGREE at one joint: a cusp in one, a corner in the
/// other, same arc (so the loft's pcurve lane admits it).
#[test]
fn a_loft_whose_sections_disagree_carries_the_cusp_seam() {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let a = crescent_raw(p2(2.0 * h, 0.0), true);
    let b = crescent_raw(p2(1.5, 0.2), false);
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
        // The union rule: one section's cusp carries the seam pair.
        assert_eq!(
            carried_pairs(&built.declared_contacts),
            [edge_faces(&built.body, built.seam_edges[0][j])],
        );
        // NURBS walls: tier 3 exempts the seam either way today.
        assert_eq!(
            topo::validate_geometric(&built.body, Tol::witness()),
            Ok(())
        );
        assert_eq!(
            topo::validate_geometric_declared(
                &built.body,
                &built.declared_contacts,
                Tol::witness()
            ),
            Ok(())
        );
    }
}
