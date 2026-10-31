//! **Review probes for PR 5781** (`flush/aims-ties`): bodies outside the
//! batteries and the corpus, read on the merge base and on the PR head
//! alike, so the two outputs diff line for line.
//!
//! - `review_5781_sides`: each body's (and its revert's) every aim, its
//!   tier-3′ check, its shells' roles, uniform rays read whole against
//!   an analytic truth, and `point_in_solid` at random points.
//! - `review_5781_sym_ties`: `Sym<f64>` builds of symmetric planar bodies,
//!   counting the side reader's exact ties.
//!
//! Evidence, not gates: run by hand with `--run-ignored only --no-capture`.

#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::print_stdout,
    clippy::panic,
    clippy::type_complexity
)]

use core::f64::consts::PI;

use geom_core::{Band, Point2, Point3, Tol, Vec3};
use sweep::test_support::{
    bored_block_of_arcs, brick, cylinder_of_arcs_at, disc_of_arcs, extruded, prism_at,
};
use topo::{Body, ContactRecords};

use super::curved_edge_corpus::{Truth, corpus, in_bulge_loop, robust};

fn p2(x: f64, y: f64, b: f64) -> (Point2<f64>, f64) {
    (Point2::new(x, y), b)
}

fn op(which: &str, a: &Body<f64>, b: &Body<f64>, tol: Tol) -> Option<Body<f64>> {
    let at = |x: &Body<f64>| topo::AtRestBody::validate(x.clone(), tol).ok();
    let (a, b) = (at(a)?, at(b)?);
    let r = match which {
        "union" => topo::union(&a, &b, tol),
        "intersect" => topo::intersect(&a, &b, tol),
        _ => topo::subtract(&a, &b, tol),
    };
    match r {
        Ok(r) => r.body().map(|b| b.body.clone().into_body()),
        Err(e) => {
            println!("BUILD-REFUSED {which}: {:.160}", format!("{e:?}"));
            None
        }
    }
}

fn polygon_truth(verts: Vec<(Point2<f64>, f64)>, z: (f64, f64)) -> Truth {
    Box::new(move |p: Point3<f64>| p.z > z.0 && p.z < z.1 && in_bulge_loop(&verts, (p.x, p.y)))
}

fn quarter() -> f64 {
    (PI / 8.0).tan()
}

