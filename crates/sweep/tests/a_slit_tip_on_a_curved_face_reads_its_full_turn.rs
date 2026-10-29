//! PROBE (measurement on main).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{SketchPlane, test_support::bulge_loop};
use sweep::test_support::{extruded, finished, revolved_about_y, sketch_from_axes};
use topo::{BooleanResult, intersect, split, subtract};

fn tol() -> Tol {
    Tol::witness()
}

fn lp(pts: &[(f64, f64, f64)]) -> profile::ProfileLoop<f64> {
    bulge_loop(pts.iter().map(|&(x, y, b)| (Point2::new(x, y), b)).collect())
}

/// The 4 × 4 × 4 block with a bore of radius 1 along z, less a V-trench
/// whose keel lies on the bore's ruling at (1, 0), from z = 2 up out of
/// the top.
fn bored_block() -> topo::AtRestBody<f64> {
    let outer = lp(&[(-2.0, -2.0, 0.0), (2.0, -2.0, 0.0), (2.0, 2.0, 0.0), (-2.0, 2.0, 0.0)]);
    let bore = lp(&[(0.0, 1.0, -1.0), (0.0, -1.0, -1.0)]);
    let block = finished("block", extruded(SketchPlane::xy(), vec![outer, bore], 4.0, tol()), tol());
    let tri = lp(&[(1.0, 0.0, 0.0), (1.6, -0.3, 0.0), (1.6, 0.3, 0.0)]);
    let trench = finished(
        "trench",
        extruded(SketchPlane::xy_at(Point3::new(0.0, 0.0, 2.0)), vec![tri], 3.0, tol()),
        tol(),
    );
    match subtract(&block, &trench, tol()) {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("bored block less trench: {other:?}"),
    }
}

/// The cone of half-angle 60° about +y, apex at the origin, height 1,
/// less a triangular prism along the ruling in the yz-plane from the
/// apex, opening inward.
fn notched_cone() -> topo::AtRestBody<f64> {
    let r = 3f64.sqrt();
    let cone = finished(
        "cone",
        revolved_about_y(
            vec![(Point2::new(0.0, 0.0), 0.0), (Point2::new(r, 1.0), 0.0), (Point2::new(0.0, 1.0), 0.0)],
            sweep::Revolution::Full,
            tol(),
        ),
        tol(),
    );
    let (s, c) = (60f64.to_radians().sin(), 60f64.to_radians().cos());
    let w = Vec3::new(0.0, s, -c);
    let t = Vec3::new(1.0, 0.0, 0.0);
    let plane = sketch_from_axes(Point3::new(0.0, 0.0, 0.0), t, w, tol());
    let tri = lp(&[(0.0, 0.0, 0.0), (0.1, 0.3, 0.0), (-0.1, 0.3, 0.0)]);
    let notch = finished("notch", extruded(plane, vec![tri], 0.6, tol()), tol());
    match subtract(&cone, &notch, tol()) {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("cone less notch: {other:?}"),
    }
}

fn slab(o: Point3<f64>, n: Vec3<f64>) -> topo::AtRestBody<f64> {
    // A 10-wide slab whose top face is the plane through `o` along `n`.
    let n = n / n.norm();
    let u = if n.x.abs() < 0.9 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) };
    let u = (u - n * u.dot(n)) / (u - n * u.dot(n)).norm();
    let v = n.cross(u);
    let base = o - n * 10.0;
    let plane = sketch_from_axes(base, u, v, tol());
    let sq = lp(&[(-5.0, -5.0, 0.0), (5.0, -5.0, 0.0), (5.0, 5.0, 0.0), (-5.0, 5.0, 0.0)]);
    finished("slab", extruded(plane, vec![sq], 10.0, tol()), tol())
}

fn probe(label: &str, body: &topo::AtRestBody<f64>, o: Point3<f64>, n: Vec3<f64>) {
    let plane = topo::test_support::split_plane(o, n / n.norm(), tol());
    match split(body, &plane, tol()) {
        Ok(_) => eprintln!("PROBE {label} split: ok"),
        Err(e) => eprintln!("PROBE {label} split: {e:?}"),
    }
    let s = slab(o, n);
    match intersect(body, &s, tol()) {
        Ok(_) => eprintln!("PROBE {label} intersect slab: ok"),
        Err(e) => eprintln!("PROBE {label} intersect slab: {e:?}"),
    }
}

