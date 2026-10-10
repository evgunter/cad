---
id: an-assertion-relates-by-equality
kind: issue
title: D10 stage 5 PR A: an Assertion's relation is ≤, ≥ or =
status: dispatched
opened: 2026-10-08
priority: P0
cost: M
branch: intent/s5-a-relation
---


INTENT stage 5, PR A. Spec: `docs/INTENT-STAGE5-SPEC.md` §2.

D10's `Assert { measure, relation, bound }` admits `≤`, `≥` and `=`. Today
`AssertionDir` (`crates/editor-core/src/measure.rs:799`) has only
`AtLeast` and `AtMost`. `AssertionRelation { AtLeast, AtMost, Equal }`
replaces it, and the field `dir` on `Node::Assertion` becomes `relation`.
`decide_assertion` (`measure.rs:1084`) gains the `=` arm: Zero holds,
either definite sign is violated, and the sliver band is `Unevaluated`.
Over a window-superset enclosure, `=` needs both ends admitted to hold.

The analysis lanes need no special case: the box driver, MC and stackup
read the verdict. Python, the `.pyi`, the census, the viewer rows and
ERROR-DESIGN E10's persisted line follow. Ids move after a document's
first assertion insert (the wire field is renamed). Verdict bits do not
move.

It needs no stage-3 or stage-4 work. It waits on stage 2 D, which
rewrites the same `Node::Assertion` arm (`value: S`). It could land on
today's tree, but would then collide with D.
