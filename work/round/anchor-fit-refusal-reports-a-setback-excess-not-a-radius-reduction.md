---
id: anchor-fit-refusal-reports-a-setback-excess-not-a-radius-reduction
kind: issue
title: The anchor-fit refusal's number meters the setback, and the recourse an author follows is un-metered
status: open
opened: 2026-09-13
priority: P1
cost: E
---


Found by the BLEND unit 11 dual review (PR 2495), by execution, in both
lanes independently.

## The finding

`CornerReason::AnchorOutsideTrimmedExtent { side, carrier, setback, available }`
(`crates/profile/src/path.rs`) renders "tangent setback {setback} m exceeds
the {available} m the anchor pins — reduce the radius or move the anchor".
The number an author reads, `setback − available`, is the reported leg's
overrun in the SETBACK metric. It is not a radius amount: a tangent setback
does not scale 1:1 with the fillet radius on either leg kind (linearly with
`1/tan(θ/2)` on a straight leg, non-linearly through the offset-circle
intersection on a circular one), so the recourse an author is most likely to
follow — "reduce the radius by that much" — is un-metered by the payload.

## The measurement (both reviewers, grid A of `tests/common::grid_a`)

Over the 3 185 anchor-fit entries grid A reports, reducing the radius by
exactly `setback − available` (with `f64::EPSILON` of slack):

- **469** entries: not a request — the number is not below the radius;
- **2 111** entries: the same corner still refuses with the same reason — the
  number was too SMALL (median about 2×, up to 29× at authoring 3608, where
  0.0013 m is reported and 0.0384 m is needed; authoring 120 reports
  0.00228 m against 0.0481 m needed);
- **605** entries: the loop builds — the number was too LARGE (overshoot
  1.0016× at authoring 4831 to 27× at authoring 10829, where 0.208 m is
  reported and 0.0077 m already builds);
- **0** entries where it is the reduction that makes the corner fit.

The census rows are `tests/review_fillet_overrun_nearest_fit_r1_probes.rs`
and `tests/review_fillet_overrun_nearest_fit_r2_probes.rs`;
`tests/fillet_recourse_followability.rs`'s fit row (`straight_leg(1.9, 0.05)`)
is an unremarked overshoot instance of the same class.

## What the sites say now

The pick reports the candidate nearest to fitting in the setback metric on
its worse leg (`crates/profile/README.md`, "The refusal envelope", one level
down), and every site states that the number is not a radius amount and the
recourse is un-metered. That is true and followable — "reduce the radius" in
any amount the author chooses, and iterate — but it is not the metered
recourse the sentence's shape invites.

## What would close it

A payload an author can follow in one step: the largest building radius at
the reported corner (a bisection the door can run at `f64` on the diagnostic
channel, or a closed form on the straight-leg class), rendered beside or
instead of the setback number — or no number at all beside the recourse.
Either is a payload-shape change the editor renders, which is why it is a
residue rather than part of unit 11. Not decided here.
