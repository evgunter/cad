---
id: main-red-rustdoc-topo-intra-doc-links-unresolved
kind: issue
title: Main red (nightly rustdoc gate): 22 unresolved intra-doc links across topo
status: closed
opened: 2026-10-20
priority: P1
cost: E
closed: 2026-10-24
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

**Still red on the 2026-10-23 nightly (INTENT triage).** Run 39906156937
(main `be9b907694`) lists the same family plus
`boolean/solid_contain.rs:4653` (`census::line_passes`, first printed
on the 2026-10-21 nightly, run 39549991622) and `census.rs:5874`
(`face_piece`) and `:7641` (`Sector::place`) where the census lines
moved. The `wedge_reach` and `validate` errors are ambiguity, not
absence (`crate::wedge_reach` and `crate::validate` are each a function
and a module; `mod@` or `()` settles each). INTENT's triage PR fixes the
editor-core, pncad-py and viewer links the same job reported.

## Closed (2026-10-24, INTENT nightly triage)

Nightly run 40066431545 (main `3dfb96b91c`) still listed the family:
`attach.rs`, `boolean/{mod,node,rim_wedge,solid_contain,sphere_region,torus_region}.rs`,
`census.rs`, `euler.rs`, `offset_restate.rs`, `policy_lane.rs`,
`side_reader.rs` and `split.rs`. Each is now pathed the way the code
names its item. `crate::wedge_reach` and `crate::validate` become
`fn@…`, each naming a function that shares its name with a module.
`Coincide` becomes `super::Coincide`, and `census::line_passes` becomes
`crate::census::line_passes`. `ConicArc::hit` links to
`crate::splitting::containment::ConicArc::hit`. In `census.rs`'s `metric`
and `rest` submodules, `face_piece` and `Sector::place` link through
`super::`. The same PR fixed geom-brep's `props/spiric.rs` (five links
to `#[cfg(test)]` helpers, now plain code spans) and editor-core's
`mate/corner.rs` (`super::pin::moved_frame`).
`scripts/doc-gate.sh` passes locally over every root. Docs only.
