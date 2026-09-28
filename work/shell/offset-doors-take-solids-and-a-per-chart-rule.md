---
id: offset-doors-take-solids-and-a-per-chart-rule
kind: unit
title: The offset doors take the solids plus a per-chart distance rule; an inward-by-t door sits over them; ChartMove and its policing refusals go
status: open
opened: 2026-09-28
priority: P3
cost: H
---


## Ruling

Ev ruled on PR 3310 (2026-09-28): *"the recommendation sounds good!"*. The question and the two designers' reports are in that PR. The design-fork record is row 2 of `docs/DESIGN-FORK-LOG.md`, and the originating row is `work/dup/the-chart-partition-has-a-topo-src-home-and-a-test-home.md`.

## What to build

1. **One home for the grouping.** `group_by_chart` moves into the offset module as the single crate-private home, in first-appearance face-arena order (D9).
2. **A read-only chart view.** It exposes the surface key, faces, surface, and `sense() -> Option<bool>`, which is `None` when the faces disagree.
3. **The simultaneous doors take solids and a rule.** `offset_planes_together` and `offset_charts_together` take `(body, solids, rule: Fn(&ChartView<T>) -> T, band, tol)`. The door builds the charts of the named solids and calls the rule once per chart.
   - Delete `ChartMove`, `scope_of_moves`, the check block copied in both doors (`offset_together.rs` ~159–185, `offset_axial.rs` ~381–404), and the refusals `EmptyGroup`, `TogetherChartMixed`, `TogetherFaceRepeated` and `TogetherPartialSet`.
   - One chart split across two moves then can't be written. Today it fails late, and its message blames the wrong thing.
4. **`replace_faces_offset` takes the chart's surface key.** Delete `GroupChartsDiffer` and `SharedSurfaceKey`.
5. **A data-shaped inward door over the rule door.** It is `offset_solids_inward(body, solids, t, band, tol)`, the `offset_inward` of OFFSET-DESIGN O4.
   - It owns the door ladder (`shell`'s `offset_door`: planar, axial, or chart by chart), the sense turn (`shell::inward`), and the mixed-sense refusal (`ChartSenseMixed` moves here).
   - The general rule door does not gate sense.
6. **`shell` moves onto the new doors.** The cavity becomes one inward call per solid. The open-shell lift becomes `|c| if c.key() == counterpart { back } else { zero }`. `chart_groups` and `faces_wearing` go wherever nothing else needs them; rim surgery may keep a private grouping.
7. **Tests.**
   - Sweep's hollowing rows use the inward door.
   - `sweep/tests/common/charts.rs` shrinks to distance rules, or goes.
   - `topo`'s `scope_walks::moves_of` goes.
   - Tests that build deliberately malformed move sets are deleted, because the type now forbids those states.
   - Every test of the door's own behaviour is rewritten as a rule. That covers per-plane distances, zero moves, one solid of several, the interval lanes, `TogetherNonPlanar` and `TogetherCorner`.
   - New rows pin:
     - a solid the call does not name comes back bitwise untouched;
     - a stale solid refuses with a typed error;
     - the rule is called once per chart, in arena order;
     - a chart moved by zero keeps its key;
     - the inward door refuses a mixed-sense chart.
8. **Docs.** O4's sentence naming the door ladder is re-worded with the change. The side findings in `offset-doors-small-doc-and-message-drift` are fixed along the way.

## Left to the implementer

- **Closure or data.** A closure is preferred; a door-built `SurfaceKey → distance` table is the fallback if a printable form is needed.
- **Where the door ladder sits.** It may also go behind the rule door, not just the inward door.
- **The all-planar lift.** Whether it can leave the per-chart door, which it takes on purpose today.
