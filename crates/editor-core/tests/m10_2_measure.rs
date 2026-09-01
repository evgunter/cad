//! **M10-2 — measurement sinks and assertions**, evaluated.
//!
//! The schema half lives in `m10_2_schema_v17.rs`; this suite is about
//! what a measure MEANS. Its spine is the worked example of
//! ERROR-DESIGN's two-hole plate: a plate with two circular holes, a
//! measure of the web between their walls, and an assertion that the
//! web clears a minimum — then a parameter edit that flips the verdict.
//!
//! Every closed form is checked against an INDEPENDENT oracle:
//! geometry whose answer is known from how it was authored, never from
//! a previous run of the code under test.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[path = "fixture/mod.rs"]
mod fixture;

use editor_core::UnitSym;
use editor_core::{
    AssertionDir, AssertionVerdict, CancelToken, Dimension, DocEdit, DocParam, DocParamValue,
    DocumentId, EvalOptions, Evaluation, Expr, LoopProgram, MeasureExpr, MeasurePrimitive,
    MeasureRef, Node, NodeErrorKind, NodeResult, ParamName, ProfileDoc, ProfileProgram,
    ProgramStep, ProgramTarget, RecipeNodeId, SlotId, StableName, ValuePayload, apply, evaluate,
};
use fixture::{ang, len, scl};
use geom_core::Tol;
use profile::SketchPlane;

/// The plate's hole radius, as a document parameter — the thing the
/// e2e edits to flip the assertion.
const HOLE_R: &str = "hole_r";
/// Hole centres at x = ±`HOLE_X`.
const HOLE_X: f64 = 0.30;
/// The bound the assertion carries: the web must be at least this.
const MIN_WEB: f64 = 0.0005;

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn push(doc: &ProfileDoc, edit: &DocEdit<ProfileProgram>) -> ProfileDoc {
    apply(doc, edit, Tol::witness())
        .unwrap_or_else(|e| panic!("edit refused: {e}"))
        .doc
}

/// **The two-hole plate**, authored through the public edit door as a
/// user would: a rectangular plate, and beside it two
/// parameter-driven cylindrical hole tools.
///
/// The tools are separate extrudes, which is what the worked example
/// describes ("two cylindrical holes") and what makes the web measure
/// a genuinely CROSS-NODE one: each wall is minted by its own node, so
/// the measure's two references are two DAG edges into two different
/// values.
///
/// Returns the plate body and the two hole nodes.
fn plate() -> (ProfileDoc, RecipeNodeId, [RecipeNodeId; 2]) {
    let mut doc = ProfileDoc::empty(DocumentId::derive("m10-2-plate"), Tol::witness());
    doc = push(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new(HOLE_R),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.2,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: None,
            },
        },
    );
    let outer = LoopProgram::Chain(vec![
        ProgramStep::At([len(-1.0), len(-0.5)]),
        ProgramStep::LineTo(ProgramTarget::Point([len(1.0), len(-0.5)])),
        ProgramStep::LineTo(ProgramTarget::Point([len(1.0), len(0.5)])),
        ProgramStep::LineTo(ProgramTarget::Point([len(-1.0), len(0.5)])),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram {
                plane: SketchPlane::xy(),
                loops: vec![outer],
            }),
        },
    );
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile: RecipeNodeId(0),
                distance: len(0.1),
            },
        },
    );
    let mut holes = [RecipeNodeId(3), RecipeNodeId(5)];
    for (i, cx) in [-HOLE_X, HOLE_X].into_iter().enumerate() {
        let i = i as u64;
        doc = push(
            &doc,
            &DocEdit::InsertNode {
                node: Node::Profile(ProfileProgram {
                    plane: SketchPlane::xy(),
                    loops: vec![LoopProgram::Circle {
                        centre: [len(cx), len(0.0)],
                        radius: Expr::param(ParamName::new(HOLE_R), Dimension::Length),
                    }],
                }),
            },
        );
        doc = push(
            &doc,
            &DocEdit::InsertNode {
                node: Node::Extrude {
                    profile: RecipeNodeId(2 + 2 * i),
                    distance: len(0.1),
                },
            },
        );
        holes[i as usize] = RecipeNodeId(3 + 2 * i);
    }
    (doc, RecipeNodeId(1), holes)
}

