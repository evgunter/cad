//! **The collinear-seam differential** — the positive pole of the
//! trigger the reviewers' attack fixtures are the negative pole of.
//!
//! R1's P2 and R2's mid-vertex probes both place their interior vertex
//! deliberately OFF the chord ("not on segment V0–V2, so the chain is
//! a genuine bent cut, not a degenerate straight edge split"). That
//! parenthesis names exactly the shape this file builds: the same
//! subdivided chord with the vertex ON the segment. It is the licence
//! for the repair — a vertex interior to one straight carrier — and
//! the reason the two arms' fixtures stay refusals.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::prism_z;
use geom_core::{Point3, Tol};
use topo::{MefSite, MevSite, validate_closed};

/// A prism whose top face is split by a chord from `(0,0)` to `(2,2)`
/// carrying an interior vertex; `on_segment` places that vertex on the
/// chord (collinear) or off it (bent).
fn split_top(on_segment: bool) -> topo::Body<f64> {
    let p = prism_z::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)], 0.0, 1.0);
    let mut b = p.body;
    let tol = Tol::witness();
    let he_at = |b: &topo::Body<f64>, x: f64, y: f64| {
        let outer = b.get_face(p.top_face).unwrap().outer;
        let topo::LoopBoundary::Cycle { first } = b.get_loop(outer).unwrap().boundary else {
            panic!("top face is a cycle")
        };
        b.loop_cycle(first)
            .unwrap()
            .into_iter()
            .find(|&he| {
                let v = b.get_half_edge(he).unwrap().start;
                let pt = b.get_point(b.get_vertex(v).unwrap().point).unwrap();
                (pt.x - x).abs() < 1e-12 && (pt.y - y).abs() < 1e-12
            })
            .expect("a half-edge starting there")
    };
    let he1 = he_at(&b, 0.0, 0.0);
    let mid = if on_segment {
        Point3::new(1.0, 1.0, 1.0) // ON the (0,0)–(2,2) diagonal
    } else {
        Point3::new(0.9, 0.6, 1.0) // R1's P2 point, off it
    };
    let strut = b
        .mev_line(MevSite::Fan { he1, he2: he1 }, mid, tol)
        .unwrap();
    let he2 = he_at(&b, 2.0, 2.0);
    b.mef_chord(
        MefSite::Chords {
            he1: strut.he_minus,
            he2,
        },
        tol,
    )
    .unwrap();
    assert_eq!(validate_closed(&b), Ok(()), "fixture is tier-2 legal");
    b
}

/// The DIFFERENTIAL, one screen tall: the same construction twice,
/// differing only in whether the interior vertex sits on the chord.
///
/// Printed rather than asserted on the trigger itself (the trigger is
/// private); what this row pins is that the two bodies are both legal
/// and structurally identical apart from that one coordinate, so any
/// difference in how the merge treats them is the collinearity and
/// nothing else.
#[test]
fn collinear_and_bent_seams_are_one_coordinate_apart() {
    for on_segment in [true, false] {
        let mut b = split_top(on_segment);
        let planar_pairs = b
            .edges()
            .filter(|(_, e)| {
                let f = |he| {
                    let l = b.get_half_edge(he)?.parent_loop;
                    Some(b.get_loop(l)?.face)
                };
                match (f(e.he_plus), f(e.he_minus)) {
                    (Some(a), Some(c)) => {
                        a != c
                            && b.get_face(a).map(|x| x.surface) == b.get_face(c).map(|x| x.surface)
                    }
                    _ => false,
                }
            })
            .count();
        let merged = b.merge_coplanar_faces(Tol::witness());
        println!(
            "F7SEAM on_segment={on_segment} shared_same_key_edges={planar_pairs} \
             merge={:?}",
            merged.as_ref().map(|o| o.groups.len())
        );
    }
}
