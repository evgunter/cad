//! **Probe: does the conventional chart of an under-determined smooth
//! seam change what a single-face offset does?** Two bodies whose
//! cylinder–plane tangent seam is short enough that the must-carry rule
//! leaves it under-determined: A, the thin shoulder cut at its ruling
//! (the seam is `Chart(Plane)`, via `topo::chord_join`); B, the wall's
//! piece cut again at `z = d` (`split_edge`'s short piece). Each body
//! has its cylinder face, then separately its plane face, offset by a
//! small distance through `topo::replace_face_offset`; the outcome is
//! printed. Run with `--ignored --nocapture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop};
use sweep::test_support::{finished, sketch_at};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{Body, EdgeKey, FaceKey};

fn tol() -> Tol {
    Tol::witness()
}

fn shoulder(h: f64) -> topo::AtRestBody<f64> {
    let bulge = (std::f64::consts::PI / 8.0).tan();
    let outline: ProfileLoop<f64> = bulge_loop(vec![
        (Point2::new(-1.0, 0.0), 0.0),
        (Point2::new(2.0, 0.0), 0.0),
        (Point2::new(2.0, 2.0), 0.0),
        (Point2::new(0.0, 2.0), 0.0),
        (Point2::new(0.0, 1.0), bulge),
    ]);
    let profile = Profile::new(sketch_at(0.0), vec![outline])
        .validate(tol())
        .unwrap();
    let e = extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap();
    finished("the shoulder", e.body, tol())
}

fn shoulder_cut() -> topo::SplitPlane<f64> {
    topo::test_support::split_plane(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0), tol())
}

/// The cylinder–plane edge on the ruling `x = 0, y = 1`, with its
/// cylinder face and its plane face.
fn seam(body: &Body<f64>) -> (EdgeKey, FaceKey, FaceKey) {
    let eps = tol().eps();
    let on = |p: Point3<f64>| p.x.abs() <= eps && (p.y - 1.0).abs() <= eps;
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let mut found = vec![];
    for (key, e) in body.edges() {
        let start = body.get_half_edge(e.he_plus).unwrap().start;
        let end = body.half_edge_end(e.he_plus).unwrap();
        if !(on(point(start)) && on(point(end))) {
            continue;
        }
        let faces = [e.he_plus, e.he_minus].map(|he| body.face_of_half_edge(he).unwrap());
        let kinds = faces.map(|f| {
            body.get_surface(body.get_face(f).unwrap().surface)
                .unwrap()
                .kind()
        });
        let cyl = kinds.iter().position(|k| *k == geom::SurfaceKind::Cylinder);
        let pla = kinds.iter().position(|k| *k == geom::SurfaceKind::Plane);
        if let (Some(c), Some(p)) = (cyl, pla) {
            found.push((key, faces[c], faces[p]));
        }
    }
    let [one] = found[..] else {
        panic!("one seam, got {}", found.len())
    };
    one
}

fn describe(body: &Body<f64>, edge: EdgeKey) -> String {
    let e = body.get_edge(edge).unwrap();
    let kind_of = |s| format!("{:?}", body.get_surface(s).unwrap().kind());
    let face_kind = |he| {
        kind_of(
            body.get_face(body.face_of_half_edge(he).unwrap())
                .unwrap()
                .surface,
        )
    };
    let Some(g) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
        return "uncertified".into();
    };
    let d = match g.description() {
        geom_brep::EdgeDescription::Chart(c) => format!("Chart({})", kind_of(c.surface)),
        geom_brep::EdgeDescription::Intersection { .. } => "Intersection".into(),
        geom_brep::EdgeDescription::TangentIntersection { .. } => "TangentIntersection".into(),
        geom_brep::EdgeDescription::Scaffold(_) => "Scaffold".into(),
    };
    format!(
        "{d} [he+ {}, he- {}], {:?}",
        face_kind(e.he_plus),
        face_kind(e.he_minus),
        g.authority()
    )
}

/// Every edge on the ruling after the move (the seam may not survive
/// under its old key).
fn ruling_edges(body: &Body<f64>) -> Vec<String> {
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    body.edges()
        .filter(|(_, e)| {
            let a = point(body.get_half_edge(e.he_plus).unwrap().start);
            let b = point(body.half_edge_end(e.he_plus).unwrap());
            a.x.abs() < 0.1 && b.x.abs() < 0.1 && (a.y - 1.0).abs() < 0.1 && (b.y - 1.0).abs() < 0.1
        })
        .map(|(k, e)| {
            let a = point(body.get_half_edge(e.he_plus).unwrap().start);
            let b = point(body.half_edge_end(e.he_plus).unwrap());
            format!("{} {a:?}->{b:?}", describe(body, k))
        })
        .collect()
}

fn run(label: &str, body: &Body<f64>) {
    let (edge, cyl, pla) = seam(body);
    println!("== {label}: seam {edge:?} stores {}", describe(body, edge));
    for (moved, face) in [("cylinder", cyl), ("plane", pla)] {
        for d in [0.01_f64, -0.01, 1e-4, -1e-4] {
            // The seam's own planned verdict (the door stops at the
            // first refusing edge, which may be another one).
            for (k, v) in topo::offset_edge_plans_for_tests(body, face, d, tol()) {
                if k == edge {
                    println!("  {moved} d={d:e}: seam {k:?} plans {v:?}");
                }
            }
            let mut b = body.clone();
            let out = topo::replace_face_offset(&mut b, face, d, tol());
            match out {
                Err(e) => println!("  {moved} d={d:e}: REFUSES {e:?}"),
                Ok(o) => {
                    let valid = topo::validate_geometric(&b, tol());
                    println!(
                        "  {moved} d={d:e}: BUILDS joins={:?}; validate={valid:?}",
                        o.joins
                    );
                    match b.get_edge(edge) {
                        Some(_) => println!("    seam now: {}", describe(&b, edge)),
                        None => println!("    seam key gone"),
                    }
                    for s in ruling_edges(&b) {
                        println!("    near ruling: {s}");
                    }
                }
            }
        }
    }
}

#[test]
#[ignore = "probe: run with --ignored --nocapture"]
fn probe_smooth_seam_chart_offset() {
    let unit = (2.0 * tol().eps()).sqrt();
    let h = 0.5 * unit;

    // A: the thin shoulder cut at its ruling; the wall's piece holds the seam.
    let halves = topo::split(&shoulder(h), &shoulder_cut(), tol()).unwrap();
    let a = halves.above.body().unwrap().clone();
    run(&format!("A (shoulder h = {h:e}, chord_join)"), &a);

    // B: the unit-depth wall's piece cut at z = d; the d-tall piece.
    let topo::SplitPart::Body(wall) = topo::split(&shoulder(1.0), &shoulder_cut(), tol())
        .unwrap()
        .above
    else {
        panic!("the wall's piece")
    };
    let wall = finished("the wall's piece", wall, tol());
    let plane =
        topo::test_support::split_plane(Point3::new(0.0, 0.0, h), Vec3::new(0.0, 0.0, 1.0), tol());
    let halves = topo::split(&wall, &plane, tol()).unwrap();
    let b = halves.below.body().unwrap().clone();
    run(&format!("B (wall cut at z = d = {h:e}, split_edge)"), &b);
}
