---
id: tilted-cut-upper-digest-moved-by-5734
kind: issue
title: sweep thread-count digest: tilted_cut_upper's pads narrowed and midpoints moved by ulps at every ε after PR 5734; re-cut
status: closed
opened: 2026-10-27
priority: P1
cost: E
closed: 2026-10-27
branch: quad/digest-after-flux-5801
pr: 5818
---


## Finding

`mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_four_threads`
(`crates/sweep/tests/mass_props_are_thread_count_invariant.rs:430`) is red
on main at 1e-6, 1e-9 and 1e-12. Only the `tilted_cut_upper` line of
`crates/sweep/tests/thread-count-digest/eps-*.txt:3` differs; its verdict
and escalation hashes are unchanged.

| ε | v | a | vpad | apad |
|---|---|---|---|---|
| 1e-6 | held | held | `3ee1bc8cf3091556 → 3ee16a7af0134000` | `3f0a9ad36c8d2000 → 3f0a1fb8681c6000` |
| 1e-9 | `…2d18 → …2d1b` | `…c6c9 → …c6cb` | `3e812c48b2000000 → 3e81270c3c2aaaac` | `3ea9c26d0ae00000 → 3ea9ba925a200000` |
| 1e-12 | held | `…c6bd → …c6bc` | `3df11376aaaaaaac → 3df110d915555556` | `3e199d31c0000000 → 3e19994560000000` |

Every pad narrowed. No number got wider.

**Cause: PR 5734** (FLUX, `flux/cylinder-sliver-exact-bracket`, merge
`ee4a094e`), not PR 5801. Bisect on first-parent main: `eb9dc903` and
`ee4a094e~1` pass, `ee4a094e` fails with the bits above, and every later
merge through `9fb4d389` (5801's parent included) reads the same bits.
5734 changed how the cylinder lane gets `(cos, sin)` at each trim end
(`crates/topo/src/props/quad_lane.rs`). Before, it was recovered from the
endpoint vertex and padded by `ε / radius`. Now it is `quad::trig_at_limit`
at the exact limit the lane integrates from
(`crates/geom-brep/src/props/quad.rs`). That bracket is
transcendental-free and at most 3.4e-13 wide. The trig brackets narrow,
so the pads narrow, and their midpoints move, so `v`/`a` move by 1–3 ulps.

**Oracle.** The body is the upper half of a radius-0.5, height-1 cylinder,
cut by a plane through the axis at `z = 0.5` tilted 0.3 rad. Closed form:
`V = π/8 = 0.392699081698724154…` and
`A = π/4 + π/2 + π/(4 cos 0.3) = 3.178311275573694986…`. Each new bracket
holds the closed form at every ε:

| ε | \|v − V\| | vpad | \|a − A\| | apad |
|---|---|---|---|---|
| 1e-6 | 1.5e-17 | 8.30e-6 | 1.3e-16 | 4.98e-5 |
| 1e-9 | 1.5e-16 | 1.28e-7 | 7.6e-16 | 7.67e-7 |
| 1e-12 | 9.6e-16 | 2.48e-10 | 5.9e-15 | 1.49e-9 |

The rows `work/tcost/tilted-cut-upper-pads-moved-in-the-thread-count-digest-at-1e-12`
and `main-red-the-mass-props-thread-golden-since-4938`, closed by PR 5369 describe an
earlier move (PR 4952) and are already closed. This is a new move.

## Closed (2026-10-27)

Re-cut on `quad/digest-after-flux-5801`: the `tilted_cut_upper` line of
each `crates/sweep/tests/thread-count-digest/eps-*.txt` now carries the
bits above. The suite's four rows pass at 1e-6, 1e-9 and 1e-12.
