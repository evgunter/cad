---
id: bit-identity-doc-says-no-body-instantiates-dual
kind: issue
title: geom-core's bit_identity.rs says Dual is a scalar no Body instantiates, but Body<Dual64> exists in tests and Dual has an AtRestPolicy impl
status: open
opened: 2026-10-03
---


(Found by the TOPO kef/kfmrh design fork, round 2, while checking
whether payload identity is expressible per scalar.)

`crates/geom-core/src/bit_identity.rs` describes `Dual` as a scalar
"which no `Body` instantiates". That is stale:
- `Body::<Dual64>` is built in `crates/topo/src/tier3_tests.rs` (~:2380),
  in `crates/topo/tests/mate9_crossing_rung.rs`, and in sweep and verbs
  tests;
- `Dual` has an `AtRestPolicy` impl (`crates/topo/src/props.rs`, ~:3281).

Correct the sentence to say what holds. Any reasoning in that file that
rests on no `Body<Dual>` existing needs re-checking.
