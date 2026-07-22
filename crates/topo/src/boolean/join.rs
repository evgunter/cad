//! Boolean joining (ch. 15 §15.7, Programs 15.13/15.14): the ch. 14
//! join skeleton (PR 3's [`ChordJoiner`]) driven **in lockstep across
//! both solids**, with `scanjoin`'s dual requirement — a candidate
//! joins only where a topological neighbor exists in BOTH solids at
//! the SAME loose end — realized through PR 4's explicit
//! [`NullEdgePairRecord`](super::NullEdgePairRecord) correspondence
//! keys and the **germ facings** ([`HalfGerm`]) recorded at insertion
//! (F9: correspondence as data, never correlated array order — the
//! book's `ssortnulledges` ordering/orientation discipline is
//! enforced as the derived sense data below, not as a sort).
//!
//! # Matching is germ identity, not slots or senses (the below-copy
//! # audit)
//!
//! A section-polygon edge lies on the intersection line of one A-face
//! and one B-face; its two end sites offer null-edge halves *facing*
//! that germ. The neighbor test is therefore pure data:
//!
//! - the two halves' germ **face pairs agree** (both components — this
//!   subsumes the book's same-face test and is immune to mid-join face
//!   divisions);
//! - their record **parities oppose** (the book's "opposite he1/he2
//!   roles" — 15.11's IN-record→OUT-record orientation carried as
//!   data);
//! - the A-side and B-side germs of one match are the SAME face pair
//!   (one spatial polygon edge).
//!
//! Boolean runs mint copies of BOTH parities (In-runs mint
//! `NewVertexSide::Below` copies — the PR 4 interface fact); nothing
//! here assumes above-only: polygon-completion roles are resolved by
//! membership of the IN/OUT end-vertex sets built from the F9
//! attributes (`in_copy` = the loop through IN ends), and the germ
//! facings identify halves regardless of which side was minted.
//!
//! # The seam-orientation discipline (PR 5.5 — the derived form of
//! # the book's ssortnulledges / he1↔he2 crossover)
//!
//! Derived from the ratified conventions (outward normals, loops
//! CCW-from-outside: a half-edge with tangent `t` on a face with
//! normal `n` has interior to its LEFT, `n×t` pointing in), each step
//! mirror-checked in the M3-LOG PR 5.5 record:
//!
//! 1. **Required end state.** On the germ line of face pair (fA, fB),
//!    the boundary of fA's region inside B runs `tA(in) = nA×nB`
//!    (check: `nA×(nA×nB) ∝ proj(−nB)`, the into-B direction);
//!    `tA(out)`, `tB(in)`, `tB(out)` follow by the A↔B and in↔out
//!    mirrors, giving `tA(x) = −tB(x)`. Section loops are mates of
//!    region boundaries, so for every op — ∩ (IN,IN), ∪ (OUT,OUT),
//!    ∖ (A-OUT, revert(B-IN)) — the kept loops are antiparallel at
//!    the zip **iff each solid's section loops attach to its own
//!    regions geometrically-CCW-consistently**. Op-independent.
//! 2. **The sense theorem.** The half FACING germ `g` is UP (starts
//!    at `below_end`) iff `g`'s own-solid forward-wedge code is Out;
//!    geometrically, with the orbit-forward direction `w = σ·n_own×d`
//!    (σ the fixed orbit handedness, shared by both solids), UP ⟺
//!    `σ·det[n_own, d, n_other] > 0`. Mirror checks: a strut's two
//!    germs share the line with `d1 = −d0` ⇒ opposite senses; the two
//!    facing germs of one polygon side ⇒ opposite senses within each
//!    solid (the neighbor test); `det[nA,d,nB] = −det[nB,d,nA]` ⇒
//!    **sense_A(g) = ¬sense_B(g) at every germ** — the cross-solid
//!    anti-correlation. Insertion mints attributes to this rule
//!    (struts included — their facing swap swaps the labels with it),
//!    so the attributes ARE the discipline; nothing rebinds later.
//! 3. **What the join controls.** Surgery never reverses existing
//!    halves, and chords close cycles forced by arc endpoints, so the
//!    directed cycles after every join are fixed by the senses alone:
//!    per polygon side the IN copies are chorded `up-site → down-site`
//!    on the region side (that direction CCW-bounds the IN region —
//!    the same determinant as step 2), and the section face receives
//!    the antiparallel copies. Role order moves only FACE identity:
//!    which cycle becomes the mef's new face vs stays with the old.
//!    That is orientation-neutral for outer-loop splits and mekr
//!    merges, and load-bearing exactly for RING splits, where the
//!    remainder becomes the old face's ring (a hole boundary must
//!    anti-enclose): the run must take the cycle opposite the face's
//!    residual-material side — [`choose_roles`]' derived rule.
//! 4. **Consistency theorem.** With (2) as data, (3)'s ring rule per
//!    solid, and matching that consumes the SAME germ in both solids
//!    ([`find_match`]'s slot lock), every completed polygon pair has
//!    A's IN loop antiparallel to B's IN loop, and the zip assertion
//!    ([`BooleanError::SeamOrientation`]) is a theorem with a runtime
//!    witness, not a hope.
//!
//! # The fixpoint sweep and lockstep discipline
//!
//! All pair records register up front; the sweep repeatedly executes
//! the first valid match in deterministic scan order until quiescent
//! (joins can unlock others — order sensitivity of a single greedy
//! pass is real). Joins, retirements, and completions must occur in
//! BOTH solids together; any divergence is the typed
//! [`BooleanError::JoinDesync`] refusal, never a silent mis-join.
//! There is no geometric sort and no section-area certification here:
//! boolean intersection polygons are in general non-planar (§15.7);
//! degenerate results are netted at the component stage instead.

