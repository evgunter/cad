//! M4 PR 4 spec D1/D3: the N5 resolution ladder end to end —
//! Resolved / Ambiguous (direct tie and the `rank_by` over-tie
//! widening) / NodeGone / Vanished with the verdict-diff diagnosis
//! (PredicateFlip, StructuralParam, Cascade) + tombstones / typed
//! Indeterminate — plus N3 offers, the rebind suggestion ladder, and
//! the R6 name-level edit-time validation door.
//!
//! SWEEP-STRATEGY NOTE (Ev's 2026-07-29 ruling): this file's pins
//! are about diff/resolve engine behavior GIVEN verdicts, so its
//! evaluator deliberately runs the idealized (verdict-rich) sweep;
//! the production-path degradation is pinned in `m4_pr4_banked`
//! (both strategies side by side) and in the re-pinned `m4_pr4_ci`
//! golden.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::AuthoredNode;

use editor_core::NodeStanding;
use editor_core::{
    CancelToken, CapEnd, Diagnosis, DocEdit, EntityKind, Entry, EvalOptions, Evaluation, NameTable,
    Node, ProfileDoc, Qualifier, RecipeEditRef, RecipeNodeId, Resolution, ResolveError,
    ResolveIndeterminate, RoleSeg, RunCtx, SlotId, StableName, apply_with_names, evaluate,
    rebind_suggestions,
};
use fixture::{insert, len, minted, on_frame, step};
use geom_core::Tol;

/// Idealized (brute-force) boolean sweep since M5 PR 8: this file
/// pins the DIFF/RESOLVE engine's semantics, whose evidence substrate
/// is the verdict log — the idealized sweep keeps interaction-boundary
/// scenarios (overlapping ↔ disjoint) verdict-rich on both sides.
/// The realized sweep prunes the disjoint side's pair space empty
/// (its job); that production-path degradation is pinned in
/// `m4_pr4_banked` (both strategies) — see `fixture/pr4.rs`'s note.
fn run(doc: &editor_core::ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    let opts = EvalOptions {
        boolean_sweep: topo::SweepStrategy::Idealized,
        ..EvalOptions::default()
    };
    evaluate::<f64>(doc, prior, &CancelToken::new(), &opts, Tol::witness())
}

fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
        },
    )
}

/// **A twin of the block `extrude`** ([`block`]): its profile and
/// extrude inserted again, every slot reading the original's variable
/// ([`Node::authored`]), so the two are one geometry by structure —
/// equal values typed apart are two variables, which is no sharing. A
/// variable two slots share is named (VR2), so each the original reads
/// unnamed is named first.
fn twin(doc: ProfileDoc, extrude: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    let Some(Node::Extrude { profile, .. }) = doc.node(extrude) else {
        panic!("{extrude} is a block's extrude")
    };
    let profile = doc
        .operation_of(*profile)
        .expect("the profile read is live");
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("{profile} is a block's profile")
    };
    let _ = program;
    let authored = |doc: &ProfileDoc, id: RecipeNodeId| {
        doc.node(id)
            .unwrap_or_else(|| panic!("{id} is in the document"))
            .authored(doc)
    };
    let mut doc = doc;
    for id in [profile, extrude] {
        let slots = doc.node(id).expect("held").slots();
        for slot in slots {
            let var = doc.slot(id, slot).expect("a slot reads a variable");
            if doc.var_name(var).is_none() {
                let name = editor_core::VarName::new(format!("twin_{}", doc.var_names().len()))
                    .expect("a name");
                doc = step(
                    doc,
                    DocEdit::RenameVar {
                        var: var.into(),
                        name: Some(name),
                    },
                )
                .0;
            }
        }
    }
    let mut node = authored(&doc, profile);
    if let Node::Profile(program) = &mut node {
        program.ids = Vec::new();
    }
    let (doc, profile) = insert(doc, node);
    let mut node = authored(&doc, extrude);
    if let Node::Extrude { profile: of, .. } = &mut node {
        *of = profile.into();
    }
    insert(doc, node)
}

/// The sliding union: A fixed, B placed against A with its slide
/// along x the knob, A ∪ B.
struct Slide {
    doc: ProfileDoc,
    a: RecipeNodeId,
    b0: RecipeNodeId,
    placement: RecipeNodeId,
    union: RecipeNodeId,
    /// `a`'s read, the union's first member.
    a_read: editor_core::VarId,
    /// The placement's read, the union's second member.
    t_read: editor_core::VarId,
}

/// `placement`'s slide along x, value-edited to `tx`: every id stays.
fn slide_placement(doc: ProfileDoc, placement: RecipeNodeId, tx: f64) -> ProfileDoc {
    let slot = fixture::value_slot(
        &doc,
        placement,
        editor_core::Coord::Slide(editor_core::TargetAxis::X),
    );
    step(
        doc,
        DocEdit::SetParam {
            node: placement,
            slot,
            value: len(tx).into(),
            fresh: Vec::new(),
        },
    )
    .0
}

/// The body `placement` copies.
fn placed_body(doc: &ProfileDoc, placement: RecipeNodeId) -> RecipeNodeId {
    match doc.node(placement) {
        Some(Node::Place { source, .. }) => doc
            .operation_of(source.read)
            .expect("the copied body is live"),
        other => panic!("{placement} is a placement, not {other:?}"),
    }
}

fn slide_union(tx: f64) -> Slide {
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b0) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, placement) = fixture::slid(doc, a, b0, [tx, 0.0, 0.0]);
    // The B side is read at the PLACEMENT — the boolean's operand —
    // and named in `b0`'s vocabulary under the copy's `Placed` layer.
    let (a_read, t_read) = (fixture::out(&doc, a), fixture::out(&doc, placement));
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), placement.into()]),
        },
    );
    Slide {
        doc,
        a,
        b0,
        placement,
        union,
        a_read,
        t_read,
    }
}

fn slide_to(s: &Slide, tx: f64) -> ProfileDoc {
    slide_placement(s.doc.clone(), s.placement, tx)
}

/// A joined rim edge's name from the overlapping union's table: one
/// edge along both blocks' rims, named for the set of them
/// (`[Merged([FromA(RimEdge..), FromB(RimEdge..)])]`).
fn joined_rim_name(ev: &Evaluation<f64>, union: RecipeNodeId) -> StableName {
    ev.table_at(union)
        .unwrap()
        .iter()
        .find_map(|(n, e)| {
            let joined =
                n.kind == EntityKind::Edge && matches!(n.path.as_slice(), [RoleSeg::Merged(_)]);
            (joined && matches!(e, Entry::Unique(_))).then(|| n.clone())
        })
        .expect("overlapping union has joined rim edges")
}

