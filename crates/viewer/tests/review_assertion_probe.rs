//! Reviewer probes for PR 7348 (the assertion tool). Not for merge.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use pncad::document::{
    AssertionRelation, CheckEvidence, CheckId, Claim, ClearanceBounds, Doc, Formula,
    MeasurePrimitive, Node, Operand, ProfileProgram, RecipeNodeId,
};
use pncad::geom_core::Tol;
use viewer::session::{
    AssertedClaim, AssertedSeat, DocSession, FaceSelection, Refusal, SessionOp, ValueMeasure,
};
use viewer::test_support::{CLEARANCE_SIDE as SIDE, inserted, len, placed, rectangle};

fn touching(tol: Tol) -> (DocSession, [RecipeNodeId; 2]) {
    let mut doc = Doc::<ProfileProgram>::empty_derived("two-blocks-touching", tol);
    let mut copies = [RecipeNodeId::WORLD; 2];
    for (i, x) in [0.0, SIDE].into_iter().enumerate() {
        let (with, profile) = inserted(&doc, rectangle([x, 0.0], SIDE, SIDE), tol);
        let (with, block) = inserted(
            &with,
            Node::Extrude {
                profile: profile.into(),
                distance: len(0.01),
            },
            tol,
        );
        let (with, copy) = placed(&with, block, tol);
        doc = with;
        copies[i] = copy;
    }
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    (session, copies)
}

fn wall(session: &DocSession, copy: RecipeNodeId) -> FaceSelection {
    let (_, eval) = session.landed_pair().expect("the scene landed");
    let lateral = pncad::select::Selector::of(
        pncad::select::NamePat::of_kind(pncad::select::EntityKind::Face)
            .seg(pncad::select::SegPat::tag(pncad::select::SegTag::Lateral)),
    );
    pncad::select::select(eval, copy, &lateral)
        .into_iter()
        .find(|name| {
            pncad::select::face_frame(eval, copy, name)
                .is_ok_and(|pose| (pose.origin.x - SIDE).abs() < 1e-9 && pose.axis.x.abs() > 0.5)
        })
        .map(|name| FaceSelection {
            name,
            node: copy,
            body: 0,
        })
        .expect("wall")
}

fn read_wall(session: &DocSession, copy: RecipeNodeId) -> Operand {
    let face = wall(session, copy);
    Operand::select(copy, vec![face.name])
}

fn contact_quiet(session: &DocSession) -> Option<RecipeNodeId> {
    let report = session.checks().expect("the registry ran");
    let found: Vec<_> = report
        .findings
        .iter()
        .filter(|f| {
            f.check == CheckId::UnprovenCoincidence
                && matches!(
                    &f.evidence,
                    CheckEvidence::UnprovenCoincidence { row, .. }
                        if row.relation == pncad::topo::Relation::SameOpposite
                )
        })
        .collect();
    assert_eq!(found.len(), 1, "one contact: {found:?}");
    found[0].quiet
}

fn value(measure: ValueMeasure, relation: AssertionRelation, bound: Formula) -> AssertedClaim {
    AssertedClaim::Value {
        measure,
        relation,
        bound,
    }
}

/// Claim 2: the doc the op writes is bit-equal to the Python binding's
/// pair (`Node.measure`, then `Node.assertion(Doc.output(m), ...)`,
/// the value at the output var's own kind's dimension).
#[test]
fn probe_value_claim_is_the_python_pair() {
    let tol = Tol::witness();
    for (measure, prim) in [
        (ValueMeasure::Gap, 0usize),
        (ValueMeasure::Distance, 1usize),
    ] {
        for relation in [
            AssertionRelation::Equal,
            AssertionRelation::AtLeast,
            AssertionRelation::AtMost,
        ] {
            let (mut session, copies) = touching(tol);
            let (a, b) = (read_wall(&session, copies[0]), read_wall(&session, copies[1]));
            let before = session.committed_doc().clone();
            let bound = len(0.003);
            let out = session.perform(SessionOp::AddAssertion {
                a: AssertedSeat::Read(a.clone()),
                b: AssertedSeat::Read(b.clone()),
                claim: value(measure, relation, bound.clone()),
            });
            assert!(out.refusal.is_none(), "{:?}", out.refusal);
            let primitive = if prim == 0 {
                MeasurePrimitive::Gap { outer: a, inner: b }
            } else {
                MeasurePrimitive::Distance { a, b }
            };
            let (doc, m) = inserted(&before, Node::measure(&primitive), tol);
            let output = doc.output(m, 0).expect("output");
            let dim = doc
                .var(output)
                .map(|v| v.kind())
                .and_then(|k| k.dimension())
                .expect("a scalar");
            let (doc, made) = inserted(
                &doc,
                Node::Assertion(Claim::Value {
                    value: Formula::var(output, dim),
                    bound,
                    relation,
                }),
                tol,
            );
            assert_eq!(out.made, Some(made), "{measure:?}/{relation:?}: same id");
            assert!(
                session.committed_doc().bit_eq(&doc),
                "{measure:?}/{relation:?}: bit-equal to the Python pair"
            );
        }
    }
}

