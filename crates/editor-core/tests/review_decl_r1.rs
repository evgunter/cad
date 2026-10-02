//! **Rows adopted from EDIT-DECL's first blinded review** — kept in
//! their author's file and re-headed to the invariant each pins.
//!
//! What is NOT here, because one row is enough and it lives beside the
//! claim it is about: the swapped-site row (the site is the side) is
//! `docm7_union_declare`'s
//! `a_name_the_other_operand_carries_is_not_read_at_its_site`; the
//! second dead site of an insert is that suite's
//! `the_insert_door_and_set_declare_refuse_a_name_or_site_that_is_not_live`,
//! which now exercises both sides; the rebind and the `SetMembers`
//! drop are `rebind_moves_the_name_and_leaves_the_site` and
//! `a_declared_member_removed_by_set_members_refuses`; and the bare
//! persisted side is `a_declared_pair_side_that_is_a_bare_name_does_not_load`,
//! which asserts what the refusal says.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{
    block, declared_union, declared_union_classed, failure, flush_pairs, run,
};
use crate::fixture::{ang, fname, insert, len, scl, step, wall};
use editor_core::{
    BooleanOp, CapEnd, DocEdit, Node, NodeErrorKind, ProfileDoc, RecipeNodeId, ResolveError,
    RoleSeg, SitedRef, find_flush_candidates,
};
use geom_core::Tol;

fn placed(doc: ProfileDoc, input: RecipeNodeId, dx: f64) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::transform(
            input,
            editor_core::Step::Rigid {
                translation: [len(dx), len(0.0), len(0.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        ),
    )
}

fn volume(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> f64 {
    topo::mass_properties(body_of(ev, id), Tol::witness())
        .expect("mass")
        .volume
}

// ---------------------------------------------------------------------
// A contact against a MERGED row of the accumulation.
// ---------------------------------------------------------------------

/// **A contact against what the fold merges is refused pairwise, naming
/// two member faces.** `a` and `c` are declared flush, so the fold
/// merges their tops into one `Merged({a.capEnd, c.capEnd})` row; `d`
/// rests on both, undeclared. Contact is judged between members before
/// the fold (DM4), so in every order the refusal names one member's
/// top and `d`'s bottom, the first touching pair by node id, and
/// carries no merged set: no refusal names a row the fold minted.
#[test]
fn a_contact_against_a_merged_cap_is_refused_between_two_members() {
    let doc = ProfileDoc::empty_derived("r1_merged_refusal", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (0.2, 1.3), (0.2, 0.8), 1.0, 0.5);
    for order in [[a, c, d], [d, c, a], [c, d, a]] {
        let (doc, union) = declared_union(doc.clone(), &order, flush_pairs(&doc, (a, a), (c, c)));
        let ev = run(&doc);
        let got = failure(&ev, union);
        let Some(NodeErrorKind::UndeclaredCoincidence {
            finding, merged, ..
        }) = got
        else {
            panic!("{order:?}: expected the pairwise refusal, got {got:?}")
        };
        // The undeclared pairs are `a`'s top on `d`'s bottom and `c`'s
        // top on it; each pair is spelled lower id first, and the
        // first of them by id is the one refused.
        let top = |m: RecipeNodeId| SitedRef::new(m, fname(m, RoleSeg::Cap(CapEnd::End)));
        let bottom = SitedRef::new(d, fname(d, RoleSeg::Cap(CapEnd::Start)));
        let by_id = |x: SitedRef, y: SitedRef| if x.at < y.at { (x, y) } else { (y, x) };
        let want = [by_id(top(a), bottom.clone()), by_id(top(c), bottom.clone())]
            .into_iter()
            .min_by_key(|(x, y)| (x.at, y.at))
            .expect("two candidates");
        assert_eq!(finding.pair, want, "{order:?}");
        assert!(
            merged.0.is_empty() && merged.1.is_empty(),
            "{order:?}: {merged:?}"
        );
    }
}

/// **Both member contacts declared, the fold resolves them through the
/// merge**: at `d`'s step `a`'s and `c`'s tops are one merged row, and
/// each declaration rewrites to it (the look-through). Declaring only
/// `(c, d)` leaves `(a, d)` undeclared, and that refuses.
#[test]
fn a_merged_row_contact_is_declared_through_its_constituents() {
    let doc = ProfileDoc::empty_derived("r1_merged_declared", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (0.2, 1.3), (0.2, 0.8), 1.0, 0.5);
    let rests = |m: RecipeNodeId| {
        (
            SitedRef::new(m, fname(m, RoleSeg::Cap(CapEnd::End))),
            SitedRef::new(d, fname(d, RoleSeg::Cap(CapEnd::Start))),
        )
    };
    let mut pairs: Vec<_> = flush_pairs(&doc, (a, a), (c, c))
        .into_iter()
        .map(|p| (p, editor_core::BooleanCoincidence::Continuation))
        .collect();
    pairs.push((rests(c), editor_core::BooleanCoincidence::REST));
    let (only_c, union) = declared_union_classed(doc.clone(), &[a, c, d], pairs.clone());
    let ev = run(&only_c);
    assert!(
        matches!(failure(&ev, union), Some(NodeErrorKind::UndeclaredCoincidence { finding, .. })
            if [finding.pair.0.at, finding.pair.1.at] == if a < d { [a, d] } else { [d, a] }),
        "{:?}",
        failure(&ev, union)
    );
    pairs.push((rests(a), editor_core::BooleanCoincidence::REST));
    let (doc, union) = declared_union_classed(doc, &[a, c, d], pairs);
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
                declare: decl,
            },
        )
    };
    // Sited at the minting node, which is not an operand.
    let decl = editor_core::declare_continuation(vec![(
        SitedRef::new(a, fname(a, wall(&base, a, 0))),
        SitedRef::new(b0, fname(b0, wall(&base, b0, 0))),
    )]);
    let (doc, u) = boolean(base.clone(), decl);
    let ev = run(&doc);
    assert!(
        matches!(failure(&ev, u), Some(NodeErrorKind::DeclareSiteNotAnOperand { at }) if *at == b0),
        "{:?}",
        failure(&ev, u)
    );
    // Sited at the transform, naming a row the block does not have.
    let decl = editor_core::declare_continuation(vec![(
        SitedRef::new(a, fname(a, wall(&base, a, 0))),
        SitedRef::new(
            tr,
            fname(
                b0,
                editor_core::RoleSeg::Lateral(crate::fixture::no_piece_of(&base).into()),
            ),
        ),
    )]);
    let (doc, u) = boolean(base, decl);
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
    let decl = editor_core::declare_continuation(vec![(
        SitedRef::new(a, fname(a, wall(&doc, a, 0))),
        SitedRef::new(x, fname(c, wall(&doc, c, 0))),
    )]);
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: decl,
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
// One pass: every declaring document replays in document order.
// ---------------------------------------------------------------------

