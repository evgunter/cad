//! **A slot holds a variable** — INTENT-LITERALS PR C's rows of the
//! spec's §8 test plan (`docs/INTENT-LITERALS-SPEC.md`): 5 (two typed
//! values are two variables), 6 (a token is the expansion's shape), 7
//! (the lifecycle of a slot's own variables), 8 (a variable's identity
//! survives a gesture), 9 (Monte Carlo draws only what varies), 10
//! (split carries anonymity) and 14's C half (the load door's slot
//! reads), a row per guard the fresh table adds, and the rows of VR8's
//! tolerance rule (spec §11: only a toleranced variable is an analysis
//! axis). Row 11 (range names nothing) is `docm9_range`'s.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::corpus::{body_of, failures};
use crate::fixture::{insert, len, on_frame, prism_edges, square};
use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::persist::SnapshotError;
use editor_core::{
    CancelToken, Datum, Dimension, Distribution, DocEdit, DocumentId, EditError, EvalOptions,
    Evaluation, ExtrudeSide, Formula, FreeValue, FreeVar, LoopProgram, Maintenance, Node,
    PersistError, ProfileDoc, ProfileProgram, RecipeNodeId, SlotId, SplitError, VarDecl, VarId,
    VarName, apply, evaluate, load, save, split,
};
use geom_brep::RadiusEvidence;
use geom_core::Tol;
use topo::{Body, FaceKey, SurfaceField};

/// The blend radius, millimetres (dyadic in metres).
const R_MM: f64 = 125.0;

fn n(name: &'static str) -> VarName {
    VarName::from_static(name)
}

fn try_step(
    doc: &ProfileDoc,
    edit: DocEdit<ProfileProgram>,
) -> Result<editor_core::Applied<ProfileProgram>, EditError> {
    apply(doc, &edit, Tol::witness(), &editor_core::RefusingReach)
}

fn step(doc: &ProfileDoc, edit: DocEdit<ProfileProgram>) -> editor_core::Applied<ProfileProgram> {
    try_step(doc, edit).expect("the edit applies")
}

fn declare(doc: &ProfileDoc, name: &'static str, def: VarDecl) -> ProfileDoc {
    step(doc, DocEdit::DeclareVar { name: n(name), def }).doc
}

fn length(value: f64) -> VarDecl {
    VarDecl::Free(FreeVar::continuous(Dimension::Length, value))
}

fn named(name: &'static str) -> Formula {
    Formula::named(n(name), Dimension::Length)
}

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// A unit cube at `cx` with every edge blended by `radius`: the cube's
/// id and the blend's.
fn filleted(doc: ProfileDoc, cx: f64, radius: Formula) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, 0.5)],
    );
    let (doc, cube) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let edges = prism_edges(&doc, cube, 4);
    let (doc, blend) = insert(doc, Node::fillet(cube, radius, edges));
    (doc, cube, blend)
}

/// One cylindrical blend carrier of `body`, in deterministic arena order.
fn a_cylinder_face(body: &Body<f64>) -> FaceKey {
    topo::query::all_faces(body)
        .into_iter()
        .find(|&f| {
            body.get_face(f)
                .and_then(|fd| body.get_surface(fd.surface))
                .is_some_and(|s| matches!(s, geom::Surface::Cylinder { .. }))
        })
        .expect("a blended cube carries quarter-cylinder blends")
}

/// The radius token a blend's cylinder carries.
fn radius_token(body: &Body<f64>) -> topo::ParamSource {
    let face = a_cylinder_face(body);
    let surface = body.get_face(face).expect("a live face").surface;
    body.surface_field_source(surface, SurfaceField::CylinderRadius)
        .expect("a document-built blend declares its radius")
        .clone()
}

fn evidence(a: &Body<f64>, b: &Body<f64>) -> RadiusEvidence {
    topo::field_source_evidence(
        a,
        a_cylinder_face(a),
        b,
        a_cylinder_face(b),
        SurfaceField::CylinderRadius,
    )
}

fn radius(doc: &ProfileDoc, blend: RecipeNodeId) -> VarId {
    doc.slot(blend, SlotId::Radius)
        .expect("a blend reads its radius")
}

// --------------------------------------------------------------- row 5

