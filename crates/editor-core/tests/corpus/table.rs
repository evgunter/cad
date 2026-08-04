//! Corpus document **corner_table** — the corner-aligned four-leg
//! table, the capability M4 PR 5's Declare opened (`#71`; the kernel
//! pins live in `topo/tests/corner_table.rs`). Here it is a RECIPE:
//! every flush contact is declared by NAME through a `Declare` node,
//! never inferred from values.
//!
//! Vocabulary: Profile, Extrude, Declare, Boolean (Union),
//! `InsertNode`, `SetParam`.
//!
//! Geometry (dyadic): top `[0,4] × [0,3] × [1,1.25]`; four legs
//! `0.5 × 0.5`, `z ∈ [0,1.125]`, each aligned to a corner so TWO of
//! its side faces are coplanar with the top's sides.
//!
//! Exact oracles, derived here:
//!
//! - volume = top `4·3·0.25 = 3` + per leg `0.5·0.5·1.125 = 0.28125`
//!   minus the slab overlap `0.5·0.5·0.125 = 0.03125` ⇒ `+0.25` each
//!   ⇒ **4.0**
//! - area: top alone `2·12 + 2·1 + 2·0.75 = 27.5`; a leg alone
//!   `2·0.25 + 4·(0.5·1.125) = 2.75`. Each leg's union removes
//!   `0.625` of INTERIOR boundary — the top's bottom face under the
//!   leg footprint `0.25`, the leg's top cap `0.25`, and the leg's two
//!   inner side strips inside the slab `2·(0.5·0.125) = 0.125` — plus
//!   `0.125` of DOUBLE-COUNTED coplanar overlap (each of the leg's two
//!   outer walls overlaps the top's own side face on
//!   `0.5 × 0.125 = 0.0625`; that region is one merged face, counted
//!   once). So `27.5 + 4·2.75 − 4·0.625 − 4·0.125` =
//!   `27.5 + 11 − 2.5 − 0.5` = **35.5**
//!
//! Naming note (N3): a DECLARED flush pair GLUES, so the union's face
//! carries a `Merged([FromA(top wall), FromB(leg wall)])` name — the
//! `FromA` wrap alone would have vanished. The authoring below tracks
//! both shapes per wall, which is exactly the discipline a real editor
//! owes the N3 merge lane.
//!
//! D2 bump: the first leg's `Distance` (mid-DAG — its cone is that
//! extrude plus all four unions, everything else reused).

use editor_core::{
    BooleanOp, Dimension, DocEdit, EntityKind, Expr, Node, ProfileEdgeRef, RecipeNodeId, RoleSeg,
    SlotId, StableName,
};

use super::super::fixture::{desc, len};
use super::{CorpusDoc, MassPin, Recorder};

/// A face name at `node` for outer-loop segment `seg`.
fn wall(node: RecipeNodeId, seg: u32) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Lateral(ProfileEdgeRef {
            loop_index: 0,
            segment: seg,
        })],
    }
}

/// Wraps a name one boolean deeper on the A side (N1 derivation).
fn from_a(sub: RecipeNodeId, name: &StableName) -> StableName {
    StableName {
        kind: name.kind,
        node: sub,
        path: vec![RoleSeg::FromA(Box::new(name.clone()))],
    }
}

/// Wraps a name one boolean deeper on the B side.
fn from_b(sub: RecipeNodeId, name: &StableName) -> StableName {
    StableName {
        kind: name.kind,
        node: sub,
        path: vec![RoleSeg::FromB(Box::new(name.clone()))],
    }
}

/// The N3 name of the face two DECLARED coincident faces glue into:
/// the sorted, deduplicated constituent set at the boolean's node.
fn merged(sub: RecipeNodeId, a: &StableName, b: &StableName) -> StableName {
    let mut cs = vec![from_a(sub, a), from_b(sub, b)];
    cs.sort();
    cs.dedup();
    StableName {
        kind: EntityKind::Face,
        node: sub,
        path: vec![RoleSeg::Merged(cs)],
    }
}

