---
id: containment-margin-backstop-unreachable-behind-the-screen
kind: issue
title: fillet: the ring-clearance CONTAINMENT margins are bit-identical to predicate 2's sampled screen on every coaxial pair, so the exact backstop has no reachable fixture
status: open
opened: 2026-09-08
---


Found by BLEND-6's Phase 2 while writing the two refusing rows its spec
asks for (`docs/BLEND-6-SPEC.md` §4, "a REFUSING row per form ... each
asserting `RingClearance` with a negative reading"). Neither row can
assert that today, and the reason is arithmetic rather than fixture
luck.

## The coincidence

`ring_clearance_pass`'s two new relations
(`crates/sweep/src/blend/surgery.rs`, `CircleMargins`) are

- containment of a ladder rim's trim circle in its host's circular outer
  boundary, `R − si`, at `surgery.rs:2151`;
- containment of a hostless annulus rim's host RING in that rim's host
  trim, `si − (d + aj)`, at `surgery.rs:2098`.

Predicate 2's boundary-pair consumption sweep
(`crates/sweep/src/blend/battery.rs:1910`–`:1950`, deciding
`fillet3_face_clearance` at `battery.rs:817`) meters
`gap − setback_here − setback_there` over the SAME face's boundary
loops, with `gap` the minimum distance over `CHAIN_SAMPLES` points on
each edge. On two COAXIAL circles those two expressions are the same
real number:

- ladder: `gap = R − a` and the requested rim's setback on the plane is
  `si − a`, so `gap − setback = R − si`;
- annulus: `gap = 1 − a` and the setback is `r`, and the trim is at
  `si = 1 − r`, so `gap − setback = si − a`.

and the sampled `gap` is EXACT there, because nine samples on each of a
circle's two arcs put a sample pair at a shared azimuth.

Measured, at the head of PR 2215:

| body | derived containment | screen's reading | site |
| --- | --- | --- | --- |
| boss, flat top narrowed to `0.55`, dome rim at `r = 0.1` | `-0.04160797830996155` | `-0.041607978309961546` | `fillet3_face_clearance` |
| boss, dome grown to `0.92`, top rim at `r = 0.1` | `-0.02` | `-0.020000000000000018` | `fillet3_face_clearance` |
| boss, dome grown to `0.95`, top rim at `r = 0.1` | `-0.05` | `-0.04999999999999993` | `fillet3_face_clearance` |

The screen runs first, so on every fixture in this class it answers and
the exact closed form never gets the question. Both refusing rows in
`crates/sweep/tests/blend6_ring_clearance.rs` therefore assert the
screen's site and its reading, and the door's two-tolerance trio for the
relation (`m5_pr12_refusals::trio_hostless_annulus_ring_containment`)
escalates typed with the FACE recourse rather than the ring one.

## Why it is not a defect in this unit's change

The exact form is a BACKSTOP, and the invariant it exists for is
one-sided: sampling never reads a gap SMALLER than the true one, so
nothing the exact form would pass is refused by the screen. Before this
unit the pass CONTRADICTED the screen — the external form read
`−(si + R)` on a nested pair the screen had passed, and refused a carve
that was fine (`ring-clearance-refuses-a-nested-trim-circle`). After it
the two agree wherever both are defined. That is the fix; the
unreachability is a statement about the CORPUS, not about the form.

## What it costs, and the ask

Three things have no red row today:

1. a mutant that swaps the two containment relations for each other
   reds nothing, because neither is the predicate that answers;
2. the ladder outer-cycle walk's `Curve3::Circle` arm
   (`surgery.rs:2144`–`:2156`) has no fixture that reaches it with a
   negative margin at all, so its `max(external, containment)` is
   unexercised on the refusing side;
3. `FILLET3_RING_RECOURSE`'s own claim — that a ring inside the blend's
   setback is what this predicate refuses on — is followable only
   through the screen for these two relations.

The reachable construction is a NON-coaxial ring: predicate 2 then reads
a gap strictly larger than the true one when the closest approach lands
between samples (the precedent is
`review_fillet_e2_probes`'s 30-degree-turned dimpled prism, which
reaches `fillet3_ring_clearance` at the front door for the EXTERNAL
form). For the containment relations that means a plane host carrying a
small circular ring off the axis — a pip cut into the top of a
pole-touching revolve, which needs a boolean and was outside this unit's
Phase 2 budget. The ask is one such fixture per relation, with the
sampled gap and the exact margin both recorded, so the backstop has a
row that reds when it is wrong.