/// The probe bodies, each with its truth: planar faces with arcs and
/// holes, L and U plates, bored plates, partial and seamed walls, slivers
/// and near-tangent poses, and shells with cavities.
fn bodies(tol: Tol) -> Vec<(String, Body<f64>, Truth)> {
    let eps = tol.eps();
    let mut out: Vec<(String, Body<f64>, Truth)> = Vec::new();
    let mut prism = |name: &str, verts: Vec<(Point2<f64>, f64)>, h: f64| {
        let body = prism_at(verts.clone(), 0.0, h, tol);
        out.push((name.into(), body, polygon_truth(verts, (0.0, h))));
    };
    prism(
        "L plate",
        vec![
            p2(0.0, 0.0, 0.0),
            p2(2.0, 0.0, 0.0),
            p2(2.0, 1.0, 0.0),
            p2(1.0, 1.0, 0.0),
            p2(1.0, 2.0, 0.0),
            p2(0.0, 2.0, 0.0),
        ],
        0.5,
    );
    prism(
        "U plate",
        vec![
            p2(0.0, 0.0, 0.0),
            p2(3.0, 0.0, 0.0),
            p2(3.0, 2.0, 0.0),
            p2(2.0, 2.0, 0.0),
            p2(2.0, 1.0, 0.0),
            p2(1.0, 1.0, 0.0),
            p2(1.0, 2.0, 0.0),
            p2(0.0, 2.0, 0.0),
        ],
        0.5,
    );
    prism(
        "plus plate",
        vec![
            p2(1.0, 0.0, 0.0),
            p2(2.0, 0.0, 0.0),
            p2(2.0, 1.0, 0.0),
            p2(3.0, 1.0, 0.0),
            p2(3.0, 2.0, 0.0),
            p2(2.0, 2.0, 0.0),
            p2(2.0, 3.0, 0.0),
            p2(1.0, 3.0, 0.0),
            p2(1.0, 2.0, 0.0),
            p2(0.0, 2.0, 0.0),
            p2(0.0, 1.0, 0.0),
            p2(1.0, 1.0, 0.0),
        ],
        0.5,
    );
    prism(
        "hex prism",
        (0..6)
            .map(|k| {
                let a = f64::from(k) * PI / 3.0;
                p2(a.cos(), a.sin(), 0.0)
            })
            .collect(),
        0.4,
    );
    prism(
        "regular 12-gon",
        (0..12)
            .map(|k| {
                let a = f64::from(k) * PI / 6.0;
                p2(a.cos(), a.sin(), 0.0)
            })
            .collect(),
        0.4,
    );
    prism(
        "stadium",
        vec![
            p2(0.0, 0.0, 0.0),
            p2(2.0, 0.0, 1.0),
            p2(2.0, 1.0, 0.0),
            p2(0.0, 1.0, 1.0),
        ],
        0.3,
    );
    let q = quarter();
    prism(
        "rounded square",
        vec![
            p2(0.2, 0.0, 0.0),
            p2(1.8, 0.0, q),
            p2(2.0, 0.2, 0.0),
            p2(2.0, 1.8, q),
            p2(1.8, 2.0, 0.0),
            p2(0.2, 2.0, q),
            p2(0.0, 1.8, 0.0),
            p2(0.0, 0.2, q),
        ],
        0.3,
    );
    prism(
        "lune of two arcs",
        vec![p2(0.0, 0.0, 0.6), p2(2.0, 0.0, 0.6)],
        0.3,
    );
    prism(
        "D cap (arc and chord)",
        vec![p2(-1.0, 0.0, 1.0), p2(1.0, 0.0, 0.0)],
        0.3,
    );
    prism("sliver hex", (0..6)
            .map(|k| {
                let a = f64::from(k) * PI / 3.0;
                p2(a.cos(), a.sin(), 0.0)
            })
            .collect(), 30.0 * eps);
    // A square with four symmetric two-arc holes.
    {
        let sq = vec![
            p2(0.0, 0.0, 0.0),
            p2(2.0, 0.0, 0.0),
            p2(2.0, 2.0, 0.0),
            p2(0.0, 2.0, 0.0),
        ];
        let centres = [(0.5, 0.5), (1.5, 0.5), (0.5, 1.5), (1.5, 1.5)];
        let mut loops = vec![profile::test_support::bulge_loop(sq.clone())];
        for &(x, y) in &centres {
            loops.push(profile::test_support::bulge_loop(
                sweep::test_support::arc_polygon(2, 0.25, Point2::new(x, y)),
            ));
        }
        let body = extruded(profile::SketchPlane::xy(), loops, 0.5, tol);
        out.push((
            "square with 4 holes".into(),
            body,
            Box::new(move |p: Point3<f64>| {
                p.z > 0.0
                    && p.z < 0.5
                    && in_bulge_loop(&sq, (p.x, p.y))
                    && centres
                        .iter()
                        .all(|&(x, y)| (p.x - x).powi(2) + (p.y - y).powi(2) > 0.0625)
            }),
        ));
    }
    let rr = |p: Point3<f64>| ((p.x - 0.5).powi(2) + (p.y - 0.5).powi(2)).sqrt();
    let cube = move |p: Point3<f64>, h: f64| {
        p.x > 0.0 && p.x < 1.0 && p.y > 0.0 && p.y < 1.0 && p.z > 0.0 && p.z < h
    };
    for (n, r, h, what) in [
        (3, 0.3, 1.0, "bored block 3 arcs"),
        (4, 0.3, 1.0, "bored block 4 arcs"),
        (2, 0.5 - 1e-6, 1.0, "bored block near tangent 1e-6"),
        (2, 0.5 - 30.0 * eps, 1.0, "bored block near tangent 30eps"),
        (2, 0.3, 30.0 * eps, "bored sliver plate 30eps"),
        (2, 0.3, 0.05, "bored thin plate"),
    ] {
        // A gap of 30ε is tangential contact in a sketch at the coarse ε.
        if (what.ends_with("30eps") && eps > 1e-8) || (0.5 - r) < 100.0 * eps {
            continue;
        }
        let body = bored_block_of_arcs(n, 1.0, h, r, tol);
        out.push((
            what.into(),
            body,
            Box::new(move |p| cube(p, h) && rr(p) > r),
        ));
    }
    let rod = |r: f64| move |p: Point3<f64>| (p.x * p.x + p.y * p.y).sqrt() < r;
    // A tube: two full walls, each split at its arcs.
    if let Some(body) = op(
        "subtract",
        &disc_of_arcs(2, 1.0, 1.0, tol),
        &cylinder_of_arcs_at(3, 0.5, Point2::new(0.0, 0.0), -0.5, 2.0, tol),
        tol,
    ) {
        out.push((
            "tube".into(),
            body,
            Box::new(move |p| p.z > 0.0 && p.z < 1.0 && rod(1.0)(p) && !rod(0.5)(p)),
        ));
    }
    if let Some(body) = op(
        "subtract",
        &disc_of_arcs(2, 1.0, 1.0, tol),
        &cylinder_of_arcs_at(2, 1.0 - 30.0 * eps, Point2::new(0.0, 0.0), -0.5, 2.0, tol),
        tol,
    ) {
        let inner = 1.0 - 30.0 * eps;
        out.push((
            "thin tube 30eps".into(),
            body,
            Box::new(move |p| p.z > 0.0 && p.z < 1.0 && rod(1.0)(p) && !rod(inner)(p)),
        ));
    }
    // Partial walls: rulings and rims.
    for (what, xb, yb) in [
        ("D rod", (0.3, 2.0), (-2.0, 2.0)),
        ("quarter rod", (0.0, 2.0), (0.0, 2.0)),
        ("rod less a quarter", (-2.0, 2.0), (-2.0, 2.0)),
        ("rod sliced near tangent", (1.0 - 1e-4, 2.0), (-2.0, 2.0)),
        ("rod sliced 30eps", (1.0 - 30.0 * eps, 2.0), (-2.0, 2.0)),
    ] {
        let disc = disc_of_arcs(2, 1.0, 1.0, tol);
        let body = if what == "rod less a quarter" {
            op(
                "subtract",
                &disc,
                &brick((0.0, 2.0), (0.0, 2.0), (-1.0, 2.0), tol),
                tol,
            )
        } else {
            op(
                "intersect",
                &disc,
                &brick(xb, yb, (-1.0, 2.0), tol),
                tol,
            )
        };
        if let Some(body) = body {
            let less = what == "rod less a quarter";
            out.push((
                what.into(),
                body,
                Box::new(move |p| {
                    let boxed = p.x > xb.0 && p.x < xb.1 && p.y > yb.0 && p.y < yb.1;
                    let quarter = p.x > 0.0 && p.y > 0.0;
                    p.z > 0.0
                        && p.z < 1.0
                        && rod(1.0)(p)
                        && if less { !quarter } else { boxed }
                }),
            ));
        }
    }
    // Shells with cavities, the voided rod's pose among them.
    if let Some(body) = op(
        "subtract",
        &brick((0.0, 3.0), (0.0, 3.0), (0.0, 3.0), tol),
        &cylinder_of_arcs_at(2, 0.5, Point2::new(1.5, 1.5), 1.0, 1.0, tol),
        tol,
    ) {
        out.push((
            "voided rod".into(),
            body,
            Box::new(move |p| {
                let inb = p.x > 0.0 && p.x < 3.0 && p.y > 0.0 && p.y < 3.0 && p.z > 0.0 && p.z < 3.0;
                let inr = ((p.x - 1.5).powi(2) + (p.y - 1.5).powi(2)).sqrt() < 0.5 && p.z > 1.0 && p.z < 2.0;
                inb && !inr
            }),
        ));
    }
    let big = brick((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0), tol);
    let in_big = |p: Point3<f64>| p.x.abs() < 1.0 && p.y.abs() < 1.0 && p.z.abs() < 1.0;
    if let Some(body) = op(
        "subtract",
        &big,
        &brick((-0.3, 0.3), (-0.3, 0.3), (-0.3, 0.3), tol),
        tol,
    ) {
        out.push((
            "box with a cube cavity".into(),
            body,
            Box::new(move |p| {
                in_big(p) && !(p.x.abs() < 0.3 && p.y.abs() < 0.3 && p.z.abs() < 0.3)
            }),
        ));
    }
    if let Some(body) = op(
        "subtract",
        &big,
        &cylinder_of_arcs_at(2, 0.3, Point2::new(0.0, 0.0), -0.4, 0.8, tol),
        tol,
    ) {
        out.push((
            "box with a rod cavity".into(),
            body,
            Box::new(move |p| in_big(p) && !(rod(0.3)(p) && p.z > -0.4 && p.z < 0.4)),
        ));
    }
    if let Some(body) = op(
        "subtract",
        &big,
        &cylinder_of_arcs_at(3, 0.3, Point2::new(0.1, -0.2), -0.4, 0.8, tol),
        tol,
    ) {
        out.push((
            "box with an off-centre rod cavity".into(),
            body,
            Box::new(move |p| {
                in_big(p)
                    && !(((p.x - 0.1).powi(2) + (p.y + 0.2).powi(2)).sqrt() < 0.3
                        && p.z > -0.4
                        && p.z < 0.4)
            }),
        ));
    }
    out
}

