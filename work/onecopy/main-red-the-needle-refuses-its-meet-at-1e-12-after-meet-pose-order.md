---
id: main-red-the-needle-refuses-its-meet-at-1e-12-after-meet-pose-order
kind: issue
title: Main red at 1e-12: the band-turn needle refuses its meeting count since PR 6492 (meet-pose-order)
status: dispatched
opened: 2026-11-04
priority: P0
cost: E
branch: onecopy/p0-needle
---


Filed by the PCERT orchestrator. PCERT's spline-chart incidence lane (PR 6498) found it, and the orchestrator bisected it.

## What

At ε 1e-12, `sweep::all band_turn_overrun_fillet::the_needle_builds_then_refuses_at_the_turn_as_it_flattens` fails at `crates/sweep/tests/band_turn_overrun_fillet.rs:406`:

    the needle at 3°: tier 3′, CensusUnsupported { Edge(3v3), Edge(28v3) }:
    ContactLane(NotCertifiable { "the meeting reading of two curved edges could not settle a count" })

## Bisect (first-parent merges on main, `CAD_TOLERANCE_EPS=1e-12`, `cargo nextest run -p sweep --test all`)

| main merge | result |
|---|---|
| `fd1c1949a5` (PR 6490, onecopy/face-view) | PASS |
| `e1ebc557f0` (PR 6492, onecopy/meet-pose-order) | FAIL |
| `c5d98b4eeb` (PR 6485) | FAIL |

So PR 6492 brought it in. The row is expected to build the needle and refuse at the turn as it flattens. At this ε, the meeting reading instead refuses the census at tier 3′ with a count it cannot settle, which is a live wrong answer from a public verb's census. Every PR that runs sweep at 1e-12 is red on it.

## Owed

Root-cause the meeting reading's count at the needle's 3° pose under 6492's pose order, fix it, and confirm the row at all three ε.

Precedent: the same row refused the same way at the needle's 1° pose on 2026-10-19 (`work/band/main-red-band-turn-overrun-needle-refuses-its-census-at-eps-1e-12.md`, closed), so this census reading has gone red at this ε before.
