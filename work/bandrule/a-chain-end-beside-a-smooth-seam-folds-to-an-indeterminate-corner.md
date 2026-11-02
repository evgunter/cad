---
id: a-chain-end-beside-a-smooth-seam-folds-to-an-indeterminate-corner
kind: issue
title: A blend chain ending at a vertex where a smooth seam meets folds the seam into CornerConfig::Indeterminate
status: open
opened: 2026-11-02
---

Filed by the REFS lane (`work/refs/select-all-edges-picks-edges-a-fillet-refuses.md`), from its sweep for blend doors that read a Zero dihedral as a refusal.

A requested smooth seam is now left as it is (`battery::wedge_at`, `LinkWedge::Smooth`, `crates/sweep/src/blend/battery.rs`). At an OPEN chain's end, though, `battery::corner_at` resolves the vertex's other edges as links to count their convexity, and a neighbour that resolves `Resolved::Smooth` folds into `CornerConfig::Indeterminate` exactly as the zero-wedge refusal it replaced did (the `Ok(Resolved::Smooth(..)) | Err(_)` arm). So a chain ending where a smooth seam arrives — one arc of a bore rim beside its strut, a plate's top edge ending at a profile fillet's tangent seam — refuses "the corner's configuration could not be read", whose recourse does not name what is there: a smooth seam has no wedge to count, and the vertex is a corner of the remaining edges alone (or a seam crossing, as `CornerConfig::SeamVertex` reads for valence four).

Unmeasured: no fixture in the REFS unit reaches this arm (its rows request whole rims, which close and take no corner). The first measurement is one arc of `sweep::test_support::bored_block_of_arcs(2, ..)`'s rim requested alone.
