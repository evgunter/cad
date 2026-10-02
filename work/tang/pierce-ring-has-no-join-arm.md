---
id: pierce-ring-has-no-join-arm
kind: issue
title: A pierce RING has no join arm on any carrier: three typed doors, one missing lane
status: closed
closed: 2026-10-02
opened: 2026-08-30
github: 1291
refs: [347, 1068]
priority: P0
cost: H
---

## From GitHub issue 1291

Opened 2026-08-30; 0 comments.

A **pierce ring** — the empty loop `vtxfac` mints inside a pierced face,
carrying only null-edge scaffolding — has no join arm on any carrier.
Three typed doors are the same missing lane wearing three names, and
none of them is a bug in the layer that reports it.

## The three doors, measured

| carrier | fixture | door |
|---|---|---|
| planar cap | a box driven through a cylinder CAP (`verbs_pierce.rs`, shipped by #1068) | `SplitJoinError::SectionLoopMixed` |
| cylinder wall | a bar driven through a cylinder WALL (`verbs_germarms.rs`) | `SplitJoinError::SectionArcWindow { case: NoChartedRun }` |
| cylinder wall, asymmetric pose | an off-centre bar, same half-wall (R2's probe) | `SectionArcWindow { NeitherContained }` *(see the diagnosis below — this pose now stops one layer earlier)* |

**Why they differ, and why neither reporting layer is at fault.**
`NoChartedRun` fires because `run_azimuth_window` skips null
scaffolding by contract ("zero-length, no azimuth extent"), so a run
made of nothing else leaves the divided face windowless — every time,
by construction. `SectionLoopMixed` fires because a ring's section loop
has no above/below-paired boundary to join at all; its variant doc says
"kernel bug, loudly", and that sentence is correct for every other way
of arriving there. Both docs have been amended in place to name the
pierce ring as a legitimate typed destination pending this unit, rather
than leaving a falsified sentence standing (VERBS-GERMARMS PR-1).

## The `NeitherContained` diagnosis

Asked for by the PR-1 review: is the asymmetric pose's
`NeitherContained` mis-bookkept run/chord pairing, or honest
ill-conditioning?

**The site already answers it in prose** (`chord_join.rs`, the
azimuth-window rows): *"The chord's start lying in the window is a
consequence of the run's own geometry, not an assumption: a run that
does not actually end where this chord starts fails the x₁ rows and
lands in `NeitherContained`."* So on that pose the run handed to the
arc-side rule does not co-bound the face with the chord it was paired
with — a PAIRING question, not a degenerate window.

**It is not currently reproducible**, and that is itself the finding:
after PR-1's curvature charge landed, the same off-centre fixture stops
one layer EARLIER, at `CurvedSectorSideUnsupported` — the sector-side
verdict on that pose cannot be certified against the wall's curvature,
so the crossing layer never hands the join anything. The pairing
question is therefore parked with its evidence rather than answered:
this unit owes it a fixture that reaches the join on an asymmetric
pose, and the run/chord pairing is the first thing to read there.

## Scope

- Give a pierce ring's run its own chord lane, on both carriers — a
  planar face's ring and a wall face's ring are the same topological
  object and should not be two separate arms if the walk can be shared.
- Re-read the run/chord pairing on an asymmetric pose (above).
- Retire the two amended doc paragraphs when the arm lands.

## Consumers waiting

- #347's union half: a wall pierce reaches the join and stops there.
- VERBS-GERMARMS PR-2 (the cyl×cyl germ arm) — its own chord lane is a
  different question, but a Steinmetz union that mints ring vertices
  needs this one too.
- The planar cap row in `verbs_pierce.rs`, which has been refusing at
  the join since #1068.
- **A user's blind pocket in a cylinder's end cap** (2026-09-18, Ev,
  in a scratch document of theirs that is not in version control;
  the recipe below is the whole reproduction, cut to the nodes the
  failing one reads, lengths in m):

  | node | recipe |
  |---|---|
  | 8 | `Datum Frame { origin (0,0,0), u (1,0,0), v (0,0,1) }` (the XZ frame; normal −y) |
  | 9 | `Profile { plane 8, Circle { centre (0,0), radius 0.04 } }` |
  | 10 | `Extrude { profile 9, distance −0.5 }` (cylinder along +y, y ∈ [0, 0.5]) |
  | 15 | `Profile { plane 8, Chain [At (0, 0.02), LineTo (0, −0.02), LineTo (0.008, −0.02), Tangent, TangentArcTo (0.01, 0), ArcTo Bulge { target Start, b 0.6 }] }` (a letter-ish outline) |
  | 16 | `Extrude { profile 15, distance 0.01 }` (y ∈ [−0.01, 0]) |
  | 17 | `Transform { input 16, translation (0, 0.005, 0), rotation 0 about z }` (y ∈ [−0.005, 0.005]) |
  | 19 | `Boolean { op Subtract, a 10, b 17, declare None }` |

  The tool's section with the y = 0 cap is a closed loop strictly
  inside the disc (x ∈ [0, 0.01], z ∈ [−0.02, 0.02]). Node 19 refuses:
  `boolean op: joining refused: split join: null face FaceKey(12v1)
  has a side-mixed section loop (kernel bug)`, from the backstop in
  `crates/topo/src/boolean/join.rs:1854` — the site whose comment says
  no witness that reaches it is known; this pose and the cap row above
  are two. Everyday CAD feature, far more common than the
  box-through-cap row: engraving, a keyway, a sunk pocket in a face.
  The pose and three controls, each replayed through
  `editor_core::persist::load` + `evaluate`:

  | variant | result |
  |---|---|
  | as drawn (lines + arcs) | `SectionLoopMixed` |
  | node 15 a plain rectangle (0, ±0.02)–(0.01, ±0.02) | `SectionLoopMixed`: the arcs are not the cause |
  | node 9 a square (±0.04, ±0.04), so node 10 is a box; either node 15 | **OK**: a ring in a line-bounded planar face joins |
  | node 17 translated (0.035, 0.005, 0), so the pocket crosses the rim | a different door: the curved pierce arm's typed frontier (`boolean/reduce.rs`, "a Circle carrier … stay at this typed frontier") |

  So the missing arm is specifically the ring in an **arc-bounded**
  planar face, the planar-cap door of the table above.
  The user reads the refusal as `… side-mixed section loop (kernel
  bug)`: `SplitJoinError`'s `Display` (`crates/topo/src/chord_join.rs:357`)
  still says "kernel bug", though the variant doc was amended to name
  the pierce ring as a legitimate typed destination. The message
  should say that too until this arm lands.

## Home

`work/verbs/` — the missing lane is the germ-arm/pierce ground VERBS' charter claims (Wave 2's curved boolean breadth), and S-BOOL's keep_out cedes the germ arms to VERBS explicitly.

**VERBS closed** (exit walk ratified, PR #1793); re-homed to
`work/issues/` awaiting an owner.

**Adopted by CURVED** at its opening for dispatch (2026-09-04, Ev's
in-chat direction): the plan's lane that carries this item is in
`work/curved/plan.md`.

## 2026-09-24 — the planar-cap door was point-in-solid's misread (ATREST-9)

The `planar cap` row of the table above (`verbs_pierce.rs`, a box
driven through a cylinder cap → `SectionLoopMixed`) now unions to the
truth, `6.643185307179586`, tier 3 `Ok`; so does
`verbs_pierce_r1_probes`' box through an annular cap. The join
resolves a pierce region's role by probing the other operand with
`point_in_solid` (`join.rs`, `resolve_roles_geometric`), and that
walk's planar arm read the arc-bounded cap as its zero-area vertex
polygon — so the probe read the cap as transparent and the role came
back mixed. Fixed on `atrest/9-pis-torus-pose` (the planar arm crosses
arcs on their circles). Which of this row's other doors (the wall
rows, the engraving pose) shared that cause is unmeasured; the
engraving pose's "a ring in a line-bounded planar face joins" control
is consistent with it.

## Evidence (2026-10-01, PR 3659): a SPHERE face, the spun snowman

Two full-revolve balls on one axis (r 1.0 at `y = 0`, r 0.8 at
`y = 1.4`) build under every boolean when their seams are coplanar —
each seam meridian then pierces the other sphere ON the other's seam.
Spin B about the shared axis by any angle other than `0` or `π`
(measured by the PR's review at 1e-6, 1e-3, 0.1, 0.9, π/2, 4.0 and
2π − 1e-3, at all three ε rows) and every op refuses
`Join(SectionArcWindow { case: NoChartedRun })`: the pierce lands
inside B's half-band, so the pierced sphere face carries this row's
ring. Pinned by `crates/sweep/tests/snowman.rs`,
`a_spun_snowman_refuses_at_the_pierce_ring_door` (1e-3, 0.9, π/2). The
sphere pair's join arm itself (`boolean::join`, the radical plane on
both sides) has nothing missing for this pose — the section and its
plane are the coplanar pose's — so this is the third carrier on which
the ring lane, and only it, is owed.

