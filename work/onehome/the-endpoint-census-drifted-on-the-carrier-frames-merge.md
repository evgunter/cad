---
id: the-endpoint-census-drifted-on-the-carrier-frames-merge
kind: issue
title: The certified endpoint census drifted on main: four files read more than their rows pin
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

Both arrived with the carrier-frames merges into main (`c67c03e20`,
`b3520c8dd`): `speed.rs`'s determinant read without `Real` and
`recognize_curve`'s reversed-circle row through `Frame::reversed`.

- **Again at main `e44e34003`** (2026-11-22): `crates/step-import/src/on_built.rs`
  reads 24 endpoints, none asking, and has no row. It arrived with the
  adopt-by-description merge (`a5ab84e64`; the file is `651aed96e` to
  `1259b698b`'s). The two rows above were still unpinned on that main.
- **Again at main `a3d5ce89c`** (2026-11-22):
  `crates/geom-brep/src/pair_section/ruling.rs` reads 9, all asking,
  against its pinned 3 / 3: the cone coverage walk's six (`96b964110`).

The attribution, per file, read off each file's history:

- `speed.rs`: `pole_azimuth_speed`'s three, from `bfc94c915` (two,
  rewritten in `fa3cd6593`) and `0593c7cfb` (the determinant's sign);
- `recognize_curve.rs`: five present since `603e36843`, never given a
  row;
- `on_built.rs`: a new file; 8 reads at `4e5a0eb32`, 24 by `1259b698b`;
- `ruling.rs`: six, `96b964110`.

**What was done:** each row re-counted with its dispositions carried, and
each new read argued safe at its site in its row's note (PR
`embed/census-main-drift` to main; the lane `embed/lune` first carried
the count on its own branch). **What remains:** the merge gate that let a
red census land on main, four times, looked at; and `speed.rs`'s older 34
reads, which stay `work/tesslane/the-chart-speed-bounds-endpoint-reads-are-undispositioned`.

## Again at main `a85cfefaa` (2026-11-23, `embed/census-main-drift-2`)

`crates/topo/src/props.rs` reads 11 against its pinned 7, the 3 that ask
unchanged. The 4 new reads are `closed_form_reads`' `ends` closure
(`crates/topo/src/props.rs:3723`), a face's flux and area as `(lo, hi)`
pairs, from `c4240cc65` in PR 7344 (`boxfinish/volume-premises`, merge
`b5b3fe7da`). The function is test vocabulary, built only under `test` or
`test-support`. Its rows (`crates/sweep/tests/volume_premises_*.rs`)
compare the walk's lifted sums with the closed form run again, bit for
bit, or match the refusal it returns, so no branch is taken on an end. The
row is re-counted to 11 / 3 with that disposition added to its note. Every
other file the census walks matched its pin on this tree.

- 2026-11-24 (sectcone, PR 7637): topo's `section_cert/ruling.rs` is deleted, and its rows in the gate tables with it; geom-brep's `pair_section/ruling.rs` is unchanged by that PR.