/// A ray's crossings gathered into clusters, as the curved edge-meeting
/// ray's pins read them.
fn clusters(mut hits: Vec<(f64, f64, i32, bool)>, gap: f64) -> (Vec<(f64, f64, i32)>, bool) {
    hits.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut out: Vec<(f64, f64, i32, bool)> = Vec::new();
    for (lo, hi, c, s) in hits {
        match out.last_mut() {
            Some(last) if lo <= last.1 + gap => {
                *last = (last.0, last.1.max(hi), last.2 + c, last.3 || s);
            }
            _ => out.push((lo, hi, c, s)),
        }
    }
    let stop = out.iter().position(|c| c.3);
    let read = out[..stop.unwrap_or(out.len())]
        .iter()
        .map(|&(lo, hi, c, _)| (lo, hi, c))
        .collect();
    (read, stop.is_some())
}

/// Whether every cluster of the ray counts right against `truth` (the
/// curved edge-meeting ray's `whole_ray`, one face order).
fn whole_ray(
    body: &Body<f64>,
    face: topo::FaceKey,
    (q, d): (Point3<f64>, Vec3<f64>),
    (truth, inside_out): (&Truth, bool),
    band: Band,
    clear: f64,
) -> Result<bool, String> {
    let material = |p: Point3<f64>| robust(truth, p, clear).map(|m| m != inside_out);
    let Ok(hits) = topo::test_support::ray_counts(body, face, (q, d), band, (0, false)) else {
        return Ok(false);
    };
    let (cl, stopped) = clusters(hits, 2.0 * clear);
    let Some(mut m) = material(q) else {
        return Ok(false);
    };
    let mut prev = 0.0;
    for &(lo, hi, c) in &cl {
        if lo - prev > 4.0 * clear
            && let Some(at) = material(q + d * (0.5 * (prev + lo)))
            && at != m
        {
            return Err(format!("{cl:?}: material {at} at {}, counted {m}", 0.5 * (prev + lo)));
        }
        m = match (m, c) {
            (_, 0) => m,
            (true, 1) => false,
            (false, -1) => true,
            _ => return Err(format!("{cl:?}: a count of {c} from material {m}")),
        };
        prev = hi;
    }
    if !stopped
        && let Some(at) = material(q + d * (prev + 1.0))
        && at != m
    {
        return Err(format!("{cl:?}: ends counted {m}, material {at} beyond"));
    }
    Ok(!stopped)
}

