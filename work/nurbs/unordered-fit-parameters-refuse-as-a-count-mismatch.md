---
id: unordered-fit-parameters-refuse-as-a-count-mismatch
kind: issue
title: geom fit: parameters that are not clamped and ascending refuse as ParamCountMismatch { params: n, points: n }
status: open
opened: 2026-10-04
priority: P4
cost: E
---


(SSI implementer on `ssi/banded-fit`, 2026-10-04, from PR 4023's review.
Inherited, not introduced: PR 4023 gave the rule one home and kept its
refusal.)

`check_ordered` in `crates/geom/src/curves/fit.rs` is the fit's one home
for its parameterization rule (clamped `0 → 1`, strictly ascending,
NaN refusing). Every door reaches it: `Collocation::new`,
`interpolate_with_params` and `approximate_with_params` through
`check_params`, and `interpolate_columns`. A parameter vector that
breaks the rule refuses as `FitError::ParamCountMismatch { params: n,
points: n }`: a count mismatch whose two counts are equal. The refusal
names the wrong fault, and its recourse (match the counts) is not the
repair.

Fix shape: a variant of its own (`FitError::ParamsNotClamped { index }`
or similar) naming the first offending parameter, with a recourse
("supply parameters from 0 to 1, strictly ascending"). `FitError`'s
recourse row and its matchers move with it.

## Re-homed from FLUX to KNOT (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. KNOT collects the spline, net and fit doors in `geom-core` and `geom`: refinement inside an enclosure, the net-state reads, the NURBS derivative and projection doors, and the fit's refusals and solves. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.
