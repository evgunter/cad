//! Blinded-review probes, M5 PR 9 charter A + deviation 9 + the
//! TangentIntersection-at-rest save/load row:
//!
//! 1. The boss-union DOCUMENT row the spec's shape (ii) acceptance
//!    named ("joins the Band 4 corpus"): authored here independently
//!    — f64 volume against a closed form, save/load replay identity,
//!    and the INTERVAL lane with a bit-replay pin (deviation 9's
//!    unauthored row, authored adversarially).
//! 2. A document whose evaluation carries `TangentIntersection`
//!    edges at rest (the filleted extrude): save, load, re-evaluate
//!    — the loaded document's body must carry the same intrinsic
//!    tangent descriptions (persistence is replay; the variant must
//!    survive the round trip).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    BooleanOp, BooleanValue, CancelToken, DocEdit, EvalOptions, LoopProgram, Node, ProfileDoc,
    ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, ValuePayload, apply, evaluate, load,
    save,
};
use geom_core::{Affine3, Vec3};
use profile::SketchPlane;
use geom_core::Tol;

struct Rec {
    doc: ProfileDoc,
    edits: Vec<DocEdit<ProfileProgram>>,
}

impl Rec {
    fn new() -> Self {
        Self {
            doc: ProfileDoc::empty_derived("review_m5_pr9_doc_probe", Tol::witness()),
            edits: Vec::new(),
        }
    }
    fn insert(&mut self, node: Node<ProfileProgram>) -> RecipeNodeId {
        let edit = DocEdit::InsertNode { node };
        let applied = apply(&self.doc, &edit, Tol::witness()).expect("edit applies");
        self.doc = applied.doc;
        self.edits.push(edit);
        applied.record.minted.expect("minted id")
    }
}

fn len(v: f64) -> editor_core::Expr {
    editor_core::Expr::literal(v, editor_core::Dimension::Length).unwrap()
}

/// The boss-union document: 3x3x0.8 plate, r=0.35 three-arc boss at
/// (1.2, 1.7) from z=0.3, height 1.0.
fn boss_union_doc() -> (ProfileDoc, Vec<DocEdit<ProfileProgram>>, RecipeNodeId) {
    let mut r = Rec::new();
    let plate_loop =
        LoopProgram::polygon([(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)]).unwrap();
    let plate_p = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![plate_loop],
    }));
    let plate = r.insert(Node::Extrude {
        profile: plate_p,
        distance: len(0.8),
    });
    // v4: the corpus boss re-authored as circle_split (corpus ruling
    // (a)); this probe mirrors it so the closed-form volume row keeps
    // metering the SAME document shape.
    let boss_loop = LoopProgram::circle_split(1.2, 1.7, 0.35, 3, 0.0).unwrap();
    let boss_p = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.3))),
        loops: vec![boss_loop],
    }));
    let boss = r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(1.0),
    });
    let union = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: plate,
        b: boss,
        declare: None,
    });
    (r.doc, r.edits, union)
}

fn union_body<T: geom_core::Decide>(
    ev: &editor_core::Evaluation<T>,
    id: RecipeNodeId,
) -> &topo::Body<T> {
    match &ev.value(id).expect("union evaluated").payload {
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body,
        ValuePayload::Body(b) => b,
        other => panic!("expected a body, got {}", other.kind_name()),
    }
}

#[test]
fn the_boss_union_document_row_f64_volume_and_roundtrip() {
    let (doc, _edits, union) = boss_union_doc();
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default(), Tol::witness());
    let body = union_body(&ev, union);
    let vol = topo::mass_properties(body, Tol::witness()).unwrap().volume;
    let expect = 7.2 + std::f64::consts::PI * 0.35 * 0.35 * 0.5;
    assert!(
        (vol - expect).abs() < 1e-9,
        "doc-lane boss union volume {vol} vs {expect}"
    );
    // Save/load replay identity, f64 lane.
    let text = save(&doc, &[], Tol::witness()).expect("save");
    let loaded = load(&text, Tol::witness()).expect("load");
    assert!(loaded.doc.bit_eq(&doc), "round-trip bit identity");
    let ev2 = evaluate::<f64>(
        &loaded.doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert_eq!(
        format!("{:?}", ev.nodes),
        format!("{:?}", ev2.nodes),
        "f64 replay fingerprint drifted across save/load"
    );
}

