---
id: the-knot-split-cuts-a-printed-spline-at-a-crease-on-its-trims-end
kind: issue
title: the C0-knot edge split cuts a printed spline at a crease that stands at its edge's own end, minting a zero-length piece the build refuses
status: open
opened: 2026-11-21
priority: P2
cost: E
---


Found by EXCH unit D (`exch/adopt-by-description`), measured with curve
promotion switched off in a local probe.

`tests/fixtures/arc-trim/w_semi.step` states its rim's carrier `#1023` as a
rational semicircle with a double knot at `t = 1` (the 90° point), and the
rim edge trims it over `[0, 1]`: the crease stands exactly at the edge's
end vertex. On main the carrier promotes to its circle, which has no knots,
so `knot_split::split_edges` has nothing to cut. Where recognition declines
such a carrier, the split cuts at `t = 1 = t1` and mints a zero-length
piece; the import then refuses at assembly with `IntervalNotForward`
(margin 6e-19), a file the solid path otherwise reads on its description.

The split reads the printed carrier's kind before any description is
chosen (`[ev]` PR 6014: promotion first, then the exact split), so it is
one of the readers for which promotion stays a hint. The defect is
narrower than that: a crease at or outside the edge's trim is no crease
of the edge, and should not be cut. `split_edges` states the opposite as
its premise ("A B-spline edge spans its carrier's whole knot domain
... so every crease knot is strictly inside the edge"), which a trim on
part of a spline breaks. Fix inside `split_edges`: only knots
strictly inside `(t0, t1)`, read at the file's band.
