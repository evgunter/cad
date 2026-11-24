---
id: an-in-band-pinch-corner-at-the-gate-ends-in-the-kernel-limit-recourse
kind: issue
title: An in-band pinch corner at the boolean's finished-body gate ends in the kernel-limit last resort, though its margin scales with the operands' tilt
status: open
opened: 2026-11-22
priority: P3
cost: M
design: true
refs: [the-door-gates-other-in-band-findings-are-typed-the-kernels]
---


## What

The residue of `the-door-gates-other-in-band-findings-are-typed-the-kernels`,
re-measured on main `fd7e5f47bf` at ε = 1e-9 (TALLY lane, 2026-11-22). Over
the full `near_tangent_census_probe` (28 884 runs, default tilts), the
boolean's finished-body gate refuses 6 runs, each with the single finding
`PinchCornerEscalated` (predicate `pinch_corner_turn`, all `cp S`), point
margins in band:

| run | margin |
|---|---|
| `Ltop e0 a10 d1e-9` | 1.2185e-9 |
| `Ltop e0 a11 d±1e-9` | ±3.2966e-9 |
| `Lbot e1 a2 d±1e-9` | ±1.2185e-9 |
| `Lmirror e0 a10 d1e-9` | 1.2185e-9 |

In band at a point value, the finding ends in `KERNEL_LIMIT_RECOURSE`
("loosen the tolerance, as a last resort; this refusal may indicate a
kernel bug"). ONEHOME's fork 259 (PR 6820) chose that ending on purpose,
treating the undecided pinch corner as a decision with no lever of its own.

**Evidence that the margin belongs to the geometry (read from the code, not
pinned).** The formula is `n·(a×b)/|a|` (`validate.rs` `wedge_holds_named`).
Its rounding is a few ulp × |b|, about 1e-15 at extent ~4. The margin grows
with the operands' tilt, at about 1.2–3.3 × d, so the corner really does
turn by about the tilt. Other in-band findings already end on the
operands' lever ("move the geometry … or tighten the tolerance") through
`own_close`, even where their conditioning has not been shown.

## The question

What should an in-band ending say when its margin has neither a certified
reading nor a conditioning proof? Should the pinch corner gain a geometry
lever? That would revisit fork 259, so designers weigh it first. Any answer
has to keep Ev's PR 6326 ruling that a door does not reify "in band"
against "everything else": the typing lives in each finding's own ending.