use geom_core::{Band, Decide};
use slotmap::SecondaryMap;

use super::{BooleanError, BooleanReduction, HalfGerm, Operand};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopKey, VertexKey};
use crate::null::NullFacePair;
use crate::splitting::join::{ChordJoiner, CutOutcome, SplitJoinError};
use crate::validate::decide;

/// One completed section-polygon **pair**: the 2-loop null face in
/// each solid, with the loop roles as F9 data (IN copy = the loop
/// through the IN-side ends).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompletedPolygonPair {
    /// The A-clone null face.
    pub a_face: FaceKey,
    /// A's IN-copy loop.
    pub a_in_loop: LoopKey,
    /// A's OUT-copy loop.
    pub a_out_loop: LoopKey,
    /// The B-clone null face.
    pub b_face: FaceKey,
    /// B's IN-copy loop.
    pub b_in_loop: LoopKey,
    /// B's OUT-copy loop.
    pub b_out_loop: LoopKey,
}

/// Per-solid joining state: the shared chord core plus the F9 side
/// data role resolution reads.
struct SolidJoin {
    joiner: ChordJoiner,
    /// IN-side end vertices (below ends), from the attributes.
    in_set: SecondaryMap<VertexKey, ()>,
    /// OUT-side end vertices (above ends).
    out_set: SecondaryMap<VertexKey, ()>,
}

impl SolidJoin {
    fn new<T: Decide>(red: &BooleanReduction<T>, operand: Operand, band: Band) -> Self {
        let mut in_set = SecondaryMap::new();
        let mut out_set = SecondaryMap::new();
        for r in red.null_edges_of(operand) {
            in_set.insert(r.attr.below_end, ());
            out_set.insert(r.attr.above_end, ());
        }
        Self {
            joiner: ChordJoiner::new(band),
            in_set,
            out_set,
        }
    }

    /// The up/down sense of a null-edge half — `start ∈ in_set` ⇒ up
    /// (the below-copy audit: side is attribute data, either parity of
    /// copy resolves here).
    fn is_up<T: Decide>(&self, body: &Body<T>, he: HalfEdgeKey) -> Result<bool, BooleanError> {
        let desync = |what| BooleanError::JoinDesync { what };
        let start = body
            .get_half_edge(he)
            .ok_or(desync("half no longer resolves"))?
            .start;
        if self.in_set.contains_key(start) {
            Ok(true)
        } else if self.out_set.contains_key(start) {
            Ok(false)
        } else {
            Err(desync("null half starts at a vertex of neither side set"))
        }
    }
}

/// A completed 2-loop null-face pair before role resolution.
#[derive(Clone, Copy, Debug)]
struct UnresolvedPair {
    a_face: FaceKey,
    a_outer: LoopKey,
    a_ring: LoopKey,
    b_face: FaceKey,
    b_outer: LoopKey,
    b_ring: LoopKey,
}

/// One registered pair record: each solid's two germ facings with
/// used-in-a-join flags. Slot `i` of `a` and slot `i` of `b` are the
/// SAME spatial germ (minted from one crossing-record pair with shared
/// face-pair/direction meta) — the F9 correspondence the match walks.
#[derive(Clone, Copy, Debug)]
struct OpenRecord<T: geom_core::Real> {
    a_edge: EdgeKey,
    b_edge: EdgeKey,
    a: [(HalfGerm<T>, bool); 2],
    b: [(HalfGerm<T>, bool); 2],
}

impl<T: geom_core::Real> OpenRecord<T> {
    fn fully_used(&self) -> bool {
        self.a.iter().all(|(_, u)| *u) && self.b.iter().all(|(_, u)| *u)
    }
}

