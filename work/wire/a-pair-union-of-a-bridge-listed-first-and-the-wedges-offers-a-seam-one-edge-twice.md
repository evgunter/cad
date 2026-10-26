---
id: a-pair-union-of-a-bridge-listed-first-and-the-wedges-offers-a-seam-one-edge-twice
kind: issue
title: A pair union of a low bridge listed first and the union of the wedges it spans refuses Naming(EntityNamed): a seam is offered one edge twice
status: closed
priority: P2
cost: M
opened: 2026-10-20
closed: 2026-10-26
---

Filed by WIRE's `wire-nameinv` lane (branch `wire/name-invariance-checks`),
found by `crates/editor-core/tests/name_invariance.rs`
(`an_operand_order_moves_no_name`).

## What

`union_pinch_member_order::two_bridges_short`: the wedges `a` (0°–50°,
z 0.12–2) and `b` (120°–170°, z 0.1–1.7) touching along the vertical line
through (1.5, 1), and two bridges over 30°–140°, `d` (z 0.2–0.4) and `c`
(z 0.6–1.5). As a pair union:

- `Union [d, Union [a, b, c]]` refuses `Naming(EntityNamed)`: the seam of
  `a`'s and `b`'s walls between `d`'s two cap vertices on the line "was
  offered the same edge twice — a name's candidates are distinct entities",
  ending in the kernel-defect sentence;
- `Union [Union [a, b, c], d]` builds, 157 names;
- likewise `Union [c, Union [a, b, d]]` refuses and the other order builds.

Every 4-member order of the same union builds and names alike
(`union_pinch_member_order::a_prism_bridging_two_wedges_is_named_alike_in_every_member_order_that_builds`).

## What it needs

Measure which seam rule offers one edge twice when the bridge is operand
A: the seam between the inner union's two walls is a piece of a
`Merged`-free seam of the inner union, and its pieces between the
bridge's cap vertices are offered once per wall. Then a two-member union
that publishes one table in either order.

Closed by PR 5535 (2026-10-26). The union refusal was a kernel record duplicate: `seam_edges` listed one edge twice, and `ops.rs` now dedups it (`SeamEdges`). The intersect refusal was emit_union re-spelling N2's corner copies bare; the copies are now named by `name_corner_copies`. `name_invariance`'s operand-order admissions and VACUOUS cases are empty.