fn faces_of_kind(
    ev: &Evaluation<f64>,
    body: RecipeNodeId,
    kind: geom_brep::SurfaceKind,
) -> Vec<MeasureRef> {
    use editor_core::{EntityKind, GeomPred, NamePat, Selector, SurfaceKindSet, select_where};
    let mut faces = select_where(
        ev,
        body,
        &Selector::of(NamePat::of_kind(EntityKind::Face)),
        &[GeomPred::SurfaceKind(SurfaceKindSet::just(kind))],
        &no_params(),
        Tol::witness(),
    )
    .expect("the surface-kind atom is exact and never refuses");
    faces.sort();
    // Selected FROM `body`, so that is where the carrier is read: the
    // natural authoring pattern, and the one that reports placed
    // geometry.
    faces
        .into_iter()
        .map(|name| MeasureRef::new(body, name))
        .collect()
}

fn no_params() -> editor_core::ParamEnv<f64> {
    ProfileDoc::empty_derived("m10-2-noparams", Tol::witness()).param_env::<f64>()
}

/// One wall per hole, found the way a user finds them: evaluate, then
/// ask the selection door for that hole's CYLINDRICAL faces. Nothing
/// here hand-writes a role path.
///
/// A circular extrude's wall is TWO faces sharing one cylinder
/// carrier (the disc lowers to two half-circle arcs), and the closed
/// form reads the carrier — so either face answers for the hole, and
/// the first in canonical order is taken.
fn hole_walls(ev: &Evaluation<f64>, holes: [RecipeNodeId; 2]) -> Vec<MeasureRef> {
    holes
        .into_iter()
        .map(|hole| {
            let mut walls = faces_of_kind(ev, hole, geom_brep::SurfaceKind::Cylinder);
            assert!(!walls.is_empty(), "hole {hole:?} has a cylindrical wall");
            walls.remove(0)
        })
        .collect()
}

/// The air between the two slabs of [`two_slabs`].
const SLAB_GAP: f64 = 2.0;

/// Two DISJOINT axis-aligned slabs, the lower occupying z in [0, 1]
/// and the upper z in [1 + SLAB_GAP, 2 + SLAB_GAP]. Returns
/// (doc, lower, upper).
fn two_slabs() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut doc = ProfileDoc::empty(DocumentId::derive("m10-2-slabs"), Tol::witness());
    let square = || {
        LoopProgram::Chain(vec![
            ProgramStep::At([len(0.0), len(0.0)]),
            ProgramStep::LineTo(ProgramTarget::Point([len(1.0), len(0.0)])),
            ProgramStep::LineTo(ProgramTarget::Point([len(1.0), len(1.0)])),
            ProgramStep::LineTo(ProgramTarget::Point([len(0.0), len(1.0)])),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])
    };
    for z in [0.0, 1.0 + SLAB_GAP] {
        doc = push(
            &doc,
            &DocEdit::InsertNode {
                node: Node::Profile(ProfileProgram {
                    plane: SketchPlane::new(geom_core::Affine3::translation(geom_core::Vec3::new(
                        0.0, 0.0, z,
                    ))),
                    loops: vec![square()],
                }),
            },
        );
        let profile = RecipeNodeId(doc.len() as u64 - 1);
        doc = push(
            &doc,
            &DocEdit::InsertNode {
                node: Node::Extrude {
                    profile,
                    distance: len(1.0),
                },
            },
        );
    }
    (doc, RecipeNodeId(1), RecipeNodeId(3))
}

/// A named cap of a prism, read at the node that owns it.
fn cap(ev: &Evaluation<f64>, node: RecipeNodeId, end: editor_core::CapEnd) -> MeasureRef {
    use editor_core::{EntityKind, NamePat, SegPat, SegTag, Selector, select};
    let sel =
        Selector::of(NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(end)));
    let mut found = select(ev, node, &sel);
    assert_eq!(found.len(), 1, "one {end:?} cap on node {node:?}");
    MeasureRef::new(node, found.remove(0))
}

/// One cylindrical wall of a circular extrude, read at that extrude.
fn hole_wall_of(ev: &Evaluation<f64>, node: RecipeNodeId) -> MeasureRef {
    let mut walls = faces_of_kind(ev, node, geom_brep::SurfaceKind::Cylinder);
    assert!(!walls.is_empty(), "node {node:?} has a cylindrical wall");
    walls.remove(0)
}

/// A bore and a pin on ONE axis (the z axis through the origin), so
/// the C5 cylinder arm's axis offset `d` is exactly zero and the gap
/// is exactly `r_bore - r_pin`. Returns (doc, bore, pin).
fn coaxial_pair(bore_r: f64, pin_r: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut doc = ProfileDoc::empty(DocumentId::derive("m10-2-coaxial"), Tol::witness());
    for r in [bore_r, pin_r] {
        doc = push(
            &doc,
            &DocEdit::InsertNode {
                node: Node::Profile(ProfileProgram {
                    plane: SketchPlane::xy(),
                    loops: vec![LoopProgram::Circle {
                        centre: [len(0.0), len(0.0)],
                        radius: len(r),
                    }],
                }),
            },
        );
        let profile = RecipeNodeId(doc.len() as u64 - 1);
        doc = push(
            &doc,
            &DocEdit::InsertNode {
                node: Node::Extrude {
                    profile,
                    distance: len(0.5),
                },
            },
        );
    }
    (doc, RecipeNodeId(1), RecipeNodeId(3))
}

