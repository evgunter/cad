//! Reviewer probe (reach-dual3977-r1): the far-anchored slab family,
//! upright and inverted (`Body::revert`), swept over anchor distance,
//! anchor direction, slab size and scale. Prints one line per body:
//! exact volume (closed form, a box), the f64 sum, check 7, check 10's
//! role read and classify_shells. Run identically on main and on head.
//! Lives in crates/topo/src as `#[cfg(test)] mod probe_r1_slab;`.
use crate::Body;
use crate::validate::validate_geometric;
use geom_core::{Band, Tol};
use geom::Surface;

fn slab(dx: f64, dy: f64, dz: f64, d: f64, dir: (f64, f64), tol: Tol) -> Body<f64> {
    let mut body = crate::test_support::brick::<f64>((0.0, dx), (0.0, dy), (0.0, dz), tol);
    let surfaces: Vec<_> = body.faces().map(|(_, f)| f.surface).collect();
    for key in surfaces {
        let surface = body.surfaces.get_mut(key).unwrap();
        let Surface::Plane { origin, normal, u_ref } = *surface else { panic!() };
        let along = (u_ref * dir.0 + normal.cross(u_ref) * dir.1) * d;
        *surface = Surface::Plane { origin: origin + along, normal, u_ref };
    }
    body
}

fn report(tag: &str, body: &Body<f64>, exact: f64, tol: Tol) {
    let mp = crate::mass_properties(body, tol).map(|p| p.volume);
    let v7 = validate_geometric(body, tol);
    let solid = body.solids().next().unwrap().0;
    let faces = body.faces_of_solid(solid).unwrap();
    let band = Band::linear(tol).unwrap();
    let role = crate::validate::shell_role(body, &faces, band, tol, Some(crate::props::QuadLane::certified()));
    let cls = crate::classify_shells(body, tol).map(|v| v.iter().map(|c| format!("{:?}", c.role)).collect::<Vec<_>>());
    let cls = match cls { Ok(v) => format!("Ok{v:?}"), Err(e) => format!("Err({})", format!("{e:?}").split(|c| c == ' ' || c == '{' || c == '(').next().unwrap()) };
    let v7s = match &v7 { Ok(()) => "Ok".to_string(), Err(e) => format!("{:?}", e).chars().take(60).collect() };
    println!("PROBE|{tag}|exact={exact:+.3e}|f64={:+.3e}|check7={v7s}|role={role:?}|classify={cls}", mp.unwrap_or(f64::NAN));
}

#[test]
fn probe_r1_slab_family() {
    let tol = Tol::witness();
    println!("PROBE|eps={}", tol.eps());
    for &(dx, dy, dz) in &[(1e-3, 1e-3, 1e-7), (1e-3, 1e-3, 1e-6), (1e-2, 1e-2, 1e-7), (1.0, 1.0, 1e-4)] {
        for &d in &[0.0, 1.0, 10.0, 100.0, 300.0, 1e3, 3e3, 5.3e3, 1e4] {
            for &dir in &[(1.0, 0.37), (-0.6, 0.8), (0.31, -0.95)] {
                let b = slab(dx, dy, dz, d, dir, tol);
                let exact = dx * dy * dz;
                let tag = format!("{dx:e}x{dy:e}x{dz:e}|d={d:e}|dir={dir:?}");
                report(&format!("{tag}|upright"), &b, exact, tol);
                match b.revert() {
                    Ok(inv) => report(&format!("{tag}|INVERTED"), &inv, -exact, tol),
                    Err(e) => println!("PROBE|{tag}|INVERTED|revert refused {e:?}"),
                }
            }
        }
    }
}

/// Tiny cubes (edge `s`) placed `d` metres from the origin on a skew
/// axis, upright and inverted — the contact9-sized poses.
#[test]
fn probe_r1_tiny_cubes() {
    let tol = Tol::witness();
    println!("PROBE|eps={}", tol.eps());
    for &s in &[1e-5, 1e-6, 3e-7] {
        if s < 50.0 * tol.eps() { continue; }
        for &d in &[0.0, 1.0, 3.0, 10.0, 30.0, 100.0] {
            let (ox, oy, oz) = (d * 0.6, d * 0.48, d * 0.64);
            let b = crate::test_support::brick::<f64>((ox, ox + s), (oy, oy + s), (oz, oz + s), tol);
            let tag = format!("cube s={s:e}|d={d:e}");
            report(&format!("{tag}|upright"), &b, s * s * s, tol);
            report(&format!("{tag}|INVERTED"), &b.revert().unwrap(), -s * s * s, tol);
        }
    }
}

/// Check 10: a 1 m cube with a far-anchored tiny slab grafted beside it
/// under the SAME solid — upright (a second Outer: two pieces) and
/// inverted (a Void outside every Outer). Both are invalid by
/// construction; the per-solid total is positive so check 7 passes.
#[test]
fn probe_r1_check10_tiny_second_shell() {
    let tol = Tol::witness();
    println!("PROBE|eps={}", tol.eps());
    for &d in &[0.0, 1e3, 3e3] {
        for inverted in [false, true] {
            let mut body = crate::test_support::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
            let mut s = slab(1e-3, 1e-3, 1e-7, d, (1.0, 0.37), tol);
            // move the slab 3 m along x by re-building it there
            s = {
                let mut b = crate::test_support::brick::<f64>((3.0, 3.001), (0.0, 1e-3), (0.0, 1e-7), tol);
                let surfaces: Vec<_> = b.faces().map(|(_, f)| f.surface).collect();
                for key in surfaces {
                    let surface = b.surfaces.get_mut(key).unwrap();
                    let Surface::Plane { origin, normal, u_ref } = *surface else { panic!() };
                    *surface = Surface::Plane { origin: origin + (u_ref * 1.0 + normal.cross(u_ref) * 0.37) * d, normal, u_ref };
                }
                let _ = &mut s;
                b
            };
            if inverted { s = s.revert().unwrap(); }
            crate::graft_disjoint_all_keyed(&mut body, &s).expect("graft");
            let body = body.with_solids_merged_for_tests();
            let v = validate_geometric(&body, tol);
            println!("PROBE|check10|d={d:e}|tiny shell {}|solids={}|tier3={:?}", if inverted { "INVERTED (void outside)" } else { "upright (2nd outer)" }, body.solids().count(), v);
        }
    }
}
