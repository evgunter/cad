---
id: m5-s13-pips-union-escalation-arm-runs-at-no-gated-eps
kind: issue
title: m5_s13_pips_interval's union escalation arm executes at none of the three gated eps rows
status: open
opened: 2026-09-19
priority: P3
cost: E
---

## Finding

- **Where**: `crates/sweep/tests/m5_s13_pips_interval.rs`,
  `interval_finding_union_is_bracketed` — the `Err` arm holding
  `assert!(hi <= UNION_MAPPED_ENCLOSURE_HI, …)` and the four
  assertions above it.
- **Importance**: medium
- **Confidence**: sure, and measured by execution rather than read
- **Raised by**: the `dup/private-box-builders` lane, 2026-09-19,
  reconciling the constant's doc against its code

The row chooses its arm by the OUTCOME of `topo::union`, deliberately:
its body comment records the change away from selecting the arm by
comparing ε to a pinned constant, because *"a narrower mapped-source
enclosure is the improvement the whole programme is for"*.

**Measured 2026-09-19** on `dup/private-box-builders`, running the row
under `CAD_TOLERANCE_EPS` at each of the three gated values with
`--features interval`:

| ε | arm taken |
| --- | --- |
| 1e-12 | DECIDE |
| 1e-9 (default) | DECIDE |
| 1e-6 | DECIDE |

The union decides at every ε CI runs, so the escalation arm — five
assertions, including the only guard on `UNION_MAPPED_ENCLOSURE_HI` —
**never executes in the gate**. The same is true at that branch's merge
base (`5b4979ef2`), so this is not something the fold caused; the fold
is only what made someone read it.

Two consequences, and the second is the reason this is S-TINT's:

- `UNION_MAPPED_ENCLOSURE_HI` is not guarded by anything. It is a
  measurement nothing re-takes and nothing can red.
- The row's DECIDE arm is the only one under test, so the row's name
  (*bracketed*) and its escalation prose describe behaviour no run
  observes.

Not a defect to repair by re-scoping the row back to an ε comparison —
that is exactly what was deliberately removed. The question this row
asks is what, if anything, should still witness the enclosure width now
that the chain serves this union at every gated scalar: a dispatched
row at a coarser ε, a different fixture, or an explicit statement that
the class is closed and the constant retires with it.

`m5_s12_curved_ops_interval`'s `RECUT_MAPPED_ENCLOSURE_HI` is the
contrasting case and is NOT affected: its escalation arm does execute
at (interval, 1e-12), and as of 2026-09-19 it is pinned bit-exactly
from both sides in two files over one shared fixture.

