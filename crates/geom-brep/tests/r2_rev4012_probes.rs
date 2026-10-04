//! Review probes for PR 4012 (lane r2). Not for merge.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

fn graph_wall(
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
    wall: &NurbsSurface<f64>,
    from: (f64, f64),
    to: (f64, f64),
) -> Result<geom_brep::PlaneNurbsLimbs<f64>, geom_brep::PlaneNurbsRefusal> {
    let carrier = crate::shared::fixture::segment(
        Point3::new(from.0, from.1, 0.0),
        Point3::new(to.0, to.1, 0.0),
    );
    geom_brep::plane_nurbs_limbs::<f64>(&carrier, &plane(), wall, 1.0, band())
}

/// Dense-grid labelling: the nearest sign change of phi on a grid of
/// x-lines to the point (px, py), in xy.
fn nearest_zero(phi: &dyn Fn(f64, f64) -> f64, xr: (f64, f64), yr: (f64, f64), p: (f64, f64)) -> f64 {
    let n = 2000;
    let mut best = f64::INFINITY;
    for j in 0..=n {
        let y = yr.0 + (yr.1 - yr.0) * f64::from(j) / f64::from(n);
        let mut prev: Option<(f64, f64)> = None;
        for i in 0..=n {
            let x = xr.0 + (xr.1 - xr.0) * f64::from(i) / f64::from(n);
            let f = phi(x, y);
            if f == 0.0 {
                best = best.min(((x - p.0).powi(2) + (y - p.1).powi(2)).sqrt());
            }
            if let Some((px, pf)) = prev {
                let pf: f64 = pf;
                if pf * f < 0.0 {
                    let xz = px - pf * (x - px) / (f - pf);
                    best = best.min(((xz - p.0).powi(2) + (y - p.1).powi(2)).sqrt());
                }
            }
            prev = Some((x, f));
        }
    }
    best
}

/// SIDE-ARM OVERRUN. z = k x + h(y), h linear from h0 to h1 with
/// |h| < eps on the side x = 0. The exact locus x = -h(y)/k exists only
/// where h <= 0, i.e. y <= y*. A carrier along the side x = 0 from y = 0
/// to y = 1 overruns the locus by (1 - y*).
#[test]
fn r2_side_arm_overrun() {
    let e = eps();
    for (k, h0, h1) in [
        (10.0, -0.5, 0.6),
        (10.0, -0.5, 0.5),
        (10.0, 0.4, 0.4),   // no zero anywhere: plane offset 0.4 eps
        (2.0, -0.5, 0.6),
        (100.0, -0.3, 0.8),
    ] {
        let (h0, h1) = (h0 * e, h1 * e);
        let g = move |x: f64| k * x;
        let h = move |y: f64| h0 + (h1 - h0) * y;
        let phi = move |x: f64, y: f64| g(x) + h(y);
        for (name, xr) in [("side at x=0 (domain [0,1])", (0.0, 1.0)), ("interior (domain [-1,1])", (-1.0, 1.0))] {
            let wall = graph_wall(xr, (0.0, 1.0), (g, k), (h, h1 - h0));
            let got = declared(&wall, (0.0, 0.0), (0.0, 1.0));
            let d_end = nearest_zero(&phi, xr, (0.0, 1.0), (0.0, 1.0));
            let d_start = nearest_zero(&phi, xr, (0.0, 1.0), (0.0, 0.0));
            println!(
                "R2PROBE overrun eps={e:e} k={k} h0={:.2}e h1={:.2}e {name}: nearest zero to start {d_start:.3e}, to end {d_end:.3e} -> {}",
                h0 / e,
                h1 / e,
                match &got {
                    Ok(l) => format!("OK {l:?}").chars().take(200).collect::<String>(),
                    Err(err) => format!("ERR {err:?}").chars().take(300).collect(),
                }
            );
        }
    }
}

/// CORNER: phi = k x y, flush with both x = 0 and y = 0.
#[test]
fn r2_corner_two_flush_sides() {
    let e = eps();
    // z = s*(x*y): bilinear, a biquadratic net exact.
    let k = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    for s in [1.0, 1e-3] {
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, s),
        ];
        let wall = NurbsSurface::new(k(), k(), control, vec![1.0; 4]).unwrap();
        for (from, to) in [((0.0, 0.0), (0.0, 1.0)), ((0.0, 0.0), (1.0, 0.0)), ((0.0, 0.2), (0.0, 1.0))] {
            let got = declared(&wall, from, to);
            println!(
                "R2PROBE corner eps={e:e} s={s} {from:?}->{to:?}: {}",
                match &got {
                    Ok(_) => "OK".to_string(),
                    Err(err) => format!("ERR {err:?}").chars().take(300).collect(),
                }
            );
        }
    }
}

/// The same side-arm fixtures through the SEARCH door, and the two-arc
/// join along a flush side; plus the refusal's at-rest ending.
#[test]
fn r2_side_arm_search_and_join() {
    use geom_brep::ssi::{self, SsiDomain};
    let e = eps();
    let domain = SsiDomain {
        center: Point3::new(0.5, 0.5, 0.0),
        half_extent: 1.0,
        extent: 1.0,
        floor_scale: 1.0,
    };
    let k = 10.0;
    let g = move |x: f64| k * x;
    for (name, h0, h1) in [("overrun", -0.5, 0.6), ("no zero", 0.4, 0.4)] {
        let (h0, h1) = (h0 * e, h1 * e);
        let h = move |y: f64| h0 + (h1 - h0) * y;
        let wall = graph_wall((0.0, 1.0), (0.0, 1.0), (g, k), (h, h1 - h0));
        let s = ssi::plane_nurbs_ssi(&plane(), &wall, domain, band());
        let at_rest = declared(&wall, (0.0, 0.0), (0.0, 1.0)).is_ok();
        match s {
            Ok(out) => println!(
                "R2PROBE search {name} eps={e:e}: at-rest Ok={at_rest}; search {} branches {:?}; contacts {:?}",
                out.branches.len(),
                out.branches.iter().map(|b| format!("{:?}", b.end)).collect::<Vec<_>>(),
                format!("{:?}", out.boundary).chars().take(400).collect::<String>()
            ),
            Err(err) => println!("R2PROBE search {name} eps={e:e}: at-rest Ok={at_rest}; search ERR {err}"),
        }
    }
    // Two arcs meeting the flush side near y=0 and y=1, phi>0 between.
    let (lo, hi) = (-0.5 * e, 0.5 * e);
    let h = move |y: f64| lo + 4.0 * (hi - lo) * y * (1.0 - y);
    let wall = graph_wall((0.0, 1.0), (0.0, 1.0), (g, k), (h, 4.0 * (hi - lo)));
    let phi = move |x: f64, y: f64| g(x) + h(y);
    let got = declared(&wall, (0.0, 0.0), (0.0, 1.0));
    let mid = nearest_zero(&phi, (0.0, 1.0), (0.0, 1.0), (0.0, 0.5));
    println!(
        "R2PROBE join eps={e:e}: nearest zero to carrier midpoint {mid:.3e} -> {}",
        match &got {
            Ok(_) => "OK".to_string(),
            Err(err) => format!("ERR {err:?}"),
        }
    );
    let refusal = geom_brep::PlaneNurbsRefusal::TubeNotOneArc {
        rungs: 3,
        cause: geom_brep::ssi::OneArcRefusal::Short,
    };
    for r in [geom_brep::recourse::Reading::Build, geom_brep::recourse::Reading::AtRest, geom_brep::recourse::Reading::Adopt] {
        println!("R2PROBE ending {r:?}: {refusal} || {:?}", refusal.ending(r));
    }
}
