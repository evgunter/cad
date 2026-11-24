---
id: a-subtract-tool-seat-takes-a-whole-family-and-cuts-once-per-member
kind: issue
title: The subtract tool seat takes a pattern read whole, and the cut runs once per member: N plates each with one hole
status: closed
opened: 2026-11-22
priority: P0
cost: M
design: true
refs: [denotes-body-enumerates-its-gaps-against-the-operand-door-and-misses-one, the-split-mirror-and-pattern-seats-read-a-picked-members-whole-family]
pr: 7426
branch: formwords/subtract-bodies
closed: 2026-11-23
---


Found by the review of FORMWORDS PR 7379 and pinned by
`review_denotes_body::a_pattern_read_whole_at_the_boolean_seats`
(branch `formwords/denotes-body-review`).

A plate minus a pattern of two cylinders, with the pattern picked in
the tree for the subtract's tool seat, evaluates to
`Instances [4e-5, 4e-5]`: two plates, each with ONE hole. The body gate
(`session::refuse::admits`, Body arm) admits a family read whole at a
`Body` slot. D10 `[repetition]` then does exactly what it says: an
operation whose reads reach the index is evaluated once per member. D10
also says "a subtract's tool is one `Body`, so cutting by a family is
`subtract(a, union(holes))`", and the viewer offers the other spelling
silently. The same whole-family read is consumed three ways across the
boolean seats:
- per member at the subtract (N results);
- all at once at the union (one body, correct);
- all at once at the intersect (`Empty`).

The user who cuts a plate by a pattern of holes gets N plates, so this
is a live wrong answer on everyday geometry: P0.

Open question: what a viewer seat should do when it is handed a family
whole. Designers first. Adjacent:
`work/doors/the-split-mirror-and-pattern-seats-read-a-picked-members-whole-family.md`
(a picked member read as the whole family).

## The design (2026-11-22)

A subtract's tool is a `Bodies` seat, as union's, intersect's and
assemble's members are (DESIGN `[repetition]`, REFERENCES DM4).
`subtract(a, holes)` cuts `a` by every member, as the fold
`a − t₁ − … − tₙ`; `subtract(a, holes[k])` cuts member by member. What a
seat does with a family read whole then follows from the seat's kind, in
the recipe, and no viewer seat needs a rule of its own:
- **A `Bodies` seat** (union, intersect, assemble, a subtract's tool)
  takes every member.
- **A `Body` seat** (a subtract's kept body, split, mirror, fillet,
  chamfer, shell, placement source, pattern input) is evaluated once
  per member, and its result is a family.
- **An indexed read `xs[k]`** is one member at any seat. A viewport pick
  on a drawn member writes it (DOORS'
  `the-split-mirror-and-pattern-seats-read-a-picked-members-whole-family`,
  widened to every body seat).

The intersect's `Empty` over a pattern of disjoint copies and a cutter is
the true meet, not a defect. The held-picks label says "every member" at
a `Bodies` seat and "each member" at a `Body` seat.
