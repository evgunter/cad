---
id: a-linear-patterns-direction-components-land-in-its-slides
kind: issue
title: the linear-pattern gesture writes a typed direction's components into slide formulas, unnormalized, so a diagonal row steps spacing·√2
status: open
opened: 2026-11-02
priority: P1
cost: M
---


Found by the PLACING sweep Ev asked for on PR 6281 (2026-11-02): "can you see if anything similar also got missed?" by the INTENT refactor. Read at `3bf15bef9c`; traced in code, not run unless said.

**Ruled.** `[variables.poses]`: "no slot holds a component of a direction". `[repetition]`: a row is "a slide of `scalar(k)·pitch`".

**Survives** (sure on the code; the pitch effect is traced, not run).
- `combine::row_steps` (`crates/viewer/src/combine.rs`) writes `Slide(axis_i) = spacing·d_i` for each nonzero component of the typed direction.
- Those components are `Formula::literal(…, Scalar)` from `drafts::scalars` (`crates/viewer/src/drafts.rs`), and `pncad::pattern::linear_pattern` multiplies each by `k`.
- So each direction component is a typed Scalar in a stored slot.
- A direction that is not a unit vector steps by `spacing·|d|`, so `(1,1,0)` steps `spacing·√2`. Every `combine_ops` row uses `(1,0,0)`, so nothing pins it.

**Owed.** First a row that reproduces it with a diagonal direction (if the pitch is right after all, close this as the prose half only). Then the row's direction as a pose read, a `Direction` or an `Axis`, with one slide of `scalar(k)·spacing` along it. The circular twin is `a-ring-about-a-tilted-axis-slides-each-member-along-it`, a different defect.

## Weighed by three designers (2026-11-02, fork row 242) and put to Ev

**Agreed by all three.** A placement value names the direction it runs along or the axis it turns about:
- a slide runs along the mated direction (`Slide(None)`), along a target axis carried by the least turn, or along a `Direction`/`Axis` read off a body in the target space;
- a spin may turn about a read `Axis`.

The copy stands at the sum of its slides. A row's source is its first copy `h0`, slid `scalar(k)·pitch` along the picked edge, with every other value `0`. A direction typed as numbers has no home: the gesture offers a pick or a target axis instead.

Only `[placement.values]` changes. The `[ev]` PR carries the diff and the three designers' texts. PR 6364 holds the disabled-red reproducer and the axis-aligned gate.
