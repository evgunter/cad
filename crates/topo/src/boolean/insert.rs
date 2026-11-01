//! Paired null-edge insertion (Programs 15.11/15.12 motion, F9/F12).
//!
//! Surviving records — each a section-polygon edge germ with one In and
//! one Out code per side — are paired **consecutively in A's walk
//! order** (the book's consumption): by sector entry, and round the
//! sector within one entry, never by the other solid's entry; with more
//! than two, from the germ whose run lies on the side the op keeps of
//! A. Each pair mints one null edge in each solid spanning an orbit run
//! between its two germs: in A the run that holds no third germ in A's
//! walk order, in B the same where the pair is adjacent in B's, and
//! otherwise the run that holds other pairs' runs whole ([`b_runs`]),
//! each until a shared vertex turns it ([`reconcile_shared`]), with:
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
//!   what A-consecutive pairs are in B for > 2 crossings. Each solid's
//!   link round its vertex is a simple closed curve, and A's runs on
//!   one side of B are disjoint arcs of one disk B's link bounds, so the
//!   pairs cannot cross in B's walk order. At four crossings that makes
//!   each pair adjacent in B too. At six or more a pair may hold
//!   another's germs between its own either way round B's vertex (a
//!   nested matching): B's run for it then holds the other pair's run,
//!   which mints at its copy ([`b_runs`]). We check both halves: no two
//!   pairs cross in B's walk order, and the run-side codes agree at both
//!   ends (`r.own_start_code == r'.own_end_code` per solid). Violation ⇒
//!   typed [`super::BooleanError::PairingMismatch`] — fail-loud, never
//!   a mis-joined seam.
//!
//! Run extraction: walking the array forward (array order follows the
//! orbit), an entry is entered across its END bound and left across its
//! START ([`walks_after`]), so a germ in entry `k` transitions that
//! entry's codes `end → start`. The run from germ r to germ r′ enters
//! entries `r.own+1 ..= r′.own`, and the end bounds it crosses that are
//! real edges (`end_edge`, not a subdivision twin's bisector) form the
//! `mev_null` fan ([`run_fan`]). An empty fan (both germs in one entry,
//! or in two twins of one physical sector) is the strut, dangling inside
//! that physical sector ([`mint_run`]).
//!
//! **Plan, then mint.** [`plan_null_pairs`] takes every reading a
//! pair's null edges need (codes, pairing, germ cells and directions,
//! each run's direction) and [`mint_plans`] mints from the plan without
//! reading the orbit's geometry again. The reduction plans every vertex
//! pair before it mints any: one vertex can sit in several pairs (an
//! operand whose own contact left several vertices at one point pairs
//! each with the other operand's vertex there), and a mint moves its
//! vertex's orbit. Where several crossing pairs cut one vertex,
//! [`reconcile_shared`] turns each run that would hold another pair's
//! cut the other way round its orbit, so no pair's run holds another
//! pair's and no mint moves a half-edge another pair's plan read, unless
//! one strut's segment holds another's whole: the inner then hangs at
//! the outer's tip ([`holds_whole`]). A turned run holds the rest of its
//! own pair's runs. Wherever several null edges cut one
//! vertex, several pairs' or one pair's crossing it more than twice,
//! struts mint before any fan, an outer before its inner, each spliced
//! past those hung earlier at its vertex at a lower germ
//! ([`strut_anchor`]). A run that runs of its own pair hold, nested in
//! B's walk order ([`b_runs`]) or inside a run a shared vertex turned
//! ([`hang_at_shared`]), as arcs of its walk ([`arc_holders`]), mints
//! after them, at the copy the innermost fan among them took its germs
//! to.

use geom_core::k_stats::NonzeroSign;
use geom_core::{Band, Decide, Margin, Sign, Vec3};

use super::sectors::{BoolSector, Leaned, PairRecord, leaned_lever};
use super::{
    BoolNullEdgeRecord, BooleanError, BooleanOp, NullEdgePairRecord, Operand, PairSite, SideCode,
    VvContact,
};
use super::{BooleanDecision, Coincide, DeclarationRead, SelfCheck};
use crate::body::{Body, WALKS_CLOSE};
use crate::contact::GlueClass;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::euler::{MevSite, RunSite};
use crate::live::{Proven, linked, proven};
use crate::null::NewVertexSide;

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
    declared: &super::GluedPairs<T>,
    contacts: &super::ReductionContacts,
    op: BooleanOp,
    band: Band,
) -> Result<InsertOut<T>, BooleanError> {
    let mut plans = [plan_null_pairs(
        a_body, b_body, contact, a_sectors, b_sectors, records, raw, declared, contacts, op, band,
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
    /// Whether another null edge cuts the same vertex: another run of
    /// its own pair's (a pair crossing more than twice), or another
    /// crossing pair's ([`reconcile_shared`]). A strut then splices among
    /// the struts hung there by their lower germs ([`strut_anchor`]),
    /// before any fan moves the corner. `reconcile_shared` checks a run
    /// against other pairs' cuts only: a pair's own runs are disjoint,
    /// or one holds another whole ([`Self::held`]).
    shared: bool,
    /// Its germs' positions in its solid's walk order ([`walk_order`]),
    /// `from`'s first: the run is the arc of the walk between them,
    /// forward ([`arc_holders`]).
    walk: [usize; 2],
    /// Where the runs of its own plan hold this one ([`arc_holders`]): a
    /// pair nested in B's walk order ([`b_runs`]), or one inside a run
    /// turned round a shared vertex ([`hang_at_shared`]).
    held: Option<Held>,
}

/// How a run's own plan holds it: written at the plan for B's nested
/// pairings ([`b_runs`]), and read again after the reconcile at a
/// shared vertex, where a turned run holds what lay outside it
/// ([`hang_at_shared`]).
#[derive(Clone, Copy, Debug)]
struct Held {
    /// How many of the plan's runs hold it: it mints after each.
    depth: usize,
    /// The run that holds it directly (an index into
    /// [`NullPlan::runs`]).
    holder: usize,
    /// The innermost of them that is a fan (an index into
    /// [`NullPlan::runs`]): that fan's mint carried this run's germs to
    /// its copy, where this run mints. `None`: only struts hold it, and
    /// it mints at the plan's own vertex.
    fan: Option<usize>,
    /// Whether the run that holds it directly is a strut: it then hangs
    /// at that strut's tip, which [`mint_directed`] finds by the
    /// geometry ([`holds_whole`]) and checks against this.
    by_strut: bool,
}

/// One vertex pair's two sector arrays, A's and B's.
pub(super) type Orbits<'a, T> = (&'a [BoolSector<T>], &'a [BoolSector<T>]);

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// While [`with_vertex_pairs_reversed`] runs: how many reductions
    /// inside [`ops_under_test`] took, in reverse, vertex pairs two of
    /// which cross at one vertex.
    static REVERSED: core::cell::Cell<Option<usize>> = const { core::cell::Cell::new(None) };
    /// Whether [`ops_under_test`] is running.
    static UNDER_TEST: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// Puts a thread-local back as it was when dropped, so a panic inside
/// the scope that set it cannot leak the setting into the thread's next
/// test.
#[cfg(any(test, feature = "test-support"))]
struct Restore<V: Copy + 'static> {
    cell: &'static std::thread::LocalKey<core::cell::Cell<V>>,
    prior: V,
}

#[cfg(any(test, feature = "test-support"))]
impl<V: Copy + 'static> Drop for Restore<V> {
    fn drop(&mut self) {
        self.cell.set(self.prior);
    }
}

/// Runs `f` with every reduction on this thread taking its vertex pairs
/// in reverse order, and each pair's records too ([`pairs_reversed`]),
/// and returns how many reductions inside [`ops_under_test`] held two
/// crossing pairs at one vertex: what the results must not depend on.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn with_vertex_pairs_reversed<R>(f: impl FnOnce() -> R) -> (R, usize) {
    let _restore = Restore {
        cell: &REVERSED,
        prior: REVERSED.replace(Some(0)),
    };
    let out = f();
    (out, REVERSED.get().unwrap_or(0))
}

/// Runs `f` as the ops a row tests, apart from the booleans that built
/// their operands: [`with_vertex_pairs_reversed`] counts only these.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn ops_under_test<R>(f: impl FnOnce() -> R) -> R {
    let _restore = Restore {
        cell: &UNDER_TEST,
        prior: UNDER_TEST.replace(true),
    };
    f()
}

/// Whether this thread's reductions take their vertex pairs and each
/// pair's records in reverse ([`with_vertex_pairs_reversed`]).
#[cfg(any(test, feature = "test-support"))]
fn pairs_reversed() -> bool {
    REVERSED.get().is_some()
}

/// Whether this thread's reductions take their vertex pairs and each
/// pair's records in reverse: never, outside tests.
#[cfg(not(any(test, feature = "test-support")))]
const fn pairs_reversed() -> bool {
    false
}

/// Reverses `plans` and `orbits` inside [`with_vertex_pairs_reversed`].
#[cfg(any(test, feature = "test-support"))]
pub(super) fn reverse_when_asked<T: geom_core::Real>(
    plans: &mut [NullPlan<T>],
    orbits: &mut [Orbits<'_, T>],
) {
    let Some(n) = REVERSED.get() else {
        return;
    };
    plans.reverse();
    orbits.reverse();
    let crossing: Vec<VvContact> = plans
        .iter()
        .filter(|p| !p.runs.is_empty())
        .map(|p| p.contact)
        .collect();
    let shared = crossing
        .iter()
        .enumerate()
        .any(|(i, c)| crossing[i + 1..].iter().any(|d| d.a == c.a || d.b == c.b));
    REVERSED.set(Some(n + usize::from(shared && UNDER_TEST.get())));
}

/// A cut in a vertex's orbit: its sector entry, its direction, and,
/// where it runs along an edge of the other solid, that edge's
/// flankers ([`flankers_of`]), which place it beside the edge
/// ([`beside_order`]).
#[derive(Clone, Copy, Debug)]
pub(super) struct Cut<T: geom_core::Real> {
    entry: usize,
    line: Leaned<T>,
    beside: Option<[Vec3<T>; 2]>,
}

impl<T: geom_core::Real> Cut<T> {
    /// A cut along `line` in entry `entry` that runs along no edge of
    /// the other solid.
    pub(super) fn apart(entry: usize, line: Leaned<T>) -> Self {
        Self {
            entry,
            line,
            beside: None,
        }
    }

    /// The two cuts' flankers where the orbit's reading of a tie
    /// ([`CutOrder::Tied`], [`beside_order`]) can read them at all: both
    /// run beside an edge of the other solid. The one gate on that
    /// reading.
    fn tie_flankers(self, other: Self) -> Option<[[Vec3<T>; 2]; 2]> {
        Some([self.beside?, other.beside?])
    }
}

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
    /// Its segment ([`segment`]).
    segment: Segment<T>,
    /// Whether `half` faces its lower germ.
    half_faces_lower: bool,
}

impl<T: geom_core::Real> Default for Hung<T> {
    fn default() -> Self {
        Self {
            struts: Vec::new(),
            ends: Vec::new(),
        }
    }
}

/// **The record of a germ tangent to a bound of its sector**, read in
/// the sector across that bound where its locus names that sector's
/// face. The bound is the one `raw` reads On, so it is the moved
/// sector's bound too, and the moved pair's own codes
/// ([`super::sectors::pair_codes`]) read On there; everywhere else they
/// must read what the record carries, which is checked. Any other
/// record, and one whose neighbour across the bound is not the locus's
/// face, is returned as it is.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] where a moved pair's own
/// codes off the On bound differ from the record's, and the escalation
/// of a code read in band.
fn across_tangent<T: Decide>(
    r: PairRecord,
    raw: &PairRecord,
    loci: (super::Locus, super::Locus),
    (a, b): (&[BoolSector<T>], &[BoolSector<T>]),
    band: Band,
) -> Result<PairRecord, BooleanError> {
    let across = |sectors: &[BoolSector<T>], i: usize, read: (SideCode, SideCode), locus| {
        let super::Locus::InFace(f) = locus else {
            return i;
        };
        if sectors[i].face == f {
            return i;
        }
        let n = sectors.len();
        let k = match read {
            (SideCode::On, _) => (i + 1) % n,
            (_, SideCode::On) => (i + n - 1) % n,
            _ => return i,
        };
        if sectors[k].face == f { k } else { i }
    };
    let moved = PairRecord {
        a: across(a, r.a, raw.sa, loci.0),
        b: across(b, r.b, raw.sb, loci.1),
        ..r
    };
    if (moved.a, moved.b) == (r.a, r.b) {
        return Ok(moved);
    }
    let (fa, fb) = super::sectors::pair_codes(&a[moved.a], &b[moved.b], band)?;
    let stands = |kept: (SideCode, SideCode), fresh: (SideCode, SideCode)| {
        [(kept.0, fresh.0), (kept.1, fresh.1)]
            .into_iter()
            .all(|(k, f)| f == SideCode::On || k == f)
    };
    if stands(moved.sa, fa) && stands(moved.sb, fb) {
        Ok(moved)
    } else {
        Err(BooleanError::ClassificationInvariant {
            what: "a germ moved across a tangent bound reads other crossing codes in the sector \
                   across it",
        })
    }
}

