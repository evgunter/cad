---
id: the-boolean-tool-takes-one-body-twice
kind: issue
title: the boolean tool takes the same body as two members
status: open
opened: 2026-10-31
priority: P3
---

## Finding

Found driving the viewer (2026-11-01; Ev: P3). Two viewport clicks on one plate with the boolean tool open give `member 1: Extrude a59299af4a92; member 2: Extrude a59299af4a92`: a union of a body with itself. `BooleanTool::pick_at` (`crates/viewer/src/combine.rs`) pushes every member pick without checking for one already held. A second click on a held member could drop it instead.

```text
vclick 0.3 0.4
click "Combine bodies"
click "Boolean tool…"
vclick 0.3 0.4
vclick 0.3 0.4
settle
dump
```
