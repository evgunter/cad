---
id: BOOL-9
kind: unit
title: issue 433 half (ii) — the RawLoop demotion
status: closed
closed: 2026-09-08
opened: 2026-09-01
refs: [BOOL-12, 433]
branch: bool/9-rawloop-demotion
pr: 2134
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

## Closed

PR 2134 merged 2026-09-08 (fix-pass head `9d71e442a`, run 34260713937
green). The vertex table is a cache: `RawLoop` exists only behind
`profile/test-support` (dev-dependency edges only; the shut arm's trait
is crate-private, so a re-export is E0365), the one materialization
door is scalar embedding (`ProfileLoop::map`), the lift seams at the
declared arrival. Issue 433's raw-door half is landed; the
`lattice-validate-collinear-junction-disagreement` item closes with
this. A/B row BOOL9, sample #164. Residue filed on other slates at this
merge: the forgeable certificate family (`work/props/`); the compile-arm
gate (`raw-door-compile-proof-needs-a-gate`, this slate, already open).
