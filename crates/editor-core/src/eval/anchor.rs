//! **The profile naming anchor (PROFILES-V2 §V3).**
//!
//! Profile-entity names (`ProfileEdgeRef`/`ProfileVertexRef`) index
//! CANONICAL positions — canonical loop order (outer first, holes in
//! authored order) and canonical traversal from each loop's AUTHORED
//! start — and every verb that consumes a profile emits them in that
//! one numbering. The canonical form keeps what the author wrote
//! wherever validity allows, so nothing geometric enters an index and a
//! parameter edit CANNOT renumber: the start is the author's, and the
//! one thing canonicalization decides — each loop's traversal sense —
//! cannot flip under a continuous edit without passing through a
//! sliver, which validation refuses. Structural edits (re-authoring the
//! program) may renumber; the freeze doctrine (stale selections refuse
//! Vanished, M6-5) remains that backstop, as everywhere.
//!
//! # Mechanism
//!
//! The profile value carries a per-loop [`LoopAnchor`] — the exact
//! reindexing canonicalization applied, recovered by BIT-matching the
//! canonical f64 loop against the replayed (program-order) f64 loop.
//! Its one reader is the map from an authored step to the canonical
//! segments it became (`ProfileProgram::profile_edges_of`), where a
//! loop authored against its canonical sense reads `s ↦ n − 1 − s`.
//!
//! The match is exact: canonical loops are EXACT reindexings of their
//! input (validate's own contract). Uniqueness needs positions AND
//! bulges — a valid loop's vertices are pairwise distinct, which pins
//! the offset, and the bulge sign pins the orientation parity (which
//! positions alone cannot decide at n = 2 — see `derive_naming`).

use profile::{Profile, ProfileLoop, ValidatedProfile};

/// One canonical loop's anchor: how canonical indices map back to the
/// program's authored order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopAnchor {
    /// The PROGRAM loop index this canonical loop came from
    /// (description order; canonical order puts the outer loop first).
    pub program_loop: u32,
    /// The program index of canonical vertex 0.
    pub offset: u32,
    /// Whether canonical traversal runs OPPOSITE to authored order
    /// (validate orients outer CCW / holes CW; the author may have
    /// written either).
    pub reversed: bool,
    /// The loop's vertex count.
    pub len: u32,
}

impl LoopAnchor {
    /// Canonical vertex `k` → program vertex index.
    pub fn vertex(&self, k: u32) -> u32 {
        let n = self.len;
        if self.reversed {
            (self.offset + n - (k % n)) % n
        } else {
            (self.offset + k) % n
        }
    }

    /// Canonical segment `k` (canonical vertex k → k+1) → program
    /// segment index (the segment leaving its program-order start
    /// vertex).
    pub fn segment(&self, k: u32) -> u32 {
        let n = self.len;
        if self.reversed {
            // Canonical segment k runs program vertex (offset − k) →
            // (offset − k − 1): in program order that is the segment
            // LEAVING vertex (offset − k − 1).
            (self.offset + 2 * n - (k % n) - 1) % n
        } else {
            (self.offset + k) % n
        }
    }

    /// Program segment `s` → canonical segment: the inverse of
    /// [`LoopAnchor::segment`]. The reversed map is its own inverse
    /// shape (`k ↦ offset − k − 1` mod n); the forward one subtracts
    /// the offset.
    pub fn canonical_segment(&self, s: u32) -> u32 {
        let n = self.len;
        if self.reversed {
            (self.offset + 2 * n - (s % n) - 1) % n
        } else {
            (s % n + n - self.offset % n) % n
        }
    }
}

/// The per-profile naming anchor: one [`LoopAnchor`] per CANONICAL
/// loop, in canonical loop order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProfileNaming {
    /// Anchors, indexed by canonical loop index.
    pub loops: Vec<LoopAnchor>,
}

impl ProfileNaming {
    /// Whether every anchor is the identity (canonical order == program
    /// order) — every loop authored in its canonical sense, the outer
    /// loop first.
    pub fn is_identity(&self) -> bool {
        self.loops
            .iter()
            .enumerate()
            .all(|(i, a)| a.program_loop as usize == i && a.offset == 0 && !a.reversed)
    }
}