fn measured(ev: &Evaluation<f64>, id: RecipeNodeId) -> (f64, Dimension) {
    match ev.nodes.get(&id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, dim } => (*value, *dim),
            other => panic!("node {id:?} is a {}", other.kind_name()),
        },
        other => panic!("node {id:?} did not evaluate: {other:?}"),
    }
}

fn verdict(ev: &Evaluation<f64>, id: RecipeNodeId) -> AssertionVerdict<f64> {
    match ev.nodes.get(&id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Assertion(verdict) => verdict.clone(),
            other => panic!("node {id:?} is a {}", other.kind_name()),
        },
        other => panic!("node {id:?} did not evaluate: {other:?}"),
    }
}

/// The typed refusal a node was expected to fail with, rendered — the
/// kind is not `Clone`, so rows match on it through a borrow and this
/// helper only asserts that the node failed at all.
fn failed_kind(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NodeErrorKind {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(e)) => &e.kind,
        other => panic!("node {id:?} was expected to fail, got {other:?}"),
    }
}

/// The plate with a web measure and an assertion on it. The web is
/// `distance(wall, wall) - 2 * hole_r`: the axis separation less both
/// radii, spelled as the author's own arithmetic rather than hidden
/// inside a primitive.
fn plate_with_web() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, _, holes) = plate();
    let walls = hole_walls(&eval(&doc), holes);
    assert_eq!(walls.len(), 2, "two holes, one wall reference each");
    let r = || MeasureExpr::value(Expr::param(ParamName::new(HOLE_R), Dimension::Length));
    let web = MeasureExpr::sub(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::add(r(), r()).expect("Length + Length"),
    )
    .expect("Length - Length");
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(web, walls).expect("both indices address a reference"),
        },
    );
    let measure = RecipeNodeId(6);
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Assertion {
                measure,
                bound: Expr::literal(MIN_WEB, Dimension::Length).expect("finite"),
                dir: AssertionDir::AtLeast,
            },
        },
    );
    (doc, measure, RecipeNodeId(7))
}

// ---- The worked example (spec §6) ----

/// **e2e**: author the plate, read the web, read the verdict — then
/// flip the assertion with ONE parameter edit and see `Violated`
/// carrying both numbers.
#[test]
fn the_two_hole_plate_web_measures_and_its_assertion_flips() {
    let (doc, measure, assertion) = plate_with_web();

    let ev = eval(&doc);
    let (web, dim) = measured(&ev, measure);
    assert_eq!(dim, Dimension::Length, "a distance is a Length");
    // The independent oracle: the holes were authored at x = ±0.30 with
    // radius 0.2, so the axis separation is 0.60 and the web is
    // 0.60 - 0.4 = 0.20 exactly. Nothing here reads a previous run.
    assert!(
        (web - 0.2).abs() < 1e-12,
        "the web is 2*{HOLE_X} - 2*0.2 = 0.2, got {web}"
    );
    match verdict(&ev, assertion) {
        AssertionVerdict::Holds { measured, bound } => {
            assert!((measured - 0.2).abs() < 1e-12);
            assert!((bound - MIN_WEB).abs() < 1e-15);
        }
        other => panic!("0.2 >= {MIN_WEB} holds; got {other:?}"),
    }

    // ONE parameter edit: grow each hole to 0.29999, so the two walls
    // very nearly touch. The web becomes 2*0.30 - 2*0.29999 = 2e-5,
    // which is under the 5e-4 bound and must flip the verdict.
    let doc = push(
        &doc,
        &DocEdit::SetDocParamValue {
            name: ParamName::new(HOLE_R),
            value: DocParamValue::Continuous(0.29999),
        },
    );
    let ev = eval(&doc);
    let (web, _) = measured(&ev, measure);
    match verdict(&ev, assertion) {
        AssertionVerdict::Violated { measured, bound } => {
            assert!(
                (measured - web).abs() < 1e-15,
                "the verdict reports the measure's own number"
            );
            assert!((bound - MIN_WEB).abs() < 1e-15);
            assert!(measured < bound, "{measured} must be under {bound}");
        }
        other => panic!("a 2e-5 web violates a {MIN_WEB} bound; got {other:?}"),
    }
}

// ---- Review claim 6: report-only, by construction ----