#[test]
fn probe_slit_tips() {
    eprintln!("PROBE eps {:?}", tol());
    let b = std::panic::catch_unwind(bored_block);
    match b {
        Ok(b) => {
            let tip = Point3::new(1.0, 0.0, 2.0);
            probe("cyl z=2", &b, tip, Vec3::new(0.0, 0.0, 1.0));
            probe("cyl tilted", &b, tip, Vec3::new(-0.3, 0.1, 1.0));
        }
        Err(_) => eprintln!("PROBE bored block failed to build"),
    }
    match std::panic::catch_unwind(notched_cone) {
        Ok(c) => {
            let o = Point3::new(0.0, 0.0, 0.0);
            probe("cone x+0.3z", &c, o, Vec3::new(1.0, 0.0, 0.3));
            probe("cone tilted", &c, o, Vec3::new(1.0, 0.2, 0.5));
        }
        Err(_) => eprintln!("PROBE notched cone failed to build"),
    }
}

fn dump(label: &str, body: &topo::Body<f64>, at: Point3<f64>) {
    let pt = |v: topo::VertexKey| body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let kind = |f: Option<topo::FaceKey>| {
        f.map(|f| {
            format!("{f:?}:{:?}", topo::query::face_surface_kind(body, f))
        })
    };
    for (e, edge) in body.edges() {
        let h = edge.he_plus;
        let (a, b) = (body.get_half_edge(h).unwrap().start, body.half_edge_end(h).unwrap());
        if pt(a).distance(at) < 1e-9 || pt(b).distance(at) < 1e-9 {
            eprintln!(
                "PROBE {label} {e:?} {a:?}->{b:?} {:?}->{:?} faces {:?} {:?}",
                pt(a),
                pt(b),
                kind(body.face_of_half_edge(h)),
                kind(body.mate(h).and_then(|m| body.face_of_half_edge(m)))
            );
        }
    }
}

#[test]
fn probe_dump() {
    let b = bored_block();
    dump("cyl", &b, Point3::new(1.0, 0.0, 2.0));
}

#[test]
fn probe_tiers() {
    let b = bored_block();
    eprintln!("PROBE t3 {:?}", topo::validate_geometric(&b, tol()));
    eprintln!("PROBE t3' {:?}", topo::validate_pseudomanifold(&b, &topo::ContactRecords::default(), tol()));
}

#[test]
fn probe_volume() {
    let b = bored_block();
    eprintln!("PROBE vol {:?} want {}", topo::mass_properties(&b, tol(), topo::test_support::pinned_resolution(tol())).map(|m| m.enclosure.volume), 64.0 - 4.0 * std::f64::consts::PI - 0.36);
    for (f, _) in b.faces() {
        eprintln!("PROBE face {f:?} {:?}", topo::query::face_surface_kind(&b, f));
    }
}

