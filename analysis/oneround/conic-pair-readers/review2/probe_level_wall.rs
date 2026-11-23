//! DELTA-REVIEW (D1): `../review/probe_level.rs` with the anchor stood near the
//! session box's wall and points taken along the arc as far as the box holds
//! them; only points in the box are printed (`|coordinate| <= 1e3`).
//! Review probe (C3): the level `F`, the clearance built on it, the height
//! over the plane and `on_carrier`'s verdict, for points on and beside an
//! arc-held ellipse, printed as JSON for `probe_level.py` (60 digits).
//! Run by dropping into `crates/geom-brep/examples/`:
//! `CAD_TOLERANCE_EPS=<eps> cargo run -p geom-brep --features test-support --example probe_level > level-<eps>.jsonl`
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::disallowed_methods)]
use geom::{Conic, Curve3, Ellipse, OnCarrier};
use geom_core::{Band, Point3, Tol, Vec3};

fn v3(v: Vec3<f64>) -> String {
    format!("[{:?},{:?},{:?}]", v.x, v.y, v.z)
}
fn p3(p: Point3<f64>) -> String {
    v3(p - Point3::origin())
}

fn main() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (eps, esc) = (band.zero(), band.escalate());
    let axis = Vec3::new(0.5522240050794545, 0.3110026418424273, 0.7735153553621528);
    let toward = Vec3::new(0.6816514125372342, -0.702621773621506, -0.20414209516648582);
    let v0 = axis.cross(toward).normalize();
    let anchor = Point3::new(0.3, -0.7, 0.4) - v0 * 950.0;
    for a in [1e2_f64, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9] {
        for (shape, b) in [("fat", 0.3 * a), ("thin", 1.04 * a.sqrt())] {
            for seam in [0] {
                let e0 = Ellipse::anchored(anchor, axis, toward, a, b);
                let e = if seam == 1 { e0.seamed_near(core::f64::consts::FRAC_PI_2) } else { e0 };
                let (sa, sb) = e.semi_axes();
                let theta0 = if seam == 1 { -core::f64::consts::FRAC_PI_2 } else { 0.0 };
                let c = Curve3::Ellipse(e);
                let mut thetas = vec![];
                for s in [0.0, 0.3, 30.0, 300.0, 950.0, 1500.0, 1850.0] {
                    let t: f64 = s / b;
                    if t.abs() < 3.0 { thetas.push(theta0 + t); }
                }
                for theta in thetas {
                    let p = e.point_at(theta);
                    if p.x.abs().max(p.y.abs()).max(p.z.abs()) > 1e3 - 1.0 { continue; }
                    let tan = e.derivative_at(theta).normalize();
                    let n = e.axis();
                    let out = tan.cross(n).normalize();
                    for (dir, d) in [("out", out), ("axis", n)] {
                        for k in [0.0, 0.5, -0.5, 5.0, -5.0, 30.0, -30.0] {
                            if dir == "axis" && k < 0.0 { continue; }
                            let q = p + d * (k * eps);
                            let (f, (gx, gy)) = Conic::level_at(&e, q);
                            let h = Conic::height_of(&e, q);
                            let lc = geom::level_clearance((f, (gx, gy)), sa.min(sb));
                            let cl = c.clearance(q).unwrap();
                            let verdict = match c.on_carrier(q, theta, band) {
                                Ok(OnCarrier::On { .. }) => "on".to_string(),
                                Ok(OnCarrier::Off) => "off".to_string(),
                                Err(_) => "refused".to_string(),
                            };
                            println!(
                                "{{\"zero\":{eps:?},\"esc\":{esc:?},\"a\":{sa:?},\"b\":{sb:?},\"shape\":\"{shape}\",\"seam\":{seam},\"theta\":{theta:?},\"theta0\":{theta0:?},\
                                 \"dir\":\"{dir}\",\"k\":{k:?},\"S\":{},\"axis\":{},\"u\":{},\"q\":{},\"F\":{f:?},\"gx\":{gx:?},\"gy\":{gy:?},\"h\":{h:?},\"lc\":{lc:?},\"cl\":{cl:?},\"verdict\":\"{verdict}\"}}",
                                p3(anchor), v3(e.axis()), v3(e.u_ref()), p3(q)
                            );
                        }
                    }
                }
            }
        }
    }
}
