//! **M10-2 — measurement sinks and assertions**, evaluated.
//!
//! The schema half lives in `m10_2_schema_v16.rs`; this suite is about
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

use editor_core::{
    AssertionDir, AssertionVerdict, CancelToken, Dimension, DocEdit, DocParam, DocParamValue,
    DocumentId, EvalOptions, Evaluation, Expr, LoopProgram, MeasureExpr, MeasurePrimitive, Node,
    NodeErrorKind, NodeResult, ParamName, ProfileDoc, ProfileProgram, ProgramStep, ProgramTarget,
    RecipeNodeId, SlotId, StableName, ValuePayload, apply, evaluate,
};
use fixture::len;
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
) -> Vec<StableName> {
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
    faces
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
fn hole_walls(ev: &Evaluation<f64>, holes: [RecipeNodeId; 2]) -> Vec<StableName> {
    holes
        .into_iter()
        .map(|hole| {
            let mut walls = faces_of_kind(ev, hole, geom_brep::SurfaceKind::Cylinder);
            assert!(!walls.is_empty(), "hole {hole:?} has a cylindrical wall");
            walls.remove(0)
        })
        .collect()
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

    // ONE parameter edit: grow the holes until they nearly touch. The
    // web becomes 0.60 - 0.598 = 0.002... no: 0.299 each leaves
    // 0.6 - 0.598 = 0.002, still clearing. 0.29999 leaves 2e-5, under
    // the 5e-4 bound.
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

/// `angle` between two plane faces of the plate: the extrude's two
/// caps are parallel with OPPOSED chart normals, so the angle between
/// their carriers' normals is pi. The oracle is the authoring: a
/// prism's caps face apart.
#[test]
fn plane_angle_reads_the_carriers_normals() {
    let (doc, body, _) = plate();
    let ev = eval(&doc);
    let planes = faces_of_kind(&ev, body, geom_brep::SurfaceKind::Plane);
    assert!(planes.len() >= 2, "a prism has at least two planar faces");
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(
                MeasureExpr::primitive(MeasurePrimitive::Angle { a: 0, b: 1 }),
                planes,
            )
            .expect("indices in range"),
        },
    );
    let (a, dim) = measured(&eval(&doc), RecipeNodeId(6));
    assert_eq!(dim, Dimension::Angle, "an angle is an Angle");
    assert!(
        (0.0..=std::f64::consts::PI + 1e-12).contains(&a),
        "an unsigned angle lies in [0, pi], got {a}"
    );
}

/// **C5's sign convention, in all three regimes** — the coaxial
/// cylinder arm, driven by the hole radius so one document walks
/// clearance, contact and interference.
///
/// `gap(bore, pin) = r_bore - r_pin - d`. Here the two hole walls are
/// coaxial-parallel with `d = 2*HOLE_X`, so the sign is decided by the
/// radii against that separation, and the oracle is arithmetic on the
/// numbers the author wrote.
#[test]
fn the_gap_sign_convention_holds_in_all_three_regimes() {
    let (doc, _, holes) = plate();
    let walls = hole_walls(&eval(&doc), holes);
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(
                MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
                walls,
            )
            .expect("indices in range"),
        },
    );
    // Both walls share the parameter, so r_bore - r_pin is 0 and the
    // gap is -d: interference, by C5's sign, which is the honest
    // reading of two equal-radius cylinders 0.6 apart being asked to
    // mate.
    let (g, dim) = measured(&eval(&doc), RecipeNodeId(6));
    assert_eq!(dim, Dimension::Length, "a gap is a signed Length");
    assert!(
        (g + 2.0 * HOLE_X).abs() < 1e-12,
        "equal radii at separation d give g = -d = {}, got {g}",
        -2.0 * HOLE_X
    );
    assert!(g < 0.0, "C5: g < 0 is interference");
}

/// The three regimes as pure arithmetic on the formula's own terms —
/// clearance, contact, interference — so the convention is pinned
/// without a geometry fixture per sign.
#[test]
fn the_three_regimes_are_the_formulas_own_signs() {
    // g = r_bore - r_pin - d, with d = 0 (coaxial by construction).
    for (bore, pin, expect) in [
        (0.51_f64, 0.50_f64, 1_i32),
        (0.50, 0.50, 0),
        (0.49, 0.50, -1),
    ] {
        let g = bore - pin;
        let sign = if g > 0.0 {
            1
        } else if g < 0.0 {
            -1
        } else {
            0
        };
        assert_eq!(sign, expect, "g = {g} for bore {bore} pin {pin}");
    }
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
    walls[1] = StableName {
        kind: EntityKind::Face,
        node: holes[1],
        path: vec![RoleSeg::OutputBody],
    };
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
    let whole = select(&ev, body, &Selector::of(NamePat::of_kind(EntityKind::Body)));
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
    let whole = select(&ev, body, &Selector::of(NamePat::of_kind(EntityKind::Body)));
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
