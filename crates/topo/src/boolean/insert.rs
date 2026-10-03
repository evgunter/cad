//! Paired null-edge insertion (Programs 15.11/15.12 motion, F9/F12).
//!
//! Surviving records — each a section-polygon edge germ with one In and
//! one Out code per side — are paired **consecutively in A-major
//! order** (the book's consumption), and each pair mints one null edge
//! in each solid spanning the orbit run between the two germs, with:
//!
//! - **F9 attributes as data**: the run's side (the shared code between
//!   the paired records) decides the minted copy's side —
//!   `NewVertexSide::Below` for an In-run (below ≙ IN), `Above` for an
//!   Out-run — derived from the F3 chain, never a he1/he2 slot.
//! - **Explicit cross-body correspondence keys**
//!   ([`super::NullEdgePairRecord`]): the A-edge and B-edge of a pair
//!   are tied by key, never by array position (`ssortnulledges` is
//!   engineered out).
//! - **The 15.11 consecutive-pairing invariant, GUARDED (F12)**: the
//!   book consumes surviving records two at a time and never argues
//!   that A-consecutive pairs are also B-consecutive for > 2 crossings.
//!   We check it: each pair must be cyclically adjacent among survivors
//!   in B-order too, and the run-side codes must agree at both ends
//!   (`r.own_start_code == r'.own_end_code` per solid). Violation ⇒
//!   typed [`super::BooleanError::PairingMismatch`] — fail-loud, never
//!   a mis-joined seam. The 4-crossing stress fixtures pin the passing
//!   cases.
//!
//! Run extraction: a germ in sector `k` transitions that sector's codes
//! `end → start` walking the array forward (array order follows the
//! orbit; the forward-crossed bound is the sector's START). The run
//! from germ r to germ r′ therefore spans sectors `r.own+1 ..= r′.own`,
//! whose orbit half-edges (deduplicated across subdivision twins) form
//! the `mev_null` fan; an empty span (both germs in one sector) is the
//! strut/dangling case (`Fan { he, he }` at the next sector's edge).
//!
//! **Plan, then mint.** [`plan_null_pairs`] takes every reading a
//! pair's null edges need (codes, pairing, germ cells and directions,
//! each run's direction) and [`mint_plan`] mints from the plan without
//! reading the orbit's geometry again. The reduction plans every vertex
//! pair before it mints any: one vertex can sit in several pairs (an
//! operand whose own contact left several vertices at one point pairs
//! each with the other operand's vertex there), and a mint moves its
//! vertex's orbit. Where several crossing pairs cut one vertex,
//! [`reconcile_shared`] turns each run that would hold another pair's
//! cut the other way round its orbit, so the runs there are disjoint
//! and no mint moves a half-edge another pair's plan read. Struts there
//! mint before any fan, each spliced past those hung earlier in its
//! corner at a lower germ ([`strut_anchor`]).

use geom_core::k_stats::NonzeroSign;
use geom_core::{Band, Decide, Margin, Sign, Vec3};

use super::sectors::{BoolSector, PairRecord, within};
use super::{
    BoolNullEdgeRecord, BooleanError, NullEdgePairRecord, Operand, PairSite, SideCode, VvContact,
};
use super::{BooleanDecision, Coincide, DeclarationRead, SelfCheck};
use crate::body::Body;
use crate::contact::BooleanCoincidence;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, VertexKey};
use crate::euler::MevSite;
use crate::null::{NewVertexSide, NullEdge};

/// Output of one vertex-pair insertion.
#[derive(Debug)]
pub(super) struct InsertOut<T: geom_core::Real> {
    /// Minted edges, both operands.
    pub edges: Vec<BoolNullEdgeRecord<T>>,
    /// The correspondence pairs.
    pub pairs: Vec<NullEdgePairRecord>,
}

/// One vertex pair through the reduction's own path: plan, reconcile,
/// mint (module docs).
#[cfg(test)]
#[allow(clippy::too_many_arguments)]
fn insert_null_pairs<T: Decide>(
    a_body: &mut Body<T>,
    b_body: &mut Body<T>,
    contact: VvContact,
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    records: &[PairRecord],
    raw: &[PairRecord],
    declared: &super::DeclaredPairs<T>,
    band: Band,
) -> Result<InsertOut<T>, BooleanError> {
    let mut plans = [plan_null_pairs(
        a_body, b_body, contact, a_sectors, b_sectors, records, raw, declared, band,
    )?];
    let orbits = [(a_sectors, b_sectors)];
    reconcile_shared(&mut plans, &orbits, a_body, b_body, band)?;
    mint_plans(a_body, b_body, &plans, &orbits, band)
}

/// One solid's side of a planned null edge: the germs it runs between,
/// in the order its run is walked.
#[derive(Clone, Copy, Debug)]
pub(super) struct SideRun<T: geom_core::Real> {
    /// The germ the run leaves from.
    from: Germ<T>,
    /// The germ that closes the run.
    to: Germ<T>,
    /// Whether `from` is the pair's second germ ([`mint_directed`]'s
    /// slot alignment reads it).
    swapped: bool,
    /// Whether another crossing pair cuts the same vertex
    /// ([`reconcile_shared`]): a strut then splices among the struts
    /// hung there by their lower germs ([`strut_anchor`]).
    shared: bool,
}

/// One vertex pair's two sector arrays, A's and B's.
pub(super) type Orbits<'a, T> = (&'a [BoolSector<T>], &'a [BoolSector<T>]);

/// A cut in a vertex's orbit: its sector entry and its direction.
type Cut<T> = (usize, Vec3<T>);

/// What the mints so far left at shared vertices
/// ([`reconcile_shared`]).
#[derive(Debug)]
pub(super) struct Hung<T: geom_core::Real> {
    /// Each strut hung at a shared vertex.
    struts: Vec<HungStrut<T>>,
    /// Each end of every null edge minted there: whether it is the
    /// below (In) end.
    ends: Vec<(Operand, VertexKey, bool)>,
}

/// A strut minted at a shared vertex.
#[derive(Debug)]
struct HungStrut<T: geom_core::Real> {
    operand: Operand,
    /// The shared vertex whose orbit its germs were read in.
    root: VertexKey,
    /// The vertex it hangs at: `root`, or the tip of the strut whose
    /// segment holds its own ([`nests`]).
    vertex: VertexKey,
    /// Its null half that starts at `vertex`; it ends at the tip.
    half: HalfEdgeKey,
    /// Its two germs in walk order ([`run_ends`]).
    lower: Cut<T>,
    upper: Cut<T>,
    /// Whether `half` faces `lower`.
    half_faces_lower: bool,
    /// Its run's side.
    side: SideCode,
}

impl<T: geom_core::Real> Default for Hung<T> {
    fn default() -> Self {
        Self {
            struts: Vec::new(),
            ends: Vec::new(),
        }
    }
}

/// The null edges of one vertex pair, every reading taken: what
/// [`mint_plan`] mints without reading the orbit's geometry again.
#[derive(Clone, Debug)]
pub(super) struct NullPlan<T: geom_core::Real> {
    /// The vertex pair.
    pub contact: VvContact,
    /// One `[A, B]` run per null-edge pair, in A-major order.
    runs: Vec<[SideRun<T>; 2]>,
}

