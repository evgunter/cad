//! Review probes for PR #3845 (reach-dual3845-r2). Oracle: closed form
//! (πr²h), never the kernel. Run with `--nocapture`; rows PRINT their
//! outcome (a probe, not a pinned suite) and assert only wrong bodies.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, SketchPlane, circle_split};
use sweep::test_support::{extruded, sketch_at, sketch_from_axes};
use topo::{Body, BooleanDeclarations, BooleanError, BooleanResult, SolidContainment};

fn tol() -> Tol {
    Tol::witness()
}
fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// A frame: origin, u, v (unit, orthogonal); normal = u × v.
#[derive(Clone, Copy)]
struct Fr {
    o: Point3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
}
impl Fr {
    fn world() -> Self {
        Fr {
            o: Point3::new(0.0, 0.0, 0.0),
            u: Vec3::new(1.0, 0.0, 0.0),
            v: Vec3::new(0.0, 1.0, 0.0),
        }
    }
    fn tilted() -> Self {
        // a generic rotation: u, v orthonormal, not axis aligned
        let u = Vec3::new(0.6, 0.48, 0.64);
        let n0 = Vec3::new(-0.8, 0.36, 0.48);
        let v = n0.cross(u);
        Fr {
            o: Point3::new(0.3, -1.7, 2.2),
            u,
            v,
        }
    }
    fn n(&self) -> Vec3<f64> {
        self.u.cross(self.v)
    }
    fn plane(&self, z0: f64) -> SketchPlane<f64> {
        if self.u.x == 1.0 && self.v.y == 1.0 {
            sketch_at(self.o.z + z0)
        } else {
            sketch_from_axes(self.o + self.n() * z0, self.u, self.v, tol())
        }
    }
    fn at(&self, x: f64, y: f64, z: f64) -> Point3<f64> {
        self.o + self.u * x + self.v * y + self.n() * z
    }
}

/// A rim of radius r whose vertices sit at the given angles (ascending,
/// within one turn of the first), each arc bulged to stay on the circle.
fn rim(r: f64, angles: &[f64]) -> ProfileLoop<f64> {
    let n = angles.len();
    let chain = (0..n)
        .map(|k| {
            let a0 = angles[k];
            let a1 = if k + 1 < n { angles[k + 1] } else { angles[0] + TAU };
            let p = Point2::new(r * a0.cos(), r * a0.sin());
            (p, ((a1 - a0) / 4.0).tan())
        })
        .collect();
    bulge_loop(chain)
}

fn split(r: f64, n: usize, phase: f64) -> ProfileLoop<f64> {
    circle_split(Point2::new(0.0, 0.0), r, n, phase, tol())
        .unwrap()
        .into()
}

fn declared(a: &Body<f64>, b: &Body<f64>) -> BooleanDeclarations {
    let found = topo::flush::find_flush_candidates(a, b, tol()).expect("decide");
    topo::flush::declare_all(&found)
}

fn vol(b: &Body<f64>) -> f64 {
    topo::mass_properties(b, tol()).unwrap().volume
}

type Out = Result<BooleanResult<f64>, BooleanError>;

/// Outcome: Some(body) if it built. Prints the line. Asserts the closed
/// form volume and the oracle point probes when it built.
fn check(
    label: &str,
    out: Out,
    expect: f64,
    inside: &dyn Fn(Point3<f64>) -> bool,
    samples: &[Point3<f64>],
) -> Option<Body<f64>> {
    match out {
        Ok(BooleanResult::Body(bb)) => {
            let v = vol(&bb.body);
            let t3 = topo::validate_geometric(&bb.body, tol());
            let t3p = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol());
            let mut bad = 0;
            for &q in samples {
                match topo::point_in_solid(&bb.body, q, band(), tol()) {
                    Ok(SolidContainment::In) if inside(q) => {}
                    Ok(SolidContainment::Out) if !inside(q) => {}
                    Ok(c) => {
                        bad += 1;
                        println!("    pis MISMATCH {q:?}: {c:?} oracle in={}", inside(q));
                    }
                    Err(e) => println!("    pis err {q:?}: {e:?}"),
                }
            }
            let rel = (v - expect).abs() / expect.abs().max(1e-300);
            println!(
                "{label}: BUILT faces={} vol={v:.15e} oracle={expect:.15e} rel={rel:.1e} t3={} t3'={} pis_bad={bad}/{}",
                bb.body.faces().count(),
                t3.is_ok(),
                t3p.is_ok(),
                samples.len()
            );
            assert!(rel <= 1e-9, "WRONG BODY {label}: vol {v} vs {expect}");
            assert_eq!(bad, 0, "WRONG BODY {label}: point_in_solid");
            Some(bb.body)
        }
        Ok(other) => {
            println!("{label}: non-body {other:?}");
            None
        }
        Err(e) => {
            let s = format!("{e:?}");
            println!("{label}: REFUSED {}", &s[..s.len().min(160)]);
            None
        }
    }
}

