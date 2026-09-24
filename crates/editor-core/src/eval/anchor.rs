//! **The profile naming anchor (PROFILES-V2 §V3).**
//!
//! Profile-entity names (`ProfileEdgeRef`/`ProfileVertexRef`) index
//! CANONICAL positions — canonical loop order (outer first, holes in
//! authored order) and canonical traversal from each loop's AUTHORED
//! start — and every verb that consumes a profile emits them in that
//! one numbering. The canonical form keeps what the author wrote
//! wherever validity allows, so a parameter edit renumbers only through
//! the two things canonicalization decides: each loop's traversal
//! sense, which cannot flip under a continuous edit without passing
//! through a sliver, and which loop is the outer one (canonical loop
//! 0), which cannot change without the loops crossing. A value edit can
//! still move either in one jump — a hole grown until it encloses the
//! outer loop, a vertex moved across its loop — and a document-parameter
//! edit can also land the profile in a state whose numbering cannot be
//! read at all (it does not replay, or its loops cannot be ordered:
//! refusing programs may exist at rest). The edit door compares the
//! profile's numbering before and after every value edit that reaches
//! it: where both sides read and differ it carries every name spelled
//! in the numbering across, and where either side cannot be read it
//! strands them, reporting each row either way (`reanchor_report` in
//! `edit.rs`, through `SetProgram`'s map and report). What a parameter
//! edit CAN still
//! do is change how many segments a step draws — a corner fillet whose
//! runs reach a `Zero` fit emits nothing, so a radius written through
//! `SetParam` grows or shrinks the loop and every live name after that
//! step renumbers, reported by nothing
//! (`work/edit/a-slot-edit-through-a-zero-fit-renumbers-a-loops-live-names.md`).
//! Structure changes by `DocEdit::SetProgram`, which rebinds every kept
//! name and retires the rest (DM7); the freeze doctrine (stale
//! selections refuse `Vanished`, M6-5) backstops what a reshaping
//! strands, as everywhere.
//!
//! # Mechanism
//!
//! The profile value carries a per-loop [`LoopAnchor`] — which program
//! loop a canonical loop came from and whether canonicalization
//! reversed it, recovered by BIT-matching the canonical f64 loop
//! against the replayed (program-order) f64 loop. Its one reader is the
//! map from an authored step to the canonical segments it became
//! (`ProfileProgram::profile_edges_of`), where a loop authored against
//! its canonical sense reads `s ↦ n − 1 − s`.
//!
//! The match is exact: canonical loops are EXACT reindexings of their
//! input (validate's own contract), starting at the authored vertex 0.
//! Uniqueness needs positions AND bulges — the bulge sign pins the
//! orientation parity, which positions alone cannot decide at n = 2
//! (see `derive_naming`).