fn bbox(body: &Body<f64>) -> (Point3<f64>, Point3<f64>) {
    let mut lo = Point3::new(f64::MAX, f64::MAX, f64::MAX);
    let mut hi = Point3::new(f64::MIN, f64::MIN, f64::MIN);
    for (_, v) in body.vertices() {
        let p = *body.get_point(v.point).expect("a point");
        lo = Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
        hi = Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
    }
    // Round bodies' vertices sit on their rims; widen by the extent.
    let w = (hi - lo).norm() * 0.6;
    let c = Point3::new((lo.x + hi.x) / 2.0, (lo.y + hi.y) / 2.0, (lo.z + hi.z) / 2.0);
    (
        Point3::new(c.x - w, c.y - w, c.z - w),
        Point3::new(c.x + w, c.y + w, c.z + w),
    )
}

#[test]
#[ignore = "review probe for PR 5781: run by hand"]
fn review_5781_sides() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the band");
    let k_eps = tol.k() * tol.eps();
    let rays: usize = std::env::var("REVIEW_RAYS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(400);
    let only = std::env::var("REVIEW_ONLY").ok();
    let mut all = bodies(tol);
    if std::env::var("REVIEW_CORPUS").is_ok() {
        all.extend(corpus(tol));
    }
    for (name, body, truth) in all {
        if only.as_ref().is_some_and(|o| !name.contains(o.as_str())) {
            continue;
        }
        let mut rng = test_utils::fuzz::Rng::from_seed(5781);
        let check = topo::validate_pseudomanifold(&body, &ContactRecords::default(), tol);
        println!("CHECK eps={:e} {name}: {check:?}", tol.eps());
        let reverted = body.revert().expect("the body reverts");
        for (inside_out, b) in [(false, &body), (true, &reverted)] {
            let tag = if inside_out { "rev" } else { "fwd" };
            for line in topo::test_support::each_aim(b, band) {
                println!("AIM eps={:e} {name} {tag}: {line}", tol.eps());
            }
            if let Ok(at) = topo::AtRestBody::validate(b.clone(), tol) {
                let roles = topo::classify_shells(&at, tol).map(|c| {
                    c.iter()
                        .map(|s| format!("{:?}", s.role))
                        .collect::<Vec<_>>()
                });
                println!("ROLES eps={:e} {name} {tag}: {roles:?}", tol.eps());
            } else {
                println!("ROLES eps={:e} {name} {tag}: not at rest", tol.eps());
            }
            let face = b.faces().next().expect("a face").0;
            let (lo, hi) = bbox(b);
            let (mut wrong, mut read, mut first) = (0, 0, Vec::new());
            for k in 0..rays {
                let p = Point3::new(
                    rng.range(lo.x, hi.x),
                    rng.range(lo.y, hi.y),
                    rng.range(lo.z, hi.z),
                );
                let mut d = Vec3::new(
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                );
                // A third of the rays shallow to the xy planes, a third
                // nearly along z (a wall's axis).
                match k % 3 {
                    1 => d.z *= 1e-3,
                    2 => {
                        d.x *= 1e-3;
                        d.y *= 1e-3;
                    }
                    _ => {}
                }
                let d = d.normalize();
                match whole_ray(b, face, (p - d * 8.0, d), (&truth, inside_out), band, 10.0 * k_eps)
                {
                    Ok(true) => read += 1,
                    Ok(false) => {}
                    Err(w) => {
                        wrong += 1;
                        if first.len() < 3 {
                            first.push(format!("q={:?} d={d:?}: {w}", p - d * 8.0));
                        }
                    }
                }
            }
            println!(
                "RAYS eps={:e} {name} {tag}: {wrong} wrong, {read} read whole, of {rays} {first:?}",
                tol.eps()
            );
            if !inside_out {
                let (mut bad, mut on, mut err, mut first) = (0, 0, 0, Vec::new());
                for _ in 0..rays {
                    let p = Point3::new(
                        rng.range(lo.x, hi.x),
                        rng.range(lo.y, hi.y),
                        rng.range(lo.z, hi.z),
                    );
                    let Some(t) = robust(&truth, p, 10.0 * k_eps) else {
                        continue;
                    };
                    match topo::point_in_solid(&body, p, band, tol) {
                        Ok(topo::SolidContainment::In) if !t => bad += 1,
                        Ok(topo::SolidContainment::Out) if t => bad += 1,
                        Ok(topo::SolidContainment::OnBoundary) => on += 1,
                        Err(e) => {
                            err += 1;
                            if first.len() < 2 {
                                first.push(format!("{p:?}: {:.120}", format!("{e:?}")));
                            }
                        }
                        Ok(_) => {}
                    }
                }
                println!(
                    "POINTS eps={:e} {name}: {bad} wrong, {on} on boundary, {err} refused {first:?}",
                    tol.eps()
                );
            }
        }
    }
}

