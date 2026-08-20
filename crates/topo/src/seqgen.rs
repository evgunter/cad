//! Random VALID Euler-op sequence generation + the make/kill roundtrip
//! machinery (test support, M1 PR 4).
//!
//! The proptest suite at the bottom drives the four properties the PR
//! promises; the building blocks are `pub(crate)`, and PR 5's validator
//! fuzzing (see `crate::validate`'s tests) reuses the generator as its
//! fuzz source:
//!
//! - [`choose_op`] — walk the current body, enumerate every applicable
//!   `(operator, site)` candidate deterministically, and pick one by a
//!   weighted choice driven entirely by the caller's decision integers
//!   (proptest's seeded values). No ambient randomness: the same body
//!   and decisions always pick the same op (D9-style determinism, which
//!   also makes proptest shrinking meaningful). The catalog is the ten
//!   Euler operators plus the structural non-operator mutators that
//!   mint or kill through the same postcondition: `ring_move` (the
//!   demotion claim's least obvious tier-1 preserver — separating-curve
//!   argument, `crate::euler_ring`) and `split_edge` (whose Euler
//!   vector IS `mev`'s, so nothing but a catalog row distinguishes the
//!   randomised sites it reaches — struts, self-loop digons — from the
//!   ones `mev` already covers; `crate::split` calls exactly those
//!   coincidence cases delicate). The catalog is not an inventory of
//!   the public mutation surface and does not claim to be: it is the
//!   ops whose sites this walk can enumerate.
//! - [`apply`] — execute a choice (coordinates for the vertex-minting
//!   ops come from a caller-owned counter, so every vertex gets distinct
//!   coordinates and canonical forms are sharp).
//! - [`roundtrip`] — execute a choice AND its exact inverse
//!   (make ∘ kill), or the choice and a derived re-make (kill ∘ make),
//!   asserting canonical-form restoration via [`crate::iso`].
//! - [`teardown`] — drive a body all the way back to empty arenas
//!   through kill-direction ops only (plus the ring-resolving
//!   `mfkrh`/`mekr` moves), the completeness-in-reverse check.
//! - [`Ledger`] — the running Euler–Poincaré tuple `(v, e, f, h, r, s)`,
//!   checked against derived arena counts and eq. 9.2 after every op.
//!
//! # The irreversible-by-one-op kill subcases (skipped in roundtrips)
//!
//! [`roundtrip`] skips (returns [`RoundtripOutcome::SkippedIrreversible`])
//! exactly the kill configurations that are valid kills but have **no
//! single-op re-make** (the taxonomy was sharpened by the PR 4 review —
//! the mate-alone `kef` kill is re-makeable more often than first
//! claimed):
//!
//! - `kev(he)` where `start(he)` has valence 1 and `end(he)` carries a
//!   fan (the "mirror" adjacency `next(mate(he)) == he`): restoring it
//!   would need `mev` to move the survivor's ENTIRE fan to the new
//!   vertex, but the full-orbit run is mev-inexpressible — the ratified
//!   `he1 == he2` site means the EMPTY run (strut), so the full run has
//!   no address. The strut re-make from the same site instead leaves
//!   the fan on the wrong-coordinate vertex, so with distinct vertex
//!   coordinates (the generator's minting policy) no single op reaches
//!   the original; coordinate-COINCIDENT endpoints would collapse that
//!   distinction, but they sit inside the oracle's documented twin
//!   blind spot and outside the generator's reach — kept true by
//!   [`split_site`]'s separation filter, which is what stops the one
//!   op whose point comes from geometry rather than the counter from
//!   manufacturing them. (Killing the same edge from the other half is
//!   the strut kill, which IS exactly invertible.)
//! - `kef(he)` where the mate's loop is `[mate]` alone (the killed edge
//!   is then necessarily a self-loop) AND the surviving singleton loop
//!   is a ring — or the outer of a face that has rings. The one-op
//!   re-make `mef(Chords{next(he), next(he)})` re-splits from the
//!   surviving side, which necessarily leaves the big loop on the
//!   survivor's face — the wrong side of the ring distribution. When
//!   the survivor is the outer of a RING-FREE face (the only shape
//!   `mef` itself creates), that same re-make IS exact up to
//!   isomorphism — face identities swap and the oracle does not track
//!   keys — and [`roundtrip`] performs it rather than skipping.
//!
//! Completeness is unharmed (the pre-kill bodies are still
//! Euler-reachable — build the fan/cycle first and strut/cut last); the
//! make/kill pairing is exact **per site**, not per edge half. See
//! `crate::euler_kill`'s docs.

// Test-support code: panicking is a test's failure mechanism (L5).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Decide, Point3, Sign};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, SolidKey};
use crate::euler::{MefSite, MevSite};
use crate::euler_ring::MekrSite;
use crate::test_support_impl::ArenaCounts;

/// [`crate::iso::canonical_form`], through the meter — this is the
/// oracle's deep compare, and [`roundtrip`] is the only caller in this
/// module, twice per executed roundtrip.
fn canonical_form(body: &Body<f64>) -> String {
    meter::note_iso(|| body.half_edges().count() as u64);
    meter::timed(meter::ISO, || crate::iso::canonical_form(body))
}

/// One applicable operator invocation: the op plus a fully resolved
/// site. Produced by [`choose_op`], consumed by [`apply`]/[`roundtrip`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OpChoice {
    Mvfs,
    MevLone(LoopKey),
    MevFan(HalfEdgeKey, HalfEdgeKey),
    MefChords(HalfEdgeKey, HalfEdgeKey),
    MefLone(LoopKey),
    Kemr(HalfEdgeKey, HalfEdgeKey),
    Mekr(MekrSite),
    Kfmrh(FaceKey, FaceKey),
    Mfkrh(LoopKey),
    Kev(HalfEdgeKey),
    Kef(HalfEdgeKey),
    Kvfs(SolidKey),
    /// The non-Euler public mutator (`ring_move(ring, to_face)`).
    /// ring_move's tier-1 preservation is the demotion claim's least
    /// obvious case (the separating-curve argument; see its docs in
    /// `crate::euler_ring`).
    RingMove(LoopKey, FaceKey),
    /// `split_edge(edge, t)`. The parameter is derived from the edge's
    /// own certified interval by [`split_site`], not carried here, so
    /// the choice stays `Copy`/`Eq` and the site stays deterministic.
    SplitEdge(EdgeKey),
}

impl OpChoice {
    /// The op's Euler vector (Mäntylä Table 9.1, our per-op docs).
    pub(crate) fn ep_vector(&self) -> EulerVector {
        match self {
            Self::Mvfs => EulerVector {
                v: 1,
                f: 1,
                s: 1,
                ..Default::default()
            },
            // `split_edge` shares this arm because it shares the
            // vector: an edge split IS a mev applied mid-edge.
            Self::MevLone(_) | Self::MevFan(..) | Self::SplitEdge(_) => EulerVector {
                v: 1,
                e: 1,
                ..Default::default()
            },
            Self::MefChords(..) | Self::MefLone(_) => EulerVector {
                e: 1,
                f: 1,
                ..Default::default()
            },
            Self::Kemr(..) => EulerVector {
                e: -1,
                r: 1,
                ..Default::default()
            },
            Self::Mekr(_) => EulerVector {
                e: 1,
                r: -1,
                ..Default::default()
            },
            Self::Kfmrh(..) => EulerVector {
                f: -1,
                h: 1,
                r: 1,
                ..Default::default()
            },
            Self::Mfkrh(_) => EulerVector {
                f: 1,
                h: -1,
                r: -1,
                ..Default::default()
            },
            Self::Kev(_) => EulerVector {
                v: -1,
                e: -1,
                ..Default::default()
            },
            Self::Kef(_) => EulerVector {
                e: -1,
                f: -1,
                ..Default::default()
            },
            Self::Kvfs(_) => EulerVector {
                v: -1,
                f: -1,
                s: -1,
                ..Default::default()
            },
            // NOT an Euler operator: pure reparenting, zero vector.
            Self::RingMove(..) => EulerVector::default(),
        }
    }
}

/// One operator's signed Euler–Poincaré shift `(Δv, Δe, Δf, Δh, Δr, Δs)`.
///
/// The same six components as the running [`Ledger`], carried as a
/// shift rather than a count. Call sites name only the nonzero
/// components and take the rest from the derived zero, so a site reads
/// as the op's actual shift.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct EulerVector {
    pub v: i64,
    pub e: i64,
    pub f: i64,
    pub h: i64,
    pub r: i64,
    pub s: i64,
}

/// The running Euler–Poincaré ledger `(v, e, f, h, r, s)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Ledger {
    pub v: i64,
    pub e: i64,
    pub f: i64,
    pub h: i64,
    pub r: i64,
    pub s: i64,
}

impl Ledger {
    /// Adds an op's Euler vector.
    pub(crate) fn apply(&mut self, delta: EulerVector) {
        self.v += delta.v;
        self.e += delta.e;
        self.f += delta.f;
        self.h += delta.h;
        self.r += delta.r;
        self.s += delta.s;
    }