/// Row 5: two blends each written `125 mm` read two variables, so their
/// radius tokens differ and the radii are not `Declared` the same; the
/// same two blends reading one variable lower equal and are. Breaks if
/// the lowering dedups written values by value.
#[test]
fn two_typed_values_are_two_variables() {
    let typed = || Formula::length_in(R_MM, quantity::MM).unwrap();
    let doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-c-typed"),
        Tol::witness(),
    );
    let (doc, _, a) = filleted(doc, 0.0, typed());
    let (doc, _, b) = filleted(doc, 4.0, typed());
    assert_ne!(
        radius(&doc, a),
        radius(&doc, b),
        "two writings, two variables"
    );
    let ev = eval(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let (ba, bb) = (body_of(&ev, a), body_of(&ev, b));
    assert_ne!(radius_token(ba), radius_token(bb));
    assert_ne!(evidence(ba, bb), RadiusEvidence::Declared);

    let shared = radius(&doc, a);
    let (doc, _, c) = filleted(doc, 8.0, Formula::var(shared, Dimension::Length));
    assert_eq!(
        radius(&doc, c),
        shared,
        "a variable passed is the variable read"
    );
    let ev = eval(&doc);
    let (ba, bc) = (body_of(&ev, a), body_of(&ev, c));
    assert_eq!(radius_token(ba), radius_token(bc));
    assert_eq!(evidence(ba, bc), RadiusEvidence::Declared);
}

// --------------------------------------------------------------- row 6

/// Row 6: blends written `w·2`, `w·2` and `h` (with `h := w·2`) lower to
/// one radius token — a slot's token is its variable's expansion, not
/// the id of the anonymous variable the formula minted — and `w·3`
/// lowers to another. Breaks if a token encodes a defined variable's
/// id.
#[test]
fn a_token_is_the_expansions_shape() {
    let doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-c-shape"),
        Tol::witness(),
    );
    let doc = declare(&doc, "w", length(0.0625));
    let times = |k: f64| Formula::mul(named("w"), crate::fixture::scl(k)).unwrap();
    let doc = declare(&doc, "h", VarDecl::defined(times(2.0)));
    let (doc, _, a) = filleted(doc, 0.0, times(2.0));
    let (doc, _, b) = filleted(doc, 4.0, times(2.0));
    let (doc, _, c) = filleted(doc, 8.0, named("h"));
    let (doc, _, d) = filleted(doc, 12.0, times(3.0));
    assert_ne!(
        radius(&doc, a),
        radius(&doc, b),
        "each formula mints its variable"
    );
    let ev = eval(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let token = |blend| radius_token(body_of(&ev, blend));
    assert_eq!(token(a), token(b));
    assert_eq!(token(a), token(c));
    assert_ne!(token(a), token(d));
}

// --------------------------------------------------------------- row 7

/// Row 7: a slot rewritten from `w + 5 mm` to `w` retires the variable
/// the formula minted, reported, and the mint log keeps its id; the
/// slot reads `w` itself.
#[test]
fn a_rewritten_slot_retires_the_variable_its_formula_minted() {
    let doc = ProfileDoc::empty(DocumentId::derive("intent-literals-c-life"), Tol::witness());
    let doc = declare(&doc, "w", length(0.0625));
    let plus = Formula::add(named("w"), Formula::length_in(5.0, quantity::MM).unwrap()).unwrap();
    let (doc, _, blend) = filleted(doc, 0.0, plus);
    let formula = radius(&doc, blend);
    assert!(
        doc.var(formula)
            .is_some_and(|v| v.def().defined().is_some())
    );
    let applied = step(
        &doc,
        DocEdit::SetParam {
            node: blend,
            slot: SlotId::Radius,
            expr: named("w"),
            fresh: Vec::new(),
        },
    );
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::AnonymousVarRemoved {
            var: doc.spoken_var(formula),
            distribution: None,
        }]
    );
    assert_eq!(applied.doc.slot(blend, SlotId::Radius), doc.var_named("w"));
    assert!(applied.doc.var(formula).is_none());
    assert!(applied.doc.has_minted_var(formula), "the log keeps its id");
}

/// Row 7's load half: a file whose slot's own anonymous definition is
/// left read by nothing — the blend re-pointed at `w` — refuses
/// `AnonymousVarUnread`. Liveness through a definition (a variable read
/// only by an anonymous definition nothing reads) is PR A's rows'
/// (`intent_literals_a_*`) and `an_entry_reads_the_entries_before_it`'s.
#[test]
fn an_anonymous_variable_read_only_by_an_unread_definition_refuses_at_load() {
    let doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-c-unread"),
        Tol::witness(),
    );
    let doc = declare(&doc, "w", length(0.0625));
    let plus = Formula::add(named("w"), Formula::length_in(5.0, quantity::MM).unwrap()).unwrap();
    let (doc, _, blend) = filleted(doc, 0.0, plus);
    let formula = radius(&doc, blend);
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    load(&text, Tol::witness()).expect("and loads");
    // The blend re-pointed at `w`, by hand: the formula's variable is
    // left read by nothing.
    let w = doc.var_named("w").expect("declared");
    let corrupt = crate::wire::doctored(&text, |wire| {
        wire["snapshot"]["nodes"][blend.0.to_string()]["Fillet"]["radius"] = serde_json::json!(w.0);
    });
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::AnonymousVarUnread { var })) => {
            assert_eq!(var.id(), formula);
        }
        other => panic!("an unread anonymous variable refuses at load, got {other:?}"),
    }
}

// --------------------------------------------------------------- row 8