// ---- Resolved ----

#[test]
fn union_names_resolve_uniquely_and_pass_through_transforms() {
    let s = slide_union(0.5);
    let ev = run(&s.doc, None);
    let ctx = RunCtx {
        doc: &s.doc,
        eval: &ev,
    };
    // N3: the declared flush caps GLUE — the A-cap
    // wrap retired into the Merged row, which resolves at the union;
    // the retired constituent name itself now fails typed with the
    // merged row among the OFFERS (N3's loud retirement, pinned in
    // the vanishing tests below).
    let cap = minted(EntityKind::Face, s.a, RoleSeg::Cap(CapEnd::End));
    let wrapped = minted(
        EntityKind::Face,
        s.union,
        RoleSeg::From {
            read: s.a_read,
            of: cap.clone().into(),
        },
    );
    let cap_b = minted(EntityKind::Face, s.b0, RoleSeg::Cap(CapEnd::End));
    let wrapped_b = minted(
        EntityKind::Face,
        s.union,
        RoleSeg::From {
            read: s.t_read,
            of: cap_b.clone().into(),
        },
    );
    let mut constituents = vec![wrapped.clone(), wrapped_b];
    constituents.sort_unstable();
    let merged = minted(EntityKind::Face, s.union, RoleSeg::Merged(constituents));
    match crate::fixture::resolve_at_mint(ctx, &merged) {
        Resolution::Resolved(r) => assert_eq!(r.node, s.union),
        other => panic!("expected Resolved, got {other:?}"),
    }
    match crate::fixture::resolve_at_mint(ctx, &wrapped) {
        Resolution::Failed(f) => assert!(
            f.offers.contains(&merged),
            "retired constituent must offer its merge: {f:?}"
        ),
        other => panic!("expected the retired constituent to fail typed, got {other:?}"),
    }
    // The operand-level cap name resolves too — at the EXTRUDE (first
    // carrying node in evaluation order; the transform pass-through
    // carries B's names identically).
    match crate::fixture::resolve_at_mint(ctx, &cap) {
        Resolution::Resolved(r) => assert_eq!(r.node, s.a),
        other => panic!("expected Resolved, got {other:?}"),
    }
}

// ---- Ambiguous: the direct N2 tie ----

#[test]
fn tied_name_resolves_ambiguous_with_the_tie_witness() {
    // PR 3's symmetric U cutter: two prong fragments of B's caps tie.
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, a) = block(doc, (0.0, 4.0), (0.0, 4.0), 4.0);
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (2.0, 1.0),
            (6.0, 1.0),
            (6.0, 3.0),
            (2.0, 3.0),
            (2.0, 2.5),
            (5.0, 2.5),
            (5.0, 1.5),
            (2.0, 1.5),
        ]],
    );
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(2.0),
        },
    );
    let (doc, b) = fixture::slid(doc, a, b, [0.0, 0.0, 1.0]);
    let (doc, sub) = insert(
        doc,
        Node::Subtract {
            from: a.into(),
            tool: b.into(),
        },
    );
    let ev = run(&doc, None);
    let tied: StableName = ev
        .table_at(sub)
        .unwrap()
        .iter()
        .find_map(|(n, e)| matches!(e, Entry::Tied(_)).then(|| n.clone()))
        .expect("the U fixture ties");
    match crate::fixture::resolve_at_mint(
        RunCtx {
            doc: &doc,
            eval: &ev,
        },
        &tied,
    ) {
        Resolution::Failed(f) => {
            let ResolveError::Ambiguous {
                name,
                candidates,
                tie,
            } = &f.error
            else {
                panic!("expected Ambiguous, got {:?}", f.error);
            };
            assert_eq!(*name, tied);
            assert_eq!(candidates, &vec![tied.clone()]);
            assert_eq!(tie.node, sub);
            assert_eq!(tie.at, tied);
            assert_eq!(tie.width, 2);
            assert!(f.offers.is_empty(), "a tie offers nothing to auto-pick");
        }
        other => panic!("expected Failed(Ambiguous), got {other:?}"),
    }
}

// ---- NodeGone ----

#[test]
fn deleting_a_named_node_strands_names_as_node_gone() {
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, _) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block(doc, (2.0, 3.0), (0.0, 1.0), 1.0);
    let cap_b = minted(EntityKind::Face, b, RoleSeg::Cap(CapEnd::End));
    // b has no DAG dependents: deletion is allowed and strands cap_b —
    // N5's ratified dangling semantics.
    let (doc2, _) = step(doc, DocEdit::DeleteNode { id: b });
    let ev = run(&doc2, None);
    match crate::fixture::resolve_at_mint(
        RunCtx {
            doc: &doc2,
            eval: &ev,
        },
        &cap_b,
    ) {
        Resolution::Failed(f) => {
            let ResolveError::NodeGone { name, edit } = &f.error else {
                panic!("expected NodeGone, got {:?}", f.error);
            };
            assert_eq!(*name, cap_b);
            assert_eq!(*edit, RecipeEditRef::NodeDeleted { node: b });
        }
        other => panic!("expected Failed(NodeGone), got {other:?}"),
    }
}

#[test]
fn never_minted_node_reports_foreign_not_deleted() {
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, _a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let ev = run(&doc, None);
    let foreign = minted(
        EntityKind::Face,
        RecipeNodeId::new(0, 9999),
        RoleSeg::Cap(CapEnd::End),
    );
    match crate::fixture::resolve_at_mint(
        RunCtx {
            doc: &doc,
            eval: &ev,
        },
        &foreign,
    ) {
        Resolution::Failed(f) => {
            let ResolveError::NodeGone { edit, .. } = &f.error else {
                panic!("expected NodeGone, got {:?}", f.error);
            };
            assert_eq!(
                *edit,
                RecipeEditRef::ForeignNode {
                    node: RecipeNodeId::new(0, 9999)
                },
                "a never-minted id must not be blamed on a delete"
            );
        }
        other => panic!("expected Failed(NodeGone), got {other:?}"),
    }
}

// ---- Vanished: PredicateFlip diagnosis + tombstone + collapse offer ----

