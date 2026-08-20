//! Adversarial falsification probes for Track D unit **D18** (PR #736):
//! the two `prev` proofs (`split_edge`'s `prev(he_minus)`, `kef`'s
//! `prev(he)`) and the conversion of [`crate::Body::link_half_edges`]
//! to the D2 addendum's row 4.
//!
//! Written by the correctness-lane reviewer to BREAK the unit, not to
//! describe it, and kept per the standing convention (a reviewer's
//! suite is promoted as-is at the fix pass). The claim under attack is
//! the headline the D2 addendum's taxonomy rests on — *the kernel never
//! panics on any input* — which after this unit survives only because
//! `link_half_edges`' two `unreachable!` arms are not input-reachable.
//!
//! # What each row buys
//!
//! - The **atomicity** rows are deterministic gates: a corruption that
//!   trips ONLY the new check must produce the typed `StaleKey` naming
//!   that key, with the body byte-identical after. They fail if either
//!   check ever moves below a write.
//! - The **coincidence** row enumerates the shapes in which
//!   `split_edge`'s plan-phase `hm_prev` can differ from the value the
//!   second splice actually reads — generic, strut
//!   (`next(he_plus) == he_minus`, the case the re-read exists for),
//!   full-period self-loop edge, two-half-edge loop. An ENUMERATION,
//!   not a fuzzer: the shapes are the whole content and nothing is
//!   sampled.
//! - The **torn-body sweep** drives every operator that reaches
//!   `link_half_edges` over every key of a randomly torn body. A
//!   counterexample search: varying seed, counts on `CAD_FUZZ_EFFORT`,
//!   monotone in the safe direction.
//!
//! # Why the sweep is release-only
//!
//! A torn body is ENTITLED to `Ok(garbage)` and to a typed error, and
//! in a debug build it is also entitled to the postcondition assert —
//! `release_corruption.rs` records that the "one legitimate panic site"
//! sentence holds only under the tier-1-valid input assumption. So in a
//! debug build a panic is ambiguous and has to be classified by its
//! message, which means a process-global panic hook. That works once
//! (`release_corruption::debug_postcondition_fires_on_corrupt_input`
//! does it) but not tens of thousands of times: under threaded
//! `cargo test` the window becomes the whole run, and the hook races
//! both this file's own rows and that one — MEASURED, both directions,
//! while this file was being written.
//!
//! In `--release` the postcondition is compiled out, so **any** panic
//! from these calls is a row-4 arm firing and no classification is
//! needed. The rows therefore carry `cfg(not(debug_assertions))`, catch
//! nothing, and let a real panic fail the test with its own message and
//! backtrace.
//!
//! **PROMOTION NOTE.** The only release-profile test invocation the
//! kernel workspace has is `ci.yml`'s `corrupt input (release profile)`
//! job, which selects rows BY NAME. Promoting this file means adding
//! `review_d18` to that job's filter list and a `grep -q` line for
//! [`torn_bodies_never_reach_a_row_four_unreachable`], or the sweep
//! ships without ever running. The profile-independent rows above ride
//! the standard matrix and need nothing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;

use crate::body::Body;
use crate::entity::{EntityId, HalfEdgeKey};
use crate::euler::{EulerOpError, MefSite, MevSite};
use crate::fixtures::{deep_snapshot, ops_cube};

fn p(x: f64) -> Point3<f64> {
    Point3::new(x, 0.0, 0.0)
}

