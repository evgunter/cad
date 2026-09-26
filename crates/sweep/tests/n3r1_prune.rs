//! **Adopted from a reviewer probe** (the CERT-N3 dual review) as an
//! ordinary row: the pruning-delta corpus. The reviewer ran it against both arms through an env-gated plant in `edge_box`; the plant stays out of the tree, so this row pins the adopted arm's side of the table — the candidate total on the corpus and the superset property wherever the brute-force reference runs — with the base arm's 134 recorded in the landing PR.
//!
//!
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::{
    nested_box, rim_plate, rounded_plate, three_arc_cylinder, top_rim_plate,
};
use geom_core::Tol;
use std::collections::BTreeSet;
use sweep::test_support::brick;
use topo::{Body, BooleanResult, SweepStrategy, SweepTrace, sweep_traces};

/// The corpus cylinder at `(cx, 0)`, `z in [0, 1]`: this suite poses
/// it by translating the PROFILE in `x`.
fn cylinder_at(cx: f64) -> Body<f64> {
    three_arc_cylinder(cx, 0.0, 1.0, 0.0)
}

fn cylinder() -> Body<f64> {
    cylinder_at(0.0)
}

fn corpus() -> Vec<(String, Body<f64>, Body<f64>)> {
    let cyl = cylinder();
    let rounded = rounded_plate();
    let mut v = vec![
        (
            "cylinder x nested box".to_string(),
            cyl.clone(),
            nested_box(-0.3, 0.05),
        ),
        (
            "cylinder x box beside".to_string(),
            cyl.clone(),
            nested_box(3.0, 0.2),
        ),
        (
            "cylinder x crossing box".to_string(),
            cyl.clone(),
            nested_box(0.45, 0.2),
        ),
        (
            "cylinder x plate across x-extreme".to_string(),
            cyl.clone(),
            rim_plate(-0.499),
        ),
        (
            "cylinder x plate at -0.45".to_string(),
            cyl.clone(),
            rim_plate(-0.45),
        ),
        (
            "cylinder x top plate at 0.499".to_string(),
            cyl.clone(),
            top_rim_plate(0.499),
        ),
        (
            "rounded x box clear of round".to_string(),
            rounded.clone(),
            brick((1.2, 1.6), (0.36, 0.6), (0.2, 0.5), Tol::witness()),
        ),
        (
            "rounded x box grazing round".to_string(),
            rounded.clone(),
            brick((1.18, 1.6), (0.33, 0.6), (0.2, 0.5), Tol::witness()),
        ),
        (
            "rounded x corner box".to_string(),
            rounded,
            brick((1.1, 1.6), (0.2, 0.6), (0.2, 0.5), Tol::witness()),
        ),
        (
            "cylinder x cylinder 1e-3 apart".to_string(),
            cyl.clone(),
            cylinder_at(1.001),
        ),
        (
            "cylinder x cylinder shifted 0.3".to_string(),
            cyl.clone(),
            cylinder_at(0.3),
        ),
    ];
    for &x_max in &[-0.5003, -0.5006, -0.501, -0.502, -0.51, -0.6] {
        v.push((
            format!("cylinder x rim plate clear by {:.1e}", -0.5 - x_max),
            cyl.clone(),
            rim_plate(x_max),
        ));
    }
    for &y_min in &[0.5006, 0.502, 0.51] {
        v.push((
            format!("cylinder x top rim plate clear by {:.1e}", y_min - 0.5),
            cyl.clone(),
            top_rim_plate(y_min),
        ));
    }
    v
}

fn ex(t: &SweepTrace) -> BTreeSet<(topo::EdgeKey, topo::FaceKey)> {
    t.examined.iter().copied().collect()
}

fn digest(r: &Result<BooleanResult<f64>, topo::BooleanError>) -> String {
    match r {
        Ok(BooleanResult::Empty) => "Ok(Empty)".into(),
        Ok(BooleanResult::Body(b)) => format!(
            "Ok({:?} f={} e={} v={})",
            b.kind,
            b.body.faces().count(),
            b.body.edges().count(),
            b.body.vertices().count()
        ),
        Err(e) => format!("Err({e:?})").chars().take(80).collect(),
        #[allow(unreachable_patterns)]
        Ok(_) => "Ok(other)".into(),
    }
}

/// The adopted arm's candidate set on the corpus: 98 examined pairs
/// (the base arm examined 134 — the 36 lost are the landing PR's
/// table, each on loci separated by more than the pad with no
/// reference-accepted event), and no reference-accepted pair
/// unexamined wherever the reference runs.
#[test]
fn n3r1_prune_corpus_examines_98_pairs_and_loses_no_accepted_one() {
    let mut total_prune_pairs = 0usize;
    for (name, a, b) in corpus() {
        // A pair the realized sweep refuses typed (a curved pierce the
        // crossing lanes do not handle) carries no candidate count, as
        // in the reviewer's totals.
        let Ok(real) = sweep_traces(&a, &b, SweepStrategy::Realized, None, Tol::witness()) else {
            continue;
        };
        total_prune_pairs += ex(&real.0).len() + ex(&real.1).len();
        if let Ok((ix, iy)) = sweep_traces(&a, &b, SweepStrategy::Idealized, None, Tol::witness()) {
            let lx = ix
                .accepted
                .iter()
                .filter(|p| !ex(&real.0).contains(p))
                .count();
            let ly = iy
                .accepted
                .iter()
                .filter(|p| !ex(&real.1).contains(p))
                .count();
            assert_eq!(lx + ly, 0, "{name}: an accepted pair was never examined");
        }
        let _ = digest(&topo::boolean::subtract(&a, &b, Tol::witness()));
    }
    assert_eq!(total_prune_pairs, 98, "the corpus's candidate total moved");
}
