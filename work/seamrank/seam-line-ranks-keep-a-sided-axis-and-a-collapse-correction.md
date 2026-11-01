---
id: seam-line-ranks-keep-a-sided-axis-and-a-collapse-correction
kind: issue
title: Every ranker along a seam line ranks along the minted pair's n_a × n_b, so a commutative consumer needs RankRule to re-orient it; a canonical axis would remove the correction and rename pair-boolean OrderAlong names
status: closed
opened: 2026-09-24
priority: P3
cost: D
refs: [3125]
closed: 2026-10-26
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

## Against the canonical axis (orchestrator, 2026-09-24)

Sign-normalizing a line direction lexicographically is discontinuous.
A seam line that an edit rotates through an axis plane flips its
canonical sign, and every rank along it reverses. That renames names
under small edits. The pair-line rule (`n_a × n_b`, re-oriented once
by the collapse) has no such discontinuity. What PR 3125's review
showed is that the pair-line rule needs ONE home, answering which line
a name's rank is along for every consumer, not a different axis. The
row is re-banded P3. It stays open as the record of the alternative.

## Closed (2026-10-26, tracker sweep)

Superseded. The question (a canonical geometric axis against the
pair-line rule plus `RankRule::Reverse`) is answered by removing ranks:
FORK-ORIENT (fork-log row 197; Ev, 2026-10-26; PR 5705, `964083e5`)
rules that nothing is ranked along a line and that `Reverse`,
`flip_senses`, `RefuseRun` and `OrderAlong`'s `of` retire. N2 in
`crates/editor-core/src/names/README.md` now says so. With no rank there
is no axis to choose and no collapse correction to keep. The code
(`names/canonical.rs`'s `RankRule`, the seam rankers) is unchanged on
main; its retirement is the build of
`work/wire/a-unions-line-of-several-edges-is-oriented-by-its-least-edge.md`.


## Closed by PR 5758 (2026-10-26)

`wire/nothing-is-ranked` builds Ev's PR 5705 rule: an orientation is read,
never chosen, and nothing is ranked. No rank remains to orient, so `RankRule` and `flip_senses` are deleted; a crossing of a seam names the side it enters along, which a rewrite that reorders the pair only re-sorts. The status change is the
orchestrator's.
