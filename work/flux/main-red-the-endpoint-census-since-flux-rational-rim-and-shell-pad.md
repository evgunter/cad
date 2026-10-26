---
id: main-red-the-endpoint-census-since-flux-rational-rim-and-shell-pad
kind: issue
title: Main red: certified_endpoint_census moved at every ε for spiric.rs, rational_rim.rs and topo props.rs, from FLUX's PRs 5696 and 5699, beside the quad.rs move its open row names
status: open
opened: 2026-10-26
priority: P1
cost: E
refs: [the-endpoint-census-miscounts-quad-rs-after-the-spout-bracket, 5696, 5699]
---


Filed by CLEAVE's nightly tracker lane from nightly 72 (run
40274924857, job "full suite (every eps row, slow set included)", main
`38f1a24ed3`).

## What

`geom-core` `certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned`
(`crates/geom-core/tests/certified_endpoint_census.rs`, the `assert_eq!`
at `:754`) is red at the default ε, 1e-6 and 1e-12:

    the certification endpoint-read census moved. This is not a number to silence. …

Its open row,
`work/quad/the-endpoint-census-miscounts-quad-rs-after-the-spout-bracket.md`,
names only `props/quad.rs` (now `(17, 7)` against the pinned `(7, 2)`,
from QUAD's #5611 and ISO's #5634). Tonight three more entries differ
from the pinned table, and none of them is that row's:

| file | measured (reads, asked by name) | pinned |
|---|---|---|
| `crates/geom-brep/src/props/spiric.rs` | `(12, 10)` | `(14, 13)` |
| `crates/geom-brep/src/props/rational_rim.rs` | `(1, 1)` | no entry |
| `crates/topo/src/props.rs` | `(1, 0)` | no entry |

The 10-25 nightly (run 40165703340, main `9f5b44561d`) read
`spiric.rs` at the pinned `(14, 13)` and had no entry for the other
two, so these moved between `9f5b44561d` and `38f1a24ed3`.

## Cause (from the diffs; not bisected by a run)

- **#5696** (`flux/rational-rim-vector-area`, merge `c05c7b4f47`) adds
  `props/rational_rim.rs` (its one production read is `exact`'s,
  which asks `is_certified()` first) and rewrites 263 lines of
  `props/spiric.rs`. `spiric.rs` loses two reads and three
  dispositioned reads, so the file note's "the remaining 1 is
  `trig_range`'s half-width" needs re-checking against the file as
  it is now.
- **#5699** (`flux/shell-pad-per-region`, merge `eb9dc90309`) adds
  `far_from` in `crates/topo/src/props.rs` (`:4269`), whose
  `p.distance_squared(c).lo()` / `….hi()` line (`:4276`) is a
  one-sided read in a function that does not ask `is_certified()` by
  name. It reads NaN as "near", so it may be safe at the site. That
  argument is the census's to receive.

## Owed

The census's own remedy: disposition each read. Either it asks
`is_certified()` by name or its site argues it safe and the file's
note says why. Then move the three table entries with their notes. This
lands beside the quad row's re-pin, and either order works. Both must
land before the row goes green. Do not raise the pins blindly.

Reproduce (it reads source, so any ε): `cargo nextest run -p geom-core
--profile ci --ignore-default-filter -E
'test(=certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned)'`.
