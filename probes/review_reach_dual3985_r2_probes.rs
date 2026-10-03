//! Reviewer probes, PR 3985 (`reach/arc-from-pairing`), lane
//! `reach-dual3985-r2`. Every expected value is the reviewer's own:
//! closed forms (lens, cap, sector areas) or a slice integral of the
//! z-extent over a fine xy grid, and `point_in_solid` is held to a
//! signed-distance predicate written here, never to the kernel.
//!
//! Run: `cargo nextest run -p sweep --no-capture -E 'test(review_reach_dual3985_r2)'`.
//! Each row prints a census line per (fixture, op) and panics at the end
//! if any body was WRONG (volume off its oracle, a tier red, or
//! `point_in_solid` answering the wrong side). A typed refusal is
//! counted, not failed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use std::panic::{AssertUnwindSafe, catch_unwind};

use geom_core::{Band, Point2, Point3, Tol, Vec3};
use geom_core::Affine3;
use sweep::Revolution;
use sweep::test_support::{ball_poled, prism_at, revolved_about_y};
use topo::{Body, SolidContainment};

type Sdf<'a> = &'a dyn Fn(Point3<f64>) -> f64;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[derive(Default)]
struct Census {
    built: usize,
    refused: Vec<String>,
    wrong: Vec<String>,
    pis_refused: usize,
    pis_checked: usize,
}

fn op_run(op: &str, a: &Body<f64>, b: &Body<f64>) -> Result<Body<f64>, String> {
    let tol = Tol::witness();
    let r = catch_unwind(AssertUnwindSafe(|| match op {
        "∪" => topo::boolean::union(a, b, tol),
        "∩" => topo::boolean::intersect(a, b, tol),
        _ => topo::boolean::subtract(a, b, tol),
    }));
    match r {
        Err(_) => Err("PANIC".into()),
        Ok(Err(e)) => Err(format!("{e:?}")),
        Ok(Ok(out)) => match out.body() {
            Some(b) => Ok(b.body.clone()),
            None => Ok(Body::new()),
        },
    }
}

/// Validate, volume against `want` (relative `rel`), and `n` witness
/// points in the box `lo..hi` against `sdf` (negative inside), skipping
/// points within `skip` of the predicate's boundary.
#[allow(clippy::too_many_arguments)]
fn judge(
    c: &mut Census,
    what: &str,
    body: &Body<f64>,
    sdf: Sdf,
    want: f64,
    rel: f64,
    (lo, hi): (Point3<f64>, Point3<f64>),
    n: usize,
    skip: f64,
) {
    let tol = Tol::witness();
    if want <= 1e-30 {
        if body.faces().count() != 0 {
            c.wrong.push(format!("{what}: expected empty, got faces"));
        }
        return;
    }
    if let Err(e) = topo::validate_closed(body) {
        c.wrong.push(format!("{what}: tier 2 {e:?}"));
        return;
    }
    if let Err(e) = topo::validate_geometric(body, tol) {
        c.wrong.push(format!("{what}: tier 3 {e:?}"));
    }
    match topo::mass_properties(body, tol) {
        Ok(p) => {
            if (p.volume - want).abs() > rel * want + p.volume_pad {
                c.wrong.push(format!(
                    "{what}: volume {} vs oracle {want} (rel {:.2e}, pad {})",
                    p.volume,
                    (p.volume - want) / want,
                    p.volume_pad
                ));
            }
        }
        Err(e) => c.refused.push(format!("{what}: mass_properties {e:?}")),
    }
    let band = Band::linear(tol).unwrap();
    let mut rng = Lcg(0x5eed ^ (n as u64));
    let mut k = 0;
    while k < n {
        let q = Point3::new(
            lo.x + (hi.x - lo.x) * rng.next(),
            lo.y + (hi.y - lo.y) * rng.next(),
            lo.z + (hi.z - lo.z) * rng.next(),
        );
        let f = sdf(q);
        if f.abs() < skip {
            continue;
        }
        k += 1;
        let r = catch_unwind(AssertUnwindSafe(|| topo::point_in_solid(body, q, band, tol)));
        match r {
            Ok(Ok(SolidContainment::In)) if f < 0.0 => c.pis_checked += 1,
            Ok(Ok(SolidContainment::Out)) if f > 0.0 => c.pis_checked += 1,
            Ok(Err(_)) => c.pis_refused += 1,
            Ok(Ok(got)) => c.wrong.push(format!("{what}: point_in_solid{q:?} = {got:?}, sdf {f}")),
            Err(_) => c.wrong.push(format!("{what}: point_in_solid{q:?} PANIC")),
        }
    }
}