#[test]
fn flip_vanished_name_diagnoses_the_predicate_flip_with_tombstone() {
    let s = slide_union(0.5);
    let ev1 = run(&s.doc, None);
    let probe = joined_rim_name(&ev1, s.union);
    let doc2 = slide_to(&s, 2.5); // disjoint: the joined rims vanish
    let ev2 = run(&doc2, Some(&ev1));
    let res = crate::fixture::resolve_with_prior_at_mint(
        RunCtx {
            doc: &doc2,
            eval: &ev2,
        },
        RunCtx {
            doc: &s.doc,
            eval: &ev1,
        },
        &probe,
    );
    let Resolution::Failed(f) = res else {
        panic!("expected Failed, got {res:?}");
    };
    let ResolveError::Vanished {
        name,
        diagnosis,
        last_good,
    } = &f.error
    else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    assert_eq!(*name, probe);
    // The pillar's promise: a recorded predicate flip on the
    // derivation path, with real signs.
    let Diagnosis::PredicateFlip {
        predicate,
        from,
        to,
    } = diagnosis
    else {
        panic!("expected PredicateFlip, got {diagnosis:?}");
    };
    assert!(!predicate.is_empty());
    assert_ne!(from, to);
    // The tombstone: last-good entry at the union, edge kind, owning
    // body = the union's body name.
    let t = last_good.as_ref().expect("prior run resolved the name");
    assert_eq!(t.kind, EntityKind::Edge);
    assert_eq!(t.patch.node, s.union);
    assert_eq!(
        t.body,
        minted(EntityKind::Body, s.union, RoleSeg::OutputBody)
    );
    // N3's offer: the join stopped happening, so each rim the set
    // lists is a live edge of the disjoint union again — offered for
    // the explicit Rebind, never auto-bound.
    let [RoleSeg::Merged(set)] = probe.path.as_slice() else {
        unreachable!("the probe is a set name")
    };
    for rim in set {
        assert!(
            f.offers.contains(rim),
            "expected {rim:?} among the offers: {:?}",
            f.offers
        );
    }
}

// ---- Vanished: a merged name's retired constituents are no cascade ----

/// Dropping a member from a union whose caps glued: the merged cap
/// vanishes, and neither constituent is the cause. Each constituent
/// retired into the merge (N3) and was never a row of the union, so the
/// cascade rung does not read one as an operand that vanished first;
/// the ladder goes on to the edit at the union, which is on the path.
#[test]
fn a_vanished_merged_name_is_diagnosed_at_the_edit_not_through_a_retired_constituent() {
    let s = slide_union(0.5);
    let ev1 = run(&s.doc, None);
    let merged = crate::fixture::table(&ev1, s.union)
        .iter()
        .find_map(|(n, _)| {
            (n.kind == EntityKind::Face && matches!(n.path.as_slice(), [RoleSeg::Merged(_)]))
                .then(|| n.clone())
        })
        .expect("the flush caps glue into one merged face");
    let (doc2, _) = step(
        s.doc.clone(),
        DocEdit::SetMembers {
            node: s.union,
            members: editor_core::Bodies::Spelled(vec![s.a.into()]),
        },
    );
    let ev2 = run(&doc2, Some(&ev1));
    let res = crate::fixture::resolve_with_prior_at_mint(
        RunCtx {
            doc: &doc2,
            eval: &ev2,
        },
        RunCtx {
            doc: &s.doc,
            eval: &ev1,
        },
        &merged,
    );
    let Resolution::Failed(f) = res else {
        panic!("the merged cap no longer resolves: {res:?}");
    };
    let ResolveError::Vanished { diagnosis, .. } = &f.error else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    assert_eq!(
        *diagnosis,
        Diagnosis::RecipeEdit {
            edit: RecipeEditRef::NodeChanged { node: s.union }
        },
        "the cause is the SetMembers at the union, not a retired constituent"
    );
}

/// The same at a union over that union: `U2 = U1 ∪ c`, with c flush
/// on the glued caps, merges U1's merged cap with c's, listing U1's
/// constituents re-wrapped (N3's flat set). Beneath each of U2's
/// constituents sits one U1 retired, held by a `From` rather than a
/// set. Dropping c from U2 vanishes every merged face of U2, and each
/// is diagnosed at that edit, not through a name U1 retired.
#[test]
fn a_vanished_merged_name_over_a_merged_operand_is_diagnosed_at_the_edit() {
    let s = slide_union(0.5);
    let (doc, c0) = block(s.doc.clone(), (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, c) = fixture::slid(doc, s.a, c0, [1.0, 0.0, 0.0]);
    let (doc, u2) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![s.union.into(), c.into()]),
        },
    );
    let ev1 = run(&doc, None);
    let merged: Vec<StableName> = crate::fixture::table(&ev1, u2)
        .iter()
        .filter(|(n, _)| {
            n.kind == EntityKind::Face && matches!(n.path.as_slice(), [RoleSeg::Merged(_)])
        })
        .map(|(n, _)| n.clone())
        .collect();
    // The shape this pins: a U2 constituent wrapping a name U1 minted.
    assert!(
        merged.iter().any(|m| {
            let [RoleSeg::Merged(cs)] = m.path.as_slice() else {
                return false;
            };
            cs.iter().any(
                |c| matches!(c.path.as_slice(), [RoleSeg::From { of, .. }] if of.node == s.union),
            )
        }),
        "a U2 merge lists U1's constituents re-wrapped: {merged:#?}"
    );
    let (doc2, _) = step(
        doc.clone(),
        DocEdit::SetMembers {
            node: u2,
            members: editor_core::Bodies::Spelled(vec![s.union.into()]),
        },
    );
    let ev2 = run(&doc2, Some(&ev1));
    for m in &merged {
        let res = crate::fixture::resolve_with_prior_at_mint(
            RunCtx {
                doc: &doc2,
                eval: &ev2,
            },
            RunCtx {
                doc: &doc,
                eval: &ev1,
            },
            m,
        );
        let Resolution::Failed(f) = res else {
            panic!("{m:?} no longer resolves at U2: {res:?}");
        };
        let ResolveError::Vanished { diagnosis, .. } = &f.error else {
            panic!("expected Vanished for {m:?}, got {:?}", f.error);
        };
        assert_eq!(
            *diagnosis,
            Diagnosis::RecipeEdit {
                edit: RecipeEditRef::NodeChanged { node: u2 }
            },
            "{m:?}: the cause is the SetMembers at U2, not a name U1 retired"
        );
    }
}

// ---- Vanished: a family's count shrink ----

