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

use profile::{Profile, ProfileLoop, RawLoop, ValidatedProfile};

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
/// profile plus the naming anchor that program-anchors every profile
/// ref emitted against it.
#[derive(Debug, Clone)]
pub struct ProfileValue<T: geom_core::Real> {
    /// The validated profile downstream ops consume.
    pub validated: ValidatedProfile<T>,
    /// The canonical→program naming anchor.
    pub naming: ProfileNaming,
}

/// The profile node's f64 PRECOMPUTE (LIB-SWITCH §4b): the replayed
/// loops assembled into a `Profile<f64>` plus the derived naming
/// anchor. Computed in `eval_node`'s resolution stage, OUTSIDE the
/// node's verdict-log bracket — replay and the f64 validation are C6
/// STRUCTURE SELECTION (the v1 substrate's stored bits, one
/// derivation earlier), not per-lane op decisions, so they do not
/// enter the node's logged verdicts; the lane's own `validate` (the
/// op) remains the logged surface, exactly as before the switch.
#[derive(Debug, Clone)]
pub(crate) struct ProfilePre {
    /// The replayed profile at f64 (program order).
    pub profile_f64: Profile<f64>,
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

/// Embeds an exact-`f64` placement into any evaluation scalar.
///
/// ONE HOME for a per-coordinate `from_f64` walk that had grown three:
/// the profile embed below did it inline, the lift's second pass needed
/// the same thing for a sketch plane, and each copy was three lines of
/// index-by-index transcription — the shape where a transposed `c1`/`c2`
/// is invisible in review and identical in every copy but one.
pub(crate) fn embed_affine<T: geom_core::Real>(
    a: &geom_core::Affine3<f64>,
) -> geom_core::Affine3<T> {
    let v = |w: geom_core::Vec3<f64>| {
        geom_core::Vec3::new(T::from_f64(w.x), T::from_f64(w.y), T::from_f64(w.z))
    };
    geom_core::Affine3::from_parts(
        geom_core::Mat3::from_cols(v(a.linear.c0), v(a.linear.c1), v(a.linear.c2)),
        v(a.translation),
    )
}

/// Embeds a stored exact-`f64` profile into any evaluation scalar (the
/// retired payload's `embed`, now a free function; the `from_f64`
/// embedding the parameter environment uses).
pub fn embed_profile<T: geom_core::Real>(p: &Profile<f64>) -> Profile<T> {
    let placement = embed_affine::<T>(&p.plane.placement);
    let loops = p
        .loops
        .iter()
        .map(|lp| {
            ProfileLoop::new(
                lp.vertices()
                    .iter()
                    .map(|vx| {
                        profile::ProfileVertex::new(
                            geom_core::Point2::new(
                                T::from_f64(vx.pos().x),
                                T::from_f64(vx.pos().y),
                            ),
                            T::from_f64(vx.bulge()),
                        )
                    })
                    .collect(),
            )
            .with_tangent_joints(lp.tangent_joints().to_vec())
        })
        .collect();
    Profile::new(profile::SketchPlane::new(placement), loops)
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
        | R::CornerArc { .. }
        | R::BandFace(..)
        | R::BandTrim { .. }
        | R::BandFoot(..)
        | R::BandCross(..)
        | R::BandCut(..)
        | R::BandSlit(..)
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
