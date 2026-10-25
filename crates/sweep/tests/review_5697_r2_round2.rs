//! Review R2 of PR 5697, the fix pass (head only): every crossing of a
//! ray read against an analytic truth by a checker written apart from the
//! PR's pin, over the PR's corpus and a narrow flat between two fillets
//! (a face a band or two wide), with rays shallow across it, in four face
//! orders. Evidence, not pins.

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use core::f64::consts::PI;

use geom_core::{Band, Point2, Point3, Tol, Vec3};
use topo::Body;

use super::curved_edge_corpus::{Truth, corpus, in_bulge_loop, robust};
use super::review_5697_r2_corpus::Rng;

/// The narrow flat: a prism whose top flat at `y = 1` is `w` wide between
/// two quarter-round fillets of radius 0.2.
fn flat_profile(w: f64) -> Vec<(Point2<f64>, f64)> {
    let q = (PI / 8.0).tan();
    vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(1.0, 0.8), q),
        (Point2::new(0.8, 1.0), 0.0),
        (Point2::new(0.8 - w, 1.0), q),
        (Point2::new(0.6 - w, 0.8), 0.0),
        (Point2::new(0.0, 0.8), 0.0),
    ]
}

/// What one ray reads wrong, if anything: every cluster's count against
/// the truth's change across it, in four face orders that must agree.
fn check(body: &Body<f64>, face: topo::FaceKey, q: Point3<f64>, d: Vec3<f64>, truth: &Truth, inside_out: bool, k_eps: f64, band: Band) -> Result<bool, String> {
    let mut reads = Vec::new();
    for order in [(0, false), (1, false), (2, true), (5, true)] {
        reads.push(topo::test_support::ray_counts(body, face, (q, d), band, order));
    }
    let key = |r: &Result<Vec<(f64, f64, i32, bool)>, String>| match r {
        Ok(h) => {
            let mut h: Vec<(i64, i64, i32, bool)> = h.iter().map(|&(a, b, c, s)| ((a / k_eps).round() as i64, (b / k_eps).round() as i64, c, s)).collect();
            h.sort();
            format!("{h:?}")
        }
        Err(e) => format!("Err {}", e.split(['(', ' ', '{']).next().unwrap_or("")),
    };
    let k0 = key(&reads[0]);
    for r in &reads[1..] {
        if key(r) != k0 {
            return Err(format!("face orders disagree: {k0} vs {}", key(r)));
        }
    }
    let Ok(hits) = &reads[0] else { return Ok(false) };
    let mut hits = hits.clone();
    hits.sort_by(|a, b| a.0.total_cmp(&b.0));
    // Clusters: spans within 3Kε of each other.
    let mut cl: Vec<(f64, f64, i32, bool)> = Vec::new();
    for (lo, hi, c, s) in hits {
        match cl.last_mut() {
            Some(l) if lo <= l.1 + 3.0 * k_eps => *l = (l.0, l.1.max(hi), l.2 + c, l.3 || s),
            _ => cl.push((lo, hi, c, s)),
        }
    }
    let mat = |t: f64| robust(truth, q + d * t, 1.05 * k_eps).map(|m| m != inside_out);
    let mut state = mat(0.0);
    let mut prev = 0.0f64;
    let mut checked = false;
    for &(lo, hi, c, s) in &cl {
        if s {
            return Ok(checked);
        }
        let before = mat(0.5 * (prev + lo));
        if let (Some(a), Some(b)) = (state, before)
            && a != b
        {
            return Err(format!("{cl:?}: counted material {a} before {lo:e}, truth {b}"));
        }
        let st = before.or(state);
        state = match (st, c) {
            (Some(m), 0) => Some(m),
            (Some(true), 1) => Some(false),
            (Some(false), -1) => Some(true),
            (Some(m), _) => return Err(format!("{cl:?}: count {c} from material {m}")),
            (None, 0) => None,
            (None, 1) => Some(false),
            (None, -1) => Some(true),
            (None, _) => return Err(format!("{cl:?}: count {c}")),
        };
        checked = true;
        prev = hi;
    }
    if let (Some(a), Some(b)) = (state, mat(prev + 0.5)) {
        if a != b {
            return Err(format!("{cl:?}: ends counted {a}, truth {b}"));
        }
    }
    Ok(checked)
}

