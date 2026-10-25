//! Review R1 probes for PR 5697 (not for merge): every crossing of a ray
//! about a curved edge, read along the whole ray, not only its first
//! cluster, under face-order permutations.

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use core::f64::consts::PI;

use geom_core::{Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{
    ball_poled_z, bored_block_of_arcs, boss_of_arcs, brick, disc_of_arcs, hemisphere_on_flat_base,
    prism, revolved_about_y, sphere_zone, spool, waisted,
};
use topo::Body;

type Truth = Option<Box<dyn Fn(Point3<f64>) -> bool>>;

/// 2-D point in polygon over (rho, y) for a Full revolve of straight
/// segments.
fn in_profile(poly: &[(f64, f64)], rho: f64, y: f64) -> bool {
    let mut inside = false;
    let n = poly.len();
    for i in 0..n {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i + 1) % n];
        if (y1 > y) != (y2 > y) && rho < x1 + (y - y1) * (x2 - x1) / (y2 - y1) {
            inside = !inside;
        }
    }
    inside
}

fn rev_lines(pts: &[(f64, f64)], tol: Tol) -> (Body<f64>, Truth) {
    let body = revolved_about_y(
        pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect(),
        Revolution::Full,
        tol,
    );
    let poly = pts.to_vec();
    (
        body,
        Some(Box::new(move |p: Point3<f64>| {
            in_profile(&poly, (p.x * p.x + p.z * p.z).sqrt(), p.y)
        })),
    )
}

