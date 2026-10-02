//! The parametric heat-sink strip (#91 C5): the tour's first M4-layer
//! showcase. ONE editor-core recipe document — base extrude, its edges
//! rounded by a `Fillet`, a fin extrude, a `PlacedUnion` of the fin
//! along a `Linear` rule, and a `Boolean(Union)` folding the group into
//! the rounded base — evaluated three times with the fin count edited
//! 5 → 7 → 9 through `SetStructuralParam`. Each re-eval feeds the
//! PRIOR evaluation as the memo, so the caption's recompute counters
//! are the ratified downstream-only-recompute story: the count edit
//! re-runs the group and the union below it, and the seven nodes
//! upstream of the edited slot — both frames, both profiles, both
//! extrudes and the fillet — are reused by content key. Stable names
//! (N1 `Instance(i)` wrapping) survive the edits, counted live.
//!
//! The fin group is a `PlacedUnion`, not a `Pattern`: a `Boolean`
//! recipe node refuses a `Pattern`'s `Instances` payload typed, because
//! Pattern's N-bodies-unfused contract is the ASSEMBLY product's
//! currency (`benchlayout` wants that). `PlacedUnion` (GROUP-BOOLEAN-
//! DESIGN, ratified A′) is one prototype, a placement rule and ONE BODY
//! out, `Instance(i)` naming preserved and `SlotId::Count` the
//! structural slot the edit drives.
//!
//! # Why the fins are sunk ([`flush_fins`], run live)
//!
//! **The fins sit 1/16 inside the base rather than flush on it** — "the
//! table-leg pattern", a transversal union instead of a face contact.
//! A real extruded heat sink's fins are flush with its base, and the
//! flush document builds: fins sketched ON the base top, the five
//! base-face pairs found by `find_flush_candidates` (each a `Rest`,
//! `SameOpposite`), declared through `declare_node`, and the union
//! against the five-shell `PlacedUnion` operand comes out at the
//! closed-form volume of the rounded base + 5 fins.
//!
//! The count EDIT is what the flush document cannot take in one step.
//! The `Declare` names the five instances it was detected against, the
//! edit to 7 makes `Instance(5)` and `Instance(6)` flush with nothing
//! declaring them, and the union refuses `UndeclaredContact` on
//! `Instance(5)` — correctly. No edit extends the declaration on the
//! live union (DM6: none rewires its `declare` input; the ruled
//! recourse is `work/recipe/declared-pairs-are-a-booleans-own-payload.md`).
//! The door that does exist is delete-and-re-add: delete the union and
//! its `Declare`, detect again (seven pairs), declare, insert a new
//! union — which builds at the closed-form volume of 7 fins. That is
//! four edits per count step and a NEW union node, where this scene's
//! subject is one edit recomputing only what is downstream of it, so
//! the scene keeps the sunk fins and measures the door instead
//! (`work/doors/a-union-that-becomes-flush-later-can-only-be-deleted-and-re-added.md`).
//!
//! # One wall, run live ([`wall_probes`])
//!
//! **The base is rounded BEFORE the union, not after.** Filleting the
//! base's twelve edges on the unioned part refuses: the fins' feet are
//! rectangular rings of the base's top face, and the blend's ring
//! carry-through check covers circular rings only
//! (`work/band/fillet-support-ring-must-be-a-circle.md`). Rounding the
//! plate first, then standing the fins on it, is the order that builds.
//!
//! The radius is under 1/16 because the ninth fin's outer wall stands
//! 1/16 inside the base's end face: at r = 1/16 that wall lands on the
//! band's trimline, and the nine-fin union refuses
//! `CurvedPierceUnsupported` there
//! (`work/hone/a-wall-flush-with-a-fillets-tangent-line-refuses-the-pierce.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use pncad::document::{
    BooleanOp, BooleanValue, CancelToken, Datum, Dimension, Doc, DocEdit, EvalOptions, Evaluation,
    Expr, LoopProgram, Node, NodeErrorKind, PatternKind, ProfileProgram, RecipeNodeId,
    RefusingReach, SlotId, ValuePayload, apply, evaluate, parse_expr,
};
// `probe_solids` is the only scene door pinned to the recording scalar
// (see its note), and it rides the `probe` feature with it.
#[cfg(feature = "probe")]
use pncad::geom_core::Probe;
use pncad::prelude::PlaneRelation;
use pncad::select::{
    ContactClass, EntityKind, NamePat, RoleSeg, SegPat, SegTag, Selector, all_edges, declare_node,
    find_flush_candidates, select,
};
use pncad::sweep::blend::BlendError;

