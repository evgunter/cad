//! **A measure is an operation** (D10, INTENT stage 2 unit D): one
//! `Measure` holds one primitive and defines one observed scalar; the
//! arithmetic over measured values is a definition the assertion reads;
//! only an assertion reads an observed variable, at the edit door and at
//! the load door; and a measure's content key reads its sites by
//! content, never by id.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, cap_ref, insert, len, xform};
use crate::wire::doctored;
use editor_core::{
    AssertionDir, AssertionVerdict, CapEnd, Dimension, DocEdit, EditError, Formula,
    MeasurePrimitive, Node, NodeResult, PersistError, ProfileDoc, RecipeNodeId, SlotId,
    SnapshotError, ValuePayload, VarName, apply, load, save,
};
use geom_core::Tol;

/// Two unit slabs side by side, the second extruded `depth` deep: the
/// extrudes, whose caps two measures can read.
fn slabs(seed: &str, depth: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived(seed, Tol::witness());
    let (doc, _, a_profile) = fixture::on_frame_keeping(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.5, 0.5, 0.5)],
    );
    let (doc, a) = insert(
        doc,
        Node::Extrude {
            profile: a_profile.into(),
            distance: len(1.0),
            side: editor_core::ExtrudeSide::Along,
        },
    );
    let (doc, _, b_profile) = fixture::on_frame_keeping(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(3.5, 0.5, 0.5)],
    );
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: b_profile.into(),
            distance: len(depth),
            side: editor_core::ExtrudeSide::Along,
        },
    );
    (doc, a, b)
}

fn caps(node: RecipeNodeId) -> [editor_core::SitedRef; 2] {
    [cap_ref(node, CapEnd::Start), cap_ref(node, CapEnd::End)]
}

fn verdict(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> AssertionVerdict<f64> {
    match ev.result(id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Assertion(verdict) => verdict.clone(),
            other => panic!("{id:?} is a {}", other.kind_name()),
        },
        other => panic!("{id:?} did not evaluate: {other:?}"),
    }
}

/// **(D, test 13) Measure arithmetic is a definition.** `distance(a, b)
/// − distance(c, d)` is two measures and a formula over their outputs,
/// and the assertion over it holds one anonymous defined `Length`. The
/// verdict compares exactly the difference of the two measured values,
/// in that order, and a seeded run's tangent is the difference of theirs.
#[test]
fn measure_arithmetic_is_a_definition_the_assertion_reads() {
    let (doc, a, b) = slabs("s2d-arith", 2.5);
    let [a0, a1] = caps(a);
    let [b0, b1] = caps(b);
    let (doc, measured) = fixture::measure(
        doc,
        &[
            MeasurePrimitive::Distance { a: 2, b: 3 },
            MeasurePrimitive::Distance { a: 0, b: 1 },
        ],
        &[a0, a1, b0, b1],
    );
    assert_eq!(measured.measures.len(), 2, "one measure per primitive");
    for (&node, out) in measured.measures.iter().zip(&measured.outputs) {
        assert!(matches!(doc.node(node), Some(Node::Measure { .. })));
        assert_eq!(doc.output(node, 0), Some(*out));
    }
    let margin = Formula::sub(
        fixture::read_var(&doc, measured.outputs[0]),
        fixture::read_var(&doc, measured.outputs[1]),
    )
    .expect("Length - Length");
    let (doc, assertion) = insert(
        doc,
        Node::Assertion {
            value: margin,
            bound: len(1.0),
            dir: AssertionDir::AtLeast,
        },
    );
    let value = fixture::assertion_value(&doc, assertion);
    let held = doc.var(value).expect("the value is a variable");
    assert!(held.def().defined().is_some(), "arithmetic is a definition");
    assert_eq!(doc.var_name(value), None, "an anonymous one");
    assert_eq!(held.kind().dimension(), Some(Dimension::Length));
    assert!(
        doc.observed().contains(&value),
        "a definition over outputs is observed"
    );

    let ev = crate::corpus::eval::<f64>(&doc);
    let reading = |out| fixture::reading(&doc, &ev, out).expect("a measure reads");
    let (deep, unit) = (reading(measured.outputs[0]), reading(measured.outputs[1]));
    assert_eq!(
        (deep, unit),
        (2.5, 1.0),
        "each measure is its own slab's depth"
    );
    match verdict(&ev, assertion) {
        AssertionVerdict::Holds { measured, bound } => {
            assert_eq!(
                measured.to_bits(),
                (deep - unit).to_bits(),
                "the operands in order"
            );
            assert_eq!(bound.to_bits(), 1.0f64.to_bits());
        }
        other => panic!("1.5 >= 1 holds, got {other:?}"),
    }

    // A seeded run: the definition is bound from the measures' values in
    // that lane, so its tangent is the difference of theirs, bit for bit.
    let doc = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::new("h").unwrap(),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::continuous(
                Dimension::Length,
                2.5,
            )),
        },
    )
    .0;
    let doc = fixture::step(
        doc,
        DocEdit::SetParam {
            node: b,
            slot: SlotId::Distance,
            value: Formula::named(VarName::new("h").unwrap(), Dimension::Length).into(),
            fresh: Vec::new(),
        },
    )
    .0;
    let seeded = editor_core::evaluate::<geom_core::Dual64>(
        &doc,
        None,
        &editor_core::CancelToken::new(),
        &editor_core::EvalOptions {
            seed: doc.var_named("h"),
            ..editor_core::EvalOptions::default()
        },
        Tol::witness(),
    );
    let lane = |var| fixture::reading(&doc, &seeded, var).expect("reads at Dual64");
    let (web, deep, unit) = (
        lane(value),
        lane(measured.outputs[0]),
        lane(measured.outputs[1]),
    );
    assert_eq!(web.value.to_bits(), (deep.value - unit.value).to_bits());
    assert_eq!(web.deriv.to_bits(), (deep.deriv - unit.deriv).to_bits());
    assert_eq!(
        web.deriv, 1.0,
        "the deep slab's depth is the seeded variable"
    );
}

