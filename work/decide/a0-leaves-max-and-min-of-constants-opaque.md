---
id: a0-leaves-max-and-min-of-constants-opaque
kind: issue
title: rule A0 folds sqrt and abs of a constant but not max or min of two constants, so an axis-aligned sign-hull frame stays an atom chain under A0 and its gate residual freezes
status: open
opened: 2026-09-21
priority: P1
cost: D
---


## What was measured (SYM-10 Phase 1, 2026-09-21)

PR #2468's fix 13 taught rule A0 to read a `Select` whose decision is
a constant. The frame it then selects is still an atom chain: at
`n = (0, 0, 1)` the read arm is `cy = vy / max(‖vy‖, s/4)` with
`‖vy‖ = sqrt(1) → 1` (A0), `s = min(‖n‖, max|n_i|) = min(1, max(0, 1))`
— and `max(0, 1)`, `min(1, ·)`, `max(1, ·/4)` are `max`/`min` of two
CONSTANT forms, which `combine` folds only when both are the zero form.
So `u = 1 / max(1, min(1, max(0, 1))/4)`, three opaque atoms standing
for the number one, multiplied into every identity over the derived
frame.

On `m10_derived_frame_interval`'s height document at `5e-2` under A0
replacing (`SymRules { const_fold: true, ..none() }`, the row
`m10_the_derived_frames_refusal_is_not_a_freeze`'s second rung): the
attachment gate's `carrier_endpoint_start` residual is `sqrt` over
three FROZEN kids, frozen 10, and refuses numerically at `[0, 0.32]`
where the row asserts frozen 0 and the newell `Invalid`. With
`max(A, B) → A` on a manifestly non-negative `A − B` hand-planted in
the plain walk (`CAD_SYM10_PLANT=1p` on the branch's `2a479c267`), the
rung reads frozen 0, one refusal, `newell_plane_residual … margin is
invalid` — the assertion verbatim. On constants that fold is A0's:
`max(c₁, c₂)` and `min(c₁, c₂)` of two exact rationals fold to the
exact rational, no value read, wherever A0 runs (replacing in the
plain walk, alongside in the early one).

## What a fix is

Extend A0's constant fold in `combine`'s `Atan2 | Min | Max | Copysign`
arm: when `node.op` is `Min`/`Max` and both kids are constant forms,
return the constant. Pin it beside `a_constant_decision_folds_to_the_arm_it_reads`.
Small; SYM-10's Phase 2 or its own row, whichever Ev's ruling on the
fourth piece (`the-candidate-norm-needs-a-canonical-square-root`)
lands first.

## Home

`crates/geom-core/src/sym.rs` (`combine`). Filed by SYM-10's lane.
