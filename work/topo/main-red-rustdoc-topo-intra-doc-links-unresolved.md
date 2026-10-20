---
id: main-red-rustdoc-topo-intra-doc-links-unresolved
kind: issue
title: "Main red (nightly rustdoc gate): 22 unresolved intra-doc links across topo"
status: open
opened: 2026-10-20
priority: P1
cost: E
---


## What

`cargo doc -p topo --no-deps --document-private-items --all-features`
with `RUSTDOCFLAGS="-D warnings -A rustdoc::private_intra_doc_links"`
(the flags `scripts/doc-gate.sh` uses) fails on `origin/main`
(`65fd9afd`) with 22 unresolved links:

- `attach.rs:1561`, `:1604`
- `boolean/mod.rs:7218`
- `boolean/node.rs:191` (`Coincide::Sectors`), `:233` (`Coincide::VertexOnCurvedFace`)
- `boolean/rim_wedge.rs:1030`
- `boolean/sphere_region.rs:53`, `:73`, `:89` (`ConicArc::hit`)
- `boolean/torus_region.rs:67` (`ConicArc::hit`)
- `census.rs:2109`, `:5265`, `:7032`
- `euler.rs:3956`
- `offset_restate.rs:111`, `:147`
- `policy_lane.rs:153`, `:168`, `:227`
- `side_reader.rs:8`, `:11`
- `split.rs:265`

The rustdoc gate runs only in the nightly (`ci.yml:9`), so no per-PR
check catches these. NURBS build A found them while running the doc
gate for PR 5481 and its follow-up on check 9, neither of which
touches these lines.
This item was first filed under cleave with the six `boolean/` links
only; the full set spans many programs' ground in topo, so it lives
here.

## Fix

Path each link the way the code names the item, then run
`scripts/doc-gate.sh` green. This touches docs only; no code moves.
Consider whether per-PR CI should run the rustdoc pass on changed
crates, so that drift stops reaching main unseen.
