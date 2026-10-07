//! The plane–plane band's cut-off at the CERTIFIED scalar — the
//! `Interval` replay of `band_planar_cut_off`'s box rows: one edge, and
//! the three edges of one corner, each chamfered and filleted. Every row
//! carves, is tier-3 valid with naming total, and its volume enclosure
//! brackets the closed form narrowly enough to be a claim. The fillet's
//! perpendicular end faces are decided `Zero` by `fillet3_cap_transverse`
//! at the certified scalar, read off the funnel's log.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::interval::iv;
use geom_core::k_stats::Bracket;
use geom_core::{Bounds, Interval, Point3, Sign, Tol};
use sweep::blend::build::{chamfer_edges, fillet_edges};
use sweep::test_support::{assert_naming_totality, block};
use topo::{Body, EdgeKey, mass_properties, query, validate_geometric};

const D: f64 = 0.1;

/// The box edge between two points, either way round.
fn edge(body: &Body<Interval>, a: [f64; 3], b: [f64; 3]) -> EdgeKey {
    let near = |p: &Point3<Interval>, q: [f64; 3]| {
        [p.x, p.y, p.z]
            .iter()
            .zip(q)
            .all(|(c, w)| c.lo() <= w + 1e-12 && w - 1e-12 <= c.hi())
    };
    query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let he = body.get_edge(e).unwrap().he_plus;
            let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            let (s, t) = (
                p(body.get_half_edge(he).unwrap().start),
                p(body.half_edge_end(he).unwrap()),
            );
            (near(&s, a) && near(&t, b)) || (near(&s, b) && near(&t, a))
        })
        .expect("a box edge between the two points")
}

#[test]
fn the_box_rows_carve_at_the_certified_scalar_and_bracket_their_closed_forms() {
    let tol = Tol::witness();
    let body = block::<Interval>(2.0, 1.5, 1.0, tol);
    let front = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let up = edge(&body, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let across = edge(&body, [0.0, 0.0, 1.0], [0.0, 1.5, 1.0]);
    let v0 = mass_properties(&body, tol).expect("interval props").volume;
    let chamfer = (D * D / 2.0, 2.0 / 3.0 * D.powi(3));
    let fillet = (
        (1.0 - PI / 4.0) * D * D,
        (2.0 - 7.0 * PI / 12.0) * D.powi(3),
    );
    let rows: [(&str, Vec<EdgeKey>, f64, usize); 2] = [
        ("one edge", vec![front], 2.0, 0),
        ("three edges of one corner", vec![front, up, across], 4.5, 1),
    ];
    for (what, edges, length, corners) in rows {
        for (round, (section, corner)) in [(false, chamfer), (true, fillet)] {
            let bracket = Bracket::open();
            let out = if round {
                fillet_edges(&sweep::test_support::at_rest(&body), &edges, iv(D), tol)
            } else {
                chamfer_edges(&sweep::test_support::at_rest(&body), &edges, iv(D), tol)
            }
            .unwrap_or_else(|e| panic!("{what} (round {round}): carves at Interval, got {e:?}"));
            let ends = bracket
                .finish()
                .verdicts
                .into_iter()
                .filter(|v| v.predicate == "fillet3_cap_transverse")
                .collect::<Vec<_>>();
            if round {
                assert!(
                    !ends.is_empty() && ends.iter().all(|v| v.sign == Sign::Zero),
                    "{what}: every fillet end face is perpendicular at Interval: {ends:?}"
                );
            }
            validate_geometric(&out.body, tol)
                .unwrap_or_else(|e| panic!("{what} (round {round}): tier 3, got {e:?}"));
            assert_naming_totality(&body, &out, &edges, what);
            let p = mass_properties(&out.body, tol).expect("interval props");
            assert_eq!(p.volume_pad, 0.0, "{what}: a closed-form inventory");
            let removed = v0 - p.volume;
            let truth = section * length - corner * corners as f64;
            assert!(
                removed.lo() <= truth + 1e-15 && truth - 1e-15 <= removed.hi(),
                "{what} (round {round}): ΔV {removed:?} brackets {truth}"
            );
            assert!(
                removed.hi() - removed.lo() < 1e-9,
                "{what} (round {round}): the enclosure is a claim: {removed:?}"
            );
        }
    }
}
