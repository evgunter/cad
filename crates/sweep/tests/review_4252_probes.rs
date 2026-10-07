//! Review probes for PR 4252 (the blend doors take a finished operand).
//! Diagnostic: each row prints what it measured.


use geom::Surface;
use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::blend::build::{chamfer_edges, fillet_edges};
use sweep::test_support::{at_rest, revolved_about_y, rim_arcs_at};
use topo::{AtRestBody, Body, EdgeKey, FaceKey, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn faces_of(body: &Body<f64>, e: EdgeKey) -> (FaceKey, FaceKey) {
    let ed = body.get_edge(e).unwrap();
    let f = |he| {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    };
    (f(ed.he_plus), f(ed.he_minus))
}

fn is_plane(body: &Body<f64>, f: FaceKey) -> bool {
    matches!(
        body.get_surface(body.get_face(f).unwrap().surface).unwrap(),
        Surface::Plane { .. }
    )
}

/// fillet_h5_r2_probes' `merge_curved_wall`, verbatim.
fn merge_curved_wall(body: &mut Body<f64>, arcs: &[EdgeKey]) {
    for &a in arcs {
        let ed = body.get_edge(a).unwrap();
        for he in [ed.he_plus, ed.he_minus] {
            let v = body.get_half_edge(he).unwrap().start;
            let em = body.get_vertex(v).unwrap().emanating.unwrap();
            let orbit = body.vertex_orbit(em).unwrap();
            for h in orbit {
                let e = body.get_half_edge(h).unwrap().edge;
                if arcs.contains(&e) {
                    continue;
                }
                let (fa, fb) = faces_of(body, e);
                if fa == fb || is_plane(body, fa) || is_plane(body, fb) {
                    continue;
                }
                if body.get_face(fa).unwrap().surface != body.get_face(fb).unwrap().surface {
                    continue;
                }
                let hp = body.get_edge(e).unwrap().he_plus;
                body.kef(hp).expect("kef");
                return;
            }
        }
    }
    panic!("no curved co-surface seam meridian found at a rim vertex");
}

fn cylinder(r: f64, h: f64) -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(r, h), 0.0),
            (Point2::new(0.0, h), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

fn repaired(mut body: Body<f64>) -> Body<f64> {
    body.merge_coplanar_faces(tol()).expect("repair");
    body
}

/// Claim 4, half-band arm: a pole-FREE curved wall merged into one face
/// (a cylinder: killing one of its two seam meridians leaves no strut
/// tip). Does it finish, and does it reach the half-band gate?
#[test]
fn probe_cylinder_wall_merged_into_one_face() {
    for repair in [false, true] {
        let mut body = cylinder(1.0, 1.0);
        if repair {
            body = repaired(body);
        }
        let arcs = rim_arcs_at(&body, 1.0, 0.0);
        println!("[probe] repair={repair} arcs={}", arcs.len());
        merge_curved_wall(&mut body, &arcs);
        let v = validate_geometric(&body, tol());
        println!("[probe] repair={repair} tier3 on merged cylinder: {v:?}");
        assert_eq!(v, Ok(()), "the pole-free merged wall is a finished body");
        if v.is_ok() {
            let op = at_rest(&body, tol());
            let f = fillet_edges(&op, &arcs, 0.05, tol()).map(|b| b.body.faces().count());
            println!("[probe] repair={repair} fillet: {:?}", f.as_ref().map_err(|r| &r.error));
            assert!(
                matches!(
                    f.map_err(|r| r.error),
                    Err(sweep::blend::BlendError::UnsupportedChain { detail, .. })
                        if detail.contains("does not carry exactly its own rim arc")
                ),
                "a FINISHED body reaches the half-band gate's curved-single-host arm"
            );
            let c = chamfer_edges(&op, &arcs, 0.05, tol()).map(|b| b.body.faces().count());
            println!("[probe] repair={repair} chamfer: {:?}", c.map_err(|r| r.error));
        }
    }
}

/// Claim 4, hostless host gate: a repaired cylinder's base rim split
/// into three arcs (a valence-2 vertex, not scaffolding), with the
/// request naming two of them.
#[test]
fn probe_hostless_host_with_an_unrequested_rim_arc() {
    let mut body = repaired(cylinder(1.0, 1.0));
    let arcs = rim_arcs_at(&body, 1.0, 0.0);
    let (t0, t1) = {
        let ed = body.get_edge(arcs[0]).unwrap();
        body.get_curve_geom(ed.curve).unwrap().certified().unwrap().params()
    };
    body.split_edge(arcs[0], t0 + 0.5 * (t1 - t0), tol()).unwrap();
    let v = validate_geometric(&body, tol());
    println!("[probe] split rim tier3: {v:?}");
    let arcs3 = rim_arcs_at(&body, 1.0, 0.0);
    println!("[probe] rim arcs now {}", arcs3.len());
    if v.is_ok() {
        let op = AtRestBody::validate(body.clone(), tol()).unwrap();
        let all = fillet_edges(&op, &arcs3, 0.05, tol()).map(|b| b.body.faces().count());
        println!("[probe] all three arcs: {:?}", all.map_err(|r| r.error));
        for skip in 0..arcs3.len() {
            let some: Vec<_> = arcs3
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != skip)
                .map(|(_, e)| *e)
                .collect();
            let r = fillet_edges(&op, &some, 0.05, tol()).map(|b| b.body.faces().count());
            println!("[probe] skipping arc {skip}: {:?}", r.map_err(|r| r.error));
        }
    }
}

