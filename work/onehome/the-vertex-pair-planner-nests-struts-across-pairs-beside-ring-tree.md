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

An instrumented side-by-side run over the topo suite and the first 1740 tests of the sweep suite (`b_runs` and `hang_at_shared` both read `arc_holders`). 53 931 plans compared the old `arc_holders` (shortest holding arc per run) with `ring_tree` over the plan's walk positions, rooted at the region outside every run. That is 28 000 at n = 2, 2 494 at n = 4, 309 at n = 6, 1 883 at n = 8 and 13 440 at n = 10. Every one matched. The one mismatch was the synthetic `Cover` row of `arc_holders_nest_the_runs_a_turn_reroots_and_refuse_a_cover`, which passes 2 arcs over 6 positions. The new reading compresses to the occupied positions and gives `Cover` there too. So `arc_holders` now reads `ring_tree` (`boolean/insert.rs`, `arc_holders`), and `held_by` keeps reading its holders.

## What is not `ring_tree` as it stands

`holds_whole`, `nests`, `segment` and `strut_anchor` (`boolean/insert.rs`) nest **struts of different pairs** at a vertex two plans share. This reading differs from `ring_tree` in three ways:

- **Domain.** Each plan's walk positions order only its own germs. Cross-pair nesting reads the vertex's geometric cut order (`walks_before`, `cut_order`) across plans, after `reconcile_shared` has turned runs onto their complements. No single walk holds every plan's germs.
- **Ties.** Two struts tied at both ends (one segment) nest by their codes: the Out one holds. `ring_tree` reads a strict cyclic order and has no tie.
- **Struts only.** It reads only struts in one physical sector. A fan's nesting there is the reconcile's turning, not a tree.

## Owed

In stage 2 (one walk and entry type, `Star`), every plan at a shared vertex reads one walk, and its cuts fall in one order. In that order the cross-pair nesting is `ring_tree`'s parent relation over all the vertex's chords. Turning becomes the choice of root and side, and the code tie becomes the order's tie reading. Fold `holds_whole`, `nests` and `strut_anchor`'s ordering into it then, or say why they stay.

A third nesting reading outside `topo::star`'s scope: `boolean/zip.rs` `peel` nests the cones of a body's own non-manifold vertex. That is a non-crossing partition, where a cone may hold several runs, not chords. No surface cuts that vertex, so DESIGN's `topo::star` row does not cover it. It is listed here so a later sweep of nesting readings finds it.
