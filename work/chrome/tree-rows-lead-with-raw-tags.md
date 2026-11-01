---
id: tree-rows-lead-with-raw-tags
kind: issue
title: tree rows lead with a raw tag and repeat the kind
status: open
opened: 2026-10-31
priority: P2
cost: E
---


## Finding

Found driving the viewer (2026-10-31; Ev: P2). Tree rows read `Profile 1 Profile bf935d4a4c62`, `Extrude 2 Extrude 5c8b603262ca »`, `Place 8e8a553cb250`: the label, then the kind repeated, then a 12-hex tag; an unlabelled node is its kind and tag alone. The startup `Place`'s six value rows read `constraint 1: slide along x value`.
