//! Corpus document **kitchen_sink** — every v1 node kind and every
//! `DocEdit` kind in ONE document (M4 PR 8a spec D1's "touching
//! everything at once"). It grew out of the M4 PR 6 round-trip
//! fixture, which now consumes it from here so the persistence rows
//! and the corpus rows can never drift apart.
//!
//! Node kinds: Datum (Point/Axis/Plane), Profile (plain, arc-bearing
//! by fillet construction, hand-declared tangent), Extrude, Revolve,
//! Split, Boolean (Union, with a Declare operand), Transform, Pattern
//! (Linear and Circular), Declare.
//!
//! Edit kinds: `InsertNode`, `DeleteNode`, `SetParam`,
//! `SetStructuralParam`, `SetExpression`, `SetDocParam`,
//! `SetDocParamValue`, `Rebind`,
//! `ReWitness`, `ReWitnessBulk`, `SetAppearance`, `ClearAppearance`,
//! `SetTolerance`, `SetAppearanceMeta`, `ClearAppearanceMeta`.
//!
//! No mass pin: the document is a vocabulary exhibit, not a solid —
//! its heads are several disjoint bodies. Correctness is pinned by
//! "every node evaluates green at every ε row and under Interval",
//! plus the round-trip rows.
//!
//! ε note: the `SetTolerance` edit re-records the AMBIENT ε (the
//! value `ProfileDoc::empty_derived("sink")` already carries). Pinning any other
//! value would make the document refuse to load in every CI ε row but
//! one — the golden fixture in `m4_pr6_golden.rs` is where a pinned ε
//! belongs, deliberately.

use std::collections::BTreeMap;

use editor_core::{
    Attr, AttrKind, Axis3, BooleanOp, BranchCertification, Datum, Dimension, DocEdit, DocParam,
    DocParamValue, EntityKind, Expr, ExprPath, MetaValue, Node, ParamName, PatternKind, Rgba8,
    RoleSeg, SlotId, StableName, WitnessDatum,
};

use super::super::fixture::{ang, declare_x_offset_flush, len, scl};
use super::{CorpusDoc, Recorder};

