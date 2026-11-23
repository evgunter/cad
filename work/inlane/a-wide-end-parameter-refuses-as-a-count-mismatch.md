---
id: a-wide-end-parameter-refuses-as-a-count-mismatch
kind: issue
title: fit::check_ordered refuses a wide end parameter as ParamCountMismatch, a wrong name
status: open
opened: 2026-10-19
priority: P3
---


Found by reviewer A of C6 S5c (PR 5332). It predates S5c. Every not-taken finding gets its own item (the INLANE orchestrator's rule).

## Finding

`check_ordered` (`crates/geom/src/curves/fit.rs:335-349`) refuses every way the parameterization rule fails with `FitError::ParamCountMismatch { params: n, points: n }`, a count mismatch whose two counts agree. The failures it covers:
- an end that is not the point `0` or `1`, including a wide end at an enclosure scalar;
- a step that does not ascend for every member;
- a NaN parameter.

The message names neither the parameter nor the rule.

## Owed

A refusal of its own, for example `FitError::ParamsNotOrdered { index, why }`. It should name:
- the offending parameter's index;
- whether an end is not the point 0 or 1, or a step is undecided (with its margin, as C6 asks of a refusal members may disagree on).

Also a row per arm, and the pncad-py tag for it.
