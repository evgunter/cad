---
id: MSOLVE-7
kind: unit
title: member.rs residue: one nominal environment per solve, the axis datum's refusal sited where it was raised, the flat index's account closed by citation, and the mate wire's one deny_unknown_fields hole
status: closed
opened: 2026-09-19
branch: msolve/7-member-residue
closed: 2026-09-19
pr: 2885
---


Spec: `docs/MSOLVE-7-SPEC.md`. Gathers `plan.md` items 10 and 14:
`mate-solve-rebuilds-the-nominal-environment-per-check`,
`axis-datum-names-the-pattern-where-the-evaluation-names-the-
transform`, `part-over-a-nested-pattern-reads-the-flat-index-at-check-
reference` (EVAL-6 landed the decomposition and the four-case row;
what remains is the check's account and the `(j, i)` question, ruled
against in the spec) and `mate-primitive-accepts-a-stray-field-the-
module-docs-say-refuses`. Small; no verdict moves. Dispatches from
main once the spec is in.

## Closed (2026-09-19, PR 2885)

Landed: `solve_document` builds the document's nominal environment
once and hands it down through a borrowed per-solve context; the
evaluator hands its own environment to `solve_with_env` so nothing
under evaluation builds a second one. `node_value_kind` walks a
transform chain by id, seats a dangling input and a non-placeable
operand at the transform that reads it, and returns the seated pair
(`eval::Seated`); `axis_datum` inherits the seat, so every refusal of
a circular rule's axis operand is seated where the evaluation seats
it — seven seat rows assert both roads, including the transform-over-
datum shape the correctness arm found reachable through `apply`. The
flat-index item closed by citation (EVAL-6's decomposition and its
four-case row; the walk keeps naming a copy by the flat `Part`).
`MatePrimitive` carries `deny_unknown_fields`; the census re-baselined
(`mate.rs` 3); the stray key refuses at the load door and loaded
before the attribute. Reviews on `6e27dac6a`: correctness C1–C4 HOLD
with one MINOR (the reachable two-seat shape, fixed), style no MAJOR;
the thirteen-item fix pass landed. Filed here: `mate-solve-carries-
the-cluster-maintenance-half`, `mate-primitive-unit-variants-load-
from-a-null-payload`, `gauge-of-recomputes-the-clusters-per-placement-
lookup`; on WIRE: `node-value-kind-answers-boolean-through-a-transform-
where-the-value-is-a-body`. The spec is deleted into
`docs/DOC-LEDGER.md` (recoverable at the unit head named there).
