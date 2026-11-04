---
id: main-red-band-turn-overrun-needle-refuses-its-census-at-3-degrees-at-eps-1e-12
kind: issue
title: Main red at ε 1e-12: band_turn_overrun_fillet's needle at 3° refuses its census (ContactLane NotCertifiable on two curved edges)
status: open
opened: 2026-11-04
priority: P1
cost: M
---


Found by `pcert/projected-one-inverse` (PR 6500) running the `ci` profile locally at `CAD_TOLERANCE_EPS=1e-12`. It is not that PR's: the row fails identically at its base, main `c5d98b4eeb`.

`sweep` `band_turn_overrun_fillet::the_needle_builds_then_refuses_at_the_turn_as_it_flattens` fails at `crates/sweep/tests/band_turn_overrun_fillet.rs:406`, verbatim:

    the needle at 3°: tier 3′, [CensusUnsupported { subject: Entity(Edge(EdgeKey(3v3))), cause: ContactLane(NotCertifiable { what: "the meeting reading of two curved edges could not settle a count" }) }, CensusUnsupported { subject: Entity(Edge(EdgeKey(28v3))), cause: ContactLane(NotCertifiable { what: "the meeting reading of two curved edges could not settle a count" }) }]

The same row went red at 1° on 2026-10-19 (`work/band/main-red-band-turn-overrun-needle-refuses-its-census-at-eps-1e-12.md`, closed). Now 3° refuses the same way.

## Owed

Find the main commit that turned the 3° needle red at 1e-12, and either fix the census's curved meeting reading there or re-band the row with the reason.