    /// Checks the ledger against the body: v/e/f/s are arena counts, r
    /// is the summed ring count, and h must satisfy eq. 9.2
    /// (`v − e + f = 2(s − h) + r`) with the derived genus matching the
    /// ledger's.
    ///
    /// Derived h is NOT required to be non-negative: `mfkrh` applied to
    /// a ring that never plugged a handle (a kemr-planted floating ring)
    /// legitimately drives it negative — the promotion disconnects the
    /// shell's surface into two components while keeping one shell
    /// entity, so eq. 9.2's h stops meaning "genus of a connected
    /// closed surface" and starts double-counting components. A legal
    /// tier-1 intermediate (like empty loops and struts). The
    /// component-aware per-shell form (per component
    /// v − e + f − r = 2(1 − g), g ∈ ℤ≥0; per shell the sum is
    /// 2(c − Σgᵢ)) was ratified at M1 PR 5, corrected by PR 4, and
    /// is implemented as the validator's pass 11
    /// (`crate::validate`); this ledger keeps the per-body eq. 9.2 view
    /// because the generator tracks the operator algebra, not surfaces.
    pub(crate) fn check(&self, body: &Body<f64>) -> Result<(), String> {
        let v = body.vertices().count() as i64;
        let e = body.edges().count() as i64;
        let f = body.faces().count() as i64;
        let s = body.solids().count() as i64;
        let r: i64 = body.faces().map(|(_, face)| face.rings.len() as i64).sum();
        if (v, e, f, s, r) != (self.v, self.e, self.f, self.s, self.r) {
            return Err(format!(
                "ledger mismatch: body (v{v} e{e} f{f} s{s} r{r}) vs ledger {self:?}"
            ));
        }
        // Eq. 9.2 rearranged: 2h = 2s − (v − e + f − r).
        let twice_h = 2 * s - (v - e + f - r);
        if twice_h % 2 != 0 {
            return Err(format!("Euler–Poincaré parity violated: 2h = {twice_h}"));
        }
        let derived_h = twice_h / 2;
        if derived_h != self.h {
            return Err(format!(
                "genus mismatch: derived h = {derived_h}, ledger h = {}",
                self.h
            ));
        }
        Ok(())
    }
}

/// Bodies stop growing once they hold this many half-edges (the make
/// weights drop to zero and the kill weights rise).
const GROW_CAP: usize = 28;

thread_local! {
    /// Whether the `split_edge` row carries its weight in the catalog
    /// below. Always true outside [`without_split_edge`], which is the
    /// only writer.
    static SPLIT_EDGE_ROW: std::cell::Cell<bool> = const { std::cell::Cell::new(true) };
}

/// Runs `f` against the catalog **without** the `split_edge` row —
/// a zero-weight row is skipped whole, so this is the catalog as it
/// reads with the row absent, and nothing else about the walk changes.
///
/// It exists because the lane's cost cannot be attributed to a
/// catalog row by reading one number: the row's own work and the
/// larger bodies its presence produces are different quantities, and
/// only a replay of the SAME decision streams against both catalogs
/// separates them. The walk this produces is a different walk — that
/// is the point, and it is why this is a measurement instrument and
/// not a configuration.
pub(crate) fn without_split_edge<R>(f: impl FnOnce() -> R) -> R {
    SPLIT_EDGE_ROW.with(|c| c.set(false));
    let out = f();
    SPLIT_EDGE_ROW.with(|c| c.set(true));
    out
}

/// Picks one applicable `(op, site)` from the current body, driven by
/// two decision integers (see the module docs). `None` only when
/// NOTHING is applicable, which for this catalog means the body is
/// empty and even `mvfs` was weighted out — in practice never, since
/// `mvfs` keeps weight on empty bodies.
pub(crate) fn choose_op(body: &Body<f64>, d1: u32, d2: u32) -> Option<OpChoice> {
    let grow = body.half_edges().count() < GROW_CAP;
    let w = |grown: u32, shrunk: u32| if grow { grown } else { shrunk };
    type Enumerate = fn(&Body<f64>) -> Vec<OpChoice>;
    type Probe = fn(&Body<f64>) -> bool;
    // (weight, enumerator, short-circuiting emptiness probe) per op
    // kind, in fixed catalog order.
    //
    // **What the roll actually needs from a row.** Its weight, and
    // whether it has any candidate at all — the list itself only for
    // the one row the roll lands on. A zero-weight row is asked
    // nothing: it adds nothing to the total and the selection loop
    // skips it, so its candidates cannot change the choice. A row
    // whose enumeration is cheap answers emptiness by building the
    // list, which is then kept for the roll; a row that carries a
    // `Some(probe)` is one whose enumeration is expensive enough that
    // building a list the roll then discards is not affordable —
    // `split_edge_candidates` re-certifies every edge and meters its
    // split point against every vertex, and the roll lands on that
    // row a small fraction of the times the row is asked. No ratio is
    // written here: it is a function of the weights below and of the
    // body, and a constant beside a table that determines it is a
    // constant nothing keeps true.
    let kinds: [(u32, Enumerate, Option<Probe>); 14] = [
        (
            if body.solids().count() < 2 {
                w(1, 0)
            } else {
                0
            },
            mvfs_candidates,
            None,
        ),
        (w(5, 0), mev_lone_candidates, None),
        (w(6, 0), mev_fan_candidates, None),
        (w(6, 0), mef_chords_candidates, None),
        (w(2, 0), mef_lone_candidates, None),
        (3, kemr_candidates, None),
        (w(3, 1), mekr_candidates, None),
        (2, kfmrh_candidates, None),
        (w(2, 1), mfkrh_candidates, None),
        // The non-Euler public mutator rides along at modest weight:
        // candidates exist whenever any face carries a ring, which kemr
        // (always-on weight) produces steadily.
        (w(2, 1), ring_move_candidates, None),
        // `split_edge` is make-direction (mev's vector), so it is
        // weighted out once the body stops growing. Candidates exist
        // whenever any edge does, which is nearly always — modest
        // weight keeps it from crowding out the sites only the other
        // make ops reach.
        (
            if SPLIT_EDGE_ROW.with(std::cell::Cell::get) {
                w(3, 0)
            } else {
                0
            },
            split_edge_candidates,
            Some(any_split_edge),
        ),
        (w(2, 6), kev_candidates, None),
        (w(2, 6), kef_candidates, None),
        // kvfs candidates are rare (a solid must be exactly skeletal),
        // so give the row real weight when one exists — otherwise the
        // random arm almost never rolls it (review SHOULD-3). Teardown
        // still exercises kvfs deterministically at the end of every
        // proptest case; this weight only adds mid-sequence coverage.
        (4, kvfs_candidates, None),
    ];
    // The rows the roll can land on — weighted in, and non-empty — in
    // catalog order, so the total and the walk below are the ones the
    // eager form computed.
    let mut rows: Vec<(u32, Rolled)> = Vec::new();
    for (weight, enumerate, probe) in kinds {
        if weight == 0 {
            continue;
        }
        match probe {
            None => {
                let candidates = enumerate(body);
                if !candidates.is_empty() {
                    rows.push((weight, Rolled::Built(candidates)));
                }
            }
            Some(any) => {
                if any(body) {
                    rows.push((weight, Rolled::Deferred(enumerate)));
                }
            }
        }
    }
    let total: u32 = rows.iter().map(|(weight, _)| weight).sum();
    if total == 0 {
        return None;
    }
    let mut roll = d1 % total;
    for (weight, row) in rows {
        if roll < weight {
            let candidates = match row {
                Rolled::Built(candidates) => candidates,
                Rolled::Deferred(enumerate) => enumerate(body),
            };
            let index = (d2 as usize) % candidates.len();
            return Some(candidates[index]);
        }
        roll -= weight;
    }
    None // unreachable: roll < total by construction
}

/// A row that survived the weight and emptiness gates, holding either
/// the candidates already built or the enumerator that will build them
/// if the roll lands here.
enum Rolled {
    Built(Vec<OpChoice>),
    Deferred(fn(&Body<f64>) -> Vec<OpChoice>),
}

/// `mvfs` needs no site — it mints a fresh solid — so its single
/// candidate exists whenever the row carries weight.
fn mvfs_candidates(_body: &Body<f64>) -> Vec<OpChoice> {
    vec![OpChoice::Mvfs]
}

fn mev_lone_candidates(body: &Body<f64>) -> Vec<OpChoice> {
    empty_loops(body)
        .into_iter()
        .map(OpChoice::MevLone)
        .collect()
}

fn mef_lone_candidates(body: &Body<f64>) -> Vec<OpChoice> {
    empty_loops(body)
        .into_iter()
        .map(OpChoice::MefLone)
        .collect()
}

fn empty_loops(body: &Body<f64>) -> Vec<LoopKey> {
    body.loops()
        .filter(|(_, l)| matches!(l.boundary, LoopBoundary::Empty { .. }))
        .map(|(k, _)| k)
        .collect()
}

fn mev_fan_candidates(body: &Body<f64>) -> Vec<OpChoice> {
    let mut out = Vec::new();
    for (he1, _) in body.half_edges() {
        let orbit = body.vertex_orbit(he1).expect("valid body: orbit closes");
        for he2 in orbit {
            out.push(OpChoice::MevFan(he1, he2));
        }
    }
    out
}