fn run(name: &str, body: &Body<f64>, truth: &Truth, rays: &[(Point3<f64>, Vec3<f64>)], tol: Tol) -> (usize, usize, usize) {
    let band = Band::linear(tol).expect("band");
    let k_eps = tol.k() * tol.eps();
    let face = topo::test_support::edge_frames(body, &[0.5], band)[0].face;
    let rev = body.revert().expect("reverts");
    let (mut n, mut read, mut wrong) = (0, 0, 0);
    for (inside_out, b) in [(false, body), (true, &rev)] {
        let face = if inside_out { topo::test_support::edge_frames(b, &[0.5], band)[0].face } else { face };
        for &(q, d) in rays {
            n += 1;
            match check(b, face, q, d, truth, inside_out, k_eps, band) {
                Ok(c) => read += usize::from(c),
                Err(w) => {
                    wrong += 1;
                    let kind = if w.starts_with("face orders disagree: Err") && w.contains(" vs Err") { "fault-kind" } else if w.starts_with("face orders") { "order-hits" } else { "COUNT" };
                    println!("R22K eps={:e} {name} {kind}", tol.eps());
                    if wrong <= 6 || kind != "fault-kind" {
                        println!("R22 WRONG eps={:e} {name} io={inside_out} q={q:?} d={d:?}: {}", tol.eps(), w.chars().take(400).collect::<String>());
                    }
                }
            }
        }
    }
    (n, read, wrong)
}

#[test]
#[ignore = "review probe"]
fn review_5697_r2_round2_whole_rays() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let k_eps = tol.k() * tol.eps();
    let per: usize = std::env::var("R2_PER").ok().and_then(|s| s.parse().ok()).unwrap_or(3);
    let mut bodies = if std::env::var("R2_FLATONLY").is_ok() { Vec::new() } else { corpus(tol) };
    for (name, body, truth) in &bodies {
        let mut rng = Rng::new(name.len() as u64 * 7919 + 13);
        let mut rays = Vec::new();
        let fr: Vec<f64> = (0..per).map(|_| rng.range(0.03, 0.97)).collect();
        for f in topo::test_support::edge_frames(body, &fr, band) {
            let p = f.point + f.into * (rng.range(-3.0, 3.0) * k_eps) + f.normal * (rng.range(-3.0, 3.0) * k_eps);
            let along = f.normal.cross(f.into);
            for d in [rng.unit(), (f.into * rng.range(-1.0, 1.0) + along * rng.range(-1.0, 1.0) + f.normal * rng.range(-0.003, 0.003)).normalize()] {
                for back in [3.0, 0.02] {
                    rays.push((p - d * back, d));
                }
            }
        }
        let (n, read, wrong) = run(name, body, truth, &rays, tol);
        println!("R22 eps={:e} {name}: rays {n} checked {read} WRONG {wrong}", tol.eps());
    }
    // The narrow flat.
    for m in [1.5, 2.5, 4.0, 8.0] {
        let w = m * k_eps;
        let prof = flat_profile(w);
        let body = sweep::test_support::prism(prof.clone(), 1.0, tol);
        let ok = topo::AtRestBody::validate(body.clone(), tol).is_ok();
        let p2 = prof.clone();
        let truth: Truth = Box::new(move |p: Point3<f64>| in_bulge_loop(&p2, (p.x, p.y)) && p.z > 0.0 && p.z < 1.0);
        let mut rng = Rng::new(m.to_bits());
        let mut rays = Vec::new();
        for _ in 0..(400 * per) {
            let u = rng.range(-0.8, 1.8);
            let h = rng.range(-2.5, 2.5) * k_eps;
            let p = Point3::new(0.8 - w * u, 1.0 + h, rng.range(0.1, 0.9));
            // Shallow: nearly in the flat's plane, crossing the strip on a slant.
            let phi = rng.range(0.0, 2.0 * PI);
            let tilt = rng.range(-1.0, 1.0) * 10f64.powf(rng.range(-6.0, -1.0));
            let d = Vec3::new(phi.cos(), tilt, phi.sin()).normalize();
            for back in [3.0, 0.02] {
                rays.push((p - d * back, d));
            }
        }
        let name = format!("narrow flat {m}Kε (at rest: {ok})");
        let (n, read, wrong) = run(&name, &body, &truth, &rays, tol);
        println!("R22 eps={:e} {name}: rays {n} checked {read} WRONG {wrong}", tol.eps());
    }
}