/// A resolved match: record indices plus the germ slot per record —
/// ONE slot each, consumed in BOTH solids (germ identity is shared
/// data; per-solid slot freedom was the R2 desync soup).
#[derive(Clone, Copy, Debug)]
struct Match {
    entry: usize,
    cand: usize,
    entry_slot: usize,
    cand_slot: usize,
}

/// The lockstep joining sweep (module docs). Mutates both annotated
/// clones in `red` in place; returns the completed polygon pairs in
/// completion order, with [`NullFacePair::Boolean`] records set.
pub(super) fn bool_connect<T: Decide>(
    red: &mut BooleanReduction<T>,
    a_pristine: &Body<T>,
    b_pristine: &Body<T>,
    band: Band,
) -> Result<Vec<CompletedPolygonPair>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let mut sa = SolidJoin::new(red, Operand::A, band);
    let mut sb = SolidJoin::new(red, Operand::B, band);
    let mut completed: Vec<UnresolvedPair> = Vec::new();

    // Register every pair record up front (germ facings from the
    // insertion records).
    // Per-operand germ maps (edge keys are body-lineage-scoped — one
    // map across both bodies would collide).
    let mut a_by_edge: SecondaryMap<EdgeKey, [HalfGerm<T>; 2]> = SecondaryMap::new();
    let mut b_by_edge: SecondaryMap<EdgeKey, [HalfGerm<T>; 2]> = SecondaryMap::new();
    for r in &red.null_edges {
        match r.operand {
            Operand::A => a_by_edge.insert(r.edge, r.germs),
            Operand::B => b_by_edge.insert(r.edge, r.germs),
        };
    }
    let mut open: Vec<OpenRecord<T>> = Vec::new();
    for p in &red.null_pairs {
        let a = *a_by_edge
            .get(p.a_edge)
            .ok_or(desync("pair A edge without a germ record"))?;
        let b = *b_by_edge
            .get(p.b_edge)
            .ok_or(desync("pair B edge without a germ record"))?;
        open.push(OpenRecord {
            a_edge: p.a_edge,
            b_edge: p.b_edge,
            a: [(a[0], false), (a[1], false)],
            b: [(b[0], false), (b[1], false)],
        });
    }

    // Fixpoint (module docs).
    while let Some(m) = find_match(&open, red, &sa, &sb, band)? {
        let (ea, ra) = (
            open[m.entry].a[m.entry_slot].0.he,
            open[m.cand].a[m.cand_slot].0.he,
        );
        let (eb, rb) = (
            open[m.entry].b[m.entry_slot].0.he,
            open[m.cand].b[m.cand_slot].0.he,
        );
        // Still-loose halves (choose_roles' separation constraint —
        // a role order must not wall a pending half off from its match
        // partner). The chosen halves themselves are being consumed.
        let (mut a_loose, mut b_loose) = loose_partners(&open, red, band)?;
        a_loose.remove(ea);
        a_loose.remove(ra);
        b_loose.remove(eb);
        b_loose.remove(rb);
        // Role order per solid, derived independently (module docs —
        // the PR 5.5 discipline): cross-solid seam orientation is
        // carried by the sense attributes alone; role order only
        // decides the face partition of a same-loop split, which each
        // solid resolves against its OWN geometry.
        let (a1, a2) = choose_roles(&red.a, b_pristine, &sa, ea, ra, &a_loose, band)?;
        let (b1, b2) = choose_roles(&red.b, a_pristine, &sb, eb, rb, &b_loose, band)?;
        sa.joiner
            .join(&mut red.a, a1, a2)
            .map_err(BooleanError::Join)?;
        sb.joiner
            .join(&mut red.b, b1, b2)
            .map_err(BooleanError::Join)?;
        open[m.entry].a[m.entry_slot].1 = true;
        open[m.entry].b[m.entry_slot].1 = true;
        open[m.cand].a[m.cand_slot].1 = true;
        open[m.cand].b[m.cand_slot].1 = true;
        // Retire fully-used records (higher index first: removal must
        // not shift the other's index).
        let mut done: Vec<usize> = [m.entry, m.cand]
            .into_iter()
            .filter(|&i| open[i].fully_used())
            .collect();
        done.sort_unstable_by(|x, y| y.cmp(x));
        for i in done {
            let r = open.remove(i);
            cut_pair(red, &mut sa, &mut sb, &mut completed, r.a_edge, r.b_edge)?;
        }
    }

    // ---- Role resolution, deferred to quiescence: mid-join the
    // region faces are not yet final (an operand face pierced by TWO
    // polygons still spans both seams while the first completes), so
    // probing happens only after every polygon has been cut. ----
    let mut resolved = Vec::with_capacity(completed.len());
    for c in completed {
        let (a_in_loop, a_out_loop) =
            resolve_roles_geometric(&red.a, b_pristine, c.a_face, c.a_outer, c.a_ring, band)?;
        let (b_in_loop, b_out_loop) =
            resolve_roles_geometric(&red.b, a_pristine, c.b_face, c.b_outer, c.b_ring, band)?;
        red.a.set_null_face_pair(
            c.a_face,
            NullFacePair::Boolean {
                in_copy: a_in_loop,
                out_copy: a_out_loop,
            },
        )?;
        red.b.set_null_face_pair(
            c.b_face,
            NullFacePair::Boolean {
                in_copy: b_in_loop,
                out_copy: b_out_loop,
            },
        )?;
        resolved.push(CompletedPolygonPair {
            a_face: c.a_face,
            a_in_loop,
            a_out_loop,
            b_face: c.b_face,
            b_in_loop,
            b_out_loop,
        });
    }
    let completed = resolved;

    let leftovers: usize = open
        .iter()
        .map(|r| r.a.iter().filter(|(_, u)| !u).count())
        .sum();
    if leftovers != 0 {
        #[cfg(feature = "dbg-join")]
        for r in &open {
            for (g, used) in r.a.iter().chain(r.b.iter()) {
                if !used {
                    eprintln!(
                        "loose germ: he {:?} faces ({:?},{:?}) dir ({:?},{:?},{:?})",
                        g.he, g.a_face, g.b_face, g.dir.x, g.dir.y, g.dir.z
                    );
                }
            }
        }
        return Err(BooleanError::Join(SplitJoinError::UnpairedLooseEnds {
            count: leftovers,
        }));
    }
    Ok(completed)
}