/// The kitchen-sink corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    // Re-record the ambient ε (a structural edit; see module docs).
    let ambient = r.doc.epsilon();
    r.push(DocEdit::SetTolerance { eps: ambient });
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("h"),
        value: DocParam::continuous(Dimension::Length, 1.0),
    });
    // The VALUE door, on the parameter the declaration above just
    // made: it carries the declaration forward, so `h` keeps its
    // dimension (and would keep a distribution) while the number
    // moves. The document's state after this pair is the same
    // document a single declaration at 1.25 would have produced.
    r.push(DocEdit::SetDocParamValue {
        name: ParamName::new("h"),
        value: DocParamValue::Continuous(1.25),
    });
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("n"),
        value: DocParam::Count { value: 3 },
    });

    // Datums: an inert point (deleted below — the DeleteNode arm),
    // and an axis for the revolve and the circular pattern.
    let inert = r.insert(Node::Datum(Datum::Point {
        position: [len(0.5), len(-0.25), len(0.0)],
    }));
    let axis = r.insert(Node::Datum(Datum::Axis {
        origin: [len(-2.0), len(0.0), len(0.0)],
        direction: [scl(0.0), scl(0.0), scl(1.0)],
    }));

    // A profile extruded by h · sin(π/2) — param + trig coverage, and
    // the SetExpression target (the `sin` subtree is child 1).
    let profile = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let h = Expr::param(ParamName::new("h"), Dimension::Length);
    let dist =
        Expr::mul(h, Expr::sin(ang(std::f64::consts::FRAC_PI_2)).expect("sin")).expect("mul");
    let block_a = r.insert(Node::Extrude {
        profile,
        distance: dist,
    });

    // A flush neighbour + Declare + the consuming union (F5). The
    // offset is HALF the width, so the blocks OVERLAP along x while
    // their y-walls and both caps stay flush — the sliding-overlap
    // shape `declare_x_offset_flush` declares. (A pure face-to-face
    // touch at x = 1 would be a REST contact, which the join stage
    // still refuses — the tracked envelope entry, not corpus fodder.)
    let profile_b = r.profile(
        [0.5, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let block_b = r.insert(Node::Extrude {
        profile: profile_b,
        distance: len(1.25),
    });
    let (with_declare, declare) = declare_x_offset_flush(r.doc.clone(), block_a, block_b);
    let declare_node = with_declare
        .node(declare)
        .expect("declare inserted")
        .clone();
    r.insert(declare_node);
    let union = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: block_a,
        b: block_b,
        declare: Some(declare),
    });

    // Split the union with a plane tool.
    let tool = r.insert(Node::Datum(Datum::Plane {
        origin: [len(0.0), len(0.0), len(0.625)],
        normal: [scl(0.0), scl(0.0), scl(1.0)],
    }));
    r.insert(Node::Split {
        target: union,
        tool,
    });

    // A transformed copy, patterned linearly; a lone block patterned
    // circularly about the shared axis.
    let moved = r.insert(Node::Transform {
        input: union,
        translation: [len(0.0), len(4.0), len(0.0)],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: ang(std::f64::consts::FRAC_PI_3),
    });
    let linear = r.insert(Node::Pattern {
        input: moved,
        count: Expr::count(2),
        kind: PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(3.0),
        },
    });
    let lone = r.insert(Node::Extrude {
        profile,
        distance: len(0.5),
    });
    r.insert(Node::Pattern {
        input: lone,
        count: Expr::count(2),
        kind: PatternKind::Circular {
            axis,
            step: ang(std::f64::consts::PI),
        },
    });

    // A revolve off the shared axis.
    let rev_profile = r.profile(
        [-2.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        vec![vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]],
    );
    r.insert(Node::Revolve {
        profile: rev_profile,
        axis,
        angle: ang(std::f64::consts::FRAC_PI_2),
    });

    // ---- the non-insert edit vocabulary ----
    // Structural: drive the linear pattern's count from `n`.
    r.push(DocEdit::SetStructuralParam {
        node: linear,
        slot: SlotId::Count,
        expr: Expr::param(ParamName::new("n"), Dimension::Count),
    });
    // Subtree surgery: replace `sin(π/2)` with the Scalar literal 1
    // (same dimension, same value — a pure representation edit).
    r.push(DocEdit::SetExpression {
        path: ExprPath {
            node: block_a,
            slot: SlotId::Distance,
            path: vec![1],
        },
        expr: scl(1.0),
    });
    // Continuous slot edit.
    r.push(DocEdit::SetParam {
        node: lone,
        slot: SlotId::Distance,
        expr: len(0.375),
    });
    // The inert datum has no dependents: delete it (ids never reused).
    r.push(DocEdit::DeleteNode { id: inert });
    // Witnesses: one explicit, one certified bulk adoption.
    r.push(DocEdit::ReWitness {
        node: profile,
        witness: WitnessDatum {
            schema: 1,
            bytes: vec![0x00, 0x01, 0x7f, 0x80, 0xfe, 0xff],
        },
    });
    r.push(DocEdit::ReWitnessBulk {
        entries: vec![
            (
                profile_b,
                WitnessDatum {
                    schema: 1,
                    bytes: vec![0x10, 0x20],
                },
            ),
            (
                rev_profile,
                WitnessDatum {
                    schema: 1,
                    bytes: vec![0x30],
                },
            ),
        ],
        certification: BranchCertification {
            schema: 1,
            bytes: vec![0xc0, 0xde],
        },
    });
    // Appearance + D7 metadata on the union's output body.
    let body = StableName {
        kind: EntityKind::Body,
        node: union,
        path: vec![RoleSeg::OutputBody],
    };
    r.push(DocEdit::SetAppearance {
        name: body.clone(),
        attr: Attr::Color(Rgba8::opaque(200, 40, 40)),
    });
    r.push(DocEdit::SetAppearance {
        name: body.clone(),
        attr: Attr::Label("kitchen sink".into()),
    });
    r.push(DocEdit::SetAppearanceMeta {
        name: body.clone(),
        key: "tool.example/annotation".into(),
        value: meta_tree(),
    });
    r.push(DocEdit::SetAppearanceMeta {
        name: body.clone(),
        key: "tool.example/scratch".into(),
        value: MetaValue::Map(BTreeMap::from([("v".into(), MetaValue::Int(1))])),
    });
    r.push(DocEdit::ClearAppearanceMeta {
        name: body.clone(),
        key: "tool.example/scratch".into(),
    });
    r.push(DocEdit::ClearAppearance {
        name: body,
        kind: AttrKind::Label,
    });
    // The explicit name repair (N5): an attribute attached to one
    // body name is moved, one-shot, onto another live name.
    let a_body = StableName {
        kind: EntityKind::Body,
        node: block_a,
        path: vec![RoleSeg::OutputBody],
    };
    let b_body = StableName {
        kind: EntityKind::Body,
        node: block_b,
        path: vec![RoleSeg::OutputBody],
    };
    r.push(DocEdit::SetAppearance {
        name: a_body.clone(),
        attr: Attr::Color(Rgba8::opaque(20, 90, 160)),
    });
    r.push(DocEdit::Rebind {
        from: a_body,
        to: b_body,
    });

    CorpusDoc {
        name: "kitchen_sink",
        about: "every v1 node kind and every DocEdit kind in one document",
        edits: r.edits,
        doc: r.doc,
        result: None,
        pin: None,
        bump: DocEdit::SetParam {
            node: moved,
            slot: SlotId::Translation(Axis3::Y),
            expr: len(5.0),
        },
        bump_root: moved,
    }
}

/// A D7 metadata value exercising the whole `MetaValue` vocabulary
/// (with the required `"v"` version field); floats included, and
/// `-0.0` is DATA.
pub fn meta_tree() -> MetaValue {
    let mut m = BTreeMap::new();
    m.insert("v".into(), MetaValue::Int(1));
    m.insert("flag".into(), MetaValue::Bool(true));
    m.insert("nothing".into(), MetaValue::Null);
    m.insert("neg_zero".into(), MetaValue::Float(-0.0));
    m.insert("subnormal".into(), MetaValue::Float(f64::from_bits(1)));
    m.insert("text".into(), MetaValue::Str("π ≈ 3.14159".into()));
    m.insert("blob".into(), MetaValue::Bytes(vec![0xde, 0xad, 0x00]));
    m.insert(
        "list".into(),
        MetaValue::List(vec![MetaValue::Int(-7), MetaValue::Float(0.1)]),
    );
    MetaValue::Map(m)
}