fn mef_chords_candidates(body: &Body<f64>) -> Vec<OpChoice> {
    let mut out = Vec::new();
    for (_, loop_data) in body.loops() {
        let LoopBoundary::Cycle { first } = loop_data.boundary else {
            continue;
        };
        let cycle = body.loop_cycle(first).expect("valid body: cycle closes");
        for &he1 in &cycle {
            for &he2 in &cycle {
                out.push(OpChoice::MefChords(he1, he2));
            }
        }
    }
    out
}

fn kemr_candidates(body: &Body<f64>) -> Vec<OpChoice> {
    let mut out = Vec::new();
    for (_, edge) in body.edges() {
        let plus = body.get_half_edge(edge.he_plus).expect("half resolves");
        let minus = body.get_half_edge(edge.he_minus).expect("half resolves");
        if plus.parent_loop == minus.parent_loop {
            // Both argument orders: the side association differs.
            out.push(OpChoice::Kemr(edge.he_plus, edge.he_minus));
            out.push(OpChoice::Kemr(edge.he_minus, edge.he_plus));
        }
    }
    out
}

fn mekr_candidates(body: &Body<f64>) -> Vec<OpChoice> {
    let mut out = Vec::new();
    for (_, face) in body.faces() {
        let loops: Vec<LoopKey> = core::iter::once(face.outer)
            .chain(face.rings.iter().copied())
            .collect();
        for &target in &loops {
            for &ring in &face.rings {
                if target == ring {
                    continue;
                }
                let target_boundary = body.get_loop(target).expect("loop resolves").boundary;
                let ring_boundary = body.get_loop(ring).expect("loop resolves").boundary;
                match (target_boundary, ring_boundary) {
                    (
                        LoopBoundary::Cycle { first: t_first },
                        LoopBoundary::Cycle { first: r_first },
                    ) => {
                        let t_cycle = body.loop_cycle(t_first).expect("cycle closes");
                        let r_cycle = body.loop_cycle(r_first).expect("cycle closes");
                        for &t in &t_cycle {
                            for &r in &r_cycle {
                                out.push(OpChoice::Mekr(MekrSite::Cycles { target: t, ring: r }));
                            }
                        }
                    }
                    (LoopBoundary::Cycle { first: t_first }, LoopBoundary::Empty { .. }) => {
                        for t in body.loop_cycle(t_first).expect("cycle closes") {
                            out.push(OpChoice::Mekr(MekrSite::EmptyRing { target: t, ring }));
                        }
                    }
                    (LoopBoundary::Empty { .. }, LoopBoundary::Cycle { first: r_first }) => {
                        for r in body.loop_cycle(r_first).expect("cycle closes") {
                            out.push(OpChoice::Mekr(MekrSite::EmptyTarget { target, ring: r }));
                        }
                    }
                    (LoopBoundary::Empty { vertex: u }, LoopBoundary::Empty { vertex: w }) => {
                        if u != w {
                            out.push(OpChoice::Mekr(MekrSite::BothEmpty { target, ring }));
                        }
                    }
                }
            }
        }
    }
    out
}

fn kfmrh_candidates(body: &Body<f64>) -> Vec<OpChoice> {
    let mut out = Vec::new();
    for (f1, face1) in body.faces() {
        for (f2, face2) in body.faces() {
            if f1 != f2 && face1.shell == face2.shell && face2.rings.is_empty() {
                out.push(OpChoice::Kfmrh(f1, f2));
            }
        }
    }
    out
}

fn mfkrh_candidates(body: &Body<f64>) -> Vec<OpChoice> {
    let mut out = Vec::new();
    for (_, face) in body.faces() {
        for &ring in &face.rings {
            out.push(OpChoice::Mfkrh(ring));
        }
    }
    out
}

/// Every genuine ring reparenting: each ring of each face, moved to
/// every OTHER same-shell face (the same-face no-op is excluded — it
/// exercises nothing). Deterministic double sweep of the face arena.
fn ring_move_candidates(body: &Body<f64>) -> Vec<OpChoice> {
    let mut out = Vec::new();
    for (from, face) in body.faces() {
        for &ring in &face.rings {
            for (to, target) in body.faces() {
                if to != from && target.shell == face.shell {
                    out.push(OpChoice::RingMove(ring, to));
                }
            }
        }
    }
    out
}

/// Every edge whose carrier [`split_site`] admits: the curve entry
/// must be certified (null scaffolding has nothing to split) and must
/// still describe the edge's current endpoints. A certified interval
/// is forward by construction, so an interior fraction of it is
/// definitely interior on both sides.
fn split_edge_candidates(body: &Body<f64>) -> Vec<OpChoice> {
    split_edge_sites(body).collect()
}

/// Whether [`split_edge_candidates`] would return anything, without
/// building it. Same iterator, stopped at the first item: the two
/// cannot disagree about emptiness, which is what [`choose_op`]'s roll
/// needs from this row and all it needs.
fn any_split_edge(body: &Body<f64>) -> bool {
    split_edge_sites(body).next().is_some()
}

fn split_edge_sites(body: &Body<f64>) -> impl Iterator<Item = OpChoice> + '_ {
    body.edges()
        .map(|(e, _)| e)
        .filter(move |e| split_site(body, *e).is_some())
        .map(OpChoice::SplitEdge)
}

/// Where in an edge's interval a split lands. **Not the midpoint.**
/// Every point this generator mints sits on the counter lattice
/// `(n, 0.5, 0.25)`, so the midpoint of the chord from `n` to `n + 2`
/// is vertex `n + 1`'s own point exactly: a midpoint split would
/// manufacture coordinate-coincident vertices, which `mef_chord` then
/// refuses on the zero-length chord (`IntervalNotForward`) and which
/// sit inside the isomorphism oracle's documented twin blind spot.
/// The nearest `f64` to `(√5 − 1)/2` is of course rational, so this
/// constant is a way of MISSING the lattice, not a proof that it
/// cannot be hit — what enforces distinctness is
/// [`split_site`]'s separation filter.
const SPLIT_FRACTION: f64 = 0.618_033_988_749_895;

/// `edge`'s split parameter together with the parent's OWN spec,
/// rebuilt from the certified curve — `None` when the edge is not
/// splittable.
///
/// **Why the re-certification.** This lane fuzzes STRUCTURE: the
/// fan-rebasing ops (`mev`'s fan site, `kev`'s fan merge) move a run
/// of half-edges onto a different vertex without re-describing the
/// survivors' carriers (each says so in its own docs), so an edge's
/// stored curve is routinely stale against its own endpoints. Tier 1
/// does not constrain that and the isomorphism oracle ignores
/// carriers, but `split_edge` certifies both children against the
/// CURRENT endpoint points and refuses. So the candidate test
/// re-derives the parent's certificate against those points through
/// [`geom_brep::EdgeCurve::recertify`] — the door `split_edge` itself
/// certifies through, not tier 3's `recertify_nurbs_lane`, which
/// admits a strictly wider class and so would let candidates past this
/// gate that the operator then refuses. Its consequence is a real
/// coverage limit, stated where it is caused: **only carrier-coherent
/// edges are ever split here.**
///
/// **Why the spec comes back with it.** `split_edge` is the only
/// catalog member that REPLACES existing geometry rather than only
/// minting: the parent survives as the first child, carrying the
/// `[t₀, t]` restriction. `kev` undoes the topology and leaves that
/// restriction on an edge spanning the whole original again, so the
/// captured spec is what completes the inverse — exactly, for any
/// carrier (the self-loop circles `mef_chord` mints included), which
/// a `line_between` guess would not be.
fn split_site(body: &Body<f64>, edge: EdgeKey) -> Option<(f64, EdgeCurveSpec<f64>)> {
    let edge_data = body.get_edge(edge)?;
    let hp = edge_data.he_plus;
    let start = body.get_half_edge(hp)?.start;
    let end = body.half_edge_end(hp)?;
    let p0 = *body.get_point(body.get_vertex(start)?.point)?;
    let p1 = *body.get_point(body.get_vertex(end)?.point)?;
    let curve = body.get_curve_geom(edge_data.curve)?.certified()?;
    let band = Band::linear().ok()?;
    curve
        .recertify(p0, p1, |k| body.get_surface(k).cloned(), band)
        .ok()?;
    let (t0, t1) = curve.params();
    let t = SPLIT_FRACTION.mul_add(t1 - t0, t0);
    // **The generator's coordinate-distinctness policy, enforced.**
    // Every OTHER minting op takes its point from the counter, which
    // makes distinctness free; a split point is derived from GEOMETRY,
    // so two edges over the same pair of points mint the same point —
    // two self-loops at one vertex (identical `self_loop_circle_at`
    // circles), or two parallel edges over one vertex pair. The
    // separation must be DEFINITE, not merely bitwise: two such splits
    // land one ulp apart, and `mef_chord` then refuses their chord as
    // an unmeterable zero-length carrier. Same metering the chord
    // sugar uses, so a candidate that passes here cannot poison one.
    // This filter is also what keeps the irreversible-`kev` taxonomy's
    // "outside the generator's reach" clause (module docs) true now
    // that a non-counter point source exists.
    //
    // Raw `Decide::sign_within` rather than the `k_stats` funnel, and
    // the rule genuinely does not bite here (`boolean/ops.rs` records
    // why the bypass was retired in kernel code): this is a
    // TEST-SUPPORT generator over `Body<f64>` only, never instantiated
    // at the recording scalar, so there is no K row to misattribute —
    // and the decision is a candidate filter, not a kernel predicate
    // any output depends on.
    let minted = curve.carrier().eval(t);
    if body
        .vertices()
        .filter_map(|(_, v)| body.get_point(v.point))
        .any(|p| !matches!(p.distance(minted).sign_within(band), Ok(Sign::Positive)))
    {
        return None;
    }
    Some((
        t,
        EdgeCurveSpec {
            description: *curve.description(),
            carrier: curve.carrier().clone(),
            param_start: t0,
            param_end: t1,
        },
    ))
}

