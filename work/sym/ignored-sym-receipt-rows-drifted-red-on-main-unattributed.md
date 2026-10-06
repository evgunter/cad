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

### The re-take at `main` (`07ddd620a2`), three ε

The lane head is `07ddd620a2`, plus this item's text. Release build,
both rows, the past-the-ceiling row's receipt assertion disabled so
that all five documents print. As `[symbolic_zero, registered,
numeric, frozen]`, the stored row's own columns:

| row | document / dial | stored | ε = 1e-6 | ε = 1e-9 | ε = 1e-12 |
| --- | --- | --- | --- | --- | --- |
| `PAST_THE_CEILING` | two_hole_plate | `[803, 140, 470, 1044]` | `[955, 148, 704, 1068]` | same | same |
| ″ | r1_annulus | `[328, 140, 209, 1056]` | `[440, 148, 451, 1080]` | same | same |
| ″ | r2_link | `[214, 76, 175, 556]` | `[286, 84, 296, 568]` | same | same |
| ″ | r2_filleted_bracket | `[429, 141, 342, 1096]` | `[548, 149, 574, 1113]` | same | same |
| ″ | r2_rounded_pad | `[885, 128, 1058, 2750]` | `[1016, 152, 1186, 2587]` | same | same |
| `m10_9_the_pad_at_both_rule_f_dials` (`symbolic_zero, sign_gated, registered, numeric, frozen`) | rule F shut | `(1036, 2, 152, 1188, 2587)` | `(1016, 2, 152, 1186, 2587)` | `(1040, 2, 152, 1194, 2587)` | `(1016, 2, 152, 1186, 2587)` |
| ″ | shipped | equal to rule F shut | equal | equal | equal |

`sign_gated`, which the past-the-ceiling row does not pin, reads 0, 32,
0, 37 and 2 down the five documents at every ε. Past the ceiling, the
link and the bracket refuse at the extrude's attachment gate
(`carrier_matches_mapped_source`), as they did in the window, and the
pad refuses at `pcurve_envelope` (since #3759). The plate and the
annulus replay whole. `registrations_contradicted` and
`theorems_disputed` are 0 on every document at every ε, so no
contradiction fired.

**The receipts past the ceiling are still ε-independent. The pad's
rule-F row is not.** At its certifying scale the pad refuses at
`pcurve_envelope` under both dials, and the replay stops after a
longer prefix at 1e-9 than at 1e-6 or 1e-12. `measured_studies` pins
that dependence per ε row since #3759 (`Study::symbolic_zero`,
`[1016, 1040, 1016]`). The rule-F row's single tuple holds at one ε
only. The dials still agree at every ε, which is the row's claim.

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
The link's and the bracket's `frozen` do not move. Past the ceiling
both replays stop at the extrude's attachment gate
(`carrier_matches_mapped_source`), so their receipts are a truncated
replay's. Whether the jet is reached before that stop was not
measured.

Neither move is a defect, so the stop rule does not apply to the
window.

### Since the window: `8ee3daf171` to `main` at `07ddd620a2`

The same script was run along `main`'s first parent. It was taken at
the parent and at the merge of every PR that re-pinned either row or
`measured_studies`, and at the head. Each stretch between those
points whose ends disagreed was bisected over its commits that change
a `crates/*/src/*.rs` file. Past-the-ceiling receipts here are
`[symbolic_zero, sign_gated, registered, numeric, frozen]`, because
`sign_gated` stops being 0 at #2468. The pad's rule-F row reads the
same at both dials from #3254 on, so one tuple stands for both.

Every move, at ε = 1e-9, with the merge it is bisected to. The parent
of each merge reads the row before it.

