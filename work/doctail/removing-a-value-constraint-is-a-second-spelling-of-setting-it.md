---
id: removing-a-value-constraint-is-a-second-spelling-of-setting-it
kind: issue
title: DocEdit::RemoveConstraint on a value writes 0 on the freedom it frees: a second spelling of setting the value, accepted silently
status: open
opened: 2026-11-01
priority: P3
cost: E
---


Found by the PLACING designers weighing `work/placing/mate-tool-commits-one-mate.md` (2026-11-01; both concurred, neither built it).

`DocEdit::RemoveConstraint` (`crates/editor-core/src/edit.rs`) accepts a value constraint (`slide value`, `spin value`) and rewrites the bundle without it, writing the freed freedom as `0`, because every bundle pins its copy. On a value that is already `0`, the startup `Place`'s six rows for instance, it records an edit and changes nothing. Where the value is not `0`, it means "set it to 0", which editing the number already says. Today the viewer's tree reaches it with a `remove` button on every value row (`work/placing/mate-tool-commits-one-mate.md`, "Also found"). Once PLACING's bundle editor lands, only API callers reach it.

**Owed.** The door refuses removing a value, typed, with "set its number" as the recourse. Removing a mate stays as it is.

Signed: (PLACING orchestrator)