/// The profile node's evaluated value: the validated (canonical)
/// profile, the naming anchor that maps its program's segments onto the
/// canonical ones every profile ref names, and the per-edge radius the
/// program draws each of its segments at.
#[derive(Debug, Clone)]
pub struct ProfileValue<T: geom_core::Real> {
    /// The validated profile downstream ops consume.
    pub validated: ValidatedProfile<T>,
    /// The canonical ↔ program naming anchor.
    pub naming: ProfileNaming,
    /// **Which radius expression each profile edge is drawn at**, per
    /// CANONICAL loop and then per CANONICAL segment — the indexing a
    /// sweep's own wall record uses, so a consumer pairs the two by
    /// position and derives nothing.
    ///
    /// `None` at a position is an answer, and it has four causes:
    /// the segment is a straight one; it is an arc whose radius the
    /// program does not author as a scalar at all — a `bulge`, a
    /// `via`, a `center`; or it is a segment a radius EXTENDED rather
    /// than drew. The last is the §4 item 4 vertex-move exemption: an
    /// `arc_fillet`'s incoming spec whose carrier the arriving leg is
    /// already on moves that leg's end vertex instead of emitting a
    /// segment, so the radius is authored, enters the content key,
    /// and reaches no wall — the shape
    /// `edit_step_segments::every_attached_radius_was_keyed_first`
    /// authors as `keyed_but_never_attached()`.
    /// (`ProfileProgram::segment_radii` says why each answers
    /// nothing.)
    ///
    /// **Why it rides the VALUE.** It is the expression side of the
    /// per-edge flow source, and the node that HOLDS those expressions
    /// is this one — a sweep downstream attaches them to the walls it
    /// mints and has no other way to ask. Answering it needs the
    /// replay's records — the per-step spans and the per-radius
    /// emissions — which live on `ProfilePre` and are dropped with it,
    /// so the ANSWER is carried and the record is
    /// not: whether the record itself belongs on the value is PP1/PP2's
    /// open question
    /// (`work/wire/section-of-re-derives-the-whole-f64-precompute-the-profile-node-already-made.md`),
    /// and nothing here decides it.
    ///
    /// Expressions rather than lowered tokens, because lowering is
    /// scope-relative and the scope that matters is the ATTACHING
    /// evaluation's descent chain, read where the attach happens.
    pub edge_radii: Vec<Vec<Option<crate::expr::Expr>>>,
}

/// The profile node's f64 PRECOMPUTE (LIB-SWITCH §4b): the replayed
/// loops assembled into a `Profile<f64>`, its validated form, and the
/// derived naming anchor. Computed in `eval_node`'s resolution stage,
/// inside the node's verdict frame: replay and the f64 validation are
/// C6 STRUCTURE SELECTION (the v1 substrate's stored bits, one
/// derivation earlier), decided once on the node's behalf and logged
/// as the node's own. Under the pinned lift the op's value is
/// [`ProfilePre::validated_f64`] lifted, and the op decides nothing;
/// under the guided lift the op's own validation follows in the log.
#[derive(Debug, Clone)]
pub(crate) struct ProfilePre {
    /// The replayed profile at f64 (program order). Its `plane` is the
    /// 2-D record's assembly frame: [`ProfilePre::placement_f64`]
    /// where there is one, the conventional `SketchPlane::xy()` where
    /// there is not — validation is 2-D and the naming anchor is
    /// loop-derived, so no decision reads it. What a consumer PLACES
    /// with is `placement_f64`, never this field's plane.
    pub profile_f64: Profile<f64>,
    /// [`ProfilePre::profile_f64`]'s canonical form, minted by the
    /// pre-pass's one validation. Its plane is `profile_f64`'s
    /// assembly frame, with the same caveat: a consumer places with
    /// `placement_f64` (or the lane's derived frame), never with it.
    pub validated_f64: ValidatedProfile<f64>,
    /// The profile's `f64` placement, where the profile HAS one: an
    /// authored frame's document read, or a section's lane read pinned
    /// to `f64`. `None` for a profile on a derived frame (DM1c), whose
    /// only placement is the lane's — the invariant carried in the
    /// type rather than by a placeholder a reader could mistake for a
    /// placement.
    pub placement_f64: Option<profile::SketchPlane<f64>>,
    /// The canonical ↔ program naming anchor.
    pub naming: ProfileNaming,
    /// The discrete decisions this f64 pass made — the witness the
    /// lift's second pass consumes and re-verifies at its own scalar.
    pub structure: profile::ProfileStructure,
}

