//! Reviewer probe (PR 7470 at 090c8510493, claim 10): **a family
//! member's indexed read of ANOTHER member is served stale from a
//! prior at `f64`**, because the member's content key hashes the
//! upstream family as the member bound at its own index
//! (`members::overlay`) and the index VALUE it reads at, never the key
//! of the member that index picks.
//!
//! The fixture: `tall[k]`, `k` in `0..2`, an extrude `0.25 + k·p` deep;
//! and `reader[k] = Union([tall[(k + 1) mod 2]])`. Member 0 of `tall`
//! does not move with `p` (`0.25 + 0·p`), member 1 does; `reader[0]`
//! reads `tall[1]`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, insert, step};
use editor_core::{
    Bodies, BodyRead, BooleanValue, Dimension, DocEdit, Evaluation, Formula, FreeVar, Node,
    ProfileDoc, RecipeNodeId, UnitSym, ValuePayload, VarDecl, VarName,
};
use geom_core::Tol;

fn doc_at(p: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("review-stale-member-read", Tol::witness());
    let (doc, _) = step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("p"),
            def: VarDecl::Free(FreeVar::Continuous {
                dim: Dimension::Length,
                value: p,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: None,
            }),
        },
    );
    let (doc, k) = fixture::index_of(doc, "k", Formula::count(2), None);
    let (doc, profile) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    let scalar_k = Formula::count_to_scalar(Formula::var(k, Dimension::Count)).unwrap();
    let depth = Formula::add(
        fixture::len(0.25),
        Formula::mul(
            scalar_k,
            Formula::named(VarName::from_static("p"), Dimension::Length),
        )
        .unwrap(),
    )
    .unwrap();
    let (doc, tall) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: depth,
        },
    );
    let next = Formula::modulo(
        Formula::add(Formula::var(k, Dimension::Count), Formula::count(1)).unwrap(),
        Formula::count(2),
    )
    .unwrap();
    let (doc, reader) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![BodyRead::indexed(tall, vec![next])]),
        },
    );
    (doc, tall, reader)
}

/// The height of each member of `id`'s family.
fn heights(ev: &Evaluation<f64>, id: RecipeNodeId) -> Vec<f64> {
    ev.value(id)
        .unwrap_or_else(|| panic!("{id:?}: {:?}", ev.result(id)))
        .members
        .as_ref()
        .expect("a family")
        .members
        .iter()
        .map(|(_, member)| {
            let body = match &member.payload {
                ValuePayload::Body(b) => fixture::in_space(b),
                ValuePayload::Boolean(BooleanValue::Body { body, .. }) => fixture::in_space(body),
                other => panic!("a body, got {}", other.kind_name()),
            };
            body.vertices()
                .map(|(v, _)| fixture::point(&body, v).z)
                .fold(f64::NEG_INFINITY, f64::max)
        })
        .collect()
}

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    fixture::run(doc, &Default::default())
}

/// Measures the served evaluation against a fresh one of the same
/// document. Asserts what was measured on the reviewed head: the reader
/// of the moved member is served the prior's body.
#[test]
fn a_members_read_of_another_member_is_served_from_a_prior_that_read_its_old_body() {
    let (before, tall, reader) = doc_at(0.25);
    let prior = run(&before);
    assert_eq!(heights(&prior, tall), [0.25, 0.5], "tall, p = 0.25");
    assert_eq!(heights(&prior, reader), [0.5, 0.25], "reader, p = 0.25");

    // The same document with `p` set to 0.5 by the value door.
    let p = before.var_named("p").expect("declared");
    let (after, _) = step(
        before,
        DocEdit::SetVarValue {
            var: p.into(),
            value: editor_core::FreeValue::Continuous(0.5),
        },
    );
    let fresh = run(&after);
    assert_eq!(heights(&fresh, tall), [0.25, 0.75], "tall, p = 0.5, fresh");
    assert_eq!(
        heights(&fresh, reader),
        [0.75, 0.25],
        "reader, p = 0.5, fresh"
    );

    let served = fixture::run_with_prior(&after, &prior);
    eprintln!(
        "served over the prior: tall {:?}, reader {:?}, reused {}, recomputed {}",
        heights(&served, tall),
        heights(&served, reader),
        served.reused,
        served.recomputed
    );
    assert_eq!(heights(&served, tall), [0.25, 0.75], "tall is rebuilt");
    assert_eq!(
        heights(&served, reader),
        [0.5, 0.25],
        "MEASURED DEFECT: reader[0] reads tall[1] and is served the prior's 0.5 where a fresh \
         evaluation of the same document builds 0.75; if this row is red the stale read is closed"
    );
}