/// Rays about every edge of `body` (the curved edge-meeting ray's
/// `rays_about_edges`, one face order): points within 3Kε of each edge,
/// along its face and off it, a random, a shallow and a nearly-along ray
/// through each, from 8 m and 5 cm back.
fn rays_about_edges(
    body: &Body<f64>,
    truth: &Truth,
    rng: &mut test_utils::fuzz::Rng,
    per: usize,
    tol: Tol,
) -> (usize, usize, Vec<String>) {
    let band = Band::linear(tol).expect("the run's band");
    let k_eps = tol.k() * tol.eps();
    let reverted = body.revert().expect("the body reverts");
    let (mut n, mut read, mut wrong) = (0, 0, Vec::new());
    for (inside_out, b) in [(false, body), (true, &reverted)] {
        let fractions: Vec<f64> = (0..per).map(|_| rng.range(0.02, 0.98)).collect();
        for f in topo::test_support::edge_frames(b, &fractions, band) {
            let p = f.point
                + f.into * (rng.range(-3.0, 3.0) * k_eps)
                + f.normal * (rng.range(-3.0, 3.0) * k_eps);
            let along = f.normal.cross(f.into);
            let random = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0));
            let shallow = f.into * rng.range(-1.0, 1.0)
                + along * rng.range(-1.0, 1.0)
                + f.normal * rng.range(-0.02, 0.02);
            for d in [random, shallow, along + random * 0.05] {
                let d = d.normalize();
                for back in [8.0, 0.05] {
                    n += 1;
                    match whole_ray(b, f.face, (p - d * back, d), (truth, inside_out), band, 10.0 * k_eps) {
                        Ok(true) => read += 1,
                        Ok(false) => {}
                        Err(w) => {
                            if wrong.len() < 3 {
                                wrong.push(format!("{} (io {inside_out}) q={:?} d={d:?}: {w}", f.label, p - d * back));
                            } else {
                                wrong.push(String::new());
                            }
                        }
                    }
                }
            }
        }
    }
    (n, read, wrong)
}