#[test]
fn family_count_shrink_vanishes_the_member_it_drops_without_a_flip() {
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, k, pattern) = crate::fixture::linear(doc, body, [1.0, 0.0, 0.0], 2.0, 3);
    let ev1 = run(&doc, None);
    let master_cap = minted(
        EntityKind::Face,
        body,
        RoleSeg::Cap(editor_core::CapEnd::End),
    );
    let inst2 = fixture::in_copy(pattern, k, 2, master_cap);
    assert!(matches!(
        crate::fixture::resolve_at_mint(
            RunCtx {
                doc: &doc,
                eval: &ev1
            },
            &inst2
        ),
        Resolution::Resolved(_)
    ));
    let (doc2, _) = step(
        doc.clone(),
        fixture::recount_edit(&doc, k, editor_core::Formula::count(2)),
    );
    let ev2 = run(&doc2, Some(&ev1));
    let res = crate::fixture::resolve_with_prior_at_mint(
        RunCtx {
            doc: &doc2,
            eval: &ev2,
        },
        RunCtx {
            doc: &doc,
            eval: &ev1,
        },
        &inst2,
    );
    let Resolution::Failed(f) = res else {
        panic!("expected Failed, got {res:?}");
    };
    let ResolveError::Vanished { diagnosis, .. } = &f.error else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    assert!(
        !matches!(diagnosis, Diagnosis::PredicateFlip { .. }),
        "a count shrink is the recipe's edit, never a flip: {diagnosis:?}"
    );
}

// ---- Vanished: Cascade through a vanished operand name ----

#[test]
fn member_of_vanished_master_name_diagnoses_cascade() {
    let s = slide_union(0.5);
    // A row of the union, so its names wrap the union's.
    let (doc, k, pattern) = fixture::linear(s.doc.clone(), s.union, [0.0, 1.0, 0.0], 5.0, 2);
    let ev1 = run(&doc, None);
    let master = joined_rim_name(&ev1, s.union);
    let inst = fixture::in_copy(pattern, k, 1, master.clone());
    assert!(matches!(
        crate::fixture::resolve_at_mint(
            RunCtx {
                doc: &doc,
                eval: &ev1
            },
            &inst
        ),
        Resolution::Resolved(_)
    ));
    // Slide B disjoint: the master's joined rim vanishes, so the
    // instance name vanishes THROUGH it.
    let doc2 = slide_placement(doc.clone(), s.placement, 2.5);
    let ev2 = run(&doc2, Some(&ev1));
    let res = crate::fixture::resolve_with_prior_at_mint(
        RunCtx {
            doc: &doc2,
            eval: &ev2,
        },
        RunCtx {
            doc: &doc,
            eval: &ev1,
        },
        &inst,
    );
    let Resolution::Failed(f) = res else {
        panic!("expected Failed, got {res:?}");
    };
    let ResolveError::Vanished { diagnosis, .. } = &f.error else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    assert_eq!(
        *diagnosis,
        Diagnosis::Cascade {
            through: master.clone()
        },
        "the upstream vanish carries the root cause"
    );
    // And the through-name's own resolution chains to the flip.
    let res_master = crate::fixture::resolve_with_prior_at_mint(
        RunCtx {
            doc: &doc2,
            eval: &ev2,
        },
        RunCtx {
            doc: &doc,
            eval: &ev1,
        },
        &master,
    );
    let Resolution::Failed(fm) = res_master else {
        panic!("expected Failed, got {res_master:?}");
    };
    assert!(matches!(
        fm.error,
        ResolveError::Vanished {
            diagnosis: Diagnosis::PredicateFlip { .. },
            ..
        }
    ));
}

// ---- Indeterminate: failed and poisoned targets ----

#[test]
fn failed_and_poisoned_targets_resolve_indeterminate_not_vanished() {
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 1.0);
    let (doc, b) = fixture::slid(doc, a, b, [0.0; 3]);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    // Zero the A extrude's distance: A fails, the union poisons.
    let (doc2, _) = step(
        doc,
        DocEdit::SetParam {
            node: a,
            slot: SlotId::Distance,
            value: len(0.0).into(),
            fresh: Vec::new(),
        },
    );
    let ev = run(&doc2, None);
    let ctx = RunCtx {
        doc: &doc2,
        eval: &ev,
    };
    let cap_a = minted(EntityKind::Face, a, RoleSeg::Cap(CapEnd::End));
    assert_eq!(
        crate::fixture::resolve_at_mint(ctx, &cap_a),
        Resolution::Indeterminate(ResolveIndeterminate {
            standing: NodeStanding::Failed { node: a }
        })
    );
    let union_body = minted(EntityKind::Body, u, RoleSeg::OutputBody);
    assert_eq!(
        crate::fixture::resolve_at_mint(ctx, &union_body),
        Resolution::Indeterminate(ResolveIndeterminate {
            standing: NodeStanding::Poisoned {
                node: u,
                through: a
            }
        })
    );
}

// ---- The rebind suggestion ladder (the D9 operand→final gap) ----

#[test]
fn rebind_suggestions_offer_wrapping_derivations() {
    let s = slide_union(0.5);
    let ev = run(&s.doc, None);
    let cap = minted(EntityKind::Face, s.a, RoleSeg::Cap(CapEnd::End));
    let suggestions = rebind_suggestions(&ev, &cap);
    // M4 PR 5 (N3 live): the FromA(cap) wrap retired into the Merged
    // row — the suggestion ladder offers the MERGED name (whose
    // constituents embed the cap's wrap); nothing is followed
    // automatically — these are Rebind candidates only.
    let wrapped = minted(
        EntityKind::Face,
        s.union,
        RoleSeg::From {
            read: s.a_read,
            of: cap.into(),
        },
    );
    assert!(
        suggestions.iter().any(|n| matches!(
            n.path.first(),
            Some(RoleSeg::Merged(cs)) if cs.contains(&wrapped)
        )),
        "expected the Merged row embedding the FromA wrap among suggestions: {suggestions:?}"
    );
}

// ---- R6: name-level edit-time validation (banked from PR 3) ----

