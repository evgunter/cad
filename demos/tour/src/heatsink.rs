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

use editor_core::{
    CancelToken, Dimension, Doc, DocEdit, EvalOptions, Evaluation, Expr, Node, PatternKind,
    ProfileDesc, RecipeNodeId, SlotId, ValuePayload, apply, evaluate,
};
use geom_core::{Point3, Vec3};
use profile::{Profile, ProfileLoop, SketchPlane};

use crate::booleans::{check, expect_seamed, try_union};
use crate::{SceneBody, Stop, View};

fn p2(x: f64, y: f64) -> geom_core::Point2<f64> {
    geom_core::Point2::new(x, y)
}

const BASE_VOL: f64 = 3.0 * 1.0 * 0.25;
/// Per-fin material gain: 0.1875 x 0.75 footprint, 0.8125 tall, minus
/// the 1/16 slice overlapping into the base.
const FIN_GAIN: f64 = 0.1875 * 0.75 * (0.8125 - 0.0625);

struct Recipe {
    doc: Doc<ProfileDesc>,
    base_e: RecipeNodeId,
    pattern: RecipeNodeId,
}

fn build_doc() -> Recipe {
    let base_profile = Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::polygon([
            p2(0.0, 0.0),
            p2(3.0, 0.0),
            p2(3.0, 1.0),
            p2(0.0, 1.0),
        ])],
    );
    // Fin sketch sits at z = 0.1875 — 1/16 INSIDE the 0.25-thick base.
    let fin_plane = SketchPlane::from_frame(
        Point3::new(0.0, 0.0, 0.1875),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let fin_profile = Profile::new(
        fin_plane,
        vec![ProfileLoop::polygon([
            p2(0.25, 0.125),
            p2(0.4375, 0.125),
            p2(0.4375, 0.875),
            p2(0.25, 0.875),
        ])],
    );
    let mut doc: Doc<ProfileDesc> = Doc::empty();
    let insert = |doc: &mut Doc<ProfileDesc>, node| -> RecipeNodeId {
        let applied = apply(doc, &DocEdit::InsertNode { node }).expect("insert node");
        *doc = applied.doc;
        applied.record.minted.expect("insert mints an id")
    };
    let base_p = insert(&mut doc, Node::Profile(ProfileDesc(base_profile)));
    let base_e = insert(
        &mut doc,
        Node::Extrude {
            profile: base_p,
            distance: Expr::literal(0.25, Dimension::Length).unwrap(),
        },
    );
    let fin_p = insert(&mut doc, Node::Profile(ProfileDesc(fin_profile)));
    let fin_e = insert(
        &mut doc,
        Node::Extrude {
            profile: fin_p,
            distance: Expr::literal(0.8125, Dimension::Length).unwrap(),
        },
    );
    let scalar = |v: f64| Expr::literal(v, Dimension::Scalar).unwrap();
    let pattern = insert(
        &mut doc,
        Node::Pattern {
            input: fin_e,
            count: Expr::count(5),
            kind: PatternKind::Linear {
                direction: [scalar(1.0), scalar(0.0), scalar(0.0)],
                spacing: Expr::literal(0.3125, Dimension::Length).unwrap(),
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
fn solidify(r: &Recipe, ev: &Evaluation<f64>, n: usize) -> topo::BooleanBody<f64> {
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
    let mut last: Option<topo::BooleanBody<f64>> = None;
    for (i, fin) in fins.iter().enumerate() {
        vol += FIN_GAIN;
        let bb = expect_seamed(
            &format!("fin[{i}] union"),
            check(try_union(&acc, fin), vol),
            vol,
        );
        acc = bb.body.clone();
        last = Some(bb);
    }
    last.expect("at least one fin")
}

pub fn stops() -> Vec<Stop> {
    let r = build_doc();
    let cancel = CancelToken::new();
    let opts = EvalOptions::default();

    // Evaluate at 5, then EDIT the structural count and re-evaluate
    // against the prior — the memo counters are the demo.
    let ev5 = evaluate::<f64>(&r.doc, None, &cancel, &opts);
    let names5 = ev5.value(r.pattern).expect("pattern@5").name_table.clone();

    let mut doc = r.doc.clone();
    let mut evs: Vec<(usize, Evaluation<f64>, String)> = Vec::new();
    evs.push((
        5,
        ev5,
        format!("cold evaluation: all 5 nodes computed"),
    ));
    let mut prior_idx = 0;
    for n in [7usize, 9] {
        let applied = apply(
            &doc,
            &DocEdit::SetStructuralParam {
                node: r.pattern,
                slot: SlotId::Count,
                expr: Expr::count(n as i64),
            },
        )
        .expect("count edit");
        doc = applied.doc;
        let ev = evaluate::<f64>(&doc, Some(&evs[prior_idx].1), &cancel, &opts);
        let caption = format!(
            "count edit -> {n}: recomputed {} node(s), reused {} (downstream-only recompute)",
            ev.recomputed, ev.reused
        );
        assert_eq!(ev.recomputed, 1, "a count edit re-runs exactly the pattern node");
        assert_eq!(ev.reused, 4, "everything upstream reuses by content key");
        evs.push((n, ev, caption));
        prior_idx += 1;
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

    let recipe_ops =
        "ONE recipe doc: Profile -> Extrude (base), Profile -> Extrude (fin) -> \
         LinearPattern(count); count edited via SetStructuralParam";
    let colors = [[0.45, 0.62, 0.62], [0.38, 0.58, 0.68], [0.32, 0.54, 0.74]];
    evs.into_iter()
        .zip(colors)
        .map(|((n, ev, recompute_story), color)| {
            let bb = solidify(&r, &ev, n);
            let name: &'static str = match n {
                5 => "heatsink5",
                7 => "heatsink7",
                _ => "heatsink9",
            };
            Stop {
                name,
                caption: format!("heat sink ({n} fins)"),
                montage: true,
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
