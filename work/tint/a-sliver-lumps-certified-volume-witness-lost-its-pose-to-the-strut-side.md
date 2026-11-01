---
id: a-sliver-lumps-certified-volume-witness-lost-its-pose-to-the-strut-side
kind: issue
title: sliver_shell_role's certified-volume witness lost its pose: the d = 6e-9 lump now refuses in band at the strut side, so D10's volume sided by its enclosure has no row
status: open
opened: 2026-11-01
priority: P2
cost: M
refs: [face-line-germs-read-unit-in-the-strut-and-walk-orders-amplify-a-within-zero-tilt]
---


Filed by SPLITDOOR (`splitdoor/face-line-germ-lean`, PR 6238).

## What

`crates/topo/tests/sliver_shell_role.rs` held D10's certified-volume rule with `a_sliver_lump_its_certified_volume_sides_builds_in_both_orders`: at pose `vee300 nt e0 a1 d6e-9`, ε = 1e-9, the ∩'s lump reads its `V/A` in the zero band, and its certified enclosure `[1.69973e-9, 1.69973e-9]` sides it `Outer`. Its doc claimed that no pair reads in band.

PR 6238 levers every reading of a face-line germ by its lean, `|n_a × n_b|` (`boolean/sectors.rs` `leaned_lever`). At this pose the pierce ring's germ order (`vtxfac` `germ_order`, its `side` of germ 3 against germ 0) reads the cube face's line with the prism's top (lean 0.4714) against the germ along the wedge's edge (lean 1), at arm 1. Levered by the smaller lean, that is the offset of a point a unit along the edge from the cube face's plane: 6.8033e-9, in band. Both orders refuse `Coincidence(Sectors)`, reported as `bool_strut_side`. The pose is a sliver 6ε thick, so the pair really is in band (D10: thinness is judged by pairs). The unit reading had read it definite only by dividing that offset by the cube-face line's small lean. The row is re-stated as `a_sliver_lump_six_bands_thick_refuses_at_the_strut_side_in_both_orders`.

That leaves D10's rule with no witness: no row now shows a lump whose volume is sided by its certified enclosure while its `V/A` is in the zero band.

## What closing it takes

Find a pose where every pair reading the ∩ takes is definitely off the band, at every reach and lean it is read at (the ring's germ order among them), while the lump's `V/A` still reads in the zero band, so the enclosure decides its role. Pin it in `sliver_shell_role` at the ε rows where it holds. A different profile or tilt axis (`corner_pose`, `crates/sweep/examples/near_tangent_census_probe.rs`) may give a wedge thin in volume but thick across its pairs. If no such pose exists, because a `V/A` in the zero band forces some pair into the band, say so and re-state D10's witness as unreachable.
