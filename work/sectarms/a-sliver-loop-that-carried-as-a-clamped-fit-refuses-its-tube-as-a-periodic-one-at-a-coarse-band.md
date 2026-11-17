---
id: a-sliver-loop-that-carried-as-a-clamped-fit-refuses-its-tube-as-a-periodic-one-at-a-coarse-band
kind: issue
title: a cylinder × sphere loop a millimetre short of tangency carried at ε = 1e-6 as a clamped fit and refuses its tube limb as a periodic one
status: dispatched
opened: 2026-11-17
priority: P1
cost: M
branch: sectarms/sliver-loop-tube
design: true
---


Found by the periodic-loop carrier unit
(`a-quadric-pairs-section-spline-carries-a-c0-knot-the-chord-pass-refuses`,
branch `sectarms/periodic-loop-carrier`), running its own rows at the
three ε.

## Measured

`PairSection::carry` of a cylinder of radius 0.5 about `z` against a
sphere of radius `R` centred `d` from the axis (`geom-brep`'s
`tests/pair_section.rs` fixtures), through `SsiLane::marched()`, at the
witness band of each ε:

| `(d, R)` | ε = 1e-6 | ε = 1e-9 | ε = 1e-12 |
|---|---|---|---|
| `(0.5, 0.3)`, `(0.35, 0.3)`, `(0.1, 0.7)`, `(0.62, 0.15)`, `(0.2, 0.69)` | carries | carries | carries |
| `(0.2, 0.699)` | **refuses** | carries (2458 knots) | carries (13828 knots) |
| `(0.2, 0.6995)` | **refuses** | carries | carries |

The refusal is `CarryRefusal::March(SsiError::TubeStraddles { verdict:
Zero, margin 0.0, boxes: 437 })`: limb 3's transversality clearance over
the loop's tube boxes is not zero-free on any rung of the ladder. The
sphere reaches within `0.7 − R` of the far wall, `1e-3` and `5e-4` on
the two refusing poses.

On main (`72981a1b00`), where a closed branch is fitted clamped,
`(0.2, 0.699)` carries at ε = 1e-6: `pair_section::a_seated_loop_is_the_loop_point_for_point`,
which carries all five poses of the first two rows, passes there at
`CAD_TOLERANCE_EPS=1e-6` (run). `(0.2, 0.6995)` was not run on main.

So this is a pose the periodic fit lost at one band.

## What differs (measured by PR 7052's reviewers)

On the same 438 marched points at ε = 1e-6:

| `(d, R)` | periodic fit | clamped fit |
|---|---|---|
| `(0.2, 0.69)` | carries, tube radius 1.08e-2 | carries, tube radius 1.08e-2 |
| `(0.2, 0.699)` | refuses `TubeStraddles` (longest span 3.0e-3) | certifies at tube radius 1.7e-4, 60 times narrower than a normal pose's (longest span 4.9e-3) |
| `(0.2, 0.6995)` | refuses | refuses |

So the refusal boundary moved from between `R = 0.699` and `0.6995` to
between `0.69` and `0.699`: the clamped fit was already at its last rung
on this pose. The periodic fit's longest knot span is the *shorter* of
the two, so the span length this row first guessed at is not the cause.
**The cause is not found.**

## What would answer it

Read the box where limb 3's transversality enclosure holds zero on the
periodic fit at `(0.2, 0.699)`, and the same stretch of the clamped
fit's chain at its 1.7e-4 rung.

`pair_section::a_seated_loop_has_no_start` pins the pose as measured:
carried below a band of 1e-7, `TubeStraddles` above it.

## 2026-11-17 — the cause, measured (branch `sectarms/sliver-loop-tube`)

Read at ε = 1e-6 on the 438 marched points, with limb 3's probe
(`ssi::certify::probe_tube_analytic`) printing each rung's boxes.

**One chain box, and the enclosure's slack over it.** The loop turns
back in two tips beside the far ruling, 59 mm to either side of it, on a
24 mm radius. The march's spans shorten from 3.0e-3 to 3e-4 of the
parameter over the ten spans into a tip, and the last long one has a
control hull 36 mm by 28 mm by 8 mm, wider than the turning radius. Over
that one box, with no padding at all:

| fit | the box's span | `(∇f₁ × ∇f₂)·e` as limb 3 encloses it | zero-free while padded by less than |
|---|---|---|---|
| periodic | `(0.245556, 0.247920)`, box 95 of 437 | `[5.1e-5, 0.199]` | 7.4e-6 |
| clamped | `(0.245438, 0.247757)`, box 94 of 435 | `[1.9e-3, 0.199]` | 2.7e-4 |