## Evidence (2026-10-02, the circle × cylinder cell): parallel cylinders that pierce

Two unit cylinders, `z ∈ [0, 2]` and `z ∈ [0.5, 2.5]`, axes `d` apart,
`d ∈ {0.3, 0.8, 1.2, 1.6, 1.9}`: each rim circle's pierce of the other
wall is certified by the circle × cylinder root lane and passes its
sector side, and every op refuses
`Join(SectionArcWindow { case: NoChartedRun })`. Pinned by
`crates/sweep/tests/tang_circle_cylinder.rs`,
`parallel_cylinders_that_pierce_stop_at_the_pierce_ring`.

## 2026-10-01 — re-measured on main (TANG)

Every door was measured at `origin/main` `6000ec92d`, under all three ops
where the fixture is a `topo` pair. Refusal texts are verbatim `Display`.

**Summary: a pierce ring in a PLANAR face joins whatever bounds that
face. No row of this item reaches a planar door any more.** Both planar
arrivals, the box through a cap and Ev's engraving pose, now build to
their closed forms. `chord_join::chord_spec` answers a planar face
before it reads any window: its `Surface::Plane` arm returns the straight
chord (`JoinLane::Planar` → `Ok(None)`) and never reads the run's
carriers. The missing lane is the ring on a CURVED face only: the
cylinder wall (two sub-cases, below) and the sphere.