/// The square's first corner's x, `(loop 0, step 0, x)`.
fn corner_x() -> SlotId {
    SlotId::Profile {
        loop_: 0,
        step: 0,
        arg: editor_core::StepArg::PointX,
    }
}

/// Row 8: a distribution set on a sketch argument's own variable
/// survives a value gesture (`SetVarValue`) and a `SetProgram` that
/// re-authors the program through `Node::authored`, bit for bit. Breaks
/// if either path re-lowers the argument's written value.
#[test]
fn a_sketch_arguments_identity_survives_a_gesture_and_a_reshaping() {
    let doc = ProfileDoc::empty(
        DocumentId::derive("intent-literals-c-gesture"),
        Tol::witness(),
    );
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let var = doc.slot(profile, corner_x()).expect("the corner's x");
    let spread = Distribution::Normal { sigma: 0.001 };
    let doc = step(
        &doc,
        DocEdit::SetVarDistribution {
            var: var.into(),
            distribution: Some(spread),
        },
    )
    .doc;
    let held = |doc: &ProfileDoc| doc.free(var).cloned();
    let before = held(&doc).expect("a free variable");

    let dragged = step(
        &doc,
        DocEdit::SetVarValue {
            var: var.into(),
            value: FreeValue::Continuous(-0.25),
        },
    )
    .doc;
    assert_eq!(dragged.slot(profile, corner_x()), Some(var));
    assert_eq!(
        held(&dragged).and_then(|f| f.distribution().copied()),
        Some(spread),
        "a value gesture keeps the spread"
    );

    let Some(node @ Node::Profile(program)) = dragged.node(profile) else {
        panic!("the profile");
    };
    let Node::Profile(authored) = node.authored(&dragged) else {
        unreachable!("a profile re-authors as a profile")
    };
    let reshaped = step(
        &dragged,
        DocEdit::SetProgram {
            node: profile,
            loops: authored.loops,
            ids: program
                .ids
                .iter()
                .map(|l| l.iter().copied().map(Some).collect())
                .collect(),
            fresh: Vec::new(),
        },
    );
    assert_eq!(reshaped.doc.slot(profile, corner_x()), Some(var));
    let after = held(&reshaped.doc).expect("still free");
    assert_eq!(
        after.distribution(),
        before.distribution(),
        "the spread, bit for bit"
    );
    assert!(
        reshaped.maintenance.is_empty(),
        "a reshaping that re-authors every argument retires nothing: {:?}",
        reshaped.maintenance
    );
}

// --------------------------------------------------------------- row 9

/// Row 9: Monte Carlo draws only the variables that carry a
/// distribution. A document whose every dimension is written (each an
/// anonymous fixed axis) draws exactly its one toleranced variable, and
/// writing one more dimension moves no draw. Breaks if an anonymous
/// fixed variable enters the draw order.
#[test]
fn monte_carlo_draws_only_what_varies() {
    let doc = ProfileDoc::empty(DocumentId::derive("intent-literals-c-mc"), Tol::witness());
    let doc = declare(
        &doc,
        "w",
        VarDecl::Free(FreeVar::continuous_with(
            Dimension::Length,
            0.0625,
            Distribution::Normal { sigma: 0.001 },
        )),
    );
    let (doc, _, _) = filleted(doc, 0.0, named("w"));
    let w = doc.var_named("w").expect("declared");
    let config = editor_core::mc::McConfig::default();
    let draws = |doc: &ProfileDoc| -> Vec<std::collections::BTreeMap<VarId, f64>> {
        let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
        (0..8)
            .map(|i| editor_core::mc::sample_offsets(doc, &analyzed, &config, i).unwrap())
            .collect()
    };
    let sheet = draws(&doc);
    assert!(
        crate::fixture::continuous_vars(&doc) > 1,
        "the premise: written dimensions are variables too"
    );
    assert!(
        sheet
            .iter()
            .all(|d| d.keys().copied().collect::<Vec<_>>() == vec![w]),
        "only w draws: {sheet:?}"
    );
    let (more, _, _) = filleted(doc, 4.0, len(0.125));
    let again = draws(&more);
    assert_eq!(
        sheet.iter().map(|d| d[&w].to_bits()).collect::<Vec<_>>(),
        again.iter().map(|d| d[&w].to_bits()).collect::<Vec<_>>(),
        "one more written dimension moves no draw"
    );
}

// -------------------------------------------------------------- row 10

