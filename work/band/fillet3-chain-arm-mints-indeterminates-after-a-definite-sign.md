---
id: fillet3-chain-arm-mints-indeterminates-after-a-definite-sign
kind: issue
title: fillet3_chain_arm mints Indeterminates by hand after a definite non-positive sign
status: open
opened: 2026-10-01
---


## What

`crates/sweep/src/blend/battery.rs` asks the funnel
`decide("fillet3_chain_arm", Margin::of(arm), band)` and, on a definite
`Sign::Zero | Sign::Negative`, builds its own
`Indeterminate { margin: MarginDiag::INVALID, predicate:
Some("fillet3_chain_arm"), .. }` and wraps it in `esc(site, …)`. Two
shipped sites: `convexity_at` (the `BlendSite::Link` arm, ~:650) and
`chain_g1` (the `BlendSite::Joint` arm, ~:715).

The refusal the caller gets is on no frame's escalation log: the
funnel recorded the `Zero`/`Negative` verdict and nothing else, so a
bracket around a fillet sees a decision and no escalation, while the
op returns one.

## Shape

This is the collapsed-arm gate exactly:
`geom_core::k_stats::decide_positive("fillet3_chain_arm",
Margin::of(arm), band).map_err(|e| esc(site, e))?` replaces each
`match` and its hand mint. The same fix landed for `topo`'s
`sector_shape` (`sector_arm`, `sector_straight`) on the `linalg`
branch `linalg/decided-not-minted`, which found these two by sweeping
`MarginDiag::INVALID` literals across `crates/*/src`.
