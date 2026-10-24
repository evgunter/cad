//! Closed forms and references more than one scene checks its bodies
//! against, spelled once.

use core::f64::consts::PI;

use pncad::geom::{NurbsSurface, Surface};
use pncad::geom_core::{Point3, Vec3};
use pncad::topo::Body;

/// The volume of an `a × b × c` box with every edge rounded at radius
/// `r` (every corner a sphere octant) — the Steiner form of the core box
/// `a − 2r × b − 2r × c − 2r` grown by `r`: core, plus a slab of
/// thickness `r` on each face, plus twelve quarter-cylinders along the
/// edges, plus eight octants that sum to one ball.
pub fn rounded_box_volume([a, b, c]: [f64; 3], r: f64) -> f64 {
    let [x, y, z] = [a - 2.0 * r, b - 2.0 * r, c - 2.0 * r];
    x * y * z
        + 2.0 * r * (x * y + y * z + z * x)
        + PI * r * r * (x + y + z)
        + (4.0 / 3.0) * PI * r.powi(3)
}

/// The `n`-point Gauss–Legendre rule on `[-1, 1]`, as `(node, weight)`:
/// Newton on the three-term recurrence from Tricomi's initial guess.
fn gauss_legendre(n: usize) -> Vec<(f64, f64)> {
    let nf = n as f64;
    (1..=n)
        .map(|k| {
            let mut x = (PI * (k as f64 - 0.25) / (nf + 0.5)).cos();
            let mut dp = 0.0;
            for _ in 0..100 {
                let (mut p0, mut p1) = (1.0, x);
                for j in 2..=n {
                    let jf = j as f64;
                    (p0, p1) = (p1, ((2.0 * jf - 1.0) * x * p1 - (jf - 1.0) * p0) / jf);
                }
                dp = nf * (x * p1 - p0) / (x * x - 1.0);
                let dx = p1 / dp;
                x -= dx;
                if dx.abs() <= 1e-16 {
                    break;
                }
            }
            (x, 2.0 / ((1.0 - x * x) * dp * dp))
        })
        .collect()
}

/// A body's volume and area integrated over its OWN surfaces, with the
/// difference between two orders as each one's error estimate.
#[derive(Clone, Copy, Debug)]
pub struct SkinQuadrature {
    pub volume: f64,
    pub area: f64,
    pub volume_err: f64,
    pub area_err: f64,
}

/// **The volume and area of a body whose faces are planes and untrimmed
/// NURBS patches**, integrated over the body's own surfaces with a
/// Gauss–Legendre rule on every knot span, at 12 and at 24 points: the
/// reference a fitted skin has where it has no closed form, and the
/// one a certified enclosure of it must hold.
///
/// The volume is the divergence theorem, `V = ⅓ ∮ (x − p)·n dA`. A
/// patch's span is a smooth rational piece, where the rule converges
/// geometrically. A planar face contributes `(o − p)·n · A`, and its
/// area `A` is Green's `½ ∮ (x − o) × dx · n` around its boundary,
/// which is the patches' own boundary isolines lying in that plane,
/// walked against the patches' induced orientation.
pub fn skin_quadrature(body: &Body<f64>) -> SkinQuadrature {
    let (v_lo, a_lo) = skin_quadrature_at(body, 12);
    let (volume, area) = skin_quadrature_at(body, 24);
    SkinQuadrature {
        volume,
        area,
        volume_err: (volume - v_lo).abs(),
        area_err: (area - a_lo).abs(),
    }
}

