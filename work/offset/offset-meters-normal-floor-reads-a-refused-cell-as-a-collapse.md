---
id: offset-meters-normal-floor-reads-a-refused-cell-as-a-collapse
kind: issue
title: offset_meters: a cell whose normal enclosure is refused floors at 0 and refuses NormalFloor as a collapse, not as unmeasured
status: open
opened: 2026-10-12
priority: P2
cost: M
---


Found by PCERT's sweep of the "refused enclosure → 0.0 → read as
geometry" class (`pcert/spiric-reimport-tube`, which fixed the
uniqueness tube's instances).

## The finding

`offset_meters::cell_normal` joins three assemblies of a lower bound on
`‖S_u × S_v‖`. Each one answers `0.0` where its enclosure is refused:
`mig` on a refused component, and `root_a`, `q` and `root_c` each asked
`is_certified` by name. Its doc calls that "the floor's conservative
answer". As a lower bound it is sound.

`PatchRegularity`'s floor is the minimum over cells, so one refused cell
floors the patch at `0`. `offset_normal_floor` then decides
`Margin::over_lever_down(0, lever)` and refuses
`MeterError::NormalFloor { floor: 0.0, verdict: Zero(..) }`. That says
the patch's normal collapses: a measured verdict on the geometry. In
fact nothing was enclosed in that cell. A cell whose enclosure
straddles zero reads the same, and there the verdict is honest.

## What a fix owes

Tell the two apart: carry a refused cell to `MeterError::Escalated` (or
a typed "unmeasured" refusal) rather than to a `Zero` floor.

Mind the fold. `if n.floor < floor` skips a NaN, so a NaN floor would
silently drop the cell, which is unsound. The refusal has to be carried
explicitly; do not encode it as NaN in the fold.

The tube's fix is the pattern: `ssi::enclose::tube_margin` returns
`None` for a refused enclosure, and the probe reads that rung as
unprobed.

Unmeasured: whether any reachable patch has a refused cell. A witness
needs a cell whose derivative enclosure is NaI from finite, validated
input.
