//! Review probe for PR 3817's sense cross-check
//! (`props::curved::sphere_circle_loop_side`): hand-built tilted loops
//! on a z-poled sphere — caps, lunes, octants, a band-with-slit larger
//! than a hemisphere — at random rotations. The oracle is independent
//! of the kernel: which side of the loop is to its left is known by
//! construction (and its area in closed form), and whether that side
//! holds a pole is read off the region's own membership formula.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, MaterialSign, PropsError, boundary_material_sign, curved_face};
use geom_core::{Band, Point3, Sign, Tol, Vec3};

const R: f64 = 1.7;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(
                2.0 * self.next() - 1.0,
                2.0 * self.next() - 1.0,
                2.0 * self.next() - 1.0,
            );
            let n = v.norm();
            if n > 0.1 && n < 1.0 {
                return v / n;
            }
        }
    }
    /// A right-handed orthonormal frame.
    fn frame(&mut self) -> [Vec3<f64>; 3] {
        let a = self.unit();
        let t = self.unit();
        let b = (t - a * t.dot(a)).normalize();
        [a, b, a.cross(b)]
    }
}

fn o() -> Point3<f64> {
    Point3::new(0.0, 0.0, 0.0)
}

fn circle(center: Vec3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>) -> Curve3<f64> {
    Curve3::Circle {
        center: o() + center,
        axis,
        radius,
        u_ref,
    }
}

fn e(c: Curve3<f64>, t0: f64, t1: f64, fwd: bool, s: u32, n: u32) -> LoopEdge<f64> {
    LoopEdge::hand_built(c, t0, t1, fwd, s, n)
}

/// A loop, the closed-form area of its LEFT side under the outward
/// normal, and that side's membership test (on unit vectors).
struct Shape {
    name: &'static str,
    edges: Vec<LoopEdge<f64>>,
    area_left: f64,
    left: Box<dyn Fn(Vec3<f64>) -> bool>,
}

fn cap(n: Vec3<f64>, h: f64) -> Shape {
    let u = n.cross(Vec3::new(0.3, 0.5, 0.7)).normalize();
    let rho = R * (1.0 - h * h).sqrt();
    Shape {
        name: "cap",
        edges: vec![e(circle(n * (h * R), n, rho, u), 0.0, 2.0 * PI, true, 0, 0)],
        area_left: 2.0 * PI * R * R * (1.0 - h),
        left: Box::new(move |p| p.dot(n) > h),
    }
}

fn lune(f: [Vec3<f64>; 3], a: f64, b: f64) -> Shape {
    let p = f[0];
    let w = |phi: f64| f[1] * phi.cos() + f[2] * phi.sin();
    let semi = |phi: f64| circle(Vec3::new(0.0, 0.0, 0.0), p.cross(w(phi)), R, p);
    Shape {
        name: "lune",
        edges: vec![
            e(semi(a), 0.0, PI, true, 0, 1),
            e(semi(b), 0.0, PI, false, 1, 0),
        ],
        area_left: 2.0 * (b - a) * R * R,
        left: Box::new(move |q: Vec3<f64>| {
            let az = q.dot(f[2]).atan2(q.dot(f[1]));
            // The azimuth in [a, a + 2π).
            let az = a + (az - a).rem_euclid(2.0 * PI);
            az > a && az < b
        }),
    }
}

fn octant(f: [Vec3<f64>; 3]) -> Shape {
    let z = Vec3::new(0.0, 0.0, 0.0);
    let arc = |axis: Vec3<f64>, u: Vec3<f64>, s, n| e(circle(z, axis, R, u), 0.0, PI / 2.0, true, s, n);
    Shape {
        name: "octant",
        edges: vec![
            arc(f[2], f[0], 0, 1),
            arc(f[0], f[1], 1, 2),
            arc(f[1], f[2], 2, 0),
        ],
        area_left: R * R * PI / 2.0,
        left: Box::new(move |q: Vec3<f64>| q.dot(f[0]) > 0.0 && q.dot(f[1]) > 0.0 && q.dot(f[2]) > 0.0),
    }
}

