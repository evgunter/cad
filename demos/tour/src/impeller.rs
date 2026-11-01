//! **The impeller** — the recipe layer's ring, and one parameter that
//! moves both its count and its step.
//!
//! # What this scene is for
//!
//! `heatsink` next door is the tour's recompute story on a row: one
//! document, a fin count edited 5 → 7 → 9, exactly the nodes downstream
//! of the edit re-run. A ring is its sibling — one placement over the
//! index `k = index(blades)`, an `Axis` mate on the hub's own axis,
//! blade `k` spun `scalar(k)·(360°/scalar(blades))` about it (D10
//! `[repetition]`) — and this is the tour's scene that places anything
//! around an axis.
//!
//! It also does something a row cannot, and that is the reason this
//! scene exists rather than a second comb:
//!
//! **A fan has ONE number, and the recipe layer can say so.** A blade
//! count and an angular step are two slots, and on a comb they are
//! genuinely independent — a fin count and a fin spacing say different
//! things. On a wheel they are not: `N` blades that close the circle
//! step by `360°/N`, and authoring the two separately is authoring the
//! same fact twice. So the index's count is `blades` and the step is
//! `turn / scalar(blades)`, both against ONE document parameter, and the tour's 6 → 8 → 12 is a single
//! [`DocEdit::SetVarValue`] each time.
//!
//! `scalar(n)` is what makes that sayable: a bare `blades` is a
//! `Count`, `Div`'s divisor must be `Scalar`, and the grammar's one
//! promotion is the door between them.
//!
//! # Why the hub is a prism
//!
//! A real impeller's hub is round, and this one is a 24-gon. When the
//! scene was written a box leaving a cylinder through its wall refused
//! to union, so a round hub could not have a blade unioned into it at
//! all. The kernel's own row for that pose
//! (`sweep/tests/verbs_germarms.rs`,
//! `a_bar_leaving_through_one_side_of_a_wall_builds`) builds now, at
//! its closed form under every op (`work/tang/pierce-ring-has-no-join-arm`,
//! closed); a round hub is no longer refused for that reason, and
//! this scene's faceted hub is what it was authored as, not what the
//! kernel still requires. Remodelling it is the scene's call
//! (`work/tang/boolean-refuses-on-arc-carrier-not-arc` is the other
//! frontier its note named).
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
    Bodies, BooleanValue, CancelToken, Dimension, Doc, DocEdit, EvalOptions, Evaluation, Formula,
    FreeValue, FreeVar, LoopProgram, Node, Operand, ProfileProgram, RecipeNodeId, ValuePayload,
    VarName, apply, evaluate, parse_formula,
};
use pncad::geom_core::Tol;
use pncad::topo::Body;

use crate::{SceneBody, TourStop, View};

/// The hub's facet count. Round enough to read as a hub; why it is a
/// prism is the module docs'.
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
fn params() -> BTreeMap<VarName, Dimension> {
    [(VarName::from_static("blades"), Dimension::Count)]
        .into_iter()
        .collect()
}

/// Parse against [`params`]: every expression here may name `blades`.
fn pe(src: &str) -> Formula {
    parse_formula(src, &params()).expect("the impeller's expressions parse")
}

/// The hub's cross-section: a regular [`HUB_FACETS`]-gon of
/// circumradius [`HUB_R`].
fn hub_polygon() -> LoopProgram<Formula> {
    let pts: Vec<(f64, f64)> = (0..HUB_FACETS)
        .map(|i| {
            let [x, y, _] = hub_corner(i);
            (x, y)
        })
        .collect();
    LoopProgram::polygon(pts).expect("the hub's corners are finite")
}

/// The hub's `i`th corner on its own `xy`.
fn hub_corner(i: usize) -> [f64; 3] {
    #[allow(clippy::cast_precision_loss)]
    let a = std::f64::consts::TAU * i as f64 / HUB_FACETS as f64;
    [HUB_R * a.cos(), HUB_R * a.sin(), 0.0]
}

/// One blade, in the xy plane: a rectangle reaching from inside the
/// hub out to the tip.
fn blade_polygon() -> LoopProgram<Formula> {
    LoopProgram::polygon([
        (BLADE_R0, -BLADE_T),
        (BLADE_R1, -BLADE_T),
        (BLADE_R1, BLADE_T),
        (BLADE_R0, BLADE_T),
    ])
    .expect("the blade's corners are finite")
}

/// This scene's recipe at its first count, as a document the GUI can
/// open, the impeller placed in the world: one `SetVarValue` on
/// `blades` is the scene's whole edit.
pub fn gallery_document(tol: Tol) -> Doc<ProfileProgram> {
    let r = build_doc(tol);
    let placed = crate::world::place_in_world(&r.doc, r.solid, tol);
    crate::world::set_product(
        &placed.doc,
        &[pncad::document::Operand::Node(placed.placement)],
        tol,
    )
    .doc
}

