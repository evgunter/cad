---
id: a-sliver-loop-that-carried-as-a-clamped-fit-refuses-its-tube-as-a-periodic-one-at-a-coarse-band
kind: issue
title: a cylinder × sphere loop a millimetre short of tangency carried at ε = 1e-6 as a clamped fit and refuses its tube limb as a periodic one
status: review
opened: 2026-11-17
priority: P1
cost: M
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

## Decided (Ev, PR 7056, 2026-11-17; fork-log row 281)

Ev approved the designers' agreed plan ("ok this plan sounds good!"):

- **One point on quadric pairs.** On plane, sphere and cylinder pairs,
  limb 3 reads its graph quantity at one point (the midpoint identity).
  A torus keeps the two-point reading.
- **Limb 3 halves its own cover.** Where a box wider than its rung is no
  graph, limb 3 halves that span of its certification net, under the
  crate's stop rule. The carrier is not touched. `refine_for_tube`
  retires.
- **Tube and pad are two claims.** The tube stays the widest rung that
  proves. The pad is the reach of the narrowest rung that still proves
  one arc, read box by box.
- **Measure first.** Before the build is dispatched, the pads near a
  node are measured on the as-marched carrier.
- **Storing the cover is the build's choice.** Whether the certificate
  records the halved cover or readers re-derive it by one function is
  left to the build, under the rule that the region banked is the
  region proved.

C2's "Refusal is typed, never a retry loop" stands unchanged. Ev reads
it as not contradicting this plan: halving the cover re-proposes no
carrier. C2's limb-3, mean-value, slack and pad sentences, its node-end
clause and C3's tube-rounds paragraph are rewritten.

## Measured before the build (near-node pads)

Measured on main `362cd72fd2` with a throwaway prototype that was not
merged (2026-11-17). The prototype has two switches. The first turns off
`refine_for_tube`'s rounds, so each branch is fitted and certified once,
as marched. The second reads each component's pad and reach with
`pad_and_reach`, box by box, over the chain padded by the narrowest
rung that proves. To find that rung, the prototype walks `tube_ladder`
from its narrowest rung upward and probes each rung with
`probe_tube_analytic`: the graph margin first, then `one_arc_r3`.
The tube is still the widest rung, which limb 3 picks as it does today,
and the winding and the accounting read it. The rows' assertions run
against the narrow pad and reach. "Today" is the same build with both
switches off. The graph reading is still today's two-point
`graph_margin`, because the one-point form was not prototyped. Pads
and tubes are in metres. Each figure is the largest over a pose's
components and both operand orders.

**The rounds act only at 1e-6.** At ε = 1e-9 (the default) and at
1e-12, every row's carrier, knot count, tube and pad are bit-identical
with and without the rounds. At those two bands, retiring
`refine_for_tube` moves nothing, so the pad change below is all there
is.

**The narrowest rung proves on its first probe.** On every carried
component, at all three bands, the ladder's narrowest rung (1.07e-5 at
1e-6, 3.3e-7 at the other two) is a graph and proves one arc. That
takes one graph probe and one `one_arc` run. Today's ladder stops at the
widest rung that proves, so the only extra cost is that one `one_arc`
run per component. No row needed a wider rung for the pad.

**ε = 1e-6** (knots, tube and pad: today against as marched):

| row, pose | today | as marched | narrow pad | row |
|---|---|---|---|---|
| two loops (0.1, 0.7) | 208, 4.38e-2, 9.84e-2 | unchanged | 2.44e-2 | passes |
| two loops (0.2, 0.71) | 219, 2.22e-2, 6.65e-2 | 213, 1.11e-2, 4.75e-2 | 2.86e-2 | passes |
| two loops (0.2, 0.701) | 238, 5.48e-3, 3.92e-2 | 217, 2.74e-3, 3.45e-2 | 2.99e-2 | passes |
| one loop (0.2, 0.69) | 420, 1.08e-2, 4.81e-2 | unchanged | 2.99e-2 | passes |
| near node R 0.7003 | refuses `boxes: 211` | refuses `boxes: 211` | none | (refusal allowed) |
| near node R 0.70097 | 239, 5.48e-3, 3.92e-2 | 217, 2.74e-3, 3.45e-2 | 2.99e-2 | passes |
| near node R 0.70164 | 217, 5.48e-3, 3.91e-2 | unchanged | 2.97e-2 | passes |
| near node R 0.70231 | 246, 1.10e-2, 4.83e-2 | 216, 5.49e-3, 3.89e-2 | 2.96e-2 | passes |
| near node R 0.70299 | 225, 1.10e-2, 4.82e-2 | 216, 5.49e-3, 3.88e-2 | 2.95e-2 | passes |
| near node R 0.70366–0.705 | 215, 1.10e-2, 4.79–4.81e-2 | unchanged | 2.92–2.94e-2 | passes |
| close rings δ 3e-3 | 212, 9.42e-3, 4.25e-2 | unchanged | 2.66e-2 | passes |
| close rings δ 3e-4 | refuses `boxes: 208` | refuses `boxes: 208` | none | red, as on main |
| sliver (0.2, 0.699) | refuses `boxes: 437` | refuses `boxes: 437` | none | red, as on main |