/// `scanjoin`, germ form (module docs): among all candidate/entry slot
/// combinations whose A-side germs carry the SAME face pair and whose
/// two sites mutually FACE each other along the germ line
/// (`bool_join_facing`, decided — the polygon edge's ends point at one
/// another), with OPPOSED senses in both solids (the sense theorem's
/// neighbor test), pick the NEAREST pair of sites (`bool_join_nearest`,
/// decided — non-adjacent same-line sites must not be chorded across
/// an intermediate one). The B side consumes the SAME slots — slot `i`
/// of the A and B germ arrays is one spatial germ (registration doc);
/// a B-side face-pair or sense disagreement at matched slots is a
/// loud desync, never an alternative pairing. Zero-distance
/// combinations (distinct pair records at one coincident site) are
/// skipped. Deterministic scan order breaks exact ties (D9).
fn find_match<T: Decide>(
    open: &[OpenRecord<T>],
    red: &BooleanReduction<T>,
    sa: &SolidJoin,
    sb: &SolidJoin,
    band: Band,
) -> Result<Option<Match>, BooleanError> {
    use geom_core::Sign;
    let desync = |what| BooleanError::JoinDesync { what };
    let point_of = |he: HalfEdgeKey| -> Result<geom_core::Point3<T>, BooleanError> {
        let v = red
            .a
            .get_half_edge(he)
            .ok_or(desync("germ half no longer resolves"))?
            .start;
        red.a
            .get_vertex(v)
            .and_then(|vd| red.a.get_point(vd.point).copied())
            .ok_or(desync("germ vertex has no point"))
    };
    // Unused germ slots of one record side (half tied to germ slot —
    // the mint facing is honest data for every lane, struts included).
    fn slots<T: geom_core::Real>(side: &[(HalfGerm<T>, bool); 2]) -> Vec<usize> {
        (0..2).filter(|&g| !side[g].1).collect()
    }
    let mut best: Option<(T, Match)> = None;
    for (cand, rec) in open.iter().enumerate() {
        for (entry, e) in open.iter().enumerate() {
            if entry == cand {
                continue;
            }
            for &cs in &slots(&rec.a) {
                let rga = rec.a[cs].0;
                let r_he = rga.he;
                for &es in &slots(&e.a) {
                    let ega = e.a[es].0;
                    let e_he = ega.he;
                    if ega.a_face != rga.a_face || ega.b_face != rga.b_face {
                        continue;
                    }
                    if sa.is_up(&red.a, e_he)? == sa.is_up(&red.a, r_he)? {
                        continue;
                    }
                    // Mutual facing along the germ line (spatially
                    // shared between the bodies — decided on the A
                    // clone's coincident copies).
                    let p_c = point_of(r_he)?;
                    let p_e = point_of(e_he)?;
                    let chord = p_e - p_c;
                    let dist = chord.norm();
                    let escalate = |diag| BooleanError::Escalated { diag };
                    match decide("bool_join_nearest", dist, band).map_err(escalate)? {
                        Sign::Positive => {}
                        _ => continue, // coincident sites: no polygon edge
                    }
                    let f1 = rga.dir.dot(chord) / dist;
                    let f2 = ega.dir.dot(-chord) / dist;
                    if decide("bool_join_facing", f1, band).map_err(escalate)? != Sign::Positive
                        || decide("bool_join_facing", f2, band).map_err(escalate)? != Sign::Positive
                    {
                        continue;
                    }
                    // The B side at the SAME slots — mirror checks, not
                    // freedom: shared-germ face pairs and the
                    // anti-correlation theorem make disagreement a
                    // kernel bug, refused loudly.
                    let (rgb, egb) = (rec.b[cs].0, e.b[es].0);
                    if rgb.a_face != rga.a_face
                        || rgb.b_face != rga.b_face
                        || egb.a_face != ega.a_face
                        || egb.b_face != ega.b_face
                    {
                        return Err(desync("B germ face pair differs at matched slots"));
                    }
                    if sb.is_up(&red.b, egb.he)? == sb.is_up(&red.b, rgb.he)? {
                        return Err(desync("B senses agree at a matched pair"));
                    }
                    let m = Match {
                        entry,
                        cand,
                        entry_slot: es,
                        cand_slot: cs,
                    };
                    best = match best {
                        None => Some((dist, m)),
                        Some((bd, bm)) => {
                            match decide("bool_join_nearest", dist - bd, band).map_err(escalate)? {
                                Sign::Negative => Some((dist, m)),
                                _ => Some((bd, bm)),
                            }
                        }
                    };
                }
            }
        }
    }
    Ok(best.map(|(_, m)| m))
}