/// **`apply_with_names` holds a payload name to the evaluation it is
/// given.** A fillet's selection is checkable exactly when its minting
/// node evaluated `Ok`, so a typo role on an evaluated node is refused
/// here rather than surviving to the fillet's own resolution.
#[test]
fn apply_with_names_checks_a_fillet_selection_under_the_same_rule() {
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve_fillet_door", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let ev = run(&doc, None);
    let rim = minted(
        EntityKind::Edge,
        a,
        RoleSeg::RimEdge(CapEnd::End, crate::fixture::piece(&doc, a, 0, 0).into()),
    );
    assert!(
        apply_with_names(
            &doc,
            &DocEdit::InsertNode {
                node: Box::new(Node::fillet(a, len(0.1), vec![rim.clone()])),
                fresh: Vec::new()
            },
            &ev,
            Tol::witness()
        )
        .is_ok(),
        "a selection the tables carry passes"
    );
    let bogus = minted(
        EntityKind::Edge,
        a,
        RoleSeg::RimEdge(CapEnd::End, crate::fixture::no_piece().into()),
    );
    let err = apply_with_names(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::fillet(a, len(0.1), vec![bogus.clone()])),
            fresh: Vec::new(),
        },
        &ev,
        Tol::witness(),
    )
    .unwrap_err();
    assert_eq!(
        err,
        editor_core::EditError::NameUnresolvedInEvaluation {
            name: doc.spoken_name(&bogus)
        }
    );
}

// ---- Review Finding 2: suggestions are structural wraps only, ----
// ---- kind-filtered (adopted reviewer probe, inverted to a pin) ----

/// Whether a walk counts discriminator PARTNERS (a piece's walls) as
/// occurrences of a name.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Partners {
    /// Count partner positions: any mention at all.
    Include,
    /// Structural embedding only — a derivation OF the name.
    Skip,
}

/// True iff `needle` occurs anywhere under `hay`, counting
/// discriminator partners iff `partners` says so.
///
/// STRICTLY under: `hay` itself is not an occurrence of `needle`, so
/// `occurs(n, n, _)` is false unless a name contains itself, which no
/// name does. The one caller subtracts the two answers, where the
/// self-case cancels either way.
///
/// This is the test's OWN reading of the role vocabulary, spelled out
/// rather than borrowed from `resolve`'s walker: an oracle that asks
/// the code under test what the answer is pins nothing.
///
/// The match is EXHAUSTIVE, and that is what keeps it an oracle. A
/// role segment added to the vocabulary must be classified here —
/// embedding, discrimination, or neither — before this suite
/// compiles. Under a catch-all a new name-carrying segment reads as
/// "no occurrence", so [`only_wall_mention`] would report NO
/// PHANTOM for a phantom of exactly the new shape, and the row below
/// would pass while the property it names had failed.
fn occurs(hay: &StableName, needle: &StableName, partners: Partners) -> bool {
    fn sensed(sense: &editor_core::Sense, under: &dyn Fn(&StableName) -> bool) -> bool {
        match sense {
            editor_core::Sense::Through { entering } => entering.iter().any(|n| under(n)),
            editor_core::Sense::Enters
            | editor_core::Sense::Leaves
            | editor_core::Sense::Outside
            | editor_core::Sense::Inside => false,
        }
    }
    let under = |n: &StableName| n == needle || occurs(n, needle, partners);
    hay.path.iter().any(|seg| match seg {
        // One embedded operand name: the entity derives from it.
        RoleSeg::From { of: x, .. }
        | RoleSeg::SectionEdge { face: x, .. }
        | RoleSeg::SplitFragment { parent: x, .. }
        | RoleSeg::OnToolVertex { of: x, .. }
        | RoleSeg::Member { of: x, .. }
        | RoleSeg::BlendFace(x)
        | RoleSeg::CornerFace(x)
        | RoleSeg::Mitre { vertex: x }
        | RoleSeg::TurnFoot { vertex: x }
        | RoleSeg::BandTrim { edge: x, .. }
        | RoleSeg::BandFoot(x)
        | RoleSeg::BandCut(x)
        | RoleSeg::Inner(x)
        | RoleSeg::Rim(x)
        | RoleSeg::HoleRim { of: x, .. } => under(x),
        // A crossing: its line, its face, and the things its sense
        // names on the line.
        RoleSeg::CrossingVertex { edge, sense } => under(edge) || sensed(sense, &under),
        RoleSeg::Crossing { edge, face, sense } => {
            under(edge) || under(face) || sensed(sense, &under)
        }
        // Two.
        RoleSeg::Seam { a: x, b: y }
        | RoleSeg::TrimEdge {
            edge: x,
            support: y,
        }
        | RoleSeg::FootVertex {
            vertex: x,
            support: y,
        }
        | RoleSeg::EndArc { vertex: x, edge: y } => under(x) || under(y),
        // A set.
        RoleSeg::EdgeCrossing(run) => run
            .iter()
            .any(|l| under(&l.line) || sensed(&l.sense, &under)),
        RoleSeg::Merged(v) | RoleSeg::BandFace(v) => v.iter().any(under),
        // A source edge (derivation) and the band that crossed or slit
        // it (a discriminator, like a piece's wall).
        RoleSeg::BandCross { edge, band } | RoleSeg::BandSlit { edge, band } => {
            under(edge) || (partners == Partners::Include && band.iter().any(under))
        }
        // Discrimination, not derivation: the fragment is classified
        // AGAINST these, not built from them.
        RoleSeg::Fragment(Qualifier::Borders(v) | Qualifier::Keeps(v) | Qualifier::Ends(v)) => {
            partners == Partners::Include && v.iter().any(under)
        }
        // Segments that embed no name.
        RoleSeg::OutputBody
        | RoleSeg::Cap(_)
        | RoleSeg::Lateral(_)
        | RoleSeg::RimEdge(..)
        | RoleSeg::LateralEdge(_)
        | RoleSeg::CapVertex(..)
        | RoleSeg::Band(_)
        | RoleSeg::BandRim(_)
        | RoleSeg::BandRimPi(_)
        | RoleSeg::BandPi(_)
        | RoleSeg::Meridian(..)
        | RoleSeg::MeridianVertex(..)
        | RoleSeg::RevolveCap(_)
        | RoleSeg::Pole(_)
        | RoleSeg::AxisEdge(_)
        | RoleSeg::SplitBody
        | RoleSeg::SectionFace
        | RoleSeg::LoftWall(_)
        | RoleSeg::LoftSeam(_)
        | RoleSeg::LoftSlabWall(..)
        | RoleSeg::LoftSlabSeam(..)
        | RoleSeg::LoftSectionEdge(..)
        | RoleSeg::LoftSectionVertex(..) => false,
    })
}

