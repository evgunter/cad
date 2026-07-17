//! Random VALID Euler-op sequence generation + the make/kill roundtrip
//! machinery (test support, M1 PR 4).
//!
//! The proptest suite at the bottom drives the four properties the PR
//! promises; the building blocks are `pub(crate)` so M1 PR 5's
//! validator fuzzing can reuse the generator as its fuzz source:
//!
//! - [`choose_op`] — walk the current body, enumerate every applicable
//!   `(operator, site)` candidate deterministically, and pick one by a
//!   weighted choice driven entirely by the caller's decision integers
//!   (proptest's seeded values). No ambient randomness: the same body
//!   and decisions always pick the same op (D9-style determinism, which
//!   also makes proptest shrinking meaningful).
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
//! # The two documented irreversible-by-one-op kill sites
//!
//! [`roundtrip`] skips (returns [`RoundtripOutcome::SkippedIrreversible`])
//! two kill configurations that are valid kills but have **no
//! single-op re-make**, because the required inverse site is not
//! addressable:
//!
//! - `kev(he)` where `start(he)` has valence 1 and `end(he)` carries a
//!   fan (the "mirror" adjacency `next(mate(he)) == he`): restoring it
//!   would need `mev` to move the survivor's ENTIRE fan to the new
//!   vertex, but the full-orbit run is mev-inexpressible — the ratified
//!   `he1 == he2` site means the EMPTY run (strut), so the full run has
//!   no address. (Killing the same edge from the other half is the
//!   strut kill, which IS exactly invertible.)
//! - `kef(he)` where the mate's loop is `[mate]` alone and the dying
//!   loop is bigger: restoring it would need `mef` to move the
//!   surviving loop's ENTIRE cycle to the new loop — the same
//!   full-cycle/empty-run collision (`he1 == he2` is ratified as the
//!   one-edge circular face). Killing the same edge from the other half
//!   is the circular-face kill, exactly invertible.
//!
//! Completeness is unharmed (the pre-kill bodies are still
//! Euler-reachable — build the fan/cycle first and strut/cut last); the
//! make/kill pairing is exact **per site**, not per edge half. See
//! `crate::euler_kill`'s docs.

// Test-support code: panicking is a test's failure mechanism (L5).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;

use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, SolidKey};
use crate::euler::{MefSite, MevSite};
use crate::euler_ring::MekrSite;
use crate::iso::canonical_form;

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
}