/// Every op in both orders against the oracle `shared` (the
/// intersection volume), with `va`, `vb` the operands' volumes.
#[allow(clippy::too_many_arguments)]
fn every_op(
    c: &mut Census,
    label: &str,
    a: &Body<f64>,
    b: &Body<f64>,
    sa: Sdf,
    sb: Sdf,
    (va, vb, shared): (f64, f64, f64),
    rel: f64,
    bbox: (Point3<f64>, Point3<f64>),
    skip: f64,
) -> Option<Body<f64>> {
    let mut union = None;
    let un = |q| sa(q).min(sb(q));
    let it = |q| sa(q).max(sb(q));
    let ab = |q| sa(q).max(-sb(q));
    let ba = |q| sb(q).max(-sa(q));
    let rows: [(&str, &Body<f64>, &Body<f64>, Sdf, f64); 6] = [
        ("a∪b", a, b, &un, va + vb - shared),
        ("b∪a", b, a, &un, va + vb - shared),
        ("a∩b", a, b, &it, shared),
        ("b∩a", b, a, &it, shared),
        ("a∖b", a, b, &ab, va - shared),
        ("b∖a", b, a, &ba, vb - shared),
    ];
    for (name, x, y, sdf, want) in rows {
        let op = &name[1..name.len() - 1];
        let op = if op.contains('∪') {
            "∪"
        } else if op.contains('∩') {
            "∩"
        } else {
            "∖"
        };
        let what = format!("{label} {name}");
        match op_run(op, x, y) {
            Err(e) => {
                let short: String = e.chars().take(700).collect();
                println!("  REFUSED {what}: {short}");
                c.refused.push(format!("{what}: {short}"));
            }
            Ok(body) => {
                c.built += 1;
                let before = c.wrong.len();
                judge(c, &what, &body, sdf, want, rel, bbox, 60, skip);
                println!(
                    "  built {what}: {}",
                    if c.wrong.len() == before { "ok" } else { "WRONG" }
                );
                if name == "a∪b" {
                    union = Some(body);
                }
            }
        }
    }
    union
}

fn finish(c: &Census, row: &str) {
    println!(
        "{row}: built {}, refused {}, wrong {}, pis checked {} refused {}",
        c.built,
        c.refused.len(),
        c.wrong.len(),
        c.pis_checked,
        c.pis_refused
    );
    for r in &c.refused {
        println!("  refusal: {r}");
    }
    assert!(c.wrong.is_empty(), "{row}: WRONG bodies:\n{}", c.wrong.join("\n"));
}

// ---- shapes and their own predicates ----

fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

fn ball_sdf(c: Vec3<f64>, r: f64) -> impl Fn(Point3<f64>) -> f64 {
    move |q| ((q.x - c.x).powi(2) + (q.y - c.y).powi(2) + (q.z - c.z).powi(2)).sqrt() - r
}

/// Signed distance to a simple polygon (negative inside).
fn poly_sdf(poly: &[(f64, f64)], x: f64, y: f64) -> f64 {
    let n = poly.len();
    let mut d = f64::INFINITY;
    let mut inside = false;
    for i in 0..n {
        let (ax, ay) = poly[i];
        let (bx, by) = poly[(i + 1) % n];
        let (ex, ey) = (bx - ax, by - ay);
        let t = (((x - ax) * ex + (y - ay) * ey) / (ex * ex + ey * ey)).clamp(0.0, 1.0);
        let (px, py) = (ax + t * ex - x, ay + t * ey - y);
        d = d.min((px * px + py * py).sqrt());
        if (ay > y) != (by > y) && x < ax + (y - ay) / (by - ay) * ex {
            inside = !inside;
        }
    }
    if inside { -d } else { d }
}

fn poly_area(poly: &[(f64, f64)]) -> f64 {
    let n = poly.len();
    (0..n)
        .map(|i| {
            let (ax, ay) = poly[i];
            let (bx, by) = poly[(i + 1) % n];
            ax * by - bx * ay
        })
        .sum::<f64>()
        / 2.0
}

fn prism_of(poly: &[(f64, f64)], z0: f64, h: f64) -> Body<f64> {
    prism_at(
        poly.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect(),
        z0,
        h,
        Tol::witness(),
    )
}

