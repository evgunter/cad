---
id: main-red-the-copysign-census-names-a-walk-rs-site-since-the-horn-cusp
kind: issue
title: Main red (slow set): sym_rule_f_rows' copysign census names a new mint site in crates/mesh/src/walk.rs since PR 6806
status: closed
closed: 2026-11-11
pr: 6838
opened: 2026-11-11
priority: P1
cost: E
---

Found by PCERT's nightly-reds lane (`pcert/nightly-slowset-reds`), in nightly 42965378060 (main `b2916558d9`), at default ε and 1e-6.

**What fails.** `geom-core sym_rule_f_rows::the_copysign_mint_sites_the_tree_holds_are_these` (`crates/geom-core/tests/sym_rule_f_rows.rs`): the tree holds `"crates/mesh/src/walk.rs": 1` and the table does not.

**The site.** `crates/mesh/src/walk.rs`, in `loop_polygon`: `vp.copysign(chart.v_of(positions[cur.ids[1] as usize]))`. It is 0 at `eb319b9d^1` and 1 at `eb319b9d`, the merge of PR 6806 (`spiricarm/horn-cusp`). The nightly before (`50cfcf499d`) was red on this row for a different site (`crates/geom/src/chart_point.rs`), which has since been listed.

**What closing it takes.** The census asks for a decision, not a count: re-ask `m10_10_evidence_interval`'s `sym12_the_copysign_census_at_the_nominal` on the measured documents, decide whether the site mints a sign a `Sym` scalar can carry, then move the table and `manifest.rs`'s header list together.

The class (slow-set source censuses no PR gate selects) is on green's `a-src-change-that-moves-a-decide-site-skips-its-crates-recourse-roster`.

## Closed (2026-11-11)
PR 6838. The site mints no sign a `Sym` scalar can carry: `loop_polygon` reads `Body<f64>` and `Chart<f64>` (`is_horn` exists only there), so the call is `f64::copysign`; the census on the eight measured documents finds 0 copysign atoms. Registered `f64` only beside `svd.rs` and `banded.rs`, the table and `manifest.rs`'s list moved together, with the zero bound stated at the entry (a circle meridian's chord step ≤ π/4 puts the next vertex at |v| ≥ 3π/4). The row is red on main `865e27e41b` and green on the head. The class this exposed is filed: `the-copysign-census-counts-f64-typed-sites-a-sym-scalar-never-reaches`.