| door | fixture | today |
|---|---|---|
| planar cap | box through a cylinder cap (`verbs_pierce`) | **builds**: ∪ 6.643185307179586, − 5.923185307179587, ∩ 0.36, tier 3 `Ok` |
| planar cap, blind pocket | the same box ending inside the cylinder (z ∈ [1.5, 3]) | **builds**: ∪ 6.643185307179586, − 6.1031853071795865, ∩ 0.18, tier 3 `Ok` |
| Ev's engraving pose | nodes 8–19, through `editor_core::evaluate` | **builds** (below) |
| cylinder wall, symmetric bar | `verbs_germarms`, `brick((−1.1, 1.1), (−0.3, 0.3), (−0.3, 0.3))` | live: `SectionArcWindow { face: 3v1, case: NoChartedRun }` |
| cylinder wall, one-sided bar | `brick((0.5, 1.1), …)` | live: `NoChartedRun` at face 4v1 |
| off-centre bar, long | `brick((−3, 3), (0.15, 0.7), (−0.4, 0.1))` | live, before the join: `CurvedSectorSideUnsupported { verdict: Negative { margin: −1.955 } }` |
| off-centre bar, short | `brick((−1.1, 1.1), (0.15, 0.7), (−0.4, 0.1))` | live, **at the join**: `SectionArcWindow { face: 3v1, case: NeitherContained }` |
| sphere | spun snowman (`snowman.rs`) | live: `NoChartedRun`. The pinned row passes on main |

### Ev's engraving pose: it builds, and it was the same misread