fn prism_sdf(poly: Vec<(f64, f64)>, z0: f64, z1: f64) -> impl Fn(Point3<f64>) -> f64 {
    move |q| poly_sdf(&poly, q.x, q.y).max(z0 - q.z).max(q.z - z1)
}

/// A cylinder of radius `r` about `z`, `z ∈ [z0, z0 + h]`, as two
/// semicircular bulge arcs (seams at azimuth 0 and π).
fn cylinder(r: f64, z0: f64, h: f64) -> Body<f64> {
    prism_at(
        vec![(Point2::new(r, 0.0), 1.0), (Point2::new(-r, 0.0), 1.0)],
        z0,
        h,
        Tol::witness(),
    )
}

fn lens(r1: f64, r2: f64, d: f64) -> f64 {
    let cap = |r: f64, h: f64| PI * h * h * (3.0 * r - h) / 3.0;
    let x = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
    cap(r1, r1 - x) + cap(r2, r2 - (d - x))
}

/// Ball ∩ z-prism over the xy polygon: the slice integral of the
/// z-overlap on an `m × m` midpoint grid over the ball's shadow.
fn ball_prism_shared(c: Vec3<f64>, r: f64, poly: &[(f64, f64)], z0: f64, z1: f64, m: usize) -> f64 {
    let h = 2.0 * r / m as f64;
    let mut s = 0.0;
    for i in 0..m {
        let x = c.x - r + (i as f64 + 0.5) * h;
        for j in 0..m {
            let y = c.y - r + (j as f64 + 0.5) * h;
            let rr = r * r - (x - c.x).powi(2) - (y - c.y).powi(2);
            if rr <= 0.0 || poly_sdf(poly, x, y) > 0.0 {
                continue;
            }
            let w = rr.sqrt();
            let len = ((c.z + w).min(z1) - (c.z - w).max(z0)).max(0.0);
            s += len;
        }
    }
    s * h * h
}

fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<(f64, f64)> {
    vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
}

fn bbox(lo: (f64, f64, f64), hi: (f64, f64, f64)) -> (Point3<f64>, Point3<f64>) {
    (Point3::new(lo.0, lo.1, lo.2), Point3::new(hi.0, hi.1, hi.2))
}

// ---- claim 3: walk order, not chord length ----

/// The plate star whose outline crosses the unit circle at
/// `φ0 + {0°, 100°, 200°, 300°}`, radius `ro` over the arcs it holds
/// (`[0, 100]`, `[200, 300]`) and `ri` over the gaps. From the crossing
/// at 300° the chord-nearest site is 0° (60° away, across a gap); the
/// walk-nearest is 200°.
fn star(phi0: f64, ro: f64, ri: f64, s: f64) -> Vec<(f64, f64)> {
    let at = |r: f64, deg: f64| {
        let t = (deg + phi0).to_radians();
        (s * r * t.cos(), s * r * t.sin())
    };
    vec![
        at(ro, 0.0),
        at(ro, 50.0),
        at(ro, 100.0),
        at(ri, 100.0),
        at(ri, 150.0),
        at(ri, 200.0),
        at(ro, 200.0),
        at(ro, 250.0),
        at(ro, 300.0),
        at(ri, 300.0),
        at(ri, 330.0),
        at(ri, 360.0),
    ]
}

/// The star's area inside the disk of radius `s`: two 100° sectors and
/// the inner fans, closed form.
fn star_in_disk(ri: f64, s: f64) -> f64 {
    let sectors = 2.0 * (100.0f64.to_radians() / 2.0) * s * s;
    let tri = |deg: f64| 0.5 * (ri * s).powi(2) * deg.to_radians().sin();
    sectors + 2.0 * tri(50.0) + 2.0 * tri(30.0)
}

