---
id: a-cut-by-a-tool-whose-pieces-overlap-is-cut-as-it-stands
kind: issue
title: A cut whose tool is an assembly of overlapping pieces is cut as the tool stands, leaving one solid whose cavity walls cross: silent today, a refusal once the gate asks each solid's embedding
status: open
opened: 2026-11-18
priority: P2
cost: M
---


## Finding

From EMBED, 2026-11-18, out of the designers' work on PR 7068
(fork `embed-records`). **Read in the code and in DESIGN, not run.**

`[topo.cut-material]` has a cut unite its tool's pieces before cutting
only where they rest on each other; "a solid no verdict joins stands as
it is". `cut_by_material` (`crates/topo/src/boolean/nary.rs`) returns
`None` where the tool's solids do not rest on each other. So for a tool
that is an assembly of two pieces whose material overlaps (allowed, as
an interference finding), `t − (p ⊔ q)` cuts each piece out as it
stands, and the result is one solid whose two cavity walls cross inside
it.

That body ships silently today. With the at-rest gate asking each
solid's embedding (Ev's yes to PR 7068's first decision), it refuses at
the cut. The point set `t − (p ∪ q)` is well defined, so the refusal is
the kernel's limit, not the model's.

## What a fix needs

- First a row that builds it and reads the result (two overlapping
  cylinders as one tool, cut from a block), to confirm the crossing
  walls.
- Then the cut reads an overlapping tool as its material, as it
  already does for pieces that rest on each other.
- Beside it, also unchecked: whether the kernel's `∪` and cut classify
  correctly against an *operand* whose solids overlap. The containment
  door is a signed count, not parity (`solid_contain`'s module doc), so
  a point inside both pieces should read In; the seamed path classifies
  by sections and was not read.