#[test]
#[ignore = "review probe for PR 5781: run by hand"]
fn review_5781_edges() {
    let tol = Tol::witness();
    let per: usize = std::env::var("REVIEW_PER").ok().and_then(|s| s.parse().ok()).unwrap_or(6);
    let mut all = bodies(tol);
    if std::env::var("REVIEW_CORPUS").is_ok() {
        all.extend(corpus(tol));
    }
    for (name, body, truth) in all {
        let mut rng = test_utils::fuzz::Rng::from_seed(5781);
        let (n, read, wrong) = rays_about_edges(&body, &truth, &mut rng, per, tol);
        let shown: Vec<_> = wrong.iter().filter(|w| !w.is_empty()).collect();
        println!(
            "EDGES eps={:e} {name}: {} wrong, {read} read whole, of {n} {shown:?}",
            tol.eps(),
            wrong.len()
        );
    }
}

type S = geom_core::sym::Sym<f64>;

#[test]
#[ignore = "review probe for PR 5781: run by hand"]
fn review_5781_sym_ties() {
    use geom_core::Real as _;
    use geom_core::sym::{SymBudget, revalue::record, with_session};
    let tol = Tol::witness();
    let s = |x: f64| S::from_f64(x);
    let pts = |v: &[(f64, f64, f64)]| -> Vec<(Point2<S>, S)> {
        v.iter()
            .map(|&(x, y, b)| (Point2::new(s(x), s(y)), s(b)))
            .collect()
    };
    let q = quarter();
    let hex: Vec<(f64, f64, f64)> = (0..6)
        .map(|k| {
            let a = f64::from(k) * PI / 3.0;
            (a.cos(), a.sin(), 0.0)
        })
        .collect();
    let cases: Vec<(&str, Vec<(f64, f64, f64)>)> = vec![
        ("square", vec![(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 1.0, 0.0), (0.0, 1.0, 0.0)]),
        (
            "L",
            vec![
                (0.0, 0.0, 0.0),
                (2.0, 0.0, 0.0),
                (2.0, 1.0, 0.0),
                (1.0, 1.0, 0.0),
                (1.0, 2.0, 0.0),
                (0.0, 2.0, 0.0),
            ],
        ),
        (
            "U",
            vec![
                (0.0, 0.0, 0.0),
                (3.0, 0.0, 0.0),
                (3.0, 2.0, 0.0),
                (2.0, 2.0, 0.0),
                (2.0, 1.0, 0.0),
                (1.0, 1.0, 0.0),
                (1.0, 2.0, 0.0),
                (0.0, 2.0, 0.0),
            ],
        ),
        (
            "plus",
            vec![
                (1.0, 0.0, 0.0),
                (2.0, 0.0, 0.0),
                (2.0, 1.0, 0.0),
                (3.0, 1.0, 0.0),
                (3.0, 2.0, 0.0),
                (2.0, 2.0, 0.0),
                (2.0, 3.0, 0.0),
                (1.0, 3.0, 0.0),
                (1.0, 2.0, 0.0),
                (0.0, 2.0, 0.0),
                (0.0, 1.0, 0.0),
                (1.0, 1.0, 0.0),
            ],
        ),
        ("hex", hex),
        ("stadium", vec![(0.0, 0.0, 0.0), (2.0, 0.0, 1.0), (2.0, 1.0, 0.0), (0.0, 1.0, 1.0)]),
        (
            "rounded square",
            vec![
                (0.2, 0.0, 0.0),
                (1.8, 0.0, q),
                (2.0, 0.2, 0.0),
                (2.0, 1.8, q),
                (1.8, 2.0, 0.0),
                (0.2, 2.0, q),
                (0.0, 1.8, 0.0),
                (0.0, 0.2, q),
            ],
        ),
        ("disc of 2 arcs", vec![(1.0, 0.0, 1.0), (-1.0, 0.0, 1.0)]),
        ("disc of 4 arcs", vec![(1.0, 0.0, q), (0.0, 1.0, q), (-1.0, 0.0, q), (0.0, -1.0, q)]),
        ("lune", vec![(0.0, 0.0, 0.6), (2.0, 0.0, 0.6)]),
    ];
    for (what, v) in cases {
        let v = pts(&v);
        let f = || {
            let body = prism_at(v.clone(), s(0.0), s(0.5), tol);
            let at = topo::AtRestBody::validate(body, tol);
            at.map(|a| topo::classify_shells_structural(&a, tol).map_or(99, |c| c.len()))
                .unwrap_or(98)
        };
        let ((shells, decisions), _) = with_session(
            SymBudget {
                max_terms: 4096,
                max_degree: 128,
            },
            || record(&f),
        );
        let family: Vec<_> = decisions
            .iter()
            .filter(|d| d.family == Some(topo::SIDE_READER_FAMILY))
            .collect();
        let ties: Vec<_> = family
            .iter()
            .filter(|d| d.value == Some(0.0))
            .map(|d| d.predicate)
            .collect();
        println!(
            "SYM-TIES eps={:e} {what}: shells {shells}, {} reader decisions, {} ties {ties:?}",
            tol.eps(),
            family.len(),
            ties.len()
        );
    }
}