fn build_doc(tol: Tol) -> Recipe {
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("impeller", tol);
    let insert = |doc: &mut Doc<ProfileProgram>, node| -> RecipeNodeId {
        let applied = apply(
            doc,
            &DocEdit::InsertNode {
                node,
                fresh: Vec::new(),
            },
            tol,
        )
        .expect("insert node");
        *doc = applied.doc;
        applied.record.minted.expect("insert mints an id")
    };

    // The ONE parameter. Declared `Count`, which is what lets it drive
    // a structural slot at all (spec D3: a count is structural
    // material).
    let applied = apply(
        &doc,
        &DocEdit::DeclareVar {
            name: VarName::from_static("blades"),
            def: pncad::document::VarDecl::Free(FreeVar::Count { value: COUNTS[0] }),
        },
        tol,
    )
    .expect("the blade count declares");
    doc = applied.doc;

    let len = |v: f64| Formula::literal(v, Dimension::Length).expect("finite");

    let hub_p = insert(
        &mut doc,
        Box::new(Node::Profile(ProfileProgram {
            loops: vec![hub_polygon()],
            ids: Vec::new(),
            on: None,
        })),
    );
    let hub_e = insert(
        &mut doc,
        Box::new(Node::Extrude {
            profile: hub_p.into(),
            distance: len(HUB_H),
        }),
    );

    let blade_p = insert(
        &mut doc,
        Box::new(Node::Profile(ProfileProgram {
            loops: vec![blade_polygon()],
            ids: Vec::new(),
            on: None,
        })),
    );
    let blade_e = insert(
        &mut doc,
        Box::new(Node::Extrude {
            profile: blade_p.into(),
            distance: len(BLADE_H),
        }),
    );

    // The axis the blades step about: the hub's own. A frame read off
    // the hub's corners — `z` up its edge at corner 0, `x` toward the
    // opposite corner — slid the hub's radius along that `x`, onto the
    // hub's centre.
    let [c0, c12] = [0, HUB_FACETS / 2].map(hub_corner);
    let axis = pncad::document::pose::RoleRead::through(
        crate::world::frame_on(
            &doc,
            hub_e,
            [c0, [c0[0], c0[1], HUB_H], c12],
            &[pncad::document::FrameMove::Slide(
                pncad::document::TargetAxis::X,
                len(HUB_R),
            )],
            tol,
        ),
        pncad::document::pose::PoseRole::Z,
    );

    // The blade's own axis, drawn on its own `xy` about the same centre
    // as the hub, which is what the ring's mate reads on the copy: a
    // frame `z` up the blade's root edge at `(BLADE_R0, -BLADE_T)`, `x`
    // across the root toward `+y` — so its `y` runs toward the centre —
    // slid `BLADE_T` along `x` and `BLADE_R0` along `y`, onto it.
    let root = [BLADE_R0, -BLADE_T, 0.0];
    let on = pncad::document::pose::RoleRead::through(
        crate::world::frame_on(
            &doc,
            blade_e,
            [
                root,
                [BLADE_R0, -BLADE_T, BLADE_H],
                [BLADE_R0, BLADE_T, 0.0],
            ],
            &[
                pncad::document::FrameMove::Slide(pncad::document::TargetAxis::X, len(BLADE_T)),
                pncad::document::FrameMove::Slide(pncad::document::TargetAxis::Y, len(BLADE_R0)),
            ],
            tol,
        ),
        pncad::document::pose::PoseRole::Z,
    );
    // Each axis as a mate operand: the frame's `z` slid zero along
    // itself, which is that axis.
    let operand = |axis: pncad::document::pose::RoleRead<Operand>| {
        Operand::Pose(Box::new(pncad::document::pose::PoseDef::Slide {
            pose: axis.clone(),
            along: axis,
            by: Formula::zero(Dimension::Length),
        }))
    };
    let (on, axis) = (operand(on), operand(axis));

    // **The count and the step, one parameter.** The index's count IS
    // `blades`; the step is `turn / scalar(blades)` through the
    // grammar's Count→Scalar promotion. Nothing downstream has to be
    // told the blades moved.
    let k = VarName::new("k").expect("an index name");
    doc = apply(
        &doc,
        &pncad::pattern::index_decl(k.clone(), pe("blades"), None),
        tol,
    )
    .expect("the blade index declares")
    .doc;
    // The ring: the blade copied into the hub's space by one `Axis`
    // mate equating its axis with the hub's, blade `k` spun
    // `scalar(k)·turn/scalar(blades)` about it and slid `BLADE_Z0` up
    // it, so its base sinks into the hub's slab.
    let space = doc.node_space(hub_e).expect("the hub is in a space");
    let step = Formula::div(
        Formula::turn(),
        Formula::count_to_scalar(pe("blades")).expect("a count converts"),
    )
    .expect("an angle");
    let placed = pncad::document::Placed::of(
        &doc,
        vec![pncad::document::Mate {
            on,
            to: axis,
            orient: pncad::document::Orientation::IDENTITY,
        }],
        vec![
            (
                pncad::document::Coord::SPIN,
                Formula::mul(pncad::pattern::scalar_of(&k), step).expect("an angle"),
            ),
            (
                pncad::document::Coord::Slide(pncad::document::TargetAxis::Z),
                len(BLADE_Z0),
            ),
        ],
    );
    let ring = Node::place(
        pncad::document::BodyRead::plain(Operand::Node(blade_e)),
        Operand::Var(space),
        placed.bundle(),
    );
    let group = insert(&mut doc, Box::new(ring));
    let solid = insert(
        &mut doc,
        Box::new(Node::Union {
            members: Bodies::Spelled(vec![hub_e.into(), group.into()]),
        }),
    );
    Recipe { doc, group, solid }
}

