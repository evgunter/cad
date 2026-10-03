---
id: a-sweep-row-fails-under-all-features-on-main
kind: issue
title: sweep::tilted_sphere_pair_k_rows::a_carved_balls_meridian_fragments_record_no_clearance_charge fails under --all-features on main
status: review
opened: 2026-10-03
priority: P2
cost: E
pr: 3957
branch: cleave/all-features-row
---


Reported by the `cleave/tangent-interior-refuse` lane (PR 3938, 2026-10-03): the row fails under `cargo test --all-features` on main as well as on that branch, so it is not that PR's. CI's gate does not run `--all-features` for sweep, which is why it is green. Not measured further: reproduce on main, find which feature flips it, and route to the owner of `tilted_sphere_pair_k_rows` (REACH's clearance charge, by name).

## Measured (CLEAVE lane, 2026-10-03, origin/main `82b9ceb2`)

`cargo test -p sweep --all-features --test all -- tilted_sphere_pair_k_rows`
fails at the row's anti-vacuity floor, at ε 1e-6, 1e-9 and 1e-12 alike:

    no clearance was asked: the row would be vacuous

The feature is `probe` alone (`--features probe` reproduces it); the
suite is `#![cfg(feature = "probe")]`, so without it nothing compiles.
No feature changes an answer here.

**Cause: a stale predicate name.** The row filters K samples on
`"bool_circle_curved_clearance"`. #3805 (REACH, conic edges against a
curved face) renamed the recorded predicate to
`"bool_conic_curved_clearance"` (`topo::boolean::reduce`'s
`conic_clearance`); the row (aafc1d15) was written against the old
name and merged beside it, so it now matches no sample. Renamed in the
row, it passes at all three ε: the clearance is still asked of the
poses' decidable arcs, and every margin it records clears the metre
floor.

**Why green CI let it sit:** the PR gate never executes a probe suite;
only the nightly `k-lint (dev-probe)` row does, through
`scripts/k_probe_sweep.sh`'s plain loop. That gap is already
`work/ciw/tests-red-under-all-features-never-run-by-ci.md`.
`work/topo/the-carved-balls-clearance-row-is-vacuous-on-main.md` is the
same finding, filed by TANG; it closes with this PR.