/// Still-unused null-edge halves, each mapped to its geometric MATCH
/// PARTNER's half in the same solid: the nearest mutually-facing loose
/// germ with the same face pair — [`find_match`]'s own criteria,
/// static in the germ geometry, so a captured partner PAIR can still
/// join (same face) while splitting a pair walls one side off. Germ
/// meta is shared between the solids, so the (record, slot) partner
/// relation is computed once (A-clone points — coincident copies) and
/// translated per solid. A loose half with no partner maps to `None`
/// (conservatively separated wherever captured).
type LooseMap = SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>;

fn loose_partners<T: Decide>(
    open: &[OpenRecord<T>],
    red: &BooleanReduction<T>,
    band: Band,
) -> Result<(LooseMap, LooseMap), BooleanError> {
    use geom_core::Sign;
    let desync = |what| BooleanError::JoinDesync { what };
    let escalate = |diag| BooleanError::Escalated { diag };
    let point_of = |he: HalfEdgeKey| -> Result<geom_core::Point3<T>, BooleanError> {
        let v = red
            .a
            .get_half_edge(he)
            .ok_or(desync("germ half no longer resolves"))?
            .start;
        red.a
            .get_vertex(v)
            .and_then(|vd| red.a.get_point(vd.point).copied())
            .ok_or(desync("germ vertex has no point"))
    };
    let loose: Vec<(usize, usize)> = open
        .iter()
        .enumerate()
        .flat_map(|(i, r)| (0..2).filter(move |&s| !r.a[s].1).map(move |s| (i, s)))
        .collect();
    let mut a_map: LooseMap = SecondaryMap::new();
    let mut b_map: LooseMap = SecondaryMap::new();
    for &(i, s) in &loose {
        let g = open[i].a[s].0;
        let p = point_of(g.he)?;
        let mut best: Option<(T, (usize, usize))> = None;
        for &(j, t) in &loose {
            if (j, t) == (i, s) {
                continue;
            }
            let g2 = open[j].a[t].0;
            if g2.a_face != g.a_face || g2.b_face != g.b_face {
                continue;
            }
            let p2 = point_of(g2.he)?;
            let chord = p2 - p;
            let dist = chord.norm();
            match decide("bool_join_nearest", dist, band).map_err(escalate)? {
                Sign::Positive => {}
                _ => continue,
            }
            let f1 = g.dir.dot(chord) / dist;
            let f2 = g2.dir.dot(-chord) / dist;
            if decide("bool_join_facing", f1, band).map_err(escalate)? != Sign::Positive
                || decide("bool_join_facing", f2, band).map_err(escalate)? != Sign::Positive
            {
                continue;
            }
            best = match best {
                None => Some((dist, (j, t))),
                Some((bd, bm)) => match decide("bool_join_nearest", dist - bd, band)
                    .map_err(escalate)?
                {
                    Sign::Negative => Some((dist, (j, t))),
                    _ => Some((bd, bm)),
                },
            };
        }
        let partner = best.map(|(_, jt)| jt);
        a_map.insert(g.he, partner.map(|(j, t)| open[j].a[t].0.he));
        b_map.insert(
            open[i].b[s].0.he,
            partner.map(|(j, t)| open[j].b[t].0.he),
        );
    }
    Ok((a_map, b_map))
}

