---
id: a-one-face-ball-on-a-spline-wrap-edge-meets-the-trimmed-lane-unmeasured
kind: issue
title: A one-face ball whose wrap edge is not an iso carrier routes to the trimmed lane, which has not been measured on an all-spur loop
status: open
opened: 2026-11-11
priority: P3
cost: E
---


PR 6823 taught the iso-rectangle walk the one-face ball: a sphere face
whose loop walks one wrap edge from pole to pole and back
(`mesh::walk::wrap_strip`, `crates/mesh/src/walk.rs`). That walk sees the
face only when the dispatch sends it to the curved lane. The dispatch
(`crates/mesh/src/tessellate.rs`, near line 259) sends a face to the
trimmed lane first where `trimmed::has_trim_carrier` holds (a conic or
B-spline trim carrier, `crates/mesh/src/trimmed.rs` near line 372), or
where a sphere face carries a tilted circle (`trimmed::has_tilted_circle`,
near line 281, through `takes_sphere_arm`).

So a one-face ball whose wrap edge is a spline from pole to pole, or any
carrier other than a meridian circle, never reaches the strip. It meets
the trimmed lane, which places each loop by its lift
(`topo::Body::loop_lift`, read at `crates/mesh/src/memo.rs` near line
755) and the sphere arm's pole placement (`trimmed::choose_pole_azimuths`,
near line 2099). Nobody has measured that lane on a loop that is all
spurs. Its chart image is empty, so the polygon the lift draws has zero
width unless the lane reads the tip's reset the way the walk now does.

**Reach.** No door mints a wrap edge that is not a meridian today. The
seam clause puts a wrap edge "round the axis" at a conventional place,
and revolve, import and the Euler doors all mint a circle meridian.
A STEP file stating a spline seam pole to pole is the plausible route
once import mints the wrap tip
(`work/importgate/import-mints-the-wrap-tip-at-a-chart-singularity.md`).

**What it needs:**
1. Build the body (the one-face ball with its wrap edge restated on a
   spline that interpolates the meridian).
2. Record what `tessellate` answers.
3. If the answer is a hole or a census panic, have the trimmed lane read
   the empty chart image as the strip does, or refuse it typed.