The recipe was authored node for node through the document doors
(`crates/editor-core/tests/pierce_ring_engraving.rs`). Node 19 builds,
and tier 1, closed and tier 3 (`validate_geometric`) are all `Ok`. Its
volume is `0.0025106399340038997` against the closed form
`π·0.04²·0.5 − A·0.005 = 0.0025106399340038992`. Here `A` is the
letter's area, `5.268377735870561e-4`: the vertex polygon (`2.8e-4`)
plus the two circular segments, both bowing out. The tangent arc has
radius `0.0101`, and the bulge arc subtends `4·atan 0.6`. The tool's own
prism measures `A × 0.01` to `2e-21`.

The three controls:

- **Node 15 as a plain rectangle**: builds, `0.0025112741228718346`, the
  closed form.
- **Node 9 a square, either tool**: builds, `0.003197365811132065`
  (letter) and `0.0031980000000000008` (rectangle).
- **Node 17 slid to x = 0.035, across the rim**: a different door, and
  not a pierce ring:

  > the Boolean op refused: an edge of the first operand touches or
  > crosses a curved face of the other operand away from that face's
  > edges, and the Boolean cannot yet settle where or whether it passes
  > through. Recourse: declare the coincidence, or move the geometry

  The error is `CurvedPierceUnsupported { operand: A, face: 6v1, edge:
  1v1 }`. The edge is the cylinder's rim CIRCLE and the face is one of
  the tool's arc walls, a CYLINDER. It is raised by
  `boolean::reduce::curved_face_arm`'s circle-carrier arm
  (`Ok(Sign::Zero | Sign::Negative) => return Err(frontier())`). Circle ×
  sphere and circle × torus have root lanes; circle × cylinder has none,
  which is #347's remainder (`work/tang/boolean-refuses-on-arc-carrier-not-arc.md`).

**Cause.** The 2026-09-18 `SectionLoopMixed` came from the same
`point_in_solid` misread that ATREST-9 fixed. Three facts support this:

1. `SectionLoopMixed` on this path comes from `boolean::join::loop_roles`,
   which fires when both section loops' region readings agree. Those
   readings are `boolean::shell_witness::complex_side` →
   `point_in_solid` against the OTHER operand. For this `Subtract`, that
   operand is the cylinder with its arc-bounded cap.
2. Nothing else on the planar path reads arcs: `chord_spec`'s plane arm
   is carrier-blind (summary above).
3. The controls separate the cases the same way. With the arc-bounded
   cap the pose refused whatever the tool's carriers were. With the
   line-bounded cap it joined. The arc-bounded cap is the face the role
   probe reads, and it is the face ATREST-9's walk now crosses on its
   circles.

This checkout is shallow (history ends at a graft), so I could not
rebuild the pre-ATREST-9 tree to bisect. The attribution rests on the
three facts above, not on a before/after run.

**The message.** `SplitJoinError`'s `Display` (`chord_join.rs`, the
`SectionLoopMixed` arm) still reads `null face {face:?} has a side-mixed
section loop (kernel bug)`. Both measured arrivals now build, so no
known row reaches `loop_roles`' agreeing-verdicts arm, and "kernel bug"
is accurate again; `loop_roles`' own doc already says "No row reaches
it". The variant doc's paragraph is now stale: it says this row
"records the other measured arrival (an engraving pose) and that its
cause is unmeasured against this one". That paragraph needs a reword in
the kernel; this lane changes no kernel text.

Pinned: `pierce_ring_engraving::a_blind_pocket_in_a_cylinder_cap_cuts_to_the_closed_form`
(letter and rectangle, all tiers, volume to `1e-15`), and
`a_pocket_across_the_rim_stops_at_the_circle_cylinder_pierce`. The second
pins the rim door by kind and by its two carriers, so it goes red when
the circle × cylinder lane lands.

### The cylinder wall: two sub-cases of one missing lane

Display texts:

> the operands' sections could not be joined: the section through a
> curved face has no arc to take: the joined run carries no edge with a
> closed-form chart image, so the divided face has no azimuth window

