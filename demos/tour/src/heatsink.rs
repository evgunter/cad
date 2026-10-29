//! The parametric heat-sink strip (#91 C5): the tour's first M4-layer
//! showcase. ONE editor-core recipe document — base extrude, a fin
//! extrude, a row of the fin — one placement over the index
//! `k = index(5)`, fin `k` slid `scalar(k)·312.5 mm` (D10
//! `[repetition]`) — a `Union` folding the base and the fin family into
//! one body, and a `Fillet` rounding the base's twelve edges on the
//! unioned part — evaluated three times with the fin count edited
//! 5 → 7 → 9 by redefining the index's count. Each re-eval feeds the
//! PRIOR evaluation as the memo, so the caption's recompute counters are
//! the downstream-only-recompute story: the count edit re-runs the row,
//! the union and the fillet below it, and the five nodes upstream of the
//! index — both profiles, both extrudes and the fin's placement on the
//! base — are reused by content key. Stable names (each fin's names
//! under its `Member(k)`) survive the edits, counted live.
//!
//! # Sunk fins, and flush ones ([`flush_fins`], run live)
//!
//! **The fins sit 1/16 inside the base rather than flush on it** — "the
//! table-leg pattern", a transversal union instead of a face contact.
//! A real extruded heat sink's fins are flush with its base, and the
//! flush document builds: fins sketched ON the base top, the union
//! glues the five base-face contacts on their decided zero (D10,
//! Booleans), and the union against the five-member fin family comes
//! out at the closed-form volume of the rounded base + 5
//! fins.
//!
//! The count EDIT builds in one step too: the edit to 7 leaves
//! `Member(5)` and `Member(6)` flush with the base, and the union
//! glues them alike, at the closed-form volume of 7 fins with the
//! union keeping its id. The
//! scene keeps the sunk fins as its subject, and measures the flush
//! document's edit beside them.
//!
//! # The base is rounded after the union
//!
//! **The fillet is the last node**, so the count edit re-runs it on the
//! part it changed. Its twelve edges are the union's edges that came
//! through from the plate's read, selected once at five fins and
//! stored by name. The fins' feet are rectangular rings of the plate's
//! top face, and the blend meters each ring edge against each band's
//! trimline before it carves: the fins stand 1/8 inside the long edges
//! and 1/4 inside the near end, so at r = 1/32 every foot clears every
//! trimline.
//!
//! The radius is under 1/16 because the ninth fin's outer wall stands
//! 1/16 inside the base's end face: at r = 1/16 that wall's foot lies on
//! the end band's trimline, and the nine-fin fillet refuses
//! `FaceClearanceUncertified` at a margin of zero (the sampled screen
//! answers before the exact ring meter, because the foot's edge runs
//! parallel to the band).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::prelude::AuthoredNode;
use std::collections::BTreeMap;

use pncad::document::{
    Bodies, BodyRead, CancelToken, Doc, DocEdit, EvalOptions, Evaluation, Formula, LoopProgram,
    Node, Operand, ProfileProgram, RecipeNodeId, TargetAxis, ValuePayload, VarDecl, VarId, VarName,
    VarRef, apply, evaluate, parse_formula,
};
// `probe_solids` is the only scene door pinned to the recording scalar
// (see its note), and it rides the `probe` feature with it.
#[cfg(feature = "probe")]
use pncad::geom_core::Probe;
use pncad::select::{EntityKind, NamePat, SegPat, SegTag, Selector, select};

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
fn pe(src: &str) -> Formula {
    parse_formula(src, &BTreeMap::new()).expect("tour expression")
}
use crate::{SceneBody, TourStop, View};
use pncad::geom_core::Tol;

/// The count-5 name table's size, pinned: the fin family's table is
/// each fin's names under its `Member(k)` and nothing family-wide, so it
/// is five times one seated fin's 27. Moving it is the naming
/// vocabulary moving — see the assertion below.
const HEATSINK_NAMES_AT_5: usize = 135;
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
    /// 1/16 inside the base: a transversal union.
    Sunk,
    /// On the base top, every foot a flush contact the union glues.
    Flush,
}

pub struct Recipe {
    pub doc: Doc<ProfileProgram>,
    /// The placement whose copy is the fin family.
    group: RecipeNodeId,
    /// The index `k = index(n)` the fins are copied over: its count is
    /// the fin count, and what the count edit redefines.
    fins: VarId,
    /// The `Fillet` that rounds the plate's edges on the unioned part:
    /// the document's final body.
    pub solid: RecipeNodeId,
}

