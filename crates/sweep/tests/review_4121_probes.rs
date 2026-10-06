//! Review probes for PR 4121 (band-dual-4121-r1): the planar cut-off's
//! geometry, its meters, and the turn classifier, on fixtures the PR's
//! own rows do not build. Each prints what it measured.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code, missing_docs)]

use core::f64::consts::PI;

use geom_core::{Point2, Point3, Tol, Vec3};
use sweep::blend::BlendError;
use sweep::blend::build::{Blended, fillet_edges};
use sweep::chamfer::chamfer_edges;
use sweep::test_support::{block, pocket_die, prism, prism_on, realized, sketch_from_axes};
use topo::{Body, EdgeKey, FaceKey, mass_properties, query, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn volume(body: &Body<f64>) -> f64 {
    mass_properties(body, tol()).expect("props").volume
}

fn pt(body: &Body<f64>, v: topo::VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

fn ends(body: &Body<f64>, e: EdgeKey) -> (Point3<f64>, Point3<f64>) {
    let he = body.get_edge(e).unwrap().he_plus;
    let s = body.get_half_edge(he).unwrap().start;
    let t = body.half_edge_end(he).unwrap();
    (pt(body, s), pt(body, t))
}

fn edge(body: &Body<f64>, a: [f64; 3], b: [f64; 3]) -> EdgeKey {
    let (a, b) = (Point3::new(a[0], a[1], a[2]), Point3::new(b[0], b[1], b[2]));
    let at = |p: Point3<f64>, q: Point3<f64>| (p - q).norm() < 1e-9;
    query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let (p, q) = ends(body, e);
            (at(p, a) && at(q, b)) || (at(p, b) && at(q, a))
        })
        .unwrap_or_else(|| panic!("no edge {a:?}-{b:?}"))
}

fn outer_vertices(b: &Body<f64>, f: FaceKey) -> Option<Vec<topo::VertexKey>> {
    let lp = b.get_face(f)?.outer;
    let topo::LoopBoundary::Cycle { first } = b.get_loop(lp)?.boundary else { return None };
    Some(b.loop_cycle(first)?.into_iter().map(|h| b.get_half_edge(h).unwrap().start).collect())
}

#[derive(Clone, Copy, Debug)]
enum Verb {
    Chamfer,
    Fillet,
}

fn run(verb: Verb, body: &Body<f64>, edges: &[EdgeKey], d: f64) -> Result<Blended<f64>, BlendError> {
    match verb {
        Verb::Chamfer => chamfer_edges(body, edges, d, tol()),
        Verb::Fillet => fillet_edges(body, edges, d, tol()),
    }
    .map_err(|r| r.error)
}

/// Build, and report tier 3, Euler, and ΔV against `expect` (None: print only).
fn report(what: &str, verb: Verb, body: &Body<f64>, edges: &[EdgeKey], d: f64, expect: Option<f64>) -> Option<Blended<f64>> {
    match run(verb, body, edges, d) {
        Ok(out) => {
            let t3 = validate_geometric(&out.body, tol());
            let c = topo::readback::euler_counts(&out.body);
            let dv = volume(body) - volume(&out.body);
            let verdict = match expect {
                Some(x) => format!("expect {x:.15e} err {:.3e}", (dv - x).abs()),
                None => String::new(),
            };
            println!(
                "PROBE {what} {verb:?}: BUILT tier3={} shells={} genus={:?} dV={dv:.15e} {verdict}",
                if t3.is_ok() { "ok".to_string() } else { format!("{t3:?}") },
                c.s,
                c.genus()
            );
            Some(out)
        }
        Err(e) => {
            println!("PROBE {what} {verb:?}: REFUSED {e}");
            None
        }
    }
}