/// Chooses the join role order for one solid (PR 5.5 — the enforced
/// discipline; module docs for the derivation). The three lanes:
///
/// - **Different loops** (the mekr lane): a pure loop merge — role
///   order is orientation-neutral; keep the given order.
/// - **Same loop, the face's OUTER**: the split partitions real
///   boundary between two faces; either partition names the same two
///   directed cycles (role order moves only face identity), so the
///   order is chosen by the clean-arc constraint alone: the first
///   chord's mef run `[h1 .. h2]` must not SEPARATE a still-loose
///   scaffolding pair (walling a pending site off from its partner).
///   Both arcs dirty is a loud desync.
/// - **Same loop, a RING of its face** (the closed seam-ring lane —
///   pierce-ring scaffolding): the split's remainder stays a ring of
///   the old face and must therefore be the cycle bounding the old
///   face's own material; the mef run takes the enclosed patch. The
///   run cycle's side is the copy-side of `end(h1)` (h1 = UP half ⇒
///   OUT cycle), so the derived order is: h1 := the UP half iff the
///   old face's residual material is IN the other body (probed on the
///   face's outer loop against the pristine other operand — the same
///   anchor `resolve_roles_geometric` uses). A derived order whose
///   run separates a loose pair is a loud desync.
///
/// Cross-solid consistency needs NO coupling of the two solids' role
/// orders: the sense attributes carry the seam orientation (the
/// anti-correlation theorem), and each solid's partition is decided
/// against its own geometry — the zip's antiparallelism assertion is
/// the runtime witness.
fn choose_roles<T: Decide>(
    body: &Body<T>,
    other_pristine: &Body<T>,
    s: &SolidJoin,
    ea: HalfEdgeKey,
    ra: HalfEdgeKey,
    loose: &SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>,
    band: Band,
) -> Result<(HalfEdgeKey, HalfEdgeKey), BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let loop_of = |he: HalfEdgeKey| -> Result<LoopKey, BooleanError> {
        Ok(body
            .get_half_edge(he)
            .ok_or(desync("role half no longer resolves"))?
            .parent_loop)
    };
    let l = loop_of(ea)?;
    if l != loop_of(ra)? {
        return Ok((ea, ra)); // mekr lane
    }
    let face = body
        .get_loop(l)
        .ok_or(desync("role loop no longer resolves"))?
        .face;
    let outer = body
        .get_face(face)
        .ok_or(desync("role face no longer resolves"))?
        .outer;
    if l == outer {
        let picked = clean_dir(body, ea, ra, loose)?;
        if picked.is_none() && std::env::var("SEAM_DEBUG").is_ok() {
            let cyc = body
                .get_loop(l)
                .and_then(|ld| match ld.boundary {
                    crate::entity::LoopBoundary::Cycle { first } => body.loop_cycle(first),
                    _ => None,
                })
                .unwrap_or_default();
            eprintln!("BOTH-DIRTY outer split: ea={ea:?} ra={ra:?}");
            for h in cyc {
                let v = body.get_half_edge(h).map(|d| d.start);
                let p = v
                    .and_then(|v| body.get_vertex(v))
                    .and_then(|vd| body.get_point(vd.point));
                eprintln!(
                    "  he {h:?} loose={:?} chosen={} p={p:?}",
                    loose.get(h).copied(),
                    h == ea || h == ra,
                );
            }
        }
        return picked.ok_or(desync(
            "every chord arc separates a loose scaffolding pair",
        ));
    }
    // Ring lane: derived order from the residual-material side.
    let residual_in = residual_side_in(body, other_pristine, s, face, band)?;
    let h1_up = residual_in;
    let (h1, h2) = if s.is_up(body, ea)? == h1_up {
        (ea, ra)
    } else {
        (ra, ea)
    };
    match clean_dir(body, h1, h2, loose)? {
        Some((c1, _)) if c1 == h1 => Ok((h1, h2)),
        _ => Err(desync(
            "derived ring role order separates a loose scaffolding pair",
        )),
    }
}

