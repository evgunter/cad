---
id: a-long-fields-keyboard-edit-does-not-scroll-to-its-cursor
kind: issue
title: a numeric field's keyboard edit over a text longer than its room shows only the text's start and does not scroll to the cursor
status: open
opened: 2026-11-25
priority: P3
cost: M
---


Left by PR 7775 (`widgets::NumberField`): a numeric field is laid out
in a clipped room of `widgets::FIELD_GLYPHS` glyphs so a long positional
spelling no longer widens its pane. While a keyboard edit is open over
a text longer than the room, egui's in-`DragValue` edit is built with
`clip_text(false)` and does not scroll to the cursor, so only the
text's start shows. The edit opens with everything selected, so
ordinary typing replaces it; End followed by more digits types out of
sight. Candidate: open the keyboard edit as a scrolling `TextEdit` in
the same room (as the δ field does).