/// True iff `needle` occurs in `hay`'s path ONLY as a `Borders` wall (never as a structural embedding) — the
/// reviewer's phantom detector.
fn only_wall_mention(hay: &StableName, needle: &StableName) -> bool {
    !occurs(hay, needle, Partners::Skip) && occurs(hay, needle, Partners::Include)
}

/// The detector answers about a name reached through a segment its
/// FIRST vocabulary knew nothing about.
///
/// This row is the detector's own pin, and it is here because the
/// detector shipped for a year reading three groups of segments and
/// sweeping the rest into a catch-all. Everything the fillet emitter
/// mints — this row's `BlendFace` among them — was in that catch-all,
/// so a phantom wrapped in one read as no mention at all and the
/// suggestion row above passed by not looking.
///
/// The shape is the one that row cares about: a name that mentions
/// `needle` ONLY as a `Borders` wall, one derivation step below the
/// surface. It is a phantom, and saying so requires descending
/// through the blend segment — which is why a detector blind to that
/// segment reports the opposite.
#[test]
fn the_phantom_detector_sees_through_the_whole_vocabulary() {
    let needle = fixture::fname(RecipeNodeId::new(0, 1), RoleSeg::Cap(CapEnd::End));
    let partner_only = StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId::new(0, 2),
        path: vec![RoleSeg::Fragment(Qualifier::Borders(vec![needle.clone()]))],
    };
    let blended = fixture::fname(
        RecipeNodeId::new(0, 3),
        RoleSeg::BlendFace(partner_only.clone().into()),
    );

    assert!(
        only_wall_mention(&partner_only, &needle),
        "a bare wall mention is the phantom shape itself"
    );
    assert!(
        only_wall_mention(&blended, &needle),
        "a phantom stays a phantom under a blend segment — a detector \
         that cannot read the segment calls this NO MENTION and lets \
         the suggestion row through"
    );
    // The same segment, carrying the needle structurally: a real
    // derivation, and the detector must not call it a phantom.
    let derived = fixture::fname(
        RecipeNodeId::new(0, 3),
        RoleSeg::BlendFace(needle.clone().into()),
    );
    assert!(
        !only_wall_mention(&derived, &needle),
        "a blend OF the name is a derivation, not a phantom"
    );
}

#[test]
fn suggestions_never_offer_wall_phantoms_and_are_kind_filtered() {
    // The reviewer's band-cut rig: the subtract mints `Borders`-qualified
    // cap pieces whose walls are the cutter's walls as the subtract
    // carries them in, each the `From` of a BARE operand name — exactly
    // the shape a user paints. Suggestions for a
    // painted wall must be derivations WRAPPING it, never pieces of the
    // OTHER body that merely border it, and never a kind Rebind
    // refuses.
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, _a) = block(doc, (0.0, 4.0), (0.0, 4.0), 1.0);
    let (doc, bp) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (-2.5, 1.0),
            (2.0, 1.0),
            (4.5, 0.8),
            (4.5, 0.9),
            (2.0, 1.1),
            (-2.5, 1.1),
        ]],
    );
    let (doc, band) = insert(
        doc,
        Node::Extrude {
            profile: bp.into(),
            distance: len(2.0),
        },
    );
    let (doc, tr) = fixture::slid(doc, _a, band, [0.0, 0.0, -0.5]);
    let (doc, sub) = insert(
        doc,
        Node::Subtract {
            from: _a.into(),
            tool: tr.into(),
        },
    );
    let ev = run(&doc, None);
    // A wall recorded in some piece's `Borders` set.
    let partner: StableName = ev
        .table_at(sub)
        .expect("subtract evaluates")
        .iter()
        .find_map(|(n, e)| {
            if !matches!(e, Entry::Unique(_) | Entry::Tied(_)) {
                return None;
            }
            n.path.iter().find_map(|seg| match seg {
                RoleSeg::Fragment(Qualifier::Borders(v)) => match v.first()?.path.as_slice() {
                    [RoleSeg::From { of, .. }] => Some((**of).clone()),
                    _ => None,
                },
                _ => None,
            })
        })
        .expect("band cut mints Borders-qualified pieces");
    let suggestions = rebind_suggestions(&ev, &partner);
    assert!(
        !suggestions.is_empty(),
        "the true structural wraps are still offered"
    );
    for s in &suggestions {
        assert_eq!(
            s.kind, partner.kind,
            "cross-kind suggestion (Rebind refuses these): {s:?}"
        );
        assert!(
            !only_wall_mention(s, &partner),
            "WALL-ONLY phantom offered as a suggestion: {s:?}"
        );
    }
}

// ---- Review Finding 3: Diagnosis::RecipeEdit constructed for a ----
// ---- real recipe edit; the single-run no-prior Vanished path ----

