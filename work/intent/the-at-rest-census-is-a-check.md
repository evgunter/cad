---
id: the-at-rest-census-is-a-check
kind: issue
title: D10 stage 5 PR C: the at-rest census is a check resident; contact and interference findings, quiet or loud; A5's gate and Separation retire
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [an-assertion-relates-by-equality, interference-at-rest-is-a-finding, value-decided-coincidences-have-no-recording-door, mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion]
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
`mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion`
(stage 3) as the stage-level triggers that exist today. Re-point them to
stage 3's and stage 4's last units when those are filed.

Design forks open: FORK-S5-4 and FORK-S5-5 (spec §11).