fn fixtures(tol: Tol) -> Vec<(String, Body<f64>, Truth)> {
    let quarter = (PI / 8.0).tan();
    let tube = || vec![(Point2::new(2.5, 0.0), 1.0), (Point2::new(1.5, 0.0), 1.0)];
    let mut out: Vec<(String, Body<f64>, Truth)> = Vec::new();
    let mut push = |n: &str, b: Body<f64>, t: Truth| out.push((n.to_owned(), b, t));
    let cyl = |p: Point3<f64>| (p.x * p.x + p.y * p.y).sqrt() < 0.5 && p.z > 0.0 && p.z < 1.0;
    push("cyl2", disc_of_arcs(2, 0.5, 1.0, tol), Some(Box::new(cyl)));
    push("cyl3", disc_of_arcs(3, 0.5, 1.0, tol), Some(Box::new(cyl)));
    push("cyl5", disc_of_arcs(5, 0.5, 1.0, tol), Some(Box::new(cyl)));
    push("boss", boss_of_arcs(2, 1.0, 0.3, 0.5, 1.0, tol), None);
    push("bored", bored_block_of_arcs(2, 1.0, 1.0, 0.3, tol), None);
    push(
        "fillet",
        prism(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(1.0, 0.0), 0.0),
                (Point2::new(1.0, 0.8), quarter),
                (Point2::new(0.8, 1.0), 0.0),
                (Point2::new(0.0, 1.0), 0.0),
            ],
            1.0,
            tol,
        ),
        None,
    );
    let (b, t) = rev_lines(&[(0.0, 0.0), (1.0, 0.0), (0.6, 1.0), (0.0, 1.0)], tol);
    push("frustum", b, t);
    let (b, t) = rev_lines(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)], tol);
    push("cone-apex", b, t);
    let (b, t) = rev_lines(&[(0.0, 0.0), (0.2, 0.0), (0.0, 3.0)], tol);
    push("needle-cone", b, t);
    let (b, t) = rev_lines(&[(0.0, 0.0), (1.0, 0.0), (0.4, 0.5), (1.0, 1.0), (0.0, 1.0)], tol);
    push("hourglass-cones", b, t);
    let (b, t) = rev_lines(&[(0.5, 0.0), (1.0, 0.0), (1.0, 1.0), (0.5, 1.0)], tol);
    push("tube", b, t);
    push(
        "ball",
        ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol),
        Some(Box::new(|p: Point3<f64>| (p - Point3::new(0.0, 0.0, 0.0)).norm() < 1.0)),
    );
    // Two arcs of one circle: a ball of two sphere faces stacked.
    let b = quarter;
    push(
        "ball-2arcs",
        revolved_about_y(
            vec![
                (Point2::new(0.0, -1.0), b),
                (Point2::new(1.0, 0.0), b),
                (Point2::new(0.0, 1.0), 0.0),
            ],
            Revolution::Full,
            tol,
        ),
        Some(Box::new(|p: Point3<f64>| (p - Point3::new(0.0, 0.0, 0.0)).norm() < 1.0)),
    );
    // Four arcs: a sphere in four zones.
    let b8 = (PI / 16.0).tan();
    let s = |a: f64| Point2::new(a.cos(), a.sin());
    push(
        "ball-4arcs",
        revolved_about_y(
            vec![
                (s(-PI / 2.0), b8),
                (s(-PI / 4.0), b8),
                (s(0.0), b8),
                (s(PI / 4.0), b8),
                (Point2::new(0.0, 1.0), 0.0),
            ],
            Revolution::Full,
            tol,
        ),
        Some(Box::new(|p: Point3<f64>| (p - Point3::new(0.0, 0.0, 0.0)).norm() < 1.0)),
    );
    push(
        "ball-slit",
        revolved_about_y(
            vec![
                (Point2::new(0.0, -1.0), quarter),
                (Point2::new(1.0, 0.0), quarter),
                (Point2::new(0.0, 1.0), 0.0),
            ],
            Revolution::Partial(1.9 * PI),
            tol,
        ),
        None,
    );
    push(
        "tube-slit",
        revolved_about_y(
            vec![
                (Point2::new(0.5, 0.0), 0.0),
                (Point2::new(1.0, 0.0), 0.0),
                (Point2::new(1.0, 1.0), 0.0),
                (Point2::new(0.5, 1.0), 0.0),
            ],
            Revolution::Partial(1.95 * PI),
            tol,
        ),
        None,
    );
    push("hemisphere", hemisphere_on_flat_base(1.0, tol), None);
    push("torus", revolved_about_y(tube(), Revolution::Full, tol), None);
    push(
        "half-torus",
        revolved_about_y(tube(), Revolution::Partial(PI), tol),
        None,
    );
    push(
        "torus-3q",
        revolved_about_y(tube(), Revolution::Partial(1.5 * PI), tol),
        None,
    );
    push("sphere-zone", sphere_zone(0.4, Revolution::Full, tol), None);
    push("sphere-zone-p", sphere_zone(0.4, Revolution::Partial(PI), tol), None);
    push("spool", spool(Revolution::Full, tol), None);
    push("waisted", waisted(tol), None);
    let frustum = revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(0.5, 0.0), 0.0),
            (Point2::new(0.3, 0.5), 0.0),
            (Point2::new(0.0, 0.5), 0.0),
        ],
        Revolution::Full,
        tol,
    );
    let block = brick::<f64>((-1.0, 1.0), (-1.0, 0.0), (-1.0, 1.0), tol);
    let at_rest = |b: &Body<f64>| topo::AtRestBody::validate(b.clone(), tol).expect("at rest");
    if let Ok(r) = topo::union(&at_rest(&block), &at_rest(&frustum), tol)
        && let Some(b) = r.body()
    {
        push("frustum-on-box", b.body.clone().into_body(), None);
    }
    let ball = ball_poled_z(0.6, Vec3::new(0.3, 0.0, 0.2), tol);
    if let Ok(r) = topo::union(&at_rest(&block), &at_rest(&ball), tol)
        && let Some(b) = r.body()
    {
        push("ball-on-box", b.body.clone().into_body(), None);
    }
    if let Ok(r) = topo::subtract(&at_rest(&block), &at_rest(&ball), tol)
        && let Some(b) = r.body()
    {
        push("box-minus-ball", b.body.clone().into_body(), None);
    }
    let torus = revolved_about_y(
        vec![(Point2::new(0.9, 0.0), 1.0), (Point2::new(0.5, 0.0), 1.0)],
        Revolution::Full,
        tol,
    );
    if let Ok(r) = topo::subtract(&at_rest(&block), &at_rest(&torus), tol)
        && let Some(b) = r.body()
    {
        push("box-minus-torus", b.body.clone().into_body(), None);
    }
    for m in [12.0, 21.0, 30.0] {
        push(
            &format!("disc{m}"),
            disc_of_arcs(2, 0.5, m * tol.eps(), tol),
            None,
        );
    }
    out
}

