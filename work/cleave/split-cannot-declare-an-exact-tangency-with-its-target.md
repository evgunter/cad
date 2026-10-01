---
id: split-cannot-declare-an-exact-tangency-with-its-target
kind: issue
title: split has no way to declare an exact tangency with its target
status: open
opened: 2026-09-24
priority: P1
cost: H
design: true
needs_ev: true
---


## What

A split whose plane touches the target along an edge while cutting it
elsewhere refuses:
- `SplitJoinError::DegenerateSection` from `split` with the given
  normal, which is the direct run's area refusal;
- `SplitJoinError::SectionSpur` in the mirrored orientation (PR 3133).

**Ev's ruling on that posture (in chat, 2026-09-24):** "if it grazes
within the sliver band it should refuse and if it is exactly tangent it
should need to be declared". Near-tangency and exact tangency are both
unlikely by coincidence when the cut elsewhere does not care about
them. Refusing is therefore right, and nothing here asks for the split
to succeed undeclared.

What is missing is the declaration. `split` takes a target and a
plane, and nothing else. An author whose plane is MEANT to lie exactly
along one of the target's edges has no way to say so. The refusal's
own message (`SectionSpur`) can only tell them that an exact tangency
"would need to be declared".

## Repros (pinned as refusals today)

- **topo:** `brick(0..1.5, 0..1, 0..1) ∪ brick(1.2..1.3, −1..2, 0.5..3)`,
  split by y + z = 2, through (0,1,1) with normal (0,1,1)/√2.
  `crates/topo/tests/split_tangent_spur.rs`.
- **editor-core:** the declared union `[a,g,b]`/`[g,a,b]` of
  `crates/editor-core/tests/emit_split_duplicate.rs`, with the same
  plane.
- **The standalone case:** `crates/topo/tests/m3_pr3_split.rs`,
  `one_sided_tangency_refused_typed` (~440). A contact with nothing
  else cut; the same ruling reads onto it.

## The precedent to look at

The boolean's declared-contact vocabulary. `topo::BooleanDeclarations`
flush pairs, and editor-core's `Node::Declare` edge with the
`UndeclaredContact` refusal that names the pair to declare. A split's
analogue would name the target edge the plane is declared to lie
along. The refusal would then carry that edge as the thing to declare,
the way `UndeclaredContact` carries its face pair.

Beyond the declaration, a declared tangency needs a rule for what the
split mints for the contact. The geometrically right result: the
tangent edge stays an ordinary edge of the piece on its side, and it
contributes nothing to the section. That is a classification change in
`splitting/rules.rs`/`classify.rs`, gated on the declaration.

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

## The question

Should an exact one-sided tangency along an edge still need a
declaration once the split derives it? Or does it classify with its
material, as the face and vertex contacts do? The ruling of 2026-09-24
says the former. The derived rule is needed under either answer: a
declaration would gate it, not replace it.
