//! Reviewer probe (PR 3973 dual, lane reach-dual3973-r2). Dropped into
//! `crates/topo/src/boolean/` and declared `#[cfg(test)] mod probe_et;`
//! in `boolean/mod.rs` for the run only. Prints one JSON line per pose
//! (`PROBE {...}`) with the exact f64 inputs and the door's answer; the
//! verdicts are judged by `probes/oracle_et.py` (mpmath, 50 digits),
//! never by the kernel.
#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::{Band, Point3, Vec3};

use super::circle_roots::CircleRoots;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^= z >> 31;
        (z >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.range(-1., 1.), self.range(-1., 1.), self.range(-1., 1.));
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                return v / n;
            }
        }
    }
}

/// A random rotation as three orthonormal columns.
fn frame(rng: &mut Rng) -> [Vec3<f64>; 3] {
    let z = rng.unit();
    let w = rng.unit();
    let x = (w - z * w.dot(z)).normalize();
    [x, z.cross(x), z]
}

fn apply(f: &[Vec3<f64>; 3], v: [f64; 3]) -> Vec3<f64> {
    f[0] * v[0] + f[1] * v[1] + f[2] * v[2]
}

struct Pose {
    family: &'static str,
    center: Point3<f64>,
    axis: Vec3<f64>,
    u: Vec3<f64>,
    a: f64,
    b: f64,
    hub: Point3<f64>,
    t_axis: Vec3<f64>,
    big: f64,
    small: f64,
    t0: f64,
    t1: f64,
}

/// Families in the torus's own frame (axis z, hub 0), then rigidly
/// posed, scaled by `s` and carried `far` out.
fn pose(rng: &mut Rng, fam: usize, eps: f64, s: f64, far: f64) -> Pose {
    let big = rng.range(0.4, 2.0);
    let mut small = big * rng.range(0.05, 0.9);
    let g = eps / s * rng.range(-40.0, 40.0);
    // (family, centre, u, normal, a, b) in torus frame, unscaled
    let (family, c, u, n, a, b): (&str, [f64; 3], [f64; 3], [f64; 3], f64, f64) = match fam {
        // coaxial in the equatorial plane, crossing the tube up to 8x
        0 => (
            "coax8",
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            big + small * rng.range(1.02, 1.6),
            (big - small * rng.range(1.02, 1.6)).max(0.05 * big),
        ),
        // outer-equator graze at theta = 0 (gap g)
        1 => ("outer_graze", [0.0; 3], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0], big + small + g, big),
        // inner-equator graze at theta = pi/2 (gap g into the hole)
        2 => ("inner_graze", [0.0; 3], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0], big, big - small - g),
        // meridian ellipse grazing the tube's crown (gap g)
        3 => (
            "crown_graze",
            [big, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0],
            small + g,
            small * rng.range(0.2, 0.9),
        ),
        // meridian near-circle: a = r, b = r(1+d): touches at theta = 0, pi
        4 => {
            let d = [1e-2, 1e-6, 1e-10][rng.range(0.0, 2.999) as usize];
            ("meridian_touch", [big, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, -1.0, 0.0], small, small * (1.0 + d))
        }
        // nearly-circular, slightly tilted, through the tube
        5 => {
            let d = [1e-3, 1e-8, 1e-14][rng.range(0.0, 2.999) as usize];
            let tilt = rng.range(-0.3, 0.3);
            (
                "near_circle",
                [rng.range(0.3, 1.5) * small, 0.0, small * rng.range(-0.5, 0.5)],
                [1.0, 0.0, 0.0],
                [tilt, 0.2 * tilt, 1.0],
                big * (1.0 + d),
                big,
            )
        }
        // a fat torus grazed from OUTSIDE its outer equator, inside the
        // zero band (gap 0.1..1 eps): no crossing, but within the band
        7 => {
            let fat = big * rng.range(0.7, 0.97);
            small = fat;
            let gin = eps / s * rng.range(0.1, 1.0);
            ("fat_outer_in_band", [0.0; 3], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0], big + fat + gin, (big + fat) * rng.range(0.3, 0.999))
        }
        // anything: random ellipse through the tube region
        _ => {
            let n = rng.unit();
            let uu = rng.unit();
            ("random", [rng.range(-1., 1.) * big, rng.range(-1., 1.) * big, rng.range(-1., 1.) * small],
             [uu.x, uu.y, uu.z], [n.x, n.y, n.z], big * rng.range(0.3, 2.0), big * rng.range(0.05, 2.0))
        }
    };
    let f = frame(rng);
    let hub = Point3::new(far, -0.7 * far, 0.3 * far) + apply(&f, [rng.range(-1., 1.); 3]) * 0.0;
    let n = apply(&f, n).normalize();
    let u0 = apply(&f, u);
    let u = (u0 - n * u0.dot(n)).normalize();
    let center = hub + apply(&f, c) * s;
    let t0 = rng.range(-7.0, 7.0);
    let t1 = t0 + if rng.next() < 0.3 { core::f64::consts::TAU } else { rng.range(0.05, 6.2) };
    Pose { family, center, axis: n, u, a: a * s, b: b * s, hub, t_axis: f[2], big: big * s, small: small * s, t0, t1 }
}

