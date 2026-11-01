---
id: a-body-two-cut-placements-copy-reads-as-severed
kind: issue
title: A split's crossing check reads a body two cut placements copy as severed, not as two bodies
status: open
opened: 2026-10-18
priority: P3
cost: E
---

In `crates/editor-core/src/refactor.rs`, `home` (around lines 2852–2912) answers `None` both when no cut world placement copies a read body and when two do. The selection branch of the crossing check (around 3013) and the pose-leaf `carries` (around 2991) then see a crossing that is not carried, and the closure loop raises the generic `SeveredEdge` (around 3064). A body read in the same position gets `RemainderReadUncarried { why: Uncarried::Bodies { count: 2 } }`, because `heir_of` keeps the count. That variant's doc ("no one copy of the part's world stands where what it read did") fits both. Found by the INTENT stage 3 D lanes; the logic is identical on main.

**Done when** `home` returns `Result<Option<_>, Uncarried>`: `[p]` → `Ok(Some((p, true)))`, `[]` → `Ok(None)`, and two or more → `Err(Uncarried::Bodies { count })`. The selection branch and `carries` then refuse `SplitError::RemainderReadUncarried { reader, why }`, with a row reading a face of a body the cut places twice.
