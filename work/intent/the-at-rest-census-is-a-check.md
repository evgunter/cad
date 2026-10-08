---
id: the-at-rest-census-is-a-check
kind: issue
title: D10 stage 5 PR C: the at-rest census is a check resident; contact and interference findings, quiet or loud; A5's gate and Separation retire
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [an-assertion-relates-by-equality, interference-at-rest-is-a-finding, value-decided-coincidences-have-no-recording-door, a-mate-on-a-pinned-copy-refuses]
needs_ev: true
---


INTENT stage 5, PR C. Spec: `docs/INTENT-STAGE5-SPEC.md` §4.

A5's at-rest gate (`assembly::assemble`,
`crates/editor-core/src/assembly.rs:1171`, `verdict` at `:1273`) becomes
a check-registry resident `CheckId::AtRest` (DISCIPLINES DS6). Its
findings are:

- `Contact`: stage 4's at-rest unproven coincidences;
- `Interference`: B's.

Each finding is quiet or loud under the quieting rule, whose contact half
lands here: `Gap = 0` or `Distance = 0` over the two face sites. Where
the census has no lane, the resident reports that it could not look.
`assemble`, `Assembly` and `AssemblyError::AtRest` retire, and so does
the `Separation` resident (FORK-S5-5). The viewer's at-rest badge,
Python's `assemble` and the tour gallery's check rows are restated.

It waits on stages 3 and 4 as well as A and B:

- the contact findings are stage 4's records, and stage 4 retires the
  declared attribution that is the rest of today's gate;
- the resident checks per space, which is stage 3's.

The `blocked_on` names
`value-decided-coincidences-have-no-recording-door` (stage 4's door) and
`a-mate-on-a-pinned-copy-refuses` (stage 3's F, the unit that closes
`mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion`
and A11 (4)'s declaring mates). Re-point the stage-4 trigger to stage 4's
last unit it needs.

Design forks open: FORK-S5-4 and FORK-S5-5 (spec §11).

FORK-S5-4 and S5-5 were weighed as one fork (FORK-S5C, fork log row
91) and went to Ev in an `[ev]` PR; this unit builds on the answer
provisionally. Nothing at rest refuses, and the registry's
`enforce_checks` is the one refusing door, at a caller's `Error`. An
at-rest contact the door cannot prove structural is a finding of the
`unproven-coincidence` lint, not of `AtRest`. `AtRest` reports
interference and could-not-look, and defaults to Warn. `Separation` and
`assemble` retire. When this unit lands, A5's opening and *Interference.*
paragraphs become: "**A5 — The at-rest check.** Per space, the census
examines every pair of copies the boxes cannot prove apart, and decides
each one apart, in contact, overlapping or undecided. A contact is
recorded at the coincidence door; unless it is structural, the
`unproven-coincidence` lint reports it. An overlap is an `AtRest`
interference finding, localised to the faces bounding it, or loud and
unquietable when the intersection refuses. An undecided pair is an
`AtRest` could-not-look finding. Each finding is quiet under D10's rule
or loud. Nothing refuses: a caller that wants a gate runs
`enforce_checks` at `Error`."
