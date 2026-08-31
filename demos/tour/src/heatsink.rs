//! The parametric heat-sink strip (#91 C5): the tour's first M4-layer
//! showcase. ONE editor-core recipe document (base extrude + fin
//! extrude + `LinearPattern`), evaluated three times with the fin
//! count edited 5 → 7 → 9 through `SetStructuralParam` — each re-eval
//! feeds the PRIOR evaluation as the memo, so the caption's recompute
//! counters are the ratified downstream-only-recompute story (the
//! count edit re-runs exactly ONE node, the pattern; everything
//! upstream is reused by content key). Stable names (N1
//! `Instance(i)` wrapping) survive the edits — counted live.
//!
//! **The whole part is now IN the document** (#1344). It used to end
//! with a `Pattern` node — N placed instance bodies — and a union chain
//! written HERE, in demo code, under an F4 note saying a `Boolean`
//! recipe node cannot consume a Pattern's `Instances` payload.
//!
//! That sentence is still true, and still by design: `body_operand`
//! refuses `ValuePayload::Instances` typed, because Pattern's
//! N-bodies-unfused contract is deliberate — its instances are the
//! ASSEMBLY product's currency, gathered per-instance by
//! `product::sources_of`, which is what `benchlayout` needs. What
//! changed is that the heat sink was asking the wrong node.
//! `Node::PlacedUnion` (GROUP-BOOLEAN-DESIGN, ratified A′) is the one
//! it wanted: one prototype, a placement rule, ONE BODY out,
//! disjointness CERTIFIED through `topo::Separation`, `Instance(i)`
//! naming preserved, and `SlotId::Count` still the structural slot the
//! fin-count edit drives. Its output is an ordinary `Body`, so the
//! union into the base is an ordinary `Boolean` node beside it.
//!
//! The design record named this scene by name — *"the heatsink's
//! out-of-document union moves INTO the document (the F4 note retires
//! at its origin, both workarounds deleted per the demo doctrine)"* —
//! and this is that move.
//!
//! # The 1/16 overlap is still a dodge, and still here (#1344)
//!
//! The fins are sketched 1/16 INSIDE the base rather than sitting flush
//! on it, "the table-leg pattern". What that dodges is the
//! undeclared-coincidence refusal, and `bool_bodies::table` runs the
//! experiment live next door: a leg whose top face is EXACTLY coplanar
//! with the tabletop's underside, undeclared, refuses at the
//! coincidence door (rung (b) — value equality never classifies), while
//! the same leg overlapped 0.05 into the top unions as an ordinary
//! transversal intersection.
//!
//! A real extruded heat sink's fins ARE flush with its base; the 1/16
//! embedment is a modelling fiction the part does not have. The honest
//! version declares the contacts instead — which only became
//! practical with `PlacedUnion`, since per-fin declarations against
//! bit-identical `StableName`s had no per-instance discriminator before
//! `Instance(i)`. Recorded, not fixed here: it is #1344's own
//! follow-up, and it wants the recipe layer's `Node::Declare` path.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use pncad::document::{
    BooleanOp, BooleanValue, CancelToken, Doc, DocEdit, EvalOptions, Evaluation, Expr, LoopProgram,
    Node, PatternKind, ProfileProgram, RecipeNodeId, SlotId, ValuePayload, apply, evaluate,
    parse_expr,
};
use pncad::geom_core::{Point3, Vec3};
// `probe_solids` is the only scene door pinned to the recording scalar
// (see its note), and it rides the `probe` feature with it.
#[cfg(feature = "probe")]
use pncad::geom_core::Probe;
use pncad::profile::SketchPlane;

use crate::scalar::Scalar;

/// The U8a text door, no params in scope: the tour's expressions are
/// authored the way a user would type them (`250 mm`, `5`) and go
/// through the checking parser. The canonical-meter BITS are unchanged
/// (250·10⁻³ lands on the same dyadic 0.25 the tour used to hand-write
/// — pinned in editor-core's u8a_parse suite), so this is a SAID
/// change: exports stay byte-identical.
fn pe(src: &str) -> Expr {
    parse_expr(src, &BTreeMap::new()).expect("tour expression")
}
use crate::{SceneBody, Stop, View};
use pncad::geom_core::Tol;

