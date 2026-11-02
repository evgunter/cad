---
id: the-editor-verdict-re-evaluates-on-every-drag-frame
kind: issue
title: The placement editor's verdict re-evaluates the document on every frame of a value drag
status: open
opened: 2026-11-02
---


Found by PR 6300's dual review (NOTE); filed by `placing-bundle-editor` in the fix pass.

The panel memoises `DocSession::bundle_verdict` by history state and op (`drafts::MateVerdict`). A value field dragged or typed into changes the op on every frame, and each change runs a scratch recording and a whole-document evaluation. `MateTool::proposal` also runs on every frame (`draft_snap`'s scratch `SetConstraints`).

**Owed.** Ask the verdict once the draft settles (on focus loss, as the field already commits), or debounce it, so a drag costs no evaluation per frame. **Done when** a row drags a value field for N frames and counts at most one verdict evaluation.