/// **A `Violated` assertion changes NO downstream outcome.** The
/// document that would detect it: the same plate with and without the
/// assertion, at a parameter value that violates it — every other
/// node's result, content key and the document's PRODUCT are identical.
#[test]
fn a_violated_assertion_changes_no_downstream_outcome() {
    let (with_assertion, measure, assertion) = plate_with_web();
    let violating = DocEdit::SetDocParamValue {
        name: ParamName::new(HOLE_R),
        value: DocParamValue::Continuous(0.29999),
    };
    let with_assertion = push(&with_assertion, &violating);
    let without = push(
        &{
            let (doc, _, _) = plate_with_web();
            doc
        },
        &violating,
    );
    let without = push(&without, &DocEdit::DeleteNode { id: assertion });

    let a = eval(&with_assertion);
    let b = eval(&without);
    assert!(
        matches!(verdict(&a, assertion), AssertionVerdict::Violated { .. }),
        "the probe is only meaningful while the assertion is violated"
    );
    // Every node the two documents share evaluates identically, keys
    // included — the memo currency is what a gate would have to move.
    for id in [RecipeNodeId(0), RecipeNodeId(1), measure] {
        let (x, y) = (
            a.nodes.get(&id).expect("live"),
            b.nodes.get(&id).expect("live"),
        );
        match (x, y) {
            (NodeResult::Ok(x), NodeResult::Ok(y)) => {
                assert_eq!(
                    x.content_key, y.content_key,
                    "node {id:?} content key moved"
                );
                assert_eq!(x.naming_key, y.naming_key, "node {id:?} naming key moved");
            }
            _ => panic!("node {id:?} did not evaluate in both documents"),
        }
    }
    // And the PRODUCT — what the document MEANS — is the same body set.
    let product = |doc: &ProfileDoc, ev: &Evaluation<f64>| {
        editor_core::product(doc, ev, Tol::witness()).map(|b| b.solids().count())
    };
    assert_eq!(
        product(&with_assertion, &a).ok(),
        product(&without, &b).ok(),
        "an assertion is report-only: the product cannot see it"
    );
}

// ---- Review claim 1: zero impact where the nodes are absent ----

/// The measurement vocabulary costs a document that does not use it
/// NOTHING: same content keys, same naming keys, same saved bytes.
#[test]
fn a_document_without_measures_is_untouched() {
    let (plain, body, holes) = plate();
    let (measured_doc, _, _) = plate_with_web();
    let (a, b) = (eval(&plain), eval(&measured_doc));
    for id in [RecipeNodeId(0), body, holes[0], holes[1]] {
        match (a.nodes.get(&id), b.nodes.get(&id)) {
            (Some(NodeResult::Ok(x)), Some(NodeResult::Ok(y))) => {
                assert_eq!(x.content_key, y.content_key);
                assert_eq!(x.naming_key, y.naming_key);
            }
            other => panic!("node {id:?}: {other:?}"),
        }
    }
}

// ---- Review claim 4: closed-form honesty ----

/// `distance` between the two hole walls IS the axis separation — the
/// number the author wrote into the two circle centres.
#[test]
fn cylinder_distance_is_the_axis_separation() {
    let (doc, _, holes) = plate();
    let walls = hole_walls(&eval(&doc), holes);
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(
                MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
                walls,
            )
            .expect("indices in range"),
        },
    );
    let (d, dim) = measured(&eval(&doc), RecipeNodeId(6));
    assert_eq!(dim, Dimension::Length);
    assert!(
        (d - 2.0 * HOLE_X).abs() < 1e-12,
        "the axes are 2*{HOLE_X} apart, got {d}"
    );
}

/// `angle` between the plate's two CAPS is exactly pi: an extruded
/// prism's caps are parallel with OPPOSED chart normals, so the angle
/// between the carriers is a straight angle.
///
/// The oracle is the authoring, not the codomain. The row this
/// replaces asserted only `0 <= a <= pi`, which every possible answer
/// satisfies — it could not have gone red for any bug in the arm.
#[test]
fn plane_angle_between_opposed_caps_is_pi() {
    let (doc, body, _) = plate();
    let ev = eval(&doc);
    let planes = faces_of_kind(&ev, body, geom_brep::SurfaceKind::Plane);
    assert!(planes.len() >= 2, "a prism has at least two planar faces");
    // The caps are the two faces whose chart normals are +/-z; the
    // side walls are the rest. Picked by NAME (the cap role), not by
    // measuring, so the oracle stays independent of the arm.
    let caps: Vec<MeasureRef> = {
        use editor_core::{CapEnd, EntityKind, NamePat, SegPat, SegTag, Selector, select};
        let pick = |end| {
            let sel = Selector::of(
                NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(end)),
            );
            let mut found = select(&ev, body, &sel);
            assert_eq!(found.len(), 1, "one {end:?} cap");
            MeasureRef::new(body, found.remove(0))
        };
        vec![pick(CapEnd::Top), pick(CapEnd::Bottom)]
    };
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(
                MeasureExpr::primitive(MeasurePrimitive::Angle { a: 0, b: 1 }),
                caps,
            )
            .expect("indices in range"),
        },
    );
    let (a, dim) = measured(&eval(&doc), RecipeNodeId(6));
    assert_eq!(dim, Dimension::Angle, "an angle is an Angle");
    assert!(
        (a - std::f64::consts::PI).abs() < 1e-12,
        "opposed caps subtend pi, got {a}"
    );
}

