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
- **The chart speed (designers first)**:
  `ssi-chart-speed-usability-boundary`,
  `limb-3-chart-tube-speed-has-neither-guard-its-sibling-site-has`,
  `ssi-tube-pad-folds-both-axes-by-max-speed-where-limb-3-proved-per-axis`
  and `ssi-certify-stretch-divides-by-a-norm-with-no-sqrt-up` are four
  findings about one value: the chart speed, where it is minted, what it
  bounds, how it is rounded and who checks it. They go to a designer
  pair as one problem before any lane builds them.
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
