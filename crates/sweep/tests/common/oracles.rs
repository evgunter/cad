//! **Closed-form volumes the blend and chamfer suites meter against** —
//! derived here from the geometry, never from the kernel, so a carve
//! and its expectation cannot be wrong together.
//!
//! **The rule for what belongs here, and it is checkable by reading:**
//! a per-suite spelling comes here when it could not disagree with the
//! form below — it is the same formula in the same association, so one
//! of the two can only ever be a place for the other to drift. A
//! spelling that COULD disagree stays where it is and says so at the
//! copy: it is a second derivation, and a second derivation is the
//! whole detection value of a review probe.
//!
//! **Deliberately not absorbed**, and the whole of it:
//!
//! - `review_chamfer_r1_probes.rs` — the general `a×b×c` box form
//!   (`abc − 2d²(a+b+c) + (16/3)d³`, random dimensions each run) and
//!   the dimpled-spacer row that builds on it. It is `verbs_chamfer`'s
//!   reviewer pair and this is its own derivation: at `a = b = c` it
//!   must agree with [`chamfered_cube_volume`], and that it can fail to
//!   is the point;
//! - `blend4_r1_probes.rs::rounded_void_volume` — the Steiner sum
//!   computed term by term from an arbitrary polygon (`V + S·r +
//!   r²·Σ L_e θ_e/2 + (4π/3)r³`), which specialises to
//!   [`rounded_box_volume`] only on a rectangle;
//! - the die and surgery suites' Steiner spelling (`m5_pr12_die.rs`,
//!   `m5_pr12_die_body.rs`, `m6_surgery.rs`, `m6_surgery_interval.rs`,
//!   `review_m6_surgery_probes.rs`): `core³ + 6R·core² +
//!   12(πR²/4)·core + (4/3)πR³` sums twelve quarter-cylinders where
//!   [`rounded_box_volume`] sums one `3πlr²` term.
//!
//!   **What that difference is, measured rather than asserted.** The
//!   two are the same number in a different association, so they are
//!   not the same computation — but at the constants those rows
//!   actually use they very nearly are. Evaluated in Rust's operation
//!   order at `(1.0, 0.12)`, `(1.0, 0.15)`, `(2.0, 0.25)` and
//!   `(4.0, 0.25)` the two spellings are BIT-IDENTICAL; at
//!   `(2.0, 0.4)` (`m5_pr12_die_body.rs`'s second radius) they differ
//!   by one ulp, `8.9e-16`, against that row's `1e-9 · volume`
//!   tolerance. So keeping the copies is CONSERVATISM, not a bit-level
//!   necessity: the die family's rows were written and pinned against
//!   their own spelling, re-associating an expectation is not a
//!   test-support change, and those suites are a different family from
//!   this one. Nothing here claims a row would redden if they moved.
//!
//! - the same two forms OUTSIDE this crate, found by a sweep over the
//!   CONSTANTS rather than the names (the name-based census that cut
//!   this unit found none of them — every one is an inline expression
//!   or a differently-named helper). `chamfered_cube_volume`'s exact
//!   association appears at `crates/editor-core/tests/lib_g16_chamfer_node.rs`
//!   and, in Python, at `crates/pncad-py/tests/test_north_star.py`;
//!   `chamfered_cube_removed`'s at `demos/tour/src/diechamfer.rs`; the
//!   die family's Steiner association at
//!   `crates/editor-core/tests/m5_pr12_fillet_node.rs` — a sixth member
//!   of the five-file class above — and at
//!   `demos/tour/src/diefillet.rs`. None comes here, for the reason
//!   `super::cavity`'s own list gives its builders: a cross-crate home
//!   is LIB-U6's territory and this tree's routing rule says it is
//!   deliberately not built here, and `demos/` reaches the kernel from
//!   an outside consumer's seat, which is the point of a demo. Tracked
//!   as `work/tcost/chamfered-cube-and-steiner-oracles-outside-sweep.md`.
//!
//! For the same reason [`chamfered_cube_removed`] is not spelled as
//! `a³ − chamfered_cube_volume(a, d)` and vice versa. Same measurement:
//! at `a ∈ {2.0, 4.0}, d = 0.25` — every input the suites presently
//! use — the two associations are bit-identical; they are still two
//! computations, and each family's rows are pinned at the spelling
//! they were written against.

use core::f64::consts::PI;

/// **What a full twelve-edge chamfer at setback `d` LEAVES of a cube
/// of side `a`**: `a³ − 6ad² + (16/3)d³`.
///
/// The solid is the cube intersected with the twelve strip planes and
/// the eight corner planes, so the removed material is the union of
/// twelve triangular prisms (leg `d`, cross-section `d²/2`, the full
/// edge length) and eight corner tetrahedra `{x + y + z < 2d}`
/// (volume `4d³/3`), and inclusion–exclusion over the four sets that
/// meet at each corner over-counts by exactly `2d³` there.
///
/// At `d = a/2` it gives `a³/6` — the octahedron on the cube's face
/// centres, the degenerate end of the family.
pub fn chamfered_cube_volume(a: f64, d: f64) -> f64 {
    a.powi(3) - 6.0 * a * d * d + (16.0 / 3.0) * d.powi(3)
}

/// **What that same chamfer REMOVES**: `6ad² − (16/3)d³`, the
/// complement of [`chamfered_cube_volume`] in the cube.
///
/// A chamfer of a CONCAVE edge adds material, and what it adds is
/// congruent to what the same chamfer removes from a convex block —
/// a cavity's twelve edges and eight corners are the mirror of a
/// cube's, and neither the strip nor the corner patch has a side to
/// pick. So this is also the volume a fully chamfered cavity of side
/// `a` GAINS.
pub fn chamfered_cube_removed(a: f64, d: f64) -> f64 {
    6.0 * a * d * d - (16.0 / 3.0) * d.powi(3)
}

/// **The Steiner (Minkowski) volume of a cube of side `l` grown by a
/// ball of radius `r`**: `l³ + 6l²r + 3πlr² + (4/3)πr³` — the core,
/// six slabs, twelve quarter-cylinders and eight octants that sum to
/// one ball.
///
/// A full twelve-edge fillet at radius `r` leaves exactly this body
/// where the void was: the void of a filleted cavity of side `a` is
/// the shrunk box (`l = a − 2r`) swept by the ball, and the material
/// a filleted CUBE of side `a` keeps is `a³` less the same form at
/// that side's own scale.
pub fn rounded_box_volume(l: f64, r: f64) -> f64 {
    l.powi(3) + 6.0 * l * l * r + 3.0 * PI * l * r * r + (4.0 / 3.0) * PI * r.powi(3)
}