/// A half-edge key that is guaranteed dead in `body`'s own arena —
/// minted and then killed here, so it is a RECYCLED slot rather than
/// the null key. `HalfEdgeKey::default()` is the easy dangling key and
/// the existing atomicity rows use it; a recycled slot is the harder
/// one, because it differs from a live key only in its slotmap version.
///
/// The mint/kill pair runs on its OWN skeletal solid (an `mvfs` in the
/// same arena) rather than on the body under test: a `mev`/`kev` round
/// trip anchored on a shared vertex rebinds `start` anchors and would
/// perturb the fixture this helper is meant to leave alone.
fn recycled_dead_half_edge(body: &mut Body<f64>) -> HalfEdgeKey {
    let seed = body.mvfs(p(50.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(51.0),
        )
        .unwrap();
    let dead = seg.he_plus;
    body.kev(seg.he_minus).unwrap();
    assert!(
        body.get_half_edge(dead).is_none(),
        "fixture: the recycled key must be dead"
    );
    dead
}

// =====================================================================
// C3 — atomicity of the two new plan-phase checks.
// =====================================================================

/// `split_edge`'s new `prev(he_minus)` check: a corruption that trips
/// ONLY it yields the typed `StaleKey` naming that key, and the body is
/// byte-identical afterwards. Both the null key and a recycled slot are
/// planted, and the control (the same call on the undamaged body)
/// succeeds, so the row cannot pass by refusing everything.
#[test]
fn split_edge_dangling_prev_of_he_minus_is_typed_and_atomic() {
    let cube = ops_cube();
    let mut base = cube.body;
    let edge = cube.mevs[0].edge;
    let recycled = recycled_dead_half_edge(&mut base);
    for dead in [HalfEdgeKey::default(), recycled] {
        let mut body = base.clone();
        let hm = body.get_edge(edge).unwrap().he_minus;
        let hp = body.get_edge(edge).unwrap().he_plus;
        // The mirror link must stay live, so the refusal is attributable
        // to the NEW half of the check and not to the one #720 left.
        assert!(
            body.get_half_edge(body.get_half_edge(hp).unwrap().next)
                .is_some(),
            "fixture: next(he_plus) must stay live"
        );
        body.get_half_edge_mut(hm).unwrap().prev = dead;
        let before = deep_snapshot(&body);
        let err = body.split_edge(edge, 0.5).unwrap_err();
        assert_eq!(
            err,
            EulerOpError::StaleKey {
                key: EntityId::HalfEdge(dead)
            },
            "the new check must name the key it refused"
        );
        assert_eq!(
            deep_snapshot(&body),
            before,
            "split_edge's contract sentence: the body is untouched on Err"
        );
    }
    // Control: undamaged, the same call succeeds.
    let mut body = base;
    let control = body.split_edge(edge, 0.5);
    assert!(control.is_ok(), "control split: {control:?}");
}

/// `kef`'s new `prev(he)` check, same shape. The dying loop's cycle
/// walk steps `next`, so tearing `prev` alone leaves every earlier
/// precondition — the walk included — passing. That is exactly the gap
/// D18 closes, and the row asserts the walk still closes both before
/// and after the tear so a future change that made the walk read `prev`
/// could not silently turn this into a walk test.
#[test]
fn kef_dangling_prev_of_he_is_typed_and_atomic() {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(1.0),
        )
        .unwrap();
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_minus,
                he2: seg.he_minus,
            },
            p(2.0),
        )
        .unwrap();
    let split = body
        .mef_chord(MefSite::Chords {
            he1: seg.he_plus,
            he2: strut.he_minus,
        })
        .unwrap();
    let recycled = recycled_dead_half_edge(&mut body);
    let base = body;

    let mut control = base.clone();
    assert!(
        control.kef(split.he_minus).is_ok(),
        "control kef must succeed on the undamaged body"
    );
    for dead in [HalfEdgeKey::default(), recycled] {
        let mut body = base.clone();
        let he = split.he_minus;
        assert!(
            body.loop_cycle(he).is_some(),
            "fixture: the cycle walk must close before the tear"
        );
        body.get_half_edge_mut(he).unwrap().prev = dead;
        assert!(
            body.loop_cycle(he).is_some(),
            "fixture: tearing prev must not disturb the next-walk"
        );
        let before = deep_snapshot(&body);
        let err = body.kef(he).unwrap_err();
        assert_eq!(
            err,
            EulerOpError::StaleKey {
                key: EntityId::HalfEdge(dead)
            },
            "kef's new check must name the key it refused"
        );
        assert_eq!(
            deep_snapshot(&body),
            before,
            "kef's contract sentence: the body is untouched on Err"
        );
    }
}