use crate::scalar::Scalar;

/// The U8a text door, no params in scope: the tour's expressions are
/// authored the way a user would type them (`250 mm`, `5`) and go
/// through the checking parser.
/// **Authored CANONICALLY, deliberately**: this scene and `checks` are
/// the default half of the units exhibit, against `ring` (millimetres
/// and half-turns) and `diefillet` (millimetres and degrees). Nothing
/// here names a unit, so every literal stores the canonical row for
/// its dimension and the panel opens on `m` because the document SAYS
/// `m` — not because a reader had to pick a fallback.
fn pe(src: &str) -> Expr {
    parse_expr(src, &BTreeMap::new()).expect("tour expression")
}
use crate::{SceneBody, Stop, View};
use pncad::geom_core::Tol;

/// The count-5 name table's size, pinned. Measured, not derived: the
/// group node emits `Instance(i)` over one fused body, and how many
/// names that comes to is the naming vocabulary's answer rather than
/// something this scene can compute. Moving it is a deliberate act —
/// see the assertion below.
const HEATSINK_NAMES_AT_5: usize = 131;
/// The base plate, `3 × 1 × 0.25`.
const BASE: [f64; 3] = [3.0, 1.0, 0.25];
/// The base fillet's radius, as authored (`R` below is the same value).
const RADIUS: &str = "31.25 mm";
const R: f64 = 0.03125;
/// Per-fin material gain: a 0.1875 × 0.75 footprint standing 0.75 above
/// the base top. A sunk fin is 0.8125 tall with its bottom 1/16 inside
/// the base, a flush one 0.75 tall on the top face: the same gain.
const FIN_GAIN: f64 = 0.1875 * 0.75 * 0.75;

/// How a fin meets the base.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Seat {
    /// 1/16 inside the base: a transversal union, nothing to declare.
    Sunk,
    /// On the base top, every foot declared as a `Rest` contact.
    Flush,
}

struct Recipe {
    doc: Doc<ProfileProgram>,
    /// The `PlacedUnion` over the fin — the fin count's structural
    /// slot, and what `SetStructuralParam` edits.
    group: RecipeNodeId,
    /// The `Boolean(Union)` that folds the fin group into the base.
    solid: RecipeNodeId,
    /// The node the union takes as its base operand.
    base: RecipeNodeId,
    /// The `Declare` feeding the union, when the fins sit flush.
    declare: Option<RecipeNodeId>,
}

fn insert(doc: &mut Doc<ProfileProgram>, node: Node<ProfileProgram>, tol: Tol) -> RecipeNodeId {
    let applied = apply(
        doc,
        &DocEdit::InsertNode {
            node: Box::new(node),
        },
        tol,
        &RefusingReach,
    )
    .expect("insert node");
    *doc = applied.doc;
    applied.record.minted.expect("insert mints an id")
}

fn eval(doc: &Doc<ProfileProgram>, prior: Option<&Evaluation<f64>>, tol: Tol) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    )
}

/// The fin-count edit, the one the tour's story is about.
fn set_count(
    doc: &Doc<ProfileProgram>,
    group: RecipeNodeId,
    n: usize,
    tol: Tol,
) -> Doc<ProfileProgram> {
    apply(
        doc,
        &DocEdit::SetStructuralParam {
            node: group,
            slot: SlotId::Count,
            expr: pe(&format!("{n}")),
        },
        tol,
        &RefusingReach,
    )
    .expect("count edit")
    .doc
}

