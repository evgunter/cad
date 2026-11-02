---
id: a-flush-closed-rim-at-its-vertex-is-joined-reversed
kind: issue
title: A closed rim lying flush in a partner face, tangent inside its boundary at its one vertex, is refused at the join: joined there, the walled-off piece takes the copy's reversed half
status: open
opened: 2026-10-27
priority: P2
cost: M
---


Split from
`a-flush-closed-rim-tangent-at-its-vertex-reaches-the-fate-cut-limit`,
whose fate-cut cells build. Pinned by
`crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
`a_rim_flush_in_its_partners_top_tangent_inside_builds_or_meets_a_pinned_limit`.

## The refusal

`A` the one-segment cylinder `r = 0.3` about the origin, `z ∈ [0, 1]`,
its vertex at azimuth 0; `B` the one-segment cylinder `r = 0.5` about
`(−0.2, 0)`, `z ∈ [−0.5, 1]`, its vertex at azimuth 0 or 1. `A`'s top
rim lies flush in `B`'s top, tangent inside `B`'s rim at `(0.3, 0, 1)`,
`A`'s one vertex there. `A ∪ B` and `B ∩ A` read the two tops facing
one way and keep `A`'s; `boolean/join.rs` `closes_at_its_site` takes
the rim at `B`'s boundary vertex only where `stands_on` reads the two
faces opposed, so both refuse `SingleSiteSectionLoop { count: 1 }`, at
rest and turned. The other ops build `SOUND`, and so do these two with
`A`'s vertex at azimuth 1 or π, and in the two-arc form.

## Measured with the refusal lifted

`stands_on` answering `true` for the flush rim (`B`'s vertex at 0):

- `A ∪ B` reaches the fate cut and refuses `ClassificationInvariant {
  "a fate cut's end lies in no corner of the dropped face" }`. `B`'s top
  then holds two copies of `A`'s rim through `B`'s vertex, edges 6 and
  7 in the probe, with the null face pair between them. The piece the
  copies wall off (face 5) is bounded by edge 7's half that leaves the
  vertex along `−y`, clockwise about the top's outward normal, and the
  null face holds both copies' counterclockwise halves; in the two-arc
  form, which builds, the walled-off piece holds the counterclockwise
  halves and the null face one of each. The kept rim leaves the vertex
  along `+y`, so no face of the cut's root holds an edge along it and
  `fate.rs` `place` finds no corner: the cut reads the join's
  orientation, it does not cause it.
- `B ∩ A` refuses `SeamOrientation` (`B`'s vertex at 0) or
  `ClassificationInvariant { "… in several corners …" }` (at 1).

Where the reversal arises is not measured. The copy at the boundary
vertex is minted by `boolean/join.rs` `one_site_segments` and the
joiner's whole-turn chord (`crate::chord_join`), in the sense the
record's germs give; a rim standing on the face (opposed, a boss on a
plate) builds through the same path, so the sense the germs give for
an aligned flush rim is the first suspect. Once it is right,
`stands_on` goes, with its decision (`bool_rim_face_opposed` in
`boolean/mod.rs` `decision_words`, `offer_rows::SITES` and
`docs/predicate-dimension-audit.md`).

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). TANGRIM holds the tangent curved rims and crescents; id and body unchanged. (CLEAVE orchestrator)