fn insert(doc: &mut Doc<ProfileProgram>, node: AuthoredNode, tol: Tol) -> RecipeNodeId {
    let applied = apply(
        doc,
        &DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        tol,
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

/// The fin-count edit, the one the tour's story is about: the index
/// redefined at count `n`.
fn set_count(doc: &Doc<ProfileProgram>, fins: VarId, n: usize, tol: Tol) -> Doc<ProfileProgram> {
    apply(
        doc,
        &DocEdit::DefineVar {
            var: VarRef::Id(fins),
            def: VarDecl::Index {
                count: pe(&format!("{n}")),
                within: None,
            },
            fresh: Vec::new(),
        },
        tol,
    )
    .expect("count edit")
    .doc
}

/// The scene's own document: fins sunk.
pub fn scene_doc(tol: Tol) -> Recipe {
    build_doc(tol, Seat::Sunk)
}

/// Builds the heat sink at five fins. Where a stored selection is
/// needed — the fillet's edges — it is taken
/// from an evaluation of the document so far: evaluate, select, store.
fn build_doc(tol: Tol, seat: Seat) -> Recipe {
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
    let base_p = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            loops: base_loops,
            ids: Vec::new(),
            on: None,
        }),
        tol,
    );
    let base = insert(
        &mut doc,
        Node::Extrude {
            profile: base_p.into(),
            distance: pe("250 mm"),
        },
        tol,
    );
    let (fin_z, fin_height) = match seat {
        Seat::Sunk => (0.1875, "812.5 mm"),
        Seat::Flush => (0.25, "750 mm"),
    };
    let fin_p = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            loops: fin_loops,
            ids: Vec::new(),
            on: None,
        }),
        tol,
    );
    let fin_e = insert(
        &mut doc,
        Node::Extrude {
            profile: fin_p.into(),
            distance: pe(fin_height),
        },
        tol,
    );
    // The fin is drawn on its own `xy`; its copy seated `fin_z` up the
    // base is the fin the pattern repeats, in the base's space.
    let seated = crate::world::against(
        &doc,
        base,
        fin_e,
        crate::world::slides([0.0, 0.0, fin_z]),
        tol,
    );
    doc = seated.doc;
    let fin_e = seated.placement;
    // One fin per value of `k = index(5)`, fin `k` slid
    // `scalar(k)·312.5 mm` along the base's x.
    let k = VarName::new("k").expect("an index name");
    doc = apply(
        &doc,
        &pncad::pattern::index_decl(k.clone(), pe("5"), None),
        tol,
    )
    .expect("the index declares")
    .doc;
    let fins = doc.var_named("k").expect("the index just declared");
    let space = doc
        .node_space(fin_e)
        .expect("the seated fin is in the base's space");
    let group = insert(
        &mut doc,
        pncad::pattern::linear_pattern(
            &k,
            BodyRead::plain(Operand::Node(fin_e)),
            Operand::Var(space),
            &[(TargetAxis::X, pe("312.5 mm"))],
        )
        .expect("the pitch is a length"),
        tol,
    );
    let union = insert(
        &mut doc,
        Node::Union {
            members: Bodies::Spelled(vec![base.into(), group.into()]),
        },
        tol,
    );
    // The plate's twelve edges on the unioned part: the union's edges
    // that came through from the plate's read.
    let edges = select(
        &eval(&doc, None, tol),
        union,
        &Selector::of(
            NamePat::of_kind(EntityKind::Edge)
                .seg(SegPat::tag(SegTag::From).of([NamePat::any().node(base)])),
        ),
    );
    assert_eq!(edges.len(), 12, "the plate's edges: {edges:#?}");
    let solid = insert(&mut doc, Node::fillet(union, pe(RADIUS), edges), tol);
    Recipe {
        doc,
        group,
        fins,
        solid,
    }
}

