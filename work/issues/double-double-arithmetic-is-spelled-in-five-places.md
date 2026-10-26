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

## One instance done (2026-10-26, easy-fixes lane)

`crates/geom-brep/src/ssi/compensated.rs`, a sixth copy the list above
missed, now imports `geom_core::exact::{two_product as two_prod, two_sum}`.
Its private `two_sum` and `two_prod` were the same algorithms, Knuth's
six-operation sum and the FMA product, so its bits do not move.
`fast_two_sum` and its `Dd` stay: `geom_core::exact` has neither.

Re-sweep on main d621f67e with
`grep -rnE 'fn (fast_|quick_)?two_(sum|prod|product)|struct Dd\b'` over
`crates tools demos benches`, plus `mul_add\(\s*\w+\s*,\s*-` for an
inline product:
- `enclose.rs`'s `two_sum` is gone (its row's fix);
- still open: `profile/src/dd.rs`, `test-utils/src/cone_cut.rs`,
  `topo/src/boolean/conic_oracle/exact.rs`,
  `sweep/examples/conic_past_band_probe.rs`;
- new, not listed above: `crates/sweep/tests/near_circular_sections.rs`
  (`Dd`, `two_sum`);
- inline closures: `topo/src/boolean/boxes.rs`
  `the_aimed_frames_rounding_is_a_few_ulps_of_the_point` (`two_sum`
  and an FMA product), and `topo/tests/review_m3_pr2.rs` (`two_prod`
  beside an imported `geom_core::exact::two_sum`).

The `mul_add` pattern cannot see a Dekker split product (no FMA). A
grep for the split constant (`134217729`, `Veltkamp`) finds none.
Neither pattern sees a sum spelled inline without a binding named
`two_sum`.
