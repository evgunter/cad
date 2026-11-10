---
id: the-placement-editor-draws-no-preview-of-its-draft
kind: issue
title: The placement editor draws no held draft; the profile editor draws its held draft in place of the committed shape
status: open
opened: 2026-11-02
---


Found by `placing-bundle-editor` (the unit `mate-tool-commits-one-mate`), a known residue of its build.

The decision says "The same scratch evaluation draws the copy where the draft stands it." The editor's verdict (`DocSession::bundle_verdict`, `crates/viewer/src/session.rs`) runs the draft's Apply on a scratch `Recording` and evaluates it, and answers `BundleVerdict::Stands(frame)` with the copy's relative pose. The panel (`mate_tool_ui`, `crates/viewer/src/pane/create.rs`) says only that the copy stands; nothing draws the copy at `frame`.

No display door draws one node of the committed picture at another pose: the free-move probe (`DisplayView::preview_free_move`) moves a whole space's display frame, and the value gesture (`SessionOp::PreviewGesture`) previews one slot's number, not a bundle.

**Owed.** A preview door that draws the draft's copy where the scratch evaluation stands it, in place of where it stands now (the profile editor's `edited_in_place` is the model), taken from the same verdict the panel reads, so the picture and the sentence cannot disagree.

**Done when** a row opens the editor on a copy, adds a rest, and reads the drawn copy at the verdict's pose before Apply, and at its committed pose after Cancel.

## Narrowed (PR 6578)

A change the door takes is committed, and its own evaluation is its preview (README, *A create gesture commits the node*), so the "before Apply" case is gone. **What remains is parity with the profile editor:** it draws its draft, held ones included, in place of the committed loops (`Drafts::door_loops`, `Drafts::edited_in_place`, `sketch::committed`'s `except`), so a pending profile shows what it would be. The placement editor draws nothing for a held draft, and the copy stays where the document stands it.

**Owed.** Where a held draft's bundle solves (a draft held for an empty seat or a door refusal does not; one waiting while the picture evaluates does), draw the copy where the scratch solve stands it, in place of its committed pose; where it does not solve, draw the committed copy and the panel's `PENDING` line, as the profile editor draws a refused prefix. **Done when** a row holds a draft whose bundle solves and reads the drawn copy at the scratch pose, then Discards and reads it at the committed pose.
