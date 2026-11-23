//! SCRATCH, branch `oneround/conic-harmonic-probe`: never merges.
//! The conic-harmonic measurement's harness (analysis/oneround/conic-harmonic/).
//! Every test is `#[ignore]`d and writes JSON lines to `$CONIC_PROBE_OUT`.
#![allow(clippy::all, clippy::pedantic, clippy::disallowed_methods, dead_code)]

use std::io::Write;

use geom::{Conic, Curve3, Ellipse, Frame, Surface};
use geom_core::{Band, Point3, Tol, Vec3};

mod m1;
mod m2;

pub(crate) fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

pub(crate) fn out(name: &str) -> std::fs::File {
    let dir = std::env::var("CONIC_PROBE_OUT").expect("set CONIC_PROBE_OUT to a directory");
    let eps = band().zero();
    std::fs::File::create(format!("{dir}/{name}-eps{eps:e}.jsonl")).unwrap()
}

pub(crate) fn v3(v: Vec3<f64>) -> String {
    format!("[{:?},{:?},{:?}]", v.x, v.y, v.z)
}

pub(crate) fn p3(p: Point3<f64>) -> String {
    format!("[{:?},{:?},{:?}]", p.x, p.y, p.z)
}

/// A generic orthonormal triple, so no pose is axis-aligned.
pub(crate) fn triple() -> (Vec3<f64>, Vec3<f64>, Vec3<f64>) {
    let ez = Vec3::new(0.36, 0.48, 0.8);
    let ex = Vec3::new(0.8, -0.6, 0.0);
    (ex, ez.cross(ex), ez)
}

/// The stored numbers of an ellipse, as JSON fields.
pub(crate) fn stored(e: &Ellipse<f64>) -> String {
    let (s, (kc, ks)) = e.probe_stored();
    let (a, b) = e.semi_axes();
    format!(
        "\"S\":{},\"axis\":{},\"u\":{},\"a\":{a:?},\"b\":{b:?},\"phase\":[{kc:?},{ks:?}]",
        p3(s),
        v3(e.axis()),
        v3(e.u_ref())
    )
}

/// The real mint: the section of the cone of half-angle `alpha` about
/// the triple's `ez` with apex `apex`, by the plane of normal tilted
/// `tau` from the axis, `gap` up the axis from the apex.
pub(crate) fn mint(alpha: f64, tau: f64, gap: f64, apex: Point3<f64>) -> Option<Ellipse<f64>> {
    let (ex, ey, ez) = triple();
    let cone = Surface::Cone {
        frame: Frame::new(apex, ez, ex),
        half_angle: alpha,
    };
    let n = ex * tau.sin() + ez * tau.cos();
    let plane = Surface::Plane {
        frame: Frame::new(apex + ez * gap, n, ey),
    };
    let reach = crate::Reach::fixture(apex, 10.0);
    match crate::intersect::plane_cone_section(&plane, &cone, &reach, band()) {
        Ok(crate::intersect::PlaneConeSection::TiltedEllipse(Curve3::Ellipse(e))) => Some(e),
        _ => None,
    }
}

pub(crate) const ALPHA: f64 = 1.3;

/// A minted section with major semi-axis about `target`: `fat` holds
/// the semi-axes' ratio near 0.3 (the apex then stands about a tenth of
/// the major axis off), else the apex gap is 1 m. Its anchor stands
/// near `(1.3, −0.7, 0.4)`.
pub(crate) fn minted(target: f64, fat: bool) -> Option<Ellipse<f64>> {
    let o = Point3::new(0.0, 0.0, 0.0);
    let major = |e: &Ellipse<f64>| e.semi_axes().0.max(e.semi_axes().1);
    let ratio = |e: &Ellipse<f64>| e.semi_axes().0.min(e.semi_axes().1) / major(e);
    let (mut lo, mut hi) = (1e-3, core::f64::consts::FRAC_PI_2 - ALPHA - 1e-12);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        let grow = match mint(ALPHA, mid, 1.0, o) {
            Some(e) if fat => ratio(&e) > 0.3,
            Some(e) => major(&e) < target,
            None => false,
        };
        if grow { lo = mid } else { hi = mid }
    }
    let tau = lo;
    let unit = mint(ALPHA, tau, 1.0, o)?;
    let gap = if fat { target / major(&unit) } else { 1.0 };
    let first = mint(ALPHA, tau, gap, o)?;
    let (anchor, _) = first.probe_stored();
    let want = Point3::new(1.3, -0.7, 0.4);
    mint(ALPHA, tau, gap, o + (want - anchor))
}

#[test]
#[ignore = "probe"]
fn the_mint_ladder() {
    let mut f = out("mint");
    for fat in [true, false] {
        for target in [1e4, 1e5, 1e6, 1e7] {
            match minted(target, fat) {
                Some(e) => writeln!(f, "{{\"fat\":{fat},\"target\":{target:?},{}}}", stored(&e)).unwrap(),
                None => writeln!(f, "{{\"fat\":{fat},\"target\":{target:?},\"refused\":true}}").unwrap(),
            }
        }
    }
}
