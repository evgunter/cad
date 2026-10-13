---
id: wall-wrapper-curvature-headroom-keeps-its-own-zero-wording
kind: issue
title: topo: shell.rs's wall wrapper words CurvatureHeadroom{Zero} as 'within tolerance … ({reach} m)' while the meter's payload says 'not shown below … (margin …)'
status: open
opened: 2026-10-13
priority: P4
cost: E
---

(Filed by the ENCL orchestrator from PR 4976's delta review, NOTE 2.)

## What

`crates/topo/src/shell.rs`'s wall wrapper of `MeterError::CurvatureHeadroom { verdict: Zero }` (the "a wall this thick is within tolerance of the face's radius of curvature …" literal) composes its own sentence over the meter's numbers. Since PR 4976, the meter's own payload for the same arm reads "the offset distance's magnitude is not shown below the face's curvature radius on its bending side, {reach:e} m (margin {m:e} m), so the offset may fold".

So one decision is now worded two ways:
- the wrapper's "within tolerance" claims more than a Zero verdict carries, because a proved zero or straddle is not "within tolerance";
- the wrapper prints `{reach} m` in plain form and no margin.

## Ask

Render the meter's payload, or reword the wrapper in the same idiom ("not shown … (margin … m)"), and pin it whole.
