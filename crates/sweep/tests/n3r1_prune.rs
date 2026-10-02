//! **Adopted from a reviewer probe** (the CERT-N3 dual review) as an
//! ordinary row: the pruning-delta corpus. The reviewer ran it against both arms through an env-gated plant in `edge_box`; the plant stays out of the tree, so this row pins the adopted arm's side of the table — the candidate total on the corpus and the superset property wherever the brute-force reference runs — with the base arm's 134 recorded in the landing PR.
//!
//!
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::{
    nested_box, rim_plate, rounded_plate, three_arc_cylinder, top_rim_plate,
};
use geom_core::{Point2, Tol};
use std::collections::BTreeSet;
use sweep::test_support::brick;
use topo::{Body, BooleanResult, SweepStrategy, SweepTrace, sweep_traces};

/// The corpus cylinder at `(cx, 0)`, `z in [0, 1]`: this suite poses
/// it by translating the PROFILE in `x`.
fn cylinder_at(cx: f64) -> Body<f64> {
    three_arc_cylinder(Point2::new(cx, 0.0), 0.5, 0.0, 1.0, 0.0)
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

/// Corpus pairs whose idealized trace counts, as the pair's event, a
/// face-free vertex record made against a face its edge's box never
/// meets — the same v-v record the face holding the vertex makes, which
/// the accumulator dedups, so the realized sweep loses no contact by
/// pruning it
/// (`work/hone/sweep-trace-counts-a-face-free-vertex-record-as-the-pairs-event.md`).
/// The count is pinned, and
/// [`n3r1_prune_realized_and_idealized_sweeps_record_the_same_contacts`]
/// is the guard behind it: a pair the exemption hid that carried a real
/// event would show there as a contact or a split one strategy lacks.
const FACE_FREE_RECORDS: &[(&str, usize)] = &[("cylinder x cylinder shifted 0.3", 16)];

/// The adopted arm's candidate set on the corpus: 154 examined pairs —
/// the landing PR's 98 (the base arm examined 134; the 36 lost are that
/// PR's table, each on loci separated by more than the pad with no
/// reference-accepted event), and the 56 of the cylinder pair shifted
/// 0.3, whose rim crossings the circle × cylinder root lane now
/// certifies — and no reference-accepted pair unexamined wherever the
/// reference runs, but the face-free records above.
#[test]
fn n3r1_prune_corpus_examines_154_pairs_and_loses_no_accepted_one() {
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
            let face_free = FACE_FREE_RECORDS
                .iter()
                .find(|(n, _)| *n == name)
                .map_or(0, |&(_, k)| k);
            assert_eq!(
                lx + ly,
                face_free,
                "{name}: an accepted pair was never examined"
            );
        }
        let _ = digest(&topo::boolean::subtract(&a, &b, Tol::witness()));
    }
    assert_eq!(total_prune_pairs, 154, "the corpus's candidate total moved");
}

/// **Pruning loses no contact and no split, corpus-wide** — the guard
/// behind [`FACE_FREE_RECORDS`]. The realized and idealized sweeps must
/// record the same contacts and leave operands of the same sizes, and a
/// pair one of them refuses the other refuses the same way.
#[test]
fn n3r1_prune_realized_and_idealized_sweeps_record_the_same_contacts() {
    let key = |r: &topo::ContactRecords| {
        let mut v: Vec<String> = r.vv.iter().map(|c| format!("{c:?}")).collect();
        v.extend(r.a_on_b.iter().map(|c| format!("A{c:?}")));
        v.extend(r.b_on_a.iter().map(|c| format!("B{c:?}")));
        v.sort();
        v
    };
    let mut compared = Vec::new();
    for (name, a, b) in corpus() {
        let real = topo::sweep_records(&a, &b, SweepStrategy::Realized, Tol::witness());
        let ideal = topo::sweep_records(&a, &b, SweepStrategy::Idealized, Tol::witness());
        match (&real, &ideal) {
            (Ok((rr, rs)), Ok((ir, is))) => {
                assert_eq!(rs, is, "{name}: the split operands' sizes differ");
                assert_eq!(key(rr), key(ir), "{name}: the contact records differ");
                compared.push(name);
            }
            (Err(re), Err(ie)) => assert_eq!(
                core::mem::discriminant(re),
                core::mem::discriminant(ie),
                "{name}: the two sweeps refuse differently: {re:?} vs {ie:?}"
            ),
            _ => panic!("{name}: one sweep refuses and the other does not: {real:?} vs {ideal:?}"),
        }
    }
    assert!(
        compared
            .iter()
            .any(|n| FACE_FREE_RECORDS.iter().any(|(f, _)| f == n)),
        "the exempted pair is among those compared: {compared:?}"
    );
}
