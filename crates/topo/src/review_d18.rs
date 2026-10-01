//! Adversarial falsification probes for Track D unit **D18** (PR #736):
//! the two `prev` proofs (`split_edge`'s `prev(he_minus)`, `kef`'s
//! `prev(he)`) and the conversion of [`crate::Body::link_half_edges`]
//! to the D2 addendum's row 4.
//!
//! Written by the correctness-lane reviewer to BREAK the unit, not to
//! describe it. The claim under attack is
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
//!   only the driven set. `mekr` reaches `link_half_edges` too, at every
//!   one of its four site variants, and the sweep does not drive it (the
//!   kill-anchor rows below do, for their own proofs, and assert nothing
//!   about row 4); the site tally and the grep it came from are on the
//!   row that owns the gap,
//!   `work/probe/review-d18-drives-no-mekr-though-it-reaches-link-half-edges`.
//!
//!   **Calling is not reaching, and the two rows say which they did.**
//!   Both print an exposure per operator and both ASSERT it, per
//!   operator rather than in aggregate: the deterministic row pins every
//!   operator's count exactly against `SPENT_GRAFT_EXPOSURE`, and the
//!   sampling row floors EACH of `LINK_OPS` — a mutation phase being the
//!   only place a row-4 arm can fire. `kemr` is the one that needed a
//!   fixture: its plan phase wants the two halves of ONE edge in ONE
//!   loop, which no cube and no torn cube presents, so the sweep hammers
//!   [`crate::fixtures::ops_ring_bridge`] (the holed box with its hole
//!   rim bridged back into the top face's outer loop, which reaches the
//!   two-splice arm) and [`crate::fixtures::ops_strut_cube`] (a pendant
//!   strut, which reaches the one-splice arm) beside the cube.
//! - The **kill-anchor** rows prove that no kill, and no make that moves
//!   a walked run, writes an anchor its torn input put elsewhere or
//!   leaves a record naming one it removed:
//!   [`valid_fixtures_never_refuse_a_kill_anchor`] enumerates valid
//!   bodies for over-refusal, [`kill_anchors_on_a_few_torn_bodies`] is a
//!   counterexample search over the tear kinds, and
//!   `kill_anchors_on_torn_bodies` is its by-hand evidence run.
//! - The **revert** rows do the same for `revert`'s start and anchor
//!   writes.
//! - The **removal** rows are deterministic witnesses: each kill refuses,
//!   typed and with the body unchanged, a record it keeps naming one it
//!   removes, through every door of its operator.
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
//! ships without ever running. The profile-independent rows above need
//! nothing: they run on every job that runs this suite at all — which
//! is not every job, since the whole module is `gated_to!` the files
//! named below and a PR touching none of them does not run it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/topo/src/euler.rs",
    "crates/topo/src/euler_ring.rs",
    "crates/topo/src/euler_kill.rs",
    "crates/topo/src/movefac.rs",
    "crates/topo/src/body.rs",
    "crates/topo/src/entity.rs",
    "crates/topo/src/fixtures.rs",
    "crates/topo/src/test_support_fixtures.rs",
    "crates/topo/src/seqgen.rs",
    "crates/topo/src/revert.rs",
];

use geom_core::Point3;

use crate::body::Body;
#[cfg(not(debug_assertions))]
use crate::entity::VertexKey;
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, LoopBoundary, SolidKey};
use crate::euler::{EulerOpError, MefSite, MevSite};
use crate::euler_ring::MekrSite;
use crate::fixtures::{
    KillAnchorFault, assert_kill_refuses, deep_snapshot, kill_anchor_faults, ops_genus2,
    ops_holed_box, ops_ring_bridge, ops_strut_cube, ops_two_ring_face, through_the_scalpel,
};
use crate::null::NullFacePair;
use crate::test_support_fixtures::declined_cube;
use geom_core::Tol;
#[cfg(not(debug_assertions))]
use std::collections::BTreeSet;
#[cfg(not(debug_assertions))]
use test_utils::vacuity::Exposure;

fn p(x: f64) -> Point3<f64> {
    Point3::new(x, 0.0, 0.0)
}