// =====================================================================
// C2 — the coincidence shapes in which the plan-phase `hm_prev` can
//      differ from the value splice 2 actually reads.
// =====================================================================

/// Every such shape, each with `prev(he_minus)` torn: the refusal must
/// be typed and atomic in ALL of them, never a panic and never a
/// garbage `Ok`. Each shape carries a control split on the undamaged
/// body, so a shape that silently stopped being buildable cannot pass
/// vacuously.
#[test]
fn split_edge_new_check_covers_every_coincidence_shape() {
    use core::f64::consts::PI;

    use crate::entity::EdgeKey;

    type Shape = (&'static str, f64, Box<dyn Fn() -> (Body<f64>, EdgeKey)>);
    let shapes: Vec<Shape> = vec![
        (
            "generic cube edge",
            0.5,
            Box::new(|| {
                let cube = ops_cube();
                let edge = cube.mevs[0].edge;
                (cube.body, edge)
            }),
        ),
        (
            "strut: next(he_plus) == he_minus",
            0.5,
            Box::new(|| {
                let cube = ops_cube();
                let mut body = cube.body;
                let anchor = cube.mevs[0].he_plus;
                let strut = body
                    .mev_line(
                        MevSite::Fan {
                            he1: anchor,
                            he2: anchor,
                        },
                        p(2.0),
                    )
                    .unwrap();
                (body, strut.edge)
            }),
        ),
        (
            "full-period self-loop edge (a one-half-edge loop each side)",
            PI,
            Box::new(|| {
                let mut body = Body::<f64>::new();
                let seed = body.mvfs(p(0.0)).unwrap();
                let seg = body
                    .mev_line(
                        MevSite::Lone {
                            r#loop: seed.r#loop,
                        },
                        p(1.0),
                    )
                    .unwrap();
                let circ = body
                    .mef_chord(MefSite::Chords {
                        he1: seg.he_minus,
                        he2: seg.he_minus,
                    })
                    .unwrap();
                (body, circ.edge)
            }),
        ),
        (
            "two-half-edge loop (segment: both halves in one loop)",
            0.5,
            Box::new(|| {
                let mut body = Body::<f64>::new();
                let seed = body.mvfs(p(0.0)).unwrap();
                let seg = body
                    .mev_line(
                        MevSite::Lone {
                            r#loop: seed.r#loop,
                        },
                        p(1.0),
                    )
                    .unwrap();
                (body, seg.edge)
            }),
        ),
    ];
    for (label, t, build) in shapes {
        let (mut control, edge) = build();
        assert!(
            control.split_edge(edge, t).is_ok(),
            "{label}: control split must succeed"
        );
        let (mut body, edge) = build();
        let hm = body.get_edge(edge).unwrap().he_minus;
        let dead = HalfEdgeKey::default();
        body.get_half_edge_mut(hm).unwrap().prev = dead;
        let before = deep_snapshot(&body);
        // Not caught: a panic here IS the finding and should carry its
        // own message and backtrace.
        let err = body.split_edge(edge, t).unwrap_err();
        assert_eq!(
            err,
            EulerOpError::StaleKey {
                key: EntityId::HalfEdge(dead)
            },
            "{label}: typed refusal"
        );
        assert_eq!(deep_snapshot(&body), before, "{label}: body changed on Err");
    }
}

// =====================================================================
// C1 — the headline: neither new `unreachable!` is input-reachable.
// =====================================================================

/// Pins the SHAPE the sweep below assumes, which is the one way that
/// sweep could go quietly vacuous: if `link_half_edges` ever went back
/// to discarding a failed lookup, a torn-body hammer would report green
/// while announcing nothing. Source text, because there is no runtime
/// way to ask a function whether it announces.
#[test]
fn link_half_edges_still_announces_rather_than_discards() {
    let source = std::fs::read_to_string(crate::fixtures::src_root().join("euler.rs"))
        .expect("euler.rs must be readable");
    let at = source
        .find("pub(crate) fn link_half_edges(")
        .expect("link_half_edges must still exist");
    let body = &source[at..];
    let end = body.find("\n    }\n").expect("a function body");
    let body = &body[..end];
    assert_eq!(
        body.matches("unreachable!").count(),
        2,
        "link_half_edges must announce on BOTH keys; the torn-body sweep \
         asserts nothing if it discards again:\n{body}"
    );
    assert!(
        !body.contains("if let Some"),
        "the discard idiom is back in link_half_edges:\n{body}"
    );
}

/// The kinds of tier-1 corruption the sweep plants. Each is a shape
/// #720's review named as a way a key can be wrong: DANGLING (the
/// lookup fails — the only shape that can reach a row-4 arm) and
/// LIVE-BUT-WRONG (the lookup succeeds against an unrelated entity —
/// the shape a slotmap key laundered across arenas actually takes).
#[cfg(not(debug_assertions))]
#[derive(Clone, Copy, Debug)]
enum Tear {
    NextDangling,
    PrevDangling,
    NextForeign,
    PrevForeign,
    ParentLoopForeign,
    EdgeBijection,
    StartForeign,
    LoopAnchorForeign,
    EmanatingDangling,
}

#[cfg(not(debug_assertions))]
const TEARS: [Tear; 9] = [
    Tear::NextDangling,
    Tear::PrevDangling,
    Tear::NextForeign,
    Tear::PrevForeign,
    Tear::ParentLoopForeign,
    Tear::EdgeBijection,
    Tear::StartForeign,
    Tear::LoopAnchorForeign,
    Tear::EmanatingDangling,
];

#[cfg(not(debug_assertions))]
fn plant(body: &mut Body<f64>, tear: Tear, rng: &mut test_utils::fuzz::Rng, dead: HalfEdgeKey) {
    use crate::entity::{LoopKey, VertexKey};
    let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(k, _)| k).collect();
    let loops: Vec<LoopKey> = body.loops().map(|(k, _)| k).collect();
    let vertices: Vec<VertexKey> = body.vertices().map(|(k, _)| k).collect();
    if halves.is_empty() {
        return;
    }
    let mut pick = |n: usize| (rng.next_u64() as usize) % n;
    let he = halves[pick(halves.len())];
    let other = halves[pick(halves.len())];
    match tear {
        Tear::NextDangling => body.get_half_edge_mut(he).unwrap().next = dead,
        Tear::PrevDangling => body.get_half_edge_mut(he).unwrap().prev = dead,
        Tear::NextForeign => body.get_half_edge_mut(he).unwrap().next = other,
        Tear::PrevForeign => body.get_half_edge_mut(he).unwrap().prev = other,
        Tear::ParentLoopForeign => {
            if !loops.is_empty() {
                let l = loops[pick(loops.len())];
                body.get_half_edge_mut(he).unwrap().parent_loop = l;
            }
        }
        Tear::EdgeBijection => {
            let edge = body.get_half_edge(he).unwrap().edge;
            if let Some(e) = body.get_edge_mut(edge) {
                e.he_plus = he;
                e.he_minus = other;
            }
        }
        Tear::StartForeign => {
            if !vertices.is_empty() {
                let v = vertices[pick(vertices.len())];
                body.get_half_edge_mut(he).unwrap().start = v;
            }
        }
        Tear::LoopAnchorForeign => {
            if !loops.is_empty() {
                let l = loops[pick(loops.len())];
                if let Some(loop_data) = body.get_loop_mut(l) {
                    loop_data.boundary = crate::LoopBoundary::Cycle { first: other };
                }
            }
        }
        Tear::EmanatingDangling => {
            if !vertices.is_empty() {
                let v = vertices[pick(vertices.len())];
                if let Some(vertex) = body.get_vertex_mut(v) {
                    vertex.emanating = Some(dead);
                }
            }
        }
    }
}

