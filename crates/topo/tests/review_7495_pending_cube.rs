//! Review probe (PR 7495): every public door on an all-pending mvfs cube.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]
use geom_core::{Band, Point3, Tol, Vec3};
use topo::construction::Construct;
use topo::construction::{MefCreated, MefSite, MevCreated, MevSite, MvfsCreated};
use topo::{Body, validate};
use std::panic::{AssertUnwindSafe, catch_unwind};

/// Every operator result of one cube construction, in call order.
#[derive(PartialEq, Eq, Debug)]
struct Cube {
    seed: MvfsCreated,
    mevs: [MevCreated; 7],
    mefs: [MefCreated; 5],
}

/// Builds the unit cube, asserting tier-1 validity after EVERY operator
/// (explicitly — not relying on the operators' debug postconditions).
fn build_cube(body: &mut Body<f64>) -> Cube {
    // (a) Seed: lone vertex A in the face that will become the top.
    let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
    assert_eq!(validate(body), Ok(()));

    // (b) The bottom chain A→B→C→D: one segment, then two struts, each
    // spliced before the previous edge's minus half (the half starting
    // at the chain's current tip).
    let e_ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::new(1.0, 0.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    assert_eq!(validate(body), Ok(()));
    let e_bc = body
        .mev_line(
            MevSite::Fan {
                he1: e_ab.he_minus,
                he2: e_ab.he_minus,
            },
            Point3::new(1.0, 1.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    assert_eq!(validate(body), Ok(()));
    let e_cd = body
        .mev_line(
            MevSite::Fan {
                he1: e_bc.he_minus,
                he2: e_bc.he_minus,
            },
            Point3::new(0.0, 1.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    assert_eq!(validate(body), Ok(()));

    // (c) Close the square: the lamina step. Address the chords the
    // high-level GWB way, through find_half_edge. he1 = the half D→C
    // (start of the run that becomes the bottom face), he2 = the half
    // A→B (exclusive end); the new edge runs D→A, and the new (bottom)
    // face's outer loop walks A→D→C→B — CCW seen from below, per the
    // convention table above.
    let he_dc = body
        .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
        .unwrap();
    let he_ab = body
        .find_half_edge(seed.face, seed.vertex, e_ab.vertex)
        .unwrap();
    assert_eq!(
        he_ab, e_ab.he_plus,
        "find_half_edge agrees with created keys"
    );
    let f_bottom = body
        .mef_chord(
            MefSite::Chords {
                he1: he_dc,
                he2: he_ab,
            },
            Tol::witness(),
        )
        .unwrap();
    assert_eq!(validate(body), Ok(()));

    // (d) Raise the verticals: four struts in the remaining (top-side)
    // loop, each at a bottom corner, spliced before that corner's
    // outgoing bottom-square half.
    let strut = |body: &mut Body<f64>, at: topo::HalfEdgeKey, x: f64, y: f64| {
        let created = body
            .mev_line(
                MevSite::Fan { he1: at, he2: at },
                Point3::new(x, y, 1.0),
                Tol::witness(),
            )
            .unwrap();
        assert_eq!(validate(body), Ok(()));
        created
    };
    let e_aa = strut(body, e_ab.he_plus, 0.0, 0.0); // A→A' before A→B
    let e_bb = strut(body, e_bc.he_plus, 1.0, 0.0); // B→B' before B→C
    let e_cc = strut(body, e_cd.he_plus, 1.0, 1.0); // C→C' before C→D
    let e_dd = strut(body, f_bottom.he_plus, 0.0, 1.0); // D→D' before D→A

    // (e) Cut the four side faces out of the top-side loop. Each mef's
    // he1 is a strut's minus half (T'→T, the downward traversal), and
    // the run [he1 .. he2) — down the strut, along one bottom edge, up
    // the next strut — becomes the side face's outer loop.
    let side = |body: &mut Body<f64>, he1, he2| {
        let created = body
            .mef_chord(MefSite::Chords { he1, he2 }, Tol::witness())
            .unwrap();
        assert_eq!(validate(body), Ok(()));
        created
    };
    // Front A→B→B'→A': run [A'→A, A→B, B→B'), new edge A'→B'.
    let f_front = side(body, e_aa.he_minus, e_bb.he_minus);
    // Right B→C→C'→B': run [B'→B, B→C, C→C'), new edge B'→C'.
    let f_right = side(body, e_bb.he_minus, e_cc.he_minus);
    // Back C→D→D'→C': run [C'→C, C→D, D→D'), new edge C'→D'.
    let f_back = side(body, e_cc.he_minus, e_dd.he_minus);
    // Left D→A→A'→D': run [D'→D, D→A, A→A'), new edge D'→A'. Its he2 is
    // the front face's plus half A'→B' — found through the face, the
    // way a hand construction would address it.
    let he_a1b1 = body
        .find_half_edge(seed.face, e_aa.vertex, e_bb.vertex)
        .unwrap();
    assert_eq!(
        he_a1b1, f_front.he_plus,
        "find_half_edge agrees with created keys"
    );
    let f_left = side(body, e_dd.he_minus, he_a1b1);

    Cube {
        seed,
        mevs: [e_ab, e_bc, e_cd, e_aa, e_bb, e_cc, e_dd],
        mefs: [f_bottom, f_front, f_right, f_back, f_left],
    }
}

fn run<R: std::fmt::Debug>(name: &str, f: impl FnOnce() -> R) {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(r) => { let s = format!("{r:?}"); println!("PROBE {name}: {}", &s[..s.len().min(300)]); }
        Err(e) => {
            let m = e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default();
            println!("PROBE {name}: PANIC {}", &m[..m.len().min(300)]);
        }
    }
}

#[test]
fn pending_cube_doors() {
    let mut body = Body::<f64>::new();
    let cube = build_cube(&mut body);
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let faces = topo::query::all_faces(&body);
    let edges = topo::query::all_edges(&body);
    let f0 = faces[0];
    run("validate_closed", || topo::validate_closed(&body));
    run("validate_geometric", || topo::validate::validate_geometric(&body, tol).map_err(|v| v.into_iter().map(|e| format!("{e:?}")).take(3).collect::<Vec<_>>()));
    run("validate_pseudomanifold", || topo::validate::validate_pseudomanifold(&body, &topo::boolean::ContactRecords::default(), tol).map(|_| ()).map_err(|v| format!("{v:?}").chars().take(200).collect::<String>()));
    run("point_in_solid", || topo::solid_contain::point_in_solid(&body, Point3::new(0.5,0.5,0.5), band, tol).map(|_|()));
    run("transform_rigid", || topo::transform::transform_rigid(&body, &geom_core::RigidMotion::translation(Vec3::new(1.0,0.0,0.0)).unwrap()).map(|_|()));
    run("mirror_body", || topo::mirror_body(&body, &topo::Reflection::across(Point3::new(0.0,0.0,0.0), Vec3::new(1.0,0.0,0.0), tol).unwrap(), tol).map(|_|()));
    run("face_outward_normal", || faces.iter().map(|&f| topo::face_normal::face_outward_normal(&body, f).is_some()).collect::<Vec<_>>());
    run("face_plane", || faces.iter().map(|&f| topo::face_normal::face_plane(&body, f).is_some()).collect::<Vec<_>>());
    run("face_carrier_desc", || faces.iter().map(|&f| topo::face_normal::face_carrier_desc(&body, f).is_some()).collect::<Vec<_>>());
    run("face_surface_kind", || faces.iter().map(|&f| topo::query::face_surface_kind(&body, f)).collect::<Vec<_>>());
    run("edge_wedge", || edges.iter().map(|&e| topo::edge_wedge(&body, e, band).is_some()).collect::<Vec<_>>());
    run("face_nappe", || topo::face_nappe(&body, f0, band).map(|_|()));
    run("group_nappe", || topo::group_nappe(&body, &faces, band).map(|_|()));
    run("classify_unvalidated", || topo::test_support::classify_unvalidated(&body, tol).map(|_|()));
    run("merge_coplanar", || { let mut b = body.clone(); b.merge_coplanar_faces(tol).map(|_|()) });
    run("offset_planes_together", || { let mut b = body.clone(); topo::offset_planes_together(&mut b, &[topo::ChartMove { faces: vec![f0], distance: 0.1 }], band, tol).map(|_|()) });
    run("finish", || topo::finish(body.clone(), tol).map(|_|()).map_err(|e| format!("{e:?}").chars().take(200).collect::<String>()));
    run("chart_region_overlap", || topo::chart_region_overlap(&body, faces[0], &body, faces[1], band).map(|_|()));
    run("chart_region_overlap_self", || topo::chart_region_overlap(&body, faces[0], &body, faces[0], band).map(|_|()));
    let _ = cube;
}