/// **MAJ-3: a plane gap is a MATERIAL separation, and its sign says
/// so.** Two disjoint slabs, 2 m of air between them: the facing pair
/// must read `+2`, never negative, whichever way round the roles go.
///
/// Before the sense fold this arm read the raw chart normal, so the
/// sign was an artifact of how each face happened to be charted:
/// half the parallel pairs over exactly this geometry read NEGATIVE —
/// C5 "interference" with 2 m of air in between.
#[test]
fn a_plane_gap_over_disjoint_slabs_is_positive_both_ways() {
    let (doc, lower, upper) = two_slabs();
    let ev = eval(&doc);
    let top_of_lower = cap(&ev, lower, editor_core::CapEnd::Top);
    let bottom_of_upper = cap(&ev, upper, editor_core::CapEnd::Bottom);

    // Forward, then the roles swapped.
    for (o, i) in [
        (top_of_lower.clone(), bottom_of_upper.clone()),
        (bottom_of_upper, top_of_lower),
    ] {
        let doc = push(
            &doc,
            &DocEdit::InsertNode {
                node: Node::measure(
                    MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
                    vec![o, i],
                )
                .expect("indices in range"),
            },
        );
        let (g, dim) = measured(&eval(&doc), RecipeNodeId(4));
        assert_eq!(dim, Dimension::Length);
        assert!(
            (g - SLAB_GAP).abs() < 1e-12,
            "two slabs {SLAB_GAP} m apart have a +{SLAB_GAP} gap, got {g}"
        );
        assert!(g > 0.0, "C5: air between the faces is CLEARANCE, not {g}");
    }
}

/// The same geometry, the ALIGNED pair (both material sides facing the
/// same way): there the role swap negates, which is the other half of
/// the convention and the half that does carry information.
#[test]
fn a_plane_gap_over_an_aligned_pair_negates_under_a_role_swap() {
    let (doc, lower, upper) = two_slabs();
    let ev = eval(&doc);
    let top_of_lower = cap(&ev, lower, editor_core::CapEnd::Top);
    let top_of_upper = cap(&ev, upper, editor_core::CapEnd::Top);

    let read = |o: MeasureRef, i: MeasureRef| {
        let doc = push(
            &doc,
            &DocEdit::InsertNode {
                node: Node::measure(
                    MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
                    vec![o, i],
                )
                .expect("indices in range"),
            },
        );
        measured(&eval(&doc), RecipeNodeId(4)).0
    };
    let forward = read(top_of_lower.clone(), top_of_upper.clone());
    let swapped = read(top_of_upper, top_of_lower);
    assert!(
        (forward + swapped).abs() < 1e-12,
        "an aligned pair negates under a role swap: {forward} vs {swapped}"
    );
    assert!(
        forward.abs() > 1e-9,
        "the reading must be nonzero for the negation to say anything"
    );
}

/// **C5's three regimes, through the door under test** — coaxial
/// cylinders at a shared axis, with the radii driven so one document
/// walks clearance, contact and interference.
///
/// The row this replaces computed `let g = bore - pin;` in the test
/// and asserted its sign: it called nothing under test and could not
/// have failed for any bug in `gap`.
#[test]
fn the_gap_sign_convention_walks_all_three_regimes() {
    // A bore and a pin on ONE axis, so the axis offset d is 0 and the
    // sign is exactly r_bore - r_pin.
    for (bore_r, pin_r, expect) in [
        (0.51_f64, 0.50_f64, 1_i32),
        (0.50, 0.50, 0),
        (0.49, 0.50, -1),
    ] {
        let (doc, bore, pin) = coaxial_pair(bore_r, pin_r);
        let ev = eval(&doc);
        let refs = vec![hole_wall_of(&ev, bore), hole_wall_of(&ev, pin)];
        let doc = push(
            &doc,
            &DocEdit::InsertNode {
                node: Node::measure(
                    MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
                    refs,
                )
                .expect("indices in range"),
            },
        );
        let (g, dim) = measured(&eval(&doc), RecipeNodeId(4));
        assert_eq!(dim, Dimension::Length, "a gap is a signed Length");
        let want = bore_r - pin_r;
        assert!(
            (g - want).abs() < 1e-12,
            "coaxial g = r_bore - r_pin = {want}, got {g}"
        );
        let sign = if g > 1e-12 {
            1
        } else if g < -1e-12 {
            -1
        } else {
            0
        };
        assert_eq!(sign, expect, "C5 regime for bore {bore_r} pin {pin_r}");
    }
}