#[test]
fn repointed_input_diagnoses_recipe_edit_on_path() {
    // Two geometrically IDENTICAL operands b and c — c reads b's own
    // variables, which is how two operands are one geometry by
    // structure (two separately typed equal values are two variables,
    // whose content keys differ): re-pointing the
    // union's second member from b to c (`SetMembers`, the door that
    // re-points a node's inputs in place) changes NO verdict (the
    // computed geometry is bit-identical) and NO structural parameter
    // — the only honest evidence is the recipe edit at the union
    // node, and it is on the vanished name's path.
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, a) = block(doc, (0.0, 2.0), (0.0, 2.0), 1.0);
    // General position (no coplanar planes with A): B pierces A's
    // slab, strictly inside in y, poking out above and below.
    let (doc, b0) = block(doc, (1.0, 3.0), (0.5, 1.5), 2.0);
    let (doc, b) = fixture::slid(doc, a, b0, [0.0, 0.0, -0.5]);
    // The fold's predicate population reads its members' faces in name
    // order, and a member's names carry its node id: a twin placed on
    // the other side of `a` from `b` runs the same geometry through
    // other predicates (`point_in_loop_advance` flips, 8 of them). So
    // the twin is minted and placed until it sorts where `b` does.
    let twin_on = |doc: ProfileDoc| {
        let (doc, c0) = twin(doc, b0);
        fixture::slid(doc, a, c0, [0.0, 0.0, -0.5])
    };
    let (mut doc, mut c) = twin_on(doc);
    while (c < a) != (b < a) {
        (doc, c) = twin_on(doc);
    }
    let (doc1, bl) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    let ev1 = run(&doc1, None);
    // The union carries B's end cap as its member's.
    let member_cap = |m: RecipeNodeId| StableName {
        kind: EntityKind::Face,
        node: bl,
        path: vec![RoleSeg::From {
            read: fixture::out(&doc1, m),
            of: minted(
                EntityKind::Face,
                placed_body(&doc1, m),
                RoleSeg::Cap(CapEnd::End),
            )
            .into(),
        }],
    };
    let target = member_cap(b);
    assert!(
        matches!(
            crate::fixture::resolve_at_mint(
                RunCtx {
                    doc: &doc1,
                    eval: &ev1
                },
                &target
            ),
            Resolution::Resolved(_)
        ),
        "the union derives b's cap before the re-point"
    );
    let (doc2, _) = step(
        doc1.clone(),
        DocEdit::SetMembers {
            node: bl,
            members: editor_core::Bodies::Spelled(vec![a.into(), c.into()]),
        },
    );
    // #95 disposition 2 LANDED (M4 PR 5): the memo-TRANSFERRED run
    // honestly re-derives the union's naming half — the recursive
    // naming key includes input node ids, so the b→c re-point misses
    // the memo even though the twins are bit-identical. Pinned WITH
    // memo transfer.
    let ev2 = run(&doc2, Some(&ev1));
    let res = crate::fixture::resolve_with_prior_at_mint(
        RunCtx {
            doc: &doc2,
            eval: &ev2,
        },
        RunCtx {
            doc: &doc1,
            eval: &ev1,
        },
        &target,
    );
    let Resolution::Failed(f) = res else {
        panic!("expected Failed, got {res:?}");
    };
    let ResolveError::Vanished {
        diagnosis,
        last_good,
        ..
    } = &f.error
    else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    assert_eq!(
        *diagnosis,
        Diagnosis::RecipeEdit {
            edit: RecipeEditRef::NodeChanged { node: bl }
        },
        "a re-pointed input is a recipe edit on the path — no flip, \
         no structural param to blame"
    );
    assert!(last_good.is_some(), "the prior run resolved the name");
    // The positive half of the #95 pin: the re-derived table carries
    // c's cap — the value the recipe actually denotes.
    assert!(
        matches!(
            crate::fixture::resolve_at_mint(
                RunCtx {
                    doc: &doc2,
                    eval: &ev2
                },
                &member_cap(c)
            ),
            Resolution::Resolved(_)
        ),
        "the memo-transferred run must derive c's cap"
    );
}

/// The #95 GRANDPARENT pin (the reason disposition 2's key is
/// RECURSIVE): re-point X's INPUT to a bit-identical twin two hops
/// above N — X's content key is unchanged at N's doorstep (N's direct
/// input is X either way), so a one-level context check would reuse
/// N's stale names; the recursive naming key composes X's change
/// through and N re-derives, embedding the twin's re-derived names.
/// X is a union, whose members `SetMembers` re-points in place, so X
/// and N keep their ids across the re-point.
#[test]
fn grandparent_repoint_rederives_the_grandchild_names() {
    use editor_core::{NodeResult, ValuePayload};
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, twin) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, d) = block(doc, (5.0, 6.0), (0.0, 1.0), 1.0);
    // `c` and `d` join `b`'s space where they stand; a copy carries its
    // source's names, so `c`'s are the twin's.
    let (doc, c) = fixture::slid(doc, b, twin, [0.0; 3]);
    let (doc, d) = fixture::slid(doc, b, d, [0.0; 3]);
    let (doc, x) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![b.into(), d.into()]),
        },
    );
    let (doc1, _, n) = fixture::linear(doc, x, [0.0, 1.0, 0.0], 2.0, 2);
    let ev1 = run(&doc1, None);
    let (doc2, _) = step(
        doc1,
        DocEdit::SetMembers {
            node: x,
            members: editor_core::Bodies::Spelled(vec![c.into(), d.into()]),
        },
    );
    let ev2 = run(&doc2, Some(&ev1));
    // The grandchild's table must speak C's names now (the family
    // wraps the union's rows, whose member edge is the twin).
    let table = match ev2.nodes.get(&n) {
        Some(NodeResult::Ok(v)) => {
            assert!(
                matches!(v.payload, ValuePayload::Instances(_)),
                "grandchild is its members"
            );
            ev2.table_at(n).expect("the grandchild's table")
        }
        other => panic!("grandchild must evaluate, got {other:?}"),
    };
    let mentions = |m: RecipeNodeId| {
        table
            .iter()
            .any(|(name, _)| editor_core::derivation_nodes(name).contains(&m))
    };
    assert!(
        mentions(twin),
        "the memo-transferred grandchild table must embed the twin's names"
    );
    assert!(
        !mentions(b),
        "no stale name may survive the grandparent re-point"
    );
}

#[test]
fn single_run_vanished_falls_back_to_cause_not_in_evidence() {
    // No prior run, no cascade, no recorded qualifier delta: the
    // documented total fallback — the recorded reference disagrees
    // with the recipe as it stands, the cause not in evidence.
    let doc = ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, k, pattern) = crate::fixture::linear(doc, body, [1.0, 0.0, 0.0], 2.0, 2);
    let ev = run(&doc, None);
    // Member 5 was never minted by this 2-member family.
    let master_cap = minted(
        EntityKind::Face,
        body,
        RoleSeg::Cap(editor_core::CapEnd::End),
    );
    let inst5 = fixture::in_copy(pattern, k, 5, master_cap);
    let res = crate::fixture::resolve_at_mint(
        RunCtx {
            doc: &doc,
            eval: &ev,
        },
        &inst5,
    );
    let Resolution::Failed(f) = res else {
        panic!("expected Failed, got {res:?}");
    };
    let ResolveError::Vanished {
        diagnosis,
        last_good,
        ..
    } = &f.error
    else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    assert_eq!(
        *diagnosis,
        Diagnosis::RecipeEdit {
            edit: RecipeEditRef::NodeChanged { node: pattern }
        }
    );
    assert!(last_good.is_none(), "no prior run, no tombstone");
    assert!(f.offers.is_empty());
}

// ---- Review Finding 1 ruling: the qualifier-delta rung, for a ----
// ---- face piece the border delta ----

