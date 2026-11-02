//! **The viewer's boundary between a pick and a mate side.**
//!
//! A pick reaches the mate tool as a selection whose `name` is a bare
//! `StableName` with every field public, so what a pick reads as is its
//! NAME's entity kind, whatever selection carried it, and a kind that
//! reads no such entity is refused typed (`KindDoesNotRead`), in every
//! build — the two rows below are that answer and its sentence.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::asm;
use pncad::document::{MateSide, VarKind};
use pncad::geom_core::Tol;
use pncad::select::EntityKind;
use viewer::matetool::{MateTool, MateToolError};

/// **An edge-named pick is read as an edge, and a plane refuses it.**
/// The tool's public door takes a selection by value, and nothing but
/// the picking door's rule keeps a face selection's `name` a face. A
/// face selection whose name names an edge is an edge to the tool —
/// so the seat's plane mate refuses it, naming the side, the kind and
/// the entity — where a kind reading edges would read it as one.
#[test]
fn an_edge_named_pick_is_refused_by_a_plane_mate() {
    let tol = Tol::witness();
    let bench = asm::bench("rv-matehead", tol);
    let session = asm::open_bench(&bench, tol);
    let (mut a, b) = asm::seat_picks(&session, &bench);
    assert_eq!(a.name.kind, EntityKind::Face);
    // The one field the picking door's rule lives in, and nothing but
    // that rule keeps it a face.
    a.name.kind = EntityKind::Edge;
    let mut tool = MateTool::new();
    tool.pick(session.doc(), a);
    tool.pick(session.doc(), b);
    let (doc, _) = session.landed_pair().expect("landed");
    match tool.proposal(doc, asm::seat_choice()) {
        Err(MateToolError::KindDoesNotRead { side, kind, entity }) => {
            assert_eq!(side, MateSide::On);
            assert_eq!(kind, VarKind::Plane);
            assert_eq!(entity, EntityKind::Edge);
        }
        other => panic!("an edge-named pick must refuse as the kind, got {other:?}"),
    }
}

/// **The refusal says the entity and what the kind reads**, the
/// latter read off the kernel's seats (`MateReading::of`): a plane
/// reads a face, an axis a face or an edge. The operand arm
/// (`NotACopyPick`) names a node and says nothing about what the
/// selection denoted, so it stays silent about the kind.
#[test]
fn the_kind_refusal_says_the_entity_and_what_the_kind_reads() {
    let said = |kind, entity| {
        MateToolError::KindDoesNotRead {
            side: MateSide::To,
            kind,
            entity,
        }
        .to_string()
    };
    assert_eq!(
        said(VarKind::Plane, EntityKind::Edge),
        "the to pick is an edge, and a plane mate reads a face"
    );
    assert_eq!(
        said(VarKind::Axis, EntityKind::Vertex),
        "the to pick is a vertex, and an axis mate reads a face or an edge"
    );

    let operand = MateToolError::NotACopyPick {
        node: pncad::document::SpokenNode::absent(pncad::document::RecipeNodeId::new(0, 0)),
    }
    .to_string();
    assert!(
        !operand.contains("face") && !operand.contains("edge"),
        "the operand arm must stay silent about the kind: {operand:?}"
    );
}

/// **A face name at an instance that its copy does not carry is the
/// placement's refusal where it evaluates.** The tool reads no pose
/// before committing, so a hand-built selection naming a face at the
/// instance that its part's product holds no row of proposes and
/// commits; the instance's copy holds only its part's rows, so the
/// mate's `on` does not evaluate and the post's row fails
/// `SideUnevaluated` on the `on` side — never a placement that
/// silently stands somewhere.
#[test]
fn a_face_name_at_an_instance_its_copy_does_not_carry_refuses_where_it_evaluates() {
    let tol = Tol::witness();
    let bench = asm::bench("rv-matehead-bare", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (mut a, b) = asm::seat_picks(&session, &bench);
    assert_ne!(
        a.name.node, a.node,
        "pick a is its part's name, read at its instance"
    );
    // The part's name with its role taken off: a bare face name the
    // copy does not carry.
    let mut bare = a.name.clone();
    bare.path = Vec::new();
    a.name = bare;
    let mut tool = MateTool::new();
    tool.pick(session.doc(), a);
    tool.pick(session.doc(), b);
    asm::unpin(&mut session, bench.post_b);
    let (doc, _) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(doc, asm::seat_choice())
        .expect("no pose is read before the commit");
    let outcome = session.perform(proposal.op_into_target_space());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let (_, eval) = session.landed_pair().expect("landed");
    match eval
        .result(bench.post_b)
        .and_then(|r| r.error())
        .map(|e| &e.kind)
    {
        Some(pncad::document::NodeErrorKind::Mate(fault))
            if matches!(
                fault.as_ref(),
                pncad::document::MateFault::SideUnevaluated {
                    side: MateSide::On,
                    ..
                }
            ) => {}
        other => {
            panic!("a face name its copy does not carry refuses where it evaluates, got {other:?}")
        }
    }
}
