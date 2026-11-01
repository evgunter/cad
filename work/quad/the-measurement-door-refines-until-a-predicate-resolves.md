---
id: the-measurement-door-refines-until-a-predicate-resolves
kind: unit
title: Build [tolerance.measure]: one measurement door refines until the caller's predicate over the enclosure resolves, refinement stops paying, or one weighted work wall; no default; the round caps and QuadratureBudget go
status: closed
opened: 2026-10-22
priority: P1
cost: H
closed: 2026-10-26
pr: 5741
---


Ratified on `[ev]` PR 5574 (2026-10-22). DESIGN.md `[tolerance.measure]`
and `crates/geom-brep/README.md` C12(7) state the target; fork-log row 188
records how it was weighed. The build:

- **One door.** It refines each face until the caller's predicate over
  the enclosure resolves, refinement stops paying (a floor no round
  removes), or one per-face work wall, weighted so one unit costs the same
  in every lane (a v-exact cell is about 30× a composite one, measured). It
  returns the narrowest round reached (never the last; see
  `a-quadrature-loop-returns-its-last-round-not-its-narrowest`) and why it
  stopped.
- **The predicate is closed data at the public doors:** `Sign`,
  `Relation { op, bound }`, `Contains`, `Resolution(len)`. There is **no
  default**: every caller states its own (Ev). A deciding predicate is a
  named, recorded funnel decision that settles when definite, refines while
  straddling, and stops in the band. The width predicate is a sizing,
  unrecorded. No resolution finer than ε.
- **The deciders route through it:** check 7, shell roles, the boolean
  volume backstop, and `editor_core::checks::connectedness`.
  `SignCertificate::assembled` and `PastTarget::refine` become one
  multi-body walk. Whether the internal `settle` closure goes is the
  unit's call, once every in-tree caller fits the closed type.
- **Goes:** the four round caps, `LAST_ROUND_EVERY_LANE_RUNS`,
  `QuadratureBudget`, the after-round-0 exit (`budget_exit`'s refusal),
  `TargetUnreached`, the loosen-ε recourse, and `props_quad_converged` /
  `props_quad_last_round` as funnel verdicts.
- **Public doors take a required predicate:** `mass_properties` and
  `measure` in Rust, `pncad-py`, and step-import's `ImportReport`. The
  tests and the tour state theirs.
- **Assertions** join when a volume/area `MeasurePrimitive` exists.
  `MinClearance` stays window-refined.

It dissolves, and each closes into this unit when it lands:
`quadrature-budget-conflates-its-lanes-and-budgets`,
`quad-last-round-margin-has-no-sign-premise`,
`a-sign-walk-escalates-on-the-last-rounds-in-band-reporting-reading`,
QUADTAIL's `quad-round-capped-loops-do-not-read-the-shared-stop-rule`,
COINCIDE's `connectedness-reads-shell-roles-through-the-reporting-walk`,
and `face-run-is-a-state-machine-kept-by-convention`. `C3`'s constant
family lands here.

Design-first: slice it with a spec before dispatch. It is broad (topo
props, geom-brep quad, pncad, pncad-py, step-import, the tour), and its
review is DUAL.

## Spec (2026-10-22)

`docs/QUAD-MEASURE-SPEC.md`. It sets three units, each landing green:
- **A (lanes):** the stop rule, the weighted wall, the narrowest round. DUAL, letter H.
- **B (topo):** the `Settle` door and the deciders. DUAL, letter H.
- **C (public doors):** no default, and every consumer states its predicate. Single FULL.

It also carries the orchestrator's rulings R1–R11.

## Closed

Built in three units: A (the lanes, PR 5589), B (the door and the
deciders, PR 5628) and C (no default, PR 5741). Every public number
door takes a required `Settle` and answers `MassMeasurement { enclosure,
stop }`; `QuadratureBudget`, `TargetUnreached`, `budget_exit`, the
round caps and the interim sites are gone; an unmet resolution is the
bracket, with its stop; a sign still open at the wall is check 7's
`VolumeSignUnresolved`, carrying the walk's bracket. The acceptance walk
(items 1–8) is in PR 5741's body.