/// **The P1's pose, every face aimed** (`split_seam_vanishing_tilt`):
/// each aim's own reading of the split's joined scratch body, which holds
/// both sides. A `Side(In)` on a solid the split orients right is the
/// defect the P1 row names, reached by whichever face's aim the reader
/// takes first.
#[test]
#[ignore = "review probe for PR 5781: run by hand"]
fn review_5781_seam_tilt_aims() {
    use geom_core::Affine3;
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the band");
    for phi in [2.2_f64, 0.9, 0.3, 1.7, 2.9, 4.0, 5.5] {
        let circle = vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)];
        let body = topo::AtRestBody::validate(sweep::test_support::prism(circle, 1.0, tol), tol)
            .expect("the unit cylinder");
        let turn = Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            phi,
        );
        let turned = topo::transform_rigid(&body, &geom_core::test_support::rigid(&turn, tol), tol)
            .expect("a rigid turn");
        let cyl = topo::AtRestBody::validate(turned, tol).expect("the turned cylinder");
        let (c, s) = (phi.cos(), phi.sin());
        let v = Point3::new(c, s, 0.0);
        let along = Vec3::new(-s, c, 0.0);
        for psi in [1e-7_f64, 1.1e-7, 1e-6, 1e-5] {
            for l in [0.0, 0.02, 0.1, 0.18, 0.5] {
                for sign in [1.0, -1.0] {
                    let pose = format!("φ {phi} ψ {psi:e} L {l} sign {sign}");
                    let n = Vec3::new(c * psi.cos(), s * psi.cos(), psi.sin()) * sign;
                    let plane = topo::test_support::split_plane(v + along * l, n, tol);
                    let r = topo::splitting::split(&cyl, &plane, tol);
                    let verdict = match &r {
                        Ok(_) => "Ok".to_owned(),
                        Err(e) => format!("{e}").chars().take(140).collect(),
                    };
                    println!("SPLIT eps={:e} {pose}: {verdict}", tol.eps());
                    if let Ok(joined) = topo::test_support::split_through_the_join(&cyl, &plane, tol)
                    {
                        let aims = topo::test_support::each_aim(&joined, band);
                        let ins = aims.iter().filter(|a| a.contains("Side(In")).count();
                        let outs = aims.iter().filter(|a| a.contains("Side(Out")).count();
                        println!(
                            "SEAMAIMS eps={:e} {pose}: {} aims, {outs} Out, {ins} In {:?}",
                            tol.eps(),
                            aims.len(),
                            aims.iter().filter(|a| a.contains("Side(In")).collect::<Vec<_>>()
                        );
                    }
                }
            }
        }
    }
}