/// Points on a lattice in local frame coordinates over [-R,R]²×[z0,z1].
fn lattice(fr: Fr, rr: f64, z0: f64, z1: f64) -> Vec<Point3<f64>> {
    let mut v = Vec::new();
    for i in 0..7 {
        for j in 0..7 {
            for k in 0..5 {
                let x = -rr + 2.0 * rr * (i as f64 + 0.37) / 7.0;
                let y = -rr + 2.0 * rr * (j as f64 + 0.61) / 7.0;
                let z = z0 + (z1 - z0) * (k as f64 + 0.43) / 5.0;
                v.push(fr.at(x, y, z));
            }
        }
    }
    v
}

fn local(fr: Fr, q: Point3<f64>) -> (f64, f64, f64) {
    let d = q - fr.o;
    (d.dot(fr.u), d.dot(fr.v), d.dot(fr.n()))
}

/// Two tubes (outer r 2, inner r 1, each wall split in two at `phase`),
/// stacked; the inner and outer seam sites lie on one radial.
#[test]
fn probe_tube_stack() {
    let fr = Fr::world();
    for (theta_l, theta_u) in [(0.0, 0.0), (0.0, 0.7), (0.0, PI), (0.3, 0.3)] {
        let tube = |z0: f64, ph: f64| {
            extruded(
                fr.plane(z0),
                vec![split(2.0, 2, ph), split(1.0, 2, ph)],
                1.0,
                tol(),
            )
        };
        let (a, b) = (tube(0.0, theta_l), tube(1.0, theta_u));
        println!("tube A vol {} (oracle {})", vol(&a), 3.0 * PI);
        let inside = |q: Point3<f64>| {
            let (x, y, z) = local(fr, q);
            let r = (x * x + y * y).sqrt();
            (1.0..2.0).contains(&r) && (0.0..2.0).contains(&z)
        };
        let s = lattice(fr, 2.3, -0.2, 2.2);
        for (op, o) in [("A∪B", 0), ("B∪A", 1)] {
            let out = if o == 0 {
                topo::union_with(&a, &b, &declared(&a, &b), tol())
            } else {
                topo::union_with(&b, &a, &declared(&b, &a), tol())
            };
            check(
                &format!("tube θ=({theta_l},{theta_u}) {op}"),
                out,
                6.0 * PI,
                &inside,
                &s,
            );
        }
    }
}

