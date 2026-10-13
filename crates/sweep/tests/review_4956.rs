//! REVIEW 4956 probe (review branch only): sphere lanes against a
//! closed-form oracle, at scale, boundaries, a hole, operand order.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::SurfaceKind;
use geom_core::{Point2, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop};
use sweep::test_support::{ball_poled_y, brick, finished, sketch_at};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{Body, Cell, ContactRecords, ValidationError};

fn tol() -> Tol {
    Tol::witness()
}

fn graft(mut a: Body<f64>, b: &Body<f64>) -> (Body<f64>, Vec<topo::FaceKey>) {
    let keys = topo::graft_disjoint_all_keyed(&mut a, b).unwrap();
    let spheres = b
        .faces()
        .filter(|(_, f)| b.get_surface(f.surface).unwrap().kind() == SurfaceKind::Sphere)
        .map(|(k, _)| keys.face(k).unwrap())
        .collect();
    (a, spheres)
}

fn top(body: &Body<f64>, z: f64) -> topo::FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.z.abs() > 0.5 && (origin.z - z).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .unwrap()
}

fn read(body: &Body<f64>, x: Cell, y: Cell) -> String {
    let mut records = ContactRecords::default();
    records.extend([topo::Cited::new(
        topo::Contact::new(body, x, y).unwrap(),
        topo::Cites::decided(0),
    )]);
    let errors = topo::validate_pseudomanifold(body, &records, tol())
        .err()
        .unwrap_or_default();
    let mine: Vec<_> = errors.iter().filter(|e| !matches!(e, ValidationError::CensusUndecidable { .. })).collect();
    if mine.iter().any(|e| matches!(e, ValidationError::StaleContactDeclaration { .. })) {
        "Stale".into()
    } else if mine.iter().any(|e| matches!(e, ValidationError::CensusUnsupported { .. })) {
        "Unsupported".into()
    } else if mine.iter().any(|e| matches!(e, ValidationError::CensusEscalated { .. })) {
        "Escalated".into()
    } else if mine.is_empty() {
        "Confirmed".into()
    } else {
        format!("Other({mine:?})")
    }
}

fn reads(body: &Body<f64>, balls: &[topo::FaceKey], cell: Cell, swap: bool) -> Vec<String> {
    let mut out: Vec<String> = balls
        .iter()
        .map(|&f| if swap { read(body, cell, Cell::Face(f)) } else { read(body, Cell::Face(f), cell) })
        .collect();
    out.sort();
    out
}

fn circle(cx: f64, cy: f64, r: f64, phase: f64) -> ProfileLoop<f64> {
    let at = |t: f64| Point2::new(cx + r * t.cos(), cy + r * t.sin());
    bulge_loop(vec![(at(phase), 1.0), (at(phase + core::f64::consts::PI), 1.0)])
}

