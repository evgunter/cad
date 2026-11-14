---
id: topo-hand-mints-indeterminates-geom-brep-census-would-refuse
kind: issue
title: topo hand-builds Indeterminates off the frame's log at five shipped sites; geom-brep's escalation census would refuse them
status: open
opened: 2026-11-13
---

Found by `carrier-eq-and-plane-eq-take-no-body` (branch `onedoor/carrier-eq-home`). Moving `plane_eq.rs` into `geom-brep` brought `unreadable_norm` under `geom-brep/tests/kstats_escalation_channel.rs` (`shipped_geom_brep_code_spells_no_indeterminate_literal`), and that unit re-minted it through `geom_core::k_stats::gate_rejection`. `topo` has no such census. Its scanner (`shipped_mints`), pointed at `crates/topo/src` at that branch's head, reports 18 hits:

**Shipped mints (5), off the frame's log:**
- `bound_side.rs` (`ReadFault::Unwitnessed`, `MarginDiag::INVALID` under `UNWITNESSED`), line 1101;
- `chart_region.rs` `definite_diag` (a classified value echoed as a fresh `Indeterminate`), line 811;
- `chart_region.rs` (`chart_region_cyl_axis_sense`, `MarginDiag::INVALID`, "poisoned input"), line 1638;
- `invalid_margin.rs` `invalid`, the crate's one home for `MarginKind::Invalid` mints, line 24. Its 3 callers are mints the scanner cannot see (its "helper" blind spot);
- `validate.rs` (an enclosure `Indeterminate` with `predicate: None`), line 7365.

**Over-count (1):** `validate.rs` line 509 is a match pattern (`Err(Indeterminate { margin, .. })`), not a construction.

**Test-only (12), which the scanner reads as shipped** because their `#[cfg(test)]` (or `test-support`) gate sits on the `mod` line in the parent: `boolean/carrier_ladder_rows.rs`, `boolean/offer_rows.rs`, `boolean/section_unbuilt_rows.rs`, `boolean/standing_rows.rs`, `chord_join/ring_side_rows.rs`, `chord_join/section_end_rows.rs` (×3), `test_support_samples.rs` (×2), `tier3_tests.rs` (×2).

**Proposal:** the same census for `topo`. That needs a module-gate exclusion (a file whose `mod` declaration is `cfg(test)` or feature-gated), and a pattern/construction distinction for the over-count. Each of the five shipped sites then mints through the `k_stats` door whose doc fits its cause: `gate_rejection` for an impossible sign, `straddle` for two bounds across the band. `invalid_margin.rs`'s own module docs already name both causes.