/// Reads one ray's hits: `Err` where the walk faults. Otherwise, the
/// running count after each cluster (hits whose spans lie within `gap`
/// of each other), stopping at a sideless cluster; with the midpoints
/// between clusters.
fn clusters(mut hits: Vec<(f64, f64, i32, bool)>, gap: f64) -> (Vec<(f64, f64, i32)>, bool) {
    hits.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut out: Vec<(f64, f64, i32)> = Vec::new();
    let mut cur: Option<(f64, f64, i32)> = None;
    let mut sideless = false;
    for (lo, hi, c, s) in hits {
        match cur {
            Some((clo, chi, cc)) if lo <= chi + gap => {
                cur = Some((clo, chi.max(hi), cc + c));
                if s {
                    sideless = true;
                }
            }
            _ => {
                if let Some(x) = cur.take() {
                    if sideless {
                        return (out, true);
                    }
                    out.push(x);
                }
                sideless = s;
                cur = Some((lo, hi, c));
            }
        }
    }
    if let Some(x) = cur {
        if sideless {
            return (out, true);
        }
        out.push(x);
    }
    (out, false)
}

#[test]
#[ignore = "review probe"]
fn review_whole_ray_counts_about_curved_edges() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let k_eps = tol.k() * tol.eps();
    let mut rng = test_utils::fuzz::start("review_whole_ray_counts_about_curved_edges");
    let per = std::env::var("R1_PER")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4usize);
    let only = std::env::var("R1_ONLY").ok();
    let mut bad_total = 0;
    for (name, body, truth) in fixtures(tol) {
        if only.as_deref().is_some_and(|o| !name.contains(o)) {
            continue;
        }
        let tier = topo::validate_pseudomanifold(&body, &topo::ContactRecords::default(), tol);
        let reverted = body.revert().expect("reverts");
        let (mut n, mut fault, mut sideless_full, mut full, mut bad, mut perm_diff) =
            (0, 0, 0, 0, 0, 0);
        for (inside_out, b) in [(false, &body), (true, &reverted)] {
            let fr: Vec<f64> = (0..per).map(|_| rng.range(0.02, 0.98)).collect();
            for f in topo::test_support::edge_frames(b, &fr, band) {
                for kind in 0..3 {
                    let o1 = rng.range(-3.0, 3.0) * k_eps;
                    let o2 = rng.range(-3.0, 3.0) * k_eps;
                    let p = f.point + f.into * o1 + f.normal * o2;
                    let tang = f.normal.cross(f.into);
                    let rnd = Vec3::new(
                        rng.range(-1.0, 1.0),
                        rng.range(-1.0, 1.0),
                        rng.range(-1.0, 1.0),
                    );
                    let d = match kind {
                        0 => rnd.normalize(),
                        // shallow to the face
                        1 => (f.into * rng.range(-1.0, 1.0)
                            + tang * rng.range(-1.0, 1.0)
                            + f.normal * rng.range(-0.02, 0.02))
                        .normalize(),
                        // along the edge, nearly tangent
                        _ => (tang + rnd * 0.05).normalize(),
                    };
                    let q = p - d * 8.0;
                    n += 1;
                    let mut readings = Vec::new();
                    for (rot, rev) in [(0, false), (1, false), (3, true), (7, true)] {
                        readings.push(topo::test_support::review_ray_counts(
                            b, f.face, q, d, band, rot, rev,
                        ));
                    }
                    let norm = |r: &Result<Vec<(f64, f64, i32, bool)>, String>| match r {
                        Ok(h) => {
                            let (c, s) = clusters(h.clone(), 2.0 * k_eps);
                            format!("{:?} {s}", c.iter().map(|x| x.2).collect::<Vec<_>>())
                        }
                        Err(_) => "fault".to_owned(),
                    };
                    let n0 = norm(&readings[0]);
                    if readings.iter().any(|r| norm(r) != n0) {
                        perm_diff += 1;
                        println!(
                            "R1 PERM {name} io={inside_out} {} q={q:?} d={d:?}: {:?}",
                            f.label,
                            readings.iter().map(norm).collect::<Vec<_>>()
                        );
                    }
                    let Ok(hits) = &readings[0] else {
                        fault += 1;
                        continue;
                    };
                    let (cl, sideless) = clusters(hits.clone(), 2.0 * k_eps);
                    if sideless {
                        sideless_full += 1;
                    } else {
                        full += 1;
                    }
                    // running count: outside 0, inside -1 (entering is -1);
                    // inside out: outside 0? the material is the
                    // complement: at infinity inside, running from 0 is
                    // the offset to infinity's side.
                    let mut running = 0;
                    let mut prev_hi = 0.0f64;
                    let mut wrong = None;
                    for (i, &(lo, hi, c)) in cl.iter().enumerate() {
                        // Truth before this cluster
                        if let Some(t) = &truth {
                            let mid = 0.5 * (prev_hi + lo);
                            if i > 0 && lo - prev_hi > 20.0 * k_eps {
                                let inside = t(q + d * mid);
                                let counted = running != 0;
                                if inside != counted {
                                    wrong = Some(format!("truth at {mid}: in={inside} running={running}"));
                                    break;
                                }
                            }
                        }
                        running += c;
                        let ok = if inside_out { [0, 1] } else { [0, -1] };
                        if !ok.contains(&running) {
                            wrong = Some(format!("running {running} after cluster {i} at {lo}"));
                            break;
                        }
                        prev_hi = hi;
                    }
                    if wrong.is_none() && !sideless && running != 0 {
                        wrong = Some(format!("ends at {running}"));
                    }
                    if let Some(w) = wrong {
                        bad += 1;
                        println!(
                            "R1 BAD {name} io={inside_out} {} q={q:?} d={d:?} kind={kind}: {w}; {:?}\n   raw {:?}\n   cast {}",
                            f.label, cl, hits,
                            topo::test_support::review_cast(b, f.face, q, d, band)
                        );
                    }
                }
            }
        }
        bad_total += bad;
        println!(
            "R1 SUM eps={:e} {name}: tier3'={} rays {n}, fault {fault}, sideless {sideless_full}, full {full}, bad {bad}, perm-diff {perm_diff}",
            tol.eps(),
            tier.is_ok()
        );
    }
    println!("R1 TOTAL bad {bad_total}; {}", test_utils::fuzz::replay());
}

