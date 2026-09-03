//! Corpus document **kiss_carry** — the tier-3′ carry as a recipe: a
//! corner-kiss assembly union whose surviving v-v record is RE-ENTERED
//! by name through a `Declare` into the next boolean, so a boolean
//! value with NON-EMPTY surviving contacts exists in the corpus.
//!
//! That non-emptiness is the document's reason to exist: every other
//! corpus boolean's `ContactRecords` come out empty (declared REST
//! unions consume their records into seam structure; subtracts drop
//! the B side), so before this document no corpus row could tell a
//! lowering that carries the contacts from one that drops them. The
//! two boolean values here carry one v-v record each — the first
//! DISCOVERED by the op at the kiss, the second CARRIED through the
//! `Declare`'s same-operand vertex pair (`resolve_declarations`' third
//! arm, which no other corpus document reaches).
//!
//! Vocabulary: Profile, Extrude, Declare (carried v-v), Boolean
//! (Union), `InsertNode`, `SetParam`.
//!
//! Geometry (dyadic): block a `[0,1]³`; block b `[1,2]² × [1,2]`,
//! kissing a at the single point `(1,1,1)`; mover c
//! `[1.5,2.5]² × [1.5,2.5]`, crossing b transversally (no coincident
//! planes anywhere — the only contact in the document is the kiss).
//!
//! Exact oracles, derived here:
//!
//! - volume = `1 + 1 + 1 − 0.5³` (the b∩c corner cube) = **2.875**
//! - area: a contributes `6` (a point contact removes no area); b∪c =
//!   `6 + 6 − 6·(0.5·0.5)` (each cube loses three `0.5×0.5` face
//!   patches interior to the other) `= 10.5` ⇒ **16.5**
//!
//! D2 bump: the mover's `Distance` (mid-DAG — its cone is that
//! extrude plus the second union; the kiss chain is reused).

use editor_core::{
    BooleanOp, CapEnd, Dimension, DocEdit, EntityKind, Expr, Node, ProfileVertexRef, RecipeNodeId,
    RoleSeg, SlotId, StableName,
};

use crate::fixture::len;

use super::{CorpusDoc, MassPin, Recorder};

/// A cap-vertex name at `node`.
fn cap_vertex(node: RecipeNodeId, end: CapEnd, vertex: u32) -> StableName {
    StableName {
        kind: EntityKind::Vertex,
        node,
        path: vec![RoleSeg::CapVertex(
            end,
            ProfileVertexRef {
                loop_index: 0,
                vertex,
            },
        )],
    }
}

/// The kiss-carry corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    // Block a: [0,1]³. Profile (0,0)(1,0)(1,1)(0,1) → vertex 2 is
    // (1,1), the kiss corner on the TOP cap.
    let a_p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let a = r.insert(Node::Extrude {
        profile: a_p,
        distance: len(1.0),
    });
    // Block b: [1,2]² × [1,2]. Profile (1,1)(2,1)(2,2)(1,2) → vertex 0
    // is (1,1), the kiss corner on the BOTTOM cap.
    let b_p = r.profile(
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0)]],
    );
    let b = r.insert(Node::Extrude {
        profile: b_p,
        distance: len(1.0),
    });
    // The kiss union: nothing is declared — the v-v kiss at (1,1,1) is
    // DISCOVERED by the op and recorded in the result's contacts.
    let u1 = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a,
        b,
        declare: None,
    });

    // The mover: [1.5,2.5]² × [1.5,2.5], a transversal crossing of b.
    let c_p = r.profile(
        [0.0, 0.0, 1.5],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(1.5, 1.5), (2.5, 1.5), (2.5, 2.5), (1.5, 2.5)]],
    );
    let c = r.insert(Node::Extrude {
        profile: c_p,
        distance: len(1.0),
    });

    // The carry: the surviving kiss is re-entered BY NAME — both names
    // resolve in u1's table (the A operand of the union below), so
    // this is `resolve_declarations`' same-operand carried v-v arm,
    // and the record survives into the second union's contacts.
    let kiss_a = StableName {
        kind: EntityKind::Vertex,
        node: u1,
        path: vec![RoleSeg::FromA(Box::new(cap_vertex(a, CapEnd::Top, 2)))],
    };
    let kiss_b = StableName {
        kind: EntityKind::Vertex,
        node: u1,
        path: vec![RoleSeg::FromB(Box::new(cap_vertex(b, CapEnd::Bottom, 0)))],
    };
    let decl = r.insert(Node::declare_rest(vec![(kiss_a, kiss_b)]));
    let u2 = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: u1,
        b: c,
        declare: Some(decl),
    });

    CorpusDoc {
        name: "kiss_carry",
        about: "corner-kiss assembly; the surviving v-v record re-entered by Declare",
        edits: r.edits,
        doc: r.doc,
        result: Some(u2),
        pin: Some(MassPin {
            volume: 2.875,
            area: Some(16.5),
        }),
        bump: DocEdit::SetParam {
            node: c,
            slot: SlotId::Distance,
            expr: Expr::literal(1.25, Dimension::Length).expect("dyadic length literal"),
        },
        bump_root: c,
    }
}
