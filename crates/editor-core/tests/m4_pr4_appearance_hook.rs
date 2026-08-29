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

mod fixture;

use editor_core::{
    AppearanceLossCause, Attr, AttrKind, BooleanOp, CancelToken, CapEnd, Diagnosis, DocEdit,
    EditError, EntityKind, EvalOptions, Evaluation, Node, ProfileDoc, RecipeNodeId, Resolution,
    ResolveError, ResolveIndeterminate, Rgba8, RoleSeg, RunCtx, SlotId, StableName,
    appearance_rebind_suggestions, enrich_appearance_loss, enrich_appearance_loss_with_prior,
    evaluate,
};
use fixture::{desc, insert, len, scl, step};

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default())
}

fn rerun(doc: &ProfileDoc, prior: &Evaluation<f64>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        Some(prior),
        &CancelToken::new(),
        &EvalOptions::default(),
    )
}

fn name1(kind: EntityKind, node: RecipeNodeId, seg: RoleSeg) -> StableName {
    StableName {
        kind,
        node,
        path: vec![seg],
    }
}

fn red() -> Attr {
    Attr::Color(Rgba8::opaque(200, 30, 30))
}

fn set(doc: ProfileDoc, name: StableName, attr: Attr) -> ProfileDoc {
    step(doc, DocEdit::SetAppearance { name, attr }).0
}

/// A rectangular block: profile on the plane z = `z0`, extruded `dz`.
fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
        )),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

/// The PR 3 genuine-tie fixture: a U cutter through one wall; both
/// B-cap prong fragments tie. Returns (doc, subtract node).
fn tie_fixture() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty();
    let (doc, a) = block(doc, (0.0, 4.0), (0.0, 4.0), 0.0, 4.0);
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 1.0],
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
        )),
    );
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(2.0),
        },
    );
    let (doc, sub) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b,
            declare: None,
        },
    );
    (doc, sub)
}

/// The operand→final gap fixture (PR 7 review A1): operand `a`'s top
/// cap painted, `a` consumed by a union. Returns (doc, a, uni, cap).
fn gap_fixture() -> (ProfileDoc, RecipeNodeId, RecipeNodeId, StableName) {
    let doc = ProfileDoc::empty();
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, uni) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: None,
        },
    );
    let cap = name1(EntityKind::Face, a, RoleSeg::Cap(CapEnd::Top));
    let doc = set(doc, cap.clone(), red());
    (doc, a, uni, cap)
}

// ---- Enrichment: the per-arm mapping (spec D9) ----

