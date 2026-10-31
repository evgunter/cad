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
//! So a value edit moves no PIECE's locator. Which loop is outer,
//! which way a loop runs and how many segments a step draws are
//! decisions about geometry, and each can move a canonical position;
//! none of them moves a step's id or a piece's role. A piece the
//! current values do not draw has no canonical segment and its name
//! resolves `Vanished` until they draw it again. What a value edit CAN
//! change is which pieces one swept wall holds: runs are decided on the
//! values (N1, "Swept walls over a run"), so an edit that makes two
//! pieces collinear joins their walls into one run wall, and one that
//! bends them apart splits it — the old wall's name vanishes and N3
//! offers the wall that covers it, or the walls that cover its pieces.
//!
//! # Mechanism
//!
//! The profile value carries a per-loop [`LoopAnchor`] — which program
//! loop a canonical loop came from and whether canonicalization
//! reversed it, read off the f64 validation's own record
//! (`profile::CanonicalStructure`, `derive_naming`). A canonical
//! segment is read back to the program segment it is (`s ↦ n − 1 − s`
//! on a reversed loop), and the replay record names that segment's
//! piece (`profile::ReplayStructure::pieces`).

use profile::ValidatedProfile;

use crate::names::{PieceRole, PieceRun, ProfileEdgeRef, ProfileVertexRef, SectionCircle, to_u32};
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
    /// Per canonical loop: whether its canonical traversal runs
    /// opposite to authored order ([`LoopAnchor::reversed`]).
    pub reversed: Vec<bool>,
}

impl ProfilePieces {
    /// **An authored profile's pieces**: each canonical segment read
    /// back through its loop's anchor to the program segment it is,
    /// whose piece the replay recorded, spelled with the step's minted
    /// id.
    ///
    /// # Panics
    ///
    /// Where the three records do not describe one program, which only
    /// a kernel bug reaches: the anchor is the validation's reindexing
    /// of the loops the replay recorded ([`derive_naming`]), the replay
    /// records one piece per segment of each loop, drawn by one of its
    /// authored steps, and a document's profile carries one id list
    /// per loop and one id per authored step
    /// (`ProfileProgram::check_id_shape`, held at every door that
    /// writes ids).
    #[track_caller]
    pub(crate) fn publish(
        naming: &ProfileNaming,
        replay: &[profile::ReplayStructure],
        ids: &[Vec<StepId>],
    ) -> Self {
        let mut edges = Vec::with_capacity(naming.loops.len());
        let mut vertices = Vec::with_capacity(naming.loops.len());
        let reversed = naming.loops.iter().map(|a| a.reversed).collect();
        for anchor in &naming.loops {
            let loop_ = anchor.program_loop;
            let pl = loop_ as usize;
            let (Some(record), Some(steps)) = (replay.get(pl), ids.get(pl)) else {
                unreachable!(
                    "program loop {loop_} has no replay record or no id list ({} records, {} \
                     lists): the anchor names a loop of the program both were made from",
                    replay.len(),
                    ids.len()
                )
            };
            let pieces = &record.pieces;
            if pieces.len() != anchor.len as usize {
                unreachable!(
                    "program loop {loop_} replayed {} segments but its canonical loop has {}: the \
                     canonical loop is that loop reindexed",
                    pieces.len(),
                    anchor.len
                )
            }
            let mut refs = Vec::with_capacity(pieces.len());
            for p in pieces {
                let Some(&step) = steps.get(p.step) else {
                    unreachable!(
                        "step {} of program loop {loop_} drew a piece but the loop has only {} \
                         minted step ids: a piece is drawn by an authored step, and there is \
                         one id per authored step",
                        p.step,
                        steps.len()
                    )
                };
                refs.push(ProfileEdgeRef::Piece { step, role: p.role });
            }
            // `segment` and `vertex` answer below `anchor.len`, which is
            // `refs.len()`.
            let piece = |program_segment: u32| refs[program_segment as usize];
            edges.push((0..anchor.len).map(|k| piece(anchor.segment(k))).collect());
            // Canonical vertex `v` is program vertex `anchor.vertex(v)`,
            // where the program segment of the same index starts.
            vertices.push(
                (0..anchor.len)
                    .map(|v| piece(anchor.vertex(v)).start())
                    .collect(),
            );
        }
        Self {
            edges,
            vertices,
            reversed,
        }
    }

