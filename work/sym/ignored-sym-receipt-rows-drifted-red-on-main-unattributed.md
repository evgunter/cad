---
id: ignored-sym-receipt-rows-drifted-red-on-main-unattributed
kind: issue
title: The ignored SYM receipt rows (pad's rule-F differential, SYM-11 past the ceiling) were red on main with the drift unattributed
status: open
opened: 2026-09-26
priority: P1
cost: D
---

**Found by ENCL's must-carry-gate re-baseline, and pre-existing.** Two
`#[ignore]`d evidence rows pin the pad's receipts exactly, and both are
red on main at `8ee3daf171`, before the must-carry first-order gate.
The gating row, `m10_9_no_registrant_lies_on_any_measured_document`,
pins only `registered` and `symbolic_zero` and stays green.

**Stored vs measured on main (`8ee3daf171`)**, as `[symbolic_zero,
registered, numeric, frozen]`. The past-the-ceiling row is measured at
ε = 1e-6 and the pads-four row at the default ε; both rows claim their
receipts are ε-independent.

| row | document / dial | stored | measured | drift |
| --- | --- | --- | --- | --- |
| `sym11_exact_channel_rows.rs` `PAST_THE_CEILING` | two_hole_plate | `[803, 140, 470, 1044]` | same | — |
| ″ | r1_annulus | `[328, 140, 209, 1056]` | same | — |
| ″ | r2_link | `[214, 76, 175, 556]` | `[214, 76, 179, 556]` | numeric +4 |
| ″ | r2_filleted_bracket | `[428, 141, 341, 1096]` | `[428, 141, 343, 1096]` | numeric +2 |
| ″ | r2_rounded_pad | `[854, 128, 971, 2750]` | `[854, 128, 979, 2722]` | numeric +8, frozen −28 |
| `m10_9_pins_interval.rs` `m10_9_the_pads_four_at_both_dials` | pad, rule F off | `(858, 104, 991, 2750)` | `(858, 104, 999, 2722)` | numeric +8, frozen −28 |
| ″ | pad, rule F on | `(854, 128, 971, 2750)` | `(854, 128, 979, 2722)` | numeric +8, frozen −28 |

`registered`, `registrations_refused`, `registrations_contradicted` and
`theorems_disputed` do not move, so no registrant is implicated. Rule
F's differential (−4 theorems, +24 registered, −20 numeric) is intact.
What is **not** measured: which change moved `numeric` and shrank the
pad's `frozen` set. The sym11 row's own message calls a moved receipt
"a finding, not a table to refresh".

**On `encl/batch-split-loop-mustcarry` the stored values carry only
that batch's gate delta** (A/B-attributed: the must-carry rule's
per-station `dihedral_wedge`, +28 `symbolic_zero` and +84 `numeric` on
the pad at both dials, marked beside each value), so the two rows stay
red there by exactly the drift above.

**Remedy.** Bisect main for the commit that moved these numbers and say
whether each moved decision is right; then re-take the tables. Also
give the two rows a schedule: `#[ignore]` plus "re-taken at each SYM
unit's close" let the drift land with no one seeing it.