#[test]
fn probe_orders() {
    let outer = || lp(&[(-2.0, -2.0, 0.0), (2.0, -2.0, 0.0), (2.0, 2.0, 0.0), (-2.0, 2.0, 0.0)]);
    let block = finished("block", extruded(SketchPlane::xy(), vec![outer()], 4.0, tol()), tol());
    let tri = lp(&[(1.0, 0.0, 0.0), (1.6, -0.3, 0.0), (1.6, 0.3, 0.0)]);
    let trench = finished(
        "trench",
        extruded(SketchPlane::xy_at(Point3::new(0.0, 0.0, 2.0)), vec![tri], 3.0, tol()),
        tol(),
    );
    let cyl = finished(
        "cyl",
        extruded(SketchPlane::xy_at(Point3::new(0.0, 0.0, -1.0)), vec![lp(&[(0.0, -1.0, 1.0), (0.0, 1.0, 1.0)])], 6.0, tol()),
        tol(),
    );
    let bored = finished(
        "bored",
        extruded(SketchPlane::xy_at(Point3::new(0.0, 0.0, -1.0)), vec![lp(&[(-3.0, -3.0, 0.0), (3.0, -3.0, 0.0), (3.0, 3.0, 0.0), (-3.0, 3.0, 0.0)]), lp(&[(0.0, 1.0, -1.0), (0.0, -1.0, -1.0)])], 6.0, tol()),
        tol(),
    );
    let cut = match subtract(&block, &trench, tol()) {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("{other:?}"),
    };
    for (label, r) in [
        ("(block-trench)-cyl", subtract(&cut, &cyl, tol())),
        ("(block-trench)&bored", intersect(&cut, &bored, tol())),
        ("bored&(block-trench)", intersect(&bored, &cut, tol())),
    ] {
        match r {
            Ok(BooleanResult::Body(b)) => {
                eprintln!("PROBE {label}: ok t3' {:?}", topo::validate_pseudomanifold(&b.body, &topo::ContactRecords::default(), tol()));
                dump(label, &b.body, Point3::new(1.0, 0.0, 2.0));
            }
            other => eprintln!("PROBE {label}: {other:?}"),
        }
    }
}

#[test]
fn probe_union() {
    let bored = finished(
        "bored",
        extruded(SketchPlane::xy(), vec![lp(&[(-2.0, -2.0, 0.0), (2.0, -2.0, 0.0), (2.0, 2.0, 0.0), (-2.0, 2.0, 0.0)]), lp(&[(0.0, 1.0, -1.0), (0.0, -1.0, -1.0)])], 4.0, tol()),
        tol(),
    );
    let tool = |pts: &[(f64, f64)], z0: f64, h: f64| {
        let l: Vec<_> = pts.iter().map(|&(x, y)| (x, y, 0.0)).collect();
        finished("tool", extruded(SketchPlane::xy_at(Point3::new(0.0, 0.0, z0)), vec![lp(&l)], h, tol()), tol())
    };
    // Trench above z = 2 only: below it P and Q share the V region.
    let p_tool = tool(&[(1.0, 0.0), (5.0, 2.0), (5.0, 5.0), (-5.0, 5.0), (-5.0, 0.0)], 2.0, 3.0);
    let q_tool = tool(&[(1.0, 0.0), (-5.0, 0.0), (-5.0, -5.0), (5.0, -5.0), (5.0, -2.0)], 2.0, 3.0);
    let lower = tool(&[(-5.0, -5.0), (5.0, -5.0), (5.0, 5.0), (-5.0, 5.0)], -1.0, 3.0);
    let and = |a: &topo::AtRestBody<f64>, b: &topo::AtRestBody<f64>, what: &str| match intersect(a, b, tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{what}: {other:?}"),
    };
    let p = and(&bored, &p_tool, "P");
    let q = and(&bored, &q_tool, "Q");
    let l = and(&bored, &lower, "lower");
    let pq = match topo::union(&p, &q, tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => { eprintln!("PROBE P∪Q: {other:?}"); return; }
    };
    eprintln!("PROBE P∪Q ok t3' {:?}", topo::validate_pseudomanifold(&pq, &topo::ContactRecords::default(), tol()));
    dump("PQ", &pq, Point3::new(1.0, 0.0, 2.0));
    match topo::union(&pq, &l, tol()) {
        Ok(BooleanResult::Body(r)) => {
            eprintln!("PROBE all ok t3' {:?}", topo::validate_pseudomanifold(&r.body, &topo::ContactRecords::default(), tol()));
            dump("all", &r.body, Point3::new(1.0, 0.0, 2.0));
            let tip = Point3::new(1.0, 0.0, 2.0);
            probe("all z=2", &r.body, tip, Vec3::new(0.0, 0.0, 1.0));
            probe("all tilted", &r.body, tip, Vec3::new(-0.3, 0.1, 1.0));
            probe("all tilted2", &r.body, tip, Vec3::new(0.3, 0.2, 1.0));
        }
        other => eprintln!("PROBE PQ∪L: {other:?}"),
    }
}

