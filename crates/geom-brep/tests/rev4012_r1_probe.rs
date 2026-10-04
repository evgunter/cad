//! Review probes (PR 4012, lane r1): declared carriers at rest.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::shared::tol::{band, eps};
use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

/// `z = g(x) + h(y)` over `[x0,x1] × [y0,y1]`, g and h quadratics.
fn wall(
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    (g, dg0): (impl Fn(f64) -> f64, f64),
    (h, dh0): (impl Fn(f64) -> f64, f64),
) -> NurbsSurface<f64> {
    let k = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let gb = [g(x0), g(x0) + 0.5 * (x1 - x0) * dg0, g(x1)];
    let hb = [h(y0), h(y0) + 0.5 * (y1 - y0) * dh0, h(y1)];
    let (xs, ys) = ([x0, 0.5 * (x0 + x1), x1], [y0, 0.5 * (y0 + y1), y1]);
    let control = (0..9)
        .map(|k| Point3::new(xs[k / 3], ys[k % 3], gb[k / 3] + hb[k % 3]))
        .collect();
    NurbsSurface::new(k(), k(), control, vec![1.0; 9]).unwrap()
}

fn plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn declared(
    w: &NurbsSurface<f64>,
    a: Point3<f64>,
    b: Point3<f64>,
) -> Result<geom_brep::PlaneNurbsLimbs<f64>, geom_brep::PlaneNurbsRefusal> {
    let carrier = crate::shared::fixture::segment(a, b);
    geom_brep::plane_nurbs_limbs::<f64>(&carrier, &plane(), w, 1.0, band())
}

/// Distance from (x, y) to the nearest zero of phi over a dense grid on
/// [x0,x1] x [y0,y1] (sign changes along x lines and y lines).
fn nearest_zero(
    phi: &impl Fn(f64, f64) -> f64,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    p: (f64, f64),
) -> f64 {
    let n = 800;
    let at = |i: usize, lo: f64, hi: f64| lo + (hi - lo) * i as f64 / n as f64;
    let mut best = f64::INFINITY;
    for j in 0..=n {
        let y = at(j, y0, y1);
        for i in 0..n {
            let (a, b) = (at(i, x0, x1), at(i + 1, x0, x1));
            let (fa, fb) = (phi(a, y), phi(b, y));
            if fa == 0.0 || fa.signum() != fb.signum() {
                let x = if fa == 0.0 { a } else { a - fa * (b - a) / (fb - fa) };
                best = best.min(((x - p.0).powi(2) + (y - p.1).powi(2)).sqrt());
            }
        }
    }
    for i in 0..=n {
        let x = at(i, x0, x1);
        for j in 0..n {
            let (a, b) = (at(j, y0, y1), at(j + 1, y0, y1));
            let (fa, fb) = (phi(x, a), phi(x, b));
            if fa == 0.0 || fa.signum() != fb.signum() {
                let y = if fa == 0.0 { a } else { a - fa * (b - a) / (fb - fa) };
                best = best.min(((x - p.0).powi(2) + (y - p.1).powi(2)).sqrt());
            }
        }
    }
    best
}

/// **The fold, moved onto a domain side.** Wall `z = s·x + h(y)` with
/// `h = 4β·y(L−y)/L²`, `x ∈ [0, X]`, `y ∈ [−0.1L, 1.1L]`. Along the side
/// `x = 0`, φ = h ∈ [0, β] between y = 0 and y = L; the locus is two arcs,
/// one for y < 0 and one for y > L, each meeting the side at one point.
/// The declared carrier (0,0,0) → (0,L,0) runs along the side where there
/// is no zero; its midpoint is L/2 from the locus.
#[test]
fn r1_side_fold() {
    for (s, bk, l) in [
        (1.0, 0.5, 1e-3),
        (1.0, 0.5, 1e-2),
        (1.0, 0.25, 1e-3),
        (2.0, 0.9, 1e-3),
        (1.0, 0.5, 0.1),
    ] {
        let beta = bk * eps();
        let xr = (0.0, 1e-3_f64.max(l * 0.2));
        let yr = (-0.1 * l, 1.1 * l);
        let g = move |x: f64| s * x;
        let h = move |y: f64| 4.0 * beta * y * (l - y) / (l * l);
        let dh = move |y: f64| 4.0 * beta * (l - 2.0 * y) / (l * l);
        let w = wall(xr, yr, (g, s), (h, dh(yr.0)));
        let phi = move |x: f64, y: f64| g(x) + h(y);
        let mid = nearest_zero(&phi, xr, yr, (0.0, 0.5 * l));
        let got = declared(&w, Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, l, 0.0));
        eprintln!(
            "R1 side_fold s={s} beta={bk}eps L={l} eps={:e}: carrier mid to nearest zero = {mid:e}; result = {}",
            eps(),
            match &got {
                Ok(c) => format!("OK {c:?}").chars().take(300).collect::<String>(),
                Err(e) => format!("Err {e:?}"),
            }
        );
    }
}

