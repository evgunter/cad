---
id: meet-reads-two-conics-tangent-at-a-shared-end-in-one-order-only
kind: issue
title: meet reads two conics tangent at a shared end in one order only: across the near-circle's plane its window cannot certify the touch
status: open
opened: 2026-11-04
---


Found by `onecopy/p0-needle` (the band-turn needle's 1e-12 red). The pair is pinned in `geom-brep` `curve_pair::order_rows::conics_tangent_at_a_shared_end_tie_the_across_choice`: the needle's two rim edges at 3°, a long ellipse (semi-axes 2.1357 and 0.1) and a near-circle (radius 0.1), meeting at a shared end with one tangent. The band is 1e-12/1e-11.

## What

Each conic lies tangent to the other's plane at the shared vertex, so both of `pose.rs` `conic_conic`'s across scores are zero but for rounding. The across choice (F33) now reads that as the tie it is and reads the first conic. The two orders read differently:

- **`meet(long, round)`**, read across the circle's plane: decides one meeting, at both spans' ends.
- **`meet(round, long)`**, read across the long ellipse's plane: refuses `Unsettled::Meter`, with the end meeting in `found`.

The refusal comes from the window, then the polish:

1. The window about the near-circle's touch root (`pose/window.rs` `Window::read`) reads 40-odd pieces on `curve_pair_window_on` in the band's gap, at margins 1.0e-12 to 6.4e-12.
2. The circle bends out of the ellipse's plane at about 10 m⁻¹. The separation between the curves is the ellipse's in-plane bend at its vertex, 213 m⁻¹. So the height `H` stays under the zero band while `d` crosses the gap, and the pieces read neither off (on `H`) nor receding (on the level `F`).
3. The root then goes to the polish. Its basin reads the tangents' sine as 6.3e-15 against a turn of 1.3e-3, and refuses.

The other order's height carries the separation, so it reads off or receding there.

Before PR 6492 the first conic was always read, so `meet(round, long)` refused there too. The tie now reproduces that. This is not a regression, but it is an asymmetry `order_rows::agree` would count as one: one order decides, the other refuses.

## Owed

`meet` should decide two conics tangent at a shared end in either order. Two possible homes:

- the window reads recession on `d` where `H` is flat and `F` carries the separation;
- the across choice breaks a tie by an order-free second-order key, e.g. the height's bend `A/|C′|²` at the touch, which would read the long ellipse here.

The second is a design choice (F33's rule), so it goes to designers. Tangent-continuous conic edges at a vertex are common (fillet joints), so this pose is not rare.