/// The null edges of one vertex pair, every reading taken: what
/// [`mint_plans`] mints without reading the orbit's geometry again.
#[derive(Clone, Debug)]
pub(super) struct NullPlan<T: geom_core::Real> {
    /// The vertex pair.
    pub contact: VvContact,
    /// One `[A, B]` run per null-edge pair, in A's walk order.
    runs: Vec<[SideRun<T>; 2]>,
    /// How many germs each solid's walk orders, two per run: the
    /// length of the walk [`SideRun::walk`] reads positions in.
    walk_len: usize,
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
    declared: &super::GluedPairs<T>,
    contacts: &super::ReductionContacts,
    op: BooleanOp,
    band: Band,
) -> Result<NullPlan<T>, BooleanError> {
    let (mut survivors, mut raw): (Vec<&PairRecord>, Vec<&PairRecord>) = records
        .iter()
        .zip(raw)
        .filter(|(r, _)| r.survives())
        .unzip();
    if pairs_reversed() {
        survivors.reverse();
        raw.reverse();
    }
    let mut plan = NullPlan {
        contact,
        runs: Vec::new(),
        walk_len: survivors.len(),
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
    // Every germ's cells are read before the first mint moves an orbit.
    let loci = survivors
        .iter()
        .zip(&raw)
        .map(|(r, w)| {
            super::sectors::germ_loci(
                super::sectors::GermSide {
                    body: a_body,
                    operand: Operand::A,
                    site: contact.a,
                    sector: &a_sectors[r.a],
                    read: w.sa,
                },
                super::sectors::GermSide {
                    body: b_body,
                    operand: Operand::B,
                    site: contact.b,
                    sector: &b_sectors[r.b],
                    read: w.sb,
                },
                contacts,
                declared,
                band,
            )
        })
        .collect::<Result<Vec<_>, BooleanError>>()?;
    // A germ only tangent to a bound of its sector lies in the face
    // across that bound ([`super::sectors::germ_loci`]): it is that
    // face's germ, so it is minted in the sector across the bound.
    let survivors: Vec<PairRecord> = survivors
        .iter()
        .zip(&raw)
        .zip(&loci)
        .map(|((&&r, w), &l)| across_tangent(r, w, l, (a_sectors, b_sectors), band))
        .collect::<Result<_, _>>()?;
    // A germ along an edge runs along it: its two flankers may be
    // coplanar (an edge-edge germ is the pair of the two solids' own
    // fold flankers) or tangent (a germ only tangent to the other
    // solid's edge), so the planes' intersection is not its direction;
    // the bound read On is, the A flanker's where both solids hold the
    // edge.
    let on_bound = |s: &BoolSector<T>, read: (SideCode, SideCode)| {
        let bound = if read.0 == SideCode::On {
            s.start
        } else {
            s.end
        };
        Leaned::upright(bound.normalize())
    };
    let dirs = survivors
        .iter()
        .enumerate()
        .map(|(i, r)| match loci[i] {
            (super::Locus::OnEdge(_), _) => Ok(on_bound(&a_sectors[r.a], raw[i].sa)),
            (_, super::Locus::OnEdge(_)) => Ok(on_bound(&b_sectors[r.b], raw[i].sb)),
            _ => record_germ_dir(
                a_body,
                b_body,
                &a_sectors[r.a],
                &b_sectors[r.b],
                declared,
                band,
            ),
        })
        .collect::<Result<Vec<_>, BooleanError>>()?;
    // Each germ's branch, decided once, at the A site.
    let site = a_body.resolve_vertex_point(contact.a, Proven);
    let branches = survivors
        .iter()
        .zip(&loci)
        .map(|(r, &l)| {
            super::join::germ_branch(
                &declared.sections,
                (a_body, a_sectors[r.a].face),
                (b_body, b_sectors[r.b].face),
                l,
                site,
                band,
            )
        })
        .collect::<Result<Vec<_>, BooleanError>>()?;
    // Each survivor's cut in each solid's walk. A germ along an edge of
    // one solid lies, in the other solid's walk, beside that edge, where
    // the edge's flanker flush with the walk's sector lies
    // ([`beside_order`]): A's walk reads B's edges' flankers, and B's
    // walk A's.
    let cuts = |entry: fn(&PairRecord) -> usize,
                on_edge: fn(&(super::Locus, super::Locus)) -> bool,
                edge_sectors: &[BoolSector<T>],
                read: fn(&PairRecord) -> (usize, (SideCode, SideCode))| {
        survivors
            .iter()
            .zip(&raw)
            .zip(&loci)
            .zip(&dirs)
            .map(|(((r, w), l), &line)| {
                let (k, codes) = read(w);
                Cut {
                    entry: entry(r),
                    line,
                    beside: on_edge(l)
                        .then(|| flankers_of(edge_sectors, k, codes))
                        .flatten(),
                }
            })
            .collect::<Vec<_>>()
    };
    let a_cuts = cuts(
        |r| r.a,
        |l| matches!(l.1, super::Locus::OnEdge(_)),
        b_sectors,
        |r| (r.b, r.sb),
    );
    let b_cuts = cuts(
        |r| r.b,
        |l| matches!(l.0, super::Locus::OnEdge(_)),
        a_sectors,
        |r| (r.a, r.sa),
    );
    // Each solid's walk order of the survivors ([`walk_order`]). Two
    // survivors pair with each other and run either way round, so only
    // more need one.
    let n = survivors.len();
    let (mut a_order, b_order) = if n > 2 {
        (
            walk_order(a_sectors, &a_cuts, band)?,
            walk_order(b_sectors, &b_cuts, band)?,
        )
    } else {
        ((0..n).collect(), (0..n).collect())
    };
    // Pair consecutively in A's walk order, from the start
    // [`pairing_start_turns`] picks.
    if n > 2 && pairing_start_turns(survivors[a_order[0]].sa.0, op) {
        a_order.rotate_left(1);
    }
    let positions = |order: &[usize]| {
        let mut pos = vec![0; n];
        for (p, &i) in order.iter().enumerate() {
            pos[i] = p;
        }
        pos
    };
    let (a_pos, b_pos) = (positions(&a_order), positions(&b_order));
    // F12 guard 1: the pairs do not cross in B's walk order either
    // ([`b_runs`]), checked for every pair before the first mint.
    let pairs: Vec<(usize, usize)> = a_order.chunks(2).map(|c| (c[0], c[1])).collect();
    let b_runs = b_runs(n, &pairs, &b_pos).map_err(|refusal| match refusal {
        ArcRefusal::Crossing => BooleanError::PairingMismatch {
            a_vertex: contact.a,
            b_vertex: contact.b,
        },
        ArcRefusal::Cover => BooleanError::ClassificationInvariant {
            what: "two runs of a plan read from its walk's first entry cover the walk",
        },
    })?;
    let shared = pairs.len() > 1;
    for (&(i0, i1), &(b_run, _)) in pairs.iter().zip(&b_runs) {
        // A's pairs are consecutive in its own walk order, so each runs
        // forward, but for two survivors, which run either way.
        let a_run = walk_run(n, a_pos[i0], a_pos[i1]);
        let (r0, r1) = (survivors[i0], survivors[i1]);
        let g0_faces = ((a_sectors[r0.a].face, b_sectors[r0.b].face), loci[i0]);
        let g1_faces = ((a_sectors[r1.a].face, b_sectors[r1.b].face), loci[i1]);
        // Which way round each solid runs is its own walk order's
        // ([`walk_run`]): in A the run that holds no third germ, in B a
        // nested pair's run holds other pairs' runs whole. Two survivors
        // follow each other both ways, and the run that swallows the
        // entire orbit — impossible for the true wedge, whose far side
        // the complementary germ bounds — is the reverse of the one
        // minted. The run-side agreement guard runs against whichever
        // direction is chosen.
        let side = |sectors: &[BoolSector<T>],
                    body: &Body<T>,
                    order: Option<bool>,
                    g0: Germ<T>,
                    g1: Germ<T>,
                    pos: &[usize]| {
            let swapped = match order {
                Some(swapped) => swapped,
                None => run_degenerates(body, sectors, g0.cut.entry, g1.cut.entry)?,
            };
            Ok::<_, BooleanError>(SideRun {
                shared,
                ..SideRun::directed(g0, g1, swapped, [pos[i0], pos[i1]])
            })
        };
        plan.runs.push([
            side(
                a_sectors,
                a_body,
                a_run,
                Germ::new(a_cuts[i0], r0.sa, g0_faces, branches[i0]),
                Germ::new(a_cuts[i1], r1.sa, g1_faces, branches[i1]),
                &a_pos,
            )?,
            side(
                b_sectors,
                b_body,
                b_run,
                Germ::new(b_cuts[i0], r0.sb, g0_faces, branches[i0]),
                Germ::new(b_cuts[i1], r1.sb, g1_faces, branches[i1]),
                &b_pos,
            )?,
        ]);
    }
    let holders: Vec<Option<usize>> = b_runs.iter().map(|&(_, h)| h).collect();
    let strut = plan
        .runs
        .iter()
        .map(|r| is_strut(b_sectors, r[1].from.cut.entry, r[1].to.cut.entry))
        .collect::<Result<Vec<bool>, BooleanError>>()?;
    for (r, held) in plan.runs.iter_mut().zip(held_by(&holders, &strut)) {
        r[1].held = held;
    }
    Ok(plan)
}

/// Each run's [`Held`] from its innermost `holders` in its plan
/// ([`arc_holders`]) and whether each run is a strut: the chain of runs
/// that hold it, innermost first, gives its depth, the innermost fan
/// among them and whether the innermost is a strut.
fn held_by(holders: &[Option<usize>], strut: &[bool]) -> Vec<Option<Held>> {
    (0..holders.len())
        .map(|k| {
            let chain: Vec<usize> = std::iter::successors(holders[k], |&h| holders[h]).collect();
            chain.first().map(|&h| Held {
                depth: chain.len(),
                holder: h,
                fan: chain.iter().copied().find(|&h| !strut[h]),
                by_strut: strut[h],
            })
        })
        .collect()
}

/// Whether the pairing starts at A's second germ rather than its first:
/// where the first's forward run, on side `first`, is not the side `op`
/// keeps of A. The runs alternate In and Out round the vertex, so either
/// start pairs the corner, and this one makes each run A keeps a copy of
/// its own, so no kept vertex of A holds two null edges.
///
/// B is not consulted. Both solids pair the same germs, so the start
/// that fixes A's runs fixes B's too: B keeps either its run copies,
/// and the result pinches at the point, or its own vertex with every
/// null edge there, and the finish builds both. Only one solid's side
/// can be chosen; A's is the one the finish needs.
fn pairing_start_turns(first: SideCode, op: BooleanOp) -> bool {
    first != super::finish::kept_side(op, Operand::A)
}

/// Which way round a solid runs the null edge between the survivors at
/// walk positions `p0` and `p1` of `n`: `Some(false)` forward from
/// `p0`, `Some(true)` forward from `p1`, the one each follows the other
/// from; `None` where either way holds no third germ (two survivors).
/// Neither cyclically adjacent: no run between them is one.
fn run_order(n: usize, p0: usize, p1: usize) -> Option<Option<bool>> {
    match ((p0 + 1) % n == p1, (p1 + 1) % n == p0) {
        (true, true) => Some(None),
        (true, false) => Some(Some(false)),
        (false, true) => Some(Some(true)),
        (false, false) => None,
    }
}

/// **Which way round a solid runs the pair at its walk positions `p0`
/// and `p1` of `n`**, for both solids and every survivor count: as
/// [`run_order`] says where the two are adjacent, so the run holds no
/// third germ, `None` where either way does (two survivors); else
/// forward from the earlier position to the later, so the run is an
/// interval of the walk order ([`b_runs`]).
fn walk_run(n: usize, p0: usize, p1: usize) -> Option<bool> {
    run_order(n, p0, p1).unwrap_or(Some(p1 < p0))
}

/// **B's run for each of A's `pairs`, and the pair whose run holds it.**
/// `b_pos` is each survivor's position in B's walk order. Each pair
/// joins the ends of one of A's runs, and A's runs on one side of B's
/// surface are disjoint arcs in one of the two disks B's link bounds on
/// the sphere round the vertex, so the pairs cannot cross in B's walk
/// order. At four crossings that makes each pair adjacent in B too; at
/// six or more a pair may hold others between its two germs either way
/// round (a nested matching). Each runs as [`walk_run`] says, so the
/// runs are arcs of B's walk order ([`arc_holders`]): disjoint
/// or nested. A pair whose run holds another's
/// mints first, and the one it holds mints at its copy ([`mint_plans`]).
/// [`ArcRefusal::Crossing`]: two pairs cross, and the walks are not two simple links'.
fn b_runs(n: usize, pairs: &[(usize, usize)], b_pos: &[usize]) -> Result<Vec<BRun>, ArcRefusal> {
    let runs: Vec<Option<bool>> = pairs
        .iter()
        .map(|&(i0, i1)| walk_run(n, b_pos[i0], b_pos[i1]))
        .collect();
    let arcs: Vec<[usize; 2]> = pairs
        .iter()
        .zip(&runs)
        .map(|(&(i0, i1), run)| match run {
            Some(true) => [b_pos[i1], b_pos[i0]],
            _ => [b_pos[i0], b_pos[i1]],
        })
        .collect();
    let holders = arc_holders(n, &arcs)?;
    Ok(runs.into_iter().zip(holders).collect())
}

/// A pair's run in B ([`b_runs`]): which way round, as [`walk_run`]
/// says, and the pair whose run holds it innermost.
type BRun = (Option<bool>, Option<usize>);

/// Why [`arc_holders`] refuses a plan's arcs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ArcRefusal {
    /// An arc holds one end of another: their pairs cross in the walk.
    Crossing,
    /// Two arcs each hold the other's ends, so they cover the walk.
    Cover,
}

/// **Which of one plan's runs holds which, read as arcs of their
/// solid's walk.** `arcs[k]` is run `k`'s two walk positions of `n`,
/// `from`'s first, and the run is the arc walked forward between them.
/// A run holds another when its arc holds both the other's positions
/// ([`arc_holds`]);
/// returns each run's innermost holder, the shortest arc holding it.
/// Refuses [`ArcRefusal::Crossing`] where an arc holds one position of
/// another: two pairs crossing, which only [`b_runs`] can meet, as it
/// checks a reading of B's walk. Two arcs of non-crossing pairs, either
/// way round, are disjoint, nested, or each holds the other's ends, so
/// a turn ([`reconcile_pass`]), which keeps a run's ends, can only make
/// two runs cover the walk between them: [`ArcRefusal::Cover`], which no
/// known pose reaches. Otherwise every run's holders nest, and the
/// innermost names the rest.
fn arc_holders(n: usize, arcs: &[[usize; 2]]) -> Result<Vec<Option<usize>>, ArcRefusal> {
    let len = |[from, to]: [usize; 2]| (to + n - from) % n;
    let before = |p: usize, q: usize| Ok::<_, std::convert::Infallible>((p != q).then_some(p < q));
    let holds = |o: usize, k: usize| {
        let Ok(held) = arc_holds(before, arcs[o], arcs[k]);
        held
    };
    let mut out = Vec::with_capacity(arcs.len());
    for k in 0..arcs.len() {
        let mut holders = Vec::new();
        for o in (0..arcs.len()).filter(|&o| o != k) {
            match (holds(o, k), holds(k, o)) {
                ([true, false] | [false, true], _) | (_, [true, false] | [false, true]) => {
                    return Err(ArcRefusal::Crossing);
                }
                ([true, true], [true, true]) => return Err(ArcRefusal::Cover),
                ([true, true], _) => holders.push(o),
                _ => {}
            }
        }
        let inner = holders.iter().copied().min_by_key(|&o| len(arcs[o]));
        debug_assert!(
            inner.is_none_or(|i| holders
                .iter()
                .all(|&o| o == i || holds(o, i) == [true, true])),
            "the holders of run {k} nest: {arcs:?}"
        );
        out.push(inner);
    }
    Ok(out)
}

/// The survivors' walk order round one solid's orbit ([`walks_before`])
/// at their `cuts`: the order's ranks, the walk positions
/// [`arc_holders`] reads; a tie refuses.
fn walk_order<T: Decide>(
    sectors: &[BoolSector<T>],
    cuts: &[Cut<T>],
    band: Band,
) -> Result<Vec<usize>, BooleanError> {
    let mut order: Vec<usize> = Vec::with_capacity(cuts.len());
    for i in 0..cuts.len() {
        let mut at = order.len();
        for (p, &j) in order.iter().enumerate() {
            let before = walks_before(sectors, cuts[i], cuts[j], band)?.ok_or(
                BooleanError::ClassificationInvariant {
                    what: "two crossing germs of a vertex pair lie along one direction in one \
                           sector entry",
                },
            )?;
            if before {
                at = p;
                break;
            }
        }
        order.insert(at, i);
    }
    Ok(order)
}

impl<T: geom_core::Real> SideRun<T> {
    fn directed(g0: Germ<T>, g1: Germ<T>, swapped: bool, [p0, p1]: [usize; 2]) -> Self {
        let ((from, to), walk) = if swapped {
            ((g1, g0), [p1, p0])
        } else {
            ((g0, g1), [p0, p1])
        };
        Self {
            from,
            to,
            swapped,
            shared: false,
            walk,
            held: None,
        }
    }

    /// The same null edge run the other way round its orbit.
    fn reversed(self) -> Self {
        Self {
            from: self.to,
            to: self.from,
            swapped: !self.swapped,
            shared: self.shared,
            walk: [self.walk[1], self.walk[0]],
            held: self.held,
        }
    }
}