/// A frame whose origin's x and y read ONE anonymous variable, written
/// once in the insert's fresh table and carrying a spread; a square on
/// it, extruded. The frame, the profile and the extrude.
fn frame_sharing_a_fresh_entry(seed: &str) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let doc = ProfileDoc::empty(DocumentId::derive(seed), Tol::witness());
    let shared = || Formula::fresh(0, Dimension::Length);
    let scalar = |v: f64| crate::fixture::scl(v);
    let frame = step(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(Node::Datum(Datum::Frame {
                origin: [shared(), shared(), len(0.0)],
                u: [scalar(1.0), scalar(0.0), scalar(0.0)],
                v: [scalar(0.0), scalar(1.0), scalar(0.0)],
            })),
            fresh: vec![VarDecl::Free(FreeVar::continuous_with(
                Dimension::Length,
                0.25,
                Distribution::Normal { sigma: 0.002 },
            ))],
        },
    );
    assert_eq!(applied_fresh(&frame), 1, "the table minted its one entry");
    let frame_id = frame.record.minted.expect("the frame");
    let (doc, profile) = insert(
        frame.doc,
        Node::Profile(ProfileProgram {
            plane: frame_id,
            loops: vec![LoopProgram::polygon(square(0.0, 0.0, 0.5)).unwrap()],
            ids: Vec::new(),
        }),
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    (doc, [frame_id, profile, extrude])
}

fn applied_fresh(applied: &editor_core::Applied<ProfileProgram>) -> usize {
    applied.record.fresh.len()
}

/// Row 10: a cut node whose two slots read one anonymous variable lands
/// in the part reading ONE anonymous variable twice, its definition
/// (spread included) bit for bit, and the part loads. Breaks if the
/// carry writes each reader's value as its own written quantity.
#[test]
fn a_split_carries_an_anonymous_variable_read_twice_as_one() {
    let (doc, cut) = frame_sharing_a_fresh_entry("intent-literals-c-split");
    let source = doc
        .slot(cut[0], SlotId::Origin(editor_core::Axis3::X))
        .expect("the origin's x");
    assert_eq!(
        doc.slot(cut[0], SlotId::Origin(editor_core::Axis3::Y)),
        Some(source),
        "the premise: x and y read one variable"
    );
    let out = split(
        &doc,
        &BTreeSet::from(cut),
        DocumentId::derive("intent-literals-c-split-part"),
        Tol::witness(),
        None,
    )
    .expect("the cut reads only its own variables");
    let frame = out.node_map[&cut[0]];
    let x = out
        .part
        .slot(frame, SlotId::Origin(editor_core::Axis3::X))
        .expect("the carried x");
    assert_eq!(
        out.part.slot(frame, SlotId::Origin(editor_core::Axis3::Y)),
        Some(x),
        "x and y still read one variable"
    );
    assert!(out.part.var_name(x).is_none(), "and it stays anonymous");
    assert!(
        out.part
            .var(x)
            .expect("held")
            .bit_eq(doc.var(source).expect("held")),
        "its definition, spread included, bit for bit"
    );
    let text = save(&out.part, &[], Tol::witness()).expect("the part saves");
    assert!(
        load(&text, Tol::witness())
            .expect("and loads")
            .doc
            .bit_eq(&out.part)
    );
}

/// Row 10's refusal: an anonymous variable read on both sides of the
/// cut refuses `UncutVarReference`, as a named one does.
#[test]
fn an_anonymous_variable_read_on_both_sides_refuses_the_cut() {
    let (doc, cut) = frame_sharing_a_fresh_entry("intent-literals-c-uncut");
    let source = doc
        .slot(cut[0], SlotId::Origin(editor_core::Axis3::X))
        .expect("the origin's x");
    // A kept point reading the same variable.
    let (doc, kept) = insert(
        doc,
        Node::Datum(Datum::Point {
            position: [Formula::var(source, Dimension::Length), len(0.0), len(0.0)],
        }),
    );
    match split(
        &doc,
        &BTreeSet::from(cut),
        DocumentId::derive("intent-literals-c-uncut-part"),
        Tol::witness(),
        None,
    ) {
        Err(SplitError::UncutVarReference { var, kept_node, .. }) => {
            assert_eq!((var.id(), kept_node.id()), (source, kept));
        }
        other => panic!("a variable read on both sides refuses, got {other:?}"),
    }
}

// ------------------------------------------------------ the fresh table

fn point(position: [Formula; 3], fresh: Vec<VarDecl>) -> DocEdit<ProfileProgram> {
    DocEdit::InsertNode {
        node: Box::new(Node::Datum(Datum::Point { position })),
        fresh,
    }
}

fn empty(seed: &str) -> ProfileDoc {
    ProfileDoc::empty(DocumentId::derive(seed), Tol::witness())
}

/// A formula reading an entry the table does not hold refuses
/// `FreshUnheld`, and the document is unchanged.
#[test]
fn a_read_of_an_entry_the_table_does_not_hold_refuses() {
    let doc = empty("intent-literals-c-unheld");
    let edit = point(
        [Formula::fresh(1, Dimension::Length), len(0.0), len(0.0)],
        vec![length(1.0)],
    );
    match try_step(&doc, edit) {
        Err(EditError::FreshUnheld { index, referenced }) => {
            assert_eq!((index, referenced), (1, Dimension::Length));
        }
        other => panic!("an unheld entry refuses, got {other:?}"),
    }
}

