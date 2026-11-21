---
id: a-loop-of-three-collinear-chords-passes-the-short-loop-floor
kind: issue
title: a loop whose three chords are collinear passes the short-loop floor and refuses Triangulation
status: open
opened: 2026-11-21
priority: P3
cost: M
---


Filed by the memo-holders unit, found while building its fixture.

## What

The short-loop floor (`crates/mesh/src/chords.rs`, `short_loop_floors`)
counts a loop's chords and raises a loop drawing fewer than three. It
does not ask whether three chords bound a region. A plane cap of a
circular segment (an arc and its chord line) with the chord line split
in two at its midpoint draws three chords once δ is above the arc's
sagitta: the arc as one chord, which lies on the split line's two
halves. The polygon has zero area and the planar lane refuses
`Triangulation`, where the same cap with its chord line whole (two
chords, so the floor raises the arc to two) meshes.

Measured with `patch_memo.rs`'s `shallow_segment` and
`chord_face_split` fixtures (bulge 0.1, height 1, tilted cut at 0.5;
sagitta about 0.1): the bottom cap `FaceKey(2v1)` refuses
`Triangulation` at δ 0.2 on the split body and meshes at δ 0.1, and
the unsplit body meshes at both. The same holds at bulge 0.05 from
δ 0.1 and at bulge 0.02 and 0.01 from δ 0.05.

## Owed

A floor that reads whether the drawn polygon has a region, or a
refusal whose words name the δ-coarse chord polygon rather than the
triangulator. Compare
`zero-area-laminae-whose-traversals-differ-by-rounding-refuse-under-words-for-other-causes`,
the lamina residue of the empty-patch row.
