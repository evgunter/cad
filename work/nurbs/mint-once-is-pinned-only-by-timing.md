---
id: mint-once-is-pinned-only-by-timing
kind: issue
title: Mint-once is pinned only by timing: a never-filled chart slot leaves every row green
status: open
opened: 2026-10-20
---


**P3.** Found by #5419's FULL review (https://github.com/evgunter/cad/pull/5419#issuecomment-6285986929).

**What.** #5419 mints each wall's `ChartedNurbs` once per operation, through `topo::offset_derive::WallCharts`'s `OnceCell` slots. The review's mutant M3, where a slot is never filled so every section re-mints, leaves every row green: only timing shows mint-once. A later change that quietly re-mints would pass review.

**Done when** a row counts mints, like derivative's test-only `WEIGHT_HALVINGS`. On the shelling fixture it asserts 5 mints for 4 walls: one per wall in the planning frame, plus the post-frame `plane_nurbs_limbs` mint. M3 must go red on it.

**Ride-alongs** from the same review:
- the `Wall` doc still says the door "refuses", though topo now panics on `WrongLane`;
- the slot type is spelled out four times in nested tuples;
- the moved-slot keying relies on a unique `old_key`, which the `planned_charts` test hook doesn't gate;
- two dead `work/shell/` citations in `replace_face.rs` (`:2099`, `:3682`).
