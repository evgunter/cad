---
id: a-hover-diff-counts-the-index-builds-progress-label
kind: issue
title: app's whole-app hover diff counts the toolbar's indexing label when the index build changes state between its two frames
status: closed
closed: 2026-09-29
branch: chrome/viewer-small
opened: 2026-09-29
priority: P3
cost: E
---


`crate::app`'s `properties_pane_tests` harness answers
`gained_hovering(text)` as the runs a hovered frame paints that a quiet
frame did not. Both are frames of the whole app, and the toolbar's
progress read (`frame::progress` in `toolbar_ui`) paints `indexing…`
while the pick index is building on its background thread. When the
build changes state between the quiet frame and the hovered one, the
label counts as something the hover added.

Seen once in four full `cargo test -p viewer --all-features --lib` runs
on `chrome/status-line` (2026-09-29, a loaded shared box), and not in
three runs of the test on its own:
`a_driven_slots_unit_picker_is_disabled_with_the_refusal_it_would_get`
failed with `left: ["indexing…", "the distance slot on node 2 is
computed, …"]`. The branch does not touch the progress read or the
harness.

The repair is the harness's: wait for the index seam to settle before
the quiet frame (the startup document's build is finite), or compare
only runs inside the pane under test.

## Closed (2026-09-29, `chrome/viewer-small`)

The harness settles before its quiet frame (`crate::app`'s
`properties_pane_tests`, `Driven::settle`/`Driven::settled`):
`ViewerApp::progress` (the toolbar's read, now one method the harness
calls too) is `None` AND the pick cache holds an answer for the picture
on screen (an index or a refusal). Every input to that moves only
inside a frame — the session's `pump`, the fit's `poll`, the cache's
`pump` and `sync` — so read after a frame it holds until something
submits, and only an op or a new δ does; the hover is neither. The
"answered" half is what `progress` alone misses: on the frame an
evaluation lands and before `sync` asks, nothing is in flight and the
label is off, and the next frame's submit is the flake.

Reproduced deterministically:
`a_hover_diff_waits_for_the_index_build_it_would_count` hands the app a
fresh `PickCache` over a seam that holds its answer behind a gate,
between a quiet frame and the hovered one. The diff is exactly
`["indexing…"]`, and `settled()` reads false both before the first
submit and while the build is held.
