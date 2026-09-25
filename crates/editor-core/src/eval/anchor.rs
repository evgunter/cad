//! **The profile naming anchor (PROFILES-V2 §V3).**
//!
//! Every verb that consumes a profile iterates its CANONICAL positions
//! — canonical loop order (outer first, holes in authored order) and
//! canonical traversal from each loop's AUTHORED start — and none of
//! them is a name. A published name spells the PIECE a canonical
//! segment is (`names/README.md`, "N1, the profile pieces"): the step
//! that drew it, by the id the step was minted with, and its role in
//! that step's fixed list. This module pairs the two when a profile's
//! value is built: [`ProfilePieces`], one locator per canonical
//! segment and per canonical vertex, which the sweep emitters read in
//! place of the position they iterate.
//!
//! So a value edit moves no name. Which loop is outer, which way a
//! loop runs and how many segments a step draws are decisions about
//! geometry, and each can move a canonical position; none of them
//! moves a step's id or a piece's role. A piece the current values do
//! not draw has no canonical segment and its name resolves `Vanished`
//! until they draw it again.
//!
//! # Mechanism
//!
//! The profile value carries a per-loop [`LoopAnchor`] — which program
//! loop a canonical loop came from and whether canonicalization
//! reversed it, recovered by BIT-matching the canonical f64 loop
//! against the replayed (program-order) f64 loop. A canonical segment
//! is read back to the program segment it is (`s ↦ n − 1 − s` on a
//! reversed loop), and the replay record names that segment's piece
//! (`profile::ReplayStructure::pieces`).
//!
//! The match is exact: canonical loops are EXACT reindexings of their
//! input (validate's own contract), starting at the authored vertex 0.
//! Uniqueness needs positions AND bulges — the bulge sign pins the
//! orientation parity, which positions alone cannot decide at n = 2
//! (see `derive_naming`).

use profile::{Profile, ProfileLoop, ValidatedProfile};

use crate::names::{PieceRole, ProfileEdgeRef, ProfileVertexRef, SectionCircle};
use crate::node::StepId;

/// One canonical loop's anchor: how canonical indices map back to the
/// program's authored order. Canonical vertex 0 is always program
/// vertex 0 (the authored start), so the map is the identity or the
/// reflection that keeps vertex 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopAnchor {
    /// The PROGRAM loop index this canonical loop came from
    /// (description order; canonical order puts the outer loop first).
    pub program_loop: u32,
    /// Whether canonical traversal runs OPPOSITE to authored order
    /// (validate orients outer CCW / holes CW; the author may have
    /// written either).
    pub reversed: bool,
    /// The loop's vertex count.
    pub len: u32,
}

impl LoopAnchor {
    /// Canonical vertex `k` → program vertex index: `k`, or its
    /// reflection `(n − k) mod n` on a reversed loop.
    pub fn vertex(&self, k: u32) -> u32 {
        let n = self.len;
        if self.reversed {
            (n - (k % n)) % n
        } else {
            k % n
        }
    }

    /// Canonical segment `k` (canonical vertex k → k+1) → program
    /// segment index (the segment leaving its program-order start
    /// vertex): `k`, or `n − 1 − k` on a reversed loop, whose canonical
    /// segment `k` runs program vertex `n − k` back to `n − k − 1`.
    pub fn segment(&self, k: u32) -> u32 {
        let n = self.len;
        if self.reversed {
            n - 1 - (k % n)
        } else {
            k % n
        }
    }

    /// Program vertex `p` → canonical vertex: the inverse of
    /// [`LoopAnchor::vertex`], which is an involution.
    pub fn canonical_vertex(&self, p: u32) -> u32 {
        self.vertex(p)
    }

    /// Program segment `s` → canonical segment: the inverse of
    /// [`LoopAnchor::segment`], which is an involution.
    pub fn canonical_segment(&self, s: u32) -> u32 {
        self.segment(s)
    }
}

/// The per-profile naming anchor: one [`LoopAnchor`] per CANONICAL
/// loop, in canonical loop order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProfileNaming {
    /// Anchors, indexed by canonical loop index.
    pub loops: Vec<LoopAnchor>,
}

