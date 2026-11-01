//! M4 PR 4, spec D9: the appearance hook. PR 7's coarse
//! `AppearanceLossCause` rows enriched onto the N5 ladder
//! (`enrich_appearance_loss[_with_prior]` — Ambiguous by table lookup
//! at the recorded site, Vanished through the diagnosis ladder with
//! offers riding NEXT TO the verbatim error, NodeGone with the
//! derived edit, indeterminate arms preserved), and the BANKED
//! operand→final repair ergonomics (PR 7 review A1): every
//! appearance-carrying name gets wrapping-derivation Rebind
//! suggestions, and `Rebind` MOVES the appearance key — with a typed
//! per-kind collision refusal.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::NodeStanding;
use editor_core::{
    AppearanceLossCause, Attr, AttrKind, CancelToken, CapEnd, Diagnosis, DocEdit, EditError,
    EntityKind, EvalOptions, Evaluation, Node, ProfileDoc, RecipeNodeId, Resolution, ResolveError,
    ResolveIndeterminate, Rgba8, RoleSeg, RunCtx, SlotId, StableName,
    appearance_rebind_suggestions, enrich_appearance_loss, enrich_appearance_loss_with_prior,
    evaluate,
};
use fixture::{insert, len, minted, on_frame, step};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn rerun(doc: &editor_core::ProfileDoc, prior: &Evaluation<f64>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        Some(prior),
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn red() -> Attr {
    Attr::Color(Rgba8::opaque(200, 30, 30))
}

fn set(doc: ProfileDoc, name: StableName, attr: Attr) -> ProfileDoc {
    let read = crate::fixture::painted_in(&doc, &name);
    step(
        doc,
        DocEdit::SetAppearance {
            body: read,
            name,
            attr,
        },
    )
    .0
}

/// A rectangular block: profile on its own xy, extruded `dz`.
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

/// [`block`] placed against `target`, `z0` up its own z: the
/// placement, whose copy a combine with `target` reads.
fn block_on(
    doc: ProfileDoc,
    target: RecipeNodeId,
    xs: (f64, f64),
    ys: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, b) = block(doc, xs, ys, dz);
    crate::fixture::slid(doc, target, b, [0.0, 0.0, z0])
}

/// The PR 3 genuine-tie fixture: a U cutter through one wall; both
/// B-cap prong fragments tie. Returns (doc, subtract node).
fn tie_fixture() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("m4_pr4_appearance_hook", Tol::witness());
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
    let (doc, b) = crate::fixture::slid(doc, a, b, [0.0, 0.0, 1.0]);
    let (doc, sub) = insert(
        doc,
        Node::Subtract {
            from: a.into(),
            tool: b.into(),
        },
    );
    (doc, sub)
}

/// The operand→final gap fixture (PR 7 review A1): operand `a`'s top
/// cap painted, `a` consumed by a union. Returns (doc, a, uni, cap).
fn gap_fixture() -> (ProfileDoc, RecipeNodeId, RecipeNodeId, StableName) {
    let doc = ProfileDoc::empty_derived("m4_pr4_appearance_hook", Tol::witness());
    // Decoupled overlap (M4 PR 5: the original y/z-flush variant now
    // demands a Declare and MERGES the caps — the A1 ergonomics this
    // fixture pins are the plain wrapping-derivation kind).
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (0.5, 1.5), (0.25, 0.75), 0.25, 1.25);
    let (doc, uni) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    let cap = minted(EntityKind::Face, a, RoleSeg::Cap(CapEnd::End));
    let doc = set(doc, cap.clone(), red());
    (doc, a, uni, cap)
}

// ---- Enrichment: the per-arm mapping (spec D9) ----

