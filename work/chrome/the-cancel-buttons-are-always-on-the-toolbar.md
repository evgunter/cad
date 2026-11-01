---
id: the-cancel-buttons-are-always-on-the-toolbar
kind: issue
title: Cancel drag and Cancel free-move are on the toolbar when nothing is in flight
status: open
opened: 2026-10-31
priority: P2
cost: E
---


## Finding

Found driving the viewer (2026-10-31; Ev: P2). **Cancel drag** and **Cancel free-move** are on the toolbar from startup, with nothing being dragged and, in a document with no instance, nothing that can be free-moved. Ev asked how to free-move on seeing the button; the only way in is the probe in an instance's Properties. A cancel belongs where its gesture is in flight.