/// The two halves of `edge` — the file's ONE spelling of the lookup
/// every caller here needs, so the mate pair is read the same way in
/// the hammer and in the rows.
///
/// It cannot miss and is not guarded: every caller collected `edge`
/// from the arena of the very body it passes, and the operators run on
/// clones, so nothing between the collection and the read can retire
/// the key.
fn mate_halves(body: &Body<f64>, edge: crate::entity::EdgeKey) -> (HalfEdgeKey, HalfEdgeKey) {
    let d = body
        .get_edge(edge)
        .expect("the edge key was collected from this body");
    (d.he_plus, d.he_minus)
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
    let seed = body.mvfs(p(50.0), true).unwrap();
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
    let cube = declined_cube::<f64>(tol);
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
    let seed = body.mvfs(p(0.0), true).unwrap();
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
                let cube = declined_cube::<f64>(tol);
                let edge = cube.mevs[0].edge;
                (cube.body, edge)
            }),
        ),
        (
            "strut: next(he_plus) == he_minus",
            0.5,
            Box::new(move || {
                let cube = declined_cube::<f64>(tol);
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
                let seed = body.mvfs(p(0.0), true).unwrap();
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
                let seed = body.mvfs(p(0.0), true).unwrap();
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

// =====================================================================
// C1 coverage — that the hammer rows' `kemr` inputs exist at all, on
// every job rather than only where the hammer runs. Not a falsification
// of either C1 arm: a witness that the inputs C1's sweep needs are
// present, and that the fixtures which do not present them cannot.
// =====================================================================

/// **The `kemr` coverage claim, wherever this suite runs** — which the
/// two hammer rows are not: they are `cfg(not(debug_assertions))` and
/// run in one job, and until this row the fact that `kemr` reaches a
/// mutation phase at all was carried only by a printed exposure line
/// that a passing test's captured stdout never shows. This states it in
/// one call, profile-independently, and states its converse too.
///
/// The converse is the half that explains the gap: it is not that the
/// sweep drew too few samples, it is that the closed fixtures have no
/// input for `kemr` at all. Every mate pair of every edge of
/// [`crate::test_support_fixtures::declined_cube`], [`crate::fixtures::ops_holed_box`]
/// and [`crate::fixtures::ops_genus2`] — both argument orders — refuses
/// at `NotSameLoop`, because in each of them every edge borders two
/// distinct faces and its halves therefore sit in two loops. No amount
/// of tearing adds an edge whose halves share a loop, which is why a
/// sweep of torn cubes found nothing however many calls it made, and
/// why one fixture does.
#[test]
fn kemr_splices_twice_on_the_ring_bridge_once_on_a_strut_and_never_on_a_closed_fixture_edge() {
    let tol = Tol::witness();
    let bridge = ops_ring_bridge(tol);
    let edge = bridge.bridge.edge;
    let outer = bridge.outer;
    let mut body = bridge.body;
    let (hp, hm) = mate_halves(&body, edge);
    // The four neighbours the two splices must join, read BEFORE the
    // op: `kemr` links prev(he2) → next(he1) to close the ring side and
    // prev(he1) → next(he2) to close the old loop, and re-anchors each
    // loop at the `next` it kept.
    let (next_hp, prev_hp) = {
        let d = body.get_half_edge(hp).unwrap();
        (d.next, d.prev)
    };
    let (next_hm, prev_hm) = {
        let d = body.get_half_edge(hm).unwrap();
        (d.next, d.prev)
    };
    let out = body
        .kemr(hp, hm)
        .expect("the bridge edge's halves share a loop, so kemr's plan phase passes");
    // BOTH splices, each named by the link it wrote. Both sides of the
    // split were non-empty here, which is the arm that splices twice;
    // the arm a strut kill reaches writes only the second of these and
    // mints an `Empty` ring ([`crate::fixtures::ops_strut_cube`], driven
    // through [`hammer`]).
    assert_eq!(
        body.get_half_edge(prev_hm).unwrap().next,
        next_hp,
        "the ring side's splice did not run: prev(he2) must close onto next(he1)"
    );
    assert_eq!(
        body.get_half_edge(prev_hp).unwrap().next,
        next_hm,
        "the old loop's splice did not run: prev(he1) must close onto next(he2)"
    );
    // …and each side's anchor and closed cycle, so a splice that wrote
    // the link but left the loop unreachable is caught too.
    for (side, anchor, l) in [("ring", next_hp, out.ring), ("old loop", next_hm, outer)] {
        assert_eq!(
            body.get_loop(l).unwrap().boundary,
            crate::LoopBoundary::Cycle { first: anchor },
            "the {side} must be a cycle re-anchored at the half it kept"
        );
        let cycle = body
            .loop_cycle(anchor)
            .unwrap_or_else(|| panic!("the {side}'s cycle must close after the splice"));
        for he in cycle {
            assert_eq!(
                body.get_half_edge(he).unwrap().parent_loop,
                l,
                "every member of the {side}'s cycle must belong to it"
            );
        }
    }
    assert_eq!(crate::validate::validate(&body), Ok(()));

    // THE OTHER ARM, on the fixture that presents it: when the two
    // halves are adjacent the side strictly between them is empty, the
    // ring loop is minted `Empty` at he2's start vertex, and only the
    // old loop's splice runs. [`hammer`] reaches this arm too — it is
    // what [`KEMR_EMPTY_RING`] counts — but that is release-only, and
    // this is the arm `ops_holed_box`'s own builder depends on.
    let strut = ops_strut_cube(tol);
    let mut body = strut.body;
    let (hp, hm) = (strut.strut.he_plus, strut.strut.he_minus);
    let prev_hp = body.get_half_edge(hp).unwrap().prev;
    let (next_hm, w) = {
        let d = body.get_half_edge(hm).unwrap();
        (d.next, d.start)
    };
    let out = body
        .kemr(hp, hm)
        .expect("the strut's halves share a loop, so kemr's plan phase passes");
    assert_eq!(
        body.get_loop(out.ring).unwrap().boundary,
        crate::LoopBoundary::Empty { vertex: w },
        "an adjacent pair leaves no ring side, so the ring loop is the empty one"
    );
    assert_eq!(
        body.get_half_edge(prev_hp).unwrap().next,
        next_hm,
        "the old loop's splice — the only one this arm runs — did not run"
    );
    assert_eq!(
        body.get_loop(strut.outer).unwrap().boundary,
        crate::LoopBoundary::Cycle { first: next_hm },
        "the old loop must stay a cycle re-anchored at the half it kept"
    );
    assert_eq!(crate::validate::validate(&body), Ok(()));

    for (name, body) in [
        ("declined_cube", declined_cube::<f64>(tol).body),
        ("ops_holed_box", ops_holed_box(tol).body),
        ("ops_genus2", ops_genus2(tol)),
    ] {
        let edges: Vec<crate::entity::EdgeKey> = body.edges().map(|(k, _)| k).collect();
        assert!(!edges.is_empty(), "fixture: {name} must present edges");
        for e in edges {
            let (hp, hm) = mate_halves(&body, e);
            for (he1, he2) in [(hp, hm), (hm, hp)] {
                assert_eq!(
                    body.clone().kemr(he1, he2),
                    Err(EulerOpError::NotSameLoop { he1, he2 }),
                    "{name}: every edge's halves must lie in two faces' loops; if one \
                     pair ever shares a loop the hammer's rows over this body stop \
                     being vacuous for `kemr` and the ring-bridge fixture's reason \
                     needs re-reading"
                );
            }
        }
    }
}

/// The kinds of tier-1 corruption the sweep plants. Each is a shape
/// #720's review named as a way a key can be wrong: DANGLING (the
/// lookup fails — the only shape that can reach a row-4 arm) and
/// LIVE-BUT-WRONG (the lookup succeeds against an unrelated entity —
/// the shape a slotmap key laundered across arenas actually takes).
#[derive(Clone, Copy, Debug)]
#[cfg_attr(debug_assertions, allow(dead_code))] // a dev build plants `ANCHOR_TEARS` only
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
    /// A half-edge's `edge` set to another live edge.
    EdgeForeign,
    /// A loop set `Empty` at a live vertex.
    EmptyVertexForeign,
    /// A loop's `face` set to another live face.
    LoopFaceForeign,
    /// A live loop appended to a face's `rings`.
    RingForeign,
    /// A face's `outer` set to another live loop.
    OuterForeign,
    /// A face's `shell` set to another live shell.
    FaceShellForeign,
    /// A live face appended to a shell's `faces`.
    ShellFacesForeign,
    /// A shell's `solid` set to another live solid.
    ShellSolidForeign,
    /// A live shell appended to a solid's `shells`.
    SolidShellsForeign,
    /// A vertex's `emanating` set to another live half-edge.
    EmanatingForeign,
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

fn plant(body: &mut Body<f64>, tear: Tear, rng: &mut test_utils::fuzz::Rng, dead: HalfEdgeKey) {
    use crate::entity::{EdgeKey, LoopKey, VertexKey};
    let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(k, _)| k).collect();
    let loops: Vec<LoopKey> = body.loops().map(|(k, _)| k).collect();
    let vertices: Vec<VertexKey> = body.vertices().map(|(k, _)| k).collect();
    let mut pick = |n: usize| (rng.next_u64() as usize) % n;
    // The spine tears draw from arenas a body with no half-edge still
    // has; every other tear keeps its draws as they were.
    if plant_spine(body, tear, &mut pick) {
        return;
    }
    if halves.is_empty() {
        return;
    }
    let he = halves[pick(halves.len())];
    let other = halves[pick(halves.len())];
    match tear {
        Tear::EdgeForeign => {
            let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
            body.get_half_edge_mut(he).unwrap().edge = edges[pick(edges.len())];
        }
        Tear::EmptyVertexForeign
        | Tear::LoopFaceForeign
        | Tear::RingForeign
        | Tear::OuterForeign
        | Tear::FaceShellForeign
        | Tear::ShellFacesForeign
        | Tear::ShellSolidForeign
        | Tear::SolidShellsForeign => unreachable!("plant_spine planted {tear:?}"),
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
        Tear::EmanatingForeign => {
            let v = vertices[pick(vertices.len())];
            body.get_vertex_mut(v).unwrap().emanating = Some(other);
        }
    }
}

/// Plants a tear of the spine records (a loop's boundary or face, a
/// face's rings, outer or shell, a shell's faces or solid, a solid's shells),
/// each drawn over the live keys of its arenas; `false` for a tear of
/// the half-edge records, which [`plant`] draws itself.
fn plant_spine(body: &mut Body<f64>, tear: Tear, pick: &mut impl FnMut(usize) -> usize) -> bool {
    let loops: Vec<_> = body.loops().map(|(k, _)| k).collect();
    let faces: Vec<_> = body.faces().map(|(k, _)| k).collect();
    let shells: Vec<_> = body.shells().map(|(k, _)| k).collect();
    let solids: Vec<_> = body.solids().map(|(k, _)| k).collect();
    let vertices: Vec<_> = body.vertices().map(|(k, _)| k).collect();
    match tear {
        Tear::EmptyVertexForeign => {
            let (l, v) = (loops[pick(loops.len())], vertices[pick(vertices.len())]);
            body.get_loop_mut(l).unwrap().boundary = LoopBoundary::Empty { vertex: v };
        }
        Tear::LoopFaceForeign => {
            let (l, f) = (loops[pick(loops.len())], faces[pick(faces.len())]);
            body.get_loop_mut(l).unwrap().face = f;
        }
        Tear::RingForeign => {
            let (f, l) = (faces[pick(faces.len())], loops[pick(loops.len())]);
            body.get_face_mut(f).unwrap().rings.push(l);
        }
        Tear::OuterForeign => {
            let (f, l) = (faces[pick(faces.len())], loops[pick(loops.len())]);
            body.get_face_mut(f).unwrap().outer = l;
        }
        Tear::FaceShellForeign => {
            let (f, s) = (faces[pick(faces.len())], shells[pick(shells.len())]);
            body.get_face_mut(f).unwrap().shell = s;
        }
        Tear::ShellFacesForeign => {
            let (s, f) = (shells[pick(shells.len())], faces[pick(faces.len())]);
            body.get_shell_mut(s).unwrap().faces.push(f);
        }
        Tear::ShellSolidForeign => {
            let (s, solid) = (shells[pick(shells.len())], solids[pick(solids.len())]);
            body.get_shell_mut(s).unwrap().solid = solid;
        }
        Tear::SolidShellsForeign => {
            let (solid, s) = (solids[pick(solids.len())], shells[pick(shells.len())]);
            body.get_solid_mut(solid).unwrap().shells.push(s);
        }
        _ => return false,
    }
    true
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
/// `work/probe/review-d18-drives-no-mekr-though-it-reaches-link-half-edges`).
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

/// `kemr`'s mutation phase has TWO arms, and the exposure counts them
/// apart: it splices twice when both sides of the split are non-empty
/// (the ring loop is minted as a cycle) and once when the ring side is
/// empty (the ring loop is minted `Empty` and only the old loop's
/// splice runs). A count of `kemr` alone cannot tell a sweep that
/// reached both from one that reached either.
#[cfg(not(debug_assertions))]
const KEMR_CYCLE_RING: &str = "kemr: cycle ring side";

/// The one-splice arm — see [`KEMR_CYCLE_RING`].
#[cfg(not(debug_assertions))]
const KEMR_EMPTY_RING: &str = "kemr: empty ring side";

/// Every operator's exposure on the spent graft destination,
/// **exactly**, because that row is deterministic end to end — one
/// fixture, one tear, one graft, one hammer, no `Rng` and no effort
/// dial. Asserted rather than written in a comment: the comment this
/// replaced named a per-operator count that nothing held, and it had
/// been wrong for the row's whole life.
///
/// A change that moves one of these is not a failure to be edited back
/// into line; re-derive the row and say in the PR what moved and why it
/// is right.
#[cfg(not(debug_assertions))]
const SPENT_GRAFT_EXPOSURE: [(&str, usize); 9] = [
    ("kef", 16),
    ("kemr", 2),
    (KEMR_CYCLE_RING, 2),
    (KEMR_EMPTY_RING, 0),
    ("kev", 25),
    ("mef_chord", 90),
    ("mev_line", 54),
    ("mfkrh_plug", 7),
    ("split_edge", 93),
];

/// Calls the spent-destination row makes, exactly — see
/// [`SPENT_GRAFT_EXPOSURE`] for why an exact number and not a floor.
#[cfg(not(debug_assertions))]
const SPENT_GRAFT_CALLS: usize = 1_420;

/// Calls one ROUND of the torn sweep makes — every [`FIXTURES`] entry
/// hammered once — exactly.
///
/// The sweep's call count is a function of the key enumeration alone:
/// [`plant`] rewrites fields and never adds or removes an entity, and
/// [`hammer`] iterates collections it took before the first call, so
/// the count is `trials × TEARS.len() ×` this and does not depend on
/// the draw. Asserted rather than floored, for the reason
/// [`SPENT_GRAFT_EXPOSURE`] gives.
#[cfg(not(debug_assertions))]
const TORN_SWEEP_CALLS_PER_ROUND: usize = 1_928;

#[cfg(not(debug_assertions))]
const CALLS: &str = "operator calls";

/// The bodies the TORN SWEEP is run over, by name. (The spent-graft row
/// builds its own destination directly and reads nothing here.)
///
/// **The cube alone cannot get `kemr` past its plan phase, and no
/// amount of tearing changes that.** `kemr` wants the two halves of one
/// edge lying in one loop; every edge of a closed cube borders two
/// distinct faces, so its halves sit in two loops and the plan phase
/// refuses at `NotSameLoop` — a structural fact about the fixture, not
/// a sampling shortfall, which is why the gap survived every `kemr`
/// call a sweep of torn cubes could make. The other two present the two
/// shapes of its mutation phase: [`crate::fixtures::ops_ring_bridge`]
/// (the holed box with its hole rim joined back into the top face's
/// outer loop by one `mekr`) splits a non-empty side off a non-empty
/// side and runs both splices, and [`crate::fixtures::ops_strut_cube`]
/// (a pendant strut on that same loop) empties the ring side and runs
/// one.
///
/// **Each is floored by name** ([`fixture_swept`]), the way the tear
/// KINDS are: this is an enumerated dimension, and an aggregate over it
/// would let a body silently leave the sweep while the others carried
/// the floors.
///
/// Untorn, they are also the valid bodies the kill anchors' over-refusal
/// row sweeps ([`valid_fixtures_never_refuse_a_kill_anchor`]).
const FIXTURES: [(&str, BuildFixture); 3] = [
    ("declined_cube", |tol| declined_cube::<f64>(tol).body),
    ("ops_ring_bridge", |tol| ops_ring_bridge(tol).body),
    ("ops_strut_cube", |tol| ops_strut_cube(tol).body),
];

/// How a [`FIXTURES`] entry builds its body.
type BuildFixture = fn(Tol) -> Body<f64>;

/// A segment and a circle, the bodies whose kills empty a loop, each
/// beside a lone vertex: another loop's `Empty` vertex, for a kill's
/// `Empty` write to be told apart from, and for a torn start to land on.
const BESIDE_A_LONE_VERTEX: [(&str, BuildFixture); 2] = [
    ("segment beside a lone vertex", |tol| {
        let mut body = Body::new();
        let seed = body.mvfs(p(0.0), true).unwrap();
        let site = MevSite::Lone {
            r#loop: seed.r#loop,
        };
        body.mev_line(site, p(1.0), tol).unwrap();
        body.mvfs(p(5.0), true).unwrap();
        body
    }),
    ("circle beside a lone vertex", |tol| {
        let mut body = Body::new();
        let seed = body.mvfs(p(0.0), true).unwrap();
        let site = MefSite::Lone {
            r#loop: seed.r#loop,
        };
        body.mef_chord(site, tol).unwrap();
        body.mvfs(p(5.0), true).unwrap();
        body
    }),
];

/// A [`FIXTURES`] entry's exposure category: a body of that fixture the
/// sweep actually hammered.
#[cfg(not(debug_assertions))]
fn fixture_swept(name: &str) -> String {
    format!("fixture swept: {name}")
}

/// A [`Tear`] kind's exposure category: a planting of that kind that
/// actually changed the body.
///
/// **The floor the CALLS one cannot be.** The call count is a function
/// of the key enumeration alone ([`TORN_SWEEP_CALLS_PER_ROUND`]) and is
/// identical whether every planting landed or none did, so a sweep that
/// corrupted nothing hits it exactly while hammering intact bodies —
/// the vacuity this row is most exposed to. Counted per KIND rather
/// than per trial: a single draw may legitimately find no eligible
/// entity for the kind it was asked to plant, so *every trial
/// corrupted* is a claim about luck, while *every corruption shape was
/// planted somewhere* is the claim the sweep's own docs make —
/// [`TEARS`] is an enumeration, and every kind is meant to be
/// exercised on every run.
#[cfg(not(debug_assertions))]
fn tear_landed(tear: Tear) -> String {
    format!("tear landed: {tear:?}")
}

/// One kill at `he` through whichever door runs it. The keys-only door
/// refuses in its plan phase wherever the merge would re-base a
/// certified member ([`EulerOpError::MergeRebasesCarriers`]) — on these
/// fixtures, every kill whose far vertex carries a fan — so where it
/// does, the same kill is driven again through [`Body::kev_describing`]
/// with every merged member re-described as its chord
/// ([`crate::seqgen::try_chord_redescriptions`]). The first door's
/// refusal leaves `body` untouched, so the second runs on the same one.
fn kev_either_door(
    body: &mut Body<f64>,
    he: HalfEdgeKey,
    tol: Tol,
) -> Result<crate::KevResult, EulerOpError> {
    match body.kev(he) {
        Err(refusal @ EulerOpError::MergeRebasesCarriers { .. }) => {
            match crate::seqgen::try_chord_redescriptions(body, he) {
                Some(chords) => body.kev_describing(he, &chords, tol),
                None => Err(refusal),
            }
        }
        outcome => outcome,
    }
}

/// Whether a kill at `he` ran [`Body::kev`]'s mutation phase, which
/// both kill doors share ([`kev_either_door`]): the arms under attack
/// sit behind the keys-only door's certified-member refusal, and a
/// plan-phase refusal would otherwise be the whole of what the pass
/// reaches there. One call to the census either way, since it is one
/// kill.
#[cfg(not(debug_assertions))]
fn kill_reaches_its_mutation_phase(body: &Body<f64>, he: HalfEdgeKey, tol: Tol) -> bool {
    kev_either_door(&mut body.clone(), he, tol).is_ok()
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
/// of the spent destination and `mfkrh_plug` still returns `Ok` at
/// every loop it is handed — seven times on today's destination, which
/// is the figure [`SPENT_GRAFT_EXPOSURE`] holds — so a floor on the
/// total is one almost nothing can break. [`LINK_OPS`] is therefore
/// what the floor counts over, and `mfkrh_plug` is not in it.
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
/// substituted so the refusal paths keep their attack. Each mate call
/// that lands is also counted by ARM ([`KEMR_CYCLE_RING`],
/// [`KEMR_EMPTY_RING`]), because the two write a different number of
/// splices.
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
    // The two arms of `kemr`'s mutation phase, tallied beside the
    // closure below and folded in after it: `note` holds the census.
    let mut cycle_ring = 0usize;
    let mut empty_ring = 0usize;
    let mut note = |op: &str, ok: bool| {
        census.note(CALLS);
        if ok {
            census.note(op);
        }
    };
    for &he in &halves {
        note("kef", body.clone().kef(he).is_ok());
        note("kev", kill_reaches_its_mutation_phase(body, he, tol));
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
        let (hp, hm) = mate_halves(body, edge);
        for (he1, he2) in [(hp, hm), (hm, hp)] {
            let mut trial = body.clone();
            let out = trial.kemr(he1, he2);
            note("kemr", out.is_ok());
            if let Ok(out) = out {
                match trial.get_loop(out.ring).map(|l| l.boundary) {
                    Some(crate::LoopBoundary::Cycle { .. }) => cycle_ring += 1,
                    _ => empty_ring += 1,
                }
            }
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
        note("mfkrh_plug", body.clone().mfkrh_plug(l, true).is_ok());
    }
    census.add(KEMR_CYCLE_RING, cycle_ring);
    census.add(KEMR_EMPTY_RING, empty_ring);
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
/// `kemr`'s coverage carries its own controls, each run against the
/// others held fixed: point the mate-pair calls in [`hammer`] at a
/// half's own key instead of its mate — the same number of calls — and
/// `kemr` returns to 0 with both fixtures still swept, red at the named
/// `kemr` floor; drop [`crate::fixtures::ops_ring_bridge`] or
/// [`crate::fixtures::ops_strut_cube`] from [`FIXTURES`] and the arm
/// only it reaches returns to 0, red at that arm's floor. So the
/// enumeration and each fixture are necessary, and none is decoration
/// on the others.
///
/// The fixture floor carries its own control too: leave a [`FIXTURES`]
/// entry in place but never hammer it and this row reds by that
/// fixture's name.
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
            for (fixture, build) in FIXTURES {
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
                let round = hammer(&body, tol);
                // What this BODY put under the hammer, kept apart from
                // the sweep's total so no other fixture can carry it.
                census.add(&fixture_swept(fixture), round.count(CALLS));
                census.merge(&round);
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
    census.require_each(
        &landed,
        1,
        &format!(
            "a corruption shape never landed on any body in the whole sweep, so the \
             hammer below ran on bodies that shape had not torn — {}",
            fuzz::replay()
        ),
    );
    // AND EVERY BODY, by name, for the same reason and derived the same
    // way. [`FIXTURES`] is an enumerated dimension too, and an aggregate
    // over it would let one body go silent — the cube, say, on which
    // every row in this file was written — while the others kept every
    // other floor green.
    //
    // Derived from the enumeration, so it cannot drift from it; what
    // that cannot see is an entry DELETED from [`FIXTURES`], since the
    // floor list shrinks with it. The exact call identity below is what
    // catches that: it is pinned to a constant rather than to the
    // enumeration's length, so a body leaving the sweep reds it.
    let swept: Vec<String> = FIXTURES.iter().map(|(n, _)| fixture_swept(n)).collect();
    let swept: Vec<&str> = swept.iter().map(String::as_str).collect();
    census.require_each(
        &swept,
        1,
        &format!(
            "a fixture never reached the hammer, so the shapes only it presents were \
             swept by nothing — {}",
            fuzz::replay()
        ),
    );
    // The KEY ENUMERATION, and only that; the tear floor above carries
    // the other half. EXACT rather than floored: the count is a function
    // of the dial and the fixtures, never of the draw
    // ([`TORN_SWEEP_CALLS_PER_ROUND`]).
    assert_eq!(
        census.count(CALLS),
        trials * TEARS.len() * TORN_SWEEP_CALLS_PER_ROUND,
        "the key enumeration has moved — the bodies no longer present the half-edges, \
         edges and loops this sweep hammers, or the hammer no longer drives them all: \
         {census} — {}",
        fuzz::replay()
    );
    // EVERY link-reaching operator reaches a mutation phase here, and
    // each is floored BY NAME. An aggregate with slack in it cannot see
    // one operator go silent — which is not hypothetical: `kemr` sat at
    // 0 for this row's whole life under an aggregate floor with slack in
    // it, and the operators that did reach carried that floor.
    //
    // The floor is 1 because the headroom is enormous, not because one
    // is a lot: at the floor of the effort dial, across six seeds, the
    // smallest of these was `kemr` at ~90 and every other was in the
    // thousands (the exposure line prints the run's own figures).
    //
    // This is NOT a coverage target. When a legitimate plan-phase change
    // takes an operator out of reach, the repair is to re-derive the
    // list and say in the PR what moved and why the sweep is still
    // attacking the arms — not to add slack back so the row cannot see
    // it.
    census.require_each(
        &LINK_OPS,
        1,
        &format!(
            "an operator reached no mutation phase, so the arms under attack in it \
             were never in scope and this green says nothing about them — {}",
            fuzz::replay()
        ),
    );
    // And `kemr`'s two arms apart, since they write a different number
    // of splices: the bridge fixture reaches the two-splice one, the
    // strut fixture the one-splice one, and a count of `kemr` alone
    // cannot tell a sweep that lost one of them from one that did not.
    census.require_each(
        &[KEMR_CYCLE_RING, KEMR_EMPTY_RING],
        1,
        &format!(
            "an arm of `kemr`'s mutation phase was never entered, so the splices only \
             it runs are attacked by nothing — {}",
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
    let cube = declined_cube::<f64>(tol);
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
    // THE EXACT TALLY, which this row alone can carry and which is the
    // whole floor here: the row is deterministic, so every operator's
    // exposure is a fact about the tree rather than about a draw, and a
    // per-operator number is the only form in which the module doc's
    // coverage claim is checkable at all. It sees `kemr` falling from
    // its 2 back to 0, and a change that quietly halves `mef_chord`.
    //
    // No aggregate floor beside it: `require_nonzero_among(&LINK_OPS,
    // …)` over the same census asserts strictly less than this loop
    // does — every entry below is nonzero but `kemr`'s empty-ring arm,
    // which no input on this destination reaches — so it would be a
    // guard nothing could break alone.
    //
    // The floor a spent graft destination needs is a per-operator one
    // for the reason the twin row gives: it is more structurally damaged
    // than a randomly torn body, so it is the likelier of the two to
    // have its calls refuse in a plan phase — the run that proves
    // nothing about the arms under attack while passing green.
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

/// A face whose outer loop is `Empty` beside a cycle ring
/// ([`crate::fixtures::ops_strutted`], the strut killed from its tip by
/// `kemr`): the one valid body here that offers `mekr` an `Empty`
/// target.
const RING_ABOUT_AN_EMPTY_OUTER: (&str, BuildFixture) = ("ring about an empty outer", |tol| {
    let (mut body, _, _, strut) = crate::fixtures::ops_strutted(tol);
    body.kemr(strut.he_minus, strut.he_plus).unwrap();
    body
});

/// The operators whose loop and vertex anchors [`kill_anchor_rows`]
/// measures and [`valid_fixtures_never_refuse_a_kill_anchor`] sweeps:
/// the kills, and the makes that move a walked run.
const ANCHOR_OPS: [&str; 7] = ["kef", "kemr", "kev", "mef", "mekr", "kvfs", "kfmrh"];

/// One call of an [`ANCHOR_OPS`] operator ([`anchor_calls`]).
#[derive(Clone, Copy, Debug)]
enum AnchorCall {
    Kef(HalfEdgeKey),
    /// `kemr` at the mate pair.
    Kemr(HalfEdgeKey, HalfEdgeKey),
    /// `kev` through [`kev_either_door`].
    Kev(HalfEdgeKey),
    /// `mef_chord` at [`MefSite::Chords`].
    Mef(HalfEdgeKey, HalfEdgeKey),
    /// `mekr_chord`, at any site.
    Mekr(MekrSite),
    Kvfs(SolidKey),
    /// `kfmrh(f1, f2)`.
    Kfmrh(FaceKey, FaceKey),
}

impl AnchorCall {
    /// The call's row in [`ANCHOR_OPS`].
    fn op(self) -> usize {
        match self {
            Self::Kef(_) => 0,
            Self::Kemr(..) => 1,
            Self::Kev(_) => 2,
            Self::Mef(..) => 3,
            Self::Mekr(_) => 4,
            Self::Kvfs(_) => 5,
            Self::Kfmrh(..) => 6,
        }
    }

    fn run(self, body: &mut Body<f64>, tol: Tol) -> Result<(), EulerOpError> {
        match self {
            Self::Kef(he) => body.kef(he).map(|_| ()),
            Self::Kemr(he, m) => body.kemr(he, m).map(|_| ()),
            Self::Kev(he) => kev_either_door(body, he, tol).map(|_| ()),
            Self::Mef(he1, he2) => body
                .mef_chord(MefSite::Chords { he1, he2 }, tol)
                .map(|_| ()),
            Self::Mekr(site) => body.mekr_chord(site, tol).map(|_| ()),
            Self::Kvfs(solid) => body.kvfs(solid).map(|_| ()),
            Self::Kfmrh(f1, f2) => body.kfmrh(f1, f2).map(|_| ()),
        }
    }

    /// The call through its operator's other door, where it has one
    /// that runs the same plan: `kef_minting`, `kev_describing` with
    /// every merged member re-described as its chord, `mekr` handed the
    /// chord `mekr_chord` derives, and `kfmrh_minting`. `None` for an
    /// operator with one door.
    fn run_twin(self, body: &mut Body<f64>, tol: Tol) -> Option<Result<(), EulerOpError>> {
        Some(match self {
            Self::Kef(he) => body.kef_minting(he, tol).map(|_| ()),
            Self::Kev(he) => {
                let chords = crate::seqgen::try_chord_redescriptions(body, he).unwrap_or_default();
                body.kev_describing(he, &chords, tol).map(|_| ())
            }
            Self::Mekr(site) => {
                let chord = mekr_site_chord(body, site)?;
                body.mekr(site, chord, tol).map(|_| ())
            }
            Self::Kfmrh(f1, f2) => body.kfmrh_minting(f1, f2, tol).map(|_| ()),
            Self::Kemr(..) | Self::Mef(..) | Self::Kvfs(_) => return None,
        })
    }
}

/// The chord `mekr_chord` derives at `site`: the line between its two
/// anchor vertices' points, or the scaffolding circle where both are one
/// vertex. `None` where an anchor does not resolve.
fn mekr_site_chord(body: &Body<f64>, site: MekrSite) -> Option<geom_brep::EdgeCurveSpec<f64>> {
    let start = |he| body.get_half_edge(he).map(|h| h.start);
    let lone = |l| match body.get_loop(l)?.boundary {
        LoopBoundary::Empty { vertex } => Some(vertex),
        LoopBoundary::Cycle { .. } => None,
    };
    let (u, w) = match site {
        MekrSite::Cycles { target, ring } => (start(target)?, start(ring)?),
        MekrSite::EmptyRing { target, ring } => (start(target)?, lone(ring)?),
        MekrSite::EmptyTarget { target, ring } => (lone(target)?, start(ring)?),
        MekrSite::BothEmpty { target, ring } => (lone(target)?, lone(ring)?),
    };
    let point = |v| crate::readback::vertex_point(body, v).ok();
    Some(if u == w {
        geom_brep::EdgeCurveSpec::self_loop_circle_at(point(u)?)
    } else {
        geom_brep::EdgeCurveSpec::line_between(point(u)?, point(w)?)
    })
}

/// Every [`AnchorCall`] on `body`, as its keys read: each kill at every
/// half-edge (`kemr` at the mate pair); `mef` at every ordered pair of
/// distinct half-edges that claim one loop; `mekr` with every half-edge
/// as the ring, at every half-edge of another loop of the ring's face
/// ([`MekrSite::Cycles`]) and every `Empty` loop of it
/// ([`MekrSite::EmptyTarget`]), and with every `Empty` loop as the ring,
/// at every half-edge of another loop of its face
/// ([`MekrSite::EmptyRing`]) and every other `Empty` loop of it
/// ([`MekrSite::BothEmpty`]); `kvfs` at every solid; and `kfmrh` at
/// every ordered pair of distinct faces.
fn anchor_calls(body: &Body<f64>) -> Vec<AnchorCall> {
    let halves: Vec<(HalfEdgeKey, crate::entity::HalfEdge)> =
        body.half_edges().map(|(k, h)| (k, h.clone())).collect();
    let face_of = |l| body.get_loop(l).map(|data| data.face);
    let mut calls = Vec::new();
    for (ring, ring_data) in body.loops() {
        if !matches!(ring_data.boundary, LoopBoundary::Empty { .. }) {
            continue;
        }
        for (target, other) in &halves {
            if other.parent_loop != ring && face_of(other.parent_loop) == Some(ring_data.face) {
                calls.push(AnchorCall::Mekr(MekrSite::EmptyRing {
                    target: *target,
                    ring,
                }));
            }
        }
        for (target, other) in body.loops() {
            if target != ring
                && other.face == ring_data.face
                && matches!(other.boundary, LoopBoundary::Empty { .. })
            {
                calls.push(AnchorCall::Mekr(MekrSite::BothEmpty { target, ring }));
            }
        }
    }
    for (he, data) in &halves {
        calls.push(AnchorCall::Kef(*he));
        if let Some(m) = body.mate(*he) {
            calls.push(AnchorCall::Kemr(*he, m));
        }
        calls.push(AnchorCall::Kev(*he));
        for (he2, other) in &halves {
            if he2 != he && other.parent_loop == data.parent_loop {
                calls.push(AnchorCall::Mef(*he, *he2));
            }
        }
        let Some(face) = face_of(data.parent_loop) else {
            continue;
        };
        for (target, other) in &halves {
            if other.parent_loop != data.parent_loop && face_of(other.parent_loop) == Some(face) {
                calls.push(AnchorCall::Mekr(MekrSite::Cycles {
                    target: *target,
                    ring: *he,
                }));
            }
        }
        for (target, loop_data) in body.loops() {
            if loop_data.face == face && matches!(loop_data.boundary, LoopBoundary::Empty { .. }) {
                calls.push(AnchorCall::Mekr(MekrSite::EmptyTarget {
                    target,
                    ring: *he,
                }));
            }
        }
    }
    calls.extend(body.solids().map(|(solid, _)| AnchorCall::Kvfs(solid)));
    for (f1, _) in body.faces() {
        for (f2, _) in body.faces() {
            if f1 != f2 {
                calls.push(AnchorCall::Kfmrh(f1, f2));
            }
        }
    }
    calls
}

/// An `Empty` ring beside a cycle outer loop
/// ([`crate::fixtures::ops_strutted`], the strut killed from its root by
/// `kemr`): the valid body that offers `mekr` an `Empty` ring.
const EMPTY_RING_BESIDE_A_CYCLE: (&str, BuildFixture) = ("empty ring beside a cycle", |tol| {
    let (mut body, _, _, strut) = crate::fixtures::ops_strutted(tol);
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    body
});

/// Two `Empty` loops of one face ([`crate::fixtures::ops_segment`], the
/// segment killed by `kemr`): the valid body that offers `mekr` two
/// `Empty` loops.
const TWO_EMPTY_LOOPS: (&str, BuildFixture) = ("two empty loops", |tol| {
    let (mut body, _, seg) = crate::fixtures::ops_segment(tol);
    body.kemr(seg.he_plus, seg.he_minus).unwrap();
    body
});

/// A pillow and a detached digon distributed by `movefac` into two
/// shells of one solid ([`crate::fixtures::detached_digons`]): the valid
/// body that offers `kfmrh` its fusion form.
const TWO_SHELLS_OF_ONE_SOLID: (&str, BuildFixture) = ("two shells of one solid", |_| {
    let (mut body, shell, _, _) = crate::fixtures::detached_digons(1);
    body.movefac(shell).unwrap();
    body
});

/// Null scaffolding on the declined cube ([`Body::mev_null`]): a null
/// strut at the seed vertex, and a null fan split there, whose kill
/// merges a fan and moves nothing.
const NULL_SCAFFOLDING: [(&str, BuildFixture); 2] = [
    ("null strut on the declined cube", |tol| {
        let cube = declined_cube::<f64>(tol);
        let mut body = cube.body;
        let he = body
            .get_vertex(cube.seed.vertex)
            .unwrap()
            .emanating
            .unwrap();
        let site = MevSite::Fan { he1: he, he2: he };
        body.mev_null(site, crate::NewVertexSide::Above).unwrap();
        body
    }),
    ("null fan split on the declined cube", |tol| {
        let cube = declined_cube::<f64>(tol);
        let mut body = cube.body;
        let he1 = body
            .get_vertex(cube.seed.vertex)
            .unwrap()
            .emanating
            .unwrap();
        let orbit = body.vertex_orbit(he1).unwrap();
        let site = MevSite::Fan {
            he1,
            he2: orbit[orbit.len() - 1],
        };
        body.mev_null(site, crate::NewVertexSide::Above).unwrap();
        body
    }),
];

/// Marks every face that has a ring as a null face, through the door:
/// its outer loop above, its first ring below. The bodies whose kills
/// and moves read the null-face hygiene
/// ([`Body::drop_null_face_records_naming`]).
fn mark_ringed_faces(body: &mut Body<f64>) {
    let marks: Vec<(FaceKey, NullFacePair)> = body
        .faces()
        .filter_map(|(face, data)| {
            let below_loop = *data.rings.first()?;
            let above_loop = data.outer;
            Some((
                face,
                NullFacePair::Split {
                    above_loop,
                    below_loop,
                },
            ))
        })
        .collect();
    for (face, pair) in marks {
        body.set_null_face_pair(face, pair)
            .expect("a face's own outer loop and ring mark it");
    }
}

/// How an op that took `before` to `after` left `before`'s null-face
/// records: `[kept, dropped]`. A record stands exactly where both its
/// loops are still its face's own and falls exactly where one died or
/// left the face, so a drop of a record whose loops stay (an
/// over-reach) and a record left naming a loop gone from its face each
/// panic, naming `context`.
fn null_records_maintained(
    before: &Body<f64>,
    after: &Body<f64>,
    context: impl Fn() -> String,
) -> [usize; 2] {
    let mut counts = [0usize; 2];
    for (face, record) in before.null_faces() {
        let held = record.loops().iter().all(|&l| {
            after.get_loop(l).is_some_and(|data| data.face == face)
                && after
                    .get_face(face)
                    .is_some_and(|data| data.outer == l || data.rings.contains(&l))
        });
        match (held, after.null_face_pair(face)) {
            (true, Some(kept)) if kept == record => counts[0] += 1,
            (false, None) => counts[1] += 1,
            (held, left) => panic!(
                "{}: {face:?}'s record {record:?} with its loops held {held} is left {left:?}",
                context()
            ),
        }
    }
    assert!(
        after
            .null_faces()
            .all(|(face, _)| before.null_face_pair(face).is_some()),
        "{}: a record appeared",
        context()
    );
    counts
}

/// No over-refusal of the anchor, run and removal proofs: on every
/// valid body [`FIXTURES`], [`BESIDE_A_LONE_VERTEX`],
/// [`RING_ABOUT_AN_EMPTY_OUTER`], [`EMPTY_RING_BESIDE_A_CYCLE`],
/// [`TWO_EMPTY_LOOPS`], [`TWO_SHELLS_OF_ONE_SOLID`],
/// [`NULL_SCAFFOLDING`], the genus-2 body, the holed box and its
/// two-ring face build, every
/// [`anchor_calls`] call, through each door its operator has
/// ([`AnchorCall::run_twin`]), and `movefac` at every shell, refuses
/// nothing that reports a torn arena
/// ([`EulerOpError::reports_tier1_corruption`]). An enumeration, not a
/// sample. Every ringed face is marked as a null face
/// ([`mark_ringed_faces`]), and each `Ok`, and `mfkrh` and `ring_move`
/// (to every face of its shell, its own included) at every ring of a
/// marked face, keeps exactly the records whose loops stay on their
/// face ([`null_records_maintained`]). The two-ring face's second ring
/// is one no record names, so a drop keyed on the face rather than the
/// loop reds here.
///
/// Each proof sits late in its plan, so a sweep whose calls all refused
/// earlier would pass having asked none of them; the floors say each
/// operator ran to `Ok` somewhere, through each of its doors, `mekr` at
/// each of its four sites, `kfmrh` in its fusion form, `movefac`,
/// `kev` at a strut, the one kill whose anchor
/// its merged fan does not prove, and at a null edge, whose merge moves
/// nothing, and each kill emptied a loop somewhere, the write whose
/// proof reads every member: `kev` at the segment, `kef` at the circle
/// (the `Lone` inverse), and `kemr` at the strut from its tip; and a
/// record stood somewhere, and fell to `mekr`; and `mfkrh` and
/// `ring_move` each moved off a marked face a ring its record names and
/// one it does not, and a `ring_move` within a marked face ran.
#[test]
fn valid_fixtures_never_refuse_a_kill_anchor() {
    let tol = Tol::witness();
    let bodies: [(&str, BuildFixture); 14] = [
        FIXTURES[0],
        FIXTURES[1],
        FIXTURES[2],
        BESIDE_A_LONE_VERTEX[0],
        BESIDE_A_LONE_VERTEX[1],
        RING_ABOUT_AN_EMPTY_OUTER,
        EMPTY_RING_BESIDE_A_CYCLE,
        TWO_EMPTY_LOOPS,
        TWO_SHELLS_OF_ONE_SOLID,
        NULL_SCAFFOLDING[0],
        NULL_SCAFFOLDING[1],
        ("ops_genus2", ops_genus2),
        ("ops_holed_box", |tol| ops_holed_box(tol).body),
        ("ops_two_ring_face", |tol| ops_two_ring_face(tol).body),
    ];
    // Per operator: calls run to `Ok` through the first door, kills
    // that emptied a loop, and calls run to `Ok` through the twin.
    let mut ran = [[0usize; 3]; ANCHOR_OPS.len()];
    let mut kev_at_a_strut = 0usize;
    let mut kev_at_a_null_edge = 0usize;
    let mut mekr_sites = [0usize; 4];
    let mut kfmrh_fusions = 0usize;
    let mut movefacs = 0usize;
    // Null-face records kept and dropped by the anchor calls; the rings
    // `mfkrh` and `ring_move` moved off a marked face, [unnamed, named]
    // by its record; and the `ring_move`s within a marked face.
    let mut records = [0usize; 2];
    let mut moves = [[0usize; 2]; 2];
    let mut same_face_moves = 0usize;
    for (fixture, build) in bodies {
        let mut body = build(tol);
        mark_ringed_faces(&mut body);
        let body = body;
        assert_eq!(
            crate::validate::validate(&body),
            Ok(()),
            "{fixture} is valid"
        );
        for (shell, _) in body.shells() {
            match body.clone().movefac(shell) {
                Ok(_) => movefacs += 1,
                Err(refusal) if refusal.reports_tier1_corruption() => {
                    panic!("movefac({shell:?}) on the valid {fixture} refuses {refusal:?}")
                }
                Err(_) => {}
            }
        }
        for call in anchor_calls(&body) {
            let mut twin = body.clone();
            match call.run_twin(&mut twin, tol) {
                Some(Ok(())) => {
                    ran[call.op()][2] += 1;
                    null_records_maintained(&body, &twin, || {
                        format!("{call:?}'s twin on {fixture}")
                    });
                }
                Some(Err(refusal)) if refusal.reports_tier1_corruption() => {
                    panic!("{call:?}'s twin door on the valid {fixture} refuses {refusal:?}")
                }
                Some(Err(_)) | None => {}
            }
            let mut trial = body.clone();
            match call.run(&mut trial, tol) {
                Ok(()) => {
                    ran[call.op()][0] += 1;
                    let [kept, dropped] =
                        null_records_maintained(&body, &trial, || format!("{call:?} on {fixture}"));
                    records[0] += kept;
                    records[1] += dropped;
                    // The loop a kill empties where it empties one: the
                    // mate's for `kef`, whose own loop dies, else `he`'s.
                    let emptiable = match call {
                        AnchorCall::Kef(he) => body
                            .mate(he)
                            .and_then(|m| body.get_half_edge(m))
                            .map(|m| m.parent_loop),
                        AnchorCall::Kemr(he, _) | AnchorCall::Kev(he) => {
                            body.get_half_edge(he).map(|h| h.parent_loop)
                        }
                        _ => None,
                    };
                    let emptied = emptiable
                        .and_then(|l| trial.get_loop(l))
                        .is_some_and(|l| matches!(l.boundary, LoopBoundary::Empty { .. }));
                    ran[call.op()][1] += usize::from(emptied);
                    if let AnchorCall::Kev(he) = call {
                        let m = body.mate(he).expect("a valid body's half-edge has a mate");
                        let strut = body.vertex_orbit(m) == Some(vec![m])
                            && body.get_half_edge(m).map(|mate| mate.next) != Some(he);
                        kev_at_a_strut += usize::from(strut);
                        let edge = body.get_half_edge(he).unwrap().edge;
                        let curve = body.get_edge(edge).unwrap().curve;
                        let null = body
                            .get_curve_geom(curve)
                            .is_some_and(|geom| geom.null_scaffold().is_some());
                        kev_at_a_null_edge += usize::from(null);
                    }
                    if let AnchorCall::Kfmrh(f1, f2) = call {
                        let shell = |f| body.get_face(f).map(|face| face.shell);
                        kfmrh_fusions += usize::from(shell(f1) != shell(f2));
                    }
                    if let AnchorCall::Mekr(site) = call {
                        mekr_sites[match site {
                            MekrSite::Cycles { .. } => 0,
                            MekrSite::EmptyRing { .. } => 1,
                            MekrSite::EmptyTarget { .. } => 2,
                            MekrSite::BothEmpty { .. } => 3,
                        }] += 1;
                    }
                }
                Err(refusal) if refusal.reports_tier1_corruption() => {
                    panic!("{call:?} on the valid {fixture} refuses {refusal:?}")
                }
                Err(_) => {}
            }
        }
        // The moves `anchor_calls` does not make, at every ring of a
        // marked face, named or not: `mfkrh` promoting it, and
        // `ring_move` to every face of its shell, its own included.
        for (face, record) in body.null_faces() {
            let data = body.get_face(face).expect("a record's face resolves");
            for &ring in &data.rings {
                let named = usize::from(record.loops().contains(&ring));
                let mut trial = body.clone();
                if trial.mfkrh_plug(ring, true).is_ok() {
                    null_records_maintained(&body, &trial, || {
                        format!("mfkrh({ring:?}) on {fixture}")
                    });
                    moves[0][named] += 1;
                }
                for (to, to_data) in body.faces() {
                    let mut trial = body.clone();
                    if to_data.shell == data.shell && trial.ring_move(ring, to).is_ok() {
                        null_records_maintained(&body, &trial, || {
                            format!("ring_move({ring:?}, {to:?}) on {fixture}")
                        });
                        if to == face {
                            same_face_moves += 1;
                        } else {
                            moves[1][named] += 1;
                        }
                    }
                }
            }
        }
    }
    assert!(
        records.iter().all(|&n| n > 0),
        "the anchor calls kept and dropped [{}, {}] null-face records on the valid bodies",
        records[0],
        records[1]
    );
    assert!(
        moves.iter().flatten().all(|&n| n > 0),
        "`mfkrh` and `ring_move` moved [unnamed, named] rings {moves:?} off a marked face \
         on the valid bodies"
    );
    assert!(
        same_face_moves > 0,
        "no `ring_move` within a marked face ran to Ok on the valid bodies"
    );
    for (op, [ok, emptied, twin]) in ANCHOR_OPS.iter().zip(ran) {
        assert!(ok > 0, "no `{op}` ran to Ok on the valid bodies: {ran:?}");
        assert!(
            !["kef", "kemr", "kev"].contains(op) || emptied > 0,
            "no `{op}` emptied a loop on the valid bodies: {ran:?}"
        );
        assert!(
            !["kef", "kev", "mekr", "kfmrh"].contains(op) || twin > 0,
            "no `{op}` ran to Ok through its twin door on the valid bodies: {ran:?}"
        );
    }
    assert!(
        kfmrh_fusions > 0,
        "no `kfmrh` ran to Ok in its fusion form on the valid bodies"
    );
    assert!(movefacs > 0, "no `movefac` ran to Ok on the valid bodies");
    assert!(
        kev_at_a_strut > 0,
        "no `kev` ran to Ok at a strut on the valid bodies"
    );
    assert!(
        kev_at_a_null_edge > 0,
        "no `kev` ran to Ok at a null edge on the valid bodies"
    );
    assert!(
        mekr_sites.iter().all(|&n| n > 0),
        "`mekr` ran to Ok at [Cycles, EmptyRing, EmptyTarget, BothEmpty] {mekr_sites:?} \
         times on the valid bodies"
    );
}

/// The tears [`kill_anchor_rows`] plants: a live-but-foreign `next`,
/// the step every kill anchor is read through and every cycle walk
/// takes; an edge that claims a foreign half, which gives a kill a mate
/// whose own edge is another; a foreign start, which puts the vertex a
/// kill anchors or empties a loop at on another vertex; a foreign
/// `parent_loop`, which puts a member a plan anchors at or walks in
/// another loop; and a foreign reference of every other kind a record
/// a kill removes is named by: a half-edge's `edge` and `prev`, a
/// loop's `Empty` vertex, `first` and `face`, a vertex's `emanating`, a
/// face's `rings`, `outer` and `shell`, a shell's `faces` and `solid`,
/// and a solid's `shells`.
const ANCHOR_TEARS: [Tear; 16] = [
    Tear::NextForeign,
    Tear::EdgeBijection,
    Tear::StartForeign,
    Tear::ParentLoopForeign,
    Tear::EdgeForeign,
    Tear::EmptyVertexForeign,
    Tear::LoopFaceForeign,
    Tear::RingForeign,
    Tear::OuterForeign,
    Tear::FaceShellForeign,
    Tear::ShellFacesForeign,
    Tear::ShellSolidForeign,
    Tear::SolidShellsForeign,
    Tear::PrevForeign,
    Tear::LoopAnchorForeign,
    Tear::EmanatingForeign,
];

/// One [`kill_anchor_rows`] cell per [`ANCHOR_OPS`] operator: calls,
/// `Err`, and the `Ok` results that wrote each of [`ANCHOR_COLUMNS`].
type AnchorRows = [[usize; 2 + ANCHOR_COLUMNS.len()]; ANCHOR_OPS.len()];

/// [`kill_anchor_rows`] for each tear kind of [`ANCHOR_TEARS`].
type AnchorTable = [AnchorRows; ANCHOR_TEARS.len()];

/// The anchor proofs' tear measurement under one tear kind: for each
/// seed, one and two tears on every [`FIXTURES`] body, the genus-2 body,
/// the holed box, [`BESIDE_A_LONE_VERTEX`],
/// [`RING_ABOUT_AN_EMPTY_OUTER`], [`EMPTY_RING_BESIDE_A_CYCLE`],
/// [`TWO_EMPTY_LOOPS`] and [`TWO_SHELLS_OF_ONE_SOLID`], each ringed
/// face marked first ([`mark_ringed_faces`]), then every
/// [`anchor_calls`] call, each on a
/// clone. An `Ok` counts in a fault column where it leaves a
/// [`kill_anchor_faults`] fault the tear did not plant, which is one the
/// operator wrote. Each call runs inside a surgery scope, so a debug
/// build's tier-1 postcondition, which a torn input fails whatever the
/// operator writes, does not answer first; under the scalpel, whose
/// sweep answers after the operator's last write, a fired sweep counts
/// as the `Ok` it stood in front of ([`through_the_scalpel`]).
fn kill_anchor_rows(tear: Tear, seeds: &[u64]) -> AnchorRows {
    use test_utils::fuzz::Rng;
    let tol = Tol::witness();
    let bodies: [(&str, BuildFixture); 11] = [
        FIXTURES[0],
        FIXTURES[1],
        FIXTURES[2],
        ("ops_genus2", ops_genus2),
        ("ops_holed_box", |tol| ops_holed_box(tol).body),
        BESIDE_A_LONE_VERTEX[0],
        BESIDE_A_LONE_VERTEX[1],
        RING_ABOUT_AN_EMPTY_OUTER,
        EMPTY_RING_BESIDE_A_CYCLE,
        TWO_EMPTY_LOOPS,
        TWO_SHELLS_OF_ONE_SOLID,
    ];
    let mut table = [[0usize; 2 + ANCHOR_COLUMNS.len()]; ANCHOR_OPS.len()];
    for &seed in seeds {
        for tears in [1, 2] {
            for (_, build) in bodies {
                let mut body = build(tol);
                mark_ringed_faces(&mut body);
                let mut rng = Rng::from_seed(seed);
                for _ in 0..tears {
                    plant(&mut body, tear, &mut rng, HalfEdgeKey::default());
                }
                let planted = kill_anchor_faults(&body);
                for call in anchor_calls(&body) {
                    let cells = &mut table[call.op()];
                    let mut trial = body.clone();
                    let doors: &[&str] = match call {
                        AnchorCall::Kev(_) => &["kev", "kev_describing"],
                        _ => &ANCHOR_OPS[call.op()..=call.op()],
                    };
                    let mut scope = trial.begin_surgery();
                    let outcome =
                        through_the_scalpel(doors, || call.run(&mut scope, tol)).unwrap_or(Ok(()));
                    drop(scope);
                    cells[0] += 1;
                    if outcome.is_err() {
                        cells[1] += 1;
                        continue;
                    }
                    let mut columns = [false; ANCHOR_COLUMNS.len()];
                    for fault in kill_anchor_faults(&trial) {
                        if planted.contains(&fault) {
                            continue;
                        }
                        let column = match fault {
                            KillAnchorFault::AnchorOff(_) => 0,
                            KillAnchorFault::NoneWithEdges(_) => 1,
                            KillAnchorFault::LoopOff(_)
                            | KillAnchorFault::HeldTwice(_)
                            | KillAnchorFault::Orphan(_) => 2,
                            KillAnchorFault::DeadLoop(_) => 3,
                            KillAnchorFault::DeadStart(_) => 4,
                            KillAnchorFault::DeadFace(_) => 5,
                            KillAnchorFault::DeadShell(_) => 6,
                            KillAnchorFault::DeadSolid(_) => 7,
                            KillAnchorFault::DeadEdge(_) => 8,
                            KillAnchorFault::DeadHalfEdge(_) => 9,
                            KillAnchorFault::DeadNullFaceLoop(_) => 10,
                        };
                        columns[column] = true;
                    }
                    for (cell, written) in cells[2..].iter_mut().zip(columns) {
                        *cell += usize::from(written);
                    }
                }
            }
        }
    }
    table
}

/// The fault columns of an [`AnchorRows`] cell, after calls and `Err`.
const ANCHOR_COLUMNS: [&str; 11] = [
    "a vertex anchor off its vertex",
    "`None` on a vertex that keeps edges",
    "a loop anchor off its loop",
    "a dead loop left named",
    "a dead vertex left named",
    "a dead face left named",
    "a dead shell left named",
    "a dead solid left named",
    "a dead edge left named",
    "a dead half-edge left named",
    "a null-face record naming a dead loop",
];

/// Asserts every fault column of `table` is 0, naming the cell and
/// `context` otherwise.
fn assert_no_anchor_written(table: &AnchorTable, context: &str) {
    for (tear, rows) in ANCHOR_TEARS.iter().zip(table) {
        for (op, cells) in ANCHOR_OPS.iter().zip(rows) {
            for (column, &count) in ANCHOR_COLUMNS.iter().zip(&cells[2..]) {
                assert_eq!(
                    count, 0,
                    "`{op}` under {tear:?} wrote {column} through `Ok` ({context})"
                );
            }
        }
    }
}

/// The anchor proofs' tear measurement ([`kill_anchor_rows`]) on a few
/// seeds, asserting that no `Ok` writes an anchor fault: a
/// counterexample search, on the shared fuzz seed and effort dial.
#[test]
fn kill_anchors_on_a_few_torn_bodies() {
    let mut rng = test_utils::fuzz::start("kill_anchors_on_a_few_torn_bodies");
    let seeds: Vec<u64> = (0..test_utils::fuzz::scaled(2))
        .map(|_| rng.next_u64())
        .collect();
    let table = ANCHOR_TEARS.map(|tear| kill_anchor_rows(tear, &seeds));
    assert_no_anchor_written(&table, &test_utils::fuzz::replay());
}

/// **Evidence, not a gate**: [`kill_anchor_rows`] on seeds `1..=2000`,
/// run by hand, which prints the table and asserts every fault column
/// is 0.
///
/// `CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false cargo test --release -p
/// topo --lib -- --ignored --nocapture
/// review_d18::kill_anchors_on_torn_bodies`
#[test]
#[ignore = "evidence: the anchor proofs' tear measurement, run by hand"]
#[cfg(not(debug_assertions))]
fn kill_anchors_on_torn_bodies() {
    let range = 1..=2000_u64;
    let seeds: Vec<u64> = range.clone().collect();
    // Two tear kinds at a time, one thread each.
    let mut table: AnchorTable =
        [[[0usize; 2 + ANCHOR_COLUMNS.len()]; ANCHOR_OPS.len()]; ANCHOR_TEARS.len()];
    for (pair, rows) in ANCHOR_TEARS.chunks(2).zip(table.chunks_mut(2)) {
        std::thread::scope(|scope| {
            let handles: Vec<_> = pair
                .iter()
                .map(|&tear| {
                    let seeds = &seeds;
                    scope.spawn(move || kill_anchor_rows(tear, seeds))
                })
                .collect();
            for (row, handle) in rows.iter_mut().zip(handles) {
                *row = handle.join().unwrap();
            }
        });
    }
    println!(
        "| tear | op | calls | `Err` | `Ok`, {} |",
        ANCHOR_COLUMNS.join(" | `Ok`, ")
    );
    println!("| --- | --- |{}", " --- |".repeat(2 + ANCHOR_COLUMNS.len()));
    for (tear, rows) in ANCHOR_TEARS.iter().zip(&table) {
        for (op, cells) in ANCHOR_OPS.iter().zip(rows) {
            let cells: Vec<String> = cells.iter().map(ToString::to_string).collect();
            println!("| `{tear:?}` | `{op}` | {} |", cells.join(" | "));
        }
    }
    assert_no_anchor_written(&table, &format!("seeds {range:?}"));
}

/// The valid bodies [`revert_anchor_rows`] and [`revert_rename_rows`]
/// tear and
/// [`valid_fixtures_never_refuse_a_revert_anchor`] reverts: those
/// [`kill_anchor_rows`] tears, among them the holed box and the genus-2
/// body, whose faces carry rings, and the bodies with a lone vertex or
/// an `Empty` loop.
const REVERT_BODIES: [(&str, BuildFixture); 8] = [
    FIXTURES[0],
    FIXTURES[1],
    FIXTURES[2],
    ("ops_genus2", ops_genus2),
    ("ops_holed_box", |tol| ops_holed_box(tol).body),
    BESIDE_A_LONE_VERTEX[0],
    BESIDE_A_LONE_VERTEX[1],
    RING_ABOUT_AN_EMPTY_OUTER,
];

/// No over-refusal of `revert`'s start and anchor proofs: every valid
/// body [`REVERT_BODIES`] builds, the geometric cube, a planar block
/// with two through-holes, the pillows, a raw prism and the lone `mvfs`
/// seed reverts, and so does its reversal. An enumeration, not a
/// sample. Every body but the seed asks every proof something: it has
/// a half-edge, a vertex anchor and a cycle loop; the strut cube and
/// the segment hold half-edges whose mate is their own `next` or
/// `prev`.
#[test]
fn valid_fixtures_never_refuse_a_revert_anchor() {
    use crate::fixtures::{mvfs_state, ngon_pillow, pillow, raw_prism};
    use crate::test_support_fixtures::{geometric_cube, holed_block};
    let tol = Tol::witness();
    let more: [(&str, BuildFixture); 6] = [
        ("geometric_cube", |tol| geometric_cube::<f64>(tol).body),
        ("holed_block", |tol| {
            holed_block::<f64>(4.0, &[1.0, 3.0], tol)
        }),
        ("pillow", |tol| pillow(tol).body),
        ("ngon_pillow(5)", |tol| ngon_pillow(5, tol).body),
        ("raw_prism(3)", |tol| raw_prism(3, tol).body),
        ("mvfs_state", |_| mvfs_state().body),
    ];
    for (fixture, build) in REVERT_BODIES.into_iter().chain(more) {
        let body = build(tol);
        assert_eq!(
            crate::validate::validate(&body),
            Ok(()),
            "{fixture} is valid"
        );
        let anchored = body.vertices().any(|(_, v)| v.emanating.is_some());
        let cycled = body
            .loops()
            .any(|(_, l)| matches!(l.boundary, LoopBoundary::Cycle { .. }));
        assert!(
            fixture == "mvfs_state" || (anchored && cycled),
            "{fixture} asks both proofs something"
        );
        let reverted = body
            .revert()
            .unwrap_or_else(|e| panic!("the valid {fixture} refuses {e:?}"));
        reverted
            .revert()
            .unwrap_or_else(|e| panic!("the reversed {fixture} refuses {e:?}"));
    }
}

/// A link [`crate::Body::revert`] reads: every half-edge's `next` (its
/// new start is the end it derives), `prev` (a loop's new `first`) and
/// `start` (its predecessor's end, and its mate's end).
#[cfg(not(debug_assertions))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RevertTear {
    Next,
    Prev,
    Start,
}

#[cfg(not(debug_assertions))]
const REVERT_TEARS: [RevertTear; 3] = [RevertTear::Next, RevertTear::Prev, RevertTear::Start];

/// Every single `tear` of `body`: each half-edge's link set to each
/// half-edge (`next`, `prev`) or vertex (`start`) in turn, handed to
/// `visit` with the tear's image under the reversal: the intact body
/// reversed, with the torn link written into the field the map swaps
/// it to (`next` ↔ `prev`), which is the same fault under the other
/// name. A torn `start` has no image: the map reads it only as other
/// half-edges' ends.
#[cfg(not(debug_assertions))]
fn each_revert_tear(
    body: &Body<f64>,
    tear: RevertTear,
    mut visit: impl FnMut(Body<f64>, Option<Body<f64>>),
) {
    let reverted = body.revert().expect("the intact body reverts");
    let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(k, _)| k).collect();
    let vertices: Vec<VertexKey> = body.vertices().map(|(k, _)| k).collect();
    let targets = match tear {
        RevertTear::Next | RevertTear::Prev => halves.len(),
        RevertTear::Start => vertices.len(),
    };
    for &at in &halves {
        for target in 0..targets {
            let mut torn = body.clone();
            let he = torn.get_half_edge_mut(at).unwrap();
            let mut image = reverted.clone();
            let renamed = image.get_half_edge_mut(at).unwrap();
            let image = match tear {
                RevertTear::Next => {
                    he.next = halves[target];
                    renamed.prev = halves[target];
                    Some(image)
                }
                RevertTear::Prev => {
                    he.prev = halves[target];
                    renamed.next = halves[target];
                    Some(image)
                }
                RevertTear::Start => {
                    he.start = vertices[target];
                    None
                }
            };
            visit(torn, image);
        }
    }
}

/// Every single `tear` of `body`, then `revert` on it. Returns calls,
/// `Err`, and the `Ok` results carrying a [`kill_anchor_faults`] fault
/// the tear did not plant, which is one `revert` wrote. Release-only,
/// as the module docs say: a debug build's postcondition answers every
/// torn `Ok` first.
#[cfg(not(debug_assertions))]
fn revert_anchor_rows(body: &Body<f64>, tear: RevertTear) -> [usize; 3] {
    let mut cells = [0usize; 3];
    each_revert_tear(body, tear, |torn, _| {
        let planted = kill_anchor_faults(&torn);
        cells[0] += 1;
        match torn.revert() {
            Err(_) => cells[1] += 1,
            Ok(reverted) => {
                let wrote = kill_anchor_faults(&reverted)
                    .into_iter()
                    .any(|fault| !planted.contains(&fault));
                cells[2] += usize::from(wrote);
            }
        }
    });
    cells
}

/// **`revert` writes no anchor off a torn `next` or `prev`**: every
/// single tear of each [`REVERT_BODIES`] body, and no `Ok` carries an
/// anchor fault the tear did not plant. An enumeration, not a sample.
/// Each body, and each tear kind, is refused somewhere, so the tears
/// reach the proofs. (A `Prev` tear of a body with one cycle loop,
/// the segment, cannot move a `first` out of its loop.)
#[test]
#[cfg(not(debug_assertions))]
fn revert_writes_no_anchor_off_a_torn_next_or_prev() {
    let tol = Tol::witness();
    let tears = [RevertTear::Next, RevertTear::Prev];
    let rows: Vec<(&str, [[usize; 3]; 2])> = REVERT_BODIES
        .iter()
        .map(|&(name, build)| {
            let body = build(tol);
            (name, tears.map(|tear| revert_anchor_rows(&body, tear)))
        })
        .collect();
    println!("| body | tear | calls | `Err` | `Ok`, anchor fault written |");
    println!("| --- | --- | --- | --- | --- |");
    for (name, cells) in &rows {
        for (tear, [calls, refused, wrote]) in tears.iter().zip(cells) {
            println!("| {name} | `{tear:?}` | {calls} | {refused} | {wrote} |");
        }
    }
    let mut refused_per_tear = [0usize; 2];
    for (name, cells) in &rows {
        for ((tear, [calls, refused, wrote]), per_tear) in
            tears.iter().zip(cells).zip(&mut refused_per_tear)
        {
            assert_eq!(
                *wrote, 0,
                "`revert` wrote an anchor fault through `Ok` on {wrote} of {calls} `{tear:?}` tears of {name}"
            );
            *per_tear += refused;
        }
        assert!(
            cells.iter().any(|&[_, refused, _]| refused > 0),
            "no tear of {name} was refused"
        );
    }
    assert!(
        refused_per_tear.iter().all(|&n| n > 0),
        "refusals per tear kind {tears:?}: {refused_per_tear:?}"
    );
}

/// The topology links `revert` reads and writes, per entity in arena
/// order: what a torn result is compared with its tear's image on.
#[cfg(not(debug_assertions))]
fn revert_links(body: &Body<f64>) -> Vec<String> {
    let halves = body.half_edges().map(|(k, h)| format!("{k:?} {h:?}"));
    let edges = body
        .edges()
        .map(|(k, e)| format!("{k:?} {:?} {:?}", e.he_plus, e.he_minus));
    let vertices = body
        .vertices()
        .map(|(k, v)| format!("{k:?} {:?}", v.emanating));
    let loops = body.loops().map(|(k, l)| format!("{k:?} {:?}", l.boundary));
    halves.chain(edges).chain(vertices).chain(loops).collect()
}

/// The kinds of `validate`'s errors on `body`, by name.
#[cfg(not(debug_assertions))]
fn validator_kinds(body: &Body<f64>) -> BTreeSet<String> {
    crate::validate::validate(body)
        .err()
        .unwrap_or_default()
        .iter()
        .map(|e| format!("{:?}", crate::validate::ValidationErrorKind::from(e)))
        .collect()
}

/// The validator kinds a reversed torn body carries beyond its torn
/// source's, per tear kind, each on a result that is the tear's image
/// ([`each_revert_tear`]): the same link under the name the map swaps
/// it to. The validator walks a cycle by `next` and reads a vertex
/// orbit through `prev`, so a torn `prev` moved into `next` is reported
/// by other kinds than the same link in `prev`. A `next` or `start` tear
/// that `revert` does not refuse carries no new kind.
#[cfg(not(debug_assertions))]
const RENAMED_KINDS: [(RevertTear, &[&str]); 3] = [
    (RevertTear::Next, &[]),
    (
        RevertTear::Prev,
        &[
            "EdgeNotAntiparallel",
            "LoopCycleOverrun",
            "SplitVertexOrbit",
            "UnreachableHalfEdge",
            "VertexOrbitOverrun",
        ],
    ),
    (RevertTear::Start, &[]),
];

/// One [`revert_rename_rows`] tally.
#[cfg(not(debug_assertions))]
#[derive(Default)]
struct RenameCells {
    calls: usize,
    refused: usize,
    /// `Ok` results that are the tear's image and carry a kind the
    /// torn source does not.
    renamed: usize,
    /// Those kinds.
    renamed_kinds: BTreeSet<String>,
    /// `Ok` results that are not the tear's image, or, for a tear with
    /// no image, carry a kind the torn source does not: a fault
    /// `revert` wrote.
    written: usize,
}

/// Every single `tear` of `body`, then `revert` on it, tallied into
/// [`RenameCells`]. Release-only, as [`revert_anchor_rows`].
#[cfg(not(debug_assertions))]
fn revert_rename_rows(body: &Body<f64>, tear: RevertTear) -> RenameCells {
    let mut cells = RenameCells::default();
    each_revert_tear(body, tear, |torn, image| {
        cells.calls += 1;
        let Ok(reverted) = torn.revert() else {
            cells.refused += 1;
            return;
        };
        let source = validator_kinds(&torn);
        let new: BTreeSet<String> = validator_kinds(&reverted)
            .into_iter()
            .filter(|kind| !source.contains(kind))
            .collect();
        match image {
            Some(image) if revert_links(&image) != revert_links(&reverted) => cells.written += 1,
            None if !new.is_empty() => cells.written += 1,
            _ if !new.is_empty() => {
                cells.renamed += 1;
                cells.renamed_kinds.extend(new);
            }
            _ => {}
        }
    });
    cells
}

/// **`revert` writes no fault off a torn `next`, `prev` or `start`**:
/// every single tear of each [`REVERT_BODIES`] body, and every `Ok` of
/// a `next` or `prev` tear is the tear's image, and no `Ok` of a `start`
/// tear carries a validator kind its torn source does not. The kinds
/// the images carry beyond their sources are [`RENAMED_KINDS`], pinned
/// per tear kind. An enumeration, not a sample; each tear kind is
/// refused somewhere, so the tears reach the proofs.
#[test]
#[cfg(not(debug_assertions))]
fn revert_writes_no_fault_off_a_torn_next_prev_or_start() {
    let tol = Tol::witness();
    let rows: Vec<(&str, [RenameCells; 3])> = REVERT_BODIES
        .iter()
        .map(|&(name, build)| {
            let body = build(tol);
            (
                name,
                REVERT_TEARS.map(|tear| revert_rename_rows(&body, tear)),
            )
        })
        .collect();
    println!("| body | tear | calls | `Err` | `Ok`, renamed | `Ok`, written | renamed kinds |");
    println!("| --- | --- | --- | --- | --- | --- | --- |");
    for (name, cells) in &rows {
        for (tear, c) in REVERT_TEARS.iter().zip(cells) {
            println!(
                "| {name} | `{tear:?}` | {} | {} | {} | {} | {:?} |",
                c.calls, c.refused, c.renamed, c.written, c.renamed_kinds
            );
        }
    }
    let mut renamed_kinds: [BTreeSet<String>; 3] = Default::default();
    let mut refused_per_tear = [0usize; 3];
    for (name, cells) in &rows {
        for (((tear, c), kinds), refused) in REVERT_TEARS
            .iter()
            .zip(cells)
            .zip(&mut renamed_kinds)
            .zip(&mut refused_per_tear)
        {
            assert_eq!(
                c.written, 0,
                "`revert` wrote a fault through `Ok` on {} of {} `{tear:?}` tears of {name}",
                c.written, c.calls
            );
            kinds.extend(c.renamed_kinds.iter().cloned());
            *refused += c.refused;
        }
    }
    for ((tear, kinds), (pinned_tear, pinned)) in
        REVERT_TEARS.iter().zip(&renamed_kinds).zip(RENAMED_KINDS)
    {
        assert_eq!(*tear, pinned_tear);
        let pinned: BTreeSet<String> = pinned.iter().map(|k| (*k).to_string()).collect();
        assert_eq!(
            *kinds, pinned,
            "the kinds `{tear:?}` tears' images carry beyond their sources"
        );
    }
    assert!(
        refused_per_tear.iter().all(|&n| n > 0),
        "refusals per tear kind {REVERT_TEARS:?}: {refused_per_tear:?}"
    );
}

// ----------------------------------------------------------------------
// The removal proofs' rows: each kill removes a record another record
// still names. Every row is the measured witness of a filed row, or the
// constructed case it names, and every one returns `Ok` with the name
// left dangling where its proof is absent. Each runs through every door
// that runs its operator's plan, inside a surgery scope
// (`fixtures::assert_kill_refuses`), so it holds with debug assertions
// off as on; the release-profile job runs this module.
// ----------------------------------------------------------------------

/// Asserts that `kev(he)` refuses `torn` with the body deep-unchanged
/// through each door: `kev_describing` with every merged member
/// re-described as its chord where the plan reads members, which takes
/// the describing door past its gates to the writes wherever the plan
/// passes, then `kev`, and `kev_merged_members`.
fn assert_kev_refuses(body: &mut Body<f64>, he: HalfEdgeKey, torn: &EulerOpError) {
    let tol = Tol::witness();
    let chords = crate::seqgen::try_chord_redescriptions(body, he).unwrap_or_default();
    assert_kill_refuses(body, torn, |b| b.kev_describing(he, &chords, tol));
    assert_kill_refuses(body, torn, |b| b.kev(he));
    assert_eq!(body.kev_merged_members(he).map(|_| ()), Err(torn.clone()));
}

/// Asserts that `kef(he)` and `kef_minting(he, tol)` refuse `torn` with
/// the body deep-unchanged.
fn assert_kef_refuses(body: &mut Body<f64>, he: HalfEdgeKey, torn: &EulerOpError) {
    assert_kill_refuses(body, torn, |b| b.kef(he));
    assert_kill_refuses(body, torn, |b| b.kef_minting(he, Tol::witness()));
}

/// Asserts that `mekr_chord(site)` and `mekr` handed the same chord
/// refuse `torn` with the body deep-unchanged.
fn assert_mekr_refuses(body: &mut Body<f64>, site: MekrSite, torn: &EulerOpError) {
    let tol = Tol::witness();
    let chord = mekr_site_chord(body, site).expect("the site's anchors resolve");
    assert_kill_refuses(body, torn, |b| b.mekr_chord(site, tol));
    assert_kill_refuses(body, torn, |b| b.mekr(site, chord, tol));
}

/// The first half-edge in arena order that starts at `end(he)` and is
/// off the orbit `kev(he)` walks from the mate: the record the kill would
/// leave starting at its dead far vertex.
fn stray_at_the_far_vertex(body: &Body<f64>, he: HalfEdgeKey) -> HalfEdgeKey {
    let m = body.mate(he).unwrap();
    let w = body.get_half_edge(m).unwrap().start;
    let orbit = body.vertex_orbit(m).unwrap();
    body.half_edges()
        .find(|&(x, data)| data.start == w && !orbit.contains(&x))
        .map(|(x, _)| x)
        .expect("a half-edge off the walk starts at the far vertex")
}

fn arena_halves(body: &Body<f64>) -> Vec<HalfEdgeKey> {
    body.half_edges().map(|(k, _)| k).collect()
}

#[test]
fn kev_refuses_a_far_vertex_a_torn_next_hides_from_its_walk() {
    // `kill_anchors_on_torn_bodies`' first `NextForeign` witness: two
    // `next` tears close the far vertex's orbit early, so the walk from
    // the mate misses a half-edge that starts there.
    let mut body = ops_ring_bridge(Tol::witness()).body;
    let halves = arena_halves(&body);
    body.get_half_edge_mut(halves[5]).unwrap().next = halves[37];
    body.get_half_edge_mut(halves[11]).unwrap().next = halves[1];
    let he = halves[0];
    let stray = stray_at_the_far_vertex(&body, he);
    assert_kev_refuses(&mut body, he, &EulerOpError::OrbitBroken { he: stray });
}

#[test]
fn kev_refuses_a_far_vertex_a_torn_bijection_hides_from_its_walk() {
    // The strut cube with one `EdgeBijection` tear, `kev(halves[8])`:
    // the torn edge claims `halves[17]`, so the orbit walk through it
    // leaves the far vertex's fan short of a half-edge that starts there.
    let mut body = ops_strut_cube(Tol::witness()).body;
    let halves = arena_halves(&body);
    let edge = body.get_half_edge(halves[23]).unwrap().edge;
    let torn = body.get_edge_mut(edge).unwrap();
    (torn.he_plus, torn.he_minus) = (halves[23], halves[17]);
    let he = halves[8];
    let stray = stray_at_the_far_vertex(&body, he);
    assert_kev_refuses(&mut body, he, &EulerOpError::OrbitBroken { he: stray });
}

#[test]
fn kev_refuses_a_far_vertex_a_torn_start_puts_a_half_edge_on() {
    // A `StartForeign` tear on the declined cube: a half-edge of another
    // vertex torn to start at the far vertex, where no walk reaches it.
    let mut body = declined_cube::<f64>(Tol::witness()).body;
    let halves = arena_halves(&body);
    let vertices: Vec<_> = body.vertices().map(|(k, _)| k).collect();
    body.get_half_edge_mut(halves[0]).unwrap().start = vertices[1];
    let he = halves[3];
    assert_eq!(stray_at_the_far_vertex(&body, he), halves[0]);
    let torn = EulerOpError::OrbitBroken { he: halves[0] };
    assert_kev_refuses(&mut body, he, &torn);
}

#[test]
fn kev_refuses_a_far_vertex_a_torn_empty_loop_holds() {
    // A loop of the declined cube torn `Empty` at the far vertex: the
    // kill would leave it holding a dead vertex.
    let mut body = declined_cube::<f64>(Tol::witness()).body;
    let halves = arena_halves(&body);
    let vertices: Vec<_> = body.vertices().map(|(k, _)| k).collect();
    let (l, _) = body.loops().next().unwrap();
    body.get_loop_mut(l).unwrap().boundary = LoopBoundary::Empty {
        vertex: vertices[0],
    };
    let he = halves[1];
    let m = body.mate(he).unwrap();
    assert_eq!(body.get_half_edge(m).unwrap().start, vertices[0]);
    let torn = dangling(EntityId::Loop(l), EntityId::Vertex(vertices[0]));
    assert_kev_refuses(&mut body, he, &torn);
}

#[test]
fn kev_refuses_a_mate_whose_own_edge_is_another() {
    // An `EdgeBijection` tear on the declined cube: `halves[0]`'s edge
    // claims `halves[3]`, whose own edge is another. Unchecked, the kill
    // removes that edge's half, and the half its edge used to claim is
    // left naming the dead edge.
    let mut body = declined_cube::<f64>(Tol::witness()).body;
    let halves = arena_halves(&body);
    let (he, m) = (halves[0], halves[3]);
    let edge = body.get_half_edge(he).unwrap().edge;
    let torn = body.get_edge_mut(edge).unwrap();
    (torn.he_plus, torn.he_minus) = (he, m);
    assert_ne!(body.get_half_edge(m).unwrap().edge, edge);
    assert_kev_refuses(
        &mut body,
        he,
        &EulerOpError::NotSameEdge { he1: he, he2: m },
    );
}

#[test]
fn kef_refuses_a_mate_whose_own_edge_is_another() {
    // A theta: the digon pillow with its back face split by a chord
    // parallel to both edges. The front face's `a0` has its edge torn to
    // claim the chord's `v1 → v0` half, which lies in another face and
    // runs where `a0`'s mate runs, so every anchor the kill reads holds.
    // Unchecked, the kill removes the chord's half and `a0`'s edge, and
    // `a0`'s true mate is left naming the dead edge.
    let tol = Tol::witness();
    let pillow = crate::fixtures::pillow(tol);
    let mut body = pillow.body;
    let (a0, b0, b1) = (pillow.hes_a[0], pillow.hes_b[0], pillow.hes_b[1]);
    let chord = body
        .mef_chord(MefSite::Chords { he1: b0, he2: b1 }, tol)
        .unwrap();
    assert_eq!(crate::validate::validate(&body), Ok(()));
    let torn = body.get_edge_mut(pillow.edges[0]).unwrap();
    assert_eq!((torn.he_plus, torn.he_minus), (a0, b0));
    torn.he_minus = chord.he_plus;
    let refusal = EulerOpError::NotSameEdge {
        he1: a0,
        he2: chord.he_plus,
    };
    assert_kef_refuses(&mut body, a0, &refusal);
}

#[test]
fn kef_and_kev_refuse_an_edge_claiming_one_half_in_both_slots() {
    // Adopted from the kill-proof review's torn-slot probe: an edge of
    // the declined cube torn to claim `he` in both slots, so the mate the
    // kill reads from them is `he` itself. The pair check refuses the
    // torn bijection before the plan reads that mate's loop or start.
    let mut body = declined_cube::<f64>(Tol::witness()).body;
    let he = arena_halves(&body)[6];
    let edge = body.get_half_edge(he).unwrap().edge;
    let torn = body.get_edge_mut(edge).unwrap();
    (torn.he_plus, torn.he_minus) = (he, he);
    let refusal = EulerOpError::NotSameEdge { he1: he, he2: he };
    assert!(refusal.reports_tier1_corruption());
    assert_kef_refuses(&mut body, he, &refusal);
    assert_kev_refuses(&mut body, he, &refusal);
}

#[test]
fn kef_and_kev_refuse_a_third_half_edge_naming_the_killed_edge() {
    // A half-edge of the declined cube whose `edge` is torn to the edge
    // the kill removes. Unchecked, the kill leaves it naming the dead
    // edge.
    let tol = Tol::witness();
    let mut cube = declined_cube::<f64>(tol).body;
    let halves = arena_halves(&cube);
    let edge = cube.get_half_edge(halves[2]).unwrap().edge;
    cube.get_half_edge_mut(halves[0]).unwrap().edge = edge;
    let torn = EulerOpError::UnclaimedHalfEdge {
        he: halves[0],
        edge,
    };
    assert_kef_refuses(&mut cube, halves[2], &torn);
    assert_kev_refuses(&mut cube, halves[2], &torn);
}

#[test]
fn kemr_refuses_a_third_half_edge_naming_the_killed_edge() {
    // The strut cube with a half-edge's `edge` torn to the strut's, then
    // `kemr` at the strut. Unchecked, the kill leaves it naming the dead
    // edge.
    let fixture = ops_strut_cube(Tol::witness());
    let mut body = fixture.body;
    let (he1, he2) = (fixture.strut.he_plus, fixture.strut.he_minus);
    let stray = arena_halves(&body)[0];
    body.get_half_edge_mut(stray).unwrap().edge = fixture.strut.edge;
    let torn = EulerOpError::UnclaimedHalfEdge {
        he: stray,
        edge: fixture.strut.edge,
    };
    assert_kill_refuses(&mut body, &torn, |b| b.kemr(he1, he2));
}

#[test]
fn kef_refuses_a_dying_face_or_loop_another_record_names() {
    // The declined cube: a third loop's `face` torn to the dying face,
    // then a third face's `rings`, then its `outer`, torn to name the
    // dying loop, then another shell torn to list the dying face.
    let tol = Tol::witness();
    let build = || {
        let body = declined_cube::<f64>(tol).body;
        let he = arena_halves(&body)[6];
        let l1 = body.get_half_edge(he).unwrap().parent_loop;
        let l2 = body
            .get_half_edge(body.mate(he).unwrap())
            .unwrap()
            .parent_loop;
        let f1 = body.get_loop(l1).unwrap().face;
        let (third, _) = body.loops().find(|&(l, _)| l != l1 && l != l2).unwrap();
        (body, he, l1, f1, third)
    };
    let (mut body, he, _, f1, third) = build();
    body.get_loop_mut(third).unwrap().face = f1;
    let torn = EulerOpError::KillLeavesDangling {
        from: EntityId::Loop(third),
        to: EntityId::Face(f1),
    };
    assert_kef_refuses(&mut body, he, &torn);

    type Tear = fn(&mut crate::entity::Face, crate::entity::LoopKey);
    let tears: [Tear; 2] = [|f, l1| f.rings.push(l1), |f, l1| f.outer = l1];
    for tear in tears {
        let (mut body, he, l1, _, third) = build();
        let face = body.get_loop(third).unwrap().face;
        tear(body.get_face_mut(face).unwrap(), l1);
        let torn = EulerOpError::KillLeavesDangling {
            from: EntityId::Face(face),
            to: EntityId::Loop(l1),
        };
        assert_kef_refuses(&mut body, he, &torn);
    }

    let (mut body, he, _, f1, _) = build();
    let other = body.mvfs(p(9.0), true).unwrap();
    body.get_shell_mut(other.shell).unwrap().faces.push(f1);
    let torn = EulerOpError::KillLeavesDangling {
        from: EntityId::Shell(other.shell),
        to: EntityId::Face(f1),
    };
    assert_kef_refuses(&mut body, he, &torn);
}

/// A segment's solid beside a lone vertex's: the segment's `mvfs` keys,
/// and the lone solid's.
fn segment_beside_a_lone_solid() -> (
    Body<f64>,
    crate::euler::MvfsCreated,
    crate::euler::MvfsCreated,
) {
    let mut body = Body::new();
    let seed = body.mvfs(p(0.0), true).unwrap();
    let site = MevSite::Lone {
        r#loop: seed.r#loop,
    };
    body.mev_line(site, p(1.0), Tol::witness()).unwrap();
    let lone = body.mvfs(p(5.0), true).unwrap();
    (body, seed, lone)
}

/// A kill's refusal of a record it keeps naming one it removes.
fn dangling(from: EntityId, to: EntityId) -> EulerOpError {
    EulerOpError::KillLeavesDangling { from, to }
}

#[test]
fn kvfs_refuses_a_lone_record_another_record_names() {
    // A segment's solid beside a lone solid, one tear at a time: the
    // segment's loop, face or shell torn to name the lone face, shell or
    // solid, the segment's face torn to list the lone loop as a ring and
    // as its outer, its shell to list the lone face, its solid to list
    // the lone shell, and a third
    // solid's loop torn `Empty` at the lone vertex. Unchecked, each kill
    // leaves the torn record naming a dead one.
    use crate::euler::MvfsCreated;
    type Tear = fn(&mut Body<f64>, &MvfsCreated, &MvfsCreated) -> EulerOpError;
    let tears: [Tear; 8] = [
        |b, seg, lone| {
            b.get_loop_mut(seg.r#loop).unwrap().face = lone.face;
            dangling(EntityId::Loop(seg.r#loop), EntityId::Face(lone.face))
        },
        |b, seg, lone| {
            b.get_face_mut(seg.face).unwrap().shell = lone.shell;
            dangling(EntityId::Face(seg.face), EntityId::Shell(lone.shell))
        },
        |b, seg, lone| {
            b.get_shell_mut(seg.shell).unwrap().solid = lone.solid;
            dangling(EntityId::Shell(seg.shell), EntityId::Solid(lone.solid))
        },
        |b, seg, lone| {
            b.get_face_mut(seg.face).unwrap().rings.push(lone.r#loop);
            dangling(EntityId::Face(seg.face), EntityId::Loop(lone.r#loop))
        },
        |b, seg, lone| {
            b.get_face_mut(seg.face).unwrap().outer = lone.r#loop;
            dangling(EntityId::Face(seg.face), EntityId::Loop(lone.r#loop))
        },
        |b, seg, lone| {
            b.get_shell_mut(seg.shell).unwrap().faces.push(lone.face);
            dangling(EntityId::Shell(seg.shell), EntityId::Face(lone.face))
        },
        |b, seg, lone| {
            b.get_solid_mut(seg.solid).unwrap().shells.push(lone.shell);
            dangling(EntityId::Solid(seg.solid), EntityId::Shell(lone.shell))
        },
        |b, _, lone| {
            let third = b.mvfs(p(9.0), true).unwrap();
            b.get_loop_mut(third.r#loop).unwrap().boundary = LoopBoundary::Empty {
                vertex: lone.vertex,
            };
            dangling(EntityId::Loop(third.r#loop), EntityId::Vertex(lone.vertex))
        },
    ];
    for tear in tears {
        let (mut body, seg, lone) = segment_beside_a_lone_solid();
        let torn = tear(&mut body, &seg, &lone);
        assert_kill_refuses(&mut body, &torn, |b| b.kvfs(lone.solid));
    }
}

#[test]
fn mekr_refuses_an_empty_ring_a_torn_half_edge_claims() {
    // An `Empty` ring (the strut killed from its root), and two `Empty`
    // loops of one face (the segment killed), each beside a bystander
    // segment's solid whose plus half is torn to claim the ring.
    // Unchecked, the kill removes the ring the half-edge claims.
    let tol = Tol::witness();
    for build in [EMPTY_RING_BESIDE_A_CYCLE, TWO_EMPTY_LOOPS] {
        let mut body = (build.1)(tol);
        let empties: Vec<_> = body
            .loops()
            .filter(|(_, l)| matches!(l.boundary, LoopBoundary::Empty { .. }))
            .map(|(k, _)| k)
            .collect();
        let ring = *empties.last().unwrap();
        let face = body.get_loop(ring).unwrap().face;
        assert!(body.get_face(face).unwrap().rings.contains(&ring));
        let site = match empties[..] {
            [_] => {
                let (target, _) = body
                    .half_edges()
                    .find(|(_, h)| h.parent_loop != ring)
                    .unwrap();
                MekrSite::EmptyRing { target, ring }
            }
            [target, _] => MekrSite::BothEmpty { target, ring },
            _ => unreachable!("{}: one or two empty loops", build.0),
        };
        let seed = body.mvfs(p(7.0), true).unwrap();
        let lone = MevSite::Lone {
            r#loop: seed.r#loop,
        };
        let bystander = body.mev_line(lone, p(8.0), tol).unwrap();
        body.get_half_edge_mut(bystander.he_plus)
            .unwrap()
            .parent_loop = ring;
        let torn = EulerOpError::LoopCycleBroken { r#loop: ring };
        assert_mekr_refuses(&mut body, site, &torn);
    }
}

/// Each `mekr` site on the valid body that offers it, with the ring
/// loop the site removes: `Cycles` on the holed box, `EmptyTarget` on
/// [`RING_ABOUT_AN_EMPTY_OUTER`], `EmptyRing` on
/// [`EMPTY_RING_BESIDE_A_CYCLE`] and `BothEmpty` on [`TWO_EMPTY_LOOPS`].
fn every_mekr_site(tol: Tol) -> [(Body<f64>, MekrSite, crate::entity::LoopKey); 4] {
    let member = |body: &Body<f64>, l| {
        body.half_edges()
            .find(|(_, h)| h.parent_loop == l)
            .map(|(k, _)| k)
            .unwrap()
    };
    let is_empty = |body: &Body<f64>, l| {
        matches!(
            body.get_loop(l).unwrap().boundary,
            LoopBoundary::Empty { .. }
        )
    };
    // The face with a ring, its outer loop and its first ring.
    let ringed = |body: &Body<f64>| {
        let (_, face) = body.faces().find(|(_, f)| !f.rings.is_empty()).unwrap();
        (face.outer, face.rings[0])
    };
    let hb = ops_holed_box(tol);
    let ring = hb.plug.ring;
    let top = hb.body.get_loop(ring).unwrap().face;
    let outer = hb.body.get_face(top).unwrap().outer;
    let cycles = MekrSite::Cycles {
        target: member(&hb.body, outer),
        ring: member(&hb.body, ring),
    };
    let sites = [
        RING_ABOUT_AN_EMPTY_OUTER,
        EMPTY_RING_BESIDE_A_CYCLE,
        TWO_EMPTY_LOOPS,
    ]
    .map(|(name, build)| {
        let body = build(tol);
        let (outer, ring) = ringed(&body);
        let site = match (is_empty(&body, outer), is_empty(&body, ring)) {
            (true, false) => MekrSite::EmptyTarget {
                target: outer,
                ring: member(&body, ring),
            },
            (false, true) => MekrSite::EmptyRing {
                target: member(&body, outer),
                ring,
            },
            (true, true) => MekrSite::BothEmpty {
                target: outer,
                ring,
            },
            (false, false) => unreachable!("{name}: an `Empty` loop beside the ring"),
        };
        (body, site, ring)
    });
    let [a, b, c] = sites;
    [(hb.body, cycles, ring), a, b, c]
}

#[test]
fn mekr_refuses_a_ring_another_face_lists() {
    // At each site, a bystander face's `rings`, then its `outer`, torn
    // to name the ring. Unchecked, the ring's own face drops it and the
    // bystander is left naming a dead loop.
    let tol = Tol::witness();
    type Tear = fn(&mut crate::entity::Face, crate::entity::LoopKey);
    let tears: [Tear; 2] = [|f, ring| f.rings.push(ring), |f, ring| f.outer = ring];
    for tear in tears {
        for (mut body, site, ring) in every_mekr_site(tol) {
            let bystander = body.mvfs(p(9.0), true).unwrap();
            tear(body.get_face_mut(bystander.face).unwrap(), ring);
            let torn = dangling(EntityId::Face(bystander.face), EntityId::Loop(ring));
            assert_mekr_refuses(&mut body, site, &torn);
        }
    }
}

/// A loop of `body` none of whose half-edges starts or ends at a vertex
/// of `ends`, with its `first`: records no walk of a kill with those
/// endpoints reads, since every walk a kill takes steps the killed
/// halves' loops or the orbits of their endpoints.
fn far_loop(
    body: &Body<f64>,
    ends: &[crate::entity::VertexKey],
) -> (crate::entity::LoopKey, HalfEdgeKey) {
    let touches = |h: HalfEdgeKey| {
        let start = body.get_half_edge(h).unwrap().start;
        ends.contains(&start) || body.half_edge_end(h).is_some_and(|end| ends.contains(&end))
    };
    body.loops()
        .find_map(|(l, data)| {
            let LoopBoundary::Cycle { first } = data.boundary else {
                return None;
            };
            let cycle = body.loop_cycle(first)?;
            (!cycle.into_iter().any(touches)).then_some((l, first))
        })
        .expect("a loop away from both endpoints")
}

/// Every way a record a kill keeps can name one of the halves `[he, m]`
/// it removes, each planted on its own copy of `body` away from every
/// walk the kill takes ([`far_loop`]), with the refusal each earns: a
/// half-edge's `next`, then its `prev`, a loop's `first`, a vertex's
/// `emanating`, and an edge's slot.
fn half_edge_tears(body: &Body<f64>, [he, m]: [HalfEdgeKey; 2]) -> [(Body<f64>, EulerOpError); 5] {
    let start = |h| body.get_half_edge(h).unwrap().start;
    let (l, x) = far_loop(body, &[start(he), start(m)]);
    let (v, e) = (start(x), body.get_half_edge(x).unwrap().edge);
    let torn = |tear: &dyn Fn(&mut Body<f64>)| {
        let mut copy = body.clone();
        tear(&mut copy);
        copy
    };
    [
        (
            torn(&|b| b.get_half_edge_mut(x).unwrap().next = he),
            dangling(EntityId::HalfEdge(x), EntityId::HalfEdge(he)),
        ),
        (
            torn(&|b| b.get_half_edge_mut(x).unwrap().prev = m),
            dangling(EntityId::HalfEdge(x), EntityId::HalfEdge(m)),
        ),
        (
            torn(&|b| b.get_loop_mut(l).unwrap().boundary = LoopBoundary::Cycle { first: he }),
            dangling(EntityId::Loop(l), EntityId::HalfEdge(he)),
        ),
        (
            torn(&|b| b.get_vertex_mut(v).unwrap().emanating = Some(m)),
            dangling(EntityId::Vertex(v), EntityId::HalfEdge(m)),
        ),
        (
            torn(&|b| b.get_edge_mut(e).unwrap().he_plus = he),
            dangling(EntityId::Edge(e), EntityId::HalfEdge(he)),
        ),
    ]
}

#[test]
fn kef_refuses_a_killed_half_another_record_names() {
    // The declined cube, a half-edge of a face away from both endpoints
    // torn to name a killed half, then a loop's `first`, a vertex's
    // `emanating` and an edge's slot. Unchecked, the kill leaves each
    // naming a dead half-edge.
    let cube = declined_cube::<f64>(Tol::witness()).body;
    let he = arena_halves(&cube)[6];
    let m = cube.mate(he).unwrap();
    for (mut body, torn) in half_edge_tears(&cube, [he, m]) {
        assert_kef_refuses(&mut body, he, &torn);
    }
}

#[test]
fn kev_refuses_a_killed_half_another_record_names() {
    // As `kef_refuses_a_killed_half_another_record_names`, for `kev`.
    let cube = declined_cube::<f64>(Tol::witness()).body;
    let he = arena_halves(&cube)[3];
    let m = cube.mate(he).unwrap();
    for (mut body, torn) in half_edge_tears(&cube, [he, m]) {
        assert_kev_refuses(&mut body, he, &torn);
    }
}

#[test]
fn kemr_refuses_a_killed_half_another_record_names() {
    // As `kef_refuses_a_killed_half_another_record_names`, for `kemr` at
    // the strut cube's strut.
    let fixture = ops_strut_cube(Tol::witness());
    let (he1, he2) = (fixture.strut.he_plus, fixture.strut.he_minus);
    for (mut body, torn) in half_edge_tears(&fixture.body, [he1, he2]) {
        assert_kill_refuses(&mut body, &torn, |b| b.kemr(he1, he2));
    }
}

/// Asserts that `kfmrh(f1, f2)` and `kfmrh_minting(f1, f2, tol)` refuse
/// `torn` with the body deep-unchanged.
fn assert_kfmrh_refuses(body: &mut Body<f64>, f1: FaceKey, f2: FaceKey, torn: &EulerOpError) {
    assert_kill_refuses(body, torn, |b| b.kfmrh(f1, f2));
    assert_kill_refuses(body, torn, |b| b.kfmrh_minting(f1, f2, Tol::witness()));
}

#[test]
fn kfmrh_refuses_a_face_another_record_names() {
    // The same-shell form on the declined cube: a loop other than the
    // ring torn to name `f2`, then another shell torn to list it.
    // Unchecked, the kill leaves each naming a dead face.
    let tol = Tol::witness();
    let build = || {
        let body = declined_cube::<f64>(tol).body;
        let faces: Vec<FaceKey> = body.faces().map(|(f, _)| f).collect();
        (body, faces[0], faces[1])
    };
    let (mut body, f1, f2) = build();
    let ring = body.get_face(f2).unwrap().outer;
    let (third, _) = body
        .loops()
        .find(|&(l, data)| l != ring && data.face != f1)
        .unwrap();
    body.get_loop_mut(third).unwrap().face = f2;
    let torn = dangling(EntityId::Loop(third), EntityId::Face(f2));
    assert_kfmrh_refuses(&mut body, f1, f2, &torn);

    let (mut body, f1, f2) = build();
    let other = body.mvfs(p(9.0), true).unwrap();
    body.get_shell_mut(other.shell).unwrap().faces.push(f2);
    let torn = dangling(EntityId::Shell(other.shell), EntityId::Face(f2));
    assert_kfmrh_refuses(&mut body, f1, f2, &torn);
}

#[test]
fn kfmrh_refuses_in_its_fusion_form_a_face_or_shell_another_record_names() {
    // The fusion form on two shells of one solid: `f1`'s own shell torn
    // to list `f2`, a face of `f1`'s shell torn to name `f2`'s, and
    // another solid torn to list `f2`'s shell. Unchecked, the fusion
    // leaves each naming a dead face or shell.
    let tol = Tol::witness();
    let build = || {
        let body = (TWO_SHELLS_OF_ONE_SOLID.1)(tol);
        let (_, solid) = body.solids().next().unwrap();
        let (a, b) = (solid.shells[0], solid.shells[1]);
        let faces = |s| body.get_shell(s).unwrap().faces.clone();
        let (fa, fb) = (faces(a), faces(b));
        (body, a, b, fa, fb[0])
    };
    let (mut body, a, _, fa, f2) = build();
    body.get_shell_mut(a).unwrap().faces.push(f2);
    let torn = dangling(EntityId::Shell(a), EntityId::Face(f2));
    assert_kfmrh_refuses(&mut body, fa[0], f2, &torn);

    let (mut body, _, b, fa, f2) = build();
    body.get_face_mut(fa[1]).unwrap().shell = b;
    let torn = dangling(EntityId::Face(fa[1]), EntityId::Shell(b));
    assert_kfmrh_refuses(&mut body, fa[0], f2, &torn);

    let (mut body, _, b, fa, f2) = build();
    let other = body.mvfs(p(9.0), true).unwrap();
    body.get_solid_mut(other.solid).unwrap().shells.push(b);
    let torn = dangling(EntityId::Solid(other.solid), EntityId::Shell(b));
    assert_kfmrh_refuses(&mut body, fa[0], f2, &torn);
}