#[test]
fn ambiguous_loss_enriches_by_table_lookup_at_the_recorded_site() {
    let (doc, sub) = tie_fixture();
    let ev = run(&doc);
    let tied = ev
        .table_at(sub)
        .unwrap()
        .iter()
        .find_map(|(n, e)| {
            matches!(e, editor_core::Entry::Tied(c) if c.len() == 2).then(|| n.clone())
        })
        .expect("the U-cutter fixture ties");
    let doc = set(doc, tied.clone(), red());
    let ev = run(&doc);
    assert_eq!(ev.appearance.losses.len(), 1);
    let loss = &ev.appearance.losses[0];
    assert_eq!(
        loss.cause,
        AppearanceLossCause::Ambiguous { at: sub, width: 2 }
    );

    let r = enrich_appearance_loss(
        RunCtx {
            doc: &doc,
            eval: &ev,
        },
        loss,
    );
    let Resolution::Failed(f) = r else {
        panic!("expected Failed, got {r:?}");
    };
    let ResolveError::Ambiguous {
        name,
        candidates,
        tie,
    } = &f.error
    else {
        panic!("expected Ambiguous, got {:?}", f.error);
    };
    // Derived by table.lookup(name) at the recorded node: the tie row
    // itself is the candidate set expressed in names; the witness
    // carries the recorded site and the looked-up width.
    assert_eq!(*name, tied);
    assert_eq!(*candidates, vec![tied.clone()]);
    assert_eq!(tie.node, sub);
    assert_eq!(tie.at, tied);
    assert_eq!(tie.width, 2);
    assert!(f.offers.is_empty());
}

/// **A tie a pass-through table carries is reported at the table that
/// defined it**: `at` is the first carrying table in evaluation order.
/// The fixture carries the defining node's name downstream through a
/// split above the tie, whose lower half passes it through unchanged
/// and so carries the tie too: a walk that took the last carrying
/// table would report the split.
#[test]
fn a_carried_tie_is_reported_at_its_defining_table() {
    let (doc, sub) = tie_fixture();
    let ev = run(&doc);
    let tied = ev
        .table_at(sub)
        .unwrap()
        .iter()
        .find_map(|(n, e)| {
            matches!(e, editor_core::Entry::Tied(c) if c.len() == 2).then(|| n.clone())
        })
        .expect("the U-cutter fixture ties");
    let tool = crate::fixture::plane_on(&doc, sub, [0.0, 0.0, 3.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: sub.into(),
            tool,
        },
    );
    let doc = set(doc, tied.clone(), red());
    let ev = run(&doc);
    assert!(
        crate::fixture::half(&ev, split, editor_core::SplitHalf::Below)
            .iter()
            .any(|(n, e)| *n == tied && matches!(e, editor_core::Entry::Tied(_))),
        "the split's lower half carries the tie"
    );
    let causes: Vec<_> = ev.appearance.losses.iter().map(|l| &l.cause).collect();
    assert_eq!(
        causes,
        [&AppearanceLossCause::Ambiguous { at: sub, width: 2 }],
        "one loss, at the defining table"
    );
}

#[test]
fn node_gone_loss_enriches_with_the_derived_deletion_edit() {
    let (doc, ext) = block(
        ProfileDoc::empty_derived("m4_pr4_appearance_hook", Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        1.0,
    );
    let cap = minted(EntityKind::Face, ext, RoleSeg::Cap(CapEnd::Start));
    // Read in the lower half of a split of the extrude, which passes
    // the cap through unchanged, so the entry's read outlives the
    // name's minting node: the loss is the name's, `NodeGone`.
    let tool = fixture::plane_on(&doc, ext, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: ext.into(),
            tool,
        },
    );
    let below = doc
        .output(split, editor_core::SplitHalf::Below.port())
        .expect("a split defines its lower half");
    let (doc, _) = step(
        doc.clone(),
        DocEdit::SetAppearance {
            body: below,
            name: cap.clone(),
            attr: red(),
        },
    );
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: ext });
    let ev = run(&doc);
    assert_eq!(ev.appearance.losses.len(), 1);
    let loss = &ev.appearance.losses[0];
    assert_eq!(loss.cause, AppearanceLossCause::NodeGone);

    let r = enrich_appearance_loss(
        RunCtx {
            doc: &doc,
            eval: &ev,
        },
        loss,
    );
    let Resolution::Failed(f) = r else {
        panic!("expected Failed, got {r:?}");
    };
    assert_eq!(
        f.error,
        ResolveError::NodeGone {
            name: cap,
            edit: editor_core::RecipeEditRef::NodeDeleted { node: ext },
        }
    );
}

