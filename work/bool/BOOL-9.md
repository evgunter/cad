---
id: BOOL-9
kind: unit
title: issue 433 half (ii) — the RawLoop demotion
status: spec
opened: 2026-09-01
refs: [BOOL-12, 433]
---

Q1 ruling half (ii): `RawLoop` does not remain writable — the vertex table
demotes to the materialized/cache form, authoring goes through the lattice
only, every in-repo writer migrates (fixtures to the lattice or a dev-only
door per the LoopBuilder precedent; step-import marked as a materialization
door), `validate` stays the data checker for materialized loops.
Survey-first; difficulty L. Spec: `docs/BOOL-9-SPEC.md`.

Sequenced after BOOL-12 (lily leaves `RawLoop` only once the declared
arrival lands). From `work/bool/log.md`, "BOOL-8 merged (2026-09-01)" and
the BOOL-13 entry's slate line ("Then BOOL-9, BOOL-10").