/// **A canonical position** — canonical loop `loop_index` (0 the outer
/// loop, then the holes in description order) and segment `segment`
/// counted along its canonical traversal from the loop's authored
/// start. The order the emitters, the loft's correspondence and the
/// viewer's per-segment marks iterate (V3, DM8); a position, never a
/// name ([`ProfilePieces`] pairs the two).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CanonicalSegment {
    /// The canonical loop.
    pub loop_index: u32,
    /// The segment along it.
    pub segment: u32,
}

/// **The piece every canonical position is** — what a sweep emitter
/// names each wall, rim and vertex it iterates by (`names/README.md`,
/// "N1, the profile pieces"): per CANONICAL loop, the locator of each
/// canonical segment and of each canonical vertex (the start of the
/// piece of the same spelling).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProfilePieces {
    /// Per canonical loop, per canonical segment.
    pub edges: Vec<Vec<ProfileEdgeRef>>,
    /// Per canonical loop, per canonical vertex.
    pub vertices: Vec<Vec<ProfileVertexRef>>,
}

impl ProfilePieces {
    /// **An authored profile's pieces**: each canonical segment read
    /// back through its loop's anchor to the program segment it is,
    /// whose piece the replay recorded, spelled with the step's minted
    /// id. `None` where the three records do not describe one program
    /// — an anchor naming a loop the record or the ids lack, or a
    /// record of another length — which the evaluation surfaces typed.
    pub(crate) fn publish(
        naming: &ProfileNaming,
        replay: &[profile::ReplayStructure],
        ids: &[Vec<StepId>],
    ) -> Option<Self> {
        let mut edges = Vec::with_capacity(naming.loops.len());
        let mut vertices = Vec::with_capacity(naming.loops.len());
        for anchor in &naming.loops {
            let pl = anchor.program_loop as usize;
            let pieces = &replay.get(pl)?.pieces;
            let steps = ids.get(pl)?;
            if pieces.len() != anchor.len as usize {
                return None;
            }
            let piece = |program_segment: u32| -> Option<ProfileEdgeRef> {
                let p = pieces.get(program_segment as usize)?;
                Some(ProfileEdgeRef::Piece {
                    step: *steps.get(p.step)?,
                    role: p.role.into(),
                })
            };
            edges.push(
                (0..anchor.len)
                    .map(|k| piece(anchor.segment(k)))
                    .collect::<Option<Vec<_>>>()?,
            );
            // Canonical vertex `v` is program vertex `anchor.vertex(v)`,
            // where the program segment of the same index starts.
            vertices.push(
                (0..anchor.len)
                    .map(|v| piece(anchor.vertex(v)).map(|e| e.start()))
                    .collect::<Option<Vec<_>>>()?,
            );
        }
        Some(Self { edges, vertices })
    }

    /// **A kernel-built section's pieces** — a tube's: loop 0 the outer
    /// circle and loop 1 a hollow tube's bore, each of `counts[l]`
    /// pieces, named structurally under the node that builds them.
    /// `None` for a shape no tube door builds (more than two loops).
    pub(crate) fn section(counts: &[usize]) -> Option<Self> {
        let circle = |l: usize| match l {
            0 => Some(SectionCircle::Outer),
            1 => Some(SectionCircle::Bore),
            _ => None,
        };
        let mut edges = Vec::with_capacity(counts.len());
        let mut vertices = Vec::with_capacity(counts.len());
        for (l, &n) in counts.iter().enumerate() {
            let circle = circle(l)?;
            let roles = (0..n).map(|k| PieceRole::Piece(u32::try_from(k).unwrap_or(u32::MAX)));
            edges.push(
                roles
                    .clone()
                    .map(|role| ProfileEdgeRef::Section { circle, role })
                    .collect(),
            );
            vertices.push(
                roles
                    .map(|role| ProfileVertexRef::Section { circle, role })
                    .collect(),
            );
        }
        Some(Self { edges, vertices })
    }

