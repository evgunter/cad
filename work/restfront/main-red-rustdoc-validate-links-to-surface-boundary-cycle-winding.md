---
id: main-red-rustdoc-validate-links-to-surface-boundary-cycle-winding
kind: issue
title: main red: rustdoc rejects topo validate.rs's link to crate::surface_boundary::cycle_winding
status: open
opened: 2026-11-09
priority: P0
---

Nightly run 42567046151 (sha `836426e07e`), job `rustdoc (gate, every root)`: `unresolved link to crate::surface_boundary::cycle_winding` at `crates/topo/src/validate.rs:1962` (`/// deck element, [`crate::surface_boundary::cycle_winding`]) are not a`).

Red since before 11-02 (run 41393708363, then at `validate.rs:1920`), so it is not BOXES'.

(BOXES nightly-reds triage, 2026-11-09)
