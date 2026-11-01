---
id: a-bridge-met-with-the-wedges-and-the-other-bridge-names-vertices-on-no-single-seam-alike
kind: issue
title: A bridge intersected with the union of the wedges and the other bridge refuses with the kernel-defect sentence: vertices sharing one name lie on no single seam
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

`union_pinch_member_order::two_bridges_short` is the wedges `a`
(0°–50°, z 0.12–2) and `b` (120°–170°, z 0.1–1.7), touching along the
vertical line through (1.5, 1), and two bridges over 30°–140°, `d`
(z 0.2–0.4) and `c` (z 0.6–1.5). As a pair intersect,
`Intersect [d, Union [a, b, c]]` refuses in both operand orders, and so
does `Intersect [c, Union [a, b, d]]`:

    name emission found a kernel bug — an invariant it relies on does not hold:
    several vertices of a union share one name once they cite member edges
    whole, and that name is not a single seam to rank them along. There is
    no way through: this is a kernel defect; report it

The sentence is `CITED_GROUP_NOT_ONE_SEAM` in
`crates/editor-core/src/names/emit_union.rs`. The same pairs as unions
are `a-pair-union-of-a-bridge-listed-first-and-the-wedges-offers-a-seam-one-edge-twice`.

`name_invariance.rs` lists both intersect cases as vacuous against this
row: each compares two refusals.

## What it needs

Measure which vertices share the name, and why the name is no single
seam. The kernel-defect wording says the emitter assumed it could not
happen. Then either rank them along something the name does hold, or
change the refusal to say what the author can do.

Closed by PR 5535 (2026-10-26). The union refusal was a kernel record duplicate: `seam_edges` listed one edge twice, and `ops.rs` now dedups it (`SeamEdges`). The intersect refusal was emit_union re-spelling N2's corner copies bare; the copies are now named by `name_corner_copies`. `name_invariance`'s operand-order admissions and VACUOUS cases are empty.
