---
id: declared-surface-pairs-emits-duplicate-pairs
kind: issue
title: boolean::ops declared_surface_pairs emits one surviving declared pair per FACE sharing the key - nine identical pairs per three-arc bore wall
status: open
opened: 2026-09-07
refs: [cylindrical-rest-pair-hits-planar-merge, 2105]
---


## What

Measured by the CURVED merge-door unit (PR #2105, row 0): on a
peg-in-bore with a three-arc bore wall, `declared_surface_pairs`
(`crates/topo/src/boolean/ops.rs`) hands the merge door NINE identical
`(SurfaceKey, SurfaceKey)` pairs — one per (face, face) combination
sharing the two surface keys — so the door's skip record repeats nine
times for one declaration. Harmless for the planar merge (union is
idempotent) and noisy for the new record. Also in the same function's
family: `ops.rs`'s `describe_minted_edges` error arm drops a
`Certification { ResidualExceeded }` payload into a bare `JoinDesync`,
which is the refusal a user sees for the zip's chord-on-cylinder defect
(`work/curved/rest-zip-seam-chord-on-cylinder-wall.md`).

## Fix

Deduplicate on the surface-key pair before handing to the door; carry
the certification payload through `JoinDesync`'s `what`. E.

## Home

Unowned at filing — `boolean/ops.rs` is S-BOOL's glob; on the CURVED
handover list. Filed by the CURVED orchestrator.
