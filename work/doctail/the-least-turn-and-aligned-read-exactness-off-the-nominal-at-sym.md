---
id: the-least-turn-and-aligned-read-exactness-off-the-nominal-at-sym
kind: issue
title: At Sym the least turn and the chart's aligned axis read exactness off the nominal, so a zero-tilt chart is constant in the symbols
status: closed
closed: 2026-11-22
opened: 2026-11-22
priority: P1
cost: M
---


Found by PR 7396's review: with a chart's spin set from its frames at `Sym`, the door proved a false tangency at a zero-tilt nominal. This is the 7396-blocking subset of `decide/is-exact-zero-reads-tags-copies-and-decisions-off-a-number`.

## What

At `Sym`, three reads chose what is built by reading "exactly this value" off the nominal (DESIGN `[scalars]`: "a lane's choice is the same across the box, at a `Sym` over the unknowns' whole range"; ERROR-DESIGN E12's structural question):

- **`RigidMotion::least_turn`'s identity shortcut** (`least_turn_minted`, `a.is_exactly(b)`). `UnitVec3::is_exactly` read `Real::is_exact_zero_in_every_channel`, which `Sym` forwarded to its value. At a tilt variable's zero, a face's normal is the space's `z` in value alone. The turn came back as `RigidMotion::world()`, constant in the symbols.
- **`Chart::charted`'s copy of it** (`from.is_exactly(d)`): it skipped the renormalization.
- **`mate::turn::zero`**: `exactly_zero() || (quoted() == 0 && upper() == 0)`, the value channel at `Sym` and at `Dual`. It feeds `aligned`, and through `aligned` it feeds `seated`, `spans`, `across`, `slide_direction` and `settle`'s frames-lane translation. `SolveScalar::exactly_zero` at `Sym` forwards to its value: false at `Sym<f64>`, the box's point zero at `Sym<Interval>`. Its readers (`on_an_axis`, `box_atan2`, `turn_to`'s measured zero) take it only to tighten a reading, so it is left as it is.

**Repro (on 7396's branch):** `intent_s6_e_traced_on_a_face::a_boss_traced_across_two_cylinders_coaxial_only_at_zero_tilt_is_not_proved`. There the door answered `Structural(MarginIdentity)`; the same document at `a = 0.2` has no tangency.

**Main.** Main's measured-spin spelling still read the constant least turn, for the spin's reference and for the carried slide axes. It proved nothing false in the shapes tried, though: a boss slid flush onto a face at the slide's own variable left every row `Unproven … Budget`. So main was fail-closed only by the budget.

## Closed

- **The least turn reads no exactness.** `RigidMotion::least_turn_minted` has no identity shortcut. Its Rodrigues arm is `v + k × v + k × (k × v)/(1 + c)`, so a pair that is one direction turns by the identity through `k = 0`: bit for bit at `f64`, as a form at `Sym`, with the tangent flowing at `Dual`. Such a pair now takes the pole verdict and the conditioning like any other.
- **The exactness readers are gone or `f64`-only.** `Real::is_exact_zero_in_every_channel` is removed. `UnitVec3::is_exactly` is `f64`-only; the sweep skin's still step is its one caller.
- **`Chart::charted`** skips renormalizing only where the pair is one direction at every point the scalar stands for.
- **`mate::turn::zero`** reads `SolveScalar::zero_everywhere`, answered per lane:
  - `f64`: its zero;
  - `Interval`: `[0, 0]`;
  - `Dual`: value and derivative both zero;
  - `Sym<Interval>`: the box's point zero;
  - `Sym<f64>`: never.
- **Rows:**
  - `intent_s6_e_traced_on_a_face::a_boss_traced_across_two_cylinders_coaxial_only_at_zero_tilt_is_not_proved` (the reviewer's scene);
  - `revalue_corpus::revalue_a_chart_on_a_cap_tilted_at_zero`;
  - `mate::turn::tests::a_normal_on_an_axis_in_value_alone_is_not_aligned`.
- **Filed:** `a-mate-charts-zero-decision-builds-on-a-coincidence-the-door-never-sees`.