/// A filleted prism (fillet tangent to the side `x = 1` at `y = 0.8`)
/// bored along `x` through that side: a ray just inside `x = 1`, shallow
/// to it, steep to the bore wall at its mouth, shallow to the fillet at
/// its tangent edge.
fn fillet_bore(tol: Tol) -> Body<f64> {
    let quarter = std::env::var("R1_BULGE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or((PI / 8.0).tan());
    let block = prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 0.8), quarter),
            (Point2::new(0.8, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        1.0,
        tol,
    );
    let plane = profile::SketchPlane::from_frame(geom_core::RigidMotion::axes_yz(Point3::new(
        -0.5, 0.0, 0.0,
    )));
    let bore = sweep::test_support::prism_on(
        plane,
        vec![(Point2::new(0.6, 0.5), 1.0), (Point2::new(0.2, 0.5), 1.0)],
        2.0,
        tol,
    );
    let at_rest = |b: &Body<f64>| topo::AtRestBody::validate(b.clone(), tol).expect("at rest");
    let r = topo::subtract(&at_rest(&block), &at_rest(&bore), tol).expect("subtract");
    r.body().expect("a body").body.clone().into_body()
}

fn fillet_bore_truth(p: Point3<f64>) -> bool {
    let in_prism = p.x > 0.0 && p.x < 1.0 && p.y > 0.0 && p.y < 1.0 && p.z > 0.0 && p.z < 1.0;
    let b: f64 = std::env::var("R1_BULGE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or((PI / 8.0).tan());
    let th = 4.0 * b.atan();
    let rr = (0.2 * 2f64.sqrt()) / (2.0 * (th / 2.0).sin());
    let off = rr * (th / 2.0).cos() / 2f64.sqrt();
    let (cx, cy) = (0.9 - off, 0.9 - off);
    let corner = p.x > 0.8 && p.y > 0.8 && (p.x - cx).powi(2) + (p.y - cy).powi(2) > rr * rr;
    let in_bore = (p.y - 0.4).powi(2) + (p.z - 0.5).powi(2) < 0.04;
    in_prism && !corner && !in_bore
}

#[test]
#[ignore = "review probe"]
fn review_fillet_bore_shallow_ray() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let k_eps = tol.k() * tol.eps();
    let body = fillet_bore(tol);
    println!(
        "R1 FB tier3' {:?}",
        topo::validate_pseudomanifold(&body, &topo::ContactRecords::default(), tol).is_ok()
    );
    let face = body.faces().next().expect("a face").0;
    let (mut bad, mut n, mut read) = (0, 0, 0);
    let phi: f64 = std::env::var("R1_PHI").ok().and_then(|s| s.parse().ok()).unwrap_or(0.6);
    let gap2 = 0.4 * phi.cos();
    for di in 1..40 {
        let delta = f64::from(di) * 0.025 * k_eps;
        for si in 0..13 {
            let s = (0.8 + f64::from(si) * 0.1) * k_eps / gap2;
            for &tz in &[1e-3, -2e-3, 0.0] {
                let p1 = Point3::new(1.0 - delta, 0.4 + 0.2 * phi.cos(), 0.5 + 0.2 * phi.sin());
                let d = Vec3::new(s, 1.0, tz).normalize();
                for back in [3.0, 0.05] {
                    let q = p1 - d * back;
                    n += 1;
                    let got = topo::test_support::review_ray_counts(&body, face, q, d, band, 0, false);
                    if std::env::var("R1_DBG").is_ok() && back > 1.0 && tz == 1e-3 && si == 4 {
                        println!("R1 FB DBG delta={:.2} {got:?}", delta / k_eps);
                    }
                    let Ok(hits) = got else {
                        continue;
                    };
                    let (cl, sideless) = clusters(hits, 2.0 * k_eps);
                    let mut running = if fillet_bore_truth(q) { -1 } else { 0 };
                    let mut prev = 0.0f64;
                    let mut wrong = None;
                    for &(lo, hi, c) in &cl {
                        if lo - prev > 20.0 * k_eps {
                            let mid = 0.5 * (prev + lo);
                            if fillet_bore_truth(q + d * mid) != (running != 0) {
                                wrong = Some(format!("at {mid} running {running}"));
                                break;
                            }
                        }
                        running += c;
                        prev = hi;
                    }
                    if wrong.is_none() && !sideless && running != 0 {
                        wrong = Some(format!("ends {running}"));
                    }
                    if !sideless {
                        read += 1;
                    }
                    if let Some(w) = wrong {
                        bad += 1;
                        println!(
                            "R1 FB BAD delta={:.3}kε s·gap={:.2}kε tz={tz} back={back}: {w}; {cl:?}",
                            delta / k_eps,
                            s * gap2 / k_eps
                        );
                    }
                }
            }
        }
    }
    println!("R1 FB eps={:e} rays {n}, fully read {read}, bad {bad}", tol.eps());
}

