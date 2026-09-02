//! M4 PR 3 review-rework fixtures (adversarial review, binding
//! rulings R1/R2/R7): the reviewer's falsifying reproductions,
//! committed. Excluded here by ruling: the boolean-of-boolean shape
//! (pre-existing kernel panic, issue #86, R13) and the edge-flush
//! union (kernel envelope refusal `UnpairedLooseEnds`, not naming).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    BooleanOp, CancelToken, Datum, EvalOptions, Evaluation, Node, ProfileDoc, RecipeNodeId,
    RoleSeg, evaluate,
};
use fixture::{insert, len, on_frame, scl};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

// ---- R1 (was BLOCKER): a B edge pierced through TWO A walls has
// its middle fragment die pre-graft; `chase_b` must fall through to
// the chord_kind rescue (A-lane parity), never hard-refuse a union
// the kernel completed. Both orientations pinned. ----

#[test]
fn union_cross_bar_names_totally() {
    let doc = ProfileDoc::empty_derived("m4_pr3_names_rework", Tol::witness());
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, b) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.25, 0.5);
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: None,
        },
    );
    let ev = run(&doc);
    let v = ev.value(u);
    assert!(
        v.is_some(),
        "cross-bar union failed: {:?}",
        ev.nodes.get(&u)
    );
    assert!(!v.unwrap().name_table.is_empty());
}

#[test]
fn union_cross_bar_swapped_names_totally() {
    let doc = ProfileDoc::empty_derived("m4_pr3_names_rework", Tol::witness());
    let (doc, a) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.25, 0.5);
    let (doc, b) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: None,
        },
    );
    let ev = run(&doc);
    assert!(
        ev.value(u).is_some(),
        "swapped cross-bar union failed: {:?}",
        ev.nodes.get(&u)
    );
}

#[test]
fn subtract_cross_bar_names_totally() {
    let doc = ProfileDoc::empty_derived("m4_pr3_names_rework", Tol::witness());
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, b) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.25, 0.5);
    let (doc, s) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b,
            declare: None,
        },
    );
    let ev = run(&doc);
    assert!(
        ev.value(s).is_some(),
        "cross-bar subtract failed: {:?}",
        ev.nodes.get(&s)
    );
}

#[test]
fn subtract_block_from_bar_never_fails_in_naming() {
    let doc = ProfileDoc::empty_derived("m4_pr3_names_rework", Tol::witness());
    let (doc, a) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.25, 0.5);
    let (doc, b) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, s) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b,
            declare: None,
        },
    );
    let ev = run(&doc);
    // Two stub bodies: a typed BOOLEAN refusal (body cardinality) is
    // acceptable; failing in NAMING is not.
    if ev.value(s).is_none() {
        let err = format!("{:?}", ev.nodes.get(&s));
        assert!(
            !err.contains("Naming"),
            "bar-minus-block failed IN NAMING: {err}"
        );
    }
}

// ---- R2: a tool plane THROUGH operand vertices (the diagonal
// split): on-plane operand vertices take the side-tagged
// `OnToolVertex` pass-through — both halves' coincident copies
// disambiguated by the recorded side verdict; naming total. ----

#[test]
fn split_through_operand_edges_names_totally() {
    let doc = ProfileDoc::empty_derived("m4_pr3_names_rework", Tol::witness());
    let (doc, d) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 2.0);
    // Plane x + z = 2: contains the cube edges (0,·,2) and (2,·,0).
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(2.0), len(0.0), len(0.0)],
            normal: [scl(1.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, sp) = insert(
        doc,
        Node::Split {
            target: d,
            tool: plane,
        },
    );
    let ev = run(&doc);
    let v = ev
        .value(sp)
        .unwrap_or_else(|| panic!("diagonal split failed: {:?}", ev.nodes.get(&sp)));
    // The four passed-through corner vertices: 2 per half.
    let mut above = 0;
    let mut below = 0;
    for (n, _) in v.name_table.iter() {
        if let Some(RoleSeg::OnToolVertex { side, .. }) = n.path.first() {
            match side {
                editor_core::SplitHalf::Above => above += 1,
                editor_core::SplitHalf::Below => below += 1,
            }
        }
    }
    assert_eq!(
        (above, below),
        (4, 4),
        "expected 4 on-plane vertex copies per half"
    );
}

// ---- R7: pattern of a multi-body master must refuse TYPED (never
// silently conflate the split halves under instance body indices). ----
//
// **R7 register, narrowed (ASM-2K, PR #381).** This row is the whole
// of what R7 still defers. The refusal is scoped to a master with
// several output BODIES — body index is the instance index there, so
// admitting one needs a ratified instance×body layout. A master whose
// single body holds several SOLIDS is NOT this case and is admitted:
// `Instance(i)` wraps every name uniformly, pinned by
// `names::emit::pattern_tests` (the rule is stated at `name_pattern`'s
// docs). The multi-solid reading of R7 retires there; this row stands.

#[test]
fn pattern_of_split_output_refuses_typed_never_misnames() {
    let doc = ProfileDoc::empty_derived("m4_pr3_names_rework", Tol::witness());
    let (doc, d) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 2.0);
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.0), len(1.0)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, sp) = insert(
        doc,
        Node::Split {
            target: d,
            tool: plane,
        },
    );
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: sp,
            count: editor_core::Expr::count(2),
            kind: editor_core::PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(5.0),
            },
        },
    );
    let ev = run(&doc);
    assert!(
        ev.value(pat).is_none(),
        "pattern of a split output must refuse (typed), got a value"
    );
    let err = format!("{:?}", ev.nodes.get(&pat));
    assert!(
        err.contains("WrongOperand") || err.contains("Naming"),
        "expected a typed refusal, got: {err}"
    );
}
