---
id: the-tour-authors-its-numbers-through-the-bare-door
kind: issue
title: The tour enters the numbers its program wrote through Real::from_f64, beside the authoring door
status: open
opened: 2026-11-24
priority: P3
cost: E
parent: from-f64-is-split-by-where-the-number-comes-from
---


Found by `from-f64-is-split-by-where-the-number-comes-from`, whose gate (`scripts/gates/from-f64-allowlist.sh`) now walks `demos/*/src`. The tour's scenes are generic over the scalar and spell `S::from_f64(x)` for the numbers their program wrote: 83 sites in `demos/tour/src`, 48 with a literal argument (exempt by shape, and counted on `literal-from-f64-sites-move-onto-the-constant-door`) and 35 whose argument is a local, a field or an `f64` expression (`r`, `m.rf + half`, `z.1 - z.0`, `spec.center.map(S::from_f64)`), listed `owed` to this row in `scripts/gates/from-f64-sites.tsv`.

Each is a number the demo's author wrote or computed at `f64` from ones they wrote, before the kernel saw it: an authored value in DL5.2's sense (`crates/geom-core/README.md`), and `pncad::authoring::real` is the door a programmatic author's number enters by. The demos are written the way a user would write them (`docs/prompts/implementer-discipline.md`, section 3), so the move is `real(x)` at each site, and what it shows is whether the authoring door reads naturally in a scene generic over `S`.

Not this row: the frames and points `lily.rs`, `klein.rs` and `projectbox.rs` enter through `from_computed` (`f64-data-left-lifted-when-the-built-at-t-unit-closed`, `the-tour-enters-computed-frame-inputs-through-from-computed`).

Cost: E.