/// **(D, test 14) Observed is read only by an assertion.** A
/// construction's slot reading a measure's output refuses at the edit
/// door, and so does one reading `m + 1 mm` with `m` observed, and a
/// redefinition that makes a slot's variable observed; an assertion
/// reading it is accepted; and a file whose extrude reads it refuses at
/// the load door.
#[test]
fn an_observed_variable_is_read_only_by_an_assertion() {
    let (doc, a, b) = slabs("s2d-observed", 2.5);
    let (doc, measure) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        caps(a).to_vec(),
    );
    let out = fixture::output(&doc, measure);
    let refused = |doc: &ProfileDoc, edit: DocEdit<editor_core::ProfileProgram>| match apply(
        doc,
        &edit,
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Ok(_) => panic!("a construction reading a measured value was accepted"),
        Err(refusal) => refusal,
    };
    let set_depth = |expr: Formula| DocEdit::SetParam {
        node: b,
        slot: SlotId::Distance,
        value: expr.into(),
        fresh: Vec::new(),
    };

    // Directly.
    match refused(&doc, set_depth(Formula::var(out, Dimension::Length))) {
        EditError::ConstructionReadsObserved { node, slot, var } => {
            assert_eq!((node.id(), slot, var.id()), (b, SlotId::Distance, out));
        }
        other => panic!("a slot reading a measure's output refuses, got {other:?}"),
    }
    // As a read, written.
    match apply(
        &doc,
        &DocEdit::SetParam {
            node: b,
            slot: SlotId::Distance,
            value: editor_core::Operand::from(out).into(),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::ConstructionReadsObserved { node, slot, var }) => {
            assert_eq!((node.id(), slot, var.id()), (b, SlotId::Distance, out));
        }
        other => panic!("a slot's read of a measure's output refuses, got {other:?}"),
    }
    // Through a definition the slot's own formula defines.
    let doc = fixture::step(
        doc,
        DocEdit::RenameVar {
            var: out.into(),
            name: Some(VarName::new("m").unwrap()),
        },
    )
    .0;
    let m_plus = Formula::add(
        Formula::named(VarName::new("m").unwrap(), Dimension::Length),
        len(0.001),
    )
    .unwrap();
    match refused(&doc, set_depth(m_plus)) {
        EditError::ConstructionReadsObserved { node, slot, .. } => {
            assert_eq!((node.id(), slot), (b, SlotId::Distance));
        }
        other => panic!("a slot reading `m + 1 mm` refuses, got {other:?}"),
    }
    // Through a redefinition of a variable a slot already reads.
    let doc = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::new("depth").unwrap(),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::continuous(
                Dimension::Length,
                2.5,
            )),
        },
    )
    .0;
    let doc = fixture::step(
        doc,
        set_depth(Formula::named(
            VarName::new("depth").unwrap(),
            Dimension::Length,
        )),
    )
    .0;
    match refused(
        &doc,
        DocEdit::DefineVar {
            var: VarName::new("depth").unwrap().into(),
            def: editor_core::VarDecl::Defined(Formula::named(
                VarName::new("m").unwrap(),
                Dimension::Length,
            )),
            fresh: Vec::new(),
        },
    ) {
        EditError::ConstructionReadsObserved { node, var, .. } => {
            assert_eq!(node.id(), b);
            assert_eq!(Some(var.id()), doc.var_named("depth"));
        }
        other => panic!("a redefinition making a slot observed refuses, got {other:?}"),
    }
    // An assertion reads it.
    let (doc, assertion) = insert(
        doc,
        Node::Assertion {
            value: Formula::add(
                Formula::named(VarName::new("m").unwrap(), Dimension::Length),
                len(0.001),
            )
            .unwrap(),
            bound: len(0.5),
            dir: AssertionDir::AtLeast,
        },
    );
    assert!(matches!(
        verdict(&crate::corpus::eval::<f64>(&doc), assertion),
        AssertionVerdict::Holds { .. }
    ));

    // The load door: the extrude's depth forged to read the output.
    let depth = doc.var_named("depth").expect("declared");
    let text = save(&doc, &[], Tol::witness()).expect("the document saves");
    load(&text, Tol::witness()).expect("its own bytes load");
    let corrupt = doctored(&text, |wire| {
        let field = &mut wire["snapshot"]["nodes"][b.0.to_string()]["Extrude"]["distance"];
        assert_eq!(
            *field,
            serde_json::json!(depth.0),
            "aimed at the extrude's depth"
        );
        *field = serde_json::json!(out.0);
    });
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::ObservedRead { node, slot, var })) => {
            assert_eq!((node.id(), slot, var.id()), (b, SlotId::Distance, out));
        }
        other => panic!("a construction reading an output refuses at load, got {other:?}"),
    }
}

