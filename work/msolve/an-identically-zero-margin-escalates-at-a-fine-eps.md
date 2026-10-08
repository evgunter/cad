---
id: an-identically-zero-margin-escalates-at-a-fine-eps
kind: issue
title: At eps = 1e-12 an identically-zero mate membership margin escalates unboxed, so no certified run over a pinned assembly can certify
status: parked
priority: P2
cost: M
design: true
opened: 2026-10-03
blocked_on: [intent-stage3-is-built]
refs: [MSOLVE-14, a-box-over-a-solved-clocking-widens-thirty-thousandfold]
---

## Finding

MSOLVE-14 (PR 3986) runs the mate solve at the evaluation's own scalar.
At `Interval` the fold re-measures, through `coset::member_of`, margins
that are zero by construction: the constructed candidate's own residual
against each coset it was built to satisfy, and a true checked offset's
residual (`solve::check_offsets`). An enclosure of an identically-zero
margin straddles zero by the lane's rounding, so where the zero band is
narrower than that rounding the predicate escalates, with no box at all.
This is the spec's §5 (b) ("an identically-zero margin cannot
converge"), disclosed by the unit and scheduled here.

## Measured

An unboxed `Interval` run over the MSOLVE-14 fence corpus
(`crates/editor-core/tests/msolve14_run_scalar.rs`, `corpus()`; the
table there is `INTERVAL_ESCALATIONS`). At ε = 1e-12 (zero band 1e-12,
escalate band 1e-11), six of the thirteen documents escalate; at 1e-9
and 1e-6, none.

| Document | Predicate | Enclosure |
|---|---|---|
| coaxial-clocked-rest | `mate_member_rotation_identity` | `[0, 1.10e-12]` |
| gauge-chain | `mate_member_translation_zero` (a true checked offset) | `[0, 1.04e-12]` |
| two-pin | `mate_member_axis_fixed` | `[0, 1.31e-11]` |
| cross-pin | `mate_member_axis_fixed` | `[0, 1.41e-12]` |
| shaft-clocked-rider | `mate_member_rotation_identity` | `[0, 1.17e-12]` |
| shaft-parallel-pin | `mate_member_axis_fixed` | `[0, 2.74e-12]` |

## Consequence

At ε = 1e-12 a certified run (`clearance` with its interval leaf, or any
box driver) over almost any pinned assembly cannot certify, and a box
driver cannot bisect it: narrowing a box does not narrow rounding, so
every sub-box escalates the same way. On main these documents solved at
`f64` and were lifted as constants, so this is a practical regression at
the finest ε row, though every escalation is honest (the enclosures are
sound).

The same margins over a box are box-wide, not rounding-wide: the shaft
in two bores (`c2_a_shaft_in_two_coaxial_bores_solves_in_every_lane`)
escalates the rest's `mate_member_translation_in_plane` over
`[-0.5, 0.5]` for a box of ±0.25 on the rest's rise, so its rows box a
fraction of ε. `a-box-over-a-solved-clocking-widens-thirty-thousandfold`
is the extreme case of that.

## Directions (a design question)

- Do not re-measure a residual the construction meets exactly; check
  only the constraints the candidate was not built from.
- Or evaluate the membership margin in a centred (mean-value) form over
  the box, so an identically-zero residual encloses to second order.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage3-is-built`, not on the whole program: the escalation is member_of and check_offsets re-measuring zero-by-construction residuals; stage 3 decides overconstraint by subgroup algebra and closes checked offsets as assertions. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)