/// Derives the anchor by bit-matching the canonical f64 loops against
/// the replayed program-order f64 loops. `None` on a failed match —
/// an internal invariant break (validate's exact-reindexing contract),
/// surfaced typed by the caller, never a panic.
///
/// The match covers vertex POSITIONS, segment BULGES, and the declared
/// joint set. Positions alone are NOT enough (PR #291 review MAJOR-1,
/// both reviewers, executed): on a 2-vertex loop the forward and
/// reversed maps agree on every position (index arithmetic mod 2), so
/// a reversed hole circle — `circle()` lowers CCW, canonicalization
/// orients holes CW — would recover `reversed: false` and swap the two
/// semicircles' program names. Bulges disambiguate the parity exactly:
/// canonicalization's reversal NEGATES bulges (bit-exact sign flip)
/// and reindexes them (canonical segment k = program segment
/// offset−k−1 traversed backward), while rotation carries them
/// verbatim — so the bulge condition holds for precisely one
/// orientation whenever any segment is an arc. (An all-straight loop
/// has ±0.0 bulges either way, but needs n ≥ 3 to close, where
/// positions already decide.) Declared joints ride the same maps and
/// are checked as sets.
pub(crate) fn derive_naming(
    validated: &ValidatedProfile<f64>,
    program_loops: &[ProfileLoop<f64>],
) -> Option<ProfileNaming> {
    let mut anchors = Vec::with_capacity(validated.loops().len());
    for vl in validated.loops() {
        let cv = vl.vertices();
        let mut found = None;
        'progs: for (pi, pl) in program_loops.iter().enumerate() {
            let pv = &pl.vertices();
            let n = pv.len();
            if n != cv.len() || n == 0 {
                continue;
            }
            let bits = |p: &geom_core::Point2<f64>| (p.x.to_bits(), p.y.to_bits());
            for offset in 0..n {
                for reversed in [false, true] {
                    // Vertex map (canonical k → program index).
                    let vmap = |k: usize| {
                        if reversed {
                            (offset + n - k) % n
                        } else {
                            (offset + k) % n
                        }
                    };
                    // Segment map: canonical segment k starts at
                    // canonical vertex k; forward it is program
                    // segment (offset+k), reversed it is program
                    // segment (offset−k−1) traversed BACKWARD.
                    let smap = |k: usize| {
                        if reversed {
                            (offset + 2 * n - k - 1) % n
                        } else {
                            (offset + k) % n
                        }
                    };
                    let positions_ok =
                        (0..n).all(|k| bits(&cv[k].pos()) == bits(&pv[vmap(k)].pos()));
                    if !positions_ok {
                        continue;
                    }
                    // Bulges: verbatim under rotation, negated under
                    // reversal — bit-exact either way.
                    let bulges_ok = (0..n).all(|k| {
                        let pb = pv[smap(k)].bulge();
                        let want = if reversed { -pb } else { pb };
                        cv[k].bulge().to_bits() == want.to_bits()
                    });
                    if !bulges_ok {
                        continue;
                    }
                    // Declared joints as SETS under the vertex map
                    // (canonical joints are canonical vertex indices).
                    let mut mapped: Vec<usize> =
                        vl.tangent_joints().iter().map(|&j| vmap(j)).collect();
                    mapped.sort_unstable();
                    let mut prog_joints = pl.tangent_joints().to_vec();
                    prog_joints.sort_unstable();
                    prog_joints.dedup();
                    if mapped != prog_joints {
                        continue;
                    }
                    found = Some(LoopAnchor {
                        program_loop: pi as u32,
                        offset: offset as u32,
                        reversed,
                        len: n as u32,
                    });
                    break 'progs;
                }
            }
        }
        anchors.push(found?);
    }
    Some(ProfileNaming { loops: anchors })
}
