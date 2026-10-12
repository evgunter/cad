---
id: a-same-side-tangent-touch-refuses-at-every-op
kind: issue
title: A tangent pair that leaves its shared edge on one side and touches only along it refuses every op: the declared-Tangent short-circuit marks a germ whatever the op
status: closed
opened: 2026-10-11
priority: P1
cost: M
design: true
refs: [a-flush-quarter-round-refuses-an-odd-crossing-count-at-its-rim-vertex, two-discs-kissing-externally-refuse-classification-invariant]
branch: contacthold/same-side-tangent-touch
pr: 4781
closed: 2026-10-12
---


## Finding

PR 4704's dual review (R1 MAJOR, R2 MINOR). Both reviewers executed this.

In `recl::resolve_edge_edge`, the declared-`Tangent` short-circuit marks
a germ whatever the op, wherever a glued `Tangent` flank pair leaves the
common edge on one side. PR 4704 kept the short-circuit for those pairs
only. A same-side pair that meets only along the line still refuses at
every op, on the PR head and on its base:

- `review_4704_flush_round_oracle.rs` `sameside-corner`, and
  `review_4704_r1_probes.rs` `r1_same_side_beyond_the_rim`
  `beyond-corner` (slab `x ∈ [2, 5]`, `y ∈ [0, 4]`): every op refuses
  `ClassificationInvariant` (odd crossing count);
- `sameside-flush` and `beyond` (the slab's `y` running past the
  round): every op refuses `CurvedBooleanUnsupported`, a separate stop.

TANG's `two-discs-kissing-externally-refuse-classification-invariant`
is the same short-circuit on a cylinder pair.

With the short-circuit removed, so that the membership decides every
tangent pair (measured on `contacthold/flush-quarter-round`):

- `sameside-corner`'s ∖, ∖ and ∩ answer right against R2's oracle
  (volume, tier 3, point membership);
- its ∪ builds a two-solid `Assembly` at `vs + vr`;
- the external kiss's ∪ builds the same kind of result (two solids at
  2π);
- over topo, sweep and editor-core (both reviewers' always-off runs; R1's also covered pncad, mesh, stl and step-import) only two pins move: `wedge_end_doors`' kiss row
  and `verbs_cylcyl_r1_review_probes`' externally tangent walls;
- the m9_3 rims and the capsule stay green.

The kiss `Assembly` records only the two vertex–vertex touches at the
ruling's ends. It has no record of the line the walls touch along.

## The question

What should a ∪ produce where two operands touch only along a tangent
line, with the composed material at wedge 0/2π?

- **(A) A two-solid `Assembly` with the touch recorded.** D1 tier 3′
  admits touching results, and D1's record rule asks for every pair
  whose interiors meet, the line included. This would need the carry
  to record the ruling's edge–edge (or face–face tangent) pair, which
  it does not today.
- **(B) A typed refusal.** `crates/topo/README.md` C7 routes a 0/2π
  rim to the cusp family, defined but unbuilt (`RimCuspArmUnbuilt`).
  `docs/MATE-7-TANGENCY-DESIGN.md` Q1 says the same. The
  `TangentSlitArmUnbuilt` doc says the same of the doubled cusp a
  tangent union leaves, and admits ∖ and ∩. Neither variant fits as it
  stands: `RimCuspArmUnbuilt` is raised where no locus derives, and
  `TangentSlitArmUnbuilt` where the locus runs through a face's
  interior. So (B) would raise one of them, adjusted, or a new variant,
  from `resolve_edge_edge`, under ∪ alone, where a same-side tangent
  pair finds no germ.
- **(C) Keep the short-circuit** (the current state): op-blind, and
  `ClassificationInvariant` at every op.

Whichever of (A) or (B) is chosen, the short-circuit goes. TANG's
kissing-disc row and the two pins re-baseline to that answer.

The `bool_dir_same` side test inside the short-circuit, and its
`Coincide::CurvedFlankSense` escalation (unreachable today, unpinned),
belong to the same code. Whatever replaces the short-circuit decides
whether a same-side test survives at all, and that escalation goes or
gets its own decision route with it.