#[test]
fn review_reach_dual3985_r2_four_crossings_pair_by_walk() {
    let mut c = Census::default();
    for s in [1e-3, 1.0, 1e3] {
        for phi0 in [20.0, 47.0, 73.0, 131.0] {
            let poly = star(phi0, 2.0, 0.5, s);
            let (z0, z1) = (0.5 * s, 1.5 * s);
            let cyl = cylinder(s, 0.0, 2.0 * s);
            let plate = prism_of(&poly, z0, z1 - z0);
            let cs = move |q: Point3<f64>| {
                ((q.x * q.x + q.y * q.y).sqrt() - s)
                    .max(-q.z)
                    .max(q.z - 2.0 * s)
            };
            let ps = prism_sdf(poly.clone(), z0, z1);
            let vc = PI * s * s * 2.0 * s;
            let vp = poly_area(&poly) * (z1 - z0);
            let shared = star_in_disk(0.5, s) * (z1 - z0);
            let label = format!("star φ0 {phi0} s {s:e}");
            println!("{label}");
            every_op(
                &mut c,
                &label,
                &cyl,
                &plate,
                &cs,
                &ps,
                (vc, vp, shared),
                1e-8,
                bbox((-2.2 * s, -2.2 * s, -0.2 * s), (2.2 * s, 2.2 * s, 2.2 * s)),
                1e-6 * s,
            );
        }
    }
    finish(&c, "four crossings");
}


/// The same star on a FULL-TURN wall: the cylinder `ρ ≤ s` about `y`,
/// `y ∈ [0, 2s]`, revolved a whole turn (one wall face, so no face-pair
/// filter splits the four sites), the plate extruded along `y` (the
/// z-prism turned −90° about `x`: `(x, y, z) → (x, z, −y)`).
#[test]
fn review_reach_dual3985_r2_four_crossings_on_one_full_turn_face() {
    let mut c = Census::default();
    let tol = Tol::witness();
    for s in [1e-3, 1.0, 1e3] {
        for phi0 in [0.0, 20.0, 47.0, 73.0, 131.0, 200.0] {
            for (ro, ri) in [(2.0, 0.5), (1.3, 0.8)] {
                let poly = star(phi0, ro, ri, s);
                let (z0, z1) = (0.5 * s, 1.5 * s);
                let cyl = revolved_about_y(
                    vec![
                        (Point2::new(0.0, 0.0), 0.0),
                        (Point2::new(s, 0.0), 0.0),
                        (Point2::new(s, 2.0 * s), 0.0),
                        (Point2::new(0.0, 2.0 * s), 0.0),
                    ],
                    Revolution::Full,
                    tol,
                );
                let flat = prism_of(&poly, z0, z1 - z0);
                let turn = Affine3::rotation_about_axis(
                    Point3::new(0.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0),
                    -core::f64::consts::FRAC_PI_2,
                );
                let plate = topo::transform_rigid(&flat, &turn, tol).unwrap();
                let cs = move |q: Point3<f64>| {
                    ((q.x * q.x + q.z * q.z).sqrt() - s)
                        .max(-q.y)
                        .max(q.y - 2.0 * s)
                };
                let flat_sdf = prism_sdf(poly.clone(), z0, z1);
                let ps = move |q: Point3<f64>| flat_sdf(Point3::new(q.x, -q.z, q.y));
                let vc = PI * s * s * 2.0 * s;
                let vp = poly_area(&poly) * (z1 - z0);
                let shared = star_in_disk(ri, s) * (z1 - z0);
                let label = format!("full-turn star φ0 {phi0} ro {ro} ri {ri} s {s:e}");
                println!("{label}");
                every_op(
                    &mut c,
                    &label,
                    &cyl,
                    &plate,
                    &cs,
                    &ps,
                    (vc, vp, shared),
                    1e-8,
                    bbox((-2.2 * s, -0.2 * s, -2.2 * s), (2.2 * s, 2.2 * s, 2.2 * s)),
                    1e-6 * s,
                );
            }
        }
    }
    finish(&c, "four crossings, one full-turn face");
}

// ---- claim 2: the newly built bodies, widened ----

