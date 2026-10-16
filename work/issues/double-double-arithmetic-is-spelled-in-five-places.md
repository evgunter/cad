---
id: double-double-arithmetic-is-spelled-in-five-places
kind: issue
title: Double-double arithmetic (two_sum, two_prod, a Dd type) is spelled privately in five places beside geom_core::exact::two_sum
status: open
opened: 2026-10-14
priority: P3
cost: E
refs: [ssi-enclose-carries-a-private-two-sum-beside-geom-core-exact]
---


## What

Error-free transformations and a double-double type are written out
privately at each site that needs an extended-precision oracle, beside
the public `geom_core::exact::two_sum`:

- `crates/profile/src/dd.rs` (`Dd`, `two_sum`, `quick_two_sum`,
  `two_prod`; test-only), the oracle of `seg.rs`
  `circle_pair_points_sit_within_a_few_ulps_of_scale`;
- `crates/test-utils/src/cone_cut.rs` (`Dd`, `two_sum`);
- `crates/topo/src/boolean/conic_oracle/exact.rs` (`Dd`, `two_sum`);
- `crates/sweep/examples/conic_past_band_probe.rs` (`Dd`, `two_sum`);
- `crates/geom-brep/src/ssi/enclose.rs` `two_sum`, already filed as
  `ssi-enclose-carries-a-private-two-sum-beside-geom-core-exact`.

Each new oracle mints another copy because no shared double-double
type exists. Found at adjudication of PR 5032 (a fix that minted a
fresh instance of the duplication class).

## Fix

One double-double type with its error-free primitives, in
`geom-core::exact` (or `test-utils` for the test-only users), and the
sites above call it. Check first that the copies are the same
algorithms (`two_prod` by FMA or by splitting).

Swept with `git grep 'fn two_sum\|fn two_prod\|struct Dd'`; a copy
under another name (`fast_two_sum`, a tuple return) would escape it.
