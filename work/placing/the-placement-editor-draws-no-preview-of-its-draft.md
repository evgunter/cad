---
id: the-placement-editor-draws-no-preview-of-its-draft
kind: issue
title: The placement editor judges its draft before Apply but the viewport does not draw the copy where the draft stands it
status: open
opened: 2026-11-02
---


Found by `placing-bundle-editor` (the unit `mate-tool-commits-one-mate`), a known residue of its build.

The decision says "The same scratch evaluation draws the copy where the draft stands it." The editor's verdict (`DocSession::bundle_verdict`, `crates/viewer/src/session.rs`) runs the draft's Apply on a scratch `Recording` and evaluates it, and answers `BundleVerdict::Stands(frame)` with the copy's relative pose. The panel (`mate_tool_ui`, `crates/viewer/src/pane/create.rs`) says only that the copy stands; nothing draws the copy at `frame`.

No display door draws one node of the committed picture at another pose: the free-move probe (`DisplayView::preview_free_move`) moves a whole space's display frame, and the value gesture (`SessionOp::PreviewGesture`) previews one slot's number, not a bundle.

**Owed.** A preview door that draws the draft's copy where the scratch evaluation stands it, in place of where it stands now (the profile editor's `edited_in_place` is the model), taken from the same verdict the panel reads, so the picture and the sentence cannot disagree.

**Done when** a row opens the editor on a copy, adds a rest, and reads the drawn copy at the verdict's pose before Apply, and at its committed pose after Cancel.
