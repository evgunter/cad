//! **Rows adopted from EDIT-DECL's first blinded review** — kept in
//! their author's file and re-headed to the invariant each pins.
//!
//! What is NOT here, because one row is enough and it lives beside the
//! claim it is about: the swapped-site row (the site is the side) is
//! `docm7_union_declare`'s
//! `a_name_the_other_operand_carries_is_not_read_at_its_site`; the
//! second dead site of an insert is that suite's
//! `the_insert_door_refuses_a_declare_whose_name_or_site_is_not_live`,
//! which now exercises both sides; the rebind and the `SetMembers`
//! drop are `rebind_moves_the_name_and_leaves_the_site` and
//! `a_declared_member_removed_by_set_members_refuses`; and the bare
//! persisted side is `a_declared_pair_side_that_is_a_bare_name_does_not_load`,
//! which asserts what the refusal says.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, run};
use crate::fixture::{ang, fname, insert, len, scl, step, wall};
use editor_core::{
    BooleanOp, CapEnd, DocEdit, Node, NodeErrorKind, ProfileDoc, RecipeNodeId, ResolveError,
    RoleSeg, SitedRef, find_flush_candidates,
};
use geom_core::Tol;

fn placed(doc: ProfileDoc, input: RecipeNodeId, dx: f64) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Transform {
            input,
            translation: [len(dx), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    )
}

fn volume(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> f64 {
    topo::mass_properties(body_of(ev, id), Tol::witness())
        .expect("mass")
        .volume
}

// ---------------------------------------------------------------------
// A union's refusal against a MERGED row of the accumulation.
// ---------------------------------------------------------------------

/// **A contact against a merged CAP is refused sited at a
/// constituent.** `a` and `c` fuse at step 1 (declared), so the
/// accumulation's top is one `Merged({a.capEnd, c.capEnd})` row; `d`
/// rests on that merged row at step 2, undeclared. The merged row is
/// the fold's own and has no site, so the refusal takes the
/// constituent whose member comes first in the list and carries the
/// whole set beside it.
#[test]
fn a_union_refusal_against_a_merged_cap_is_sited_at_a_constituent() {
    let doc = ProfileDoc::empty_derived("r1_merged_refusal", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (0.2, 1.3), (0.2, 0.8), 1.0, 0.5);
    let pairs = flush_pairs(&doc, (a, a), (c, c));
    let (doc, union, _) = declared_union(doc, &[a, c, d], pairs);
    let ev = run(&doc);
    let got = failure(&ev, union);
    let Some(NodeErrorKind::UndeclaredContact {
        finding, merged, ..
    }) = got
    else {
        panic!(
            "a user's undeclared contact against a merged row is not reported as an \
             UndeclaredContact finding: {got:?}"
        )
    };
    assert_eq!(
        merged.0.iter().map(|r| r.at).collect::<Vec<_>>(),
        vec![a, c],
        "the constituents, in member order"
    );
    assert_eq!(finding.pair.0, merged.0[0]);
    assert_eq!(finding.pair.1.at, d);
}

/// **Declaring the contact through a CONSTITUENT of the merged row,
/// sited at its member, resolves through the look-through** — which
/// is what makes the refusal above actionable.
#[test]
fn a_merged_row_contact_is_declared_through_a_constituent() {
    let doc = ProfileDoc::empty_derived("r1_merged_declared", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (0.2, 1.3), (0.2, 0.8), 1.0, 0.5);
    let mut pairs = flush_pairs(&doc, (a, a), (c, c));
    pairs.push((
        SitedRef::new(c, fname(c, RoleSeg::Cap(CapEnd::End))),
        SitedRef::new(d, fname(d, RoleSeg::Cap(CapEnd::Start))),
    ));
    let (doc, union, _) = declared_union(doc, &[a, c, d], pairs);
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let v = volume(&ev, union);
    assert!((v - (1.5 + 1.1 * 0.6 * 0.5)).abs() < 1e-9, "{v}");
}

// ---------------------------------------------------------------------
// Claim 2 — the site is the side, through a pass-through operand.
// ---------------------------------------------------------------------

/// Sited at the EXTRUDE (the minting node) when the boolean's operand
/// is a transform of it: `DeclareSiteNotAnOperand`. Sited at the
/// transform but naming a row it does not carry: `Vanished`.
#[test]
fn a_pair_boolean_site_at_the_minting_node_refuses_and_an_absent_row_vanishes() {
    let base = ProfileDoc::empty_derived("r1_site_mint", Tol::witness());
    let (base, a) = block(base, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (base, b0) = block(base, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (base, tr) = placed(base, b0, 0.5);
    let boolean = |doc: ProfileDoc, decl| {
        insert(
            doc,
            Node::Boolean {
                op: BooleanOp::Union,
                a,
                b: tr,
                declare: Some(decl),
            },
        )
    };
    // Sited at the minting node, which is not an operand.
    let (doc, decl) = insert(
        base.clone(),
        Node::declare_rest(vec![(
            SitedRef::new(a, fname(a, wall(&base, a, 0))),
            SitedRef::new(b0, fname(b0, wall(&base, b0, 0))),
        )]),
    );
    let (doc, u) = boolean(doc, decl);
    let ev = run(&doc);
    assert!(
        matches!(failure(&ev, u), Some(NodeErrorKind::DeclareSiteNotAnOperand { at }) if *at == b0),
        "{:?}",
        failure(&ev, u)
    );
    // Sited at the transform, naming a row the block does not have.
    let (doc, decl) = insert(
        base.clone(),
        Node::declare_rest(vec![(
            SitedRef::new(a, fname(a, wall(&doc, a, 0))),
            SitedRef::new(
                tr,
                fname(
                    b0,
                    editor_core::RoleSeg::Lateral(crate::fixture::no_piece()),
                ),
            ),
        )]),
    );
    let (doc, u) = boolean(doc, decl);
    let ev = run(&doc);
    assert!(
        matches!(
            failure(&ev, u),
            Some(NodeErrorKind::DeclareResolve { error }) if matches!(**error, ResolveError::Vanished { .. })
        ),
        "{:?}",
        failure(&ev, u)
    );
}

/// **Rung 1 outranks the site question at the PAIR boolean too** — a
/// name whose minting node is gone says `NodeGone`, even when its
/// site is also not an operand. Both declaring doors ask the question
/// through one function (`wire.rs`'s `site_operand`), which is where
/// that order is written.
#[test]
fn rung_one_outranks_a_foreign_site_at_the_pair_boolean() {
    let doc = ProfileDoc::empty_derived("r1_rung1_pair", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (8.0, 9.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, x) = block(doc, (12.0, 13.0), (0.0, 1.0), 0.0, 1.0);
    // The name is `c`'s; the site is `x`, live but not an operand.
    let node = Node::declare_rest(vec![(
        SitedRef::new(a, fname(a, wall(&doc, a, 0))),
        SitedRef::new(x, fname(c, wall(&doc, c, 0))),
    )]);
    let (doc, decl) = insert(doc, node);
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
        },
    );
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: c });
    let ev = run(&doc);
    let got = failure(&ev, u);
    assert!(
        matches!(got, Some(NodeErrorKind::DeclareResolve { error }) if matches!(**error, ResolveError::NodeGone { .. })),
        "rung 1 does not outrank the site question at the pair boolean: {got:?}"
    );
}

