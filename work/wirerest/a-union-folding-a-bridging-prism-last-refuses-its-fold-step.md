---
id: a-union-folding-a-bridging-prism-last-refuses-its-fold-step
kind: issue
title: A union that folds union_pinch_member_order's bridging prism last refuses UnionFoldStep (topo CrossingsAlongAContactLine)
status: open
priority: P3
cost: H
refs: [an-edge-crossing-two-wedges-about-a-contact-line-is-unbuilt]
opened: 2026-10-20
---

Filed by WIRE's `wire-nameinv` lane (branch `wire/name-invariance-checks`).
This row is the editor-core witness of
`work/tangtail/an-edge-crossing-two-wedges-about-a-contact-line-is-unbuilt.md`,
whose rows are topo's own (`three_solids_on_one_line.rs`).

## What

`crates/editor-core/tests/union_pinch_member_order.rs::bridging` is the
plate, two sector prisms about (1.5, 1) (0°–50° and 120°–170°, touching
along the vertical line through it), and a third prism over 30°–140°
that bridges the gap between them. As `Union [plate, a, b, bridge]`,
the six member orders that fold the bridge last (index 3 last) refuse:

    UnionFoldStep { member: 3, read: .., refusal: Boolean(CrossingsAlongAContactLine { .. }) }

The other 18 orders build a 114-name table. As a pair, the bridge
against the union of the rest refuses
`Boolean(CrossingsAlongAContactLine)` in both operand orders.

`crates/editor-core/tests/name_invariance.rs` admits these cases against
this row:
- `bridging / union` in `a_member_order_moves_no_name` (6 orders);
- `bridging / a far member` and `bridging / after an unrelated node`,
  which fold in the listed order and so refuse on both sides. They are
  vacuous, comparing no table.

The pair case cites the tangtail row directly.

## What it needs

The tangtail row's fix: topo building an edge whose wedge crosses two
coincident edges on one ray of a vertex pair. When it lands, the
admission and the two vacuous entries go red and come out.

## Moved to WIREREST (2026-10-20)

WIRE measured 69 budget points against 30. This row is P3 or P4, and WIRETAIL already stood at 25 of 30, so it moved by `git mv` (id and body unchanged) to WIRE's second tail. Signed (WIRE orchestrator).
