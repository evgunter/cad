//! Review R1 probes of PR 5422 on mixed planar and curved bodies:
//! `point_in_solid` against the analytic truth near the seams. Evidence,
//! not pins; uses only API present on the merge base too.

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Band, Point2, Point3, Tol};
use sweep::test_support as ts;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

fn sd_box(p: [f64; 3], lo: [f64; 3], hi: [f64; 3]) -> f64 {
    let q: Vec<f64> = (0..3).map(|i| (p[i] - (lo[i] + hi[i]) / 2.0).abs() - (hi[i] - lo[i]) / 2.0).collect();
    let out = q.iter().map(|x| x.max(0.0).powi(2)).sum::<f64>().sqrt();
    out + q[0].max(q[1]).max(q[2]).min(0.0)
}

fn sd_cyl(p: [f64; 3], c: (f64, f64), r: f64, z0: f64, z1: f64) -> f64 {
    let dx = ((p[0] - c.0).powi(2) + (p[1] - c.1).powi(2)).sqrt() - r;
    let dy = (p[2] - (z0 + z1) / 2.0).abs() - (z1 - z0) / 2.0;
    dx.max(dy).min(0.0) + (dx.max(0.0).powi(2) + dy.max(0.0).powi(2)).sqrt()
}

fn sd_rrect(p: [f64; 3], r: f64, h: f64) -> f64 {
    let q = [(p[0] - 0.5).abs() - (0.5 - r), (p[1] - 0.5).abs() - (0.5 - r)];
    let d2 = (q[0].max(0.0).powi(2) + q[1].max(0.0).powi(2)).sqrt() + q[0].max(q[1]).min(0.0) - r;
    let w = [d2, (p[2] - h / 2.0).abs() - h / 2.0];
    w[0].max(w[1]).min(0.0) + (w[0].max(0.0).powi(2) + w[1].max(0.0).powi(2)).sqrt()
}

/// A random point on a cube edge of `[0,1]^3` or a circle `(c, r, z)`.
fn feature(rng: &mut Lcg, circles: &[((f64, f64), f64, f64)], lines: &[([f64; 3], [f64; 3])]) -> [f64; 3] {
    let pick = rng.next();
    if !circles.is_empty() && pick < 0.5 {
        let (c, r, z) = circles[(rng.next() * circles.len() as f64) as usize % circles.len()];
        let a = rng.next() * core::f64::consts::TAU;
        [c.0 + r * a.cos(), c.1 + r * a.sin(), z]
    } else {
        let (a, b) = lines[(rng.next() * lines.len() as f64) as usize % lines.len()];
        let s = if rng.next() < 0.3 { if rng.next() < 0.5 { 0.0 } else { 1.0 } } else { rng.next() };
        [a[0] + s * (b[0] - a[0]), a[1] + s * (b[1] - a[1]), a[2] + s * (b[2] - a[2])]
    }
}

fn cube_edges(h: f64) -> Vec<([f64; 3], [f64; 3])> {
    let mut v = Vec::new();
    for &(a, b) in &[((0.0, 0.0), (1.0, 0.0)), ((1.0, 0.0), (1.0, 1.0)), ((1.0, 1.0), (0.0, 1.0)), ((0.0, 1.0), (0.0, 0.0))] {
        let (a, b): ((f64, f64), (f64, f64)) = (a, b);
        v.push(([a.0, a.1, 0.0], [b.0, b.1, 0.0]));
        v.push(([a.0, a.1, h], [b.0, b.1, h]));
        v.push(([a.0, a.1, 0.0], [a.0, a.1, h]));
    }
    v
}