/// The count-5 name table's size, pinned. Re-measured at the
/// `PlacedUnion` migration; see the assertion for why it moved.
const HEATSINK_NAMES_AT_5: usize = 135;
const BASE_VOL: f64 = 3.0 * 1.0 * 0.25;
/// Per-fin material gain: 0.1875 x 0.75 footprint, 0.8125 tall, minus
/// the 1/16 slice overlapping into the base.
const FIN_GAIN: f64 = 0.1875 * 0.75 * (0.8125 - 0.0625);

struct Recipe {
    doc: Doc<ProfileProgram>,
    /// The `PlacedUnion` over the fin — the fin count's structural
    /// slot, and what `SetStructuralParam` edits.
    group: RecipeNodeId,
    /// The `Boolean(Union)` that folds the fin group into the base:
    /// the node that used to be a chain of unions in demo code.
    solid: RecipeNodeId,
}

fn build_doc(tol: Tol) -> Recipe {
    // v4 (LIB-SWITCH): the document stores the PROGRAM — the polygon
    // chain (`At`, `LineTo`…, `LineTo(Start)`), replayed through the
    // driver at every evaluation.
    let base_profile = ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.0, 1.0)])
                .expect("finite corners"),
        ],
    };
    // Fin sketch sits at z = 0.1875 — 1/16 INSIDE the 0.25-thick base.
    let fin_plane = SketchPlane::from_frame(
        Point3::new(0.0, 0.0, 0.1875),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let fin_profile = ProfileProgram {
        plane: fin_plane,
        loops: vec![
            LoopProgram::polygon([
                (0.25, 0.125),
                (0.4375, 0.125),
                (0.4375, 0.875),
                (0.25, 0.875),
            ])
            .expect("finite corners"),
        ],
    };
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("heatsink", tol);
    let insert = |doc: &mut Doc<ProfileProgram>, node| -> RecipeNodeId {
        let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("insert node");
        *doc = applied.doc;
        applied.record.minted.expect("insert mints an id")
    };
    let base_p = insert(&mut doc, Node::Profile(base_profile));
    let base_e = insert(
        &mut doc,
        Node::Extrude {
            profile: base_p,
            distance: pe("250 mm"),
        },
    );
    let fin_p = insert(&mut doc, Node::Profile(fin_profile));
    let fin_e = insert(
        &mut doc,
        Node::Extrude {
            profile: fin_p,
            distance: pe("812.5 mm"),
        },
    );
    // The fin group: ONE node, ONE body out. `placed_union` is the
    // PARAMETRIC-rule constructor, so the count stays a structural slot
    // and the tour's 5 -> 7 -> 9 edit drives it exactly as it drove
    // Pattern's.
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
    );
    // ... and the fold into the base, in the document rather than
    // beside it. No declarations: the fins overlap the base by 1/16, so
    // this is an ordinary transversal union (see the module docs for
    // why that overlap is a dodge and what retires it).
    let solid = insert(
        &mut doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: base_e,
            b: group,
            declare: None,
        },
    );
    Recipe { doc, group, solid }
}

/// The document's OWN final body, read back — no demo-side boolean.
///
/// This function used to BE the union chain (#1344). What is left is
/// the read plus the volume gate the chain used to run per step: the
/// exact dyadic oracle still has to hold of the node's answer, or the
/// group node has quietly changed what it builds.
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
    let want = BASE_VOL + n as f64 * FIN_GAIN;
    let got = pncad::topo::mass_properties(body, tol)
        .expect("mass properties")
        .volume
        .f();
    assert!(
        (got - want).abs() <= 1e-9,
        "the {n}-fin solid measures {got}, and base + {n} fins is {want}"
    );
    ((**body).clone(), (**contacts).clone())
}

