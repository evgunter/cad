---
id: the-torus-doors-tube-straddles-transverse-poses-at-a-coarse-band
kind: issue
title: At ε 1e-6 the torus doors refuse TubeStraddles on transverse poses far from any tangency, the tube's crossing-angle bound reading zero
status: open
opened: 2026-10-28
priority: P2
cost: M
---


Filed by SECT's `sect-torusdoors` lane (branch `sect/torus-doors`).

## Measured

`crates/geom-brep/tests/torus_doors_ssi.rs` `germ_row`, debug build,
after merging PR 5961. At the compiled ε (1e-9) every transverse
placement of the germ row certifies at `ssi::cylinder_torus_ssi` and
`ssi::torus_torus_ssi`. At `CAD_TOLERANCE_EPS=1e-6`:

- moved donut × drum: placements 2, 3 and 5 refuse `TubeStraddles`
  (187-, 185- and 486-box chains, "the uniqueness tube's lower bound on
  the crossing angle is not clear of the zero band (bound 0e0 m)");
- two donuts: placements 2 and 5 refuse the same (431- and 683-box
  chains), while donut × moved drum certifies at all six;
- the lily's stem and arch tori refuse the same (150 boxes) where at
  1e-9 they refuse `CellBudget`
  (`crates/sweep/tests/mate7a_r1_probes.rs`
  `the_lilys_tori_exhaust_the_doors_cell_budget`).

At 1e-12 one two-donut placement (5) refuses `StepBudget` (20000 steps).
These poses' critical gaps are decided transverse with margin, and at
1e-9 their tubes clear, so a lower bound of exactly zero on the crossing
angle at a coarser band (a wider tube) reads as the enclosure of
`(∇f₁ × ∇f₂)·e` over the torus's boxes losing its sign, not as a
geometric near-tangency. The refusals are typed and sound; the rows
admit them off the compiled ε and pin the verdicts at it.

## What a fix has to supply

Where the bound goes to zero: the torus's gradient enclosure over the
tube's boxes at the coarse band (`ssi::enclose`
`implicit_gradient_enclosure`'s torus arm), or the chain's tangent `e`
across a box. Then the rows' verdicts at 1e-6 pinned as at 1e-9.