/// Claim 5: what the added tier-3 pass costs beside the blend it gates,
/// and whether a blended result finishes at f64 and at `Interval` (the
/// chained-blend seat pays tier 3 on its target).
#[test]
fn probe_chained_blend_tier3_at_f64_and_interval() {
    use geom_core::interval::Interval;
    use std::time::Instant;
    fn run<T: geom_core::Decide + geom_core::Bounds + topo::AtRestPolicy + geom_core::CertifiedBounds>(
        name: &str,
    ) {
        let cube: Body<T> = sweep::test_support::cube(1.0, tol());
        let edges: Vec<_> = cube.edges().map(|(k, _)| k).collect();
        let t = Instant::now();
        let op = AtRestBody::validate(cube.clone(), tol()).expect("cube finishes");
        let t_gate0 = t.elapsed();
        let t = Instant::now();
        let first = fillet_edges(&op, &edges[..4], T::from_f64(0.1), tol())
            .unwrap_or_else(|r| panic!("{name}: first fillet {:?}", r.error));
        let t_blend = t.elapsed();
        let t = Instant::now();
        let gated = AtRestBody::validate(first.body.clone(), tol());
        let t_gate = t.elapsed();
        println!(
            "[probe] {name}: cube gate {t_gate0:?}, first fillet (4 edges) {t_blend:?}, tier 3 on the result {t_gate:?}: {:?}",
            gated.as_ref().map(|_| ()).map_err(|e| e.clone())
        );
        if let Ok(op2) = gated {
            let rest: Vec<_> = edges[4..]
                .iter()
                .copied()
                .filter(|e| op2.get_edge(*e).is_some())
                .collect();
            let second = fillet_edges(&op2, &rest, T::from_f64(0.1), tol());
            println!(
                "[probe] {name}: chained fillet of {} remaining edges: {:?}",
                rest.len(),
                second.map(|b| b.body.faces().count()).map_err(|r| r.error)
            );
        }
        // Every edge at once, then the gate over the result.
        let t = Instant::now();
        let all = fillet_edges(&op, &edges, T::from_f64(0.1), tol())
            .unwrap_or_else(|r| panic!("{name}: all-edge fillet {:?}", r.error));
        let t_all = t.elapsed();
        let t = Instant::now();
        let g = AtRestBody::validate(all.body.clone(), tol()).map(|_| ());
        println!(
            "[probe] {name}: all-12 fillet {t_all:?}, tier 3 on it {:?}: {g:?}",
            t.elapsed()
        );
    }
    run::<f64>("f64");
    run::<Interval>("Interval");
}