fn kev_candidates(body: &Body<f64>) -> Vec<OpChoice> {
    body.half_edges()
        .filter(|&(he, he_data)| body.half_edge_end(he) != Some(he_data.start))
        .map(|(he, _)| OpChoice::Kev(he))
        .collect()
}

fn kef_candidates(body: &Body<f64>) -> Vec<OpChoice> {
    let mut out = Vec::new();
    for (he, he_data) in body.half_edges() {
        let mate = body.mate(he).expect("valid body: mate resolves");
        let mate_loop = body.get_half_edge(mate).expect("half resolves").parent_loop;
        if mate_loop == he_data.parent_loop {
            continue;
        }
        let dying_face = body
            .get_loop(he_data.parent_loop)
            .expect("loop resolves")
            .face;
        let mate_face = body.get_loop(mate_loop).expect("loop resolves").face;
        if dying_face == mate_face {
            continue;
        }
        if !body
            .get_face(dying_face)
            .expect("face resolves")
            .rings
            .is_empty()
        {
            continue;
        }
        out.push(OpChoice::Kef(he));
    }
    out
}

fn kvfs_candidates(body: &Body<f64>) -> Vec<OpChoice> {
    body.solids()
        .filter(|&(solid, _)| is_skeletal(body, solid))
        .map(|(solid, _)| OpChoice::Kvfs(solid))
        .collect()
}

/// `true` iff the solid is exactly the skeletal `mvfs` state.
fn is_skeletal(body: &Body<f64>, solid: SolidKey) -> bool {
    let solid_data = body.get_solid(solid).expect("solid resolves");
    let [shell] = solid_data.shells[..] else {
        return false;
    };
    let [face] = body.get_shell(shell).expect("shell resolves").faces[..] else {
        return false;
    };
    let face_data = body.get_face(face).expect("face resolves");
    face_data.rings.is_empty()
        && matches!(
            body.get_loop(face_data.outer)
                .expect("loop resolves")
                .boundary,
            LoopBoundary::Empty { .. }
        )
}

/// Distinct coordinates for the vertex-minting ops, from a caller-owned
/// counter.
fn next_point(counter: &mut u32) -> Point3<f64> {
    *counter += 1;
    Point3::new(f64::from(*counter), 0.5, 0.25)
}

