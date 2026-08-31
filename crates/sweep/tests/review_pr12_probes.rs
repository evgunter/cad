//! Blinded-review probes for M5 PR 12 (NOT for merge — reviewer scratch).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use core::f64::consts::PI;
use geom_core::Tol;
use geom_core::{Affine3, Band, Point2, Point3, Vec2, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::blend::battery::{BlendRequest, run_battery};
use sweep::blend::build::fillet_edges;
use sweep::test_support::cube;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}
fn band() -> Band {
    Band::new(tol().eps(), tol().k() * tol().eps()).unwrap()
}
fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}
fn prism(pts: &[(f64, f64)], h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .unwrap()
        .body
}
fn all_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    body.edges().map(|(k, _)| k).collect()
}
fn ball_at(r: f64, c: Vec3<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -r), 1.0),
        ProfileVertex::new(p2(0.0, r), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let ball = revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body;
    topo::transform_rigid(&ball, &Affine3::translation(c), Tol::witness()).unwrap()
}

/// PROBE A: pips first, then request ALL edges (box + rims) — where
/// exactly does the composition stop? Print the battery verdict and
/// the assembly refusal.
#[test]
fn probe_a_pipped_cube_all_edges() {
    let c = cube(1.0, Tol::witness());
    let ball = ball_at(0.09, Vec3::new(0.5, 0.5, 1.04));
    let pipped = boolean_op_with(
        BooleanOp::Subtract,
        &c,
        &ball,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .unwrap()
    .body()
    .unwrap()
    .body
    .clone();
    let edges = all_edges(&pipped);
    println!("PROBE A: pipped cube has {} edges", edges.len());
    let req = BlendRequest {
        body: &pipped,
        edges: edges.clone(),
        size: 0.12,
    };
    match run_battery(&req, band()) {
        Ok(v) => {
            println!(
                "PROBE A: battery PASSES on pipped cube, {} chains ({} closed)",
                v.chains.len(),
                v.chains
                    .iter()
                    .filter(|ch| matches!(ch.closure, sweep::blend::battery::ChainClosure::Closed))
                    .count()
            );
        }
        Err(e) => println!("PROBE A: battery refuses: {e}"),
    }
    match fillet_edges(&pipped, &edges, 0.12, band(), Tol::witness()) {
        Ok(_) => println!("PROBE A: fillet_edges SUCCEEDED (composition works!)"),
        Err(e) => println!("PROBE A: fillet_edges refuses: {e}"),
    }
}

/// PROBE B: hexagonal prism, all edges. Inradius (apothem) of the
/// hexagon = a*sqrt(3)/2 for side a. True consumption limit is the
/// apothem; the cap pair-sweep gap between second-neighbour cap edges
/// is only `a`, so the battery should over-refuse near r = a/2.
#[test]
fn probe_b_hexagonal_prism_over_refusal() {
    let a = 1.0f64; // side
    let pts: Vec<(f64, f64)> = (0..6)
        .map(|i| {
            let th = PI / 3.0 * f64::from(i);
            (th.cos(), th.sin())
        })
        .collect();
    // circumradius 1 => side a = 1, apothem = sqrt(3)/2 = 0.866
    let h = 4.0; // tall so cap-cap pairs never bind
    let body = prism(&pts, h);
    let edges = all_edges(&body);
    assert_eq!(edges.len(), 18);
    for r in [0.30 * a, 0.45 * a, 0.499 * a, 0.51 * a, 0.6 * a, 0.8 * a] {
        match fillet_edges(&body, &edges, r, band(), Tol::witness()) {
            Ok(f) => {
                let t3 = topo::validate_geometric(&f.body, Tol::witness()).is_ok();
                println!("PROBE B: r={r:.3} builds, tier3 ok = {t3}");
            }
            Err(e) => println!("PROBE B: r={r:.3} refuses: {e}"),
        }
    }
}

/// PROBE D: per-arc diagnosis of the pip-rim convexity flip — does
/// each rim arc's he_plus run WITH its carrier?
#[test]
fn probe_d_rim_arc_orientation() {
    let c = cube(1.0, Tol::witness());
    let ball = ball_at(0.09, Vec3::new(0.5, 0.5, 1.04));
    let pipped = boolean_op_with(
        BooleanOp::Subtract,
        &c,
        &ball,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .unwrap()
    .body()
    .unwrap()
    .body
    .clone();
    for (k, e) in pipped.edges() {
        let curve = pipped.get_curve_geom(e.curve).unwrap().certified().unwrap();
        let carrier = curve.carrier();
        let (t0, t1) = curve.params();
        let start_v = pipped.get_half_edge(e.he_plus).unwrap().start;
        let sp = *pipped
            .get_point(pipped.get_vertex(start_v).unwrap().point)
            .unwrap();
        let c0 = carrier.eval(t0);
        let c1e = carrier.eval(t1);
        let aligned = (c0 - sp).norm() <= (c1e - sp).norm();
        let is_circle = matches!(carrier, geom::Curve3::Circle { .. });
        if is_circle {
            let req = BlendRequest {
                body: &pipped,
                edges: vec![k],
                size: 0.02,
            };
            let verdict = run_battery(&req, band());
            let conv = verdict.as_ref().ok().map(|v| v.chains[0].first().convexity);
            println!(
                "PROBE D: rim arc {k:?}: he_plus aligned with carrier = {aligned}, single-edge battery => {:?} (err: {})",
                conv,
                verdict
                    .as_ref()
                    .err()
                    .map(|e| e.to_string())
                    .unwrap_or_default()
            );
        }
    }
}

/// PROBE E: hexagon tier-3 error detail at a modest radius.
#[test]
fn probe_e_hexagon_tier3_error() {
    let pts: Vec<(f64, f64)> = (0..6)
        .map(|i| {
            let th = PI / 3.0 * f64::from(i);
            (th.cos(), th.sin())
        })
        .collect();
    let body = prism(&pts, 4.0);
    let edges = all_edges(&body);
    let f = fillet_edges(&body, &edges, 0.3, band(), Tol::witness()).expect("builds");
    println!(
        "PROBE E: tier3 = {:?}",
        topo::validate_geometric(&f.body, Tol::witness())
    );
    println!(
        "PROBE E: props = {:?}",
        topo::mass_properties(&f.body, Tol::witness()).map(|p| (p.volume, p.volume_pad))
    );
    if let Err(errs) = topo::validate_geometric(&f.body, Tol::witness()) {
        for e in &errs {
            let txt = format!("{e:?}");
            if let Some(i) = txt.find("FaceKey(") {
                let key: String = txt[i..].chars().take_while(|c| *c != ' ').collect();
                for (k, fc) in f.body.faces() {
                    if format!("{k:?}").starts_with(key.trim_end_matches(',')) {
                        println!(
                            "PROBE E: failing face {k:?} kind {:?} corner? {} blend? {}",
                            f.body.get_surface(fc.surface).map(|s| match s {
                                geom::Surface::Plane { .. } => "plane",
                                geom::Surface::Cylinder { .. } => "cylinder",
                                geom::Surface::Sphere { .. } => "sphere",
                                _ => "other",
                            }),
                            f.corner_faces.contains(&k),
                            f.blend_faces.contains(&k)
                        );
                    }
                }
            }
        }
    }
}

/// PROBE G: door A with a SINGLE centre pip on the blank — fillet
/// FIRST, then cut. This probe was written to record which door the
/// composition stopped at; it now records that it does not stop. The
/// blocker was the trimmed sphere face the cut leaves behind, and the
/// sphere chart's `[azimuth] × [latitude]` window serves that class,
/// so the containment stage answers and the operation completes.
///
/// **What the volume check is, precisely.** The CAP subtraction is
/// closed form; the filleted blank's own volume is not, and the `want`
/// below embeds it as MEASURED. So this row pins "the cut removes
/// exactly one spherical cap from whatever the blank was", not "the
/// result equals a formula end to end". The fully-closed-form pin is
/// `m5_pr12_die::deviation_1`.
#[test]
fn probe_g_door_a_fields() {
    let c = cube(1.0, Tol::witness());
    let edges = all_edges(&c);
    let blank = fillet_edges(&c, &edges, 0.12, band(), Tol::witness())
        .unwrap()
        .body;
    let blank_v = topo::mass_properties(&blank, Tol::witness())
        .unwrap()
        .volume;
    let ball = ball_at(0.09, Vec3::new(0.5, 0.5, 1.04));
    let out = boolean_op_with(
        BooleanOp::Subtract,
        &blank,
        &ball,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .expect("door A composes");
    let body = &out.body().expect("a body").body;
    assert_eq!(topo::validate_geometric(body, Tol::witness()), Ok(()));
    assert_eq!(body.shells().count(), 1, "one shell: the pipped blank");
    // The pip is a spherical cap of height r - 0.04 off a radius-0.09
    // ball, cut from a face the fillet left flat.
    let h = 0.09 - 0.04;
    let want = blank_v - PI * h * h * (3.0 * 0.09 - h) / 3.0;
    let got = topo::mass_properties(body, Tol::witness()).unwrap().volume;
    assert!((got - want).abs() <= 1e-9 * want, "{got} vs {want}");
}

/// PROBE H: door A with a multi-ball CLOSED-group tool. The same
/// answer as PROBE G, at two pips: the trimmed sphere faces the cut
/// leaves are each a chart rectangle, so nothing stops.
#[test]
fn probe_h_door_a_closed_tool() {
    let c = cube(1.0, Tol::witness());
    let edges = all_edges(&c);
    let blank = fillet_edges(&c, &edges, 0.12, band(), Tol::witness())
        .unwrap()
        .body;
    // Two pips on the top face (the diag pair of face value 2 layout,
    // scaled): a multi-ball closed-group tool, unioned first.
    let b1 = ball_at(0.09, Vec3::new(0.28, 0.28, 1.04));
    let b2 = ball_at(0.09, Vec3::new(0.72, 0.72, 1.04));
    let tool = boolean_op_with(
        BooleanOp::Union,
        &b1,
        &b2,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .unwrap()
    .body()
    .unwrap()
    .body
    .clone();
    let out = boolean_op_with(
        BooleanOp::Subtract,
        &blank,
        &tool,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .expect("door A composes for a multi-ball tool too");
    let body = &out.body().expect("a body").body;
    assert_eq!(topo::validate_geometric(body, Tol::witness()), Ok(()));
    assert_eq!(body.shells().count(), 1);
    let blank_v = topo::mass_properties(&blank, Tol::witness())
        .unwrap()
        .volume;
    let h = 0.09 - 0.04;
    let want = blank_v - 2.0 * PI * h * h * (3.0 * 0.09 - h) / 3.0;
    let got = topo::mass_properties(body, Tol::witness()).unwrap().volume;
    assert!((got - want).abs() <= 1e-9 * want, "{got} vs {want}");
}

/// PROBE I: the full 21-pip tool against the blank — the exact door-A
/// refusal variant, faithfully replicated from the die test.
#[test]
fn probe_i_door_a_full_tool() {
    let c = cube(1.0, Tol::witness());
    let edges = all_edges(&c);
    let blank = fillet_edges(&c, &edges, 0.12, band(), Tol::witness())
        .unwrap()
        .body;
    let (pip_r, pip_h, pip_d, h) = (0.09, 0.05, 0.22, 0.5);
    let layout = |n: u32| -> Vec<(f64, f64)> {
        let c = vec![(0.0, 0.0)];
        let diag = vec![(-1.0, -1.0), (1.0, 1.0)];
        let anti = vec![(-1.0, 1.0), (1.0, -1.0)];
        let sides = vec![(-1.0, 0.0), (1.0, 0.0)];
        match n {
            1 => c,
            2 => diag,
            3 => [diag.clone(), c].concat(),
            4 => [diag.clone(), anti.clone()].concat(),
            5 => [diag.clone(), anti.clone(), c].concat(),
            _ => [diag, anti, sides].concat(),
        }
    };
    type Face = (u32, Vec3<f64>, Vec3<f64>, Vec3<f64>);
    let faces: [Face; 6] = [
        (
            1,
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            6,
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            2,
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        (
            5,
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        (
            3,
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        (
            4,
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
    ];
    let mut places: Vec<Vec3<f64>> = Vec::new();
    for (n, normal, ex, ey) in faces {
        let base = Vec3::new(h, h, h) + normal * (h + (pip_r - pip_h));
        for (u, v) in layout(n) {
            places.push(base + ex * (u * pip_d) + ey * (v * pip_d));
        }
    }
    let mut tool = ball_at(pip_r, places[0]);
    for c in &places[1..] {
        tool = boolean_op_with(
            BooleanOp::Union,
            &tool,
            &ball_at(pip_r, *c),
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            Tol::witness(),
        )
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    }
    let err = boolean_op_with(
        BooleanOp::Subtract,
        &blank,
        &tool,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .map(|_| ())
    .expect_err("door A full");
    println!("PROBE I: {err:?}");
    for (body, tag) in [(&blank, "blank"), (&tool, "tool")] {
        for (k, f) in body.faces() {
            if format!("{k:?}").starts_with("FaceKey(9v") {
                println!(
                    "PROBE I: {tag} face {k:?} surface {:?}",
                    body.get_surface(f.surface)
                );
            }
        }
        for (k, e) in body.edges() {
            if format!("{k:?}").starts_with("EdgeKey(15v") {
                let cg = body.get_curve_geom(e.curve).and_then(|c| c.certified());
                println!(
                    "PROBE I: {tag} edge {k:?} carrier {:?} params {:?}",
                    cg.map(geom_brep::EdgeCurve::carrier),
                    cg.map(geom_brep::EdgeCurve::params)
                );
            }
        }
    }
}

/// PROBE F: skinny triangular prism — is the consumption sweep's
/// refusal boundary exactly the cap triangle's inradius (0.0734),
/// as the wall vertical-pair identity predicts?
#[test]
fn probe_f_skinny_triangle_refusal_boundary() {
    let body = prism(&[(0.0, 0.0), (1.0, 0.0), (0.5, 0.15)], 2.0);
    let edges = all_edges(&body);
    for r in [0.05, 0.06, 0.07, 0.072, 0.0735, 0.075, 0.08] {
        match fillet_edges(&body, &edges, r, band(), Tol::witness()) {
            Ok(f) => {
                let t2 = topo::validate_closed(&f.body).is_ok();
                let mesh_ok = mesh::tessellate(&f.body, 5e-3, Tol::witness())
                    .ok()
                    .map(|m| mesh::validate::check_mesh(&m).is_ok());
                println!("PROBE F: r={r} builds, tier2={t2}, mesh watertight={mesh_ok:?}");
            }
            Err(e) => println!("PROBE F: r={r} refuses: {e}"),
        }
    }
}

/// PROBE C: an oblique trihedron from cube ∩ rotated cube — does the
/// battery pass, does construction succeed, and what do the tiers say?
#[test]
fn probe_c_oblique_trihedron() {
    let c1 = cube(1.0, Tol::witness());
    // A slab rotated so one face slices the (1,1,1) corner obliquely.
    let c2 = cube(3.0, Tol::witness());
    let rot = Affine3::rotation_about_axis(
        Point3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, -1.0, 0.0).normalize(),
        0.4,
    );
    let c2 = topo::transform_rigid(
        &c2,
        &Affine3::translation(Vec3::new(-1.55, -1.55, -2.45)),
        Tol::witness(),
    )
    .unwrap();
    let c2 = topo::transform_rigid(&c2, &rot, Tol::witness()).unwrap();
    let out = boolean_op_with(
        BooleanOp::Intersect,
        &c1,
        &c2,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    );
    let body = match out {
        Ok(o) => o.body().unwrap().body.clone(),
        Err(e) => {
            println!("PROBE C: intersection refused: {e}");
            return;
        }
    };
    println!(
        "PROBE C: clipped body F/E/V = {}/{}/{}",
        body.faces().count(),
        body.edges().count(),
        body.vertices().count()
    );
    let edges = all_edges(&body);
    match fillet_edges(&body, &edges, 0.08, band(), Tol::witness()) {
        Ok(f) => {
            println!(
                "PROBE C: builds; tier1 {:?} tier2 {:?} tier3 {:?}",
                topo::validate(&f.body).is_ok(),
                topo::validate_closed(&f.body).is_ok(),
                topo::validate_geometric(&f.body, Tol::witness())
            );
        }
        Err(e) => println!("PROBE C: fillet refuses: {e}"),
    }
}