#[test]
fn ambiguous_loss_enriches_by_table_lookup_at_the_recorded_site() {
    let (doc, sub) = tie_fixture();
    let ev = run(&doc);
    let tied = ev
        .value(sub)
        .unwrap()
        .name_table
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

#[test]
fn node_gone_loss_enriches_with_the_derived_deletion_edit() {
    let (doc, ext) = block(ProfileDoc::empty(), (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let cap = name1(EntityKind::Face, ext, RoleSeg::Cap(CapEnd::Top));
    let doc = set(doc, cap.clone(), red());
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
    // Count 3 -> 2 vanishes instance 2's face name; the coarse loss
    // (empty structural candidates) enriches to the pinned
    // StructuralParam diagnosis plus the last-good tombstone.
    let (doc, ext) = block(ProfileDoc::empty(), (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: ext,
            count: editor_core::Expr::count(3),
            kind: editor_core::PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let ev = run(&doc);
    let inst2_face = ev
        .value(pat)
        .unwrap()
        .name_table
        .iter()
        .find_map(|(n, _)| {
            (n.kind == EntityKind::Face
                && matches!(n.path.first(), Some(RoleSeg::Instance { i: 2, .. })))
            .then(|| n.clone())
        })
        .expect("pattern mints instance-2 face names");
    let doc = set(doc, inst2_face.clone(), red());
    let prior_doc = doc.clone();
    let prior_ev = run(&prior_doc);
    assert!(prior_ev.appearance.is_lossless());

    let (doc, _) = step(
        doc,
        DocEdit::SetStructuralParam {
            node: pat,
            slot: SlotId::Count,
            expr: editor_core::Expr::count(2),
        },
    );
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
    assert_eq!(
        *diagnosis,
        Diagnosis::StructuralParam {
            node: pat,
            param: SlotId::Count
        }
    );
    let t = last_good.as_ref().expect("the prior run resolved the name");
    assert_eq!(t.kind, EntityKind::Face);
    assert_eq!(t.patch.node, pat);
    // The coarse structural candidates (empty here) are a subset of
    // the ladder's offers by construction; nothing structural offers
    // itself for a dropped instance.
    assert!(f.offers.is_empty());
}

#[test]
fn indeterminate_losses_enrich_to_the_matching_indeterminate_arm() {
    // Poisoned: operand A degenerates, the union is poisoned through
    // it, and enrichment preserves the indeterminate verdict (not
    // Vanished — same vocabulary on both sides of the hook).
    let doc = ProfileDoc::empty();
    let (doc, a) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, b) = block(doc, (1.0, 3.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, uni) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: None,
        },
    );
    let ev = run(&doc);
    let uni_face = ev
        .value(uni)
        .unwrap()
        .name_table
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
            expr: len(0.0),
        },
    );
    let ev = run(&doc);
    assert_eq!(ev.appearance.losses.len(), 1);
    let loss = &ev.appearance.losses[0];
    assert_eq!(
        loss.cause,
        AppearanceLossCause::TargetPoisoned { through: a }
    );
    assert_eq!(
        enrich_appearance_loss(
            RunCtx {
                doc: &doc,
                eval: &ev
            },
            loss
        ),
        Resolution::Indeterminate(ResolveIndeterminate::TargetPoisoned { through: a })
    );

    // Canceled: the not-evaluated arm, with the node made explicit.
    let (doc2, ext) = block(ProfileDoc::empty(), (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let cap = name1(EntityKind::Face, ext, RoleSeg::Cap(CapEnd::Top));
    let doc2 = set(doc2, cap, red());
    let cancel = CancelToken::new();
    cancel.cancel();
    let ev2 = evaluate::<f64>(&doc2, None, &cancel, &EvalOptions::default());
    assert_eq!(ev2.appearance.losses.len(), 1);
    let loss2 = &ev2.appearance.losses[0];
    assert_eq!(loss2.cause, AppearanceLossCause::TargetNotEvaluated);
    assert_eq!(
        enrich_appearance_loss(
            RunCtx {
                doc: &doc2,
                eval: &ev2
            },
            loss2
        ),
        Resolution::Indeterminate(ResolveIndeterminate::TargetNotEvaluated { node: ext })
    );
}

// ---- The banked obligation: operand→final Rebind ergonomics ----

/// The union-side suggestion for `cap`: a face name minted at `uni`
/// wrapping `cap` (`FromA(cap)`-headed derivation).
fn union_suggestion(
    suggestions: &std::collections::BTreeMap<StableName, Vec<StableName>>,
    cap: &StableName,
    uni: RecipeNodeId,
) -> StableName {
    suggestions
        .get(cap)
        .expect("the suggestion map is total over the appearance store")
        .iter()
        .find(|s| s.node == uni && s.kind == EntityKind::Face)
        .expect("the final node's wrapping derivation is offered")
        .clone()
}

#[test]
fn suggestions_offer_the_final_wrapping_derivation_and_rebind_repairs_the_gap() {
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
        matches!(target.path.first(), Some(RoleSeg::FromA(inner)) if **inner == cap),
        "the offered name wraps the painted one: {target:?}"
    );

    // The explicit repair: Rebind moves the appearance key (the
    // attribute rides the name) and is presentation-only here — no
    // Declare pair moved, nothing recomputes.
    let applied = doc
        .apply(&DocEdit::Rebind {
            from: cap.clone(),
            to: target.clone(),
        })
        .expect("an appearance key is a rebind site");
    assert!(!applied.record.structural);
    assert!(applied.doc.appearance_of(&cap).is_none());
    assert_eq!(
        applied.doc.appearance_of(&target).unwrap()[&AttrKind::Color],
        red()
    );

    // The paint now lands on the final node.
    let ev = run(&applied.doc);
    assert!(ev.appearance.is_lossless());
    assert!(
        ev.appearance.for_node(uni).is_some(),
        "the repaired attachment resolves on the final body"
    );
}

#[test]
fn appearance_only_rebind_counts_as_a_site_not_no_references() {
    // Before the appearance store landed, this exact edit was refused
    // RebindNoReferences (no Declare pair references `cap`); the key
    // itself is now a site.
    let (doc, _, uni, cap) = gap_fixture();
    let ev = run(&doc);
    let suggestions = appearance_rebind_suggestions(doc.appearance(), &ev);
    let target = union_suggestion(&suggestions, &cap, uni);
    assert!(
        doc.apply(&DocEdit::Rebind {
            from: cap,
            to: target
        })
        .is_ok()
    );
}

#[test]
fn rebind_appearance_collision_is_refused_typed() {
    let (doc, _, uni, cap) = gap_fixture();
    let ev = run(&doc);
    let suggestions = appearance_rebind_suggestions(doc.appearance(), &ev);
    let target = union_suggestion(&suggestions, &cap, uni);
    // The target already carries a Color of its own: which value
    // survives would be an auto-pick — refused loudly.
    let doc = set(doc, target.clone(), Attr::Color(Rgba8::opaque(1, 2, 3)));
    assert_eq!(
        doc.apply(&DocEdit::Rebind {
            from: cap.clone(),
            to: target.clone(),
        })
        .unwrap_err(),
        EditError::RebindAppearanceCollision {
            name: target.clone(),
            kind: AttrKind::Color,
        }
    );
    // Disjoint kinds merge: clear the source's Color, carry a Label
    // instead — both kinds land on the target.
    let (doc, _) = step(
        doc,
        DocEdit::ClearAppearance {
            name: cap.clone(),
            kind: AttrKind::Color,
        },
    );
    let doc = set(doc, cap.clone(), Attr::Label("lid".into()));
    let applied = doc
        .apply(&DocEdit::Rebind {
            from: cap,
            to: target.clone(),
        })
        .expect("disjoint attribute kinds merge");
    let merged = applied.doc.appearance_of(&target).unwrap();
    assert_eq!(merged.len(), 2);
    assert_eq!(merged[&AttrKind::Label], Attr::Label("lid".into()));
}

#[test]
fn suggestion_map_is_total_over_the_store() {
    // A terminal node's cap: nothing wraps it, and the map still
    // carries the key (empty offers, never a dropped row).
    let (doc, ext) = block(ProfileDoc::empty(), (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let cap = name1(EntityKind::Face, ext, RoleSeg::Cap(CapEnd::Top));
    let doc = set(doc, cap.clone(), red());
    let ev = run(&doc);
    let suggestions = appearance_rebind_suggestions(doc.appearance(), &ev);
    assert_eq!(suggestions.len(), 1);
    assert_eq!(suggestions[&cap], Vec::<StableName>::new());
}
