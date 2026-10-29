//! Corpus document **kitchen_sink** — every v1 node kind and every
//! REQUIRED `DocEdit` kind in ONE document (M4 PR 8a spec D1's
//! "touching everything at once"); "required" is `EDIT_KINDS`, which
//! is a subset of the arms and says at its own definition which stand
//! outside it and why. It grew out of the M4 PR 6 round-trip
//! fixture, which now consumes it from here so the persistence rows
//! and the corpus rows can never drift apart.
//!
//! Node kinds: Profile (plain, arc-bearing by fillet construction,
//! hand-declared tangent), Extrude, Revolve, Split, Boolean (Union),
//! Place (a copy against a body, and in the world, and two families:
//! a row and a ring, D10 `[repetition]`).
//!
//! Edit kinds — the `EDIT_KINDS` names, which is every arm the corpus
//! is required to cover and NOT every arm `DocEdit` has (that list's
//! own doc says which stand outside it and what guards a new
//! one):
//! `InsertNode`, `DeleteNode`, `SetParam`,
//! `SetStructuralParam`, `SetExpression`, `DeclareVar`, `DefineVar`,
//! `SetVarValue`, `SetVarUnit`, `SetVarDistribution`, `Rebind`,
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
    Attr, AttrKind, BranchCertification, Coord, Dimension, Distribution, DocEdit, EntityKind,
    ExprPath, Formula, Frame, FreeValue, FreeVar, MetaValue, Node, Rgba8, RoleSeg, SlotId,
    StableName, TargetAxis, UnitSym, VarName, WitnessDatum,
};
use geom_core::Tol;
use geom_core::predicate::Band;

use crate::fixture::{ang, len, plane_on, revolve_about, scalar_of, scl, value_slot};

use super::{CorpusDoc, PlacePart, Recorder};

