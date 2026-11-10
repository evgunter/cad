---
id: meet-reads-two-conics-tangent-at-a-shared-end-in-one-order-only
kind: issue
title: meet reads two conics tangent at a shared end in one order only: across the near-circle's plane its window cannot certify the touch
status: closed
closed: 2026-11-04
pr: 6525
opened: 2026-11-04
---


Found by `onecopy/p0-needle` (the band-turn needle's 1e-12 red). The pair is pinned in `geom-brep` `curve_pair::order_rows::conics_tangent_at_a_shared_end_meet_once_in_either_order` (first pinned, on PR 6525's first head, as `conics_tangent_at_a_shared_end_tie_the_across_choice`, since removed): the needle's two rim edges at 3°, a long ellipse (semi-axes 2.1357 and 0.1) and a near-circle (radius 0.1), meeting at a shared end with one tangent. The band is 1e-12/1e-11.

## What

Each conic lies tangent to the other's plane at the shared vertex, so both of `pose.rs` `conic_conic`'s across scores are zero but for rounding. As filed, on PR 6525's first head, the across choice (F33) charged the scores, read that as a tie, and read the first conic; the decision below reverted that to main's bare difference. Under that first head the two orders read differently:

- **`meet(long, round)`**, read across the circle's plane: decides one meeting, at both spans' ends.
- **`meet(round, long)`**, read across the long ellipse's plane: refuses `Unsettled::Meter`, with the end meeting in `found`.

The refusal comes from the window, then the polish:

1. The window about the near-circle's touch root (`pose/window.rs` `Window::read`) reads 40-odd pieces on `curve_pair_window_on` in the band's gap, at margins 1.0e-12 to 6.4e-12.
2. The circle bends out of the ellipse's plane at about 10 m⁻¹. The separation between the curves is the ellipse's in-plane bend at its vertex, 213 m⁻¹. So the height `H` stays under the zero band while `d` crosses the gap, and the pieces read neither off (on `H`) nor receding (on the level `F`).
3. The root then goes to the polish. Its basin reads the tangents' sine as 6.3e-15 against a turn of 1.3e-3, and refuses.

The other order's height carries the separation, so it reads off or receding there.

This is a regression against main's parent before PR 6492 (`fd1c1949a5`), in the order the census reads:

- **On `fd1c1949a5`** the first conic was always read. `meet(long, round)` read the long ellipse across the circle's plane and decided; the census passed. `meet(round, long)` read the circle, the reading that refuses.
- **On main `c70b3bb82`**, measured: PR 6492's bare difference reads the circle in both orders, so both refuse `Unsettled::Meter`.
- **This PR's first head** charged the scores and read the first conic at the tie. That restored the parent's one-sided answer, deciding in one order and refusing in the other.
- **With the window's height and level read locally** (the decision below), both readings certify the touch, in either order.

## Owed

`meet` should decide two conics tangent at a shared end in either order. Two possible homes:

- the window reads recession on `d` where `H` is flat and `F` carries the separation;
- the across choice breaks a tie by an order-free second-order key, e.g. the height's bend `A/|C′|²` at the touch, which would read the long ellipse here.

The second is a design choice (F33's rule), so it goes to designers. Tangent-continuous conic edges at a vertex are common (fillet joints), so this pose is not rare.

## Decided (2026-11-04)

Three designers agreed in one round. No ratified text changes, so this lands without Ev in PR 6525.

- **The defect is in the window, not the across choice.** `Window::read` read `b`'s level `F` from its global harmonics, posed about `b`'s centre, charged `rounding_charge(terms)` with `terms ≈ ((|w| + R)/s)²`. At the needle, reading the near-circle across the long ellipse's plane, that charge is about 9.7e-13, the size of the band's zero, so no piece read receding. Main's bare across difference is antisymmetric bit for bit, so order-free. The charged tie of PR 6525's first head, which read the first conic, is what made `meet` order-dependent on tangent pairs.
- **What landed:**
  1. The across choice is back to main's bare `s_b − s_a`.
  2. The level is read locally at each window piece (`pose.rs` `Level`: `jet_at(θ) → [F, F′, F″, F‴]` and `fourth()`), uncharged at f64. `level_on` hands out only `Level`. `coplanar_ellipses` keeps a private `coplanar_level`: see `coplanar-ellipses-reads-the-level-from-its-global-harmonics`.
  3. The height `H` is read locally the same way (`Window::height_jet`). The decision took it to be posed this way already; it was not, and its global charge (`Window.noise`, ~7.5e-13 at coordinates of 180) left `sweep`'s leaning prisms red at 1e-12 on main. `Window.noise` is gone.
  4. The level-to-distance conversion is read locally: `D ≥ 2|F|/(g + √(g² + 12|F|/s²))`, `D ≤ 2|F|/(g + √(g² − 12|F|/s²))` on the smaller branch, and growth over `min(2/s, g + 2D/s²)` (`pose/window.rs` module docs, (ii)).
- **Rows:**
  - `order_rows::conics_tangent_at_a_shared_end_meet_once_in_either_order` (the needle);
  - `the_leaning_rims_tangent_at_a_shared_end_meet_once_in_either_order`;
  - `the_needle_spun_about_its_vertex_reads_one_answer_in_either_order`;
  - `conics_tangent_at_a_shared_end_read_one_answer_in_either_order` (the fuzz).
- **Residue, filed:**
  - `meet-refuses-a-near-tangent-tilt-in-both-orders`;
  - `a-rounding-charge-through-rounding-charge-escapes-the-bare-epsilon-gate`;
  - F33's `Tie`, on `a-heuristics-choice-has-no-door-of-its-own`.