// ---------------------------------------------------------------------
// Claim 5 — deleting a site; rebinding a name away from its site.
// ---------------------------------------------------------------------

/// **A `Declare` orphaned by a cascade is reported at the delete that
/// orphans it.** `DeleteNode` of a site (a transform member) cascades
/// the union away, so no consumer is left to refuse: the `Declare`
/// survives, it strands nothing (its names are the prototype's, which
/// lives; a site is not a name), and the next evaluation says nothing
/// about it — a consumerless declaration is a legal document.
///
/// What the cascade owes is the maintenance row, ONCE, at the step
/// that took the last consumer: the union's. The member's own step
/// names no orphan, because the member consumed no declaration, so a
/// walk reporting every consumerless `Declare` at every step reports
/// two rows here.
#[test]
fn a_declare_orphaned_by_a_cascade_is_reported_at_the_delete_that_orphans_it() {
    let doc = ProfileDoc::empty_derived("r1_delete_site", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, m1) = placed(doc, proto, 0.0);
    let (doc, m2) = placed(doc, proto, 0.5);
    let pairs = flush_pairs(&doc, (m1, proto), (m2, proto));
    let (doc, union, decl) = declared_union(doc, &[m1, m2], pairs);
    let order = editor_core::cascade_delete_order(&doc, m2);
    assert_eq!(order, vec![union, m2], "{order:?}");
    let mut doc = doc;
    let mut per_step = Vec::new();
    for id in order {
        let applied = doc
            .apply(
                &DocEdit::DeleteNode { id },
                Tol::witness(),
                &editor_core::RefusingReach,
            )
            .unwrap_or_else(|e| panic!("deleting {id:?}: {e:?}"));
        per_step.push(applied.maintenance);
        doc = applied.doc;
    }
    assert!(doc.node(union).is_none(), "the union cascaded");
    assert!(doc.node(decl).is_some(), "the Declare survives");
    let ev = run(&doc);
    assert!(failure(&ev, decl).is_none(), "{:?}", failure(&ev, decl));
    // Each step's whole list, rather than a filtered count: what the
    // cascade owes is one row in one place, so the position and the
    // uniqueness are pinned together, and the absent strand (the
    // names are `proto`'s, which lives; the site is `m2`, which is
    // not a name) is pinned by the same assertion.
    assert_eq!(
        per_step,
        vec![
            vec![editor_core::Maintenance::OrphanedDeclare { declare: decl }],
            Vec::new(),
        ],
        "the union's step reports the orphan; the member's step consumes no declaration"
    );
}

