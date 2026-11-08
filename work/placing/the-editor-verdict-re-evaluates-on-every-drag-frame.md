---
id: the-editor-verdict-re-evaluates-on-every-drag-frame
kind: issue
title: The placement editor derives its draft's bundle every frame and solves each change twice
status: open
opened: 2026-11-02
priority: P3
cost: E
---


Found by PR 6300's dual review (NOTE); filed by `placing-bundle-editor` in the fix pass.

The panel memoises `DocSession::bundle_verdict` by history state and op (`drafts::MateVerdict`). A value field dragged or typed into changes the op on every frame, and each change runs a scratch recording and a whole-document evaluation. `MateTool::proposal` also runs on every frame (`draft_snap`'s scratch `SetConstraints`).

**Owed.** Ask the verdict once the draft settles (on focus loss, as the field already commits), or debounce it, so a drag costs no evaluation per frame. **Done when** a row drags a value field for N frames and counts at most one verdict evaluation.

## Narrowed (PR 6578)

The verdict memo is gone: the live door (`DocSession::bundle_door`) runs once per change offered, and value fields commit on focus loss, so a drag is not a change per frame. **What remains:**
- `MateTool::proposal`, with `draft_snap`'s scratch `Recording`, runs every frame the panel is drawn (`mate_tool_ui`), to draw the value rows.
- Each change pays two whole-document solves: one at its offer (`bundle_door`) and one again where `SetBundle` lands (`bundle_judged` in the `SetBundle` arm of `DocSession::perform`).

**Owed.** Derive the proposal once per draft state, not once per frame, and land a change the door has just judged without solving it again (the op could carry the verdict's document, or the door's answer be memoised by history state and op). **Done when** a row draws the panel for N frames over one draft and counts one proposal, and a change counts one solve.
