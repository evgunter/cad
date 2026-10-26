---
id: wall-wrapper-curvature-headroom-keeps-its-own-zero-wording
kind: issue
title: topo: shell.rs's wall wrapper words CurvatureHeadroom{Zero} as 'within tolerance … ({reach} m)' while the meter's payload says 'not shown below … (margin …)'
status: closed
opened: 2026-10-13
priority: P4
cost: E
closed: 2026-10-26
---

(Filed by the ENCL orchestrator from PR 4976's delta review, NOTE 2.)

## What

`crates/topo/src/shell.rs`'s wall wrapper of `MeterError::CurvatureHeadroom { verdict: Zero }` (the "a wall this thick is within tolerance of the face's radius of curvature …" literal) composes its own sentence over the meter's numbers. Since PR 4976, the meter's own payload for the same arm reads "the offset distance's magnitude is not shown below the face's curvature radius on its bending side, {reach:e} m (margin {m:e} m), so the offset may fold".

So one decision is now worded two ways:
- the wrapper's "within tolerance" claims more than a Zero verdict carries, because a proved zero or straddle is not "within tolerance";
- the wrapper prints `{reach} m` in plain form and no margin.

## Ask

Render the meter's payload, or reword the wrapper in the same idiom ("not shown … (margin … m)"), and pin it whole.

## Update (`encl/zero-arm-wording`)

The meter's payloads now read "the offset distance's magnitude is not shown below the face's curvature radius on its bending side, {reach} m (margin {m} m), so the result may fold" (Zero) and "… passes the face's radius of curvature on the side it bends toward, {reach} m (margin {m} m), so the result folds over itself" (Negative), each number through `geom_core::Readable` / `MarginDiag::readable()`, and no content word said twice. Both wrapper arms also say "face's" twice ("the face's radius of curvature … so the face's offset …"), and the Negative arm prints no margin either; the same rewording covers them.

## Closed (2026-10-26, easy-fixes lane)

`crates/topo/src/shell.rs` `AsShelled`'s `CurvatureHeadroom` arms now read "a wall this thick is not shown below the face's radius of curvature on the wall's side, {reach} m (margin {m} m), so its offset may fold" (Zero) and "… passes the face's radius of curvature on the wall's side, {reach} m (margin {m} m), so its offset folds over itself" (Negative). Each number goes through `geom_core::Readable` or `MarginDiag::readable()`, and "face's" appears once. Pinned whole by `shell::tests::the_wall_wrapper_states_the_headroom_reach_and_margin`. Verified: `cargo nextest run -p topo --lib -E 'test(/…|shell::|…/)'`: 367 passed.
