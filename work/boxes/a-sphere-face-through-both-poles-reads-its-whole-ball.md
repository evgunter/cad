---
id: a-sphere-face-through-both-poles-reads-its-whole-ball
kind: issue
title: A sphere face that runs pole to pole reads no chart window, so its cover is the whole ball and the session box refuses as a limit a body in the box whose ball pokes out
status: open
opened: 2026-11-21
priority: P2
cost: M
---

Found by the r2 review of PR 7225 (MAJOR 1), measured by that PR's lane.

**What.** `topo::cover::face_cover`'s sphere arm meets the ball with the face's chart window (`sphere_window`) only where the outer loop encodes its side (`geom_brep::props::boundary_material_sign` reading `Encoded`). A face whose loop runs through both poles (a ball revolved part of a turn, a wedge) reads `Unencoded`: the chart image drops the loop's polar tips, and what is left encodes no side. So its cover is the whole ball, and since PR 7225 the session box refuses as the limit `SessionBoxUnshown` a body in the box whose ball pokes out of it.

**Measured.** A ball of radius 300 revolved 1.1 rad, turned to face `−x`, its axis at `x = 900`: every point within 900 m, the ball reaching 1200 m; refused as a limit 200 m past, where main admits it. The row `sweep` `session_box_reach::a_ball_wedge_in_the_box_whose_ball_pokes_out_refuses_as_a_limit` holds the same wedge at radius 30, its axis at `x = 990`, refused 20 m past (the radius-300 wedge does not revolve at ε = 1e-12). In r2's 756-placement differential, 15 of the 85 in-box bodies refused as a limit are such wedges. The overshoot is up to the ball's diameter.

**The shape of a fix.** A side for a loop through both poles read another way (the wedge's two meridians bound an azimuth window whatever the tips), so the window covers the wedge. Not a few lines: the side reading is the material sign's.

**Why P2.** The session box (PR 7225) refuses a valid body that main admits, wherever such a face's whole carrier reaches past the box. It needs a face whose carrier stands within its own diameter of a box face, so not everyday parts, but site-scale models stand there by design.