/// Mints every plan's null edges, each solid in its own order: a run
/// its own plan's runs hold after each of them, at the copy of the
/// innermost fan among them ([`Held`]); then at a shared vertex its struts
/// before any fan, each strut after every strut whose segment holds its
/// own ([`nests`]), so it can hang at that strut's tip.
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
    // The pairs' order: plans that hang a shared strut first, and those
    // runs first within a plan (`join::find_match` breaks ties by it).
    let hangs = |i: usize, k: usize| {
        let r = &plans[i].runs[k];
        [orbits[i].0, orbits[i].1]
            .into_iter()
            .zip(r)
            .any(|(secs, r)| {
                r.shared && is_strut(secs, r.from.cut.entry, r.to.cut.entry).is_ok_and(|s| s)
            })
    };
    let mut order: Vec<usize> = (0..plans.len()).collect();
    order.sort_by_key(|&i| !(0..plans[i].runs.len()).any(|k| hangs(i, k)));
    let runs: Vec<(usize, usize)> = order
        .into_iter()
        .flat_map(|i| {
            let mut ks: Vec<usize> = (0..plans[i].runs.len()).collect();
            ks.sort_by_key(|&k| !hangs(i, k));
            ks.into_iter().map(move |k| (i, k))
        })
        .collect();
    let mut edges: [Vec<Option<EdgeKey>>; 2] = [vec![None; runs.len()], vec![None; runs.len()]];
    for (slot, operand) in [Operand::A, Operand::B].into_iter().enumerate() {
        let vertex = |i: usize| {
            let c = plans[i].contact;
            if slot == 0 { c.a } else { c.b }
        };
        let orbit = |i: usize| if slot == 0 { orbits[i].0 } else { orbits[i].1 };
        let run = |(i, k): (usize, usize)| plans[i].runs[k][slot];
        let shared_strut = |at: (usize, usize)| -> Result<bool, BooleanError> {
            let r = run(at);
            Ok(r.shared && is_strut(orbit(at.0), r.from.cut.entry, r.to.cut.entry)?)
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
            // A run a fan holds after every run that holds it, at the
            // fan's copy: the reconcile cleared that fan of every other
            // pair's cut ([`reconcile_pass`]), so nothing there nests a
            // strut of theirs. A run only struts hold mints at the plan's
            // own vertex, whose struts another pair's strut may nest in
            // ([`held_cut`] leaves a nested strut to [`holds_whole`]), so
            // it sorts among that vertex's struts by its nesting `depth`,
            // which counts its own holders: the walk's arcs and the
            // sectors' geometry agree on them, asserted below. Ties at one
            // `depth` break by `n`; a `holds_whole` read in band that
            // called nested struts disjoint would leave a false tie, which
            // no pose exercises. Then the vertex's struts before its fans.
            let held = run(at)
                .held
                .filter(|h| h.fan.is_some())
                .map_or(0, |h| h.depth);
            keyed.push((held, depth.is_none(), depth.unwrap_or(0), n));
        }
        if cfg!(debug_assertions) {
            for &(_, _, d, n) in &keyed {
                let (i, k) = runs[n];
                if let Some(h) = run((i, k)).held.filter(|h| h.fan.is_none()) {
                    let m = runs.iter().position(|&r| r == (i, h.holder));
                    let holder_d = m.map(|m| keyed.iter().find(|e| e.3 == m).map_or(0, |e| e.2));
                    debug_assert!(
                        shared_strut((i, k)).unwrap_or(false)
                            && m.is_some_and(|m| shared_strut(runs[m]).unwrap_or(false))
                            && holder_d.is_some_and(|hd| d > hd),
                        "a run only struts hold nests deeper than its strut holder \
                         by the sectors' geometry: run {k} of plan {i} at depth {d}, \
                         its holder {} at {holder_d:?}",
                        h.holder
                    );
                }
            }
        }
        keyed.sort();
        let body = if slot == 0 {
            &mut *a_body
        } else {
            &mut *b_body
        };
        let mut hung = Hung::default();
        // Each minted run's copy and its null half that starts there.
        let mut copies: Vec<Option<(VertexKey, HalfEdgeKey)>> = vec![None; runs.len()];
        for (.., n) in keyed {
            let (i, k) = runs[n];
            // A held run mints at the copy of the innermost fan that holds
            // it, and a strut holding it there hangs it at its tip.
            let held = run((i, k)).held;
            let fan_copy = match held.and_then(|h| h.fan) {
                Some(f) => {
                    let m = runs
                        .iter()
                        .position(|&r| r == (i, f))
                        .and_then(|m| copies[m]);
                    Some(m.ok_or(BooleanError::ClassificationInvariant {
                        what: "a nested null edge's holder was not minted before it",
                    })?)
                }
                None => None,
            };
            let mismatch = || BooleanError::PairingMismatch {
                a_vertex: plans[i].contact.a,
                b_vertex: plans[i].contact.b,
            };
            let site = MintSite {
                operand,
                vertex: fan_copy.map_or(vertex(i), |(v, _)| v),
                fan_half: fan_copy.map(|(_, h)| h),
                by_strut: plans[i]
                    .runs
                    .iter()
                    .any(|r| r[slot].held.is_some())
                    .then(|| held.is_some_and(|h| h.by_strut)),
            };
            let (mut rec, copy) = mint_directed(
                body,
                site,
                orbit(i),
                plans[i].runs[k][slot],
                &mut hung,
                mismatch,
                band,
            )?;
            copies[n] = Some(copy);
            // Slot canonicalization (the joining's slot lock): germ slot
            // i of the A and B records must be the SAME spatial germ; a
            // degeneracy swap in one solid only would misalign them, so
            // align B's array to A's (entries carry their halves — the
            // germ↔half binding is untouched).
            let [a_run, b_run] = plans[i].runs[k];
            if operand == Operand::B && a_run.swapped != b_run.swapped {
                rec.germs.swap(0, 1);
            }
            edges[slot][n] = Some(rec.edge);
            out.edges.push(rec);
        }
    }
    for (n, &(i, _)) in runs.iter().enumerate() {
        let (Some(a_edge), Some(b_edge)) = (edges[0][n], edges[1][n]) else {
            return Err(BooleanError::ClassificationInvariant {
                what: "a planned null edge was not minted",
            });
        };
        out.pairs.push(NullEdgePairRecord {
            a_edge,
            b_edge,
            site: PairSite::VertexVertex(plans[i].contact),
        });
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
    let strut = |r: SideRun<T>| is_strut(secs, r.from.cut.entry, r.to.cut.entry);
    Ok(strut(outer)?
        && strut(inner)?
        && holds_whole(
            secs,
            segment(secs, outer, band)?,
            segment(secs, inner, band)?,
            band,
        )?)
}

/// A run's two ends in walk order, and its piece's code between them,
/// the first end's forward code. The walk order is a fan's leaving germ
/// then its closing germ, a strut's earlier germ through their one
/// physical sector then its later. A pair of more than two survivors
/// runs from the earlier in its solid's walk order already; one of two
/// runs in record order, so its strut's ends may come reversed. A strut
/// whose germs the walk ties is an invariant: a pair's two crossings
/// are distinct.
type Segment<T> = (Cut<T>, Cut<T>, SideCode);

/// `run`'s [`Segment`].
fn segment<T: Decide>(
    secs: &[BoolSector<T>],
    run: SideRun<T>,
    band: Band,
) -> Result<Segment<T>, BooleanError> {
    let (from, to) = (run.from.cut, run.to.cut);
    Ok(if leads(secs, run, band)? {
        (from, to, run.from.codes.0)
    } else {
        (to, from, run.to.codes.0)
    })
}

/// **Whether the strut segment `outer` holds the strut segment `inner`
/// whole**: both in one physical sector, and `outer`'s arc holds both
/// `inner`'s germs ([`arc_holds`]), an end along `outer`'s counting
/// inside (a pinch line in the corner's face). The inner strut then
/// hangs at the outer's tip ([`mint_plans`]).
///
/// Two struts tied at both ends (one arc) are two pieces touching along
/// both directions, one In on the segment and the other Out. The Out
/// one holds: the vertex between the two, the holder's tip and the
/// inner's root, keeps the corner outside both pieces, which is empty
/// in the corner's face. The other way round it would be inside both.
/// One code for both is an invariant: two pieces of one operand In on
/// the segment would overlap there, and two Out on it are each In
/// on the rest of the orbit, and overlap there.
fn holds_whole<T: Decide>(
    secs: &[BoolSector<T>],
    (lo, hi, code): Segment<T>,
    (ilo, ihi, icode): Segment<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    if secs[lo.entry].he != secs[ilo.entry].he {
        return Ok(false);
    }
    let before = |p: Cut<T>, q: Cut<T>| walks_before(secs, p, q, band);
    if (before(lo, ilo)?, before(ihi, hi)?) == (None, None) {
        return match (code, icode) {
            (SideCode::Out, SideCode::In) => Ok(true),
            (SideCode::In, SideCode::Out) => Ok(false),
            _ => Err(BooleanError::ClassificationInvariant {
                what: "two dangling null edges with one segment and one code: two pieces of one \
                       operand overlap at the point",
            }),
        };
    }
    Ok(arc_holds(before, [lo, hi], [ilo, ihi])? == [true, true])
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
/// make another's nested run the one to turn), and the runs of its own
/// pair it then holds mint at its copy ([`hang_at_shared`]); cuts along one
/// direction lie where the orbit's order reads their tie, by the sides
/// of the line they lie beside ([`cut_order`]), and where it does not,
/// by the runs ([`tied_held`]). A strut (whose
/// other way round is the whole orbit) whose segment holds another
/// pair's strut whole keeps it, the inner hanging at its tip
/// ([`holds_whole`]): a piece with a reflex corner at the point leaves
/// such a segment outside it. Another pair's cut strictly inside a
/// strut's segment lies outside the strut's piece, and so does that
/// pair's piece's In arc from it, up to the segment's ends: both its
/// cuts lie in the segment, and its run is a strut that nests (the
/// other way round would be the whole orbit). A null edge both of
/// whose runs hold another pair's cut otherwise refuses
/// [`BooleanError::SharedVertexCrossings`]: a fan whose two ways round
/// both hold one, which needs a pair paired across the arcs outside its
/// piece. Two crossing pairs that share both their
/// vertices refuse
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
            return hang_at_shared(plans, sectors);
        }
    }
    Err(BooleanError::ClassificationInvariant {
        what: "the runs at a shared vertex do not settle",
    })
}

/// **A run turned round a shared vertex holds what lay outside it.**
/// [`reconcile_pass`] turns a run onto its complement to clear another
/// pair's cuts, and the runs of its own pair that lay outside it then
/// lie inside that complement, while those it held lie outside. So at
/// each shared vertex every plan's holders are read again off its runs'
/// arcs ([`arc_holders`]), B's nested pairings and turned runs as one
/// laminar family, and each held run mints after its holders, at the
/// copy of the innermost fan among them ([`Held`]): the shared vertex
/// keeps what lies outside the plan's outermost runs, with the other
/// pairs' ends, and each copy what its own fan holds, so no vertex is
/// the In end of one null edge and the Out end of another
/// ([`mint_directed`]). A run only struts hold stays at the shared
/// vertex, among its struts ([`mint_plans`]). A turned run and a run of
/// its plan that held it would cover the walk between them, and refuse
/// [`BooleanError::SharedVertexCrossings`]: a backstop no known pose
/// reaches.
fn hang_at_shared<T: Decide>(
    plans: &mut [NullPlan<T>],
    sectors: &[Orbits<'_, T>],
) -> Result<(), BooleanError> {
    for slot in 0..2 {
        for i in 0..plans.len() {
            let Some(at) = shared_at(plans, sectors, slot, i) else {
                continue;
            };
            let runs: Vec<SideRun<T>> = plans[i].runs.iter().map(|r| r[slot]).collect();
            let strut = runs
                .iter()
                .map(|r| is_strut(at.secs, r.from.cut.entry, r.to.cut.entry))
                .collect::<Result<Vec<bool>, BooleanError>>()?;
            let partners = [at.partner, ends(plans[at.others[0]].contact, slot).1];
            let arcs: Vec<[usize; 2]> = runs.iter().map(|r| r.walk).collect();
            debug_assert_eq!(plans[i].walk_len, 2 * runs.len(), "two germs per run");
            let holders =
                arc_holders(plans[i].walk_len, &arcs).map_err(|refusal| match refusal {
                    ArcRefusal::Cover => BooleanError::SharedVertexCrossings {
                        operand: at.operand,
                        vertex: at.vertex,
                        partners,
                    },
                    ArcRefusal::Crossing => BooleanError::ClassificationInvariant {
                        what: "two runs of one plan cross in its walk after the reconcile",
                    },
                })?;
            let held = held_by(&holders, &strut);
            for (r, held) in plans[i].runs.iter_mut().zip(held) {
                r[slot].held = held;
            }
        }
    }
    Ok(())
}

/// Contact `c`'s two vertices as solid `slot` reads it: its own, then
/// the other solid's.
fn ends(c: VvContact, slot: usize) -> (VertexKey, VertexKey) {
    if slot == 0 { (c.a, c.b) } else { (c.b, c.a) }
}

/// Plan `i`'s reading of its vertex's orbit in solid `slot`.
fn orbit_of<'s, T: geom_core::Real>(
    sectors: &[Orbits<'s, T>],
    slot: usize,
    i: usize,
) -> &'s [BoolSector<T>] {
    if slot == 0 {
        sectors[i].0
    } else {
        sectors[i].1
    }
}

/// A crossing plan whose vertex in one solid other crossing plans share
/// ([`shared_at`]).
struct SharedAt<'s, T: geom_core::Real> {
    operand: Operand,
    vertex: VertexKey,
    /// The plan's vertex in the other solid.
    partner: VertexKey,
    /// The other crossing plans at `vertex`.
    others: Vec<usize>,
    /// The plan's reading of `vertex`'s orbit.
    secs: &'s [BoolSector<T>],
}

