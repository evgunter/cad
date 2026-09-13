---
id: eager-sweep-messages-on-the-effort-dial
kind: issue
title: sweep messages built eagerly once per iteration, so the EFFORT dial buys formatting nobody reads
status: open
opened: 2026-09-12
---


A randomized sweep's failure message is wanted on the iteration that
fails and on no other. Built as an argument to `assert!` or `panic!` it
costs nothing on a pass — the macro evaluates its format arguments only
when the condition is false. Built eagerly, into a `String` the assert
then interpolates, it is paid on **every** iteration, and the bill rides
`CAD_FUZZ_EFFORT` along with the sweep: the whole point of the dial is
that `CAD_FUZZ_EFFORT=100` is affordable to reach for, and formatting is
not what anyone is buying with it.

`fuzz::replay()` makes this expensive out of proportion to its size — it
is itself a `format!` over two values, so an eager site is two
allocations and two integer formats per iteration rather than one.

**Fixed in PR #2433** (`tcost/r1-seeds-on-the-harness`), named here so
the class has a home:

- `crates/viewer/tests/review_gui0_r1.rs`
  `the_camera_contract_survives_random_operation_walks` — three
  `&format!(…)` arguments to `assert_camera_contract`, one per step of a
  `scaled(48) x 16` walk: 816 eager `format!`s per run at EFFORT=1, ~81 600
  at 100. `assert_camera_contract` now takes `impl Fn() -> String`.
- `crates/editor-core/tests/r1_m10_1_probes.rs`
  `sampled_masses_match_an_independent_integration_oracle` — one `ctx`
  string per round of `scaled(40)`, interpolated by four asserts. Now a
  closure.

**Open, and the reason this file exists** —
`crates/geom-core/tests/d8_knot_queries_adversarial.rs`, the drawn-vector
loop of `d8-adversarial-mutation-sequence`:

```rust
simulate_decomposition(&format!("drawn#{i} [{}]", fuzz::replay()), &kv, &extra);
```

once per `fuzz::scaled(120)` iteration, and `simulate_decomposition` reads
`name` only to interpolate it into its own assertion messages. Not fixed
in #2433: it is another program's crate and no measurement here sizes it.

**What the sweep that found these could not match.** The pattern was
`fuzz::replay()` with `format!` / `println!` / `write!` in the six lines
above it, over `crates/`, `demos/`, `tools/`, `benches/` and
`interval-transcendentals/` (289 `replay()` sites, 20 with a nearby
eager-looking macro, each read by hand). It cannot see:

1. an eager message that does NOT mention `fuzz::replay()` — a
   `format!("case {i}")` context string in a sweep loop is the same defect
   and matches nothing here;
2. an eager message more than six lines from its `replay()`;
3. a cost that is not `format!` — a `.to_vec()`, a `clone()` or a
   `{:?}` of a body built per iteration purely to be quotable;
4. sweeps outside the `test_utils::fuzz` harness entirely (the
   fixed-literal-seed rows, which have their own row).

Checked and NOT this defect: the `census.require(…, &format!(…))` /
`seen.require(…)` anti-vacuity floors in `crates/bvh/tests/ray_r2.rs`,
`crates/sweep/tests/review_d2_adv_probes.rs`,
`crates/topo/src/review_d18.rs` and
`crates/editor-core/tests/gui1_pick_r2.rs` — every one of those sits
AFTER its loop and runs once per run. `crates/profile/tests/review_s2.rs`
already passes its context as `&|| format!(…)`, which is the shape this
issue asks for.
