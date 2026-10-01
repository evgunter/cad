//! The labelling proofs' tear measurement: `movefac` on torn bodies,
//! counting the `Ok` results whose partition misreads the records
//! ([`super::claimed_components`]).

// A counterexample search over the labelling's walks and the proofs
// they call: the operator, `Body::require_run_of`, the cycle walk it
// proves, and the fixtures it tears.
test_utils::gated_to![
    "crates/topo/src/movefac.rs",
    "crates/topo/src/euler.rs",
    "crates/topo/src/body.rs",
    "crates/topo/src/fixtures.rs",
    "crates/topo/src/test_support_fixtures.rs",
];

use geom_core::Tol;
use test_utils::fuzz::Rng;

use super::{claimed_components, misread};
use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, ShellKey};
use crate::fixtures::{
    detached_digons, ops_genus2, ops_holed_box, ops_ring_bridge, ops_strut_cube,
    through_the_scalpel,
};
use crate::test_support_fixtures::declined_cube;

/// A link the labelling reads, torn live-but-foreign: a `next` its
/// cycle walk steps, a `parent_loop` the proof reads a member's loop
/// from, a loop's `first` the walk starts at, and an edge's slots the
/// mate hop reads; or a loop's boundary torn `Empty` at a live vertex
/// while its half-edges still claim it, so the walk from its face
/// steps none of them and a mate hop into it has no hop back; or a
/// loop's `face` torn to another live face, which does not list it, so
/// a mate hop into the loop lands on a face that does not own it: the
/// converse's subject, which no other kind writes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tear {
    NextForeign,
    ParentLoopForeign,
    LoopAnchorForeign,
    EdgeBijection,
    LoopEmptied,
    LoopFaceForeign,
}

const TEARS: [Tear; 6] = [
    Tear::NextForeign,
    Tear::ParentLoopForeign,
    Tear::LoopAnchorForeign,
    Tear::EdgeBijection,
    Tear::LoopEmptied,
    Tear::LoopFaceForeign,
];

/// The bodies torn: one to three components, rings, a strut, genus.
/// Each is one shell, so every half-edge a tear reaches lies in a loop
/// the labelling walks.
const BODIES: [(&str, Build); 7] = [
    ("declined_cube", || declined_cube(Tol::witness()).body),
    ("ops_strut_cube", || ops_strut_cube(Tol::witness()).body),
    ("ops_holed_box", || ops_holed_box(Tol::witness()).body),
    ("ops_genus2", || ops_genus2(Tol::witness())),
    ("ops_ring_bridge", || ops_ring_bridge(Tol::witness()).body),
    ("one detached digon", || detached_digons(1).0),
    ("two detached digons", || detached_digons(2).0),
];

/// How a [`BODIES`] entry builds its body.
type Build = fn() -> Body<f64>;

fn plant(body: &mut Body<f64>, tear: Tear, rng: &mut Rng) {
    let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(k, _)| k).collect();
    let loops: Vec<LoopKey> = body.loops().map(|(k, _)| k).collect();
    let mut pick = |n: usize| (rng.next_u64() as usize) % n;
    let he = halves[pick(halves.len())];
    let other = halves[pick(halves.len())];
    let l = loops[pick(loops.len())];
    match tear {
        Tear::NextForeign => body.get_half_edge_mut(he).unwrap().next = other,
        Tear::ParentLoopForeign => body.get_half_edge_mut(he).unwrap().parent_loop = l,
        Tear::LoopAnchorForeign => {
            body.get_loop_mut(l).unwrap().boundary = LoopBoundary::Cycle { first: other };
        }
        Tear::EdgeBijection => {
            let edge = body.get_half_edge(he).unwrap().edge;
            let e = body.get_edge_mut(edge).unwrap();
            e.he_plus = he;
            e.he_minus = other;
        }
        Tear::LoopEmptied => {
            let vertex = body.get_half_edge(he).unwrap().start;
            body.get_loop_mut(l).unwrap().boundary = LoopBoundary::Empty { vertex };
        }
        // Drawn here, after the three draws every kind makes, so the
        // other kinds' streams, and so their cells, are the ones they
        // were before this kind existed.
        Tear::LoopFaceForeign => {
            let own = body.get_loop(l).unwrap().face;
            let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).filter(|&k| k != own).collect();
            body.get_loop_mut(l).unwrap().face = faces[pick(faces.len())];
        }
    }
}