Without the rounds, the tubes drop one rung on the five poses the rounds
touched. The narrow pad does not depend on the rounds: on (0.2, 0.701)
it is 2.987e-2 on both carriers. The rounds halve spans near the tips,
and the pad comes from the reach of the largest hull box, which they
leave alone. The near-node row reads 2716 sites with 2 refused and none
on the wrong loop, both today and in the prototype. The 2 refused are
the R 0.7003 carry in each operand order.

**ε = 1e-9 (default)**, today's pad against the narrow pad. Every row
passes, the near-node row reads 3104 of 3104 sites, and both close-ring
widths carry:

| pose | knots | tube | today's pad | narrow pad |
|---|---|---|---|---|
| (0.1, 0.7) | 1139 | 4.38e-2 | 7.97e-2 | 4.27e-3 |
| (0.2, 0.71) | 1170 | 2.22e-2 | 4.32e-2 | 5.02e-3 |
| (0.2, 0.701) | 1192 | 5.48e-3 | 1.46e-2 | 5.25e-3 |
| (0.2, 0.69) | 2331 | 1.08e-2 | 2.37e-2 | 5.23e-3 |
| sliver (0.2, 0.699) | 2466 | 5.46e-3 | 1.47e-2 | 5.32e-3 |
| near node R 0.7003 | 1196 | 2.74e-3 | 9.96e-3 | 5.27e-3 |
| near node R 0.70097–0.705 | 1178–1192 | 5.48e-3–1.10e-2 | 1.46e-2–2.41e-2 | 5.12–5.25e-3 |
| close rings δ 3e-3 / 3e-4 | 1163 / 1181 | 9.42e-3 / 4.69e-3 | 2.08e-2 / 1.28e-2 | 4.66e-3 / 4.72e-3 |

**ε = 1e-12**, set out the same way. Every row passes:

| pose | knots | tube | today's pad | narrow pad |
|---|---|---|---|---|
| (0.1, 0.7) | 6374 | 4.38e-2 | 7.65e-2 | 7.57e-4 |
| (0.2, 0.71) | 6547 | 2.22e-2 | 3.93e-2 | 8.91e-4 |
| (0.2, 0.701) | 6674 | 5.48e-3 | 1.04e-2 | 9.32e-4 |
| (0.2, 0.69) | 13076 | 1.08e-2 | 1.95e-2 | 9.27e-4 |
| sliver (0.2, 0.699) | 13836 | 5.46e-3 | 1.04e-2 | 9.45e-4 |
| near node R 0.7003 | 6697 | 2.74e-3 | 5.65e-3 | 9.36e-4 |
| near node R 0.70097–0.705 | 6595–6675 | 5.48e-3–1.10e-2 | 1.04e-2–1.99e-2 | 9.08–9.32e-4 |
| close rings δ 3e-3 / 3e-4 | 6512 / 6614 | 9.42e-3 / 4.69e-3 | 1.71e-2 / 8.93e-3 | 8.27e-4 / 8.38e-4 |

