---
id: not-crossed-means-one-side-at-a-pinch
kind: issue
title: the construction sites' 'not crossed, so one side' premise is the division's (measured, none disagrees on a valid body); its checked form waits on [ev] 6592's Alike
status: parked
opened: 2026-11-09
priority: P3
cost: E
blocked_on: [6592]
---


## Finding

Filed by `onecopy/side-at-infinity` (PR 6595's review). That PR found
`whole_body_side`'s premise, "the plane does not cut the body, so it lies
on one side", false at a pinch: two lumps that meet only within the plane
lie on opposite sides. The same shape of premise underwrites other
`Answering::Any` ladders, which take their first answer:

- `boolean/fate.rs`, the ring rehome (~1358): "a ring the cut does not
  cross is on one side".
- `one_region.rs` `read_cycle` (~233).
- `chord_join.rs`'s side ladders (the premise at ~4021; climbs at ~3981,
  ~4042, ~4086, ~4834, ~5470, ~5549).

A ring or cycle that meets the dividing loop only at a vertex, with its
two halves on either side of the loop there, is not crossed and yet
reads both sides. **Probe:** for each site, build a ring that touches the
dividing loop at one vertex (a figure-eight pinch), read every rung, and
log whether the answers disagree, as PR 6595's scratch instrument did.
Disagreement makes it a defect at that site; agreement for a reason the
site can state settles the premise.

## Narrowed (`onecopy/pinch-premise`)

The premise holds where the loop does not cross the run. A simple loop
that meets the run only at touches that do not cross lies on one side,
because a touch switches no side. That covers a touch at a vertex the loop
shares with the run, and a touch at a copy of one: the split's reduction
keeps one vertex per piece at a pinch, so a ring can touch the run at a
vertex it does not share, and it reads `OnBoundary` there and passes
(`chord_join::ring_side`). The argument has one home, `chord_join::paths_rung`'s
premise doc. Fate's rehome, `one_region::read_cycle`, chord_join's
`ring_side`, `path_ring_side` and the lone ring's outer side cite it.

**What remains** is not specific to figure-eights:

- **A loop that crosses the run at a vertex the division did not
  identify.** Such a loop reads both sides, where the division should
  have joined it. A touch there still reads one side and passes. The
  paths that can produce it are fate's rehome (the
  boolean's cut through a divided face) and `ChordJoiner::rehome_rings`.
  The plane split identifies every vertex within band of the plane, so
  it cannot. No probe of this case is built. Its guard is the debug check
  on the `Any` ports that their answers agree
  (`a-ladder-that-checks-its-answers-agree-has-no-home`), which waits on
  `[ev]` PR 6592.
- **A torus face's essential cycles** in `one_region`, which are not
  probed.

`one_region` is a validator, and nothing joins there. Two distinct loops
of one face meeting at a point are check 9's contact arm 1
(`RingMeetsOuter` / `RingMeetsRing` `Vertex`, any carrier, shared keys
not exempt). Two surface cycles of one loop, split by a seam or a strut,
that share a point are check 9's `PinchCornerCrossed`, which
`one-region-reads-a-cycle-crossing-at-a-shared-vertex-by-its-first-vertex`
found and closed.

**Evidence that a touch at a pinch reads one side.**

- *The instrument.* A scratch check read every rung at fate's rehome,
  `one_region::read_cycle`, chord_join's planar `ring_side`, the lone
  ring's outer side, the quadric ring pairs and the winding references.
  It found no disagreement at these sites over topo ci and sweep ci.
- *The figure-eight row.* It ships as a test,
  `crates/topo/tests/a_figure_eight_hole_cut_through_its_pinch.rs`. A
  plate with a figure-eight hole, one loop through the origin twice, is
  built raw (`drill_hole`, described, `finish`), since the public doors
  refuse it (`LineWedges` overlap). It is then cut through the pinch by
  `split` (x + y = 0) and by `subtract` and `intersect` against a prism
  whose face lies on x + y = 0. Each side keeps its own lobe, merged into
  its face's outer loop, and the boolean results pass the census.

