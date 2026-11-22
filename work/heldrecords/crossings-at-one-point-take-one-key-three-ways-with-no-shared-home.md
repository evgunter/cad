---
id: crossings-at-one-point-take-one-key-three-ways-with-no-shared-home
kind: issue
title: Crossings at one point share one key three ways, keyed three ways, with no shared home
status: open
opened: 2026-11-22
priority: P1
cost: M
---

Found by the review of PR 7421 (`heldrecords/split-keys`). The fact is
`crates/topo/README.md` tier 3′: "vertices an op knows or decides to be one
point share one point key". Three doors state it, each keyed its own way,
and no function or page is home to it.

## What

- **The Boolean's output stage.** `boolean/zip.rs` `share_points` keys by
  class: the seam correspondence's classes (`point_classes`) and the
  result's vertex-vertex record classes (`boolean::ops::record_classes`).
  It moves each class onto the first-written side's smallest key
  (`ops::PointOwners`). A mint that lands on another vertex's bits gets
  into a class only by its `VertexVertex` record. That is how
  `boolean/reduce.rs` `split_logged` works: its `at` is `Some(p)` from the
  `FaceContainment::OnVertex` pierce and from `split_other_at_point`, it
  mints a fresh key, and `push_vv` records the pair.
- **The Boolean's slit lane.** `boolean/slit.rs` (the crossing walk over
  `crossed_edges`, then `ordered`) calls `split_edge_onto` with
  `ordered[k].point` on the crossed loop edge and again on the slit edge.
  Each call mints its own key on the same bits, and the two vertices are
  `joined`. **Unchecked:** whether those copies, and the slit lane's
  `mev_null` copies, reach `share_points` through a record, or stay on two
  keys at one point.
- **The split.** `splitting/classify.rs` `insert_crossings` (PR 7421) keys
  by structure: two line edges on one pair of end point keys. It lands the
  later crossing with `split_edge_onto` on the earlier crossing's point,
  then calls `Body::share_point` onto its key, and writes no record.

`Body::share_point`'s doc now names both callers and the premise each
holds.

## Direction

Give the fact one home: a mint that lands on a point the op already holds
lands on its key (`split_edge_onto` taking a `PointKey`, say). Then each
door's own question is only which point it knows is the same, by
structure, by a record, or by a decided Zero. `split_logged`'s and
`slit.rs`'s callers would stop minting a key they later fuse away.
