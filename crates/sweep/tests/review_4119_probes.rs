//! Review probes for PR 4119 (ring carry-through reads polygonal rings).
//! Exploratory: each row prints what it measured; asserts pin only what
//! the review report states.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Tol, Vec3};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{cube, prism, realized};
use topo::boolean::BooleanOp;
use topo::{Body, EdgeKey, query, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn v(x: f64, y: f64, bulge: f64) -> (Point2<f64>, f64) {
    (Point2::new(x, y), bulge)
}

fn sub(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    realized(BooleanOp::Subtract, a, b, tol())
}

fn uni(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    realized(BooleanOp::Union, a, b, tol())
}

fn at(b: Body<f64>, x: f64, y: f64, z: f64) -> Body<f64> {
    topo::transform_rigid(&b, &Affine3::translation(Vec3::new(x, y, z)), tol()).unwrap()
}

fn carrier(body: &Body<f64>, k: EdgeKey) -> (geom::Curve3<f64>, (f64, f64)) {
    let e = body.get_edge(k).unwrap();
    let g = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
    (g.carrier().clone(), g.params())
}

fn mid(body: &Body<f64>, k: EdgeKey) -> geom_core::Point3<f64> {
    let (c, (t0, t1)) = carrier(body, k);
    c.eval((t0 + t1) / 2.0)
}

fn is_line(body: &Body<f64>, k: EdgeKey) -> bool {
    matches!(carrier(body, k).0, geom::Curve3::Line { .. })
}

fn outer_box_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let on = |c: f64| c.abs() < 1e-9 || (c - 1.0).abs() < 1e-9;
    query::all_edges(body)
        .into_iter()
        .filter(|k| is_line(body, *k))
        .filter(|k| {
            let m = mid(body, *k);
            on(m.x) || on(m.y)
        })
        .collect()
}

fn vol(b: &Body<f64>) -> f64 {
    topo::mass_properties(b, tol()).unwrap().volume
}

/// The material a unit cube keeps once its twelve edges are rounded at r.
fn rounded_cube(r: f64) -> f64 {
    let l = 1.0 - 2.0 * r;
    l.powi(3) + 6.0 * l * l * r + 3.0 * PI * l * r * r + (4.0 / 3.0) * PI * r.powi(3)
}

/// Run a fillet and report: Ok(volume, tier3) or the error text.
fn attempt(body: &Body<f64>, edges: &[EdgeKey], r: f64) -> String {
    match fillet_edges(body, edges, r, tol()) {
        Ok(out) => {
            let t3 = validate_geometric(&out.body, tol());
            format!(
                "BUILT vol={:.12} tier3={}",
                vol(&out.body),
                match t3 {
                    Ok(()) => "ok".to_string(),
                    Err(e) => format!("FAIL {e:?}"),
                }
            )
        }
        Err(e) => format!("REFUSED {:?}", e.error),
    }
}

fn short(s: &str) -> String {
    s.chars().take(400).collect()
}

/// Claim 1: a polygon vertex walked across the setback, at several
/// abscissae; margin printed vs the closed form `d − r`.
#[test]
fn p1_diamond_vertex_sweep() {
    for &(cx, d) in &[(0.45, 0.20), (0.5, 0.20), (0.12, 0.20), (0.88, 0.2), (0.2, 0.2)] {
        let h = 0.1;
        let dia = prism(
            vec![
                v(cx, d, 0.0),
                v(cx + h, d + h, 0.0),
                v(cx, d + 2.0 * h, 0.0),
                v(cx - h, d + h, 0.0),
            ],
            0.3,
            tol(),
        );
        let body = sub(&cube(1.0, tol()), &at(dia, 0.0, 0.0, 0.8));
        if validate_geometric(&body, tol()).is_err() {
            println!("cx={cx}: fixture invalid");
            continue;
        }
        let outer = outer_box_edges(&body);
        for r in [0.15, 0.199, 0.201, 0.25] {
            let res = attempt(&body, &outer, r);
            let want = rounded_cube(r) - 2.0 * h * h * 0.2;
            println!("P1 cx={cx} d={d} r={r}: closed d-r={:+.4} want_vol={want:.12} -> {}", d - r, short(&res));
        }
    }
}

