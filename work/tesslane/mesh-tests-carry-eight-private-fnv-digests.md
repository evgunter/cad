---
id: mesh-tests-carry-eight-private-fnv-digests
kind: issue
title: mesh's tests carry eight private FNV-1a digests of a mesh's bytes, no shared one
status: open
opened: 2026-11-23
priority: P4
cost: C
---


Found by the review of `tesslane/one-slit-band` (PR 7498), which removed a ninth.

## What

Eight test files in `crates/mesh/tests` each define their own FNV-1a over a mesh's bytes: `d9_mesh_goldens.rs`, `mesh10r1_digest.rs`, `mesh10r2_digest.rs`, `patch_memo.rs`, `r1_probe_hash.rs`, `r2_bytes.rs`, `review_m2_pr6_determinism.rs`, `route_rows.rs` (each `fn fnv`). They differ in what they fold (positions only, positions and triangles, boundaries and keys), so their constants are not interchangeable, and a new byte pin has no shared door to reach for.

## What closing it takes

One digest in `tests/common` (or `test-utils`), with the fold named (what bytes it reads), and the files moved onto it where their fold matches. A constant that moves with the fold is re-pinned in the same change.
