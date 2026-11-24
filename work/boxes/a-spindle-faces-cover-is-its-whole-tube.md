---
id: a-spindle-faces-cover-is-its-whole-tube
kind: issue
title: A spindle face's cover is its whole tube, so the session box refuses as a limit a body in the box whose tube pokes out
status: open
opened: 2026-11-21
priority: P2
cost: M
---

Found by the r2 review of PR 7225 (MAJOR 1), measured by that PR's lane.

**What.** `topo::cover::face_cover`'s spindle arm is the carrier's tube: the chart window's closed form (`torus_chart_window`) reads a ring torus only. Since PR 7225 the session box refuses as the limit `SessionBoxUnshown` a body in the box whose spindle tube pokes out of it.

**Measured.** In r2's 756-placement differential (14 revolved profiles, ε = 1e-9), 70 of the 85 in-box bodies refused as a limit have a spindle face: the lemon (26), the apple (14), and the two profiles r2 labels torus parts, the D bulge (12) and the lens's outer arc (18), whose arcs revolve with `R < r`. The overshoot is up to the tube's diameter.

**The shape of a fix.** A spindle face's chart window, as a ring torus's is read, so its cover is the patch its boundary spans.

**Why P2.** The session box (PR 7225) refuses a valid body that main admits, wherever such a face's whole carrier reaches past the box. It needs a face whose carrier stands within its own diameter of a box face, so not everyday parts, but site-scale models stand there by design.
