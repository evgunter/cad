---
id: disp-zero-can-be-satisfied-by-freezes
kind: issue
title: The digest's disp=0 can be satisfied by freezes: a frozen contradiction check records no dispute
status: open
opened: 2026-10-12
priority: P4
cost: M
---


Found by the STYLE review of PR 4887. For later.

PR 4887 added `disp=` (`SymCounts::theorems_disputed`) to the thread-count
digest's `Sym` line (`counts_line`,
`crates/sweep/tests/mass_props_are_thread_count_invariant.rs`). It reads
0 on every line. But a contradiction check whose form froze can't record
a dispute, so `disp=0` gets weaker as `frozen` rises, and the digest has
no way to tell the two apart. Across one day, `sym_arc_loft
validate_geometric` read `frozen` 1060 → 1916 → 1542 (PRs 4869 and
4879) while `disp` stayed 0. At ε 1e-9, the contradiction-check freeze
sites alone went 54 → 1848 when the meter began computing in-session.

Proposal: a guard that goes red when dispute coverage degrades, rather
than another count to re-cut. Candidates:
- a floor on the contradiction checks that ran on an expanded form;
- `disp` reported per non-frozen check.

Either needs the check count split by frozen and not frozen in
`SymCounts` (SYM/DECIDE's ground in `geom-core/src/sym.rs`), so it is
an M and a seam.