/// Validates codes, pairs survivors and reads every germ's cells and
/// direction and each run's direction (module docs), touching neither
/// body.
#[allow(clippy::too_many_arguments)]
pub(super) fn plan_null_pairs<T: Decide>(
    a_body: &Body<T>,
    b_body: &Body<T>,
    contact: VvContact,
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    records: &[PairRecord],
    raw: &[PairRecord],
    declared: &super::DeclaredPairs<T>,
    band: Band,
) -> Result<NullPlan<T>, BooleanError> {
    // A-major order: `pair_search` mints records in it, and an edge-edge
    // germ minted after it (`recl::place_germ`) takes its place in it.
    let mut ordered: Vec<(&PairRecord, &PairRecord)> = records
        .iter()
        .zip(raw)
        .filter(|(r, _)| r.survives())
        .collect();
    ordered.sort_by_key(|(r, _)| (r.a, r.b));
    let (survivors, raw): (Vec<&PairRecord>, Vec<&PairRecord>) = ordered.into_iter().unzip();
    let mut plan = NullPlan {
        contact,
        runs: Vec::new(),
    };
    if survivors.is_empty() {
        return Ok(plan); // touching without crossing: 3′ contact only
    }
    if !survivors.len().is_multiple_of(2) {
        return Err(BooleanError::ClassificationInvariant {
            what: "odd number of surviving crossing records at a vertex pair",
        });
    }
    for r in &survivors {
        let clean = |c: (SideCode, SideCode)| {
            (c.0 == SideCode::In && c.1 == SideCode::Out)
                || (c.0 == SideCode::Out && c.1 == SideCode::In)
        };
        if !clean(r.sa) || !clean(r.sb) {
            return Err(BooleanError::ClassificationInvariant {
                what: "surviving record without one In and one Out code per side",
            });
        }
    }
    // Survivors are now in A-major order (the sort above); pair consecutively
    // (cyclically, starting at the first survivor).
    let mismatch = || BooleanError::PairingMismatch {
        a_vertex: contact.a,
        b_vertex: contact.b,
    };
    // B-order of survivors for the adjacency guard.
    let mut b_order: Vec<usize> = (0..survivors.len()).collect();
    b_order.sort_by_key(|&i| (survivors[i].b, survivors[i].a));
    let b_pos = |i: usize| b_order.iter().position(|&j| j == i).unwrap_or(usize::MAX);
    // F12 guard 1: B-cyclic adjacency of each pair among survivors,
    // checked for every pair before the first mint.
    let n = survivors.len();
    for pair_idx in 0..n / 2 {
        let (p0, p1) = (b_pos(2 * pair_idx), b_pos(2 * pair_idx + 1));
        if (p0 + 1) % n != p1 && (p1 + 1) % n != p0 {
            return Err(mismatch());
        }
    }
    // Every germ's cells are read before the first mint moves an orbit.
    let loci = survivors
        .iter()
        .zip(&raw)
        .map(|(r, w)| {
            Ok((
                super::sectors::germ_locus(a_body, &a_sectors[r.a], w.sa)?,
                super::sectors::germ_locus(b_body, &b_sectors[r.b], w.sb)?,
            ))
        })
        .collect::<Result<Vec<_>, BooleanError>>()?;

    for pair_idx in 0..survivors.len() / 2 {
        let (i0, i1) = (2 * pair_idx, 2 * pair_idx + 1);
        let (r0, r1) = (survivors[i0], survivors[i1]);
        // Which forward run is the corner's wedge is decided by DATA,
        // not index parity (ambiguous at two survivors): default to the
        // forward run r0 → r1 (the book's consumption order); if that
        // run would swallow the entire orbit — impossible for the true
        // wedge, whose far side the complementary germ bounds — the
        // wedge is the other direction (r1 → r0). Applied per solid;
        // the run-side agreement guard runs against whichever
        // direction is chosen.
        let g0_faces = ((a_sectors[r0.a].face, b_sectors[r0.b].face), loci[i0]);
        let g1_faces = ((a_sectors[r1.a].face, b_sectors[r1.b].face), loci[i1]);
        // A germ along an edge of BOTH solids runs along that common
        // edge: its two flankers may be coplanar (an edge-edge germ is
        // the pair of the two solids' own fold flankers), so the planes'
        // intersection is not its direction; the A flanker's bound read
        // On is.
        let record_dir = |i: usize, r: &PairRecord| match loci[i] {
            (super::Locus::OnEdge(_), super::Locus::OnEdge(_)) => {
                let s = &a_sectors[r.a];
                let bound = if raw[i].sa.0 == SideCode::On {
                    s.start
                } else {
                    s.end
                };
                Ok(bound.normalize())
            }
            _ => record_germ_dir(
                a_body,
                b_body,
                &a_sectors[r.a],
                &b_sectors[r.b],
                declared,
                band,
            ),
        };
        let g0_dir = record_dir(i0, r0)?;
        let g1_dir = record_dir(i1, r1)?;
        let side = |sectors: &[BoolSector<T>], body: &Body<T>, g0: Germ<T>, g1: Germ<T>| {
            let swapped = run_degenerates(body, sectors, g0.0, g1.0)?;
            Ok::<_, BooleanError>(SideRun::directed(g0, g1, swapped))
        };
        plan.runs.push([
            side(
                a_sectors,
                a_body,
                (r0.a, r0.sa, g0_faces, g0_dir),
                (r1.a, r1.sa, g1_faces, g1_dir),
            )?,
            side(
                b_sectors,
                b_body,
                (r0.b, r0.sb, g0_faces, g0_dir),
                (r1.b, r1.sb, g1_faces, g1_dir),
            )?,
        ]);
    }
    Ok(plan)
}

impl<T: geom_core::Real> SideRun<T> {
    fn directed(g0: Germ<T>, g1: Germ<T>, swapped: bool) -> Self {
        let (from, to) = if swapped { (g1, g0) } else { (g0, g1) };
        Self {
            from,
            to,
            swapped,
            shared: false,
        }
    }

    /// The same null edge run the other way round its orbit.
    fn reversed(self) -> Self {
        Self {
            from: self.to,
            to: self.from,
            swapped: !self.swapped,
            shared: self.shared,
        }
    }
}

