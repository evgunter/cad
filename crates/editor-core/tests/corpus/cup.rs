//! Corpus document **cup** — a box hollowed through `Node::Shell` with
//! its top face opened into a rim: the smallest document that
//! exercises the hollowing door, and the one whose closed forms are
//! exact.
//!
//! The recipe is four nodes — frame → profile → extrude → shell — and
//! the shell's `open` list is ONE authored name, the extrude's end
//! cap. The bump edits the extrude's HEIGHT and the wall THICKNESS,
//! both dyadic, which moves every wall of the cavity and the rim
//! without minting or retiring an entity, so the frozen designation
//! still resolves and the three names `lib_g17_shell_node.rs` reads
//! (the rim, the cavity floor, an outer wall) resolve to the same
//! roles after the bump.
//!
//! # The closed forms, derived
//!
//! The blank is `L × L × H`. Every boundary face moves inward by `t`,
//! so the cavity is the box `(L−2t) × (L−2t) × (H−t)`: the four side
//! walls and the floor each lose `t`, and the opened top loses nothing
//! — its inward offset becomes the rim's ring and then the cavity's
//! open end, which is why the cavity's height is `H−t` and not `H−2t`.
//!
//! ```text
//! V = L²H − (L−2t)²(H−t)
//! ```
//!
//! The surface has eleven faces: the outer bottom `L²`, four outer
//! sides `LH`, the rim annulus `L² − (L−2t)²`, the cavity floor
//! `(L−2t)²`, and four cavity walls `(L−2t)(H−t)`. The annulus's hole
//! and the floor cancel, so
//!
//! ```text
//! A = 2L² + 4LH + 4(L−2t)(H−t)
//! ```
//!
//! which at `H = L` is `6L² + 4(L−2t)(L−t)`. With every dimension
//! dyadic both are exact in `f64`, so the document carries a
//! [`MassPin`](super::MassPin) asserted with `==`.
//!
//! # Not in the registry, and why
//!
//! This document sits BESIDE [`super::documents`] rather than in it,
//! the way `die_composed` once did: registry membership runs every
//! document at `Dual64` and requires it green, and a dual has no shell
//! door — the kernel verb validates what it built with a certified
//! claim, which a dual cannot make (DL3) — so a shell node at that
//! scalar refuses TYPED (`ShellLaneUnsupported`). The refusal is pinned
//! by name in `lib_g17_shell_node.rs`, which also runs the rows the
//! registry would have run (both lanes, persistence, the bump). The
//! item that schedules the resolution is on LIB's slate.

use editor_core::{CapEnd, DocEdit, LoopProgram, Node, ProfileProgram, RoleSeg, SlotId};

use crate::fixture::{fname, len, xy_frame};

use super::{CorpusDoc, MassPin, Recorder};

/// The blank's side, meters (dyadic).
pub const L: f64 = 1.0;
/// The blank's height, meters (dyadic; `H = L` in the base document,
/// so the closed forms below read as the cube's).
pub const H: f64 = 1.0;
/// The wall thickness, meters (dyadic, and well under `L/2`, so the
/// two inward offsets of every facing pair clear each other).
pub const T: f64 = 0.125;
/// The bumped height (dyadic; still a box every wall of which clears
/// the bumped thickness).
pub const H_BUMPED: f64 = 1.5;
/// The bumped thickness (dyadic).
pub const T_BUMPED: f64 = 0.25;

/// The closed forms of the open cup (module docs), for any dyadic
/// `l`, `h`, `t`.
pub fn closed_forms(l: f64, h: f64, t: f64) -> MassPin {
    let inner = l - 2.0 * t;
    MassPin {
        volume: l * l * h - inner * inner * (h - t),
        area: Some(2.0 * l * l + 4.0 * l * h + 4.0 * inner * (h - t)),
    }
}

/// The closed forms of the SEALED hollow — every face offset inward,
/// no rim: the cavity is `(L−2t)³`-shaped, `(L−2t) × (L−2t) × (H−2t)`,
/// and both boundaries are complete boxes.
pub fn sealed_forms(l: f64, h: f64, t: f64) -> MassPin {
    let inner = l - 2.0 * t;
    let inner_h = h - 2.0 * t;
    MassPin {
        volume: l * l * h - inner * inner * inner_h,
        area: Some(2.0 * l * l + 4.0 * l * h + 2.0 * inner * inner + 4.0 * inner * inner_h),
    }
}

/// The cup's corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();

    let square = LoopProgram::polygon([(0.0, 0.0), (L, 0.0), (L, L), (0.0, L)]).unwrap();
    let plane = r.insert(xy_frame());
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![square],
    }));
    let blank = r.insert(Node::Extrude {
        profile,
        distance: len(H),
    });
    // The top: the extrude's END cap, the face the sweep vector points
    // out of. Authored, not queried — a designation FREEZES, so the
    // document states the face it means.
    let cup = r.insert(Node::shell(blank, len(T), vec![top(blank)]));

    CorpusDoc {
        name: "cup",
        about: "a unit box hollowed to a wall of 0.125 with its top opened into a rim",
        edits: r.edits,
        doc: r.doc,
        result: Some(cup),
        pin: Some(closed_forms(L, H, T)),
        bump: DocEdit::SetParam {
            node: blank,
            slot: SlotId::Distance,
            expr: len(H_BUMPED),
        },
        bump_root: blank,
    }
}

/// The blank's top face, by name: the extrude's end cap.
pub fn top(blank: editor_core::RecipeNodeId) -> editor_core::StableName {
    fname(blank, RoleSeg::Cap(CapEnd::End))
}

/// The blank's bottom face, by name: the extrude's start cap.
pub fn bottom(blank: editor_core::RecipeNodeId) -> editor_core::StableName {
    fname(blank, RoleSeg::Cap(CapEnd::Start))
}
