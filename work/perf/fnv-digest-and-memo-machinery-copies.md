---
id: fnv-digest-and-memo-machinery-copies
kind: issue
title: the FNV-1a digest, the goldens' mesh digest and the picture-counter memo machinery exist in many copies
status: open
opened: 2026-09-12
---


## The residue

Three things are spelled more than once, each copy correct and none
shared. PERF-5 added the last copies and cites this item from both
memo types rather than consolidating in a perf unit.

**The FNV-1a offset basis `0xcbf29ce484222325` (with the prime),
hand-rolled at each site:** `crates/editor-core/src/eval/memo.rs:59`
(the content key), `crates/editor-core/src/stackup.rs:794`,
`crates/mesh/src/memo.rs:82` (the patch digest),
`crates/topo/src/seqgen.rs:1684`, `crates/test-utils/src/fuzz.rs:242`,
and in tests `crates/mesh/tests/d9_mesh_goldens.rs:102`,
`mesh10r1_digest.rs:35,82`, `mesh10r2_digest.rs:19`, `r1_probe_hash.rs:35`,
`r2_bytes.rs:61`, `review_m2_pr6_determinism.rs:22`, `probe_review.rs:237`,
`patch_memo.rs:47`, `crates/viewer/tests/index_memo.rs:62`,
`crates/editor-core/tests/fixture/digest.rs`, `asm2b_multisolid.rs:382`,
`perf2_name_keying_differential.rs:62`, `crates/sweep/tests/blend4_r1_probes.rs:289`,
`crates/step-import/tests/inst_review_probes.rs:330`,
`crates/stl/tests/export.rs:13`.

**The goldens' whole-`Mesh` digest** (positions by bits, patches by
face key and triangles, boundaries by edge key, ids and vertex keys):
`d9_mesh_goldens.rs:101`, `patch_memo.rs:45`, `index_memo.rs:66`, and
the reviewer's probe files derive from it. One `mesh::validate`-side
`digest(&Mesh) -> u128` (or a `test-utils` helper taking the goldens'
form) would make "the goldens' way" one function rather than a phrase.

**The picture counter machinery** — a `picture` stamp, an `open`
that zeroes counters on the first use after a close, `end_picture`
retaining stamped entries — is written twice: `mesh::PatchMemo`
(`crates/mesh/src/memo.rs`) and `editor_core::PickMemo`
(`crates/editor-core/src/resolve/pick.rs`). A generic
`Generational<K, V>` in one crate both depend on (`geom-core` is the
only common ancestor with no persistence) would hold it once.

## What a fix is

One consolidation unit, no behaviour change: a `test-utils` FNV/digest
helper for the tests, a `pub` digest beside `mesh::validate` for the
two production consumers that hash meshes, and the generational map
lifted out of the two memos. Not scheduled by PERF; filed so the
copies are named where the next one would otherwise be added.
