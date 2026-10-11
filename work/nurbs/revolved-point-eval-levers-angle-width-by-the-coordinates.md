---
id: revolved-point-eval-levers-angle-width-by-the-coordinates
kind: issue
title: RevolvedPoint::eval's anchored rotation carries an angle's interval width to the point times the coordinates' magnitude, so restrictions with an inexact start still grow at a far placement; the radius-levered spellings cost f64 agreement on origin-axis geometry
status: closed
opened: 2026-10-09
closed: 2026-10-10
branch: nurbs/revolved-point-axis-anchored
pr: 4518
refs: [mapped-curve-restrict-composes-placements-per-split]
priority: P2
cost: M
---

Found by `nurbs/restrict-in-the-parameter`, which moved
`MappedCurve::restrict` onto a `SweepRange`: a sub-range of the whole
sweep's normalized parameter, stored as a start and a span, with the
sweep's angle stored once beside it and applied at evaluation. The
placement is kept as built, so the per-split stored rotation is gone. A
dyadic split is exact in the normalized parameter, so chains anchored
at either end (`(0, ½)`, `(½, 1)`, `(0, a)`) store no rounding at all,
and every chain measured is at or under the composed placement except
two: `(0.3, 0.7)` far at 1.06× and its quotient form at 1.03×. These are
pinned by `crates/geom-brep/tests/revolved_point_anchor.rs`
`restricted_widths_stay_under_their_ceilings`,
`end_anchored_chains_stay_at_the_f64_floor` and
`restriction_is_no_wider_than_composing_into_the_placement`. What
still grows is one mechanism, and this row is about it.

**The mechanism.** A split whose start moves by an inexact amount
rounds the range's start once, at the start's own ulp (`start +
span·s0`). `MappedCurve::eval` (`crates/geom-brep/src/mapped.rs`) turns
the point through `Affine3::rotation_about_axis(q, n, θ).transform_point(p)`,
that is `R·p + (I − R)·q`. At `T = Interval` an angle width `w` therefore
reaches the point as about `w·(|p| + |q|)`, the coordinates' magnitude,
not as `w·|p − q|`, the radius about the axis. Far from the origin that
lever turns one ulp of the start per split into ~2e-12 per split, about
what the composed placement paid per split for its stored rotation.

Measured (Interval; widest of `eval` at `s = 0, ½, 1`; a 1 m-radius rim
on a `+z` axis, nested `restrict` 64 times; N = 1 / N = 64):

| placement | split | composed placement (main) | normalized range, anchored eval |
|---|---|---|---|
| near origin | (0.3, 0.7) | 3.9e-14 / 5.5e-13 | 2.2e-14 / 2.3e-13 |
| near origin | (½, 1) | 1.8e-14 / 3.0e-13 | 1.0e-14 / 1.1e-13 |
| (1000, −700, 300) | (0.3, 0.7) | 2.2e-11 / 1.1e-10 | 1.2e-11 / 1.1e-10 |
| (1000, −700, 300) | (a, 1) | 2.6e-11 / 2.0e-10 | 1.8e-11 / 1.9e-10 |
| (1000, −700, 300) | (½, 1) | 1.0e-11 / 1.2e-10 | 6.1e-12 / 5.4e-11 |
| (1000, −700, 300) | (0, ½) | 7.5e-12 / 6.7e-11 | 6.3e-12 / 1.0e-12 |

`(½, 1)` is flat to N = 52 (7.3e-12 far). Past that, `1 − 2⁻ᴺ` is no
longer an `f64` and the start rounds; by then the range is `2π·2⁻⁵³`
of turn.

The same lever governs `crates/topo/src/offset_axial.rs` `reauthor`'s
reading of a turned start corner. It reads the corner back through the
composed placement `(R(φ)·place)⁻¹`, main's spelling. On a placement
tilted and shifted a thousand metres out (the module's
`a_turned_start_on_a_tilted_far_placement_*` rows), that reading is
1.1e-6 to 2.8e-6 wide at `Interval` from exact inputs. Turning the
corner back on its offset from the axis, `place⁻¹(p − (I − R(−φ))(p −
q))`, held it at 1.3e-10 to 3.7e-10. At `f64`, though, that spelling
lands 2.5–3× farther from the corner (1.0e-12 against 3.4e-13 at 1e3;
7.3e-11 against 2.9e-11 at 1e5), so the composite stays.

**Why the respell is not simply taken.** Two spellings carry the angle
on the offset `p − q`:

- `p − (I − R)·(p − q)` (`Mat3::identity_minus_rotation_about`) keeps
  the start sample free of the axis's width, which the shipped
  spelling also held (`revolved_point_anchor.rs` now pins what the
  chosen spelling gives up there:
  `an_uncertain_axis_reaches_the_start_sample_at_most_twice_over`).
- `p − 2·sin(θ/2)·(sin(θ/2)·v⊥ − cos(θ/2)·(n × v))` (Rodrigues anchored
  at `p`, `v = p − q`) is the other.

At `f64`, on an axis through the origin with a large radius, both agree
with the carrier's own evaluation worse than `R·p` does. On
`crates/topo/tests/mesh8_coherence.rs` `tilted_lune` (R ≈ 2.3e6 m,
ε = 1e-9, the radius chosen to sit at the band), the max
`|description − carrier|` over 17 samples per edge is:

