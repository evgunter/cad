---
id: emit-topo-raises-a-correct-kernel-may-reach-have-no-variant
kind: issue
title: Twelve emit_topo Emission raises a correct kernel may reach have no row-1 or row-2 variant to take
status: open
opened: 2026-10-20
---



Filed by `wire-emis-topo` (the `emit_topo.rs` third of
`names-emission-raises-panic-or-take-their-true-ending`).

## What

Under D9 as PR 5411 wrote it, a raise a correct kernel can reach takes
row 1 (a precision limit: a lever and `KERNEL_LIMIT_RECOURSE`) or row 2
(`NOT_YET_ENDING`, a variant naming the construction). These twelve in
`crates/editor-core/src/names/emit_topo.rs` can be reached by a sound
body as far as anyone has shown, and `NamingError` has no variant for
any of them, so each still refuses `NamingError::Emission`. Adding the
variants changes `names/emit.rs`'s shared enum (its `Display`,
`category`, `pncad-py`'s tag table and `display_contract`'s census),
which the three-lane split left to the orchestrator.

Row 2 (a shape no rule names):

- `split_joined_readings`: "a split joined a section chord to an
  operand edge's piece, which no input cell reads". A vertex on the
  plane between a chord and a collinear in-plane operand edge piece
  makes one; the function's own comment says no `ci` row builds one.
- `split_join_seam`: "a split joined a section chord beside a merged
  operand face to whole operand edges, which no rule names". Its
  `Joined::Seam` reading names the case where the merged face holds the
  section; the mirror (a plain section face beside a merged operand
  face) has no rule. (It refused `BETWEEN_SECTIONS` before, which is
  false of it; the case where both faces hold the section now panics.)
- `name_boolean_edges`'s `chord_kind` and `joined_cover`: "seam edge
  without exactly two adjacent faces", "a joined edge without exactly
  two adjacent faces". `Incidence` dedups a wrap edge's one face; a wrap
  edge pierced at a point and joined back after the boolean is one way
  in.
- `name_boolean_edges`'s `add_seam`: "one seam name holds edges between
  faces of one operand and of two". Tied names can spell two seams
  alike.
- `name_boolean_edges`'s zip loop: "seam edge between same-operand
  faces". A zip seam along an operand rim beside one merged face.
- `side_in_body`: "a crossing's edge has pieces on both sides of the
  other operand on one side of it". A legal union reached it
  (`a-prism-crossing-a-wedge-about-a-contact-line-names-a-crossed-edge-both-ways`,
  closed by an emitter fix); a vertex holding several keys can still
  class one edge's pieces against two wedges.
- `sense_of`: "edges of one name read different senses at one
  crossing". Two tied edges at one vertex.

Row 1 (a verdict at the ambiguity factor K, which `Tolerance` takes as
any finite K > 1, `geom-core/src/tolerance.rs`):

- `Track::runs_with`: "a certified edge on another's carrier runs
  neither way along it". The two stretches it compares are a quarter of
  the edge's length apart, at least K/4 zero bands for a certified edge,
  so at K ≤ 4 they can decide zero. Under K = 1.5 an arc two escalate
  bands long does (`an_arc_a_few_tolerances_long_at_a_small_k_refuses_typed`).
- `Segment::runs_with`: "two segments on one line run neither way along
  it". Each end may lie up to a zero band off the line, so at small K a
  certified segment can project into the zero band
  (`a_segment_across_a_line_at_a_small_k_refuses_typed`).

Row 1 (two band verdicts over different levers can disagree):

- `name_split_edges_vertices`: "a split set piece lies off an operand
  edge it covers". The joined edge's far end is placed against the
  root's line over the root's length.
- `name_boolean_vertices`: "a vertex of a joined edge lies on none of
  its set". `edges_holding` places the vertex over each member's length;
  the cover that named the set read the members over the joined edge's.

## Next

One variant per fact, or a small family, in `names/emit.rs`, each site
moved onto it; or a witness for a site, which makes it a defect to fix
where it starts.
