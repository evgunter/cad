//! M4 PR 3 review-rework fixtures (adversarial review, binding
//! rulings R1/R2/R7): the reviewer's falsifying reproductions,
//! committed. Excluded here by ruling: the boolean-of-boolean shape
//! (pre-existing kernel panic, issue #86, R13) and the edge-flush
//! union (kernel envelope refusal `UnpairedLooseEnds`, not naming).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::{
    BooleanOp, CancelToken, Datum, EvalOptions, Evaluation, Node, NodeErrorClass, ProfileDoc,
    RecipeNodeId, RoleSeg, evaluate,
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
            profile: p.into(),
            distance: len(dz),
            side: ExtrudeSide::Along,
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
            a: a.into(),
            b: b.into(),
            declare: Vec::new(),
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
            a: a.into(),
            b: b.into(),
            declare: Vec::new(),
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
            a: a.into(),
            b: b.into(),
            declare: Vec::new(),
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
            a: a.into(),
            b: b.into(),
            declare: Vec::new(),
        },
    );
    let ev = run(&doc);
    // Two stub bodies: a typed BOOLEAN refusal (body cardinality) is
    // acceptable; failing in NAMING is not.
    if ev.value(s).is_none() {
        let err = format!("{:?}", ev.nodes.get(&s));
        assert_ne!(
            ev.node_error(s).map(|e| e.kind.class()),
            Some(NodeErrorClass::Naming),
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
            target: d.into(),
            tool: plane.into(),
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

/// A plane through the 315° prism's reflex top corner `(0, 0, 1)`,
/// tilted back over the prism so the corner's three edges read Above
/// and its reflex bisector Below: the splitter's whole-orbit strut,
/// whose copy is the BELOW end. Above, the corner keeps its three edges
/// and takes an `OnToolVertex` naming the operand corner. Below, the
/// copy holds only the two section chords the plane cuts across the top
/// cap on either side of the corner, one line between the section face
/// and the cap: the split ends with the join (`docs/DESIGN.md`, maximal
/// edges), so the copy is gone and the chord is one edge, named for the
/// cap it crosses with no ends to tell pieces apart.
#[test]
fn split_through_a_reflex_corner_names_its_copy_where_the_corner_stands() {
    let doc = ProfileDoc::empty_derived("m4_pr3_names_rework", Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.0, 0.0),
            (2.0, 2.0),
            (-2.0, 2.0),
            (-2.0, -2.0),
            (2.0, -2.0),
            (2.0, 0.0),
        ]],
    );
    let (doc, prism) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.0), len(1.0)],
            normal: [scl(1.0), scl(0.0), scl(-1.0)],
        }),
    );
    let (doc, sp) = insert(
        doc,
        Node::Split {
            target: prism.into(),
            tool: plane.into(),
        },
    );
    let ev = run(&doc);
    let v = ev
        .value(sp)
        .unwrap_or_else(|| panic!("reflex-corner split failed: {:?}", ev.nodes.get(&sp)));
    let copies: Vec<_> = v
        .name_table
        .iter()
        .filter_map(|(n, _)| match n.path.first() {
            Some(RoleSeg::OnToolVertex { side, of }) => Some((*side, of.clone())),
            _ => None,
        })
        .collect();
    let sides: Vec<_> = copies.iter().map(|c| c.0).collect();
    assert_eq!(
        sides,
        vec![editor_core::SplitHalf::Above],
        "the corner's copy Above, and none Below: {copies:?}"
    );
    let below_chords: Vec<_> = v
        .name_table
        .iter()
        .filter(|(n, _)| {
            matches!(
                n.path.as_slice(),
                [RoleSeg::SectionEdge {
                    side: editor_core::SplitHalf::Below,
                    ..
                }]
            )
        })
        .collect();
    assert!(
        !below_chords.is_empty(),
        "Below's section chords are named for the faces they cross, each one edge"
    );
}

// ---- R7: a pattern never conflates a split's halves under instance
// body indices. ----
//
// **R7 register, closed by reads (INTENT stage 2 unit B).** The
// deferral was a master with several output BODIES — body index is
// the instance index there. A split named alone is either of its two
// bodies, so the door refuses it as a pattern's operand
// (`AmbiguousOutput`), and a read of one port IS that half: the
// pattern is a pattern of one body, named as the pattern of
// `Part { SplitHalf }` is. A master whose single body holds several
// SOLIDS was always admitted (`names::emit::pattern_tests`).

