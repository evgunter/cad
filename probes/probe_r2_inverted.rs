//! Reviewer probe (reach-dual3977-r2): inside-out far-anchored slabs.
//! Mounted temporarily as `crates/topo/src/probe_r2_inverted.rs` with
//! `#[cfg(test)] mod probe_r2_inverted;` in lib.rs. Run on main and head.
use crate::validate::validate_geometric;
use crate::Body;
use geom::Surface;
use geom_core::{Point3, Tol};

fn slab(t: f64, w: f64, inside_out: bool, far: f64, tol: Tol) -> Body<f64> {
    let mut body = Body::<f64>::new();
    crate::test_support_fixtures::cube_into(
        &mut body,
        move |x, y, z| {
            let x = if inside_out { 1.0 - x } else { x };
            Point3::new(x * w, y * w, z * t)
        },
        tol,
    );
    let surfaces: Vec<_> = body.faces().map(|(_, f)| f.surface).collect();
    for key in surfaces {
        let s = body.surfaces.get_mut(key).unwrap();
        let Surface::Plane { origin, normal, u_ref } = *s else { panic!() };
        let along = u_ref * far + normal.cross(u_ref) * (0.37 * far);
        *s = Surface::Plane { origin: origin + along, normal, u_ref };
    }
    body
}

#[test]
fn probe_inverted_far_slabs() {
    let tol = Tol::witness();
    let mut newly = 0;
    for &(t, w) in &[(1e-7, 1e-3), (1e-6, 1e-3), (1e-5, 1e-3), (1e-4, 1e-2), (1e-3, 1e-1)] {
        for &far in &[10.0, 1e2, 1e3, 5e3, 2e4, 1e5] {
            for &inv in &[false, true] {
                let b = slab(t, w, inv, far, tol);
                let exact = if inv { -t * w * w } else { t * w * w };
                let v = crate::mass_properties(&b, tol).map(|p| p.volume);
                let g = validate_geometric(&b, tol);
                let wrong = (inv && g.is_ok()) || (!inv && g.is_err());
                if wrong { newly += 1; }
                println!(
                    "PROBE eps={:e} t={t:e} w={w:e} far={far:e} inv={inv} exact={exact:e} f64={v:?} tier3={g:?}{}",
                    tol.eps(),
                    if wrong { "  <-- WRONG VERDICT" } else { "" }
                );
            }
        }
    }
    println!("PROBE wrong verdicts: {newly}");
}
