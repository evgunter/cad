---
id: gate-maximal-faces-may-miss-coincident-curved-faces-on-different-keys
kind: issue
title: gate_maximal_faces checks planar pairs and same-key curved pairs; coincident adjacent curved faces on different surface keys may pass the F7 gate unseen
status: open
opened: 2026-09-28
priority: P3
cost: M
---


Raised by one of the area-overlap fork's designers (PR #3350) and not
investigated: `gate_maximal_faces` only checks planar pairs and
same-key curved pairs. Measure first: build two adjacent curved faces
on different keys that coincide, and see what the next boolean does.
