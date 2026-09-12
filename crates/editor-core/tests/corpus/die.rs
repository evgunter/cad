//! Corpus document **die** — the M3 exact-oracle die as a recipe
//! (77 nodes), reused verbatim from the shared `fixture::die()`
//! authoring so the corpus and the PR 2/5/6 acceptance rows can never
//! drift apart.
//!
//! Vocabulary: Profile, Extrude, Transform, Declare, Boolean
//! (Subtract), `SetDocParam`, `InsertNode`.
//!
//! Exact oracles (dyadic throughout — cube `[0,2]³`, 21 pips of
//! 0.25 × 0.25 × 0.125):
//!
//! - volume = 8 − 21 · (1/4 · 1/4 · 1/8) = 8 − 21/128 = **7.8359375**
//! - area = 24 − 21 · (1/4)² (the pip mouths) + 21 · (1/4)² (their
//!   floors) + 21 · 4 · (1/4 · 1/8) (their walls) = 24 + 21/8 =
//!   **26.625**
//!
//! D2 bump: the +z pip master's `Distance` (the LAST face authored,
//! so its cone is exactly {master extrude, its Transform, the final
//! Subtract} — the minimal-cone probe).

use editor_core::{Dimension, DocEdit, Expr, SlotId};

use super::{CorpusDoc, MassPin};
use crate::fixture::die;

/// The die corpus document.
pub fn document() -> CorpusDoc {
    let d = die();
    CorpusDoc {
        name: "die",
        about: "M3 exact-oracle die: 21 declared pip subtracts (77 nodes)",
        edits: d.edits,
        doc: d.doc,
        result: Some(d.final_node),
        pin: Some(MassPin {
            volume: 7.8359375,
            area: Some(26.625),
        }),
        // A dyadic re-depth of the +z pip (still inward, still flush
        // at the declared cap): 3/16 instead of 1/8.
        bump: DocEdit::SetParam {
            node: d.pz_extrude,
            slot: SlotId::Distance,
            expr: Expr::literal(-0.1875, Dimension::Length).expect("dyadic length literal"),
        },
        bump_root: d.pz_extrude,
    }
}
