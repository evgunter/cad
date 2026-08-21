//! Reviewer consumer probes for **D18 / PR #736** — the two `prev`
//! proofs and the `link_half_edges` conversion behind them.
//!
//! These are **independent derivations** of what that unit claims, not
//! a re-reading of its diff. Three of the four rows gate; the fourth is
//! marked as evidence for the review and asserts a fact about a
//! *fixture*, not about the kernel.
//!
//! What each row is for:
//!
//! - The two new plan-phase link checks (`split_edge`'s `prev(hm)` and
//!   `kef`'s `prev(he)`) are claimed to be **reachable** and to preserve
//!   atomicity. Nothing in the tree plants a dangling `prev`, so nothing
//!   in the tree distinguishes "the check is there" from "the check is
//!   dead code". These rows plant one and assert both the typed
//!   `StaleKey` and a deep-equal body.
//! - The unit's PR body claims
//!   `review_m1_pr4::kill_ops_survive_torn_bodies_without_panicking` is
//!   "the row most exposed to `kef`'s new check" and that under it "some
//!   of those calls return `StaleKey` earlier than they used to". That
//!   fixture tears only `next` links and edge bijections. The evidence
//!   row below rebuilds the tear and shows every `prev` stays live, so
//!   the new check never fires there and the row carries no coverage of
//!   it. Recorded so the coverage claim is not inherited.
//!
//! No fixed seeds (nothing here is randomized); the one loop count is
//! on the workspace `CAD_FUZZ_EFFORT` dial via `fuzz::scaled`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;

use crate::entity::{EntityId, HalfEdgeKey};
use crate::euler::{MefSite, MevSite};
use crate::fixtures::{deep_snapshot, ops_cube};
use crate::{Body, EulerOpError};
use geom_core::Tol;

/// Strut count for the torn-body evidence row. On the workspace's
/// effort dial rather than a private one: `CAD_FUZZ_EFFORT=15`
/// reproduces `review_m1_pr4`'s scale. `test-utils` is the ratified
/// home for that knob — a second env read inside `crates/*/src` is
/// what `scripts/gates/no-ambient-env.sh` exists to refuse, and its
/// allowlist is two files argued one at a time.
fn struts() -> usize {
    test_utils::fuzz::scaled(40)
}

fn p(x: f64) -> Point3<f64> {
    Point3::new(x, 0.0, 0.0)
}

/// `split_edge`'s new `prev(he_minus)` check is reachable: a dangling
/// `prev` on the minus half is refused **typed and atomically**, before
/// any mutation and before the geometry gate.
///
/// This is the check the unit added. Without it the dangling key would
/// reach `link_half_edges` — which, after this unit, announces. So the
/// row also pins that the refusal is a `StaleKey`, never a panic.
#[test]
fn d18_split_edge_refuses_a_dangling_prev_of_he_minus() {
    let tol = Tol::witness();
    let cube = ops_cube(tol);
    let mut body = cube.body;
    let edge = cube.mevs[0].edge;
    let hm = body.get_edge(edge).unwrap().he_minus;
    body.get_half_edge_mut(hm).unwrap().prev = HalfEdgeKey::default();

    let before = deep_snapshot(&body);
    let err = body.split_edge(edge, 0.5, tol).unwrap_err();
    assert_eq!(
        err,
        EulerOpError::StaleKey {
            key: EntityId::HalfEdge(HalfEdgeKey::default()),
        },
        "split_edge must refuse a dangling prev(he_minus) typed"
    );
    assert_eq!(
        deep_snapshot(&body),
        before,
        "split_edge atomicity: the body must be untouched on Err"
    );
}

/// The symmetric control: `split_edge`'s pre-existing `next(he_plus)`
/// check still refuses the same way, so the new check joined a pair
/// rather than replacing one.
#[test]
fn d18_split_edge_still_refuses_a_dangling_next_of_he_plus() {
    let tol = Tol::witness();
    let cube = ops_cube(tol);
    let mut body = cube.body;
    let edge = cube.mevs[0].edge;
    let hp = body.get_edge(edge).unwrap().he_plus;
    body.get_half_edge_mut(hp).unwrap().next = HalfEdgeKey::default();

    let before = deep_snapshot(&body);
    let err = body.split_edge(edge, 0.5, tol).unwrap_err();
    assert_eq!(
        err,
        EulerOpError::StaleKey {
            key: EntityId::HalfEdge(HalfEdgeKey::default()),
        },
        "split_edge must still refuse a dangling next(he_plus) typed"
    );
    assert_eq!(
        deep_snapshot(&body),
        before,
        "split_edge atomicity on the pre-existing check"
    );
}

/// The digon pillow, built by operators: two vertices, two edges, two
/// faces. `kef` applies to either half of the second edge.
fn pillow(tol: Tol) -> (Body<f64>, crate::MevCreated, crate::MefCreated) {
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
    let split = body
        .mef_chord(MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        }, tol)
        .unwrap();
    (body, seg, split)
}

