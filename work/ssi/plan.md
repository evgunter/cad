# SSI — the plan

the plane×NURBS surface intersection: its bounds, its tubes and the diagnoses that survive a bad one

Opened 2026-09-20 by CHART's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

12 rows, 29 budget points against a ceiling of 30 (priced 2026-10-01).
`python3 scripts/work.py status --program ssi` is the live table.

## Units

Each unit is a branch `ssi/<unit>` and one PR; the review tier is named
here at dispatch.

- **`ssi/lever-arm-fold`**: `ssi-lever-arm-min-fold-hides-poison`. The
  lever-arm folds propagate a poisoned operand, at one home, and the
  shape sweep covers the crate (`dihedral.rs`'s `folded_lever_arm` is
  the same fold outside SSI's paths). Review: single FULL, because the
  claim is that no NaN can survive the fold at any site, which takes a
  sweep to believe.
- **`ssi/diagnoses`**: the refusal-side rows, which all decide what a
  refusal names and how it ends under D4 ¶1:
  `plane-nurbs-ssi-misblames-control-net`,
  `plane-nurbs-certificate-escalation-does-not-name-its-limb`,
  `ssi-transversality-death-says-lower-the-tolerance` and
  `ssi-refusal-prose-outgrows-the-viewer`. Combined into one PR because
  runners are a budget and the four share one file and one routing
  table. Review: single FULL, because routing a refusal to the wrong
  ending is a correctness defect that reads as plausible prose.
- **The chart speed**: four rows, one design. The design was weighed by
  two designers and converged in three rounds without going to Ev; the
  spec is the "Design" section of `ssi-chart-speed-usability-boundary`.
  There are three sequential units, each building on the last:
  - **`ssi/chart-rate`**: one outward `norm_sup` in `geom_core`, with
    `Box3::speed_sup` deleted, plus NaN-keeping folds on
    `SupSpeed`/`InfSpeed`. Closes `ssi-certify-stretch-divides-by-a-norm-with-no-sqrt-up`
    and the `offset_meters` NaN-dropping folds. Review: single FULL,
    because it moves bits on a soundness claim.
  - **`ssi/chart-tube`**: the per-axis mint, the per-kind certificate
    tube, `chart_tube_windows`, and the NaN-window door. Closes
    `limb-3-chart-tube-speed-has-neither-guard-its-sibling-site-has` and
    `ssi-tube-pad-folds-both-axes-by-max-speed-where-limb-3-proved-per-axis`.
    Review: single FULL.
  - **`ssi/chart-floor`**: the floor door on both lanes, and the
    stepper's guard on its own step. Closes
    `ssi-chart-speed-usability-boundary`. Review: single FULL.
- **`ssi/eps-refine`**, after the first wave:
  `plane-nurbs-certificate-bound-does-not-refine-with-eps`. Measure
  first, as the row says, and then fix or document what the measurement shows.
- **The probes**: `a-rigid-map-re-derives-the-plane-nurbs-edge-certificate-in-a-frame-that-moves-it`
  and `chart-uniform-breaks-lean-on-exact-dedup` are both "reproduce,
  then decide". They ride with whichever lane is closest when it
  lands.

## Review posture

Protocol v7's A/B triage lapsed with the model A/B experiment
(suspended 2026-09-23, `memories/experiments.md`). This track uses the
review tiers in `memories/orchestration-model.md`.