#[test]
fn vanished_loss_with_prior_enriches_diagnosis_and_tombstone() {
    // The index's count 3 -> 2 vanishes member 2's face name; the
    // coarse loss (empty structural candidates) enriches to the pinned
    // diagnosis of the edit that took it plus the last-good tombstone.
    let (doc, ext) = block(
        ProfileDoc::empty_derived("m4_pr4_appearance_hook", Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        1.0,
    );
    let (doc, k, pat) = crate::fixture::linear(doc, ext, [1.0, 0.0, 0.0], 2.0, 3);
    let ev = run(&doc);
    let inst2_face = ev
        .table_at(pat)
        .unwrap()
        .iter()
        .find_map(|(n, _)| {
            (n.kind == EntityKind::Face
                && matches!(n.path.first(), Some(RoleSeg::Member { at, .. }) if at.as_slice() == [(k, 2)]))
            .then(|| n.clone())
        })
        .expect("the family mints member-2 face names");
    let doc = set(doc, inst2_face.clone(), red());
    let prior_doc = doc.clone();
    let prior_ev = run(&prior_doc);
    assert!(prior_ev.appearance.is_lossless());

    let edit = crate::fixture::recount_edit(&doc, k, editor_core::Formula::count(2));
    let (doc, _) = step(doc, edit);
    let ev = rerun(&doc, &prior_ev);
    assert_eq!(ev.appearance.losses.len(), 1);
    let loss = &ev.appearance.losses[0];
    assert_eq!(
        loss.cause,
        AppearanceLossCause::Vanished {
            candidates: Vec::new()
        }
    );

    let r = enrich_appearance_loss_with_prior(
        RunCtx {
            doc: &doc,
            eval: &ev,
        },
        RunCtx {
            doc: &prior_doc,
            eval: &prior_ev,
        },
        loss,
    );
    let Resolution::Failed(f) = r else {
        panic!("expected Failed, got {r:?}");
    };
    let ResolveError::Vanished {
        name,
        diagnosis,
        last_good,
    } = &f.error
    else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    assert_eq!(*name, inst2_face);
    // The count is the index's, a definition and no slot of the
    // family: what took the name is the index's count.
    assert!(
        matches!(diagnosis, Diagnosis::IndexCount { node, .. } if *node == pat),
        "{diagnosis:?}"
    );
    let t = last_good.as_ref().expect("the prior run resolved the name");
    assert_eq!(t.kind, EntityKind::Face);
    assert_eq!(t.patch.node, pat);
    // The coarse structural candidates (empty here) are a subset of
    // the ladder's offers by construction; nothing structural offers
    // itself for a dropped member.
    assert!(f.offers.is_empty());
}

#[test]
fn indeterminate_losses_enrich_to_the_matching_indeterminate_arm() {
    // Poisoned: operand A degenerates, the union is poisoned through
    // it, and enrichment preserves the indeterminate verdict (not
    // Vanished — same vocabulary on both sides of the hook).
    let doc = ProfileDoc::empty_derived("m4_pr4_appearance_hook", Tol::witness());
    // Decoupled overlap (M4 PR 5: coincident planes now demand a
    // Declare; this test wants a plain transversal union).
    let (doc, a) = block(doc, (0.0, 2.0), (0.0, 2.0), 1.0);
    let (doc, b) = block_on(doc, a, (1.0, 3.0), (0.25, 1.75), 0.125, 0.75);
    let (doc, uni) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    let ev = run(&doc);
    let uni_face = ev
        .table_at(uni)
        .unwrap()
        .iter()
        .find_map(|(n, e)| {
            (n.kind == EntityKind::Face
                && n.node == uni
                && matches!(e, editor_core::Entry::Unique(_)))
            .then(|| n.clone())
        })
        .expect("union mints face names");
    let doc = set(doc, uni_face.clone(), red());
    let (doc, _) = step(
        doc,
        DocEdit::SetParam {
            node: a,
            slot: SlotId::Distance,
            value: len(0.0).into(),
            fresh: Vec::new(),
        },
    );
    let ev = run(&doc);
    assert_eq!(ev.appearance.losses.len(), 1);
    let loss = &ev.appearance.losses[0];
    let poisoned = NodeStanding::Poisoned {
        node: uni,
        through: a,
    };
    assert_eq!(loss.cause, AppearanceLossCause::Indeterminate(poisoned));
    assert_eq!(
        enrich_appearance_loss(
            RunCtx {
                doc: &doc,
                eval: &ev
            },
            loss
        ),
        Resolution::Indeterminate(ResolveIndeterminate { standing: poisoned })
    );

    // Canceled: the not-evaluated arm, with the node made explicit.
    let (doc2, ext) = block(
        ProfileDoc::empty_derived("m4_pr4_appearance_hook", Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        1.0,
    );
    let cap = minted(EntityKind::Face, ext, RoleSeg::Cap(CapEnd::End));
    let doc2 = set(doc2, cap, red());
    let cancel = CancelToken::new();
    cancel.cancel();
    let ev2 = evaluate::<f64>(
        &doc2,
        None,
        &cancel,
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert_eq!(ev2.appearance.losses.len(), 1);
    let loss2 = &ev2.appearance.losses[0];
    let unevaluated = NodeStanding::NotEvaluated { node: ext };
    assert_eq!(loss2.cause, AppearanceLossCause::Indeterminate(unevaluated));
    assert_eq!(
        enrich_appearance_loss(
            RunCtx {
                doc: &doc2,
                eval: &ev2
            },
            loss2
        ),
        Resolution::Indeterminate(ResolveIndeterminate {
            standing: unevaluated
        })
    );
}

// ---- The banked obligation: operand→final re-attachment ergonomics ----

/// The union-side offer for `cap`: a face name minted at `uni`
/// wrapping `cap` (`From(cap)`-headed derivation), with the read that
/// holds it.
fn union_suggestion(
    suggestions: &std::collections::BTreeMap<
        editor_core::AppearanceKey,
        Vec<editor_core::AppearanceKey>,
    >,
    cap: &StableName,
    uni: RecipeNodeId,
) -> editor_core::AppearanceKey {
    suggestions
        .iter()
        .find_map(|(key, offers)| (key.name == *cap).then_some(offers))
        .expect("the suggestion map is total over the appearance store")
        .iter()
        .find(|s| s.name.node == uni && s.name.kind == EntityKind::Face)
        .expect("the final node's wrapping derivation is offered")
        .clone()
}

#[test]
fn suggestions_offer_the_final_wrapping_derivation_and_a_reattachment_repairs_the_gap() {
    let (doc, a, uni, cap) = gap_fixture();
    let ev = run(&doc);
    // The gap is silent by design (ruled A1): lossless, painted on
    // the operand node only.
    assert!(ev.appearance.is_lossless());
    assert!(ev.appearance.for_node(a).is_some());
    assert!(ev.appearance.for_node(uni).is_none());

    // The ladder still offers the repair — suggestions cannot be
    // gated on a loss, precisely because the gap raises none.
    let suggestions = appearance_rebind_suggestions(doc.appearance(), &ev);
    let target = union_suggestion(&suggestions, &cap, uni);
    assert!(
        matches!(target.name.path.first(), Some(RoleSeg::From { read, of: inner }) if *read == fixture::out(&doc, a) && **inner == cap),
        "the offered name wraps the painted one: {target:?}"
    );
    assert_eq!(
        target.at,
        fixture::out(&doc, uni),
        "read in the final node's output"
    );

    // The explicit repair moves the paint to another body, so it is a
    // re-attachment and not a rebind (which never moves a key's read):
    // set it at the offered read, clear it at the operand's.
    // Presentation-only — nothing recomputes.
    let apply = |doc: &ProfileDoc, edit| {
        doc.apply(&edit, Tol::witness())
            .expect("a re-attachment edit applies")
    };
    let set_it = apply(
        &doc,
        DocEdit::SetAppearance {
            body: target.at,
            name: target.name.clone(),
            attr: red(),
        },
    );
    let cleared = apply(
        &set_it.doc,
        DocEdit::ClearAppearance {
            body: fixture::out(&doc, a),
            name: cap.clone(),
            kind: AttrKind::Color,
        },
    );
    assert!(!set_it.record.structural && !cleared.record.structural);
    let doc = cleared.doc;
    assert!(crate::fixture::painted(&doc, &cap).is_none());
    assert_eq!(
        doc.appearance_of(target.at, &target.name).unwrap().attrs[&AttrKind::Color],
        red()
    );

    // The paint now lands on the final node.
    let ev = run(&doc);
    assert!(ev.appearance.is_lossless());
    assert!(
        ev.appearance.for_node(uni).is_some(),
        "the re-attached paint resolves on the final body"
    );
}

#[test]
fn appearance_only_rebind_counts_as_a_site_not_no_references() {
    // Before the appearance store landed, this exact edit was refused
    // RebindNoReferences (no Declare pair references `cap`); the key
    // itself is now a site. It keeps the read it is read in: a rebind
    // repairs the name, and an appearance key is a `SitedRef` whose
    // `at` no rebind moves.
    let (doc, a, uni, cap) = gap_fixture();
    let ev = run(&doc);
    let suggestions = appearance_rebind_suggestions(doc.appearance(), &ev);
    let target = union_suggestion(&suggestions, &cap, uni);
    let applied = doc
        .apply(
            &DocEdit::Rebind {
                body: None,
                from: cap,
                to: target.name.clone(),
            },
            Tol::witness(),
        )
        .expect("an appearance key is a rebind site");
    assert!(
        applied
            .doc
            .appearance_of(fixture::out(&doc, a), &target.name)
            .is_some(),
        "the key kept the operand's read"
    );
}

#[test]
fn rebind_appearance_collision_is_refused_typed() {
    let (doc, a, uni, cap) = gap_fixture();
    let ev = run(&doc);
    let suggestions = appearance_rebind_suggestions(doc.appearance(), &ev);
    let target = union_suggestion(&suggestions, &cap, uni).name;
    let read = fixture::out(&doc, a);
    // The target already carries a Color of its own in the same read:
    // which value survives would be an auto-pick — refused loudly.
    let (doc, _) = step(
        doc,
        DocEdit::SetAppearance {
            body: read,
            name: target.clone(),
            attr: Attr::Color(Rgba8::opaque(1, 2, 3)),
        },
    );
    assert_eq!(
        doc.apply(
            &DocEdit::Rebind {
                body: None,
                from: cap.clone(),
                to: target.clone(),
            },
            Tol::witness()
        )
        .unwrap_err(),
        EditError::RebindAppearanceCollision {
            name: doc.spoken_name(&target),
            kind: AttrKind::Color,
        }
    );
    // Disjoint kinds merge: clear the source's Color, carry a Label
    // instead — both kinds land on the target.
    let (doc, _) = step(
        doc,
        DocEdit::ClearAppearance {
            body: read,
            name: cap.clone(),
            kind: AttrKind::Color,
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetAppearance {
            body: read,
            name: cap.clone(),
            attr: Attr::Label(editor_core::Label::new("lid").unwrap()),
        },
    );
    let applied = doc
        .apply(
            &DocEdit::Rebind {
                body: None,
                from: cap,
                to: target.clone(),
            },
            Tol::witness(),
        )
        .expect("disjoint attribute kinds merge");
    let merged = applied.doc.appearance_of(read, &target).unwrap();
    assert_eq!(merged.attrs.len(), 2);
    assert_eq!(
        merged.attrs[&AttrKind::Label],
        Attr::Label(editor_core::Label::new("lid").unwrap())
    );
}

#[test]
fn suggestion_map_is_total_over_the_store() {
    // A terminal node's cap: nothing wraps it, and the map still
    // carries the key (empty offers, never a dropped row).
    let (doc, ext) = block(
        ProfileDoc::empty_derived("m4_pr4_appearance_hook", Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        1.0,
    );
    let cap = minted(EntityKind::Face, ext, RoleSeg::Cap(CapEnd::End));
    let doc = set(doc, cap.clone(), red());
    let ev = run(&doc);
    let suggestions = appearance_rebind_suggestions(doc.appearance(), &ev);
    assert_eq!(suggestions.len(), 1);
    let (key, offers) = suggestions.first_key_value().expect("one key");
    assert_eq!(key.name, cap);
    assert_eq!(*offers, Vec::<editor_core::AppearanceKey>::new());
}

/// **A count read through a named variable** is the index's count too:
/// `k = index(N)`, `N` a free count 3 -> 2, vanishes member 2's face name
/// with the diagnosis the index's own count edit gives.
#[test]
fn a_named_count_shrinking_is_the_indexs_count() {
    let (doc, ext) = block(
        ProfileDoc::empty_derived("m4_pr4_appearance_hook", Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        1.0,
    );
    let n = editor_core::VarName::from_static("n");
    let declare = |doc, count| {
        step(
            doc,
            editor_core::DocEdit::DefineVar {
                var: n.clone().into(),
                def: editor_core::VarDecl::Free(editor_core::FreeVar::Count { value: count }),
                fresh: Vec::new(),
            },
        )
        .0
    };
    let doc = step(
        doc,
        editor_core::DocEdit::DeclareVar {
            name: n.clone(),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::Count { value: 3 }),
        },
    )
    .0;
    let count = editor_core::Formula::var(
        doc.var_named("n").expect("n is declared"),
        editor_core::Dimension::Count,
    );
    let (doc, k) = crate::fixture::index_of(doc, "k", count, None);
    let pitch =
        editor_core::Formula::mul(crate::fixture::scalar_of(k), len(2.0)).expect("a length");
    let (doc, pat) = crate::fixture::family_of(
        doc,
        ext,
        vec![(editor_core::Coord::Slide(editor_core::TargetAxis::X), pitch)],
    );
    let ev = run(&doc);
    let inst2_face = ev
        .table_at(pat)
        .unwrap()
        .iter()
        .find_map(|(n, _)| {
            (n.kind == EntityKind::Face
                && matches!(n.path.first(), Some(RoleSeg::Member { at, .. }) if at.as_slice() == [(k, 2)]))
            .then(|| n.clone())
        })
        .expect("the family mints member-2 face names");
    let doc = set(doc, inst2_face.clone(), red());
    let prior_doc = doc.clone();
    let prior_ev = run(&prior_doc);
    let doc = declare(doc, 2);
    let ev = rerun(&doc, &prior_ev);
    assert_eq!(ev.appearance.losses.len(), 1);
    let r = enrich_appearance_loss_with_prior(
        RunCtx {
            doc: &doc,
            eval: &ev,
        },
        RunCtx {
            doc: &prior_doc,
            eval: &prior_ev,
        },
        &ev.appearance.losses[0],
    );
    let Resolution::Failed(f) = r else {
        panic!("expected Failed, got {r:?}");
    };
    let ResolveError::Vanished { diagnosis, .. } = &f.error else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    assert!(
        matches!(diagnosis, Diagnosis::IndexCount { node, index } if *node == pat && *index == k),
        "{diagnosis:?}"
    );
}
