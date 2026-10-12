//! Review probes for PR 4765: curved operands at a pinch, the partial
//! revolve's axis vertices, the split's curved graze. Printing probes.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use crate::revolve_common::{axis_y, validated};
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::test_support::{brick, prism_on, sketch_from_axes};
use sweep::{Revolution, revolve};
use topo::test_support::finished;
use topo::{AtRestBody, BooleanResult, DecisionSite};

fn tol() -> Tol {
    Tol::witness()
}

fn unit(v: [f64; 3]) -> [f64; 3] {
    let n = v.iter().map(|c| c * c).sum::<f64>().sqrt();
    v.map(|c| c / n)
}

fn shape(r: &topo::BooleanBody<f64>) -> (usize, usize, usize, bool) {
    let mut by_point: std::collections::BTreeMap<[u64; 3], Vec<_>> = Default::default();
    for (k, p) in r.body.vertex_points() {
        by_point
            .entry([p.x.to_bits(), p.y.to_bits(), p.z.to_bits()])
            .or_default()
            .push(r.body.get_vertex(k).unwrap().point);
    }
    let same = by_point
        .values()
        .filter(|k| {
            let mut d = (*k).clone();
            d.sort();
            d.dedup();
            d.len() < k.len()
        })
        .count();
    let vf = r.coincidences.iter().filter(|c| c.site == DecisionSite::VertexFusion).count();
    (same, r.contacts.rows().len(), vf, topo::validate_pseudomanifold(&r.body, &r.contacts, tol()).is_ok())
}

/// A radius-`rad` cylinder of length 8 whose side passes through `v`
/// with outward normal −m there, its axis along the frame's `u`.
fn cylinder_beyond(v: [f64; 3], m: [f64; 3], rad: f64) -> AtRestBody<f64> {
    let m = unit(m);
    let u = unit([m[1], -m[0], 0.0]);
    let w = [m[1] * u[2] - m[2] * u[1], m[2] * u[0] - m[0] * u[2], m[0] * u[1] - m[1] * u[0]];
    let c = [0, 1, 2].map(|i| v[i] + rad * m[i] - 4.0 * u[i]);
    let plane = sketch_from_axes(
        Point3::new(c[0], c[1], c[2]),
        Vec3::new(w[0], w[1], w[2]),
        Vec3::new(m[0], m[1], m[2]),
        tol(),
    );
    let disc = vec![(Point2::new(rad, 0.0), 1.0), (Point2::new(-rad, 0.0), 1.0)];
    finished("cylinder", prism_on(plane, disc, 8.0, tol()), tol())
}

#[test]
fn probe_curved_pinch() {
    let ell = topo::test_support::prism::<f64>(
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
        1.0,
        tol(),
    );
    let ell = finished("ell", ell.body, tol());
    for m in [[1.0, 1.3, 0.7], [1.0, 1.3, 0.4], [1.2, 1.0, 0.3]] {
        for rad in [3.0, 6.0] {
            let cyl = cylinder_beyond([1.0, 1.0, 1.0], m, rad);
            for (order, p, q) in [("ell-cyl", &ell, &cyl), ("cyl-ell", &cyl, &ell)] {
                for (op, run) in [
                    ("intersect", topo::intersect::<f64> as fn(_, _, _) -> _),
                    ("union", topo::union::<f64>),
                    ("subtract", topo::subtract::<f64>),
                ] {
                    match run(p, q, tol()) {
                        Ok(BooleanResult::Body(r)) => {
                            let s = shape(&r);
                            println!("CURVED {m:?} r{rad} {order} {op}: same-key {} contacts {} vf {} t3 {}", s.0, s.1, s.2, s.3);
                            if !s.3 {
                                println!("    t3 err {:?}", topo::validate_pseudomanifold(&r.body, &r.contacts, tol()).err().map(|e| e.into_iter().take(2).collect::<Vec<_>>()));
                                println!("    rows {:?}", r.coincidences.iter().map(|c| (c.site, c.cells)).collect::<Vec<_>>());
                            }
                        }
                        other => println!("CURVED {m:?} r{rad} {order} {op}: {other:?}"),
                    }
                }
            }
        }
    }
}

/// Partial revolve of a profile touching the axis at two isolated
/// vertices and along one segment.
#[test]
fn probe_partial_revolve_axis() {
    let lp = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 3.0),
        Point2::new(0.0, 3.0),
        Point2::new(0.0, 2.0),
        Point2::new(0.5, 1.5),
        Point2::new(0.0, 1.0),
        Point2::new(0.5, 0.5),
    ]);
    let vp = validated(vec![lp]);
    println!("REVOLVE full: {:?}", revolve(&vp, axis_y(), Revolution::Full, tol()).err());
    for a in [1.0, 3.0, 6.0] {
        match revolve(&vp, axis_y(), Revolution::Partial(a), tol()) {
            Ok(t) => {
                let at = |y: f64| {
                    t.body
                        .vertex_points()
                        .filter(|(_, p)| p.x.abs() + (p.y - y).abs() + p.z.abs() < 1e-12)
                        .count()
                };
                println!(
                    "REVOLVE partial {a}: at y=0 {} y=1 {} y=2 {} y=3 {} t3 {:?}",
                    at(0.0),
                    at(1.0),
                    at(2.0),
                    at(3.0),
                    topo::validate_pseudomanifold(&t.body, &topo::ContactRecords::default(), tol())
                );
            }
            Err(e) => println!("REVOLVE partial {a}: {e:?}"),
        }
    }
}

/// The split grazing a curved edge: a rod's rim circle touched by a
/// plane (convex), and a bore's rim in a block touched (concave).
#[test]
fn probe_split_curved_graze() {
    let rad = 1.0;
    let plane = sketch_from_axes(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), tol());
    let disc = vec![(Point2::new(rad, 0.0), 1.0), (Point2::new(-rad, 0.0), 1.0)];
    let rod = finished("rod", prism_on(plane, disc.clone(), 2.0, tol()), tol());
    let block = finished("block", brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.0), tol()), tol());
    let bored = match topo::subtract(&block, &rod, tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{other:?}"),
    };
    // planes tangent to the rim at (1, 0, 0) but oblique to the side
    // (so only the rim circle is grazed, at one point).
    for (name, body) in [("rod", &rod), ("bored", &bored)] {
        for n in [[1.0, 0.0, 0.3], [1.0, 0.0, -0.3], [1.0, 0.0, 0.0]] {
            let n = unit(n);
            let r = topo::split(
                body,
                &topo::test_support::split_plane(Point3::new(1.0, 0.0, 0.0), Vec3::new(n[0], n[1], n[2]), tol()),
                tol(),
            );
            match r {
                Ok(s) => println!(
                    "SPLIT {name} {n:?}: coincidences {:?}",
                    s.coincidences.iter().map(|c| (c.site, c.relation)).collect::<Vec<_>>()
                ),
                Err(e) => println!("SPLIT {name} {n:?}: {e:?}"),
            }
        }
    }
}
