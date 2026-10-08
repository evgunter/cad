---
id: contain-doc-links-a-wrap-rims-that-moved
kind: issue
title: topo boolean/contain.rs links super::solid_contain::wrap_rims, which does not resolve, so rustdoc --document-private-items fails (nightly rustdoc row)
status: open
opened: 2026-10-01
priority: P4
cost: E
---


(SSI orchestrator, 2026-10-01; found by SSI's chart-tube lane running rustdoc
with `--all-features --document-private-items`.)

`crates/topo/src/boolean/contain.rs` (~823) has a doc link
``[`super::solid_contain::wrap_rims`]`` that does not resolve on `main`, so
rustdoc fails on main's own code, and the nightly's rustdoc row will go
red. The link came in with `70be4e1c3` ("REACH: one structural full-turn
test for every curved chart, asked first"). The fix is to point the link
at wherever `wrap_rims` lives now, or to make it plain text.
