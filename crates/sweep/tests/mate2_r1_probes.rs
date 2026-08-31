//! MATE-2 R1 adversarial probes (issue 1032 / PR 1417).
//!
//! The unit's fix reads a CERTIFIED `Out` from the cylinder chart trim
//! as "eventless at THIS face", on the argument that the endpoint is a
//! seam site whose incidence the NEIGHBOURING face on the same carrier
//! records. These probes ask what happens when that argument's premise
//! does not hold.
//!
//! The unit's own fixture holds both bodies' azimuth splits ALIGNED
//! (`three_arc(_, 0.0)` on both), so every arc meets a face's window at
//! an endpoint and never crosses one in its interior. Offsetting one
//! body's split breaks exactly that.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use topo::{
    Body, BooleanDeclarations, BooleanResult, ContactClass, FacePairDeclaration, mass_properties,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn three_arc(radius: f64, deg0: f64) -> ProfileLoop<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th: f64 = deg.to_radians();
        p2(radius * th.cos(), radius * th.sin())
    };
    ProfileLoop::new(vec![
        ProfileVertex::new(at(deg0), b120),
        ProfileVertex::new(at(deg0 + 120.0), b120),
        ProfileVertex::new(at(deg0 + 240.0), b120),
    ])
}

fn extruded(loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> Body<f64> {
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, loops).validate(Tol::witness()).unwrap();
    sweep::extrude(&profile, sweep::Extrusion::Distance(h), Tol::witness())
        .unwrap()
        .body
}

/// The unit's own collar: annulus (outer 1.5, bore 0.5), z in [1, 2],
/// bore split at 0/120/240.
fn collar() -> Body<f64> {
    extruded(vec![three_arc(1.5, 0.0), three_arc(0.5, 0.0)], 1.0, 1.0)
}

/// A peg of radius 0.5 whose three-arc split starts at `deg0`.
fn peg_at(z0: f64, h: f64, deg0: f64) -> Body<f64> {
    extruded(vec![three_arc(0.5, deg0)], z0, h)
}

fn walls_at(body: &Body<f64>, r: f64) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { radius, .. }) if (radius - r).abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .collect()
}