fn ext(what: &str, loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> topo::AtRestBody<f64> {
    let profile = Profile::new(sketch_at(z0), loops).validate(tol()).unwrap();
    let body: Body<f64> = extrude(&profile, Extrusion::Distance { depth: h, side: ExtrudeSide::Along }, tol())
        .unwrap()
        .body;
    finished(what, body, tol())
}

fn report(what: &str, got: Vec<String>, want: &[&str]) -> bool {
    let ok = got.iter().map(String::as_str).collect::<Vec<_>>() == want;
    eprintln!("R4956 {} {what}: got {got:?} want {want:?}", if ok { "ok " } else { "BAD" });
    ok
}

#[test]
#[ignore = "review probe"]
fn review_4956_sphere_lanes() {
    let band = geom_core::Band::linear(tol()).unwrap();
    let (eps, k) = (band.zero(), band.escalate() / band.zero());
    eprintln!("R4956 eps={eps} k={k}");
    let esc_or = |inside: &'static str| if k > 3.0 { "Escalated" } else { inside };
    let mut bad = 0;
    let slab = || brick::<f64>((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), tol());
    // A: sphere on plane.
    for &swap in &[false, true] {
        for (what, r, c, want) in [
            ("ball r=0.005 touching", 0.005, Vec3::new(2.0, 2.0, 1.005), vec!["Confirmed", "Stale"]),
            ("ball r=50 touching", 50.0, Vec3::new(2.0, 2.0, 51.0), vec!["Confirmed", "Stale"]),
            ("ball r=0.5 off 0.5 eps", 0.5, Vec3::new(2.0, 2.0, 1.5 + 0.5 * eps), vec!["Confirmed", "Stale"]),
            ("ball r=0.5 sunk 3 eps", 0.5, Vec3::new(2.0, 2.0, 1.5 - 3.0 * eps), vec![esc_or("Stale"), esc_or("Stale")]),
            ("ball r=0.5 up 2k eps", 0.5, Vec3::new(2.0, 2.0, 1.5 + 2.0 * k * eps), vec!["Stale", "Stale"]),
            ("ball r=0.5 sunk 2k eps", 0.5, Vec3::new(2.0, 2.0, 1.5 - 2.0 * k * eps), vec!["Stale", "Stale"]),
            ("touch on face edge x=0", 0.5, Vec3::new(0.0, 2.0, 1.5), vec!["Stale", "Stale"]),
            ("touch at face corner", 0.5, Vec3::new(0.0, 0.0, 1.5), vec!["Stale", "Stale"]),
            ("touch 3 eps inside face edge", 0.5, Vec3::new(3.0 * eps, 2.0, 1.5), vec![esc_or("Confirmed"), "Stale"]),
            ("touch 2k eps inside face edge", 0.5, Vec3::new(2.0 * k * eps, 2.0, 1.5), vec!["Confirmed", "Stale"]),
            ("touch 2k eps outside face edge", 0.5, Vec3::new(-2.0 * k * eps, 2.0, 1.5), vec!["Stale", "Stale"]),
            ("ball inside slab touching top from below", 0.4, Vec3::new(2.0, 2.0, 0.6), vec!["Confirmed", "Stale"]),
        ] {
            let ball = ball_poled_y(r, c, tol());
            let (body, balls) = graft(slab(), &ball);
            let face = top(&body, 1.0);
            if !report(&format!("{what} swap={swap}"), reads(&body, &balls, Cell::Face(face), swap), &want) {
                bad += 1;
            }
        }
    }
    // A': the touch point over a hole in the slab's face.
    {
        let drill = ext("drill", vec![circle(2.0, 2.0, 0.2, 0.0)], -1.0, 3.0);
        let holed = match topo::subtract(&finished("slab", slab(), tol()), &drill, tol()) {
            Ok(topo::boolean::BooleanResult::Body(b)) => b.body.into_body(),
            other => panic!("drill: {:?}", other.map(|_| ())),
        };
        let ball = ball_poled_y(0.5, Vec3::new(2.0, 2.0, 1.5), tol());
        let (body, balls) = graft(holed.clone(), &ball);
        let face = top(&body, 1.0);
        if !report("touch over a hole", reads(&body, &balls, Cell::Face(face), false), &["Stale", "Stale"]) {
            bad += 1;
        }
        let ball = ball_poled_y(0.5, Vec3::new(2.5, 2.0, 1.5), tol());
        let (body, balls) = graft(holed, &ball);
        let face = top(&body, 1.0);
        if !report("touch beside a hole", reads(&body, &balls, Cell::Face(face), false), &["Confirmed", "Stale"]) {
            bad += 1;
        }
    }
    // B: two spheres.
    for &swap in &[false, true] {
        for (what, (big, cb), (r, c), want) in [
            ("outside 1e-3 apart", (0.5, Vec3::new(2.0, 2.0, 1.5)), (0.5, Vec3::new(2.0, 2.0, 2.501)), vec!["Stale"; 4]),
            ("outside 1e-3 overlap", (0.5, Vec3::new(2.0, 2.0, 1.5)), (0.5, Vec3::new(2.0, 2.0, 2.499)), vec!["Stale"; 4]),
            ("outside 3 eps apart", (0.5, Vec3::new(2.0, 2.0, 1.5)), (0.5, Vec3::new(2.0, 2.0, 2.5 + 3.0 * eps)), vec![esc_or("Stale"); 4]),
            ("outside unequal 0.01 and 7", (7.0, Vec3::new(2.0, 2.0, -5.0)), (0.01, Vec3::new(2.0, 2.0, 2.01)), vec!["Confirmed", "Stale", "Stale", "Stale"]),
            ("inside near-equal (1e-3)", (1.0, Vec3::new(2.0, 2.0, 1.5)), (0.999, Vec3::new(2.0, 2.0, 1.501)), vec!["Confirmed", "Stale", "Stale", "Stale"]),
            ("inside, small inside not touching", (1.0, Vec3::new(2.0, 2.0, 1.5)), (0.25, Vec3::new(2.0, 2.0, 2.0)), vec!["Stale"; 4]),
            ("inside, small 3 eps off touching", (1.0, Vec3::new(2.0, 2.0, 1.5)), (0.25, Vec3::new(2.0, 2.0, 2.25 - 3.0 * eps)), vec![esc_or("Stale"); 4]),
            ("inside radii 3eps apart, centres 3eps", (1.0, Vec3::new(2.0, 2.0, 1.5)), (1.0 - 3.0 * eps, Vec3::new(2.0, 2.0, 1.5 + 3.0 * eps)), vec!["?"]),
            ("inside radii 2k eps apart, centres 2k eps", (1.0, Vec3::new(2.0, 2.0, 1.5)), (1.0 - 2.0 * k * eps, Vec3::new(2.0, 2.0, 1.5 + 2.0 * k * eps)), vec!["?"]),
            ("same sphere 0.5 eps off", (1.0, Vec3::new(2.0, 2.0, 1.5)), (1.0, Vec3::new(2.0, 2.0, 1.5 + 0.5 * eps)), vec!["?"]),
            ("concentric 1e-3 radii apart", (1.0, Vec3::new(2.0, 2.0, 1.5)), (0.999, Vec3::new(2.0, 2.0, 1.5)), vec!["Stale"; 4]),
        ] {
            let a = ball_poled_y(big, cb, tol());
            let b = ball_poled_y(r, c, tol());
            let (body, fa) = graft(Body::new(), &a);
            let (body, fb) = graft(body, &b);
            let fa: Vec<_> = fa.into_iter().filter(|f| !fb.contains(f)).collect();
            let mut got: Vec<String> = fa
                .iter()
                .flat_map(|&x| fb.iter().map(move |&y| (x, y)))
                .map(|(x, y)| if swap { read(&body, Cell::Face(y), Cell::Face(x)) } else { read(&body, Cell::Face(x), Cell::Face(y)) })
                .collect();
            got.sort();
            if want == ["?"] {
                eprintln!("R4956 obs {what} swap={swap}: {got:?}");
            } else if !report(&format!("{what} swap={swap}"), got, &want) {
                bad += 1;
            }
        }
    }
    // C: line on sphere — the slab's top edge y = 0, z = 1, x over 0..4.
    let h = 0.5 / 2.0_f64.sqrt();
    for &swap in &[false, true] {
        for (what, x, dz, want) in [
            ("on edge interior", 2.0, 0.0, vec!["Confirmed", "Stale"]),
            ("at the edge's end vertex", 0.0, 0.0, vec!["Stale", "Stale"]),
            ("past the edge's end", -0.3, 0.0, vec!["Stale", "Stale"]),
            ("3 eps inside the end", 3.0 * eps, 0.0, vec![esc_or("Confirmed"), "Stale"]),
            ("2k eps inside the end", 2.0 * k * eps, 0.0, vec!["Confirmed", "Stale"]),
            ("lifted 3 eps", 2.0, 3.0 * eps, vec![esc_or("Stale"), esc_or("Stale")]),
            ("lifted 1e-3", 2.0, 1e-3, vec!["Stale", "Stale"]),
            ("sunk 1e-3 (line crosses)", 2.0, -1e-3, vec!["Stale", "Stale"]),
        ] {
            let ball = ball_poled_y(0.5, Vec3::new(x, -h, 1.0 + h + dz), tol());
            let (body, balls) = graft(slab(), &ball);
            let at = |v: topo::VertexKey| {
                let (_, p) = body.vertex_points().find(|(k, _)| *k == v).unwrap();
                [p.x, p.y, p.z]
            };
            let edge = body
                .edges()
                .find(|(_, e)| {
                    let ends = [e.he_plus, e.he_minus].map(|h| at(body.get_half_edge(h).unwrap().start));
                    let (p, q) = ([0.0, 0.0, 1.0], [4.0, 0.0, 1.0]);
                    ends == [p, q] || ends == [q, p]
                })
                .map(|(k, _)| k)
                .unwrap();
            if !report(&format!("line {what} swap={swap}"), reads(&body, &balls, Cell::Edge(edge), swap), &want) {
                bad += 1;
            }
        }
    }
    eprintln!("R4956 bad = {bad}");
}