#[test]
fn a_pattern_of_a_split_port_is_the_pattern_of_its_half() {
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
            target: d.into(),
            tool: plane.into(),
        },
    );
    let pattern = |input: editor_core::Operand| Node::Pattern {
        input,
        count: editor_core::Formula::count(2),
        kind: editor_core::PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(5.0),
        },
    };
    let refusal = crate::fixture::insert_refused(&doc, pattern(sp.into()));
    assert!(
        matches!(&refusal, editor_core::EditError::AmbiguousOutput { input, .. } if input.id() == sp),
        "a split named alone is either half: {refusal:?}"
    );
    let (doc, half) = insert(
        doc,
        Node::Part {
            of: sp.into(),
            select: editor_core::PartSelect::SplitHalf(editor_core::SplitHalf::Above),
        },
    );
    let (doc, by_port) = insert(
        doc,
        pattern(editor_core::Operand::Output { node: sp, port: 0 }),
    );
    let (doc, by_part) = insert(doc, pattern(half.into()));
    let ev = run(&doc);
    let (port, part) = (
        ev.value(by_port)
            .unwrap_or_else(|| panic!("{:?}", ev.nodes.get(&by_port))),
        ev.value(by_part).expect("the part spelling patterns"),
    );
    // Each pattern mints its own copies' names; read the port
    // spelling's as the part spelling's, and the two are one.
    let as_part = format!("{:?}", port.name_table)
        .replace(&format!("{:?}", by_port.0), &format!("{:?}", by_part.0));
    assert_eq!(
        as_part,
        format!("{:?}", part.name_table),
        "the port's copies are named as the half's"
    );
}

/// **An edge the split cut and joined back whole keeps its own name.**
/// A plane that touches a unit cylinder (seam at `+x`) without
/// separating it cuts each rim it touches at one point; the cut
/// separates nothing, so the split ends by joining each such rim back
/// (`docs/DESIGN.md`, maximal edges). The landed cylinder's every edge
/// is named as the operand's own, never as a fragment of itself.
#[test]
fn a_graze_split_lands_the_cylinder_under_its_own_edge_names() {
    // Tangent along the wall ruling at `-x`, which runs through a vertex
    // of each rim: the cut inserts nothing, so nothing is joined. The
    // row below is the one that cuts a rim inside an arc.
    let (landed, operand) = graze_split_edge_names([-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
    assert_eq!(
        landed, operand,
        "the landed cylinder's edges keep the operand's names"
    );
}

/// [`a_graze_split_lands_the_cylinder_under_its_own_edge_names`] at a
/// plane tilted to touch the TOP rim alone, at its point at angle 3π/4,
/// inside one rim arc: that arc cut and joined back, the other rim
/// never touched.
#[test]
fn a_split_touching_one_rim_at_a_point_keeps_that_rims_name() {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let (landed, operand) = graze_split_edge_names([-h, h, 1.0], [-0.5, 0.5, h]);
    assert_eq!(landed, operand, "the touched rim keeps the operand's name");
}

/// The edge names of a unit cylinder (height 1, axis `+z`) split by the
/// plane through `origin` with normal `normal`, and of the operand.
fn graze_split_edge_names(
    origin: [f64; 3],
    normal: [f64; 3],
) -> (Vec<Vec<RoleSeg>>, Vec<Vec<RoleSeg>>) {
    use editor_core::{LoopProgram, ProfileProgram};
    let doc = ProfileDoc::empty_derived("m4_pr3_names_rework", Tol::witness());
    let (doc, frame) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: frame.into(),
            loops: vec![LoopProgram::circle(0.0, 0.0, 1.0).expect("a finite circle")],
            ids: Vec::new(),
        }),
    );
    let (doc, cylinder) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: origin.map(len),
            normal: normal.map(scl),
        }),
    );
    let (doc, sp) = insert(
        doc,
        Node::Split {
            target: cylinder.into(),
            tool: plane.into(),
        },
    );
    let ev = run(&doc);
    let edges = |node| -> Vec<Vec<RoleSeg>> {
        let mut out: Vec<Vec<RoleSeg>> = ev
            .value(node)
            .unwrap_or_else(|| panic!("evaluates: {:?}", ev.nodes.get(&node)))
            .name_table
            .iter()
            .filter(|(n, _)| n.kind == editor_core::EntityKind::Edge)
            .map(|(n, _)| n.path.clone())
            .collect();
        out.sort();
        out
    };
    (edges(sp), edges(cylinder))
}
