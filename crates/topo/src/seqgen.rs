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
//!   argument, `crate::euler_ring`), `split_edge` (whose Euler
//!   vector IS `mev`'s, so nothing but a catalog row distinguishes the
//!   randomised sites it reaches — struts, self-loop digons — from the
//!   ones `mev` already covers; `crate::split` calls exactly those
//!   coincidence cases delicate) and `movefac` (the shell partition).
//!   `kfmrh` carries TWO rows for the same reason `split_edge` needs
//!   one of its own: its shell-fusion form's Euler vector is not its
//!   same-shell form's — the shell term carries the surgery instead of
//!   the genus term — so only a catalog row separates the two, and
//!   with the two forms behind one row the fusion had no witness at
//!   all (the site filter asked for one shell). The catalog is not an
//!   inventory of the public mutation surface and does not claim to
//!   be: it is the ops whose sites this walk can enumerate.
//!
//!   **`movefac` earns its row as the enabler, not only for itself.**
//!   Of the doors THIS CATALOG holds it is the only one that puts a
//!   second shell in a solid — `mvfs` mints one shell per solid, and
//!   `mfkrh`'s component split leaves the single shell ENTITY in place
//!   — so without it no walk ever builds the multi-shell solid the
//!   fusion form needs, and the fusion row would be asked on every
//!   step and drawn on none.
//!
//!   **Scoped to the catalog on purpose**, because the crate has other
//!   shell-minting doors and they are not this walk's: the boolean
//!   pipeline's combine step inserts a whole operand's shells into the
//!   destination (`crate::boolean::combine`) and `insert_void`
//!   transplants a cavity's shells into an existing solid
//!   (`crate::boolean::voids`). Neither is an Euler surgery or a
//!   structural mutator over one body, which is what this catalog
//!   enumerates sites for.
//! - [`apply`] — execute a choice (coordinates for the vertex-minting
//!   ops come from a caller-owned counter, so every vertex gets distinct
//!   coordinates and canonical forms are sharp).
//! - [`roundtrip`] — execute a choice AND its exact inverse
//!   (make ∘ kill), or the choice and a derived re-make (kill ∘ make),
//!   asserting canonical-form restoration via [`crate::iso`].
//! - [`teardown`] — drive a body all the way back to empty arenas
//!   through kill-direction ops only (plus the ring-resolving
//!   `mfkrh`/`mekr` moves), the completeness-in-reverse check.
//! - [`Ledger`] — the running Euler–Poincaré tuple `(v, e, f, h, r, s)`
//!   — `s` the SHELL count, eq. 9.2's own term, with the solid arena
//!   length carried beside it — checked against derived arena counts
//!   and eq. 9.2 after every op.
//!
//! # The sites with no re-make (skipped in roundtrips)
//!
//! [`roundtrip`] skips (returns [`RoundtripOutcome::SkippedIrreversible`])
//! exactly the configurations that are valid ops but have **no
//! re-make from the site** (the taxonomy was sharpened by the PR 4
//! review — the mate-alone `kef` kill is re-makeable more often than
//! first claimed). [`OpChoice::may_skip_roundtrip`] is the list of
//! arms this section covers, and the fuzz row asserts a skip lands in
//! one of them.
//!
//! Two of the four are shell bookkeeping rather than kill sites:
//!
//! - `movefac`, always — and the reason is that the re-make is
//!   UNBUILT, not that none exists. The obvious inverse mirrors the
//!   fusion arm's: `kfmrh` fuses the minted shell back and `mfkrh`
//!   re-promotes the face that fusion demoted. What stops it here is
//!   that choosing its `(f1, f2)` is a search — `f2` must be a
//!   ring-free face of the component that moved — and that search has
//!   to run BEFORE the partition, since a `roundtrip` that discovers
//!   mid-way that it cannot finish has already mutated the body. The
//!   pairing is exercised from the fusion's side instead (below), so
//!   nothing about the `movefac`/`kfmrh` pair goes untested; what is
//!   missing is the pair driven from this end.
//!   `work/topo/movefac-row-skips-three-component-shells.md` carries
//!   it.
//! - `kfmrh`'s fusion form, where the three-op re-make
//!   (`mfkrh` re-promotes the demoted ring, `movefac` re-partitions
//!   the complex that promotion disconnects again) does not land back
//!   on the same body: when either side was already more than one
//!   component, `movefac` re-splits into every component it finds
//!   rather than the one shell it was; and when `f2`'s shell is not
//!   its solid's last, the fusion `retain`s it out and the re-make
//!   appends the replacement at the end, moving a shell order the
//!   canonical form compares positionally (`crate::iso`'s honest
//!   limits). Both are read before the kill by [`fusion_remake_shell`].
//!
//! The other two are kill sites proper:
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
//!   is then necessarily a self-loop, tol) AND the surviving singleton loop
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
use geom_core::{Band, Decide, Point3, Sign, Tol};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, ShellKey, SolidKey};
use crate::euler::{MefSite, MevSite};
use crate::euler_ring::MekrSite;
use crate::iso::canonical_form;
use crate::readback::{EulerCounts, euler_counts};

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
    /// `kfmrh`'s **shell-fusion** form: `f1` and `f2` lie in DIFFERENT
    /// shells of ONE solid, so the operator re-homes `f2`'s shell's
    /// surviving faces into `f1`'s shell and kills `f2`'s shell
    /// (`crate::euler_ring`'s `kfmrh`). Its own row because its Euler
    /// vector is not the same-shell form's: the shell term carries the
    /// surgery instead of the genus term (`s −1, h 0` against
    /// `s 0, h +1`), so nothing but a catalog row separates the two.
    KfmrhFuse(FaceKey, FaceKey),
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
    /// The other non-Euler public mutator (`movefac(shell)`): the
    /// shell whose incidence complex has fallen into two components is
    /// partitioned into one shell per component (`crate::movefac`).
    ///
    /// **It is in the catalog because it is the only door that mints a
    /// shell into an existing solid**, and therefore the only way a
    /// walk reaches the multi-shell solids [`OpChoice::KfmrhFuse`]
    /// needs: `mvfs` mints one shell per solid, and `mfkrh`'s
    /// component split leaves the single shell ENTITY in place.
    Movefac(ShellKey),
}