/// The scene's own document: fins sunk, base rounded.
fn scene_doc(tol: Tol) -> Recipe {
    build_doc(tol, Seat::Sunk, true)
}

/// Builds the heat sink at five fins. Where a stored selection is
/// needed — the fillet's edges, the declared contacts — it is taken
/// from an evaluation of the document so far: evaluate, select, store.
fn build_doc(tol: Tol, seat: Seat, round_base: bool) -> Recipe {
    let base_loops = vec![
        LoopProgram::polygon([(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.0, 1.0)])
            .expect("finite corners"),
    ];
    let fin_loops = vec![
        LoopProgram::polygon([
            (0.25, 0.125),
            (0.4375, 0.125),
            (0.4375, 0.875),
            (0.25, 0.875),
        ])
        .expect("finite corners"),
    ];
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("heatsink", tol);
    let len = |v: f64| Expr::literal(v, Dimension::Length).expect("finite");
    let scl = |v: f64| Expr::literal(v, Dimension::Scalar).expect("finite");
    let frame_at = |z: f64| {
        Node::Datum(Datum::Frame {
            origin: [len(0.0), len(0.0), len(z)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        })
    };
    let base_plane = insert(&mut doc, frame_at(0.0), tol);
    let base_p = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            plane: base_plane,
            loops: base_loops,
            ids: Vec::new(),
        }),
        tol,
    );
    let mut base = insert(
        &mut doc,
        Node::Extrude {
            profile: base_p,
            distance: pe("250 mm"),
        },
        tol,
    );
    if round_base {
        let edges = all_edges(&eval(&doc, None, tol), base);
        base = insert(&mut doc, Node::fillet(base, pe(RADIUS), edges), tol);
    }
    let (fin_z, fin_height) = match seat {
        Seat::Sunk => (0.1875, "812.5 mm"),
        Seat::Flush => (0.25, "750 mm"),
    };
    let fin_plane = insert(&mut doc, frame_at(fin_z), tol);
    let fin_p = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            plane: fin_plane,
            loops: fin_loops,
            ids: Vec::new(),
        }),
        tol,
    );
    let fin_e = insert(
        &mut doc,
        Node::Extrude {
            profile: fin_p,
            distance: pe(fin_height),
        },
        tol,
    );
    // `placed_union` is the PARAMETRIC-rule constructor, so the count
    // is a structural slot.
    let group = insert(
        &mut doc,
        Node::placed_union(
            fin_e,
            pe("5"),
            PatternKind::Linear {
                direction: [pe("1.0"), pe("0.0"), pe("0.0")],
                spacing: pe("312.5 mm"),
            },
        )
        .expect("a Linear rule is parametric, so it carries a count"),
        tol,
    );
    let declare = match seat {
        Seat::Sunk => None,
        Seat::Flush => {
            let found = find_flush_candidates(&eval(&doc, None, tol), base, group, tol)
                .expect("the fin feet are definite flush pairs");
            // The inspection: one resting contact per fin, the base top
            // against that fin's foot.
            assert_eq!(found.len(), 5, "one contact per fin: {found:#?}");
            assert!(
                found.iter().all(|f| f.class == ContactClass::Rest
                    && f.evidence.relation == PlaneRelation::SameOpposite),
                "every foot rests on the top: {found:#?}"
            );
            Some(insert(
                &mut doc,
                declare_node(&found).expect("nonempty findings"),
                tol,
            ))
        }
    };
    let solid = insert(
        &mut doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: base,
            b: group,
            declare,
        },
        tol,
    );
    Recipe {
        doc,
        group,
        solid,
        base,
        declare,
    }
}