| merge | plate | annulus | link | bracket | pad, past the ceiling | pad rule-F row | credited where |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `8ee3daf171` (start) | `[803,0,140,470,1044]` | `[328,0,140,209,1056]` | `[214,0,76,179,556]` | `[428,0,141,343,1096]` | `[854,0,128,979,2722]` | off `(858,0,104,999,2722)`, on `(854,0,128,979,2722)` | — |
| #3313 `387e2e1447` must-carry gate | — | — | — | — | sz +28, num +84 | sz +28, num +84 both dials | both rows (ENCL) |
| #3254 `b5b7dcc342` arc span from the stored sweep | — | — | reg +9, num −9 | — | frozen −130 | frozen −130; rule F off now reads rule F on's receipt | `Study` at `certifies_at` (link) |
| #3266 `1bdbdeb726` run-out carrier | — | — | — | sz +1, num +1 | sz +3, num +3 | sz +3, num +3 | both rows |
| #3455 `6552c509ad` step-id digest chain | — | — | — | — | frozen −15 | frozen −15 | **not credited** |
| #3612 `fefdc50aba` the schedule assigns its ends | — | — | — | frozen −3 | frozen −48 | frozen −48 | **not credited** |
| #3645 `993456137f` one `mid_point` | frozen +96 | frozen +96 | frozen +48 | frozen +96 | frozen +168 | frozen +168 | **not credited** |
| #2468 `dc39bce95d` `props/sign-hull` | sz +8, num −8 | — | reg −5, num +5 | sz +7, sg +5, num −12 | sz +8, sg +34, reg +22, num −64 | the same | `Study` (`7e6371fbae`); rule-F row (`304e61c6a6`) |
| #3697 `1346a31745` the schedule's middle is `mid_param` | frozen −96 | frozen −96 | frozen −48 | frozen −96 | frozen −168 | frozen −168 | **not credited** |
| #3594 `c28f9aba17` node-id digest mint | — | — | — | — | frozen +15 | frozen +15 | **not credited** |
| #3527 `44b64db0ba` copied arc carriers | — | — | — | frozen −4 | reg −2, num +2, frozen −34 | the same | `Study` (`registered`, `numeric`) |
| #3807 `c983e6e299` DECIDE-9 | — | — | — | — | sz +32, sg −32 | the same | `Study`, rule-F row |
| #3759 `82b52c36cb` PCERT | sz +136, reg +8, num +236, frozen +24 | sz +112, sg +24, reg +8, num +236, frozen +24 | sz +68, reg +4, num +118, frozen +12 | sz +112, sg +24, reg +8, num +236, frozen +24 | sz +87, reg +4, num +176, frozen +77; refuses at `pcurve_envelope` | `(1036,2,152,1188,2587)`, refuses at `pcurve_envelope` | `Study`, rule-F row |
| #3981 `6d1cef94cf` check 5's escape | sz +8, num −8 | sg +8, num −8 | sz +4, num −4 | sg +8, num −8 | sz +4, num −4 | sz +4, num −4 | `Study` (plate, link, pad) |
| #4037 `201239b0e4` joint elements | num +14 | num +14 | num +7 | num +14 | num +10 | num +10 | **not credited** |

Every other stretch reads the same at both ends. #4011 (INTENT-VARS-1
PR 2) moves nothing past the ceiling. The parents taken, in table
order: `453faa7d18`, `69b0ce3b4a`, `65f395e88e`, `40bfe407bf`,
`0eccbbff6b`, `c191268379`, `f497cf22eb`, `81a70cb612`, `8247568db8`,
`a049d2c5ed`, `1c1c23be85`, `0612478485`, `675fdccc69`, and `d01d8ae1fe`
for #4037. #3804 (SYM-15, `810bc55336`) reads #3807's values.

**At the head (`07ddd620a2`), ε = 1e-9:** plate `[955,0,148,704,1068]`,
annulus `[440,32,148,451,1080]`, link `[286,0,84,296,568]`, bracket
`[548,37,149,574,1113]`, pad `[1016,2,152,1186,2587]`. The pad's
rule-F row reads `(1040,2,152,1194,2587)` at both dials. The stored
`PAST_THE_CEILING` carries only #3313's and #3266's deltas. The rule-F
row was last re-pinned by #3759 (PCERT), so #3981's and #4037's
deltas, `symbolic_zero` +4 and `numeric` +6, leave it red.

#### The moves no PR credited

