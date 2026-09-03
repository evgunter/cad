//! The counterexample-SEARCH half of the ray-torus root claim: poses
//! nobody chose, against the same geometric oracle its enumeration
//! sibling uses.
//!
//! It lives in its own file because a marker gates a whole file's
//! module, and `r1_probes.rs` next door holds deterministic pins — the
//! regime enumeration and the two `cbrt` rows — that must keep running
//! on every leg. Gating those to buy this one sweep would be the wrong
//! half of the trade.

#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

// Gated to the code it tests (TCOST-1), as `memories/test-suite-cost.md`
// requires of every fuzzer. The sweep draws ray poses at random and holds
// the CERTIFIED root count and the roots themselves against a geometric
// oracle, so its claim is `line_torus_roots` and the `cbrt` chain it calls
// — both in `solid_contain.rs` — plus the oracle and tally it borrows from
// `r1_probes.rs`. Named wider than that: the band a certification is
// decided against and the tolerance it is written in units of live in
// `geom-core`, and `linalg` carries the `Point3`/`Vec3` a pose is, so a
// change to any of them moves what this row asserts with `topo/` untouched.
test_utils::gated_to![
    "crates/topo/src/boolean/solid_contain.rs",
    "crates/topo/src/boolean/r1_probes.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/linalg/",
];

use geom_core::{Point3, Vec3};

use super::r1_probes::{SHAPES, Tally};

/// A varying seed, because this is the search shape: successive runs
/// explore new poses instead of replaying one lattice forever. The
/// count is on the workspace `CAD_FUZZ_EFFORT` dial, shipped at the
/// smoke level a gated run should cost; `CAD_FUZZ_EFFORT=60` restores
/// roughly the ray count the fixed lattice used to run.
#[test]
fn r1_generic_poses_agree_with_the_geometric_oracle() {
    use test_utils::fuzz;
    let mut rng = fuzz::start("boolean::r1_probes::generic_poses");
    let per_shape = fuzz::scaled(4);
    let mut tally = Tally::new();
    for (rr, r) in SHAPES {
        for _ in 0..per_shape {
            let o = Point3::new(
                rng.range(-3.0, 3.6),
                rng.range(-2.0, 2.2),
                rng.range(-3.0, 2.2),
            );
            // Uniform on the sphere by rejection from the ball: a
            // direction drawn per-component and normalized would
            // over-weight the cube's diagonals, which is the bias the
            // retired lattice's arithmetic directions already had.
            let dir = loop {
                let v = Vec3::new(
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                );
                let n = v.norm();
                if (1e-3..=1.0).contains(&n) {
                    break v / n;
                }
            };
            tally.compare("generic", rr, r, o, dir);
        }
    }
    tally.report("generic poses");
    assert!(
        tally.bad.is_empty(),
        "{} disagreements with the oracle at generic poses — {}",
        tally.bad.len(),
        fuzz::replay()
    );
}
