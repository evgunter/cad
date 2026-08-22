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
//! The pattern node yields placed instance BODIES (`transform_rigid`
//! placements, key-stable); the printable single solid per variant is
//! made by sequential inset-overlap unions of the fins into the base
//! (the table-leg pattern, 1/16 overlap — flush fin bases would
//! refuse). F4 note, probed 2026-07-25: a Boolean recipe node cannot
//! consume a Pattern node's `Instances` payload today, so the
//! union-to-one-solid step lives HERE in demo code, honestly outside
//! the document.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use pncad::document::{
    CancelToken, Doc, DocEdit, EvalOptions, Evaluation, Expr, LoopProgram, Node, PatternKind,
    ProfileProgram, RecipeNodeId, SlotId, ValuePayload, apply, evaluate, parse_expr,
};
use pncad::geom_core::{Point3, Vec3};
use pncad::profile::SketchPlane;

use crate::booleans::{check, expect_seamed, try_union};
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

const BASE_VOL: f64 = 3.0 * 1.0 * 0.25;
/// Per-fin material gain: 0.1875 x 0.75 footprint, 0.8125 tall, minus
/// the 1/16 slice overlapping into the base.
const FIN_GAIN: f64 = 0.1875 * 0.75 * (0.8125 - 0.0625);

struct Recipe {
    doc: Doc<ProfileProgram>,
    base_e: RecipeNodeId,
    pattern: RecipeNodeId,
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
    let pattern = insert(
        &mut doc,
        Node::Pattern {
            input: fin_e,
            count: pe("5"),
            kind: PatternKind::Linear {
                direction: [pe("1.0"), pe("0.0"), pe("0.0")],
                spacing: pe("312.5 mm"),
            },
        },
    );
    Recipe {
        doc,
        base_e,
        pattern,
    }
}

/// Unions the pattern's fin instances into the base — one solid, exact
/// volume after every union (demo-side; see module docs).
fn solidify<S: Scalar>(
    r: &Recipe,
    ev: &Evaluation<S>,
    n: usize,
    tol: Tol,
) -> pncad::topo::BooleanBody<S> {
    let base = match &ev.value(r.base_e).expect("base evaluated").payload {
        ValuePayload::Body(b) => (**b).clone(),
        other => panic!("base payload: {other:?}"),
    };
    let fins = match &ev.value(r.pattern).expect("pattern evaluated").payload {
        ValuePayload::Instances(v) => v.clone(),
        other => panic!("pattern payload: {other:?}"),
    };
    assert_eq!(fins.len(), n, "pattern instance count");
    let mut acc = base;
    let mut vol = BASE_VOL;
    let mut last: Option<pncad::topo::BooleanBody<S>> = None;
    for (i, fin) in fins.iter().enumerate() {
        vol += FIN_GAIN;
        let bb = expect_seamed(
            &format!("fin[{i}] union"),
            check(try_union(&acc, fin, tol), vol, tol),
            vol,
        );
        acc = bb.body.clone();
        last = Some(bb);
    }
    last.expect("at least one fin")
}

/// The recipe evaluated + solidified at every fin count the tour
/// shows (5 → 7 → 9, each re-eval fed the prior as memo), generic —
/// the Probe sweep records the document-evaluation predicates AND the
/// union chain at every count.
/// Only `crate::probe` calls this, so it rides the `probe` feature with
/// it — otherwise a default build trips `dead_code` under CI's
/// `-D warnings`.
#[cfg(feature = "probe")]
pub(crate) fn probe_solids<S: Scalar>(tol: Tol) -> Vec<pncad::topo::BooleanBody<S>> {
    let r = build_doc(tol);
    let cancel = CancelToken::new();
    let opts = EvalOptions::default();
    let ev5 = evaluate::<S>(&r.doc, None, &cancel, &opts, tol);
    let mut out = vec![solidify(&r, &ev5, 5, tol)];
    let mut doc = r.doc.clone();
    let mut prior = ev5;
    for n in [7usize, 9] {
        let applied = apply(
            &doc,
            &DocEdit::SetStructuralParam {
                node: r.pattern,
                slot: SlotId::Count,
                expr: pe(&format!("{n}")),
            },
            tol,
        )
        .expect("count edit");
        doc = applied.doc;
        let ev = evaluate::<S>(&doc, Some(&prior), &cancel, &opts, tol);
        out.push(solidify(&r, &ev, n, tol));
        prior = ev;
    }
    out
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    let r = build_doc(tol);
    let cancel = CancelToken::new();
    let opts = EvalOptions::default();

    // Evaluate at 5, then EDIT the structural count and re-evaluate
    // against the prior — the memo counters are the demo.
    let ev5 = evaluate::<f64>(&r.doc, None, &cancel, &opts, tol);
    let names5 = ev5.value(r.pattern).expect("pattern@5").name_table.clone();

    let mut doc = r.doc.clone();
    let mut evs: Vec<(usize, Evaluation<f64>, String)> = Vec::new();
    evs.push((5, ev5, "cold evaluation: all 5 nodes computed".to_string()));
    for (prior_idx, n) in [7usize, 9].into_iter().enumerate() {
        let applied = apply(
            &doc,
            &DocEdit::SetStructuralParam {
                node: r.pattern,
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
        assert_eq!(
            ev.recomputed, 1,
            "a count edit re-runs exactly the pattern node"
        );
        assert_eq!(ev.reused, 4, "everything upstream reuses by content key");
        evs.push((n, ev, caption));
    }

    // Stable names survive the structural edits (N1 Instance(i)).
    assert_eq!(
        names5.len(),
        135,
        "the count-5 pattern name table is pinned at 135 entries; a \
         change means the naming emission vocabulary moved - update \
         this pin deliberately"
    );
    let names9 = &evs[2].1.value(r.pattern).expect("pattern@9").name_table;
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
            let bb = solidify(&r, &ev, n, tol);
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
                bodies: vec![SceneBody::seamed(name, color, bb.body, bb.contacts)],
            }
        })
        .collect()
}
