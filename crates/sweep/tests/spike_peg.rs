//! SPIKE (M9-3 substrate, throwaway — never merge): does a
//! peg-in-bore (cylindrical declared-Rest, full engagement) reach the
//! rest lane's segment discovery today, or does reduction refuse
//! upstream?

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanDeclarations, BooleanResult, ContactClass, FacePairDeclaration};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The plate: 4×4×1, z ∈ [0, 1].
fn plate() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(4.0, 0.0), p2(4.0, 4.0), p2(0.0, 4.0)]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

/// A radius-0.5 three-arc cylinder at (2, 2), z ∈ [z0, z0 + h]
/// (boss_union's authorship).
fn cyl(z0: f64, h: f64) -> Body<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th = deg.to_radians();
        p2(2.0 + 0.5 * th.cos(), 2.0 + 0.5 * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(0.0), b120),
        ProfileVertex::new(at(120.0), b120),
        ProfileVertex::new(at(240.0), b120),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .unwrap()
        .body
}

/// The cylinder-surface faces of a body.
fn cyl_faces(body: &Body<f64>) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Cylinder { .. })))
        .map(|(k, _)| k)
        .collect()
}

#[test]
fn spike_peg_in_bore_reachability() {
    // Bored plate: through-hole subtract (transverse — works today).
    let bored = match topo::subtract(&plate(), &cyl(-0.2, 1.4), Tol::witness())
        .expect("through-hole subtract is the shipped transverse lane")
    {
        BooleanResult::Body(b) => b,
        BooleanResult::Empty => panic!("bored plate cannot be empty"),
    };
    // Peg: exactly fills the bore (full engagement), z ∈ [0, 1].
    let peg = cyl(0.0, 1.0);

    let bore_walls = cyl_faces(&bored.body);
    let peg_walls = cyl_faces(&peg);
    eprintln!("SPIKE: bore walls = {bore_walls:?}, peg walls = {peg_walls:?}");
    assert!(!bore_walls.is_empty() && !peg_walls.is_empty());

    // Declare every bore-wall × peg-wall pair Rest (one shared
    // carrier; senses opposed).
    let mut decls = BooleanDeclarations::none();
    for &a in &bore_walls {
        for &b in &peg_walls {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(a, b, ContactClass::Rest));
        }
    }

    match topo::union_with(&bored.body, &peg, &decls, Tol::witness()) {
        Ok(BooleanResult::Body(out)) => {
            let vol = topo::mass_properties(&out.body, Tol::witness())
                .unwrap()
                .volume;
            eprintln!("SPIKE: union SUCCEEDED (kind {:?}), vol = {vol}", out.kind);
        }
        Ok(BooleanResult::Empty) => eprintln!("SPIKE: union came back EMPTY"),
        Err(e) => {
            eprintln!("SPIKE: union refused: {e:?}");
            eprintln!("SPIKE: display: {e}");
        }
    }
    panic!("spike complete (always red — read the eprintln transcript)");
}

/// Second probe: do the two-peg fixture's declared pairs CERTIFY
/// through M9-2's shared Door 1 today? The bore walls and the peg
/// walls come from different extrudes (different sketch planes), so
/// their descriptions are not bit-identical — this measures whether
/// the carrier ladder still reaches a verdict.
#[test]
fn spike_contact_pair_verdict_probe() {
    let bored = match topo::subtract(&plate(), &cyl(-0.2, 1.4), Tol::witness())
        .expect("through-hole subtract is the shipped transverse lane")
    {
        BooleanResult::Body(b) => b,
        BooleanResult::Empty => panic!("bored plate cannot be empty"),
    };
    let peg = cyl(0.0, 1.0);
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    for &a in &cyl_faces(&bored.body) {
        for &b in &cyl_faces(&peg) {
            let v = topo::boolean::contact_pair_verdict(
                &bored.body,
                a,
                &peg,
                b,
                ContactClass::Rest,
                None,
                band,
            );
            eprintln!("SPIKE verdict {a:?} x {b:?}: {v:?}");
        }
    }
    panic!("spike probe complete (always red — read the eprintln transcript)");
}