/// A body-kind piece name `[From(read, f), Fragment(Borders(walls))]`
/// at `node` — the hand-built shape for the border-delta pins, `read`
/// an arbitrary fixed operand read (the evaluation is hand-built, so no
/// document holds it).
fn piece(node: RecipeNodeId, f: &StableName, walls: &[&StableName]) -> StableName {
    StableName {
        kind: EntityKind::Body,
        node,
        path: vec![
            RoleSeg::From {
                read: editor_core::VarId::new(0, 1),
                of: f.clone().into(),
            },
            RoleSeg::Fragment(Qualifier::Borders(
                walls.iter().map(|&w| w.clone()).collect(),
            )),
        ],
    }
}

/// A node with no inputs and no evaluated body, for a doc whose
/// evaluation is hand-built.
fn leaf() -> AuthoredNode {
    Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, 0.5)]))
}

/// One-node hand-built evaluation whose table is `t` (the over-tie
/// pin's construction, reused).
fn one_node_eval(
    document: editor_core::DocumentId,
    node: RecipeNodeId,
    t: NameTable,
) -> Evaluation<f64> {
    let mut nodes = std::collections::BTreeMap::new();
    nodes.insert(
        node,
        editor_core::NodeResult::Ok(editor_core::NodeValue::hand_built(
            t,
            editor_core::FragmentGroups::default(),
            vec![],
        )),
    );
    Evaluation::<f64>::hand_built(document, vec![node], nodes, Tol::witness())
}

fn body_ent(i: u32) -> editor_core::EntityRef {
    editor_core::EntityRef {
        body: i,
        key: editor_core::EntityKey::Body,
    }
}

#[test]
fn border_delta_reads_the_walls_off_the_names_without_any_flip_set_evidence() {
    // The change is recorded IN the names: the old piece borders P, and
    // the piece that still borders P now borders S as well. Both runs
    // have EMPTY verdict logs and the doc is UNCHANGED — the
    // diff-engine and doc-diff lanes have nothing (the
    // population-cancel shape), yet the diagnosis names the wall that
    // moved.
    let (mut doc, n) = insert(
        ProfileDoc::empty_derived("m4_pr4_resolve", Tol::witness()),
        leaf(),
    );
    let mut walls = Vec::new();
    for _ in 0..7 {
        let (d, at) = insert(doc, leaf());
        doc = d;
        walls.push(minted(EntityKind::Body, at, RoleSeg::OutputBody));
    }
    let [p, q, r, s, t, r2, r3] = <[StableName; 7]>::try_from(walls).unwrap();
    let f = minted(EntityKind::Body, n, RoleSeg::OutputBody);
    let old_name = piece(n, &f, &[&p]);
    let table = |pieces: &[StableName]| {
        let mut tb = NameTable::new();
        for (i, name) in pieces.iter().enumerate() {
            tb.insert(name.clone(), body_ent(i as u32)).unwrap();
        }
        tb.insert(f.clone(), body_ent(10)).unwrap();
        tb
    };
    let eval = |pieces: &[StableName]| one_node_eval(doc.id(), n, table(pieces));
    let diagnosis_of = |res: Resolution| {
        let Resolution::Failed(fail) = res else {
            panic!("expected Failed, got {res:?}");
        };
        let ResolveError::Vanished {
            diagnosis,
            last_good,
            ..
        } = fail.error
        else {
            panic!("expected Vanished, got {:?}", fail.error);
        };
        (diagnosis, last_good)
    };
    let single = |ev: &Evaluation<f64>| {
        diagnosis_of(crate::fixture::resolve_at_mint(
            RunCtx {
                doc: &doc,
                eval: ev,
            },
            &old_name,
        ))
    };
    let with_prior = |ev: &Evaluation<f64>, prior: &Evaluation<f64>| {
        diagnosis_of(crate::fixture::resolve_with_prior_at_mint(
            RunCtx {
                doc: &doc,
                eval: ev,
            },
            RunCtx {
                doc: &doc,
                eval: prior,
            },
            &old_name,
        ))
    };
    let fallback = Diagnosis::RecipeEdit {
        edit: RecipeEditRef::NodeChanged { node: n },
    };

    // The untouched sibling {Q, R} borders none of the vanished walls,
    // so it is no counterpart; the piece bordering P and S is.
    let prior_ev = eval(&[old_name.clone(), piece(n, &f, &[&q, &r])]);
    let new_ev = eval(&[piece(n, &f, &[&p, &s]), piece(n, &f, &[&q, &r])]);
    let delta = Diagnosis::BorderDelta {
        node: n,
        gone: vec![],
        new: vec![s.clone()],
    };
    // Single-run: the rung is the FIRST evidence (no prior at all).
    let (d, last_good) = single(&new_ev);
    assert_eq!(d, delta);
    assert!(last_good.is_none());
    // With-prior, empty FlipSet (both logs empty), unchanged doc: every
    // earlier lane is silent; the rung still fires, and the tombstone
    // rides from the prior run.
    let (d, last_good) = with_prior(&new_ev, &prior_ev);
    assert_eq!(d, delta);
    let tomb = last_good.expect("the prior run resolved the name");
    assert_eq!(tomb.patch.node, n);
    assert_eq!(tomb.patch.entity, body_ent(0));

    // Two pieces equally near are no one counterpart: the rung does not
    // pick, and the fallback names the site without claiming an edit.
    let (d, _) = single(&eval(&[piece(n, &f, &[&p, &s]), piece(n, &f, &[&p, &t])]));
    assert_eq!(
        d, fallback,
        "two equally near pieces: choosing one would be a guess"
    );

    // A piece that borders none of the vanished walls is another piece,
    // however near its set is: no counterpart, no answer.
    let (d, _) = single(&eval(&[piece(n, &f, &[&q])]));
    assert_eq!(
        d, fallback,
        "a piece sharing no wall is not the vanished one changed"
    );

    // The review's probe: {P} vanishes beside a sibling {P, R, R2, R3}
    // that the last-good run already published, and a new piece {Q}.
    // {Q} shares no wall, and the sibling is untouched, so with the
    // prior run the rung declines; it never answers `new: [Q]`.
    let sibling = piece(n, &f, &[&p, &r, &r2, &r3]);
    let prior_ev = eval(&[old_name.clone(), sibling.clone()]);
    let new_ev = eval(&[sibling, piece(n, &f, &[&q])]);
    let (d, _) = with_prior(&new_ev, &prior_ev);
    assert_eq!(d, fallback, "an untouched sibling is not the counterpart");
    let (d, _) = single(&new_ev);
    assert!(
        !matches!(&d, Diagnosis::BorderDelta { new, .. } if new.contains(&q)),
        "a piece sharing no wall is never the counterpart: {d:?}"
    );
}