/// Claim 3 extended: a Gap AT LEAST 0 (and a Distance = 0 over faces)
/// does not quiet the contact. Goes red if the relation is dropped.
#[test]
fn probe_only_gap_equal_zero_quiets() {
    let tol = Tol::witness();
    let (mut session, copies) = touching(tol);
    let (wa, wb) = (wall(&session, copies[0]), wall(&session, copies[1]));
    let seats = || [AssertedSeat::Face(wa.clone()), AssertedSeat::Face(wb.clone())];
    for (m, r, bound, quiets) in [
        (ValueMeasure::Gap, AssertionRelation::AtLeast, 0.0, false),
        (ValueMeasure::Gap, AssertionRelation::AtMost, 0.0, false),
        (ValueMeasure::Distance, AssertionRelation::Equal, 0.0, false),
        (ValueMeasure::Gap, AssertionRelation::Equal, 0.0, true),
    ] {
        let [a, b] = seats();
        let out = session.perform(SessionOp::AddAssertion {
            a,
            b,
            claim: value(m, r, len(bound)),
        });
        assert!(out.refusal.is_none(), "{:?}", out.refusal);
        session.pump();
        let q = contact_quiet(&session);
        assert_eq!(q.is_some(), quiets, "{m:?} {r:?} {bound}: quiet {q:?}");
        assert!(session.perform(SessionOp::Undo).refusal.is_none());
        session.pump();
    }
}

/// Claim 3/written order: the Gap's outer is seat a.
#[test]
fn probe_seat_order_is_written() {
    let tol = Tol::witness();
    let (mut session, copies) = touching(tol);
    let (a, b) = (read_wall(&session, copies[0]), read_wall(&session, copies[1]));
    let out = session.perform(SessionOp::AddAssertion {
        a: AssertedSeat::Read(a.clone()),
        b: AssertedSeat::Read(b.clone()),
        claim: value(ValueMeasure::Gap, AssertionRelation::Equal, len(0.0)),
    });
    let made = out.made.expect("made");
    let doc = session.committed_doc();
    let Some(Node::Assertion(Claim::Value { value, .. })) = doc.node(made) else {
        panic!("value claim")
    };
    let v = *value;
    let (m, 0) = doc.var(v).and_then(|v| v.def().output()).expect("output") else {
        panic!()
    };
    let Some(Node::Measure {
        primitive: MeasurePrimitive::Gap { outer, inner },
    }) = doc.node(m)
    else {
        panic!("a gap")
    };
    let (oa, ob) = (doc.var(*outer).expect("o"), doc.var(*inner).expect("i"));
    assert_ne!(format!("{oa:?}"), format!("{ob:?}"));
    assert!(format!("{oa:?}").contains(&format!("{:?}", wall(&session, copies[0]).name)));
}

/// Claim 4: perform, undo, redo — undo is bit-equal to before, redo to after.
#[test]
fn probe_undo_redo_bit_equal() {
    let tol = Tol::witness();
    let (mut session, copies) = touching(tol);
    for claim in [
        AssertedClaim::Clearance(ClearanceBounds::Within {
            lo: len(0.0),
            hi: len(0.001),
        }),
        value(ValueMeasure::Gap, AssertionRelation::Equal, len(0.0)),
        value(ValueMeasure::Distance, AssertionRelation::AtMost, len(0.001)),
    ] {
        let before = session.committed_doc().clone();
        let out = session.perform(SessionOp::AddAssertion {
            a: AssertedSeat::Face(wall(&session, copies[0])),
            b: AssertedSeat::Face(wall(&session, copies[1])),
            claim: claim.clone(),
        });
        assert!(out.refusal.is_none(), "{:?}", out.refusal);
        session.pump();
        let after = session.committed_doc().clone();
        assert!(session.perform(SessionOp::Undo).refusal.is_none());
        session.pump();
        assert!(session.committed_doc().bit_eq(&before), "{claim:?} undo");
        assert!(session.perform(SessionOp::Redo).refusal.is_none());
        session.pump();
        assert!(session.committed_doc().bit_eq(&after), "{claim:?} redo");
        // A second undo now, for the next iteration's start.
        assert!(session.perform(SessionOp::Undo).refusal.is_none());
        session.pump();
        assert!(session.committed_doc().bit_eq(&before), "{claim:?} undo 2");
    }
}