impl OpChoice {
    /// The op's Euler vector (Mäntylä Table 9.1, our per-op docs).
    pub(crate) fn ep_vector(&self) -> EulerVector {
        match self {
            Self::Mvfs => EulerVector {
                v: 1,
                f: 1,
                s: 1,
                solids: 1,
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
            // The fusion form pays for the demoted face out of the
            // SHELL term, not the genus term: eq. 9.2 with Δf = −1,
            // Δr = +1 and Δs = −1 forces Δh = 0 (the connected sum of
            // two components' genera is their sum, so no handle is
            // made). Reading this row's `s` as the solid count — which
            // the fusion leaves alone — is what hid the whole form
            // from property (b).
            Self::KfmrhFuse(..) => EulerVector {
                f: -1,
                r: 1,
                s: -1,
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
                solids: -1,
                ..Default::default()
            },
            // NOT an Euler operator: pure reparenting, zero vector.
            Self::RingMove(..) => EulerVector::default(),
            // NOT an Euler operator either, but not zero: the
            // partition mints a shell, and the ledger's `s` is the
            // shell count. Eq. 9.2 then moves `h` with it — the
            // promotion that disconnected the shell drove the derived
            // genus one BELOW the honest per-component sum (see
            // [`Ledger::check`]), and distributing the components into
            // real shells is what pays it back.
            Self::Movefac(_) => EulerVector {
                h: 1,
                s: 1,
                ..Default::default()
            },
        }
    }

    /// Whether [`roundtrip`] is DOCUMENTED as possibly skipping this
    /// choice — the four arms that hold a site with no re-make
    /// (module docs). A skip on any other choice is property
    /// (c) quietly ceasing to run, so the fuzz row asserts against
    /// this list rather than against a measured constant.
    pub(crate) fn may_skip_roundtrip(&self) -> bool {
        matches!(
            self,
            Self::Kev(_) | Self::Kef(_) | Self::KfmrhFuse(..) | Self::Movefac(_)
        )
    }
}

/// One operator's signed Euler–Poincaré shift `(Δv, Δe, Δf, Δh, Δr, Δs)`,
/// plus the solid count the identity does not read.
///
/// The same components as the running [`Ledger`], carried as a shift
/// rather than a count. Call sites name only the nonzero components
/// and take the rest from the derived zero, so a site reads as the
/// op's actual shift.
///
/// **`s` is the SHELL count** — eq. 9.2's `S`, per Mäntylä ch. 9 — and
/// `solids` is the arena length beside it, which no term of the
/// identity reads. They move together for `mvfs` and `kvfs` and
/// nowhere else, which is exactly why one cannot stand in for the
/// other: `movefac` mints a shell inside a solid and `kfmrh`'s fusion
/// form kills one, both leaving the solid count alone.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct EulerVector {
    pub v: i64,
    pub e: i64,
    pub f: i64,
    pub h: i64,
    pub r: i64,
    pub s: i64,
    pub solids: i64,
}

