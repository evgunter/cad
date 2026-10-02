---
id: two-parts-of-one-body-at-one-boolean-refuse-as-ray-exhausted
kind: issue
title: A shell lying wholly on the other operand's boundary (two Parts of one body at one boolean, and three shapes sharing no Arc) refuses ShellWitnessExhausted, naming no coincidence
status: open
opened: 2026-09-04
priority: P0
cost: M
pr: 3897
branch: fuse/on-verdict
design: true
---

## What

`Node::Part` (DOCM-2, PR #1860) hands on the selected half's or
instance's own `Arc` (`crates/editor-core/src/eval/wire.rs`,
`wire_part`). Two Parts selecting the SAME half of one split — or
`Part(Instance(0))` beside its master, since instance 0 IS the input's
`Arc` (`wire_pattern`) — are two node ids, so DM5's pairwise-distinct
check on a boolean's inputs (`Node::input_fault`,
`crates/editor-core/src/node.rs`) admits them, and the boolean receives
the identical body twice: same allocation, identical `GeomSource`s on
every description.

## Measured

Main @bdfdda30c, re-measured through the public API (the rows of
`crates/editor-core/tests/on_verdict_rows.rs`, run on main with each
refusal printed). The row's earlier `Containment(RayExhausted)` no
longer reproduces: the witness ladder
(`crates/topo/src/boolean/shell_witness.rs`) reads past rays to edge
midpoints and face interiors, and every one lies ON the other
operand's boundary. Each shape refuses
`Boolean(ShellWitnessExhausted { operand: A, on_boundary: 26, in_band: 0 })`
under ∪, ∩ and − (24 nodes, every one):

- two `Part(Above)` of one split (one `Arc` at both seats);
- `master` with `Part(Instance(0))`, both orders (one `Arc`);
- `(X ∪ Z) op X` and `X op (X ∪ Z)`, Z disjoint from X;
- `(X ∪ Y) op X` and `X − (X ∪ Y)`, Y strictly inside X (the union
  carries exactly X's sources);
- two placements of one block, all six face pairs declared.

The last three share no `Arc`: the defect is the boolean's, not the
`Part`'s. The undeclared twins — `Transform(X, 0)` beside X, two
placements, two independent identical extrudes — refuse
`UndeclaredCoincidence`.

## What it is not

Not a DOCM-2 defect: the projection is right to hand on the Arc, and
DM5 is stated over node ids (a Part is a distinct node). Not the
boolean's either — it was handed a state nothing could produce before
this node existed.

## What a ruling decides

Whether "the same body twice" is a DM5 refusal at the edit door
(`InputFault` widened to a through-Part identity, which the door
cannot see without an evaluation) or a typed evaluation refusal at the
boolean (a `WrongOperand`-class arm naming both inputs, decided by
`Arc::ptr_eq` on the two operands before the kernel runs — cheap,
exact, and the one place both bodies are in hand). The second is
where the fact is readable; the first is where DM5 lives.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/bool/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): the defect is the boolean's (`crates/topo/src/boolean/*` is S-BOOL's), reached from a declared union. Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to CURVED (its charter names S-BOOL's ceded ground and inherits at S-BOOL's exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.