> the operands' sections could not be joined: the section through a
> curved face has no arc to take: neither arc of the section conic lies
> inside the divided face's azimuth window (a degenerate window)
> ('split_arc_window', band (1e-9, 1e-8)). Recourse: declare the
> coincidence, move the geometry, or lower the tolerance

Both come from `chord_join::chord_spec`'s arc-side tail for a cylinder or
sphere face. `NoChartedRun` fires when `run_azimuth_window` returns
`None`. `NeitherContained` fires when neither candidate arc passes the
window's x₁ rows.

**What picks the sub-case.** I scanned bar poses against the pipe
(`r = 1`, `z ∈ [−2, 2]`, arms `x = ±1.1`). The pipe's wall is TWO faces,
split at the seam rulings `(±1, 0, z)`: face 3v1 holds `y > 0` and face
4v1 holds `y < 0`.

| bar `y` (z ∈ [−0.3, 0.3] unless noted) | door |
|---|---|
| [−0.3, 0.3], also with z ∈ [−0.4, 0.1] | `NoChartedRun` (3v1) |
| [−0.2, 0.35] | `NoChartedRun` (4v1) |
| [0, 0.55] | `NoChartedRun` (3v1) |
| [0.15, 0.7], also with z ∈ [−0.4, 0.1] | `NeitherContained` (3v1) |
| [−0.7, −0.15] | `NeitherContained` (4v1) |
| [−0.1, 0.45] / [0.05, 0.6] / [0.1, 0.65] | `CurvedSectorSideUnsupported`, margins −5.26e-4 / −5.19e-3 / −5.26e-4 |

The two sub-cases split by one condition:

- `NoChartedRun`: the section crosses or touches a seam ruling, so each
  half-wall's section runs seam to seam.
- `NeitherContained`: the section closes INSIDE one wall face, clear of
  both seams. Each side's closed loop is two rulings and two arcs,
  4 pierce vertices.

My reading, which I inferred from the site's code and did not
instrument: in the in-face case the run handed to an arc chord is
charted, because it carries section edges minted earlier in the same
loop. A ruling has a single azimuth, so neither 36° candidate arc fits
the window. That is the "pairing" reading the site's x₁ prose names,
not an ill-conditioned operand.

This pays the fixture debt the `NeitherContained` diagnosis above
records. The short off-centre bar reaches the join on an asymmetric pose.
`verbs_ga_r2_probes::r2_an_off_centre_bar_reaches_the_same_join_door`
already reached it, but it matched only `SectionArcWindow { .. }`. The
row's scope ("re-read the run/chord pairing on an asymmetric pose") can
start from this fixture. It is pinned by case as
`verbs_germarms::a_bar_whose_section_closes_inside_one_wall_face_reaches_the_join`.

The three short-arm poses that stop at `CurvedSectorSideUnsupported`
with small margins are a sector-side verdict, not this item. I did not
chase them.

### The sphere

`snowman::a_spun_snowman_refuses_at_the_pierce_ring_door` (1e-3, 0.9,
π/2, all three ops) passes on main and still reaches `NoChartedRun`. It
is the same `chord_spec` tail: the `Surface::Sphere` arm takes the
cylinder's chart frame (centre, polar axis, radius, seam).

### What the item now owes

The planar half of the scope is done, with no arm missing. These
consumers are served:

- the planar cap row in `verbs_pierce`;
- the "user's blind pocket" consumer (Ev's pose);
- `verbs_pierce_r1_probes`' annular, slot, rounded-rectangle and
  half-disc caps.

These rows all pass on main.

What is left: a ring's chord lane on a cylinder face, where both the
seam-to-seam `NoChartedRun` and the in-face `NeitherContained` occur,
and on a sphere face. Retiring the `NoChartedRun` doc paragraph waits on
that lane. The `SectionLoopMixed` paragraph can be reworded now (above).

## 2026-10-01 — the wall rows reach the join again, asymmetric pose included (REACH)

The sector-side curvature charge (`boolean::sectors::side_code`) is now
read at the distance along the bound where it is largest
(`min(slope·R/2, reach)`) instead of at the sector's arm and the bound's
far end. A bound longer than the wall's radius no longer refuses
`CurvedSectorSideUnsupported` when its first-order side is definite, so
the poses that stopped one layer early now reach this unit's doors:

- **`NeitherContained` is reproducible again**, on two asymmetric
  poses: `verbs_ga_r2_probes::r2_an_off_centre_bar_reaches_the_same_join_door`
  (the R2 probe's own long bar, `x = ±3`, offset `y ∈ [0.15, 0.7]`) and
  `germ_torus_doors::a_three_face_cylinder_rod_union_reaches_a_typed_door_not_a_body`
  (a rod through a three-face wall, under all four ops). That is the
  fixture the diagnosis section above asked for; the run/chord pairing
  is still the first thing to read there.
- **`NoChartedRun`**, new consumers: the round crenellation (a slab cut
  through a drum, the rook a user carves —
  `editor-core` `reach_slab_cut_sector_side`, whose rows check the
  closed-form volume once this arm lands), a slab across a round boss
  in every union member order (same file), the long bar
  (`verbs_germarms::a_long_armed_bar_reaches_the_same_join_door`),
  `block ∖ cylinder` grooves (`review_fillet_h7_r1_probes`) and the
  axis laps (`axis_lap::laps_off_the_rulings_stop_at_the_wall_pierce_ring`).
- **A sphere face, too**: the bar through the ball
  (`snowman::a_bar_through_a_ball_crosses_the_sphere`, every op at every
  ε row) now reaches this door rather than the sector side.
- The story suite's rook keeps its square crown for this door; its
  module docs say so (`crates/viewer/tests/story_authoring.rs`).

## 2026-10-02 — closed: the ring joins on a curved face (`tang/pierce-ring`)

**Neither wall door was the ring's scaffolding.** Instrumented on main:

- `NoChartedRun` (the bars through the wall, the laps, the spun
  snowman) came from the cross-loop (`mekr`) join's SECOND chord, which
  was handed `run_halves = []` — computed only for a same-loop join.
- `NeitherContained` (the in-face pose) came from the `mekr` arm
  reading its window from the TARGET RING's cycle: by then that ring
  held a ruling and its mate, a window one azimuth wide. This is the
  PAIRING reading the site's x₁ prose named.

**The fork, and what settled it.** The ring stays a ring: no bridge
chord, no hole bookkeeping on the chart. A cross-loop chord selects its
arc by the S9 containment rule against the divided FACE's own window
(`chord_join::cross_loop_window_cycle`, the face's outer cycle), the
statement `bool_planar_chord_spec` already asks of the mate's whole
face window. The code settled it; no C-clause names a ring
representation.

**What else the curved ring needed**, each a carrier generalisation of
an existing planar door:

- the join's ring lane (`boolean::join::choose_roles`) winds the island
  on the wall's chart, `−∮ v du`, closed exactly along the section
  plane (`chord_join::chart_island_winding`; same predicate,
  `bool_ring_run_winding`);
- ring re-homing reads a cylinder or sphere chart by ray parity
  (`chord_join::chart_ring_side`);
- the cylinder flux is its chart Green form over every loop
  (`geom_brep::props::curved_face_loops`), so a notched or ringed wall
  measures — which retires
  `work/props/a-notched-cylinder-wall-has-no-volume-measurement.md`.

**Rows moved.** The bars through the wall (symmetric, one-sided, long,
and the in-face pose) build under ∪, ∩ and both ∖ at tier 3 and to the
closed form; so do the laps, the grooves, the three-face rod, the
off-centre bars, the crenellation, the boss in four member orders, and
the spun snowman. The parallel cylinders now reach the cylinder pair's
join (`work/tang/cylinder-pair-germ-has-no-join-arm.md`); the bar
through a ball reaches the polar gate (`SectionNotPolar`); the boss's
two plate-last orders stop at `point_in_solid`'s ringed-wall outline
(`work/contact/point-in-solid-refuses-a-ringed-cylinder-wall.md`).