fn v(p: Vec3<f64>) -> String {
    format!("[{:?}, {:?}, {:?}]", p.x, p.y, p.z)
}

fn answer(r: Result<CircleRoots<f64>, super::BooleanError>) -> (String, Vec<f64>) {
    match r {
        Ok(CircleRoots::Certified { count, thetas }) => ("certified".into(), thetas[..count].to_vec()),
        Ok(CircleRoots::Miss) => ("miss".into(), vec![]),
        Ok(CircleRoots::Uncertain) => ("uncertain".into(), vec![]),
        Ok(other) => (format!("{other:?}").replace('"', "'"), vec![]),
        Err(e) => (format!("err {e:?}").replace('"', "'").replace('\n', " "), vec![]),
    }
}

fn run(circle: bool, seed: u64, per: usize) {
    let mut rng = Rng(seed);
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for (s, far) in [(1e-3, 0.0), (1.0, 0.0), (1e3, 0.0), (1.0, 1e3)] {
            for fam in 0..8 {
                for _ in 0..per {
                    let mut p = pose(&mut rng, fam, eps, s, far);
                    if circle {
                        p.b = p.a;
                    }
                    let carrier = if circle {
                        geom::Curve3::Circle { center: p.center, axis: p.axis, radius: p.a, u_ref: p.u }
                    } else {
                        geom::Curve3::Ellipse { center: p.center, axis: p.axis, major: p.a, minor: p.b, u_ref: p.u }
                    };
                    let x = Vec3::new(1.0, 0.0, 0.0);
                    let x = if p.t_axis.cross(x).norm() < 0.1 { Vec3::new(0.0, 1.0, 0.0) } else { x };
                    let torus = geom::Surface::Torus {
                        center: p.hub,
                        axis: p.t_axis,
                        major_radius: p.big,
                        minor_radius: p.small,
                        u_ref: (x - p.t_axis * x.dot(p.t_axis)).normalize(),
                    };
                    let r = if circle {
                        super::circle_torus::circle_torus_roots(&carrier, p.t0, p.t1, &torus, band)
                    } else {
                        super::ellipse_roots::ellipse_roots(&carrier, p.t0, p.t1, &torus, band)
                    };
                    let (ans, roots) = answer(r);
                    println!(
                        "PROBE {{\"family\": \"{}\", \"eps\": {eps:?}, \"scale\": {s:?}, \"far\": {far:?}, \"center\": {}, \"axis\": {}, \"u_ref\": {}, \"major\": {:?}, \"minor\": {:?}, \"hub\": {}, \"t_axis\": {}, \"R\": {:?}, \"r\": {:?}, \"t0\": {:?}, \"t1\": {:?}, \"answer\": \"{ans}\", \"roots\": {roots:?}}}",
                        p.family,
                        v(p.center - Point3::new(0.0, 0.0, 0.0)),
                        v(p.axis),
                        v(p.u),
                        p.a,
                        p.b,
                        v(p.hub - Point3::new(0.0, 0.0, 0.0)),
                        v(p.t_axis),
                        p.big,
                        p.small,
                        p.t0,
                        p.t1,
                    );
                }
            }
        }
    }
}

fn per() -> usize {
    std::env::var("PROBE_PER").ok().and_then(|s| s.parse().ok()).unwrap_or(20)
}

#[test]
#[ignore = "reviewer probe"]
fn probe_ellipse_torus() {
    run(false, 0x3973_0002, per());
}

#[test]
#[ignore = "reviewer probe"]
fn probe_circle_torus() {
    run(true, 0x3973_0003, per());
}