/// Mints every plan's null edges, each solid in its own order: at a
/// shared vertex its struts before any fan, each strut after every
/// strut whose segment holds its own ([`nests`]), so it can hang at
/// that strut's tip.
pub(super) fn mint_plans<T: Decide>(
    a_body: &mut Body<T>,
    b_body: &mut Body<T>,
    plans: &[NullPlan<T>],
    orbits: &[Orbits<'_, T>],
    band: Band,
) -> Result<InsertOut<T>, BooleanError> {
    let mut out = InsertOut {
        edges: Vec::new(),
        pairs: Vec::new(),
    };
    let runs: Vec<(usize, usize)> = plans
        .iter()
        .enumerate()
        .flat_map(|(i, p)| (0..p.runs.len()).map(move |k| (i, k)))
        .collect();
    let mut recs: [Vec<Option<BoolNullEdgeRecord<T>>>; 2] =
        [vec![None; runs.len()], vec![None; runs.len()]];
    for (slot, operand) in [Operand::A, Operand::B].into_iter().enumerate() {
        let vertex = |i: usize| {
            let c = plans[i].contact;
            if slot == 0 { c.a } else { c.b }
        };
        let orbit = |i: usize| if slot == 0 { orbits[i].0 } else { orbits[i].1 };
        let run = |(i, k): (usize, usize)| plans[i].runs[k][slot];
        let shared_strut = |at: (usize, usize)| -> Result<bool, BooleanError> {
            let r = run(at);
            Ok(r.shared && run_fan(orbit(at.0), r.from.0, r.to.0)?.is_empty())
        };
        let mut keyed = Vec::with_capacity(runs.len());
        for (n, &at) in runs.iter().enumerate() {
            let depth = if shared_strut(at)? {
                let mut d = 0;
                for &other in &runs {
                    if other != at
                        && vertex(other.0) == vertex(at.0)
                        && shared_strut(other)?
                        && nests(orbit(at.0), run(other), run(at), band)?
                    {
                        d += 1;
                    }
                }
                Some(d)
            } else {
                None
            };
            keyed.push((depth.is_none(), depth.unwrap_or(0), n));
        }
        keyed.sort();
        let body = if slot == 0 {
            &mut *a_body
        } else {
            &mut *b_body
        };
        let mut hung = Hung::default();
        for (.., n) in keyed {
            let (i, k) = runs[n];
            let mismatch = || BooleanError::PairingMismatch {
                a_vertex: plans[i].contact.a,
                b_vertex: plans[i].contact.b,
            };
            let rec = mint_directed(
                body,
                (operand, vertex(i)),
                orbit(i),
                plans[i].runs[k][slot],
                &mut hung,
                mismatch,
                band,
            )?;
            out.edges.push(rec.clone());
            recs[slot][n] = Some(rec);
        }
    }
    for (n, &(i, k)) in runs.iter().enumerate() {
        let [a_run, b_run] = plans[i].runs[k];
        let (Some(a_rec), Some(b_rec)) = (&recs[0][n], &recs[1][n]) else {
            return Err(BooleanError::ClassificationInvariant {
                what: "a planned null edge was not minted",
            });
        };
        out.pairs.push(NullEdgePairRecord {
            a_edge: a_rec.edge,
            b_edge: b_rec.edge,
            site: PairSite::VertexVertex(plans[i].contact),
        });
        // Slot canonicalization (the joining's slot lock): germ slot i
        // of the A and B records must be the SAME spatial germ; a
        // degeneracy swap in one solid only would misalign them, so
        // align B's array to A's (entries carry their halves — the
        // germ↔half binding is untouched).
        if a_run.swapped != b_run.swapped
            && let Some(b) = out
                .edges
                .iter_mut()
                .find(|r| r.operand == Operand::B && r.edge == b_rec.edge)
        {
            b.germs.swap(0, 1);
        }
    }
    Ok(out)
}

/// Whether `outer` and `inner` are struts and `outer`'s segment holds
/// `inner`'s whole ([`holds_whole`]).
fn nests<T: Decide>(
    secs: &[BoolSector<T>],
    outer: SideRun<T>,
    inner: SideRun<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    let strut = |r: SideRun<T>| run_fan(secs, r.from.0, r.to.0).map(|f| f.is_empty());
    let segment = |r: SideRun<T>| run_ends(secs, r, band).map(|(lo, hi)| (lo, hi, r.from.1.0));
    Ok(
        strut(outer)?
            && strut(inner)?
            && holds_whole(secs, segment(outer)?, segment(inner)?, band)?,
    )
}

/// A strut's segment: its germs in walk order ([`run_ends`]) and its
/// run's side.
type Segment<T> = (Cut<T>, Cut<T>, SideCode);

/// **Whether the strut segment `outer` holds the strut segment `inner`
/// whole**: both in one physical sector, `inner`'s germs between
/// `outer`'s or along their directions, `outer` an Out run and `inner`
/// an In run. Another piece's boundary crosses no piece's In segment,
/// so a segment strictly inside another is always an In run inside an
/// Out run; where ends tie, two pieces touch along a pinch line in the
/// corner's face, and the sides are what decide which holds which (at
/// both ends, the two runs are one arc). The inner strut hangs at the
/// outer's tip ([`mint_plans`]), which is the Out end of the one and the
/// In end's hang vertex of the other: the same end of both.
fn holds_whole<T: Decide>(
    secs: &[BoolSector<T>],
    (lo, hi, side): Segment<T>,
    (ilo, ihi, iside): Segment<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    let not_after = |p: Cut<T>, q: Cut<T>| precedes(secs, p, q, band).map(|o| o != Some(false));
    Ok(side == SideCode::Out
        && iside == SideCode::In
        && secs[lo.0].he == secs[ilo.0].he
        && not_after(lo, ilo)?
        && not_after(ihi, hi)?)
}

/// **Several vertex pairs' null edges at one vertex.** An operand
/// whose own contact left two vertices at one point pairs both with
/// the other operand's vertex there, and when both pairs cross, both
/// cut that vertex's orbit. Each plan read the orbit before any mint,
/// and a mint keeps every half-edge outside its run where it was, so
/// each run is planned to hold no other pair's cut: the runs are then
/// disjoint, every later mint finds its run's half-edges still at the
/// vertex, and its cuts land beside the earlier mint's in the order
/// the corner reads. A run that holds one is turned the other way
/// round its orbit, every run re-read until none turns (one turn can
/// make another's nested run the one to turn); cuts along one
/// direction are placed by the runs ([`tied_held`]). A null edge both of whose runs hold another pair's
/// cut refuses [`BooleanError::SharedVertexCrossings`]: a strut (whose
/// other way round is the whole orbit) whose segment holds one, which
/// a piece with a reflex corner at the point reaches; a fan whose two
/// ways round both do, which needs a pair paired across the arcs
/// outside its piece; or a run that is another pair's arc whole. Two
/// crossing pairs that share both their vertices refuse
/// [`BooleanError::NonManifoldResult`]: the result would hold a
/// shared-entity wedge fan there.
pub(super) fn reconcile_shared<T: Decide>(
    plans: &mut [NullPlan<T>],
    sectors: &[Orbits<'_, T>],
    a_body: &Body<T>,
    b_body: &Body<T>,
    band: Band,
) -> Result<(), BooleanError> {
    // Two crossing pairs that share both their vertices: each operand
    // holds two vertices at the point, and the result's would be one
    // vertex whose orbit passes the pinch line twice.
    let crossing = |j: usize| !plans[j].runs.is_empty();
    for i in (0..plans.len()).filter(|&i| crossing(i)) {
        let c = plans[i].contact;
        let other = |same: &dyn Fn(VvContact) -> bool| {
            (0..plans.len()).find(|&j| j != i && crossing(j) && same(plans[j].contact))
        };
        if let (Some(j), Some(_)) = (other(&|d| d.a == c.a), other(&|d| d.b == c.b)) {
            return Err(BooleanError::NonManifoldResult {
                a_vertex: c.a,
                b_vertices: [c.b, plans[j].contact.b],
            });
        }
    }
    // Each pass re-reads every run against the others' current runs; a
    // pass that turns none is the fixed point.
    let limit = plans.iter().map(|p| p.runs.len()).sum::<usize>() + 1;
    for _ in 0..limit {
        if !reconcile_pass(plans, sectors, [a_body, b_body], band)? {
            return Ok(());
        }
    }
    Err(BooleanError::ClassificationInvariant {
        what: "the runs at a shared vertex do not settle",
    })
}

/// One pass of [`reconcile_shared`]: keeps each run that holds none of
/// the other pairs' cuts, else turns it the other way round when that
/// holds none, else refuses. Returns whether any run turned.
fn reconcile_pass<T: Decide>(
    plans: &mut [NullPlan<T>],
    sectors: &[Orbits<'_, T>],
    bodies: [&Body<T>; 2],
    band: Band,
) -> Result<bool, BooleanError> {
    let mut turned = false;
    for (slot, operand) in [Operand::A, Operand::B].into_iter().enumerate() {
        let body = bodies[slot];
        let key = |c: VvContact| if slot == 0 { (c.a, c.b) } else { (c.b, c.a) };
        let orbit = |i: usize| {
            if slot == 0 {
                sectors[i].0
            } else {
                sectors[i].1
            }
        };
        for i in 0..plans.len() {
            let (vertex, partner) = key(plans[i].contact);
            let others: Vec<usize> = (0..plans.len())
                .filter(|&j| j != i && !plans[j].runs.is_empty())
                .filter(|&j| key(plans[j].contact).0 == vertex)
                .collect();
            if plans[i].runs.is_empty() || others.is_empty() {
                continue;
            }
            let secs = orbit(i);
            let same_reading = |o: &[BoolSector<T>]| {
                o.len() == secs.len()
                    && o.iter()
                        .zip(secs)
                        .all(|(x, y)| x.he == y.he && x.end_edge() == y.end_edge())
            };
            if !others.iter().all(|&j| same_reading(orbit(j))) {
                return Err(BooleanError::ClassificationInvariant {
                    what: "two readings of one vertex's orbit disagree",
                });
            }
            let mut cuts: Vec<OtherCut<T>> = Vec::new();
            for &j in &others {
                for r in &plans[j].runs {
                    let (lo, hi) = run_ends(secs, r[slot], band)?;
                    let strut = run_fan(secs, r[slot].from.0, r[slot].to.0)?.is_empty();
                    let side = r[slot].from.1.0;
                    cuts.push(OtherCut {
                        at: lo,
                        mate: hi,
                        leaves: true,
                        owner: j,
                        strut,
                        side,
                    });
                    cuts.push(OtherCut {
                        at: hi,
                        mate: lo,
                        leaves: false,
                        owner: j,
                        strut,
                        side,
                    });
                }
            }
            for k in 0..plans[i].runs.len() {
                let current = plans[i].runs[k][slot];
                let mut blocker = None;
                let mut chosen = None;
                for run in [current, current.reversed()] {
                    if run_degenerates(body, secs, run.from.0, run.to.0)? {
                        continue;
                    }
                    match held_cut(secs, run, &cuts, band)? {
                        Some(j) => blocker = blocker.or(Some(j)),
                        None => {
                            chosen = Some(run);
                            break;
                        }
                    }
                }
                let chosen = chosen.ok_or_else(|| {
                    let other = blocker.map_or(partner, |j| key(plans[j].contact).1);
                    BooleanError::SharedVertexCrossings {
                        operand,
                        vertex,
                        partners: [partner, other],
                    }
                })?;
                turned |= chosen.swapped != current.swapped;
                plans[i].runs[k][slot] = SideRun {
                    shared: true,
                    ..chosen
                };
            }
        }
    }
    Ok(turned)
}

/// Another pair's cut at a shared vertex: where it lies, the other end
/// of its run, whether that run lies after it walking forward (else
/// before it), and the pair.
#[derive(Clone, Copy, Debug)]
struct OtherCut<T: geom_core::Real> {
    at: Cut<T>,
    mate: Cut<T>,
    leaves: bool,
    owner: usize,
    /// Whether its run is a strut.
    strut: bool,
    /// Its run's side.
    side: SideCode,
}

/// A run's two ends in walk order: a fan's leaving germ then its
/// closing germ, a strut's earlier germ through their one physical
/// sector then its later (its run order follows the A-major record
/// sort, not the angle). A strut whose germs lie along one direction
/// is an invariant: a pair's two crossings are distinct.
fn run_ends<T: Decide>(
    secs: &[BoolSector<T>],
    run: SideRun<T>,
    band: Band,
) -> Result<(Cut<T>, Cut<T>), BooleanError> {
    let (from, to) = ((run.from.0, run.from.3), (run.to.0, run.to.3));
    if !run_fan(secs, from.0, to.0)?.is_empty() {
        return Ok((from, to));
    }
    match precedes(secs, from, to, band)? {
        Some(true) => Ok((from, to)),
        Some(false) => Ok((to, from)),
        None => Err(BooleanError::ClassificationInvariant {
            what: "a dangling null edge whose two germs lie along one direction",
        }),
    }
}

/// The owner of the first cut `run` holds, walking its orbit forward
/// from its first end to its last: a cut in an entry the walk crosses
/// whole, or in an end entry on the run's side of that end's germ, or a
/// cut along an end germ's own direction that [`tied_held`] counts. A
/// strut holds a cut of its own physical sector between its two germs,
/// or along either that [`tied_held`] counts.
fn held_cut<T: Decide>(
    secs: &[BoolSector<T>],
    run: SideRun<T>,
    cuts: &[OtherCut<T>],
    band: Band,
) -> Result<Option<usize>, BooleanError> {
    let n = secs.len();
    let (lo, hi) = run_ends(secs, run, band)?;
    let strut = run_fan(secs, lo.0, hi.0)?.is_empty();
    let rel = |k: usize| (k + n - lo.0) % n;
    for &cut in cuts {
        let (j, d) = cut.at;
        let held = if strut {
            if secs[j].he != secs[lo.0].he || nested(secs, (lo, hi, run.from.1.0), cut, band)? {
                false
            } else {
                match (
                    precedes(secs, lo, (j, d), band)?,
                    precedes(secs, (j, d), hi, band)?,
                ) {
                    (Some(false), _) | (_, Some(false)) => false,
                    (None, _) => tied_held(secs, (lo, true), hi, cut, band)?,
                    (_, None) => tied_held(secs, (hi, false), lo, cut, band)?,
                    (Some(true), Some(true)) => true,
                }
            }
        } else if rel(j) == 0 {
            match walks_after(&secs[lo.0], lo.1, d, band)? {
                Some(after) => after,
                None => tied_held(secs, (lo, true), hi, cut, band)?,
            }
        } else if rel(j) < rel(hi.0) {
            true
        } else if rel(j) == rel(hi.0) {
            match walks_after(&secs[hi.0], d, hi.1, band)? {
                Some(before) => before,
                None => tied_held(secs, (hi, false), lo, cut, band)?,
            }
        } else {
            false
        };
        if held {
            return Ok(Some(cut.owner));
        }
    }
    Ok(None)
}

/// Whether `cut`'s run is a strut that the strut segment `own` holds
/// whole, or that holds `own` whole ([`holds_whole`]): one hangs at
/// the other's tip, and neither holds the other's cuts.
fn nested<T: Decide>(
    secs: &[BoolSector<T>],
    own: Segment<T>,
    cut: OtherCut<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    let (lo, hi) = if cut.leaves {
        (cut.at, cut.mate)
    } else {
        (cut.mate, cut.at)
    };
    let other = (lo, hi, cut.side);
    Ok(cut.strut && (holds_whole(secs, own, other, band)? || holds_whole(secs, other, own, band)?))
}

/// **Another pair's cut along one end germ's direction** (a pinch line
/// lying flat in a face of the shared corner): whether `run` holds it.
/// Each pair's run lies on the side of the line its codes, read off its
/// own piece's faces, give it, which [`run_ends`] records as after or
/// before the cut. Runs on opposite sides are disjoint: not held. Runs
/// on one side are nested: not held either, as the longer holds the
/// shorter's far cut strictly and turns ([`reconcile_shared`]'s fixed
/// point); unless both ends tie, one arc, which holds. No witness
/// reaches that arm, nor the invariant that the two directions are
/// opposite rays (a convex sector holds none).
fn tied_held<T: Decide>(
    secs: &[BoolSector<T>],
    ((k, own), own_leaves): (Cut<T>, bool),
    far: Cut<T>,
    cut: OtherCut<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    if !super::sectors::direction_sense(own, cut.at.1, secs[k].arm, band)? {
        return Err(BooleanError::ClassificationInvariant {
            what: "two cuts in one convex sector along opposite rays",
        });
    }
    if cut.leaves != own_leaves {
        return Ok(false);
    }
    Ok(cut.mate.0 == far.0 && walks_after(&secs[far.0], far.1, cut.mate.1, band)?.is_none())
}

/// Whether `y` lies after `x` walking the convex sector `s` forward,
/// from its end bound toward its start (`None`: the two lie along one
/// direction). The rotation from start to end is positive about the
/// sector's normal, the convention [`within`] reads. An in-band reading
/// refuses as the coincidence it is, as the strut order's does.
fn walks_after<T: Decide>(
    s: &BoolSector<T>,
    x: Vec3<T>,
    y: Vec3<T>,
    band: Band,
) -> Result<Option<bool>, BooleanError> {
    let m = Margin::levered(x.cross(y).dot(s.normal.vec()), s.arm);
    match crate::validate::decide("bool_shared_cut_order", m, band) {
        Ok(Sign::Negative) => Ok(Some(true)),
        Ok(Sign::Positive) => Ok(Some(false)),
        Ok(Sign::Zero) => Ok(None),
        Err(diag) => Err(BooleanError::coincidence(
            Coincide::Sectors,
            DeclarationRead::Moot,
            diag,
        )),
    }
}

/// A germ in one solid: its sector entry, its codes there, its cells
/// and its direction.
type Germ<T> = (usize, (SideCode, SideCode), Cells, Vec3<T>);

/// A germ's `(A face, B face)` and `(A locus, B locus)`.
pub(super) type Cells = ((FaceKey, FaceKey), (super::Locus, super::Locus));

/// Mints one solid's null edge along its planned run, forward
/// `from → to`: `g0 → g1` unless that run swallows the whole orbit
/// (detected structurally: the fan's orbit successor of its last edge
/// is its first — the true wedge cannot, its far side being bounded by
/// the complementary germ) or holds another pair's cut at a shared
/// vertex ([`reconcile_shared`]). The F12 run-side agreement guard
/// (`entry germ's exit code == closing germ's entry code`) applies to
/// whichever direction was planned.
fn mint_directed<T: Decide>(
    body: &mut Body<T>,
    (operand, vertex): (Operand, VertexKey),
    sectors: &[BoolSector<T>],
    run: SideRun<T>,
    hung: &mut Hung<T>,
    mismatch: impl Fn() -> BooleanError,
    band: Band,
) -> Result<BoolNullEdgeRecord<T>, BooleanError> {
    let (gf, gt) = (run.from, run.to);
    // Strut spike ORDER (PR 5.5, the sort half of ssortnulledges): a
    // dangling strut's two halves splice consecutively into the loop
    // as [he_plus, he_minus]; interleaved (crossing) chords at
    // multi-germ corner sites wall pending pairs off, so the half the
    // loop walk meets FIRST (he_plus) must face the right germ. A germ
    // along an edge of this solid names it structurally
    // ([`strut_facing`]); otherwise the half meeting the loop first
    // faces the germ angularly nearest the splice corner's arrival
    // edge, measured inside the sector ([`strut_order`]). Senses
    // follow the facing by the sense theorem, so only the splice order
    // moves. Run direction is untouched (a strut's reverse run spans
    // the whole orbit).
    let empty = run_fan(sectors, gf.0, gt.0)?.is_empty();
    let corrupt = || BooleanError::corrupt_at(operand, vertex);
    // A strut at a shared vertex: its germs in walk order, and the
    // innermost strut hung earlier whose segment holds its own.
    let walk = if empty && run.shared {
        let (lo, hi) = run_ends(sectors, run, band)?;
        let mut holder: Option<&HungStrut<T>> = None;
        for h in hung
            .struts
            .iter()
            .filter(|h| (h.operand, h.root) == (operand, vertex))
        {
            let segment = |h: &HungStrut<T>| (h.lower, h.upper, h.side);
            let holds = holds_whole(sectors, segment(h), (lo, hi, gf.1.0), band)?;
            let inner = match holder {
                None => true,
                Some(o) => holds_whole(sectors, segment(o), segment(h), band)?,
            };
            if holds && inner {
                holder = Some(h);
            }
        }
        let from_is_lo = precedes(sectors, (gf.0, gf.3), (gt.0, gt.3), band)? == Some(true);
        Some((
            lo,
            from_is_lo,
            holder.map(|h| (h.vertex, h.half, h.half_faces_lower)),
        ))
    } else {
        None
    };
    let holder = walk.and_then(|(.., holder)| holder);
    let structural = if let (Some((_, from_is_lo, _)), Some((.., faces_lower))) = (walk, holder) {
        // Hung at the tip of the strut whose segment holds it, its half
        // met first follows that strut's half from the vertex, so it
        // faces its own germ on the side of the one that half faces.
        Some(from_is_lo == faces_lower)
    } else if empty {
        let arrival = body
            .get_half_edge(sectors[gf.0].he)
            .ok_or_else(corrupt)?
            .edge;
        // At a shared vertex the corner may already hold another
        // pair's strut, so its departure edge is read off the sectors.
        let departure_he = if run.shared {
            sectors[next_edge_bound(sectors, gf.0)].he
        } else {
            let mate = body.mate(sectors[gf.0].he).ok_or_else(corrupt)?;
            body.get_half_edge(mate).ok_or_else(corrupt)?.next
        };
        let departure = body.get_half_edge(departure_he).ok_or_else(corrupt)?.edge;
        // A germ inside a face, or along a closed edge at its lone
        // vertex, falls through to the angular reading below.
        match strut_facing(
            arrival,
            departure,
            own_locus_edge(operand, gf.2),
            own_locus_edge(operand, gt.2),
        )? {
            StrutFacing::PlusFirst => Some(true),
            StrutFacing::MinusFirst => Some(false),
            StrutFacing::Unnamed | StrutFacing::ClosedEdge => None,
        }
    } else {
        None
    };
    let spike_from_first = if let Some(first) = structural {
        first
    } else if empty {
        let s = &sectors[gf.0];
        strut_order(
            anchor_dir(body, s.he)?,
            s.normal.vec(),
            (gf.3, gt.3),
            s.arm.min(sectors[gt.0].arm),
            band,
        )?
    } else {
        false
    };
    let (from, to, side, closing) = (gf.0, gt.0, gf.1.0, gt.1.1);
    // F12 guard 2: the closing germ must approach the run with the
    // run's own side as its entry code.
    if closing != side {
        return Err(mismatch());
    }
    // Germ facings as data (module docs): he_plus faces the from-germ,
    // he_minus the to-germ (the mev splice contract).
    let meta = [(gf.2, gf.3), (gt.2, gt.3)];
    // A nested strut hangs at its holder's tip, past the struts hung
    // there earlier at a lower germ; any other shared strut at the
    // vertex, past those hung in its corner.
    let site = match (walk, holder) {
        (Some((lo, ..)), Some((_, arrival, _))) => {
            let tip = body.half_edge_end(arrival).ok_or_else(corrupt)?;
            let first = body.get_half_edge(arrival).ok_or_else(corrupt)?.next;
            Some((
                tip,
                strut_anchor(body, (operand, tip), sectors, first, lo, hung, band)?,
            ))
        }
        (Some((lo, ..)), None) => {
            let mate = body.mate(sectors[lo.0].he).ok_or_else(corrupt)?;
            let first = body.get_half_edge(mate).ok_or_else(corrupt)?.next;
            Some((
                vertex,
                strut_anchor(body, (operand, vertex), sectors, first, lo, hung, band)?,
            ))
        }
        (None, _) => None,
    };
    let at = site.map_or(vertex, |(v, _)| v);
    let rec = mint_run(
        body,
        operand,
        at,
        sectors,
        from,
        to,
        side,
        meta,
        spike_from_first,
        site.map(|(_, he)| he),
    )?;
    if let Some((lo, from_is_lo, _)) = walk {
        let edge = body.get_edge(rec.edge).ok_or_else(corrupt)?;
        let half = [edge.he_plus, edge.he_minus]
            .into_iter()
            .find(|&h| body.get_half_edge(h).is_some_and(|d| d.start == at))
            .ok_or_else(corrupt)?;
        let hi = run_ends(sectors, run, band)?.1;
        hung.struts.push(HungStrut {
            operand,
            root: vertex,
            vertex: at,
            half,
            lower: lo,
            upper: hi,
            half_faces_lower: (rec.germs[0].he == half) == from_is_lo,
            side: gf.1.0,
        });
    }
    // The join reads a null half's sense off the side its start vertex
    // is the end of, so no vertex at a shared point is the In end of
    // one null edge and the Out end of another: each run takes its own
    // pair's region, the vertex keeps what lies outside them all, and a
    // nested strut's region lies inside its holder's, so its holder's
    // tip is the same end of both.
    if run.shared {
        for (end, below) in [(rec.attr.below_end, true), (rec.attr.above_end, false)] {
            if hung
                .ends
                .iter()
                .any(|&(o, v, b)| (o, v) == (operand, end) && b != below)
            {
                return Err(BooleanError::ClassificationInvariant {
                    what: "a vertex at a shared point is the In end of one null edge and the \
                           Out end of another",
                });
            }
            hung.ends.push((operand, end, below));
        }
    }
    Ok(rec)
}

/// The entry past `k` whose end bound is a real edge: the one holding
/// the half-edge that bounds `k`'s physical sector at its start.
fn next_edge_bound<T: geom_core::Real>(sectors: &[BoolSector<T>], k: usize) -> usize {
    let n = sectors.len();
    let mut j = (k + 1) % n;
    while !sectors[j].end_edge() && j != k {
        j = (j + 1) % n;
    }
    j
}

/// Where a strut at a shared vertex splices: before the first half-edge
/// from `first` (the one past its corner's arrival) that is not a strut
/// hung earlier at that vertex at a lower germ (`germ`, the earlier of
/// a strut's two). The struts [`reconcile_shared`] leaves in one corner
/// hold none of each other's cuts unless one nests whole inside the
/// other, which hangs at the other's tip, so the segments hung at one
/// vertex are disjoint, and two lower germs along one direction would
/// make one segment hold the other's cut or both be one segment: an
/// invariant.
fn strut_anchor<T: Decide>(
    body: &Body<T>,
    (operand, vertex): (Operand, VertexKey),
    sectors: &[BoolSector<T>],
    first: HalfEdgeKey,
    germ: Cut<T>,
    hung: &Hung<T>,
    band: Band,
) -> Result<HalfEdgeKey, BooleanError> {
    let corrupt = || BooleanError::corrupt_at(operand, vertex);
    let successor = |he: HalfEdgeKey| -> Result<HalfEdgeKey, BooleanError> {
        let mate = body.mate(he).ok_or_else(corrupt)?;
        Ok(body.get_half_edge(mate).ok_or_else(corrupt)?.next)
    };
    let mut he = first;
    while let Some(at) = hung
        .struts
        .iter()
        .find(|h| (h.operand, h.vertex, h.half) == (operand, vertex, he))
        .map(|h| h.lower)
    {
        match precedes(sectors, at, germ, band)? {
            Some(true) => he = successor(he)?,
            Some(false) => break,
            None => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "two dangling null edges in one corner start along one direction",
                });
            }
        }
    }
    Ok(he)
}

