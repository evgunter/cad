---
id: no-fitted-class-misuse-is-excused-by-not-owed
kind: issue
title: not_owed excuses UncoveredClass::NoFittedClass, a wrong image offered to the fitted door, as if it were a class with no route
status: open
opened: 2026-10-07
---


Found by the spline-carrier designers (PR 4261, fork-log row 85).

`UncoveredClass::NoFittedClass` (`crates/geom-brep/src/pcurve_cache.rs`) is what the fitted door answers when it is offered a carrier that is a line, an ellipse or a spiric. Those carriers' images are closed-form or iso. So this is a caller's misuse, not a class of carrier the chart holds with no route yet.

`not_owed` (`crates/topo/src/pcurves.rs`) excuses it alongside the genuinely uncovered classes, so a misrouted face is left uncached silently instead of refusing loudly.

It should leave the excusal and refuse as a defect. Check first whether any producer reaches it, because if one does, that producer is wrong.
