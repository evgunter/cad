---
id: contain-doc-links-a-wrap-rims-that-moved
kind: issue
title: topo boolean/contain.rs links super::solid_contain::wrap_rims, which does not resolve, so rustdoc --document-private-items fails (nightly rustdoc row)
status: closed
opened: 2026-10-01
priority: P4
cost: E
closed: 2026-10-26
---


(SSI orchestrator, 2026-10-01; found by SSI's chart-tube lane running rustdoc
with `--all-features --document-private-items`.)

`crates/topo/src/boolean/contain.rs` (~823) has a doc link
``[`super::solid_contain::wrap_rims`]`` that does not resolve on `main`, so
rustdoc fails on main's own code, and the nightly's rustdoc row will go
red. The link came in with `70be4e1c3` ("REACH: one structural full-turn
test for every curved chart, asked first"). The fix is to point the link
at wherever `wrap_rims` lives now, or to make it plain text.

## Closed (2026-10-26, tracker sweep)

**Fixed.** `crates/topo/src/boolean/contain.rs` no longer links `super::solid_contain::wrap_rims`: no `wrap_rims` appears in the file, and no `solid_contain::wrap_rims` appears anywhere in `crates/` at `bb25fa4f`. `wrap_rims` now lives in `boolean/surface_group.rs`, which `torn_hop_rows.rs:284` imports. The history is shallow, so the commit that dropped the link was not found.