// ---- The three review MAJORs, each pinned red-capable ----

/// **MAJ-1: a non-finite measure refuses, and NO verdict is produced
/// over it.**
///
/// `13 / s` at `s = 0` is `inf`. In an extrude slot `expr::eval`'s
/// door 2 has always refused it; the measurement sublanguage restated
/// the arithmetic without that door, so the same division came back a
/// typed SUCCESS and an assertion over it reported
/// `Holds { measured: inf }` — a false PASS from the node whose whole
/// job is certifying intent.
#[test]
fn a_non_finite_measure_refuses_and_asserts_nothing() {
    let mut doc = ProfileDoc::empty(DocumentId::derive("m10-2-inf"), Tol::witness());
    doc = push(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("s"),
            value: DocParam::Continuous {
                dim: Dimension::Scalar,
                value: 0.0,
                display_unit: UnitSym::canonical_for(Dimension::Scalar),
                distribution: None,
            },
        },
    );
    // 13 m / s, with s bound to zero.
    let over_zero = MeasureExpr::div(
        MeasureExpr::value(Expr::literal(13.0, Dimension::Length).expect("finite")),
        MeasureExpr::value(Expr::param(ParamName::new("s"), Dimension::Scalar)),
    )
    .expect("Length / Scalar");
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(over_zero, Vec::new()).expect("no references to bound"),
        },
    );
    let measure = RecipeNodeId(0);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Assertion {
                measure,
                bound: Expr::literal(1.0, Dimension::Length).expect("finite"),
                dir: AssertionDir::AtLeast,
            },
        },
    );
    let ev = eval(&doc);

    let err = failed_kind(&ev, measure);
    assert!(
        matches!(err, NodeErrorKind::MeasureNonFinite { .. }),
        "a division by zero must refuse, got {err:?}"
    );
    // And the assertion produces NO verdict at all: it is poisoned
    // through the DAG edge, never `Holds` over infinity.
    assert!(
        matches!(
            ev.nodes.get(&RecipeNodeId(1)),
            Some(NodeResult::Poisoned { .. })
        ),
        "no verdict may be reported over a non-finite measure, got {:?}",
        ev.nodes.get(&RecipeNodeId(1))
    );
}

/// The same expression in an ordinary SLOT has always refused — the
/// row that shows the measure lane was the one out of step, not the
/// door.
#[test]
fn the_same_division_in_a_slot_has_always_refused() {
    let mut doc = ProfileDoc::empty(DocumentId::derive("m10-2-inf-slot"), Tol::witness());
    doc = push(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("s"),
            value: DocParam::Continuous {
                dim: Dimension::Scalar,
                value: 0.0,
                display_unit: UnitSym::canonical_for(Dimension::Scalar),
                distribution: None,
            },
        },
    );
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram {
                plane: SketchPlane::xy(),
                loops: vec![LoopProgram::Circle {
                    centre: [len(0.0), len(0.0)],
                    radius: len(0.2),
                }],
            }),
        },
    );
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile: RecipeNodeId(0),
                distance: Expr::div(
                    Expr::literal(13.0, Dimension::Length).expect("finite"),
                    Expr::param(ParamName::new("s"), Dimension::Scalar),
                )
                .expect("Length / Scalar"),
            },
        },
    );
    let ev = eval(&doc);
    let err = failed_kind(&ev, RecipeNodeId(1));
    assert!(
        matches!(
            err,
            NodeErrorKind::Expr {
                source: editor_core::EvalError::NonFiniteResult,
                ..
            }
        ),
        "the slot lane refuses non-finite, got {err:?}"
    );
}

