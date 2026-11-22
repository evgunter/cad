---
id: the-endpoint-census-drifted-on-the-carrier-frames-merge
kind: issue
title: The certified endpoint census drifted on the carrier-frames merge: speed.rs and recognize_curve.rs read more than their rows pin
status: open
priority: P2
cost: C
opened: 2026-11-21
---


## Finding

From EMBED's lane `embed/lune` (2026-11-21), merging `origin/main` at
`d6985bf9f`'s second parent. On that main the certified endpoint census
(`crates/geom-core/tests/certified_endpoint_census.rs`,
`every_production_endpoint_read_is_counted_and_dispositioned`) is red on
two rows no lane of EMBED touches:

- `crates/geom-brep/src/pcurve_cache/speed.rs` reads 41 endpoints, 4 of
  them asking; its row pins 38 / 4.
- `crates/step-import/src/recognize_curve.rs` reads 5, none asking, and
  has no row (it is listed among the census's holders).
- **Again at main `e44e34003`** (2026-11-22, merged at `embed/lune`'s
  `19dbfe621`): `crates/step-import/src/on_built.rs` reads 24 endpoints,
  none asking, and has no row. It arrived with the adopt-by-description
  merge (`a5ab84e64`; the file is `92f8deba0`'s and `1259b698b`'s). The
  two rows above were still unpinned on that main.

Both arrived with the carrier-frames merges into main (`c67c03e20`,
`b3520c8dd`): `speed.rs`'s determinant read without `Real` and
`recognize_curve`'s reversed-circle row through `Frame::reversed`.

**What the lane did:** re-counted both rows, so its PR's gate (which runs
the census, the lane editing it) reads green: `speed.rs` 41 / 4 with its
disposition unchanged, and `recognize_curve.rs` 5 / 0 as "not yet
dispositioned", pointing here, and `on_built.rs` 24 / 0 the same way.
**What remains:** the three new reads in `speed.rs`, the five in
`recognize_curve.rs` and the 24 in `on_built.rs` dispositioned by the
program that wrote them, and the merge gate that let a red census land
looked at.