/// The recipe evaluated + solidified at every fin count the tour
/// shows (5 → 7 → 9, each re-eval fed the prior as memo) — the Probe
/// sweep records the document-evaluation predicates AND the union
/// chain at every count.
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
    let r = build_doc(tol);
    let cancel = CancelToken::new();
    let opts = EvalOptions::default();
    let ev5 = evaluate::<Probe>(&r.doc, None, &cancel, &opts, tol);
    let mut out = vec![solidify(&r, &ev5, 5, tol)];
    let mut doc = r.doc.clone();
    let mut prior = ev5;
    for n in [7usize, 9] {
        let applied = apply(
            &doc,
            &DocEdit::SetStructuralParam {
                node: r.group,
                slot: SlotId::Count,
                expr: pe(&format!("{n}")),
            },
            tol,
        )
        .expect("count edit");
        doc = applied.doc;
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
    build_doc(tol).doc
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    let r = build_doc(tol);
    let cancel = CancelToken::new();
    let opts = EvalOptions::default();

    // Evaluate at 5, then EDIT the structural count and re-evaluate
    // against the prior — the memo counters are the demo.
    let ev5 = evaluate::<f64>(&r.doc, None, &cancel, &opts, tol);
    let names5 = ev5
        .value(r.group)
        .expect("the fin group @ 5")
        .name_table
        .clone();

    let mut doc = r.doc.clone();
    let mut evs: Vec<(usize, Evaluation<f64>, String)> = Vec::new();
    evs.push((5, ev5, "cold evaluation: all 6 nodes computed".to_string()));
    for (prior_idx, n) in [7usize, 9].into_iter().enumerate() {
        let applied = apply(
            &doc,
            &DocEdit::SetStructuralParam {
                node: r.group,
                slot: SlotId::Count,
                expr: pe(&format!("{n}")),
            },
            tol,
        )
        .expect("count edit");
        doc = applied.doc;
        let ev = evaluate::<f64>(&doc, Some(&evs[prior_idx].1), &cancel, &opts, tol);
        let caption = format!(
            "count edit -> {n}: recomputed {} node(s), reused {} (downstream-only recompute)",
            ev.recomputed, ev.reused
        );
        // TWO nodes, not one, and the second is the point: the count
        // edit re-runs the fin group AND the union that consumes it —
        // which is what it means for the whole part to live in the
        // document now (#1344). Everything upstream of the edited slot
        // — both profiles and both extrudes — is still reused by
        // content key, so this is the same downstream-only-recompute
        // claim measured over a chain that is one node longer, not a
        // weaker one. It read 1 while the union lived in demo code.
        assert_eq!(
            ev.recomputed, 2,
            "a count edit re-runs exactly the fin group and the union below it"
        );
        assert_eq!(ev.reused, 4, "everything upstream reuses by content key");
        evs.push((n, ev, caption));
    }

    // Stable names survive the structural edits (N1 Instance(i)).
    // PIN RE-MEASURED at the PlacedUnion migration (#1344), which is
    // what its own instruction asks for: the count-5 table is read off
    // the GROUP node now rather than off a Pattern, and the group emits
    // `Instance(i)` names over one fused body where the pattern emitted
    // N unfused ones. A moved number here means the naming emission
    // vocabulary moved and wants deciding, not silencing.
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

    let recipe_ops = "ONE recipe doc: Profile -> Extrude (base), Profile -> Extrude (fin) -> \
         LinearPattern(count); count edited via SetStructuralParam";
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
                // Montage carries only the fullest variant (#91
                // revision note 5); 5/7 stay in the tour + standalone.
                montage: n == 9,
                story: "parametric heat-sink strip from ONE recipe document — fin count \
                        is a structural parameter; this render is one evaluation",
                ops: recipe_ops,
                delta: 1e-2,
                note: Some(format!(
                    "{recompute_story}; fins union-inset into the base (1/16 overlap, \
                     volume {} — observed bit-exact, gated 1e-9); the union-to-solid step is demo-side — a \
                     Boolean recipe node cannot consume Pattern Instances today (F4)",
                    BASE_VOL + n as f64 * FIN_GAIN
                )),
                view: View { elev: 24.0, azim: -62.0, up: 'z' },
                bodies: vec![SceneBody::seamed(name, color, body, contacts)],
            }
        })
        .collect()
}
