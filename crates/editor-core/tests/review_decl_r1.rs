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
use crate::fixture::{ang, built_bits, fname, insert, len, scl, step, wall};
use editor_core::{
    BooleanOp, CapEnd, DocEdit, EditError, Node, NodeErrorKind, ProfileDoc, RecipeNodeId,
    ResolveError, RoleSeg, SitedRef, find_flush_candidates,
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

/// The rest of `d`'s bottom on member `m`'s top, sited at each.
fn rests(m: RecipeNodeId, d: RecipeNodeId) -> (SitedRef, SitedRef) {
    (
        SitedRef::new(m, fname(m, RoleSeg::Cap(CapEnd::End))),
        SitedRef::new(d, fname(d, RoleSeg::Cap(CapEnd::Start))),
    )
}

/// **A contact against what the fold merges glues in every member
/// order, declared or not.** `a` and `c` are declared flush, so the
/// fold merges their tops into one `Merged({a.capEnd, c.capEnd})` row;
/// `d` rests on both, undeclared. The margins decide each rest one
/// carrier, so every order builds, and declaring both rests — each
/// resolving to the merged row through the look-through at `d`'s step
/// — builds that order's body bit for bit.
#[test]
fn a_contact_against_a_merged_cap_glues_in_every_order_declared_or_not() {
    let doc = ProfileDoc::empty_derived("r1_merged_refusal", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (0.2, 1.3), (0.2, 0.8), 1.0, 0.5);
    let flush: Vec<_> = flush_pairs(&doc, (a, a), (c, c))
        .into_iter()
        .map(|p| (p, editor_core::BooleanCoincidence::Continuation))
        .collect();
    for order in [[a, c, d], [d, c, a], [c, d, a]] {
        let (bare, union) = declared_union_classed(doc.clone(), &order, flush.clone());
        let undeclared = run(&bare);
        assert!(
            failure(&undeclared, union).is_none(),
            "{order:?}: {:?}",
            failure(&undeclared, union)
        );
        let v = volume(&undeclared, union);
        assert!((v - (1.5 + 1.1 * 0.6 * 0.5)).abs() < 1e-9, "{order:?}: {v}");
        let mut pairs = flush.clone();
        for m in [a, c] {
            pairs.push((rests(m, d), editor_core::BooleanCoincidence::REST));
        }
        let (full, union) = declared_union_classed(doc.clone(), &order, pairs);
        let declared = run(&full);
        assert!(
            failure(&declared, union).is_none(),
            "{order:?}: {:?}",
            failure(&declared, union)
        );
        assert_eq!(
            built_bits(&declared, union),
            built_bits(&undeclared, union),
            "{order:?}: the declared union is the undeclared one's body"
        );
    }
}

/// **Both member contacts declared, the fold resolves them through the
/// merge**: at `d`'s step `a`'s and `c`'s tops are one merged row, and
/// each declaration rewrites to it (the look-through). Declaring only
/// `(c, d)` leaves `(a, d)` undeclared, which glues by its margins, and
/// builds the fully declared body bit for bit.
#[test]
fn a_merged_row_contact_is_declared_through_its_constituents() {
    let doc = ProfileDoc::empty_derived("r1_merged_declared", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (0.2, 1.3), (0.2, 0.8), 1.0, 0.5);
    let mut pairs: Vec<_> = flush_pairs(&doc, (a, a), (c, c))
        .into_iter()
        .map(|p| (p, editor_core::BooleanCoincidence::Continuation))
        .collect();
    pairs.push((rests(c, d), editor_core::BooleanCoincidence::REST));
    let (only_c, union) = declared_union_classed(doc.clone(), &[a, c, d], pairs.clone());
    let partial = run(&only_c);
    assert!(
        failure(&partial, union).is_none(),
        "{:?}",
        failure(&partial, union)
    );
    pairs.push((rests(a, d), editor_core::BooleanCoincidence::REST));
    let (doc, union) = declared_union_classed(doc, &[a, c, d], pairs);
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let v = volume(&ev, union);
    assert!((v - (1.5 + 1.1 * 0.6 * 0.5)).abs() < 1e-9, "{v}");
    assert_eq!(
        built_bits(&ev, union),
        built_bits(&partial, union),
        "the partly declared union is the fully declared one's body"
    );
}

// ---------------------------------------------------------------------
// Claim 2 — the site is the side, through a pass-through operand.
// ---------------------------------------------------------------------

/// Sited at the EXTRUDE (the minting node) when the boolean's operand
/// is a transform of it: the insert door refuses
/// `DeclaredSiteNotAnOperand`, naming the extrude. Sited at the
/// transform but naming a row it does not carry: the door admits it
/// and the evaluation refuses `Vanished`.
#[test]
fn a_pair_boolean_site_at_the_minting_node_refuses_and_an_absent_row_vanishes() {
    let base = ProfileDoc::empty_derived("r1_site_mint", Tol::witness());
    let (base, a) = block(base, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (base, b0) = block(base, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (base, tr) = placed(base, b0, 0.5);
    let boolean = |declare| Node::Boolean {
        op: BooleanOp::Union,
        a: a.into(),
        b: tr.into(),
        declare,
    };
    // Sited at the minting node, which is not an operand.
    let decl = editor_core::declare_continuation(vec![(
        SitedRef::new(a, fname(a, wall(&base, a, 0))),
        SitedRef::new(b0, fname(b0, wall(&base, b0, 0))),
    )]);
    let refused = base.apply(
        &DocEdit::InsertNode {
            node: Box::new(boolean(decl)),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    assert!(
        matches!(&refused, Err(EditError::DeclaredSiteNotAnOperand { site, .. }) if site.id() == b0),
        "{refused:?}"
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
    let (doc, u) = insert(base, boolean(decl));
    let ev = run(&doc);
    assert!(
        matches!(
            failure(&ev, u),
            Some(NodeErrorKind::DeclareResolve { error, .. }) if matches!(**error, ResolveError::Vanished { .. })
        ),
        "{:?}",
        failure(&ev, u)
    );
}

/// **A dead name outranks a foreign site** — at the pair boolean's
/// door, and in the evaluation of a union whose site was stranded.
///
/// A pair boolean's foreign site never reaches its evaluation: the
/// door refuses it. So its order is the door's — name liveness
/// (`DeclareNamesMissingNode`) is asked before the side rule, and a
/// pair naming a dead node at a site that is not an operand says the
/// former, where the same pair with the node live says the latter.
///
/// A union's site can still stop being an operand, by a `SetMembers`
/// that drops it, and there rung 1 is the evaluation's: a name whose
/// minting node is gone says `NodeGone`, where the same strand with the
/// node live says `Vanished`. Both declaring nodes ask the evaluation
/// question through one function (`wire.rs`'s `site_operand`), which
/// is where that order is written.
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
    let boolean = Node::Boolean {
        op: BooleanOp::Union,
        a: a.into(),
        b: b.into(),
        declare: decl.clone(),
    };
    let insert_into = |doc: &ProfileDoc| {
        doc.apply(
            &DocEdit::InsertNode {
                node: Box::new(boolean.clone()),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
    };
    let live = insert_into(&doc);
    assert!(
        matches!(&live, Err(EditError::DeclaredSiteNotAnOperand { site, .. }) if site.id() == x),
        "with the name's node live, the foreign site is the fault: {live:?}"
    );
    let (gone, _) = step(doc.clone(), DocEdit::DeleteNode { id: c });
    let dead = insert_into(&gone);
    assert!(
        matches!(&dead, Err(EditError::DeclareNamesMissingNode { .. })),
        "the dead name does not outrank the foreign site at the door: {dead:?}"
    );

    // The union: sited at member `x`, naming member `c`'s wall (D10: a
    // pair names what its node reads); `SetMembers` then drops both,
    // reporting the name out of reach and never refusing it.
    let (doc, u) = declared_union(doc, &[a, b, x, c], vec![decl[0].0.clone()]);
    let (stranded, _) = step(
        doc,
        DocEdit::SetMembers {
            node: u,
            members: vec![a.into(), b.into()],
        },
    );
    let ev = run(&stranded);
    let got = failure(&ev, u);
    assert!(
        matches!(got, Some(NodeErrorKind::DeclareResolve { error, .. }) if matches!(**error, ResolveError::Vanished { .. })),
        "with the name's node live, the stranded site vanishes: {got:?}"
    );
    let (stranded, _) = step(stranded, DocEdit::DeleteNode { id: c });
    let ev = run(&stranded);
    let got = failure(&ev, u);
    assert!(
        matches!(got, Some(NodeErrorKind::DeclareResolve { error, .. }) if matches!(**error, ResolveError::NodeGone { .. })),
        "rung 1 does not outrank the stranded site at the union: {got:?}"
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
        let positions = |id: RecipeNodeId| doc.ids().iter().position(|n| *n == id);
        let mut has_declare = false;
        for id in doc.ids() {
            if let Some(Node::Boolean { declare, .. } | Node::Union { declare, .. }) = doc.node(id)
            {
                has_declare |= !declare.is_empty();
                for r in declare.iter().flat_map(|((x, y), _)| [x, y]) {
                    assert!(positions(r.at) < positions(id), "{}: site forward", d.name);
                    assert!(
                        positions(r.name.node) < positions(id),
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
            replay.ids(),
            doc.ids(),
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

/// **The detector's findings declare verbatim on a union**: one finding
/// `declare`d leaves the union building, `declare_all` over every
/// finding replaces the list whole, and each declared union is the
/// undeclared one's body bit for bit.
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
            members: vec![m1.into(), m2.into()],
            declare: Vec::new(),
        },
    );
    let undeclared = run(&bare);
    assert!(
        failure(&undeclared, union).is_none(),
        "{:?}",
        failure(&undeclared, union)
    );
    assert_eq!(volume(&undeclared, union), 1.5);
    let one = editor_core::declare(&bare, union, &findings[0], Tol::witness())
        .expect("declares one finding verbatim");
    let ev = run(&one.doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    assert_eq!(
        built_bits(&ev, union),
        built_bits(&undeclared, union),
        "one pair declared of four builds the undeclared body"
    );
    let all =
        editor_core::declare_all(&one.doc, union, &findings, Tol::witness()).expect("declares");
    assert_eq!(
        all.doc.node(union).expect("live").declared_pairs().len(),
        findings.len(),
        "declare_all replaces the list with every finding"
    );
    let ev = run(&all.doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    assert_eq!(
        built_bits(&ev, union),
        built_bits(&undeclared, union),
        "the fully declared union is the undeclared one's body"
    );
}
