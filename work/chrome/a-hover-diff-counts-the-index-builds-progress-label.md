---
id: a-hover-diff-counts-the-index-builds-progress-label
kind: issue
title: app's whole-app hover diff counts the toolbar's indexing label when the index build changes state between its two frames
status: open
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
