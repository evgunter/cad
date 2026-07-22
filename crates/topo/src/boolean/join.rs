//! Boolean joining (ch. 15 §15.7, Programs 15.13/15.14): the ch. 14
//! join skeleton (PR 3's [`ChordJoiner`]) driven **in lockstep across
//! both solids**, with `scanjoin`'s dual requirement — a candidate
//! joins only where a topological neighbor exists in BOTH solids at
//! the SAME loose end — realized through PR 4's explicit
//! [`NullEdgePairRecord`](super::NullEdgePairRecord) correspondence
//! keys and the **germ facings** ([`HalfGerm`]) recorded at insertion
//! (F9: correspondence as data, never correlated array order —
//! `ssortnulledges` stays engineered out).
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
/// used-in-a-join flags.
#[derive(Clone, Copy, Debug)]
struct OpenRecord<T: geom_core::Real> {
    a_edge: EdgeKey,
    b_edge: EdgeKey,
    a: [(HalfGerm<T>, bool); 2],
    b: [(HalfGerm<T>, bool); 2],
    /// Per-half used flags (indexed like the germ slots; for wild
    /// struts a germ may consume the OTHER half — the physical
    /// splice's germ facing is not trusted for empty fans).
    a_half_used: [bool; 2],
    b_half_used: [bool; 2],
    /// Whether the A / B edge is an empty-fan strut whose side labels
    /// are still provisional (bound at first use — module docs).
    a_wild: bool,
    b_wild: bool,
}

impl<T: geom_core::Real> OpenRecord<T> {
    fn fully_used(&self) -> bool {
        self.a.iter().all(|(_, u)| *u) && self.b.iter().all(|(_, u)| *u)
    }

    /// Whether this record's side labels are fixed: non-strut, or a
    /// strut that has already joined once (first use binds — module
    /// docs).
    fn a_bound(&self) -> bool {
        !self.a_wild || self.a.iter().any(|(_, u)| *u)
    }

    fn b_bound(&self) -> bool {
        !self.b_wild || self.b.iter().any(|(_, u)| *u)
    }
}

/// A resolved match: record indices plus the germ-slot choice per
/// side.
#[derive(Clone, Copy, Debug)]
struct Match {
    entry: usize,
    cand: usize,
    /// (germ slot, half slot) per record per solid.
    entry_a: (usize, usize),
    cand_a: (usize, usize),
    entry_b: (usize, usize),
    cand_b: (usize, usize),
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
    let mut a_by_edge: SecondaryMap<EdgeKey, ([HalfGerm<T>; 2], bool)> = SecondaryMap::new();
    let mut b_by_edge: SecondaryMap<EdgeKey, ([HalfGerm<T>; 2], bool)> = SecondaryMap::new();
    for r in &red.null_edges {
        match r.operand {
            Operand::A => a_by_edge.insert(r.edge, (r.germs, r.dangling)),
            Operand::B => b_by_edge.insert(r.edge, (r.germs, r.dangling)),
        };
    }
    let mut open: Vec<OpenRecord<T>> = Vec::new();
    for p in &red.null_pairs {
        let (a, a_wild) = *a_by_edge
            .get(p.a_edge)
            .ok_or(desync("pair A edge without a germ record"))?;
        let (b, b_wild) = *b_by_edge
            .get(p.b_edge)
            .ok_or(desync("pair B edge without a germ record"))?;
        open.push(OpenRecord {
            a_edge: p.a_edge,
            b_edge: p.b_edge,
            a: [(a[0], false), (a[1], false)],
            b: [(b[0], false), (b[1], false)],
            a_half_used: [false; 2],
            b_half_used: [false; 2],
            a_wild,
            b_wild,
        });
    }

