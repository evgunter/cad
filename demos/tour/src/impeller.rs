//! **The impeller** — the recipe layer's CIRCULAR rule, and one
//! parameter that moves two slots.
//!
//! # What this scene is for
//!
//! `heatsink` next door is the tour's recompute story on
//! `PatternKind::Linear`: one document, a fin count edited 5 → 7 → 9,
//! exactly the nodes downstream of the edit re-run. The circular rule
//! is its sibling and had no scene at all — nothing in the tour placed
//! anything around an axis, and `PlacedUnion::Circular` had no corpus
//! document either.
//!
//! It also does something the linear rule cannot, and that is the
//! reason this scene exists rather than a second comb:
//!
//! **A fan has ONE number, and the recipe layer can say so.** A blade
//! count and an angular step are two slots, and on a comb they are
//! genuinely independent — a fin count and a fin spacing say different
//! things. On a wheel they are not: `N` blades that close the circle
//! step by `360°/N`, and authoring the two separately is authoring the
//! same fact twice. So the count slot holds `blades` and the step
//! holds `360 deg / scalar(blades)`, both against ONE document
//! parameter, and the tour's 6 → 8 → 12 is a single
//! [`DocEdit::SetDocParamValue`] each time.
//!
//! `scalar(n)` is what makes that sayable: a bare `blades` is a
//! `Count`, `Div`'s divisor must be `Scalar`, and the grammar's one
//! promotion is the door between them.
//!
//! # Why the hub is a prism
//!
//! A real impeller's hub is round, and this one is a 24-gon. That is
//! not a stylistic choice: `cylinder ∪ box` refuses
//! `CurvedPierceUnsupported` at the boolean's curved-pierce door, so a
//! round hub cannot have a blade unioned into it at all. The frontier
//! is filed (`work/curved/boolean-refuses-on-arc-carrier-not-arc`) and
//! this scene is one of the parts that meets it; the faceted hub is
//! the modelling the kernel currently permits, said out loud rather
//! than passed off as the part.
//!
//! # The overlap, and why it is here
//!
//! Each blade's root reaches inside the hub's inradius, so the union
//! is an ordinary transversal intersection — `heatsink`'s 1/16 dodge,
//! for `heatsink`'s reason, and the same honest note applies: a real
//! impeller's blades are flush with the hub and a flush one is a
//! declared contact this scene does not author.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use pncad::document::{
    BooleanOp, BooleanValue, CancelToken, Datum, Dimension, Doc, DocEdit, DocParam, DocParamValue,
    EvalOptions, Evaluation, Expr, LoopProgram, Node, ParamName, PatternKind, ProfileProgram,
    RecipeNodeId, ValuePayload, apply, evaluate, parse_expr,
};
use pncad::geom_core::Tol;
use pncad::topo::Body;

use crate::{SceneBody, Stop, View};

/// The hub's facet count. Round enough to read as a hub, and a PRISM
/// because the boolean has no arm for a curved operand its blade could
/// reach.
const HUB_FACETS: usize = 24;
/// The hub's circumradius, metres.
const HUB_R: f64 = 1.0;
/// The hub's height.
const HUB_H: f64 = 0.75;

/// Where a blade's root starts — inside the hub's INRADIUS
/// (`HUB_R·cos(π/24)` ≈ 0.9914), so the union is transversal.
const BLADE_R0: f64 = 0.85;
/// Where a blade's tip ends.
const BLADE_R1: f64 = 2.4;
/// Half the blade's thickness.
const BLADE_T: f64 = 0.11;
/// The blade's height, and its base — sunk into the hub's own slab so
/// the union has material on both sides of every wall it crosses.
const BLADE_Z0: f64 = 0.15;
const BLADE_H: f64 = 0.45;

