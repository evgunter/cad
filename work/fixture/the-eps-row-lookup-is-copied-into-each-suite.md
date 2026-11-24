---
id: the-eps-row-lookup-is-copied-into-each-suite
kind: issue
title: The lookup from this run's ε to its row of a per-ε table is spelled privately in each suite that pins per ε
status: open
opened: 2026-11-22
priority: P4
cost: E
---


Found by PR 7389's review (GREEN, `green/editor-1e-12`, 2026-11-22). A row pinned per ε row needs this run's ε turned into the row of its table, failing loud at any other ε. Each suite spells that lookup itself, mostly as one copied idiom, `[1.0e-6, 1.0e-9, 1.0e-12].iter().position(|&e| (eps / e - 1.0).abs() < 1.0e-3)`:

- `crates/editor-core/tests/m10_sym_profile_interval.rs`, `eps_row`;
- `crates/editor-core/tests/decide_6_read_cost_interval.rs` (inline, near line 94);
- `crates/editor-core/tests/m10_9_pins_interval.rs` (inline, near line 690);
- `crates/sweep/tests/sym11_far_placement_rows.rs`, `eps_row`;
- `crates/mesh/tests/split_rulings.rs`, `eps_row` (returns its own `EpsRow`);
- `crates/editor-core/tests/revalue_corpus.rs`, `PerRow::here` (PR 7389: a struct named by ε, so a swapped pair cannot pass).

`test_utils::pinned_red::EVERY_ROW` already names the three rows CI draws from. The fix is one door in `test-utils` (a per-ε value named by its ε, as `PerRow` is), used at each site.

**What the grep above cannot see:** a site that branches on ε by `==` or by a match. A second pass for `eps() == 1e-6`-shaped comparisons lists about twenty more test files under `crates/{sweep,mesh,geom-brep,profile}/tests/` (for example `sweep/tests/ellipse_torus.rs`, `mesh/tests/newell_probes.rs`, `geom-brep/tests/m5_pr7_ssi.rs`). Some of those are an ε-gated arm and not a table lookup, so each needs reading before it joins the class.
