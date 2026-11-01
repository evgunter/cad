---
id: the-cancel-buttons-are-always-on-the-toolbar
kind: issue
title: Cancel drag and Cancel free-move are on the toolbar when nothing is in flight
status: closed
opened: 2026-10-31
priority: P2
cost: E
branch: chrome/forms-at-rest
rides_with: the-chrome-shows-every-form-at-once
closed: 2026-11-01
pr: 6223
---


## Finding

Found driving the viewer (2026-10-31; Ev: P2). **Cancel drag** and **Cancel free-move** are on the toolbar from startup, with nothing being dragged and, in a document with no instance, nothing that can be free-moved. Ev asked how to free-move on seeing the button; the only way in is the probe in an instance's Properties. A cancel belongs where its gesture is in flight.

## Closed (2026-11-01, PR 6223)

Cancel drag and Cancel free-move are on the toolbar only while their gesture is in flight. Escape still cancels. The README's "a door that cannot act says so rather than vanishing" no longer covers these two. That clause was agent text (f3fae31ba2), and this row is Ev's; the posture stands for Undo/Redo, Open/Save and the self-instance entry.
