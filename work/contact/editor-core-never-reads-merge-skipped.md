---
id: editor-core-never-reads-merge-skipped
kind: issue
title: No code outside topo reads BooleanNaming::merge_skipped, so a document union whose merge stage skipped a group publishes the body with no trace
status: open
opened: 2026-09-28
priority: P3
cost: E
---


Found by the area-overlap fork's designers (PR #3350). If Ev takes
that PR's second decision, planar skips become refusals and only curved
skips remain in `merge_skipped`. Those are still published silently. A
document union should say so. Check the consumers in `editor-core`
first; this was reported, not traced.