/// **(D, test 15) A measure's content key reads its sites by content.**
/// Two measures of one name read at two identity placements of one body
/// are one content and two names: their content keys agree and their
/// naming keys do not.
#[test]
fn a_measure_key_reads_its_sites_by_content_not_by_id() {
    let (doc, a, _) = slabs("s2d-key", 2.5);
    let (doc, first) = insert(doc, xform(a, [0.0; 3], [0.0, 0.0, 1.0], 0.0));
    let (doc, second) = insert(doc, xform(a, [0.0; 3], [0.0, 0.0, 1.0], 0.0));
    let at = |site| caps(a).map(|r| editor_core::SitedRef::new(site, r.name));
    let distance = |[a, b]: [editor_core::SitedRef; 2]| Node::Measure {
        primitive: MeasurePrimitive::Distance { a, b },
    };
    let (doc, on_first) = insert(doc, distance(at(first)));
    let (doc, on_second) = insert(doc, distance(at(second)));
    let ev = crate::corpus::eval::<f64>(&doc);
    let value = |node| ev.value(node).expect("the measure evaluates");
    assert_eq!(
        value(first).content_key,
        value(second).content_key,
        "the premise: two placements of equal content"
    );
    assert_eq!(
        value(on_first).content_key,
        value(on_second).content_key,
        "a site enters the key by its content"
    );
    assert_ne!(
        value(on_first).naming_key,
        value(on_second).naming_key,
        "and by its id in the naming key"
    );
}

