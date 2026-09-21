# SSI — the plan

the plane×NURBS surface intersection: its bounds, its tubes and the diagnoses that survive a bad one

Opened 2026-09-20 by CHART's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**26 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `limb-3-chart-tube-speed-has-neither-guard-its-sibling-site-has` | E | limb 3's chart tube divides by a chart speed with neither the zero nor the non-finite guard plane_nurbs_ssi refuses on, and both collapses certify silently instead of refusing |
| P0 | `ssi-certify-stretch-divides-by-a-norm-with-no-sqrt-up` | H | ssi/certify: the chart stretch divisor is a raw sqrt of an f64 fold, so it is not an upper bound |
| P0 | `ssi-chart-speed-usability-boundary` | D | ssi: wrong diagnoses survive at finite-but-unusable speeds — the usability boundary is ~5.6e-312, not 0, and both guards test only the class |
| P0 | `ssi-lever-arm-min-fold-hides-poison` | H | ssi - the curvature lever arm is folded with f64::min, so a poisoned operand cannot reach the transversality guard's NaN arm |
| P1 | `plane-nurbs-certificate-bound-does-not-refine-with-eps` | H | Limb 2's between-samples bound is a function of a FIXED sample schedule, so an exact intensional description refuses at small enough epsilon |
| P1 | `plane-nurbs-ssi-misblames-control-net` | D | ssi - plane_nurbs_ssi blames the wall's control net for the PLANE's own non-finite origin |
| P1 | `ssi-tube-pad-folds-both-axes-by-max-speed-where-limb-3-proved-per-axis` | H | plane_nurbs_ssi pads its uniqueness tubes by max(su, sv) on both chart axes where limb 3 proved a per-axis pad, and the comment says they are the same region |

## Order

`ssi-lever-arm-min-fold-hides-poison` first: a curvature lever arm
folded with `f64::min` lets a POISONED operand pass as the smaller of
two, which is the one row here that returns a confident wrong answer
rather than a loose one. `ssi-certify-stretch-divides-by-a-norm-with-no-sqrt-up`
is the same shape one layer down — a raw `sqrt` of an `f64` fold used
as a divisor is not an outward bound — and the two are likely one
sitting.

Then `ssi-chart-speed-usability-boundary`, the design question under
both: where the boundary between a usable and an unusable chart speed
lies, and what a diagnosis may claim on the far side of it.

## Review posture

OPEN, for this program's first dispatch. CHART inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
