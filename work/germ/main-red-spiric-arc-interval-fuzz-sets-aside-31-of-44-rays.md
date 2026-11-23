---
id: main-red-spiric-arc-interval-fuzz-sets-aside-31-of-44-rays
kind: issue
title: main-red (ε 1e-6): topo's splitting::spiric_arc::fuzz::random_arcs_at_interval sets aside 31 of 44 rays at seed 0xeae65a327c6e4602
status: open
opened: 2026-11-23
priority: P1
cost: M
---

Found by LATHE's `lathe/skin-sub-arc-walls` on PR 7478's gate (run 45189986017, `test (kernel)`, the ε 1e-6 leg), and reproduced on `main`.

## Measured

    CAD_FUZZ_SEED=0xeae65a327c6e4602 CAD_TOLERANCE_EPS=1e-6 \
      cargo nextest run -p topo -E 'test(=splitting::spiric_arc::fuzz::random_arcs_at_interval)'

On `origin/main` at `e0f688549b` (local debug build) it fails as on the PR:

    panicked at crates/topo/src/splitting/spiric_arc/fuzz.rs:409:5:
    Interval: 31 of 44 rays set aside

The PR changes nothing under `crates/topo` against its merge base, and the fuzzer builds tori, not lofts. The seed is CI's (`effort=1`), drawn per run, so other runs pass and this is intermittent on `main`.

## Lead

The last change under `crates/topo/src/splitting/spiric_arc/` is GERM's `fef5755439` (2026-11-23, "review fix pass: P ∩ Q read in the spiric's plane with its rounding metered; a near miss in the band refuses; … witnesses for … the fuzz"). A near miss that now refuses in band would raise the set-aside count at the interval scalar. Not bisected.

## Owed (GERM)

Bisect at this seed. Either the rays set aside are honest refusals and the fuzz's bound on them is wrong at the interval scalar, or the read refuses where it should decide; fix whichever, and pin this seed as a fixture (`test_utils::fuzz::pinned_sweep`).