#[test]
#[ignore = "review probe"]
fn review_fillet_bore_one() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let k_eps = tol.k() * tol.eps();
    let body = fillet_bore(tol);
    let face = body.faces().next().expect("a face").0;
    let delta: f64 = std::env::var("R1_DELTA").ok().and_then(|s| s.parse().ok()).unwrap_or(0.9) * k_eps;
    let s04: f64 = std::env::var("R1_S").ok().and_then(|s| s.parse().ok()).unwrap_or(1.2);
    let s = s04 * k_eps / 0.4;
    let phi = 0.6f64;
    let s = s * 0.4 / (0.4 * phi.cos());
    let p1 = Point3::new(1.0 - delta, 0.4 + 0.2 * phi.cos(), 0.5 + 0.2 * phi.sin());
    let d = Vec3::new(s, 1.0, 1e-3).normalize();
    for back in [3.0, 0.05] {
        let q = p1 - d * back;
        let got = topo::test_support::review_ray_counts(&body, face, q, d, band, 0, false);
        println!(
            "R1 ONE back={back} q={q:?} truth(q)={} cast={} {got:?}",
            fillet_bore_truth(q),
            topo::test_support::review_cast(&body, face, q, d, band)
        );
    }
    for (k, (f, _)) in body.faces().enumerate() {
        let _ = k;
        println!("R1 FACE {f:?}");
    }
}

/// One ray about the 30ε disc's seam, from 8 m outside: point
/// containment's own ray reading.
#[test]
#[ignore = "review probe"]
fn review_disc30_seam_one() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let body = disc_of_arcs(2, 0.5, 30.0 * tol.eps(), tol);
    let face = body.faces().next().expect("a face").0;
    let q = Point3::new(0.431777029158541, 7.738305151126824, 2.028294585136746);
    let d = Vec3::new(0.008527870324942689, -0.9672881451929304, -0.25353682098847374);
    println!(
        "R1 D30 eps={:e} cast={} hits={:?}",
        tol.eps(),
        topo::test_support::review_cast(&body, face, q, d, band),
        topo::test_support::review_ray_counts(&body, face, q, d, band, 0, false)
    );
}