#[test]
fn probe_fill() {
    let bored = finished(
        "bored",
        extruded(SketchPlane::xy(), vec![lp(&[(-2.0, -2.0, 0.0), (2.0, -2.0, 0.0), (2.0, 2.0, 0.0), (-2.0, 2.0, 0.0)]), lp(&[(0.0, 1.0, -1.0), (0.0, -1.0, -1.0)])], 4.0, tol()),
        tol(),
    );
    let tool = |pts: &[(f64, f64)], z0: f64, h: f64| {
        let l: Vec<_> = pts.iter().map(|&(x, y)| (x, y, 0.0)).collect();
        finished("tool", extruded(SketchPlane::xy_at(Point3::new(0.0, 0.0, z0)), vec![lp(&l)], h, tol()), tol())
    };
    let p_tool = tool(&[(1.0, 0.0), (5.0, 2.0), (5.0, 5.0), (-5.0, 5.0), (-5.0, 0.0)], -1.0, 6.0);
    let q_tool = tool(&[(1.0, 0.0), (-5.0, 0.0), (-5.0, -5.0), (5.0, -5.0), (5.0, -2.0)], -1.0, 6.0);
    let and = |a: &topo::AtRestBody<f64>, b: &topo::AtRestBody<f64>, what: &str| match intersect(a, b, tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{what}: {other:?}"),
    };
    let p = and(&bored, &p_tool, "P");
    let q = and(&bored, &q_tool, "Q");
    let pq = match topo::union(&p, &q, tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => { eprintln!("PROBE P∪Q: {other:?}"); return; }
    };
    // The V between the walls inside the block, below z = 2.
    let w = tool(&[(1.0, 0.0), (3.0, -1.0), (3.0, 1.0)], 0.0, 2.0);
    let w = and(&w, &tool(&[(-2.0, -2.0), (2.0, -2.0), (2.0, 2.0), (-2.0, 2.0)], -1.0, 6.0), "W");
    for (label, r) in [("PQ∪W", topo::union(&pq, &w, tol())), ("W∪PQ", topo::union(&w, &pq, tol()))] {
        match r {
            Ok(BooleanResult::Body(r)) => {
                eprintln!("PROBE {label} ok t3' {:?}", topo::validate_pseudomanifold(&r.body, &topo::ContactRecords::default(), tol()));
                dump(label, &r.body, Point3::new(1.0, 0.0, 2.0));
                let tip = Point3::new(1.0, 0.0, 2.0);
                probe(&format!("{label} z=2"), &r.body, tip, Vec3::new(0.0, 0.0, 1.0));
                probe(&format!("{label} tilted"), &r.body, tip, Vec3::new(-0.3, 0.1, 1.0));
            }
            other => eprintln!("PROBE {label}: {other:?}"),
        }
    }
}

