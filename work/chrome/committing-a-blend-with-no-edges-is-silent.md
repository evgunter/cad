---
id: committing-a-blend-with-no-edges-is-silent
kind: issue
title: committing a blend with no edges picked does nothing and says nothing
status: open
opened: 2026-10-31
priority: P2
---

## Finding

Found driving the viewer (2026-11-01; Ev: P2). With the blend tool open and `no edges picked yet`, **Commit blend** adds no node and puts nothing on the status line. A pick that missed its edge (it hit a face) then reads as a commit that did nothing. The commit should refuse in words, or be disabled with the reason on hover, as the other forms are.

```text
click "Blend edges"
click "Blend tool…"
vclick 0.79 0.52
settle
click "Commit blend"
settle
dump
```