#[cfg(feature = "interval")]
#[test]
fn the_boss_union_document_row_interval_bit_replay() {
    // Deviation 9's unauthored row: the first curved boolean under
    // the certified interval scalar, twice, bit-identical — and green.
    use geom_core::Interval;
    let (doc, _edits, union) = boss_union_doc();
    let fp = || {
        let ev = evaluate::<Interval>(&doc, None, &CancelToken::new(), &EvalOptions::default());
        let bad: Vec<String> = ev
            .nodes
            .iter()
            .filter_map(|(id, r)| match r {
                editor_core::NodeResult::Ok(_) => None,
                other => Some(format!("{id:?}: {other:?}")),
            })
            .collect();
        assert!(
            bad.is_empty(),
            "the Interval lane of the curved boolean is not green:\n{}",
            bad.join("\n")
        );
        // The union body's volume enclosure must bracket the truth.
        let body = union_body(&ev, union);
        let vol = topo::mass_properties(body).unwrap().volume;
        let expect = 7.2 + std::f64::consts::PI * 0.35 * 0.35 * 0.5;
        assert!(
            geom_core::Bounds::lo(vol) <= expect && expect <= geom_core::Bounds::hi(vol),
            "Interval volume enclosure {vol:?} does not bracket {expect}"
        );
        format!("{:?}|{:?}", ev.order, ev.nodes)
    };
    let a = fp();
    let b = fp();
    assert_eq!(a, b, "Interval evaluation is not bit-replayable");
}

#[test]
fn tangent_intersection_edges_survive_save_load_at_rest() {
    // The filleted extrude mints TangentIntersection at rest (the
    // PR's own upgrade pass). Save the DOCUMENT, load, re-evaluate:
    // the loaded body must carry the same count of intrinsic tangent
    // edges — the round trip may not silently downgrade them.
    let mut r = Rec::new();
    // v4: the declared-tangent joints [2, 3] author structurally —
    // `.tangent()` before the arc and before the leg out of it.
    let lpt = |x: f64, y: f64| {
        [
            editor_core::Expr::literal(x, editor_core::Dimension::Length).unwrap(),
            editor_core::Expr::literal(y, editor_core::Dimension::Length).unwrap(),
        ]
    };
    let lp = LoopProgram::Chain(vec![
        ProgramStep::At(lpt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(1.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(1.0, 0.75))),
        ProgramStep::Tangent,
        ProgramStep::TangentArcTo(ProgramTarget::Point(lpt(0.75, 1.0))),
        ProgramStep::Tangent,
        // Declared-tangent straight leg: rides the inherited
        // direction, authored as its LENGTH (0.75 → lands at (0,1)).
        ProgramStep::Line(
            editor_core::Expr::literal(0.75, editor_core::Dimension::Length).unwrap(),
        ),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![lp],
    }));
    let ex = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    });
    let count = |doc: &ProfileDoc| -> usize {
        let ev = evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default(), Tol::witness());
        let body = match &ev.value(ex).expect("extrude evaluated").payload {
            ValuePayload::Body(b) => b.clone(),
            other => panic!("expected a body, got {}", other.kind_name()),
        };
        body.edges()
            .filter(|(_, e)| {
                matches!(
                    body.get_curve_geom(e.curve)
                        .and_then(|g| g.certified())
                        .map(topo::EdgeCurve::description),
                    Some(topo::EdgeGeometry::TangentIntersection { .. })
                )
            })
            .count()
    };
    let n0 = count(&r.doc);
    assert_eq!(n0, 2, "the fillet mints its two tangent struts at rest");
    let text = save(&r.doc, &[], Tol::witness()).expect("save");
    let loaded = load(&text, Tol::witness()).expect("load");
    assert_eq!(
        count(&loaded.doc),
        n0,
        "TangentIntersection edges lost across save/load"
    );
}
