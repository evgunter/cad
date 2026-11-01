---
id: a-vertex-in-an-arcs-gap-within-eps-of-its-carrier-exhausts-the-rays
kind: issue
title: a loop whose representative vertex stands within ε of another loop's arc carrier, in that arc's gap, refuses RayCastingExhausted
status: open
opened: 2026-10-15
priority: P3
cost: E
---



Found by the review of #5175 (its NOTE-3). The same on `main` (the PR's
merge base `70ec028a81`) at 1e-9, 1e-6 and 1e-12.

## The witness

A 10 × 10 square with two holes. One is the arc of the unit circle
about the origin from 0.01 rad through 6.2 rad, closed by its chord,
leaving a gap of 0.083 rad about angle 0. The other is the arc of the
circle about `(d, 0)` of radius `1 − d + g` from −0.005 through 0.01
rad, closed by its chord, so it sits in the first arc's gap, its
vertices within ε of the first carrier. At `d = 17ε` and `g = ±0.5ε`,
ε = 1e-9, `f64` and `Interval` both refuse
`RayCastingExhausted { loop_index: 1, against_loop: 2 }`.

The pair pass reads the two holes clear (their arcs are 5e-3 rad
apart at the nearest), and the small hole lies outside the large one:
the profile is valid.

## The likely mechanism (not root-caused)

`point_in_loop` casts every ray from the loop's lex-min vertex, which
stands within ε of the large arc's carrier. Each ray meets that carrier
at its origin, `ray_advance` reads Zero, and the ray grazes, though
the point lies in the arc's gap where no span holds it. Every one of
the `N_RAYS` rays does the same.

## Band

P3: a typed refusal of a valid profile whose loops sit within ε of
each other's carriers, with no recourse but moving the geometry
(`profile-ray-casting-exhaustion-is-a-copy-of-the-ray-walk` owes that
sentence). Reading the span before the graze, or casting from a point
off every other carrier, would settle it.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILEREST (`work/profilerest/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.
