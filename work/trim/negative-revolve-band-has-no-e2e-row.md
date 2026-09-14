---
id: negative-revolve-band-has-no-e2e-row
kind: issue
title: The [0, tau]-intersection mutant on a cylinder root has no e2e row: no revolve replays at Interval
status: open
opened: 2026-09-13
refs: [clearance-window-tightening-needs-chart-boundary]
---



## What

TRIM-3 PR-2's `window_of` takes a cylinder's tightened `u` from the
description's metred hull VERBATIM when that hull spans no more than a
turn, and never as `[0, τ] ∩ hull`. The spec's E7 row exists to kill
that second spelling, which is meaningless mod `τ` and empties a band
that runs negative. **PR-2 ships the rule with no e2e row that kills
it**, and the two measurements that leave it that way are:

1. **No revolve replays at the interval scalar over an ε-scaled box on
   this tree.** Both M10-5 R1 revolve fixtures — the y-axis quarter
   annulus and its z-axis control — refuse at the SELECTION door
   (`node did not build in this leaf's replay`), so no revolved band
   ever reaches `window_of`. R1 already carried the skip as a printed
   note; PR-2 tried to turn the z-axis control into an assertion and
   had to put it back.
2. **An extruded arc's band is never negative.** The mint puts `u_ref`
   at the arc's start, so the band is `[0, θ]` with `θ > 0` and
   `[0, τ] ∩ hull` is the identity on it. Measured on PR-2's own
   scallop fixture (a semicircular notch, bulge −1): the witness on
   the arc reads `u = π/2`, `v = −1` — the axis came out `−ẑ` and the
   azimuth positive again.

So the only construction that discriminates is a negative-angle
revolve (`revolve/mod.rs`, the `[θ, 0]` band), and it cannot be
evaluated at the lane the engine runs in.

## Fix shape

Whichever comes first: a revolve that replays at `Interval` over an
ε-scaled box (the 1191 class, not this program's), or a unit-level row
that reaches `window_of`'s root rule without a document — which today
means making the rule a named function the suite can call, rather than
an arm inside a private `window_of`. The second is cheap and is
probably the right answer regardless: a root rule with two branches
and a periodicity argument deserves to be addressable.

## Home

TRIM — the rule is PR-2's, in the announced seam.
