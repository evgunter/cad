---
id: no-fitted-class-misuse-is-excused-by-not-owed
kind: issue
title: not_owed excuses UncoveredClass::NoFittedClass, a wrong image offered to the fitted door, as if it were a class with no route
status: closed
opened: 2026-10-07
closed: 2026-10-12
branch: pcert/fitted-row-reads-carrier-and-chart
---


Found by the spline-carrier designers (PR 4261, fork-log row 85).

`UncoveredClass::NoFittedClass` (`crates/geom-brep/src/pcurve_cache.rs`) is what the fitted door answers when it is offered a carrier that is a line, an ellipse or a spiric. Those carriers' images are closed-form or iso. So this is a caller's misuse, not a class of carrier the chart holds with no route yet.

`not_owed` (`crates/topo/src/pcurves.rs`) excuses it alongside the genuinely uncovered classes, so a misrouted face is left uncached silently instead of refusing loudly.

It should leave the excusal and refuse as a defect. Check first whether any producer reaches it, because if one does, that producer is wrong.

## Closed

The premise is refuted. A line, ellipse or spiric on a spline chart is
not a misuse: a spline chart has no closed-form image of an off-iso
carrier, so `General` is that carrier's honest image. The fitted door
refused it only because its check 4 ran the SSI pair certificate
(hull and tube), which needs a spline carrier and a mate. C4 already
says the row reads no mate, so check 4 is now the hull bound against
the row's own chart for a carrier of any kind, and check 1 reads only
the chart (`pcert/fitted-row-reads-carrier-and-chart`).
`UncoveredClass::NoFittedClass` and `FittedMateMissing` are deleted,
so `not_owed` no longer excuses that pair. A line or a spline stated
on the lamina certifies and mints at every certifying scalar
(`stated_general_image_mint`).
