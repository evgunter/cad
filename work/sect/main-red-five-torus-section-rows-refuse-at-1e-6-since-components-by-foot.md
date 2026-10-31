---
id: main-red-five-torus-section-rows-refuse-at-1e-6-since-components-by-foot
kind: issue
title: main red at ε 1e-6: five torus section and germ rows refuse (Winds(Unseparated), a zero-margin tube straddle, an indeterminate carrier straddle) since PR 6011
status: closed
closed: 2026-10-31
opened: 2026-10-30
priority: P1
cost: M
refs: [a-counted-sections-winding-is-a-claim-not-a-reading]
---


**What fails.** Five rows are red at ε 1e-6 only, in nightly [41008276467](https://github.com/evgunter/cad/actions/runs/41008276467) (main `39bbdfc777`). They pass at default ε and 1e-12.

| row | message |
|---|---|
| `geom-brep` `pair_section::wiring::close_rings_carry_counted_and_marched` (`crates/geom-brep/src/pair_section.rs:1867`) | `δ 3e-4, counted: March(TubeStraddles { verdict: Zero(Classified { margin: 0.0, band: 1e-6 }), boxes: 206 })` |
| `geom-brep::all` `pair_section::two_loops_keep_their_sites_and_order_near_the_node` (`crates/geom-brep/tests/pair_section.rs:819`) | `d 0.2, R 0.701, θ 2.784…: Escalated(Indeterminate { … predicate: curve_carrier_straddle })` |
| `geom-brep::all` `torus_doors_ssi::the_germ_rows_donut_against_its_moved_drum` (`crates/geom-brep/tests/torus_doors_ssi.rs:324`) | `placement 5: Winds(Unseparated)` |
| `sweep::all` `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs` (`crates/sweep/tests/germ_tilted_rod.rs:372`) | every β/spin refuses `GermSectionUnbuilt { Torus × Cylinder, why: Winds(Unseparated) }` |
| `sweep::all` `torus_doors_join::a_square_rod_on_the_outer_equator_stops_at_the_join` (`crates/sweep/tests/torus_doors_join.rs:116`) | `an unpinned stop: Err(GermSectionUnbuilt { Torus × Cylinder, why: Winds(Unseparated) })` |

**Attribution (bisected).** These run at ε 1e-6 with `cargo nextest run -p geom-brep -p sweep --run-ignored all`:

- At `a0f3096e^1` (`b56d8802`), the four that exist all pass. `close_rings_carry_counted_and_marched` does not exist there yet.
- At `a0f3096e`, the merge of PR 6011 (`sect/components-by-foot`), all five fail.
- At `a0f3096e^2`, the PR's own head, all five fail as well. So the failure is the PR's own change, not a merge collision.
- At `c59d2622`, the four that exist pass.

Four of the five ask a germ section that PR 6011's march now refuses `Winds(Unseparated)`, or decides on a zero margin. That is the counted-winding cross-check PR 6011 filed as `a-counted-sections-winding-is-a-claim-not-a-reading`, read at 1e-6's band.

**Reproduce.** `CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p geom-brep -p sweep --run-ignored all -E 'test(=pair_section::wiring::close_rings_carry_counted_and_marched) | test(=pair_section::two_loops_keep_their_sites_and_order_near_the_node) | test(=torus_doors_ssi::the_germ_rows_donut_against_its_moved_drum) | test(=torus_doors_join::a_square_rod_on_the_outer_equator_stops_at_the_join) | test(=germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs)'`

**What closing it takes.** Decide, per row, whether the 1e-6 refusal is the march being honest at that band or the winding reading failing to separate what it should. Then either fix the reading, or pin each row's 1e-6 stop as typed with the reason. All five are green at 1e-6.

## 2026-10-30 (GREEN, lane `green/nightly-77-pins`)

Pinned at 1e-6 only (`pinned_red::row`, `stops_at = [1e-6]`), each at main's stop on `2e1c1bff`. At default ε and 1e-12 they run as written, and pass:

- `close_rings_carry_counted_and_marched`: `δ 3e-4, counted: March(TubeStraddles { verdict: Zero(Classified { margin: …(0.0` … `boxes: 206 })`. δ 3e-4 is the last δ, so nothing runs past it.
- `two_loops_keep_their_sites_and_order_near_the_node`: `d 0.2, R 0.701, θ 2.7842143000352855: Escalated(Indeterminate {` … `curve_carrier_straddle`. The remaining θ of that sheet, the other sheet and the swapped order of that last pose are unrun.
- `the_germ_rows_donut_against_its_moved_drum`: `placement 5: Winds(Unseparated)`. 5 is the row's last placement, so only what `germ_row` checks after its placements is unrun.
- `a_square_rod_on_the_outer_equator_stops_at_the_join`: `an unpinned stop: Err(GermSectionUnbuilt {` Torus × Cylinder `why: Winds(Unseparated) })`.
- `other_spins_and_steeper_tilts_refuse_at_their_germs`: all eight poses, from β 0.5 spin 0 to β 0.7 spin π/2, `GermSectionUnbuilt { … Winds(Unseparated) }`.

Un-pinning owes a 1e-6 re-measure of the unrun remainders named above.

## Closed

2026-10-31, by `sect/torus-rows-at-1e-6` (lane `sect-onesix`).

**Bisected inside PR 6011.** At ε 1e-6 and opt-level 2, the four rows
that exist pass at `5d84e965` (the confirmation review's head) and at
`3e79f199` (its merge of main). All five fail at `b568f3ef8c`, whose
own commit grew the marched loop's pad from the tube radius to the
radius plus √3 times the largest raw hull box side. At 1e-6 the widest
rung that certifies is wide: 4.4e-2 m on the cylinder × sphere poses.
So the pad there was 0.11 to 0.13 m. `winds` read every halved piece's
hull padded by that much, and on the torus rows those reached the axis
or the core circle (`Unseparated`).

**What the reading owed.** The winding needs no closeness bound. Limb
3 proves the section's points in the tube one arc, with a piece in
every box of the chain. The component and the carrier therefore run
through the same convex boxes in the same order, and where no box
reaches a centre they wind alike. `winds` now reads the loop over its
tube boxes (`ssi::certify::tube_chain`), each cleared of the centre
with nothing added. It no longer halves, since halving a box cannot
narrow where the section lies inside it.

The pad stays the closeness bound for `reads`. It is now the farthest
point of any tube box from the carrier, bounded slab by slab
(`Tube::of`), and no longer the box diameter. That bound also takes in
the box's cubic padding, which the old `radius +` term read as
spherical. W3 had read the pad as the witness's distance from the
component, which closeness does not give. It now reads its own number,
`CarriedComponent::reach`: the diameter of a tube box that holds both
the witness and a piece of the component.

**Per row, at 1e-6:**

- `the_germ_rows_donut_against_its_moved_drum`: passes, every
  placement, including what `germ_row` checks after them.
- `two_loops_keep_their_sites_and_order_near_the_node`: passes, every θ
  of both sheets in both orders. Its pads read about 0.10 m, so there
  is little room left. The tighter reading is filed as
  `a-point-is-read-on-a-marched-loop-by-distance-not-by-its-tube`.
- `a_square_rod_on_the_outer_equator_stops_at_the_join` and
  `other_spins_and_steeper_tilts_refuse_at_their_germs`: their germ
  sections carry. They then stop exactly where main stops them at the
  default ε and 1e-12, which is GREEN's open
  `main-red-eighteen-sweep-analytic-rows-red-on-main-at-the-default-eps`
  (evidence added there). The pins are gone, and the rows read alike at
  all three ε.
- `close_rings_carry_counted_and_marched`: still pinned red at 1e-6, now
  citing `ssimarch/an-r3-tube-no-rung-makes-a-graph-is-never-refined`.
  At δ 3e-4 every rung down to the floor reads graph margin 0, so the
  stop is the march's sample spacing and not the reading.

Also filed from the sweep: evidence on
`sectrung/the-torus-gaps-rounding-charge-is-first-order-and-reads-the-world-origin`
(a wrong `Tangent` verdict at 1e-6, seed `0xb8c56b2ac7806cae`).