/// One row per tear kind: calls, `Err`, `Ok`, and the `Ok` results that
/// join two of the records' components into one shell or split one
/// across two.
type Row = [usize; 5];

/// The measurement under `tear`: for each seed, one and two tears on
/// every [`BODIES`] body, then `movefac` on each of its shells, each on
/// a clone inside a surgery scope, so a debug build's tier-1
/// postcondition, which a torn input fails whatever the operator
/// writes, does not answer first.
fn rows(tear: Tear, seeds: &[u64]) -> Row {
    let mut row = [0; 5];
    for &seed in seeds {
        for tears in [1, 2] {
            for (_, build) in BODIES {
                let mut body = build();
                let mut rng = Rng::from_seed(seed);
                for _ in 0..tears {
                    plant(&mut body, tear, &mut rng);
                }
                let shells: Vec<ShellKey> = body.shells().map(|(k, _)| k).collect();
                for shell in shells {
                    let truth = claimed_components(&body, shell);
                    let mut trial = body.clone();
                    let mut scope = trial.begin_surgery();
                    let outcome = through_the_scalpel(&["movefac"], || scope.movefac(shell));
                    drop(scope);
                    // A fired sweep stood in front of the `Ok` naming the
                    // shell and the shells the move minted.
                    let outcome = outcome.unwrap_or_else(|_| {
                        Ok(std::iter::once(shell)
                            .chain(
                                trial
                                    .shells()
                                    .map(|(k, _)| k)
                                    .filter(|&k| body.get_shell(k).is_none()),
                            )
                            .collect())
                    });
                    row[0] += 1;
                    let Ok(result) = outcome else {
                        row[1] += 1;
                        continue;
                    };
                    row[2] += 1;
                    let (joined, split) = misread(&trial, &result, &truth);
                    row[3] += usize::from(joined);
                    row[4] += usize::from(split);
                }
            }
        }
    }
    row
}

/// Asserts no tear let an `Ok` misread the records.
fn assert_no_misread(table: &[Row; TEARS.len()], context: &str) {
    for (tear, row) in TEARS.iter().zip(table) {
        assert_eq!(
            row[3..],
            [0, 0],
            "`movefac` under {tear:?} joined or split the records' components through `Ok` \
             ({context})"
        );
    }
}

/// [`rows`] on a few seeds: a counterexample search, on the shared fuzz
/// seed and effort dial.
#[test]
fn movefac_on_a_few_torn_bodies() {
    let mut rng = test_utils::fuzz::start("movefac_on_a_few_torn_bodies");
    let seeds: Vec<u64> = (0..test_utils::fuzz::scaled(4))
        .map(|_| rng.next_u64())
        .collect();
    let table = TEARS.map(|tear| rows(tear, &seeds));
    assert_no_misread(&table, &test_utils::fuzz::replay());
}

/// **Evidence, not a gate**: [`rows`] on seeds `1..=2000`, run by hand,
/// which prints the table and asserts it as the gate does.
///
/// `cargo test -p topo --lib --config
/// 'profile.dev.package.topo.debug-assertions=false' -- --ignored
/// --nocapture movefac::tests::tears::movefac_on_torn_bodies`
#[test]
#[ignore = "evidence: the labelling proofs' tear measurement, run by hand"]
fn movefac_on_torn_bodies() {
    let seeds: Vec<u64> = (1..=2000).collect();
    let table = std::thread::scope(|scope| {
        let handles = TEARS.map(|tear| {
            let seeds = &seeds;
            scope.spawn(move || rows(tear, seeds))
        });
        handles.map(|handle| handle.join().unwrap())
    });
    println!("| tear | calls | `Err` | `Ok` | `Ok`, joined | `Ok`, split |");
    println!("| --- | --- | --- | --- | --- | --- |");
    for (tear, row) in TEARS.iter().zip(&table) {
        println!(
            "| `{tear:?}` | {} | {} | {} | {} | {} |",
            row[0], row[1], row[2], row[3], row[4]
        );
    }
    assert_no_misread(&table, "seeds 1..=2000");
}