/// **A phantom edge on a side.** Wall `z = s·x + β` (constant offset
/// β < ε): there is no intersection with z = 0 over x ≥ 0 at all.
#[test]
fn r1_side_phantom() {
    for (s, bk, l) in [(1.0, 0.5, 1e-3), (1.0, 0.5, 0.1), (0.5, 0.3, 1e-2)] {
        let beta = bk * eps();
        let xr = (0.0, 1e-3_f64.max(l * 0.2));
        let yr = (-0.1 * l, 1.1 * l);
        let g = move |x: f64| s * x;
        let h = move |_y: f64| beta;
        let w = wall(xr, yr, (g, s), (h, 0.0));
        let got = declared(&w, Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, l, 0.0));
        eprintln!(
            "R1 side_phantom s={s} beta={bk}eps L={l} eps={:e}: locus empty; result = {}",
            eps(),
            match &got {
                Ok(c) => format!("OK {c:?}").chars().take(300).collect::<String>(),
                Err(e) => format!("Err {e:?}"),
            }
        );
    }
}

/// Biquadratic net of `z = f(x, y)` for f bilinear plus separable
/// quadratics: control z at Greville points equals f for bilinear terms.
fn net(xr: (f64, f64), yr: (f64, f64), z: impl Fn(usize, usize) -> f64) -> NurbsSurface<f64> {
    let k = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let (xs, ys) = (
        [xr.0, 0.5 * (xr.0 + xr.1), xr.1],
        [yr.0, 0.5 * (yr.0 + yr.1), yr.1],
    );
    let control = (0..9)
        .map(|k| Point3::new(xs[k / 3], ys[k % 3], z(k / 3, k % 3)))
        .collect();
    NurbsSurface::new(k(), k(), control, vec![1.0; 9]).unwrap()
}

/// **A plane flush with two adjacent sides.** `z = k·x·y + δ·x` over
/// `[0, X] × [0, Y]`: φ = 0 on x = 0 exactly, and on y = 0 φ = δx
/// (δ = 0: flush with both). Carriers along the side x = 0, from the
/// corner and from beside it.
#[test]
fn r1_corner() {
    for (kk, dk) in [(1.0, 0.0), (1.0, 1e-3), (1.0, 1.0), (0.01, 0.0)] {
        let (xm, ym) = (1e-2, 1e-2);
        let xs = [0.0, 0.5 * xm, xm];
        let ys = [0.0, 0.5 * ym, ym];
        let w = net((0.0, xm), (0.0, ym), |i, j| kk * xs[i] * ys[j] + dk * xs[i]);
        for (a, b) in [(0.0, 0.5 * ym), (0.1 * ym, 0.6 * ym), (0.0, ym)] {
            let got = declared(&w, Point3::new(0.0, a, 0.0), Point3::new(0.0, b, 0.0));
            eprintln!(
                "R1 corner k={kk} delta={dk} carrier y {a}..{b} eps={:e}: {}",
                eps(),
                match &got {
                    Ok(c) => format!("OK {c:?}").chars().take(200).collect::<String>(),
                    Err(e) => format!("Err {e:?}"),
                }
            );
        }
    }
}