/// Face against a body: a clearance is written; a Gap or a Distance
/// refuses at the door and writes nothing.
#[test]
fn probe_face_against_body() {
    let tol = Tol::witness();
    let (mut session, copies) = touching(tol);
    let before = session.committed_doc().clone();
    let out = session.perform(SessionOp::AddAssertion {
        a: AssertedSeat::Face(wall(&session, copies[0])),
        b: AssertedSeat::Body(copies[1].into()),
        claim: AssertedClaim::Clearance(ClearanceBounds::AtLeast(len(0.0))),
    });
    assert!(out.refusal.is_none(), "clearance face/body: {:?}", out.refusal);
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    session.pump();
    for m in [ValueMeasure::Gap, ValueMeasure::Distance] {
        let out = session.perform(SessionOp::AddAssertion {
            a: AssertedSeat::Face(wall(&session, copies[0])),
            b: AssertedSeat::Body(copies[1].into()),
            claim: value(m, AssertionRelation::Equal, len(0.0)),
        });
        eprintln!("{m:?} face/body: {:?}", out.refusal);
        assert!(
            matches!(out.refusal, Some(Refusal::Edit(_))),
            "{m:?}: {:?}",
            out.refusal
        );
        assert!(session.committed_doc().bit_eq(&before), "{m:?}: nothing written");
    }
}

/// A pose seat: the Distance between the two walls' planes is written as
/// given (or refused in the door's words, with nothing written).
#[test]
fn probe_pose_seat() {
    let tol = Tol::witness();
    let (mut session, copies) = touching(tol);
    let plane = |op: Operand| {
        Operand::Pose(Box::new(pncad::document::pose::PoseDef::Point { of: op }))
    };
    let before = session.committed_doc().clone();
    let a = plane(read_wall(&session, copies[0]));
    let b = plane(read_wall(&session, copies[1]));
    let out = session.perform(SessionOp::AddAssertion {
        a: AssertedSeat::Read(a),
        b: AssertedSeat::Read(b),
        claim: value(ValueMeasure::Distance, AssertionRelation::AtLeast, len(0.0)),
    });
    eprintln!("pose seat: refusal {:?} made {:?}", out.refusal, out.made);
    if out.refusal.is_some() {
        assert!(session.committed_doc().bit_eq(&before));
    } else {
        session.pump();
        let r = session.checks().expect("ran");
        eprintln!("pending {}", r.pending.len());
    }
}

/// Claim 6: a Read seat built from the finding's raw cell names.
#[test]
fn probe_read_from_raw_finding_cells() {
    let tol = Tol::witness();
    let (mut session, _copies) = touching(tol);
    let report = session.checks().expect("ran");
    let row = report
        .findings
        .iter()
        .find_map(|f| match &f.evidence {
            CheckEvidence::UnprovenCoincidence { row, .. }
                if row.relation == pncad::topo::Relation::SameOpposite =>
            {
                Some(row.clone())
            }
            _ => None,
        })
        .expect("row");
    eprintln!("cells: {:#?}", row.cells);
    let seats: Vec<AssertedSeat> = row
        .cells
        .iter()
        .map(|c| match c {
            pncad::document::NamedCell::Entity { input, name } => {
                AssertedSeat::Read(Operand::select(Operand::Var(*input), vec![name.clone()]))
            }
            other => panic!("{other:?}"),
        })
        .collect();
    let out = session.perform(SessionOp::AddAssertion {
        a: seats[0].clone(),
        b: seats[1].clone(),
        claim: value(ValueMeasure::Gap, AssertionRelation::Equal, len(0.0)),
    });
    eprintln!("raw-cell read: refusal {:?}", out.refusal);
    session.pump();
    eprintln!("raw-cell read quiet: {:?}", contact_quiet(&session));
}

