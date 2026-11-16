---
id: a-margin-decided-through-a-bool-helper-or-outside-pcurve-cache-is-unswept
kind: issue
title: Decided-Zero margins beside an envelope were swept only in pcurve_cache and certify; bool-returning helpers and other doors are unswept
status: open
opened: 2026-11-10
priority: P3
cost: M
---

Filed by PR 6801's FULL review (fix JJ6), from that PR's sweep.

## What

PR 6801 found that the arc-rim certificate decided its sub-arc weight
margins Zero in band without paying them into its envelope, and swept
for the shape: a `decide` whose Zero arm admits a quantity that no
envelope sum then carries. The sweep covered two files,
`crates/geom-brep/src/pcurve_cache.rs` and
`crates/geom-brep/src/certify.rs`, with two patterns: `Sign::Zero => {}`
arms, then `if let Sign::Zero`, `matches!` and grouped or working Zero
arms. Its disclosed blind spot is still unswept:

- **a margin decided through a helper that returns a `bool`** (or an
  `Option<bool>`), whose caller then treats `true` as exact. Neither
  pattern matches the decision at the caller;
- **doors outside those two files** that decide a margin Zero and then
  build or certify geometry as though it were exact.

## Where to look first

`topo`'s `chord_join.rs`, `split_conic_inplane_mid` in the run-edge
reading beside `split_conic_inplane` (a conic's mid-point plane offset
decided Zero reads `Some(true)`, "in plane"). Ask whether the offset
the band admits is paid by whatever certificate later stands on that
answer, or only assumed away.

## Done when

Each hit of a sweep shaped at the gap (bool and `Option<bool>` helpers
over `decide`, across `crates/*/src`) is dispositioned: paid, filed, or
not a margin beside a claim, with the hit list recorded here.