/// `kef`'s new `prev(he)` check is reachable: a dangling `prev` on the
/// dying half-edge is refused typed and atomically.
///
/// The tear is on `prev` only, so `loop_cycle(he)` — which steps
/// `next` — still closes; the refusal therefore comes from the link
/// loop the unit widened from `[c, d]` to `[a, c, d]`, and not from
/// `LoopCycleBroken`. That distinction is the whole content of the
/// unit's `kef` half.
#[test]
fn d18_kef_refuses_a_dangling_prev_of_he() {
    let tol = Tol::witness();
    let (mut body, _seg, split) = pillow(tol);
    let he = split.he_minus;
    body.get_half_edge_mut(he).unwrap().prev = HalfEdgeKey::default();

    let before = deep_snapshot(&body);
    let err = body.kef(he).unwrap_err();
    assert_eq!(
        err,
        EulerOpError::StaleKey {
            key: EntityId::HalfEdge(HalfEdgeKey::default()),
        },
        "kef must refuse a dangling prev(he) typed, not LoopCycleBroken \
         and not a panic"
    );
    assert_eq!(
        deep_snapshot(&body),
        before,
        "kef atomicity: the body must be untouched on Err"
    );
}

/// EVIDENCE, not a kernel gate: the torn-body fixture that
/// `review_m1_pr4::kill_ops_survive_torn_bodies_without_panicking`
/// builds tears `next` links and edge bijections and **never touches a
/// `prev` field**, so every `prev` in it stays live and `kef`'s new
/// `[a, c, d]` check cannot fire there.
///
/// The assertion is about the fixture, so it goes red only if someone
/// changes the tear — which is exactly when the coverage claim made for
/// that row would need re-checking.
#[test]
fn d18_torn_body_fixture_leaves_every_prev_live() {
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
    let mut anchor = seg.he_minus;
    for i in 0..struts() {
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: anchor,
                    he2: anchor,
                },
                p(2.0 + i as f64),
                tol,
            )
            .unwrap();
        anchor = strut.he_minus;
    }
    let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(k, _)| k).collect();
    let foreign = {
        let mut other = Body::<f64>::new();
        let s = other.mvfs(p(0.0)).unwrap();
        let sg = other
            .mev_line(MevSite::Lone { r#loop: s.r#loop }, p(1.0), tol)
            .unwrap();
        sg.he_plus
    };
    for (i, &he) in halves.iter().enumerate() {
        if i % 2 == 0 {
            body.get_half_edge_mut(he).unwrap().next = foreign;
        }
        if i % 3 == 0 {
            let edge = body.get_half_edge(he).unwrap().edge;
            if let Some(e) = body.get_edge_mut(edge) {
                e.he_plus = he;
                e.he_minus = he;
            }
        }
    }
    let dangling_prev = halves
        .iter()
        .filter(|&&he| {
            let prev = body.get_half_edge(he).unwrap().prev;
            body.get_half_edge(prev).is_none()
        })
        .count();
    assert_eq!(
        dangling_prev, 0,
        "the review_m1_pr4 tear leaves every prev live, so kef's new \
         prev(he) check is never exercised by it"
    );
}

/// The `debug_postcondition_fires_on_corrupt_input` discriminator is a
/// **substring test on a panic message**, and nothing in the tree keeps
/// it discriminating.
///
/// That row is the unit's only claimed red-signal for a mis-converted
/// row-4 arm: it asserts the panic it caught said `"postcondition"`,
/// on the stated premise that *"Both `assert_euler_postcondition`
/// messages carry the literal asserted below; no `unreachable!` message
/// does."* The premise is true today and is held by nothing — one
/// `unreachable!` whose message happens to contain the word would make
/// the discriminator pass on exactly the failure it exists to catch,
/// silently. This row is that premise as a gate.
///
/// It reads the crate's own sources (the `source_walk::crate_sources`
/// walk), so it costs a directory walk and no fixture.
#[test]
fn d18_no_unreachable_message_can_impersonate_the_postcondition() {
    let mut offenders: Vec<String> = Vec::new();
    let mut postcondition_messages = 0_usize;
    for path in crate::source_walk::crate_sources() {
        if path.ends_with("review_d18_probes.rs") {
            continue; // this row quotes the literal it forbids
        }
        let text = std::fs::read_to_string(&path).unwrap();
        for (i, line) in text.lines().enumerate() {
            if line.contains("postcondition:") && line.contains('"') {
                postcondition_messages += 1;
            }
            let trimmed = line.trim_start();
            // Calls only: doc prose that mentions the macro is not one.
            if trimmed.starts_with("//") || !line.contains("unreachable!(") {
                continue;
            }
            // The message may sit on the next lines (rustfmt wraps).
            let window: String = text.lines().skip(i).take(4).collect::<Vec<_>>().join(" ");
            let head = window.split("unreachable!(").nth(1).unwrap_or("");
            let msg = head.split(')').next().unwrap_or("");
            if msg.contains("postcondition") {
                offenders.push(format!("{}:{}", path.display(), i + 1));
            }
        }
    }
    assert!(
        postcondition_messages >= 2,
        "the walk found {postcondition_messages} postcondition assertion \
         message(s); expected at least the two in \
         `assert_euler_postcondition` — the walk is not reading topo/src"
    );
    assert!(
        offenders.is_empty(),
        "an `unreachable!` message contains the literal \
         `postcondition`, which vacates \
         `debug_postcondition_fires_on_corrupt_input`'s discriminator — \
         that row would then pass on a mis-converted row-4 arm: \
         {offenders:?}"
    );
}
