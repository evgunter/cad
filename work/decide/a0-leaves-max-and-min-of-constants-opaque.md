---
id: a0-leaves-max-and-min-of-constants-opaque
kind: issue
title: rule A0 folds sqrt and abs of a constant but not max or min of two constants, so an axis-aligned sign-hull frame stays an atom chain under A0 and its gate residual freezes
status: open
opened: 2026-09-21
priority: P1
cost: D
---


## What was measured (SYM-10 Phase 1, 2026-09-21)

PR #2468's fix 13 taught rule A0 to read a `Select` whose decision is
a constant. The frame it then selects is still an atom chain: at
`n = (0, 0, 1)` the read arm is `cy = vy / max(‖vy‖, s/4)` with
`‖vy‖ = sqrt(1) → 1` (A0), `s = min(‖n‖, max|n_i|) = min(1, max(0, 1))`
— and `max(0, 1)`, `min(1, ·)`, `max(1, ·/4)` are `max`/`min` of two
CONSTANT forms, which `combine` folds only when both are the zero form.
So `u = 1 / max(1, min(1, max(0, 1))/4)`, three opaque atoms standing
for the number one, multiplied into every identity over the derived
frame.

On `m10_derived_frame_interval`'s height document at `5e-2` under A0
replacing (`SymRules { const_fold: true, ..none() }`, the row
`m10_the_derived_frames_refusal_is_not_a_freeze`'s second rung): the
attachment gate's `carrier_endpoint_start` residual is `sqrt` over
three FROZEN kids, frozen 10, and refuses numerically at `[0, 0.32]`
where the row asserts frozen 0 and the newell `Invalid`. With
`max(A, B) → A` on a manifestly non-negative `A − B` hand-planted in
the plain walk (`CAD_SYM10_PLANT=1p` on the branch's `2a479c267`), the
rung reads frozen 0, one refusal, `newell_plane_residual … margin is
invalid` — the assertion verbatim. On constants that fold is A0's:
`max(c₁, c₂)` and `min(c₁, c₂)` of two exact rationals fold to the
exact rational, no value read, wherever A0 runs (replacing in the
plain walk, alongside in the early one).

## What a fix is

Extend A0's constant fold in `combine`'s `Atan2 | Min | Max | Copysign`
arm: when `node.op` is `Min`/`Max` and both kids are constant forms,
return the constant. Pin it beside `a_constant_decision_folds_to_the_arm_it_reads`.
Small; SYM-10's Phase 2 or its own row, whichever Ev's ruling on the
fourth piece (`the-candidate-norm-needs-a-canonical-square-root`)
lands first.

## Home

`crates/geom-core/src/sym.rs` (`combine`). Filed by SYM-10's lane.

## More evidence (DECIDE-3, 2026-09-21)

The gap is now visible in three more places, and in each one a
THEOREM is being reported as a READ because A0 leaves the comparison
opaque and the decision read then answers it over the box:

- the M10-3 **slab**: eight `numeric` decisions become `sign_gated`
  under the shipped set (`m10_8_pins_interval::m10_8_the_shipped_set_is_inert_on_straight_geometry`,
  `symbolic_zero` unmoved at 482, `numeric` 263 → 255). On straight
  geometry every one of those comparisons is between two CONSTANTS;
  with this row answered they would be theorems and the slab would be
  inert again.
- the **plate**'s `line_span`, `[0, 0, 0, 8] → [0, 8, 0, 0]`
  (`m10_10_pins_interval`), and the segment boss's, `[0, 0, 0, 2] →
  [0, 2, 0, 0]` (`m10_bulge_interval`).
- the derived-frame row's **A0 rung**: A0 alone still freezes ten on
  the height document — the products under `1/max(1, min(1, max(0,
  1))/4)` — and stops at the gate's `carrier_endpoint_start` one step
  before the plain tier's clause-1 margin
  (`m10_derived_frame_interval::m10_the_derived_frames_refusal_is_not_a_freeze`,
  re-aimed by DECIDE-3 with this row as the reason).

So the fold is worth its own unit: it is value-free, it belongs in
A0's own class (an exact comparison of two rationals), and it converts
reads back into theorems, which is the one direction the receipt is
allowed to move in.

## ANSWERED by DECIDE-3's fix pass (2026-09-22)

Ev, 02:05Z on #3039: *"make sure that the code that goes in is clean,
and doesn't have any concession towards skipping a rebaseline"*. The
concession was on this row: DECIDE-3's first fix pass made the decision
READ decline a comparison of two rational constants so that the slab's
byte-identity pin could stand. The change for the better is the other
way round, and it is this row's fix — **A0 now decides a `max`/`min`
of two rational constants exactly, and a `max`/`min` whose two
arguments are ONE form**, both value-free, both counted
`symbolic_zero`. The read declines nothing and no longer consults
`Session::params` to decide whether it may answer.

What that yielded, every number measured on this branch:

| row | before | after |
| --- | --- | --- |
| M10-3 slab (`m10_8_the_shipped_set_is_inert_on_straight_geometry`) | `symbolic_zero 482, numeric 263` | **`490 / 255`, nothing gated**; with the form-level algebra off it is still `none()` bit for bit |
| plate, `line_span` at the nominal | `[0, 0, 0, 8]` | **`[8, 0, 0, 0]`** |
| plate study (`m10_9`), `symbolic_zero` | 803 | **811** (`numeric` 470 → 462) |
| bracket study (`m10_9`), `symbolic_zero` | 1098 | **1104** (six more, `registered` unmoved) |
| D-tab, `line_span`, literal and parameter | `[0, 0, 0, 8]` | **`[4, 0, 0, 4]`** — four are constants, four carry a parameter |
| segment boss, `line_span` | `[0, 0, 0, 2]` | **`[2, 0, 0, 0]`** |
| derived-frame row's A0 rung | frozen **10**, stopping at the gate's `carrier_endpoint_start` | frozen **0**, reaching the same clause-1 `newell_plane_residual` the plain tier does — the row's original claim, restored by the fold rather than re-aimed |
| slab and plate walk ledgers | `*/Report` rows at 16 and 8 calls | **absent** — nothing left for the shape report to report |
| `min(x, x) − x`, `max(x, x) − x` at the scalar door | opaque, or `sign_gated` through the read | **theorems** (`m10_7_r1_sym_probes::r1_min_and_max_of_one_form_are_that_form`) |

The rows that carried this row's evidence are re-aimed to the fold:
`m10_7_r1_sym_probes`' conservative list, `m10_7_r2_sym_probes`' kink
row, and `geom-core/tests/sym_root_rows::a_comparison_of_two_constants_is_a_theorem`,
which names A0 as the dial that decides it.

**This row closes with DECIDE-3's merge.** What it does NOT cover, and
what stays open elsewhere: `max(A, B)` for two distinct non-constant
forms is still the decision read's, and that is right — it is a fact
about the box, not about the form.
