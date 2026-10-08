//! REVIEW PROBES (PR 4300), scratch branch only. Self-contained so it
//! also runs on main.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Point3, Tol, Vec3};
use topo::test_support::meeting::{MEET, PLATE, Pose, apex_pyramid, corners_disjoint, posed_box, poses};
use topo::test_support::split_plane;
use topo::{AtRestBody, BooleanResult, SplitPart, intersect, mass_properties, split, subtract, union, validate_geometric};

fn t() -> Tol {
    Tol::witness()
}

fn meander(path: &[(f64, f64)]) -> Vec<[f64; 3]> {
    path.iter().map(|&(y, z)| [0.5, (y - 2.5) * 0.15, z * 0.2]).collect()
}
const COMB: [(f64, f64); 12] = [(0.0, 2.0), (5.0, 2.0), (5.0, -1.0), (4.0, -1.0), (4.0, 1.0), (3.0, 1.0), (3.0, -1.0), (2.0, -1.0), (2.0, 1.0), (1.0, 1.0), (1.0, -1.0), (0.0, -1.0)];
const ARCH: [(f64, f64); 12] = [(0.0, 3.0), (5.0, 3.0), (5.0, -2.0), (2.0, -2.0), (2.0, 1.0), (3.0, 1.0), (3.0, -1.0), (4.0, -1.0), (4.0, 2.0), (1.0, 2.0), (1.0, -1.0), (0.0, -1.0)];
/// Three arches up, apart (the CLEAVE filing's crown): runs side by side.
const CROWN: [(f64, f64); 12] = [(0.0, -2.0), (5.0, -2.0), (5.0, 1.0), (4.0, 1.0), (4.0, -1.0), (3.0, -1.0), (3.0, 1.0), (2.0, 1.0), (2.0, -1.0), (1.0, -1.0), (1.0, 1.0), (0.0, 1.0)];

fn area(p: &[(f64, f64)]) -> f64 {
    let n = p.len();
    (0..n).map(|i| p[i].0 * p[(i + 1) % n].1 - p[i].1 * p[(i + 1) % n].0).sum::<f64>().abs() / 2.0
}
fn below_area(p: &[(f64, f64)]) -> f64 {
    let mut out = Vec::new();
    let n = p.len();
    for i in 0..n {
        let (a, b) = (p[i], p[(i + 1) % n]);
        if a.1 <= 0.0 { out.push(a); }
        if (a.1 <= 0.0) != (b.1 <= 0.0) {
            let s = -a.1 / (b.1 - a.1);
            out.push((a.0 + s * (b.0 - a.0), 0.0));
        }
    }
    area(&out)
}
fn scaled(p: &[(f64, f64)]) -> Vec<(f64, f64)> {
    p.iter().map(|&(y, z)| ((y - 2.5) * 0.15, z * 0.2)).collect()
}

/// (f): `split` of each apex pyramid on the top's plane through the apex.
#[test]
fn review_split_through_the_apex() {
    let mut lines = Vec::new();
    for (name, base) in [("crown", &CROWN), ("comb", &COMB), ("arch cone", &ARCH)] {
        let sc = scaled(base);
        let (whole, below) = (area(&sc) / 6.0, below_area(&sc) / 6.0);
        for pose in poses() {
            let u = apex_pyramid(&meander(base), &pose, t());
            let o = pose.at(MEET);
            let z = pose.at([MEET[0], MEET[1], MEET[2] + 1.0]);
            let n = Vec3::new(z.x - o.x, z.y - o.y, z.z - o.z).normalize();
            let r = match split(&u, &split_plane(o, n, t()), t()) {
                Ok(r) => r,
                Err(e) => { lines.push(format!("{name} {}: REFUSED {e}", pose.label)); continue; }
            };
            for (side, part, want) in [("above", &r.above, whole - below), ("below", &r.below, below)] {
                let SplitPart::Body(b) = part else { lines.push(format!("{name} {} {side}: EMPTY", pose.label)); continue; };
                let v = mass_properties(b, t()).unwrap().volume;
                let corners = corners_disjoint(b).is_ok();
                let t3 = match validate_geometric(b, t()) { Ok(()) => "ok".to_string(), Err(e) => format!("{:?}", e).chars().take(300).collect() };
                lines.push(format!("{name} {} {side}: vol {v:.6} want {want:.6} corners_disjoint {corners} tier3 {t3}", pose.label));
            }
        }
    }
    for l in &lines { println!("{l}"); }
}

