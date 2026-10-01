# LINALG — the plan

geom-core's vectors, frames and interval conventions: the answers that are wrong at Interval

Live state is in `python3 scripts/work.py status --program linalg`; the
log carries the narrative.

## Order

1. **`pole-branch-pick-two-integer-shift`** (P0, H). This is the one
   P0 left. Measure first. The question is downstream of the strict
   branch pick: at `Interval`, does `shift_branch`'s consumer get a
   non-singleton shift silently? If it does, does a typed refusal or a
   hull belong there? Classify it under the D2 addendum.
2. **The missing doors, as one lane.** Each row has its fix written in
   its body:
   - `geom-core-linalg-has-no-array-doors`. It needs the
     `Vec3`/`Point3` array pair as well as the `Mat3`/`Affine3` one,
     plus the `Affine3` read-out doors; leaving either half out is a
     half-fix.
   - The sup-norm half of `point3-has-no-order-and-vec3-no-sup-norm-door`.
   - `linalg-escalations-offer-a-declaration-the-door-cannot-take`.
   - `svd-sigma-folds-re-spell-real-min-max`.
3. **`certification-gains-a-sqrt-door`** (P2, M). CURVED specified it
   in its body. It crosses QUAD, ENCL and TESS ground by announced
   seam.
4. **Two design questions, weighed by the designer pair before any
   lane:**
   - whether `Point3` carries an order, or only a named comparator
     (the order half of `point3-has-no-order-…`);
   - whether `geom_core` carries a public literal constructor below
     `pncad::authoring` (`coordinates-lifted-into-a-point-…`).

   Both are surface questions that could go either way. They go to Ev
   only if the designers find a fork that is Ev's to decide.

## Review posture

The tiers are Ev's, from 2026-09-19 and 2026-09-23. Each dispatch
records its tier and its reason in `log.md`.
