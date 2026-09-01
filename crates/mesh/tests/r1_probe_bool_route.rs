//! R1 review probe (MESH-3, PR 1460): the no-route verdict enumerates
//! the import, revolve and profile doors — but the tree also has a
//! BOOLEAN door ((Plane, Sphere) opened at S13), which can in
//! principle mint a sphere face whose rim sits within eps of the
//! chart pole: subtract a flat-topped cutter whose plane lies
//! ~1 ulp below the pole of a macroscopic ball. This probe measures
//! whether that door is shut, and by which bar.
//!
//! NOT part of the unit under review; lives only on the review probes
//! branch.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use geom_core::Tol;
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, revolve};

fn p2(x: f64, y: f64) -> geom_core::Point2<f64> {
    geom_core::Point2::new(x, y)
}

fn validated(loops: Vec<ProfileLoop<f64>>) -> Result<profile::ValidatedProfile<f64>, String> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .map_err(|e| format!("profile validation: {e:?}"))
}

fn axis_y() -> sweep::RevolveAxis<f64> {
    sweep::RevolveAxis {
        origin: geom_core::Point2::new(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    }
}

/// Ball of radius r about the origin (two half-bands on one sphere).
fn ball(r: f64) -> Result<topo::Body<f64>, String> {
    let bulge = 1.0; // semicircle
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -r), bulge),
        ProfileVertex::new(p2(0.0, r), 0.0),
    ]);
    revolve(
        &validated(vec![lp])?,
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .map(|r| r.body)
    .map_err(|e| format!("revolve: {e:?}"))
}

/// Slab covering x,y up to y = d (top wall a y-normal plane), z in
/// [-L, L]: an EXTRUDED prism, the pips suite's operand shape, so it
/// clears the maximal-face gate a full-revolve cylinder's split caps
/// do not.
fn slab(d: f64, l: f64) -> Result<topo::Body<f64>, sweep::ExtrudeError> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(-l, -l), 0.0),
        ProfileVertex::new(p2(l, -l), 0.0),
        ProfileVertex::new(p2(l, d), 0.0),
        ProfileVertex::new(p2(-l, d), 0.0),
    ]);
    let plane = SketchPlane::new(geom_core::Affine3::translation(geom_core::Vec3::new(
        0.0, 0.0, -l,
    )));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    sweep::extrude(
        &profile,
        sweep::Extrusion::Distance(2.0 * l),
        Tol::witness(),
    )
    .map(|r| r.body)
}

fn attempt(r: f64, gap: f64) {
    let d = r - gap;
    let rho = (r * r - d * d).max(0.0).sqrt();
    println!(
        "--- r = {r:e}, r - d = {:e} (requested gap {gap:e}), rim rho = {rho:e}",
        r - d
    );
    if r - d == 0.0 {
        println!("    gap not representable at this r: exact tangency, skip");
        return;
    }
    let a = match ball(r) {
        Ok(b) => b,
        Err(e) => {
            println!("    ball({r:e}) REFUSED at revolve: {e:?}");
            return;
        }
    };
    let b = match slab(d, 3.0 * r) {
        Ok(b) => b,
        Err(e) => {
            println!("    slab REFUSED at extrude: {e:?}");
            return;
        }
    };
    match topo::boolean::subtract(&b, &a, Tol::witness()) {
        Err(e) => println!("    boolean subtract REFUSED: {e:?}"),
        Ok(res) => {
            let Some(body) = res.body() else {
                println!("    boolean Ok but empty/void result: {:?}", res);
                return;
            };
            println!("    boolean ADMITTED a body: the door is open this far");
            match mesh::tessellate(&body.body, r / 10.0, Tol::witness()) {
                Ok(m) => println!(
                    "    tessellated OK, guard quiet: {} triangles",
                    m.patches.iter().map(|p| p.triangles.len()).sum::<usize>()
                ),
                Err(e) => println!("    tessellate refused typed: {e:?}"),
            }
        }
    }
}

/// Run with --nocapture; panics (the guard firing) are the loud outcome.
#[test]
fn r1_probe_boolean_near_tangent_pole_cut() {
    let eps = Tol::witness().get().eps;
    println!("ambient eps = {eps:e}");
    // Macroscopic ball, 1-ulp and few-ulp gaps: rim rho ~ sqrt(2 r gap).
    for r in [0.05f64, 0.1, 1.0] {
        let ulp = f64::from_bits(r.to_bits() + 1) - r;
        attempt(r, ulp);
        attempt(r, 4.0 * ulp);
    }
    // Small balls where the gap is comfortably representable.
    attempt(1e-5, 5e-14); // rho ~ 1e-9.5
    attempt(1e-7, 2e-12); // rho ~ 6.3e-10
    attempt(1e-7, 5e-12); // rho ~ 1e-9
}