/// **Every corpus document that declares replays in document order**:
/// the `Declare` precedes its consumer in `order()`, every name and
/// every site points BACKWARD, and re-inserting the nodes edit by
/// edit is accepted. The one-pass claim, measured over the whole
/// corpus rather than over one fixture.
#[test]
fn every_declaring_corpus_document_replays_in_document_order() {
    let mut declaring = 0;
    for d in crate::corpus::documents() {
        let doc = &d.doc;
        let positions = |id: RecipeNodeId| doc.order().iter().position(|n| *n == id);
        let mut has_declare = false;
        for id in doc.order() {
            if let Some(Node::Declare { pairs }) = doc.node(*id) {
                has_declare = true;
                for r in pairs.iter().flat_map(|((x, y), _)| [x, y]) {
                    assert!(positions(r.at) < positions(*id), "{}: site forward", d.name);
                    assert!(
                        positions(r.name.node) < positions(*id),
                        "{}: name forward",
                        d.name
                    );
                }
            }
        }
        if !has_declare {
            continue;
        }
        declaring += 1;
        // The edit log replays from empty, edit by edit, and every
        // `Declare` insert precedes the insert of its consumer.
        let mut replay = ProfileDoc::empty_derived("r1_replay", Tol::witness());
        let mut declares_seen = 0;
        for (i, entry) in d.edits.iter().enumerate() {
            if let DocEdit::InsertNode { node } = &entry.edit {
                match node {
                    Node::Declare { .. } => declares_seen += 1,
                    Node::Boolean {
                        declare: Some(_), ..
                    }
                    | Node::Union {
                        declare: Some(_), ..
                    } => assert!(declares_seen > 0, "{}: consumer at edit {i} first", d.name),
                    _ => {}
                }
            }
            // The logged replay: the recorded rows, never a solve.
            replay = editor_core::apply_logged(&replay, entry, Tol::witness())
                .unwrap_or_else(|e| panic!("{}: edit {i} refused: {e:?}", d.name))
                .doc;
        }
        assert_eq!(
            replay.order(),
            doc.order(),
            "{}: the replay is the document",
            d.name
        );
    }
    // Six: `kiss_carry`, `slots`, `part_select`, `corner_table`,
    // `kitchen_sink` and `die`. Exact, so a document that stops
    // declaring is not silently dropped from this row's reach.
    assert_eq!(declaring, 6, "the declaring corpus documents");
}

// ---------------------------------------------------------------------
// The DM4 headline through the flush door: findings of two placements
// of one prototype, declared verbatim, consumed by a union in one pass.
// ---------------------------------------------------------------------

/// **A union's own refusal is declarable verbatim**, one finding at a
/// time: `declare_all` over the detector's findings fuses the pair,
/// and declaring the union's refusal itself leaves the remaining
/// contacts refusing — each one a finding of the same shape.
#[test]
fn flush_findings_of_two_placements_declare_and_fuse_through_a_union() {
    let doc = ProfileDoc::empty_derived("r1_flush_union", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, m1) = placed(doc, proto, 0.0);
    let (doc, m2) = placed(doc, proto, 0.5);
    let ev = run(&doc);
    let findings = find_flush_candidates(&ev, m1, m2, Tol::witness()).expect("detects");
    assert_eq!(findings.len(), 4, "{findings:?}");
    for f in &findings {
        assert_eq!((f.pair.0.at, f.pair.1.at), (m1, m2));
        assert_eq!((f.pair.0.name.node, f.pair.1.name.node), (proto, proto));
    }
    let (applied, decl) =
        editor_core::declare_all(&doc, &findings, Tol::witness()).expect("declares");
    let (doc, union) = insert(
        applied.doc,
        Node::Union {
            members: vec![m1, m2],
            declare: Some(decl),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    assert_eq!(volume(&ev, union), 1.5);
    // And the union's OWN refusal, declared verbatim, is the same set.
    let (bare, plain) = insert(
        doc.clone(),
        Node::Union {
            members: vec![m1, m2],
            declare: None,
        },
    );
    let ev = run(&bare);
    let Some(NodeErrorKind::UndeclaredContact { finding, .. }) = failure(&ev, plain) else {
        panic!("{:?}", failure(&ev, plain))
    };
    let (applied, decl2) =
        editor_core::declare_all(&bare, core::slice::from_ref(&**finding), Tol::witness())
            .expect("declares the refusal verbatim");
    let (doc2, union2) = insert(
        applied.doc,
        Node::Union {
            members: vec![m1, m2],
            declare: Some(decl2),
        },
    );
    let ev = run(&doc2);
    // One pair declared of four: the union refuses the NEXT undeclared
    // contact, which is a finding of the same shape — so the loop
    // "declare what it names, evaluate again" terminates rather than
    // changing character.
    let next = failure(&ev, union2);
    assert!(
        matches!(next, Some(NodeErrorKind::UndeclaredContact { .. })),
        "{next:?}"
    );
}
