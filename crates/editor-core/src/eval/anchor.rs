//! **Program-anchored profile naming (LIB-SWITCH §6 under the ratified
//! resolution; PROFILES-V2 §V3 round 2).**
//!
//! Profile-entity naming for program loops anchors to PROGRAM-
//! STRUCTURAL positions: `ProfileEdgeRef`/`ProfileVertexRef` indices
//! mean "the segment/vertex the program's step order authored", not
//! "the canonical rotation's position". Nothing geometric enters the
//! index, so a parameter edit CANNOT renumber, by construction — the
//! renumbering class (lex-band crossings under the canonical rotation)
//! is eliminated for program loops. Structural edits (re-authoring the
//! program) may renumber; the freeze doctrine (stale selections refuse
//! Vanished, M6-5) remains that backstop, as everywhere.
//!
//! # Mechanism
//!
//! `validate` still canonicalizes (its ladder is out of this unit's
//! fence, and downstream GEOMETRY follows canonical order — exports
//! stay byte-identical). What changes is the NAMING SUBSTRATE: the
//! profile value carries a per-loop [`LoopAnchor`] — the exact
//! reindexing canonicalization applied, recovered by BIT-matching the
//! canonical f64 loop against the replayed (program-order) f64 loop —
//! and every emitted name's profile refs are rewritten canonical →
//! program before the table is published ([`remap_table`]). Emitters
//! stay untouched (`names/` is fenced); the rewrite is a pure
//! reindexing at the emission call sites.
//!
//! The match is exact: canonical loops are EXACT reindexings of their
//! input (validate's own contract). Uniqueness needs positions AND
//! bulges — a valid loop's vertices are pairwise distinct, which pins
//! the offset, and the bulge sign pins the orientation parity (which
//! positions alone cannot decide at n = 2 — see `derive_naming`).

use profile::{Profile, ProfileLoop, ValidatedProfile};

use crate::names::{Entry, NameTable, ProfileEdgeRef, ProfileVertexRef, RoleSeg, StableName};

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
    /// shape (`k ↦ offset − k − 1` mod n), the forward one subtracts
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

/// **How one profile's refs reach a published name table**: the
/// profile's OWN anchor, which says where each of its program segments
/// sits canonically, and the anchor the table's canonical refs were
/// PUBLISHED through.
///
/// A profile's own sweep publishes through the profile's own anchor,
/// so the two are one and a program ref reaches the table unchanged —
/// that is `From<&ProfileNaming>`. A loft publishes one table for all
/// of its sections, through one of their anchors
/// ([`SectionAnchors`]), and a section's program ref reaches that
/// table through its canonical position: canonical loop `l`, segment
/// `k` of every section is the one wall the skin built for them.
///
/// Neither half is settable from outside: the published half is read
/// off the evaluation that published it, so a caller cannot pair a
/// section with an anchor its table was not written through.
#[derive(Debug, Clone, Copy)]
pub struct Anchoring<'a> {
    own: &'a ProfileNaming,
    published: &'a ProfileNaming,
}

impl<'a> Anchoring<'a> {
    /// The profile's own anchor — the one the evaluation's structure
    /// record is checked against.
    pub fn own(&self) -> &'a ProfileNaming {
        self.own
    }

    /// The anchor the table's refs were published through.
    pub fn published(&self) -> &'a ProfileNaming {
        self.published
    }
}

impl<'a> From<&'a ProfileNaming> for Anchoring<'a> {
    /// A profile consumed by a node that publishes through the
    /// profile's own anchor (extrude, revolve, the profile-operand
    /// sweeps).
    fn from(naming: &'a ProfileNaming) -> Self {
        Self {
            own: naming,
            published: naming,
        }
    }
}

/// **A loft's section anchors**: every section's own
/// [`ProfileNaming`], in section order, and the one the loft's name
/// table was published through — section 0's.
///
/// The loft's emitter mints ONE ref per wall — a canonical
/// `(loop, segment)` shared by every section, because the skin pairs
/// canonical segment `k` of each section into one wall — so one anchor
/// publishes the table, and a section authored rotated or reversed
/// relative to section 0 reaches it through [`SectionAnchors::section`].
/// Carried on the loft's value ([`crate::eval::NodeValue::section_anchors`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionAnchors {
    /// Section 0's anchor — the published one.
    first: ProfileNaming,
    /// Sections 1.., in section order.
    rest: Vec<ProfileNaming>,
}

impl SectionAnchors {
    /// The anchors of a loft's sections, publishing through `first`'s.
    pub(crate) fn new(first: ProfileNaming, rest: Vec<ProfileNaming>) -> Self {
        Self { first, rest }
    }

    /// The anchor the loft's table was published through.
    pub fn published(&self) -> &ProfileNaming {
        &self.first
    }

    /// Section `i`'s anchoring into the loft's table, `None` past the
    /// last section.
    pub fn section(&self, i: usize) -> Option<Anchoring<'_>> {
        let own = match i {
            0 => &self.first,
            _ => self.rest.get(i - 1)?,
        };
        Some(Anchoring {
            own,
            published: &self.first,
        })
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
    /// order) — the common corpus case; callers skip the table rebuild.
    pub fn is_identity(&self) -> bool {
        self.loops
            .iter()
            .enumerate()
            .all(|(i, a)| a.program_loop as usize == i && a.offset == 0 && !a.reversed)
    }
}