/// Oblique end faces at several slopes, leaning along the support
/// (trapezoid in xy) — inward and outward. The band's region is a
/// prism of section A cut by two planes; ΔV = A·L(centroid).
#[test]
fn probe_oblique_end_faces_at_several_slopes() {
    let d = 0.1;
    for s in [0.05, 0.3, 0.9, -0.3, -1.0, -3.0] {
        // trapezoid: (0,0),(2,0),(2-s,1),(s,1). L(y) = 2 - 2 s y.
        let body = prism(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(2.0, 0.0), 0.0),
                (Point2::new(2.0 - s, 1.0), 0.0),
                (Point2::new(s, 1.0), 0.0),
            ],
            1.0,
            tol(),
        );
        let e = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
        // section triangle on the front wall y=0 and the top z=1:
        // centroid y = d/3.
        let expect = d * d / 2.0 * (2.0 - 2.0 * s * d / 3.0);
        report(&format!("trapezoid-top s={s}"), Verb::Chamfer, &body, &[e], d, Some(expect));
        report(&format!("trapezoid-top s={s}"), Verb::Fillet, &body, &[e], d, None);
        // the bottom front edge too: same end planes (vertical), y-centroid d/3.
        let e = edge(&body, [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
        report(&format!("trapezoid-bottom s={s}"), Verb::Chamfer, &body, &[e], d, Some(expect));
        // the back top edge (y = 1): length 2 - 2s, end planes lean the
        // other way: L(y) = 2 - 2 s y with centroid y = 1 - d/3.
        let e = edge(&body, [s, 1.0, 1.0], [2.0 - s, 1.0, 1.0]);
        let expect = d * d / 2.0 * (2.0 - 2.0 * s * (1.0 - d / 3.0));
        report(&format!("trapezoid-back s={s}"), Verb::Chamfer, &body, &[e], d, Some(expect));
    }
    // End faces leaning in BOTH transverse directions: a trapezoid in
    // xz, extruded along y, then the edge along x on the bottom front.
    // plus a trapezoid lean in y via a second cut is out of reach of
    // `prism`; the xz family covers the other transverse lean.
    for s in [0.3, -0.5] {
        let plane = sketch_from_axes(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            tol(),
        );
        let body = prism_on(
            plane,
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(2.0, 0.0), 0.0),
                (Point2::new(2.0 - s, 1.0), 0.0),
                (Point2::new(s, 1.0), 0.0),
            ],
            1.5,
            tol(),
        );
        // the extrusion runs along -y (x × z); find the x-edge at z = 0
        // on whichever y-face is at y = 0.
        let es: Vec<EdgeKey> = query::all_edges(&body)
            .into_iter()
            .filter(|&e| {
                let (p, q) = ends(&body, e);
                p.z.abs() < 1e-12 && q.z.abs() < 1e-12 && p.y.abs() < 1e-12 && q.y.abs() < 1e-12
            })
            .collect();
        assert_eq!(es.len(), 1, "one bottom x edge at y = 0");
        let expect = 0.1 * 0.1 / 2.0 * (2.0 - 2.0 * s * 0.1 / 3.0);
        report(&format!("xz-trapezoid s={s}"), Verb::Chamfer, &body, &es, 0.1, Some(expect));
    }
}

/// Non-right dihedral, perpendicular ends: the triangular prism's
/// vertical edges.
#[test]
fn probe_non_right_dihedrals() {
    let body = prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(0.7, 1.3), 0.0),
        ],
        1.0,
        tol(),
    );
    let e = edge(&body, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let alpha = 1.3_f64.atan2(0.7);
    let r = 0.1;
    let fillet = r * r * (1.0 / (alpha / 2.0).tan() - (PI - alpha) / 2.0);
    report("tri-vertical", Verb::Fillet, &body, &[e], r, Some(fillet));
    let c = report("tri-vertical", Verb::Chamfer, &body, &[e], r, None);
    if let Some(c) = c {
        let dv = volume(&body) - volume(&c.body);
        println!(
            "PROBE tri-vertical chamfer candidates: setback-along-faces {:.15e}, leg d {:.15e}, measured {dv:.15e}",
            r * r * alpha.sin() / 2.0,
            r * r * alpha.sin() / 2.0
        );
    }
    // A sloped top edge: (0,0)-(2,0) top at z = 1, dihedral between the
    // front wall y=0 and top is 90° — and the hypotenuse edge's top
    // edge with oblique ends, chamfer only.
    let e = edge(&body, [2.0, 0.0, 1.0], [0.7, 1.3, 1.0]);
    report("tri-hyp-top", Verb::Chamfer, &body, &[e], r, None);
    report("tri-hyp-top", Verb::Fillet, &body, &[e], r, None);
}

