---
id: ring-path-torus-sections-fuzz-reads-a-wrong-parity-at-1e-12
kind: issue
title: ring_path: random_torus_sections decides a wrong parity at eps 1e-12 (seed 0xfcf36c24b2674116)
status: open
opened: 2026-10-30
---


Found by the ε sweep of `splitdoor/ring-path-faults`.

`crates/topo/src/ring_path/spline_fuzz.rs` `random_torus_sections`, at
`CAD_TOLERANCE_EPS=1e-12`, finds a path whose decided parity
(`path_parity` returns `Ok(Some(got))`) disagrees with the oracle's
`ia != ib`: the assertion beside `decided += 1`.

- Replay: `CAD_TOLERANCE_EPS=1e-12 CAD_FUZZ_SEED=0xfcf36c24b2674116
  CAD_FUZZ_EFFORT=1 cargo nextest run -p topo -E
  'test(=ring_path::spline_fuzz::random_torus_sections)'` (about 200 s).
- The failing case: partner `Ball { centre: (-0.5397891774416801,
  -0.11337425021305902, 0.24144415153948218), radius: 0.44379298000701617 }`,
  path on the partner from `(-0.4316821610610483, -0.5390314599102256,
  0.1775612685078327)` to `(-0.3659144262943794, -0.519264341531642,
  0.1970255317234141)`.
- The branch's change cannot cause it: it only turns two `Ok(None)`
  endings of `spline_crossings` into `unreachable!`, so no decided
  answer moves. At 1e-9 and 1e-6 the same filter passed.

A wrong decided parity is a wrong answer, not a refusal. Pin the case as
a deterministic row beside the fix.
