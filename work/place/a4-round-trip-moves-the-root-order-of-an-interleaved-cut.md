---
id: a4-round-trip-moves-the-root-order-of-an-interleaved-cut
kind: issue
title: A4's round trip moves the root order of a cut whose roots a kept root separates
status: open
opened: 2026-10-03
cost: E
priority: P1
---


**Finding** (PR 3930's fix pass, once the R1 comparator read root order).
ASSEMBLY.md A4 says inline-of-split "returns the document split was
given, up to node ids", and the ruling on `[ev]` #3888 says that holds
on every shape. One shape split admits does not: three lone instances
x, y, z listed in that order, and a cut of x and z. Split puts the
instance at x's one root-list position (`refactor::split`, the
"A10 on the remainder" block), so inline splices x and z there and the
round trip lists z before y. Root order is the product's solid order,
semantic and in the content pin (`roots.rs` module docs).

The code says this is intended: `refactor.rs`'s module docs call it the
split amendment's rider (i), "adjudicated at review ordinal 40", and
`asm4_split_inline::root_interleaving_collapses_onto_the_instance_at_d4_identity`
pins it. No ratification by Ev turns up for the rider (`git log -S'NOT
the original interleaving'` finds the ASM-4 review fix pass,
`44d59b7b92`), and A4's current text does not carve it out.

`p2_split::r1_a_cut_whose_roots_a_kept_root_separates_collapses_the_order`
pins the disagreement as the comparator's only line.

**The fork.**
- Refuse such a cut at split (a typed arm, recourse "list the cut's
  roots together (SetRoots), then split"). Tried in the fix pass and
  reverted: it refuses ordinary cuts, because a mate is a root and is
  usually inserted after an unrelated kept instance. Three existing rows
  went red (`asm_r2a_mate_solve::row4e_…`,
  `p2_split::a_cut_of_gauges_or_a_datum_alone_refuses_no_material`,
  `rev_fix_xsplit_unreachable::no_cut_whatsoever_…`).
- A narrower refusal: only body-denoting roots carry product order, so
  refuse only when a kept body-denoting root separates two cut
  body-denoting roots, and let mates and gauges reorder. That would
  still move the content pin.
- Keep rider (i) and re-word A4 to say so ("up to node ids and the
  interleaving of the cut's roots with kept ones").

This needs Ev's ruling on which A4 means.

## RULED (2026-10-03, Ev on `[ev]` #3939)

- **The regrouping stands and A4 states it.** Split brings the cut's roots together where the first of them was, and inline-of-split returns the document up to node ids and that one regrouping. No refusal.
- **The anchor is A10's existing rule.** A node that replaces roots goes where the first of them was. Ev: either anchor works, so take "whichever rule is simpler or more elegant". The orchestrator took the existing rule, which needs no code change.

## What remains (the build, cost E)

- `refactor.rs`'s module docs state the rule in place of "rider (i) … adjudicated at review ordinal 40 … D-4's ratified identity".
- `tests/fixture/round_trip.rs`'s module docs cite A10's replacement rule.
- `p2_split::r1_a_cut_whose_roots_a_kept_root_separates_collapses_the_order` asserts the expected regrouped roots, and that a second round trip changes nothing.
- `asm4_split_inline::root_interleaving_collapses_onto_the_instance_at_d4_identity` drops "D-4 identity" from its name.