fn skin_quadrature_at(body: &Body<f64>, order: usize) -> (f64, f64) {
    let rule = gauss_legendre(order);
    // `(origin, outward normal, area so far)`.
    let mut planes: Vec<(Point3<f64>, Vec3<f64>, f64)> = Vec::new();
    let mut patches: Vec<(&NurbsSurface<f64>, f64)> = Vec::new();
    for (_, f) in body.faces() {
        let sense = if f.sense { 1.0 } else { -1.0 };
        match body.get_surface(f.surface) {
            Some(Surface::Plane { origin, normal, .. }) => {
                planes.push((*origin, *normal * sense, 0.0));
            }
            Some(Surface::Nurbs(n)) => patches.push((n, sense)),
            other => panic!("a quadrature over planes and NURBS, got {other:?}"),
        }
    }
    let (count, sum) = body
        .vertex_points()
        .fold((0.0, Vec3::zero()), |(k, s), (_, q)| {
            (k + 1.0, s + (q - Point3::origin()))
        });
    let p = Point3::origin() + sum / count;
    let breaks = |knots: &[f64]| {
        let mut b = knots.to_vec();
        b.dedup();
        b
    };
    // Feeds `f` the rule's nodes and weights on every span of `breaks`.
    let along = |breaks: &[f64], f: &mut dyn FnMut(f64, f64)| {
        for t in breaks.windows(2) {
            let h = 0.5 * (t[1] - t[0]);
            for &(x, w) in &rule {
                f(t[0] + h * (1.0 + x), w * h);
            }
        }
    };
    let (mut flux, mut area, mut closure) = (0.0, 0.0, Vec3::zero());
    for &(n, sense) in &patches {
        let (bu, bv) = (
            breaks(n.knot_values_u().values()),
            breaks(n.knot_values_v().values()),
        );
        along(&bu, &mut |u, wu| {
            along(&bv, &mut |v, wv| {
                let jet = n.ders(u, v);
                let normal = jet.du.cross(jet.dv) * sense;
                flux += wu * wv * (jet.point - p).dot(normal);
                area += wu * wv * normal.norm();
                closure = closure + normal * (wu * wv);
            });
        });
        // The four boundary isolines, each with the direction the
        // patch's outward orientation walks it: `+u` along `v` low,
        // `+v` along `u` high, `−u` along `v` high, `−v` along `u` low.
        let ((u0, u1), (v0, v1)) = ((bu[0], bu[bu.len() - 1]), (bv[0], bv[bv.len() - 1]));
        // `(along u, the other parameter's value, the walk)`.
        for (along_u, at, walk) in [
            (true, v0, 1.0),
            (false, u1, 1.0),
            (true, v1, -1.0),
            (false, u0, -1.0),
        ] {
            let ts: &[f64] = if along_u { &bu } else { &bv };
            let curve = |t: f64| {
                if along_u {
                    let j = n.ders(t, at);
                    (j.point, j.du)
                } else {
                    let j = n.ders(at, t);
                    (j.point, j.dv)
                }
            };
            let mid = 0.5 * (ts[0] + ts[ts.len() - 1]);
            let probes = [ts[0], mid, ts[ts.len() - 1]];
            let Some(cap) = planes.iter_mut().find(|(o, nn, _)| {
                probes
                    .iter()
                    .all(|&t| (curve(t).0 - *o).dot(*nn).abs() <= 1e-9)
            }) else {
                continue;
            };
            let (o, nn) = (cap.0, cap.1);
            // The cap walks its boundary against the patch's walk.
            along(ts, &mut |t, w| {
                let (x, dx) = curve(t);
                cap.2 -= 0.5 * w * walk * sense * (x - o).cross(dx).dot(nn);
            });
        }
    }
    for &(o, nn, a) in &planes {
        assert!(
            a > 0.0,
            "a planar face's boundary isolines enclose area {a:e}: not every cap is bounded \
             by its patches' isolines, or its orientation disagrees with theirs"
        );
        flux += (o - p).dot(nn) * a;
        area += a;
        closure = closure + nn * a;
    }
    // A closed surface's `∮ n dA` vanishes, so a cap whose boundary
    // missed an isoline shows here.
    assert!(
        closure.norm() <= 1e-9 * area,
        "the faces do not close: ∮ n dA = {closure:?} against an area of {area}"
    );
    (flux / 3.0, area)
}
