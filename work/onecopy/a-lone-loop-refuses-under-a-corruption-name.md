---
id: a-lone-loop-refuses-under-a-corruption-name
kind: issue
title: Readers that refuse a face holding a lone-vertex loop report it as corruption (Unbuilt, TouchUnread::Corrupt, CorruptLoop), though tier 1 admits the loop
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-09
pr: 6683
---


Found by `onecopy/lone-vertices` (PR 6654's review, note L2), whose
sweep classed these sites as refusing a lone loop loudly rather than
dropping it.

## The finding

A lone-vertex loop (`LoopBoundary::Empty`) is tier-1 valid. Tier 2's
check 1 (`ScaffoldingEmptyLoop`) refuses it ahead of the census and a
boolean's inputs, so at rest these readers do not meet one. Where they
do (an ungated caller, a body mid-operation), they refuse it under a
name that says the body is corrupt:

- `topo/src/census.rs`: the `OnVertex` cone (~1908, `Unread::Unbuilt`),
  `tangent_sectors` (×2, `TouchUnread::Corrupt`), and the planar
  touch reading through `FaceGeo.walked` (`TouchUnread::Corrupt`);
- `topo/src/link.rs` `of_body_face` and ~2429 (`TouchUnread::Corrupt`);
- `topo/src/splitting/containment.rs` `loop_points` (`CorruptLoop`),
  `splitting/section.rs` `section_walk` and `splitting/join.rs`
  `loop_starts` (`SectionInvariant`), `coherence.rs` `traversals`
  (`Unexaminable::Corrupt { EmptyLoop }`).

Each reads `unchecked_face_cycles` (or its own walk), where a lone loop
is an `Err` beside a torn one, so the two cannot be told apart.

## Fix

Read `Body::unchecked_face_boundary`, whose `UncheckedLoop::Lone` is a
member, and give a lone loop its own refusal (or reading) at each site,
leaving the corruption names to a loop that does not walk.