The same instrument found one real disagreement that is not a pinch: a
cone winding whose fixture's island left its sheet,
`cone-ring-rows-sheets-do-not-hold-their-islands` (PR 6607).

## Narrowed (`onecopy/pinch-side`): measured, and waits on `[ev]` PR 6592

**Measured: no reading answers wrong, and none disagrees on a valid body.** A scratch
check read every rung beside the `Any` climb (`climb_agreeing`) at four
sites: fate's rehome, chord_join's `ring_side`, the lone ring's outer
side and `one_region::read_cycle`. It ran over:

- **topo `ci` (3746 tests).** It found two disagreements, both in
  `read_cycle`, on the bodies of
  `two_surface_cycles_of_one_loop_crossing_at_a_shared_point_are_check_9s`
  and `a_ring_crossing_its_outer_loop_at_shared_points_is_check_9s`.
  These are invalid bodies whose crossing check 9 refuses, as those
  tests pin.
- **sweep `ci` (2874 tests).** None.
- **Targeted fixtures.** A plate holed by a diamond, or by an arrow with
  a reflex vertex, was cut through the ring's vertices along `x = d`,
  `x + y = d`, `y = 0` and a quadrant, by subtract, intersect, union and
  split, with `d` at 0, ±½ zero, 3 zeros and ±3 escalation bands.
  - At `d` = 0 and inside the zero, the division identifies the
    vertices and divides the ring there.
  - In the band, it refuses: the booleans on `Coincidence(VertexOnFace)`,
    the split on `SliverVertex`.
  - Past the band, it divides the ring's edges.
  - Every result's volume is its closed form, and subtract plus
    intersect is the plate.

The torus item is measured too. sweep `ci` reads a torus face's
essential pair 3639 times in 337 tests, 29 of them with contractible
cycles read against the pair, and finds no disagreement.

So the premise holds, and it is a construction premise. A bystander
ring does not cross the run, because the division identifies every
vertex within the zero of the cut and refuses every vertex in its band.
The residual case, a crossing at a vertex the division did not
identify, is therefore not reached by any fixture or suite measured.
Pinned:
`crates/topo/tests/a_ring_crossed_at_its_vertices_is_divided_or_refused.rs`.

**What remains: making the premise a checked fact.** That is `[ev]` PR
6592's `Alike`: the rung source carries a token whose doc names the
gate that establishes its premise, here the division's vertex
identification. Reading every vertex at each site (`climb_agreeing`)
would be the alternative 6592 rejects, a cost on valid bodies for a
weaker second reading of a fact the division decides. This row waits on
that answer. Then the sites read their ring through an `Alike` source:
fate's rehome, `ring_side`, `path_ring_side` and the lone ring
(`one_region::read_cycle` moves out, below).

### The validator's site moves out (review of PR 6651)

`one_region::read_cycle` is a validator. No division runs there, and its
premise is check 9's, not the division's: both topo `ci` disagreements
above are at `read_cycle`, on invalid bodies. It is taken by PR 6649
(the gate-blind-spots lane's `loop_side::inside_at_every_vertex` at
`read_cycle` and `ring_nesting`), and leaves this row. What stays,
parked on 6592, are the construction sites:
- fate's rehome;
- chord_join's `rehome_rings` / `ring_side` and `path_ring_side`;
- the lone ring's outer side. Its premise is the division's too: the
  ring meets the pierced face's boundary only at its one site, and on
  the sphere its plane meets the surface in the ring alone
  (`RimSide::holds_outer`).

**Families measured but not shipped as tests.** The pin ships the
diamond cut along `x = d`, at `d` ∈ {0, ½z, 3z, 3e}, by subtract,
intersect and split. Measured and not shipped:
- `d` = −½z and −3e;
- the cuts `x + y = d` (and a tilted line through one corner), `y = 0`
  and the quadrant `x, y ≥ 0`;
- union beside subtract and intersect;
- the reflex arrow ring `(1,−1), (0,0), (1,1), (−1,1), (−1,−1)` under
  every cut above.

Every one read its closed-form volume, or refused in band.