/// The operators this pass drives, by name — the census categories, and
/// the list the anti-vacuity floor counts over.
#[cfg(not(debug_assertions))]
const OPS: [&str; 7] = [
    "kef",
    "kev",
    "kemr",
    "mev_line",
    "mef_chord",
    "split_edge",
    "mfkrh_plug",
];

#[cfg(not(debug_assertions))]
const CALLS: &str = "operator calls";

/// Drives every operator that reaches [`crate::Body::link_half_edges`]
/// over every key of a torn body, and returns the pass as the tree's
/// anti-vacuity census ([`test_utils::census`]).
///
/// Nothing is caught: in this profile the postcondition is compiled out,
/// so a panic escaping here is a row-4 arm and it should fail the test
/// with its own message.
///
/// **What the census counts, and why it is per operator.** [`CALLS`] is
/// every operator driven at a key; each operator's own category counts
/// the calls that returned `Ok`, i.e. that RAN THEIR MUTATION PHASE,
/// which is the only place a row-4 `unreachable!` can fire. A pass whose
/// calls all died in a plan phase proves nothing about the arms under
/// attack. The count is kept per operator because a total does not
/// distinguish *seven operators exercised* from *one exercised and six
/// refused at the door*: on a destination whose every arena field is
/// nulled, one loop-keyed operator still returns `Ok`, so a floor on the
/// total alone is close to unfalsifiable here.
#[cfg(not(debug_assertions))]
fn hammer(body: &Body<f64>) -> test_utils::census::Census {
    use crate::entity::{EdgeKey, LoopKey};
    let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(k, _)| k).collect();
    let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    let loops: Vec<LoopKey> = body.loops().map(|(k, _)| k).collect();
    let mut census = test_utils::census::Census::new("review_d18 hammer");
    for op in OPS {
        census.add(op, 0);
    }
    let mut note = |op: &str, ok: bool| {
        census.note(CALLS);
        if ok {
            census.note(op);
        }
    };
    for &he in &halves {
        note("kef", body.clone().kef(he).is_ok());
        note("kev", body.clone().kev(he).is_ok());
        note(
            "mev_line",
            body.clone()
                .mev_line(MevSite::Fan { he1: he, he2: he }, p(41.0))
                .is_ok(),
        );
        note(
            "mef_chord",
            body.clone()
                .mef_chord(MefSite::Chords { he1: he, he2: he })
                .is_ok(),
        );
        for &other in halves.iter().take(4) {
            note("kemr", body.clone().kemr(he, other).is_ok());
            note(
                "mev_line",
                body.clone()
                    .mev_line(
                        MevSite::Fan {
                            he1: he,
                            he2: other,
                        },
                        p(42.0),
                    )
                    .is_ok(),
            );
            note(
                "mef_chord",
                body.clone()
                    .mef_chord(MefSite::Chords {
                        he1: he,
                        he2: other,
                    })
                    .is_ok(),
            );
        }
    }
    for &edge in &edges {
        for t in [0.25, 0.5, 0.75] {
            note("split_edge", body.clone().split_edge(edge, t).is_ok());
        }
    }
    for &l in &loops {
        note(
            "mef_chord",
            body.clone().mef_chord(MefSite::Lone { r#loop: l }).is_ok(),
        );
        note(
            "mev_line",
            body.clone()
                .mev_line(MevSite::Lone { r#loop: l }, p(43.0))
                .is_ok(),
        );
        note("mfkrh_plug", body.clone().mfkrh_plug(l).is_ok());
    }
    census
}

/// **The headline row.** Randomly torn bodies, every operator over
/// every key, in the profile where the only panic available is a row-4
/// `unreachable!`.
///
/// A counterexample search (shape 1 in `test_utils::fuzz`'s taxonomy):
/// varying seed, counts on `CAD_FUZZ_EFFORT`, monotone in the safe
/// direction — cutting the count loses detection power, never
/// correctness. The tear KINDS are enumerated (every kind is planted on
/// every trial); what varies is which entity each lands on and how many
/// tears compound.
///
/// VALIDATED AGAINST ITS OWN NEGATIVE CONTROL at review time: with
/// `kef`'s `[a, c, d]` reverted to `[c, d]` this row reds from `kef`,
/// and with `split_edge`'s `[hp_next, hm_prev]` reverted to
/// `[hp_next]` it reds from `split_edge`. Both gaps are independently
/// reachable, so the two checks D18 adds are load-bearing rather than
/// belt-and-braces.
#[test]
#[cfg(not(debug_assertions))]
fn torn_bodies_never_reach_a_row_four_unreachable() {
    use test_utils::fuzz;
    let mut rng = fuzz::start("review_d18::torn_bodies_row_four");
    let trials = fuzz::scaled(3);
    let mut census = test_utils::census::Census::new("review_d18 torn sweep");
    for trial in 0..trials {
        for tear in TEARS {
            let cube = ops_cube();
            let mut body = cube.body;
            let dead = recycled_dead_half_edge(&mut body);
            // Compound the tears: one on the first pass, more as the
            // trial index rises, so single and multiple corruption both
            // get exercised.
            for _ in 0..=trial {
                plant(&mut body, tear, &mut rng, dead);
            }
            let extra = TEARS[(rng.next_u64() as usize) % TEARS.len()];
            plant(&mut body, extra, &mut rng, dead);
            census.merge(&hammer(&body));
        }
    }
    census.report();
    census.require(
        CALLS,
        1_001,
        &format!(
            "the tear planting or the key enumeration has collapsed — {}",
            fuzz::replay()
        ),
    );
    census.require_nonzero_among(
        &OPS,
        4,
        &format!(
            "too few operators reached a mutation phase, so most of the arms under \
             attack were never in scope and this green says nothing about them — {}",
            fuzz::replay()
        ),
    );
}

/// The spent-destination attack: `graft_disjoint_all_keyed` is a PUBLIC
/// door whose own docs concede that a mid-transplant refusal leaves
/// `dst` partially written and never resumable. Take a body through a
/// failed graft, KEEP it, and run the operators over it.
///
/// The graft is made to fail in its cross-reference patch pass, which
/// is the failure that leaves half-edges holding SOURCE-arena keys in
/// `next`/`prev` — the #720 hazard, where such a key may resolve to an
/// unrelated LIVE entity of `dst` rather than dangle. Tearing the
/// source needs `pub(crate)` reach, so this row is a SUPERSET of what a
/// public consumer can build, which is the right direction for a
/// falsification attempt.
#[test]
#[cfg(not(debug_assertions))]
fn a_spent_graft_destination_never_reaches_a_row_four_unreachable() {
    let cube = ops_cube();
    let mut src = cube.body;
    src.get_half_edge_mut(cube.mevs[0].he_plus).unwrap().next = HalfEdgeKey::default();
    let mut dst = ops_cube().body;
    let before = deep_snapshot(&dst);
    assert!(
        crate::graft_disjoint_all_keyed(&mut dst, &src).is_err(),
        "the torn source must refuse to graft"
    );
    assert_ne!(
        deep_snapshot(&dst),
        before,
        "this row only means something if the refusal really did leave \
         `dst` partially written; if the graft ever becomes atomic, retire it"
    );
    let census = hammer(&dst);
    census.report();
    census.require(
        CALLS,
        101,
        "the spent destination no longer presents the keys this row hammers",
    );
    // THE FLOOR THIS ROW EXISTS BEHIND, and the one its twin already
    // had. A spent graft destination is more structurally damaged than
    // a randomly torn cube, so it is the likelier of the two to have
    // its calls refuse in a plan phase — the run that proves nothing
    // about the arms under attack while passing green.
    //
    // Counted over OPERATORS, not over the total: see `hammer`.
    census.require_nonzero_among(
        &OPS,
        4,
        "too few operators reached a mutation phase on the spent destination, so the \
         arms under attack were never in scope and this green says nothing about them",
    );
}