/// Crossing plan `i` read in solid `slot`, where another crossing plan
/// shares its vertex there; `None` elsewhere. What [`reconcile_pass`]
/// and [`hang_at_shared`] read at a shared vertex.
fn shared_at<'s, T: geom_core::Real>(
    plans: &[NullPlan<T>],
    sectors: &[Orbits<'s, T>],
    slot: usize,
    i: usize,
) -> Option<SharedAt<'s, T>> {
    let (vertex, partner) = ends(plans[i].contact, slot);
    let others: Vec<usize> = (0..plans.len())
        .filter(|&j| j != i && !plans[j].runs.is_empty())
        .filter(|&j| ends(plans[j].contact, slot).0 == vertex)
        .collect();
    (!plans[i].runs.is_empty() && !others.is_empty()).then(|| SharedAt {
        operand: [Operand::A, Operand::B][slot],
        vertex,
        partner,
        others,
        secs: orbit_of(sectors, slot, i),
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
    for slot in 0..2 {
        let body = bodies[slot];
        let key = |c: VvContact| ends(c, slot);
        for i in 0..plans.len() {
            let Some(SharedAt {
                operand,
                vertex,
                partner,
                others,
                secs,
            }) = shared_at(plans, sectors, slot, i)
            else {
                continue;
            };
            let orbit = |j: usize| orbit_of(sectors, slot, j);
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
                    cuts.extend(run_cuts(secs, r[slot], j, band)?);
                }
            }
            for k in 0..plans[i].runs.len() {
                let current = plans[i].runs[k][slot];
                let mut blocker = None;
                let mut chosen = None;
                for run in [current, current.reversed()] {
                    // Not the run rule ([`walk_run`]): whether this way
                    // round can mint at all.
                    if run_degenerates(body, secs, run.from.cut.entry, run.to.cut.entry)? {
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

/// `run`'s two cuts, as [`OtherCut`]s of `owner`.
fn run_cuts<T: Decide>(
    secs: &[BoolSector<T>],
    run: SideRun<T>,
    owner: usize,
    band: Band,
) -> Result<[OtherCut<T>; 2], BooleanError> {
    let (lo, hi, code) = segment(secs, run, band)?;
    let strut = is_strut(secs, run.from.cut.entry, run.to.cut.entry)?;
    let cut = |at, mate, leaves| OtherCut {
        at,
        mate,
        leaves,
        owner,
        strut,
        code,
    };
    Ok([cut(lo, hi, true), cut(hi, lo, false)])
}

/// Another pair's cut at a shared vertex: where it lies, the other end
/// of its run, whether that run lies after it walking forward (else
/// before it), and the pair.
#[derive(Clone, Copy, Debug)]
struct OtherCut<T: geom_core::Real> {
    at: Cut<T>,
    mate: Cut<T>,
    leaves: bool,
    /// The plan whose run it is.
    owner: usize,
    /// Whether its run is a strut.
    strut: bool,
    /// Its piece's code on its run ([`Segment`]).
    code: SideCode,
}

/// Whether `run`'s first end in walk order ([`Segment`]) is its `from`.
fn leads<T: Decide>(
    secs: &[BoolSector<T>],
    run: SideRun<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    if !is_strut(secs, run.from.cut.entry, run.to.cut.entry)? {
        return Ok(true);
    }
    walks_before(secs, run.from.cut, run.to.cut, band)?.ok_or(
        BooleanError::ClassificationInvariant {
            what: "a dangling null edge whose two germs lie along one direction",
        },
    )
}

/// The owner of the first cut `run` holds, walking its orbit forward
/// from its first end to its last: a cut in an entry the walk crosses
/// whole, or in an end entry on the run's side of that end's germ, or a
/// cut the orbit's order ties with an end germ ([`cut_order`]) that
/// [`tied_held`] counts. A strut holds a cut of its own physical sector
/// between its two germs, or tied with either that [`tied_held`]
/// counts, unless that cut's strut
/// and it nest ([`nested`]).
fn held_cut<T: Decide>(
    secs: &[BoolSector<T>],
    run: SideRun<T>,
    cuts: &[OtherCut<T>],
    band: Band,
) -> Result<Option<usize>, BooleanError> {
    let own = segment(secs, run, band)?;
    let (lo, hi, _) = own;
    let strut = is_strut(secs, lo.entry, hi.entry)?;
    debug_assert!(
        strut || lo.entry != hi.entry,
        "a fan's two ends share entry {}: on_arc would read it as the orbit less a sliver",
        lo.entry
    );
    let before = |p: Cut<T>, q: Cut<T>| walks_before(secs, p, q, band);
    for &cut in cuts {
        let at = cut.at;
        let held =
            if strut && (secs[at.entry].he != secs[lo.entry].he || nested(secs, own, cut, band)?) {
                false
            } else {
                match on_arc(before, [lo, hi], at)? {
                    [Some(false), _] | [_, Some(false)] => false,
                    [None, _] => tied_held(secs, (lo, true), hi, cut, band)?,
                    [_, None] => tied_held(secs, (hi, false), lo, cut, band)?,
                    [Some(true), Some(true)] => true,
                }
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
    let other = (lo, hi, cut.code);
    Ok(cut.strut && (holds_whole(secs, own, other, band)? || holds_whole(secs, other, own, band)?))
}

/// **Another pair's cut along one end germ's direction** (a pinch line
/// lying flat in a face of the shared corner): whether `run` holds it.
/// Each pair's run lies on the side of the line its codes, read off its
/// own piece's faces, give it, which [`segment`] records as after or
/// before the cut. Runs on opposite sides are disjoint: not held. Runs
/// on one side are nested: not held either, as the longer holds the
/// shorter's far cut strictly and turns ([`reconcile_shared`]'s fixed
/// point); unless both ends tie, one arc, which holds, and the other
/// way round is the complement, on the opposite side, so a fan turns,
/// while a strut's is the whole orbit. Struts nest before this reading
/// ([`nested`]), at one arc too. No witness reaches a fan's one arc, nor
/// the invariant that the two directions are opposite rays (a convex
/// sector holds none).
///
/// The far ends are one arc's only where they tie in the walk's own
/// order ([`CutOrder::Tied`] with nothing reading the tie). Far ends
/// along one direction that the tie's reading orders lie at two
/// positions, so the runs would nest and the longer hold the shorter's
/// far cut strictly: not held. That case is not argued unreachable:
/// near ends the tie's reading leaves unread with far ends it reads
/// are two pieces touching along two lines, only the far one beside
/// flush faces. No pose reaches it, and the answer is unwitnessed, so
/// it refuses: a limit of the reading, in the invariant's voice until
/// a typed limit lands
/// (`work/cleave/a-fate-cut-placement-limit-refuses-in-the-voice-of-a-kernel-bug.md`).
fn tied_held<T: Decide>(
    secs: &[BoolSector<T>],
    (own, own_leaves): (Cut<T>, bool),
    far: Cut<T>,
    cut: OtherCut<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    if !super::sectors::direction_sense(own.line.dir, cut.at.line.dir, secs[own.entry].arm, band)? {
        return Err(BooleanError::ClassificationInvariant {
            what: "two cuts in one convex sector along opposite rays",
        });
    }
    if cut.leaves != own_leaves {
        return Ok(false);
    }
    match cut_order(secs, far, cut.mate, band)? {
        CutOrder::Apart(_) => Ok(false),
        CutOrder::Tied(Sign::Zero) => Ok(true),
        CutOrder::Tied(_) => Err(BooleanError::ClassificationInvariant {
            what: "a limit, not a broken invariant: two runs tied at one end, unread, whose \
                   other ends the tie's reading orders",
        }),
    }
}

/// How two cuts lie round a vertex orbit ([`cut_order`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CutOrder {
    /// In two entries, or apart within one: whether the first comes
    /// before the second.
    Apart(bool),
    /// In one entry along one direction ([`walks_after`] reads `Zero`),
    /// read one order further ([`beside_order`]): `Negative` where the
    /// first comes before the second, `Zero` where nothing reads them.
    Tied(Sign),
}

impl CutOrder {
    /// Whether the first comes before the second; `None` where nothing
    /// reads them apart.
    fn before(self) -> Option<bool> {
        match self {
            Self::Apart(before) => Some(before),
            Self::Tied(Sign::Negative) => Some(true),
            Self::Tied(Sign::Positive) => Some(false),
            Self::Tied(Sign::Zero) => None,
        }
    }
}

/// **A vertex orbit's one position order**: how cut `p` lies from cut
/// `q` walking the orbit forward from its first entry, by their entries
/// and within one entry round the sector ([`walks_after`]). Two along
/// one direction in one entry are the tie, read one order further here
/// and nowhere else: by the sides of the line they lie beside
/// ([`beside_order`]). The first entry opens a physical sector
/// (`build_sectors` emits a corner's edge-bounded entry first), so no
/// physical sector wraps past the origin and two cuts of one read in
/// their walk through it.
fn cut_order<T: Decide>(
    sectors: &[BoolSector<T>],
    p: Cut<T>,
    q: Cut<T>,
    band: Band,
) -> Result<CutOrder, BooleanError> {
    if !sectors.first().is_some_and(BoolSector::end_edge) {
        return Err(BooleanError::ClassificationInvariant {
            what: "a vertex orbit whose first entry opens no physical sector",
        });
    }
    if p.entry != q.entry {
        return Ok(CutOrder::Apart(p.entry < q.entry));
    }
    let s = &sectors[p.entry];
    Ok(match walks_after(s, p.line, q.line, band)? {
        Sign::Negative => CutOrder::Apart(true),
        Sign::Positive => CutOrder::Apart(false),
        Sign::Zero => CutOrder::Tied(beside_order(s, p, q, band)?),
    })
}

/// Whether cut `p` comes before cut `q` in the orbit's one order
/// ([`cut_order`]); `None` where nothing reads them apart.
fn walks_before<T: Decide>(
    sectors: &[BoolSector<T>],
    p: Cut<T>,
    q: Cut<T>,
    band: Band,
) -> Result<Option<bool>, BooleanError> {
    Ok(cut_order(sectors, p, q, band)?.before())
}

/// **Where `x` lies on the arc walked forward from `lo` to `hi`**, in
/// one order `before` round an orbit (a cut's, [`walks_before`], whose
/// tie reads `None`; a walk position's, `<`): `[past lo, short of hi]`,
/// each `None` where
/// `x` ties that end. An arc whose `hi` comes before its `lo` wraps
/// past the order's origin: a position not before `lo` is short of
/// `hi`, and one before `lo` is past `lo` and read against `hi`. The
/// one interval reading round an
/// orbit: what a run holds ([`held_cut`]), and whether it holds another
/// whole ([`arc_holds`]).
fn on_arc<P: Copy, E>(
    before: impl Fn(P, P) -> Result<Option<bool>, E>,
    [lo, hi]: [P; 2],
    x: P,
) -> Result<[Option<bool>; 2], E> {
    let past_lo = before(lo, x)?;
    if before(hi, lo)? != Some(true) {
        return Ok([past_lo, before(x, hi)?]);
    }
    Ok(if past_lo == Some(false) {
        [Some(true), before(x, hi)?]
    } else {
        [past_lo, Some(true)]
    })
}

/// **Which of `inner`'s ends the arc `outer` holds** ([`on_arc`]); it
/// holds `inner` whole where it holds both. An end along one of
/// `outer`'s own ends counts inside: the tie rule for every reader of
/// "holds whole". A plan's walk positions never tie ([`arc_holders`]);
/// two strut segments tied at both ends are one arc, which
/// [`holds_whole`] settles by the pieces' codes.
fn arc_holds<P: Copy, E>(
    before: impl Fn(P, P) -> Result<Option<bool>, E>,
    outer: [P; 2],
    inner: [P; 2],
) -> Result<[bool; 2], E> {
    let inside = |x| Ok(!on_arc(&before, outer, x)?.contains(&Some(false)));
    Ok([inside(inner[0])?, inside(inner[1])?])
}

/// **How cut `q` turns from cut `p`, two along one direction `d` in
/// the convex sector `s`, by the sides they lie beside**: `Negative`
/// where `q` comes after `p` walking `s` forward, as [`walks_after`]
/// reads. Each runs along one of the other solid's coincident edges
/// there (a slit's or ring's, or a pinch line's), on the side of the
/// line where that edge's flanker flush with `s` lies: the one of its
/// two flankers ([`flankers_of`]) in `s`'s plane (`bool_beside_flush`).
/// Read at second order, `d + δ op` against `d + δ oq`, that is
/// [`walks_after`] of `d` and `oq − op`: flankers on two sides read
/// apart, and so do two on one side at different offsets from the
/// line. It reads no first-order direction: [`cut_order`] reads it at
/// a tie. `Zero` where a cut runs beside no edge ([`Cut::tie_flankers`]),
/// where either edge has no one flanker flush with `s` (neither, or
/// both), or where the two flush flankers lie at one offset from the
/// line (`oq − op` along `d`: one flanker, or two mirrored across the
/// line's normal in the plane).
fn beside_order<T: Decide>(
    s: &BoolSector<T>,
    p: Cut<T>,
    q: Cut<T>,
    band: Band,
) -> Result<Sign, BooleanError> {
    let Some([bp, bq]) = p.tie_flankers(q) else {
        return Ok(Sign::Zero);
    };
    let n = s.normal.vec();
    let off = |x: Vec3<T>| {
        crate::validate::decide("bool_beside_flush", Margin::levered(n.dot(x), s.arm), band)
            .map_err(|diag| {
                BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag)
            })
    };
    let flush = |b: [Vec3<T>; 2]| -> Result<Option<Vec3<T>>, BooleanError> {
        Ok(match [off(b[0])?, off(b[1])?] {
            [Sign::Zero, Sign::Positive | Sign::Negative] => Some(b[0]),
            [Sign::Positive | Sign::Negative, Sign::Zero] => Some(b[1]),
            _ => None,
        })
    };
    let (Some(op), Some(oq)) = (flush(bp)?, flush(bq)?) else {
        return Ok(Sign::Zero);
    };
    // `oq − op` is an offset between two unit bounds, read at its own
    // length one order down: its sides are the flankers', lean 1.
    walks_after(s, p.line, Leaned::upright(oq - op), band)
}

/// The far bounds of the two sectors flanking the bound of entry `i`
/// that `codes` read On, unit: the faces either side of the edge a germ
/// read there runs along ([`beside_order`]). `None` where neither bound
/// reads On.
fn flankers_of<T: Decide>(
    sectors: &[BoolSector<T>],
    i: usize,
    codes: (SideCode, SideCode),
) -> Option<[Vec3<T>; 2]> {
    let n = sectors.len();
    let s = &sectors[i];
    match codes {
        (SideCode::On, _) => Some([s.end, sectors[(i + 1) % n].start]),
        (_, SideCode::On) => Some([s.start, sectors[(i + n - 1) % n].end]),
        _ => None,
    }
    .map(|b| b.map(Vec3::normalize))
}

/// How `y` turns from `x` walking the convex sector `s` forward, from
/// its end bound toward its start: the sign of `(x × y)·n`, `Negative`
/// where `y` lies after `x`, `Zero` where the two lie along one
/// direction. The rotation from start to end is positive about the
/// sector's normal, the convention `sectors::within` reads. It is
/// levered at the sector's arm and the two directions' leans
/// (`sectors::leaned_lever`). An in-band reading refuses as the
/// coincidence it is, as the strut order's does.
fn walks_after<T: Decide>(
    s: &BoolSector<T>,
    x: Leaned<T>,
    y: Leaned<T>,
    band: Band,
) -> Result<Sign, BooleanError> {
    let lever = leaned_lever(s.arm, (x.lean, y.lean));
    let m = Margin::levered(x.dir.cross(y.dir).dot(s.normal.vec()), lever);
    crate::validate::decide("bool_shared_cut_order", m, band)
        .map_err(|diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag))
}

/// A germ in one solid.
#[derive(Clone, Copy, Debug)]
struct Germ<T: geom_core::Real> {
    /// Its cut in the solid's walk.
    cut: Cut<T>,
    /// Its codes there.
    codes: (SideCode, SideCode),
    /// Its cells.
    cells: Cells,
    /// Its branch.
    branch: super::Branch,
}

impl<T: geom_core::Real> Germ<T> {
    fn new(cut: Cut<T>, codes: (SideCode, SideCode), cells: Cells, branch: super::Branch) -> Self {
        Self {
            cut,
            codes,
            cells,
            branch,
        }
    }
}

/// A germ's `(A face, B face)` and `(A locus, B locus)`.
pub(super) type Cells = ((FaceKey, FaceKey), (super::Locus, super::Locus));

/// Where [`mint_directed`] mints a run: in `operand` at `vertex`, the
/// plan's own or the copy of the innermost fan of its plan that holds it,
/// whose null half from that copy is `fan_half` ([`corner_bound`]).
/// `by_strut`, in a plan any of whose runs is held ([`Held`]: B's
/// nested pairing, or a run a turned run holds), says whether a strut of
/// the plan holds the run directly; a turned run is a fan, so the runs
/// it holds directly read `Some(false)`.
#[derive(Clone, Copy)]
struct MintSite {
    operand: Operand,
    vertex: VertexKey,
    fan_half: Option<HalfEdgeKey>,
    by_strut: Option<bool>,
}

/// Mints one solid's null edge along its planned run, forward
/// `from → to`: `g0 → g1` unless that run swallows the whole orbit
/// (detected structurally: the fan's orbit successor of its last edge
/// is its first — the true wedge cannot, its far side being bounded by
/// the complementary germ) or holds another pair's cut at a shared
/// vertex ([`reconcile_shared`]). The F12 run-side agreement guard
/// (`entry germ's exit code == closing germ's entry code`) applies to
/// whichever direction was planned. Returns the record with the null
/// edge's copy and its half from the copy.
fn mint_directed<T: Decide>(
    body: &mut Body<T>,
    MintSite {
        operand,
        vertex,
        fan_half,
        by_strut,
    }: MintSite,
    sectors: &[BoolSector<T>],
    run: SideRun<T>,
    hung: &mut Hung<T>,
    mismatch: impl Fn() -> BooleanError,
    band: Band,
) -> Result<(BoolNullEdgeRecord<T>, (VertexKey, HalfEdgeKey)), BooleanError> {
    let (gf, gt) = (run.from, run.to);
    // Strut spike ORDER (PR 5.5, the sort half of ssortnulledges): a
    // dangling strut's two halves splice consecutively into the loop
    // as [he_plus, he_minus]; interleaved (crossing) chords at
    // multi-germ corner sites wall pending pairs off, so the half the
    // loop walk meets FIRST (he_plus) must face the right germ
    // ([`strut_faces_first`]). Senses follow the facing by the sense
    // theorem, so only the splice order moves. Run direction is
    // untouched (a strut's reverse run spans the whole orbit).
    let empty = is_strut(sectors, gf.cut.entry, gt.cut.entry)?;
    // A strut at a shared vertex: its germs in walk order, and the
    // innermost strut hung earlier whose segment holds its own.
    let walk = if empty && run.shared {
        let own = segment(sectors, run, band)?;
        let mut holder: Option<&HungStrut<T>> = None;
        for h in hung
            .struts
            .iter()
            .filter(|h| (h.operand, h.root) == (operand, vertex))
        {
            let holds = holds_whole(sectors, h.segment, own, band)?;
            let inner = match holder {
                None => true,
                Some(o) => holds_whole(sectors, o.segment, h.segment, band)?,
            };
            if holds && inner {
                holder = Some(h);
            }
        }
        // In a nested plan the strut that holds it was named twice: by
        // the walk order ([`b_runs`]) and here by the geometry.
        if by_strut.is_some_and(|b| b != holder.is_some()) {
            return Err(BooleanError::ClassificationInvariant {
                what: "the walk order and the corner's geometry disagree on which strut holds a \
                       nested strut",
            });
        }
        Some(SharedStrut {
            segment: own,
            from_is_lo: leads(sectors, run, band)?,
            holder: holder.map(|h| (h.half, h.half_faces_lower)),
        })
    } else {
        None
    };
    let structural = if let Some(SharedStrut {
        from_is_lo,
        holder: Some((_, faces_lower)),
        ..
    }) = walk
    {
        // Hung at the tip of the strut whose segment holds it, its half
        // met first follows that strut's half from the vertex, so it
        // faces its own germ on the side of the one that half faces.
        Some(from_is_lo == faces_lower)
    } else if empty {
        // A strut's corner half still starts at the vertex: wherever
        // several runs cut it, struts mint before any fan
        // (`SideRun::shared`), and a fan is the only mint that moves a
        // sector's half. A breach refuses typed here rather than at the
        // orbit step.
        let arrival_he = corner_bound(body, vertex, sectors[gf.cut.entry].he, fan_half)?;
        let arrival = proven(&body.half_edges, arrival_he, EntityId::HalfEdge).edge;
        // At a shared vertex the corner may already hold another
        // pair's strut, so its departure edge is read off the sectors.
        // In the last corner of a fan that holds it, the corner ends at
        // the fan's null edge instead; neither edge can name a facing
        // there (`strut_facing` reads only an edge a germ runs along, and
        // no germ of the strut runs along the null edge or past the fan's
        // cut), so the sectors' edge stands for both.
        let departure_he = if run.shared {
            sectors[next_edge_bound(sectors, gf.cut.entry)].he
        } else {
            body.proven_orbit_step(arrival_he)
        };
        let departure = proven(&body.half_edges, departure_he, EntityId::HalfEdge).edge;
        let facing = strut_faces_first(
            (body, vertex),
            (operand, sectors),
            (arrival, departure),
            (gf.cut, gf.cells),
            (gt.cut, gt.cells),
            band,
        )?;
        // At a closed edge's lone vertex the walk orders the germs.
        Some(match facing {
            Some(first) => first,
            None => walk_faces_first(sectors, gf.cut, gt.cut, band)?,
        })
    } else {
        None
    };
    let spike_from_first = structural.unwrap_or(false);
    let (from, to, side, closing) = (gf.cut.entry, gt.cut.entry, gf.codes.0, gt.codes.1);
    // F12 guard 2: the closing germ must approach the run with the
    // run's own side as its entry code.
    if closing != side {
        return Err(mismatch());
    }
    // Germ facings as data (module docs): he_plus faces the from-germ,
    // he_minus the to-germ (the mev splice contract).
    let meta = [
        (gf.cells, gf.cut.line.dir, gf.branch),
        (gt.cells, gt.cut.line.dir, gt.branch),
    ];
    // A nested strut hangs at its holder's tip, past the struts hung
    // there earlier at a lower germ; any other shared strut at the
    // vertex, past those hung in its corner.
    let site = match walk {
        Some(w) => {
            // The first half past the arrival at the corner the strut
            // splices into, starting at `at`: the holder's end by
            // construction, and at `vertex` by a proven orbit step.
            let (at, first) = match w.holder {
                Some((half, _)) => (
                    body.proven_half_edge_end(half),
                    proven(&body.half_edges, half, EntityId::HalfEdge).next,
                ),
                None => (
                    vertex,
                    orbit_step_at(
                        body,
                        vertex,
                        corner_bound(body, vertex, sectors[w.segment.0.entry].he, fan_half)?,
                    ),
                ),
            };
            Some((
                at,
                strut_anchor(body, (operand, at), sectors, first, w.segment.0, hung, band)?,
            ))
        }
        // Any other strut splices past its corner's bound half.
        None if empty => Some((
            vertex,
            orbit_step_at(
                body,
                vertex,
                corner_bound(body, vertex, sectors[gf.cut.entry].he, fan_half)?,
            ),
        )),
        None => None,
    };
    let at = site.map_or(vertex, |(v, _)| v);
    let (rec, copy) = mint_run(
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
    if let Some(SharedStrut {
        segment: own,
        from_is_lo,
        ..
    }) = walk
    {
        let edge = proven(&body.edges, rec.edge, EntityId::Edge);
        let half = [(edge.he_plus, "he_plus"), (edge.he_minus, "he_minus")]
            .into_iter()
            .find(|&(h, field)| {
                linked(
                    &body.half_edges,
                    h,
                    EntityId::HalfEdge,
                    EntityId::Edge(rec.edge),
                    field,
                )
                .start
                    == at
            })
            .map(|(h, _)| h)
            .unwrap_or_else(|| {
                unreachable!(
                    "the null edge {:?} just minted at {at:?} has no half starting there: \
                     a minted edge's halves start at its two ends",
                    rec.edge
                )
            });
        hung.struts.push(HungStrut {
            operand,
            root: vertex,
            vertex: at,
            half,
            segment: own,
            half_faces_lower: (rec.germs[0].he == half) == from_is_lo,
        });
    }
    // The join reads a null half's sense off the side its start vertex
    // is the end of, so no vertex at a shared point may be the In end
    // of one null edge and the Out end of another. Each run takes its
    // own pair's region and the vertex keeps what lies outside them
    // all; a nested strut hangs at its holder's tip, and its facing
    // (above) makes that tip the same end of both.
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
    Ok((rec, copy))
}

/// A strut at a shared vertex, as [`mint_directed`] places it: its
/// segment, whether its run leaves from the lower, and the half
/// from the vertex of the innermost strut hung earlier whose segment
/// holds its own, with whether that half faces its lower germ.
#[derive(Clone, Copy)]
struct SharedStrut<T: geom_core::Real> {
    segment: Segment<T>,
    from_is_lo: bool,
    holder: Option<(HalfEdgeKey, bool)>,
}

/// The entry past `k` whose end bound is a real edge: the one holding
/// the half-edge that bounds `k`'s physical sector at its start.
fn next_edge_bound<T: geom_core::Real>(sectors: &[BoolSector<T>], k: usize) -> usize {
    let n = sectors.len();
    (1..=n)
        .map(|i| (k + i) % n)
        .find(|&j| sectors[j].end_edge())
        .unwrap_or(k)
}

/// The half at `vertex` that bounds a corner where a sector's half `he`
/// does: `he` itself, or, where a fan holding the strut carried the
/// corner to its copy `vertex` and left `he` behind, the fan's null
/// half `fan_half` from the copy, which the mint put where the fan's
/// germ cut the sector.
fn corner_bound<T: geom_core::Real>(
    body: &Body<T>,
    vertex: VertexKey,
    he: HalfEdgeKey,
    fan_half: Option<HalfEdgeKey>,
) -> Result<HalfEdgeKey, BooleanError> {
    let starts_here =
        |h: HalfEdgeKey| proven(&body.half_edges, h, EntityId::HalfEdge).start == vertex;
    if starts_here(he) {
        return Ok(he);
    }
    fan_half
        .filter(|&h| starts_here(h))
        .ok_or(BooleanError::ClassificationInvariant {
            what: "an earlier run at the vertex carried a strut's corner to its copy",
        })
}

/// One clockwise orbit step from `he`, a half-edge starting at
/// `vertex`, proven to land on one that starts there too: a strut site
/// is one half, which no later read ties to `vertex`.
///
/// The caller establishes that `he` starts at `vertex`: every strut's
/// corner half is [`corner_bound`]'s, which refuses one an earlier run
/// carried to its copy (`ClassificationInvariant`), and [`strut_anchor`]
/// steps on from an orbit step's result.
#[track_caller]
fn orbit_step_at<T: geom_core::Real>(
    body: &Body<T>,
    vertex: VertexKey,
    he: HalfEdgeKey,
) -> HalfEdgeKey {
    let next = body.proven_orbit_step(he);
    let start = proven(&body.half_edges, next, EntityId::HalfEdge).start;
    if start != vertex {
        unreachable!(
            "the orbit step from {he:?} at {vertex:?} lands on {next:?}, which starts at \
             {start:?}: mint_directed checks that a strut's corner half still starts at its \
             vertex, and the keyed sort mints shared struts before any fan moves a half"
        );
    }
    next
}

/// Where a strut at a shared vertex splices: before the first half-edge
/// from `first` (the one past its corner's arrival) that is not a strut
/// hung earlier at that vertex at a lower germ (`germ`, the earlier of
/// a strut's two). The struts [`reconcile_shared`] leaves in one corner
/// hold none of each other's cuts unless one nests whole inside the
/// other, which hangs at the other's tip, so the segments hung at one
/// vertex are disjoint. Two lower germs tied unread ([`CutOrder::Tied`]
/// `Zero`) are then an invariant: the two segments would start at one
/// position. Read apart, the earlier segment holds the later's lower
/// germ unless it ends between the two, its upper germ along that line
/// too: a strut along the line, its germs beside a slit's or ring's two
/// edges ([`leads`]), sharing its vertex with another pair's strut that
/// leaves the same line. That pose is not an invariant break, and no
/// pose reaches it, so the read tie refuses unwitnessed: a limit, in
/// the invariant's voice until a typed limit lands
/// (`work/cleave/a-fate-cut-placement-limit-refuses-in-the-voice-of-a-kernel-bug.md`).
fn strut_anchor<T: Decide>(
    body: &Body<T>,
    (operand, vertex): (Operand, VertexKey),
    sectors: &[BoolSector<T>],
    first: HalfEdgeKey,
    germ: Cut<T>,
    hung: &Hung<T>,
    band: Band,
) -> Result<HalfEdgeKey, BooleanError> {
    let mut he = first;
    while let Some(at) = hung
        .struts
        .iter()
        .find(|h| (h.operand, h.vertex, h.half) == (operand, vertex, he))
        .map(|h| h.segment.0)
    {
        match cut_order(sectors, at, germ, band)? {
            CutOrder::Apart(true) => he = orbit_step_at(body, vertex, he),
            CutOrder::Apart(false) => break,
            CutOrder::Tied(Sign::Zero) => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "two dangling null edges in one corner start along one direction",
                });
            }
            CutOrder::Tied(_) => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "a limit, not a broken invariant: two dangling null edges in one \
                           corner start along one direction on two sides of it",
                });
            }
        }
    }
    Ok(he)
}

