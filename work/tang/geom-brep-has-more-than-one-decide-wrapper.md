---
id: geom-brep-has-more-than-one-decide-wrapper
kind: issue
title: "geom-brep: three decide funnels where its docs promise one per crate (dihedral, enters, props/quad)"
status: open
opened: 2026-10-01
priority: P4
cost: E
---

## What

`dihedral::decide` (`crates/geom-brep/src/dihedral.rs`) calls itself
"the one classification funnel of this crate … so this remains the
crate's single greppable decision site". It is not the only one.
`geom_core::k_stats::decide` is also wrapped by:

- `enters::decide` (`crates/geom-brep/src/enters.rs`), private to that
  file, next to its `decide_arm`. Its own doc calls it "the crate-local
  funnel wrapper (the `geom-brep` pattern: one greppable `sign_within`
  door per crate …)";
- `props/quad.rs`'s `classify_len`, which calls
  `geom_core::k_stats::decide` directly.

All three reach the same `k_stats` funnel, so telemetry is unaffected.
What is false is the "one per crate" sentence, and a grep for the
crate's decision sites that trusts it misses two of them. `topo` has the
same shape (`transform.rs` calls `k_stats::decide` directly beside
`validate::decide`); that half is
`work/restfront/topo-calls-k-stats-past-its-own-classification-funnel.md`.

## Close it

Either route `enters.rs` and `props/quad.rs` through `dihedral`'s
wrappers (`classify_len` keeps its lift and its error mapping around the
call), or restate the doc sentences to name every wrapper. The first
keeps the promise.

Found by PR 3747's style review (S6): `geom-brep/src/locus.rs` had
repeated the "one decision door" claim.
