---
id: a-split-through-a-pocket-floor-touching-the-block-walls-refuses-a-sliver-sector
kind: issue
title: A split along a pocket floor whose diamond outline touches the block's walls at its corners refuses SliverSector (sector_straight indeterminate)
status: closed
opened: 2026-10-20
priority: P1
cost: M
refs: [a-split-touching-a-wall-at-a-vertex-refuses-a-crossing-vertex-emission, a-pinch-carried-through-a-later-fold-step-refuses-naming]
branch: splitdoor/pocket-floor-sliver
closed: 2026-10-29
pr: 5982
---



Filed by WIRE's `wire-splitvertex` lane (branch
`wire/split-wall-vertex-crossing`) while probing variants of the moat
fixture in `crates/editor-core/tests/split_pinch_keeps.rs`
(`moat_split`), measured on main at `179890fe2f`.

## What

`moat_split(pocket, moat)` builds a 4 × 3 × 4 block, subtracts a pocket
of outline `pocket` from `y = 1` up round the pin `[1.5, 2.5]²`, then a
moat of outline `moat` from `y = 0.5` up round the same pin, and splits
at `y = 1` (outlines drawn in `z`, `x`). With the pocket a diamond whose
corners touch the block's four walls,
`[(2, 0), (4, 2), (2, 4), (0, 2)]`, every moat tried that leaves rim
material refuses at the split's reduction:

```
Split(Reduce(SliverSector { vertex: VertexKey(21v3), face: FaceKey(2v1),
  reading: Escalated(Indeterminate { margin: MarginDiag(Value(2.0,
  Some(Rejection { sign: Positive, passes: Negative }), ZeroWord(0))),
  terminal_sliver: false, band: Band { zero: 1e-9, escalate: 1e-8 },
  predicate: Some("sector_straight") }) }))
```

Moats measured: the square `[1.2, 2.8]²`, the square `[0.5, 3.5]²`, and
diamonds of half-diagonal 1.2, 1.5 and 1.75 about `(2, 2)`. The moat
`[0, 4]²`, which takes the whole rim, splits, and with the square
pocket `[0.5, 3.5]²` each of the others splits on that branch.

A margin of 2.0 read as indeterminate is not a near call; the escalation
looks like a rejected certificate rather than a tight decision.

## Open

- Whether the cut is a legal body. Each pocket corner lies in a block
  wall from `y = 1` to the top, so the rim there is two wedges meeting
  along a line; the subtracts build it without complaint. If it is
  legal (D9), the split's refusal is a defect; if not, the subtract
  should refuse it.
- With the moat the same diamond as the pocket, the second subtract
  refuses first, `Naming(Emission { what: "a copy of a corner held as
  several meets no recorded discard between them" })`: evidence on
  `work/refs/a-pinch-carried-through-a-later-fold-step-refuses-naming.md`.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P1: a split of a body the subtracts build refuses on a margin of 2.0: verb breadth beyond the everyday shapes (a split or boolean that refuses, or ships a body its own gate fails, on a legal input), P1 by `work/README.md` Priority.
- Moved to SPLITDOOR (`work/splitdoor/`) by `git mv`, id and body unchanged: the P1 track opened for split and boolean shapes that refuse, or ship a side their own gate fails, where a uniform reading would build: the split door's tier-3′ bar, tangent and pinch poses, and the curved-carrier lanes that read lines and circles only. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Closed (2026-10-29, PR 5982)

The `sector_straight` refusal was a definite reading taken the wrong way. At a slit tip (two distinct edges on one segment, legal per [topology]) the wall wraps 2π, and cos θ = +1 cannot tell that from a 0° spike. `sector_shape::corner_shape` is now the one reading. It takes `CornerBounds { Distinct, OneEdge, SlitTip }` and turns a positive straight-corner rejection at a slit tip into a full turn whose quarters each lane takes in its own order (`FullTurn::quarters(TurnOrder)`). The split's `wide_entries` and the Boolean's `push_full_turn` both call it.

All six moats and the ring's four tip cuts build at their closed forms and tier 3′ at three ε. The reviewer's 11 oblique tip cuts, each checked against an independent integration, refused on the base and build now.

FULL single review: APPROVE. A fix pass made the full-turn reading single.

Filed: `a-slit-tip-on-a-curved-face-or-at-a-cone-apex-reads-its-full-turn-as-a-spike` (P2), covering `vertex_section::entered_faces` and the cone-apex arm, plus the (vertex, face) keying gap.
