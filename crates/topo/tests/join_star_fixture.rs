//! **The star fixture folds in every member order.** Three bricks in a
//! row along x — `a` = x∈(0,1), `c` = x∈(0.5,1.5), `d` = x∈(1.2,2.2) —
//! and a fourth, `f` = x∈(0.5,1.5) × y∈(0.5,1.5), standing across the
//! row's y = 1 wall; all z∈(0,1), every flush pair declared at each
//! step. Folding two overlapping bricks merges their caps and walls but
//! keeps every boundary vertex, so the bar carries valence-2 vertices
//! on its y = 1 rims (x = 1.2 and 1.5 for `c ∪ d`, x = 0.5 and 1 for
//! `a ∪ c`). When `f` joins, its bottom and top caps cover such a
//! vertex, coplanar with the bar's caps: the vertex's only In bound is
//! the wall sector's bisector, every real edge reads Out, and the
//! pierce insertion mints a strut. Each order builds the same
//! eight-sided prism and certifies.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, flush_declarations};
use geom_core::Tol;
use topo::validate::{validate_closed, validate_geometric};
use topo::{Body, BooleanResult, mass_properties, union_with, validate_pseudomanifold};

fn member(name: char) -> Body<f64> {
    let tol = Tol::witness();
    match name {
        'a' => brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
        'c' => brick::<f64>((0.5, 1.5), (0.0, 1.0), (0.0, 1.0), tol),
        'd' => brick::<f64>((1.2, 2.2), (0.0, 1.0), (0.0, 1.0), tol),
        'f' => brick::<f64>((0.5, 1.5), (0.5, 1.5), (0.0, 1.0), tol),
        _ => unreachable!("the star has four members"),
    }
}

/// Every permutation of the four members, in lexicographic order.
fn orders() -> Vec<[char; 4]> {
    let names = ['a', 'c', 'd', 'f'];
    let mut out = Vec::new();
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                for l in 0..4 {
                    let idx = [i, j, k, l];
                    if (0..4).all(|m| idx.contains(&m)) {
                        out.push(idx.map(|m| names[m]));
                    }
                }
            }
        }
    }
    out
}

/// All 24 orders fold to the bar-and-stub prism: every step merges
/// what it was licensed to merge, the finished body is two caps and
/// eight walls, it validates at tiers 2, 3 and 3′, and its volume is
/// the bar's 2.2 plus the stub's 0.5.
#[test]
fn the_star_fixture_folds_in_every_member_order() {
    let tol = Tol::witness();
    let all = orders();
    assert_eq!(all.len(), 24);
    for order in all {
        let mut acc = member(order[0]);
        let mut contacts = None;
        for (step, &name) in order.iter().enumerate().skip(1) {
            let next = member(name);
            let decls = flush_declarations(&acc, &next, tol);
            let r = union_with(&acc, &next, &decls, tol)
                .unwrap_or_else(|e| panic!("{order:?} step {step} refused: {e:?}"));
            let BooleanResult::Body(bb) = r else {
                panic!("{order:?} step {step}: an overlapping union cannot be Empty");
            };
            assert!(
                bb.naming.merge_skipped.is_empty(),
                "{order:?} step {step}: {:?}",
                bb.naming.merge_skipped
            );
            acc = bb.body;
            contacts = Some(bb.contacts);
        }
        let contacts = contacts.unwrap();
        assert_eq!(validate_closed(&acc), Ok(()), "{order:?}: tier 2");
        assert_eq!(validate_geometric(&acc, tol), Ok(()), "{order:?}: tier 3");
        assert_eq!(
            validate_pseudomanifold(&acc, &contacts, tol),
            Ok(()),
            "{order:?}: tier 3′"
        );
        assert_eq!(acc.faces().count(), 10, "{order:?}: two caps, eight walls");
        // 1.2 and 2.2 are not dyadic, so 2.7 is the nearest double to
        // the exact volume; the divergence sum lands within 4 ulps of
        // it (an ulp at 2.7 is 2·EPSILON).
        let volume = mass_properties(&acc, tol).unwrap().volume;
        assert!(
            (volume - 2.7).abs() <= 8.0 * f64::EPSILON,
            "{order:?}: volume {volume}"
        );
    }
}
