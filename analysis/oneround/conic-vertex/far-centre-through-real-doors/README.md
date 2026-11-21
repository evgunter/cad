# A centre-held conic with a far centre, built through real doors: right, wrong, or refused

Measured 2026-11-21 on `oneround/anchored-conic` at `c3fb679d1` (a conic
built from a centre is stored and read as on `main`) with two temporary
probes, reverted. Facts only.

## The doors

Each at centre distances `D` = 3e3, 1e4, 1e5, 1e6, 1e7 m, at `f64`:

- **sketch arc**: a 2 × 1 rectangle whose top side is a bulge arc of
  radius about `D` (bulge `2/(4D)`), validated and extruded 1 m.
- **revolve**: a unit square revolved through `2/D` rad about an axis `D`
  away (`RevolveAxis { origin: (−D, 0), dir: (0, 1) }`).
- **plane section**: that sketch-arc prism intersected with a box whose
  bottom plane cuts the large cylinder wall square to its axis.

A STEP circle was not run. No body was built at `Interval`.

## Does the body build

`builds-K10.txt`, `builds-Kmin.txt` (K = 1.002 at ε = 1e-9, K = 2.81 at
ε = 1e-12, each just above `1 + ADMISSION_FLOOR/ε`). The outcomes are
the same at both K but for the two noted.

| D | ε = 1e-9: sketch arc | revolve | plane section | ε = 1e-12: sketch arc | revolve |
|---|---|---|---|---|---|
| 3e3 | builds, tier 3 ok | builds, ok | builds, ok | refused: extrude, `AnalyticRung3(KernelLimit)` on the cylinder, bound 2.8e-12 | refused: `KernelLimit`, 1.0e-11 |
| 1e4 | builds, ok | builds, ok | builds, ok | refused: extrude, `Escalated { MappedSource }` 1.4e-12 | refused: `KernelLimit`, 3.7e-11 |
| 1e5 | builds, ok | builds, ok | builds, ok | refused: validate, `ArcBelowSceneResolution { OnCarrier }`, margin 1.5e-11, headroom −9.4e4 | refused: `Escalated { MappedSource }` 6.8e-12 (K 10), `ResidualExceeded` (K 2.81) |
| 1e6 | refused: extrude, `KernelLimit` 3.4e-9 | refused: `KernelLimit` 2.7e-9 | no prism | refused: extrude, `Escalated`/`ResidualExceeded { MappedSource }` 5.6e-12 | refused: `ResidualExceeded { MappedSource }` 1.0e-10 |
| 1e7 | refused: validate, `Escalated { ArcStartOnCarrier }` 1.9e-9 (K 10), `ArcBelowSceneResolution { OnCarrier }` headroom −9.4e6 (K 1.002) | refused: `KernelLimit` 3.7e-8 | no prism | refused: validate, `ArcBelowSceneResolution { OnCarrier }` margin 1.9e-9, headroom −1.0e7 | refused: `ResidualExceeded { MappedSource }` 7.0e-10 |

- `ArcBelowSceneResolution` fires for the sketch arc at ε = 1e-12 from
  `D = 1e5` (not at 1e4 or 1e6, where extrude's certification refuses
  first or instead) and at ε = 1e-9, K = 1.002 at `D = 1e7`.
- Nothing refuses a revolve about a far axis before the conic is minted:
  every revolve refusal here is the edge certificate's (`KernelLimit`,
  `MappedSource`), raised by the revolve door on the conic it built.
- So through these doors a far-centred conic is stored only at
  ε = 1e-9 and `D ≤ 1e5`. Each such body carries eight far-centred
  circle edges in all (two on the prism, two on its section, four on the
  revolve), radii `D` and `D + 1`.

## Decisions on the stored conics