    /// **Distinct stand-in pieces for a profile no document holds** —
    /// a kernel profile a unit test sweeps directly: loop `l`'s segment
    /// `k` is the leg of step `1000·l + k`. Only the distinctness of the
    /// names is what such a test reads.
    #[cfg(test)]
    pub(crate) fn numbered(counts: &[usize]) -> Self {
        let step = |l: usize, k: usize| StepId((1000 * l + k) as u64);
        Self {
            edges: counts
                .iter()
                .enumerate()
                .map(|(l, &n)| {
                    (0..n)
                        .map(|k| ProfileEdgeRef::Piece {
                            step: step(l, k),
                            role: PieceRole::Leg,
                        })
                        .collect()
                })
                .collect(),
            vertices: counts
                .iter()
                .enumerate()
                .map(|(l, &n)| {
                    (0..n)
                        .map(|k| ProfileVertexRef::Piece {
                            step: step(l, k),
                            role: PieceRole::Leg,
                        })
                        .collect()
                })
                .collect(),
        }
    }

    /// The locator of canonical segment `k` of canonical loop `l`.
    #[must_use]
    pub fn edge(&self, l: usize, k: usize) -> Option<ProfileEdgeRef> {
        self.edges.get(l)?.get(k).copied()
    }

    /// The locator of canonical vertex `v` of canonical loop `l`.
    #[must_use]
    pub fn vertex(&self, l: usize, v: usize) -> Option<ProfileVertexRef> {
        self.vertices.get(l)?.get(v).copied()
    }
}

/// The profile node's evaluated value: the validated (canonical)
/// profile, the naming anchor that maps its program's segments onto the
/// canonical ones, the piece every canonical position is, and the
/// per-edge radius the program draws each of its segments at.
#[derive(Debug, Clone)]
pub struct ProfileValue<T: geom_core::Real> {
    /// The validated profile downstream ops consume.
    pub validated: ValidatedProfile<T>,
    /// The canonical ↔ program naming anchor.
    pub naming: ProfileNaming,
    /// The piece every canonical segment and vertex is — what the
    /// sweeps over this profile name their entities by.
    pub pieces: ProfilePieces,
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
    /// The piece every canonical position is ([`ProfilePieces::publish`]).
    pub pieces: ProfilePieces,
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
/// and reindexes them (canonical segment k = program segment n−1−k
/// traversed backward), while the identity carries them verbatim — so
/// the bulge condition holds for precisely one orientation whenever
/// any segment is an arc. (An all-straight loop
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
            if pv.len() != cv.len() || pv.is_empty() {
                continue;
            }
            // An anchor stores its loop and count as `u32`; one that
            // does not fit is no anchor, and the derivation fails typed
            // rather than anchoring to a wrapped loop.
            let program_loop = u32::try_from(pi).ok()?;
            let n = u32::try_from(pv.len()).ok()?;
            let bits = |p: &geom_core::Point2<f64>| (p.x.to_bits(), p.y.to_bits());
            for reversed in [false, true] {
                let a = LoopAnchor {
                    program_loop,
                    reversed,
                    len: n,
                };
                let vmap = |k: u32| a.vertex(k) as usize;
                let smap = |k: u32| a.segment(k) as usize;
                let positions_ok =
                    (0..n).all(|k| bits(&cv[k as usize].pos()) == bits(&pv[vmap(k)].pos()));
                if !positions_ok {
                    continue;
                }
                // Bulges: verbatim forward, negated under reversal —
                // bit-exact either way.
                let bulges_ok = (0..n).all(|k| {
                    let pb = pv[smap(k)].bulge();
                    let want = if reversed { -pb } else { pb };
                    cv[k as usize].bulge().to_bits() == want.to_bits()
                });
                if !bulges_ok {
                    continue;
                }
                // Declared joints as SETS under the vertex map
                // (canonical joints are canonical vertex indices).
                let mut mapped: Vec<usize> = vl
                    .tangent_joints()
                    .iter()
                    .map(|&j| u32::try_from(j).ok().map(vmap))
                    .collect::<Option<_>>()?;
                mapped.sort_unstable();
                let mut prog_joints = pl.tangent_joints().to_vec();
                prog_joints.sort_unstable();
                prog_joints.dedup();
                if mapped != prog_joints {
                    continue;
                }
                found = Some(a);
                break 'progs;
            }
        }
        anchors.push(found?);
    }
    Some(ProfileNaming { loops: anchors })
}
