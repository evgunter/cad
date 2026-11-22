---
id: a-selection-cannot-name-a-repeated-piece-by-an-index-expression
kind: issue
title: A selection cannot name a repeated piece by an index expression, so a bolt cannot mate to hole k's wall
status: open
opened: 2026-11-22
priority: P2
cost: M
design: true
---


Filed by the cut of `a-loop-cannot-repeat-inside-a-profile`. Both
designers name it as residue that nothing represents (fork log row 296).

## What

After `a-profiles-loop-list-repeats-a-loop-with-each`, hole `k`'s wall
is named `Lateral(Piece { step, role, member: [(k, 3)] })`, with a
literal member. `[repetition]`'s lockstep example
(`bolt.axis ≡ holes[k].axis`) needs a placement inside a family over
`k` to read **its own member's** hole wall, so the member key has to be
an index expression (`k`, or `(k + 1) mod N`), evaluated per member of
the reader. A name today is a value with no expression in it
(`crates/editor-core/src/names/role.rs`, `MemberAt = Vec<(VarId, u32)>`).
A face selection on one body therefore cannot say "the hole of my
member".

## The question (`design`)

Whether such a selection is a name with an expression key, a selection
form that reads an index (as `xs[k]` does for a family's member), or
the mate reading a family of faces whole. This was not weighed. It is
the build's one open design fork that is not one of the cut's
questions.
