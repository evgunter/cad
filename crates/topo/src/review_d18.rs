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
//! - The **torn-body sweep** CALLS the six operators `LINK_OPS` names —
//!   `kef`, `kev`, `kemr`, `mev_line`, `mef_chord`, `split_edge` — at
//!   every key of a randomly torn body, plus `mfkrh_plug`, which is
//!   driven and printed as evidence and is **not** in the class (`OPS`
//!   against `LINK_OPS` below). A counterexample search: varying seed,
//!   counts on `CAD_FUZZ_EFFORT`, monotone in the safe direction.
//!
//!   **The driven set is not the whole class**, and this file claims
//!   only the driven set. `mekr` reaches `link_half_edges` too — twelve
//!   splices across its four site variants — and nothing here drives it;
//!   that gap is
//!   `work/topo/review-d18-drives-no-mekr-though-it-reaches-link-half-edges`.
//!
//!   **Calling is not reaching, and the two rows say which they did.**
//!   Both print an exposure per operator and both ASSERT it: the
//!   deterministic row pins every operator's count exactly against
//!   `SPENT_GRAFT_EXPOSURE`, and the sampling row floors how many of
//!   `LINK_OPS` entered a mutation phase — the only place a row-4 arm
//!   can fire. `kemr` is the one that needed a fixture: its plan phase
//!   wants the two halves of ONE edge in ONE loop, which no cube and no
//!   torn cube presents, so both rows hammer
//!   [`crate::fixtures::ops_ring_bridge`] — the holed box with its hole
//!   rim bridged back into the top face's outer loop — beside the cube.
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
//! job, which selects rows BY NAME. Promoting a file means adding
//! `review_d18` to that job's filter list and a `grep -q` line for
//! [`torn_bodies_never_reach_a_row_four_unreachable`], or the sweep
//! ships without ever running. The profile-independent rows above ride
//! the standard matrix and need nothing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/topo/src/euler.rs",
    "crates/topo/src/euler_ring.rs",
    "crates/topo/src/euler_kill.rs",
    "crates/topo/src/body.rs",
    "crates/topo/src/entity.rs",
    "crates/topo/src/fixtures.rs",
];

use geom_core::Point3;