/// **A plain flush side.** `z = s·x + k·x·y` over `[0, X] × [y0, y1]`:
/// φ = 0 exactly on x = 0. Carriers along x = 0 well inside the side.
#[test]
fn r1_flush_plain() {
    for (s, kk, xm, yr) in [
        (1.0, 0.0, 1e-2, (0.0, 1e-2)),
        (1.0, 1.0, 1e-2, (0.0, 1e-2)),
        (1.0, 0.0, 1.0, (0.0, 1.0)),
        (1.0, 1.0, 1.0, (0.0, 1.0)),
        (1.0, 0.0, 1.0, (-1.0, 2.0)),
    ] {
        let xs = [0.0, 0.5 * xm, xm];
        let ys = [yr.0, 0.5 * (yr.0 + yr.1), yr.1];
        let w = net((0.0, xm), yr, |i, j| s * xs[i] + kk * xs[i] * ys[j]);
        let span = yr.1 - yr.0;
        for (a, b) in [(yr.0 + 0.2 * span, yr.0 + 0.8 * span), (0.0, 0.5)] {
            let got = declared(&w, Point3::new(0.0, a, 0.0), Point3::new(0.0, b, 0.0));
            eprintln!(
                "R1 flush s={s} k={kk} X={xm} y={yr:?} carrier y {a}..{b} eps={:e}: {}",
                eps(),
                match &got {
                    Ok(c) => format!("OK {c:?}").chars().take(160).collect::<String>(),
                    Err(e) => format!("Err {e:?}").chars().take(200).collect::<String>(),
                }
            );
        }
    }
}

#[test]
fn r1_dbg() {
    let xs = [0.0, 0.5, 1.0];
    let ys = [0.0, 0.5, 1.0];
    let w = net((0.0, 1.0), (0.0, 1.0), |i, j| xs[i] + xs[i] * ys[j]);
    let got = declared(&w, Point3::new(0.0, 0.2, 0.0), Point3::new(0.0, 0.8, 0.0));
    eprintln!("R1 dbg {got:?}");
}

#[test]
fn r1_endings() {
    use geom_brep::recourse::Reading;
    let xs = [0.0, 0.5, 1.0];
    let ys = [0.0, 0.5, 1.0];
    let w = net((0.0, 1.0), (0.0, 1.0), |i, j| xs[i] + xs[i] * ys[j]);
    let und = declared(&w, Point3::new(0.0, 0.2, 0.0), Point3::new(0.0, 0.8, 0.0)).unwrap_err();
    let mk = |c| geom_brep::PlaneNurbsRefusal::TubeNotOneArc { rungs: 3, cause: c };
    for r in [
        und,
        mk(geom_brep::ssi::OneArcRefusal::Short),
        mk(geom_brep::ssi::OneArcRefusal::Count { solutions: 0 }),
    ] {
        eprintln!("R1E display: {r}");
        for rd in [Reading::Build, Reading::AtRest, Reading::Adopt] {
            eprintln!("R1E   {rd:?}: {:?}", r.ending(rd));
        }
    }
}

/// **A quarter frustum and the plane through its seam ruling.** The
/// m7_8 quarter cylinder with its top arc at radius `r1` instead of 1:
/// the `u = 0` ruling from (1,0,0) to (r1,0,1) lies in y = 0 exactly, a
/// planar cap on a conical wall. r1 = 1 is m7_8 itself.
#[test]
fn r1_frustum() {
    use crate::shared::fixture::{segment, transverse_plane};
    for r1 in [1.0, 1.001, 1.1, 2.0] {
        let control = vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(r1, 0.0, 1.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(r1, r1, 1.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, r1, 1.0),
        ];
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let kv2 = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv1 = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let wall = NurbsSurface::new(kv2, kv1, control, vec![1.0, 1.0, w, w, 1.0, 1.0]).unwrap();
        let carrier = segment(Point3::new(1.0, 0.0, 0.0), Point3::new(r1, 0.0, 1.0));
        let got = geom_brep::plane_nurbs_limbs::<f64>(&carrier, &transverse_plane(), &wall, 1.0, band());
        eprintln!(
            "R1 frustum r1={r1} eps={:e}: {}",
            eps(),
            match &got {
                Ok(c) => format!("OK {c:?}").chars().take(160).collect::<String>(),
                Err(e) => format!("Err {e:?}").chars().take(220).collect::<String>(),
            }
        );
    }
}
