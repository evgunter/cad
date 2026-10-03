---
id: locus-matching-moves-frontier-refusals-to-join-desync
kind: issue
title: Locus matching moves 394 seam-battery and reflex-probe poses from typed frontier refusals to JoinDesync and SeamOrientation
status: closed
opened: 2026-10-02
closed: 2026-10-02
branch: join/pole-strut-binding
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

## Fix pass 2 (PR 3790): the strut binding fixed, re-measured

The strut's half beside a germ's own locus edge now faces that germ, in
both strut mints (`insert`'s spike order, `vtxfac`'s pierce struts).
Against main da396111f, release, the seam battery moves no frontier
refusal to `JoinDesync` any more (84 `SectionArcWindow` →
`SectionNotPolar`, 42 `JoinDesync` → `SectionNotPolar`, 24
`UnpairedLooseEnds` → sound); the tube and bored-capsule batteries move
only from `JoinDesync`/`UnpairedLooseEnds` to typed frontiers or sound.
The same fix restored `join1_delta_probes::overlapping_lens_prisms_declared_union_builds`.
What remains is the reflex probe's 32 `SeamOrientation` moves (28 from
`UnpairedLooseEnds`, 4 from `RestZipUnsupported`), still not traced.

## Measured (`join/pole-strut-binding`, 2026-10-02)

JOIN-1's head `0b6e39ca` against main `3ee0e4b6`, release, all six
batteries of `join1_r1_probes.rs`; outcome classes, main → head:

| battery | poses | moved | moves |
|---|---|---|---|
| prisms | 42335 | 6960 | `UnpairedLooseEnds` → sound 6960 |
| seam | 12149 | 1245 | `SectionArcSide` → `SectionNotPolar` 993; `SectionInvariant` → `SectionNotPolar` 102; `SectionArcWindow` → `SectionNotPolar` 84; `JoinDesync` → `SectionNotPolar` 42; `UnpairedLooseEnds` → sound 24 |
| declared | 26999 | 6268 | → sound: `UnpairedLooseEnds` 5977, `ClassificationInvariant` 245, `RestZipUnsupported` 46 |
| reflex | 1151 | 186 | → sound 148; `UnpairedLooseEnds` → `SeamOrientation` 28; `RestZipUnsupported` → `SeamOrientation` 4; wrong body → `Euler(FanStartMismatch)` 4; `JoinDesync` → `JoinDesync` ("B senses agree") 2 |
| tube | 6899 | 342 | `JoinDesync` → `SectionArcWindow` 204, → sound 102; `UnpairedLooseEnds` → `SingleSiteSectionLoop` 36 |
| bored capsule | 2399 | 1133 | between typed frontiers 674; `JoinDesync` → typed frontier 429; `UnpairedLooseEnds` → `SingleSiteSectionLoop` 30 |

No pose moves from sound, and none to a wrong body. No frontier
refusal of main becomes a `JoinDesync` in any battery: the strut
binding is fixed. The row's pose, `ball_poled_y(0.5)` against
`brick((−1, 0.25), (−1, 1), (−1, 0))`, refuses `Join(SectionNotPolar)`
under every op in both orders, and `join1_r1_rows`
`a_pole_struts_halves_face_their_own_meridians` pins it (red, with
this row's `JoinDesync`, when `vtxfac`'s pierce strut ignores
`insert::strut_facing`).

## Closed

The pole-strut binding is fixed by `insert::strut_facing` (PR 3790's
second fix pass) and pinned. The two reflex moves are traced and
filed: the 32 `SeamOrientation` moves are edge-in-face poses that,
once joined, reach the parallel-ring defect main already refuses at
the neighbouring shears
(`reflex-corner-edge-in-face-poses-zip-a-ring-parallel-to-its-section-loop`),
and the 2 `JoinDesync` → `JoinDesync` moves are a B sense bound wrong
at the reflex vertex
(`a-reflex-vertex-and-its-partner-read-the-same-b-sense-along-an-edge-through-the-corner`).
Both are evidence for `reflex-corner-vertex-vertex-sites-refuse-under-a-tilted-cap`.

## The reflex `SeamOrientation` moves, fixed (reflex-corner lane, PR 3900)

The first step that goes wrong at these poses is `bool_strut_order`:
a strut whose two germs both lie in the 315° top face, at least one more than a half-turn from the corner's
arrival edge, had its halves ordered by a bare cosine, which is not
monotone past a half-turn. The order now reads the angle
(`insert::strut_order`), and all 159 of the probe's `SeamOrientation`
poses (∩, ∪, `a ∖ b`) build sound, the 32 movers among them. The
parallel ring is that wrong facing, read at the zip
(`work/join/reflex-corner-vertex-vertex-sites-refuse-under-a-tilted-cap`).
