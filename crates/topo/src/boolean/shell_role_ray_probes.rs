//! FLUSH probe, not for merge: a thin shell's role read off its `V/A`
//! (`props::shell_role_read`) against a role read off rays aimed at its
//! faces (`aimed_side` / `cast_ray`), with the census's reading of the
//! pair each ray could not order. Run with `--ignored --nocapture` at
//! each ε (`CAD_TOLERANCE_EPS`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use super::*;
use crate::entity::ShellKey;

const TS: [f64; 6] = [9.9, 10.0, 12.0, 15.0, 19.0, 20.0];

fn short(s: String, n: usize) -> String {
    if s.len() > n { format!("{}…", &s[..n]) } else { s }
}

/// A box `lo..hi` into `body`, inside out when `void`; returns its solid
/// and its one shell.
fn boxed(body: &mut Body<f64>, lo: [f64; 3], hi: [f64; 3], void: bool, tol: Tol) -> (SolidKey, ShellKey) {
    let before: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    crate::test_support_fixtures::cube_into(
        body,
        move |x, y, z| {
            let x = if void { 1.0 - x } else { x };
            Point3::new(
                lo[0] + x * (hi[0] - lo[0]),
                lo[1] + y * (hi[1] - lo[1]),
                lo[2] + z * (hi[2] - lo[2]),
            )
        },
        tol,
    );
    let solid = body.solids().map(|(k, _)| k).find(|k| !before.contains(k)).unwrap();
    let shell = body.shells_of_solid(solid).unwrap()[0];
    (solid, shell)
}

fn face_plane(body: &Body<f64>, sel: &SolidFaces, f: FaceKey, band: Band) -> (Point3<f64>, Vec3<f64>) {
    match face_geo(body, f, &sel.charts, band).unwrap() {
        FaceGeo::Plane(o, n) => (o, n),
        _ => panic!("planar only"),
    }
}

/// The census's own vertex–face residual reading on the pair `(a, b)`:
/// every vertex of `a` against `b`'s plane and back, the least |residual|.
fn census_pair(body: &Body<f64>, sel: &SolidFaces, a: FaceKey, b: FaceKey, band: Band) -> String {
    let mut least = f64::INFINITY;
    for (x, y) in [(a, b), (b, a)] {
        let (o, n) = face_plane(body, sel, y, band);
        let f = body.get_face(x).unwrap();
        let LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
            panic!("a cycle")
        };
        for h in body.loop_walk(first).closed("loop", first) {
            let v = *body
                .get_point(body.get_vertex(body.get_half_edge(h).unwrap().start).unwrap().point)
                .unwrap();
            least = least.min(((v - o).dot(n)).abs());
        }
    }
    let verdict = match decide("pm_census_vf_residual", Margin::of(least), band) {
        Ok(Sign::Zero) => "Zero".to_string(),
        Ok(_) => "definite apart".to_string(),
        Err(_) => "IN BAND".to_string(),
    };
    format!("{verdict} (|r| {:.3}ε)", least / band.zero())
}

/// The planar crossings of the ray `q + s d` ahead of `q`, inside their
/// faces, sorted: (t, face, d·n).
fn crossings(body: &Body<f64>, sel: &SolidFaces, q: Point3<f64>, d: Vec3<f64>, band: Band) -> Vec<(f64, FaceKey, f64)> {
    let mut out = Vec::new();
    for &f in sel.faces() {
        let (o, n) = face_plane(body, sel, f, band);
        let den = d.dot(n);
        if den == 0.0 {
            continue;
        }
        let t = (o - q).dot(n) / den;
        if t <= 0.0 {
            continue;
        }
        if let Ok(Some(true) | None) = point_in_face(body, f, n, q + d * t, band) {
            out.push((t, f, den));
        }
    }
    out.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    out
}

