---
id: a-wall-row-in-an-oblique-plane-reads-empty-by-where-the-planes-origin-is-stored
kind: issue
title: A wall whose bottom row lies in an oblique plane reads Counted([]) or a reach by where the plane's origin is stored
status: open
priority: P2
cost: M
opened: 2026-11-03
---


Found by PR 6479's dual review (R2, NOTE-1). It is on main too, so it
predates that PR.

## What happens

The plane of normal `(0, 0.6, 0.8)`, `u_ref` `(0, 0.8, −0.6)`, has a
wall whose bottom row lies in the plane. With the plane's origin stored
at `0`, `geom_brep::pair_section(plane, wall, None, Some(SsiLane::marched()), band)`
reads the Side contact: `Reaching`, the wall's `V`-low side. Move the
stored origin along the plane (`origin = far · u_ref`) and the reading
changes, though the plane does not.

R2's measurements, on main and on the PR head alike:

| far (m) | ε 1e-9 | ε 1e-12 |
|---|---|---|
| 0 | `Reaching` (`V`-low) | `Reaching` (`V`-low) |
| 1 | `Reaching` | |
| 10 | `Counted([])` | |
| 100 | `Reaching` | |
| 1e3 | `Counted([])` | `Counted([])` |
| 1e5, 1e6, 1e7 | `Counted([])` | `Counted([])` |

`ssi::plane_nurbs_ssi` on the same pose reads `Ok`, with no branch and
one `Side` region, at every one of these offsets. So the empty answer
comes before the 1×2 lane is read.

## Where to look

`crates/geom-brep/src/pair_section/spline_implicit.rs`, `classified`.
Against a plane, stage 1's hull (`SplineImplicitPose::hull`) and the
door read the stored plane at every scalar, before the anchored pivot
is consulted. A hull reading `Hull::Apart` returns `Counted(Vec::new())`.
The likely cause is the stage-1 hull deciding the row's `f64` heights
by their sign, when those heights are rounding about the far origin.
The cause is not confirmed. This is the same class as DR-402's "the
stored origin changes the verdict".

## What closes it

The hull and door read the plane about the wall, as the pivot does,
or charge their reading the stored origin's rounding. Add a row at the
offsets above.

## Measured mechanism (2026-11-11)

The hull is not the cause. On a wall whose bottom row lies in the
plane (`(0, 0, 0)–(1, 0, 0)`, rising 0.3 rad off it), the stage-1 row
heights in `f64` read `0` or `8.9e-16`, and the hull says Cuts. There
are two mechanisms:

1. **Fixed: the side's clear answer was read on the plane at its
   pivot.** `clears` took the exact sign of φ along a side lying in the
   band, on the pivot plane, which stands a rounding off the stored
   one. On main at ε 1e-9 the pose read `Counted([])` from 10 m out.
   That decision now reads the stored plane's sign (PR 6830).
   - **What holds now.** A scan of 324 stored origins (`far` from 0.1 m
     to 1e7 m on a log grid, along `±u_ref`, with and without an `x`
     offset of `0.3·far`) reads `Reaching` (`V`-low) at every origin at
     ε 1e-9 and 1e-6.
   - **The row.**
     `a_side_in_an_oblique_plane_reads_alike_wherever_the_planes_origin_is_stored`
     pins offsets 0 m to 1e7 m at those ε, and 0 m to 1e3 m at ε 1e-12.
2. **Open: at ε 1e-12, from about 1e4 m out, the pose still reads by
   where the origin is stored.** The same scan at ε 1e-12 finds 22 of
   324 origins deviating:
   - **`Reaching` on `U`-low and `U`-high**
     (`sides: [[true, true], [false, false]]`), a branch across the
     wall, at 1.26e4 m and 3.98e5 m along `+u_ref`;
   - **`Counted([])`** from 1.58e5 m along `+u_ref` and 2.0e5 m along
     `−u_ref`, at scattered offsets out to 7.9e6 m.

   The review saw the same verdicts at −1.26e4 m, and `Counted([])` at
   about 6.7e4 m for a random origin. All of them read at the origin
   less than 1.26e4 m out on the grid.

   **The measured mechanism, at 1e6 m:** the side's sign is now
   undecided (the lift is `[−3.5e-10, 2.3e-10]`), but the pivot plane
   stands `1.3e-10` m off the row (`sup`). The side's cover is read on
   the pivot plane, and its distance, `sup` over the slope across the
   side, does not fit a band padded by the shift alone. So `side_region`
   says `SideClass::Apart`, the side has no roots, and the pose reads
   `Counted([])`. The shift pads the band, but this distance moves by
   the shift over the sine. The `U`-low/`U`-high reading has not been
   traced; the likely cause is the same pivot plane standing off the row
   by more than the band, its crossing of the wall read as a branch. Main reads `Counted([])` from about
   5 m out at this ε.

The hull and the door read the stored plane's heights in `f64` about the
stored origin (`SplineImplicitPose::height`). They do not decide this
fixture, because its row's heights round to `0`, but nothing charges
them the origin's rounding (an ulp of 1e6 m is about 1e-10 m, against a
1e-12 m band). That is still the hazard this issue first named.
