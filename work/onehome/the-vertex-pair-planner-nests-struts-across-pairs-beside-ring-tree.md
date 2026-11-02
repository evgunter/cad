---
id: the-vertex-pair-planner-nests-struts-across-pairs-beside-ring-tree
kind: issue
title: boolean/insert.rs nests one shared vertex's struts across pairs by holds_whole, a second nesting reading beside star::ring_tree
status: open
opened: 2026-11-02
priority: P1
cost: M
---

Found by `run-loops-of-split-and-pierce-are-twins` stage 1 (`onehome/star-sites`), which tested whether the vertex–vertex planner's own chord nesting is `star::ring_tree` in substance.

## What was tested

A one-off instrumented side-by-side run, since removed, compared the old `arc_holders` (the shortest arc holding each run) against `ring_tree` over the plan's walk positions, rooted at the region outside every run. `b_runs` and `hang_at_shared` both read `arc_holders`. It covered the topo suite and the first 1 740 tests of the sweep suite: 54 090 calls, of which 35 715 had n = 2, 2 653 had n = 4, 363 had n = 6, 1 919 had n = 8 and 13 440 had n = 10. Every call agreed but one: the synthetic `Cover` row of `arc_holders_nest_the_runs_a_turn_reroots_and_refuse_a_cover`, which passes 2 arcs over 6 positions. The new reading compresses to the occupied positions and gives `Cover` there too.

So `arc_holders` now reads `ring_tree` (`boolean/insert.rs`, `arc_holders`), and `held_by` keeps reading its holders. The agreement is pinned by `b_runs_nest_every_non_crossing_matching_and_refuse_every_crossing_one`. That test checks every holder against a containment oracle over every non-crossing matching to ten crossings. Its refusal class differs from the old one only on inputs that both cross and cover, which now refuse as crossing (PR 6392).

## What is not `ring_tree` as it stands

`holds_whole`, `nests`, `segment` and `strut_anchor` (`boolean/insert.rs`) nest **struts of different pairs** at a vertex two plans share. This reading differs from `ring_tree` in three ways:

- **Domain.** Each plan's walk positions order only its own germs. Cross-pair nesting reads the vertex's geometric cut order (`walks_before`, `cut_order`) across plans, after `reconcile_shared` has turned runs onto their complements. No single walk holds every plan's germs.
- **Ties.** Two struts tied at both ends (one segment) nest by their codes: the Out one holds. `ring_tree` reads a strict cyclic order and has no tie.
- **Struts only.** It reads only struts in one physical sector. A fan's nesting there is the reconcile's turning, not a tree.

## Owed

In stage 2 (one walk and entry type, `Star`), every plan at a shared vertex reads one walk, and its cuts fall in one order. In that order the cross-pair nesting is `ring_tree`'s parent relation over all the vertex's chords. Turning becomes the choice of root and side, and the code tie becomes the order's tie reading. Fold `holds_whole`, `nests` and `strut_anchor`'s ordering into it then, or say why they stay.

A third nesting reading outside `topo::star`'s scope: `boolean/zip.rs` `peel` nests the cones of a body's own non-manifold vertex. That is a non-crossing partition, where a cone may hold several runs, not chords. No surface cuts that vertex, so DESIGN's `topo::star` row does not cover it. It is listed here so a later sweep of nesting readings finds it.