fn fault(r: &Result<Option<SolidContainment>, RayFault<PointInSolidError>>) -> String {
    match r {
        Ok(Some(s)) => format!("{s:?}"),
        Ok(None) => "None(missed)".into(),
        Err(RayFault::Graze) => "Graze".into(),
        Err(RayFault::Unread) => "Unread".into(),
        Err(RayFault::InBand(PointInSolidError::Escalated { diag, .. })) => {
            format!("IN BAND {:?} {}", diag.predicate, short(format!("{:?}", diag.margin), 60))
        }
        Err(RayFault::InBand(e)) => format!("InBand {}", short(format!("{e:?}"), 80)),
        Err(RayFault::Blocked(e)) => format!("Blocked {}", short(format!("{e:?}"), 80)),
        Err(RayFault::Fatal(e)) => format!("Fatal {}", short(format!("{e:?}"), 80)),
    }
}

/// One ray's reading, with the binding margins: the order gap between
/// the two closest crossings (`bool_point_in_solid_order`) and the
/// least levered denom (`bool_point_in_solid_denom`), both in ε.
fn ray(body: &Body<f64>, sel: &SolidFaces, q: Point3<f64>, d: Vec3<f64>, band: Band) -> String {
    let reach = selection_reach(body, sel.faces(), q).unwrap();
    let read = cast_ray(body, sel, q, d, reach, band);
    let xs = crossings(body, sel, q, d, band);
    let eps = band.zero();
    let denom = xs.iter().map(|x| (x.2 * reach).abs()).fold(f64::INFINITY, f64::min);
    let pair = if xs.len() >= 2 {
        format!(
            "order gap {:.3}ε; denom·reach ≥ {:.3e}ε; census on that pair: {}",
            (xs[1].0 - xs[0].0) / eps,
            denom / eps,
            census_pair(body, sel, xs[0].1, xs[1].1, band)
        )
    } else {
        format!("{} crossing(s)", xs.len())
    };
    format!("{} [{} crossings; {}]", fault(&read), xs.len(), pair)
}

fn report(what: &str, body: &Body<f64>, shell: ShellKey, tol: Tol) {
    let band = Band::linear(tol).unwrap();
    println!("== {what}");
    // V/A
    match crate::props::shell_role_read(body, shell, band, tol, <f64 as crate::props::AtRestPolicy>::quad_lane()) {
        Ok((role, _)) => println!("  V/A: {role:?}"),
        Err(r) => println!(
            "  V/A: REFUSED {}",
            short(format!("{:?}", r.error), 260)
        ),
    }
    let sel = SolidFaces::of_shell(body, shell);
    // (a) the operand gate's far point and aim_points
    let mut corners = Vec::new();
    for &f in sel.faces() {
        let (lo, hi) = crate::census::face_reach(body, f, band).unwrap();
        corners.extend([lo, hi]);
    }
    let (lo, hi) = crate::census::point_hull(corners).unwrap();
    let span = (hi.x - lo.x).max(hi.y - lo.y).max(hi.z - lo.z) + 1.0;
    let far = Point3::new(hi.x + span, hi.y + span, hi.z + span);
    let aims = aim_points(body, &sel, band).unwrap();
    let verdict = match aimed_side(body, &sel, far, &aims, band).unwrap() {
        Aimed::Side(s) => format!("{s:?}"),
        Aimed::Undecided { in_band, limits } => format!(
            "UNDECIDED in_band={} limits={}",
            short(format!("{in_band:?}"), 120),
            short(format!("{limits:?}"), 80)
        ),
    };
    println!("  (a) aimed_side from far {far:?}, {} aims: {verdict}", aims.len());
    for (i, &aim) in aims.iter().enumerate() {
        println!("    aim {i}: {}", ray(body, &sel, far, (aim - far).normalize(), band));
    }
    // (b) along each planar face's normal, from beyond the shell's reach
    for (i, &f) in sel.faces().iter().enumerate() {
        let Some(aim) = crate::stands::planar_interior_point(body, f, band).unwrap() else {
            continue;
        };
        let (_, n) = face_plane(body, &sel, f, band);
        let from = aim + n * (2.0 * selection_reach(body, sel.faces(), aim).unwrap() + 1.0);
        println!("    (b) face {i} n={:?}: {}", (n.x, n.y, n.z), ray(body, &sel, from, -n, band));
    }
}

