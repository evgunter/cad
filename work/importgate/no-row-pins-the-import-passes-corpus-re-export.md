---
id: no-row-pins-the-import-passes-corpus-re-export
kind: issue
title: No committed row pins the import passes' corpus re-export: two surgery mutants move six fixtures' bytes and only synthetic unit rows see it
status: open
opened: 2026-11-19
priority: P2
cost: M
---

Found by the single review of PR 7134 (`importgate/split-share`).

## What

Two mutants of `crates/step-import/src/normalize/surgery.rs` change real fixtures' re-exports, yet pass every knot, band, singularity and round-trip row:
- `relink` ignoring its seeds (surgery.rs ~313) moves 6 fixtures: `band/ftc11_uref_off`, `c0-knot/{c_prism,half_cone,ridge_u,ridge_v}` and `nist_ftc_11_asme1_rb`;
- `split_at_point` keeping the parent's interval (surgery.rs ~158–181) moves 3: `band/ftc11_uref_off`, `seam-region/side_cut_meridian_split` and `nist_ftc_11_asme1_rb`.

Only the two synthetic `surgery::tests` rows added by PR 7134 kill them. That PR's "byte-identical corpus" claim rests on an uncommitted probe: a per-fixture hash of the ledger and the `step_export::step_string`.

## What would close it

A committed corpus fingerprint row (the ledger and the re-export, per fixture under `tests/fixtures`), pinned, that turns red when either measurement moves. Alternatively, per-fixture rows asserting what the seeds and the interval decide.