/// Claim 1: a polygon EDGE grazing parallel to the box edge, and a
/// polygon near a corner where two bands meet.
#[test]
fn p1_edge_parallel_and_corner() {
    // parallel edge at distance d from y=0
    let d = 0.2;
    let sq = prism(
        vec![v(0.3, d, 0.0), v(0.7, d, 0.0), v(0.7, 0.5, 0.0), v(0.3, 0.5, 0.0)],
        0.3,
        tol(),
    );
    let body = sub(&cube(1.0, tol()), &at(sq, 0.0, 0.0, 0.8));
    let outer = outer_box_edges(&body);
    for r in [0.19, 0.2 - 5.0 * tol().get().eps, 0.21] {
        let want = rounded_cube(r) - 0.4 * 0.3 * 0.2;
        println!("P1par r={r}: want {want:.12} -> {}", short(&attempt(&body, &outer, r)));
    }
    // a square near the corner (0,0): its corner at (0.22,0.22)
    let sq = prism(
        vec![v(0.22, 0.22, 0.0), v(0.4, 0.22, 0.0), v(0.4, 0.4, 0.0), v(0.22, 0.4, 0.0)],
        0.3,
        tol(),
    );
    let body = sub(&cube(1.0, tol()), &at(sq, 0.0, 0.0, 0.8));
    let outer = outer_box_edges(&body);
    for r in [0.2, 0.215, 0.225] {
        let want = rounded_cube(r) - 0.18 * 0.18 * 0.2;
        println!("P1cor r={r}: want {want:.12} -> {}", short(&attempt(&body, &outer, r)));
    }
}

/// Over-refusal / wrong refusal: fillet the MOUTH of a square pocket
/// (its four edges are a ring of the top face), and the FOOT of a
/// square boss unioned on.
#[test]
fn p1_pocket_mouth_and_boss_foot() {
    let pocket = prism(
        vec![v(0.3, 0.3, 0.0), v(0.7, 0.3, 0.0), v(0.7, 0.7, 0.0), v(0.3, 0.7, 0.0)],
        0.3,
        tol(),
    );
    let body = sub(&cube(1.0, tol()), &at(pocket, 0.0, 0.0, 0.8));
    let mouth: Vec<EdgeKey> = query::all_edges(&body)
        .into_iter()
        .filter(|k| is_line(&body, *k))
        .filter(|k| {
            let m = mid(&body, *k);
            (m.z - 1.0).abs() < 1e-9 && m.x > 0.2 && m.x < 0.8 && m.y > 0.2 && m.y < 0.8
        })
        .collect();
    println!("mouth edges {}", mouth.len());
    for r in [0.02, 0.05] {
        println!("P1mouth4 r={r}: {}", short(&attempt(&body, &mouth, r)));
        println!("P1mouth1 r={r}: {}", short(&attempt(&body, &mouth[..1], r)));
    }
    let boss = prism(
        vec![v(0.3, 0.3, 0.0), v(0.7, 0.3, 0.0), v(0.7, 0.7, 0.0), v(0.3, 0.7, 0.0)],
        0.3,
        tol(),
    );
    let body = uni(&cube(1.0, tol()), &at(boss, 0.0, 0.0, 0.9));
    let foot: Vec<EdgeKey> = query::all_edges(&body)
        .into_iter()
        .filter(|k| is_line(&body, *k))
        .filter(|k| {
            let m = mid(&body, *k);
            (m.z - 1.0).abs() < 1e-9 && m.x > 0.2 && m.x < 0.8 && m.y > 0.2 && m.y < 0.8
        })
        .collect();
    println!("foot edges {}", foot.len());
    for r in [0.02, 0.05] {
        println!("P1foot4 r={r}: {}", short(&attempt(&body, &foot, r)));
        println!("P1foot1 r={r}: {}", short(&attempt(&body, &foot[..1], r)));
    }
    let outer = outer_box_edges(&body);
    println!("P1boss outer12 r=0.1 n={}: {}", outer.len(), short(&attempt(&body, &outer, 0.1)));
}