/// The document's own final body, read back.
fn body_of(ev: &Evaluation<f64>, at: RecipeNodeId) -> Body<f64> {
    match &ev.value(at).expect("the node evaluated").payload {
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body.body().clone(),
        other => panic!("the impeller's root is a union, got {other:?}"),
    }
}

pub fn stops(tol: Tol) -> Vec<TourStop> {
    let r = build_doc(tol);
    let cancel = CancelToken::new();
    let opts = EvalOptions::default();

    let mut doc = r.doc.clone();
    let mut evs: Vec<(i64, Evaluation<f64>, String)> = Vec::new();
    let first = evaluate::<f64>(&doc, None, &cancel, &opts, tol);
    let names_at_first = first
        .table_at(r.group)
        .expect("the blade group evaluates")
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
            &DocEdit::SetVarValue {
                var: VarName::from_static("blades").into(),
                value: FreeValue::Count(n),
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
    // proportional in general — a name the family carried once rather
    // than once per member would be an intercept. So what is asserted
    // is the shape that claim really has, a slope and an intercept, with
    // the second difference exactly zero.
    let names: Vec<usize> = evs
        .iter()
        .map(|(_, ev, _)| {
            ev.table_at(r.group)
                .expect("the blade group evaluates")
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
    // and no two blades meet at all (the union certifies the members
    // apart), so the solid's volume is the hub's plus N times
    // one blade's protruding part. Read off the bodies rather than
    // derived from the sketch: what is asserted is that the kernel's
    // own numbers are linear in the count with no residue.
    let vols: Vec<f64> = evs
        .iter()
        .map(|(_, ev, _)| {
            pncad::topo::mass_properties(
                &crate::booleans::at_rest(&body_of(ev, r.solid), tol),
                tol,
                crate::printed(),
            )
            .expect("the impeller has a volume")
            .enclosure
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
            let vol = pncad::topo::mass_properties(
                &crate::booleans::at_rest(&body, tol),
                tol,
                crate::printed(),
            )
            .expect("volume")
            .enclosure
            .volume;
            // Literal, not `format!` into a leaked `&'static str`:
            // the roster scan in `crates/pncad/tests/all.rs` reads
            // every tour stop's NAME out of this source, and a name it
            // cannot resolve statically is a stop the north-star audit
            // never has to carry a row for. `heatsink` next door has
            // the same shape for the same reason.
            let name: &'static str = match n {
                6 => "impeller6",
                8 => "impeller8",
                _ => "impeller12",
            };
            TourStop {
                name,
                caption: caption.clone(),
                montage: *n == COUNTS[2],
                story: "ONE recipe document: a 24-gon hub, one blade, and one \
                        placement over an index `k` that rings the blade around the \
                        hub's own axis, its family unioned with the hub into ONE body. \
                        The blade COUNT and the angular STEP read the SAME document \
                        parameter — the index's count is `blades`, the step is \
                        `turn / scalar(blades)` — so \
                        the tour's 6 -> 8 -> 12 is one `SetVarValue` each time \
                        and the blades still close the circle. A comb's count and \
                        spacing are genuinely independent; a wheel's are not, and the \
                        recipe layer can say which it is. The hub is a PRISM because \
                        it was authored when a box leaving a cylinder through its wall \
                        refused to union; that pose builds now, and the faceted hub is \
                        what the scene was authored as",
                ops: "Profile(24-gon) -> Extrude; Profile(blade) -> Extrude; \
                      k = index(blades); Place(blade into the hub's space, Axis mate of \
                      its axis on the hub's, spin = scalar(k) * turn / scalar(blades), \
                      slid BLADE_Z0 up it) -> Union with the hub; then \
                      DocEdit::SetVarValue",
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
                     {name_intercept} the family carries once rather than once per \
                     member — so the members stay addressable across every edit"
                )),
                view: View {
                    elev: 34.0,
                    azim: -55.0,
                    up: 'z',
                },
                bodies: vec![SceneBody::plain(name, [0.66, 0.58, 0.44], body)],
            }
        })
        .collect()
}
