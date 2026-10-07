---
id: interval-steep-cut-through-cylinder-caps-refuses-order-escalated
kind: issue
title: plane_section at Interval refuses OrderEscalated (split_join_order_u, enclosure ~±7e-15) on a steep cut through a cylinder's caps that f64 answers
status: open
opened: 2026-10-02
priority: P3
cost: E
---


Seen by PR 3877's fix pass (section arcs), not chased. A steep cut
through a cylinder's caps (the first fixture tried for
`plane_section_area_of_an_uncancelled_arc_at_f64_and_interval`) answers
at f64; built and cut at `Interval`, `plane_section` refuses
`OrderEscalated` on `split_join_order_u` with an enclosure of about
±7e-15, also with the seams turned off the mirror plane. Measure first:
whether the two crossings are distinct in truth (an over-wide enclosure
on the join order's `u` key) or coincide (an honest refusal).

## Built (branch cleave/interval-join-order)

**Measured on main (`dfcd7f3504`).** The unit cylinder (`z ∈ [0, 1]`)
cut through `(0, 0, 0.5)` with normal `(sin t, 0, cos t)`, `t = 1.2`:
`f64` answers (area `2.13224086614229`, the closed form
`2(x₀√(1 − x₀²) + asin x₀)/cos t`, `x₀ = 0.5·cot t`); `Interval`
refuses `OrderEscalated` on `split_join_order_u`, enclosure
`[−6.66e−15, 6.88e−15]`. Same at `t = 1.0, 1.4` and with the seams at
0.7 rad. The pair it straddles on is the bottom cap's two crossings,
`(0.19439, ∓0.98092, 0)` — distinct in truth, `2y₀ = 1.96` apart, and
with `u` keys equal in truth (`0.5/sin t = 0.53646` for both). Not an
over-wide enclosure (it encloses a true zero, ~30 ulps wide) and not a
coincidence: the frame's `u` was the x axis projected into the plane,
`(0.362, 0, −0.932)` — perpendicular to y, so every section line along
y (a cap's chord) ties in `u`.

**Fix.** `splitting::order::in_plane_frame`: an axis plane (two normal
components exactly zero, `split_join_frame_axis` at the exact band)
keeps the exact coordinate frame; any other plane takes the first
OBLIQUE schedule member. All 24 tilted poses of the new row answer at
`Interval`, enclosing the closed form.

**What that exposed.** The new order reddened five `f64` rows (seam-ruling
splits, `axis_parallel_cuts_left_to_the_book_rule_still_answer`, the
near-tangent hole-wall row): a curved face whose section is straight
(rulings) was paired by the book's rule, which follows each ruling only
where the rulings run along the order's `v`. That was already a live
`f64` defect on main: a cylinder along x cut parallel to its axis
refused `DegenerateSection` in 6 of 12 poses where y and z answer.
`join::ruling_pairs` now pairs a straight section's crossings along each
line (`split_join_ruling`), so no face's pairing reads the sweep order.

Tests: `a_steep_cut_through_a_cylinders_caps_answers_at_f64_and_interval`,
`an_axis_parallel_cut_of_a_cylinder_along_any_axis_pairs_each_ruling`,
`a_cut_along_a_cylinders_axis_refuses_its_rims_tie_at_interval` (the
axis-plane residual, filed as
`interval-axis-plane-cut-along-a-cylinder-refuses-its-rims-tie`), and
two `splitting::order` unit rows. The steep row and the oblique unit
row go red on the old frame; the axis-parallel row goes red on main and
with `ruling_pairs` disabled (as do three seam-ruling and wrong-arc
rows). Locally: `topo`, `sweep`, `editor-core` under the `ci` profile,
7492 passed.