fn bore(x: f64, y: f64, rad: f64, z: f64, h: f64) -> Body<f64> {
    at(prism(vec![v(x - rad, y, 1.0), v(x + rad, y, 1.0)], h, tol()), 0.0, 0.0, z)
}

/// Claim 2: a ring of THREE arcs (three overlapping bores), every
/// subtraction order; and the lens with a trim crossing near the cusp.
#[test]
fn p2_three_arc_ring() {
    let c = [(0.5, 0.5), (0.45, 0.35), (0.6, 0.38)];
    let orders = [[0, 1, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    for o in orders {
        let mut body = cube(1.0, tol());
        for (i, &k) in o.iter().enumerate() {
            body = sub(&body, &bore(c[k].0, c[k].1, 0.1, 0.8 - 0.05 * i as f64, 0.5));
        }
        if let Err(e) = validate_geometric(&body, tol()) {
            println!("P2 order {o:?}: fixture invalid {e:?}");
            continue;
        }
        let outer = outer_box_edges(&body);
        // nearest approach to y=0: bore (0.45,0.35) at 0.25 ; to x=1? no.
        let floor_area_note = "";
        for r in [0.24, 0.249, 0.2502, 0.252, 0.26] {
            println!("P2 three-arc order {o:?} n={} r={r}: {}{floor_area_note}", outer.len(), short(&attempt(&body, &outer, r)));
        }
    }
}

/// Material a convex fillet of radius r removes at a hole's planar rim
/// of radius `big_r` (Pappus over the corner region).
fn hole_rim_removed(big_r: f64, r: f64) -> f64 {
    let c = (10.0 - 3.0 * PI) / (3.0 * (4.0 - PI));
    2.0 * PI * (big_r + c * r) * (1.0 - PI / 4.0) * r * r
}

fn rim_at(body: &Body<f64>, z: f64, rad: f64) -> Vec<EdgeKey> {
    query::all_edges(body)
        .into_iter()
        .filter(|k| {
            matches!(carrier(body, *k).0, geom::Curve3::Circle { center, radius, .. }
                if (center.z - z).abs() < 1e-9 && (radius - rad).abs() < 1e-9)
        })
        .collect()
}

fn attempt_rim(body: &Body<f64>, rim: &[EdgeKey], r: f64, big_r: f64) -> String {
    let want = vol(body) - hole_rim_removed(big_r, r);
    format!("want={want:.12} {}", attempt(body, rim, r))
}

/// Claim 2: a lens ring BESIDE a rim — arm (b) via the support-boundary
/// walk — in both subtraction orders.
#[test]
fn p2_lens_beside_rim() {
    for flip in [false, true] {
        // lens arc of bore a nearest the hole: centre 0.4 from axis, radius 0.1 -> near 0.3
        let (a, b) = ((0.3 + 0.4 * 0.19635f64.cos(), 0.5 + 0.4 * 0.19635f64.sin()), (0.3 + 0.46 * 0.45f64.cos(), 0.5 + 0.46 * 0.45f64.sin()));
        let hole = bore(0.3, 0.5, 0.2, -0.2, 1.4);
        let mut body = sub(&cube(1.0, tol()), &hole);
        let (p, q) = if flip { (b, a) } else { (a, b) };
        body = sub(&body, &bore(p.0, p.1, 0.1, 0.8, 0.5));
        body = sub(&body, &bore(q.0, q.1, 0.1, 0.7, 0.5));
        if let Err(e) = validate_geometric(&body, tol()) {
            println!("P2lensrim fixture invalid {e:?}");
            continue;
        }
        let rim = rim_at(&body, 1.0, 0.2);
        for r in [0.09, 0.099, 0.101, 0.11] {
            println!("P2lensrim flip={flip} n={} r={r} closed={:+.4}: {}", rim.len(), 0.3 - 0.2 - r, short(&attempt_rim(&body, &rim, r, 0.2)));
        }
    }
}

/// Claim 3: a curved mate's ring. A hole's wall carries a notch ring of
/// two rulings and two arcs at z in [0.8,0.9], at +y (inside one wall
/// face); filleting the top rim excises the wall strip z in [1-r, 1].
#[test]
fn p3_mate_ring_notch() {
    let hole = bore(0.5, 0.5, 0.3, -0.2, 1.4);
    let base = sub(&cube(1.0, tol()), &hole);
    let notch = at(
        prism(vec![v(0.47, 0.75, 0.0), v(0.53, 0.75, 0.0), v(0.53, 0.86, 0.0), v(0.47, 0.86, 0.0)], 0.1, tol()),
        0.0,
        0.0,
        0.8,
    );
    let body = sub(&base, &notch);
    validate_geometric(&body, tol()).expect("fixture");
    let rim = rim_at(&body, 1.0, 0.3);
    for f in topo::query::all_faces(&body) {
        let fd = body.get_face(f).unwrap();
        if !fd.rings.is_empty() {
            println!("P3 face {f:?} rings {} surf {:?}", fd.rings.len(), core::mem::discriminant(body.get_surface(fd.surface).unwrap()));
        }
    }
    for r in [0.09, 0.099, 0.1 - 5.0 * tol().get().eps, 0.101, 0.11, 0.15] {
        println!("P3 notch n={} r={r} closed={:+.4}: {}", rim.len(), 0.1 - r, short(&attempt_rim(&body, &rim, r, 0.3)));
    }
}

/// A stadium pocket's mouth (two lines, two arcs) filleted whole.
#[test]
fn p1_stadium_mouth() {
    let st = prism(vec![v(0.35, 0.4, 0.0), v(0.65, 0.4, -1.0), v(0.65, 0.6, 0.0), v(0.35, 0.6, -1.0)], 0.3, tol());
    let body = sub(&cube(1.0, tol()), &at(st, 0.0, 0.0, 0.8));
    let mouth: Vec<EdgeKey> = query::all_edges(&body)
        .into_iter()
        .filter(|k| {
            let m = mid(&body, *k);
            (m.z - 1.0).abs() < 1e-9 && m.x > 0.2 && m.x < 0.8 && m.y > 0.2 && m.y < 0.8
        })
        .collect();
    println!("P1stadium n={} r=0.03: {}", mouth.len(), short(&attempt(&body, &mouth, 0.03)));
    let outer = outer_box_edges(&body);
    let want = vol(&body) - (1.0 - rounded_cube(0.3));
    println!("P1stadium outer r=0.3 want={want:.12}: {}", short(&attempt(&body, &outer, 0.3)));
    println!("P1stadium outer r=0.401: {}", short(&attempt(&body, &outer, 0.401)));
}

/// Hostless annulus (the trim becomes the top face's outer boundary):
/// a disc top with a polygon / lens ring inside, rim filleted convex.
/// Trim radius R - r; clear iff the ring's far reach < R - r.
#[test]
fn p2_hostless_annulus_rings() {
    let big = 0.5;
    let cyl = bore(0.5, 0.5, big, 0.0, 1.0);
    // a square whose far corner is 0.3 from the axis at 11.25 deg off x
    let a = 11.25f64.to_radians();
    let (ux, uy) = (a.cos(), a.sin());
    let h = 0.08;
    let (px, py) = (0.5 + 0.3 * ux, 0.5 + 0.3 * uy);
    let sq = prism(
        vec![
            v(px, py, 0.0),
            v(px - h * (ux - uy), py - h * (uy + ux), 0.0),
            v(px - 2.0 * h * ux, py - 2.0 * h * uy, 0.0),
            v(px - h * (ux + uy), py - h * (uy - ux), 0.0),
        ],
        0.3,
        tol(),
    );
    let body = sub(&cyl, &at(sq, 0.0, 0.0, 0.8));
    validate_geometric(&body, tol()).expect("fixture");
    let rim = rim_at(&body, 1.0, big);
    let pocket_vol = 2.0 * h * h * 0.2;
    for r in [0.15, 0.199, 0.2 - 5.0 * tol().get().eps, 0.201, 0.21] {
        // convex rim of a disc top: removed corner region at radius R - c r
        let c = (10.0 - 3.0 * PI) / (3.0 * (4.0 - PI));
        let want = PI * big * big - pocket_vol - 2.0 * PI * (big - c * r) * (1.0 - PI / 4.0) * r * r;
        println!("P2annulus-square n={} r={r} closed={:+.5}: want={want:.12} {}", rim.len(), 0.3 - (big - r), short(&attempt(&body, &rim, r)));
    }
    // a lens of two bores: far reach of the outer bore 0.25+0.06 = 0.31 at 11.25 deg
    for flip in [false, true] {
        let p = (0.5 + 0.25 * ux, 0.5 + 0.25 * uy);
        let q = (0.5 + 0.2 * (0.6f64).cos(), 0.5 + 0.2 * (0.6f64).sin());
        let (f, g) = if flip { (q, p) } else { (p, q) };
        let mut body = sub(&cyl, &bore(f.0, f.1, 0.06, 0.8, 0.5));
        body = sub(&body, &bore(g.0, g.1, 0.06, 0.7, 0.5));
        if let Err(e) = validate_geometric(&body, tol()) {
            println!("P2annulus-lens fixture invalid {e:?}");
            continue;
        }
        let rim = rim_at(&body, 1.0, big);
        for r in [0.15, 0.189, 0.191, 0.2] {
            let c = (10.0 - 3.0 * PI) / (3.0 * (4.0 - PI));
            let want = vol(&body) - 2.0 * PI * (big - c * r) * (1.0 - PI / 4.0) * r * r;
            println!("P2annulus-lens flip={flip} n={} r={r} closed={:+.5}: want={want:.12} {}", rim.len(), 0.31 - (big - r), short(&attempt(&body, &rim, r)));
        }
    }
}

/// Claim 5 at `Interval`: the diamond pocket and the hostless-annulus
/// square, at a clear, a crossed and an in-band radius.
mod interval_lane {
    use crate::common::interval::{iv, p2, v3};
    use geom_core::{Affine3, Bounds, Interval, Tol};
    use sweep::blend::build::fillet_edges;
    use sweep::test_support::{cube, finished, prism};
    use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
    use topo::{Body, BooleanDeclarations, EdgeKey};

    fn t() -> Tol {
        Tol::witness()
    }
    fn sub(a: Body<Interval>, b: Body<Interval>) -> Body<Interval> {
        boolean_op_with(
            BooleanOp::Subtract,
            &finished("a", a, t()),
            &finished("b", b, t()),
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            t(),
        )
        .expect("the cut at Interval")
        .body()
        .expect("a body")
        .body
        .clone()
        .into_body()
    }
    fn at(b: Body<Interval>, z: f64) -> Body<Interval> {
        topo::transform_rigid(&b, &Affine3::translation(v3(0.0, 0.0, z)), t()).unwrap()
    }
    fn report(body: &Body<Interval>, edges: &[EdgeKey], r: f64, what: &str) {
        match fillet_edges(body, edges, iv(r), t()) {
            Ok(out) => {
                let v = topo::mass_properties(&out.body, t()).unwrap().volume;
                println!(
                    "IV {what} r={r}: BUILT vol=[{:.12},{:.12}] tier3={:?}",
                    v.lo(),
                    v.hi(),
                    topo::validate_geometric(&out.body, t())
                );
            }
            Err(e) => println!("IV {what} r={r}: REFUSED {:?}", e.error),
        }
    }

    #[test]
    fn diamond_and_annulus_at_interval() {
        let dia = prism(
            vec![
                (p2(0.45, 0.20), iv(0.0)),
                (p2(0.55, 0.30), iv(0.0)),
                (p2(0.45, 0.40), iv(0.0)),
                (p2(0.35, 0.30), iv(0.0)),
            ],
            iv(0.3),
            t(),
        );
        let body = sub(cube(1.0, t()), at(dia, 0.8));
        let on = |c: Interval| c.lo().abs() < 1e-9 || (c.lo() - 1.0).abs() < 1e-9;
        let outer: Vec<EdgeKey> = body
            .edges()
            .filter_map(|(k, e)| {
                let g = body.get_curve_geom(e.curve)?.certified()?;
                let geom::Curve3::Line { .. } = g.carrier() else { return None };
                let (t0, t1) = g.params();
                let m = g.carrier().eval((t0 + t1) / iv(2.0));
                (on(m.x) || on(m.y)).then_some(k)
            })
            .collect();
        println!("IV diamond outer n={}", outer.len());
        for r in [0.15, 0.2 - 5.0 * t().get().eps, 0.2 + 5.0 * t().get().eps, 0.201] {
            report(&body, &outer, r, "diamond");
        }
    }
}

/// Claim 1 beside a CONCAVE band: the cavity's twelve concave edges,
/// with a diamond vent through the cavity ceiling whose vertex sits 0.2
/// from the y = 1 wall at x = 2.125 (between the screen's stations).
/// Filleting a cavity of side 2 at r adds 8 - rounded(2 - 2r, r).
#[test]
fn p1_concave_cavity_diamond() {
    use crate::common::cavity::{brick, cavity_edges, cut};
    use geom_core::Point3;
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let (vx, vy, h) = (2.125, 1.2, 0.3);
    let vent = at(prism(vec![v(vx, vy, 0.0), v(vx + h, vy + h, 0.0), v(vx, vy + 2.0 * h, 0.0), v(vx - h, vy + h, 0.0)], 2.5, tol()), 0.0, 0.0, 2.5);
    let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let body = cut("cavity", &cut("vent", &block, &vent), &cavity);
    validate_geometric(&body, tol()).expect("fixture");
    let edges = cavity_edges(&body);
    let rb = |l: f64, r: f64| l.powi(3) + 6.0 * l * l * r + 3.0 * PI * l * r * r + (4.0 / 3.0) * PI * r.powi(3);
    for r in [0.1, 0.199, 0.2 - 5.0 * tol().get().eps, 0.2 + 5.0 * tol().get().eps, 0.201, 0.22] {
        let want = vol(&body) + 8.0 - rb(2.0 - 2.0 * r, r);
        println!("P1concave n={} r={r} closed={:+.4} want={want:.12}: {}", edges.len(), 0.2 - r, short(&attempt(&body, &edges, r)));
    }
}

/// The PR's lens fixture, printed (for a before/after on main).
#[test]
fn p2_lens_outer_edges() {
    for (first, second) in [((0.5, 0.5), (0.45, 0.35)), ((0.45, 0.35), (0.5, 0.5))] {
        let body = sub(&sub(&cube(1.0, tol()), &bore(first.0, first.1, 0.1, 0.8, 0.5)), &bore(second.0, second.1, 0.1, 0.7, 0.5));
        let outer = outer_box_edges(&body);
        for r in [0.249, 0.252] {
            let want = vol(&body) - (1.0 - rounded_cube(r));
            println!("P2lens first={first:?} r={r} want={want:.12}: {}", short(&attempt(&body, &outer, r)));
        }
    }
}

/// A hole rim beside a far-away ELLIPTICAL ring (a tilted bore): the
/// support-boundary walk the ring's edges join meters an ellipse by its
/// certified box, but `ring_read` refuses it first.
#[test]
fn p_ellipse_ring_beside_rim() {
    let hole = bore(0.3, 0.5, 0.15, -0.2, 1.4);
    let tilted = at(crate::common::tilted_bore(), 0.3, 0.0, 0.0);
    let body = sub(&sub(&cube(1.0, tol()), &hole), &tilted);
    validate_geometric(&body, tol()).expect("fixture");
    let rim = rim_at(&body, 1.0, 0.15);
    for r in [0.05, 0.1] {
        println!("Pellipse n={} r={r}: {}", rim.len(), short(&attempt_rim(&body, &rim, r, 0.15)));
    }
}