/// **Every corpus document that declares replays in document order**:
/// every name and every site a Boolean or Union declares points
/// BACKWARD from it in `order()`, and re-inserting the nodes edit by
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
            if let Some(Node::Boolean { declare, .. } | Node::Union { declare, .. }) = doc.node(*id)
            {
                has_declare |= !declare.is_empty();
                for r in declare.iter().flat_map(|((x, y), _)| [x, y]) {
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
        // The edit log replays from empty, edit by edit.
        let mut replay = ProfileDoc::empty_derived("r1_replay", Tol::witness());
        for (i, entry) in d.edits.iter().enumerate() {
            // The logged replay: the recorded rows, never a solve.
            replay = editor_core::apply_replayed(&replay, entry, Tol::witness())
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

/// **A union's own refusal is declarable verbatim**, on the live
/// union: declaring the union's refusal leaves the remaining contacts
/// refusing — each one a finding of the same shape — and `declare_all`
/// over the detector's findings, which replaces the list whole, fuses
/// the pair.
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
    let (bare, union) = insert(
        doc,
        Node::Union {
            members: vec![m1, m2],
            declare: Vec::new(),
        },
    );
    let ev = run(&bare);
    let Some(NodeErrorKind::UndeclaredCoincidence { finding, .. }) = failure(&ev, union) else {
        panic!("{:?}", failure(&ev, union))
    };
    let one = editor_core::declare(&bare, union, finding, Tol::witness())
        .expect("declares the refusal verbatim");
    let ev = run(&one.doc);
    // One pair declared of four: the union refuses the NEXT undeclared
    // contact, which is a finding of the same shape — so the loop
    // "declare what it names, evaluate again" terminates rather than
    // changing character.
    let next = failure(&ev, union);
    assert!(
        matches!(next, Some(NodeErrorKind::UndeclaredCoincidence { .. })),
        "{next:?}"
    );
    let all =
        editor_core::declare_all(&one.doc, union, &findings, Tol::witness()).expect("declares");
    let ev = run(&all.doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    assert_eq!(volume(&ev, union), 1.5);
}
