//! R2 review probes for INTENT stage 3 G (PR 5954).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, insert};
use editor_core::{
    Bodies, BodyRead, Coord, Dimension, Formula, Node, Operand, ProfileDoc, RecipeNodeId,
    TargetAxis,
};
use geom_core::Tol;

/// A family over `k = index(2)` of per-member `⊔`s: `tall[k] ⊔ lid`,
/// `tall[k]` an extrude of depth `(k + 1)·0.25` (or reversed), `lid` a
/// slab over z ∈ [0.3, 0.55]. Only the member whose extrude is 0.5 tall
/// overlaps its lid.
fn per_member_assembles(reversed: bool) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("r2-carry", Tol::witness());
    let (doc, k) = fixture::index_of(doc, "k", Formula::count(2), None);
    let (doc, profile) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    let kv = Formula::var(k, Dimension::Count);
    let count = if reversed {
        Formula::sub(Formula::count(2), kv).unwrap()
    } else {
        Formula::add(kv, Formula::count(1)).unwrap()
    };
    let depth = Formula::mul(Formula::count_to_scalar(count).unwrap(), fixture::len(0.25)).unwrap();
    let (doc, tall) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: depth,
        },
    );
    let (doc, slab) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: fixture::len(0.25),
        },
    );
    let (doc, lid) = fixture::family_of(
        doc,
        slab,
        vec![(Coord::Slide(TargetAxis::Z), fixture::len(0.3))],
    );
    let target = fixture::space(&doc, slab);
    let (doc, tall) = insert(
        doc,
        Node::place(
            BodyRead::plain(Operand::Node(tall)),
            Operand::Var(target),
            editor_core::stand().bundle(),
        ),
    );
    let (doc, sq) = insert(
        doc,
        Node::Assemble {
            members: Bodies::Spelled(vec![
                BodyRead::indexed(tall, vec![Formula::var(k, Dimension::Count)]),
                BodyRead::plain(Operand::Node(lid)),
            ]),
        },
    );
    (doc, tall, sq)
}

fn interference_count(doc: &ProfileDoc) -> (usize, String) {
    let ev = fixture::run(doc, &Default::default());
    match fixture::at_rest(doc, &ev) {
        Ok(findings) => {
            let i = fixture::interference(&findings);
            (
                i.len(),
                i.iter()
                    .map(|f| {
                        format!(
                            "a=({:?},{}) b=({:?},{})",
                            f.a.var.0.ordinal(),
                            f.a.member,
                            f.b.var.0.ordinal(),
                            f.b.member
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("; "),
            )
        }
        Err(e) => (usize::MAX, format!("checks refused: {e}")),
    }
}

/// Claim 6: a family whose members are NOT copies of one source (a
/// `⊔` per member, built per member). Its overlap stands in one member
/// only; the at-rest check must report it whichever member it is.
#[test]
fn r2_a_per_member_overlap_is_reported_whichever_member_holds_it() {
    let mut seen = Vec::new();
    for reversed in [false, true] {
        let (doc, _, sq) = per_member_assembles(reversed);
        let ev = fixture::run(&doc, &Default::default());
        let n = ev
            .value(sq)
            .and_then(|v| v.members.as_ref())
            .map(|m| m.members.len());
        let (count, dump) = interference_count(&doc);
        eprintln!("reversed={reversed}: members={n:?} interference={count}\n{dump}");
        // Read the family whole in a second ⊔.
        let (doc2, whole) = insert(
            doc.clone(),
            Node::Assemble {
                members: Bodies::Family(sq.into()),
            },
        );
        let _ = whole;
        let (count2, dump2) = interference_count(&doc2);
        eprintln!("reversed={reversed}, read whole: interference={count2}\n{dump2}");
        seen.push((count, count2));
    }
    assert_eq!(
        seen[0], seen[1],
        "the same overlap, whichever member holds it: {seen:?}"
    );
    assert!(seen[0].0 >= 1, "the overlap is reported: {seen:?}");
}

/// Claim 3: after a count edit 4 → 6, the first four members are
/// SERVED from the prior evaluation (the same `Arc`s), not rebuilt with
/// equal keys. Uses a construction per member, whose body is its own.
#[test]
fn r2_a_count_edit_serves_the_first_members_from_the_memo() {
    let doc = ProfileDoc::empty_derived("r2-reuse", Tol::witness());
    let (doc, k) = fixture::index_of(doc, "k", Formula::count(4), None);
    let (doc, profile) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    let count = Formula::add(Formula::var(k, Dimension::Count), Formula::count(1)).unwrap();
    let depth = Formula::mul(Formula::count_to_scalar(count).unwrap(), fixture::len(0.25)).unwrap();
    let (doc, tall) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: depth,
        },
    );
    let first = fixture::run(&doc, &Default::default());
    let grown = fixture::recount(doc, k, Formula::count(6));
    let after = fixture::run_with_prior(&grown, &first);
    let bodies = |ev: &editor_core::Evaluation<f64>| -> Vec<editor_core::Framed<f64>> {
        ev.value(tall)
            .unwrap()
            .members
            .as_ref()
            .unwrap()
            .members
            .iter()
            .map(|(_, m)| match &m.payload {
                editor_core::ValuePayload::Body(b) => b.clone(),
                other => panic!("{}", other.kind_name()),
            })
            .collect()
    };
    let (b0, b1) = (bodies(&first), bodies(&after));
    assert_eq!(b1.len(), 6);
    let shared: Vec<bool> = (0..4)
        .map(|i| std::sync::Arc::ptr_eq(b0[i].shared(), b1[i].shared()))
        .collect();
    eprintln!(
        "reused counter: {} / recomputed {}; shared {shared:?}",
        after.reused, after.recomputed
    );
    assert!(
        shared.iter().all(|&s| s),
        "the first four members are served, not rebuilt: {shared:?}"
    );
}