use crate::body::Body;
use crate::entity::{EntityId, HalfEdgeKey};
use crate::euler::{EulerOpError, MefSite, MevSite};
use crate::fixtures::{deep_snapshot, ops_cube, ops_ring_bridge};
use geom_core::Tol;
#[cfg(not(debug_assertions))]
use test_utils::vacuity::Exposure;

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
fn recycled_dead_half_edge(body: &mut Body<f64>, tol: Tol) -> HalfEdgeKey {
    let seed = body.mvfs(p(50.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(51.0),
            tol,
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
    let tol = Tol::witness();
    let cube = ops_cube(tol);
    let mut base = cube.body;
    let edge = cube.mevs[0].edge;
    let recycled = recycled_dead_half_edge(&mut base, tol);
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
        let err = body.split_edge(edge, 0.5, tol).unwrap_err();
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
    let control = body.split_edge(edge, 0.5, tol);
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
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(1.0),
            tol,
        )
        .unwrap();
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_minus,
                he2: seg.he_minus,
            },
            p(2.0),
            tol,
        )
        .unwrap();
    let split = body
        .mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: strut.he_minus,
            },
            tol,
        )
        .unwrap();
    let recycled = recycled_dead_half_edge(&mut body, tol);
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
    let tol = Tol::witness();
    use core::f64::consts::PI;

    use crate::entity::EdgeKey;

    type Shape = (&'static str, f64, Box<dyn Fn() -> (Body<f64>, EdgeKey)>);
    let shapes: Vec<Shape> = vec![
        (
            "generic cube edge",
            0.5,
            Box::new(move || {
                let cube = ops_cube(tol);
                let edge = cube.mevs[0].edge;
                (cube.body, edge)
            }),
        ),
        (
            "strut: next(he_plus) == he_minus",
            0.5,
            Box::new(move || {
                let cube = ops_cube(tol);
                let mut body = cube.body;
                let anchor = cube.mevs[0].he_plus;
                let strut = body
                    .mev_line(
                        MevSite::Fan {
                            he1: anchor,
                            he2: anchor,
                        },
                        p(2.0),
                        tol,
                    )
                    .unwrap();
                (body, strut.edge)
            }),
        ),
        (
            "full-period self-loop edge (a one-half-edge loop each side)",
            PI,
            Box::new(move || {
                let mut body = Body::<f64>::new();
                let seed = body.mvfs(p(0.0)).unwrap();
                let seg = body
                    .mev_line(
                        MevSite::Lone {
                            r#loop: seed.r#loop,
                        },
                        p(1.0),
                        tol,
                    )
                    .unwrap();
                let circ = body
                    .mef_chord(
                        MefSite::Chords {
                            he1: seg.he_minus,
                            he2: seg.he_minus,
                        },
                        tol,
                    )
                    .unwrap();
                (body, circ.edge)
            }),
        ),
        (
            "two-half-edge loop (segment: both halves in one loop)",
            0.5,
            Box::new(move || {
                let mut body = Body::<f64>::new();
                let seed = body.mvfs(p(0.0)).unwrap();
                let seg = body
                    .mev_line(
                        MevSite::Lone {
                            r#loop: seed.r#loop,
                        },
                        p(1.0),
                        tol,
                    )
                    .unwrap();
                (body, seg.edge)
            }),
        ),
    ];
    for (label, t, build) in shapes {
        let (mut control, edge) = build();
        assert!(
            control.split_edge(edge, t, tol).is_ok(),
            "{label}: control split must succeed"
        );
        let (mut body, edge) = build();
        let hm = body.get_edge(edge).unwrap().he_minus;
        let dead = HalfEdgeKey::default();
        body.get_half_edge_mut(hm).unwrap().prev = dead;
        let before = deep_snapshot(&body);
        // Not caught: a panic here IS the finding and should carry its
        // own message and backtrace.
        let err = body.split_edge(edge, t, tol).unwrap_err();
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

/// The head this guard carves from.
const LINK_HALF_EDGES: &str = "pub(crate) fn link_half_edges(";

/// The body of the item whose head is `head`, carved out of `source`
/// **as code**: comments and literal bodies blanked by the shared
/// lexer, then the shared item carve.
///
/// **The view and the carve are one decision**, which is why both are
/// `test_utils::source`'s. Over a blanked view every bracket is a real
/// bracket, so the carve is a parse; over raw text this was a search
/// for `"\n    }\n"`, which ended the body at whichever line happened
/// to be indented like a method's close — a `}` spelled in a string or
/// in commented-out code among them — and which could not tell a live
/// call from a commented-out one either.
fn code_body(source: &str, head: &str) -> String {
    let code = test_utils::source::code_only(source);
    let at = code
        .find(head)
        .unwrap_or_else(|| panic!("`{head}` must still exist"));
    match test_utils::source::item_body(&code, at) {
        test_utils::source::ItemBody::Body(body) => code[body].to_string(),
        other => panic!("`{head}` has no body: {other:?}"),
    }
}

/// **The carve reads the code, not the text**, so a call that has been
/// commented out stops counting. That direction is the whole point of
/// the guard below: over raw source the text of a commented-out
/// `unreachable!` stays in the file, the count does not move, and the
/// guard is green over exactly the change it exists to catch. The
/// third assertion is that raw read, on the same fixture, not moving.
#[test]
fn a_commented_out_announcement_stops_counting() {
    let live = "\
impl Body {
    pub(crate) fn link_half_edges(&mut self, a: Live, b: Live) {
        let Some(he) = self.get_half_edge_mut(a.key()) else {
            unreachable!(\"link_half_edges: `a`'s proof outlived its key\")
        };
        he.next = b.key();
        let Some(he) = self.get_half_edge_mut(b.key()) else {
            unreachable!(\"link_half_edges: `b`'s proof outlived its key\")
        };
    }
}
";
    assert_eq!(
        code_body(live, LINK_HALF_EDGES)
            .matches("unreachable!")
            .count(),
        2,
        "the clean fixture must carve both announcements"
    );
    let planted = live.replacen("unreachable!", "// unreachable!", 1);
    assert_eq!(
        code_body(&planted, LINK_HALF_EDGES)
            .matches("unreachable!")
            .count(),
        1,
        "commenting an announcement out must MOVE the count"
    );
    assert_eq!(
        planted.matches("unreachable!").count(),
        2,
        "the raw text does not move, which is why the carve is over the code view"
    );
}

/// **A brace inside a literal does not end the carve** — the other
/// defect the `"\n    }\n"` search had. A body whose message spells a
/// `}` (or whose commented-out line does) ended there, and every
/// announcement after the cut was silently not counted; the guard
/// stayed green while reading a fraction of the function. The row goes
/// red if the carve ever runs over raw text or over the literal view,
/// where the blanked brace is a real one again.
///
/// The trailing item is the other direction: the carve must stop at
/// the body's own close and not swallow the next item.
#[test]
fn a_brace_inside_a_literal_does_not_end_the_carve() {
    let live = "\
impl Body {
    pub(crate) fn link_half_edges(&mut self, a: Live, b: Live) {
        let Some(he) = self.get_half_edge_mut(a.key()) else {
            unreachable!(\"link_half_edges: `a` }\\n    }\\n outlived its key\")
        };
        // }
        he.next = b.key();
        let Some(he) = self.get_half_edge_mut(b.key()) else {
            unreachable!(\"link_half_edges: `b`'s proof outlived its key\")
        };
    }
    pub(crate) fn decoy(&mut self) { unreachable!(\"not this body\") }
}
";
    let body = code_body(live, LINK_HALF_EDGES);
    assert_eq!(
        body.matches("unreachable!").count(),
        2,
        "a `}}` spelled in a literal or a comment ended the carve early:\n{body}"
    );
    assert!(
        !body.contains("decoy"),
        "the carve ran past the body's own closing brace:\n{body}"
    );
}

/// Pins the SHAPE the sweep below assumes, which is the one way that
/// sweep could go quietly vacuous: if `link_half_edges` ever went back
/// to discarding a failed lookup, a torn-body hammer would report green
/// while announcing nothing. Source text, because there is no runtime
/// way to ask a function whether it announces.
#[test]
fn link_half_edges_still_announces_rather_than_discards() {
    let source = std::fs::read_to_string(crate::source_walk::src_root().join("euler.rs"))
        .expect("euler.rs must be readable");
    let body = code_body(&source, LINK_HALF_EDGES);
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

/// **D107's claim on the standard matrix**, which is where the two
/// hammer rows are not: they are `cfg(not(debug_assertions))` and run in
/// one job, and until this row the fact that `kemr` reaches a mutation
/// phase at all was carried only by a printed exposure line that a
/// passing test's captured stdout never shows. This states it in one
/// call, profile-independently, and states its converse too.
///
/// The converse is the half that explains S161: it is not that the
/// sweep drew too few samples, it is that a cube has no input for
/// `kemr` at all. Every mate pair of every cube edge — both argument
/// orders — refuses at `NotSameLoop`, because a closed cube's every
/// edge borders two distinct faces. No amount of tearing adds an edge
/// whose halves share a loop, which is why ~1900 calls found nothing
/// and one fixture does.
#[test]
fn kemr_reaches_a_mutation_phase_on_the_ring_bridge_and_on_no_cube_edge() {
    let tol = Tol::witness();
    let bridge = ops_ring_bridge(tol);
    let edge = bridge.bridge.edge;
    let mut body = bridge.body;
    let (hp, hm) = {
        let d = body.get_edge(edge).unwrap();
        (d.he_plus, d.he_minus)
    };
    let out = body
        .kemr(hp, hm)
        .expect("the bridge edge's halves share a loop, so kemr's plan phase passes");
    // A CYCLE ring, not an empty one: both sides of the split were
    // non-empty, so the mutation phase ran BOTH of its `link_half_edges`
    // splices rather than the one a strut kill reaches.
    assert!(
        matches!(
            body.get_loop(out.ring).unwrap().boundary,
            crate::LoopBoundary::Cycle { .. }
        ),
        "the split's ring side must be a cycle, or only one splice ran"
    );
    assert_eq!(crate::validate::validate(&body), Ok(()));

    let cube = ops_cube(tol);
    let body = cube.body;
    let edges: Vec<crate::entity::EdgeKey> = body.edges().map(|(k, _)| k).collect();
    assert!(!edges.is_empty(), "fixture: the cube must present edges");
    for e in edges {
        let (hp, hm) = {
            let d = body.get_edge(e).unwrap();
            (d.he_plus, d.he_minus)
        };
        for (he1, he2) in [(hp, hm), (hm, hp)] {
            assert_eq!(
                body.clone().kemr(he1, he2),
                Err(EulerOpError::NotSameLoop { he1, he2 }),
                "a cube edge's halves must lie in two faces' loops; if one pair ever \
                 shares a loop the hammer's cube rows stop being vacuous for `kemr` \
                 and the ring-bridge fixture's reason needs re-reading"
            );
        }
    }
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

/// The operators this pass drives whose mutation phase **reaches
/// [`crate::Body::link_half_edges`]**, and therefore the row-4 arms
/// under attack: `kef`/`kev` (`euler_kill.rs`), `kemr`
/// (`euler_ring.rs`), `mev_line`/`mef_chord` (`euler.rs`) and
/// `split_edge` (`split.rs`).
///
/// **This is the list the floor counts over, and it is not the list the
/// pass drives.** `mfkrh_plug` is driven and counted as evidence, but
/// its mutation phase mints a face surface, adds a face and touches
/// `face`/`loop`/`shell` records — it calls `link_half_edges` NOWHERE,
/// so a run that reached only it has reached none of the arms, which is
/// exactly the run that made a floor on the total unfalsifiable here
/// (`mfkrh`'s plan phase reads only `loop.face`, `face.outer`,
/// `face.shell`, so it survives a fully nulled arena). An operator that
/// cannot reach the arms cannot be evidence that the arms were reached.
///
/// **Nor is it the whole class**: `mekr` reaches the arms and this pass
/// does not drive it (module docs, and
/// `work/topo/review-d18-drives-no-mekr-though-it-reaches-link-half-edges`).
/// Adding it here without driving it would floor over a category that
/// can never be nonzero.
#[cfg(not(debug_assertions))]
const LINK_OPS: [&str; 6] = ["kef", "kev", "kemr", "mev_line", "mef_chord", "split_edge"];

/// Every operator the pass drives — [`LINK_OPS`] plus the one that does
/// not reach the arms. Counted, printed, and deliberately absent from
/// the floor.
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

/// How many of [`LINK_OPS`] an intact tree reaches a mutation phase in,
/// **measured, on both rows**. A measurement, not a target: when it
/// moves the question is whether the new behaviour is right, and the
/// repair is to re-derive it and say what moved — never to steer the
/// sweep back to the old number.
#[cfg(not(debug_assertions))]
const MEASURED_LINK_OPS: usize = 6;

/// Every operator's exposure on the spent graft destination,
/// **exactly**, because that row is deterministic end to end — one
/// fixture, one tear, one graft, one hammer, no `Rng` and no effort
/// dial. Asserted rather than written in a comment: the comment this
/// replaced said `mev_line 60` against a measured 52, which is what a
/// count with nothing holding it does.
///
/// A change that moves one of these is not a failure to be edited back
/// into line; re-derive the row and say in the PR what moved and why it
/// is right.
#[cfg(not(debug_assertions))]
const SPENT_GRAFT_EXPOSURE: [(&str, usize); 7] = [
    ("kef", 40),
    ("kemr", 2),
    ("kev", 50),
    ("mef_chord", 90),
    ("mev_line", 78),
    ("mfkrh_plug", 7),
    ("split_edge", 93),
];

/// Calls the spent-destination row makes, exactly — see
/// [`SPENT_GRAFT_EXPOSURE`] for why an exact number and not a floor.
#[cfg(not(debug_assertions))]
const SPENT_GRAFT_CALLS: usize = 1_420;

#[cfg(not(debug_assertions))]
const CALLS: &str = "operator calls";

/// The bodies both hammer rows are run over.
///
/// **The cube alone cannot get `kemr` past its plan phase, and no
/// amount of tearing changes that.** `kemr` wants the two halves of one
/// edge lying in one loop; every edge of a closed cube borders two
/// distinct faces, so its halves sit in two loops and the plan phase
/// refuses at `NotSameLoop` — a structural fact about the fixture, not
/// a sampling shortfall, which is why the gap survived a sweep of
/// ~1900 `kemr` calls (S161 / D107).
/// [`crate::fixtures::ops_ring_bridge`] is the holed box with its hole
/// rim joined back into the top face's outer loop by one `mekr`: the
/// bridge edge has both halves in that loop, both sides of the split
/// are non-empty, and `kemr` runs both of its splices there.
#[cfg(not(debug_assertions))]
const FIXTURES: [fn(Tol) -> Body<f64>; 2] =
    [|tol| ops_cube(tol).body, |tol| ops_ring_bridge(tol).body];

/// A [`Tear`] kind's exposure category: a planting of that kind that
/// actually changed the body.
///
/// **The floor the CALLS one cannot be.** An intact cube hammers to 438
/// calls and reaches five operators, so 27 no-op plantings clear both
/// other floors while sweeping nothing but intact cubes — a sweep with
/// no corruption in it, which is the vacuity this row is most exposed
/// to. Counted per KIND rather than per trial: a single draw may
/// legitimately find no eligible entity for the kind it was asked to
/// plant (measured, 26 of 27 trials land on a typical seed), so *every
/// trial corrupted* is a claim about luck, while *every corruption
/// shape was planted somewhere* is the claim the sweep's own docs make
/// — [`TEARS`] is an enumeration, and every kind is meant to be
/// exercised on every run.
#[cfg(not(debug_assertions))]
fn tear_landed(tear: Tear) -> String {
    format!("tear landed: {tear:?}")
}

/// Calls every operator [`OPS`] names at every key of a torn body, and
/// returns what the pass actually reached as an anti-vacuity exposure
/// ([`test_utils::vacuity`]).
///
/// Nothing is caught: in this profile the postcondition is compiled out,
/// so a panic escaping here is a row-4 arm and it should fail the test
/// with its own message.
///
/// **What it counts, and why per operator.** [`CALLS`] is every operator
/// driven at a key; each operator's own category counts the calls that
/// returned `Ok`, i.e. that RAN THEIR MUTATION PHASE, which is the only
/// place a row-4 `unreachable!` can fire. A pass whose calls all died in
/// a plan phase proves nothing about the arms under attack.
///
/// Per operator, because a total does not distinguish *six operators
/// exercised* from *one exercised and five refused at the door* — and on
/// this fixture family that is not hypothetical: null every arena field
/// of the spent destination and `mfkrh_plug` still returns `Ok` six
/// times, so a floor on the total is one almost nothing can break.
/// [`LINK_OPS`] is therefore what the floor counts over, and
/// `mfkrh_plug` is not in it.
///
/// **Two enumerations per operator, because `kemr`'s arguments are not
/// free.** Every other operator here takes keys that may be drawn
/// independently, and the arbitrary pairs below are the adversarial
/// half of the pass — they attack each plan phase with keys no caller
/// would pass. `kemr` refuses all of them at `NotSameEdge` or
/// `NotSameLoop` before it reads anything else, so the arbitrary pairs
/// alone can never carry it past its plan phase however many bodies
/// they are run over. The MATE pair — an edge's own two halves, both
/// argument orders, since the order selects which side becomes the ring
/// — is the enumeration that can, and it is added rather than
/// substituted so the refusal paths keep their attack.
#[cfg(not(debug_assertions))]
fn hammer(body: &Body<f64>, tol: Tol) -> Exposure {
    use crate::entity::{EdgeKey, LoopKey};
    let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(k, _)| k).collect();
    let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    let loops: Vec<LoopKey> = body.loops().map(|(k, _)| k).collect();
    let mut census = Exposure::new("review_d18 hammer");
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
                .mev_line(MevSite::Fan { he1: he, he2: he }, p(41.0), tol)
                .is_ok(),
        );
        note(
            "mef_chord",
            body.clone()
                .mef_chord(MefSite::Chords { he1: he, he2: he }, tol)
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
                        tol,
                    )
                    .is_ok(),
            );
            note(
                "mef_chord",
                body.clone()
                    .mef_chord(
                        MefSite::Chords {
                            he1: he,
                            he2: other,
                        },
                        tol,
                    )
                    .is_ok(),
            );
        }
    }
    for &edge in &edges {
        for t in [0.25, 0.5, 0.75] {
            note("split_edge", body.clone().split_edge(edge, t, tol).is_ok());
        }
        if let Some(edge_data) = body.get_edge(edge) {
            let (hp, hm) = (edge_data.he_plus, edge_data.he_minus);
            note("kemr", body.clone().kemr(hp, hm).is_ok());
            note("kemr", body.clone().kemr(hm, hp).is_ok());
        }
    }
    for &l in &loops {
        note(
            "mef_chord",
            body.clone()
                .mef_chord(MefSite::Lone { r#loop: l }, tol)
                .is_ok(),
        );
        note(
            "mev_line",
            body.clone()
                .mev_line(MevSite::Lone { r#loop: l }, p(43.0), tol)
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
/// every trial) and so are the BODIES ([`FIXTURES`]); what varies is
/// which entity each tear lands on and how many tears compound.
///
/// VALIDATED AGAINST ITS OWN NEGATIVE CONTROL at review time: with
/// `kef`'s `[a, c, d]` reverted to `[c, d]` this row reds from `kef`,
/// and with `split_edge`'s `[hp_next, hm_prev]` reverted to
/// `[hp_next]` it reds from `split_edge`. Both gaps are independently
/// reachable, so the two checks D18 adds are load-bearing rather than
/// belt-and-braces.
///
/// `kemr`'s coverage carries its own two controls, each run against the
/// other half held fixed: drop [`crate::fixtures::ops_ring_bridge`] from
/// [`FIXTURES`] and `kemr` returns to 0 with the mate-pair enumeration
/// still in place; drop the mate-pair calls from [`hammer`] and it
/// returns to 0 with the fixture still swept. So the fixture and the
/// enumeration are each necessary, and neither is decoration on the
/// other.
#[test]
#[cfg(not(debug_assertions))]
fn torn_bodies_never_reach_a_row_four_unreachable() {
    use test_utils::fuzz;
    let tol = Tol::witness();
    let mut rng = fuzz::start("review_d18::torn_bodies_row_four");
    let trials = fuzz::scaled(3);
    let mut census = Exposure::new("review_d18 torn sweep");
    for trial in 0..trials {
        for tear in TEARS {
            for build in FIXTURES {
                let mut body = build(tol);
                let dead = recycled_dead_half_edge(&mut body, tol);
                // Compound the tears: one on the first pass, more as the
                // trial index rises, so single and multiple corruption both
                // get exercised. Each planting is snapshotted individually,
                // so the exposure says which KINDS landed rather than which
                // trials did.
                let extra = TEARS[(rng.next_u64() as usize) % TEARS.len()];
                for kind in core::iter::repeat_n(tear, trial + 1).chain([extra]) {
                    let before = deep_snapshot(&body);
                    plant(&mut body, kind, &mut rng, dead);
                    if deep_snapshot(&body) != before {
                        census.note(&tear_landed(kind));
                    }
                }
                census.merge(&hammer(&body, tol));
            }
        }
    }
    census.report();
    // EVERY corruption shape must have landed somewhere: `plant` takes a
    // `Tear` and a drawn key and can quietly become a no-op for a shape
    // it no longer knows how to reach. Derived from `TEARS` rather than
    // written out, so the floor cannot drift from the enumeration.
    let landed: Vec<String> = TEARS.iter().map(|t| tear_landed(*t)).collect();
    let landed: Vec<&str> = landed.iter().map(String::as_str).collect();
    census.require_nonzero_among(
        &landed,
        TEARS.len(),
        &format!(
            "a corruption shape never landed on any body in the whole sweep, so the \
             hammer below ran on bodies that shape had not torn — {}",
            fuzz::replay()
        ),
    );
    // The KEY ENUMERATION, and only that; the tear floor above carries
    // the other half. Measured on an intact tree at the floor of the
    // effort dial: 38502 — a fixed number, since a tear never adds or
    // removes an entity and the pass iterates fixed collections.
    census.require(
        CALLS,
        3_001,
        &format!(
            "the key enumeration has collapsed — the bodies no longer present the \
             half-edges, edges and loops this sweep hammers — {}",
            fuzz::replay()
        ),
    );
    // ALL SIX link-reaching operators reach a mutation phase here, minus
    // one of slack: MEASURED_LINK_OPS is what an intact tree reaches and
    // the floor sits one below it.
    //
    // THE ONE OPERATOR OF SLACK IS DELIBERATE AND IS NOT A ROUNDING
    // ERROR. Do not "fix" it to 6. At 6 the floor would be a coverage
    // TARGET pinned to today's measurement, and the next legitimate
    // change to a plan-phase precondition reds a row that is still
    // attacking the arms perfectly well. One below, a genuine loss — a
    // second operator joining the first at the door — reds it, which is
    // the behaviour the row exists for, while the exposure line above
    // reports the drop on every run for anyone reading.
    census.require_nonzero_among(
        &LINK_OPS,
        MEASURED_LINK_OPS - 1,
        &format!(
            "too few operators reached a mutation phase, so most of the arms under \
             attack were never in scope and this green says nothing about them — {}",
            fuzz::replay()
        ),
    );
    // `kemr` BY NAME, under the slack. It is the operator D107 bought
    // and the only one whose mutation phase needs a fixture the sweep
    // would not otherwise build: drop [`crate::fixtures::ops_ring_bridge`]
    // from [`FIXTURES`], or let the mate-pair enumeration in [`hammer`]
    // go back to arbitrary pairs, and `kemr` silently returns to 0 while
    // the floor above still passes on the other five.
    census.require(
        "kemr",
        1,
        &format!(
            "`kemr` entered no mutation phase in the whole sweep, so its two \
             `link_half_edges` splices are attacked by nothing and this row is back \
             where S161 found it — {}",
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
    let tol = Tol::witness();
    let cube = ops_cube(tol);
    let mut src = cube.body;
    src.get_half_edge_mut(cube.mevs[0].he_plus).unwrap().next = HalfEdgeKey::default();
    // The DESTINATION carries the ring bridge, so the spent body still
    // presents the one edge whose halves share a loop and `kemr` has an
    // input here too; the source stays the cube, since what it has to be
    // is torn enough to refuse mid-patch.
    let mut dst = ops_ring_bridge(tol).body;
    let before = deep_snapshot(&dst);
    assert!(
        crate::graft_disjoint_all_keyed(&mut dst, &src, tol).is_err(),
        "the torn source must refuse to graft"
    );
    assert_ne!(
        deep_snapshot(&dst),
        before,
        "this row only means something if the refusal really did leave \
         `dst` partially written; if the graft ever becomes atomic, retire it"
    );
    let census = hammer(&dst, tol);
    census.report();
    // THE FLOOR THIS ROW EXISTS BEHIND, and the one its twin already
    // had. A spent graft destination is more structurally damaged than
    // a randomly torn cube, so it is the likelier of the two to have
    // its calls refuse in a plan phase — the run that proves nothing
    // about the arms under attack while passing green.
    //
    // Not over the total, and not over `OPS`: see `LINK_OPS`. The one
    // operator of slack is deliberate — see the twin row above for what
    // it buys.
    census.require_nonzero_among(
        &LINK_OPS,
        MEASURED_LINK_OPS - 1,
        "too few operators reached a mutation phase on the spent destination, so the \
         arms under attack were never in scope and this green says nothing about them",
    );
    // AND THE EXACT TALLY, which this row alone can carry: it is
    // deterministic, so every operator's exposure is a fact about the
    // tree rather than about a draw, and a per-operator number is the
    // only form in which the module doc's coverage claim is checkable at
    // all. The floor above cannot see `kemr` falling from its 2 back to
    // 0 as long as the other five still reach; this can, and so can a
    // change that quietly halves `mef_chord`.
    for (op, expected) in SPENT_GRAFT_EXPOSURE {
        assert_eq!(
            census.count(op),
            expected,
            "`{op}` reached {} mutation phases on the spent destination, against {expected} \
             measured: {census}",
            census.count(op)
        );
    }
    assert_eq!(
        census.count(CALLS),
        SPENT_GRAFT_CALLS,
        "the spent destination no longer presents the keys this row hammers: {census}"
    );
}