fn census(what: &str, body: &Body<f64>, tol: Tol) {
    match crate::AtRestBody::validate(body.clone(), tol) {
        Ok(_) => println!("  at-rest gate ({what}): passes"),
        Err(errs) => println!(
            "  at-rest gate ({what}): {} error(s): {}",
            errs.len(),
            short(format!("{:?}", &errs[..errs.len().min(2)]), 300)
        ),
    }
}

#[test]
#[ignore = "FLUSH probe"]
fn probe_shell_role_ray() {
    let tol = Tol::witness();
    let eps = tol.eps();
    println!("ε = {eps:e}, K = {}", tol.k());
    for m in TS {
        let t = m * eps;
        println!("\n######## t = {m}ε");
        let built = std::panic::catch_unwind(|| {
            let mut b = Body::<f64>::new();
            boxed(&mut b, [0.0; 3], [1.0, 1.0, t], false, tol);
        });
        if built.is_err() {
            println!("  the slab does not build at {m}ε (see panic above)");
            continue;
        }
        // 1. brick (−1,2)³ with a slab cavity, one solid two shells
        let mut body = Body::<f64>::new();
        let (keep, _) = boxed(&mut body, [-1.0; 3], [2.0; 3], false, tol);
        let (cav, cshell) = boxed(&mut body, [0.0; 3], [1.0, 1.0, t], true, tol);
        crate::fixtures::refile_shells(&mut body, cav, keep);
        report(&format!("cavity of brick∖slab({m}ε)"), &body, cshell, tol);
        census("brick with slab cavity", &body, tol);
        // the public boolean
        let brick = crate::AtRestBody::<f64>::validate(crate::test_support_fixtures::brick((-1.0, 2.0), (-1.0, 2.0), (-1.0, 2.0), tol), tol).unwrap();
        let slab = crate::AtRestBody::<f64>::validate(crate::test_support_fixtures::brick((0.0, 1.0), (0.0, 1.0), (0.0, t), tol), tol);
        let farb = crate::AtRestBody::<f64>::validate(crate::test_support_fixtures::brick((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), tol), tol).unwrap();
        match &slab {
            Err(e) => println!("  lone slab at rest: REFUSED {}", short(format!("{e:?}"), 200)),
            Ok(s) => {
                println!("  lone slab at rest: passes");
                println!("  brick ∖ slab: {}", short(format!("{:?}", crate::subtract(&brick, s, tol).map(|_| "builds")), 300));
                println!("  slab ∪ far: {}", short(format!("{:?}", crate::union(s, &farb, tol).map(|_| "builds")), 300));
                println!("  far ∪ slab: {}", short(format!("{:?}", crate::union(&farb, s, tol).map(|_| "builds")), 300));
            }
        }
        // 2. slab ∪ brick (5,6)³, both orders: the two shells in one solid
        for (order, slab_first) in [("slab ∪ far", true), ("far ∪ slab", false)] {
            let mut body = Body::<f64>::new();
            let (a, b, sshell);
            if slab_first {
                let s = boxed(&mut body, [0.0; 3], [1.0, 1.0, t], false, tol);
                let f = boxed(&mut body, [5.0; 3], [6.0; 3], false, tol);
                (a, b, sshell) = (s.0, f.0, s.1);
                crate::fixtures::refile_shells(&mut body, b, a);
            } else {
                let f = boxed(&mut body, [5.0; 3], [6.0; 3], false, tol);
                let s = boxed(&mut body, [0.0; 3], [1.0, 1.0, t], false, tol);
                (a, b, sshell) = (f.0, s.0, s.1);
                crate::fixtures::refile_shells(&mut body, b, a);
            }
            report(&format!("slab shell in {order} ({m}ε), one solid"), &body, sshell, tol);
        }
        // 3. a lone slab
        let mut body = Body::<f64>::new();
        let (_, sshell) = boxed(&mut body, [0.0; 3], [1.0, 1.0, t], false, tol);
        report(&format!("lone slab ({m}ε)"), &body, sshell, tol);
        println!(
            "  classify_shells: {}",
            short(format!("{:?}", crate::classify_shells(&body, tol).map(|v| v.iter().map(|c| c.role).collect::<Vec<_>>())), 260)
        );
        census("lone slab", &body, tol);
    }
}
