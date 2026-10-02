---
id: thread-count-digest-moved-on-main-loft-area-pads
kind: issue
title: mass_props_are_thread_count_invariant's serial golden is red on main at every eps: the two NURBS lofts' area pads moved down since the last cut, and #3524 is the suspect
status: closed
opened: 2026-10-01
priority: P1
cost: E
refs: [3524, 3737]
closed: 2026-10-01
pr: 3751
branch: reach/threadgolden-red
---


## What

`crates/sweep/tests/mass_props_are_thread_count_invariant.rs`,
`the_walk_matches_the_serial_golden_at_four_threads` (slow set; nightly
only unless a diff touches `sweep`), fails on `origin/main`
`978feeb296` at ε = 1e-9, 1e-6 and 1e-12. Found by #3737's fix pass:
red there at all three ε, and **main's own tree, run against the
digests #3737's tree produces, passes** — so the move is main's, not
#3737's. Removing #3737's hull meet from `algebra::convex_step` also
leaves the new digest unchanged.

What moved (`crates/sweep/tests/thread-count-digest/eps-*.txt`):

- `loft_prism`, at every ε: `a` `4039524a6c672f4c` → `4039524a6c672fe1`
  (149 ulps), `apad` `3fc84532b9a396b8` → `3fc84532b9a1b030`
  (0.18961175980 → 0.18961175979655, down 1.8e-11 of itself).
- `arc_loft_1e9eps`, at ε = 1e-9 and 1e-6: `apad` down 4.2e-11 of
  itself (`3fb5fb0bfc3b7d20` → `3fb5fb0bfc377c50` at 1e-9,
  `40f4f65eba1950a0` → `40f4f65eba156940` at 1e-6).
- Nothing else: volumes, volume pads, refusals, verdict hashes and the
  sym-session counts are unchanged.

## Suspect, not bisected

The digests were last cut at `b74adc4a5e` (the #3527 branch, merged at
08:40 UTC−7). #3524 (`props/convex-insert`, the convex Boehm step,
merged 09:48) is the first later merge that changes NURBS refinement
arithmetic, and a pad that shrinks on two NURBS lofts is what a form
reading each coefficient once would do. #3725/#3727 (LINALG doors,
sqrt) are the other candidates in that window. A first-parent bisect
over `b74adc4a5e..978feeb296` settles it.

## What is owed

Bisect, confirm the moving change is right (the pads go DOWN, which is
the licensed direction under H5's ruling 2), and re-cut the three
digests with the cause named in `expected`'s re-cut log, as that
function's docs require.

## Closed

PR 3751. The first-parent bisect clears #3524: its merge `ba06ed4bf3` reads the old digest. The first bad merge is #3727 (`43333da8ae`, `linalg/certification-sqrt`), whose parent `a266a5a796` reads the old digest and whose own tree reads main's.

#3727 replaced `props/quad.rs`'s local `sqrt_enclosure` with the backend root (`Certification::sqrt`). The old root stepped one ulp outward unconditionally, and the backend root steps only where the root is inexact, so the area pads tightened.

The one- and four-thread digests are byte-identical on main at all three ε. Both moved enclosures still contain the independently computed true areas: 25.312438347381 m² for `loft_prism` (Simpson), and s²·(18 + π(1+√2)) for the arc loft. The digests are re-cut, and the cause is named in `expected`'s log.
