---
id: skin-coincident-section-check-is-an-unbanded-f64-compare
kind: issue
title: skin.rs refuses coincident loft sections by a bare f64 strict comparison (params[j-1] < params[j] → DegenerateSection) — per-pair and named, but unbanded
status: open
opened: 2026-09-16
refs: [BOOL-6, 2752]
---

Found by BOOL-6 (PR 2752) while placing the per-slab stacking fold's
degenerate row and filed by the S-BOOL orchestrator on BLEND's slate
(`crates/sweep/src/skin.rs` is BLEND's path). The skin already checks
adjacent sections per pair and names the section it refuses, but the
check is a raw `<` on the section parameters rather than a banded
decide, so EXACTLY coincident sections refuse at the skin while
sections within ε of each other pass the skin and reach the loft's
stacking fold, which refuses them as a sliver (`DegenerateStacking
{ slab }`) or escalates at the band's midpoint. Two doors, two
vocabularies, for one fact. The fix is the skin's check as a named
decide with the run's band (or the skin ceding the question to the
fold and refusing nothing itself), stated once. Measured, not acted
on; difficulty S.