/// The document's OWN final body, read back, gated on the exact
/// volume: the rounded plate's closed form plus `n` fins.
pub fn solidify<S: Scalar>(
    r: &Recipe,
    ev: &Evaluation<S>,
    n: usize,
    tol: Tol,
) -> pncad::topo::Body<S> {
    let value = ev.value(r.solid).expect("the fillet node evaluated");
    let ValuePayload::Body(body) = &value.payload else {
        panic!("fillet payload: {:?}", value.payload);
    };
    let body = &crate::world::in_its_space(body, tol);
    let want = volume(n);
    let got =
        pncad::topo::mass_properties(&crate::booleans::at_rest(body, tol), tol, crate::printed())
            .expect("mass properties")
            .enclosure
            .volume
            .f();
    assert!(
        (got - want).abs() <= 1e-9,
        "the {n}-fin solid measures {got}, and the rounded base + {n} fins is {want}"
    );
    pncad::topo::validate_geometric(body, tol)
        .unwrap_or_else(|e| panic!("the {n}-fin solid is tier-3 valid, got {e:?}"));
    body.clone()
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
pub(crate) fn probe_solids(tol: Tol) -> Vec<pncad::topo::Body<Probe>> {
    let r = scene_doc(tol);
    let cancel = CancelToken::new();
    let opts = EvalOptions::default();
    let ev5 = evaluate::<Probe>(&r.doc, None, &cancel, &opts, tol);
    let mut out = vec![solidify(&r, &ev5, 5, tol)];
    let mut doc = r.doc.clone();
    let mut prior = ev5;
    for n in [7usize, 9] {
        doc = set_count(&doc, r.fins, n, tol);
        let ev = evaluate::<Probe>(&doc, Some(&prior), &cancel, &opts, tol);
        out.push(solidify(&r, &ev, n, tol));
        prior = ev;
    }
    out
}

/// This scene's recipe, as a document the GUI can open: the finished
/// heat sink, placed in the world.
///
/// The same `build_doc` the stops walk — the gallery must not be a
/// second authoring of the scene, or it would stop being evidence
/// about this one.
pub fn gallery_document(tol: Tol) -> Doc<ProfileProgram> {
    let r = scene_doc(tol);
    let placed = crate::world::place_in_world(&r.doc, r.solid, tol);
    crate::world::set_product(
        &placed.doc,
        &[pncad::document::Operand::Node(placed.placement)],
        tol,
    )
    .doc
}

/// Flush fins, measured live — the module docs' first section: the
/// count edit builds the flush document in one edit.
fn flush_fins(tol: Tol) {
    let flush = build_doc(tol, Seat::Flush);
    let ev5 = eval(&flush.doc, None, tol);
    solidify(&flush, &ev5, 5, tol);
    println!(
        "   flush fins, 5 Rest contacts: the union glues them and builds, volume {} (gated 1e-9)",
        volume(5)
    );

    let doc7 = set_count(&flush.doc, flush.fins, 7, tol);
    let ev7 = eval(&doc7, Some(&ev5), tol);
    let edited = Recipe { doc: doc7, ..flush };
    solidify(&edited, &ev7, 7, tol);
    println!(
        "   flush fins, count edited 5 -> 7: the union glues the two new feet and \
         builds, volume {} (one edit; recomputed {}, reused {}; the union keeps its id)",
        volume(7),
        ev7.recomputed,
        ev7.reused
    );
}

pub fn stops(tol: Tol) -> Vec<TourStop> {
    let r = scene_doc(tol);

    // Evaluate at 5, then EDIT the structural count and re-evaluate
    // against the prior — the memo counters are the demo.
    let ev5 = eval(&r.doc, None, tol);
    assert_eq!(
        ev5.recomputed,
        r.doc.ids().len(),
        "a cold evaluation computes every node"
    );
    let names5 = ev5.table_at(r.group).expect("the fin group @ 5").clone();

    let mut doc = r.doc.clone();
    let mut evs: Vec<(usize, Evaluation<f64>, String)> = Vec::new();
    let cold = format!("cold evaluation: all {} nodes computed", ev5.recomputed);
    evs.push((5, ev5, cold));
    for (prior_idx, n) in [7usize, 9].into_iter().enumerate() {
        doc = set_count(&doc, r.fins, n, tol);
        let ev = eval(&doc, Some(&evs[prior_idx].1), tol);
        let caption = format!(
            "count edit -> {n}: recomputed {} node(s), reused {} (downstream-only recompute)",
            ev.recomputed, ev.reused
        );
        // The fin family, the union that consumes it and the fillet on
        // the union re-run; both profiles, both extrudes and the fin's
        // placement read no index and reuse by content key.
        assert_eq!(
            ev.recomputed, 3,
            "a count edit re-runs exactly the fin group, the union and the fillet below it"
        );
        assert_eq!(
            ev.reused,
            doc.ids().len() - 3,
            "everything upstream reuses by content key"
        );
        evs.push((n, ev, caption));
    }

    // Stable names survive the count edits (each fin's under its Member(k)).
    assert_eq!(
        names5.len(),
        HEATSINK_NAMES_AT_5,
        "the count-5 fin-group name table is pinned at {HEATSINK_NAMES_AT_5} entries; a \
         change means the naming emission vocabulary moved - update \
         this pin deliberately"
    );
    let names9 = evs[2].1.table_at(r.group).expect("the fin group @ 9");
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
         after both edits (each fin's names under its Member(k))",
        names5.len()
    );
    flush_fins(tol);

    let recipe_ops = "ONE recipe doc: Profile -> Extrude (base), Profile -> Extrude (fin) -> \
         Place over k = index(n) (a row) -> Union -> Fillet (the base's edges); count edited \
         by redefining the index";
    let colors = [[0.45, 0.62, 0.62], [0.38, 0.58, 0.68], [0.32, 0.54, 0.74]];
    evs.into_iter()
        .zip(colors)
        .map(|((n, ev, recompute_story), color)| {
            let body = solidify(&r, &ev, n, tol);
            let name: &'static str = match n {
                5 => "heatsink5",
                7 => "heatsink7",
                _ => "heatsink9",
            };
            TourStop {
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
                        is an index's count; this render is one evaluation",
                ops: recipe_ops,
                delta: 1e-2,
                note: Some(format!(
                    "{recompute_story}; fins sunk 1/16 into a base rounded at r = 1/32 \
                     after the union (volume {} = the rounded plate's closed form + {n} fins, \
                     gated 1e-9); flush fins build too, and a count edit's new feet glue \
                     on their decided zero",
                    volume(n)
                )),
                view: View {
                    elev: 24.0,
                    azim: -62.0,
                    up: 'z',
                },
                bodies: vec![SceneBody::plain(name, color, body).named(&ev, r.solid)],
            }
        })
        .collect()
}
