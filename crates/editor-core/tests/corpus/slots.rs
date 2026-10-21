//! Corpus document **crossing_slots** — the R13 / issue #86 shape as
//! a recipe: a plate with two crossing slots, so the SECOND subtract's
//! A operand is itself a boolean result (boolean-of-boolean), and the
//! second tool's floor is coplanar with the first cavity's floor — a
//! contact the second subtract glues.
//!
//! Vocabulary: Profile, Extrude, Boolean (Subtract),
//! `InsertNode`, `SetParam`.
//!
//! Geometry (dyadic): plate `[0,3]² × [0,1]`; slot 1
//! `x ∈ [1,2], y ∈ [−1,4], z ∈ [0.5,1.5]`; slot 2
//! `x ∈ [−1,4], y ∈ [1,2], z ∈ [0.5,1.5]` (both run past the plate,
//! both stand proud of its top face — only the floors are flush).
//!
//! Exact oracles, derived here:
//!
//! - volume = `9 − (1·3·0.5) − (3·1·0.5) + (1·1·0.5)` (inclusion–
//!   exclusion on the crossing) = `9 − 1.5 − 1.5 + 0.5` = **6.5**
//! - area = top `9 − 5` (the cross-shaped opening `3+3−1`) `= 4`,
//!   bottom `9`, sides `4·3 − 4·(1·0.5)` `= 10`, cavity floor `5`,
//!   cavity walls `4 · (2 · 0.5)` `= 4` ⇒ **32.0**
//!
//! D2 bump: slot 1's `Distance` (mid-DAG — its cone is that extrude
//! plus both subtracts).

use super::PlacePart;
use editor_core::{CapEnd, DocEdit, EntityKind, Node, RecipeNodeId, RoleSeg, SlotId, StableName};

use crate::fixture::len;

use super::{CorpusDoc, MassPin, Recorder};

/// A cap face name at `node`.
fn cap(node: RecipeNodeId, end: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Cap(end)],
    }
}

/// The crossing-slots corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    let plate_p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)]],
    );
    let plate = r.insert(Node::Extrude {
        profile: plate_p.into(),
        distance: len(1.0),
    });

    // Slot 1: runs in y, placed against the plate with its floor at
    // z = 0.5.
    let slot1_p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(1.0, -1.0), (2.0, -1.0), (2.0, 4.0), (1.0, 4.0)]],
    );
    let slot1 = r.insert(Node::Extrude {
        profile: slot1_p.into(),
        distance: len(1.0),
    });
    // Nothing is coincident yet (the tool pierces the plate top and
    // overhangs both ends).
    let placed1 = r.against(
        plate,
        slot1,
        &editor_core::Frame::translation([0.0, 0.0, 0.5]),
    );
    let sub1 = r.insert(Node::Subtract {
        from: plate.into(),
        tool: placed1.into(),
    });

    // Slot 2: runs in x, SAME floor plane z = 0.5 — the coincident
    // floors, and the reason the second operand is a boolean result.
    let slot2_p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(-1.0, 1.0), (4.0, 1.0), (4.0, 2.0), (-1.0, 2.0)]],
    );
    let slot2 = r.insert(Node::Extrude {
        profile: slot2_p.into(),
        distance: len(1.0),
    });
    let placed2 = r.against(
        plate,
        slot2,
        &editor_core::Frame::translation([0.0, 0.0, 0.5]),
    );
    let sub2 = r.insert(Node::Subtract {
        from: sub1.into(),
        tool: placed2.into(),
    });

    r.place_part(sub2);

    CorpusDoc {
        name: "crossing_slots",
        about: "R13 / #86 crossing slots: boolean-of-boolean, coincident floors",
        edits: r.edits,
        doc: r.doc,
        result: Some(sub2),
        pin: Some(MassPin {
            volume: 6.5,
            area: Some(32.0),
        }),
        bump: DocEdit::SetParam {
            node: slot1,
            slot: SlotId::Distance,
            value: len(1.25).into(),
            fresh: Vec::new(),
        },
        bump_root: slot1,
    }
}
