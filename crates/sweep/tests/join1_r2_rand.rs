//! JOIN-1 review (lane r2): randomized z-prism pairs on a coarse grid
//! (shared corners, collinear vertical edges). Every body that builds
//! must hold the exact oracle `area(P ∩ Q) · z-overlap`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::test_support::finished;
use sweep::{Extrusion, extrude};
use topo::AtRestBody;

fn tol() -> Tol {
    Tol::witness()
}

fn zprism(pts: &[(f64, f64)], z: (f64, f64)) -> AtRestBody<f64> {
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z.0)));
    let lp = bulge_loop(pts.iter().map(|&(a, b)| (Point2::new(a, b), 0.0)).collect());
    let p = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    let prism = extrude(
        &p,
        Extrusion::Distance {
            depth: z.1 - z.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    finished("the prism", prism, tol())
}

fn hull(mut p: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    p.sort_by(|a, b| a.partial_cmp(b).unwrap());
    p.dedup();
    if p.len() < 3 {
        return p;
    }
    let cross = |o: (f64, f64), a: (f64, f64), b: (f64, f64)| {
        (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
    };
    let mut lo: Vec<(f64, f64)> = Vec::new();
    for &q in &p {
        while lo.len() >= 2 && cross(lo[lo.len() - 2], lo[lo.len() - 1], q) <= 0.0 {
            lo.pop();
        }
        lo.push(q);
    }
    let mut hi: Vec<(f64, f64)> = Vec::new();
    for &q in p.iter().rev() {
        while hi.len() >= 2 && cross(hi[hi.len() - 2], hi[hi.len() - 1], q) <= 0.0 {
            hi.pop();
        }
        hi.push(q);
    }
    lo.pop();
    hi.pop();
    lo.extend(hi);
    lo
}

fn clip_area(subject: &[(f64, f64)], clipper: &[(f64, f64)]) -> f64 {
    let mut out = subject.to_vec();
    for i in 0..clipper.len() {
        let (a, b) = (clipper[i], clipper[(i + 1) % clipper.len()]);
        let side = |p: (f64, f64)| (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
        let inp = out.clone();
        out.clear();
        for j in 0..inp.len() {
            let (p, q) = (inp[(j + inp.len() - 1) % inp.len()], inp[j]);
            let (sp, sq) = (side(p), side(q));
            if sq >= 0.0 {
                if sp < 0.0 {
                    let t = sp / (sp - sq);
                    out.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)));
                }
                out.push(q);
            } else if sp >= 0.0 {
                let t = sp / (sp - sq);
                out.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)));
            }
        }
        if out.is_empty() {
            return 0.0;
        }
    }
    let n = out.len();
    0.5 * (0..n)
        .map(|i| out[i].0 * out[(i + 1) % n].1 - out[(i + 1) % n].0 * out[i].1)
        .sum::<f64>()
}

