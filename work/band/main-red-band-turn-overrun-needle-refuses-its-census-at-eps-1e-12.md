---
id: main-red-band-turn-overrun-needle-refuses-its-census-at-eps-1e-12
kind: issue
title: Main red at ε 1e-12: band_turn_overrun_fillet's needle at 1° refuses its census (ContactLane NotCertifiable on two curved edges)
status: closed
opened: 2026-10-19
priority: P1
cost: M
closed: 2026-10-19
---


Found by FLUSH's PR 5119 lane while sorting that PR's reds on a merge
with main. At `CAD_TOLERANCE_EPS=1e-12`, `sweep`
`band_turn_overrun_fillet::the_needle_builds_then_refuses_at_the_turn_as_it_flattens`
fails identically on main `88f431ad6` and on PR 5119's merged tree. It
is not 5119's.

The failure (`crates/sweep/tests/band_turn_overrun_fillet.rs:406`),
verbatim:

    the needle at 1°: tier 3′, [CensusUnsupported { subject: Entity(Edge(EdgeKey(3v3))), cause: ContactLane(NotCertifiable { what: "the meeting reading of two curved edges could not settle a count" }) }, CensusUnsupported { subject: Entity(Edge(EdgeKey(28v3))), cause: ContactLane(NotCertifiable { what: "the meeting reading of two curved edges could not settle a count" }) }]

**What is owed.**
- Find the main commit that turned it red. PR 5119's CI first showed
  it after merging main e953ab92 (2026-10-15 onward).
- Say whether the census should certify this meeting at 1e-12 or the
  test's expectation should change, and fix it on its owner's ground.

## Closed

A duplicate of CPAIR's
`work/cpair/curve-pair-polish-meters-refuse-shallow-meetings.md`, which
carries this failure with the other two 1e-12 rows that share its
refusal ("the meeting reading of two curved edges could not settle a
count"), bisected by CONTACTHOLD's nightly-red lane to PR 5227 (all
three pass at its first parent `f4ebb63a56` and fail at `37edb540a9`).
The meter is CPAIR's ground; the fix and the test's expectation are
decided there.
