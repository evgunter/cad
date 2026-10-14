---
id: disp-zero-can-be-satisfied-by-freezes
kind: issue
title: The digest's disp=0 can be satisfied by freezes: a frozen contradiction check records no dispute
status: closed
opened: 2026-10-12
priority: P4
cost: M
branch: nurbs/dispute-coverage
closed: 2026-10-13
pr: 4944
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

## Closed (2026-10-13, PR 4944)

`SymCounts` gained `checks_expanded` and `checks_frozen`, which split the
contradiction checks by whether a form they read reaches only free
indeterminates: π, or the parameters, opaque values and hulls the
session recorded. A node frozen on the budget, a node minted before the
session, or a poisoned form puts the check in `checks_frozen`.

The digest now builds its `Sym` bodies inside the session and receipts
the door alone. It prints the split as `chk=<expanded>/<checks>`, and
guards every receipt with `dispute_coverage`: at least one check, and,
over 20 checks or more, at least 4/5 of them on whole forms. Measured at
every ε, `validate_geometric` reads 103/110 on the arc loft and 104/116
(106/116 at 1e-6) on the strip, with `disp=0`. Built outside the
session, the arc door reads 14/110 and still `disp=0`, and the guard
trips
(`bodies_built_before_the_session_trip_the_guard_while_disp_still_reads_zero`).

The sibling in the SYM-11 far-placement table is filed as
`work/sym/far-placement-zero-dispute-cells-can-be-satisfied-by-freezes`.
