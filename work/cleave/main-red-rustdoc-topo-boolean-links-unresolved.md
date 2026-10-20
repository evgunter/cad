---
id: main-red-rustdoc-topo-boolean-links-unresolved
kind: issue
title: Main red (nightly rustdoc gate): six unresolved intra-doc links in topo's boolean node, sphere_region and torus_region
status: open
opened: 2026-10-20
priority: P1
cost: E
---


## What

`cargo doc -p topo --no-deps --document-private-items --all-features`
with `RUSTDOCFLAGS="-D warnings -A rustdoc::private_intra_doc_links"`
(the flags `scripts/doc-gate.sh` uses) fails on `origin/main`
(`bea03a9f`) with six unresolved links:

- `crates/topo/src/boolean/node.rs:191`: `Coincide::Sectors`
- `crates/topo/src/boolean/node.rs:233`: `Coincide::VertexOnCurvedFace`
- `crates/topo/src/boolean/sphere_region.rs:53`, `:73`, `:89`: `ConicArc::hit`
- `crates/topo/src/boolean/torus_region.rs:67`: `ConicArc::hit`

The `Coincide` links are written without the `super::` path the code
uses. The rustdoc gate runs only in the nightly (`ci.yml:9`), so no
per-PR check caught these. They were found by NURBS build A while
running the doc gate for PR 5481, which touches none of these files.

## Fix

Path each link the way the code names the item (`super::Coincide::…`,
the module path of `ConicArc`), then run `scripts/doc-gate.sh` green.
Docs only; no code moves.
