---
id: main-red-the-sense-fold-census-flags-knot-split-a
kind: issue
title: Main red: face_normal's sense-fold census flags step-import knot_split.rs's a, so no_source_file_folds_the_bit_by_hand fails
status: dispatched
opened: 2026-11-18
priority: P1
cost: E
---

Found by lane `embed/conic-vertex` (EMBED, 2026-11-18) at the PR 7105
reviewer's prompt, on a checkout of main `d0579ebd` with that PR merged.
The PR touches no file this census reads, so the red is main's.

`topo face_normal::tests::no_source_file_folds_the_bit_by_hand`
(`crates/topo/src/face_normal.rs`, the assertion at ~1552) finds one
hand fold the sanctioned list lacks:

```text
left (found) adds: ("crates/step-import/src/knot_split.rs", "a")
right (sanctioned): the other seven entries, unchanged
```

`knot_split.rs` last moved in `87801ff8` ("knot_split fix pass: … pieces
keep sense …"), most likely where `a` is folded under a face's sense
bit. The census is in the slow set and the PR gate does not select it.

**What closes it:** the fold routed through `OutwardNormal::from_chart`
or a `face_normal` door, or, if `a` is a genuinely signed scalar, its
entry added to the sanctioned list saying what it negates.