#[test]
fn probe_tb() {
    let bored = finished(
        "bored",
        extruded(SketchPlane::xy(), vec![lp(&[(-2.0, -2.0, 0.0), (2.0, -2.0, 0.0), (2.0, 2.0, 0.0), (-2.0, 2.0, 0.0)]), lp(&[(0.0, 1.0, -1.0), (0.0, -1.0, -1.0)])], 4.0, tol()),
        tol(),
    );
    let tool = |pts: &[(f64, f64)], z0: f64, h: f64| {
        let l: Vec<_> = pts.iter().map(|&(x, y)| (x, y, 0.0)).collect();
        finished("tool", extruded(SketchPlane::xy_at(Point3::new(0.0, 0.0, z0)), vec![lp(&l)], h, tol()), tol())
    };
    let p_tool = tool(&[(1.0, 0.0), (5.0, 2.0), (5.0, 5.0), (-5.0, 5.0), (-5.0, 0.0)], -1.0, 6.0);
    let q_tool = tool(&[(1.0, 0.0), (-5.0, 0.0), (-5.0, -5.0), (5.0, -5.0), (5.0, -2.0)], -1.0, 6.0);
    let v_low = tool(&[(1.0, 0.0), (5.0, -2.0), (5.0, 2.0)], -1.0, 3.0);
    let tb = match topo::union(&q_tool, &v_low, tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => { eprintln!("PROBE TB': {other:?}"); return; }
    };
    let and = |a: &topo::AtRestBody<f64>, b: &topo::AtRestBody<f64>, what: &str| match intersect(a, b, tol()) {
        Ok(BooleanResult::Body(r)) => Some(r.body),
        other => { eprintln!("PROBE {what}: {other:?}"); None }
    };
    let Some(p) = and(&bored, &p_tool, "P") else { return };
    let Some(q) = and(&bored, &tb, "Q'") else { return };
    for (label, r) in [("P∪Q'", topo::union(&p, &q, tol())), ("Q'∪P", topo::union(&q, &p, tol()))] {
        match r {
            Ok(BooleanResult::Body(r)) => {
                eprintln!("PROBE {label} ok t3' {:?}", topo::validate_pseudomanifold(&r.body, &topo::ContactRecords::default(), tol()));
                dump(label, &r.body, Point3::new(1.0, 0.0, 2.0));
                let tip = Point3::new(1.0, 0.0, 2.0);
                probe(&format!("{label} z=2"), &r.body, tip, Vec3::new(0.0, 0.0, 1.0));
                probe(&format!("{label} tilted"), &r.body, tip, Vec3::new(-0.3, 0.1, 1.0));
            }
            other => eprintln!("PROBE {label}: {other:?}"),
        }
    }
}

#[test]
fn probe_groove() {
    let tool = |pts: &[(f64, f64)], z0: f64, h: f64| {
        let l: Vec<_> = pts.iter().map(|&(x, y)| (x, y, 0.0)).collect();
        finished("tool", extruded(SketchPlane::xy_at(Point3::new(0.0, 0.0, z0)), vec![lp(&l)], h, tol()), tol())
    };
    // The block [0, 4] × [-2, 2] × [0, 4] with a half-bore of radius 1
    // about the z axis in its x = 0 face.
    let grooved = finished(
        "grooved",
        extruded(
            SketchPlane::xy(),
            vec![lp(&[(0.0, -2.0, 0.0), (4.0, -2.0, 0.0), (4.0, 2.0, 0.0), (0.0, 2.0, 0.0), (0.0, 1.0, -1.0), (0.0, -1.0, 0.0)])],
            4.0,
            tol(),
        ),
        tol(),
    );
    let p_tool = tool(&[(1.0, 0.0), (5.0, 2.0), (5.0, 5.0), (-5.0, 5.0), (-5.0, 0.0)], -1.0, 6.0);
    let q_tool = tool(&[(1.0, 0.0), (-5.0, 0.0), (-5.0, -5.0), (5.0, -5.0), (5.0, -2.0)], -1.0, 6.0);
    let v_low = tool(&[(1.0, 0.0), (5.0, -2.0), (5.0, 2.0)], -1.0, 3.0);
    let and = |a: &topo::AtRestBody<f64>, b: &topo::AtRestBody<f64>, what: &str| match intersect(a, b, tol()) {
        Ok(BooleanResult::Body(r)) => Some(r.body),
        other => { eprintln!("PROBE {what}: {other:?}"); None }
    };
    let tb = match topo::union(&q_tool, &v_low, tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => { eprintln!("PROBE TB': {other:?}"); return; }
    };
    let Some(p) = and(&grooved, &p_tool, "P") else { return };
    let Some(q) = and(&grooved, &tb, "Q'") else { return };
    let trench = tool(&[(1.0, 0.0), (5.0, -2.0), (5.0, 2.0)], 2.0, 3.0);
    for (label, r) in [
        ("P∪Q'", topo::union(&p, &q, tol())),
        ("grooved-trench", subtract(&grooved, &trench, tol())),
    ] {
        match r {
            Ok(BooleanResult::Body(r)) => {
                eprintln!("PROBE {label} ok t3' {:?}", topo::validate_pseudomanifold(&r.body, &topo::ContactRecords::default(), tol()));
                dump(label, &r.body, Point3::new(1.0, 0.0, 2.0));
                let tip = Point3::new(1.0, 0.0, 2.0);
                probe(&format!("{label} z=2"), &r.body, tip, Vec3::new(0.0, 0.0, 1.0));
                probe(&format!("{label} tilted"), &r.body, tip, Vec3::new(-0.3, 0.1, 1.0));
            }
            other => eprintln!("PROBE {label}: {other:?}"),
        }
    }
}

