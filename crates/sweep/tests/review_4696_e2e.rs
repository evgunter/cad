//! Review e2e for PR 4696: a frustum whose cone wall is notched by a
//! partial-revolve sector (rims + generators), measured end to end.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::Tol;
use sweep::Revolution;
use sweep::test_support::{corners, finished, revolved_about_y};

fn tol() -> Tol {
    Tol::witness()
}

fn exact(theta: f64, y0: f64, r_cut: f64) -> f64 {
    // frustum: r(y) = 1 - 0.5 y on y in [0, 0.8], bore 0.2
    let h = 0.8;
    let (r1, r2) = (1.0, 0.6);
    let whole = PI * h / 3.0 * (r1 * r1 + r1 * r2 + r2 * r2) - PI * 0.04 * h;
    // removed: theta * ∫_{y0}^{0.8} (r(y)^2 - r_cut^2)/2 dy
    let n = 20000;
    let mut s = 0.0;
    for i in 0..n {
        let y = y0 + (0.8 - y0) * (i as f64 + 0.5) / n as f64;
        let r = 1.0 - 0.5 * y;
        s += (r * r - r_cut * r_cut) / 2.0;
    }
    whole - theta * s * (0.8 - y0) / n as f64
}

#[test]
fn notched_frustum_measures() {
    for &(theta, y0, r_cut) in &[(1.0, 0.6, 0.5), (2.5, 0.3, 0.45), (0.4, 0.7, 0.3)] {
        let frustum = finished("frustum", revolved_about_y(
            corners(&[(0.2, 0.0), (1.0, 0.0), (0.6, 0.8), (0.2, 0.8)]),
            Revolution::Full,
            tol(),
        ), tol());
        let cutter = finished("cutter", revolved_about_y(
            corners(&[(r_cut, y0), (1.5, y0), (1.5, 1.2), (r_cut, 1.2)]),
            Revolution::Partial(theta),
            tol(),
        ), tol());
        let whole = topo::mass_properties(&frustum, tol()).unwrap();
        let out = topo::boolean::subtract(&frustum, &cutter, tol()).unwrap_or_else(|e| panic!("subtract: {e:?}"));
        let body = out.body().expect("non-empty").body.clone();
        let got = topo::mass_properties(&body, tol());
        let want = exact(theta, y0, r_cut);
        eprintln!("theta {theta} y0 {y0} r_cut {r_cut}: whole {} (exact frustum {}) notched {got:?} want {want}", whole.volume, exact(0.0, 0.8, 0.0));
        let got = got.unwrap();
        assert!((got.volume - want).abs() < 1e-9 + got.volume_pad, "volume {} vs {want}", got.volume);
    }
}
