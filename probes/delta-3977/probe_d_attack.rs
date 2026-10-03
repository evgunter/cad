//! Delta reviewer probe (reach-delta-3977): the new enclosure on bodies
//! placed far from the world origin (translated, and rotated so the
//! planes are oblique and the vertices sit off them by a rounding),
//! upright and inside out. Oracle: the box closed form t·w².
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::validate::validate_geometric;
use crate::Body;
use geom_core::{Affine3, Band, Point3, Tol, Vec3};

fn placed(w: f64, t: f64, angle: f64, at: Vec3<f64>, tol: Tol) -> Option<Body<f64>> {
    let b = crate::test_support::brick::<f64>((0.0, w), (0.0, w), (0.0, t), tol);
    let r = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.3, 0.5, 0.81), angle);
    let b = crate::transform_rigid(&b, &r, tol).ok()?;
    crate::transform_rigid(&b, &Affine3::translation(at), tol).ok()
}

fn line(tag: &str, body: &Body<f64>, exact: f64, tol: Tol) {
    let band = Band::linear(tol).unwrap();
    let mp = crate::mass_properties(body, tol).map(|p| p.volume).unwrap_or(f64::NAN);
    let v7 = validate_geometric(body, tol);
    let (shell, _) = body.shells.iter().next().unwrap();
    let solid = body.solids().next().unwrap().0;
    let faces = body.faces_of_solid(solid).unwrap();
    let role = crate::validate::shell_role(body, shell, &faces, band, tol, Some(crate::props::QuadLane::certified()));
    let wrong = match (&v7, exact > 0.0) {
        (Ok(()), false) => " <-- INSIDE-OUT PASSES",
        (Err(_), true) => " <-- VALID REFUSED",
        _ => "",
    };
    let v7s: String = format!("{v7:?}").chars().take(70).collect();
    let roles: String = format!("{role:?}").chars().take(70).collect();
    println!("DPROBE|{tag}|exact={exact:+.3e}|f64={mp:+.3e}|check7={v7s}|role={roles}{wrong}");
}

#[test]
fn dprobe_far_placed_slabs() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    println!("DPROBE|eps={:e}|escalate={:e}", tol.eps(), band.escalate());
    for &(w, t) in &[(1e-3, 1e-7), (1e-3, 4.0 * band.escalate()), (1e-3, 1e-6), (1e-2, 1e-6)] {
        if t < 1.5 * band.escalate() { continue; }
        for &angle in &[0.0, 0.7] {
            for &d in &[0.0, 1e3, 5e3, 3e4] {
                let at = Vec3::new(d * 0.6, d * 0.48, d * 0.64);
                let Some(b) = placed(w, t, angle, at, tol) else {
                    println!("DPROBE|w={w:e} t={t:e} a={angle} d={d:e}|no build");
                    continue;
                };
                let tag = format!("w={w:e} t={t:e} a={angle} d={d:e}");
                line(&format!("{tag}|upright"), &b, w * w * t, tol);
                match b.revert() {
                    Ok(inv) => line(&format!("{tag}|INVERTED"), &inv, -w * w * t, tol),
                    Err(e) => println!("DPROBE|{tag}|revert {e:?}"),
                }
            }
        }
    }
}