#[test]
fn review_reach_dual3985_r2_tilted_sphere_pairs() {
    let mut c = Census::default();
    let poses: [(f64, f64, Vec3<f64>, Vec3<f64>, Vec3<f64>); 5] = [
        (1.0, 0.8, v(0.6, 0.5, 0.3), v(0.0, 1.0, 0.0), v(0.0, 1.0, 0.0)),
        (1.0, 0.8, v(0.6, 0.5, 0.3), v(0.3, 1.0, 0.2), v(-0.5, 0.2, 1.0)),
        (0.7, 1.3, v(1.0, -0.2, 0.05), v(0.0, 1.0, 0.0), v(1.0, 1.0, 1.0)),
        (1.0, 0.4, v(0.2, 0.9, -0.4), v(0.0, 0.0, 1.0), v(0.0, 1.0, 0.0)),
        (1.0, 1.0, v(1.0, 0.0, 0.3), v(0.0, 1.0, 0.0), v(0.0, 1.0, 0.0)),
    ];
    for s in [1e-3, 1.0, 1e3] {
        for (k, (r1, r2, dir, p1, p2)) in poses.iter().enumerate() {
            let u = *dir / dir.norm();
            let d = if k == 3 { 1.1 } else { 1.2 * (r1 + r2) / 2.0 };
            let c1 = v(0.1, -0.2, 0.3) * s;
            let c2 = c1 + u * (d * s);
            let a = ball_poled(r1 * s, c1, *p1, Tol::witness());
            let b = ball_poled(r2 * s, c2, *p2, Tol::witness());
            let (sa, sb) = (ball_sdf(c1, r1 * s), ball_sdf(c2, r2 * s));
            let vol = |r: f64| 4.0 / 3.0 * PI * (r * s).powi(3);
            let shared = lens(*r1 * s, *r2 * s, d * s);
            let label = format!("sphere pair {k} s {s:e}");
            println!("{label}");
            let m = 3.0 * s;
            let lo = (c1.x - m, c1.y - m, c1.z - m);
            let hi = (c1.x + m, c1.y + m, c1.z + m);
            let un = every_op(
                &mut c,
                &label,
                &a,
                &b,
                &sa,
                &sb,
                (vol(*r1), vol(*r2), shared),
                1e-8,
                bbox(lo, hi),
                1e-6 * s,
            );
            // Reuse: the union as an operand, minus a slab through the
            // section's plane region; MC volume from the predicate.
            if let Some(un) = un {
                reuse(&mut c, &label, &un, &|q| sa(q).min(sb(q)), (lo, hi), s);
            }
        }
    }
    finish(&c, "tilted sphere pairs");
}

/// `body ∖ slab`, the slab `y ∈ [cy − 0.15s, cy + 0.15s]` through the
/// box's centre: `point_in_solid` against the predicate and the volume
/// against a Monte Carlo estimate of the predicate (5σ).
fn reuse(
    c: &mut Census,
    label: &str,
    body: &Body<f64>,
    sdf: Sdf,
    (lo, hi): ((f64, f64, f64), (f64, f64, f64)),
    s: f64,
) {
    let cy = (lo.1 + hi.1) / 2.0;
    let poly = rect(lo.0 - s, cy - 0.15 * s, hi.0 + s, cy + 0.15 * s);
    let slab = prism_of(&poly, lo.2 - s, hi.2 - lo.2 + 2.0 * s);
    let ss = prism_sdf(poly, lo.2 - s, hi.2 + s);
    let f = |q: Point3<f64>| sdf(q).max(-ss(q));
    let what = format!("{label} (a∪b)∖slab");
    match op_run("∖", body, &slab) {
        Err(e) => {
            let short: String = e.chars().take(700).collect();
            println!("  REFUSED {what}: {short}");
            c.refused.push(format!("{what}: {short}"));
        }
        Ok(r) => {
            c.built += 1;
            let mut rng = Lcg(77);
            let n = 400_000;
            let mut inside = 0usize;
            for _ in 0..n {
                let q = Point3::new(
                    lo.0 + (hi.0 - lo.0) * rng.next(),
                    lo.1 + (hi.1 - lo.1) * rng.next(),
                    lo.2 + (hi.2 - lo.2) * rng.next(),
                );
                if f(q) < 0.0 {
                    inside += 1;
                }
            }
            let bv = (hi.0 - lo.0) * (hi.1 - lo.1) * (hi.2 - lo.2);
            let p = inside as f64 / n as f64;
            let est = p * bv;
            let sigma = bv * (p * (1.0 - p) / n as f64).sqrt();
            let before = c.wrong.len();
            // `judge` with a loose rel: the MC estimate's 5σ.
            judge(
                c,
                &what,
                &r,
                &f,
                est,
                5.0 * sigma / est,
                bbox(lo, hi),
                60,
                1e-6 * s,
            );
            println!(
                "  built {what}: {}",
                if c.wrong.len() == before { "ok" } else { "WRONG" }
            );
        }
    }
}