#[test]
#[ignore = "probe: run by hand"]
fn mixed_bodies_point_in_solid_against_truth() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let ke = tol.k() * tol.eps();
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let rr = 0.2;
    let rrect = || {
        let v = |x: f64, y: f64, b: f64| (Point2::new(x, y), b);
        ts::extruded(
            profile::SketchPlane::xy(),
            vec![profile::test_support::bulge_loop(vec![
                v(rr, 0.0, 0.0),
                v(1.0 - rr, 0.0, q),
                v(1.0, rr, 0.0),
                v(1.0, 1.0 - rr, q),
                v(1.0 - rr, 1.0, 0.0),
                v(rr, 1.0, q),
                v(0.0, 1.0 - rr, 0.0),
                v(0.0, rr, q),
            ])],
            0.6,
            tol,
        )
    };
    type Sd = Box<dyn Fn([f64; 3]) -> (bool, f64)>;
    let cases: Vec<(&str, Box<dyn Fn() -> topo::Body<f64>>, Sd, Vec<((f64, f64), f64, f64)>, Vec<([f64; 3], [f64; 3])>)> = vec![
        (
            "boss",
            Box::new(|| ts::boss_of_arcs(4, 1.0, 0.3, 0.5, 1.0, tol)),
            Box::new(|p| {
                let (a, b) = (sd_box(p, [0.0; 3], [1.0; 3]), sd_cyl(p, (0.5, 0.5), 0.3, 0.5, 1.5));
                (a < 0.0 || b < 0.0, a.abs().min(b.abs()))
            }),
            vec![((0.5, 0.5), 0.3, 1.0), ((0.5, 0.5), 0.3, 1.5)],
            cube_edges(1.0),
        ),
        (
            "pocket",
            Box::new(|| ts::pocket_of_arcs(4, 1.0, 0.3, 0.4, tol)),
            Box::new(|p| {
                let (a, b) = (sd_box(p, [0.0; 3], [1.0; 3]), sd_cyl(p, (0.5, 0.5), 0.3, 0.4, 1.4));
                (a < 0.0 && b > 0.0, a.abs().min(b.abs()))
            }),
            vec![((0.5, 0.5), 0.3, 1.0), ((0.5, 0.5), 0.3, 0.4)],
            cube_edges(1.0),
        ),
        (
            "bored",
            Box::new(|| ts::bored_block_of_arcs(4, 1.0, 1.0, 0.3, tol)),
            Box::new(|p| {
                let (a, b) = (sd_box(p, [0.0; 3], [1.0; 3]), sd_cyl(p, (0.5, 0.5), 0.3, -1.0, 2.0));
                (a < 0.0 && b > 0.0, a.abs().min(b.abs()))
            }),
            vec![((0.5, 0.5), 0.3, 1.0), ((0.5, 0.5), 0.3, 0.0)],
            cube_edges(1.0),
        ),
        (
            "rounded-rect",
            Box::new(rrect),
            Box::new(move |p| {
                let a = sd_rrect(p, rr, 0.6);
                (a < 0.0, a.abs())
            }),
            vec![
                ((rr, rr), rr, 0.0), ((rr, rr), rr, 0.6), ((1.0 - rr, rr), rr, 0.0), ((1.0 - rr, rr), rr, 0.6),
                ((rr, 1.0 - rr), rr, 0.0), ((rr, 1.0 - rr), rr, 0.6), ((1.0 - rr, 1.0 - rr), rr, 0.0), ((1.0 - rr, 1.0 - rr), rr, 0.6),
            ],
            vec![
                ([rr, 0.0, 0.0], [rr, 0.0, 0.6]), ([1.0 - rr, 0.0, 0.0], [1.0 - rr, 0.0, 0.6]),
                ([1.0, rr, 0.0], [1.0, rr, 0.6]), ([1.0, 1.0 - rr, 0.0], [1.0, 1.0 - rr, 0.6]),
                ([rr, 0.0, 0.6], [1.0 - rr, 0.0, 0.6]), ([1.0, rr, 0.6], [1.0, 1.0 - rr, 0.6]),
                ([rr, 0.0, 0.0], [1.0 - rr, 0.0, 0.0]), ([0.0, rr, 0.6], [0.0, 1.0 - rr, 0.6]),
            ],
        ),
    ];
    for (name, build, truth, circles, lines) in cases {
        let built = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| build()));
        let Ok(body) = built else {
            println!("R1MIX eps={:e} {name}: BUILD PANIC", tol.eps());
            continue;
        };
        for inverted in [false, true] {
            let body = if inverted { body.revert().expect("reverts") } else { body.clone() };
            let mut rng = Lcg(0x5422);
            let (mut read, mut refused, mut wrong, mut skipped) = (0, 0, 0, 0);
            let mut ex = Vec::new();
            let mut why: std::collections::BTreeMap<String, usize> = Default::default();
            for _ in 0..300 {
                let f = feature(&mut rng, &circles, &lines);
                let scale = [2.5, 4.0, 8.0, 20.0, 100.0, 1e4][(rng.next() * 6.0) as usize % 6] * ke;
                let off = [rng.next() - 0.5, rng.next() - 0.5, rng.next() - 0.5];
                let on = (off[0] * off[0] + off[1] * off[1] + off[2] * off[2]).sqrt();
                let p = [f[0] + off[0] / on * scale, f[1] + off[1] / on * scale, f[2] + off[2] / on * scale];
                let (inside, dist) = truth(p);
                if dist < 2.0 * ke {
                    skipped += 1;
                    continue;
                }
                let want = inside != inverted;
                match topo::point_in_solid(&body, Point3::new(p[0], p[1], p[2]), band, tol) {
                    Ok(topo::SolidContainment::In) | Ok(topo::SolidContainment::Out) => {
                        read += 1;
                        let said = matches!(
                            topo::point_in_solid(&body, Point3::new(p[0], p[1], p[2]), band, tol),
                            Ok(topo::SolidContainment::In)
                        );
                        if said != want {
                            wrong += 1;
                            if ex.len() < 3 {
                                ex.push(format!("p={p:?} d={dist:e} want_in={want}"));
                            }
                        }
                    }
                    other => {
                        refused += 1;
                        let k: String = format!("{other:?}").chars().take(40).collect();
                        *why.entry(k).or_default() += 1;
                    }
                }
            }
            println!(
                "R1MIX eps={:e} {name} inv={inverted}: read {read} refused {refused} skipped {skipped} WRONG {wrong} {ex:?} {why:?}",
                tol.eps()
            );
        }
    }
}