/// The tool's own Gap path: AssertionTool + AssertionMeasureChoice +
/// RelationChoice write the relation chosen, and the form's "gap,
/// exactly" quiets the contact. No PR row goes through this path.
#[test]
fn probe_the_tools_gap_path_writes_the_relation_chosen_and_quiets() {
    use viewer::combine::{AssertionMeasureChoice, AssertionTool, ClearanceBoundChoice, RelationChoice};
    let tol = Tol::witness();
    let (mut session, copies) = touching(tol);
    let mut tool = AssertionTool::new();
    tool.seat_face(session.committed_doc(), wall(&session, copies[0]));
    tool.seat_face(session.committed_doc(), wall(&session, copies[1]));
    for (choice, relation) in [
        (RelationChoice::AtLeast, AssertionRelation::AtLeast),
        (RelationChoice::AtMost, AssertionRelation::AtMost),
        (RelationChoice::Equal, AssertionRelation::Equal),
    ] {
        for (measure, verb) in [
            (AssertionMeasureChoice::Distance, pncad::document::MeasureVerb::Distance),
            (AssertionMeasureChoice::Gap, pncad::document::MeasureVerb::Gap),
        ] {
            let op = tool
                .op(measure.claim(ClearanceBoundChoice::default(), choice, len(0.0)))
                .expect("both seats held");
            let out = session.perform(op);
            let made = out.made.expect("made");
            let doc = session.committed_doc();
            let Some(Node::Assertion(Claim::Value { value, relation: written, .. })) = doc.node(made)
            else {
                panic!("{measure:?}: a value claim")
            };
            assert_eq!(*written, relation, "{measure:?} {choice:?}");
            let (m, _) = doc.var(*value).and_then(|v| v.def().output()).expect("output");
            let Some(Node::Measure { primitive }) = doc.node(m) else { panic!() };
            assert_eq!(primitive.kind(), verb, "{measure:?}");
            session.pump();
            let quiet = contact_quiet(&session);
            assert_eq!(
                quiet.is_some(),
                measure == AssertionMeasureChoice::Gap && choice == RelationChoice::Equal,
                "{measure:?} {choice:?}: {quiet:?}"
            );
            assert!(session.perform(SessionOp::Undo).refusal.is_none());
            session.pump();
        }
    }
}

#[test]
fn probe_verdicts_of_pose_and_raw_cell_reads() {
    let tol = Tol::witness();
    let (mut session, copies) = touching(tol);
    let report = session.checks().expect("ran");
    let row = report.findings.iter().find_map(|f| match &f.evidence {
        CheckEvidence::UnprovenCoincidence { row, .. } if row.relation == pncad::topo::Relation::SameOpposite => Some(row.clone()),
        _ => None,
    }).expect("row");
    let raw: Vec<AssertedSeat> = row.cells.iter().map(|c| match c {
        pncad::document::NamedCell::Entity { input, name } => AssertedSeat::Read(Operand::select(Operand::Var(*input), vec![name.clone()])),
        other => panic!("{other:?}"),
    }).collect();
    let out = session.perform(SessionOp::AddAssertion { a: raw[0].clone(), b: raw[1].clone(), claim: value(ValueMeasure::Gap, AssertionRelation::Equal, len(0.0)) });
    let made = out.made.expect("made");
    session.pump();
    let (_, eval) = session.landed_pair().expect("landed");
    eprintln!("RAW verdict: {:?}", eval.value(made).map(|v| &v.payload));
    eprintln!("RAW node errors: {:?}", eval.node_error(made));
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    session.pump();
    let point = |op: Operand| Operand::Pose(Box::new(pncad::document::pose::PoseDef::Point { of: op }));
    let out = session.perform(SessionOp::AddAssertion {
        a: AssertedSeat::Read(point(read_wall(&session, copies[0]))),
        b: AssertedSeat::Read(point(read_wall(&session, copies[1]))),
        claim: value(ValueMeasure::Distance, AssertionRelation::AtLeast, len(0.0)),
    });
    let made = out.made.expect("made");
    session.pump();
    let (_, eval) = session.landed_pair().expect("landed");
    eprintln!("POSE verdict: {:?}", eval.value(made).map(|v| &v.payload));
    eprintln!("POSE node errors: {:?}", eval.node_error(made));
}
