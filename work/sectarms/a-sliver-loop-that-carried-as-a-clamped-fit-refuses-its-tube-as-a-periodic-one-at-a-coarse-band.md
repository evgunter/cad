---
id: a-sliver-loop-that-carried-as-a-clamped-fit-refuses-its-tube-as-a-periodic-one-at-a-coarse-band
kind: issue
title: a cylinder × sphere loop a millimetre short of tangency carried at ε = 1e-6 as a clamped fit and refuses its tube limb as a periodic one
status: open
opened: 2026-11-17
priority: P1
cost: M
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

*Correction (both designers of the fork below, 2026-11-17).* This is
not slack in the arithmetic. C2's limb 3 proves at most one solution per
slice by a mean-value argument: two solutions x, y on a slice give
`∇f₁(ξ₁)·(x−y) = 0` and `∇f₂(ξ₂)·(x−y) = 0` at two different points, so
the determinant has to be zero-free with its two gradients read at
independent points of the box. `graph_margin` encloses that two-point
range to within about 1% (a reconstruction of the pose). The 0.076 at
the corners is the one-point quantity, which alone shows only that no
slice is tangent to the section. On quadratic forms (plane, sphere,
cylinder), `f(x) − f(y) = ∇f((x+y)/2)·(x−y)` exactly, so both
mean-value points are the chord's midpoint, which lies in the box, and
one point does prove the same theorem there. A torus form is quartic,
and one point does not.

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

## The fork

The lane (branch `sectarms/sliver-loop-tube`, `7467415a2f`) built one
fix and measured two more. Each carries the pose, and the two it
measured both move rows near a node at 1e-6:

- **A tighter enclosure** of the quantity (`two_loops_keep_their_sites_and_order_near_the_node`,
  `near_the_node_no_site_reads_on_the_other_sheets_loop` and
  `close_rings_carry_counted` at δ 3e-3 escalate or refuse). The tube
  widens, the pad is the tube's reach, so the pad widens with it.
- **The same enclosure only where the plain one gives no rung** (the rows
  pass, but which enclosure selects a rung is not something C2 says).
- **Refinement under a box that is a graph on no rung** (not built).
  C2's "typed, never a retry loop" and C3's "refusal stands" read
  against it.

**The question.** How should limb 3 certify a carrier whose box is a
graph in truth but whose enclosure leaves no rung, so that this pose
carries at 1e-6, without making the readers near a node worse? Where
does the change live, what ratified text does it change, and should
the tube the certificate banks and the pad the readers read be one
radius?

Two designers answered it (fork-log row 281; the `[ev]` PR carries
their reports). They agree:
- on plane, sphere and cylinder pairs, limb 3 reads its graph quantity
  at one point (the midpoint identity above), and a torus keeps the
  two-point reading;
- where a box wider than its rung is no graph, limb 3 halves its own
  cover (its certification net's spans), under the crate's stop rule,
  and the carrier is not touched; `refine_for_tube` retires;
- the pad becomes its own claim, the reach of the narrowest rung that
  still proves one arc, read box by box; the tube stays the widest;
- the pads near a node must be measured on the as-marched carrier
  before a build is dispatched.

C2's limb-3 and pad sentences, its node-end clause and C3's tube-rounds
paragraph change.

## Also red on main at 1e-6 with this cause

- `pair_section::one_loop_keeps_its_order_near_the_node` carries this
  pose and refuses the same `TubeStraddles { boxes: 437 }`.
- `pair_section::wiring::close_rings_carry_counted`'s pin names
  `boxes: 206`; the periodic fit's chain has 208 (filed as
  `main-red-since-pr-7052-the-recourse-roster-and-a-close-rings-pin`).