/// Whether cut `p` comes before cut `q` walking forward through their
/// one physical sector (`None`: the two lie along one direction).
fn precedes<T: Decide>(
    sectors: &[BoolSector<T>],
    p: Cut<T>,
    q: Cut<T>,
    band: Band,
) -> Result<Option<bool>, BooleanError> {
    if p.0 == q.0 {
        return walks_after(&sectors[p.0], p.1, q.1, band);
    }
    let n = sectors.len();
    let mut first = p.0;
    while !sectors[first].end_edge() {
        first = (first + n - 1) % n;
        if first == p.0 {
            return Err(BooleanError::ClassificationInvariant {
                what: "a vertex orbit whose sectors hold no edge bound",
            });
        }
    }
    let rel = |e: usize| (e + n - first) % n;
    Ok(Some(rel(p.0) < rel(q.0)))
}

/// The edge of `operand`'s own solid a germ runs along, if its locus
/// there is `OnEdge`.
pub(super) fn own_locus_edge(operand: Operand, (_, loci): Cells) -> Option<EdgeKey> {
    match (operand, loci) {
        (Operand::A, (super::Locus::OnEdge(e), _)) | (Operand::B, (_, super::Locus::OnEdge(e))) => {
            Some(e)
        }
        _ => None,
    }
}