/// The legs, as (footprint polygon, the top segments each of the
/// leg's two flush walls meets). Loop order is
/// `p0→p1→p2→p3`, so segment `i` is the edge leaving `p[i]`.
/// Top loop `(0,0)→(4,0)→(4,3)→(0,3)`: seg 0 = `y=0`, 1 = `x=4`,
/// 2 = `y=3`, 3 = `x=0`.
type Leg = ([(f64, f64); 4], [(u32, u32); 2]);
fn legs() -> [Leg; 4] {
    [
        // (0,0): leg seg 0 is y=0 (top seg 0); leg seg 3 is x=0 (top seg 3).
        (
            [(0.0, 0.0), (0.5, 0.0), (0.5, 0.5), (0.0, 0.5)],
            [(0, 0), (3, 3)],
        ),
        // (4,0): leg seg 0 is y=0 (top 0); leg seg 1 is x=4 (top 1).
        (
            [(3.5, 0.0), (4.0, 0.0), (4.0, 0.5), (3.5, 0.5)],
            [(0, 0), (1, 1)],
        ),
        // (4,3): leg seg 1 is x=4 (top 1); leg seg 2 is y=3 (top 2).
        (
            [(3.5, 2.5), (4.0, 2.5), (4.0, 3.0), (3.5, 3.0)],
            [(1, 1), (2, 2)],
        ),
        // (0,3): leg seg 2 is y=3 (top 2); leg seg 3 is x=0 (top 3).
        (
            [(0.0, 2.5), (0.5, 2.5), (0.5, 3.0), (0.0, 3.0)],
            [(2, 2), (3, 3)],
        ),
    ]
}

/// The corner-table corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    let top_profile = r.insert(Node::Profile(desc(
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (0.0, 3.0)]],
    )));
    let top = r.insert(Node::Extrude {
        profile: top_profile,
        distance: len(0.25),
    });

    // The top's four side names, re-wrapped after every union.
    let mut top_walls: [StableName; 4] = [wall(top, 0), wall(top, 1), wall(top, 2), wall(top, 3)];
    let mut acc = top;
    let mut first_leg = top; // overwritten by the first leg's extrude
    for (i, (poly, flush)) in legs().into_iter().enumerate() {
        let prof = r.insert(Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![poly.to_vec()],
        )));
        let ext = r.insert(Node::Extrude {
            profile: prof,
            distance: len(1.125),
        });
        if i == 0 {
            first_leg = ext;
        }
        let pairs: Vec<_> = flush
            .iter()
            .map(|&(top_seg, leg_seg)| (top_walls[top_seg as usize].clone(), wall(ext, leg_seg)))
            .collect();
        let decl = r.insert(Node::Declare { pairs });
        let uni = r.insert(Node::Boolean {
            op: BooleanOp::Union,
            a: acc,
            b: ext,
            declare: Some(decl),
        });
        acc = uni;
        // Declared walls GLUE (an N3 `Merged` row); the rest just wrap.
        let next: [StableName; 4] = std::array::from_fn(|j| {
            match flush.iter().find(|&&(top_seg, _)| top_seg as usize == j) {
                Some(&(_, leg_seg)) => merged(uni, &top_walls[j], &wall(ext, leg_seg)),
                None => from_a(uni, &top_walls[j]),
            }
        });
        top_walls = next;
    }
    CorpusDoc {
        name: "corner_table",
        about: "corner-aligned four-leg table (declared flush contacts)",
        edits: r.edits,
        doc: r.doc,
        result: Some(acc),
        pin: Some(MassPin {
            volume: 4.0,
            area: Some(35.5),
        }),
        bump: DocEdit::SetParam {
            node: first_leg,
            slot: SlotId::Distance,
            expr: Expr::literal(1.0625, Dimension::Length).expect("dyadic length literal"),
        },
        bump_root: first_leg,
    }
}