/// Arm (d): a support boundary edge that crosses the strip must refuse;
/// one that stays clear must build.
#[test]
fn probe_strip_meter() {
    let d = 0.5;
    // A straight notch reaching y = 0.3 < d into the top face.
    let notch = |bottom: f64| {
        prism(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(10.0, 0.0), 0.0),
                (Point2::new(10.0, 5.0), 0.0),
                (Point2::new(5.0, 5.0), 0.0),
                (Point2::new(5.0, bottom), 0.0),
                (Point2::new(4.0, bottom), 0.0),
                (Point2::new(4.0, 5.0), 0.0),
                (Point2::new(0.0, 5.0), 0.0),
            ],
            1.0,
            tol(),
        )
    };
    for (bottom, should) in [(0.3, "refuse"), (0.6, "build")] {
        let body = notch(bottom);
        let e = edge(&body, [0.0, 0.0, 1.0], [10.0, 0.0, 1.0]);
        for verb in [Verb::Chamfer, Verb::Fillet] {
            let sec = match verb {
                Verb::Chamfer => d * d / 2.0,
                Verb::Fillet => (1.0 - PI / 4.0) * d * d,
            };
            report(&format!("notch bottom={bottom} (should {should})"), verb, &body, &[e], d, Some(sec * 10.0));
        }
    }
    // An arc that dips into the strip between two endpoints outside it.
    for bulge in [0.6, -0.6] {
        let body = prism(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(10.0, 0.0), 0.0),
                (Point2::new(10.0, 5.0), 0.0),
                (Point2::new(6.0, 5.0), 0.0),
                (Point2::new(6.0, 1.0), bulge),
                (Point2::new(4.0, 1.0), 0.0),
                (Point2::new(4.0, 5.0), 0.0),
                (Point2::new(0.0, 5.0), 0.0),
            ],
            1.0,
            tol(),
        );
        let e = edge(&body, [0.0, 0.0, 1.0], [10.0, 0.0, 1.0]);
        // which way the arc bows: lowest y on the body
        let low = query::all_edges(&body)
            .into_iter()
            .filter_map(|e| {
                let (p, q) = ends(&body, e);
                ((p.x - 6.0).abs() < 1e-9 && (q.x - 4.0).abs() < 1e-9 || (p.x - 4.0).abs() < 1e-9 && (q.x - 6.0).abs() < 1e-9)
                    .then_some(e)
            })
            .count();
        println!("PROBE arc bulge={bulge}: arcs found {low}");
        report(&format!("arc-dip bulge={bulge}"), Verb::Chamfer, &body, &[e], d, Some(d * d / 2.0 * 10.0));
    }
}

/// The concave side: is the end face's sliver really gained? Measure
/// the end face's area before and after, and put a hole in the end
/// wall inside the sliver triangle — nothing meters it on the concave
/// side.
#[test]
fn probe_concave_sliver() {
    let body = pocket_die(0.0, 0.0, 0.0, tol());
    let e = edge(&body, [0.25, 0.25, 0.5], [0.75, 0.25, 0.5]);
    let out = report("pocket floor edge", Verb::Chamfer, &body, &[e], 0.1, Some(-0.1 * 0.1 / 2.0 * 0.5));
    let wall_area = |b: &Body<f64>| -> Vec<(FaceKey, f64)> {
        // planar faces on x = 0.25: polygon area in (y, z) from vertices of the outer cycle
        query::all_faces(b)
            .into_iter()
            .filter_map(|f| {
                let vs = outer_vertices(b, f)?;
                let ps: Vec<Point3<f64>> = vs.iter().map(|v| pt(b, *v)).collect();
                if !ps.iter().all(|p| (p.x - 0.25).abs() < 1e-12) {
                    return None;
                }
                let mut a = 0.0;
                for i in 0..ps.len() {
                    let (p, q) = (ps[i], ps[(i + 1) % ps.len()]);
                    a += p.y * q.z - q.y * p.z;
                }
                Some((f, (a / 2.0).abs()))
            })
            .collect()
    };
    println!("PROBE concave end wall area before {:?}", wall_area(&body));
    if let Some(out) = out {
        println!("PROBE concave end wall area after {:?}", wall_area(&out.body));
    }
}

/// Overlapping strips on one support, and coincident feet on a shared rim.
#[test]
fn probe_thin_plate() {
    for (w, d) in [(0.15, 0.1), (0.2, 0.1), (0.25, 0.1)] {
        let body = block(2.0, w, 1.0, tol());
        let a = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
        let b = edge(&body, [0.0, w, 1.0], [2.0, w, 1.0]);
        for verb in [Verb::Chamfer, Verb::Fillet] {
            let sec = match verb {
                Verb::Chamfer => d * d / 2.0,
                Verb::Fillet => (1.0 - PI / 4.0) * d * d,
            };
            report(&format!("thin plate w={w} d={d}"), verb, &body, &[a, b], d, Some(sec * 4.0));
        }
    }
}