/// `|q·m| < a` minus the wedge of half-width `w` about azimuth 0
/// (about `m`, from `f[0]`): larger than a hemisphere for `a = 0.8`.
fn slit_band(f: [Vec3<f64>; 3], a: f64, w: f64) -> Shape {
    let m = f[2];
    let rho = R * (1.0 - a * a).sqrt();
    let el = a.asin();
    let dir = |psi: f64| f[0] * psi.cos() + f[1] * psi.sin();
    let lower = circle(m * (-a * R), m, rho, f[0]);
    let upper = circle(m * (a * R), m, rho, f[0]);
    let meridian = |psi: f64| circle(Vec3::new(0.0, 0.0, 0.0), dir(psi).cross(m), R, dir(psi));
    Shape {
        name: "slit-band",
        edges: vec![
            e(lower, w, 2.0 * PI - w, true, 0, 1),
            e(meridian(2.0 * PI - w), -el, el, true, 1, 2),
            e(upper, w, 2.0 * PI - w, false, 2, 3),
            e(meridian(w), -el, el, false, 3, 0),
        ],
        area_left: 4.0 * PI * a * R * R * (1.0 - w / PI),
        left: Box::new(move |q: Vec3<f64>| {
            let az = q.dot(f[1]).atan2(q.dot(f[0]));
            q.dot(m).abs() < a && az.abs() > w
        }),
    }
}

fn reversed(edges: &[LoopEdge<f64>]) -> Vec<LoopEdge<f64>> {
    edges
        .iter()
        .rev()
        .map(|x| LoopEdge::hand_built(x.carrier.clone(), x.t0, x.t1, !x.forward, x.end, x.start))
        .collect()
}

#[derive(Default, Debug)]
struct Tally {
    loops: usize,
    encoded_right: usize,
    unencoded_pole_free: usize,
    unencoded_split: usize,
    wrong_encoded: usize,
    false_refusal: usize,
    flip_measured_pole_free: usize,
    area_bad: usize,
    other_err: usize,
}