/// A formula reading an entry at another kind refuses `FreshKind`.
#[test]
fn a_read_of_an_entry_at_another_kind_refuses() {
    let doc = empty("intent-literals-c-kind");
    let edit = point(
        [Formula::fresh(0, Dimension::Length), len(0.0), len(0.0)],
        vec![VarDecl::Free(FreeVar::continuous(Dimension::Angle, 1.0))],
    );
    match try_step(&doc, edit) {
        Err(EditError::FreshKind {
            index,
            held,
            referenced,
        }) => {
            assert_eq!(
                (index, held, referenced),
                (0, Dimension::Angle, Dimension::Length)
            );
        }
        other => panic!("an entry read at another kind refuses, got {other:?}"),
    }
}

/// An entry nothing the edit writes reads refuses `FreshUnread`: the
/// variable it would mint would have no reader (VR7).
#[test]
fn an_entry_nothing_reads_refuses() {
    let doc = empty("intent-literals-c-unread-entry");
    let edit = point(
        [Formula::fresh(0, Dimension::Length), len(0.0), len(0.0)],
        vec![length(1.0), length(2.0)],
    );
    match try_step(&doc, edit) {
        Err(EditError::FreshUnread { index }) => assert_eq!(index, 1),
        other => panic!("an unread entry refuses, got {other:?}"),
    }
}

/// An entry's definition reads an earlier entry, and the record names
/// the ids the table minted, entry by entry.
#[test]
fn an_entry_reads_the_entries_before_it() {
    let doc = empty("intent-literals-c-chain");
    let twice = Formula::mul(
        Formula::fresh(0, Dimension::Length),
        crate::fixture::scl(2.0),
    )
    .unwrap();
    let applied = step(
        &doc,
        point(
            [Formula::fresh(1, Dimension::Length), len(0.0), len(0.0)],
            vec![length(0.5), VarDecl::defined(twice)],
        ),
    );
    let [base, double] = applied.record.fresh[..] else {
        panic!("two entries minted: {:?}", applied.record.fresh)
    };
    let point = applied.record.minted.expect("the point");
    assert_eq!(
        applied
            .doc
            .slot(point, SlotId::Origin(editor_core::Axis3::X)),
        Some(double)
    );
    let definition = applied
        .doc
        .var(double)
        .and_then(|v| v.def().defined())
        .expect("the second entry is defined");
    let mut reads = Vec::new();
    definition.var_reads(&mut reads);
    assert_eq!(reads, vec![(base, Dimension::Length)]);
}

// ------------------------------------- an edit at a path keeps its reads

/// The variables `definition` reads, in order.
fn reads_of(doc: &ProfileDoc, var: VarId) -> Vec<VarId> {
    let definition = doc
        .var(var)
        .and_then(|v| v.def().defined())
        .expect("a formula's slot reads its anonymous definition");
    let mut reads = Vec::new();
    definition.var_reads(&mut reads);
    reads.into_iter().map(|(var, _)| var).collect()
}

/// A path edit to `x`'s right operand (`v · 2` → `v · 3`).
fn retimes(node: RecipeNodeId) -> DocEdit<ProfileProgram> {
    DocEdit::SetExpression {
        path: editor_core::ExprPath {
            node,
            slot: SlotId::Origin(editor_core::Axis3::X),
            path: vec![1],
        },
        expr: crate::fixture::scl(3.0),
    }
}

/// **An edit at a path re-lowers only the path** (r1 M1): a slot written
/// `v · 2`, where `v` is another node's toleranced anonymous variable,
/// edited at the `2` still reads `v` — by id, its distribution intact —
/// and no variable but the slot's own definition is retired. Breaks if
/// the door rebuilds the slot from its written expansion, which inlines
/// `v` as its value (`0.5 · 3`).
#[test]
fn an_edit_at_a_path_keeps_another_nodes_anonymous_read() {
    let doc = empty("intent-literals-c-path-shared");
    let applied = step(&doc, point([len(0.5), len(0.0), len(0.0)], Vec::new()));
    let a = applied.record.minted.expect("the point");
    let x = SlotId::Origin(editor_core::Axis3::X);
    let v = applied.doc.slot(a, x).expect("a reads its x");
    let doc = toleranced(&applied.doc, v);
    let twice = Formula::mul(Formula::var(v, Dimension::Length), crate::fixture::scl(2.0)).unwrap();
    let applied = step(&doc, point([twice, len(0.0), len(0.0)], Vec::new()));
    let b = applied.record.minted.expect("the second point");
    let before = applied.doc.slot(b, x).expect("b reads its x");
    assert_eq!(reads_of(&applied.doc, before), vec![v]);

    let edited = step(&applied.doc, retimes(b));
    let after = edited.doc.slot(b, x).expect("b still reads its x");
    assert_eq!(
        reads_of(&edited.doc, after),
        vec![v],
        "b still reads v by id"
    );
    assert_eq!(
        edited.doc.free(v).and_then(FreeVar::distribution),
        Some(&Distribution::Normal { sigma: 0.001 }),
        "v keeps its tolerance"
    );
    let retired: Vec<VarId> = edited
        .maintenance
        .iter()
        .filter_map(|m| match m {
            Maintenance::AnonymousVarRemoved { var, .. } => Some(var.id()),
            _ => None,
        })
        .collect();
    assert_eq!(retired, vec![before], "only b's own definition retires");
}

