---
id: a-failed-boolean-says-so-only-in-its-tree-row
kind: issue
title: viewer: a failed boolean is reported only in its feature-tree row, so its parts vanish from the view with no word where the person is looking
status: open
opened: 2026-11-22
priority: P1
cost: M
---

**Ev reported this** (in chat, 2026-11-22): "if a boolean fails, there appears to be no error displayed at all (?), parts just disappear. oh actually i see it, i had to go up to the list of parts and scroll down".

The failure reaches only its tree row: `RowStatus::Failed` (`crates/viewer/src/tree.rs:1314`), drawn by `failure_lines` (`crates/viewer/src/pane/features.rs:231-261`). `badge_site` (`crates/viewer/src/frame.rs:2848-2852`) sends `ReadFailed` / `ReadPoisoned` / `UnknownNode` to `BadgeSite::FeatureTree` on purpose. The comment at `frame.rs:2802-2840` argues a frame badge "would say strictly less", so `product_badge` (`frame.rs:2904`) is silent and nothing in the viewport or toolbar says anything failed. That policy came from the closed `work/vnews/the-at-rest-badge-restates-a-failed-root-louder-than-its-row.md`.

Ev's report is the case that policy misses: the row that says it can be scrolled out of sight while the geometry vanishes on screen. The fork is how to keep the row as the one place that tells the failure while making it reachable from where the person is looking. Options include a frame line that names the failed node and links to its row (`a-node-name-the-viewer-shows-is-not-a-link-to-it`), scrolling the tree to the row when the commit that failed it lands, or both. It is weighed with the earlier ruling, hence `design: true`. The geometry half (keep the operands drawn, mark the failure site) is `a-failed-boolean-hides-its-operands-and-marks-no-failure-site`.