/// Rod stacks, widened: poses, piece counts, unequal arcs, scales, frames.
fn rod_case(label: &str, fr: Fr, s: f64, lower: ProfileLoop<f64>, upper: ProfileLoop<f64>, r: f64) {
    let a = extruded(fr.plane(0.0), vec![lower], s, tol());
    let b = extruded(fr.plane(s), vec![upper], s, tol());
    let inside = move |q: Point3<f64>| {
        let (x, y, z) = local(fr, q);
        (x * x + y * y).sqrt() < r && (0.0..2.0 * s).contains(&z)
    };
    let smp = lattice(fr, 1.2 * r, -0.1 * s, 2.1 * s);
    let o = 2.0 * PI * r * r * s;
    let ab = check(
        &format!("{label} A∪B"),
        topo::union_with(&a, &b, &declared(&a, &b), tol()),
        o,
        &inside,
        &smp,
    );
    check(
        &format!("{label} B∪A"),
        topo::union_with(&b, &a, &declared(&b, &a), tol()),
        o,
        &inside,
        &smp,
    );
    if let Some(st) = ab {
        // reuse as operand: a third rod (2 pieces at phase 0.4) and a fourth
        let c = extruded(fr.plane(2.0 * s), vec![split_s(r * 1.0, 2, 0.4, s / s)], s, tol());
        let in3 = move |q: Point3<f64>| {
            let (x, y, z) = local(fr, q);
            (x * x + y * y).sqrt() < r && (0.0..3.0 * s).contains(&z)
        };
        let smp3 = lattice(fr, 1.2 * r, -0.1 * s, 3.1 * s);
        if let Some(st3) = check(
            &format!("{label} +3rd"),
            topo::union_with(&st, &c, &declared(&st, &c), tol()),
            1.5 * o,
            &in3,
            &smp3,
        ) {
            let d = extruded(fr.plane(3.0 * s), vec![split_s(r, 4, 0.0, 1.0)], s, tol());
            let in4 = move |q: Point3<f64>| {
                let (x, y, z) = local(fr, q);
                (x * x + y * y).sqrt() < r && (0.0..4.0 * s).contains(&z)
            };
            let smp4 = lattice(fr, 1.2 * r, -0.1 * s, 4.1 * s);
            check(
                &format!("{label} +4th"),
                topo::union_with(&d, &st3, &declared(&d, &st3), tol()),
                2.0 * o,
                &in4,
                &smp4,
            );
        }
    }
}

fn split_s(r: f64, n: usize, phase: f64, _k: f64) -> ProfileLoop<f64> {
    split(r, n, phase)
}

#[test]
fn probe_rod_poses() {
    let thetas = [0.0, 1e-7, 0.7, FRAC_PI_2, PI - 1e-7, PI];
    for fr_name in ["world", "tilted"] {
        let fr = if fr_name == "world" { Fr::world() } else { Fr::tilted() };
        for s in [1e-3, 1.0, 1e3] {
            for n in [2usize, 3, 4] {
                for &th in &thetas {
                    if fr_name == "tilted" && n != 2 {
                        continue;
                    }
                    if s != 1.0 && (n != 2 || fr_name == "tilted") {
                        continue;
                    }
                    rod_case(
                        &format!("rod {fr_name} s={s} n={n} θ={th}"),
                        fr,
                        s,
                        split(s, n, 0.0),
                        split(s, n, th),
                        s,
                    );
                }
            }
        }
    }
}

/// Unequal arcs: lower seams at 0 and π+δ, upper at the same or turned.
#[test]
fn probe_unequal_arcs() {
    let fr = Fr::world();
    for d in [1e-7, 1e-3, 0.3, 1.0] {
        let lo = || rim(1.0, &[0.0, PI + d]);
        rod_case(&format!("unequal δ={d} aligned"), fr, 1.0, lo(), lo(), 1.0);
        rod_case(
            &format!("unequal δ={d} mirrored"),
            fr,
            1.0,
            lo(),
            rim(1.0, &[0.0, PI - d]),
            1.0,
        );
        // three unequal arcs: 0, 2.0, 2.0+π+... (one arc > π)
        rod_case(
            &format!("3-arc long δ={d}"),
            fr,
            1.0,
            rim(1.0, &[0.0, 0.5, PI + d]),
            rim(1.0, &[0.0, 0.5, PI + d]),
            1.0,
        );
        rod_case(
            &format!("3-arc split vs 2 δ={d}"),
            fr,
            1.0,
            rim(1.0, &[0.0, 0.5, PI + d]),
            rim(1.0, &[0.0, PI + d]),
            1.0,
        );
    }
}

