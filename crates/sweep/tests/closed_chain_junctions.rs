//! **A closed chain's junctions are judged against the links that
//! touch them.** Phase 1 probes: the pairing as data, and what the
//! closed-rim doors say past the check.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::approx::band;
use geom_core::Tol;
use sweep::blend::battery::{BlendRequest, run_battery};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{bored_block_of_arcs, cube, disc_of_arcs};
use topo::{Body, EdgeKey, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

/// The arcs whose stored carrier is a circle centred at station `z`
/// — the raw scan, seeded by no rim door.
fn arcs_at_z(body: &Body<f64>, z: f64) -> Vec<EdgeKey> {
    body.edges()
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            match c.carrier() {
                geom::Curve3::Circle { center, .. } if (center.z - z).abs() < 1e-9 => Some(k),
                _ => None,
            }
        })
        .collect()
}

fn describe_arcs(body: &Body<f64>, edges: &[EdgeKey]) {
    for &k in edges {
        let e = body.get_edge(k).unwrap();
        let sv = |he| body.get_half_edge(he).unwrap().start;
        let sf = |he| {
            let l = body.get_half_edge(he).unwrap().parent_loop;
            body.get_face(body.get_loop(l).unwrap().face).unwrap().surface
        };
        let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
        eprintln!(
            "  edge {k:?} start={:?} end={:?} surfaces=({:?},{:?}) params={:?} carrier={:?}",
            sv(e.he_plus), sv(e.he_minus), sf(e.he_plus), sf(e.he_minus), c.params(), c.carrier()
        );
    }
}

fn census(b: &Body<f64>) -> (usize, usize, usize) {
    (b.vertices().count(), b.edges().count(), b.faces().count())
}

/// Three consecutive edges of the cube's top face: an OPEN three-link
/// chain with two junctions (the top face's other edge is not
/// requested, so its two ends are free ends).
fn open_three_link_chain(body: &Body<f64>) -> Vec<EdgeKey> {
    let top: Vec<EdgeKey> = body
        .edges()
        .filter(|(_, e)| {
            let ends = [e.he_plus, e.he_minus].map(|he| {
                let h = body.get_half_edge(he).unwrap();
                body.get_point(body.get_vertex(h.start).unwrap().point)
                    .unwrap()
                    .z
            });
            ends.iter().all(|z| (*z - 1.0).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(top.len(), 4, "the top face has four edges");
    // Drop one edge; the remaining three are consecutive on a 4-cycle.
    top[..3].to_vec()
}

#[test]
fn phase1_pairing_dump() {
    for (name, body, arcs) in [
        ("three-arc rim", disc_of_arcs(3, 0.5, 1.0, tol()), None),
        ("four-arc rim", disc_of_arcs(4, 0.5, 1.0, tol()), None),
        ("two-arc rim", disc_of_arcs(2, 0.5, 1.0, tol()), None),
        ("open three-link chain", cube(1.0, tol()), Some(())),
    ] {
        let edges = match arcs {
            None => arcs_at_z(&body, 1.0),
            Some(()) => open_three_link_chain(&body),
        };
        eprintln!("=== {name}: edges={edges:?}");
        describe_arcs(&body, &edges);
        eprintln!("  rim_of: {:?}", topo::query::rim_of(&body, edges[0]).map_err(|e| e.to_string()));
        let r = run_battery(
            &BlendRequest {
                body: &body,
                edges,
                size: 0.05,
            },
            band(),
        );
        eprintln!("  battery: {:?}", r.as_ref().map(|_| ()).map_err(|e| e.to_string()));
    }
}

#[test]
fn phase1_door_dump() {
    for (name, body) in [
        ("three-arc disc", disc_of_arcs(3, 0.5, 1.0, tol())),
        ("four-arc disc", disc_of_arcs(4, 0.5, 1.0, tol())),
        ("two-arc disc", disc_of_arcs(2, 0.5, 1.0, tol())),
        ("three-arc bore", bored_block_of_arcs(3, 2.0, 1.0, 0.5, tol())),
        ("four-arc bore", bored_block_of_arcs(4, 2.0, 1.0, 0.5, tol())),
        ("two-arc bore", bored_block_of_arcs(2, 2.0, 1.0, 0.5, tol())),
    ] {
        let arcs = arcs_at_z(&body, 1.0);
        let c0 = census(&body);
        let p0 = mass_properties(&body, tol()).unwrap();
        eprintln!("=== {name}: arcs={} census={c0:?} V0={} pad0={}", arcs.len(), p0.volume, p0.volume_pad);
        match fillet_edges(&body, &arcs, 0.1, tol()) {
            Ok(out) => {
                let c1 = census(&out.body);
                let p1 = mass_properties(&out.body, tol()).unwrap();
                let t3 = validate_geometric(&out.body, tol());
                eprintln!(
                    "  CARVED band={} blend={} corner={} census={c1:?} V1={} pad1={} tier3={:?}",
                    out.band_faces.len(), out.blend_faces.len(), out.corner_faces.len(),
                    p1.volume, p1.volume_pad, t3.as_ref().map(|_| ()).map_err(|e| format!("{e:?}"))
                );
            }
            Err(e) => eprintln!("  REFUSED {e:?}\n  {e}"),
        }
    }
}
