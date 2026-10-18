---
id: under-the-guided-lift-declared-tangencys-pcurve-chart-azimuth-frame-flips-from-an-exact-zero
kind: issue
title: Under the guided lift, declared_tangency's two pcurve_chart_azimuth_frame margins are exactly zero at the nominal, so the revalue apart move flips them
status: open
opened: 2026-10-17
priority: P3
cost: M
---

Split out of `under-the-guided-lift-plate-params-apart-move-parts-at-a-point-exactly-on-its-loop` by PR 5257's review. That item now names one mechanism, `walk_schedule`'s ray gate, and its fix would not close these two flips.

- **Row:** `revalue_declared_tangency`, the apart move, under the guided lift.
- **Decisions:** #2874 and #2961, both `pcurve_chart_azimuth_frame`. Each margin is 0.0 exactly at p0 and −1.5416663312218278e-7 at p1, so the sign flips.
- **The builds do not part.** All 2,965 decisions compare, and nothing re-values away after the flips (`after_flip` is empty).
- **Repro:** `REVALUE_DETAIL=1 REVALUE_DOC=declared_tangency cargo nextest run -p editor-core --run-ignored only -E 'test(=revalue_corpus::revalue_census)' --no-capture`. The census line lists `flips`.

**The predicate, read from the code.** `stable_azimuth` (`crates/geom-brep/src/pcurve_cache.rs:7480`) chooses between `atan2(y, x)` and `atan2(−y, −x) + π` by the sign of `x`. The two forms agree mod τ and differ by 2π on the negative-`y` side. At the nominal the point sits on the chart frame's y axis (`x` exactly 0), so the zero arm takes the direct form; the move puts `x` a hair negative and takes the other. Nothing re-values away after, which fits two forms equal mod τ. What remains to read: whether the frame choice should be held at an exact zero by name, or whether the choice is immaterial and so should not be a sign decision at all.

**Pinned.** The row owes both flips here through its `flips` entry, and a row that names no flips owes none. So the row goes red when they go, and when any other decision flips.