/// The kitchen-sink corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    // Re-record the ambient ε (a structural edit; see module docs).
    let ambient = r.doc.epsilon();
    r.push(DocEdit::SetTolerance { eps: ambient });
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("h"),
        def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, 1.0)),
    });
    // The VALUE door, on the variable the declare above just minted:
    // it carries the definition forward, so `h` keeps its kind (and
    // would keep a distribution) while the number moves.
    r.push(DocEdit::SetVarValue {
        var: VarName::from_static("h").into(),
        value: FreeValue::Continuous(1.25),
    });
    // The NOTATION door, the value door's mirror over the other field
    // of the same declaration: `h` is now written in millimetres and
    // keeps its dimension, its exact value and any annotation. The
    // edit is invisible to `bit_eq` by ruling (`display_unit` is
    // presentation metadata), so the round-trip rows read it as the
    // same document and the FILE is where it has to survive.
    r.push(DocEdit::SetVarUnit {
        var: VarName::from_static("h").into(),
        unit: UnitSym::from_def(&quantity::MM.def()),
    });
    // The ANNOTATION door, the third field of the same declaration:
    // `h` acquires an E1/E2 tolerance and keeps the millimetres the
    // edit above wrote. Through a whole definition this pair reverts
    // the notation, which is the trap the door removes; here the FILE
    // carries both, so the round-trip rows read them back together.
    r.push(DocEdit::SetVarDistribution {
        var: VarName::from_static("h").into(),
        distribution: Some(Distribution::Band {
            lo: -0.0001,
            hi: 0.0001,
        }),
    });
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("n"),
        def: editor_core::VarDecl::Free(FreeVar::Count { value: 2 }),
    });
    // The DEFINITION door: `n` keeps its identity, its name and its
    // kind while its definition is replaced whole.
    r.push(DocEdit::DefineVar {
        var: VarName::from_static("n").into(),
        def: editor_core::VarDecl::Free(FreeVar::Count { value: 3 }),
        fresh: Vec::new(),
    });

    // An inert profile, deleted below (the DeleteNode arm).
    let inert = r.insert(Node::Profile(crate::fixture::desc(vec![
        crate::fixture::square(0.5, -0.25, 0.125),
    ])));

    // A profile extruded by h · sin(π/2) — param + trig coverage, and
    // the SetExpression target (the `sin` subtree is child 1).
    let profile = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let h = Formula::named(VarName::from_static("h"), Dimension::Length);
    let dist = Formula::mul(
        h,
        Formula::sin(ang(std::f64::consts::FRAC_PI_2)).expect("sin"),
    )
    .expect("mul");
    let block_a = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: dist,
    });

    // A flush neighbour, placed against the first block, and their
    // union (F5). The offset is HALF the width, so the blocks OVERLAP
    // along x while their y-walls and both caps stay flush — the
    // sliding-overlap shape.
    let profile_b = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let block_b = r.insert(Node::Extrude {
        profile: profile_b.into(),
        distance: len(1.25),
    });
    let placed_b = r.against(block_a, block_b, &Frame::translation([0.5, 0.0, 0.0]));
    let union = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![block_a.into(), placed_b.into()]),
    });

    // Split the union with a plane read off its own geometry.
    let tool = plane_on(&r.doc, union, [0.0, 0.0, 0.625], [0.0, 0.0, 1.0]);
    let split = r.insert(Node::Split {
        target: union.into(),
        tool,
    });

    // A moved copy of the union in its own space, its row of `n`; a
    // lone block's ring of two about the vertical line through
    // (-2, 0, 0).
    let band = Band::linear(Tol::witness()).expect("a band");
    let turned = Frame::rotate_then_translate(
        [0.0, 0.0, 1.0],
        std::f64::consts::FRAC_PI_3,
        [0.0, 4.0, 0.0],
        band,
    )
    .expect("a turn about z");
    let moved = r.against(union, union, &turned);
    let j = r.index(
        "j",
        Formula::named(VarName::from_static("n"), Dimension::Count),
    );
    let linear = r.copies(
        moved,
        vec![(
            Coord::Slide(TargetAxis::X),
            Formula::mul(scalar_of(j), len(3.0)).expect("a length"),
        )],
    );
    let lone = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: len(0.5),
    });
    // The ring turns about the vertical line through (-2, 0, 0): a half
    // turn about the copy's origin, then the slide that carries that
    // line back onto itself, `(-4, 0, 0)` per member.
    let c = r.index("c", Formula::count(2));
    let circular = r.copies(
        lone,
        vec![
            (
                Coord::Spin(Some(TargetAxis::Z)),
                Formula::mul(scalar_of(c), Formula::turn_over(2).expect("a half turn"))
                    .expect("an angle"),
            ),
            (
                Coord::Slide(TargetAxis::X),
                Formula::mul(scalar_of(c), len(-4.0)).expect("a length"),
            ),
        ],
    );
    // One member of the row, read by its index (a STRUCTURAL slot).
    let first = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![editor_core::BodyRead::indexed(
            linear,
            vec![Formula::count(1)],
        )]),
    });

    // A revolve about an axis in its own sketch, its own +y, placed in
    // the world at (-2, 0, 0) with its own y along the world's +z: the
    // line the pattern turns about.
    let rev_profile = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]],
    );
    let revolve = r.insert(revolve_about(
        rev_profile,
        (0.0, 0.0),
        crate::fixture::along_y(),
        ang(std::f64::consts::FRAC_PI_2),
    ));

    // ---- the non-insert edit vocabulary ----
    // Structural: the member the read takes.
    r.push(DocEdit::SetStructuralParam {
        node: first,
        slot: SlotId::Index {
            seat: editor_core::OperandSlot::Member(0),
            k: 0,
        },
        expr: Formula::count(0),
        fresh: Vec::new(),
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
        value: len(0.375).into(),
        fresh: Vec::new(),
    });
    // The inert profile has no dependents: delete it (ids never
    // reused).
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
    // A node label on the union (document data beside the node).
    r.push(DocEdit::SetLabel {
        node: union,
        label: Some(editor_core::Label::new("kitchen sink").unwrap()),
    });
    // Appearance + D7 metadata on the union's output body.
    let body = StableName {
        kind: EntityKind::Body,
        node: union,
        path: vec![RoleSeg::OutputBody],
    };
    r.push(DocEdit::SetAppearance {
        body: crate::fixture::painted_in(&r.doc, &body),
        name: body.clone(),
        attr: Attr::Color(Rgba8::opaque(200, 40, 40)),
    });
    r.push(DocEdit::SetAppearance {
        body: crate::fixture::painted_in(&r.doc, &body),
        name: body.clone(),
        attr: Attr::Label(editor_core::Label::new("kitchen sink").unwrap()),
    });
    r.push(DocEdit::SetAppearanceMeta {
        body: crate::fixture::painted_in(&r.doc, &body),
        name: body.clone(),
        key: "tool.example/annotation".into(),
        value: meta_tree(),
    });
    r.push(DocEdit::SetAppearanceMeta {
        body: crate::fixture::painted_in(&r.doc, &body),
        name: body.clone(),
        key: "tool.example/scratch".into(),
        value: MetaValue::map(BTreeMap::from([("v".into(), MetaValue::Int(1.into()))]))
            .expect("a shallow value"),
    });
    r.push(DocEdit::ClearAppearanceMeta {
        body: crate::fixture::painted_in(&r.doc, &body),
        name: body.clone(),
        key: "tool.example/scratch".into(),
    });
    r.push(DocEdit::ClearAppearance {
        body: crate::fixture::painted_in(&r.doc, &body),
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
        body: crate::fixture::painted_in(&r.doc, &a_body),
        name: a_body.clone(),
        attr: Attr::Color(Rgba8::opaque(20, 90, 160)),
    });
    r.push(DocEdit::Rebind {
        body: None,
        from: a_body,
        to: b_body,
    });

    // The world, in the order the document's product held it.
    // The lone block's copies hang below the world's xy, their tops on
    // it: each slid down the block's height (its `distance`, set to
    // 0.375 above), clear of the union's blocks, which stand on it.
    r.in_world(circular, &Frame::translation([0.0, 0.0, -0.375]));
    super::place_halves(&mut r, split);
    r.place_part(linear);
    r.in_world_by(revolve, crate::fixture::upright_at([-2.0, 0.0, 0.0]));

    let bump = value_slot(&r.doc, moved, Coord::Slide(TargetAxis::Y));
    CorpusDoc {
        name: "kitchen_sink",
        about: "every v1 node kind and every DocEdit kind in one document",
        edits: r.edits,
        doc: r.doc,
        result: None,
        pin: None,
        bump: DocEdit::SetParam {
            node: moved,
            slot: bump,
            value: len(5.0).into(),
            fresh: Vec::new(),
        },
        bump_root: moved,
    }
}

/// A D7 metadata value exercising the whole `MetaValue` vocabulary
/// (with the required `"v"` version field); floats included, and
/// `-0.0` is DATA.
pub fn meta_tree() -> MetaValue {
    let mut m = BTreeMap::new();
    m.insert("v".into(), MetaValue::Int(1.into()));
    m.insert("flag".into(), MetaValue::Bool(true));
    m.insert("nothing".into(), MetaValue::Null);
    m.insert("neg_zero".into(), MetaValue::Float(-0.0));
    m.insert("subnormal".into(), MetaValue::Float(f64::from_bits(1)));
    m.insert("text".into(), MetaValue::Str("π ≈ 3.14159".into()));
    m.insert("blob".into(), MetaValue::Bytes(vec![0xde, 0xad, 0x00]));
    m.insert(
        "list".into(),
        MetaValue::list(vec![MetaValue::Int((-7).into()), MetaValue::Float(0.1)])
            .expect("a shallow value"),
    );
    MetaValue::map(m).expect("a shallow value")
}