fn check(sh: &Shape, edges: &[LoopEdge<f64>], left_is_face_under_true: bool, t: &mut Tally, band: Band) {
    let sphere = Surface::Sphere {
        center: o(),
        radius: R,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    t.loops += 1;
    let z = Vec3::new(0.0, 0.0, 1.0);
    // The loop's left side, by construction (or its complement when the
    // loop was reversed).
    let in_left = |q: Vec3<f64>| (sh.left)(q) == left_is_face_under_true;
    let area_left = if left_is_face_under_true {
        sh.area_left
    } else {
        4.0 * PI * R * R - sh.area_left
    };
    let (n_in, s_in) = (in_left(z), in_left(-z));
    let expect = match (n_in, s_in) {
        (false, false) => Some(Sign::Positive),
        (true, true) => Some(Sign::Negative),
        _ => None,
    };
    let got = boundary_material_sign(&sphere, edges, band);
    match (&got, expect) {
        (Ok(MaterialSign::Encoded(s)), Some(x)) if *s == x => t.encoded_right += 1,
        (Ok(MaterialSign::Encoded(_)), _) => {
            t.wrong_encoded += 1;
            println!("WRONG ENCODED {} got {got:?} expect {expect:?}", sh.name);
        }
        (Ok(MaterialSign::Unencoded), Some(_)) => t.unencoded_pole_free += 1,
        (Ok(MaterialSign::Unencoded), None) => t.unencoded_split += 1,
        (Err(err), _) => {
            t.other_err += 1;
            println!("material sign err {} {err:?}", sh.name);
        }
    }
    // The face under each bit: sense true is the left side.
    for sense in [true, false] {
        let face_area = if sense { area_left } else { 4.0 * PI * R * R - area_left };
        let pole_free = match (sense, expect) {
            (true, Some(Sign::Positive)) | (false, Some(Sign::Negative)) => Some(true),
            (true, Some(Sign::Negative)) | (false, Some(Sign::Positive)) => Some(false),
            _ => None,
        };
        match curved_face(&sphere, edges, sense, band) {
            Ok(fc) => {
                if pole_free == Some(false) {
                    t.flip_measured_pole_free += 1;
                    println!(
                        "FLIP MEASURED {} sense {sense}: area {} (face {face_area})",
                        sh.name, fc.area
                    );
                }
                if (fc.area - face_area).abs() > 1e-9 * R * R {
                    t.area_bad += 1;
                    println!("AREA {} sense {sense}: {} vs {face_area}", sh.name, fc.area);
                }
            }
            Err(PropsError::SenseContradicted) => {
                if pole_free == Some(true) {
                    t.false_refusal += 1;
                    println!("FALSE REFUSAL {} sense {sense}", sh.name);
                }
            }
            Err(err) => {
                if pole_free == Some(true) {
                    t.false_refusal += 1;
                    println!("REFUSED pole-free {} sense {sense}: {err:?}", sh.name);
                } else {
                    t.other_err += 1;
                }
            }
        }
    }
}

#[test]
fn review_3817_side_against_construction() {
    let band = Band::linear(Tol::witness()).unwrap();
    let mut rng = Rng(0x3817);
    let mut t = Tally::default();
    // Monte Carlo sanity of the closed-form areas the oracle uses.
    {
        let f = rng.frame();
        let shapes = [cap(f[2], 0.3), lune(f, 0.4, 2.9), octant(f), slit_band(f, 0.8, 0.1)];
        let n = 400_000;
        let pts: Vec<Vec3<f64>> = (0..n).map(|_| rng.unit()).collect();
        for sh in &shapes {
            let k = pts.iter().filter(|p| (sh.left)(**p)).count();
            let mc = 4.0 * PI * R * R * k as f64 / n as f64;
            let sigma = 4.0 * PI * R * R * ((k as f64 / n as f64) * (1.0 - k as f64 / n as f64) / n as f64).sqrt();
            println!("MC {}: {mc} vs closed {} ({}σ)", sh.name, sh.area_left, (mc - sh.area_left) / sigma);
            assert!((mc - sh.area_left).abs() < 5.0 * sigma);
        }
    }
    for _ in 0..400 {
        let f = rng.frame();
        let h = 1.8 * rng.next() - 0.9;
        let a = 2.0 * PI * rng.next();
        let b = a + 0.2 + (2.0 * PI - 0.4) * rng.next();
        let a_band = 0.5 + 0.45 * rng.next();
        let shapes = [
            cap(f[2], h),
            lune(f, a, b),
            octant(f),
            slit_band(f, a_band, 0.05 + 0.5 * rng.next()),
        ];
        for sh in &shapes {
            check(sh, &sh.edges, true, &mut t, band);
            check(sh, &reversed(&sh.edges), false, &mut t, band);
        }
    }
    // Near-pole caps: the rim passing a hair from the pole.
    for k in 0..200 {
        let d = 10f64.powf(-(1.0 + 11.0 * (k as f64) / 200.0));
        let phi = 2.0 * PI * rng.next();
        // A cap of angular radius 0.6 whose rim passes at angular
        // distance `±d` from the north pole.
        let ang = 0.6 + if k % 2 == 0 { d } else { -d };
        let n = Vec3::new(ang.sin() * phi.cos(), ang.sin() * phi.sin(), ang.cos());
        let sh = cap(n, 0.6f64.cos());
        let before = t.unencoded_pole_free;
        check(&sh, &sh.edges, true, &mut t, band);
        if t.unencoded_pole_free != before {
            println!("NEAR-POLE unencoded at angular distance {d:e} (pole {})", if k % 2 == 0 { "outside" } else { "inside" });
        }
        check(&sh, &reversed(&sh.edges), false, &mut t, band);
    }
    println!("{t:#?}");
    assert_eq!(t.wrong_encoded, 0);
    assert_eq!(t.false_refusal, 0);
    // Only the in-band near-pole caps (rim within ~1e-9 rad of the
    // pole) read no side; a flip there measures under the bit alone.
    assert_eq!(t.flip_measured_pole_free, t.unencoded_pole_free);
    // (How many is ε-dependent: ~1e-9 rad at ε 1e-9, ~1e-6 at 1e-6.)
    assert_eq!(t.area_bad, 0);
}