/// The blade counts the scene evaluates, in order.
///
/// **Every one divides [`HUB_FACETS`], and that is load-bearing.** A
/// 24-gon hub has 15° rotational symmetry, not the full circle's, so
/// where a blade's root crosses the hub's boundary depends on its
/// clocking — and a count that does not divide 24 puts its blades at
/// angles the hub does not repeat at. 6, 8 and 12 step by 60°, 45°
/// and 30°, all multiples of 15°, so every blade of every count meets
/// the hub identically and the volume is EXACTLY linear in the count.
/// Measured, not assumed: 5 blades (72°, not a multiple of 15°) breaks
/// the linearity at 8e-5 relative, which is the faceted hub's own
/// asymmetry showing up in a number.
const COUNTS: [i64; 3] = [6, 8, 12];

/// The scene's chordal deviation.
const DELTA: f64 = 4e-3;

/// The document, plus the two node ids the narration reads.
struct Recipe {
    doc: Doc<ProfileProgram>,
    group: RecipeNodeId,
    solid: RecipeNodeId,
}

/// The parameter table every expression in this document resolves
/// against — one entry, which is the scene's whole point.
fn params() -> BTreeMap<ParamName, Dimension> {
    [(ParamName::new("blades"), Dimension::Count)]
        .into_iter()
        .collect()
}

/// Parse against [`params`]: every expression here may name `blades`.
fn pe(src: &str) -> Expr {
    parse_expr(src, &params()).expect("the impeller's expressions parse")
}

/// The hub's cross-section: a regular [`HUB_FACETS`]-gon of
/// circumradius [`HUB_R`].
fn hub_polygon() -> LoopProgram {
    #[allow(clippy::cast_precision_loss)]
    let pts: Vec<(f64, f64)> = (0..HUB_FACETS)
        .map(|i| {
            let a = std::f64::consts::TAU * i as f64 / HUB_FACETS as f64;
            (HUB_R * a.cos(), HUB_R * a.sin())
        })
        .collect();
    LoopProgram::polygon(pts).expect("the hub's corners are finite")
}

/// One blade, in the xy plane: a rectangle reaching from inside the
/// hub out to the tip.
fn blade_polygon() -> LoopProgram {
    LoopProgram::polygon([
        (BLADE_R0, -BLADE_T),
        (BLADE_R1, -BLADE_T),
        (BLADE_R1, BLADE_T),
        (BLADE_R0, BLADE_T),
    ])
    .expect("the blade's corners are finite")
}

fn build_doc(tol: Tol) -> Recipe {
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("impeller", tol);
    let insert = |doc: &mut Doc<ProfileProgram>, node| -> RecipeNodeId {
        let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("insert node");
        *doc = applied.doc;
        applied.record.minted.expect("insert mints an id")
    };

    // The ONE parameter. Declared `Count`, which is what lets it drive
    // a structural slot at all (spec D3: a count is structural
    // material).
    let applied = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("blades"),
            value: DocParam::Count { value: COUNTS[0] },
        },
        tol,
    )
    .expect("the blade count declares");
    doc = applied.doc;

    let len = |v: f64| Expr::literal(v, Dimension::Length).expect("finite");
    let scl = |v: f64| Expr::literal(v, Dimension::Scalar).expect("finite");
    let frame_at = |z: f64| {
        Node::Datum(Datum::Frame {
            origin: [len(0.0), len(0.0), len(z)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        })
    };

    let hub_plane = insert(&mut doc, frame_at(0.0));
    let hub_p = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            plane: hub_plane,
            loops: vec![hub_polygon()],
        }),
    );
    let hub_e = insert(
        &mut doc,
        Node::Extrude {
            profile: hub_p,
            distance: len(HUB_H),
        },
    );

    let blade_plane = insert(&mut doc, frame_at(BLADE_Z0));
    let blade_p = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            plane: blade_plane,
            loops: vec![blade_polygon()],
        }),
    );
    let blade_e = insert(
        &mut doc,
        Node::Extrude {
            profile: blade_p,
            distance: len(BLADE_H),
        },
    );

    // The axis the blades step about: the hub's own.
    let axis = insert(
        &mut doc,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );

    // **The two slots, one parameter.** The count IS `blades`; the
    // step is `360°/blades` through the grammar's Count→Scalar
    // promotion. Nothing downstream has to be told the blades moved.
    let group = insert(
        &mut doc,
        Node::placed_union(
            blade_e,
            pe("blades"),
            PatternKind::Circular {
                axis,
                step: pe("360 deg / scalar(blades)"),
            },
        )
        .expect("a Circular rule is parametric, so it carries a count"),
    );
    let solid = insert(
        &mut doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: hub_e,
            b: group,
            declare: None,
        },
    );
    Recipe { doc, group, solid }
}