    /// **A kernel-built section's pieces** — a tube's: loop 0 the outer
    /// circle and loop 1 a hollow tube's bore, each of `counts[l]`
    /// pieces, named structurally under the node that builds them.
    ///
    /// # Panics
    ///
    /// For more than two loops, a shape no tube door builds.
    #[track_caller]
    pub(crate) fn section(counts: &[usize]) -> Self {
        let mut edges = Vec::with_capacity(counts.len());
        let mut vertices = Vec::with_capacity(counts.len());
        for (l, &n) in counts.iter().enumerate() {
            let circle = match l {
                0 => SectionCircle::Outer,
                1 => SectionCircle::Bore,
                _ => unreachable!(
                    "a tube's section has {} loops, though the tube doors build an outer circle \
                     and at most one bore",
                    counts.len()
                ),
            };
            let mut roles: Vec<PieceRole> = Vec::with_capacity(n);
            for k in 0..n {
                roles.push(PieceRole::Piece(to_u32(
                    k,
                    "a tube section's piece index, on a circle a tube door builds as two arcs",
                )));
            }
            edges.push(
                roles
                    .iter()
                    .map(|&role| ProfileEdgeRef::Section { circle, role })
                    .collect(),
            );
            vertices.push(
                roles
                    .iter()
                    .map(|&role| ProfileVertexRef::Section { circle, role })
                    .collect(),
            );
        }
        Self {
            reversed: vec![false; edges.len()],
            edges,
            vertices,
        }
    }

    /// **Distinct stand-in pieces for a profile no document holds** —
    /// a kernel profile a unit test sweeps directly: loop `l`'s segment
    /// `k` is the leg of step `1000·l + k`. Only the distinctness of the
    /// names is what such a test reads.
    #[cfg(test)]
    pub(crate) fn numbered(counts: &[usize]) -> Self {
        let step = |l: usize, k: usize| StepId::new(0, (1000 * l + k) as u64);
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
            reversed: vec![false; counts.len()],
        }
    }

    /// The locator of canonical segment `k` of canonical loop `l`.
    #[must_use]
    pub fn edge(&self, l: usize, k: usize) -> Option<ProfileEdgeRef> {
        self.edges.get(l)?.get(k).copied()
    }

    /// **The run of canonical segments `run` of loop `l` as a wall's
    /// pieces**, in authored order (`names/README.md`, N1 "Swept walls
    /// over a run"): by program segment, so a run wrapping through the
    /// loop's start begins at its first piece after the start vertex.
    /// `None` when a segment has no piece or the run is empty.
    #[must_use]
    pub fn run(&self, l: usize, run: &[usize]) -> Option<PieceRun> {
        let n = self.edges.get(l)?.len();
        let reversed = *self.reversed.get(l)?;
        let mut order: Vec<usize> = run.to_vec();
        order.sort_by_key(|&k| if reversed { n - 1 - k } else { k });
        PieceRun::new(
            order
                .iter()
                .map(|&k| self.edge(l, k))
                .collect::<Option<Vec<_>>>()?,
        )
    }

    /// The locator of canonical vertex `v` of canonical loop `l`.
    #[must_use]
    pub fn vertex(&self, l: usize, v: usize) -> Option<ProfileVertexRef> {
        self.vertices.get(l)?.get(v).copied()
    }

    /// **Whether the vertex `v` bounds a piece of `run`**: the start or
    /// the end of one of its canonical segments, so the wall swept over
    /// `run` holds the strut and the cap vertices over `v`.
    #[must_use]
    pub fn bounds(&self, run: &PieceRun, v: &ProfileVertexRef) -> bool {
        self.edges
            .iter()
            .zip(&self.vertices)
            .any(|(edges, vertices)| {
                let n = vertices.len();
                edges.iter().enumerate().any(|(k, e)| {
                    run.holds(e)
                        && (vertices.get(k) == Some(v) || vertices.get((k + 1) % n) == Some(v))
                })
            })
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
    /// Variables rather than lowered tokens, because lowering is
    /// scope-relative and the scope that matters is the ATTACHING
    /// evaluation's descent chain, read where the attach happens.
    pub edge_radii: Vec<Vec<Option<crate::var::VarId>>>,
    /// **The frame the profile is drawn on**, where it is drawn on one
    /// ([`crate::DrawnOn`]): a sweep of it is built in the profile's own
    /// coordinates and stands in that frame of its space.
    pub(crate) frame: Option<super::Frame<T>>,
}