fn in_poly(p: &[(f64, f64)], q: (f64, f64)) -> bool {
    let mut c = false;
    let n = p.len();
    for i in 0..n {
        let (a, b) = (p[i], p[(i + 1) % n]);
        if (a.1 > q.1) != (b.1 > q.1) && q.0 < a.0 + (q.1 - a.1) / (b.1 - a.1) * (b.0 - a.0) { c = !c; }
    }
    c
}
/// Analytic membership at rest, independent of the kernel.
fn pyr_in(base: &[(f64, f64)], q: [f64; 3]) -> bool {
    let d = [q[0] - MEET[0], q[1] - MEET[1], q[2] - MEET[2]];
    if d[0] <= 0.0 || d[0] > 0.5 { return false; }
    let s = d[0] / 0.5;
    in_poly(base, (d[1] / s, d[2] / s))
}
fn plate_in(q: [f64; 3]) -> bool {
    (0..3).all(|i| PLATE[i].0 < q[i] && q[i] < PLATE[i].1)
}

fn result_body(r: Result<BooleanResult<f64>, topo::BooleanError>) -> Result<AtRestBody<f64>, String> {
    match r {
        Ok(BooleanResult::Body(b)) => Ok(b.body),
        Ok(_) => Err("empty".into()),
        Err(e) => Err(format!("{e:?}").chars().take(160).collect()),
    }
}

/// (c)/(e): the comb with its base list rotated and reversed (moving
/// which run the apex's orbit reads first) in every op at every pose;
/// at rest, an analytic-membership oracle over 20 000 pseudo-random
/// probes near the apex.
#[test]
fn review_comb_rotations_and_analytic_oracle() {
    let band = geom_core::Band::linear(t()).unwrap();
    let sc = scaled(&COMB);
    let (pyr, below) = (area(&sc) / 6.0, below_area(&sc) / 6.0);
    let mut bad = Vec::new();
    let mut built = 0;
    for rev in [false, true] {
        for rot in 0..12 {
            let mut base: Vec<_> = COMB.to_vec();
            base.rotate_left(rot);
            if rev { base.reverse(); }
            for pose in poses() {
                let p = posed_box("plate", PLATE, &pose, t());
                let u = apex_pyramid(&meander(&base), &pose, t());
                for (what, r, want) in [
                    ("P-U", subtract(&p, &u, t()), 6.0 - below),
                    ("U-P", subtract(&u, &p, t()), pyr - below),
                    ("PuU", union(&p, &u, t()), 6.0 + pyr - below),
                    ("UuP", union(&u, &p, t()), 6.0 + pyr - below),
                    ("PnU", intersect(&p, &u, t()), below),
                    ("UnP", intersect(&u, &p, t()), below),
                ] {
                    let label = format!("rev={rev} rot={rot} {} {what}", pose.label);
                    let b = match result_body(r) { Ok(b) => b, Err(e) => { bad.push(format!("{label}: {e}")); continue; } };
                    let v = mass_properties(&b, t()).unwrap().volume;
                    let mut errs = Vec::new();
                    if (v - want).abs() > 1e-9 { errs.push(format!("vol {v}")); }
                    if validate_geometric(&b, t()).is_err() { errs.push("tier3".into()); }
                    if let Err(e) = corners_disjoint(&b) { errs.push(format!("corners {e}")); }
                    if pose.label == "at rest" {
                        let mut seed: u64 = 0x9E37_79B9_7F4A_7C15 ^ (rot as u64);
                        let mut rnd = || { seed ^= seed << 13; seed ^= seed >> 7; seed ^= seed << 17; (seed >> 11) as f64 / (1u64 << 53) as f64 };
                        let (mut read, mut wrong) = (0, 0);
                        for _ in 0..20_000 {
                            let rad = 0.6 * rnd().powi(2);
                            let q = [MEET[0] + rad * (2.0 * rnd() - 1.0), MEET[1] + rad * (2.0 * rnd() - 1.0), MEET[2] + rad * (2.0 * rnd() - 1.0)];
                            let (ip, iu) = (plate_in(q), pyr_in(&sc, q));
                            let want = match what { "P-U" => ip && !iu, "U-P" => iu && !ip, "PuU" | "UuP" => ip || iu, _ => ip && iu };
                            match topo::point_in_solid(&b, Point3::new(q[0], q[1], q[2]), band, t()).unwrap_or(topo::SolidContainment::OnBoundary) {
                                topo::SolidContainment::In => { read += 1; if !want { wrong += 1; } }
                                topo::SolidContainment::Out => { read += 1; if want { wrong += 1; } }
                                topo::SolidContainment::OnBoundary => {}
                            }
                        }
                        if wrong > 0 || read < 15_000 { errs.push(format!("oracle wrong {wrong} of {read}")); }
                    }
                    if errs.is_empty() { built += 1; } else { bad.push(format!("{label}: {errs:?}")); }
                }
            }
        }
    }
    println!("built sound {built}, bad {}", bad.len());
    for l in &bad { println!("{l}"); }
    assert!(bad.is_empty());
}