/// What a strut's corner edges say about which half faces which germ
/// ([`strut_facing`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum StrutFacing {
    /// `he_plus` faces the first germ and `he_minus` the second.
    PlusFirst,
    /// `he_minus` faces the first germ and `he_plus` the second.
    MinusFirst,
    /// Neither germ runs along the corner's edges: the edges name no
    /// facing.
    Unnamed,
    /// The corner's two edges are one closed edge (its lone vertex) and
    /// a germ runs along it: the germ leaves along one end of that edge
    /// and arrives along the other, so the edge key names no half.
    ClosedEdge,
}

/// **Which half of a strut faces which germ, where a germ runs along an
/// edge of its own solid.** A strut splices its halves
/// `[he_plus, he_minus]` into its corner between the arrival half and
/// the departure half, so `he_plus` lies beside the arrival edge and
/// `he_minus` beside the departure edge, and the half beside a germ's
/// own locus edge is the half that faces it — an angular reading would
/// meet such a germ exactly ON its comparison's bound.
///
/// Each germ along one of the two edges is a vote; the votes must
/// agree, and two that name opposite facings are no strut the
/// classification mints: refused. A germ inside a face casts none
/// ([`StrutFacing::Unnamed`] when neither votes), and at a closed
/// edge's lone vertex the edge key cannot say which end a germ along it
/// runs from ([`StrutFacing::ClosedEdge`]); each caller states what it
/// does then.
pub(super) fn strut_facing(
    arrival: EdgeKey,
    departure: EdgeKey,
    first: Option<EdgeKey>,
    second: Option<EdgeKey>,
) -> Result<StrutFacing, BooleanError> {
    if arrival == departure {
        let along = first == Some(arrival) || second == Some(arrival);
        return Ok(if along {
            StrutFacing::ClosedEdge
        } else {
            StrutFacing::Unnamed
        });
    }
    let mut votes = [
        (first == Some(arrival)).then_some(StrutFacing::PlusFirst),
        (first == Some(departure)).then_some(StrutFacing::MinusFirst),
        (second == Some(departure)).then_some(StrutFacing::PlusFirst),
        (second == Some(arrival)).then_some(StrutFacing::MinusFirst),
    ]
    .into_iter()
    .flatten();
    let Some(facing) = votes.next() else {
        return Ok(StrutFacing::Unnamed);
    };
    if votes.any(|v| v != facing) {
        return Err(BooleanError::ClassificationInvariant {
            what: "a strut's germs run along its corner's edges in contradictory order",
        });
    }
    Ok(facing)
}

/// **Whether the strut half met first faces the first germ**: whether
/// `germs.0` lies angularly nearer than `germs.1` to the splice
/// corner's arrival edge `e_dir`, measured inside the sector, which
/// sweeps from its end bound (the arrival edge) clockwise about its
/// face's outward `normal`.
///
/// Each germ is first placed in its half-turn by its side of the
/// arrival edge, `(g × e)·n` (`bool_strut_side`): the near half-turn
/// `[0, π)` reads positive, the far one `[π, 2π)` negative, and a germ
/// on the edge's line is placed by [`super::sectors::direction_sense`]
/// (along `e`: angle 0, near; against it: π, far). A germ in the near
/// half-turn is the nearer. Within one half-turn the nearer germ is the
/// one the other follows clockwise, `(g0 × g1)·n < 0`
/// (`bool_strut_order`). Both readings are sines levered at `arm`, the
/// shorter of the two sectors' bounding chords: the distance at that
/// arm from one direction's line, so they are linear in the spacing
/// at every angle, where a cosine is flat beside 0 and π. That is
/// `splitting::containment`'s doctrine for angular windows: decided as
/// distances, never as an angle. A decided zero, of a side whose
/// direction sense is in band or of two germs along one direction,
/// refuses: nothing orders the germs.
fn strut_order<T: Decide>(
    e_dir: Vec3<T>,
    normal: Vec3<T>,
    germs: (Vec3<T>, Vec3<T>),
    arm: T,
    band: Band,
) -> Result<bool, BooleanError> {
    let refuse = |diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag);
    let far = |g: Vec3<T>| -> Result<bool, BooleanError> {
        let side = Margin::levered(g.cross(e_dir).dot(normal), arm);
        match crate::validate::decide("bool_strut_side", side, band).map_err(refuse)? {
            Sign::Positive => Ok(false),
            Sign::Negative => Ok(true),
            Sign::Zero => Ok(!super::sectors::direction_sense(g, e_dir, arm, band)?),
        }
    };
    let (far0, far1) = (far(germs.0)?, far(germs.1)?);
    if far0 != far1 {
        return Ok(far1);
    }
    let order = Margin::levered(germs.0.cross(germs.1).dot(normal), arm);
    match crate::validate::decide_nonzero_reported("bool_strut_order", order, band)
        .map_err(refuse)?
    {
        NonzeroSign::Negative => Ok(true),
        NonzeroSign::Positive => Ok(false),
    }
}

