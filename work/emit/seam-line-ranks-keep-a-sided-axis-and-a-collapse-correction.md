---
id: seam-line-ranks-keep-a-sided-axis-and-a-collapse-correction
kind: issue
title: Every ranker along a seam line ranks along the minted pair's n_a × n_b, so a commutative consumer needs RankRule to re-orient it; a canonical axis would remove the correction and rename pair-boolean OrderAlong names
status: open
opened: 2026-09-24
priority: P1
cost: D
refs: [union-seam-edge-ranks-follow-which-step-split-the-seam, name-ordered-positions-in-a-path-have-no-single-home, 3125]
---


## The finding

After PR 3125, every ranker along a seam line ranks along the minted
pair's `n_a × n_b`. That covers the seam chain and the seam-root
descent chain, and the seam-vertex carrier follows the same line. The
union's `collapse` then applies `RankRule::Reverse` whenever the
canonicalization swaps the pair. So order-independence rests on a
sided convention plus a correction applied where the names are
consumed. A second commutative consumer would need the same
correction.

## The alternative, measured in 3125's lane

Rank along a canonical geometric axis: each line direction,
sign-normalized lexicographically. That is symmetric by construction,
and `RankRule` would go. Measured over the editor-core suite, it would
flip the ranks of 33 of 68 pair-boolean seam-chain evaluations and 417
of 1,587 pair descent-chain evaluations. Those pair-boolean
`OrderAlong` names would be renamed. It also needs a new recorded
predicate, and it changes N2's wording in `names/README.md` ("rank
along the parent's oriented carrier").

## What the answer has to settle

Whether the uniform axis is worth the name migration and the N2
change. That is Ev's call, and it goes as an `[ev]` PR. Ev ruled on
3102 that re-baselining is never a reason to skip a good change, but
this one changes a ratified clause's substance.
