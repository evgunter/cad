---
id: boolean-structural-refusals-drop-their-payload
kind: issue
title: The Boolean's carve, single-solid and re-cut refusals drop the error they map into a desync
status: open
opened: 2026-10-11
priority: P3
cost: E
---


## What

Found by the sweep of `cleave/folded-graze-join-desync` (the
`map_err(|_| …)` shape around the describe pass it fixed). Each site
below maps an operator's typed refusal into `JoinDesync` or
`ClassificationInvariant` and discards it, so the refusal a user sees
names the step, never the cause (which shell, which carve, which
certification check):

- `boolean/ops.rs` `apply_recuts`, the sphere re-cut: `single_solid`
  twice, `carve` twice, and
  `transform_rigid`'s re-certification ("re-cut rotation failed to
  re-certify", whose `TransformError` is dropped).
- `boolean/ops.rs` `fallback`: `single_solid` twice and `carve`.
  Note: `apply_recuts` raises `ClassificationInvariant`, the rest
  `JoinDesync`.
- `boolean/finish.rs` `setopfinish`: `single_solid` three times and
  `carve` twice.
- `boolean/join.rs` `along_edge_conic`: the split lineage's walk.

Each is a state the reduction should not reach, so the defect voice is
right; only the payload is lost. Unmeasured whether any is reachable.

## Owed

Carry the discarded error in the refusal (a sibling variant, as
`BooleanError::EdgeUndescribed` carries its attachment refusal), or
show why a site's error has nothing to add.

## Slate triage (2026-10-26, CLEAVE)

- Moved to HONEWORD (`work/honeword/`) by `git mv`, id and body unchanged: whose charter is split and boolean refusal text, payloads and endings. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