/// The same through a fresh entry: a slot whose definition reads its own
/// edit's toleranced fresh entry, edited at a path off that read, still
/// reads the entry's variable, spread and all.
#[test]
fn an_edit_at_a_path_keeps_a_fresh_entrys_read() {
    let doc = empty("intent-literals-c-path-fresh");
    let twice = Formula::mul(
        Formula::fresh(0, Dimension::Length),
        crate::fixture::scl(2.0),
    )
    .unwrap();
    let spread = VarDecl::Free(
        FreeVar::continuous(Dimension::Length, 0.5)
            .with_distribution(Some(Distribution::Normal { sigma: 0.001 }))
            .expect("a length takes a normal"),
    );
    let applied = step(
        &doc,
        point(
            [twice, Formula::fresh(0, Dimension::Length), len(0.0)],
            vec![spread],
        ),
    );
    let node = applied.record.minted.expect("the point");
    let [entry] = applied.record.fresh[..] else {
        panic!("one entry minted: {:?}", applied.record.fresh)
    };
    let edited = step(&applied.doc, retimes(node));
    let after = edited
        .doc
        .slot(node, SlotId::Origin(editor_core::Axis3::X))
        .expect("x");
    assert_eq!(reads_of(&edited.doc, after), vec![entry]);
    assert_eq!(
        edited.doc.slot(node, SlotId::Origin(editor_core::Axis3::Y)),
        Some(entry),
        "y still shares it"
    );
    assert_eq!(
        edited.doc.free(entry).and_then(FreeVar::distribution),
        Some(&Distribution::Normal { sigma: 0.001 })
    );
}

// -------------------------------------------------- row 14's C half

/// A slot reading an id the document never minted refuses at load
/// (`ReaderOfUnmintedVar`); a length slot reading a `Count` variable,
/// and a count slot reading a length one, refuse `SlotVarKind`: the
/// structural divide, both ways.
#[test]
fn the_load_door_reads_every_slots_variable() {
    let doc = empty("intent-literals-c-load");
    let (doc, point) = insert(
        doc,
        Node::Datum(Datum::Point {
            position: [len(1.0), len(0.0), len(0.0)],
        }),
    );
    let doc = declare(&doc, "n", VarDecl::Free(FreeVar::Count { value: 3 }));
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let at = |wire: &mut serde_json::Value, var: u64| {
        wire["snapshot"]["nodes"][point.0.to_string()]["Datum"]["Point"]["position"][1] =
            serde_json::json!(var);
    };
    let unminted = crate::wire::doctored(&text, |wire| at(wire, 1));
    match load(&unminted, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::ReaderOfUnmintedVar { node, var })) => {
            assert_eq!((node.id(), var), (point, VarId(1)));
        }
        other => panic!("a slot of an unminted id refuses, got {other:?}"),
    }
    let count = doc.var_named("n").expect("declared");
    let counted = crate::wire::doctored(&text, |wire| at(wire, count.0));
    match load(&counted, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::SlotVarKind {
            node,
            slot,
            declared,
            referenced,
            ..
        })) => assert_eq!(
            (node.id(), slot, declared, referenced),
            (
                point,
                SlotId::Origin(editor_core::Axis3::Y),
                Dimension::Count,
                Dimension::Length
            )
        ),
        other => panic!("a length slot reading a count refuses, got {other:?}"),
    }

    // The divide's other direction (r1 n3): a count slot reading a
    // continuous variable.
    let (doc, body) = crate::docm7_union_declare::block(
        empty("intent-literals-c-load-count"),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        1.0,
    );
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: body,
            count: Formula::count(3),
            kind: editor_core::PatternKind::Linear {
                direction: [
                    crate::fixture::scl(1.0),
                    crate::fixture::scl(0.0),
                    crate::fixture::scl(0.0),
                ],
                spacing: len(3.0),
            },
        },
    );
    let doc = declare(&doc, "w", length(0.5));
    let w = doc.var_named("w").expect("declared");
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let measured = crate::wire::doctored(&text, |wire| {
        wire["snapshot"]["nodes"][pattern.0.to_string()]["Pattern"]["count"] =
            serde_json::json!(w.0);
    });
    match load(&measured, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::SlotVarKind {
            node,
            slot,
            declared,
            referenced,
            ..
        })) => assert_eq!(
            (node.id(), slot, declared, referenced),
            (pattern, SlotId::Count, Dimension::Length, Dimension::Count)
        ),
        other => panic!("a count slot reading a length refuses, got {other:?}"),
    }
}

