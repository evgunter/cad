---
id: an-overtaken-pin-landing-is-never-pruned
kind: issue
title: viewer: a pin's landing a value-gesture preview overtakes is never pruned, so the frame goes silently mid-drag
status: closed
opened: 2026-10-18
priority: P2
cost: E
closed: 2026-10-18
---


Found by PR 5296's post-merge review, minor 1, demonstrated by execution. The
orchestrator gave it an item of its own, because it is a behaviour bug on the
landing-prune path.

`DocSession::land` ran its second prune only on a landing of the committed
document. Take this sequence: commit a frame on a loose copy, undo (which pins
the copy again) without pumping, then preview a value gesture and pump. The
undo's own landing is stale by then, and the scratch's landing skipped the
prune. Mid-drag the probe treatment left the picture with nothing said, and
the `InTheWorld` notice arrived only after the cancel, worded as if the cancel
had caused it.

## Built

A scratch landing now prunes with `DisplayState::prune_structural`. It acts on
what the node graph decides (a pin, a join, a split, a node gone) and not on a
placement the previewed values refuse. The scratch shares the committed graph,
so the overtaken pin takes effect mid-drag, while a preview the user may still
cancel discards nothing committed over a value.

`space_check` and `gesture_check` now share one mapping from a space answer to
a fault (`in_space`). The overlap argument at `permitted_during_value_gesture`
says what a scratch landing prunes.

The row is `a_pin_landing_a_value_gesture_overtakes_still_prunes` in
`crates/viewer/tests/assembly_display.rs`. It was red before the fix: the
landing withdrew nothing.

Still without a row: the structural half's other side, that a value-refused
placement keeps its frame at a scratch landing. It is listed in
`free-move-landing-prune-gaps-have-no-row`.
