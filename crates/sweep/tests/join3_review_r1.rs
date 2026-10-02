//! JOIN-3 review lane r1: differential batteries over planar faces whose
//! section loops MIX lines and arcs — the shape the ring lane's closing
//! curve (`loop_winding::RunClosing`) decides. JOIN-1's R2 random battery
//! is all-planar z-prisms, so every chord it mints is straight and the
//! closing conic never runs; these batteries are shaped so it does.
//!
//! - `j3r1_mixed_pockets`: a random bulge profile (lines and arcs,
//!   convex and concave arcs, 2 to 5 sides) extruded upright against the
//!   block `[−1, 1]² × [0, 1]`, blind from the top, blind from the
//!   bottom, through, and as a buried void; ∪, ∖ and ∩ in both orders.
//! - `j3r1_tilted_through`: the same profiles tilted about a random
//!   horizontal axis, through the block, so the section loops on the
//!   block's top and bottom faces are ellipse arcs and lines.
//! - `j3r1_d_family`: the D rod (`rod_chord_at`) at flats from the
//!   near-tangent sliver through the half-disc to the near-full disc,
//!   blind from either face, through, and buried.
//!
//! Each line is one pose: `SOUND` (tiers 2 and 3′, the certificate, the
//! closed form, and a legal operand), `BAD` (any of the first four
//! fails), `NONOP` (sound but no legal operand), `UNMEAS` or `ERR`. The
//! closed forms read the tool's own measured volume, never the result's,
//! and assume the profile lies inside the block's `[−1, 1]²`: a random arc
//! of bulge 1 can leave it, and those poses print `BAD` on the volume
//! alone and are re-graded against the clipped area off-line (each line
//! prints every check and `v`, so nothing else is lost).
//! `#[ignore]`d: run with `--ignored --nocapture` and diff two trees.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::brick;
use sweep::{Extrusion, extrude};
use topo::Body;

fn tol() -> Tol {
    Tol::witness()
}

fn block() -> Body<f64> {
    brick((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol())
}

fn tool(plane: SketchPlane<f64>, chain: &[(f64, f64, f64)], h: f64) -> Option<Body<f64>> {
    let lp = bulge_loop(
        chain
            .iter()
            .map(|&(x, y, b)| (Point2::new(x, y), b))
            .collect(),
    );
    let profile = Profile::new(plane, vec![lp]).validate(tol()).ok()?;
    extrude(&profile, Extrusion::Distance(h), tol())
        .ok()
        .map(|e| e.body)
}

fn volume(b: &Body<f64>) -> Option<f64> {
    topo::mass_properties(b, tol()).ok().map(|m| m.volume)
}

fn verdict(r: Result<topo::BooleanResult<f64>, topo::BooleanError>, want: f64) -> String {
    match r {
        Err(e) => {
            let s: String = format!("{e:?}").chars().take(120).collect();
            format!("ERR {s}")
        }
        Ok(r) => match r.body() {
            None if want.abs() < 1e-9 => "SOUND empty".into(),
            None => format!("BAD empty want={want}"),
            Some(bb) => {
                let t2 = topo::validate_closed(&bb.body).is_ok();
                let t3 = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()).is_ok();
                let cert = topo::validate_geometric_certificate(&bb.body, tol()).is_ok();
                let Some(v) = volume(&bb.body) else {
                    return format!("UNMEAS t2={t2} t3p={t3} cert={cert}");
                };
                let good = (v - want).abs() < 1e-7 * want.abs().max(1.0);
                let far = brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol());
                let op = match topo::union(&bb.body, &far, tol()) {
                    Ok(_) => "op=ok".to_string(),
                    Err(e) => {
                        let s: String = format!("{e:?}").chars().take(100).collect();
                        format!("op=NONOP {s}")
                    }
                };
                let tag = match (t2 && t3 && cert && good, op == "op=ok") {
                    (true, true) => "SOUND",
                    (true, false) => "NONOP",
                    _ => "BAD",
                };
                format!("{tag} t2={t2} t3p={t3} cert={cert} v={v:.10} want={want:.10} {op}")
            }
        },
    }
}

/// Every op in both orders against the block, given the tool, its
/// volume and the overlap volume.
fn ops(tag: &str, t: &Body<f64>, vt: f64, ov: f64) {
    let b = block();
    let vb = 4.0;
    for (op, want_bt, want_tb) in [
        ("U", vb + vt - ov, vb + vt - ov),
        ("S", vb - ov, vt - ov),
        ("I", ov, ov),
    ] {
        for (order, want) in [("BT", want_bt), ("TB", want_tb)] {
            let (l, r) = if order == "BT" { (&b, t) } else { (t, &b) };
            let res = match op {
                "U" => topo::union(l, r, tol()),
                "S" => topo::subtract(l, r, tol()),
                _ => topo::intersect(l, r, tol()),
            };
            println!("POSE {tag} {op} {order} => {}", verdict(res, want));
            // The flush poses again, with every flush contact declared.
            if tag.contains(" flush") {
                use topo::flush::{declare_all, find_flush_candidates};
                let line = match find_flush_candidates(l, r, tol()) {
                    Err(e) => format!("FLUSHERR {e:?}"),
                    Ok(found) => {
                        let d = declare_all(&found);
                        let res = match op {
                            "U" => topo::union_with(l, r, &d, tol()),
                            "S" => topo::subtract_with(l, r, &d, tol()),
                            _ => topo::intersect_with(l, r, &d, tol()),
                        };
                        verdict(res, want)
                    }
                };
                println!("POSE {tag} DECL {op} {order} => {line}");
            }
        }
    }
}

struct Rng(u64);
impl Rng {
    fn next(&mut self, m: u64) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 33) % m
    }
    fn unit(&mut self) -> f64 {
        self.next(1 << 20) as f64 / (1u64 << 20) as f64
    }
}