    // Fixpoint (module docs).
    loop {
        let Some(m) = find_match(&open, red, &sa, &sb, band)? else {
            break;
        };
        let (ea, ra) = (
            open[m.entry].a[m.entry_a.1].0.he,
            open[m.cand].a[m.cand_a.1].0.he,
        );
        let (eb, rb) = (
            open[m.entry].b[m.entry_b.1].0.he,
            open[m.cand].b[m.cand_b.1].0.he,
        );
        // Bind provisional strut senses (module docs): the consumed
        // halves must oppose; an unbound strut relabels to comply.
        bind_senses(red, &mut sa, Operand::A, &open, m, ea, ra)?;
        bind_senses(red, &mut sb, Operand::B, &open, m, eb, rb)?;
        // Chord-run role choice per solid (module docs): the first
        // chord's mef run [h1 .. h2] must be the germ-path arc — the
        // side free of any still-loose scaffolding half (a polygon
        // edge connects ADJACENT crossings; the far arc holds the rest
        // of the seam). Swap roles when the direct arc is dirty.
        let mut a_loose = loose_siblings(&open, Operand::A);
        let mut b_loose = loose_siblings(&open, Operand::B);
        // The chosen halves themselves are being consumed.
        a_loose.remove(ea);
        a_loose.remove(ra);
        b_loose.remove(eb);
        b_loose.remove(rb);
        // Role order controls the minted chords' orientation; the
        // cross-solid relation is the book's he1 ↔ he2 crossover: the
        // B side executes ANTI-correlated with A's executed order
        // (preferring the mirrored order, yielding only to the
        // clean-arc constraint) — this is what makes the two kept
        // section loops antiparallel at the zip.
        let (a1, a2) = choose_roles(&red.a, ea, ra, &a_loose, false)?;
        let a_swapped = a1 == ra;
        let (b1, b2) = choose_roles(&red.b, eb, rb, &b_loose, a_swapped)?;
        sa.joiner
            .join(&mut red.a, a1, a2)
            .map_err(BooleanError::Join)?;
        sb.joiner
            .join(&mut red.b, b1, b2)
            .map_err(BooleanError::Join)?;
        open[m.entry].a[m.entry_a.0].1 = true;
        open[m.entry].b[m.entry_b.0].1 = true;
        open[m.cand].a[m.cand_a.0].1 = true;
        open[m.cand].b[m.cand_b.0].1 = true;
        open[m.entry].a_half_used[m.entry_a.1] = true;
        open[m.entry].b_half_used[m.entry_b.1] = true;
        open[m.cand].a_half_used[m.cand_a.1] = true;
        open[m.cand].b_half_used[m.cand_b.1] = true;
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
/// combinations whose A-side and B-side germs all carry the SAME face
/// pair and whose two sites mutually FACE each other along the germ
/// line (`bool_join_facing`, decided — the polygon edge's ends point
/// at one another), pick the NEAREST pair of sites
/// (`bool_join_nearest`, decided — non-adjacent same-line sites must
/// not be chorded across an intermediate one). Zero-distance
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
    // (germ slot, half slot) options per record side: bound records
    // tie half to germ; unbound struts offer every combination (the
    // physical splice's germ facing is provisional for empty fans —
    // the arc test below resolves it structurally).
    fn options<T: geom_core::Real>(
        side: &[(HalfGerm<T>, bool); 2],
        half_used: &[bool; 2],
        bound: bool,
    ) -> Vec<(usize, usize)> {
        // Half tied to germ slot (the mint facing); wildness affects
        // only sense binding, never which half serves which germ.
        let _ = bound;
        let mut out = Vec::new();
        for g in 0..2 {
            if !side[g].1 && !half_used[g] {
                out.push((g, g));
            }
        }
        out
    }
    let mut best: Option<(T, Match)> = None;
    for (cand, rec) in open.iter().enumerate() {
        for (entry, e) in open.iter().enumerate() {
            if entry == cand {
                continue;
            }
            for &(cga, cha) in &options(&rec.a, &rec.a_half_used, rec.a_bound()) {
                let rga = rec.a[cga].0;
                let r_he = rec.a[cha].0.he;
                for &(ega_slot, eha) in &options(&e.a, &e.a_half_used, e.a_bound()) {
                    let ega = e.a[ega_slot].0;
                    let e_he = e.a[eha].0.he;
                    if ega.a_face != rga.a_face || ega.b_face != rga.b_face {
                        continue;
                    }
                    if e.a_bound()
                        && rec.a_bound()
                        && sa.is_up(&red.a, e_he)? == sa.is_up(&red.a, r_he)?
                    {
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
                        || decide("bool_join_facing", f2, band).map_err(escalate)?
                            != Sign::Positive
                    {
                        continue;
                    }
                    for &(cgb, chb) in &options(&rec.b, &rec.b_half_used, rec.b_bound()) {
                        let rgb = rec.b[cgb].0;
                        let rb_he = rec.b[chb].0.he;
                        if rgb.a_face != rga.a_face || rgb.b_face != rga.b_face {
                            continue;
                        }
                        for &(egb_slot, ehb) in &options(&e.b, &e.b_half_used, e.b_bound()) {
                            let egb = e.b[egb_slot].0;
                            let eb_he = e.b[ehb].0.he;
                            if egb.a_face != rgb.a_face || egb.b_face != rgb.b_face {
                                continue;
                            }
                            if e.b_bound()
                                && rec.b_bound()
                                && sb.is_up(&red.b, eb_he)? == sb.is_up(&red.b, rb_he)?
                            {
                                continue;
                            }
                            let m = Match {
                                entry,
                                cand,
                                entry_a: (ega_slot, eha),
                                cand_a: (cga, cha),
                                entry_b: (egb_slot, ehb),
                                cand_b: (cgb, chb),
                            };
                            best = match best {
                                None => Some((dist, m)),
                                Some((bd, bm)) => {
                                    match decide("bool_join_nearest", dist - bd, band)
                                        .map_err(escalate)?
                                    {
                                        Sign::Negative => Some((dist, m)),
                                        _ => Some((bd, bm)),
                                    }
                                }
                            };
                        }
                    }
                }
            }
        }
    }
    Ok(best.map(|(_, m)| m))
}

/// Still-unused null-edge halves of one solid, each mapped to its
/// record's OTHER loose half (or None when it is the record's last).
fn loose_siblings<T: geom_core::Real>(
    open: &[OpenRecord<T>],
    operand: Operand,
) -> SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>> {
    let mut set = SecondaryMap::new();
    for r in open {
        let (side, half_used) = match operand {
            Operand::A => (&r.a, &r.a_half_used),
            Operand::B => (&r.b, &r.b_half_used),
        };
        let loose: Vec<HalfEdgeKey> = (0..2)
            .filter(|&h| !half_used[h])
            .map(|h| side[h].0.he)
            .collect();
        match loose.as_slice() {
            [one] => {
                set.insert(*one, None);
            }
            [x, y] => {
                set.insert(*x, Some(*y));
                set.insert(*y, Some(*x));
            }
            _ => {}
        }
    }
    set
}

/// Chooses the join role order (module docs): keep `(ea, ra)` unless
/// the `next`-order arc `ea → ra` (the first chord's mef run) passes a
/// still-loose scaffolding half — then the germ path is the other arc,
/// so swap. Different loops keep the given order (the mekr lane). Both
/// arcs dirty is a loud desync.
fn choose_roles<T: Decide>(
    body: &Body<T>,
    ea: HalfEdgeKey,
    ra: HalfEdgeKey,
    loose: &SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>,
    prefer_swap: bool,
) -> Result<(HalfEdgeKey, HalfEdgeKey), BooleanError> {
    let (p1, p2) = if prefer_swap { (ra, ea) } else { (ea, ra) };
    clean_dir(body, p1, p2, loose)?.ok_or(BooleanError::JoinDesync {
        what: "every chord arc separates a loose scaffolding pair",
    })
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
    // A direction is BAD iff its arc SEPARATES some record's loose
    // halves (captures exactly one of a loose pair, or a record's last
    // loose half — the capture would wall it off from its partner
    // site). Capturing a complete loose pair together is harmless.
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

/// Binds provisional strut side labels at first use (module docs):
/// the two consumed halves must start at opposite-side ends; if they
/// do not, the unbound strut's labels are swapped (attribute record +
/// side sets) — fixed thereafter. Both fixed and disagreeing is a
/// loud desync.
fn bind_senses<T: Decide>(
    red: &mut BooleanReduction<T>,
    s: &mut SolidJoin,
    operand: Operand,
    open: &[OpenRecord<T>],
    m: Match,
    entry_he: HalfEdgeKey,
    cand_he: HalfEdgeKey,
) -> Result<(), BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let (entry_rec, cand_rec) = (&open[m.entry], &open[m.cand]);
    let (entry_bound, cand_bound, entry_edge, cand_edge) = match operand {
        Operand::A => (
            entry_rec.a_bound(),
            cand_rec.a_bound(),
            entry_rec.a_edge,
            cand_rec.a_edge,
        ),
        Operand::B => (
            entry_rec.b_bound(),
            cand_rec.b_bound(),
            entry_rec.b_edge,
            cand_rec.b_edge,
        ),
    };
    let body = match operand {
        Operand::A => &red.a,
        Operand::B => &red.b,
    };
    let entry_up = s.is_up(body, entry_he)?;
    let cand_up = s.is_up(body, cand_he)?;
    if cand_up != entry_up {
        return Ok(()); // already opposed
    }
    if !cand_bound {
        relabel_strut(red, s, operand, cand_edge)
    } else if !entry_bound {
        relabel_strut(red, s, operand, entry_edge)
    } else {
        Err(desync("bound senses agree at a matched pair"))
    }
}

/// Swaps a provisional strut's below/above labels (record + sets).
fn relabel_strut<T: Decide>(
    red: &mut BooleanReduction<T>,
    s: &mut SolidJoin,
    operand: Operand,
    edge: EdgeKey,
) -> Result<(), BooleanError> {
    let rec = red
        .null_edges
        .iter_mut()
        .find(|r| r.operand == operand && r.edge == edge)
        .ok_or(BooleanError::JoinDesync {
            what: "relabel target edge has no record",
        })?;
    let (u, v) = (rec.attr.below_end, rec.attr.above_end);
    rec.attr.below_end = v;
    rec.attr.above_end = u;
    s.in_set.remove(u);
    s.in_set.insert(v, ());
    s.out_set.remove(v);
    s.out_set.insert(u, ());
    Ok(())
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
            if let Some(ring_in) = probe(ring)? {
                if ring_in == outer_in {
                    return Err(BooleanError::Join(SplitJoinError::SectionLoopMixed {
                        face,
                    }));
                }
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

