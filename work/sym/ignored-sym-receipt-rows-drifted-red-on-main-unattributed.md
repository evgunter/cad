---
id: ignored-sym-receipt-rows-drifted-red-on-main-unattributed
kind: issue
title: The ignored SYM receipt rows (pad's rule-F differential, SYM-11 past the ceiling) were red on main with the drift unattributed
status: dispatched
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

**Measured on the run-out carrier's merge of main** (PR 3266),
identical at ε = 1e-6, 1e-9 and
1e-12 — pad past the ceiling `[885, 128, 1066, 2722]`, rule F off/on
`(889, 104, 1086, 2722)` / `(885, 128, 1066, 2722)`, link
`[214, 76, 179, 556]`, bracket `[429, 141, 344, 1096]`. That PR's own
share is +3 `symbolic_zero` and +3 `numeric` on the pad and +1 / +1 on
the bracket; the drift in the table above (`numeric` +8 and `frozen`
2750 → 2722 on the pad, `numeric` +4 link, +2 bracket) is carried in
those values and still unattributed, so the bisect's target is
unchanged.

The rows store main's values plus only that PR's own delta, not main's
drift, so they stay red by exactly the drift above.

## What Phase 1 found (SYM-16)

Receipts below are `[symbolic_zero, registered, numeric, frozen]` for
the past-the-ceiling row and `(symbolic_zero, registered, numeric,
frozen)` for the pad's rule-F row at both dials, measured with
`/home/user/sym-16-tmp/bisect-step.sh` (in the PR body): release
build, ε = 1e-9, both rows in one process, the past-the-ceiling row's
receipt assertion disabled so that a moved document does not hide the
ones after it. `sign_gated` was 0 everywhere in the window.

### The line bisected

`main`, first parent. The two rows lived on `main` through the whole
window: SYM-11 Phase 1 (`03ac24d8ba`) landed there with #3028
(`11616e2a3e`, 2026-09-21), and `8ee3daf171` is a `main` merge. What
landed on `props/sign-hull` reached them only at #2468 (2026-10-01),
after the window. The candidates were the 107 first-parent commits in
`11616e2a3e..8ee3daf171` that change a `crates/*/src/*.rs` file;
commits that change no library source cannot move a receipt. One
(`1b744533a2`, #3185) does not compile (`topo`'s `RingContact` match
is non-exhaustive), so its neighbour `dbf928c841` was taken instead.

| commit | past the ceiling: link / bracket / pad | pad rule F off / on |
| --- | --- | --- |
| `11616e2a3e` #3028 (window opens) | `[214,76,175,556]` / `[428,141,341,1096]` / `[854,128,971,2750]` | `(858,104,991,2750)` / `(854,128,971,2750)` |
| `eceba7e6ef` (#3257's parent) | as above | as above |
| `31c764e9fe` #3257 | `[214,76,179,556]` / `[428,141,343,1096]` / `[854,128,979,2750]` | `(858,104,999,2750)` / `(854,128,979,2750)` |
| `60f6b2aab7` (#3270's parent) | as #3257 | as #3257 |
| `f1b33a3cf6` #3270 | as #3257, pad `[854,128,979,2722]` | `(858,104,999,2722)` / `(854,128,979,2722)` |
| `8ee3daf171` (window closes) | as #3270 | as #3270 |

The plate (`[803,140,470,1044]`) and the annulus (`[328,140,209,1056]`)
read the same at every commit taken. `11616e2a3e` reads exactly what
SYM-11 stored and `8ee3daf171` exactly what ENCL measured, so the
window holds two moves and nothing else.

### Move 1: `numeric` +8 pad, +4 link, +2 bracket — #3257 (`31c764e9fe`), right

`band/extrude-carries-declared-cusps`: validation now asks
`path_junction_side` of every declared tangent joint, to record which
of them are cusps (`profile::ValidatedLoop::cusp_joints`), so that the
sweep verbs can carry a `Tangent` contact for each one.

The per-predicate split, taken at #3257 and at its parent with a
scratch probe (`m10_8_harness::split` over the shape-report replay at
`refuses_at`, shipped set), differs on the link and the bracket in one
row only. That row is new: `path_junction_side [0, 0, 0, 4]` on the
link and `[0, 0, 0, 2]` on the bracket. Every other predicate's
`[theorem, sign_gated, registered, numeric]` is identical. The pad's
split cannot be taken on this box: the shape report on the pad was
OOM-killed at the 14 GB memory limit
(`the-pads-nominal-replay-is-not-takeable-on-a-four-core-box` has the
same wall). Its +8 is the same predicate by count: the pad is four
straight legs and four fillets, so it has eight declared tangent
joints, one decision each. Nothing else in its receipt moves.

**Right.** At a fillet joint the leaving heading continues the arriving
one, so the margin `cos φ · arm` is the arm itself: definitely
positive, and nothing the tier can answer, since the tier answers only
Zero. A new decision of that kind belongs in `numeric`, and the move
follows from what the PR set out to do.

### Move 2: `frozen` 2750 → 2722 on the pad — #3270 (`f1b33a3cf6`), right

`encl/tangent-parallel-transverse-arc`. Its only non-comment change to
library source is two lines of `geom_brep::tangent::tangent_jet`. Each
curvature used to divide by its gradient dotted with the first unit
normal, `∇Fᵢ · n̂`. It now divides by the gradient's own norm (`n1`,
`n2`), with `copysign(1, ∇F₂ · n̂)` carrying the orientation, so that a
right-angle crossing can no longer certify as a tangency.

No decision column moves on any of the five documents, or at either
rule-F dial. What moves is the set of nodes the tier could not build a
form for: 28 fewer of the pad's freeze. `κ₁` now divides by `n1`, a
node the jet had already built for `n̂`, where it used to divide by the
compound `∇F₁ · (∇F₁ / n1)`.

**Right.** No decision changed class, and the receipt reads the
re-spelled jet as fewer frozen nodes, which follows from the change.
The link and the bracket do not move: past the ceiling their replays
stop at the extrude's attachment gate (`carrier_matches_mapped_source`)
before any tangency is read.

Neither move is a defect, so the stop rule does not apply to the
window.
