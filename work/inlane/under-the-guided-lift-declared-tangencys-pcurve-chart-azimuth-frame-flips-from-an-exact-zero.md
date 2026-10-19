---
id: under-the-guided-lift-declared-tangencys-pcurve-chart-azimuth-frame-flips-from-an-exact-zero
kind: issue
title: Under the guided lift, declared_tangency's two pcurve_chart_azimuth_frame margins were exactly zero at the nominal, so the revalue apart move flipped them; PCERT B1 removed that decision
status: open
opened: 2026-10-17
priority: P3
cost: M
---

Split out of `under-the-guided-lift-plate-params-apart-move-parts-at-a-point-exactly-on-its-loop` by PR 5257's review. That item now names one mechanism, `walk_schedule`'s ray gate, and its fix would not close these two flips.

- **Row:** `revalue_declared_tangency`, the apart move, under the guided lift.
- **Decisions, before PCERT B1:** #2874 and #2961, both `pcurve_chart_azimuth_frame`. Each margin was 0.0 exactly at p0 and −1.5416663312218278e-7 at p1, so the sign flipped.
- **The builds did not part.** All 2,965 decisions of that tree compared, and nothing re-valued away after the flips (`after_flip` was empty). Under PCERT B1 the census compares 2,989 (Evidence, below).
- **Repro:** `REVALUE_DETAIL=1 REVALUE_DOC=declared_tangency cargo nextest run -p editor-core --run-ignored only -E 'test(=revalue_corpus::revalue_census)' --no-capture`. The census line lists `flips`.

**The predicate, as it was.** `stable_azimuth` chose between `atan2(y, x)` and `atan2(−y, −x) + π` by the sign of `x` (`pcurve_chart_azimuth_frame`). The two forms agree mod τ and differ by 2π on the negative-`y` side. At the nominal the point sat on the chart frame's y axis (`x` exactly 0), so the zero arm took the direct form; the move put `x` a hair negative and took the other. That frame decision no longer exists (PCERT B1, below).

**Pinned.** The row owes no flips and no `after` entry: `revalue_declared_tangency: "declared_tangency", Apart, 1217, []`. A row that names no flips owes none, so it goes red if any decision flips.

**Downstream of the flip, as it was (the ε-legs fix, `inlane/revalue-move-scales-with-eps`).** After the first flip, the chart's `pcurve_map_residual` margins were a few ulps, computed in the other frame at `p1`; at ε 1e-6 they re-valued to other bits than `p1`'s. The row named `pcurve_map_residual` in its `after` bound for that. With the flips gone it computes nothing in the other frame, and the entry is gone with them.

**Evidence, PCERT B1 (PR 5225, 2026-10-18): the premise is gone with that PR.**
- PCERT B1 decides the frame across the window's cut line (`pcurve_chart_cut_side`), not on `x`'s sign, so the two exact-zero flips at #2874 and #2961 are gone.
- The census after, with the repro above: `flips []`, 2,989 compared, nothing laundered, `after_flip {}`.
- The row's `flips` entry came out with that PR's merge of main (`DECLARED_TANGENCY_FLIP` removed); the row now owes no flips.
- A direction in band on the cut line itself (θ = −1) reads the closed end τ − 1 at every scalar, pinned by `a_direction_on_the_windows_cut_reads_its_closed_end_at_every_scalar`.
- With the flips gone, the row's `after` entry for `pcurve_map_residual` came out too: no frame flips, so nothing is computed in the other frame at `p1`.
