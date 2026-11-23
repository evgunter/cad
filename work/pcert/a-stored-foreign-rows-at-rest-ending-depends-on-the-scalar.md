---
id: a-stored-foreign-rows-at-rest-ending-depends-on-the-scalar
kind: issue
title: A stored projected row that is not its edge's carrier ends differently at rest by the scalar: a MapResidual defect at f64, the Fidelity envelope's tolerance offer at Interval
status: open
opened: 2026-11-04
priority: P2
cost: M
---



Filed by PCERT unit E4 (`pcert/circle-general-lane`, PR 6485), from the
delta verifier's probe (`review/6485-delta`,
`review_6485_delta_stored_foreign_row_at_rest`), kept as the row
`pcurve_cache::circle_lane_rows::a_foreign_stored_row_never_ends_not_yet`.

## What

A projected row minted and certified for a carrier `A`, re-checked at
rest (`PcurveCache::recertify`) against its edge's carrier `B`, `A`
turned about the chart's axis by `1e-6` to `1e-2` rad, so the row is
not `B`'s while `B` is as much on the chart as `A`: the row is wrong,
and refuses at both scalars, but by different checks.

| carrier (on its chart) | `f64` | `Interval` |
|---|---|---|
| torus crest circle | `ResidualExceeded { MapResidual }` | `ResidualExceeded { EnvelopeTerm(Fidelity) }` |
| cone ruling, clear of the apex | `ResidualExceeded { MapResidual }` | `ResidualExceeded { EnvelopeTerm(Fidelity) }` |
| cone tilted section | `ResidualExceeded { MapResidual }` | `ResidualExceeded { EnvelopeTerm(Fidelity) }` |

The same at every one of the three angles, so a row a `1e-2` turn (about
2 cm here) off ends, at rest, as a sampled miss (`Unsized::Fit`, the
stored image does not represent the carrier) at `f64`, and as the
envelope's bound (`Unsized::Bound`, whose recourse offers a tolerance)
at `Interval`. The user reads opposite advice for the same row
(`Unsized::recourse`, `crates/geom-brep/src/recourse.rs:283`):

- at `f64`, a definite `Fit` miss at rest ends in the defect ending:
  "There is no way through: this is a kernel defect or a damaged file;
  report it" (`KERNEL_OR_FILE_DEFECT_ENDING`);
- at `Interval`, a `Bound` ends in the last resort on every arm:
  "Recourse: loosen the tolerance, …" (`KERNEL_LIMIT_RECOURSE`). For a
  row up to 2 cm wrong, that tells the user to loosen the tolerance
  past the error rather than report a damaged file.

The delta verifier's probe is
`review_6485_delta_stored_foreign_row_at_rest`
(`crates/geom-brep/src/pcurve_cache/review_6485_delta.rs` on
`review/6485-delta` @ aeda72a69). The routing is `PcurveCheck::routing`'s, before E4 and
after.

Which check reads the miss first differs by the scalar: the schedule's
sampled residual (check 3, `MapResidual`) at `f64`, check 4's fidelity
term at `Interval`. Why check 3 does not refuse first at `Interval` is
not established here.

## Question

Which ending a stored foreign row owes at rest, and whether it may
depend on the scalar the at-rest pass runs at. A wrong stored row is a
stored contradiction, which suggests one ending at both scalars (the
fit's, or a defect), whatever check reads it first.

## Not

E4's lane relabel (`minted_refusal`) reads only the mint's own fresh
image, so a stored row never ends "not yet"; the row above pins that.