/// The edge of `operand`'s own solid a germ runs along, if its locus
/// there is `OnEdge`.
fn own_locus_edge(operand: Operand, (_, loci): Cells) -> Option<EdgeKey> {
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
enum StrutFacing {
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
/// runs from ([`StrutFacing::ClosedEdge`]); each caller of
/// [`strut_faces_first`] states what it does then.
///
/// At a wrap edge's tip (`tip`, [`strut_faces_first`] reads it) the
/// corner's two edges are the one edge too, walked out and back, and the
/// corner is the face's full turn from the arriving half round to the
/// departing one. A germ along the edge lies on both ends of that turn:
/// as the run's start germ it is met first, beside the arrival, and as
/// its end germ last, beside the departure, so `he_plus` faces `first`
/// either way.
fn strut_facing(
    (arrival, departure): (EdgeKey, EdgeKey),
    tip: bool,
    first: Option<EdgeKey>,
    second: Option<EdgeKey>,
) -> Result<StrutFacing, BooleanError> {
    if arrival == departure {
        let along = first == Some(arrival) || second == Some(arrival);
        return Ok(match (along, tip) {
            (false, _) => StrutFacing::Unnamed,
            (true, true) => StrutFacing::PlusFirst,
            (true, false) => StrutFacing::ClosedEdge,
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

/// **Whether a strut's `he_plus` faces its `first` germ**, the one
/// facing rule both strut minters read. `he_plus` lies beside the
/// splice corner's `arrival` edge and `he_minus` beside its
/// `departure` edge, so `he_plus` faces the germ the corner meets
/// first walking from its arrival edge toward its departure. A germ
/// along one of those edges of `operand`'s own solid names its half
/// structurally ([`strut_facing`]): an angular reading would meet it
/// exactly on its bound. Otherwise the walk orders the germs' cuts
/// ([`walk_faces_first`]).
///
/// `body` and `vertex`, the corner's, are read only to tell a wrap
/// edge's tip from a closed edge's lone vertex, where arrival and
/// departure are one edge ([`strut_facing`]'s `tip`). The tip is
/// `validate::wrap_tips`' (a valence-1 end of a wrap edge), read at this
/// corner: its one edge is a wrap edge whose two ends differ, and
/// `vertex` is one of them. At rest that is the same set: a corner
/// bounded both ways by one edge of two ends is that edge's halves
/// following each other round `vertex`, so its orbit holds no other
/// edge. Mid-run it is the reading that stands, where `wrap_tips`'
/// valence would count the struts already minted at the vertex.
///
/// `None` at a closed edge's lone vertex with a germ along it
/// ([`StrutFacing::ClosedEdge`]): the edge key names no half there, and
/// each caller states what it does then.
pub(super) fn strut_faces_first<T: Decide>(
    (body, vertex): (&Body<T>, VertexKey),
    (operand, sectors): (Operand, &[BoolSector<T>]),
    (arrival, departure): (EdgeKey, EdgeKey),
    first: (Cut<T>, Cells),
    second: (Cut<T>, Cells),
    band: Band,
) -> Result<Option<bool>, BooleanError> {
    let tip = arrival == departure && {
        let e = proven(&body.edges, arrival, EntityId::Edge);
        let ends = [Some(e.he_plus), body.mate(e.he_plus)]
            .map(|h| h.and_then(|h| body.get_half_edge(h)).map(|d| d.start));
        let wraps = body
            .get_curve_geom(e.curve)
            .and_then(crate::null::CurveGeom::certified)
            .is_some_and(|c| c.description().is_wrap());
        wraps && ends[0] != ends[1] && ends.contains(&Some(vertex))
    };
    match strut_facing(
        (arrival, departure),
        tip,
        own_locus_edge(operand, first.1),
        own_locus_edge(operand, second.1),
    )? {
        StrutFacing::PlusFirst => Ok(Some(true)),
        StrutFacing::MinusFirst => Ok(Some(false)),
        StrutFacing::ClosedEdge => Ok(None),
        StrutFacing::Unnamed => walk_faces_first(sectors, first.0, second.0, band).map(Some),
    }
}

/// Whether the corner meets germ `first` before `second`, walking its
/// one physical sector from the arrival edge: germs in different
/// entries by their entries ([`cut_order`]), two in one entry by their
/// directions ([`strut_order`]). Inside one entry the two orderings
/// agree, since `build_sectors` bisects every sector of a half-turn or
/// more and both then read the sign of `(g0 × g1)·n` at the entry's
/// arm (`the_strut_order_agrees_with_the_walk_within_an_entry`);
/// `cut_order` keeps [`walks_after`] there for the shared-vertex cuts,
/// which have no arrival edge to anchor on. Two germs along one
/// direction face as the orbit's order reads its tie, read before the
/// strut order: it reads each germ's half-turn at the arrival edge's
/// reach first, and two germs along one direction in band of that
/// edge's line could read apart there, where the tie's reading orders
/// them. Only cuts the tie's reading can read ([`Cut::tie_flankers`]) ask
/// it, so every other pair reads the strut order alone.
fn walk_faces_first<T: Decide>(
    sectors: &[BoolSector<T>],
    first: Cut<T>,
    second: Cut<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    if first.entry != second.entry {
        return Ok(walks_before(sectors, first, second, band)? == Some(true));
    }
    if first.tie_flankers(second).is_some()
        && let order @ CutOrder::Tied(_) = cut_order(sectors, first, second, band)?
        && let Some(before) = order.before()
    {
        return Ok(before);
    }
    let s = &sectors[first.entry];
    // The germs are read from the corner's arrival bound: the arrival
    // edge's departure from the vertex, which a germ leaving along a
    // curved edge's tangent lies on.
    let (arrival, reach) = super::sectors::corner_edge_bound(sectors, s, true).ok_or(
        BooleanError::ClassificationInvariant {
            what: "no sector of a corner ends on its arrival edge",
        },
    )?;
    strut_order(
        (Leaned::upright(arrival.normalize()), reach.length()),
        s.normal.vec(),
        (first.line, second.line),
        s.arm,
        || Ok(None),
        band,
    )
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
/// (`bool_strut_order`). Both readings are sines levered as distances:
/// a germ's side of the arrival bound at its `reach` (a line edge's
/// length, a curved edge's extent from below), as a germ's side of a
/// bound is read throughout (`sectors::within_reaching`); two germs'
/// order at `arm`, the germs' entry's shorter bound reach. Each is
/// levered further by the smaller lean of the directions it reads
/// (`sectors::leaned_lever`): a face-line germ's side of a line edge in
/// the sector's plane is then that edge's far end's offset from the
/// germ's other face, and a face line's own steepness does not read a
/// tilt within the zero off the upright order. A sine is
/// linear in the spacing at every angle, where a cosine is flat beside
/// 0 and π. That is
/// `splitting::containment`'s doctrine for angular windows: decided as
/// distances, never as an angle. A decided zero of a side whose
/// direction sense is in band refuses. Two germs along one direction
/// are the caller's `tie`: the sign of `(g0 × g1)·n` one order down
/// (`geom_brep::tied_germ_turn`), or `None` where nothing orders them,
/// which refuses with the first-order reading. A strut's two germs in
/// one entry are its only comparands, where it agrees with
/// [`walks_before`] ([`walk_faces_first`]).
pub(super) fn strut_order<T: Decide>(
    (e, reach): (Leaned<T>, T),
    normal: Vec3<T>,
    germs: (Leaned<T>, Leaned<T>),
    arm: T,
    tie: impl FnOnce() -> Result<Option<Sign>, BooleanError>,
    band: Band,
) -> Result<bool, BooleanError> {
    let refuse = |diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag);
    let far = |g: Leaned<T>| -> Result<bool, BooleanError> {
        let lever = leaned_lever(reach, (e.lean, g.lean));
        let side = Margin::levered(g.dir.cross(e.dir).dot(normal), lever);
        match crate::validate::decide("bool_strut_side", side, band).map_err(refuse)? {
            Sign::Positive => Ok(false),
            Sign::Negative => Ok(true),
            Sign::Zero => Ok(!super::sectors::direction_sense(g.dir, e.dir, reach, band)?),
        }
    };
    let (far0, far1) = (far(germs.0)?, far(germs.1)?);
    if far0 != far1 {
        return Ok(far1);
    }
    let (g0, g1) = germs;
    let lever = leaned_lever(arm, (g0.lean, g1.lean));
    let order = Margin::levered(g0.dir.cross(g1.dir).dot(normal), lever);
    match crate::validate::decide_nonzero("bool_strut_order", order, band) {
        Ok(sign) => Ok(sign == NonzeroSign::Negative),
        Err(diag) if diag.margin.rejected_sign() == Some(Sign::Zero) => match tie()? {
            Some(Sign::Negative) => Ok(true),
            Some(Sign::Positive) => Ok(false),
            Some(Sign::Zero) | None => Err(refuse(diag)),
        },
        Err(diag) => Err(refuse(diag)),
    }
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
    declared: &super::GluedPairs<T>,
    band: Band,
) -> Result<Leaned<T>, BooleanError> {
    // What the door read of the pair, which the questions below refuse
    // alike declared or not: no class settles them.
    let read = declared.read(
        &[(super::Operand::A, sa.face, super::Operand::B, sb.face)],
        Coincide::TangentLocus,
        &[],
    );
    if read != DeclarationRead::Spent(GlueClass::Tangent) {
        return germ_dir(sa, sb, read, band);
    }
    let s_a = super::sectors::tangent_carrier(a_body, sa.face)?;
    let s_b = super::sectors::tangent_carrier(b_body, sb.face)?;
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
    let line = Leaned::upright(d);
    match super::sectors::rays_within(line, &[sa, sb], read, band)? {
        (true, false) => Ok(line),
        (false, true) => Ok(line.reversed()),
        _ => Err(BooleanError::ClassificationInvariant {
            what: "germ direction not uniquely within its sector pair",
        }),
    }
}

/// The germ's outgoing direction: the unit intersection direction of
/// the two sector faces' planes with its lean `|n_a × n_b|`
/// ([`Leaned`]), signed to lie within both sectors
/// (grazes count — an on-bound germ's direction IS the bound). An
/// ambiguous or coplanar configuration refuses loudly.
///
/// The sign is the membership `sectors::pair_search` made this crossing
/// record on: the same readings of the same line at the same reaches
/// (`sectors::rays_within` of the un-normalized line), so they single
/// out the ray they found there. `±(n_a × n_b)` is
/// a LINE and membership picks the ray, so neither normal's sign
/// survives into it; they arrive outward from `sectors::sector_face`
/// and nothing is multiplied here. (A pierce germ is signed by its
/// transition's codes instead, `vtxfac::pierce_germ_dir`, which reads
/// both normals as outward.)
fn germ_dir<T: Decide>(
    sa: &BoolSector<T>,
    sb: &BoolSector<T>,
    read: DeclarationRead,
    band: Band,
) -> Result<Leaned<T>, BooleanError> {
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
    let line = Leaned::of_line(int);
    match super::sectors::rays_within(line, &[sa, sb], read, band)? {
        (true, false) => Ok(line),
        (false, true) => Ok(line.reversed()),
        _ => Err(BooleanError::ClassificationInvariant {
            what: "germ direction not uniquely within its sector pair",
        }),
    }
}

/// Whether the forward run `from → to` is [`RunSite::WholeOrbit`], which
/// the planner mints as its reverse: the empty run whose strut is the
/// same null edge.
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
    let site = body
        .run_site(first, last)
        .ok_or(BooleanError::ClassificationInvariant {
            what: "run edge without a mate",
        })?;
    Ok(matches!(site, RunSite::WholeOrbit { .. }))
}

/// Whether the run between entries `from` and `to` crosses no edge
/// bound ([`run_fan`]): a strut, dangling inside one physical sector.
fn is_strut<T: Decide>(
    sectors: &[BoolSector<T>],
    from: usize,
    to: usize,
) -> Result<bool, BooleanError> {
    Ok(run_fan(sectors, from, to)?.is_empty())
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
/// sector before `anchor`, which [`mint_directed`] reads past the
/// corner's bound half ([`corner_bound`]; twin-stable, twins sharing
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
    germ_meta: [(Cells, Vec3<T>, super::Branch); 2],
    spike_from_first: bool,
    anchor: Option<HalfEdgeKey>,
) -> Result<(BoolNullEdgeRecord<T>, (VertexKey, HalfEdgeKey)), BooleanError> {
    let hes = run_fan(sectors, from, to)?;
    let (site, dangling) = match (hes.first(), hes.last()) {
        (Some(&first), Some(&last)) => match body.run_site(first, last).unwrap_or_else(|| {
            unreachable!(
                "the run {first:?} ..= {last:?} at {vertex:?} has no fan end: its keys are the \
                 sector walk's, and {WALKS_CLOSE}"
            )
        }) {
            site @ RunSite::Fan { .. } => (site.mev_site(), false),
            // The planner mints a whole-orbit run as its reverse, the
            // empty run whose strut this is ([`run_degenerates`]), so
            // reaching one here is a run-selection bug.
            RunSite::WholeOrbit { .. } => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "null-edge run spans the entire vertex orbit",
                });
            }
        },
        // The dangling strut, inside `from`'s physical sector.
        _ => {
            let he = anchor.unwrap_or_else(|| {
                unreachable!(
                    "the strut {from} -> {to} at {vertex:?} has no anchor: mint_directed anchors \
                     every run whose fan `is_strut` reads empty, and this arm reads the same fan"
                )
            });
            (MevSite::Fan { he1: he, he2: he }, true)
        }
    };
    let run_side = match run_side {
        SideCode::In => NewVertexSide::Below,
        SideCode::Out => NewVertexSide::Above,
        SideCode::On => {
            return Err(BooleanError::ClassificationInvariant {
                what: "run with On side reached insertion",
            });
        }
    };
    // A fan puts he_plus at the from-germ cut and he_minus at the
    // to-germ cut; a strut's spike splices [he_plus, he_minus] into one
    // corner, and which germ he_plus faces is decided at the mint site
    // ([`strut_faces_first`]).
    let plus_faces_start = !dangling || spike_from_first;
    let mint = body.mev_null_run(site, vertex, run_side, plus_faces_start)?;
    let (created, attr, [start_he, end_he]) = (mint.created, mint.attr, mint.halves);
    let germ = |i: usize, he: crate::entity::HalfEdgeKey| {
        let (((a_face, b_face), (a_locus, b_locus)), dir, branch) = germ_meta[i];
        super::HalfGerm {
            he,
            a_face,
            b_face,
            a_locus,
            b_locus,
            dir,
            branch,
        }
    };
    let germs = [germ(0, start_he), germ(1, end_he)];
    let rec = BoolNullEdgeRecord {
        operand,
        at_vertex: vertex,
        edge: created.edge,
        attr,
        dangling,
        germs,
    };
    Ok((rec, (created.vertex, created.he_minus)))
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
        let f = |first, second| strut_facing((arr, dep), false, first, second);
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
        let closed = |first, second| strut_facing((arr, arr), false, first, second).unwrap();
        assert_eq!(closed(Some(arr), None), ClosedEdge);
        assert_eq!(closed(None, Some(arr)), ClosedEdge);
        assert_eq!(closed(None, None), Unnamed);
        assert_eq!(closed(Some(other), None), Unnamed);
        // A wrap tip's one edge, walked out and back: a germ along it
        // faces `he_plus`, whichever germ it is.
        let tip = |first, second| strut_facing((arr, arr), true, first, second).unwrap();
        assert_eq!(
            tip(Some(arr), None),
            PlusFirst,
            "the start germ along the tip's edge"
        );
        assert_eq!(
            tip(None, Some(arr)),
            PlusFirst,
            "the end germ along the tip's edge"
        );
        assert_eq!(tip(None, None), Unnamed);
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
            &crate::boolean::GluedPairs::default(),
            &crate::boolean::ReductionContacts::default(),
            BooleanOp::Union,
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
            &crate::boolean::GluedPairs::default(),
            &crate::boolean::ReductionContacts::default(),
            BooleanOp::Union,
            geom_core::Band::linear(Tol::witness()).unwrap(),
        )
        .unwrap_err();
        assert!(matches!(err, BooleanError::ClassificationInvariant { .. }));
    }

    /// The pairing start reads the op: it turns exactly when A's first
    /// forward run lies on the side the op discards of A — In for ∪ and
    /// `A ∖ B`, Out for ∩.
    #[test]
    fn the_pairing_start_puts_as_runs_on_the_side_the_op_keeps() {
        use SideCode::{In, Out};
        for (op, turns_on) in [
            (BooleanOp::Union, In),
            (BooleanOp::Intersect, Out),
            (BooleanOp::Subtract, In),
        ] {
            for first in [In, Out] {
                assert_eq!(
                    pairing_start_turns(first, op),
                    first == turns_on,
                    "{op:?}, first run {first:?}"
                );
            }
        }
    }

    /// A pair runs from the germ the other follows in the solid's walk
    /// order, either way round when there are two; a pair neither of
    /// whose germs follows the other has no run that holds no third
    /// germ, and [`b_runs`] nests it.
    #[test]
    fn f12_run_order_reads_cyclic_adjacency() {
        assert_eq!(run_order(4, 1, 2), Some(Some(false)), "forward");
        assert_eq!(run_order(4, 2, 1), Some(Some(true)), "backward");
        assert_eq!(run_order(4, 3, 0), Some(Some(false)), "forward, wrapping");
        assert_eq!(run_order(4, 0, 3), Some(Some(true)), "backward, wrapping");
        assert_eq!(run_order(2, 0, 1), Some(None), "two survivors");
        assert_eq!(run_order(4, 0, 2), None, "interleaved");
    }

    /// **One run rule, both solids, every survivor count** ([`walk_run`]):
    /// adjacent germs run the way that holds no third, either way for
    /// two survivors, and a pair adjacent neither way runs from its
    /// earlier position to its later, wrapping or not.
    #[test]
    fn walk_run_is_adjacency_then_the_interval() {
        assert_eq!(walk_run(2, 0, 1), None, "two survivors");
        assert_eq!(walk_run(2, 1, 0), None, "two survivors, swapped");
        assert_eq!(walk_run(6, 2, 3), Some(false), "adjacent forward");
        assert_eq!(walk_run(6, 3, 2), Some(true), "adjacent backward");
        assert_eq!(walk_run(6, 5, 0), Some(false), "adjacent across the origin");
        assert_eq!(
            walk_run(6, 0, 5),
            Some(true),
            "adjacent across the origin, swapped"
        );
        assert_eq!(walk_run(6, 1, 4), Some(false), "nested, earlier first");
        assert_eq!(walk_run(6, 4, 1), Some(true), "nested, later first");
    }

    /// **A's run, through the planner** ([`plan_null_pairs`]): with more
    /// than two survivors A pairs consecutive germs of its walk order and
    /// runs each pair forward from its first, from either pairing start;
    /// two run the way [`run_degenerates`] leaves them. Synthetic sectors
    /// on a cube vertex's orbit, every germ along `+y` in its own entry.
    #[test]
    fn as_runs_go_forward_between_consecutive_germs_and_two_as_the_orbit_allows() {
        use SideCode::{In, Out};
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let abody = crate::test_support_fixtures::declined_cube::<f64>(Tol::witness()).body;
        let bbody = crate::test_support_fixtures::declined_cube::<f64>(Tol::witness()).body;
        // `laps` times round the orbit, and with `twin` a last entry
        // bisecting the last physical sector.
        let sectors_of = |body: &Body<f64>, normal, start, end, (laps, twin): (usize, bool)| {
            let (vk, v) = body.vertices().next().unwrap();
            let orbit = body.vertex_orbit(v.emanating.unwrap()).unwrap();
            let mut secs: Vec<BoolSector<f64>> = std::iter::repeat_n(orbit, laps)
                .flatten()
                .map(|he| BoolSector {
                    he,
                    start,
                    end,
                    start_reach: crate::boolean::sectors::Reach::Extent(1.0),
                    end_reach: crate::boolean::sectors::Reach::Extent(1.0),
                    face: FaceKey::default(),
                    normal,
                    arm: 1.0,
                })
                .collect();
            if twin {
                let last = secs[secs.len() - 1].clone();
                secs.push(BoolSector {
                    end_reach: crate::boolean::sectors::Reach::Bisector(1.0),
                    ..last
                });
            }
            (vk, secs)
        };
        let v3 = geom_core::Vec3::new;
        let z = geom_brep::OutwardNormal::from_chart(v3(0.0, 0.0, 1.0), true);
        let x = geom_brep::OutwardNormal::from_chart(v3(1.0, 0.0, 0.0), true);
        let rec = |e: usize, sa: (SideCode, SideCode)| PairRecord {
            a: e,
            b: e,
            sa,
            sb: sa,
            intersect: true,
        };
        let plan = |laps: (usize, bool), recs: &[PairRecord], op| {
            let (va, a_secs) = sectors_of(&abody, z, v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0), laps);
            let (vb, b_secs) = sectors_of(&bbody, x, v3(0.0, 1.0, 0.0), v3(0.0, 0.0, 1.0), laps);
            let p = plan_null_pairs(
                &abody,
                &bbody,
                VvContact { a: va, b: vb },
                &a_secs,
                &b_secs,
                recs,
                recs,
                &crate::boolean::GluedPairs::default(),
                &crate::boolean::ReductionContacts::default(),
                op,
                band,
            )
            .unwrap();
            (p, a_secs)
        };
        // Four survivors in entries 0, 1, 3, 4 of a doubled orbit, their
        // forward codes alternating Out, In: ∪ keeps A's Out runs and
        // pairs from the first germ, ∩ keeps the In runs and turns.
        let four = [
            rec(0, (Out, In)),
            rec(1, (In, Out)),
            rec(3, (Out, In)),
            rec(4, (In, Out)),
        ];
        for (op, want) in [
            (BooleanOp::Union, [(0, 1), (3, 4)]),
            (BooleanOp::Intersect, [(1, 3), (4, 0)]),
        ] {
            let (p, _) = plan((2, false), &four, op);
            let got: Vec<(usize, usize)> = p
                .runs
                .iter()
                .map(|r| (r[0].from.cut.entry, r[0].to.cut.entry))
                .collect();
            assert_eq!(got, want, "{op:?}: A's runs");
            assert!(
                p.runs.iter().all(|r| !r[0].swapped),
                "{op:?}: each runs forward from its pair's first germ"
            );
        }
        // Two survivors, either record order: the run [`run_degenerates`]
        // leaves. On the bisected orbit, the forward run from the twin
        // entry 3 to entry 2 crosses every edge, so it runs from 2.
        for (recs, degenerates) in [
            ([rec(0, (Out, In)), rec(1, (In, Out))], false),
            ([rec(1, (In, Out)), rec(0, (Out, In))], false),
            ([rec(3, (Out, In)), rec(2, (In, Out))], true),
            ([rec(2, (In, Out)), rec(3, (Out, In))], false),
        ] {
            let (p, secs) = plan((1, true), &recs, BooleanOp::Union);
            let r = p.runs[0][0];
            let degenerate = run_degenerates(&abody, &secs, recs[0].a, recs[1].a).unwrap();
            assert_eq!(degenerate, degenerates, "{recs:?}: the fixture");
            assert_eq!(r.swapped, degenerate, "{recs:?}");
            assert_eq!(
                (r.from.cut.entry, r.to.cut.entry),
                if degenerate {
                    (recs[1].a, recs[0].a)
                } else {
                    (recs[0].a, recs[1].a)
                }
            );
        }
    }

    /// **One position order round a vertex** ([`cut_order`], read as
    /// [`walks_before`]), from the orbit's first entry, and the arcs read
    /// off it ([`on_arc`]). Four entries, the second a bisected twin of
    /// the first: across entries the order is the entries', within one it
    /// is the turn about the sector's normal, one direction tying (and
    /// refusing in [`walk_order`]). Two along one direction, each beside
    /// an edge whose one flush flanker lies on its side of the line, are
    /// read apart by those sides ([`beside_order`]); beside one side, or
    /// beside nothing, they tie. An arc from entry 3 to entry 1 wraps past
    /// the origin, and an arc may end at a cut read beside the line. An
    /// orbit whose first entry is a twin's second opens no physical
    /// sector there, and refuses.
    #[test]
    fn walks_before_reads_one_order_from_the_orbits_first_entry() {
        use super::super::sectors::Reach;
        use geom_core::Vec3;
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let (x, y) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let sector = |end_reach| BoolSector {
            he: HalfEdgeKey::default(),
            start: x,
            end: y,
            start_reach: Reach::Extent(1.0),
            end_reach,
            face: FaceKey::default(),
            normal: geom_brep::OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true),
            arm: 1.0,
        };
        let secs = vec![
            sector(Reach::Extent(1.0)),
            sector(Reach::Bisector(1.0)),
            sector(Reach::Extent(1.0)),
            sector(Reach::Extent(1.0)),
        ];
        let at = |k, d| Cut::apart(k, up(d));
        let before = |p, q| walks_before(&secs, p, q, band);
        assert_eq!(
            before(at(1, x), at(0, x)).unwrap(),
            Some(false),
            "across entries"
        );
        assert_eq!(
            before(at(0, x), at(3, x)).unwrap(),
            Some(true),
            "across entries"
        );
        assert_eq!(
            before(at(2, y), at(2, x)).unwrap(),
            Some(true),
            "within an entry"
        );
        assert_eq!(
            before(at(2, x), at(2, y)).unwrap(),
            Some(false),
            "within an entry"
        );
        assert_eq!(before(at(2, x), at(2, x)).unwrap(), None, "one direction");
        // Two along `x`, each beside an edge whose one flanker in the
        // sector's plane lies on its side of the line: the walk meets `y`
        // before `x`, so the cut beside `y` comes first.
        let beside = |side: Vec3<f64>| Cut {
            beside: Some([side, Vec3::new(0.0, 0.0, 1.0)]),
            ..at(2, x)
        };
        let (minus, plus) = (beside(-y), beside(y));
        assert_eq!(before(plus, minus).unwrap(), Some(true), "beside");
        assert_eq!(before(minus, plus).unwrap(), Some(false), "beside");
        assert_eq!(before(plus, plus).unwrap(), None, "beside one side");
        assert_eq!(before(plus, at(2, x)).unwrap(), None, "beside nothing");
        let arc = |lo, hi, x| on_arc(before, [lo, hi], x).unwrap();
        assert_eq!(
            arc(at(0, x), plus, minus),
            [Some(true), Some(false)],
            "past an arc ending on the line's side the walk meets first"
        );
        assert_eq!(
            arc(at(0, x), minus, plus),
            [Some(true), Some(true)],
            "held by an arc ending on the line's side the walk meets last"
        );
        let (lo, hi) = (at(0, x), at(2, x));
        assert_eq!(arc(lo, hi, at(1, x)), [Some(true), Some(true)], "held");
        assert_eq!(arc(lo, hi, at(3, x)), [Some(true), Some(false)], "past hi");
        assert_eq!(
            arc(lo, hi, at(0, y)),
            [Some(false), Some(true)],
            "short of lo"
        );
        assert_eq!(arc(lo, hi, at(2, x)), [Some(true), None], "along hi");
        let (lo, hi) = (at(3, x), at(1, x));
        assert_eq!(
            arc(lo, hi, at(0, x)),
            [Some(true), Some(true)],
            "held past the origin"
        );
        assert_eq!(
            arc(lo, hi, at(3, x)),
            [None, Some(true)],
            "along lo, wrapped"
        );
        assert_eq!(
            arc(lo, hi, at(3, y)),
            [Some(true), Some(false)],
            "before lo, wrapped"
        );
        assert_eq!(
            arc(lo, hi, at(2, x)),
            [Some(true), Some(false)],
            "outside, wrapped"
        );
        assert_eq!(
            walk_order(&secs, &[at(3, x), at(0, x), at(0, y)], band).unwrap(),
            vec![2, 1, 0]
        );
        assert!(matches!(
            walk_order(&secs, &[at(2, x), at(2, x)], band),
            Err(BooleanError::ClassificationInvariant { .. })
        ));
        let mut twin_first = secs.clone();
        twin_first.rotate_left(1);
        assert!(matches!(
            walks_before(&twin_first, at(0, x), at(1, x), band),
            Err(BooleanError::ClassificationInvariant {
                what: "a vertex orbit whose first entry opens no physical sector"
            })
        ));
    }

    /// **[`arc_holders`]' readings, and the chain [`held_by`] reads
    /// off them.** Six walk positions:
    /// - three nested arcs: each is held by the shortest arc round it;
    /// - the pinch-first shared corner's six-crossing pair: a fan from 1
    ///   to 4 holds a strut from 2 to 3 beside a fan from 5 to 0. Turned
    ///   onto its complement, from 4 to 1, the fan holds the other fan
    ///   and no longer the strut;
    /// - a turned run inside a run of its plan that held it: the two
    ///   cover the walk, each holding the other's ends, and refuse;
    /// - two pairs crossing refuse.
    #[test]
    fn arc_holders_nest_the_runs_a_turn_reroots_and_refuse_a_cover() {
        assert_eq!(
            arc_holders(6, &[[0, 5], [1, 4], [2, 3]]),
            Ok(vec![None, Some(0), Some(1)]),
            "nested: the innermost holder"
        );
        assert_eq!(
            arc_holders(6, &[[1, 4], [2, 3], [5, 0]]),
            Ok(vec![None, Some(0), None]),
            "the plan's nesting"
        );
        assert_eq!(
            arc_holders(6, &[[4, 1], [2, 3], [5, 0]]),
            Ok(vec![None, None, Some(0)]),
            "the turn holds the run outside it, and leaves the one inside"
        );
        assert_eq!(
            arc_holders(6, &[[0, 5], [4, 1]]),
            Err(ArcRefusal::Cover),
            "a turned run and its holder cover the walk"
        );
        assert_eq!(
            arc_holders(4, &[[0, 2], [1, 3]]),
            Err(ArcRefusal::Crossing),
            "crossing"
        );
        let held = held_by(&[None, Some(0), Some(1)], &[false, true, false]);
        assert_eq!(
            held.iter()
                .map(|h| h.map(|h| (h.depth, h.fan, h.by_strut)))
                .collect::<Vec<_>>(),
            vec![None, Some((1, Some(0), false)), Some((2, Some(0), true))],
            "depth, innermost fan, and whether a strut holds it directly"
        );
    }

    /// **F12 guard 1 over six survivors.** Review r1's trace of the
    /// 343° notch: A pairs `(0, 3) (2, 4) (5, 1)`, and B reads them at
    /// positions `(2, 5) (0, 1) (4, 3)`. The first pair runs forward from
    /// 2 to 5 and holds the third, which runs from 3 to 4; the second is
    /// adjacent and held by none. Two pairs that cross in B's walk order
    /// are no two simple links' crossings: refused.
    #[test]
    fn f12_b_runs_nest_a_pair_and_refuse_a_crossing() {
        let pairs = [(0, 3), (2, 4), (5, 1)];
        let b_pos = [2, 3, 0, 5, 1, 4];
        assert_eq!(
            b_runs(6, &pairs, &b_pos),
            Ok(vec![
                (Some(false), None),
                (Some(false), None),
                (Some(true), Some(0)),
            ]),
            "nested"
        );
        // The same pairs read from B's other start: the holder now runs
        // backward from its later germ.
        let turned = b_pos.map(|p| (p + 3) % 6);
        assert_eq!(
            b_runs(6, &pairs, &turned),
            Ok(vec![
                (Some(true), None),
                (Some(false), Some(0)),
                (Some(true), None),
            ]),
            "nested, read from another start"
        );
        assert_eq!(
            b_runs(4, &[(0, 2), (1, 3)], &[0, 1, 2, 3]),
            Err(ArcRefusal::Crossing),
            "crossing"
        );
        assert_eq!(
            b_runs(4, &[(0, 1), (2, 3)], &[0, 1, 2, 3]),
            Ok(vec![(Some(false), None), (Some(false), None)]),
            "four, adjacent"
        );
    }

    /// Every perfect matching of `0..n`, crossing or not.
    fn matchings(pts: &[usize]) -> Vec<Vec<(usize, usize)>> {
        let Some((&a, rest)) = pts.split_first() else {
            return vec![Vec::new()];
        };
        let mut out = Vec::new();
        for j in 0..rest.len() {
            let others: Vec<usize> = rest
                .iter()
                .enumerate()
                .filter(|&(k, _)| k != j)
                .map(|(_, &p)| p)
                .collect();
            for mut m in matchings(&others) {
                m.insert(0, (a, rest[j]));
                out.push(m);
            }
        }
        out
    }

    /// **[`b_runs`] over every matching B's walk can read, to ten
    /// crossings.** A pairing is laid out with each germ at its own B
    /// position, read from every start round the walk and with each pair
    /// either way round. For every non-crossing one: each run walks
    /// forward from its from-germ past whole pairs only (the runs nest
    /// or are disjoint), a run whose germs wrap the walk's origin holds
    /// nothing, and a run's holder is the innermost run that holds its
    /// germs, by containment. A pairing is refused exactly when two of
    /// its pairs cross (every perfect matching to eight).
    #[test]
    fn b_runs_nest_every_non_crossing_matching_and_refuse_every_crossing_one() {
        let crosses = |p: (usize, usize), q: (usize, usize)| {
            let ((a, b), (c, d)) = ((p.0.min(p.1), p.0.max(p.1)), (q.0.min(q.1), q.0.max(q.1)));
            (a < c && c < b) != (a < d && d < b)
        };
        let mut readings = 0;
        let mut deepest = 0;
        for n in [4usize, 6, 8, 10] {
            let all = matchings(&(0..n).collect::<Vec<_>>());
            for m in &all {
                let crossing = m
                    .iter()
                    .enumerate()
                    .any(|(k, &p)| m[k + 1..].iter().any(|&q| crosses(p, q)));
                if crossing {
                    if n <= 8 {
                        let b_pos: Vec<usize> = (0..n).collect();
                        assert_eq!(
                            b_runs(n, m, &b_pos),
                            Err(ArcRefusal::Crossing),
                            "crossing {m:?}"
                        );
                    }
                    continue;
                }
                for rot in 0..n {
                    for flips in 0..1u32 << (n / 2) {
                        readings += 1;
                        let pairs: Vec<(usize, usize)> = m
                            .iter()
                            .enumerate()
                            .map(|(k, &(x, y))| {
                                let (x, y) = ((x + rot) % n, (y + rot) % n);
                                if flips >> k & 1 == 1 { (y, x) } else { (x, y) }
                            })
                            .collect();
                        let b_pos: Vec<usize> = (0..n).collect();
                        let runs = b_runs(n, &pairs, &b_pos)
                            .unwrap_or_else(|e| panic!("refused {pairs:?}: {e:?}"));
                        let span = |k: usize| {
                            let (i0, i1) = pairs[k];
                            match runs[k].0 {
                                Some(true) => (i1, i0),
                                _ => (i0, i1),
                            }
                        };
                        // The positions a run walks past, forward.
                        let past = |k: usize| {
                            let (from, to) = span(k);
                            let mut out = Vec::new();
                            let mut p = (from + 1) % n;
                            while p != to {
                                out.push(p);
                                p = (p + 1) % n;
                            }
                            out
                        };
                        for k in 0..pairs.len() {
                            let held = past(k);
                            let (from, to) = span(k);
                            assert!(
                                from < to || held.is_empty(),
                                "{pairs:?}: run {k} wraps the origin and holds {held:?}"
                            );
                            for (j, &(x, y)) in pairs.iter().enumerate() {
                                assert!(
                                    j == k || held.contains(&x) == held.contains(&y),
                                    "{pairs:?}: run {k} holds half of pair {j}"
                                );
                            }
                            let holders: Vec<usize> = (0..pairs.len())
                                .filter(|&o| o != k && past(o).contains(&pairs[k].0))
                                .collect();
                            let innermost = holders.iter().copied().find(|&o| {
                                holders
                                    .iter()
                                    .all(|&q| q == o || past(q).contains(&pairs[o].0))
                            });
                            assert_eq!(runs[k].1, innermost, "{pairs:?}: run {k}'s holder");
                            deepest = deepest.max(holders.len());
                        }
                    }
                }
            }
        }
        assert_eq!(readings, 2 * 4 * 4 + 5 * 6 * 8 + 14 * 8 * 16 + 42 * 10 * 32);
        // Five chords nested in one another nest four deep, but the
        // outermost wraps the walk's origin and holds nothing there.
        assert_eq!(deepest, 3, "ten crossings nest three deep");
    }

    /// F12 at mechanism level: four survivors, two consecutive record
    /// pairs, each a strut in BOTH solids, on real cube vertices but
    /// synthetic sectors. Every germ runs along `+y`, so nothing orders
    /// two germs in one sector entry: the walk order ([`walk_order`])
    /// refuses them, before any mint. Four
    /// survivors at one vertex pair in real geometry are
    /// `four-germ-vertex-pairs-run-b-in-a-order`'s (JOIN, closed by PR 4036).
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
            &crate::boolean::GluedPairs::default(),
            &crate::boolean::ReductionContacts::default(),
            BooleanOp::Union,
            geom_core::Band::linear(Tol::witness()).unwrap(),
        )
        .expect_err("nothing orders the struts' germs");
        assert!(
            matches!(
                err,
                BooleanError::ClassificationInvariant {
                    what: "two crossing germs of a vertex pair lie along one direction in one \
                           sector entry"
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
                strut_order(
                    (up(e), 1.0),
                    n,
                    (up(at(g0)), up(at(g1))),
                    1.0,
                    || Ok(None),
                    band
                )
                .unwrap(),
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
            match strut_order(
                (up(e), 1.0),
                n,
                (up(at(g0)), up(at(g1))),
                1.0,
                || Ok(None),
                band,
            ) {
                Ok(got) if got != truth => wrong.push(format!("{g0}°/{g1}°: got {got}")),
                _ => {}
            }
        }
        assert!(wrong.is_empty(), "silently misordered: {wrong:?}");
    }

    fn up(d: Vec3<f64>) -> Leaned<f64> {
        Leaned::upright(d)
    }

    /// The face line of the plane through the z axis whose normal lies
    /// at `deg`, tipped by `tilt` toward −z, with the upright plane whose
    /// normal lies at 30°, pointing down: their germ, with its lean.
    fn germ_beside_30(deg: f64, tilt: f64) -> Leaned<f64> {
        let at = |deg: f64| {
            let (s, c) = deg.to_radians().sin_cos();
            Vec3::new(c, s, 0.0)
        };
        let n_a = (at(deg) + Vec3::new(0.0, 0.0, -tilt)).normalize();
        let g = Leaned::of_line(n_a.cross(at(30.0)));
        if g.dir.z > 0.0 { g.reversed() } else { g }
    }

    /// **A germ's side of the arrival edge decides as upright under a
    /// tilt within the zero**: two faces 10° apart, one tipped 0.2·zero,
    /// their germ read against the edge down their shared bound at its
    /// reach 1.23 (`bool_strut_side`). Read unit, the germ is steeper than
    /// the tilt by `1/sin 10° = 5.76` and the side reads in band; levered
    /// by its lean, it reads the edge's far end off the tilted face, a
    /// quarter of the zero.
    #[test]
    fn a_germ_tilted_within_the_zero_sides_the_arrival_edge_as_upright() {
        let band = Band::linear(Tol::witness()).unwrap();
        let at = |deg: f64| {
            let (s, c) = deg.to_radians().sin_cos();
            Vec3::new(c, s, 0.0)
        };
        let down = up(Vec3::new(0.0, 0.0, -1.0));
        let order = |g| {
            strut_order(
                (down, 1.23),
                at(30.0),
                (g, up(at(120.0))),
                0.11,
                || Ok(None),
                band,
            )
        };
        let upright = order(germ_beside_30(20.0, 0.0)).unwrap();
        let tilted = germ_beside_30(20.0, 0.2 * band.zero());
        assert_eq!(order(tilted).ok(), Some(upright), "tilted as upright");
        assert!(
            order(up(tilted.dir)).is_err(),
            "the unit germ reads in band"
        );
    }

    /// **Face lines tilted within the zero decide as upright, against an
    /// edge and against each other**. Face `b` is `z = 0`; each germ is
    /// its line with a face leaning `φ` off it about an axis in it, for
    /// `φ` from 1° to 90°, read at levers 0.01, 1.23 and 100. Against an
    /// edge (`strut_order`'s side) the edge lies along the germ (Zero), 30
    /// zeros off it at the lever, or square to it; two germs
    /// (`walks_after`) lie along one line or 30 zeros apart. Each face is
    /// tipped by ±0.2 or ±0.9 of the zero at the lever, about its line
    /// with `b` or square to it, and `b` itself likewise. One tilted
    /// direction decides as upright; two tilted at once may add past the
    /// zero and escalate, as two edges' tilts do, but never read another
    /// definite order. Read unit, 30 zeros at `φ = 1°` would be lost.
    #[test]
    fn face_lines_tilted_within_the_zero_decide_as_upright() {
        let band = Band::linear(Tol::witness()).unwrap();
        let z = band.zero();
        let flat = Vec3::new(0.0, 0.0, 1.0);
        let germ = |n_a: Vec3<f64>, n_b: Vec3<f64>| {
            let g = Leaned::of_line(n_a.normalize().cross(n_b.normalize()));
            if g.dir.x < 0.0 { g.reversed() } else { g }
        };
        let phis = [1.0f64, 5.0, 10.0, 30.0, 90.0];
        let tilts = [0.2, -0.2, 0.9, -0.9];
        let (mut read, mut definite, mut moved) = (0usize, 0usize, Vec::new());
        for &phi in &phis {
            let (sp, cp) = phi.to_radians().sin_cos();
            let a0 = Vec3::new(0.0, -sp, cp);
            for lever in [0.01, 1.23, 100.0] {
                let s30 = 30.0 * z / (lever * sp);
                let mut thetas = vec![0.0, std::f64::consts::FRAC_PI_2];
                if s30 < 0.5 {
                    thetas.extend([s30.asin(), -s30.asin()]);
                }
                for theta in thetas {
                    let e = up(Vec3::new(theta.cos(), theta.sin(), 0.0));
                    let far =
                        up((Vec3::new(theta.sin(), -theta.cos(), 0.0) + e.dir * 0.3).normalize());
                    let side = |g, n_b| {
                        strut_order((e, lever), n_b, (g, far), lever, || Ok(None), band).ok()
                    };
                    let upright = side(germ(a0, flat), flat);
                    definite += usize::from(theta != 0.0 && upright.is_some());
                    for (axis, t) in (0..4).flat_map(|axis| tilts.map(|t| (axis, t))) {
                        let d = t * z / lever;
                        let (n_a, n_b) = match axis {
                            0 => (a0 + Vec3::new(d, 0.0, 0.0), flat),
                            1 => (a0 + Vec3::new(0.0, d * cp, d * sp), flat),
                            2 => (a0, flat + Vec3::new(d, 0.0, 0.0)),
                            _ => (a0, flat + Vec3::new(0.0, d, 0.0)),
                        };
                        read += 1;
                        let tilted = side(germ(n_a, n_b), n_b.normalize());
                        if tilted != upright {
                            moved.push(format!("edge φ {phi} L {lever} θ {theta:e} axis {axis} t {t}: {upright:?} → {tilted:?}"));
                        }
                    }
                }
                for &phi1 in &phis {
                    let (s1, c1) = phi1.to_radians().sin_cos();
                    let s30 = 30.0 * z / (lever * sp.min(s1));
                    let entry = BoolSector {
                        he: HalfEdgeKey::default(),
                        start: Vec3::new(1.0, -1.0, 0.0),
                        end: Vec3::new(1.0, 1.0, 0.0),
                        start_reach: crate::boolean::sectors::Reach::Extent(lever),
                        end_reach: crate::boolean::sectors::Reach::Extent(lever),
                        face: FaceKey::default(),
                        normal: geom_brep::OutwardNormal::from_chart(flat, true),
                        arm: lever,
                    };
                    for psi in [0.0, s30.asin()].into_iter().filter(|_| s30 < 0.5) {
                        let (sy, cy) = psi.sin_cos();
                        let turn =
                            |v: Vec3<f64>| Vec3::new(cy * v.x - sy * v.y, sy * v.x + cy * v.y, v.z);
                        let a1 = turn(Vec3::new(0.0, s1, c1));
                        let walk = |d0: f64, d1: f64| {
                            let g0 = germ(a0 + Vec3::new(d0, 0.0, 0.0), flat);
                            let g1 = germ(a1 + turn(Vec3::new(d1, 0.0, 0.0)), flat);
                            walks_after(&entry, g0, g1, band).ok()
                        };
                        let upright = walk(0.0, 0.0);
                        assert!(
                            upright.is_some(),
                            "germs φ {phi}/{phi1} L {lever} ψ {psi:e}: upright decides"
                        );
                        definite += usize::from(psi != 0.0);
                        for (t0, t1) in tilts
                            .iter()
                            .flat_map(|&t0| tilts.map(|t1| (t0, t1)))
                            .chain(tilts.iter().flat_map(|&t| [(t, 0.0), (0.0, t)]))
                        {
                            read += 1;
                            let tilted = walk(t0 * z / lever, t1 * z / lever);
                            let one = t0 == 0.0 || t1 == 0.0;
                            if tilted != upright && (one || tilted.is_some()) {
                                moved.push(format!("germs φ {phi}/{phi1} L {lever} ψ {psi:e} t {t0}/{t1}: {upright:?} → {tilted:?}"));
                            }
                        }
                    }
                }
            }
        }
        assert!(
            moved.is_empty(),
            "{} of {read} readings moved: {:#?}",
            moved.len(),
            &moved[..moved.len().min(20)]
        );
        assert!(definite > 0, "definite upright readings among {read}");
    }

    /// **The strut order and the walk agree within an entry**
    /// ([`walk_faces_first`] reads the one, [`walks_before`] the other). A
    /// physical sector of 60° to 350° from its arrival edge, bisected at
    /// its middle where it is a half-turn or more as `build_sectors`
    /// does: for every two germs on a 7° grid inside one entry,
    /// [`strut_order`] (against the arrival edge) puts first the germ
    /// [`walks_after`] (round the entry) puts first.
    #[test]
    fn the_strut_order_agrees_with_the_walk_within_an_entry() {
        use super::super::sectors::Reach;
        let band = Band::linear(Tol::witness()).unwrap();
        let (e, n) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let at = |deg: f64| {
            let t = -deg.to_radians();
            Vec3::new(t.cos(), t.sin(), 0.0)
        };
        let entry = BoolSector {
            he: HalfEdgeKey::default(),
            start: at(1.0),
            end: e,
            start_reach: Reach::Extent(1.0),
            end_reach: Reach::Extent(1.0),
            face: FaceKey::default(),
            normal: geom_brep::OutwardNormal::from_chart(n, true),
            arm: 1.0,
        };
        let mut pairs = 0;
        for alpha in [60.0, 179.0, 181.0, 270.0, 350.0] {
            let entries: Vec<(f64, f64)> = if alpha >= 180.0 {
                vec![(0.0, alpha / 2.0), (alpha / 2.0, alpha)]
            } else {
                vec![(0.0, alpha)]
            };
            for (lo, hi) in entries {
                let grid: Vec<f64> = (1..)
                    .map(|k| lo + 7.0 * f64::from(k) - 3.5)
                    .take_while(|&d| d < hi)
                    .filter(|&d| d > lo && (d - 180.0).abs() > 1.0)
                    .collect();
                for &d0 in &grid {
                    for &d1 in grid.iter().filter(|&&d1| d1 != d0) {
                        let walk = walks_after(&entry, up(at(d0)), up(at(d1)), band).unwrap();
                        assert_ne!(walk, Sign::Zero, "{alpha}°: {d0}° then {d1}°");
                        let strut = strut_order(
                            (up(e), 1.0),
                            n,
                            (up(at(d0)), up(at(d1))),
                            1.0,
                            || Ok(None),
                            band,
                        )
                        .unwrap();
                        assert_eq!(walk == Sign::Negative, strut, "{alpha}°: {d0}° then {d1}°");
                        assert_eq!(strut, d0 < d1, "{alpha}°: {d0}° then {d1}°");
                        pairs += 1;
                    }
                }
            }
        }
        assert!(pairs > 1000, "{pairs} pairs");
    }

    /// **A germ along a curved arrival edge's tangent is met first.** A
    /// corner of a half-turn, bisected as `build_sectors` does, its
    /// arrival bound the edge's departure tangent `e`: a germ leaving
    /// along `e` (a section tangent to the edge there, the pinch of two
    /// rims on tangent walls) lies at the bound, before every germ
    /// inside the entry, and [`walk_faces_first`] says so either way
    /// round. An anchor off the bound, such as the edge's chord, which
    /// lies inside the corner, reads it past a full half-turn.
    #[test]
    fn a_germ_along_the_arrival_tangent_is_met_first() {
        use super::super::sectors::Reach;
        let band = Band::linear(Tol::witness()).unwrap();
        let (e, n) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let at = |deg: f64| {
            let t = -deg.to_radians();
            Vec3::new(t.cos(), t.sin(), 0.0)
        };
        let sector = |end: f64, end_reach, start: f64, start_reach| BoolSector {
            he: HalfEdgeKey::default(),
            start: at(start),
            end: at(end),
            start_reach,
            end_reach,
            face: FaceKey::default(),
            normal: geom_brep::OutwardNormal::from_chart(n, true),
            arm: 1.0,
        };
        let sectors = [
            sector(0.0, Reach::Extent(1.0), 90.0, Reach::Bisector(1.0)),
            sector(90.0, Reach::Bisector(1.0), 180.0, Reach::Extent(1.0)),
        ];
        for inside in [10.0, 45.0, 80.0] {
            let along = Cut::apart(0, up(e));
            let other = Cut::apart(0, up(at(inside)));
            assert!(
                walk_faces_first(&sectors, along, other, band).unwrap(),
                "along the tangent, then {inside}°"
            );
            assert!(
                !walk_faces_first(&sectors, other, along, band).unwrap(),
                "{inside}°, then along the tangent"
            );
        }
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
        let germs = (up(Vec3::new(1.0, 0.0, 0.0)), up(Vec3::new(0.0, -1.0, 0.0)));
        let got = strut_order((up(n), 1.0), n, germs, 1.0, || Ok(None), band);
        assert!(
            got.is_err(),
            "decided with no comparand away from zero: {got:?}"
        );
    }

    /// **Two germs along one direction take the caller's tie, and only
    /// at a decided zero.** Both along `+y`, in one half-turn of
    /// `e = +x` about `+z`: the tie's sign of `(g0 × g1)·n` one order down
    /// orders them (`Negative`: `g0` first), and a tie that reads nothing
    /// or reads them alike refuses as the sectors' coincidence. Two germs
    /// apart, or in band of each other, never read the tie. Red if the
    /// tie's sign is read the other way round, or read away from a zero.
    #[test]
    fn the_strut_order_reads_the_callers_tie_only_at_a_decided_zero() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (e, n) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let y = Vec3::new(0.0, 1.0, 0.0);
        let tie = |sign: Option<Sign>| move || Ok(sign);
        assert_eq!(
            strut_order(
                (up(e), 1.0),
                n,
                (up(y), up(y)),
                1.0,
                tie(Some(Sign::Negative)),
                band
            )
            .ok(),
            Some(true),
            "g1 curving clockwise of g0: g0 first"
        );
        assert_eq!(
            strut_order(
                (up(e), 1.0),
                n,
                (up(y), up(y)),
                1.0,
                tie(Some(Sign::Positive)),
                band
            )
            .ok(),
            Some(false),
            "g1 curving counterclockwise of g0: g1 first"
        );
        for (what, sign) in [("reads nothing", None), ("curves alike", Some(Sign::Zero))] {
            assert!(
                matches!(
                    strut_order((up(e), 1.0), n, (up(y), up(y)), 1.0, tie(sign), band),
                    Err(BooleanError::Escalated {
                        decision: super::super::BooleanDecision::Coincidence(Coincide::Sectors, _),
                        ..
                    })
                ),
                "a tie that {what} refuses"
            );
        }
        let unread = || -> Result<Option<Sign>, BooleanError> { panic!("the tie was read") };
        let apart = Vec3::new(0.1, 1.0, 0.0).normalize();
        assert_eq!(
            strut_order((up(e), 1.0), n, (up(y), up(apart)), 1.0, unread, band).ok(),
            Some(true),
            "germs apart are ordered at first order"
        );
        let near = Vec3::new(-2.0 * band.zero(), 1.0, 0.0).normalize();
        assert!(
            strut_order((up(e), 1.0), n, (up(y), up(near)), 1.0, unread, band).is_err(),
            "germs in band of each other refuse without the tie"
        );
    }

    /// **A germ `θ` off the arrival edge, the edge long and the entry's
    /// arm short**: `θ` times the arm reads Zero, `θ` times the edge's
    /// reach definite, so the germ's half-turn is read at the edge's
    /// reach. The turn runs clockwise from the edge: just
    /// counterclockwise of it the germ lies at the turn's far end, after a
    /// germ square to the edge; just clockwise, at its near end, before
    /// it. Read at the arm both would lie along the edge, at angle 0,
    /// before it.
    #[test]
    fn a_germ_beside_a_long_arrival_edge_is_placed_at_the_edges_reach() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (e, n) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let arm = 100.0 * band.escalate();
        let theta = band.zero() / (10.0 * arm);
        let square = Vec3::new(0.0, 1.0, 0.0);
        let unread = || -> Result<Option<Sign>, BooleanError> { panic!("the tie was read") };
        for (side, first_nearer) in [(-1.0, true), (1.0, false)] {
            let g = Vec3::new(theta.cos(), side * theta.sin(), 0.0);
            assert_eq!(
                strut_order((up(e), 1.0), n, (up(g), up(square)), arm, unread, band).ok(),
                Some(first_nearer),
                "at the edge's reach, side {side}"
            );
            assert_eq!(
                strut_order((up(e), arm), n, (up(g), up(square)), arm, unread, band).ok(),
                Some(true),
                "at the arm, side {side}"
            );
        }
    }

    /// **A germ tangent to a bound of its sector moves to the sector
    /// across that bound, and only there.** A's orbit is a cube's
    /// corner: sectors on `z = 0` (from `+x` to `+y`), `x = 0` (`+y` to
    /// `+z`) and `y = 0` (`+z` to `+x`), each `start` its successor's
    /// `end`. B's sector lies in the plane through `+y` that parts `+x`
    /// from `+z`, so the `z = 0` sector reads its `start` On. A germ there
    /// whose locus is the `x = 0` face moves one sector on; one whose
    /// locus is the `y = 0` face, which lies across no On bound, stays.
    /// Read from the `x = 0` sector, whose `end` is On, the germ moves one
    /// back to `z = 0`. A plane through `+y` that leaves `+x` and `+z` on
    /// one side reads the moved sector's far bound against the record's,
    /// which refuses.
    #[test]
    fn a_germ_tangent_to_a_bound_reads_in_the_sector_across_it() {
        use super::super::sectors::{Reach, pair_codes};
        use crate::boolean::Locus;
        use geom_brep::OutwardNormal;
        use geom_core::Point3;
        let band = Band::linear(Tol::witness()).unwrap();
        let prism = crate::test_support_fixtures::prism_z::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            0.0,
            1.0,
            Tol::witness(),
        );
        let faces: Vec<FaceKey> = prism.body.faces().map(|(f, _)| f).take(4).collect();
        let o = Point3::new(0.0, 0.0, 0.0);
        let (x, y, z) = (Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z());
        let sector = |end: Vec3<f64>, start: Vec3<f64>, normal: Vec3<f64>, face| BoolSector {
            he: HalfEdgeKey::default(),
            start,
            end,
            start_reach: Reach::Chord {
                base: o,
                far: o + start,
            },
            end_reach: Reach::Chord {
                base: o,
                far: o + end,
            },
            face,
            normal: OutwardNormal::from_chart(normal, true),
            arm: 1.0,
        };
        let a = [
            sector(x, y, -z, faces[0]),
            sector(y, z, -x, faces[1]),
            sector(z, x, -y, faces[2]),
        ];
        // Clean the On of a raw tuple to the opposite of its other code.
        let clean = |c: (SideCode, SideCode)| match c {
            (SideCode::On, k) | (k, SideCode::On) => {
                let other = if k == SideCode::In {
                    SideCode::Out
                } else {
                    SideCode::In
                };
                if c.0 == SideCode::On {
                    (other, k)
                } else {
                    (k, other)
                }
            }
            c => c,
        };
        let read = |n: Vec3<f64>, i: usize, locus: FaceKey| {
            let n = n.normalize();
            // B's sector in its plane, straddling A's faces' planes.
            let e = y.cross(n).normalize();
            let b = [sector(y - e, y + e, n, faces[3])];
            let (sa, sb) = pair_codes(&a[i], &b[0], band).unwrap();
            let raw = PairRecord {
                a: i,
                b: 0,
                sa,
                sb,
                intersect: true,
            };
            let r = PairRecord {
                sa: clean(sa),
                sb: clean(sb),
                ..raw
            };
            let loci = (Locus::InFace(locus), Locus::InFace(faces[3]));
            (raw, across_tangent(r, &raw, loci, (&a, &b), band))
        };
        let parting = Vec3::new(1.0, 0.0, -1.0);
        let (raw, moved) = read(parting, 0, faces[1]);
        assert_eq!(raw.sa.0, SideCode::On, "the z = 0 sector's start is On");
        assert_eq!(moved.unwrap().a, 1, "moved one on, to x = 0");
        let (_, kept) = read(parting, 0, faces[2]);
        assert_eq!(kept.unwrap().a, 0, "no On bound faces y = 0: it stays");
        let (raw, back) = read(parting, 1, faces[0]);
        assert_eq!(raw.sa.1, SideCode::On, "the x = 0 sector's end is On");
        assert_eq!(back.unwrap().a, 0, "moved one back, to z = 0");
        let (_, refused) = read(Vec3::new(1.0, 0.0, 1.0), 0, faces[1]);
        assert!(
            matches!(refused, Err(BooleanError::ClassificationInvariant { .. })),
            "{refused:?}"
        );
    }
}