/// **MAJ-2: a measure reads the PLACED carrier.**
///
/// A box is built at the origin and translated 100 m. A vertex
/// selected from the TRANSFORM (`at` = the transform) must measure
/// against the placed geometry; reading it at the minting extrude
/// gives the authored position, and the two must differ by exactly the
/// translation. Before the fix both spellings returned the authored
/// number and said `Ok`.
#[test]
fn a_measure_at_a_transform_reads_the_placed_carrier() {
    use editor_core::{EntityKind, NamePat, Selector, select};
    const SHIFT: f64 = 100.0;
    let mut doc = ProfileDoc::empty(DocumentId::derive("m10-2-placed"), Tol::witness());
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram {
                plane: SketchPlane::xy(),
                loops: vec![LoopProgram::Chain(vec![
                    ProgramStep::At([len(0.0), len(0.0)]),
                    ProgramStep::LineTo(ProgramTarget::Point([len(1.0), len(0.0)])),
                    ProgramStep::LineTo(ProgramTarget::Point([len(1.0), len(1.0)])),
                    ProgramStep::LineTo(ProgramTarget::Point([len(0.0), len(1.0)])),
                    ProgramStep::LineTo(ProgramTarget::Start),
                ])],
            }),
        },
    );
    let solid = RecipeNodeId(1);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile: RecipeNodeId(0),
                distance: len(1.0),
            },
        },
    );
    let placed = RecipeNodeId(2);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Transform {
                input: solid,
                translation: [len(SHIFT), len(0.0), len(0.0)],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: ang(0.0),
            },
        },
    );

    let ev = eval(&doc);
    // ONE vertex name, measured against itself at two different
    // reading sites: the only thing that differs is `at`.
    let vname = {
        let mut vs = select(
            &ev,
            placed,
            &Selector::of(NamePat::of_kind(EntityKind::Vertex)),
        );
        vs.sort();
        assert!(!vs.is_empty(), "the box has vertices");
        vs.remove(0)
    };
    // The SAME vertex name, read at the two sites: the distance between
    // the authored carrier and the placed one is exactly the
    // translation. Nothing else in the document differs.
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(
                MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
                vec![
                    MeasureRef::new(solid, vname.clone()),
                    MeasureRef::new(placed, vname),
                ],
            )
            .expect("indices in range"),
        },
    );
    let (d, _) = measured(&eval(&doc), RecipeNodeId(3));
    assert!(
        (d - SHIFT).abs() < 1e-9,
        "the same vertex read at the extrude and at the transform is {SHIFT} m apart, got {d} \
         — a measure at the transform must see the PLACED carrier"
    );
}

// ---- Review claim 5: scalar genericity ----

/// A measure evaluates at `Interval` and its bracket CONTAINS the f64
/// value — the containment claim, on the measurement channel.
#[cfg(feature = "interval")]
#[test]
fn a_measure_at_interval_contains_the_f64_value() {
    use geom_core::{Bounds, Interval};
    let (doc, measure, _) = plate_with_web();
    let at_f64 = measured(&eval(&doc), measure).0;
    let ev = evaluate::<Interval>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match ev.nodes.get(&measure) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => {
                assert!(
                    value.lo() <= at_f64 && at_f64 <= value.hi(),
                    "[{}, {}] must contain {at_f64}",
                    value.lo(),
                    value.hi()
                );
            }
            other => panic!("expected a measure, got {}", other.kind_name()),
        },
        other => panic!("the measure did not evaluate at Interval: {other:?}"),
    }
}

// ---- Review claim 3: refusal completeness ----

/// A reference that no longer RESOLVES refuses with N5's typed
/// vocabulary — never silence, never a measurement of what is left.
///
/// Note which dangling case this is. A measure's references ARE DAG
/// edges, so deleting a referenced node is refused at the delete door
/// like any other consumer's input (`DeleteWouldDangle`) — that
/// departs from the `Declare`/`Mate` carve-out, deliberately, because
/// a measure consumes the value it names. What remains reachable is
/// the case the N5 ladder is really for: a well-formed name that the
/// still-live minting node's table does not carry.
#[test]
fn a_reference_that_stops_resolving_refuses_typed() {
    use editor_core::{EntityKind, RoleSeg};
    let (doc, _, holes) = plate();
    let mut walls = hole_walls(&eval(&doc), holes);
    // A FACE name at the body role: well-formed, minted by a live
    // node, and carried by no table — the `Vanished` rung.
    walls[1] = MeasureRef::new(
        holes[1],
        StableName {
            kind: EntityKind::Face,
            node: holes[1],
            path: vec![RoleSeg::OutputBody],
        },
    );
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(
                MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
                walls,
            )
            .expect("indices in range"),
        },
    );
    let ev = eval(&doc);
    let err = failed_kind(&ev, RecipeNodeId(6));
    assert!(
        matches!(err, NodeErrorKind::MeasureRefResolve { .. }),
        "got {err:?}"
    );
}