/// The profile node's evaluated value: the validated (canonical)
/// profile, the naming anchor that program-anchors every profile ref
/// emitted against it, and the per-edge radius the program draws each
/// of its segments at.
#[derive(Debug, Clone)]
pub struct ProfileValue<T: geom_core::Real> {
    /// The validated profile downstream ops consume.
    pub validated: ValidatedProfile<T>,
    /// The canonical→program naming anchor.
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
    /// The canonical→program naming anchor.
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

/// Rewrites one profile ref canonical → program.
fn remap_edge(naming: &ProfileNaming, e: ProfileEdgeRef) -> ProfileEdgeRef {
    match naming.loops.get(e.loop_index as usize) {
        None => e,
        Some(a) => ProfileEdgeRef {
            loop_index: a.program_loop,
            segment: a.segment(e.segment),
        },
    }
}

fn remap_vertex(naming: &ProfileNaming, v: ProfileVertexRef) -> ProfileVertexRef {
    match naming.loops.get(v.loop_index as usize) {
        None => v,
        Some(a) => ProfileVertexRef {
            loop_index: a.program_loop,
            vertex: a.vertex(v.vertex),
        },
    }
}

/// Rewrites the profile-ref-bearing role segments an emitter minted
/// DIRECTLY (extrude/revolve/loft emitters; wrapped upstream names are
/// already program-anchored, so composition variants are untouched).
///
/// Not to be confused with [`crate::refactor`]'s function of the same
/// name: that one partitions [`RoleSeg`] by whether the variant embeds
/// a [`StableName`] and recurses into the ones that do; this one
/// partitions it by whether the variant embeds a PROFILE LOCATOR, and
/// does not recurse — the sentence above is why, and nothing enforces
/// it.
fn remap_seg(naming: &ProfileNaming, seg: RoleSeg) -> RoleSeg {
    use RoleSeg as R;
    match seg {
        R::Lateral(e) => R::Lateral(remap_edge(naming, e)),
        R::RimEdge(c, e) => R::RimEdge(c, remap_edge(naming, e)),
        R::LateralEdge(v) => R::LateralEdge(remap_vertex(naming, v)),
        R::CapVertex(c, v) => R::CapVertex(c, remap_vertex(naming, v)),
        R::Band(e) => R::Band(remap_edge(naming, e)),
        R::BandRim(v) => R::BandRim(remap_vertex(naming, v)),
        R::BandRimPi(v) => R::BandRimPi(remap_vertex(naming, v)),
        R::BandPi(e) => R::BandPi(remap_edge(naming, e)),
        R::Meridian(m, e) => R::Meridian(m, remap_edge(naming, e)),
        R::MeridianVertex(m, v) => R::MeridianVertex(m, remap_vertex(naming, v)),
        R::Pole(v) => R::Pole(remap_vertex(naming, v)),
        R::AxisEdge(e) => R::AxisEdge(remap_edge(naming, e)),
        // EXHAUSTIVE on purpose (the `walk_names` rule): the arms above
        // are exactly the variants that embed a `ProfileEdgeRef` or a
        // `ProfileVertexRef`, and these are exactly the ones that do
        // not. A future variant carrying a profile locator must be
        // classified here or the compile breaks; a catch-all would let
        // it cross a re-anchor with a stale locator, silently.
        R::OutputBody
        | R::Cap(..)
        | R::RevolveCap(..)
        | R::FromMember { .. }
        | R::FromA(..)
        | R::FromB(..)
        | R::Seam { .. }
        | R::Merged(..)
        | R::Fragment(..)
        | R::SplitBody(..)
        | R::SectionFace { .. }
        | R::SectionEdge { .. }
        | R::SplitFragment { .. }
        | R::CrossingVertex { .. }
        | R::OnToolVertex { .. }
        | R::FromTarget(..)
        | R::BlendFace(..)
        | R::CornerFace(..)
        | R::TrimEdge { .. }
        | R::FootVertex { .. }
        | R::EndArc { .. }
        | R::BandFace(..)
        | R::BandTrim { .. }
        | R::BandFoot(..)
        | R::BandCross(..)
        | R::BandCut(..)
        | R::BandSlit(..)
        | R::Inner(..)
        | R::Rim(..)
        | R::HoleRim { .. }
        | R::InPart { .. }
        | R::Instance { .. } => seg,
    }
}

fn remap_name(naming: &ProfileNaming, name: StableName) -> StableName {
    StableName {
        kind: name.kind,
        node: name.node,
        path: name
            .path
            .into_iter()
            .map(|seg| remap_seg(naming, seg))
            .collect(),
    }
}

/// Rewrites every name in an emitted table canonical → program (the
/// anchor rewrite, applied at the emission call sites of the ops that
/// mint profile refs). The rewrite is a bijection per loop, so
/// injectivity is preserved; a collision is therefore an internal bug
/// and surfaces as `None` (the caller refuses typed).
pub(crate) fn remap_table(table: &NameTable, naming: &ProfileNaming) -> Option<NameTable> {
    let mut out = NameTable::new();
    for (name, entry) in table.iter() {
        let new_name = remap_name(naming, name.clone());
        match entry {
            Entry::Unique(ent) => out.insert(new_name, *ent).ok()?,
            Entry::Tied(ents) => out.insert_tied(new_name, ents.clone()).ok()?,
        }
    }
    Some(out)
}
