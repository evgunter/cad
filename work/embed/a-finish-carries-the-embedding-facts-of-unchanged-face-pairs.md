---
id: a-finish-carries-the-embedding-facts-of-unchanged-face-pairs
kind: issue
title: A finish reuses the embedding answers of face pairs it finds unchanged in a finished operand and asks only the rest; the budget stays a backstop on one ask and a carried answer keeps no count (Ev, PR 7101)
status: open
opened: 2026-11-18
priority: P0
cost: H
refs: [the-at-rest-gate-certifies-embedding, a-refusing-ask-runs-on-to-its-whole-budget-looking-for-a-verified-crossing]
---


## Ruling

Ev on PR 7101 (2026-11-18): "ah. yes, finish can trust other finished
bodies' caches"; and on the budget, "i think 'not stored' makes sense,
since this is, like, a 'don't make the program hang forever' backstop
rather than a sharp bound". The text is `docs/DESIGN.md`
`[topology.tiers]` (after the isometry sentence) and the at-rest row of
`[one-way.places]`. The PR body has the plate example and both
designers' reports.

## Why

The at-rest gate as built on `embed/gate` asks the injectivity
certificate of a whole body at every finish: 247,480 asks and 27,344 s
over the workspace's suites, about 90% of `sweep`'s test time. A chain
of ops makes a chain of bodies that are mostly the same faces, so a
pair of faces no op has touched is asked again at every step. At that
price the gate (`the-at-rest-gate-certifies-embedding`, P0) cannot land
under the CI bar.

## The unit

1. **The certificate is one fact per pair of faces**, of the two faces
   and the cells they share, kept on the finished body.
2. **`finish` is handed the finished operands the body was built
   from** and compares faces itself: surface, sense, loops, edge
   geometry, and which cells the two faces share. A pair bit-identical
   to a pair of one operand keeps that operand's fact. The door
   declares nothing.
3. **Asked, always:** a pair with a new face; a pair from two
   operands; a pair sharing a cell the door made or changed; every
   pair with a face the door trimmed. Trimmed pieces of operand faces
   are not carried.
4. **Doors that change no face ask nothing:** the `⊔` graft, a
   product's aggregate, `distribute`, `carved` (`boolean/nary.rs`),
   the graft in `boolean/material.rs`, and a moved leaf
   (`LeafBody::moved_by` finishes the image again at `f64` today,
   though an isometry keeps a verdict). A leaf door asks its whole
   body.
5. **The budget is unchanged:** the gate's one constant
   (`GATE_MAX_CELL_PAIRS` on `embed/gate`) counts only what that
   finish asks. A carried fact costs nothing and keeps no count. A
   budget refusal therefore follows the route, which Ev accepts.

**The order.** The doors of point 4 first: they are pure repeats and
need no comparison beyond "no face changed". Then the deciding doors
(Boolean, split, shell, blend).

## Measured (the gate lane, 2026-11-18)

On 250 holding asks of the tour's release walk (2 to 111 faces: the
Klein loop, the die chain, lofts, the teapot vessel, revolves,
impellers), every face pair asked alone: the lone candidates are the
whole ask's, the lone leaves, splits and discharges sum to the whole's,
and every lone verdict holds, 250 of 250. The shared memos
(`RegionMemo`, `LocalMemo`, `across`, `cornered`, `widths`) change time
only. Refusing asks were not decomposed: the walk refuses a whole level
that does not fit, so a pair's spend there is not defined.

Still being measured: the carried share of the gate's seconds by
finish site and by test binary, with the misses split by cause (a new
face; two operands; an old shared edge with a face trimmed elsewhere).

## Not verified

- That a Boolean copies its untouched faces bit-identically (both
  designers assumed it; `out_a = a.clone()` suggests so).
- That a shelled solid's outer faces carry.

## Waits on

- `embed/records` (HELDRECORDS's first unit): `finish` takes a `Built`
  value, and the operands are one more builder method on it.
- The certificate itself, which is on `embed/gate` (lane C), unmerged.
  This unit stacks on that branch.

## Pins

- The measurement above as a test: over a set of holding bodies that
  spans the shapes, each face pair asked alone agrees with the whole
  ask in verdict and in count. Before anything reads a carried fact.
- A chain (a plate with slots cut one at a time): step k asks only its
  new faces and the pairs of the faces it trimmed. Mutant: the gate
  asks every pair (the count moves); mutant: the gate carries a pair
  whose face the door trimmed (a crossing built there passes).
- A door that changes no face asks nothing, pinned for each door of
  point 4.
- A body carrying a crossing in a pair the door touched still refuses
  at that door.

## What it does not do

Leaves pay in full, and refusals (45% of the measured price) are
`a-refusing-ask-runs-on-to-its-whole-budget-looking-for-a-verified-crossing`.
