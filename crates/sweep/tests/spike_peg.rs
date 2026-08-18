//! SPIKE (M9-3 substrate, throwaway — never commit): does a
//! peg-in-bore (cylindrical declared-Rest, full engagement) reach the
//! rest lane's segment discovery today, or does reduction refuse
//! upstream?

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tolerance, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanDeclarations, ContactClass, FacePairDeclaration};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The plate: 4×4×1, z ∈ [0, 1].
fn plate() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(0.0, 0.0),
            bulge: 0.0,
        },
        ProfileVertex {
            pos: p2(4.0, 0.0),
            bulge: 0.0,
        },
        ProfileVertex {
            pos: p2(4.0, 4.0),
            bulge: 0.0,
        },
        ProfileVertex {
            pos: p2(0.0, 4.0),
            bulge: 0.0,
        },
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0)).unwrap().body
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
        ProfileVertex {
            pos: at(0.0),
            bulge: b120,
        },
        ProfileVertex {
            pos: at(120.0),
            bulge: b120,
        },
        ProfileVertex {
            pos: at(240.0),
            bulge: b120,
        },
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&profile, Extrusion::Distance(h)).unwrap().body
}

/// The cylinder-surface faces of a body.
fn cyl_faces(body: &Body<f64>) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom_surfaces::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect()
}

#[test]
fn spike_peg_in_bore_reachability() {
    // Bored plate: through-hole subtract (transverse — works today).
    let bored = topo::subtract(&plate(), &cyl(-0.2, 1.4))
        .expect("through-hole subtract is the shipped transverse lane")
        .body()
        .expect("seamed result")
        .body
        .clone();
    // Peg: exactly fills the bore (full engagement), z ∈ [0, 1].
    let peg = cyl(0.0, 1.0);

    let bore_walls = cyl_faces(&bored);
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

    match topo::union_with(&bored, &peg, &decls) {
        Ok(out) => {
            let vol = topo::mass_properties(&out.body().unwrap().body)
                .unwrap()
                .volume;
            eprintln!("SPIKE: union SUCCEEDED, vol = {vol}");
        }
        Err(e) => {
            eprintln!("SPIKE: union refused: {e:?}");
            eprintln!("SPIKE: display: {e}");
        }
    }
    panic!("spike complete (always red — read the eprintln transcript)");
}