fn cone() -> topo::AtRestBody<f64> {
    let r = 3f64.sqrt();
    finished(
        "cone",
        revolved_about_y(
            vec![(Point2::new(0.0, 0.0), 0.0), (Point2::new(r, 1.0), 0.0), (Point2::new(0.0, 1.0), 0.0)],
            sweep::Revolution::Full,
            tol(),
        ),
        tol(),
    )
}

/// A prism over `pts` drawn in the plane through `o` spanned by (u, v),
/// pushed `h` along u × v, from `-h/2`.
fn slab_prism(o: Point3<f64>, u: Vec3<f64>, v: Vec3<f64>, pts: &[(f64, f64)], h: f64) -> topo::AtRestBody<f64> {
    let n = u.cross(v);
    let plane = sketch_from_axes(o - n * (h / 2.0), u, v, tol());
    let l: Vec<_> = pts.iter().map(|&(x, y)| (x, y, 0.0)).collect();
    finished("prism", extruded(plane, vec![lp(&l)], h, tol()), tol())
}

#[test]
fn probe_cone() {
    let c = cone();
    dump("cone", &c, Point3::new(0.0, 0.0, 0.0));
    // The ruling d in the yz-plane; walls through it at ±φ about it.
    let (s, co) = (60f64.to_radians().sin(), 60f64.to_radians().cos());
    let d = Vec3::new(0.0, co, s);
    let w = Vec3::new(0.0, s, -co); // inward, square to d
    let t = Vec3::new(1.0, 0.0, 0.0);
    let o = Point3::new(0.0, 0.0, 0.0);
    // In the (t, w) frame along d: wall a through (0,0) toward (0.3, 1),
    // wall b toward (-0.3, 1).
    let p_tool = slab_prism(o, t, w, &[(0.0, 0.0), (3.0, 10.0), (10.0, 10.0), (10.0, -10.0), (0.0, -10.0)], 20.0);
    let q_tool = slab_prism(o, t, w, &[(0.0, 0.0), (0.0, -10.0), (-10.0, -10.0), (-10.0, 10.0), (-3.0, 10.0)], 20.0);
    let v_tool = slab_prism(o, t, w, &[(0.0, 0.0), (3.0, 10.0), (-3.0, 10.0)], 20.0);
    let and = |a: &topo::AtRestBody<f64>, b: &topo::AtRestBody<f64>, what: &str| match intersect(a, b, tol()) {
        Ok(BooleanResult::Body(r)) => Some(r.body),
        other => { eprintln!("PROBE {what}: {other:?}"); None }
    };
    let mut bodies = vec![];
    match subtract(&c, &v_tool, tol()) {
        Ok(BooleanResult::Body(r)) => bodies.push(("C-V", r.body)),
        other => eprintln!("PROBE C-V: {other:?}"),
    }
    if let (Some(p), Some(q)) = (and(&c, &p_tool, "P"), and(&c, &q_tool, "Q")) {
        match topo::union(&p, &q, tol()) {
            Ok(BooleanResult::Body(r)) => bodies.push(("P∪Q", r.body)),
            other => eprintln!("PROBE P∪Q: {other:?}"),
        }
    }
    for (label, b) in &bodies {
        eprintln!("PROBE {label} ok t3' {:?}", topo::validate_pseudomanifold(b, &topo::ContactRecords::default(), tol()));
        dump(label, b, o);
        probe(&format!("{label} n=(1,0,.3)"), b, o, Vec3::new(1.0, 0.0, 0.3));
        probe(&format!("{label} n=(1,.2,.5)"), b, o, Vec3::new(1.0, 0.2, 0.5));
    }
}
