---
id: a-construction-reads-a-tied-or-ranked-vertex-name-as-one-point
kind: issue
title: The coincidence walk reads a tied or ranked vertex name as one construction
status: open
opened: 2026-10-23
priority: P3
cost: E
refs: [a-plane-poses-foot-outgrows-the-coincidence-doors-budget]
---

Found by PR 5624's review (MSOLVE; probes on `review/msolve-plane-pose-R1`), filed by its implementer.

`coincide::construction` (`crates/editor-core/src/coincide.rs`) reads a minted vertex by its name. `one_construction` then calls two cells one construction when their names are equal. Two shapes of vertex name can be equal for two different points.

- **A tied name.** Where a split cannot orient a crossed edge, `rank_crossings` (`crates/editor-core/src/names/emit_topo.rs`) falls back to `mint_candidates`, and the crossings of one sense on that line share one `Tied` name (`NameTable::is_tied`). The walk never asks `is_tied`, so two tied crossings read as one construction. PR 5624 reads a crossing against the split's `Above` half, which extends this across the two halves.
- **A ranked name, truncated.** The boolean's ranked vertex roles, `Crossing { … }` and `EdgeCrossing(…)` (`crates/editor-core/src/names/role.rs`), tell several vertices apart by a trailing `Fragment(OrderAlong)`. The walk's minted arm truncates every fragment tail (`fragment_tail_start`), so two ranked crossings read as one construction. PR 5624 keeps the tail for a split's `CrossingVertex`, but not for these.

One class: a vertex name whose qualifiers are what tell two points apart, read by the walk as one point. A row needs both cells at once for this to matter, and every row the kernel records is a coincidence it decided, so no wrong proof is known today. The walk claims more than the names support, though.

**Done when** the walk reads a tied vertex name as no construction (or one per candidate), and keeps a ranked vertex's ordering qualifiers. A row should pin each case.

## Partly moot (WIRE orchestrator, 2026-10-26)

Closed by PR 5758 (2026-10-26, "nothing is ranked", Ev's ruling on PR 5705). Nothing is ranked any more, so the ranked half is moot. The tied half stands: a construction still reads a tied vertex name as one point.