// ------------------------------------------- the analysis axes (VR8)

fn sym_session<R>(f: impl FnOnce() -> R) -> (R, geom_core::SymCounts) {
    geom_core::sym::with_session_rules(
        geom_core::SymBudget {
            max_terms: 4096,
            max_degree: 128,
        },
        geom_core::SymRules::shipped(),
        f,
    )
}

/// `a − b` as the symbolic lane binds the two variables over `doc`'s
/// nominal box, decided, with the session's counts.
fn sym_difference(
    doc: &ProfileDoc,
    a: VarId,
    b: VarId,
) -> (
    Result<geom_core::predicate::Sign, geom_core::predicate::Indeterminate>,
    geom_core::SymCounts,
) {
    use geom_core::Sym;
    sym_session(|| {
        let leaf = editor_core::ParamBox::from_axes(std::collections::BTreeMap::new());
        let env = editor_core::var_env_over::<Sym<f64>, _>(doc, &leaf).expect("binds");
        let bound = |var: VarId| match env.bindings[&var] {
            editor_core::ParamValue::Continuous { value, .. } => value,
            ref other => panic!("{var}: {other:?}"),
        };
        geom_core::k_stats::decide(
            "intent_literals_c",
            geom_core::predicate::Margin::of(bound(a) - bound(b)),
            geom_core::predicate::Band::new(1.0e-9, 1.0e-8).unwrap(),
        )
    })
}

/// Two blends typed `125 mm` apart, the two radius variables, and the
/// document.
fn typed_twice(seed: &str) -> (ProfileDoc, VarId, VarId, RecipeNodeId, RecipeNodeId) {
    let typed = || Formula::length_in(R_MM, quantity::MM).unwrap();
    let (doc, _, a) = filleted(empty(seed), 0.0, typed());
    let (doc, _, b) = filleted(doc, 4.0, typed());
    let (x, y) = (radius(&doc, a), radius(&doc, b));
    (doc, x, y, a, b)
}

fn toleranced(doc: &ProfileDoc, var: VarId) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetVarDistribution {
            var: var.into(),
            distribution: Some(Distribution::Normal { sigma: 0.001 }),
        },
    )
    .doc
}