- **#4037 (`numeric` +14 / +14 / +7 / +14 / +10): right.** This merge
  makes "the closure joint decided as every joint" (its branch log):
  a loop's closing joint is read by the same walk as every other
  joint. The per-predicate split at the parent and at the merge, taken
  with the scratch probe on the four documents whose shape report fits
  in this box's memory, moves only the loop walk. On the plate:
  - `pcurve_loop_closure` `[0,0,0,4]` and `pcurve_loop_closure_height`
    `[4,0,0,0]` are gone;
  - `pcurve_loop_continuity` `[12,0,8,4]` → `[16,0,8,8]`;
  - `pcurve_loop_pole_joint` `[0,0,0,12]` → `[0,0,0,16]`;
  - `pcurve_loop_branch` `[0,0,0,24]` → `[0,0,0,34]`.

  The annulus and the bracket move identically, and the link moves by
  half. Theorems are unchanged in count, since the closing joint's
  height theorem becomes a continuity theorem, and no decision leaves
  a discharge column. The closing joint now asks the questions every
  joint asks, and those answers are definite margins, so they land in
  `numeric`.
- **#3645 then #3697 (`frozen` up 96 / 96 / 48 / 96 / 168, then down by
  exactly as much): right, both.** #3645 spelled each edge's witness as
  the carrier's `mid_point(ta, tb)`. The certification schedule's
  middle station was still `t0 + (t1 − t0)·0.5`, so the same point was
  built twice and the second copy froze as well. #3697 made the
  schedule's middle station `mid_param` ("a certificate evaluates an
  edge's middle once"), and the duplicates went. No decision moves at
  either merge.
- **#3612 (`frozen` −48 pad, −3 bracket): right.** The schedule now
  assigns its last sample `t1` itself rather than `t0 + (t1 − t0)·1`,
  so the end sample is the endpoint's node and not a second one. No
  decision moves.
- **#3455 (−15) and #3594 (+15), the pad's `frozen` only: the merges
  are right; the receipt's dependence on them is a finding.** Both
  change how ids are minted and no arithmetic, yet the pad's frozen
  set moves at both. Filed on SYM's slate as
  `the-pads-frozen-set-moves-with-the-documents-id-mint`.
- **#3254's `frozen` −130 on the pad and its rule-F convergence:
  right.** The carrier's span became the stored sweep signed by the
  decided turn instead of `4·atan|b|`, so the `|b|` atom is gone. With
  rule F shut, the pad now reads what rule F used to give it
  (`symbolic_zero` −4, `registered` +24, `numeric` −20). With rule F on
  no decision moves. Both are consistent with rule F having no `abs`
  left to open, and with fewer compounds over the old span to freeze.
  Neither was isolated further. On `main` this, not the axis-order basis that
  #2468 brought, is where rule F stopped moving anything on the pad.
  The evidence is added to `the-negative-arm-lost-its-document-consumer`.

#### The moves a PR credited

Each was credited at `certifies_at` in `measured_studies`, or in the
rule-F row, and argued there. Past the ceiling each moves the same
predicates in the same direction:
- #3313: the must-carry stations' new `dihedral_wedge` decisions;
- #3254: the link's span sharing the pushforward's atom;
- #3266: the run-outs proved zero;
- #2468: DECIDE-3's fold, rule G and the decision read, then the
  read's pre-emption of 32 theorems;
- #3527: copied carriers;
- #3807: DECIDE-9 returns those 32 to theorems;
- #3759: the pcurve mint's new checks, with the bracket and the pad
  now refusing at `pcurve_envelope`;
- #3981: check 5's escape.

The calls on these stand as those PRs made them. One stored value was
not the merged tree's: #2468 stored the rule-F row's `frozen` as 2577,
where its merge reads 2697. The difference is #3612's −48 and #3645's
+168 taken together. Two numbers differ
from the credited ones only because the scale differs:
- at #2468 the link's `registered` falls by 5 past the ceiling, where
  `certifies_at` falls by 2 ("main's arc_span trade does not show with
  rule G in", `7e6371fbae`);
- at #3981 the annulus and the bracket gain 8 `sign_gated` past the
  ceiling, where `certifies_at` is unmoved.

**No move since the window is a defect in code SYM does not own**, so
the stop rule does not apply. The one finding is on SYM's own slate.
