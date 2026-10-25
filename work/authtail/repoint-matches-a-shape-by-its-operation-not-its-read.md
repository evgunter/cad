---
id: repoint-matches-a-shape-by-its-operation-not-its-read
kind: issue
title: repoint re-points every shape of a placement whose operation the result reads, not only the read it took over: one half of a two-half Place takes the other over too
status: open
opened: 2026-10-25
priority: P1
cost: E
---


Found by the third review of PR 5683 (sure, probed). `repoint` (`crates/viewer/src/session.rs`) matches a placement's shape by `read.contains(&doc.operation_of(var))`, that is, by operation and not by the read it took over. Build one `Place` drawing `[split.Above, split.Below]` and mirror the lower half: both shapes now read the union. The upper half disappears and the part is drawn twice. `feature_over`, `sketch_on_face` and `combine_in_world` share `repoint` and the same fault. It shows only on a multi-shape `Place`, which D′3 retires (one source per placement), so the fix is to compare the var, plus a row on the two-half `Place`. If D′3 lands first, close this as absorbed.