/// Executes one choice. Panics on operator errors: [`choose_op`] only
/// returns applicable sites, so an error here is a bug in either the
/// enumeration or the operator.
pub(crate) fn apply(body: &mut Body<f64>, choice: OpChoice, counter: &mut u32) {
    match choice {
        OpChoice::Mvfs => {
            body.mvfs(next_point(counter)).unwrap();
        }
        OpChoice::MevLone(l) => {
            body.mev_line(MevSite::Lone { r#loop: l }, next_point(counter))
                .unwrap();
        }
        OpChoice::MevFan(he1, he2) => {
            body.mev_line(MevSite::Fan { he1, he2 }, next_point(counter))
                .unwrap();
        }
        OpChoice::MefChords(he1, he2) => {
            body.mef_chord(MefSite::Chords { he1, he2 }).unwrap();
        }
        OpChoice::MefLone(l) => {
            body.mef_chord(MefSite::Lone { r#loop: l }).unwrap();
        }
        OpChoice::Kemr(he1, he2) => {
            body.kemr(he1, he2).unwrap();
        }
        OpChoice::Mekr(site) => {
            body.mekr_chord(site).unwrap();
        }
        OpChoice::Kfmrh(f1, f2) => {
            body.kfmrh(f1, f2).unwrap();
        }
        OpChoice::Mfkrh(ring) => {
            body.mfkrh_plug(ring).unwrap();
        }
        OpChoice::Kev(he) => {
            body.kev(he).unwrap();
        }
        OpChoice::Kef(he) => {
            body.kef(he).unwrap();
        }
        OpChoice::Kvfs(solid) => {
            body.kvfs(solid).unwrap();
        }
        OpChoice::RingMove(ring, to_face) => {
            body.ring_move(ring, to_face).unwrap();
        }
        OpChoice::SplitEdge(e) => meter::timed(meter::APPLY_SPLIT, || {
            let (t, _) = split_site(body, e).expect("a split candidate has a splittable carrier");
            body.split_edge(e, t).unwrap();
        }),
    }
}

/// What [`roundtrip`] did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RoundtripOutcome {
    /// Op and inverse ran; canonical form was asserted restored.
    Done,
    /// The choice was one of the two documented irreversible-by-one-op
    /// kill sites (module docs); nothing was executed.
    SkippedIrreversible,
}

/// Executes `choice` followed by its exact inverse (make ∘ kill) or by
/// the derived re-make (kill ∘ make), asserting that the canonical form
/// is restored. The body nets zero change up to isomorphism.
pub(crate) fn roundtrip(
    body: &mut Body<f64>,
    choice: OpChoice,
    counter: &mut u32,
) -> RoundtripOutcome {
    let before = canonical_form(body);
    match choice {
        // ---- make ∘ kill: the created keys address the inverse. ----
        OpChoice::Mvfs => {
            let created = body.mvfs(next_point(counter)).unwrap();
            body.kvfs(created.solid).unwrap();
        }
        OpChoice::MevLone(l) => {
            let created = body
                .mev_line(MevSite::Lone { r#loop: l }, next_point(counter))
                .unwrap();
            body.kev(created.he_plus).unwrap();
        }
        OpChoice::MevFan(he1, he2) => {
            let created = body
                .mev_line(MevSite::Fan { he1, he2 }, next_point(counter))
                .unwrap();
            body.kev(created.he_plus).unwrap();
        }
        OpChoice::MefChords(he1, he2) => {
            let created = body.mef_chord(MefSite::Chords { he1, he2 }).unwrap();
            body.kef(created.he_minus).unwrap();
        }
        OpChoice::MefLone(l) => {
            let created = body.mef_chord(MefSite::Lone { r#loop: l }).unwrap();
            body.kef(created.he_minus).unwrap();
        }
        OpChoice::Mekr(site) => {
            let created = body.mekr_chord(site).unwrap();
            body.kemr(created.he_plus, created.he_minus).unwrap();
        }
        OpChoice::Mfkrh(ring) => {
            let old_face = body.get_loop(ring).expect("ring resolves").face;
            let created = body.mfkrh_plug(ring).unwrap();
            body.kfmrh(old_face, created.face).unwrap();
        }
        OpChoice::RingMove(ring, to_face) => {
            // Self-paired: move there, move back. Exact up to ring-list
            // order, which the canonical form sorts away (iso docs).
            let old_face = body.get_loop(ring).expect("ring resolves").face;
            body.ring_move(ring, to_face).unwrap();
            body.ring_move(ring, old_face).unwrap();
        }
        OpChoice::SplitEdge(e) => {
            // Two-op inverse, and the second op is not bookkeeping:
            // `split_edge` replaces the parent's description with the
            // `[t₀, t]` child, so `kev` restores the topology and the
            // re-attach restores the geometry (see `split_site`).
            meter::timed(meter::ROUNDTRIP_SPLIT, || {
                let (t, spec) =
                    split_site(body, e).expect("a split candidate has a splittable carrier");
                let created = body.split_edge(e, t).unwrap();
                body.kev(created.he_minus).unwrap();
                body.set_edge_curve(e, spec).unwrap();
            });
        }
        // ---- kill ∘ make: the re-make site is derived pre-kill. ----
        OpChoice::Kemr(he1, he2) => {
            let he1_next = body.get_half_edge(he1).expect("resolves").next;
            let he2_next = body.get_half_edge(he2).expect("resolves").next;
            let old_loop = body.get_half_edge(he1).expect("resolves").parent_loop;
            let ring_side_empty = he1_next == he2;
            let old_side_empty = he2_next == he1;
            let result = body.kemr(he1, he2).unwrap();
            let site = match (ring_side_empty, old_side_empty) {
                (false, false) => MekrSite::Cycles {
                    target: he2_next,
                    ring: he1_next,
                },
                (true, false) => MekrSite::EmptyRing {
                    target: he2_next,
                    ring: result.ring,
                },
                (false, true) => MekrSite::EmptyTarget {
                    target: old_loop,
                    ring: he1_next,
                },
                (true, true) => MekrSite::BothEmpty {
                    target: old_loop,
                    ring: result.ring,
                },
            };
            body.mekr_chord(site).unwrap();
        }
        OpChoice::Kfmrh(f1, f2) => {
            let result = body.kfmrh(f1, f2).unwrap();
            body.mfkrh_plug(result.ring).unwrap();
        }
        OpChoice::Kvfs(solid) => {
            // Record the lone vertex's coordinates for the re-make.
            let shell = body.get_solid(solid).expect("resolves").shells[0];
            let face = body.get_shell(shell).expect("resolves").faces[0];
            let outer = body.get_face(face).expect("resolves").outer;
            let LoopBoundary::Empty { vertex } = body.get_loop(outer).expect("resolves").boundary
            else {
                panic!("kvfs candidate must be skeletal");
            };
            let point = body.get_vertex(vertex).expect("resolves").point;
            let coords = *body.get_point(point).expect("resolves");
            body.kvfs(solid).unwrap();
            body.mvfs(coords).unwrap();
        }
        OpChoice::Kev(he) => {
            let he_data = body.get_half_edge(he).expect("resolves").clone();
            let mate = body.mate(he).expect("mate resolves");
            let mate_data = body.get_half_edge(mate).expect("resolves").clone();
            let (b, d) = (he_data.next, mate_data.next);
            let w = mate_data.start;
            let w_coords = *body
                .get_point(body.get_vertex(w).expect("resolves").point)
                .expect("resolves");
            if d == he && b != mate {
                // The mirror site: no single-mev re-make (module docs).
                return RoundtripOutcome::SkippedIrreversible;
            }
            let l1 = he_data.parent_loop;
            body.kev(he).unwrap();
            let site = if b == mate && d == he {
                MevSite::Lone { r#loop: l1 } // segment kill
            } else if b == mate {
                MevSite::Fan { he1: d, he2: d } // strut kill
            } else {
                MevSite::Fan { he1: b, he2: d } // general fan merge
            };
            body.mev_line(site, w_coords).unwrap();
        }
        OpChoice::Kef(he) => {
            let he_data = body.get_half_edge(he).expect("resolves").clone();
            let mate = body.mate(he).expect("mate resolves");
            let mate_data = body.get_half_edge(mate).expect("resolves").clone();
            let (b, d) = (he_data.next, mate_data.next);
            let l2 = mate_data.parent_loop;
            if d == mate && b != he {
                // The mate-alone site (the killed edge is necessarily a
                // self-loop). Re-makeable by one op iff the surviving
                // singleton loop is the outer of a ring-free face —
                // module docs; the re-make re-splits from the surviving
                // side, so the survivor's face must be shaped like the
                // face mef mints.
                let survivor_face = body.get_loop(l2).expect("resolves").face;
                let survivor_face_data = body.get_face(survivor_face).expect("resolves");
                if survivor_face_data.outer != l2 || !survivor_face_data.rings.is_empty() {
                    return RoundtripOutcome::SkippedIrreversible;
                }
                body.kef(he).unwrap();
                body.mef_chord(MefSite::Chords { he1: b, he2: b }).unwrap();
            } else {
                body.kef(he).unwrap();
                let site = if b == he && d == mate {
                    MefSite::Lone { r#loop: l2 } // self-loop pair kill
                } else if b == he {
                    MefSite::Chords { he1: d, he2: d } // circular-face kill
                } else {
                    MefSite::Chords { he1: b, he2: d } // general splice
                };
                body.mef_chord(site).unwrap();
            }
        }
    }
    let after = canonical_form(body);
    assert_eq!(
        before, after,
        "roundtrip failed to restore the canonical form for {choice:?}",
    );
    RoundtripOutcome::Done
}

/// Drives the body back to completely empty arenas through the kill
/// direction (the ultimate kill-hygiene check): kef/kev/kemr shrink the
/// structure, mfkrh/mekr/kfmrh resolve rings and empty-outer faces, and
/// kvfs retires each skeletal solid. Panics if no progress is possible
/// (a completeness bug) or the step cap is exceeded.
///
/// **The `*_candidates(body).first()` reads below build a list to take
/// one element, and that is deliberate.** It is the shape
/// [`choose_op`] does not use, for a reason that does not carry here:
/// these three enumerators are pointer walks over the arenas, and the
/// cost of the shape is a rounding error against what the kill ops
/// themselves cost — the whole of teardown is ~1% of this lane, and
/// making these reads lazy moved ~1% of THAT. Enumerate lazily where
/// the enumerator is expensive, which here it is not.
pub(crate) fn teardown(body: &mut Body<f64>) {
    let cap =
        10 * (body.half_edges().count() + body.faces().count() + body.vertices().count()) + 100;
    for _ in 0..cap {
        if body.solids().count() == 0 {
            assert_eq!(body.shells().count(), 0);
            assert_eq!(body.faces().count(), 0);
            assert_eq!(body.loops().count(), 0);
            assert_eq!(body.half_edges().count(), 0);
            assert_eq!(body.edges().count(), 0);
            assert_eq!(body.vertices().count(), 0);
            assert_eq!(body.points().count(), 0);
            assert_eq!(body.curves().count(), 0);
            assert_eq!(body.surfaces().count(), 0);
            // Kill hygiene, the whole point: no provenance record
            // outlives its entity.
            assert_eq!(body.solid_provenance.len(), 0);
            assert_eq!(body.shell_provenance.len(), 0);
            assert_eq!(body.face_provenance.len(), 0);
            assert_eq!(body.loop_provenance.len(), 0);
            assert_eq!(body.half_edge_provenance.len(), 0);
            assert_eq!(body.edge_provenance.len(), 0);
            assert_eq!(body.vertex_provenance.len(), 0);
            return;
        }
        if let Some(OpChoice::Kef(he)) = kef_candidates(body).first().copied() {
            body.kef(he).unwrap();
            continue;
        }
        if let Some(OpChoice::Kev(he)) = kev_candidates(body).first().copied() {
            body.kev(he).unwrap();
            continue;
        }
        if let Some(OpChoice::Kemr(he1, he2)) = kemr_candidates(body).first().copied() {
            body.kemr(he1, he2).unwrap();
            continue;
        }
        // Cycle rings: promote to a face (kef will consume it next).
        if let Some(ring) = first_cycle_ring(body) {
            body.mfkrh_plug(ring).unwrap();
            continue;
        }
        // Empty rings: absorb with mekr, then kill the fresh edge (and
        // the stranded vertex) with kev — a compound step so the
        // potential still shrinks.
        if let Some(site) = first_empty_ring_site(body) {
            let created = body.mekr_chord(site).unwrap();
            body.kev(created.he_plus).unwrap();
            continue;
        }
        // Extra empty-outer faces (mfkrh leftovers): fold into a
        // sibling face as an empty ring.
        if let Some((f1, f2)) = first_empty_outer_extra_face(body) {
            body.kfmrh(f1, f2).unwrap();
            continue;
        }
        if let Some(OpChoice::Kvfs(solid)) = kvfs_candidates(body).first().copied() {
            body.kvfs(solid).unwrap();
            continue;
        }
        panic!("teardown stuck: no applicable kill-direction step");
    }
    panic!("teardown step cap exceeded");
}

/// The first ring with a `Cycle` boundary, in face/ring scan order.
fn first_cycle_ring(body: &Body<f64>) -> Option<LoopKey> {
    for (_, face) in body.faces() {
        for &ring in &face.rings {
            if matches!(
                body.get_loop(ring).expect("loop resolves").boundary,
                LoopBoundary::Cycle { .. }
            ) {
                return Some(ring);
            }
        }
    }
    None
}

/// A mekr site absorbing the first empty ring into a sibling loop of
/// its face.
fn first_empty_ring_site(body: &Body<f64>) -> Option<MekrSite> {
    for (_, face) in body.faces() {
        for &ring in &face.rings {
            let LoopBoundary::Empty { vertex: w } =
                body.get_loop(ring).expect("loop resolves").boundary
            else {
                continue;
            };
            // Prefer a cycle target (outer first, then other rings).
            for target in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
                if target == ring {
                    continue;
                }
                match body.get_loop(target).expect("loop resolves").boundary {
                    LoopBoundary::Cycle { first } => {
                        return Some(MekrSite::EmptyRing {
                            target: first,
                            ring,
                        });
                    }
                    LoopBoundary::Empty { vertex: u } if u != w => {
                        return Some(MekrSite::BothEmpty { target, ring });
                    }
                    LoopBoundary::Empty { .. } => {}
                }
            }
        }
    }
    None
}

/// An `(f1, f2)` pair where `f2` is a ring-free empty-outer face in a
/// shell with at least one other face — `kfmrh` folds it away.
fn first_empty_outer_extra_face(body: &Body<f64>) -> Option<(FaceKey, FaceKey)> {
    for (f2, face2) in body.faces() {
        if !face2.rings.is_empty() {
            continue;
        }
        if !matches!(
            body.get_loop(face2.outer).expect("loop resolves").boundary,
            LoopBoundary::Empty { .. }
        ) {
            continue;
        }
        let sibling = body
            .faces()
            .find(|&(f1, face1)| f1 != f2 && face1.shell == face2.shell);
        if let Some((f1, _)) = sibling {
            return Some((f1, f2));
        }
    }
    None
}

/// **The phase meter: where this lane's time goes, and how much body
/// it goes over.**
///
/// The lane's cost is not concentrated in one function — every step
/// runs a whole-body tier-1 validation (twice: the operator's own
/// debug postcondition and the property's explicit check), a ledger
/// reconciliation against derived arena counts, and, on a roundtrip
/// step, two canonical forms. So *the size of the body a step runs
/// against* is a cost input on the same footing as the step's own
/// work, and attributing a change in this lane's wall clock means
/// separating the two.
///
/// Two kinds of reading, and the second is the one that travels:
///
/// - **Elapsed time per phase**, which is only meaningful against the
///   box that produced it and against the other phases of the SAME
///   process (`memories/perf-measurement-lane.md`).
/// - **Counters** — steps, entities walked, validations run — which
///   are exact, deterministic, and identical on every machine. Reach
///   for these first: they say what work was done, and the clock only
///   says what it cost here.
///
/// Recording is per-thread and costs two `Instant::now()` calls per
/// phase, which is why it is always on rather than behind a flag. The
/// arena walks that feed the counters are NOT: they are O(body) and
/// would perturb the very timings they annotate, so they run only when
/// [`reset`] is asked for them.
pub(crate) mod meter {
    use std::cell::RefCell;
    use std::time::{Duration, Instant};

    use super::ArenaCounts;

    /// Phase index for [`timed`]. The first six PARTITION a replay of
    /// the properties; the last three are sub-phases of the row above
    /// them (already counted there, never added again).
    pub(crate) const CHOOSE: usize = 0;
    pub(crate) const APPLY: usize = 1;
    pub(crate) const ROUNDTRIP: usize = 2;
    pub(crate) const VALIDATE: usize = 3;
    pub(crate) const LEDGER: usize = 4;
    pub(crate) const TEARDOWN: usize = 5;
    /// Sub-phase of [`ROUNDTRIP`]: the isomorphism oracle's deep
    /// compare, twice per executed roundtrip.
    pub(crate) const ISO: usize = 6;
    /// Sub-phase of [`APPLY`]: a split's own work — the site's
    /// re-certification and the operator, which certifies both
    /// children.
    pub(crate) const APPLY_SPLIT: usize = 7;
    /// Sub-phase of [`ROUNDTRIP`]: a split and its two-op inverse.
    pub(crate) const ROUNDTRIP_SPLIT: usize = 8;
    /// How many phase slots there are.
    pub(crate) const PHASES: usize = 9;

    /// Row labels, in phase-index order; a leading `of which` marks a
    /// sub-phase of the row above it.
    pub(crate) const NAMES: [&str; PHASES] = [
        "choose_op",
        "apply",
        "roundtrip",
        "validate (explicit)",
        "ledger.check",
        "teardown",
        "of which iso (canonical_form)",
        "of which split (apply arm)",
        "of which split (roundtrip arm)",
    ];

    /// One replay's worth of readings.
    #[derive(Clone, Copy, Default)]
    pub(crate) struct Readings {
        /// Wall clock per phase — this box only.
        pub(crate) elapsed: [Duration; PHASES],
        /// How many times each phase ran — exact.
        pub(crate) calls: [u64; PHASES],
        /// Generator steps replayed — exact.
        pub(crate) steps: u64,
        /// Σ over steps of the body's entity count at step entry:
        /// what the O(body) work of a step is proportional to. Exact,
        /// and only collected when [`reset`] asks for it.
        pub(crate) entities_at_step: u64,
        /// Tier-1 validations run, from every caller — the operators'
        /// debug postconditions included. Exact.
        pub(crate) validations: u64,
        /// Σ over those validations of the body's entity count.
        /// Exact, and only collected when [`reset`] asks for it.
        pub(crate) entities_validated: u64,
        /// Σ, Σ of squares and Σ of cubes of the HALF-EDGE count at
        /// each isomorphism-oracle call. The oracle labels a shell by
        /// minimising a component encoding over every unlabeled dart
        /// as a root, so its cost is a polynomial in that count and
        /// not in the entity total — these three moments are what say
        /// which polynomial, without a clock. Exact, and only
        /// collected when [`reset`] asks for them.
        pub(crate) iso_darts: [u64; 3],
        /// Whether the O(body) counters above are being collected.
        pub(crate) walk_arenas: bool,
    }

    thread_local! {
        static READINGS: RefCell<Readings> = const { RefCell::new(Readings {
            elapsed: [Duration::ZERO; PHASES],
            calls: [0; PHASES],
            steps: 0,
            entities_at_step: 0,
            validations: 0,
            entities_validated: 0,
            iso_darts: [0; 3],
            walk_arenas: false,
        }) };
    }

    /// Clears this thread's readings. `walk_arenas` buys the exact
    /// body-size counters at the price of an O(body) walk per step and
    /// per validation, so a timing pass asks for `false` and a
    /// counting pass for `true`.
    pub(crate) fn reset(walk_arenas: bool) {
        READINGS.with(|r| {
            *r.borrow_mut() = Readings {
                walk_arenas,
                ..Readings::default()
            }
        });
    }

    /// This thread's readings so far.
    pub(crate) fn readings() -> Readings {
        READINGS.with(|r| *r.borrow())
    }

    /// Runs `f`, charging its wall clock and one call to `phase`.
    pub(crate) fn timed<R>(phase: usize, f: impl FnOnce() -> R) -> R {
        let start = Instant::now();
        let out = f();
        let elapsed = start.elapsed();
        READINGS.with(|r| {
            let mut r = r.borrow_mut();
            r.elapsed[phase] += elapsed;
            r.calls[phase] += 1;
        });
        out
    }

    /// Records one generator step against a body of `counts`. The
    /// closure is called only when the arena walk was asked for.
    pub(crate) fn note_step(counts: impl FnOnce() -> ArenaCounts) {
        READINGS.with(|r| {
            let mut r = r.borrow_mut();
            r.steps += 1;
            if r.walk_arenas {
                r.entities_at_step += entities(&counts());
            }
        });
    }

    /// Records one tier-1 validation against a body of `counts`.
    /// Called from [`crate::validate::validate`] itself, so it sees
    /// the operators' debug postconditions as well as the property's
    /// own checks.
    pub(crate) fn note_validation(counts: impl FnOnce() -> ArenaCounts) {
        READINGS.with(|r| {
            let mut r = r.borrow_mut();
            r.validations += 1;
            if r.walk_arenas {
                r.entities_validated += entities(&counts());
            }
        });
    }

    /// Records one isomorphism-oracle call against a body carrying
    /// `darts` half-edges. The closure is called only when the arena
    /// walk was asked for.
    pub(crate) fn note_iso(darts: impl FnOnce() -> u64) {
        READINGS.with(|r| {
            let mut r = r.borrow_mut();
            if r.walk_arenas {
                let n = darts();
                r.iso_darts[0] += n;
                r.iso_darts[1] += n * n;
                r.iso_darts[2] += n * n * n;
            }
        });
    }

    /// The entity count a whole-body walk is proportional to.
    fn entities(counts: &ArenaCounts) -> u64 {
        (counts.solids
            + counts.shells
            + counts.faces
            + counts.loops
            + counts.half_edges
            + counts.edges
            + counts.vertices) as u64
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::fixtures::ops_holed_box;
    use crate::validate::validate;

    /// One proptest decision per step: op-kind roll, site roll, and a
    /// mode roll (every fourth mode value turns the step into a
    /// make/kill roundtrip instead of a plain op).
    type Decision = (u32, u32, u32);

    /// What one decision vector did to property (c): how many steps the
    /// mode roll SELECTED for a roundtrip, how many of those executed,
    /// and how many were skipped as documented irreversible-by-one-op
    /// kills. `skippable` counts the selections that could legally skip
    /// at all — see [`RoundtripTally::skippable`].
    #[derive(Clone, Copy, Default)]
    struct RoundtripTally {
        selected: usize,
        executed: usize,
        skipped: usize,
        /// Selections on `Kev`/`Kef`. The two documented irreversible
        /// subcases both live in those two arms of [`roundtrip`], so a
        /// selection on any other choice MUST execute — which is what
        /// bounds the skip count against the run rather than against a
        /// measured constant.
        skippable: usize,
    }

    impl RoundtripTally {
        fn add(&mut self, other: Self) {
            self.selected += other.selected;
            self.executed += other.executed;
            self.skipped += other.skipped;
            self.skippable += other.skippable;
        }
    }

    /// Runs properties (a)–(d) over one decision vector and returns what
    /// it did to property (c).
    fn run_properties(decisions: &[Decision]) -> Result<RoundtripTally, TestCaseError> {
        let mut body = Body::<f64>::new();
        let mut ledger = Ledger::default();
        let mut counter = 0_u32;
        let mut tally = RoundtripTally::default();
        for &(d1, d2, d3) in decisions {
            meter::note_step(|| body.arena_counts());
            let Some(choice) = meter::timed(meter::CHOOSE, || choose_op(&body, d1, d2)) else {
                return Err(TestCaseError::fail("no applicable op (kernel bug)"));
            };
            if d3 % 4 == 0 {
                // Property (c): op ∘ exact inverse nets nothing.
                tally.selected += 1;
                if matches!(choice, OpChoice::Kev(_) | OpChoice::Kef(_)) {
                    tally.skippable += 1;
                }
                let outcome = meter::timed(meter::ROUNDTRIP, || {
                    roundtrip(&mut body, choice, &mut counter)
                });
                if outcome == RoundtripOutcome::Done {
                    tally.executed += 1;
                } else {
                    // Both documented irreversible subcases sit in
                    // `roundtrip`'s `Kev`/`Kef` arms. A skip anywhere
                    // else is property (c) quietly ceasing to run, not
                    // a case the design excuses.
                    prop_assert!(
                        matches!(choice, OpChoice::Kev(_) | OpChoice::Kef(_)),
                        "roundtrip skipped {:?}, which has no documented \
                         irreversible-by-one-op subcase",
                        choice
                    );
                    tally.skipped += 1;
                }
                // The ledger is unchanged by a balanced pair.
            } else {
                meter::timed(meter::APPLY, || apply(&mut body, choice, &mut counter));
                ledger.apply(choice.ep_vector());
            }
            // Property (a): tier-1 validity after every op. (The debug
            // postconditions inside each op already asserted this along
            // the way, including mid-roundtrip; this is the explicit
            // end-of-step check.)
            let verdict = meter::timed(meter::VALIDATE, || validate(&body));
            prop_assert_eq!(verdict, Ok(()), "after {:?}", choice);
            // Property (b): the E–P ledger matches the derived counts
            // at every step.
            if let Err(msg) = meter::timed(meter::LEDGER, || ledger.check(&body)) {
                return Err(TestCaseError::fail(format!("after {choice:?}: {msg}")));
            }
        }
        // Property (d): everything built can be killed back to nothing;
        // arenas AND provenance maps end empty (asserted inside).
        meter::timed(meter::TEARDOWN, || teardown(&mut body));
        Ok(tally)
    }

    /// Properties (a)–(d) over random valid op sequences (module
    /// docs): tier-1 validity after every op, the E–P ledger at
    /// every step, make/kill roundtrips at random points, and full
    /// teardown to empty arenas + empty provenance maps.
    ///
    /// How much of property (c) ran is checked, and the check is
    /// per-step: the two documented irreversible-by-one-op subcases
    /// live in [`roundtrip`]'s `Kev`/`Kef` arms only, so a selection on
    /// any other choice must execute, and `run_properties` asserts
    /// exactly that as each step happens. **That per-step assertion is
    /// the whole of the bar.**
    ///
    /// The totals below are two different things, and the difference
    /// matters more than either:
    ///
    /// - `executed > 0` is the only one that can fail on its own, and
    ///   it is a COLLAPSE FLOOR, not a bar. It is not implied by the
    ///   design — a run whose every selection landed on an irreversible
    ///   site would legally execute none — but such a run tested
    ///   nothing and should say so rather than pass green.
    /// - `executed + skipped == selected` and `skipped <= skippable`
    ///   are BOOKKEEPING IDENTITIES. The first follows from how the
    ///   tally is accumulated, the second from the per-step assertion
    ///   that has already run. Neither can independently go red; they
    ///   are here to state the shape of the tally for a reader, and
    ///   nothing more.
    ///
    /// No numeric threshold is asserted, because there is no number to
    /// assert: proptest seeds its RNG from entropy, so every run draws a
    /// fresh sample. Four consecutive runs gave 339/335/4/47,
    /// 331/325/6/43, 333/328/5/51 and 351/345/6/47 for
    /// selected/executed/skipped/skippable — a threshold would have
    /// pinned that spread, not a property.
    #[test]
    fn random_op_sequences_hold_all_properties() {
        let tally = std::cell::Cell::new(RoundtripTally::default());
        proptest!(
            ProptestConfig {
                cases: 48,
                ..ProptestConfig::default()
            },
            |(decisions in proptest::collection::vec(
                (any::<u32>(), any::<u32>(), any::<u32>()),
                1..48,
            ))| {
                let mut total = tally.get();
                total.add(run_properties(&decisions)?);
                tally.set(total);
            }
        );
        let t = tally.get();
        // The only run-level check that can fail on its own.
        assert!(
            t.executed > 0,
            "no make/kill roundtrip executed across the whole run: \
             property (c) went untested",
        );
        // Bookkeeping identities (see the doc): the first is how the
        // tally is accumulated, the second is what the per-step
        // assertion has already guaranteed. Stated, not relied on.
        assert_eq!(
            t.executed + t.skipped,
            t.selected,
            "every selected step either roundtripped or skipped",
        );
        assert!(
            t.skipped <= t.skippable,
            "{} skips against {} selections that could legally skip",
            t.skipped,
            t.skippable,
        );
    }

    /// Issue #60, distilled: the shrunken proptest vector whose final
    /// step is a `kef` make/kill roundtrip on a degenerate self-loop
    /// chain carrying two coordinate-identical detached ring twins.
    /// The kef/mef surgery restores ring ownership exactly (raw keys
    /// verified); the failure was the ISO ORACLE's scan-order tie-break
    /// leaking into the face section's outer/ring pairing (a false
    /// negative between isomorphic bodies) — fixed by the committed-
    /// label references in `dart_attachment`. Fast deterministic
    /// counterpart of the re-enabled `cc dda6d5e0…` regression entry.
    #[test]
    fn issue_60_kef_roundtrip_on_coincident_ring_twins() {
        let decisions: Vec<Decision> = vec![
            (0, 0, 14868277),
            (472887944, 3650618882, 1063284414),
            (3974652754, 53826561, 1194232959),
            (3782466455, 750708255, 4131529245),
            (1488373601, 4187241673, 3209066119),
            (2687912414, 268335096, 2031319549),
            (4224888891, 1920513483, 1450773358),
            (2822490516, 3933833678, 313172872),
            (3514019111, 1100428855, 3964918795),
            (3436239477, 3172141881, 1592581843),
            (3802339228, 1771633292, 3668445379),
            (319016958, 3829296104, 758000795),
            (325828484, 2991645150, 1544977337),
            (488374670, 2078562704, 1362431087),
            (2335041233, 1578878958, 801679061),
            (3357132999, 2384639771, 1140201813),
            (1045913028, 1493252069, 1090158457),
            (2343923927, 76525332, 740650425),
            (2655057477, 2335352993, 3173262319),
            (1340868668, 3579545898, 3421628349),
            (2695053902, 1905818496, 3424962875),
            (3658407461, 1803296621, 3777469705),
            (1666265687, 2248595257, 94376607),
            (2980353478, 2061075975, 2860288224),
        ];
        // The vector's final step IS a kef roundtrip, so this run pins
        // that the roundtrip machinery executes rather than skipping.
        assert!(run_properties(&decisions).unwrap().executed > 0);
    }

    #[test]
    fn teardown_handles_the_genus_one_acceptance_body() {
        // Deterministic teardown of the holed box: genus, rings, and 24
        // edges all unwound to nothing.
        let t = ops_holed_box();
        let mut body = t.body;
        teardown(&mut body);
    }

    #[test]
    fn generator_is_deterministic_for_equal_decisions() {
        // Same decision stream ⇒ same op sequence ⇒ deep-identical
        // bodies (the D9 replay story, through the generator).
        let decisions: Vec<Decision> = (0..24_u32)
            .map(|i| (i.wrapping_mul(2_654_435_761), i.wrapping_mul(40_503), i))
            .collect();
        let build = || {
            let mut body = Body::<f64>::new();
            let mut ledger = Ledger::default();
            let mut counter = 0_u32;
            for &(d1, d2, _) in &decisions {
                let choice = choose_op(&body, d1, d2).expect("an op applies");
                apply(&mut body, choice, &mut counter);
                ledger.apply(choice.ep_vector());
                assert_eq!(ledger.check(&body), Ok(()));
            }
            body
        };
        let a = build();
        let b = build();
        assert_eq!(
            crate::fixtures::deep_snapshot(&a),
            crate::fixtures::deep_snapshot(&b)
        );
    }

    /// The fixed stream set the attribution replays: 400 deterministic
    /// decision streams whose lengths span the same `1..48` the
    /// proptest draws, built by the same LCG idiom as
    /// [`selection_is_pinned_over_a_fixed_stream_set`]. A fixed set is
    /// the whole point — the proptest seeds from entropy, so two of
    /// its runs do not share a workload and their wall clocks are not
    /// comparable with each other, let alone across a change.
    fn fixed_streams() -> Vec<Vec<Decision>> {
        (0..400_u32)
            .map(|seed| {
                let mut x = seed.wrapping_mul(2_654_435_761).wrapping_add(12_345);
                let mut roll = || {
                    x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                    x
                };
                let len = 1 + (roll() % 47) as usize;
                (0..len).map(|_| (roll(), roll(), roll())).collect()
            })
            .collect()
    }

    /// Replays the whole fixed stream set through [`run_properties`]
    /// and returns what the meter saw.
    fn replay(streams: &[Vec<Decision>], walk_arenas: bool) -> meter::Readings {
        meter::reset(walk_arenas);
        for decisions in streams {
            run_properties(decisions).expect("the properties hold over the fixed streams");
        }
        meter::readings()
    }

    /// **Where this lane's time goes, phase by phase, with the
    /// `split_edge` row in the catalog and without it.**
    ///
    /// `#[ignore]` because it is a REPORT, not a gate: it asserts only
    /// that the instrument covered the replay, it costs minutes
    /// single-threaded, and nothing it prints can fail. Run it
    /// deliberately:
    ///
    /// ```text
    /// cargo test -p topo --lib phase_attribution -- --ignored --nocapture
    /// ```
    ///
    /// `CAD_SEQGEN_ATTRIBUTION_REPS` (default 3) sets how many times
    /// each catalog is replayed. The two catalogs are replayed
    /// ALTERNATELY inside one process, which is what makes their
    /// comparison survive a box shared with other work: a load spike
    /// lands on both sides, where two separate runs would charge it to
    /// one.
    ///
    /// **What a reading is worth.** The elapsed columns belong to the
    /// box that produced them and to nothing else
    /// (`memories/perf-measurement-lane.md`); quote them with their
    /// spread and their machine or not at all. The counter columns —
    /// steps, validations, entities walked — are exact and identical
    /// everywhere, and they are what an attribution should rest on:
    /// they say what work was done rather than what it cost here.
    ///
    /// **What it cannot see.** The meter charges a phase's whole
    /// closure, so an operator's own debug postcondition (a full
    /// tier-1 validation, `assert_euler_postcondition`) is charged to
    /// `apply`/`roundtrip`/`teardown` rather than to a row of its own
    /// — the `validations` counter is what separates that, not the
    /// clock. Sub-phases are inside the row above them. And a replay
    /// runs `debug_assertions` on at opt-2, which is the suite's
    /// configuration and not a release one.
    #[test]
    fn phase_attribution_over_a_fixed_stream_set() {
        let reps: usize = std::env::var("CAD_SEQGEN_ATTRIBUTION_REPS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3);
        let streams = fixed_streams();

        // The counting pass first, once per catalog: exact, machine
        // independent, and O(body) per step, so it never shares a
        // process moment with a timing pass.
        let counted_with = replay(&streams, true);
        let counted_without = without_split_edge(|| replay(&streams, true));

        let mut timed_with: Vec<meter::Readings> = Vec::new();
        let mut timed_without: Vec<meter::Readings> = Vec::new();
        for _ in 0..reps {
            timed_with.push(replay(&streams, false));
            timed_without.push(without_split_edge(|| replay(&streams, false)));
        }

        let report = attribution_report(
            reps,
            (&counted_with, &timed_with),
            (&counted_without, &timed_without),
        );
        println!("{report}");
        panic!("TEMPORARY measurement run on hosted CI — the report is the deliverable:\n{report}");

        // The only claims this test makes: the instrument covered the
        // replay it reports on, and both catalogs actually ran.
        assert!(counted_with.steps > 0 && counted_without.steps > 0);
        for readings in [&timed_with[0], &timed_without[0]] {
            let partition: std::time::Duration = readings.elapsed[..=meter::TEARDOWN].iter().sum();
            let inner: std::time::Duration = readings.elapsed[meter::APPLY_SPLIT]
                + readings.elapsed[meter::ROUNDTRIP_SPLIT]
                + readings.elapsed[meter::ISO];
            assert!(
                inner <= partition,
                "sub-phases must sit inside the phases they subdivide",
            );
        }
    }

    /// Formats one attribution: per-phase medians with their spread,
    /// the exact counters beside them, and the delta the `split_edge`
    /// row accounts for.
    fn attribution_report(
        reps: usize,
        with: (&meter::Readings, &[meter::Readings]),
        without: (&meter::Readings, &[meter::Readings]),
    ) -> String {
        use std::fmt::Write as _;

        let median = |runs: &[meter::Readings], phase: usize| -> f64 {
            let mut ms: Vec<f64> = runs
                .iter()
                .map(|r| r.elapsed[phase].as_secs_f64() * 1e3)
                .collect();
            ms.sort_by(f64::total_cmp);
            ms[ms.len() / 2]
        };
        let spread = |runs: &[meter::Readings], phase: usize| -> (f64, f64) {
            let ms: Vec<f64> = runs
                .iter()
                .map(|r| r.elapsed[phase].as_secs_f64() * 1e3)
                .collect();
            (
                ms.iter().copied().fold(f64::INFINITY, f64::min),
                ms.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            )
        };
        let total = |runs: &[meter::Readings]| -> f64 {
            (0..=meter::TEARDOWN).map(|p| median(runs, p)).sum()
        };

        let mut out = String::new();
        let _ = writeln!(
            out,
            "\nseqgen phase attribution — {} streams, {reps} interleaved reps per catalog",
            400,
        );
        let _ = writeln!(
            out,
            "  catalog WITH split_edge: {} steps, {} validations over {} entities",
            with.0.steps, with.0.validations, with.0.entities_validated,
        );
        let _ = writeln!(
            out,
            "  catalog WITHOUT it:      {} steps, {} validations over {} entities",
            without.0.steps, without.0.validations, without.0.entities_validated,
        );
        let _ = writeln!(
            out,
            "  Σ body entities at step entry: {} with, {} without",
            with.0.entities_at_step, without.0.entities_at_step,
        );
        for (power, label) in ["Σn", "Σn²", "Σn³"].iter().enumerate() {
            let (a, b) = (with.0.iso_darts[power], without.0.iso_darts[power]);
            let _ = writeln!(
                out,
                "  {label} over the oracle's darts: {a} with, {b} without — ratio {:.3}",
                a as f64 / b as f64,
            );
        }
        let (tw, tn) = (total(with.1), total(without.1));
        let _ = writeln!(
            out,
            "  replay median (Σ phases): {tw:.0} ms with, {tn:.0} ms without \
             — the row costs {:+.1}%",
            (tw / tn - 1.0) * 100.0,
        );
        let _ = writeln!(
            out,
            "\n{:<32} {:>10} {:>17} {:>8} | {:>10} {:>8} | {:>10}",
            "phase", "with (ms)", "spread", "calls", "w/o (ms)", "calls", "delta ms",
        );
        for phase in 0..meter::PHASES {
            let (a, b) = spread(with.1, phase);
            let _ = writeln!(
                out,
                "{:<32} {:>10.1} {:>17} {:>8} | {:>10.1} {:>8} | {:>+10.1}",
                meter::NAMES[phase],
                median(with.1, phase),
                format!("{a:.1}–{b:.1}"),
                with.1[0].calls[phase],
                median(without.1, phase),
                without.1[0].calls[phase],
                median(with.1, phase) - median(without.1, phase),
            );
        }
        out
    }

    /// **What op a given roll selects, pinned over a fixed stream
    /// set.** 64 deterministic decision streams of 32 steps each,
    /// replayed through [`choose_op`] and [`apply`], fingerprinted by
    /// the sequence of ops chosen. Every candidate filter in the
    /// catalog feeds this, so the pin covers the whole selection path:
    /// the weights, the catalog order, the emptiness tests, and the
    /// enumerators' order.
    ///
    /// **Why a pin and not a property.** Making enumeration lazy — or
    /// reordering a filter, or short-circuiting one — is supposed to
    /// leave the choice for a given roll alone, and nothing else here
    /// can tell: the properties hold for any distribution, and
    /// `generator_is_deterministic_for_equal_decisions` compares a run
    /// against itself, so it stays green while the walk moves under
    /// it. This is the differential the in-process comparison is not.
    ///
    /// **The constant is not a target to preserve.** It is a report of
    /// what this generator selects. If it moves, the question is
    /// whether the new distribution is the one intended: re-pin
    /// deliberately and say in the PR what moved the walk and why —
    /// never adjust a filter to bring the old number back.
    #[test]
    fn selection_is_pinned_over_a_fixed_stream_set() {
        const FINGERPRINT: u64 = 8_352_206_392_020_392_659;
        let mut hash = 0xcbf2_9ce4_8422_2325_u64;
        let mut fold = |bytes: &[u8]| {
            for b in bytes {
                hash ^= u64::from(*b);
                hash = hash.wrapping_mul(0x100_0000_01b3);
            }
        };
        let mut chosen = 0_usize;
        for seed in 0..64_u32 {
            let mut x = seed.wrapping_mul(2_654_435_761).wrapping_add(12_345);
            let mut roll = || {
                x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                x
            };
            let mut body = Body::<f64>::new();
            let mut counter = 0_u32;
            for _ in 0..32 {
                let (d1, d2) = (roll(), roll());
                let Some(choice) = choose_op(&body, d1, d2) else {
                    fold(b"NONE");
                    continue;
                };
                fold(format!("{choice:?}").as_bytes());
                apply(&mut body, choice, &mut counter);
                chosen += 1;
            }
        }
        // A walk that chose nothing would fingerprint stably too.
        assert_eq!(chosen, 64 * 32, "every step should have an applicable op");
        assert_eq!(
            hash, FINGERPRINT,
            "the generator selects a different op sequence than this pin records. \
             That is a change to the DISTRIBUTION, not a test failure to paper over: \
             decide whether the new walk is the one intended, then re-pin here and say \
             in the PR what moved it.",
        );
    }
}