/// Whether the residual material of `face` (the region holding its
/// outer boundary) lies INSIDE the other body: the first definitive
/// [`point_in_solid`] verdict walking the outer loop's non-scaffolding
/// vertices in cycle order (D9-deterministic; seam copies are skipped
/// by side-set membership, contact vertices land `OnBoundary` and are
/// skipped by the trilean itself). No classifiable vertex is a loud
/// desync.
fn residual_side_in<T: Decide>(
    body: &Body<T>,
    other_pristine: &Body<T>,
    s: &SolidJoin,
    face: FaceKey,
    band: Band,
) -> Result<bool, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let outer = body
        .get_face(face)
        .ok_or(desync("ring-lane face no longer resolves"))?
        .outer;
    let crate::entity::LoopBoundary::Cycle { first } = body
        .get_loop(outer)
        .ok_or(desync("ring-lane outer loop no longer resolves"))?
        .boundary
    else {
        return Err(desync("ring-lane outer loop is empty"));
    };
    for he in body
        .loop_cycle(first)
        .ok_or(desync("ring-lane outer loop not walkable"))?
    {
        let v = body
            .get_half_edge(he)
            .ok_or(desync("ring-lane outer half no longer resolves"))?
            .start;
        if s.in_set.contains_key(v) || s.out_set.contains_key(v) {
            continue;
        }
        let p = body
            .get_vertex(v)
            .and_then(|vd| body.get_point(vd.point).copied())
            .ok_or(desync("ring-lane outer vertex has no point"))?;
        match super::solid_contain::point_in_solid(other_pristine, p, band)
            .map_err(BooleanError::Containment)?
        {
            super::solid_contain::SolidContainment::In => return Ok(true),
            super::solid_contain::SolidContainment::Out => return Ok(false),
            super::solid_contain::SolidContainment::OnBoundary => continue,
        }
    }
    Err(desync(
        "ring-lane face has no classifiable residual vertex",
    ))
}

