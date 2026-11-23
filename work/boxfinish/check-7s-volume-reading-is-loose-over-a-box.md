---
id: check-7s-volume-reading-is-loose-over-a-box
kind: issue
title: Check 7's volume reading encloses a slab of volume [0.95, 1.05] as [-1.62, 4.13] over a ±0.05 box, and reads only below width 0.03
status: open
opened: 2026-11-14
priority: P2
cost: M
design: true
---


Found by BOXFINISH's M1 measurement (2026-11-14, main `b363f5fa`, the
switch's box arm off on `boxfinish/probe-switch-off`).

Where no aimed ray reads, check 7 reads a solid's side off its volume
(`positive_volume_exact`). Over the slab of depth 1 ± 0.05 (true volume
in [0.95, 1.05]) the reading is `Enclosure { -1.617, 4.129 }`, and over
the derived-frame cube `[-5.163, 15.214]`. It reads only at a box width
of 0.03 or less, and a finish that falls to it costs 254 to 787 ms at
`Sym<Interval>`, against 50 to 110 ms where the rays read. Where the
width enters is not traced.

It is not behind the switch's deletion: once the rays read
(`the-side-reader-cannot-read-a-solids-orientation-over-a-box`,
`the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box`)
the slab class does not reach it. It still holds every body the rays
cannot read: a selection holding a NURBS face casts no ray
(`cast_aims`' `uncrossable`), so a loft always goes to the volume.

## Re-banded P1 (2026-11-15, M4): it holds the switch's deletion

With the frames built (PR 6972) and the switch's box arm off, a slab
boxed wider than ± 0.215 of its depth still loses every aimed ray and
then the volume reading (`OrientationUndecided { reading: Volume {
Enclosure{-0.9699, 2.9699} }, aimed: 6, set_aside: 6 }` at depth
`[0.75, 1.25]`): 40 refusing finishes across the box studies, one
extra bisection level, and two red rows (m10_8's decision pin; m10_3's
`the_certification_width_is_no_longer_bounded_by_epsilon`, which
settles at a half-width of 0.215 where it wants about 1). The stored
normals are exact now; per ray the refusals are the caps'
`side_at_infinity_winding` (`{-3.827, 13.758}`, `{-1.412, 8.154}`,
raised by `turned_inside` / `crosses_by_turn`), `side_at_infinity_edge`
(`{0, 0.177}` to `{0, 1.227}`) and `side_at_infinity_advance`
(`{-1.199, 0.927}`). `delete-the-box-scalar-sweep-leaf-switch` is
parked on this row, so it takes that row's band.

## Measured (2026-11-15, M6): where the width enters; back to P2

M6 (`boxfinish/probe-volume-reading`, `probe/M6-VOLUME-REPORT.md`, on
main `2e08069a`), `Sym<Interval>`, the unit square extruded by a boxed
depth.

- **The route.** Every face is a plane bounded by lines, so the reading
  is `rederive_polygons`' fallback sum of `polygon_face_about` over the
  six faces (the exact polygon volume is `None`: a vertex is no point
  interval), about the shell's reference `corner_of`, a TOP vertex,
  itself boxed. No pad and no quadrature: at a point box it reads
  `[1, 1]` exactly.
- **All of the excess is dependency**: the one boxed depth subtracted
  from itself where the true value is zero or constant, at three sites.
  (a) The anchor less the centre, on the cap holding the centre
  (`(d − d) + mean(d − d)`). (b) `p − l` inside that cap's fan, which
  feeds its area and its `tilt_slack`. (c) `across = va − n(va·n)/(n·n)`
  on every wall: the wall's own area less itself. The slack then
  multiplies two such widths and is added symmetrically per face. At
  depth `[0.75, 1.25]` the reading's width of 3.940 is 0.500 real,
  0.667 the top cap's lever, 1.155 the top cap's slack and 1.618 the
  four walls' slack.
- **The symbolic channel is not read.** The fan sum formed by hand at
  `Sym<Interval>` has the form `(6·d)/6`, the depth itself; the reading
  lifts each coordinate through `Interval::from_certified`, which reads
  the value channel. The role walk takes no session decision.
- **Scaling:** linear, 6.24 times the box (the truth is 1 times), the
  same placed at 10 or at 1000.
- **Hand re-posings, as instruments, not proposals** (width at ± 0.25;
  truth 0.500): the code's arithmetic 3.940; the same about an exact
  point of the body with the slack dropped 0.833; the plain fan
  `Σ det/6` about an exact point 0.667; the caps paired and the walls
  about an exact point, no slack, 0.500. The ones that drop the slack
  drop what covers a loop standing off its plane, so none is shown
  sound in general.
- **Other bodies:** a pentagon prism with two turning walls reads 18
  times the box (the walls' stored normals are truly boxed, and
  `plane_flux` holds that box in three operands); a loft of two squares
  reads 63 times the box, four fifths of it in the NURBS walls'
  quadrature enclosures (5e-15 wide at `f64` members, so it is the box),
  and decides through ± 0.05 of a unit height.

**Corrections to this row.**
1. "Reads only at a box width of 0.03 or less": with the frames exact
   the slab's reading decides through ± 0.14 (`[0.679, 1.321]` at
   ± 0.05, where this row says `[-1.617, 4.129]`).
2. The slab's volume gives out (± 0.15) before its rays do (± 0.215),
   so on a slab the volume never rescues a body the rays lost.
3. "A finish that falls to it costs 254 to 787 ms" is not the
   reading's cost: the reading is 0.5 ms and no decisions, and runs on
   every finish before the rays. The cost is the rays' set-asides (198
   numeric decisions and 72 ms where all six are set aside).
4. A loft's wide reading refuses as `VolumeSignUnresolved { Quadrature }`.

**Back to P2, and off the switch's deletion.** It was raised to P1 an
hour earlier on M4's reading that the volume stands behind the slab's
rays. It does not: where the rays read (the ray-readings unit) the slab
class never reaches it. It still holds every body no ray reads (a loft
over a box wider than ± 0.05 of its height), which no box study pins
today.

**What a fix has to decide** (a design question, not a re-arrangement):
whether the reading is formed about an exact point and with paired or
cancelled terms, what `tilt_slack` is for over a box (it covers a loop
off its plane; over a box it is charged for a width that is not a
tilt), and whether the reading may use the symbolic channel, where the
slab's volume is its depth.

## Measured (2026-11-22, the volume-premises review pass): a plane read off the walk's sums

PR 7344 reads a curved closed-form face's term off the role walk's own
sums at a scalar whose sums are enclosures, and leaves planes on
`quad_lane::planar_face_about`. With the plane exclusion taken out of
`topo::props` `walked_closed_form` (a mutant, never committed), the
seamless drum of boxed radius `[0.7625, 0.8625]` (two plane caps
bounded by one circle each, `Sym<Interval>`) reads `[1.7414568,
2.4221352]` where it reads `[1.5338833, 2.6297088]` today: width 0.681
against 1.096, the members' volumes (1.827, 2.074, 2.337) held by both.
No row of `topo`'s or `sweep`'s slow set at the default ε changes
colour under that mutant. So on a cap bounded by an arc the walk's own
sum is the tighter reading, with no tilt slack; whether it may stand in
for the plane formula in general (a loop standing off its stored plane)
is this row's question.
`crates/sweep/tests/volume_premises_over_a_box.rs`
`a_planes_term_is_read_by_its_own_formula_over_a_box` pins today's
reading and moves with the answer.