/// The document's OWN final body, read back, gated on the exact
/// volume: the rounded plate's closed form plus `n` fins.
fn solidify<S: Scalar>(
    r: &Recipe,
    ev: &Evaluation<S>,
    n: usize,
    tol: Tol,
) -> (pncad::topo::Body<S>, pncad::topo::ContactRecords) {
    let value = ev.value(r.solid).expect("the union node evaluated");
    let ValuePayload::Boolean(BooleanValue::Body { body, contacts, .. }) = &value.payload else {
        panic!("union payload: {:?}", value.payload);
    };
    let want = volume(n);
    let got = pncad::topo::mass_properties(body, tol)
        .expect("mass properties")
        .volume
        .f();
    assert!(
        (got - want).abs() <= 1e-9,
        "the {n}-fin solid measures {got}, and the rounded base + {n} fins is {want}"
    );
    ((**body).clone(), (**contacts).clone())
}

/// The rounded plate's closed form plus `n` fins.
fn volume(n: usize) -> f64 {
    crate::oracles::rounded_box_volume(BASE, R) + n as f64 * FIN_GAIN
}

/// The recipe evaluated + solidified at every fin count the tour
/// shows (5 → 7 → 9, each re-eval fed the prior as memo) — the Probe
/// sweep records the document-evaluation predicates AND the union
/// at every count.
/// Only `crate::probe` calls this, so it rides the `probe` feature with
/// it — otherwise a default build trips `dead_code` under CI's
/// `-D warnings`.
///
/// AT `Probe`, NOT GENERIC OVER [`Scalar`], and the reason is a door
/// that does not exist rather than a preference. `evaluate` requires
/// `EvalScalar`, which since the interval parameter door landed
/// requires `editor_core::analysis::AxisScalar` — a scalar that can
/// bind a widened lane environment. `Scalar` does not imply it, and
/// this crate cannot add it as a bound: `AxisScalar` is deliberately
/// interior to the façade (pncad's own surface census lists it under
/// the E6 driver's vocabulary, NOT carried), so `pncad::` has no
/// spelling for it. The genericity was never exercised either way —
/// `sweep` takes `Vec<ProbeBody>`, so the sole call site could only
/// ever instantiate this at `Probe`, and every other `evaluate` in the
/// demos is concrete at `f64`. Widening the façade so a consumer can
/// name the evaluation contract's own bound is a design question for
/// that census, not something to settle from here.
#[cfg(feature = "probe")]
pub(crate) fn probe_solids(
    tol: Tol,
) -> Vec<(pncad::topo::Body<Probe>, pncad::topo::ContactRecords)> {
    let r = scene_doc(tol);
    let cancel = CancelToken::new();
    let opts = EvalOptions::default();
    let ev5 = evaluate::<Probe>(&r.doc, None, &cancel, &opts, tol);
    let mut out = vec![solidify(&r, &ev5, 5, tol)];
    let mut doc = r.doc.clone();
    let mut prior = ev5;
    for n in [7usize, 9] {
        doc = set_count(&doc, r.group, n, tol);
        let ev = evaluate::<Probe>(&doc, Some(&prior), &cancel, &opts, tol);
        out.push(solidify(&r, &ev, n, tol));
        prior = ev;
    }
    out
}

/// This scene's recipe, as a document the GUI can open.
///
/// The same `build_doc` the stops walk — the gallery must not be a
/// second authoring of the scene, or it would stop being evidence
/// about this one.
pub fn gallery_document(tol: Tol) -> Doc<ProfileProgram> {
    scene_doc(tol).doc
}

/// A node's refusal, or `Ok(())` when it built — both arms reachable,
/// so a wall whose refusal goes away is noticed.
fn outcome(ev: &Evaluation<f64>, node: RecipeNodeId) -> Result<(), &NodeErrorKind> {
    ev.node_error(node).map_or(Ok(()), |e| Err(&e.kind))
}

