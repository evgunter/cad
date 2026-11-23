---
id: a-torus-face-bounded-by-non-harmonic-images-reads-its-whole-tube
kind: issue
title: A ring-torus face bounded by an edge whose chart image is not harmonic (a spiric section a cut leaves) reads its whole tube as its cover
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Filed by PR 7225 (`oneround/box-region`), on its reviewers' fix-pass finding (r1 MAJOR 1, r2 D1).

**What happens.** A ring-torus face's cover is the torus met with its chart window (`topo::cover`'s torus arm, `torus_chart_window`). The window walk (`TorusChartWindow::step`, `crates/topo/src/cover.rs:1107`) reads each boundary step's closed-form chart box. It abandons the window wherever an image is not harmonic (`:1118`), and the face then covers the whole tube. A boolean's cut of a torus leaves spiric section edges, whose images are `Pcurve::Spiric`, so every torus face a cut leaves reads the tube whole. The window is also abandoned on:
- a ring (a second loop);
- a loop that winds a channel;
- a lone vertex;
- a loop with no lift;
- an envelope past half the tube.

**What it costs.** The session box refuses such a piece as a limit wherever the tube reaches past the box and no point on the body does. The overshoot is up to the tube's whole span, `2(R + r)` across. `sweep session_box_reach::a_ring_torus_face_a_cut_leaves_reads_its_whole_tube` measures it: the bar's cut of an `R = 700`, `r = 50` torus reaches 743 m when moved 300 m, and is refused 50 m past by the tube's 1050 m.

**Why P2.** It refuses a valid in-box body that main admitted. The box is not wrong, but the bound is too loose to read the body.

**The shape of a fix.** `Pcurve::chart_box` already bounds every image kind conservatively, `Spiric` among them, by its whole-period box (`crates/geom-brep/src/pcurve_cache.rs`, the `Spiric` arm). Reading it where the walk now abandons would give a window. But a whole-period spiric box can span the whole channel, so it would tighten only some faces. A certified window from the trim loops' own spans is the real fix. The slack argument (`torus_window_slack`) would have to be re-derived for images whose chart box is not their span's. Measure the class this moves with the PR's differential.
