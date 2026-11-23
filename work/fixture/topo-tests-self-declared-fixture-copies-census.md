---
id: topo-tests-self-declared-fixture-copies-census
kind: issue
title: A prose census of self-declared fixture copies in crates/topo/tests
status: open
opened: 2026-09-16
priority: P4
cost: E
---

## Finding

- **Where**: `crates/topo/tests/`, twelve sites below, of which one
  is resolved (struck through).
- **Importance**: low-medium — **this row is the census, not the
  verdict**
- **Confidence**: sure that the prose says what it says; unsure which
  ones are duplication rather than a deliberate review promotion
- **Raised by**: the style review of the `dup-brick` lane's PR (S-DUP),
  2026-09-16

A fixture copy that no construction sweep can see will often **say so in
its own doc comment**. `rg -i 'verbatim|by hand|rebuilt|op sequence of'
crates/topo/tests/` finds twelve in seconds:

| site | what the prose says |
| --- | --- |
| ~~`review_m2_pr7.rs:23`~~ | **gone (PR #2727).** The prose *"The geometric-cube op sequence of `common::geometric_cube`, but…"* and the private `mapped_cube` it labelled are both deleted; the suite's six call sites name `common::mapped_cube`, proved equal at four maps. Row `topo-tests-review-m2-pr7-rederives-the-shared-cube` closed. **One of twelve walked, eleven to go** |
| `m9_c1_r1_probes.rs:72` | *"The unit's own flush seat, rebuilt verbatim"* — see `topo-tests-straddle-seat-hand-copies` |
| `mate8_witness_schedule.rs:63` | *"MATE-4a's overhang seat, verbatim"* — same row |
| `mate8_witness_schedule.rs:84` | *"`r1_mate4a_probes`'s probe fixture, verbatim"* — same row |
| `r1_mate5_probe.rs:52` | *"Verbatim from `tests/mate5_cyl_eps_rung.rs` (the unit's own fixture…)"* |
| `r2_probes.rs:46` | *"The mate5 suite's `wall_sheet`, verbatim"* |
| `review_m3_pr3_bob.rs:35`, `:48` | (two more in one file) |
| `issue93_nested_islands.rs:2` | *"constructions adopted verbatim from the review"* |
| `r1_mate8_decomp_probe.rs:2`, `:16`, `:81` | *"VERBATIM port of `chart_region`'s private decomposition schedule"* — a port of **`src`** into a test, which is a different and possibly worse case |
| `cube_by_hand.rs:1` | *"build a unit cube by hand through the public Euler API"* — almost certainly deliberate: the suite IS the by-hand construction |
| `box_with_hole.rs:2` | *"(genus 1) by hand through the public Euler-operator API"* — likewise |

**Some of these are correct.** A review-lane probe that pins a unit's
fixture deliberately holds its own copy so that a change to the unit's
fixture reddens the probe instead of silently following it; the last two
rows are suites whose whole subject is the by-hand construction. The
work here is to walk the twelve and separate *that* from plain
duplication, then route what is left.

**Why the census is worth its own row**: the three sweep patterns the
`dup-brick` unit used (`\bfn brick\b`, `fn [a-z_]*brick[a-z_]*`,
`prism_z`) found none of these, because a copy that re-derives a
construction shares neither a name nor a builder with what it copied.
Prose is the only channel that catches it, and nothing in this tree
sweeps prose.