/// Flush fins, measured live — the module docs' first section.
///
/// Not a wall: an undeclared contact after the count edit refusing is
/// the boolean failing loud, and stays right whatever declaration door
/// lands. What is missing is an edit that extends the declaration on
/// the live union, and there is none to attempt; the delete-and-re-add
/// that does exist is measured here instead.
fn flush_fins(tol: Tol) {
    let flush = build_doc(tol, Seat::Flush, true);
    let declare = flush.declare.expect("flush fins are declared");
    let ev5 = eval(&flush.doc, None, tol);
    solidify(&flush, &ev5, 5, tol);
    println!(
        "   flush fins, 5 declared Rest contacts: the union builds, volume {} (gated 1e-9)",
        volume(5)
    );

    let doc7 = set_count(&flush.doc, flush.group, 7, tol);
    let ev7 = eval(&doc7, Some(&ev5), tol);
    let refusal = ev7.node_error(flush.solid).map(|e| &e.kind);
    assert!(
        matches!(refusal, Some(NodeErrorKind::UndeclaredContact { finding, .. })
            if matches!(finding.pair.1.name.path.first(), Some(RoleSeg::Instance { i: 5, .. }))),
        "the count edit leaves Instance(5) flush and undeclared: {refusal:?}"
    );
    println!(
        "   flush fins, count edited 5 -> 7: the union refuses UndeclaredContact on Instance(5)"
    );

    // The recourse that exists: delete the union and its Declare,
    // detect again at 7, declare, insert a NEW union.
    let mut doc = doc7;
    for id in [flush.solid, declare] {
        doc = apply(&doc, &DocEdit::DeleteNode { id }, tol, &RefusingReach)
            .expect("the union is a sink, and then its Declare is")
            .doc;
    }
    let ev_cut = eval(&doc, Some(&ev7), tol);
    let found = find_flush_candidates(&ev_cut, flush.base, flush.group, tol)
        .expect("the fin feet are definite flush pairs");
    assert_eq!(found.len(), 7, "one contact per fin: {found:#?}");
    let redeclared = insert(
        &mut doc,
        declare_node(&found).expect("nonempty findings"),
        tol,
    );
    let solid = insert(
        &mut doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: flush.base,
            b: flush.group,
            declare: Some(redeclared),
        },
        tol,
    );
    let ev = eval(&doc, Some(&ev_cut), tol);
    let readded = Recipe {
        doc,
        solid,
        declare: Some(redeclared),
        ..flush
    };
    solidify(&readded, &ev, 7, tol);
    println!(
        "   flush fins, delete + re-detect + re-add at 7: the union builds, volume {} \
         (four edits; recomputed {}, reused {}; the union is a new node)",
        volume(7),
        ev.recomputed,
        ev.reused
    );
}