/// A random bulge chain: `k` vertices on a circle of radius `rho`
/// about `c`, sorted by angle (counterclockwise), each side a line or an
/// arc. Two sides always carry at least one arc.
fn chain(rng: &mut Rng, rho: f64, c: f64) -> Vec<(f64, f64, f64)> {
    let k = 2 + rng.next(4) as usize;
    let (cx, cy) = ((rng.unit() * 2.0 - 1.0) * c, (rng.unit() * 2.0 - 1.0) * c);
    let r = rho * (0.5 + 0.5 * rng.unit());
    let mut ang: Vec<f64> = (0..k).map(|_| rng.unit() * core::f64::consts::TAU).collect();
    ang.sort_by(f64::total_cmp);
    let bulges = [0.0, 0.0, 0.15, 0.4, 1.0, -0.1, -0.25];
    (0..k)
        .map(|i| {
            let mut b = bulges[rng.next(bulges.len() as u64) as usize];
            if k == 2 && i == 1 && b == 0.0 {
                b = 0.4;
            }
            let a = ang[i];
            (cx + r * a.cos(), cy + r * a.sin(), b)
        })
        .collect()
}

fn seed() -> u64 {
    std::env::var("J3_SEED")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(4242)
}
fn count() -> usize {
    std::env::var("J3_N")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(100)
}

#[test]
#[ignore = "differential battery; run with --ignored"]
fn j3r1_mixed_pockets() {
    let mut rng = Rng(seed());
    for case in 0..count() {
        let ch = chain(&mut rng, 0.75, 0.15);
        for (zname, z0, h) in [
            ("top", 0.5, 1.0),
            ("bottom", -0.5, 1.0),
            ("through", -0.5, 2.0),
            ("void", 0.25, 0.5),
            ("flushtop", 0.5, 0.5),
            ("flushbot", 0.0, 0.5),
            ("flushboth", 0.0, 1.0),
        ] {
            let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
            let Some(t) = tool(plane, &ch, h) else {
                println!("POSE c{case} {zname} SKIP profile {ch:?}");
                continue;
            };
            let Some(vt) = volume(&t) else {
                println!("POSE c{case} {zname} SKIP tool unmeasured");
                continue;
            };
            let area = vt / h;
            let ov = area * ((z0 + h).min(1.0) - z0.max(0.0));
            ops(&format!("c{case} {zname} {ch:?}"), &t, vt, ov);
        }
    }
}

#[test]
#[ignore = "differential battery; run with --ignored"]
fn j3r1_tilted_through() {
    let mut rng = Rng(seed() ^ 0x5eed);
    for case in 0..count() {
        let ch = chain(&mut rng, 0.55, 0.08);
        let alpha = 0.05 + 0.4 * rng.unit();
        let phi = rng.unit() * core::f64::consts::TAU;
        let axis = Vec3::new(phi.cos(), phi.sin(), 0.0);
        let h = 4.0;
        let place = Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.5), axis, alpha)
            * Affine3::translation(Vec3::new(0.0, 0.0, -1.5));
        let Some(t) = tool(SketchPlane::new(place), &ch, h) else {
            println!("POSE t{case} SKIP profile {ch:?}");
            continue;
        };
        let Some(vt) = volume(&t) else {
            println!("POSE t{case} SKIP tool unmeasured");
            continue;
        };
        let ov = vt / h / alpha.cos();
        ops(&format!("t{case} a={alpha:.4} phi={phi:.4} {ch:?}"), &t, vt, ov);
    }
}

#[test]
#[ignore = "differential battery; run with --ignored"]
fn j3r1_d_family() {
    let r: f64 = 0.5;
    for flat in [
        -0.4999, -0.49, -0.45, -0.3, -0.1, 0.0, 0.1, 0.3, 0.45, 0.49, 0.499, 0.4999,
    ] {
        let half = (r * r - flat * flat).sqrt();
        let wall = 2.0 * (core::f64::consts::PI - f64::atan2(half, flat));
        let wb = (wall / 4.0).tan();
        let ch = [(flat, -half, 0.0), (flat, half, wb)];
        for (zname, z0, h) in [
            ("top", 0.5, 1.0),
            ("bottom", -0.5, 1.0),
            ("through", -0.5, 2.0),
            ("void", 0.25, 0.5),
            ("flushtop", 0.5, 0.5),
            ("flushbot", 0.0, 0.5),
            ("flushboth", 0.0, 1.0),
        ] {
            let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
            let Some(t) = tool(plane, &ch, h) else {
                println!("POSE d{flat} {zname} SKIP profile");
                continue;
            };
            let Some(vt) = volume(&t) else {
                println!("POSE d{flat} {zname} SKIP tool unmeasured");
                continue;
            };
            let ov = vt / h * ((z0 + h).min(1.0) - z0.max(0.0));
            ops(&format!("d{flat} {zname}"), &t, vt, ov);
        }
        // Tilted through-holes: ellipse arcs on both caps.
        for alpha in [0.2, 0.6] {
            let place = Affine3::rotation_about_axis(
                Point3::new(0.0, 0.0, 0.5),
                Vec3::new(0.6, 0.8, 0.0),
                alpha,
            ) * Affine3::translation(Vec3::new(0.0, 0.0, -1.5));
            let Some(t) = tool(SketchPlane::new(place), &ch, 4.0) else {
                println!("POSE d{flat} tilt{alpha} SKIP profile");
                continue;
            };
            let Some(vt) = volume(&t) else {
                println!("POSE d{flat} tilt{alpha} SKIP tool unmeasured");
                continue;
            };
            ops(&format!("d{flat} tilt{alpha}"), &t, vt, vt / 4.0 / alpha.cos());
        }
    }
}
