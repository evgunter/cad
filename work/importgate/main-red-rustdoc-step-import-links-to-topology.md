---
id: main-red-rustdoc-step-import-links-to-topology
kind: issue
title: main red: rustdoc rejects three step-import links to `topology` (error.rs, knot_split.rs, lib.rs)
status: open
opened: 2026-11-09
priority: P0
---

Nightly run 42567046151 (sha `836426e07e`), job `rustdoc (gate, every root)`: `unresolved link to topology` at `crates/step-import/src/error.rs:398`, `crates/step-import/src/knot_split.rs:1` and `crates/step-import/src/lib.rs:413`. Each writes `` `docs/DESIGN.md` [topology] `` as a clause citation, and rustdoc reads the bracket as an intra-doc link. Spell the clause id so that it is not a link (for example, in backticks).

Red since before 11-02 (the 11-02 nightly, run 41393708363, reports all three), so it is not BOXES'.

(BOXES nightly-reds triage, 2026-11-09)