/// Unequal radii: not coincident walls; the oracle is still exact.
#[test]
fn probe_unequal_radii() {
    let fr = Fr::world();
    for (r1, r2) in [(1.0, 1.0 + 1e-7), (1.0, 1.5), (1.5, 1.0)] {
        let a = extruded(fr.plane(0.0), vec![split(r1, 2, 0.0)], 1.0, tol());
        let b = extruded(fr.plane(1.0), vec![split(r2, 2, 0.0)], 1.0, tol());
        let o = PI * (r1 * r1 + r2 * r2);
        let inside = move |q: Point3<f64>| {
            let (x, y, z) = local(fr, q);
            let r = (x * x + y * y).sqrt();
            (r < r1 && (0.0..1.0).contains(&z)) || (r < r2 && (1.0..2.0).contains(&z))
        };
        let smp = lattice(fr, 1.8, -0.1, 2.1);
        check(
            &format!("radii {r1}/{r2} A∪B"),
            topo::union_with(&a, &b, &declared(&a, &b), tol()),
            o,
            &inside,
            &smp,
        );
    }
}

/// Half-tubes (C sections: outer arc r 2 over the top, inner arc r 1)
/// stacked: the mating half-annulus has two half-turn arcs on two
/// coaxial circles and two radial lines. The arc pass ties (1,0)'s
/// germ between (−1,0) and (−2,0) at π.
#[test]
fn probe_half_tube_stack() {
    let fr = Fr::world();
    let c = |sgn: f64| {
        bulge_loop(vec![
            (Point2::new(2.0 * sgn, 0.0), 1.0),
            (Point2::new(-2.0 * sgn, 0.0), 0.0),
            (Point2::new(-1.0 * sgn, 0.0), -1.0),
            (Point2::new(1.0 * sgn, 0.0), 0.0),
        ])
    };
    for (sl, su) in [(1.0, 1.0), (-1.0, -1.0)] {
        let a = extruded(fr.plane(0.0), vec![c(sl)], 1.0, tol());
        let b = extruded(fr.plane(1.0), vec![c(su)], 1.0, tol());
        println!("half tube vol {} oracle {}", vol(&a), 1.5 * PI);
        let inside = move |q: Point3<f64>| {
            let (x, y, z) = local(fr, q);
            let r = (x * x + y * y).sqrt();
            (1.0..2.0).contains(&r) && y * sl > 0.0 && (0.0..2.0).contains(&z)
        };
        let s = lattice(fr, 2.3, -0.2, 2.2);
        check(&format!("half tube {sl} A∪B"), topo::union_with(&a, &b, &declared(&a, &b), tol()), 3.0 * PI, &inside, &s);
        check(&format!("half tube {sl} B∪A"), topo::union_with(&b, &a, &declared(&b, &a), tol()), 3.0 * PI, &inside, &s);
    }
}

/// Two rods stacked on a third: A = two disjoint rods (z 0..1, 2..3)
/// as one operand; B = the rod z 1..2 between. Two coaxial seam circles
/// whose sites share angles.
#[test]
fn probe_two_seam_circles() {
    let fr = Fr::world();
    let r = |z0: f64, ph: f64| extruded(fr.plane(z0), vec![split(1.0, 2, ph)], 1.0, tol());
    let (lo, hi) = (r(0.0, 0.0), r(2.0, 0.0));
    let a = match topo::union_with(&lo, &hi, &declared(&lo, &hi), tol()) {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => {
            println!("disjoint union: {other:?}");
            return;
        }
    };
    println!("A solids vol {}", vol(&a));
    for th in [0.0, 0.7, PI] {
        let b = r(1.0, th);
        let inside = move |q: Point3<f64>| {
            let (x, y, z) = local(fr, q);
            (x * x + y * y).sqrt() < 1.0 && (0.0..3.0).contains(&z)
        };
        let s = lattice(fr, 1.2, -0.1, 3.1);
        check(&format!("two circles θ={th} A∪B"), topo::union_with(&a, &b, &declared(&a, &b), tol()), 3.0 * PI, &inside, &s);
        check(&format!("two circles θ={th} B∪A"), topo::union_with(&b, &a, &declared(&b, &a), tol()), 3.0 * PI, &inside, &s);
    }
}