/// The unit direction of an orbit half-edge away from its start
/// vertex (the strut-order comparison's angular reference).
fn anchor_dir<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Result<Vec3<T>, BooleanError> {
    let corrupt = || BooleanError::ClassificationInvariant {
        what: "strut anchor edge no longer resolves",
    };
    let hd = body.get_half_edge(he).ok_or_else(corrupt)?;
    let p_of = |v: crate::entity::VertexKey| -> Result<geom_core::Point3<T>, BooleanError> {
        body.get_vertex(v)
            .and_then(|vd| body.get_point(vd.point).copied())
            .ok_or_else(corrupt)
    };
    let end = body.half_edge_end(he).ok_or_else(corrupt)?;
    let d = p_of(end)? - p_of(hd.start)?;
    Ok(d.normalize())
}

/// The record's germ direction, by declared class: a `Tangent` pair's
/// sector normals are PARALLEL along the contact (the tangency), so
/// its germ direction is the verified closed-form locus
/// ([`geom_brep::tangent_locus`] — the DEV-1 witness the door
/// derived), signed into the sector pair by the same membership test;
/// every other pair takes the transverse normal cross ([`germ_dir`]).
fn record_germ_dir<T: Decide>(
    a_body: &Body<T>,
    b_body: &Body<T>,
    sa: &BoolSector<T>,
    sb: &BoolSector<T>,
    declared: &super::DeclaredPairs<T>,
    band: Band,
) -> Result<Vec3<T>, BooleanError> {
    // What the door read of the pair, which the questions below refuse
    // alike declared or not: no class settles them.
    let read = declared.read(
        &[(super::Operand::A, sa.face, super::Operand::B, sb.face)],
        Coincide::TangentLocus,
        &[],
    );
    if read != DeclarationRead::Spent(BooleanCoincidence::TANGENT) {
        return germ_dir(sa, sb, read, band);
    }
    let surface_of = |body: &Body<T>, face| {
        body.get_face(face)
            .and_then(|f| body.get_surface(f.surface))
            .cloned()
            .ok_or(BooleanError::ClassificationInvariant {
                what: "declared-Tangent face lost its surface",
            })
    };
    let s_a = surface_of(a_body, sa.face)?;
    let s_b = surface_of(b_body, sb.face)?;
    let reach = declared.reach_of(super::Operand::A, sa.face, super::Operand::B, sb.face)?;
    let d = match geom_brep::tangent_locus(&s_a, &s_b, reach, band) {
        Ok(geom_brep::TangentLocus::Line { dir, .. }) => dir.normalize(),
        Err(geom_brep::TangentLocusError::Escalated(diag)) => {
            return Err(BooleanError::coincidence(
                Coincide::TangentLocus,
                read,
                diag,
            ));
        }
        // Both remaining arms mean the same thing to this door: the
        // declaration promised a locus the closed-form lane does not
        // produce. Listed rather than wildcarded, so a new
        // `TangentLocusError` arm is classified here deliberately.
        Err(
            geom_brep::TangentLocusError::NotTangent { .. }
            | geom_brep::TangentLocusError::Unsupported { .. },
        ) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "declared-Tangent germ without a closed-form locus",
            });
        }
    };
    let plus = within(sa, d, false, read, band)? && within(sb, d, false, read, band)?;
    let minus = within(sa, -d, false, read, band)? && within(sb, -d, false, read, band)?;
    match (plus, minus) {
        (true, false) => Ok(d),
        (false, true) => Ok(-d),
        _ => Err(BooleanError::ClassificationInvariant {
            what: "germ direction not uniquely within its sector pair",
        }),
    }
}

/// The germ's outgoing direction: the unit intersection direction of
/// the two sector faces' planes, signed to lie within both sectors
/// (grazes count — an on-bound germ's direction IS the bound). An
/// ambiguous or coplanar configuration refuses loudly.
///
/// Sense-invariant given its sources (S10): `±(n_a × n_b)` is a LINE,
/// and the sign is chosen by sector membership, not by either normal —
/// flipping a normal flips the raw cross product and the `within`
/// verdicts pick the same ray back out. The normals arrive already
/// oriented from `sectors::sector_face`; nothing is multiplied here.
fn germ_dir<T: Decide>(
    sa: &BoolSector<T>,
    sb: &BoolSector<T>,
    read: DeclarationRead,
    band: Band,
) -> Result<Vec3<T>, BooleanError> {
    let int = sa.normal.vec().cross(sb.normal.vec());
    // The same margin `pair_search` read as definite before it sent this
    // pair down the crossing path: a pair whose parallelism is Zero or in
    // band there is coplanar or refused, never a crossing record.
    let arm = sa.arm.min(sb.arm);
    match crate::validate::decide("bool_germ_line", Margin::levered(int.norm(), arm), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "surviving crossing record on coplanar sector faces",
            });
        }
        // The same margin, re-read: its escalation is the kernel's, as its
        // zero is.
        Err(diag) => {
            return Err(BooleanError::Escalated {
                decision: BooleanDecision::SelfCheck(SelfCheck::GermLine),
                diag,
            });
        }
    }
    let d = int.normalize();
    let plus = within(sa, d, false, read, band)? && within(sb, d, false, read, band)?;
    let minus = within(sa, -d, false, read, band)? && within(sb, -d, false, read, band)?;
    match (plus, minus) {
        (true, false) => Ok(d),
        (false, true) => Ok(-d),
        _ => Err(BooleanError::ClassificationInvariant {
            what: "germ direction not uniquely within its sector pair",
        }),
    }
}

/// Whether the forward run `from → to` would swallow the entire orbit
/// (a nonempty fan whose orbit successor wraps to its first member).
fn run_degenerates<T: Decide>(
    body: &Body<T>,
    sectors: &[BoolSector<T>],
    from: usize,
    to: usize,
) -> Result<bool, BooleanError> {
    let hes = run_fan(sectors, from, to)?;
    let Some((&first, &last)) = hes.first().zip(hes.last()) else {
        return Ok(false); // empty fan: a valid strut
    };
    let mate = body
        .mate(last)
        .ok_or(BooleanError::ClassificationInvariant {
            what: "run edge without a mate",
        })?;
    let successor = body
        .get_half_edge(mate)
        .ok_or(BooleanError::ClassificationInvariant {
            what: "run edge mate no longer resolves",
        })?
        .next;
    Ok(successor == first)
}

/// The real edge bounds crossed walking the entry chain forward from
/// entry `from` (exclusive) to entry `to` (inclusive).
fn run_fan<T: Decide>(
    sectors: &[BoolSector<T>],
    from: usize,
    to: usize,
) -> Result<Vec<HalfEdgeKey>, BooleanError> {
    let n = sectors.len();
    let mut hes: Vec<HalfEdgeKey> = Vec::new();
    if from != to {
        let mut k = (from + 1) % n;
        loop {
            if sectors[k].end_edge() {
                hes.push(sectors[k].he);
            }
            if k == to {
                break;
            }
            k = (k + 1) % n;
            if k == (from + 1) % n {
                return Err(BooleanError::ClassificationInvariant {
                    what: "run walk wrapped without reaching its closing germ",
                });
            }
        }
    }
    Ok(hes)
}