/// The running Euler–Poincaré ledger `(v, e, f, h, r, s)` — `s` the
/// shell count — with the solid arena length carried beside it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Ledger {
    pub v: i64,
    pub e: i64,
    pub f: i64,
    pub h: i64,
    pub r: i64,
    pub s: i64,
    pub solids: i64,
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
        self.solids += delta.solids;
    }

    /// Checks the ledger against the body: v/e/f/s/solids are arena
    /// counts — **`s` the SHELL count**, eq. 9.2's `S` — r is the
    /// summed ring count, and h must satisfy eq. 9.2
    /// (`v − e + f = 2(s − h) + r`) with the derived genus matching the
    /// ledger's.
    ///
    /// **Why the shell count and not the solid count.** They agree on
    /// every body a `mvfs`-only walk can build, so reading eq. 9.2's
    /// `S` off the solid arena passes for as long as no operator
    /// separates them — and then property (b) is silent about exactly
    /// the two ops that do: `movefac`'s partition and `kfmrh`'s
    /// shell-fusion form both leave the solid count alone. The solid
    /// count is checked too, on its own term, because `mvfs`/`kvfs`
    /// are the only ops that may move it and a ledger that stopped
    /// saying so would stop noticing.
    ///
    /// Derived h is NOT required to be non-negative: `mfkrh` applied to
    /// a ring that never plugged a handle (a kemr-planted floating ring)
    /// legitimately drives it negative — the promotion disconnects the
    /// shell's surface into two components while keeping one shell
    /// entity, so eq. 9.2's h stops meaning "genus of a connected
    /// closed surface" and starts double-counting components. A legal
    /// tier-1 intermediate (like empty loops and struts), and one
    /// `movefac` pays back: distributing the components into real
    /// shells raises `s`, and eq. 9.2 raises the derived `h` with it
    /// ([`OpChoice::Movefac`]'s vector is `h +1, s +1` for that
    /// reason). The
    /// component-aware per-shell form (per component
    /// v − e + f − r = 2(1 − g), g ∈ ℤ≥0; per shell the sum is
    /// 2(c − Σgᵢ)) was ratified at M1 PR 5, corrected by PR 4, and
    /// is implemented as the validator's pass 11
    /// (`crate::validate`); this ledger keeps the per-body eq. 9.2 view
    /// because the generator tracks the operator algebra, not surfaces.
    pub(crate) fn check(&self, body: &Body<f64>) -> Result<(), String> {
        let counts = euler_counts(body);
        let EulerCounts { v, e, f, r, s } = counts;
        let solids = body.solids().count() as i64;
        if (v, e, f, s, solids, r) != (self.v, self.e, self.f, self.s, self.solids, self.r) {
            return Err(format!(
                "ledger mismatch: body (v{v} e{e} f{f} s{s} solids{solids} r{r}) \
                 vs ledger {self:?}"
            ));
        }
        // Eq. 9.2 rearranged: h = s − (v − e + f − r) / 2, with the
        // parity checked before the halving by the door itself.
        let derived_h = counts.genus().map_err(|refusal| refusal.to_string())?;
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

/// Picks one applicable `(op, site)` from the current body, driven by
/// two decision integers (see the module docs). `None` only when
/// NOTHING is applicable, which for this catalog means the body is
/// empty and even `mvfs` was weighted out — in practice never, since
/// `mvfs` keeps weight on empty bodies.
pub(crate) fn choose_op(body: &Body<f64>, d1: u32, d2: u32, tol: Tol) -> Option<OpChoice> {
    let grow = body.half_edges().count() < GROW_CAP;
    let w = |grown: u32, shrunk: u32| if grow { grown } else { shrunk };
    type Enumerate = fn(&Body<f64>, Tol) -> Vec<OpChoice>;
    type Probe = fn(&Body<f64>, Tol) -> bool;
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
    let kinds: [(u32, Enumerate, Option<Probe>); 16] = [
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
        // The fusion form, whose candidates need a multi-shell solid
        // and so exist only after `movefac` has run. Real weight for
        // the same reason `kvfs` has it: the window is narrow and a
        // modest weight would leave the row asked but almost never
        // drawn. It carries a probe because its enumeration is the
        // catalog's only `O(F²)` sweep and the window it needs is shut
        // on most steps.
        (4, kfmrh_fuse_candidates, Some(any_kfmrh_fuse)),
        (w(2, 1), mfkrh_candidates, None),
        // The shell partition — the catalog's only door to a
        // multi-shell solid, and make-direction in the shell arena, so
        // it is weighted out once the body stops growing. Its
        // candidates are the two-component shells `mfkrh` leaves
        // behind, and finding them is a glue walk per shell, so the
        // row answers emptiness through a probe that stops at the
        // first one rather than labelling every shell.
        (w(4, 0), movefac_candidates, Some(any_movefac)),
        // The non-Euler public mutator rides along at modest weight:
        // candidates exist whenever any face carries a ring, which kemr
        // (always-on weight) produces steadily.
        (w(2, 1), ring_move_candidates, None),
        // `split_edge` is make-direction (mev's vector), so it is
        // weighted out once the body stops growing. Candidates exist
        // whenever any edge does, which is nearly always — modest
        // weight keeps it from crowding out the sites only the other
        // make ops reach.
        (w(3, 0), split_edge_candidates, Some(any_split_edge)),
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
                let candidates = enumerate(body, tol);
                if !candidates.is_empty() {
                    rows.push((weight, Rolled::Built(candidates)));
                }
            }
            Some(any) => {
                if any(body, tol) {
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
                Rolled::Deferred(enumerate) => enumerate(body, tol),
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
    Deferred(fn(&Body<f64>, Tol) -> Vec<OpChoice>),
}

/// `mvfs` needs no site — it mints a fresh solid — so its single
/// candidate exists whenever the row carries weight.
fn mvfs_candidates(_body: &Body<f64>, _tol: Tol) -> Vec<OpChoice> {
    vec![OpChoice::Mvfs]
}

fn mev_lone_candidates(body: &Body<f64>, _tol: Tol) -> Vec<OpChoice> {
    empty_loops(body)
        .into_iter()
        .map(OpChoice::MevLone)
        .collect()
}

fn mef_lone_candidates(body: &Body<f64>, _tol: Tol) -> Vec<OpChoice> {
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

fn mev_fan_candidates(body: &Body<f64>, _tol: Tol) -> Vec<OpChoice> {
    let mut out = Vec::new();
    for (he1, _) in body.half_edges() {
        let orbit = body.vertex_orbit(he1).expect("valid body: orbit closes");
        for he2 in orbit {
            out.push(OpChoice::MevFan(he1, he2));
        }
    }
    out
}

fn mef_chords_candidates(body: &Body<f64>, _tol: Tol) -> Vec<OpChoice> {
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

fn kemr_candidates(body: &Body<f64>, _tol: Tol) -> Vec<OpChoice> {
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

fn mekr_candidates(body: &Body<f64>, _tol: Tol) -> Vec<OpChoice> {
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

fn kfmrh_candidates(body: &Body<f64>, _tol: Tol) -> Vec<OpChoice> {
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

/// Every site of `kfmrh`'s shell-fusion form: two faces in DIFFERENT
/// shells of ONE solid, with `f2` ring-free.
///
/// The three conditions are the operator's own plan phase, in its
/// order (`crate::euler_ring`'s `kfmrh`): distinct faces, one solid
/// (`CrossSolid` is the refusal for the other case — combining bodies
/// is the boolean pipeline's combine step, not an Euler surgery), and
/// `f2` ring-free (`FaceHasRings`). Both argument orders are
/// generated: the pair is ordered, `f2` being the face that dies.
fn kfmrh_fuse_candidates(body: &Body<f64>, _tol: Tol) -> Vec<OpChoice> {
    let mut out = Vec::new();
    for (f1, face1) in body.faces() {
        let solid1 = body
            .get_shell(face1.shell)
            .expect("valid body: shell resolves")
            .solid;
        for (f2, face2) in body.faces() {
            if f1 == f2 || face1.shell == face2.shell || !face2.rings.is_empty() {
                continue;
            }
            let solid2 = body
                .get_shell(face2.shell)
                .expect("valid body: shell resolves")
                .solid;
            if solid1 == solid2 {
                out.push(OpChoice::KfmrhFuse(f1, f2));
            }
        }
    }
    out
}

/// Whether [`kfmrh_fuse_candidates`] would return anything, without
/// building it.
///
/// The cheap half is the gate: the form needs two shells in one solid,
/// so a body whose every solid holds one shell — which is every body
/// until `movefac` has run, and most bodies after — answers `false`
/// off the solid list alone and never touches the `O(F²)` sweep. The
/// sweep still runs when that gate passes, because "two shells in one
/// solid" is necessary and NOT sufficient (no face of the other shell
/// need be ring-free) and a probe that says `true` over an empty
/// enumeration would send the roll to a modulo by zero.
fn any_kfmrh_fuse(body: &Body<f64>, tol: Tol) -> bool {
    any_multi_shell_solid(body) && !kfmrh_fuse_candidates(body, tol).is_empty()
}

/// Whether any solid holds more than one shell.
fn any_multi_shell_solid(body: &Body<f64>) -> bool {
    body.solids().any(|(_, solid)| solid.shells.len() > 1)
}

/// Every shell whose incidence complex has fallen into EXACTLY two
/// connected components — the post-`mfkrh` transient `movefac` exists
/// to distribute.
///
/// **Why exactly two and not two-or-more.** A row's Euler vector is a
/// per-variant constant, and `movefac` on a `c`-component shell mints
/// `c − 1` shells; offering only `c == 2` keeps `s +1` constant
/// without carrying a derived count in the choice (the shape
/// [`OpChoice::SplitEdge`] avoids for the same reason). The coverage
/// this costs is stated rather than hidden: **a shell that reaches
/// three components is never partitioned by this walk.** It is a
/// smaller loss than it reads, because the row fires on the
/// two-component shells that a third component would have grown from.
fn movefac_candidates(body: &Body<f64>, _tol: Tol) -> Vec<OpChoice> {
    movefac_sites(body).map(OpChoice::Movefac).collect()
}

/// Whether [`movefac_candidates`] would return anything, without
/// building it. Same iterator, stopped at the first item, so the two
/// cannot disagree about emptiness — which is what [`choose_op`]'s
/// roll needs from this row and all it needs.
///
/// The saving is the tail of the shell scan and the `Vec`: a shell
/// that answers the predicate ends the walk, and a body with no
/// two-component shell — the common case — still pays one
/// [`shell_components`] per shell, which is the price of the
/// predicate itself.
///
/// **The shell-count gate [`any_kfmrh_fuse`] uses does not transfer
/// here, and gating on it would kill the row.** This row's candidates
/// are the two-component shells inside a ONE-shell solid — that is
/// the post-`mfkrh` transient the partition exists to resolve, and it
/// is the state every multi-shell solid is reached THROUGH. A gate of
/// "some solid holds two shells" would answer `false` on exactly the
/// bodies this row is for. No cheaper necessary condition is
/// available either: a second component is what the glue walk
/// detects, and a face count cannot see one.
fn any_movefac(body: &Body<f64>, _tol: Tol) -> bool {
    movefac_sites(body).next().is_some()
}

fn movefac_sites(body: &Body<f64>) -> impl Iterator<Item = ShellKey> + '_ {
    body.shells()
        .filter(move |&(shell, _)| shell_components(body, shell) == 2)
        .map(|(shell, _)| shell)
}

/// The number of connected components of `shell`'s incidence complex,
/// under the relation `movefac` partitions on and the validator's
/// pass 11 enumerates: a face glues all its loops, a cycle loop glues
/// across each edge via `mate`, and an empty-loop face is its own
/// dartless component.
fn shell_components(body: &Body<f64>, shell: ShellKey) -> usize {
    let faces = &body.get_shell(shell).expect("shell resolves").faces;
    let mut seen: slotmap::SecondaryMap<FaceKey, ()> = slotmap::SecondaryMap::new();
    let mut components = 0;
    for &seed in faces {
        if seen.insert(seed, ()).is_some() {
            continue;
        }
        components += 1;
        let mut pending = vec![seed];
        while let Some(face_key) = pending.pop() {
            let face = body.get_face(face_key).expect("face resolves");
            for loop_key in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
                let LoopBoundary::Cycle { first } =
                    body.get_loop(loop_key).expect("loop resolves").boundary
                else {
                    continue; // empty loop: glues only its vertex
                };
                for member in body.loop_cycle(first).expect("valid body: cycle closes") {
                    let mate = body.mate(member).expect("valid body: mate resolves");
                    let mate_loop = body.get_half_edge(mate).expect("half resolves").parent_loop;
                    let neighbor = body.get_loop(mate_loop).expect("loop resolves").face;
                    if seen.insert(neighbor, ()).is_none() {
                        pending.push(neighbor);
                    }
                }
            }
        }
    }
    components
}

fn mfkrh_candidates(body: &Body<f64>, _tol: Tol) -> Vec<OpChoice> {
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
fn ring_move_candidates(body: &Body<f64>, _tol: Tol) -> Vec<OpChoice> {
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
fn split_edge_candidates(body: &Body<f64>, tol: Tol) -> Vec<OpChoice> {
    split_edge_sites(body, tol).collect()
}

/// Whether [`split_edge_candidates`] would return anything, without
/// building it. Same iterator, stopped at the first item: the two
/// cannot disagree about emptiness, which is what [`choose_op`]'s roll
/// needs from this row and all it needs.
fn any_split_edge(body: &Body<f64>, tol: Tol) -> bool {
    split_edge_sites(body, tol).next().is_some()
}

fn split_edge_sites(body: &Body<f64>, tol: Tol) -> impl Iterator<Item = OpChoice> + '_ {
    body.edges()
        .map(|(e, _)| e)
        .filter(move |e| split_site(body, *e, tol).is_some())
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
fn split_site(body: &Body<f64>, edge: EdgeKey, tol: Tol) -> Option<(f64, EdgeCurveSpec<f64>)> {
    let edge_data = body.get_edge(edge)?;
    let hp = edge_data.he_plus;
    let start = body.get_half_edge(hp)?.start;
    let end = body.half_edge_end(hp)?;
    let p0 = *body.get_point(body.get_vertex(start)?.point)?;
    let p1 = *body.get_point(body.get_vertex(end)?.point)?;
    let curve = body.get_curve_geom(edge_data.curve)?.certified()?;
    let band = Band::linear(tol).ok()?;
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
            description: curve.restated_description(),
            carrier: curve.carrier().clone(),
            param_start: t0,
            param_end: t1,
        },
    ))
}

fn kev_candidates(body: &Body<f64>, _tol: Tol) -> Vec<OpChoice> {
    body.half_edges()
        .filter(|&(he, he_data)| body.half_edge_end(he) != Some(he_data.start))
        .map(|(he, _)| OpChoice::Kev(he))
        .collect()
}

fn kef_candidates(body: &Body<f64>, _tol: Tol) -> Vec<OpChoice> {
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

fn kvfs_candidates(body: &Body<f64>, _tol: Tol) -> Vec<OpChoice> {
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
pub(crate) fn apply(body: &mut Body<f64>, choice: OpChoice, counter: &mut u32, tol: Tol) {
    match choice {
        OpChoice::Mvfs => {
            body.mvfs(next_point(counter)).unwrap();
        }
        OpChoice::MevLone(l) => {
            body.mev_line(MevSite::Lone { r#loop: l }, next_point(counter), tol)
                .unwrap();
        }
        OpChoice::MevFan(he1, he2) => {
            body.mev_line(MevSite::Fan { he1, he2 }, next_point(counter), tol)
                .unwrap();
        }
        OpChoice::MefChords(he1, he2) => {
            body.mef_chord(MefSite::Chords { he1, he2 }, tol).unwrap();
        }
        OpChoice::MefLone(l) => {
            body.mef_chord(MefSite::Lone { r#loop: l }, tol).unwrap();
        }
        OpChoice::Kemr(he1, he2) => {
            body.kemr(he1, he2).unwrap();
        }
        OpChoice::Mekr(site) => {
            body.mekr_chord(site, tol).unwrap();
        }
        OpChoice::Kfmrh(f1, f2) | OpChoice::KfmrhFuse(f1, f2) => {
            body.kfmrh(f1, f2).unwrap();
        }
        OpChoice::Mfkrh(ring) => {
            body.mfkrh_plug(ring).unwrap();
        }
        OpChoice::Movefac(shell) => {
            body.movefac(shell).unwrap();
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
        OpChoice::SplitEdge(e) => {
            let (t, _) =
                split_site(body, e, tol).expect("a split candidate has a splittable carrier");
            body.split_edge(e, t, tol).unwrap();
        }
    }
}

/// The shell [`roundtrip`] must re-partition to invert a shell fusion
/// of `f1` and `f2` — `f1`'s shell — or `None` when the site is one
/// the three-op re-make does not land back on (module docs).
///
/// Both conditions are read BEFORE the kill, from the same arena the
/// operator's plan phase reads:
///
/// - **Each side is one component.** `movefac` re-splits into every
///   component it finds, so a multi-component side would come back as
///   several shells rather than the one it was.
/// - **`f2`'s shell is its solid's LAST.** The fusion `retain`s it out
///   of the solid's shell list and the re-make appends the replacement
///   at the end, so anywhere else the shell ORDER changes — which the
///   canonical form compares positionally (`crate::iso`'s honest
///   limits).
fn fusion_remake_shell(body: &Body<f64>, f1: FaceKey, f2: FaceKey) -> Option<ShellKey> {
    let shell1 = body.get_face(f1)?.shell;
    let shell2 = body.get_face(f2)?.shell;
    if shell_components(body, shell1) != 1 || shell_components(body, shell2) != 1 {
        return None;
    }
    let solid = body.get_shell(shell2)?.solid;
    (body.get_solid(solid)?.shells.last() == Some(&shell2)).then_some(shell1)
}

/// What [`roundtrip`] did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RoundtripOutcome {
    /// Op and inverse ran; canonical form was asserted restored.
    Done,
    /// The choice was one of the documented sites with no re-make
    /// (module docs); nothing was executed.
    SkippedIrreversible,
}

/// Executes `choice` followed by its exact inverse (make ∘ kill) or by
/// the derived re-make (kill ∘ make), asserting that the canonical form
/// is restored. The body nets zero change up to isomorphism.
pub(crate) fn roundtrip(
    body: &mut Body<f64>,
    choice: OpChoice,
    counter: &mut u32,
    tol: Tol,
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
                .mev_line(MevSite::Lone { r#loop: l }, next_point(counter), tol)
                .unwrap();
            body.kev(created.he_plus).unwrap();
        }
        OpChoice::MevFan(he1, he2) => {
            let created = body
                .mev_line(MevSite::Fan { he1, he2 }, next_point(counter), tol)
                .unwrap();
            body.kev(created.he_plus).unwrap();
        }
        OpChoice::MefChords(he1, he2) => {
            let created = body.mef_chord(MefSite::Chords { he1, he2 }, tol).unwrap();
            body.kef(created.he_minus).unwrap();
        }
        OpChoice::MefLone(l) => {
            let created = body.mef_chord(MefSite::Lone { r#loop: l }, tol).unwrap();
            body.kef(created.he_minus).unwrap();
        }
        OpChoice::Mekr(site) => {
            let created = body.mekr_chord(site, tol).unwrap();
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
            let (t, spec) =
                split_site(body, e, tol).expect("a split candidate has a splittable carrier");
            let created = body.split_edge(e, t, tol).unwrap();
            body.kev(created.he_minus).unwrap();
            body.set_edge_curve(e, spec, tol).unwrap();
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
            body.mekr_chord(site, tol).unwrap();
        }
        OpChoice::Kfmrh(f1, f2) => {
            let result = body.kfmrh(f1, f2).unwrap();
            body.mfkrh_plug(result.ring).unwrap();
        }
        OpChoice::KfmrhFuse(f1, f2) => {
            // The fusion is two surgeries at once, so its re-make is
            // three ops: `mfkrh` re-promotes the demoted ring, and
            // `movefac` re-mints the shell the fusion killed by
            // re-partitioning the complex that promotion just
            // disconnected again. Exact on the sites the module docs
            // name; the rest report the skip.
            let Some(surviving_shell) = fusion_remake_shell(body, f1, f2) else {
                return RoundtripOutcome::SkippedIrreversible;
            };
            let result = body.kfmrh(f1, f2).unwrap();
            body.mfkrh_plug(result.ring).unwrap();
            body.movefac(surviving_shell).unwrap();
        }
        OpChoice::Movefac(_) => {
            // The re-make here is UNBUILT, not impossible — a one-op
            // bar would be the wrong one to argue against, since the
            // arm above re-makes in three. It would mirror that arm:
            // `kfmrh` fuses the minted shell back, `mfkrh` re-promotes
            // the face the fusion demoted. What is missing is the
            // SITE: `f2` must be a ring-free face of the component
            // that moved, and that search has to succeed before the
            // partition runs, because a skip decided afterwards has
            // already mutated the body. The `movefac`/`kfmrh` pair is
            // exercised from the fusion's side meanwhile; module docs,
            // and `work/topo/movefac-row-skips-three-component-shells.md`.
            return RoundtripOutcome::SkippedIrreversible;
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
            body.mev_line(site, w_coords, tol).unwrap();
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
                body.mef_chord(MefSite::Chords { he1: b, he2: b }, tol)
                    .unwrap();
            } else {
                body.kef(he).unwrap();
                let site = if b == he && d == mate {
                    MefSite::Lone { r#loop: l2 } // self-loop pair kill
                } else if b == he {
                    MefSite::Chords { he1: d, he2: d } // circular-face kill
                } else {
                    MefSite::Chords { he1: b, he2: d } // general splice
                };
                body.mef_chord(site, tol).unwrap();
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
pub(crate) fn teardown(body: &mut Body<f64>, tol: Tol) {
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
        if let Some(OpChoice::Kef(he)) = kef_candidates(body, tol).first().copied() {
            body.kef(he).unwrap();
            continue;
        }
        if let Some(OpChoice::Kev(he)) = kev_candidates(body, tol).first().copied() {
            body.kev(he).unwrap();
            continue;
        }
        if let Some(OpChoice::Kemr(he1, he2)) = kemr_candidates(body, tol).first().copied() {
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
            let created = body.mekr_chord(site, tol).unwrap();
            body.kev(created.he_plus).unwrap();
            continue;
        }
        // Extra empty-outer faces (mfkrh leftovers): fold into a
        // sibling face as an empty ring.
        if let Some((f1, f2)) = first_empty_outer_extra_face(body) {
            body.kfmrh(f1, f2).unwrap();
            continue;
        }
        if let Some(OpChoice::Kvfs(solid)) = kvfs_candidates(body, tol).first().copied() {
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

/// An `(f1, f2)` pair where `f2` is a ring-free empty-outer face and
/// `f1` is another face of the same SOLID — `kfmrh` folds it away,
/// through the same-shell form when the sibling shares `f2`'s shell
/// and through the shell-fusion form when it does not.
///
/// **The cross-shell reach is what lets teardown finish.** `kvfs`
/// retires a solid only when it is exactly skeletal — one shell, one
/// face — so a solid that `movefac` partitioned reaches the end of
/// the kill direction as several single-face shells with no same-shell
/// sibling between them, and a same-shell-only search sticks there.
/// Fusion is the step that folds one shell into the next; the demoted
/// ring is then an empty ring, which the `mekr` + `kev` compound above
/// absorbs.
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
        let solid2 = body.get_shell(face2.shell).expect("shell resolves").solid;
        // Same shell first, so a body with one shell per solid takes
        // exactly the step it always took.
        let sibling = body
            .faces()
            .find(|&(f1, face1)| f1 != f2 && face1.shell == face2.shell)
            .or_else(|| {
                body.faces().find(|&(f1, face1)| {
                    f1 != f2 && body.get_shell(face1.shell).expect("shell resolves").solid == solid2
                })
            });
        if let Some((f1, _)) = sibling {
            return Some((f1, f2));
        }
    }
    None
}

#[cfg(test)]
mod random_op_sequences;

#[cfg(test)]
mod tests {
    use geom_core::Tol;
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
    /// and how many were skipped as documented sites with no
    /// re-make. `skippable` counts the selections that could legally skip
    /// at all — see [`RoundtripTally::skippable`].
    #[derive(Clone, Copy, Default)]
    pub(super) struct RoundtripTally {
        pub(super) selected: usize,
        pub(super) executed: usize,
        pub(super) skipped: usize,
        /// Selections on a choice [`OpChoice::may_skip_roundtrip`]
        /// names. Every documented irreversible subcase lives in one
        /// of those arms of [`roundtrip`], so a selection on any other
        /// choice MUST execute — which is what bounds the skip count
        /// against the run rather than against a measured constant.
        pub(super) skippable: usize,
    }

    impl RoundtripTally {
        pub(super) fn add(&mut self, other: Self) {
            self.selected += other.selected;
            self.executed += other.executed;
            self.skipped += other.skipped;
            self.skippable += other.skippable;
        }
    }

    /// Runs properties (a)–(d) over one decision vector and returns what
    /// it did to property (c).
    pub(super) fn run_properties(decisions: &[Decision]) -> Result<RoundtripTally, TestCaseError> {
        let mut body = Body::<f64>::new();
        let mut ledger = Ledger::default();
        let mut counter = 0_u32;
        let mut tally = RoundtripTally::default();
        for &(d1, d2, d3) in decisions {
            let Some(choice) = choose_op(&body, d1, d2, Tol::witness()) else {
                return Err(TestCaseError::fail("no applicable op (kernel bug)"));
            };
            if d3 % 4 == 0 {
                // Property (c): op ∘ exact inverse nets nothing.
                tally.selected += 1;
                if choice.may_skip_roundtrip() {
                    tally.skippable += 1;
                }
                if roundtrip(&mut body, choice, &mut counter, Tol::witness())
                    == RoundtripOutcome::Done
                {
                    tally.executed += 1;
                } else {
                    // Every documented no-re-make subcase sits in an
                    // arm [`OpChoice::may_skip_roundtrip`] names. A
                    // skip anywhere else is property (c) quietly
                    // ceasing to run, not a case the design excuses.
                    prop_assert!(
                        choice.may_skip_roundtrip(),
                        "roundtrip skipped {:?}, which has no documented \
                         no-re-make subcase",
                        choice
                    );
                    tally.skipped += 1;
                }
                // The ledger is unchanged by a balanced pair.
            } else {
                apply(&mut body, choice, &mut counter, Tol::witness());
                ledger.apply(choice.ep_vector());
            }
            // Property (a): tier-1 validity after every op. (The debug
            // postconditions inside each op already asserted this along
            // the way, including mid-roundtrip; this is the explicit
            // end-of-step check.)
            prop_assert_eq!(validate(&body), Ok(()), "after {:?}", choice);
            // Property (b): the E–P ledger matches the derived counts
            // at every step.
            if let Err(msg) = ledger.check(&body) {
                return Err(TestCaseError::fail(format!("after {choice:?}: {msg}")));
            }
        }
        // Property (d): everything built can be killed back to nothing;
        // arenas AND provenance maps end empty (asserted inside).
        teardown(&mut body, Tol::witness());
        Ok(tally)
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
        let t = ops_holed_box(Tol::witness());
        let mut body = t.body;
        teardown(&mut body, Tol::witness());
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
                let choice = choose_op(&body, d1, d2, Tol::witness()).expect("an op applies");
                apply(&mut body, choice, &mut counter, Tol::witness());
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
        const FINGERPRINT: u64 = 1_017_092_501_598_614_451;
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
                let Some(choice) = choose_op(&body, d1, d2, Tol::witness()) else {
                    fold(b"NONE");
                    continue;
                };
                fold(format!("{choice:?}").as_bytes());
                apply(&mut body, choice, &mut counter, Tol::witness());
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
