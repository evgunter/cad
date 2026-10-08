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
//! - The two plan-phase link checks (`split_edge`'s `prev(hm)` and
//!   `kef`'s `prev(he)`) are claimed to be **reachable** and to fire
//!   before any write. These rows plant a dangling `prev` and assert
//!   both the plan phase's panic naming that link and a deep-equal
//!   body.
//! - The unit's PR body claims
//!   `review_m1_pr4::kill_ops_on_torn_bodies_panic_only_naming_a_premise` is
//!   "the row most exposed to `kef`'s new check". That
//!   fixture tears only `next` links and edge bijections. The evidence
//!   row below rebuilds the tear and shows every `prev` stays live, so
//!   the new check never fires there and the row carries no coverage of
//!   it. Recorded so the coverage claim is not inherited.
//!
//! No fixed seeds (nothing here is randomized); the one loop count is
//! on the workspace `CAD_FUZZ_EFFORT` dial via `fuzz::scaled`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/topo/src/euler.rs",
    "crates/topo/src/euler_ring.rs",
    "crates/topo/src/euler_kill.rs",
    "crates/topo/src/body.rs",
    "crates/topo/src/entity.rs",
    "crates/topo/src/fixtures.rs",
    "crates/topo/src/test_support_fixtures.rs",
];

use geom_core::Point3;

use crate::Body;
use crate::entity::{EntityId, HalfEdgeKey};
use crate::euler::{MefSite, MevSite};
use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};
use crate::test_support_fixtures::declined_cube;
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

/// Asserts `op` panics in its plan phase naming `holder`'s `field` as a
/// link to the null half-edge key, with `body` deep-unchanged.
fn assert_dangling_link_panics<R: core::fmt::Debug>(
    label: &str,
    body: &mut Body<f64>,
    holder: HalfEdgeKey,
    field: &str,
    op: impl FnOnce(&mut Body<f64>) -> R,
) {
    let named = format!(
        "{}'s {field} names {}, which does not resolve",
        EntityId::HalfEdge(holder),
        EntityId::HalfEdge(HalfEdgeKey::default())
    );
    assert_torn_op_panics(label, body, &[&named, ROW_FOUR], op);
}

/// `split_edge`'s `prev(he_minus)` check is reachable: a dangling
/// `prev` on the minus half panics in the plan phase naming the link,
/// before any mutation and before the geometry gate. Without the check
/// the dangling key would reach `link_half_edges`' mutation-phase arm.
#[test]
fn d18_split_edge_panics_at_a_dangling_prev_of_he_minus() {
    let tol = Tol::witness();
    let cube = declined_cube::<f64>(tol);
    let mut body = cube.body;
    let edge = cube.mevs[0].edge;
    let hm = body.get_edge(edge).unwrap().he_minus;
    body.get_half_edge_mut(hm).unwrap().prev = HalfEdgeKey::default();
    assert_dangling_link_panics("split_edge", &mut body, hm, "prev", |b| {
        b.split_edge(edge, 0.5, tol)
    });
}

/// The symmetric control: `split_edge`'s `next(he_plus)` check panics
/// the same way, so the `prev` check joined a pair rather than
/// replacing one.
#[test]
fn d18_split_edge_panics_at_a_dangling_next_of_he_plus() {
    let tol = Tol::witness();
    let cube = declined_cube::<f64>(tol);
    let mut body = cube.body;
    let edge = cube.mevs[0].edge;
    let hp = body.get_edge(edge).unwrap().he_plus;
    body.get_half_edge_mut(hp).unwrap().next = HalfEdgeKey::default();
    assert_dangling_link_panics("split_edge", &mut body, hp, "next", |b| {
        b.split_edge(edge, 0.5, tol)
    });
}

/// The digon pillow, built by operators: two vertices, two edges, two
/// faces. `kef` applies to either half of the second edge.
fn pillow(tol: Tol) -> (Body<f64>, crate::MevCreated, crate::MefCreated) {
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
    let split = body
        .mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            tol,
        )
        .unwrap();
    (body, seg, split)
}

/// `kef`'s `prev(he)` check is reachable: a dangling `prev` on the
/// dying half-edge panics in the plan phase naming the link.
///
/// The tear is on `prev` only, so `loop_cycle(he)` — which steps
/// `next` — still closes; the panic therefore comes from the link
/// checks `[a, c, d]`, not from the loop walk. That distinction is the
/// whole content of the unit's `kef` half.
#[test]
fn d18_kef_panics_at_a_dangling_prev_of_he() {
    let tol = Tol::witness();
    let (mut body, _seg, split) = pillow(tol);
    let he = split.he_minus;
    body.get_half_edge_mut(he).unwrap().prev = HalfEdgeKey::default();
    assert!(
        body.loop_cycle(he).is_some(),
        "fixture: tearing prev must not disturb the next-walk"
    );
    assert_dangling_link_panics("kef", &mut body, he, "prev", |b| b.kef(he));
}

/// EVIDENCE, not a kernel gate: the torn-body fixture that
/// `review_m1_pr4::kill_ops_on_torn_bodies_panic_only_naming_a_premise`
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
        let s = other.mvfs(p(0.0), true).unwrap();
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
/// does."* Every postcondition message in the crate carries it — the
/// operator's delta message, the shared tier-1 one, and the
/// door-level sweep in [`crate::surgery`]. The premise is true today
/// and is held by nothing — one
/// `unreachable!` whose message happens to contain the word would make
/// the discriminator pass on exactly the failure it exists to catch,
/// silently. This row is that premise as a gate.
///
/// It reads the crate's own sources (the `source_walk::crate_sources`
/// walk) through `test_utils::source::code_and_literals`, so it costs
/// a directory walk and no fixture.
#[test]
fn d18_no_unreachable_message_can_impersonate_the_postcondition() {
    let mut offenders: Vec<String> = Vec::new();
    let mut postcondition_messages = 0_usize;
    for path in crate::source_walk::crate_sources() {
        if path.ends_with("review_d18_probes.rs") {
            continue; // this row quotes the literal it forbids
        }
        let text = std::fs::read_to_string(&path).unwrap();
        // Calls only, and the MESSAGE is what is read: the literal
        // view blanks every comment and keeps every literal, which is
        // the one combination this row can use. Under the code view
        // the message it judges would be spaces; over raw text a
        // commented-out `unreachable!` still reads as a site, and so
        // does prose after code on the same line. Line structure
        // survives blanking, so `i` is still the file's line number.
        let code = test_utils::source::code_and_literals(&text);
        for (i, line) in code.lines().enumerate() {
            if line.contains("postcondition:") && line.contains('"') {
                postcondition_messages += 1;
            }
            if !line.contains("unreachable!(") {
                continue;
            }
            // The message may sit on the next lines (rustfmt wraps).
            let window: String = code.lines().skip(i).take(4).collect::<Vec<_>>().join(" ");
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
         message(s); expected at least the operators' delta and tier-1 \
         messages (`assert_euler_postcondition` and the shared helper it \
         calls in `crate::surgery`) — the walk is not reading topo/src"
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