The enclosure's lower end falls by about 6.9 per metre of padding on
both. The ladder's floor at this band is 8e-6 (`SSI_TUBE_RADIUS · ε`),
so the periodic fit's box is a graph on no rung: its narrowest rung,
1.07e-5, reads `[−2.3e-5, 0.199]`. The clamped fit's box clears the
rungs at 1.7e-4 and below. The two fits place their knots 1.2e-4 of the
parameter apart there (the clamped fit's knots are averages of three
parameters, the periodic fit's are the parameters), and that is the
whole difference between them. The last long span into the other tip
is the same shape: the periodic fit's boxes 313 and 314 read 5.9e-4 and
8.3e-4 unpadded and are graphs from the 4.3e-5 and 8.5e-5 rungs down,
and the clamped fit's box 313 reads 2.3e-3. Every other box of either
chain is a graph at 1.7e-4.

**The quantity itself is nowhere near zero there.** At the box's eight
corners `(∇f₁ × ∇f₂)·e` is at least 0.076 (the surfaces cross at about
0.1 rad). `ssi::enclose::graph_margin` takes the cross product of the two
gradients' boxes, which reads each coordinate of the point once in the
cylinder's gradient and again in the sphere's, so its width is first
order in the box and it leaves 5e-5 of that 0.076. The pose is not at
the band's resolution, and neither fit's reading says anything about the
pose.

**The clamped fit's 1.7e-4 was its first certificate, not its last.**
`refine_for_tube` then halved the spans under that box and the carried
loop on main had a tube of 5.5e-3 on 510 points. It refines a certified
carrier only, so the periodic fit, refused on its first certificate, is
never refined
(`work/ssiarith/an-r3-tube-no-rung-makes-a-graph-is-never-refined.md`,
whose second instance is this pose).

So of the candidates: not the ladder's floor as such (the box misses it
by 6e-7), not the base opening's end spans (boxes 0 to 3 are graphs from
the 5.5e-3 rung down), not the refinement behaving differently on a
periodic net (it never runs), and not a true near-tangency.

## The fix is a design fork

Three fixes carry the pose. Each was measured at ε = 1e-6 over
`geom-brep`'s suite or the rows named.

**A. Meet the graph margin with its mean-value form** (on the branch).
The plane's, sphere's and cylinder's gradients are affine in the point,
so `m(c) + Σₖ ∂ₖm(b)·(bₖ − cₖ)` about the box's centre is an enclosure
of the same quantity with second-order excess. It is sound and inside
C2's text, which asks only that the enclosure exclude zero.

- `(0.2, R)` for `R` = 0.69, 0.699, 0.6995 and 0.6999 carry at 1e-6,
  1e-9 and 1e-12, on tubes of 4.3e-2, 2.2e-2, 1.1e-2 and 5.5e-3 at every
  band (main at 1e-6: 1.1e-2, refused, refused, not run).
- **It moves every analytic pair's rung up, and a component's pad is its
  tube's reach** (`CarriedComponent::pad`, `pad_and_reach`), which every
  site reading and the witness match pad their band by. At 1e-6 the two
  loops of `(0.2, 0.701)` go from a 5.5e-3 tube and a 3.9e-2 pad to a
  6.7e-2 pad, wider than the loops stand apart, and the carry escalates
  on `curve_carrier_straddle` in `PairSection::matched`. Rows that pass
  on main at 1e-6 and stop under A:
  `pair_section::two_loops_keep_their_sites_and_order_near_the_node`,
  `pair_section::near_the_node_no_site_reads_on_the_other_sheets_loop`
  (190 of 1756 sites refuse), and
  `pair_section::wiring::close_rings_carry_counted` at δ 3e-3. All of
  `geom-brep` passes at the default ε.

**B. Read the tighter form only where the plain one gives no rung.**
Rung selection is main's wherever main certifies. Measured: the pose
carries (tube 2.2e-2), `(0.2, 0.6995)` carries, and the three rows above
pass. The rescued pose's tube is twice that of the easier
`(0.2, 0.69)`, since only the rescue reads the tight form, and
`close_rings_carry_counted`'s δ 3e-4 moves from `TubeStraddles` to the
same witness-match escalation (pad 4.3e-2, one ring's witness 3.0e-2 to 5.5e-2 from the other ring). Which
enclosure a rung is selected on is not something C2 says.

**C. Refine under a box that is a graph on no rung**, as the tube
rounds do under one that holds a certified tube narrow. Not built. It is
what main had for this pose before the periodic fit (1.7e-4, then
5.5e-3), and it moves no rung. C2 says a tube refusal is "typed, never a
retry loop" and C3 that limb 3's "refusal stands", so it is a change to
ratified text.

Under A and B alike, a tighter enclosure widens the tube and with it the
pad, so how finely a loop's sites read near a node is set today by how
loose limb 3's arithmetic is. That coupling is the question under all
three: whether the tube the accounting banks and the pad the readers
read should be one radius.

## Also red on main at 1e-6 with this cause

- `pair_section::one_loop_keeps_its_order_near_the_node` carries this
  pose and refuses the same `TubeStraddles { boxes: 437 }`.
- `pair_section::wiring::close_rings_carry_counted`'s pin names
  `boxes: 206`; the periodic fit's chain has 208.