**Answer: the pads near a node shrink.** They shrink on every carried
row at every band, the as-marched carrier included. The narrow pad is
0.25–0.76 of today's at 1e-6, 0.05–0.53 at 1e-9 and 0.01–0.17 at
1e-12. The smallest gain is nearest the node, on (0.2, 0.701) and
R 0.70097 at 1e-6 (3.92e-2 to 2.99e-2), where today's tube is already
the narrowest. Nothing needs halving to come back under today's pad.
Further halving of the pad's cover still pays, measured uniformly at
1e-6 on (0.2, 0.701). Each halving halves the pad: 2.99e-2, then
1.48e-2, 7.41e-3, 3.71e-3 and 1.86e-3 at 210, 420, 840, 1680 and 3360
boxes. The narrowest rung still proves one arc on every halved cover,
and the same holds on every row at every band.

**The three refusals at 1e-6, with limb 3 halving its cover.** This is
a rough prototype, run on the as-marched points. For each rung, from
the narrowest upward, every span whose part is no graph at that rung
and is wider than the rung is halved, for up to 8 rounds. Then
`one_arc_r3` runs on the result. Limb 2's bound was not read against
the pad here.

| pose | marched points | narrowest proving rung, boxes, pad | widest proving rung, boxes |
|---|---|---|---|
| sliver (0.2, 0.699) | 438 | 1.07e-5, 468, 3.03e-2 | 5.46e-3, 527 |
| near node R 0.7003 | 212 | 1.07e-5, 249, 3.00e-2 | 2.74e-3, 258 |
| close rings δ 3e-4 | 209 | 9.16e-6, 241, 2.69e-2 | 4.69e-3, 295 |

All three prove, and each took one `one_arc` run at its narrowest rung.
Their pads sit with the carried rows' pads. The sliver's widest rung is
the 5.5e-3 tube the clamped fit reached on main through the rounds. The
rows' site assertions were not run on these three, because the
prototype does not bank a halved cover in a certificate.

**What the measurement could not see.** The pad is read on the
carrier's clamped net over its span, as `Tube::of` reads it. The halved
covers are built by knot insertion on 32-span pieces. Inserting knots
across the whole net at once ran out of memory on 1100-span carriers,
so the build should not do that. Separately,
`pair_section::wiring::a_marched_loops_pad_holds_its_tube_and_its_limb_two_bound`
asserts that every corner of the tube's boxes lies within the pad. A
pad read off the narrow rung breaks that by design, so the build
rewrites the row.

## Also red on main at 1e-6 with this cause

- `pair_section::one_loop_keeps_its_order_near_the_node` carries this
  pose and refuses the same `TubeStraddles { boxes: 437 }`.
- `pair_section::wiring::close_rings_carry_counted`'s pin names
  `boxes: 206`; the periodic fit's chain has 208 (filed as
  `main-red-since-pr-7052-the-recourse-roster-and-a-close-rings-pin`).
- Three sweep and topo rows carry the same pose `(0.2, 0.699)` and
  refuse the same `TubeStraddles { boxes: 437 }` at ε = 1e-6 (GREEN,
  2026-11-22, `work/green/main-red-seven-sweep-and-topo-rows-red-at-1e-6-since-11-17.md`):
  `sweep` `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_wider_than_the_wall`
  (every op, at the germ section),
  `cylinder_sphere_frame::every_segment_joins_neighbouring_sites_along_its_loop`
  (the matcher on that pose) and `topo`
  `ring_path::spline_fuzz::a_path_across_a_carried_loop_reads_the_membership_parity`
  (`PairSection::carry` itself). On hosted CI at ε = 1e-6 all three
  pass at PR 7052's first parent `fa4e9ba921` and fail at its merge
  `6c2430e888`, and fail on main `12a43a7e68`. Each is pinned to
  that refusal at 1e-6 (the first by `test_utils::pinned_red::row`, the
  other two by an in-line check on the pose, as
  `pair_section::a_seated_loop_has_no_start` does), so it goes red the
  day the pose carries.

## 2026-11-25 — built (PR 7776)

The decision is built as decided: the one-point reading on plane,
sphere and cylinder pairs (`ssi::enclose::graph_quantity`), limb 3's own
cover halved under `crate::descent` (`ssi::certify`'s `CoverBase::at`),
`refine_for_tube` retired, and the pad read off the narrowest proving
rung's cover. The certificate stores no cover: `SsiTube::Spatial`
records the two rungs, and a reader re-derives either cover by the
function limb 3 proved over (`ssi::certify::tube_chain`).

**All three 1e-6 refusals carry, and so does `(0.2, 0.6995)`.** Measured
on the branch, the largest pad over a pose's components (ε = 1e-6 both
operand orders, the other two bands cylinder first). `boxes` is the
tube's chain against the carrier's first cover:

| pose | ε 1e-6: tube, boxes, pad | ε 1e-9: tube, pad | ε 1e-12: tube, pad |
|---|---|---|---|
| (0.1, 0.7) | 8.75e-2, 201/201, 2.44e-2 | 8.75e-2, 4.27e-3 | 8.75e-2, 7.57e-4 |
| (0.2, 0.71) | 4.44e-2, 206/206, 2.86e-2 | 4.44e-2, 5.02e-3 | 4.44e-2, 8.91e-4 |
| (0.2, 0.701) | 2.19e-2, 210/210, 2.99e-2 | 2.19e-2, 5.25e-3 | 2.19e-2, 9.32e-4 |
| (0.2, 0.69) | 4.31e-2, 413/413, 2.99e-2 | 4.31e-2, 5.23e-3 | 4.31e-2, 9.27e-4 |
| sliver (0.2, 0.699) | 2.18e-2, 437/437, 3.03e-2 | 2.18e-2, 5.32e-3 | 2.18e-2, 9.45e-4 |
| (0.2, 0.6995) | 1.09e-2, 438/438, 3.02e-2 | 1.09e-2, 5.31e-3 | 1.09e-2, 9.42e-4 |
| near node R 0.7003 | 1.09e-2, 211/211, 3.00e-2 | 1.09e-2, 5.27e-3 | 1.09e-2, 9.36e-4 |
| near node R 0.70097–0.705 | 2.19e-2–4.41e-2, 2.92–2.99e-2 | 2.19e-2–4.41e-2, 5.12–5.25e-3 | 2.19e-2–4.41e-2, 9.08–9.32e-4 |
| close rings δ 3e-3 | 3.77e-2, 205/205, 2.66e-2 | 3.77e-2, 4.66e-3 | 3.77e-2, 8.27e-4 |
| close rings δ 3e-4 | 9.38e-3, 208/208, 2.69e-2 | 9.38e-3, 4.72e-3 | 9.38e-3, 8.38e-4 |

The pads are the "narrow pad" column of the measurement above at every
band, to its three figures. The tubes are one or two rungs wider than
main's on every row (the one-point reading), and the same at all three
bands.

**On these poses nothing is halved.** With the one-point reading every
box of every cylinder × sphere carrier here is a graph at its first
reading, at the tube's rung and at the narrowest. The halving acts on
pairs that read two points: at ε = 1e-6 the ball round the donut's
axis proves its tube 7.8e-3 over 293 and 235 boxes of 200 and 195, where
main reached the same rung by re-marching to 329 and 273 samples.

**Three things the decision did not say, decided in the build**, all in
the PR body's Choices. The third departs from the plan as written and is
the PR's one question:

- A site's runs stay on the tube's chain. Read off the narrow chain,
  `cylinder_pair_section::a_region_through_a_branch_point_escalates`
  desyncs at ε = 1e-6: an open arc's cut end stands 3.7e-5 past its
  carrier's end, outside a box padded by 1.07e-5.
- A box is not halved where the rung's own box about the carrier's
  point at the middle of its span is no graph: its padding holds it,
  and no halving narrows that. Without it a rung that fails on its
  padding halves every box to the work bound, and
  `pair_section::the_count_is_the_morse_count_at_every_pose_near_every_critical_height`
  ran 1116 s at ε = 1e-6 against main's 226 s.
- **A loop's winding is read over the narrow chain, not the tube.** The
  plan had the winding read the tube. With the one-point reading the
  tubes are a rung or two wider, and
  `pair_section::wiring::the_cylinder_sphere_coverage_keeps_its_headroom`
  refused `Winds(Unordered)` at ε = 1e-12 on its turned one-loop pose:
  the tube's boxes meet across the small loop, so the walk through
  them is not shown once round. Read over the narrow chain, which limb 3
  proves one arc the same way, the row passes at all three bands.

The rows pinned on this pose are unpinned and pass at ε = 1e-6:
`pair_section::a_seated_loop_has_no_start`,
`one_loop_keeps_its_order_near_the_node`,
`pair_section::wiring::close_rings_carry_counted`, and the three sweep
and topo rows.
