---
id: geom-brep-has-more-than-one-decide-wrapper
kind: issue
title: "geom-brep: three decide funnels where its docs promise one per crate (dihedral, enters, props/quad)"
status: closed
opened: 2026-10-01
priority: P4
cost: E
closed: 2026-10-26
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

## Closed (2026-10-26, tracker sweep)

Fixed, by the doc route. `enters::decide` is gone: `enters.rs` now decides
through `dihedral`'s wrappers (`crate::dihedral::decide_reported`,
`decide_positive`, `decide`, at `crates/geom-brep/src/enters.rs:336`,
`:463`, `:517` and beside them). Many other modules, though, call
`geom_core::k_stats::decide` directly (`props/quad.rs:764`'s
`classify_len`, `curve_pair.rs:279`, `edge_nurbs.rs:1025`,
`pcurve_cache.rs`, `ssi/section.rs`, `quartic.rs` and more). So routing
every one of them through `dihedral` is no longer a cheap change, and
the false sentence was the defect. The tracker sweep restated
`dihedral::decide`'s doc (`crates/geom-brep/src/dihedral.rs:197-204`):
it is now the dihedral and material-entry classifiers' funnel, and the
doc says other modules call the recorder funnel directly. No other
"one per crate" claim remains in `crates/geom-brep/src`. The topo half
is still `work/restfront/topo-calls-k-stats-past-its-own-classification-funnel.md`.

