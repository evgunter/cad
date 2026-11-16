---
id: the-torus-chart-window-abandons-a-face-that-holds-a-ring
kind: issue
title: cover::TorusChartWindow::open_loop abandons the torus window on any ring, where Body::face_lift now places the ring in its outer loop's sheet
status: open
opened: 2026-11-14
priority: P3
---


Found by the sweep of `a-rings-lift-is-placed-against-its-outer-loop-in-one-place` (branch `tesslane/ring-placement`).

`cover::TorusChartWindow::open_loop` (`crates/topo/src/cover.rs`) sets `ok = false` on every loop after the first, so a torus face with a ring gets no chart window. The cover then falls back to the whole tube: wider, never wrong.

A face's window is the hull of its region, and a placed ring lies inside its outer loop's region, so the window needs the outer loop alone. A ring's lift as stored can sit a period away (`Body::face_lift`'s module docs), so reading the rings' stored images would widen the window wrongly. Skipping the rings serves; reading them through `Body::face_lift` serves too, but adds nothing to the hull.

Not measured: which callers of `face_cover` lose a decision because a ringed torus face reads the whole tube.
