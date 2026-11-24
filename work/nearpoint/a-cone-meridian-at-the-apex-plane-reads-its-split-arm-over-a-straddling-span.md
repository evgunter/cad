---
id: a-cone-meridian-at-the-apex-plane-reads-its-split-arm-over-a-straddling-span
kind: issue
title: require_cone_generator's split arm divides by h1 - h0 over the whole box, unbounded for a meridian segment within rounding of the apex plane
status: open
opened: 2026-11-24
---


Found by `boxfinish/wall-roots`'s sweep (the class of
`geom_brep::quartic::quadratic_roots`: a value selection whose arm is
bounded only on its own side of the selector, evaluated over the whole
box at `Interval`). Read from the code, not run.

`geom_brep::props::curved::require_cone_generator`: `worst =
(h0·h1).select_le_zero(span.select_le_zero(whole, split), whole)`,
where `split` reads `qc = (q0·h1 − q1·h0)/(h1 − h0)`, the segment's
crossing of the apex's plane. Where `h0·h1` straddles zero the hull
takes `split` over the whole box; with both ends within rounding of
that plane `h1 − h0` contains zero, `worst` is the whole line, and
`props_meridian_on_surface` refuses though `whole` is bounded. Only a
meridian segment at the apex itself reaches it. Not established: that
any fixture does.