#[test]
fn r2_random_zprism_pairs() {
    let mut s: u64 = std::env::var("R2_SEED")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(12345);
    let n: usize = std::env::var("R2_N")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(150);
    let mut rnd = |m: u64| {
        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (s >> 33) % m
    };
    let mut wrong = Vec::new();
    let mut built = Vec::new();
    let mut refusals = std::collections::BTreeMap::<String, usize>::new();
    for case in 0..n {
        let mut poly = || loop {
            let k = 3 + rnd(3) as usize;
            let pts: Vec<(f64, f64)> = (0..k)
                .map(|_| (rnd(5) as f64 * 0.5, rnd(5) as f64 * 0.5))
                .collect();
            let h = hull(pts);
            if h.len() >= 3 {
                return h;
            }
        };
        let (p, q) = (poly(), poly());
        let mut zr = || loop {
            let (a, b) = (rnd(4) as f64, rnd(4) as f64);
            if a < b {
                return (a, b);
            }
        };
        let (zp, zq) = (zr(), zr());
        let (ap, aq) = (clip_area(&p, &p), clip_area(&q, &q));
        let ov = clip_area(&p, &q) * (zp.1.min(zq.1) - zp.0.max(zq.0)).max(0.0);
        let (vp, vq) = (ap * (zp.1 - zp.0), aq * (zq.1 - zq.0));
        let (bp, bq) = (zprism(&p, zp), zprism(&q, zq));
        for (op, r, want) in [
            ("∪", topo::union(&bp, &bq, tol()), vp + vq - ov),
            ("∖", topo::subtract(&bp, &bq, tol()), vp - ov),
            ("∩", topo::intersect(&bp, &bq, tol()), ov),
        ] {
            let what = format!("case {case} {op} P={p:?} z{zp:?} Q={q:?} z{zq:?}");
            match r {
                Err(e) => {
                    let k: String = format!("{:?}", e.kind()).chars().take(70).collect();
                    *refusals.entry(k).or_default() += 1;
                }
                Ok(r) => {
                    let Some(bb) = r.body() else {
                        built.push(format!("case {case} {op} empty"));
                        if want.abs() > 1e-9 {
                            wrong.push(format!("{what}: empty against {want}"));
                        }
                        continue;
                    };
                    built.push(format!("case {case} {op}"));
                    let mut bad = Vec::new();
                    if let Err(e) = topo::validate_closed(&bb.body) {
                        bad.push(format!("tier 2 {e:?}"));
                    }
                    if let Err(e) = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()) {
                        bad.push(format!("tier 3′ {e:?}"));
                    }
                    match topo::mass_properties(&bb.body, tol()) {
                        Ok(m) if (m.volume - want).abs() <= 1e-9 => {}
                        Ok(m) => bad.push(format!("volume {} against {want}", m.volume)),
                        Err(e) => bad.push(format!("volume unmeasured {e:?}")),
                    }
                    if !bad.is_empty() {
                        wrong.push(format!("{what}: {}", bad.join("; ")));
                    }
                }
            }
        }
    }
    println!("[r2-rand] built {} : {built:?}", built.len());
    println!("[r2-rand] refusals {refusals:#?}");
    assert!(wrong.is_empty(), "WRONG BODIES:\n{}", wrong.join("\n"));
}

/// Seed 2, case 390: at JOIN-1's head the union builds a body that
/// fails tier 3′ (`ScaffoldAtRest`); at its base every op refused.
#[test]
fn r2_rand_seed2_case390() {
    let p = [(0.0, 1.5), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
    let q = [(0.5, 1.5), (1.0, 0.5), (1.5, 2.0), (1.0, 2.0)];
    let (zp, zq) = ((1.0, 3.0), (0.0, 2.0));
    let ov = clip_area(&p, &q) * 1.0;
    let (vp, vq) = (clip_area(&p, &p) * 2.0, clip_area(&q, &q) * 2.0);
    let (bp, bq) = (zprism(&p, zp), zprism(&q, zq));
    let mut bad = Vec::new();
    for (ord, x, y, vx, vy) in [("P·Q", &bp, &bq, vp, vq), ("Q·P", &bq, &bp, vq, vp)] {
        for (op, r, want) in [
            ("∪", topo::union(x, y, tol()), vx + vy - ov),
            ("∖", topo::subtract(x, y, tol()), vx - ov),
            ("∩", topo::intersect(x, y, tol()), ov),
        ] {
            match r {
                Err(e) => println!("[r2-390] {ord} {op}: REFUSES {e:?}"),
                Ok(r) => match r.body() {
                    None => println!("[r2-390] {ord} {op}: EMPTY (want {want})"),
                    Some(bb) => {
                        let t3 = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol());
                        println!(
                            "[r2-390] {ord} {op}: BUILDS vol {:?} want {want}; t2 {:?}; t3' {:?}; cert {:?}",
                            topo::mass_properties(&bb.body, tol()).map(|m| m.volume),
                            topo::validate_closed(&bb.body),
                            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()),
                            topo::validate_geometric_certificate(&bb.body, tol()).map(|_| ()),
                        );
                        bad.extend(t3.err().map(|e| format!("{ord} {op}: tier 3′ {e:?}")));
                    }
                },
            }
        }
    }
    assert!(bad.is_empty(), "a built body fails tier 3′: {bad:#?}");
}