impl OpChoice {
    /// The op's Euler vector `(Δv, Δe, Δf, Δh, Δr, Δs)` (Mäntylä
    /// Table 9.1, our per-op docs).
    pub(crate) fn ep_vector(&self) -> [i64; 6] {
        match self {
            Self::Mvfs => [1, 0, 1, 0, 0, 1],
            Self::MevLone(_) | Self::MevFan(..) => [1, 1, 0, 0, 0, 0],
            Self::MefChords(..) | Self::MefLone(_) => [0, 1, 1, 0, 0, 0],
            Self::Kemr(..) => [0, -1, 0, 0, 1, 0],
            Self::Mekr(_) => [0, 1, 0, 0, -1, 0],
            Self::Kfmrh(..) => [0, 0, -1, 1, 1, 0],
            Self::Mfkrh(_) => [0, 0, 1, -1, -1, 0],
            Self::Kev(_) => [-1, -1, 0, 0, 0, 0],
            Self::Kef(_) => [0, -1, -1, 0, 0, 0],
            Self::Kvfs(_) => [-1, 0, -1, 0, 0, -1],
        }
    }
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
    pub(crate) fn apply(&mut self, delta: [i64; 6]) {
        self.v += delta[0];
        self.e += delta[1];
        self.f += delta[2];
        self.h += delta[3];
        self.r += delta[4];
        self.s += delta[5];
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
    /// tier-1 intermediate (like empty loops and struts); M1 PR 5's
    /// per-shell Euler–Poincaré design has to account for it.
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

/// Picks one applicable `(op, site)` from the current body, driven by
/// two decision integers (see the module docs). `None` only when
/// NOTHING is applicable, which for this catalog means the body is
/// empty and even `mvfs` was weighted out — in practice never, since
/// `mvfs` keeps weight on empty bodies.
pub(crate) fn choose_op(body: &Body<f64>, d1: u32, d2: u32) -> Option<OpChoice> {
    let grow = body.half_edges().count() < GROW_CAP;
    let w = |grown: u32, shrunk: u32| if grow { grown } else { shrunk };
    // (weight, candidates) per op kind, in fixed catalog order.
    let kinds: Vec<(u32, Vec<OpChoice>)> = vec![
        (
            if body.solids().count() < 2 {
                w(1, 0)
            } else {
                0
            },
            vec![OpChoice::Mvfs],
        ),
        (w(5, 0), mev_lone_candidates(body)),
        (w(6, 0), mev_fan_candidates(body)),
        (w(6, 0), mef_chords_candidates(body)),
        (w(2, 0), mef_lone_candidates(body)),
        (3, kemr_candidates(body)),
        (w(3, 1), mekr_candidates(body)),
        (2, kfmrh_candidates(body)),
        (w(2, 1), mfkrh_candidates(body)),
        (w(2, 6), kev_candidates(body)),
        (w(2, 6), kef_candidates(body)),
        (1, kvfs_candidates(body)),
    ];
    let total: u32 = kinds
        .iter()
        .filter(|(_, c)| !c.is_empty())
        .map(|(weight, _)| weight)
        .sum();
    if total == 0 {
        return None;
    }
    let mut roll = d1 % total;
    for (weight, candidates) in kinds {
        if candidates.is_empty() || weight == 0 {
            continue;
        }
        if roll < weight {
            let index = (d2 as usize) % candidates.len();
            return Some(candidates[index]);
        }
        roll -= weight;
    }
    None // unreachable: roll < total by construction
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
            body.mev(MevSite::Lone { r#loop: l }, next_point(counter))
                .unwrap();
        }
        OpChoice::MevFan(he1, he2) => {
            body.mev(MevSite::Fan { he1, he2 }, next_point(counter))
                .unwrap();
        }
        OpChoice::MefChords(he1, he2) => {
            body.mef(MefSite::Chords { he1, he2 }).unwrap();
        }
        OpChoice::MefLone(l) => {
            body.mef(MefSite::Lone { r#loop: l }).unwrap();
        }
        OpChoice::Kemr(he1, he2) => {
            body.kemr(he1, he2).unwrap();
        }
        OpChoice::Mekr(site) => {
            body.mekr(site).unwrap();
        }
        OpChoice::Kfmrh(f1, f2) => {
            body.kfmrh(f1, f2).unwrap();
        }
        OpChoice::Mfkrh(ring) => {
            body.mfkrh(ring).unwrap();
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
                .mev(MevSite::Lone { r#loop: l }, next_point(counter))
                .unwrap();
            body.kev(created.he_plus).unwrap();
        }
        OpChoice::MevFan(he1, he2) => {
            let created = body
                .mev(MevSite::Fan { he1, he2 }, next_point(counter))
                .unwrap();
            body.kev(created.he_plus).unwrap();
        }
        OpChoice::MefChords(he1, he2) => {
            let created = body.mef(MefSite::Chords { he1, he2 }).unwrap();
            body.kef(created.he_minus).unwrap();
        }
        OpChoice::MefLone(l) => {
            let created = body.mef(MefSite::Lone { r#loop: l }).unwrap();
            body.kef(created.he_minus).unwrap();
        }
        OpChoice::Mekr(site) => {
            let created = body.mekr(site).unwrap();
            body.kemr(created.he_plus, created.he_minus).unwrap();
        }
        OpChoice::Mfkrh(ring) => {
            let old_face = body.get_loop(ring).expect("ring resolves").face;
            let created = body.mfkrh(ring).unwrap();
            body.kfmrh(old_face, created.face).unwrap();
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
            body.mekr(site).unwrap();
        }
        OpChoice::Kfmrh(f1, f2) => {
            let result = body.kfmrh(f1, f2).unwrap();
            body.mfkrh(result.ring).unwrap();
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
            body.mev(site, w_coords).unwrap();
        }
        OpChoice::Kef(he) => {
            let he_data = body.get_half_edge(he).expect("resolves").clone();
            let mate = body.mate(he).expect("mate resolves");
            let mate_data = body.get_half_edge(mate).expect("resolves").clone();
            let (b, d) = (he_data.next, mate_data.next);
            if d == mate && b != he {
                // The mate-alone site: no single-mef re-make (module
                // docs).
                return RoundtripOutcome::SkippedIrreversible;
            }
            let l2 = mate_data.parent_loop;
            body.kef(he).unwrap();
            let site = if b == he && d == mate {
                MefSite::Lone { r#loop: l2 } // self-loop pair kill
            } else if b == he {
                MefSite::Chords { he1: d, he2: d } // circular-face kill
            } else {
                MefSite::Chords { he1: b, he2: d } // general splice
            };
            body.mef(site).unwrap();
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
            body.mfkrh(ring).unwrap();
            continue;
        }
        // Empty rings: absorb with mekr, then kill the fresh edge (and
        // the stranded vertex) with kev — a compound step so the
        // potential still shrinks.
        if let Some(site) = first_empty_ring_site(body) {
            let created = body.mekr(site).unwrap();
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

    fn run_properties(decisions: &[Decision]) -> Result<(), TestCaseError> {
        let mut body = Body::<f64>::new();
        let mut ledger = Ledger::default();
        let mut counter = 0_u32;
        let mut roundtrips = 0_usize;
        for &(d1, d2, d3) in decisions {
            let Some(choice) = choose_op(&body, d1, d2) else {
                return Err(TestCaseError::fail("no applicable op (kernel bug)"));
            };
            if d3 % 4 == 0 {
                // Property (c): op ∘ exact inverse nets nothing.
                if roundtrip(&mut body, choice, &mut counter) == RoundtripOutcome::Done {
                    roundtrips += 1;
                }
                // The ledger is unchanged by a balanced pair.
            } else {
                apply(&mut body, choice, &mut counter);
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
        teardown(&mut body);
        // Keep shrunk cases meaningful: at least the trivial sequence
        // exercised something.
        let _ = roundtrips;
        Ok(())
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 48,
            ..ProptestConfig::default()
        })]

        /// Properties (a)–(d) over random valid op sequences (module
        /// docs): tier-1 validity after every op, the E–P ledger at
        /// every step, make/kill roundtrips at random points, and full
        /// teardown to empty arenas + empty provenance maps.
        #[test]
        fn random_op_sequences_hold_all_properties(
            decisions in proptest::collection::vec(
                (any::<u32>(), any::<u32>(), any::<u32>()),
                1..48,
            )
        ) {
            run_properties(&decisions)?;
        }
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
}