/// The clean chord-arc role order, if any (doc at [`choose_roles`]):
/// `Some((h1, h2))` such that the `next`-order arc `h1 → h2` avoids
/// every `unused` half; different loops trivially clean.
fn clean_dir<T: Decide>(
    body: &Body<T>,
    ea: HalfEdgeKey,
    ra: HalfEdgeKey,
    loose: &SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>,
) -> Result<Option<(HalfEdgeKey, HalfEdgeKey)>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let loop_of = |he: HalfEdgeKey| -> Result<crate::entity::LoopKey, BooleanError> {
        Ok(body
            .get_half_edge(he)
            .ok_or(desync("role half no longer resolves"))?
            .parent_loop)
    };
    if loop_of(ea)? != loop_of(ra)? {
        return Ok(Some((ea, ra)));
    }
    // A direction is BAD iff its arc SEPARATES a loose half from its
    // match partner (captures exactly one of a partner pair, or a
    // half with no computable partner — the capture would wall it off
    // on the new face where its partner cannot reach it). Capturing a
    // complete partner pair together is harmless: they still share a
    // face and join there.
    let separates = |from: HalfEdgeKey, to: HalfEdgeKey| -> Result<bool, BooleanError> {
        let mut inside: Vec<HalfEdgeKey> = Vec::new();
        let mut he = body
            .get_half_edge(from)
            .ok_or(desync("role arc start no longer resolves"))?
            .next;
        let mut steps = 0usize;
        while he != to {
            if loose.contains_key(he) {
                inside.push(he);
            }
            he = body
                .get_half_edge(he)
                .ok_or(desync("role arc left the loop"))?
                .next;
            steps += 1;
            if steps > body.half_edges().count() {
                return Err(desync("role arc did not close"));
            }
        }
        for &h in &inside {
            match loose.get(h).copied().flatten() {
                // Last loose half of its record: its partner is at
                // another site — separated.
                None => return Ok(true),
                Some(sib) => {
                    if !inside.contains(&sib) {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    };
    if !separates(ea, ra)? {
        Ok(Some((ea, ra)))
    } else if !separates(ra, ea)? {
        Ok(Some((ra, ea)))
    } else {
        Ok(None)
    }
}

/// Cut the corresponding null edges in both solids; completions must
/// coincide (lockstep). Roles stay UNRESOLVED here (resolved at
/// quiescence — see `bool_connect`).
fn cut_pair<T: Decide>(
    red: &mut BooleanReduction<T>,
    sa: &mut SolidJoin,
    sb: &mut SolidJoin,
    completed: &mut Vec<UnresolvedPair>,
    a_edge: EdgeKey,
    b_edge: EdgeKey,
) -> Result<(), BooleanError> {
    let a_out = sa
        .joiner
        .cut_core(&mut red.a, a_edge)
        .map_err(BooleanError::Join)?;
    let b_out = sb
        .joiner
        .cut_core(&mut red.b, b_edge)
        .map_err(BooleanError::Join)?;
    match (a_out, b_out) {
        (CutOutcome::Merged, CutOutcome::Merged) => Ok(()),
        (
            CutOutcome::Completed {
                face: a_face,
                ring: a_ring,
            },
            CutOutcome::Completed {
                face: b_face,
                ring: b_ring,
            },
        ) => {
            let desync = |what| BooleanError::JoinDesync { what };
            let a_outer = red
                .a
                .get_face(a_face)
                .ok_or(desync("completed A face no longer resolves"))?
                .outer;
            let b_outer = red
                .b
                .get_face(b_face)
                .ok_or(desync("completed B face no longer resolves"))?
                .outer;
            completed.push(UnresolvedPair {
                a_face,
                a_outer,
                a_ring,
                b_face,
                b_outer,
                b_ring,
            });
            Ok(())
        }
        _ => Err(BooleanError::JoinDesync {
            what: "one solid completed a polygon where the other merged slivers",
        }),
    }
}

/// GEOMETRIC loop-role resolution for a completed section polygon
/// (M3 PR 5, the cookie-cutter finding): a loop of the 2-loop null
/// face is the IN copy iff the region material adjacent to it (the
/// faces holding its chords' mates) lies inside the OTHER body —
/// decided by probing the region faces' NON-seam vertices with
/// [`point_in_solid`] against the pristine other operand (seam
/// vertices sit ON the other boundary and are skipped via the
/// trilean's `OnBoundary`). The two regions flank the seam, so one
/// definitive verdict fixes both roles; agreeing verdicts on both
/// loops are the loud [`SplitJoinError::SectionLoopMixed`]. This
/// never consults strut side labels — pierce-ring struts carry
/// provisional labels (PR 4's flag), and single-face seam rings have
/// no in-solid label anchor; geometry is the anchor.
fn resolve_roles_geometric<T: Decide>(
    body: &Body<T>,
    other_pristine: &Body<T>,
    face: FaceKey,
    outer: LoopKey,
    ring: LoopKey,
    band: Band,
) -> Result<(LoopKey, LoopKey), BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let probe = |l: LoopKey| -> Result<Option<bool>, BooleanError> {
        let crate::entity::LoopBoundary::Cycle { first } = body
            .get_loop(l)
            .ok_or(desync("completed section loop no longer resolves"))?
            .boundary
        else {
            return Err(desync("completed section loop is empty"));
        };
        for ch in body
            .loop_cycle(first)
            .ok_or(desync("completed section loop not walkable"))?
        {
            let mate = body.mate(ch).ok_or(desync("section half has no mate"))?;
            let region_loop = body
                .get_half_edge(mate)
                .ok_or(desync("section mate no longer resolves"))?
                .parent_loop;
            let region_face = body
                .get_loop(region_loop)
                .ok_or(desync("region loop no longer resolves"))?
                .face;
            if region_face == face {
                continue;
            }
            let f = body
                .get_face(region_face)
                .ok_or(desync("region face no longer resolves"))?;
            for rl in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
                let crate::entity::LoopBoundary::Cycle { first: rf } = body
                    .get_loop(rl)
                    .ok_or(desync("region loop no longer resolves"))?
                    .boundary
                else {
                    continue;
                };
                for rhe in body
                    .loop_cycle(rf)
                    .ok_or(desync("region loop not walkable"))?
                {
                    let v = body
                        .get_half_edge(rhe)
                        .ok_or(desync("region half no longer resolves"))?
                        .start;
                    let p = body
                        .get_vertex(v)
                        .and_then(|vd| body.get_point(vd.point).copied())
                        .ok_or(desync("region vertex has no point"))?;
                    match super::solid_contain::point_in_solid(other_pristine, p, band)
                        .map_err(BooleanError::Containment)?
                    {
                        super::solid_contain::SolidContainment::In => return Ok(Some(true)),
                        super::solid_contain::SolidContainment::Out => return Ok(Some(false)),
                        super::solid_contain::SolidContainment::OnBoundary => continue,
                    }
                }
            }
        }
        Ok(None)
    };
    let roles = match probe(outer)? {
        Some(outer_in) => {
            // The two regions flank the seam: the other loop takes the
            // opposite role (checked when it also resolves).
            if probe(ring)? == Some(outer_in) {
                return Err(BooleanError::Join(SplitJoinError::SectionLoopMixed {
                    face,
                }));
            }
            if outer_in {
                (outer, ring)
            } else {
                (ring, outer)
            }
        }
        None => match probe(ring)? {
            Some(ring_in) => {
                if ring_in {
                    (ring, outer)
                } else {
                    (outer, ring)
                }
            }
            None => {
                return Err(desync(
                    "neither section loop's regions hold a classifiable vertex",
                ));
            }
        },
    };
    Ok(roles)
}
