---
id: locus-matching-moves-frontier-refusals-to-join-desync
kind: issue
title: Locus matching moves 394 seam-battery and reflex-probe poses from typed frontier refusals to JoinDesync and SeamOrientation
status: open
opened: 2026-10-02
priority: P1
cost: H
---


(JOIN-1 dual review, PR 3790, minor 3: R1 measured the moves on the
reviewed head; re-measured on the fix-pass head against main at
0abf909cb, release build, R1's batteries in
`crates/sweep/tests/join1_r1_probes.rs`.)

## What

Matching on germ loci joins segments along edges that main left
loose. Where main then stopped at a typed frontier further on, some
poses now stop earlier in a refusal worded as a kernel bug:

| battery | main | fix-pass head | poses |
|---|---|---|---|
| seam (`ball`) | `Join(SectionNotPolar)` | `JoinDesync { "every chord arc separates a loose scaffolding pair" }` | 306 |
| seam (`ball`) | `Join(SectionArcWindow)` | the same `JoinDesync` | 84 |
| seam (`ball`) | that `JoinDesync` | `Join(SectionNotPolar)` | 42 |
| reflex | `RestZipUnsupported` | `SeamOrientation` | 4 |
| reflex | `Join(UnpairedLooseEnds)` | `SeamOrientation` | 28 |
| reflex | `JoinDesync { "every chord arc …" }` | `JoinDesync { "B senses agree at a matched pair" }` | 2 |

The other direction is larger: the tube and bored-capsule batteries move
633 poses from that `JoinDesync` to `SectionArcWindow`,
`SectionNotPolar` or `SectionInvariant`, and 66 from
`UnpairedLooseEnds` to `SingleSiteSectionLoop`. No pose of any battery
moved from a sound body to anything else, and none moved to a wrong
body.

## Traced: the ball's pole struts

`ball_poled_y(0.5)` ∪ `brick((−1, 0.25), (−1, 1), (−1, 0))`. The box
face `z = 0` holds the sphere face's two meridian edges through the
poles (the seam at `x > 0`, its partner at `x < 0`), so each pole is a
vertex-on-face site whose two orbit edges both read On and both fold
In: one run holding every real edge, minted as a strut (`vtxfac`,
`strut`). Its two halves sit between the two meridians in the face's
loop, `… (A→N) h9 h10 (N→S) …`, and the sense theorem binds the run's
START germ — along the seam meridian `A→N` — to `he_minus`, which is
`h10`, the half beside the OTHER meridian. The join then matches
`A`'s half with `h10` and the pole-to-pole segment with `h9`, and the
chord `A → h10` separates `h9` from its partner on both arcs of the
loop: `choose_roles` refuses. Bound the other way, each segment's two
halves would sit adjacent across its own edge.

The binding is the strut's (`vtxfac.rs`, `classify_vertex_on_face`,
the `strut` arm's `(he_minus, he_plus)`), which predates JOIN-1; on
main the segments along the meridians never matched, so it was not
reached. For a strut whose germs lie inside one face the binding does
not matter; for germs along the orbit's own edges it decides which
half is adjacent to which edge.

## Not traced

The reflex `SeamOrientation` moves (`dLeft`, `eRight`, `sqQ2` at
`sx > 0` or `sy < 0`): the bottom cap's edge through the reflex corner
lies in `a`'s top face there, an edge-sector site at a 315° corner.

## The fix

Bind a strut's halves to germs along orbit edges by adjacency (the
half beside the locus edge faces it), keeping the senses the sense
theorem gives, or refuse that configuration typed. Then re-run the
seam and reflex batteries; the bar is that no frontier refusal of main
becomes a `JoinDesync` or a `SeamOrientation`.