/// **(D, test 13) The definition is bound after its measures run, in
/// any document order.** An assertion reads a free `w`; a measure is
/// inserted after it; then `w` is redefined as `m + 0.25 mm`. The edit
/// is legal — an assertion may read an observed value — and the verdict
/// compares the measured depth plus the pad: the assertion runs after
/// the measure its value now reads, though it was inserted before it.
#[test]
#[ignore = "green once C deletes the roots backstop: today `DefineVar` meets it (MINOR-2)"]
fn a_redefinition_over_a_measure_binds_after_the_measure_runs() {
    let (doc, a, _) = slabs("s2d-order", 2.5);
    let w = VarName::new("w").unwrap();
    let (doc, _) = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: w.clone(),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::continuous(
                Dimension::Length,
                0.5,
            )),
        },
    );
    let (doc, assertion) = insert(
        doc,
        Node::Assertion {
            value: Formula::named(w.clone(), Dimension::Length),
            bound: len(0.0),
            dir: AssertionDir::AtLeast,
        },
    );
    let (doc, measure) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        caps(a).to_vec(),
    );
    let out = fixture::output(&doc, measure);
    let doc = apply(
        &doc,
        &DocEdit::DefineVar {
            var: w.into(),
            def: editor_core::VarDecl::Defined(
                Formula::add(Formula::var(out, Dimension::Length), len(0.25e-3)).unwrap(),
            ),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("an assertion may read a value redefined over a measure")
    .doc;
    let ev = crate::corpus::eval::<f64>(&doc);
    match verdict(&ev, assertion) {
        AssertionVerdict::Holds { measured, .. } => {
            assert_eq!(measured.to_bits(), (1.0f64 + 0.25e-3).to_bits());
        }
        other => panic!("the measured depth plus the pad holds, got {other:?}"),
    }
}

/// **(D, test 13) A measure under a definition is upstream of the
/// assertion reading it.** The measure fails (its first reference is
/// read at a node that never minted it), and the assertion over a
/// definition over its output is poisoned THROUGH that measure — the
/// edge a definition carries — rather than reading an unbound value.
#[test]
fn an_assertion_over_a_definition_is_poisoned_by_the_measure_under_it() {
    let (doc, a, b) = slabs("s2d-poison", 2.5);
    let [a0, a1] = caps(a);
    let stray = editor_core::SitedRef::new(b, a0.name);
    let (doc, measure) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![stray, a1],
    );
    let out = fixture::output(&doc, measure);
    let (doc, assertion) = insert(
        doc,
        Node::Assertion {
            value: Formula::add(Formula::var(out, Dimension::Length), len(1.0)).unwrap(),
            bound: len(0.0),
            dir: AssertionDir::AtLeast,
        },
    );
    let ev = crate::corpus::eval::<f64>(&doc);
    assert!(
        matches!(ev.result(measure), Some(NodeResult::Failed(_))),
        "the premise: the measure fails, got {:?}",
        ev.result(measure)
    );
    match ev.result(assertion) {
        Some(NodeResult::Poisoned { through }) => assert_eq!(*through, measure),
        other => panic!("poisoned through the measure under its value, got {other:?}"),
    }
}

/// **A split refuses to carry an assertion reading a stranded measured
/// value**, typed: the measure under it was deleted, so its output is
/// defined by nothing live. The cut's own check names it before any node
/// is carried; the carry's remap refuses it too
/// (`RemapMiss::PayloadRead`), so a value is never kept silently.
#[test]
fn a_split_refuses_an_assertion_over_a_stranded_measure() {
    let (doc, a, _) = slabs("s2d-split", 2.5);
    let (doc, measure) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        caps(a).to_vec(),
    );
    let out = fixture::output(&doc, measure);
    let (doc, assertion) = insert(
        doc,
        Node::Assertion {
            value: Formula::var(out, Dimension::Length),
            bound: len(0.0),
            dir: AssertionDir::AtLeast,
        },
    );
    let (doc, _) = fixture::step(doc, DocEdit::DeleteNode { id: measure });
    let mut cut = std::collections::BTreeSet::from([assertion]);
    let mut stack = vec![a];
    while let Some(id) = stack.pop() {
        if cut.insert(id) {
            stack.extend(doc.upstream(id));
        }
    }
    match editor_core::split(
        &doc,
        &cut,
        editor_core::DocumentId::derive("s2d-split-part"),
        Tol::witness(),
        None,
    ) {
        Err(editor_core::SplitError::UnresolvedVarCrossesCut { var, node }) => {
            assert_eq!((var.id(), node.id()), (out, assertion));
        }
        other => panic!("the carry refuses the stranded value, got {other:?}"),
    }
}

/// **Which side of an assertion has no value is a word of its key.** A
/// value with no value at `f64` (a `min_clearance`) bounded by a length,
/// and that length bounded by the same clearance, leave one evaluated
/// number each and the same upstream: their keys differ by where the
/// missing one stood.
#[test]
fn an_unavailable_side_keys_by_its_position() {
    let (doc, a, b) = slabs("s2d-unavailable", 2.5);
    let (doc, clearance) = fixture::measure_node(
        &doc,
        MeasurePrimitive::MinClearance { a: 0, b: 1 },
        vec![cap_ref(a, CapEnd::End), cap_ref(b, CapEnd::End)],
    );
    let gap = Formula::var(fixture::output(&doc, clearance), Dimension::Length);
    let (doc, low) = insert(
        doc,
        Node::Assertion {
            value: gap.clone(),
            bound: len(0.5),
            dir: AssertionDir::AtLeast,
        },
    );
    let (doc, high) = insert(
        doc,
        Node::Assertion {
            value: len(0.5),
            bound: gap,
            dir: AssertionDir::AtLeast,
        },
    );
    let ev = crate::corpus::eval::<f64>(&doc);
    let value = |node| {
        ev.value(node)
            .expect("an assertion with no value is Unevaluated")
    };
    assert_ne!(value(low).content_key, value(high).content_key);
}