/// The wall the module docs name, attempted for real.
fn wall_probes(tol: Tol) {
    // The base's twelve edges rounded AFTER the union, picked as the
    // union's edges that came through from operand A.
    let sharp = build_doc(tol, Seat::Sunk, false);
    let ev = eval(&sharp.doc, None, tol);
    let base_edges = select(
        &ev,
        sharp.solid,
        &Selector::of(NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::FromA))),
    );
    assert_eq!(base_edges.len(), 12, "the plate's edges: {base_edges:#?}");
    let mut doc = sharp.doc.clone();
    let rounded = insert(
        &mut doc,
        Node::fillet(sharp.solid, pe(RADIUS), base_edges),
        tol,
    );
    crate::walls::wall(
        "heat sink",
        1,
        "fillet the base's twelve edges on the unioned part",
        outcome(&eval(&doc, Some(&ev), tol), rounded),
        |k| {
            matches!(k, NodeErrorKind::Blend {
                error: BlendError::UnsupportedGeometry { detail, .. }, ..
            } if *detail == "a ring edge's carrier is not a circle, the only ring the \
                              clearance check covers")
        },
        "move the scene's fillet below the union, so the count edit re-runs it, and \
         re-count the edit's recompute/reuse pins",
    );
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    let r = scene_doc(tol);

    // Evaluate at 5, then EDIT the structural count and re-evaluate
    // against the prior — the memo counters are the demo.
    let ev5 = eval(&r.doc, None, tol);
    assert_eq!(ev5.recomputed, 9, "a cold evaluation computes every node");
    let names5 = ev5
        .value(r.group)
        .expect("the fin group @ 5")
        .name_table
        .clone();

    let mut doc = r.doc.clone();
    let mut evs: Vec<(usize, Evaluation<f64>, String)> = Vec::new();
    let cold = format!("cold evaluation: all {} nodes computed", ev5.recomputed);
    evs.push((5, ev5, cold));
    for (prior_idx, n) in [7usize, 9].into_iter().enumerate() {
        doc = set_count(&doc, r.group, n, tol);
        let ev = eval(&doc, Some(&evs[prior_idx].1), tol);
        let caption = format!(
            "count edit -> {n}: recomputed {} node(s), reused {} (downstream-only recompute)",
            ev.recomputed, ev.reused
        );
        // The fin group and the union that consumes it re-run; both
        // frames, both profiles, both extrudes and the base's fillet
        // are upstream of the edited slot and reuse by content key.
        assert_eq!(
            ev.recomputed, 2,
            "a count edit re-runs exactly the fin group and the union below it"
        );
        assert_eq!(ev.reused, 7, "everything upstream reuses by content key");
        evs.push((n, ev, caption));
    }

    // Stable names survive the structural edits (N1 Instance(i)).
    assert_eq!(
        names5.len(),
        HEATSINK_NAMES_AT_5,
        "the count-5 fin-group name table is pinned at {HEATSINK_NAMES_AT_5} entries; a \
         change means the naming emission vocabulary moved - update \
         this pin deliberately"
    );
    let names9 = &evs[2]
        .1
        .value(r.group)
        .expect("the fin group @ 9")
        .name_table;
    let survived = names5
        .iter()
        .filter(|(name, _)| names9.lookup(name).is_some())
        .count();
    assert_eq!(
        survived,
        names5.len(),
        "every count-5 pattern name must still resolve at count 9"
    );
    println!(
        "   stable names: {survived}/{} of the count-5 pattern names still resolve \
         after both edits (N1 Instance(i) wrapping)",
        names5.len()
    );
    flush_fins(tol);
    wall_probes(tol);

    let recipe_ops = "ONE recipe doc: Profile -> Extrude -> Fillet (base), Profile -> Extrude \
         (fin) -> PlacedUnion(Linear count) -> Boolean(Union); count edited via \
         SetStructuralParam";
    let colors = [[0.45, 0.62, 0.62], [0.38, 0.58, 0.68], [0.32, 0.54, 0.74]];
    evs.into_iter()
        .zip(colors)
        .map(|((n, ev, recompute_story), color)| {
            let (body, contacts) = solidify(&r, &ev, n, tol);
            let name: &'static str = match n {
                5 => "heatsink5",
                7 => "heatsink7",
                _ => "heatsink9",
            };
            Stop {
                name,
                caption: format!("heat sink ({n} fins)"),
                // Montage cell held by `impeller12`, which makes this
                // scene's claim with a relation rather than one number:
                // its count and its angular step read the SAME
                // parameter. `bench` carries a linear pattern on the
                // sheet already; all three counts here keep their
                // standalone renders.
                montage: false,
                story: "parametric heat-sink strip from ONE recipe document — fin count \
                        is a structural parameter; this render is one evaluation",
                ops: recipe_ops,
                delta: 1e-2,
                note: Some(format!(
                    "{recompute_story}; fins sunk 1/16 into a base rounded at r = 1/32 \
                     (volume {} = the rounded plate's closed form + {n} fins, gated 1e-9); \
                     flush fins build, but a count edit outruns their declared contacts, \
                     and the base fillet refuses after the union (wall 1)",
                    volume(n)
                )),
                view: View {
                    elev: 24.0,
                    azim: -62.0,
                    up: 'z',
                },
                bodies: vec![SceneBody::seamed(name, color, body, contacts).named(&ev, r.solid)],
            }
        })
        .collect()
}