/// An untoleranced variable is a constant of the symbolic lane (VR8):
/// two separately typed equal values decide `x − y` Zero there, named
/// or anonymous, while their tokens still differ — coincidence is
/// structure's question, not the analysis's. Breaks if the lane binds
/// an untoleranced variable as a symbol: the decision is then numeric
/// ([`assert_constants`]), though it still reads Zero.
#[test]
fn an_untoleranced_variable_is_a_constant_in_the_symbolic_lane() {
    let (doc, x, y, a, b) = typed_twice("intent-literals-c-untoleranced");
    assert_ne!(x, y, "two writings, two variables");
    let (decided, counts) = sym_difference(&doc, x, y);
    assert_eq!(decided, Ok(geom_core::predicate::Sign::Zero));
    assert_constants(&counts, "two typed values");
    let ev = eval(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    assert_ne!(
        radius_token(body_of(&ev, a)),
        radius_token(body_of(&ev, b)),
        "the tokens read ids"
    );

    let doc = declare(&doc, "w", length(0.0625));
    let doc = declare(&doc, "v", length(0.0625));
    let (w, v) = (doc.var_named("w").unwrap(), doc.var_named("v").unwrap());
    let (decided, counts) = sym_difference(&doc, w, v);
    assert_eq!(decided, Ok(geom_core::predicate::Sign::Zero), "named alike");
    assert_constants(&counts, "two named variables");
}

/// That `x − y` was decided over two constants: a theorem of the tier
/// and no numeric decision. Two symbols of no width at equal nominals
/// also read Zero, but numerically (`numeric: 1`), so a `Sign` alone
/// cannot tell the lane's binding apart (reviews r1 m1, r2 MINOR-2).
fn assert_constants(counts: &geom_core::SymCounts, what: &str) {
    assert_eq!(
        (counts.symbolic_zero, counts.numeric),
        (1, 0),
        "{what}: an untoleranced variable binds a constant, so its difference is a \
         theorem, not a numeric zero between two symbols: {counts:?}"
    );
}

/// A toleranced variable is an axis and a symbol, named or anonymous:
/// the analyzed box carries it, `x − x` is a theorem and `x − y` at
/// equal nominals is not. Breaks if the tolerance rule drops a
/// toleranced anonymous variable from the axes or binds it as a number.
#[test]
fn a_toleranced_variable_is_an_axis_and_a_symbol_named_or_anonymous() {
    let (doc, x, y, _, _) = typed_twice("intent-literals-c-toleranced");
    let doc = toleranced(&toleranced(&doc, x), y);
    let doc = declare(
        &doc,
        "w",
        VarDecl::Free(FreeVar::continuous_with(
            Dimension::Length,
            R_MM / 1000.0,
            Distribution::Normal { sigma: 0.001 },
        )),
    );
    let w = doc.var_named("w").unwrap();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let mut axes = vec![x, y, w];
    axes.sort();
    assert_eq!(analyzed.params().keys().copied().collect::<Vec<_>>(), axes);
    for (a, b) in [(x, y), (x, w)] {
        let (_, same) = sym_difference(&doc, a, a);
        assert_eq!(same.symbolic_zero, 1, "{a} − {a} is a theorem");
        let (_, apart) = sym_difference(&doc, a, b);
        assert_eq!(apart.symbolic_zero, 0, "{a} − {b} is two symbols");
    }
}

/// The stackup's entries and the Monte Carlo draws list only the
/// toleranced variables: a measure reading a toleranced `w` and an
/// untoleranced `v`, beside a point at three typed lengths, has one
/// entry and one draw, `w`'s. Breaks if an untoleranced variable is an axis again.
#[test]
fn stackup_and_monte_carlo_list_only_toleranced_variables() {
    let doc = empty("intent-literals-c-entries");
    let doc = declare(
        &doc,
        "w",
        VarDecl::Free(FreeVar::continuous_with(
            Dimension::Length,
            0.0625,
            Distribution::Normal { sigma: 0.001 },
        )),
    );
    let doc = declare(&doc, "v", length(0.0625));
    let sum = Formula::add(
        Formula::add(named("w"), named("v")).unwrap(),
        Formula::length_in(5.0, quantity::MM).unwrap(),
    )
    .unwrap();
    let applied = step(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(
                Node::measure(editor_core::MeasureExpr::value(sum), Vec::new()).unwrap(),
            ),
            fresh: Vec::new(),
        },
    );
    let (doc, measure) = (applied.doc, applied.record.minted.expect("an insert mints"));
    // Typed lengths at slot roots: anonymous free variables, untoleranced.
    let doc = step(&doc, point([len(0.25), len(0.5), len(0.75)], Vec::new())).doc;
    let w = doc.var_named("w").unwrap();
    assert_eq!(
        crate::fixture::continuous_vars(&doc),
        5,
        "the premise: w, v and the point's three typed lengths"
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    assert_eq!(
        analyzed.params().keys().copied().collect::<Vec<_>>(),
        vec![w]
    );
    let entries =
        editor_core::stackup::sensitivities(&doc, measure, None, None, false, None, Tol::witness())
            .expect("the driver runs");
    assert_eq!(
        entries.iter().map(|e| e.param).collect::<Vec<_>>(),
        vec![w],
        "one entry, w's"
    );
    let draw =
        editor_core::mc::sample_offsets(&doc, &analyzed, &editor_core::mc::McConfig::default(), 0)
            .expect("draws");
    assert_eq!(
        draw.keys().copied().collect::<Vec<_>>(),
        vec![w],
        "one draw, w's"
    );
}

// ------------------------------------- re-inserting as written (r1 m2)

/// **What a written re-insert would not reproduce is named**: a point
/// whose x and y read one toleranced fresh entry is, re-inserted from
/// `Node::written`, two untoleranced variables under a new id, so the
/// document names that entry, and `as_written` refuses the node rather
/// than rebuild another document. One read once and untoleranced is
/// not named. Breaks if the precondition stops seeing sharing or a
/// distribution.
#[test]
fn a_shared_or_toleranced_anonymous_variable_is_not_rewritten() {
    let doc = empty("intent-literals-c-written");
    let spread = VarDecl::Free(
        FreeVar::continuous(Dimension::Length, 0.5)
            .with_distribution(Some(Distribution::Normal { sigma: 0.001 }))
            .expect("a length takes a normal"),
    );
    let fresh = || Formula::fresh(0, Dimension::Length);
    let applied = step(&doc, point([fresh(), fresh(), len(0.0)], vec![spread]));
    let [entry] = applied.record.fresh[..] else {
        panic!("one entry: {:?}", applied.record.fresh)
    };
    let shared = applied.record.minted.expect("the point");
    assert_eq!(applied.doc.written_would_not_reproduce(), vec![entry]);
    let lone = step(
        &empty("intent-literals-c-written-lone"),
        point([len(0.5), len(0.0), len(0.0)], Vec::new()),
    );
    assert_eq!(lone.doc.written_would_not_reproduce(), Vec::new());
    let node = applied.doc.node(shared).expect("held").clone();
    let refused =
        std::panic::catch_unwind(|| editor_core::test_support::as_written(&applied.doc, &node));
    assert!(
        refused.is_err(),
        "as_written refuses a node it cannot reproduce"
    );
}