/// Two opposite corners' three edges each: two patches, six cut-offs;
/// and the four vertical edges (the top a shared end face of four).
#[test]
fn probe_more_subsets() {
    let body = block(2.0, 1.5, 1.0, tol());
    let d = 0.1;
    let es = [
        edge(&body, [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]),
        edge(&body, [0.0, 0.0, 0.0], [0.0, 1.5, 0.0]),
        edge(&body, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        edge(&body, [2.0, 1.5, 1.0], [0.0, 1.5, 1.0]),
        edge(&body, [2.0, 1.5, 1.0], [2.0, 0.0, 1.0]),
        edge(&body, [2.0, 1.5, 1.0], [2.0, 1.5, 0.0]),
    ];
    for (verb, sec, corner) in [
        (Verb::Chamfer, d * d / 2.0, 2.0 / 3.0 * d * d * d),
        (Verb::Fillet, (1.0 - PI / 4.0) * d * d, (2.0 - 7.0 * PI / 12.0) * d * d * d),
    ] {
        report("two opposite corners", verb, &body, &es, d, Some(sec * 4.5 * 2.0 - 2.0 * corner));
        let vert: Vec<EdgeKey> = [[0.0, 0.0], [2.0, 0.0], [2.0, 1.5], [0.0, 1.5]]
            .iter()
            .map(|[x, y]| edge(&body, [*x, *y, 0.0], [*x, *y, 1.0]))
            .collect();
        report("four verticals", verb, &body, &vert, d, Some(sec * 4.0));
        // a patch corner plus a parallel edge sharing its far end face
        let mut five = es[..3].to_vec();
        five.push(edge(&body, [0.0, 1.5, 1.0], [2.0, 1.5, 1.0]));
        report("corner + parallel", verb, &body, &five, d, Some(sec * 6.5 - corner));
    }
}

fn pin(origin: Point3<f64>, r: f64, h: f64, bulge: f64) -> Body<f64> {
    let plane = sketch_from_axes(origin, Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0), tol());
    prism_on(plane, vec![(Point2::new(-r, 0.0), bulge), (Point2::new(r, 0.0), bulge)], h, tol())
}

/// A hole in the end face inside the sliver triangle: the convex side
/// meters it (arm c); the concave side skips the meter.
#[test]
fn probe_hole_in_the_sliver() {
    let d = 0.1;
    // convex control: a blind hole in the box's x = 0 face near V = (0,0,1)
    let b = block(2.0, 1.5, 1.0, tol());
    let holed = realized(topo::boolean::BooleanOp::Subtract, &b, &pin(Point3::new(-0.1, 0.03, 0.97), 0.01, 0.4, 1.0), tol());
    println!("PROBE convex holed: faces {}", query::all_faces(&holed).len());
    let e = edge(&holed, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        report("convex end face, hole inside the sliver", verb, &holed, &[e], d, None);
    }
    // concave: a through hole in the pocket's end wall x = 0.25 near
    // V = (0.25, 0.25, 0.5), piercing into the pocket.
    let die = pocket_die(0.0, 0.0, 0.0, tol());
    let holed = realized(topo::boolean::BooleanOp::Subtract, &die, &pin(Point3::new(-0.1, 0.28, 0.53), 0.01, 0.45, 1.0), tol());
    println!("PROBE concave holed: faces {} input tier3 {:?}", query::all_faces(&holed).len(), validate_geometric(&holed, tol()));
    let e = edge(&holed, [0.25, 0.25, 0.5], [0.75, 0.25, 0.5]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let out = report("concave end face, hole inside the sliver", verb, &holed, &[e], d, None);
        if let Some(out) = out {
            // where is the hole's ring now, and is it inside its face?
            let closed = topo::validate_closed(&out.body);
            println!("PROBE   validate_closed: {:?}", closed.err());
            for f in query::all_faces(&out.body) {
                let fd = out.body.get_face(f).unwrap();
                if !fd.rings.is_empty() {
                    let s = out.body.get_surface(fd.surface).unwrap();
                    println!("PROBE   face {f:?} rings {} surface {:?}", fd.rings.len(), s);
                }
            }
        }
    }
}