/// The seam-tilt split at chosen poses, its refusal in full: run with
/// `REVIEW_TRACE` on the instrumented mutant to see check 7's readings.
#[test]
#[ignore = "review probe for PR 5781: run by hand"]
fn review_5781_seam_tilt_finding() {
    use geom_core::Affine3;
    let tol = Tol::witness();
    let poses: Vec<(f64, f64, f64, f64)> = vec![
        (5.5, 1e-5, 0.0, 1.0),
        (5.5, 1e-5, 0.18, 1.0),
        (2.2, 1e-7, 0.0, 1.0),
    ];
    for (phi, psi, l, sign) in poses {
        let circle = vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)];
        let body = topo::AtRestBody::validate(sweep::test_support::prism(circle, 1.0, tol), tol)
            .expect("the unit cylinder");
        let turn = Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            phi,
        );
        let turned = topo::transform_rigid(&body, &geom_core::test_support::rigid(&turn, tol), tol)
            .expect("a rigid turn");
        let cyl = topo::AtRestBody::validate(turned, tol).expect("the turned cylinder");
        let (c, s) = (phi.cos(), phi.sin());
        let v = Point3::new(c, s, 0.0);
        let along = Vec3::new(-s, c, 0.0);
        let n = Vec3::new(c * psi.cos(), s * psi.cos(), psi.sin()) * sign;
        let plane = topo::test_support::split_plane(v + along * l, n, tol);
        eprintln!("REVIEW-TRACE pose φ {phi} ψ {psi:e} L {l} sign {sign}");
        match topo::splitting::split(&cyl, &plane, tol) {
            Ok(r) => {
                let parts = [&r.above, &r.below]
                    .iter()
                    .filter(|p| matches!(p, topo::splitting::SplitPart::Body(_)))
                    .count();
                println!("FINDING eps={:e} φ {phi} ψ {psi:e} L {l} sign {sign}: Ok, {parts} parts", tol.eps());
            }
            Err(e) => println!("FINDING eps={:e} φ {phi} ψ {psi:e} L {l} sign {sign}: {e:?}", tol.eps()),
        }
    }
}

/// **The seam plane of every revolve in the corpus**, turned about `z`:
/// `point_in_solid` on a grid in `z = 0`, the plane holding the revolve
/// axis (`y`) and its seam, against the analytic truth. The schedule's
/// in-plane rays run along the seam ruling there.
#[test]
#[ignore = "review probe for PR 5781: run by hand"]
fn review_5781_seam_plane() {
    use geom_core::{RigidMotion, UnitVec3};
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the band");
    let k_eps = tol.k() * tol.eps();
    let z = UnitVec3::new(Vec3::new(0.0, 0.0, 1.0), "z", band).unwrap();
    for (name, body, truth) in corpus(tol) {
        if !name.starts_with("rev") {
            continue;
        }
        for angle in [0.0_f64, 0.3, 1.0, -0.7, 2.0] {
            let m = RigidMotion::rotation(z, angle).unwrap();
            let Ok(turned) = topo::transform_rigid(&body, &m, tol) else {
                println!("SEAMPLANE {name} turned {angle}: no turn");
                continue;
            };
            let (s, c) = angle.sin_cos();
            let back = |q: Point3<f64>| Point3::new(c * q.x + s * q.y, -s * q.x + c * q.y, q.z);
            let (mut wrong, mut read, mut refused, mut first) = (0, 0, 0, Vec::new());
            for i in 0..41 {
                for j in 0..41 {
                    let q = Point3::new(
                        -2.5 + 5.0 * f64::from(i) / 40.0 + 0.0013,
                        -2.5 + 5.0 * f64::from(j) / 40.0 + 0.0007,
                        0.0,
                    );
                    let Some(t) = robust(&truth, back(q), 10.0 * k_eps) else {
                        continue;
                    };
                    match topo::point_in_solid(&turned, q, band, tol) {
                        Ok(topo::SolidContainment::In) if t => read += 1,
                        Ok(topo::SolidContainment::Out) if !t => read += 1,
                        Err(_) | Ok(topo::SolidContainment::OnBoundary) => refused += 1,
                        got => {
                            wrong += 1;
                            if first.len() < 2 {
                                first.push(format!("{q:?} truth {t}: {got:?}"));
                            }
                        }
                    }
                }
            }
            println!(
                "SEAMPLANE eps={:e} {name} turned {angle}: {wrong} wrong, {read} right, {refused} refused {first:?}",
                tol.eps()
            );
        }
    }
}
