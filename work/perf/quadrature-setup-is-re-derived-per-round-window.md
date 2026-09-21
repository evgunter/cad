---
id: quadrature-setup-is-re-derived-per-round-window
kind: issue
title: a certified face lane re-derives its round-independent setup on every round window
status: open
opened: 2026-09-12
priority: P4
cost: D
---


## The finding

PERF-6 made the certified face lanes enterable over a `RoundWindow`
(`crates/geom-brep/src/props/quad.rs`), so tier 3's check 7 can stop at
the round its sign settles and a caller wanting the number can continue
from there. The ROUNDS compose exactly — a face run as `0..=k` then
`k+1..=...` evaluates the pieces an uninterrupted run evaluates, once
each, which `crates/sweep/tests/sign_certified_plus_v.rs` counts through
`k_stats` (`gate + refine == one`).

What does NOT compose is everything a lane does BEFORE its round loop.
A window enters at the lane's front door, so each entry re-derives that
face's round-independent setup:

- `nurbs_patch_face` (quad.rs, the block before its loop): the
  derivative grids `su`/`sv`/`suu`/`svv`, the global second-derivative
  hulls, the interior-knot lists, the position-magnitude bound, and
  `last_round_width_lo` over the schedule's LAST round's cut lists;
- `rational_patch_face`: the same, plus the homogeneous nets, the
  per-block hull grid and the `w`-uniform-in-v decision;
- `cylinder_cut_face`: the least of the three — its per-round work is
  the whole of it.

Measured (release, 4 vCPU, medians of 3): gate-then-continue is
1.3-1.8x one measurement in wall time (1766 vs 969 ms at `1e7*eps`;
3457 vs 2662 ms at `1e9*eps`) although the piece evaluations are
identical. That is the setup being paid twice, and it is paid once per
WINDOW, so a body whose sign settles late pays it once per round.

Against the two full quadratures a gate-then-measure caller used to
pay, 1.3-1.8x is still a saving. It is not the saving the composition
argument promises.

**WHICH BODIES PAY IT — the regime, because one number reads as all
of them.** The factor above is what a certificate that stopped
PART-WAY costs. A certificate whose `settle` never accepted runs the
schedule to its end, leaves no face open, and its continuation
re-enters no lane at all: gate-then-continue is then 1.0x one
measurement and the whole second quadrature is saved. Both regimes are
in the tour (PR 2440's measurement, release, 4 vCPU, medians of 3, at
eps = 1e-9):

- polynomial walls, sign undecided to the end — 14 measurable bodies
  of the tour's 61 tier-3 stops: gate 3.048 s, measure 3.034 s,
  gate+continue 3.053 s. The door is 1.00x one measurement and the
  pair's 1.99x is all saving.
- rational walls, sign settled early — the round-spout teapot
  (`demos/teapot-round-spout`): gate 11.49 s, measure 18.13 s,
  gate+continue 29.78 s. The door is 1.64x one measurement, inside
  this item's band, and the saving against the pair is NIL (0.99x).

So the consumer-visible saving is a property of the body's schedule,
and closing this item is what would make it unconditional.

**How many faces pay it, measured** (PERF-11, release, 4 vCPU under
the slot, medians of 3, the setup term read at ONE thread as
`gate + continue − one measurement`, where the two doors are not
competing for workers):

| body | faces | open | continuation @1t | setup term | share |
|---|---:|---:|---:|---:|---:|
| arc loft @ 1e11·eps | 6 | 0 | 0.0 ms | — | — |
| `loft_prism` | 6 | 0 | 0.0 ms | — | — |
| arc loft @ 1e9·eps | 6 | 1 | 2169 ms | 493 ms | 23% |
| arc loft @ 1e7·eps | 6 | 1 | 685 ms | 680 ms | 99% |
| `quintic_prism` | 6 | 4 | 8005 ms | 639 ms | 8% |

And over the tour's 61 tier-3 stops, the census of what a continuation
actually resumes: **58 leave no face open at all**, two leave two
(`tiltedcut_above`/`below`, 2.4 ms each) and one leaves eight of ten
(`teapotspout`, 15.3 s — the whole of the tour's continuation cost).

So the regime split this item already states holds, and the share of
the continuation this item would remove runs from 8% to 99% depending
on how much schedule the open faces have left. PERF-11 divided the term
by the pool width instead of removing it (the continuation resumes its
open faces as an indexed parallel map): on `teapotspout`, through the
tour, 16.2 s → 4.1 s at four threads, while at ONE thread the same
body is 14.2 s → 14.7 s — the width is the whole of the saving, and
this item's term is untouched by it. A body with ONE open face gets nothing
from that width and is exactly this item's subject — the arc loft at
1e7·eps, 99% setup, unchanged at any width.

## What a fix is

A per-face prepared object the lane builds once and rounds read from —
so `RoundWindow` addresses rounds of a PREPARED face rather than
re-entering the lane. The natural shape is the one the round loops
already imply: everything above the loop becomes the constructor,
`round(r)` becomes the method, and `SignCertificate` holds the prepared
faces rather than re-flattening the body's loops.

Cost to weigh before it moves: the prepared object holds per-face
grids and hulls for every face of the body at once, where today one
face's setup is live at a time.

## Not in scope for a fix

The `cylinder_cut_face` lane, which has no meaningful setup, and the
exact per-span arm of `nurbs_patch_face`, which answers before any
round runs and is entered at most once per face by construction.
