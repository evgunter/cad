---
id: a-cusp-refused-at-the-close-is-filed-against-the-close
kind: issue
title: A cusp that doubles back over its leg is refused at the close, and filed against the close rather than the cusp step
status: open
priority: P2
cost: M
design: true
opened: 2026-11-20
---


Found by the sweep in the PR that files fillet refusals against the
fillet's own step (`PathError::Deferred`, branch
`profile/fillet-refusal-names-its-step`).

`Core::build` (`crates/profile/src/path.rs`) runs
`Core::cusps_leave_on_arcs` when the loop closes. That check refuses
`PathError::CuspDoublesBackOverLeg { at }` for every cusp on a straight
leg whose leaving segment came out straight, and the replay files the
refusal against the closing step (`walk`'s `ReplayError::raised_at`).
That step authored neither the cusp nor the leg that leaves it. The
pinned rows are `crates/profile/tests/cusp_fillet_keeps_its_leg.rs`
(`cusp_then_fillet_*`).

The other close-time re-reads (each fillet arc's stored form, and each
arc's read-back) now file against the step they are about. This one
cannot yet:
- `Core::cusp_on_a_line` records only the vertex (`cusps_on_lines`), so
  nothing at the close knows which step put the cusp there;
- the refusal carries no step.

## What a fix has to decide

Which step the refusal is about. Two candidates:
- the **cusp's** step, whose recourse is to drop the cusp;
- the step whose leg **leaves** it straight, whose recourse is to leave
  on an arc.

The refusal's own sentence would settle it. Once that is decided:
1. record that step beside the vertex in `cusps_on_lines`;
2. wrap the refusal with `PathError::deferred` as `build` does for the
   arc read-back.

`Deferral` would then need an arm for this check.