The instrument is the near-gap row's: `conic_quadric_roots(carrier, t0,
t1, wall, band)` on each stored edge's own carrier and span, against a
wall of radius 1 placed to graze the edge at its middle, the conic's
centre along the wall's outward normal. Poses: depths `±m·K·ε`,
`m` = 0.3, 0.6, 1, 2, 5, 20, 100, 500 (negative: the conic dips into the
wall), three tilts of the wall's axis (1.1, 1.2, 1.3 rad from the conic's
tangent): 48 per edge, 384 per distance. The oracle is
`conic_oracle::exact::truth`. The edges' frames were reproduced bit for
bit (`Frame::admitted`).

- **right**: the oracle's count of crossings in the edge's span, each
  root within ε of arc.
- **right to spacing**: the same within `8·2⁻⁵²·a`, the parameter's own
  spacing (none occurred).
- **in-band mismatch**: a definite answer that differs at a depth within
  `K·ε` (none occurred).
- **WRONG**: a definite answer that differs past `K·ε`.
- **refused**: typed. Over all runs: `BooleanError::Escalated { decision:
  ArcCylinderRoots }` 7062, `Unsettled::Tangency` 6647,
  `Unsettled::Meter { bool_conic_quadric_sub_root_slack }` 3739.

Two readings of each conic: **centre-sum**, as stored and read today;
**chord**, the same frame and size read by chords from
`fl(centre + u_ref·a)`. Two lanes: `f64`, and `Interval` with the stored
carrier and the wall lifted to point enclosures.

Rounding reach of each run, `R = box·(K − 1)·ε / ADMISSION_FLOOR`, and
`(K − 1)·ε/u`:

| ε, K | admitted | R | (K − 1)·ε/u |
|---|---|---|---|
| 1e-9, 10 | yes | 5.07e6 m | 8.11e7 m |
| 1e-9, 1.002 | yes | 1.13e3 m | 1.80e4 m |
| 1e-12, 10 | yes | 5.07e3 m | 8.11e4 m |
| 1e-12, 2.81 | yes | 1.02e3 m | 1.63e4 m |

`decisions-by-distance.txt` has every cell; `decisions-raw.txt` every
edge. Right / WRONG / refused of 384, in-band mismatch and
right-to-spacing being zero everywhere:

| ε, K | D | centre-sum f64 | chord f64 | centre-sum Interval | chord Interval |
|---|---|---|---|---|---|
| 1e-9, 10 | 3e3 | 96 / 0 / 288 | 192 / 0 / 192 | 72 / 0 / 312 | 72 / 0 / 312 |
| 1e-9, 10 | 1e4 | 48 / 0 / 336 | 96 / 0 / 288 | 48 / 0 / 336 | 48 / 0 / 336 |
| 1e-9, 10 | 1e5 | 0 / 0 / 384 | 0 / 0 / 384 | 0 / 0 / 384 | 0 / 0 / 384 |
| 1e-9, 1.002 | 3e3 | 48 / 0 / 336 | 96 / 0 / 288 | 48 / 0 / 336 | 48 / 0 / 336 |
| 1e-9, 1.002 | 1e4 | 24 / 0 / 360 | 48 / 0 / 336 | 0 / 0 / 384 | 0 / 0 / 384 |
| 1e-9, 1.002 | 1e5 | 0 / 0 / 384 | 0 / 0 / 384 | 0 / 0 / 384 | 0 / 0 / 384 |
| 1e-12, 10 and 2.81 | 3e3, 1e4, 1e5 | 0 / 0 / 384 | 0 / 0 / 384 | 0 / 0 / 384 | 0 / 0 / 384 |

In that table each reading is judged against the exact oracle run on
**its own** conic: the centre-sum against the circle about the stored
centre, the chord against the circle through `fl(centre + u_ref·a)` (the
oracle reads that anchored conic's derived centre). So judged, no cell
holds a wrong definite verdict: 18,432 decisions, 0 wrong.

### The chord reading judged against the stored centre's circle

`decisions-by-distance-three-judgings.txt`,
`decisions-raw-three-judgings.txt`: the same edges and poses, with a
third judging, the chord reading's answers against the exact roots of the
circle about the **stored centre**. Right / WRONG / refused of 384:

| ε, K | D | chord f64 | chord Interval |
|---|---|---|---|
| 1e-9, 10 | 3e3 | 192 / 0 / 192 | 72 / 0 / 312 |
| 1e-9, 10 | 1e4 | 93 / **3** / 288 | 48 / 0 / 336 |
| 1e-9, 1.002 | 3e3 | 96 / 0 / 288 | 48 / 0 / 336 |
| 1e-9, 1.002 | 1e4 | 39 / **9** / 336 | 0 / 0 / 384 |
| every other cell | | 0 / 0 / 384 | 0 / 0 / 384 |

- The twelve are on the sketch-arc prism's edges and its plane section's
  (radius 1e4), at depths −1e-6 m (K = 10) and −5.01e-7 m (K = 1.002):
  the right count of roots, each placed 1.4e-9 to 1.9e-9 m (K = 10) and
  1.4e-9 to 2.7e-9 m (K = 1.002) along the arc from the stored centre's
  true root. That is past ε = 1e-9, and at K = 10 inside `K·ε` = 1e-8.
- The same twelve answers are within ε of the roots of the circle through
  `fl(centre + u_ref·a)` (the table above).
- The two exact circles differ by the rounding of the seam vertex, at
  most `ulp(1e4)` = 1.8e-12 m a coordinate; at these near-tangent
  crossings that moves a root by more than ε.
- The `Interval` lane holds no such answer in any cell.

The ε = 1e-12 rows decide carriers the ε = 1e-9 build stored: no door
stores one at ε = 1e-12 (above). `D = 1e6` and `1e7` have no cell: no
door stored a conic there at either ε.

## Not measured

- A STEP circle, a far sphere's section, an ellipse (every stored conic
  here is a circle).
- Bodies built at `Interval`.
- Decisions other than the conic × wall root door.