/// A plane tilted against the ball's chart (the planar side of a tilted
/// plane×sphere cut): an axis-aligned block, the ball poled off the
/// block top's normal, the centre `t` above the top. Shared = the cap.
#[test]
fn review_reach_dual3985_r2_tilted_plane_sphere() {
    let mut c = Census::default();
    for s in [1e-3, 1.0, 1e3] {
        for (k, (r, t, pole, cx)) in [
            (0.5, 0.2, v(0.0, 1.0, 0.0), 0.0),
            (0.5, -0.2, v(1.0, 1.0, 0.3), 0.3),
            (0.7, 0.0, v(0.2, -1.0, 0.7), -0.4),
            (0.3, 0.25, v(1.0, 0.0, 0.0), 0.9),
        ]
        .into_iter()
        .enumerate()
        {
            let poly: Vec<(f64, f64)> = rect(-2.0, -2.0, 2.0, 2.0)
                .into_iter()
                .map(|(x, y)| (x * s, y * s))
                .collect();
            let block = prism_of(&poly, -2.0 * s, 2.0 * s);
            let bs = prism_sdf(poly, -2.0 * s, 0.0);
            let cc = v(cx * s, 0.1 * s, t * s);
            let ball = ball_poled(r * s, cc, pole, Tol::witness());
            let bls = ball_sdf(cc, r * s);
            let h = r - t;
            let cap = PI * h * h * (3.0 * r - h) / 3.0 * s.powi(3);
            let vb = 4.0 / 3.0 * PI * (r * s).powi(3);
            let label = format!("tilted plane {k} s {s:e}");
            println!("{label}");
            let lo = (-2.5 * s, -2.5 * s, -2.5 * s);
            let hi = (2.5 * s, 2.5 * s, 1.5 * s);
            let un = every_op(
                &mut c,
                &label,
                &block,
                &ball,
                &bs,
                &bls,
                (32.0 * s.powi(3), vb, cap),
                1e-8,
                bbox(lo, hi),
                1e-6 * s,
            );
            if let Some(un) = un {
                reuse(&mut c, &label, &un, &|q| bs(q).min(bls(q)), (lo, hi), s);
            }
        }
    }
    finish(&c, "tilted plane×sphere");
}

/// Bars and an L-notch through tilted balls (pierce rings, reflex
/// notch): shared by the reviewer's slice integral.
#[test]
fn review_reach_dual3985_r2_pierces_and_notches() {
    let mut c = Census::default();
    let l_poly = vec![
        (-1.0, -1.0),
        (1.0, -1.0),
        (1.0, 0.0),
        (0.0, 0.0),
        (0.0, 1.0),
        (-1.0, 1.0),
    ];
    for s in [1e-3, 1.0, 1e3] {
        for (k, (poly, z0, z1, cc, r, pole)) in [
            // A bar through a ball, off every seam.
            (rect(-0.2, -0.15, 0.25, 0.1), -2.0, 2.0, v(0.05, 0.02, 0.1), 0.8, v(0.3, 1.0, 0.1)),
            // A bar piercing only the top of a ball (blind).
            (rect(-0.2, -0.15, 0.25, 0.1), 0.3, 2.0, v(0.05, 0.02, 0.0), 0.8, v(1.0, 0.2, 0.4)),
            // An L prism whose reflex corner sits inside the ball.
            (l_poly.clone(), -0.3, 0.4, v(0.1, 0.12, 0.05), 0.5, v(0.0, 1.0, 0.0)),
            (l_poly.clone(), -0.3, 0.4, v(-0.05, 0.1, 0.0), 0.6, v(0.6, 0.3, 1.0)),
        ]
        .into_iter()
        .enumerate()
        {
            let poly: Vec<(f64, f64)> = poly.iter().map(|&(x, y)| (x * s, y * s)).collect();
            let prism = prism_of(&poly, z0 * s, (z1 - z0) * s);
            let ps = prism_sdf(poly.clone(), z0 * s, z1 * s);
            let cc = cc * s;
            let ball = ball_poled(r * s, cc, pole, Tol::witness());
            let bls = ball_sdf(cc, r * s);
            let shared = ball_prism_shared(cc, r * s, &poly, z0 * s, z1 * s, 3000);
            let vp = poly_area(&poly) * (z1 - z0) * s;
            let vb = 4.0 / 3.0 * PI * (r * s).powi(3);
            let label = format!("pierce/notch {k} s {s:e}");
            println!("{label}");
            let lo = (-1.2 * s, -1.2 * s, -2.1 * s);
            let hi = (1.2 * s, 1.2 * s, 2.1 * s);
            every_op(
                &mut c,
                &label,
                &prism,
                &ball,
                &ps,
                &bls,
                (vp, vb, shared),
                2e-5,
                bbox(lo, hi),
                1e-6 * s,
            );
        }
    }
    finish(&c, "pierces and notches");
}
