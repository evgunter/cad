---
id: a-joined-finisher-mints-the-whole-body-twice
kind: issue
title: The merge door and the boolean's graft and single-operand paths mint the whole body twice, and the merge door's join clones a staged clone
status: open
opened: 2026-10-08
---


## The finding

A cost, not a defect: every result is right, and some are paid for
twice.

- **The merge door** (`crates/topo/src/merge_faces.rs:1856`) stages the
  body on a clone and re-mints it whole (`merge_faces.rs:1982`), then
  ends with `Body::join_edges` on that clone (`merge_faces.rs:1988`).
  The join door clones the clone again
  (`crates/topo/src/boolean/edge_join.rs:595`) and, where it joined
  anything on a body that carries rows, re-mints it whole a second time
  (`edge_join.rs:601`).
- **The boolean's graft and single-operand paths**
  (`crates/topo/src/boolean/ops.rs:4941`, `ops.rs:5003`) run the
  output stage, whose join may re-mint the whole body, and then
  re-mint it whole themselves (`ops.rs:4942`, `ops.rs:5004`). The
  seamed path does the same at `ops.rs:719` and `ops.rs:739`.

## What it needs

A join that runs on a body its caller has already staged, re-minting
only the faces its kills touched: the offset doors' scoped join
(`Body::join_edges_within`) is that shape. The merge door and the
boolean's paths could take it with the whole body as the scope, and
the second whole mint would then be the caller's one closing mint.
