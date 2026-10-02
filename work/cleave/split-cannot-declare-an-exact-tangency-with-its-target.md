---
id: split-cannot-declare-an-exact-tangency-with-its-target
kind: issue
title: rule (b) sends a convex one-sided edge in the split plane to the side opposite its material, so an exact tangency along an edge refuses where it should classify with its material
status: closed
opened: 2026-09-24
priority: P1
cost: H
branch: cleave/tangency
pr: 3726
closed: 2026-10-02
---


## What

A split whose plane touches the target along an edge while cutting it
elsewhere refuses:
- `SplitJoinError::DegenerateSection` from `split` with the given
  normal, which is the direct run's area refusal;
- `SplitJoinError::SectionSpur` in the mirrored orientation (PR 3133).

Exact tangency is the case to fix, under Ev's ruling below. A graze
within the sliver band keeps refusing.

## Repros (pinned as refusals on main)

- **topo:** `brick(0..1.5, 0..1, 0..1) ∪ brick(1.2..1.3, −1..2, 0.5..3)`,
  split by y + z = 2, through (0,1,1) with normal (0,1,1)/√2.
  `crates/topo/tests/split_tangent_spur.rs`.
- **editor-core:** the declared union `[a,g,b]`/`[g,a,b]` of
  `crates/editor-core/tests/emit_split_duplicate.rs`, with the same
  plane.
- **The standalone case:** `crates/topo/tests/m3_pr3_split.rs`,
  `one_sided_tangency_refused_typed` (~440). A contact with nothing
  else cut; the same ruling reads onto it.

The geometrically right result: the tangent edge stays an ordinary
edge of the piece on its side, and it contributes nothing to the
section.

## Found by

EMIT's `split-section-face-keeps-a-zero-area-spur-along-a-tangent-edge`
(PR 3133), re-scoped by Ev's ruling.

## What the refusal is evidence of

Rule (b) (`splitting/rules.rs`, `apply_rule_b`) assigns an edge that
lies in the split plane to a side using only its two neighbours' side
labels: both neighbours Below sends it Above, and both Above sends it
Below. That is right for a REFLEX edge, where material lies on both
sides of the plane along the edge, so the plane passes through it. It
is wrong for a CONVEX edge whose two faces both lie on one side, which
is exactly a one-sided tangency. There all the material at the edge is
on that side, and the rule mints a null edge for a contact that cuts
nothing. `DegenerateSection` and `SectionSpur` are the join catching
that downstream, as the module's own "Residue, stated honestly"
paragraph says. Convexity is a fact about the body, decided from the
two flanking faces with the `enters_material` primitive rule (a)
already uses. With it, rule (b) reproduces the adjudicated reflex rows
and sends a convex one-sided edge to its material. The edge stays an
ordinary edge of the piece on that side and adds nothing to the
section.

The split already takes the other two exact one-sided contacts without
a declaration. A plane on a face of the target is rule (a). A plane
touching a single vertex has no runs (`insert.rs` `above_runs`). The
edge is the only one that refuses.

## Ruled (Ev, PR 3642, 2026-10-01)

An exact one-sided tangency along an edge classifies with its material,
as the face and vertex contacts already do. No declaration is involved.
The second half of the 2026-09-24 ruling ("if it is exactly tangent it
should need to be declared") is revised to this. The first half, that
a graze within the sliver band refuses, stands.

The work is the derived rule (b) above. `SectionSpur`'s "would need to
be declared" text goes with it. The pinned refusals
(`one_sided_tangency_refused_typed`, `split_tangent_spur.rs`,
`emit_split_duplicate.rs`) re-baseline to the split's answer.