use profile::{Profile, ProfileLoop, ValidatedProfile};

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
            let n = pv.len();
            if n != cv.len() || n == 0 {
                continue;
            }
            let bits = |p: &geom_core::Point2<f64>| (p.x.to_bits(), p.y.to_bits());
            for reversed in [false, true] {
                let a = LoopAnchor {
                    program_loop: pi as u32,
                    reversed,
                    len: n as u32,
                };
                let vmap = |k: usize| a.vertex(k as u32) as usize;
                let smap = |k: usize| a.segment(k as u32) as usize;
                let positions_ok = (0..n).all(|k| bits(&cv[k].pos()) == bits(&pv[vmap(k)].pos()));
                if !positions_ok {
                    continue;
                }
                // Bulges: verbatim forward, negated under reversal —
                // bit-exact either way.
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
                let mut mapped: Vec<usize> = vl.tangent_joints().iter().map(|&j| vmap(j)).collect();
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

/// **A program's naming anchor, from its replayed loops**: validate
/// them (at the conventional plane — validation is 2-D and the anchor
/// is loop-derived) and bit-match the canonical form against them, the
/// derivation the evaluation's pre-pass makes. `None` where the loops
/// do not validate, or the match fails.
pub(crate) fn naming_of(loops: &[ProfileLoop<f64>], tol: geom_core::Tol) -> Option<ProfileNaming> {
    let validated = Profile::new(profile::SketchPlane::xy(), loops.to_vec())
        .validate(tol)
        .ok()?;
    derive_naming(&validated, loops)
}

/// **A naming anchor read off a replay alone** — for a program that
/// replays but does not validate, whose names were published by an
/// earlier evaluation that did. One entry per CANONICAL loop, `None`
/// where that loop's anchor cannot be read; `None` overall where the
/// canonical loop ORDER cannot.
///
/// The canonical form keeps each loop's authored start, so a loop's
/// anchor is two facts: which canonical loop it is, and whether
/// canonicalization reverses it. Both are read here without the
/// validation the program fails:
///
/// - **Order.** The canonical order is the outer loop first, then the
///   holes in authored order. Validation's outer loop contains every
///   other, so on a profile that validates it is the unique loop of
///   largest enclosed area — the rule read here. Two loops tied for
///   the largest area (or none with any) give no order, and every name
///   strands.
/// - **Orientation.** The loop's signed enclosed area — shoelace plus
///   each arc's circular segment `(r²/2)(θ − sin θ)`, `θ = 4·atan(b)`,
///   the quantity validation's `loop_orientation` classifies — with
///   the outer loop canonically counter-clockwise and holes clockwise.
///   An area that is exactly zero decides no sense, and that loop's
///   names strand.
///
/// On a program that validates this agrees with [`naming_of`]: the SetProgram door
/// asserts so on every new program it admits.
pub(crate) fn replay_naming(loops: &[ProfileLoop<f64>]) -> Option<Vec<Option<LoopAnchor>>> {
    let areas: Vec<f64> = loops.iter().map(signed_area).collect();
    let largest = areas.iter().map(|a| a.abs()).fold(0.0_f64, f64::max);
    let mut at_largest = (0..loops.len()).filter(|&i| areas[i].abs() == largest);
    let outer = at_largest.next()?;
    if largest == 0.0 || at_largest.next().is_some() || !largest.is_finite() {
        return None;
    }
    let order = core::iter::once(outer).chain((0..loops.len()).filter(|&i| i != outer));
    Some(
        order
            .map(|li| {
                let a = areas[li];
                if a == 0.0 || !a.is_finite() {
                    return None;
                }
                let ccw = a > 0.0;
                Some(LoopAnchor {
                    program_loop: u32::try_from(li).ok()?,
                    reversed: ccw != (li == outer),
                    len: u32::try_from(loops[li].vertices().len()).ok()?,
                })
            })
            .collect(),
    )
}

/// The signed area a loop encloses, positive
/// counter-clockwise — the chord polygon's shoelace about the first
/// vertex plus each arc's signed circular segment.
fn signed_area(lp: &ProfileLoop<f64>) -> f64 {
    let vs = lp.vertices();
    let Some(first) = vs.first() else {
        return 0.0;
    };
    let o = first.pos();
    let n = vs.len();
    let mut twice = 0.0;
    let mut arcs = 0.0;
    for (k, v) in vs.iter().enumerate() {
        let (a, b) = (v.pos(), vs[(k + 1) % n].pos());
        twice += (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
        let bulge = v.bulge();
        if bulge != 0.0 {
            let theta = 4.0 * bulge.atan();
            let chord2 = (b.x - a.x).powi(2) + (b.y - a.y).powi(2);
            let half = (0.5 * theta).sin();
            let r2 = chord2 / (4.0 * half.powi(2));
            arcs += 0.5 * r2 * (theta - theta.sin());
        }
    }
    0.5 * twice + arcs
}

/// **A program's naming anchor as far as it can be read**, per
/// CANONICAL loop, off its replay alone ([`replay_naming`]); empty
/// where the loops cannot be ordered. Where the loops validate this IS
/// the validated anchor ([`naming_of`]) — the `SetProgram` door asserts
/// the agreement on every program it admits — so the edit doors that
/// carry names across a change read each side without paying for a
/// validation: which is what a value edit would otherwise pay per
/// profile, per edit.
pub(crate) fn readable_naming(loops: &[ProfileLoop<f64>]) -> Vec<Option<LoopAnchor>> {
    replay_naming(loops).unwrap_or_default()
}