fn wall_decls(a: &Body<f64>, b: &Body<f64>) -> BooleanDeclarations {
    let mut decls = BooleanDeclarations::none();
    for &fa in &walls_at(a, 0.5) {
        for &fb in &walls_at(b, 0.5) {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    decls
}

fn volume(b: &Body<f64>) -> f64 {
    mass_properties(b, Tol::witness()).unwrap().volume
}

/// PROBE 1 — the azimuth splits MISALIGNED by 60 degrees.
///
/// Same mate as the unit's `threaded_collar_partial_engagement_unions`
/// in every respect except that the peg's three-arc split starts at 60
/// degrees instead of 0, so the two bodies' seams interleave. Every
/// bore rim arc now CROSSES a peg wall face's seam in its interior
/// rather than meeting it at an endpoint: for the pair (bore rim arc
/// at z = 2 spanning [0,120], peg wall face spanning [60,180]) one
/// endpoint is `In` (recorded) and the other is a certified `Out`, so
/// the widened rung returns `Recorded` — while the crossing at
/// theta = 60, z = 2, where the rim arc meets the peg's meridian seam
/// edge, is recorded by NOBODY.
///
/// This test does not assert an outcome. It reports one, so the two
/// trees can be compared.
#[test]
fn probe_misaligned_azimuth_split_reports_its_outcome() {
    let c = collar();
    let p = peg_at(0.5, 2.0, 60.0);
    let decls = wall_decls(&c, &p);
    assert_eq!(decls.coincident_faces.len(), 9, "3 bore faces against 3");
    let out = topo::union_with(&c, &p, &decls, Tol::witness());
    match out {
        Err(e) => println!("PROBE1 REFUSED: {e:?}"),
        Ok(BooleanResult::Empty) => println!("PROBE1 EMPTY"),
        Ok(BooleanResult::Body(bb)) => {
            let body = bb.body;
            let (v, vc, vp) = (volume(&body), volume(&c), volume(&p));
            println!(
                "PROBE1 UNIONED: volume {v} vs {vc} + {vp} = {} (err {:e})",
                vc + vp,
                (v - (vc + vp)).abs()
            );
            println!("PROBE1 shells {}", body.shells().count());
            println!(
                "PROBE1 tier3 {:?}",
                topo::validate_geometric(&body, Tol::witness()).err()
            );
            println!(
                "PROBE1 pseudomanifold {:?}",
                topo::validate_pseudomanifold(&body, &bb.contacts, Tol::witness()).err()
            );
        }
    }
}

/// PROBE 2 — an endpoint certified `Out` by the chart's HEIGHT window.
///
/// The peg's bottom cap floats INSIDE the bore (z in [1.5, 2.5], bore z
/// in [1, 2]), so each peg meridian seam edge has its lower endpoint
/// inside a bore face's height window (recorded) and its upper endpoint
/// certified `Out` ABOVE it. `Out`-by-height is not the seam shape the
/// fix's comment argues about: no other face on the shared carrier
/// holds a point above the carrier's own faces.
#[test]
fn probe_out_by_height_reports_its_outcome() {
    let c = collar();
    let p = peg_at(1.5, 1.0, 0.0);
    let decls = wall_decls(&c, &p);
    let out = topo::union_with(&c, &p, &decls, Tol::witness());
    match out {
        Err(e) => println!("PROBE2 REFUSED: {e:?}"),
        Ok(BooleanResult::Empty) => println!("PROBE2 EMPTY"),
        Ok(BooleanResult::Body(bb)) => {
            let body = bb.body;
            let (v, vc, vp) = (volume(&body), volume(&c), volume(&p));
            println!(
                "PROBE2 UNIONED: volume {v} vs {vc} + {vp} = {} (err {:e})",
                vc + vp,
                (v - (vc + vp)).abs()
            );
            println!("PROBE2 shells {}", body.shells().count());
            println!(
                "PROBE2 tier3 {:?}",
                topo::validate_geometric(&body, Tol::witness()).err()
            );
            println!(
                "PROBE2 pseudomanifold {:?}",
                topo::validate_pseudomanifold(&body, &bb.contacts, Tol::witness()).err()
            );
        }
    }
}

/// PROBE 3 — the UNDECLARED control for probe 1's geometry.
///
/// Same misaligned mate with NO declarations at all. The fix claims the
/// undeclared arms are byte-identical in behaviour; this row is run on
/// both trees to check that.
#[test]
fn probe_undeclared_misaligned_reports_its_outcome() {
    let c = collar();
    let p = peg_at(0.5, 2.0, 60.0);
    let out = topo::union_with(&c, &p, &BooleanDeclarations::none(), Tol::witness());
    match out {
        Err(e) => println!("PROBE3 REFUSED: {e:?}"),
        Ok(BooleanResult::Empty) => println!("PROBE3 EMPTY"),
        Ok(BooleanResult::Body(bb)) => {
            println!("PROBE3 UNIONED: volume {}", volume(&bb.body));
        }
    }
}

/// PROBE 4 — the UNDECLARED control for the unit's own aligned mate.
#[test]
fn probe_undeclared_aligned_reports_its_outcome() {
    let c = collar();
    let p = peg_at(0.5, 2.0, 0.0);
    let out = topo::union_with(&c, &p, &BooleanDeclarations::none(), Tol::witness());
    match out {
        Err(e) => println!("PROBE4 REFUSED: {e:?}"),
        Ok(BooleanResult::Empty) => println!("PROBE4 EMPTY"),
        Ok(BooleanResult::Body(bb)) => {
            println!("PROBE4 UNIONED: volume {}", volume(&bb.body));
        }
    }
}
