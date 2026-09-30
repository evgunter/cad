---
id: union-partners-are-the-planes-that-tell-its-pieces-apart
kind: issue
title: Which bordering parents a union (and the pair boolean) cites in a split face's SideOf
status: open
opened: 2026-09-29
priority: P1
cost: H
---


## What

A face that the finished body holds as several pieces is qualified with
one `Fragment(SideOf)` over its PARTNERS. N2's union paragraph
(`crates/editor-core/src/names/README.md`, from #3222) made every parent
across the group's seam edges a partner. PR #3241 implements the union
end pass, and a first review found two problems with that rule:

- a curved boss on one piece became a partner and refused with an
  emission error;
- a notch in one of two tied faces de-tied them.

The fix pass (`ed6602ffb0`) narrowed partners to members that border
two or more faces, joined through 3D contact. A second review (2026-09-29,
every member order, head against base) measured that proxy wrong both
ways:

- Three overlapping strips across a plate refuse as an emission bug in
  all 24 orders, where the base publishes.
- A bar notching both tied slot ceilings de-ties or renames the tie.
- A boss straddling the slab becomes a partner, so moving it renames
  every piece.

Beneath the partner question the reviewers found a second one: the
union keys parents by name. That makes a tie's candidates "pieces of one
parent", and it is what breaks the tie cases.

## The fork

The designer pair and the options are in the `[ev]` PR. In short, both
designers agree on three things:

- parents are keyed by member-face entity;
- one rule serves the union and the pair boolean;
- a partner must separate two pieces by side (the verdict filter).

They split on one point: whether to add a planar-connectivity condition
now. That condition says a partner must also bound a covered region of
the parent that borders two or more pieces. It removes a residue, a
feature on one piece whose plane lines up with a divider elsewhere. It
costs a new planar computation, and with it a new `Escalated` exposure.

PR #3241 waits on this.

Related:
- `work/emit/the-pair-boolean-sides-a-split-face-against-every-seam-neighbour.md`
  (on #3241's branch)
- `work/emit/a-split-that-de-ties-tied-faces-swaps-their-names-under-an-edit-that-moves-the-split.md`

## Reframed (2026-09-30)

Ev read the aligned-feature case and questioned the semantics
themselves: a feature should not influence geometry it doesn't touch,
and "the whole system seems kind of fragile".

A second designer pair took the underlying question: what should
distinguish the pieces of a split face? Both designers independently
recommended the same change. Replace `SideOf` for face pieces with the
set of divider walls each piece actually borders (`Borders`). A divider
is an obstacle that borders two or more pieces. The rule reads no plane
and records no side.

The only open difference between them is mechanism, not semantics:
- one derives the obstacles from the boolean's discarded fragments,
  recorded by the kernel;
- the other walks piece loops and member-edge cells, with margined
  geometry only for islands and pinch vertices.

The `[ev]` PR carries both.

## Ruled (2026-09-30, #3454)

Ev took the `Borders` semantics: a boolean's split-face piece is named
by the divider walls it borders (N2).

The obstacle mechanism will be chosen by measurement:
- how often islands and pinches occur;
- whether the face-path variant differs from planar obstacles;
- whether the cross-fold record join is order-free.

Ev leans toward the kernel-record mechanism, with low confidence.

The row is now the build, after that prototype. #3241 is rebuilt on
`Borders`.
