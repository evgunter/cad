---
id: a-crescent-refusal-fires-where-short-arcs-carry-the-section
kind: issue
title: The crescent refusal fires where short arcs either side of the tip carry the section, refusing a union main builds
status: closed
opened: 2026-10-26
priority: P1
cost: M
closed: 2026-10-28
pr: 5949
branch: splitdoor/crescent-short-arcs
---


Found by PR 5687's claims review. Pinned by
`crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
`short_arcs_at_a_crescents_tip_refuse_the_union`.

## The pose

The boss `r = 0.3` and the through hole `r = 0.1` about `(0.2, 0)` of
`a-union-across-a-tangent-crescent-refuses-unbuilt`, each rim four arcs
with vertices at the tangent point and either side of it: the boss's at
azimuth `−s, 0, +s`, the hole's at `−3s, 0, +3s`. At `s = 1e-5` the short
arcs each side are about `3e-6` m long and lie within the band of each
other. `union` in either order refuses `TangentCrescentUnsupported`;
`main` before the refusal built it `SOUND`, one shell, every sampled point
right (default ε, `s = 1e-5` and `1e-6`; at ε 1e-6, `s = 1e-4`, `A ∪ B`).
At ε 1e-6 and 1e-12 the `s = 1e-5` profile itself escalates.

## Why it fires

At the tip the shared bounds are the two short arcs, whose opening over
their reach (`≈ 3e-11`) reads zero: no crescent there. It fires at the
vertex pair where the short arcs end (`sectors.rs` `crescent`): the two
long arcs leave that pair about `2e-5` rad apart, and `parallel_same`,
levered at the sector arm (the short arc's `3e-6` m), reads them one
tangent; the opening is then read over the long arcs' reach. The section
reaches the region through the short arcs, which lie on each other, so
the union builds.

## What does not narrow it

- Skipping the pairs `pair_search` recorded does not clear it (the
  review's experiment): the firing pair is one it reads as touching.
- Reading the shared bounds' parallelism at their own reach would clear
  this pose (`2e-5` rad over the long arcs' `0.2` m is far past the
  band), and with it the poses where only one rim carries the short arcs
  (`s = 1e-5`, `1e-6`, either rim), which `main` ships as two shells:
  instrumenting `crescent` shows their refusal, too, fires only at the
  far vertex pair, on the long arcs.

What tells the two apart is whether the short arcs on both sides lie on
each other, so that the coincident run carries the section to the far
vertex pair. The refusal would have to read that run, not the pair alone.

## Slate triage (2026-10-26, CLEAVE)

- Moved to SPLITDOOR (`work/splitdoor/`) by `git mv`, id and body unchanged: the P1 track opened for split and boolean shapes that refuse, or ship a side their own gate fails, where a uniform reading would build: the split door's tier-3′ bar, tangent and pinch poses, and the curved-carrier lanes that read lines and circles only. Band P1 as filed. CLEAVE keeps its siblings from PR 5687 (`a-union-across-a-tangent-crescent-refuses-unbuilt`, `one-segment-tangent-rims-refuse-where-their-two-arc-forms-build`); whoever takes one reads the others. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Closed (2026-10-28, PR 5949)

`sectors.rs` `crescent` skips a bound pair only where the corners' other edges are a coincident
run (lines or arcs of at most a half-turn, far ends a recorded vertex pair, tangent at their own
reach) and the read bounds part at their own reach; the short-arc unions build SOUND in one shell
at their closed form (pin `short_arcs_at_a_crescents_tip`, renamed from
`short_arcs_at_a_crescents_tip_refuse_the_union`). The one-rim-only poses this row named escalate
`Crossing(OnEdge)` upstream on today's main. FULL single review (mergeable after fixes; the reflex
bound and the 1e-12 build fixed in the fix pass).