/// Differential only (main vs head): circle x sphere, and the ellipse door
/// on spheres and walls (the degree-2 doors the generalised subdivision
/// now serves). Prints the answer and the roots' bits.
#[test]
#[ignore = "reviewer probe"]
fn probe_sphere_wall_differential() {
    let mut rng = Rng(0x3973_0004);
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for s in [1e-3, 1.0, 1e3] {
            for i in 0..per() * 8 {
                let n = rng.unit();
                let u0 = rng.unit();
                let u = (u0 - n * u0.dot(n)).normalize();
                let a = s * rng.range(0.2, 2.0);
                let b = if i % 2 == 0 { a } else { a * rng.range(0.05, 1.0) };
                let c = Point3::new(0.0, 0.0, 0.0);
                let theta = rng.range(0.0, 6.28);
                let p = c + u * (a * theta.cos()) + n.cross(u) * (b * theta.sin());
                let rad = s * rng.range(0.05, 2.0);
                let dir = rng.unit();
                let g = if i % 3 == 0 { eps * rng.range(-40.0, 40.0) } else { -rad * rng.range(0.0, 1.0) };
                let sc = p + dir * (rad + g);
                let sphere = geom::Surface::Sphere { center: sc, radius: rad, axis: Vec3::new(0.0, 0.0, 1.0), u_ref: Vec3::new(1.0, 0.0, 0.0) };
                let wax = rng.unit();
                let wd = (dir - wax * dir.dot(wax)).normalize();
                let x = Vec3::new(1.0, 0.0, 0.0);
                let x = if wax.cross(x).norm() < 0.1 { Vec3::new(0.0, 1.0, 0.0) } else { x };
                let wall = geom::Surface::Cylinder { origin: p + wd * (rad + g), axis: wax, radius: rad, u_ref: (x - wax * x.dot(wax)).normalize() };
                let t0 = rng.range(-7.0, 7.0);
                let t1 = t0 + rng.range(0.05, 6.28);
                let mut out = Vec::new();
                if b == a {
                    let circ = geom::Curve3::Circle { center: c, axis: n, radius: a, u_ref: u };
                    out.push(answer(super::circle_sphere::circle_sphere_roots(&circ, t0, t1, &sphere, band)));
                    out.push(answer(super::circle_cylinder::circle_cylinder_roots(&circ, t0, t1, &wall, band)));
                } else {
                    let e = geom::Curve3::Ellipse { center: c, axis: n, major: a, minor: b, u_ref: u };
                    out.push(answer(super::ellipse_roots::ellipse_roots(&e, t0, t1, &sphere, band)));
                    out.push(answer(super::ellipse_roots::ellipse_roots(&e, t0, t1, &wall, band)));
                }
                let bits: Vec<String> = out.iter().map(|(a, r)| format!("{a}:{:?}", r.iter().map(|x| x.to_bits()).collect::<Vec<_>>())).collect();
                println!("DIFF {eps:e} {s:e} {i} {}", bits.join(" | "));
            }
        }
    }
}

#[test]
#[ignore = "reviewer probe"]
fn probe_km_rim_door() {
    let e = geom::Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, 500.0),
        axis: Vec3::new(-0.29552020666133955, -0.0, -0.955336489125606),
        major: 523.3758007690428,
        minor: 500.0,
        u_ref: Vec3::new(0.955336489125606, 0.0, -0.29552020666133955),
    };
    let band = Band::new(1e-9, 1e-8).unwrap();
    for c in [
        [321.3752369509262, 661.5846181616804, 235.14662979504507],
        [321.375755337376, 661.585185133569, 235.14725422948206],
        [316.1395338072978, 655.8582020865741, 228.8398419813823],
    ] {
        let s = geom::Surface::Torus {
            center: Point3::new(c[0], c[1], c[2]),
            axis: Vec3::new(0.5236226766304979, 0.5726988773983616, 0.6307418555518332),
            major_radius: 300.0,
            minor_radius: 50.0,
            u_ref: Vec3::new(0.8256622984717515, -0.5236226766304979, -0.21000252712921647),
        };
        let (ans, roots) = answer(super::ellipse_roots::ellipse_roots(&e, -3.2, 3.2, &s, band));
        println!("KMDOOR {c:?} {ans} {roots:?}");
        let h = geom_brep::conic_torus_harmonics(&geom_brep::Conic::of(&e).unwrap(), Point3::new(c[0], c[1], c[2]), Vec3::new(0.5236226766304979, 0.5726988773983616, 0.6307418555518332), 300.0, 50.0);
        println!("KMDOOR noise/f_lo = {:e} m, f_hi/f_lo = {:e}", geom_brep::rounding_charge(h.terms) / h.f_per_metre_lo, h.f_per_metre_hi / h.f_per_metre_lo);
    }
}