| spelling | range over 10 edges |
|---|---|
| `R·p + (I − R)·q` (shipped) | 4.7e-10 – 7.0e-10 |
| `p − (I − R)·(p − q)` | 5.7e-10 – 1.9e-9 |
| Rodrigues at `p` | 6.7e-10 – 1.05e-9 |

With the `(I − R)` spelling, that row's `the fixture must build` went
red on hosted CI (PR 4441's first two heads), though it stayed green
locally. At far placements the same spelling moved
`crates/sweep/tests/sym11_far_placement_rows.rs`: the washer stopped
refusing on `MappedSource` and reached `Surface2Residual`, or built.

So the choice trades `f64` agreement on origin-axis, large-radius
geometry (about 2× worse) for interval width and `f64` accuracy at far
placements (orders better). It is a design call on how the description
should be evaluated, not a mechanical fix. The data above is the input
for that call.

**The segment arm shares the range, not the lever** (evidence from
`nurbs/restriction-on-the-description`, which moved `SketchSegment`
restriction onto the same range). A placed arc evaluates
`Arc2::point_from(a, range.at(s))`, which turns the authored `a`
about the carrier's centre, so the start's per-split rounding reaches
the point times the radius·sweep and not the coordinates: over 64
`(0.3, 0.7)` / `(a, 1)` splits of a unit half-turn arc it reaches
3.1e-14 / 4.7e-14 at the origin and stays at 2.3e-13 – 3.4e-13 (one
rounding of the coordinates) a thousand metres out
(`crates/geom-brep/tests/sketch_segment_restriction.rs`
`nested_segment_splits_stay_at_one_evaluations_width`). The near
figures are 1.2–1.35× what re-deriving the endpoints stored at the same
counts (2.3e-14 / 4.0e-14), the far ones 44–51× under it. A spelling
chosen for `RevolvedPoint::eval` here does not reach the segment arm.

## Closed

Decided by W1 (`crates/geom-core/README.md`): a revolved point
evaluates as `q + R·(p − q)`, through
`Affine3::rotate_point_about_axis`, and `offset_axial::reauthor` reads
a turned corner back as `place⁻¹(q + R(−θ)·(p − q))`, its azimuth
test included.

- **The mechanism is gone.** An angle's width reaches the point times
  the radius about the axis. Over every chain of the table above at
  N = 1 / 8 / 64, a thousand metres out every chain with exact split
  parameters sits at 2.3e-13 – 3.4e-13 (one to three ulps of the
  coordinates) at every count, where the anchored rotation read up to
  1.9e-10; at 1e5 out, 1.5e-11 – 2.9e-11 against up to 1.9e-8. Near
  the origin every chain is a quarter of what it was. `(0.3, 0.7)` far
  and its quotient form, the two that sat over the composed placement,
  are at 0.002× of the anchored rotation and 0.01× of the composed
  placement.
- **The read-back.** On the tilted far placement, with the turned
  corner built by the same spelling, the stored sketch point is
  2.3e-11 – 3.9e-11 wide at `Interval` against the composite's
  3.5e-7 – 5.2e-7 (which no longer fits the ordinary band), and at
  `f64` the start sample lands 2.3e-13 (1e3) and 1.5e-11 (1e5) from
  its corner, within four ulps of the coordinates; reading back
  through the composite against this evaluation lands 8.0e-13 and
  5.8e-11.
- **A revolve's far copy** turns the same way: its vertices, its cap
  arcs' carrier centres and the points its cap plane is fitted
  through are sketch points placed and then turned
  (`sweep::swept::Placing`). A full turn's return-half latitude arcs
  and a full-turn strut describe themselves as the whole turn's range
  run back, on the sketch placement. Only the far cap's segment
  descriptions carry the turned placement, the turn composed into it
  once; against a 70-digit reference they read within 3.7 ulps of the
  coordinates at 1e3 and 1.9 at 1e5, the turned point within 0.85
  (`revolved_point_anchor.rs`
  `a_far_cap_placement_reads_within_ulps_of_the_turned_point`).
- **What it gave up** is the turned point's independence from the
  axis point's width, `q` being mentioned twice. With `p` exact and
  `q` a box `w` wide, coordinate `i` of a sample is
  `(1 + Σⱼ|Rᵢⱼ|)·w` wide: `2·w` at the zero turn, up to `(1 + √2)·w`
  about a coordinate axis and `(1 + √3)·w` in general
  (`Affine3::rotate_point_about_axis`; `revolved_point_anchor.rs`
  `an_uncertain_axis_reaches_the_start_sample_at_most_twice_over` and
  `an_uncertain_axis_reaches_a_turned_sample_within_its_row_bound`,
  2.41–2.69·w at a 2.1 rad turn on a tilted axis). No producer hands
  an axis wider than its point.
- **The segment arm's near 1.2–1.35×** is `Arc2::point_from`'s
  anchoring, not this evaluation, and is no defect under W1: it scales
  with the radius times the sweep and stays flat in N
  (`sketch_segment_restriction.rs`
  `nested_segment_splits_stay_at_one_evaluations_width`), and the
  spelling it is measured against is the retired re-derivation. No row
  is opened for it.
- **`tilted_lune`** evaluates on an axis through the origin, where the
  new spelling is the old one's `R·p`; it stays green at 1e-6, 1e-9
  and 1e-12.

