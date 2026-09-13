---
id: a-chart-spans-solids-after-move-shells-to-new-solid
kind: issue
title: move_shells_to_new_solid re-homes a shell without re-minting its surfaces, so a chart can span two solids
status: open
opened: 2026-09-08
---



Measured by SHELL-8's R2 reviewer (PR #2207, 2026-09-08; probe
`r2_chart_spans_solids_through_subtract_then_move_shells` on
`shell/8-r2-probes` @ eaf2976b4) and placed here by the SHELL
orchestrator: `topo::subtract(brick 6×1×1, brick 1×3×3 across its
middle)` is a disconnecting subtract that files both fragments under
ONE solid with TWO shells, and the fragments of each cut operand face
keep their operand's surface key — so a chart (one `SurfaceKey`)
holds faces on both shells. That is legitimate inside one solid.
`Body::move_shells_to_new_solid` (`crates/topo/src/movefac.rs:228`)
then re-homes one shell into a solid of its own and leaves every
surface key as it was: after the move the chart spans two solids, a
state no other producer builds (grafts and instancing mint fresh keys
per transplanted entity, `instance.rs:123`, `graft_solids_with`).
SHELL-8 assumed "a chart lives in one solid" (its per-solid doors
group faces by chart and refuse `ShellError::ChartSpansSolids`
typed when the premise fails, `crates/topo/src/shell.rs`), and its
own PR text claimed every producer mints per solid — false, this
path. Either the mover re-mints the moved shell's surfaces (one
surface per solid, the invariant SHELL-8 wants stated at the
`Body` level), or the invariant is not one and the shell doors'
grouping is per (solid, surface) — the choice is TOPO's, with S-BOOL
on the disconnecting-subtract half (see
`work/bool/subtract-of-a-hollow-operand-files-the-island-under-one-solid`,
the same one-solid filing on a different shape). Signed (SHELL
orchestrator).