#[cfg(test)]
mod name_column {
    //! The budget sweep's `name` column, pinned where a real document
    //! mints the names: this scene is one of the six the tour can name
    //! at all, and the token it writes has to survive a CSV and mean
    //! one face.
    //!
    //! **What these rows can and cannot say.** `crate::face_names` is a
    //! crate-root item with six callers — `heatsink5`, `heatsink7`,
    //! `heatsink9` here and `diefillet`, `diepips`, `diecomposed` in
    //! `diefillet.rs` — and it is exercised from exactly one of them,
    //! this one. What that misses is a scene whose faces the node's
    //! table does not name, which would panic at the door rather than
    //! render badly; the door is the same door for all six and the
    //! panic names the node and the face, so the failure is loud
    //! wherever it happens, but it is not under test at five of them.
    //!
    //! **Non-emptiness and comma-freedom are NOT asserted here**, and
    //! deliberately: `crate::face_names` puts every token through
    //! `tess_meter::FaceName::new` and `.expect()`s exactly those two
    //! properties, so no input can reach an assertion in this module in
    //! a failing state — the door panics first. A row asserting them
    //! after the door would be a comment wearing an `assert!`. The
    //! refusal itself is `tess-meter`'s, tested there; what is left for
    //! this scene is what the door does not check, which is that the
    //! rendering means ONE face and is reversible.

    use super::scene_doc;
    use pncad::document::{CancelToken, EvalOptions, evaluate};
    use pncad::geom_core::Tol;
    use pncad::prelude::StableName;

    /// The scene's own body and the names of its faces, as
    /// `crate::face_names` renders them for the CSV.
    fn tokens() -> Vec<(String, StableName)> {
        let tol = Tol::witness();
        let r = scene_doc(tol);
        let ev = evaluate::<f64>(
            &r.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        let (body, _) = super::solidify(&r, &ev, 5, tol);
        let rendered = crate::face_names(&ev, r.solid, &body);
        let rows = body
            .faces()
            .map(|(key, _)| {
                let token = rendered
                    .get(&key)
                    .expect("face_names covers every face of the body")
                    .as_str()
                    .to_string();
                let name = pncad::select::face_name(&ev, r.solid, 0, key)
                    .expect("the node names its own face")
                    .clone();
                (token, name)
            })
            .collect::<Vec<_>>();
        assert!(
            rows.len() > 1,
            "the fixture has faces to name: {} rows",
            rows.len()
        );
        rows
    }

    /// **The token means ONE face.** A rendering that collapsed two
    /// derivation paths would hand `tools/tess-lint` a join key no
    /// better than the ordinal it already has.
    #[test]
    fn distinct_faces_render_distinct_tokens() {
        let rows = tokens();
        let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
        for (token, name) in &rows {
            assert!(seen.insert(token), "{name} shares its token: {token:?}");
        }
        assert_eq!(seen.len(), rows.len());
    }

    /// **The `,` → `;` swap is reversible**, which is what makes the
    /// flattening injective rather than merely comma-free: a
    /// `StableName` contains no string payload, so no `;` of its own
    /// can be confused with one the swap wrote. Asserted by putting
    /// the commas back and reading the name out again.
    #[test]
    fn the_swap_round_trips_to_the_same_name() {
        for (token, name) in tokens() {
            let json = token.replace(';', ",");
            let back: StableName =
                serde_json::from_str(&json).expect("the swapped token is the name's own JSON");
            assert_eq!(back, name, "round trip through {token:?}");
        }
    }
}

#[cfg(test)]
mod scene {
    //! The PR gate runs the tour's tests, not the tour: this is where the
    //! scene's own assertions — the recompute and reuse counts, the
    //! surviving names, the volume at every count — and both wall
    //! probes run on every PR that touches the tour.

    #[test]
    fn the_scene_and_its_walls_hold() {
        super::stops(pncad::geom_core::Tol::witness());
    }
}
