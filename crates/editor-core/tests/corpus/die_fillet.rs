//! Corpus document **die_fillet** — M5 PR 12's acceptance shape (v),
//! stage 1: the die BLANK. A unit cube with every one of its twelve
//! edges blended at constant radius by the rolling-ball fillet, so the
//! corpus finally carries a body whose boundary is planes AND cylinders
//! AND spheres at once (twelve quarter-cylinder blends, eight
//! sphere-octant corners, six shrunken planar supports).
//!
//! The recipe is three nodes — profile → extrude → fillet. The
//! fillet's selection is every edge of the cube, AUTHORED, and the
//! bump below mints and retires no edge, so it cannot go stale. That
//! is the point of the document: the bump edits the EXTRUDE's
//! distance, the fillet node recomputes downstream of it, and there
//! is no selection to repair.
//!
//! # REGISTERED — and the blocker that once held it out
//!
//! This document is listed in `corpus/mod.rs::documents()`, so it
//! carries the full registry battery, INTERVAL lane included
//! (`m4_pr8_corpus_interval.rs`, `m4_pr6_roundtrip_interval.rs`), and
//! `Fillet` is a COVERED node kind in the vocabulary tally.
//!
//! It spent PR 12 outside that registry, deliberately, because the
//! fillet op did not survive the Interval lane:
//!
//! > `battery.rs`'s clearance screen seeded its pair gap with
//! > `T::from_f64(f64::INFINITY)` and folded the sampled distances in
//! > with `min`. At `f64` that sentinel is ordinary; at the certified
//! > `Interval` scalar `from_f64` POISONS non-reals — `±∞` embeds as
//! > NaI (`dec: Ill`) — and NaI absorbs through `min`, so the margin
//! > reaching `decide` was NaN and the op refused
//! > `Escalated { site: Chain, predicate: "fillet3_face_clearance",
//! > margin: Invalid }`. It fired for every request with at least one
//! > non-adjacent boundary-edge pair, i.e. every prism, i.e. every
//! > fillet this recipe layer can express.
//!
//! Holding the document out — rather than registering one that reds
//! the Interval lane, or teaching that lane to skip a document — kept
//! the gap stated instead of papered over, and `m5_pr12_fillet_node.rs`
//! pinned the refusal EXACTLY so it would retire itself. It did: the
//! gate fix at `5c8540f` seeds the gap from the first sampled pair,
//! the fillet op joined the certified lane, and this document joined
//! the registry. That file's Interval row now asserts GREEN.
//!
//! # No mass pin
//!
//! The closed forms for a rounded box of side `L` and radius `r` (core
//! plus 6 slabs, 12 quarter-cylinders and 8 octants that sum to one
//! ball; `sweep/tests/m5_pr12_die.rs` meters the same shape against
//! them) are
//!
//! ```text
//! V    = (L−2r)³ + 6r(L−2r)² + 12·(π r²/4)(L−2r) + (4/3)π r³
//! area = 6(L−2r)² + 12·(π r/2)(L−2r) + 4π r²
//! ```
//!
//! Both carry π. The corpus's [`MassPin`](super::MassPin) is asserted
//! with `==` against an EXACT oracle, and a π-valued expression is not
//! a dyadic value — a pin here would be pinning `f64` rounding of a
//! transcendental, not the geometry. So the pin is `None` on purpose
//! (the `cut_cylinder` / `boss_union` precedent). Both forms ARE
//! metered, at a stated relative tolerance, by
//! `m5_pr12_fillet_node.rs`'s `f64` row alongside validity (tier 1 +
//! closed) and the face count.

use editor_core::{DocEdit, LoopProgram, Node, ProfileProgram, SlotId};
use profile::SketchPlane;

use super::{CorpusDoc, Recorder};
use crate::fixture::{len, prism_edges};

/// The blank's side, meters (dyadic).
pub const L: f64 = 1.0;
/// The blend radius, meters (dyadic, and comfortably under `L/2`).
pub const R: f64 = 0.125;
/// The bumped extrude distance, meters (dyadic; still a box every edge
/// of which admits an `R` blend).
pub const L_BUMPED: f64 = 1.25;

/// The die-blank corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();

    let square = LoopProgram::polygon([(0.0, 0.0), (L, 0.0), (L, L), (0.0, L)]).unwrap();
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![square],
    }));
    let cube = r.insert(Node::Extrude {
        profile,
        distance: len(L),
    });
    // Every edge of the cube, AUTHORED: the recipe STATES the set
    // instead of meaning "whatever edges exist". The bump stretches
    // the cube without minting or retiring an edge, so the frozen
    // selection still resolves, which is what makes this document a
    // covariance row as well as a shape row.
    let blank = r.insert(Node::fillet(cube, len(R), prism_edges(cube, 4)));

    CorpusDoc {
        name: "die_fillet",
        about: "M5 shape (v) stage 1: every edge of a unit cube blended at r = 0.125",
        edits: r.edits,
        doc: r.doc,
        result: Some(blank),
        // π-valued closed forms are not dyadic — see the module docs.
        pin: None,
        // D2's incremental probe: stretch the cube. The profile is
        // reused; the extrude and the fillet downstream of it
        // recompute — and the fillet needs no re-selection to survive
        // the edit, since the bump mints and retires no edge.
        bump: DocEdit::SetParam {
            node: cube,
            slot: SlotId::Distance,
            expr: len(L_BUMPED),
        },
        bump_root: cube,
    }
}