/// Mints one null edge spanning the run from the germ in sector entry
/// `from` (exclusive) through entry `to` (inclusive).
///
/// The fan = the **real edge bounds crossed** walking the entry chain
/// forward from the first germ to the second: entering entry `k`
/// crosses the shared bound `sectors[k].end`, which is the orbit edge
/// `sectors[k].he` exactly when `end_edge` — a subdivision-twin
/// boundary (bisector) is crossed without moving any edge. (The
/// original per-entry `he` collection mis-moved fans whenever a germ
/// sat in a wide sector's twin — the bisector-graze lane of the
/// coplanar corpus; entries are pieces, not physical sectors.)
///
/// An empty fan — `from == to`, or germs in two twins of one physical
/// sector — is the dangling strut, spliced INSIDE that physical
/// sector: at the orbit successor of the sector's own half
/// (`next(mate(sectors[from].he))`), which is twin-stable (twins share
/// `he`).
#[allow(clippy::too_many_arguments)]
fn mint_run<T: Decide>(
    body: &mut Body<T>,
    operand: Operand,
    vertex: VertexKey,
    sectors: &[BoolSector<T>],
    from: usize,
    to: usize,
    run_side: SideCode,
    germ_meta: [(Cells, Vec3<T>); 2],
    spike_from_first: bool,
    anchor: Option<HalfEdgeKey>,
) -> Result<BoolNullEdgeRecord<T>, BooleanError> {
    let hes = run_fan(sectors, from, to)?;
    let successor = |body: &Body<T>, he: HalfEdgeKey| -> Result<HalfEdgeKey, BooleanError> {
        let mate = body
            .mate(he)
            .ok_or(BooleanError::corrupt_at(operand, vertex))?;
        Ok(body
            .get_half_edge(mate)
            .ok_or(BooleanError::corrupt_at(operand, vertex))?
            .next)
    };
    let (site, dangling) = if hes.is_empty() {
        // The dangling strut, inside `from`'s physical sector.
        let he = match anchor {
            Some(he) => he,
            None => successor(body, sectors[from].he)?,
        };
        (MevSite::Fan { he1: he, he2: he }, true)
    } else {
        let first = hes[0];
        let last = *hes.last().unwrap_or(&first);
        // he2 at execution time: current orbit successor of the run's
        // last half-edge (PR 2's pattern — robust against prior
        // splices).
        let he2 = successor(body, last)?;
        if he2 == first {
            // The run would swallow the whole orbit — the complementary
            // germ must bound it (kernel bug in run selection, loudly:
            // mev would silently degrade this site to a strut).
            return Err(BooleanError::ClassificationInvariant {
                what: "null-edge run spans the entire vertex orbit",
            });
        }
        (MevSite::Fan { he1: first, he2 }, false)
    };
    // The copy takes the run; its side is the run's side (F3-derived).
    let new_side = match run_side {
        SideCode::In => NewVertexSide::Below,
        SideCode::Out => NewVertexSide::Above,
        SideCode::On => {
            return Err(BooleanError::ClassificationInvariant {
                what: "run with On side reached insertion",
            });
        }
    };
    // Side attributes per the PR 5.5 sense theorem (join module docs):
    // the half FACING a germ is UP (starts at `below_end`) iff that
    // germ's own forward-wedge code is Out. Non-dangling: he_plus
    // (old → new) faces the from-germ whose forward code is the run
    // side, so `created` is the below end exactly for In-runs.
    // Dangling struts in the default spike order swap the facing
    // (he_minus at the from-germ), so the SIDE swaps with it; the
    // angular `spike_from_first` order restores the non-dangling
    // facing. The attribute is derived sense data, never a mint-slot
    // echo; the mint side follows so the body's scaffold attribute and
    // the pipeline record stay one datum.
    let attr_side = match (new_side, dangling && !spike_from_first) {
        (side, false) => side,
        (NewVertexSide::Below, true) => NewVertexSide::Above,
        (NewVertexSide::Above, true) => NewVertexSide::Below,
    };
    let created = body.mev_null(site, attr_side)?;
    let attr = match attr_side {
        NewVertexSide::Below => NullEdge {
            below_end: created.vertex,
            above_end: vertex,
        },
        NewVertexSide::Above => NullEdge {
            below_end: vertex,
            above_end: created.vertex,
        },
    };
    let germ = |i: usize, he: crate::entity::HalfEdgeKey| {
        let (((a_face, b_face), (a_locus, b_locus)), dir) = germ_meta[i];
        super::HalfGerm {
            he,
            a_face,
            b_face,
            a_locus,
            b_locus,
            dir,
        }
    };
    // Germ ↔ half facing: for a fan the mev splice puts he_plus at the
    // from-germ cut and he_minus at the to-germ cut; a strut's spike
    // splices [he_plus, he_minus] into one corner, and which germ the
    // loop-first half (he_plus) faces is the angular spike order
    // decided at the mint site (`spike_from_first`; the default — the
    // corner walk arriving through the to-germ — was pinned
    // empirically by the joining fixtures).
    let germs = if dangling && !spike_from_first {
        [germ(0, created.he_minus), germ(1, created.he_plus)]
    } else {
        [germ(0, created.he_plus), germ(1, created.he_minus)]
    };
    Ok(BoolNullEdgeRecord {
        operand,
        at_vertex: vertex,
        edge: created.edge,
        attr,
        dangling,
        germs,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Tol;

    /// **`strut_facing`'s vote table.** `he_plus` lies beside the
    /// arrival edge and `he_minus` beside the departure edge, so each
    /// germ along one of them votes for the facing that puts the half
    /// beside it toward it; agreeing votes stand, a contradiction
    /// refuses, no vote is `Unnamed`, and one closed edge on both sides
    /// with a germ along it is `ClosedEdge`.
    #[test]
    fn strut_facing_reads_the_corner_edges() {
        use StrutFacing::{ClosedEdge, MinusFirst, PlusFirst, Unnamed};
        let mut keys = slotmap::SlotMap::<EdgeKey, ()>::with_key();
        let (arr, dep, other) = (keys.insert(()), keys.insert(()), keys.insert(()));
        let f = |first, second| strut_facing(arr, dep, first, second);
        // Each single vote.
        assert_eq!(
            f(Some(arr), None).unwrap(),
            PlusFirst,
            "first along arrival"
        );
        assert_eq!(
            f(Some(dep), None).unwrap(),
            MinusFirst,
            "first along departure"
        );
        assert_eq!(
            f(None, Some(dep)).unwrap(),
            PlusFirst,
            "second along departure"
        );
        assert_eq!(
            f(None, Some(arr)).unwrap(),
            MinusFirst,
            "second along arrival"
        );
        // Agreeing pairs.
        assert_eq!(f(Some(arr), Some(dep)).unwrap(), PlusFirst);
        assert_eq!(f(Some(dep), Some(arr)).unwrap(), MinusFirst);
        // No vote: germs inside faces, or along an edge off the corner.
        assert_eq!(f(None, None).unwrap(), Unnamed);
        assert_eq!(f(Some(other), Some(other)).unwrap(), Unnamed);
        // Contradictions refuse.
        for (first, second) in [(Some(arr), Some(arr)), (Some(dep), Some(dep))] {
            assert!(
                matches!(
                    f(first, second),
                    Err(BooleanError::ClassificationInvariant { .. })
                ),
                "{first:?} {second:?}"
            );
        }
        // A closed edge's lone vertex.
        let closed = |first, second| strut_facing(arr, arr, first, second).unwrap();
        assert_eq!(closed(Some(arr), None), ClosedEdge);
        assert_eq!(closed(None, Some(arr)), ClosedEdge);
        assert_eq!(closed(None, None), Unnamed);
        assert_eq!(closed(Some(other), None), Unnamed);
    }

    /// The survivor-validation invariants: odd counts and dirty codes
    /// refuse loudly (unit-level; the geometric paths are pinned by the
    /// acceptance fixtures).
    #[test]
    fn survivor_validation() {
        let mk = |sa, sb| PairRecord {
            a: 0,
            b: 0,
            sa,
            sb,
            intersect: true,
        };
        let mut a = crate::test_support_fixtures::declined_cube::<f64>(Tol::witness()).body;
        let mut b = crate::test_support_fixtures::declined_cube::<f64>(Tol::witness()).body;
        let contact = VvContact {
            a: VertexKey::default(),
            b: VertexKey::default(),
        };
        use SideCode::{In, Out};
        let recs = vec![mk((In, Out), (In, Out))];
        let err = insert_null_pairs(
            &mut a,
            &mut b,
            contact,
            &[],
            &[],
            &recs,
            &recs,
            &crate::boolean::DeclaredPairs::default(),
            geom_core::Band::linear(Tol::witness()).unwrap(),
        )
        .unwrap_err();
        assert!(matches!(err, BooleanError::ClassificationInvariant { .. }));
        let recs = vec![mk((In, In), (In, Out)), mk((Out, In), (Out, In))];
        let err = insert_null_pairs(
            &mut a,
            &mut b,
            contact,
            &[],
            &[],
            &recs,
            &recs,
            &crate::boolean::DeclaredPairs::default(),
            geom_core::Band::linear(Tol::witness()).unwrap(),
        )
        .unwrap_err();
        assert!(matches!(err, BooleanError::ClassificationInvariant { .. }));
    }

    /// F12 guard: a 4-survivor set whose A-consecutive pair is NOT
    /// B-cyclically adjacent refuses as the typed PairingMismatch —
    /// the 15.11 consecutive-pairing invariant is never silently
    /// assumed (a plus-sign-interleaved b-order: A pairs (0,1) but in
    /// B-order the survivors interleave 0,2,1,3).
    #[test]
    fn f12_pairing_mismatch_guard() {
        use SideCode::{In, Out};
        let mk = |a: usize, b: usize, sa, sb| PairRecord {
            a,
            b,
            sa,
            sb,
            intersect: true,
        };
        let mut abody = crate::test_support_fixtures::declined_cube::<f64>(Tol::witness()).body;
        let mut bbody = crate::test_support_fixtures::declined_cube::<f64>(Tol::witness()).body;
        let contact = VvContact {
            a: VertexKey::default(),
            b: VertexKey::default(),
        };
        // A-order: a = 0,1,2,3; B-order by b: r0(b=0), r2(b=1),
        // r1(b=2), r3(b=3) — pair (r0, r1) is not B-adjacent.
        let recs = vec![
            mk(0, 0, (In, Out), (In, Out)),
            mk(1, 2, (Out, In), (Out, In)),
            mk(2, 1, (In, Out), (In, Out)),
            mk(3, 3, (Out, In), (Out, In)),
        ];
        let err = insert_null_pairs(
            &mut abody,
            &mut bbody,
            contact,
            &[],
            &[],
            &recs,
            &recs,
            &crate::boolean::DeclaredPairs::default(),
            geom_core::Band::linear(Tol::witness()).unwrap(),
        )
        .unwrap_err();
        assert!(
            matches!(err, BooleanError::PairingMismatch { .. }),
            "{err:?}"
        );
    }

    /// F12 at mechanism level: four survivors, two consecutive record
    /// pairs, each a strut in BOTH solids, on real cube vertices but
    /// synthetic sectors. Every germ runs along `+y`, and the first
    /// strut's arrival edge (a cube edge) is normal to its sector's
    /// plane, so nothing orders the strut's two germs: the strut order
    /// refuses at the first germ's side, before any mint. Four
    /// survivors at one vertex pair in real geometry are
    /// `work/join/four-germ-vertex-pairs-run-b-in-a-order`'s.
    #[test]
    fn f12_four_unordered_struts_refuse_before_any_mint() {
        use SideCode::{In, Out};
        let mk = |a: usize, b: usize, sa, sb| PairRecord {
            a,
            b,
            sa,
            sb,
            intersect: true,
        };
        let mut abody = crate::test_support_fixtures::declined_cube::<f64>(Tol::witness()).body;
        let mut bbody = crate::test_support_fixtures::declined_cube::<f64>(Tol::witness()).body;
        // A and B sector fans on NON-parallel face planes (the germ
        // direction z×x = +y is uniquely within both — `germ_dir`
        // refuses coplanar sector pairs by design).
        let sectors_of = |body: &Body<f64>,
                          normal: geom_brep::OutwardNormal<f64>,
                          start: geom_core::Vec3<f64>,
                          end: geom_core::Vec3<f64>| {
            let (vk, v) = body.vertices().next().unwrap();
            let orbit = body.vertex_orbit(v.emanating.unwrap()).unwrap();
            let secs: Vec<BoolSector<f64>> = orbit
                .iter()
                .map(|&he| BoolSector {
                    he,
                    start,
                    end,
                    start_reach: crate::boolean::sectors::Reach::Extent(1.0),
                    end_reach: crate::boolean::sectors::Reach::Extent(1.0),
                    face: crate::entity::FaceKey::default(),
                    normal,
                    arm: 1.0,
                })
                .collect();
            (vk, secs)
        };
        // Proper quarter sectors on non-parallel planes: the germ line
        // z×x = +y is uniquely within both.
        let (va, a_sectors) = sectors_of(
            &abody,
            geom_brep::OutwardNormal::from_chart(geom_core::Vec3::new(0.0, 0.0, 1.0), true),
            geom_core::Vec3::new(1.0, 0.0, 0.0),
            geom_core::Vec3::new(0.0, 1.0, 0.0),
        );
        let (vb, b_sectors) = sectors_of(
            &bbody,
            geom_brep::OutwardNormal::from_chart(geom_core::Vec3::new(1.0, 0.0, 0.0), true),
            geom_core::Vec3::new(0.0, 1.0, 0.0),
            geom_core::Vec3::new(0.0, 0.0, 1.0),
        );
        let contact = VvContact { a: va, b: vb };
        // Two consecutive pairs, each pair a strut in both solids
        // (identical sector indices within the pair), codes mirrored.
        let recs = vec![
            mk(0, 0, (Out, In), (Out, In)),
            mk(0, 0, (In, Out), (In, Out)),
            mk(1, 1, (Out, In), (Out, In)),
            mk(1, 1, (In, Out), (In, Out)),
        ];
        let edges = (abody.edges().count(), bbody.edges().count());
        let err = insert_null_pairs(
            &mut abody,
            &mut bbody,
            contact,
            &a_sectors,
            &b_sectors,
            &recs,
            &recs,
            &crate::boolean::DeclaredPairs::default(),
            geom_core::Band::linear(Tol::witness()).unwrap(),
        )
        .expect_err("nothing orders the struts' germs");
        assert!(
            matches!(
                err,
                BooleanError::Escalated {
                    decision: BooleanDecision::DirectionSense,
                    ..
                }
            ),
            "{err:?}"
        );
        assert_eq!(
            (abody.edges().count(), bbody.edges().count()),
            edges,
            "no null edge minted"
        );
        crate::validate::validate(&abody).unwrap();
        crate::validate::validate(&bbody).unwrap();
    }

    /// **The germ line is the kernel's own re-reading**: two sector
    /// faces whose normals part by an in-band angle at the sectors' arm
    /// escalate as [`SelfCheck::GermLine`], ending as a defect (their
    /// crossing record needed this margin definitely positive upstream),
    /// never as a coincidence with a tolerance.
    #[test]
    fn an_in_band_germ_line_is_the_kernels_own_check() {
        use super::super::sectors::Reach;
        use geom_brep::OutwardNormal;
        use geom_core::{KERNEL_DEFECT_ENDING, Point3};
        let band = Band::linear(Tol::witness()).unwrap();
        let mid = (band.zero() + band.escalate()) / 2.0;
        let o = Point3::new(0.0, 0.0, 0.0);
        let (x, y) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let sector = |normal: Vec3<f64>| BoolSector {
            he: HalfEdgeKey::default(),
            start: x,
            end: y,
            start_reach: Reach::Chord {
                base: o,
                far: o + x,
            },
            end_reach: Reach::Chord {
                base: o,
                far: o + y,
            },
            face: FaceKey::default(),
            normal: OutwardNormal::from_chart(normal, true),
            arm: 1.0,
        };
        let (sa, sb) = (
            sector(Vec3::new(0.0, 0.0, 1.0)),
            sector(Vec3::new(mid.sin(), 0.0, mid.cos())),
        );
        let err = germ_dir(&sa, &sb, DeclarationRead::Moot, band).expect_err("in band");
        assert!(
            matches!(
                err,
                BooleanError::Escalated {
                    decision: BooleanDecision::SelfCheck(SelfCheck::GermLine),
                    ..
                }
            ),
            "{err:?}"
        );
        assert!(err.to_string().ends_with(KERNEL_DEFECT_ENDING), "{err}");
    }

    /// The strut order ranks two germs by their angle from the arrival
    /// edge inside the sector, past a half-turn too. The sector sweeps
    /// from the arrival edge `+x` clockwise about `+z`, so `-y` lies 90°
    /// in, `-x` 180° and `+y` 270°. Each row's answer is whether the
    /// first germ is the nearer; a cosine alone answers the far row the
    /// other way round.
    #[test]
    fn the_strut_order_reads_angles_past_a_half_turn() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (e, n) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let at = |deg: f64| {
            let t = -deg.to_radians();
            Vec3::new(t.cos(), t.sin(), 0.0)
        };
        for (g0, g1, first_nearer, row) in [
            (45.0, 90.0, true, "both in the near half-turn"),
            (90.0, 45.0, false, "both in the near half-turn, swapped"),
            (90.0, 270.0, true, "one in each half-turn"),
            (270.0, 90.0, false, "one in each half-turn, swapped"),
            (
                180.0,
                270.0,
                true,
                "both in the far half-turn, one on its bound",
            ),
            (270.0, 180.0, false, "both in the far half-turn, swapped"),
            (300.0, 225.0, false, "both strictly in the far half-turn"),
        ] {
            assert_eq!(
                strut_order(e, n, (at(g0), at(g1)), 1.0, band).unwrap(),
                first_nearer,
                "{row}: germs at {g0}° and {g1}°"
            );
        }
    }

    /// REVIEW (join/reflex-corner-review): two germs decidedly apart in
    /// angle (each germ's side `(g × e)·n` decides at these spacings),
    /// but both beside a half-turn bound, where the cosine is flat: the
    /// cosine difference is second order in the spacing and falls inside
    /// the band. The strut order must then either order them rightly or
    /// escalate; a decided zero read as "the second is nearer" is a
    /// silent wrong pick. The spacing, 0.001°, is 1.7e-5 at the unit
    /// arm: some 17 000 bands; the cross product of the two germs,
    /// `(g0 × g1)·n`, reads it to first order.
    #[test]
    fn review_the_strut_order_never_misorders_germs_beside_a_half_turn_bound() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (e, n) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let at = |deg: f64| {
            let t = -deg.to_radians();
            Vec3::new(t.cos(), t.sin(), 0.0)
        };
        let mut wrong = Vec::new();
        for (g0, g1) in [
            (0.001, 0.002),
            (179.998, 179.999),
            (180.0, 180.001),
            (180.001, 180.002),
            (0.002, 0.001),
            (180.001, 180.0),
        ] {
            let truth = g0 < g1;
            match strut_order(e, n, (at(g0), at(g1)), 1.0, band) {
                Ok(got) if got != truth => wrong.push(format!("{g0}°/{g1}°: got {got}")),
                _ => {}
            }
        }
        assert!(wrong.is_empty(), "silently misordered: {wrong:?}");
    }

    /// REVIEW (join/reflex-corner-review): an arrival edge along the
    /// face normal (the synthetic sectors of `f12_four_survivor_pairing`)
    /// leaves every germ's side, its along-reading and the cosine
    /// difference a decided zero. Nothing then orders the germs, and
    /// the order must escalate rather than answer.
    #[test]
    fn review_the_strut_order_escalates_when_nothing_orders_the_germs() {
        let band = Band::linear(Tol::witness()).unwrap();
        let n = Vec3::new(0.0, 0.0, 1.0);
        let germs = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        let got = strut_order(n, n, germs, 1.0, band);
        assert!(
            got.is_err(),
            "decided with no comparand away from zero: {got:?}"
        );
    }
}
