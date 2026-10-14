---
id: a-hover-diff-counts-the-index-builds-progress-label
kind: issue
title: app's whole-app hover diff counts the toolbar's indexing label when the index build changes state between its two frames
status: closed
closed: 2026-09-29
pr: 3487
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
`properties_pane_tests`, `Driven::settle` and `Driven::settled`):
`ViewerApp::progress` (the toolbar's read, now one method the harness
also calls) is `None`, and the pick cache holds an answer for the
picture on screen. Every input to that check changes only inside a
frame, so once it holds after a frame it keeps holding until something
submits, and only an op or a new δ does. The "answered" half catches
what `progress` alone misses: the frame where an evaluation lands
before `sync` has asked for an index.

`painted_with`, the other whole-app comparison in the module (used by
`an_undeclared_parameter_is_said_once_in_the_pane`), now reads
`Driven::quiet` too. The toolbar harness (`toolbar_driven`) never pumps
the session, so its progress state cannot move between frames.

Reproduced deterministically by
`a_hover_diff_waits_for_a_run_that_lands_between_its_frames`. The
startup evaluation is held through the frame `Driven::with` draws and
the two quiet frames, and lands on the first hovered frame; the index
build is held one frame more. With `settle()` removed from `quiet()`,
the row fails with the diff `["indexing…"]`, which is this flake.