impl<T: geom_core::Decide> ProfileValue<T> {
    /// **A body swept from this profile**, built in the profile's own
    /// coordinates: in the frame the profile is drawn on, or its own.
    pub(crate) fn framed(&self, body: std::sync::Arc<topo::Body<T>>) -> super::Framed<T> {
        match &self.frame {
            Some(frame) => super::Framed::in_frame(body, frame.clone()),
            None => super::Framed::own(body),
        }
    }
}

/// The profile node's f64 PRECOMPUTE (LIB-SWITCH §4b): the replayed
/// loops assembled into a `ConstructedProfile<f64>`, its validated
/// form, and the derived naming anchor. Computed in `eval_node`'s resolution stage,
/// inside the node's verdict frame: replay and the f64 validation are
/// C6 STRUCTURE SELECTION (the v1 substrate's stored bits, one
/// derivation earlier), decided once on the node's behalf and logged
/// as the node's own. At `f64` the op's value is
/// [`ProfilePre::validated_f64`] on the lane's plane, and the op decides
/// nothing ([`super::ProfileLane`]); at every other scalar the op's own
/// guided validation follows in the log.
#[derive(Debug, Clone)]
pub(crate) struct ProfilePre {
    /// The replayed profile's canonical form at f64, minted by the
    /// pre-pass's one validation, on the profile's own `xy` plane.
    pub validated_f64: ValidatedProfile<f64>,
    /// The canonical ↔ program naming anchor.
    pub naming: ProfileNaming,
    /// The piece every canonical position is ([`ProfilePieces::publish`]).
    pub pieces: ProfilePieces,
    /// The discrete decisions this f64 pass made — the witness the
    /// lift's second pass consumes and re-verifies at its own scalar.
    pub structure: profile::ProfileStructure,
    /// What the second pass logs at `f64` over the nominal, in order:
    /// the replay's and validation's decisions less those guidance pins
    /// ([`profile::PINNED_UNDER_GUIDANCE`]). The `f64` lane that skips
    /// the pass logs this in its place ([`super::ProfileLane`]).
    pub decided: geom_core::k_stats::Recorded,
}

/// **The anchor validation applied**, read off its own record: the
/// canonical loop order is the outer loop and then the holes in program
/// order, each loop reversed exactly where the record says
/// canonicalization reversed it, and its length the canonical chain's
/// segment count (a reversal keeps the count and the authored vertex 0),
/// so the anchor is the reindexing itself rather than a match against
/// its result.
///
/// # Errors
///
/// How many loops the record calls outer, where that is not exactly
/// one: such a record orders no loops, and no validation records one
/// ([`validated_naming`] says why).
pub(crate) fn derive_naming(
    canonical: &profile::CanonicalStructure,
) -> Result<ProfileNaming, usize> {
    let is_outer = |l: &profile::LoopCanonical| l.role == profile::LoopRole::Outer;
    let outers = canonical.loops.iter().filter(|l| is_outer(l)).count();
    let Some(outer) = canonical
        .loops
        .iter()
        .position(is_outer)
        .filter(|_| outers == 1)
    else {
        return Err(outers);
    };
    let order = core::iter::once(outer).chain((0..canonical.loops.len()).filter(|&i| i != outer));
    Ok(ProfileNaming {
        loops: order
            .map(|i| {
                let rec = &canonical.loops[i];
                LoopAnchor {
                    program_loop: crate::program::program_index(i),
                    reversed: rec.reversed,
                    len: crate::program::program_index(rec.segments.len()),
                }
            })
            .collect(),
    })
}

/// [`derive_naming`] of a record a validation of this evaluation just
/// made.
///
/// # Panics
///
/// Where the record does not name exactly one outer loop, which only a
/// kernel bug reaches: validation refuses an empty profile and a
/// profile with several outer loops (`ProfileError::EmptyProfile`,
/// `ProfileError::MultipleOuterLoops`) before it records anything, and
/// records one role per loop.
#[track_caller]
pub(crate) fn validated_naming(canonical: &profile::CanonicalStructure) -> ProfileNaming {
    match derive_naming(canonical) {
        Ok(naming) => naming,
        Err(outers) => unreachable!(
            "a validation recorded {outers} outer loops over {} loops, though it refuses any \
             profile without exactly one before recording",
            canonical.loops.len()
        ),
    }
}