/// Deleting a node a measure references is refused at the DELETE door,
/// because the reference is a consuming edge. Pinned because it is the
/// one place this node kind departs from the `Declare`/`Mate`
/// name-reference carve-out, and a silent reversal would take the
/// ordering guarantee with it.
#[test]
fn deleting_a_referenced_node_is_refused() {
    use editor_core::EditError;
    let (doc, _, holes) = plate();
    let walls = hole_walls(&eval(&doc), holes);
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(
                MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
                walls,
            )
            .expect("indices in range"),
        },
    );
    let err = apply(&doc, &DocEdit::DeleteNode { id: holes[0] }, Tol::witness())
        .expect_err("the measure consumes that node");
    assert!(
        matches!(err, EditError::DeleteWouldDangle { .. }),
        "got {err:?}"
    );
}

/// A reference to a WHOLE BODY has no carrier, and the refusal names
/// the pair class rather than guessing an arm.
#[test]
fn an_unsupported_carrier_pair_refuses_naming_the_pair() {
    use editor_core::{EntityKind, NamePat, Selector, select};
    let (doc, body, _) = plate();
    let ev = eval(&doc);
    let whole: Vec<MeasureRef> =
        select(&ev, body, &Selector::of(NamePat::of_kind(EntityKind::Body)))
            .into_iter()
            .map(|name| MeasureRef::new(body, name))
            .collect();
    assert_eq!(whole.len(), 1, "one output body");
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(
                MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 0 }),
                whole,
            )
            .expect("indices in range"),
        },
    );
    let ev = eval(&doc);
    match failed_kind(&ev, RecipeNodeId(6)) {
        NodeErrorKind::MeasureUnsupported(refusal) => {
            assert_eq!(refusal.verb, "distance");
            let msg = refusal.to_string();
            assert!(msg.contains("whole body"), "{msg}");
        }
        other => panic!("got {other:?}"),
    }
}

/// A measure with an unsupported MIXED pair: a plane face against a
/// cylinder face has no v1 closed form.
#[test]
fn a_mixed_carrier_pair_refuses() {
    let (doc, body, holes) = plate();
    let ev = eval(&doc);
    // The plate's own faces are all planar; the cylinder comes from a
    // hole tool, so the pair also crosses two nodes.
    let refs = vec![
        faces_of_kind(&ev, body, geom_brep::SurfaceKind::Plane).remove(0),
        faces_of_kind(&ev, holes[0], geom_brep::SurfaceKind::Cylinder).remove(0),
    ];
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(
                MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
                refs,
            )
            .expect("indices in range"),
        },
    );
    let ev = eval(&doc);
    let err = failed_kind(&ev, RecipeNodeId(6));
    assert!(
        matches!(err, NodeErrorKind::MeasureUnsupported(_)),
        "got {err:?}"
    );
}

/// An assertion over a FAILED measure is poisoned itself (F2), not
/// `Unevaluated`: the DAG edge is what composes, and a verdict about a
/// measurement that did not happen would be a verdict about nothing.
#[test]
fn an_assertion_over_a_failed_measure_is_poisoned() {
    use editor_core::{EntityKind, NamePat, Selector, select};
    let (doc, body, _) = plate();
    let ev = eval(&doc);
    let whole: Vec<MeasureRef> =
        select(&ev, body, &Selector::of(NamePat::of_kind(EntityKind::Body)))
            .into_iter()
            .map(|name| MeasureRef::new(body, name))
            .collect();
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            // A whole-body pair has no closed form, so the measure
            // fails and the assertion must produce no verdict.
            node: Node::measure(
                MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 0 }),
                whole,
            )
            .expect("indices in range"),
        },
    );
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Assertion {
                measure: RecipeNodeId(6),
                bound: Expr::literal(0.1, Dimension::Length).expect("finite"),
                dir: AssertionDir::AtLeast,
            },
        },
    );
    let ev = eval(&doc);
    assert!(
        matches!(ev.nodes.get(&RecipeNodeId(6)), Some(NodeResult::Failed(_))),
        "the measure must fail for this probe to mean anything"
    );
    assert!(
        matches!(
            ev.nodes.get(&RecipeNodeId(7)),
            Some(NodeResult::Poisoned { .. })
        ),
        "the assertion must be poisoned, got {:?}",
        ev.nodes.get(&RecipeNodeId(7))
    );
}

// ---- The slot vocabulary is untouched ----

/// A measure carries no SLOT: its expression is not an `Expr` and its
/// bound is not addressable by a slot id, which is exactly why both
/// are payload. Pinned so a later unit does not quietly grow one.
#[test]
fn the_measurement_nodes_carry_no_slots() {
    let (doc, measure, assertion) = plate_with_web();
    for id in [measure, assertion] {
        let node = doc.node(id).expect("live");
        assert!(
            node.slots().is_empty(),
            "node {id:?} grew a slot: {:?}",
            node.slots()
        );
        assert!(node.expr(SlotId::Distance).is_none());
    }
}