/// The document's own final body, read back.
fn body_of(ev: &Evaluation<f64>, at: RecipeNodeId) -> Body<f64> {
    match &ev.value(at).expect("the node evaluated").payload {
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body.as_ref().clone(),
        other => panic!("the impeller's root is a union, got {other:?}"),
    }
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    let r = build_doc(tol);
    let cancel = CancelToken::new();
    let opts = EvalOptions::default();

    let mut doc = r.doc.clone();
    let mut evs: Vec<(i64, Evaluation<f64>, String)> = Vec::new();
    let first = evaluate::<f64>(&doc, None, &cancel, &opts, tol);
    let names_at_first = first
        .value(r.group)
        .expect("the blade group evaluates")
        .name_table
        .len();
    evs.push((
        COUNTS[0],
        first,
        format!("cold evaluation at {} blades", COUNTS[0]),
    ));

    for (prior, &n) in COUNTS.iter().skip(1).enumerate() {
        // **ONE edit.** Not a count edit and a step edit: the two slots
        // read the same parameter, so the document's own arithmetic
        // moves the step when the count moves.
        let applied = apply(
            &doc,
            &DocEdit::SetDocParamValue {
                name: ParamName::new("blades"),
                value: DocParamValue::Count(n),
            },
            tol,
        )
        .expect("the blade count edits");
        doc = applied.doc;
        let ev = evaluate::<f64>(&doc, Some(&evs[prior].1), &cancel, &opts, tol);
        let caption = format!(
            "blades -> {n} in ONE parameter edit: recomputed {} node(s), reused {}",
            ev.recomputed, ev.reused
        );
        evs.push((n, ev, caption));
    }

    // **The name table is EXACTLY linear in the count**, which is the
    // readable form of "one blade's names per blade". It is not
    // proportional: at 6 blades the group carries 157 names, and 157
    // is not a multiple of 6 — there is a name the group has once
    // rather than once per instance. So what is asserted is the shape
    // that claim really has, a slope and an intercept, with the second
    // difference exactly zero.
    let names: Vec<usize> = evs
        .iter()
        .map(|(_, ev, _)| {
            ev.value(r.group)
                .expect("the blade group evaluates")
                .name_table
                .len()
        })
        .collect();
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let (d0, d1) = (COUNTS[1] - COUNTS[0], COUNTS[2] - COUNTS[1]);
    let per_blade_names = (names[1] - names[0]) / usize::try_from(d0).expect("positive");
    assert_eq!(
        (names[2] - names[1]) / usize::try_from(d1).expect("positive"),
        per_blade_names,
        "the group's names are linear in the count: {names:?} over {COUNTS:?}"
    );
    let name_intercept = names[0] - per_blade_names * usize::try_from(COUNTS[0]).expect("positive");
    let _ = names_at_first;

    // **The volume is additive, exactly** — and only because every
    // count divides the hub's facet count (see [`COUNTS`]). Every
    // blade then meets the hub in the SAME transversal intersection,
    // and no two blades meet at all (`PlacedUnion` certifies the
    // disjointness), so the solid's volume is the hub's plus N times
    // one blade's protruding part. Read off the bodies rather than
    // derived from the sketch: what is asserted is that the kernel's
    // own numbers are linear in the count with no residue.
    let vols: Vec<f64> = evs
        .iter()
        .map(|(_, ev, _)| {
            pncad::topo::mass_properties(&body_of(ev, r.solid), tol)
                .expect("the impeller has a volume")
                .volume
        })
        .collect();
    #[allow(clippy::cast_precision_loss)]
    let per_blade = (vols[1] - vols[0]) / (COUNTS[1] - COUNTS[0]) as f64;
    #[allow(clippy::cast_precision_loss)]
    let predicted = (COUNTS[2] - COUNTS[1]) as f64 * per_blade + vols[1];
    assert!(
        ((vols[2] - predicted) / vols[2]).abs() < 1e-12,
        "V at {} blades is {} against {predicted} predicted from the {} -> {} step \
         ({per_blade} per blade)",
        COUNTS[2],
        vols[2],
        COUNTS[0],
        COUNTS[1]
    );

    // Tier 3 on every one of them.
    for (n, ev, _) in &evs {
        assert_eq!(
            pncad::topo::validate_geometric(&body_of(ev, r.solid), tol),
            Ok(()),
            "the impeller at {n} blades: tier 3"
        );
    }

    let (_, _, cap_last) = evs.last().expect("three evaluations");
    let recompute_note = evs
        .iter()
        .skip(1)
        .map(|(_, _, c)| c.clone())
        .collect::<Vec<_>>()
        .join("; ");
    let _ = cap_last;

    evs.iter()
        .map(|(n, ev, caption)| {
            let body = body_of(ev, r.solid);
            let vol = pncad::topo::mass_properties(&body, tol)
                .expect("volume")
                .volume;
            Stop {
                name: Box::leak(format!("impeller{n}").into_boxed_str()),
                caption: caption.clone(),
                montage: *n == COUNTS[2],
                story: "ONE recipe document: a 24-gon hub, one blade, and a \
                        `PlacedUnion` that places the blade around the hub's own axis \
                        and fuses the group into ONE body, folded into the hub by a \
                        `Boolean` beside it. The blade COUNT and the angular STEP are \
                        two slots and they read the SAME document parameter — the \
                        count is `blades`, the step is `360 deg / scalar(blades)` — so \
                        the tour's 6 -> 8 -> 12 is one `SetDocParamValue` each time \
                        and the blades still close the circle. A comb's count and \
                        spacing are genuinely independent; a wheel's are not, and the \
                        recipe layer can say which it is. The hub is a PRISM because \
                        `cylinder u box` refuses at the boolean's curved-pierce door, \
                        so a round hub cannot have a blade unioned into it at all",
                ops: "Datum::Frame x2 -> Profile(24-gon) -> Extrude; Profile(blade) -> \
                      Extrude; Datum::Axis -> Node::placed_union(blade, count = \
                      blades, Circular { axis, step = 360 deg / scalar(blades) }) -> \
                      Boolean(Union) with the hub; then DocEdit::SetDocParamValue",
                delta: DELTA,
                note: Some(format!(
                    "{recompute_note}. V = {vol:.9} m^3 at {n} blades, and the three \
                     volumes are EXACTLY linear in the count: every blade meets the \
                     hub in the same transversal intersection and no two blades meet \
                     at all, so the 6 -> 8 step prices a blade and predicts the 8 -> \
                     12 volume to within 1e-12 relative — exactly, because 6, 8 and \
                     12 all divide the hub's 24 facets, so every blade meets it at a \
                     clocking the hub repeats at. A count that does not (5 blades, at \
                     72 degrees) breaks that linearity at 8e-5 relative, which is the \
                     faceted hub's own asymmetry in a number. The group's stable names are \
                     EXACTLY linear in the count — {per_blade_names} per blade plus \
                     {name_intercept} the group carries once rather than once per \
                     instance — so the instances stay addressable across every edit"
                )),
                view: View {
                    elev: 34.0,
                    azim: -55.0,
                    up: 'z',
                },
                bodies: vec![SceneBody::plain(
                    Box::leak(format!("impeller{n}").into_boxed_str()),
                    [0.66, 0.58, 0.44],
                    body,
                )],
            }
        })
        .collect()
}
