---
id: an-edited-profile-shows-no-ghost-of-its-committed-shape
kind: issue
title: viewer: a profile opened for editing hides its committed shape with no faint ghost behind the edit, so the author loses the reference
status: open
opened: 2026-09-30
priority: P2
cost: M
refs: [3440]
---

Ev's ruling on PR 3440 (AUTH-5, 2026-09-30): *hide the committed shape during a refused edit, make it consistent, and file a faint "ghost" of the committed shape as a follow-up.* This is that follow-up.

## What happens

While the edit door holds a committed profile, the viewport leaves that profile out of the committed lane whenever the door took a preview (`Drafts::edited_in_place` in `crates/viewer/src/drafts.rs`, read by `push_committed` in `crates/viewer/src/pane/viewport.rs`). What is painted in its place is only what the edit's preview draws: the whole loop, a refused step's prefix, or, for a step refused with nothing before it, nothing at all. An author who types a number that breaks the path therefore loses sight of the shape they started from, which is the reference they need to judge what the number should be.

## What is owed

Draw a faint copy of the committed shape behind a refused or unfinished edit, so the reference stays in view without reading as a second profile. The committed loops are already in hand (`sketch::committed` with no `except`); what is missing is a place to draw them that reads as neither the committed-profile lane nor the preview lane:

- a new `marks::EdgeLane` (and its `EdgeOverlay` field) placed in `EdgeLane::DRAW_ORDER` below the preview;
- its colour in `theme.rs`, faint against every palette, and its parity with the shader in `gpu.rs` (`the-shader-encodes-a-mark-strength-nothing-bounds` is the parity row on that ground);
- the viewport filling it from the node `Drafts::edited_in_place` answers.

Open for the lane to decide and say: whether the ghost also shows behind a whole, valid edit (Ev's words name refused and unfinished ones), and whether it is drawn at all while nothing replaces it (a refusal at step 1).

Row that holds the current behaviour: `pane::viewport::tests::a_refused_edit_hides_the_committed_profile_with_or_without_a_prefix`. A ghost lane leaves that row's assertions about the committed and preview lanes true; it adds a lane beside them.
