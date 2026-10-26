---
id: emit-topo-raises-a-correct-kernel-may-reach-have-no-variant
kind: issue
title: Twenty-four Emission raises in emit_topo, emit_union and join_names that a correct kernel may reach have no row-1 or row-2 variant to take
status: closed
opened: 2026-10-20
priority: P2
cost: M
closed: 2026-10-21
---



Filed by `wire-emis-topo` (the `emit_topo.rs` third of
`names-emission-raises-panic-or-take-their-true-ending`). The twelve
`emit_union.rs` and `join_names.rs` sites below came from
`wire-emis-union` (the union third, PR 5501). One `emit.rs` change
serves all of them.

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

## The union's twelve (`emit_union.rs`, `join_names.rs`)

Row 2 (a shape no rule names):

- `join_names::name_joins`, `Member::Outright`: a door's join takes an
  edge it minted outright (an end arc, a mitre, a slit, a remnant).
  `names/README.md` states the refusal as the rule ("a join that takes
  one is refused"), and nothing in `sweep::BlendNaming::edge_joins` or
  `topo`'s shell record rules out such a join.
  `emit_blend::tests::a_join_over_an_edge_the_blend_minted_outright_refuses`
  plants the record.
- `Flush::meeting`: "one member edge meets several faces of other
  members and no seam at a union vertex". The code's argument that no
  manifold union reaches this covers planar faces only.
- `cite_member_edges`, `CITED_GROUP_NO_MEMBER_EDGE` (two sites). This is
  the sibling of `CITED_GROUP_NOT_ONE_SEAM`. A legal document once
  reached that one through a member corner held as several vertices
  (`a-bridge-met-with-the-wedges-and-the-other-bridge-names-vertices-on-no-single-seam-alike`,
  now named by `name_corner_copies`), so the argument that neither is
  reachable has already failed once.
- `Parents::cited`: "a name a union publishes cites faces of several
  parents". A tied member face is keyed to the parent of each of its
  candidates, and those parents can differ.
- `collapse_one`, `RUN_LEAST_LINE_MOVED`: the collapse's map from fold
  space to member space does not keep name order, so a ranked run's
  least line, or the order of a seam-only run, can move with the member
  order (`canonical::RankRule::along_run`).
- `collapse_one`, `JUNCTION_LINES_COLLIDE`: two fold-space seam lines
  between the same two member faces, met at different steps, collapse
  to one line (as `collapse_table`'s doc says happens for seam edges).
- `name_by_parents`: "a union's seam edge does not lie between two
  faces". The same wrap edge as `chord_kind`'s above: `Incidence` dedups
  its one face.
- `name_by_parents`: "a union's seam edge lies inside one parent".
  `Parents::of` argues only one direction: what the pair leaves
  unmerged, the fold leaves unmerged too. The pair can merge A's face
  with B's while the fold skips the same curved group (`SkippedMerge`
  reads the group's shape, and the fold's group holds more faces).

Row 1 (two band verdicts can disagree):

- `Flush::line_sense`: "two member edges flush on one line disagree on
  its sense at a union vertex". The two senses come from two independent
  pairwise judgements of edges that are flush only within
  `ON_MEMBER_EDGE`.
- `Flush::sense_against`: "two of a member's pieces of one line are
  classified at one union vertex". A piece shorter than the band puts
  both of its neighbours' classes at one point.
- `Flush::meeting`: "a union's joined edge at a vertex lies on none of
  the member edges it is named for". `edge_cover`'s verdict and an end
  point's are two band decisions, and a span set made only of tied
  edges leaves `lines_at` empty.

## Next

One variant per fact, or a small family, in `names/emit.rs`, each site
moved onto it; or a witness for a site, which makes it a defect to fix
where it starts.

Closed by PR 5518 (2026-10-21). The 24 raises now take row-2 `Unsupported*` variants (`UnsupportedSeamVertex`, `…CrossingSenses`, `…SeamSides`, `…Join`, `…UnionName`) or row-1 `Category::Limit` variants (`WayInZeroBand`, `JoinedPieceOffRoot`, `BandVerdictsDisagree`). Each row-2 variant has a shape enum, and the row-1 variants end through `geom_core::lever_recourse`. The older missing-rule variants that lack the prefix are `the-older-missing-rule-naming-variants-lack-the-unsupported-prefix`.
