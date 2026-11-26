---
id: the-main-merge-drops-mains-pole-form-and-pole-side-column-intents
kind: issue
title: The main merge drops main's pole-form, pole-side-column and seam-twice intents where the stack's wrap tip and reach_graded stand
status: closed
opened: 2026-11-25
priority: P0
cost: M
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
closed: 2026-11-26
pr: 7414
---

## What

PR 7414 merges main `5e04ac6561` into the one-band stack. Four files
resolve to the stack's side: `step-import`'s `normalize/singularity.rs`
and `entities.rs`, `geom::pole_form`, and `mesh`'s `trimmed.rs`. Where
main's change in those files was an API move (unit carrier directions,
the conic-centre expects, the decide-name forwarder), the merge carries
it by hand. What it drops is main's code where the stack decides the
same question differently. Each one, with the commit it came from, what
it made true, and the main rows that pin it:

1. **`geom::pole_form`'s half-turn pair** (`azimuths`, `half_turn`,
   `half_turn_curve`; the merge base's, given a `UnitVec3` door for the
   line arm by main's `f38e2b9a47` and `0c5b846081`). On main it makes
   import's form at a chart singularity two meridians a half turn apart.
   Main pins it in `pole_form`'s own row
   (`the_meridian_carriers_run_through_the_singularity_at_their_azimuth`,
   its twin check), restated on head without the twin. Its callers on
   main are `step-import`'s `entities.rs:1847` (the edge-free sphere's
   two lunes) and `normalize/singularity.rs:802, 1028, 1753`. **Ruled:**
   the wrap tip supersedes the pair. Head mints one wrap edge.
2. **`normalize/singularity.rs`'s centroid azimuth** (`singularity.rs:21,
   937`, the merge base's, kept by main): a loop around a hole on a
   sphere is cut at its centroid's azimuth. **Ruled:** not carried; head
   keeps the stack's nearest-point rule.
3. **`mesh::trimmed`'s `pole_side_columns`** (the merge base's, kept by
   main; main's `9518f70d12` and `638e764c03` restate its call site for
   the rowless cylinder). It thins the lattice's columns beside a
   singular vertex only, so #678's one-column fan never forms at a pole
   side. Head has the stack's `repeat_columns` (every side whose ends
   repeat a mesh id) and `reach_graded` (a cone's gaps graded past a
   quarter turn, `c78017c359`, `5806e2d848`). Main pins it in its in-file
   row `pole_side_columns_doubles_past_one_column_between`, which the
   merge drops; head's `repeat_columns_doubles_past_one_column_between`
   stands in its place. Main's cylinder intent is carried: the column
   count reaches `trimmed_metric` alone, and main's
   `rowless_cylinders` rows pass on head (3 of 3). **Ruled:** mesh
   composes `pole_side_columns` with `reach_graded` and re-measures.
4. **`singular_row_shape`'s seam-twice refusal** (the merge base's, kept
   by main; `SphereTrimCause::SeamTwice`): outside a cone's development,
   a mesh id at two chart locations that is no singular vertex refuses.
   The stack retired it (`990d0c42bc`, `febb6be1bb`), because a one-band
   face's loop walks its wrap edge both ways. Main's `mesh/src/cert.rs:302`
   cites it; head's does not. No main row reds on head for it.

## Measured on head (`da9f012a31` and the re-pin commit after it)

- `step-import`'s ci profile: green at the default ε, including the
  decide-name roster (`recourse_roster`, main's `0a474152b2`, which the
  stack's singularity pass satisfies: it has no `on` forwarder).
- `mesh`'s ci profile: green but for two re-pins, `TOTAL_MESHED` and the
  tilted-slab bytes, each caused by one-band fixtures. Neither is a
  pole-side or seam-twice row.
- `geom`'s ci profile: green.

## Done when

Each of 3 and 4 is ported back by its owning lane, composed with the
one-band shapes, and the four rulings are recorded at their sites. That
means `pole_side_columns` with `reach_graded`, and seam-twice as it
reads a wrap edge. Main's dropped rows are restated or pinned, and the
re-measured counts are cited.

## Item 3, measured (LATHE mesh lane, 2026-11-25)

Subsumed, not composed. `pole_side_columns` decides columns: it doubles
`nu` until no pole side (two copies of one singular vertex at two `u`s)
has exactly one lattice column between its ends. The stack's
`repeat_columns` (`990d0c42bc`, before PR 7556) reads the same doubling
over every mesh id the polygon holds at two UV locations, and a pole
side's two ends are one such id. Its spans are a superset of the pole
sides, so its count is never below the pole-side rule's and leaves no
pole side one column. `reach_graded` decides rows only (a cone face's `v`
gaps past a quarter turn) and never reads or moves `nu`, so there is
nothing to compose. Main's row's witness stands verbatim in
`repeat_columns_doubles_past_one_column_between`, and
`trimmed::tests::repeat_columns_holds_every_pole_side` measures the
superset with the pole-side rule as its oracle: four polygons (a narrow
side, one at each end of the box, two of one pole, one beside a seam a
period apart), every starting count from 1 to 16. Main's mesh rows that
reach pole sides (`mesh7r1_probes`, `mesh8r1_probes`, `mesh8r2_probes`,
`mesh11_arc_branch`, `issue896_pole_guard`, `loops_with_no_rim`,
`meridian_free_face`, `reach_probes`' poled balls) are unchanged from the
merge base on main and pass on the stack under `--features budget`.


- 2026-11-26 — Closed with PR 7414 (LATHE orchestrator). Every dropped intent is settled:
  - items 1 and 4 are superseded by the wrap tip (Ev, `[ev]` PR 6014);
  - item 2, the centroid azimuth, is not carried: PR 7774's dual review rejected it as edge-splitting-dependent;
  - item 3 is subsumed by `repeat_columns`, measured against main's rule (PR 7803);
  - main's API moves are carried.
