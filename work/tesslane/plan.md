# TESSLANE — the plan

The slate is the item files beside this plan
(`python3 scripts/work.py status --program tesslane`); live state is
the tail of `log.md`.

## Charter

A body that builds, passes the tiers and measures is drawn, or refused
typed with a reason that names what is not built yet; and each lane
meshes as coarsely as its own worst-triangle theorem allows
(`crates/mesh/README.md`, `[mesh.sizing]`). The standing goal this
serves is authoring arbitrary geometry through the UI: a body the
viewer cannot draw is where that goal most visibly breaks.

## The two halves of the slate

- **Reach.** Faces no lane meshes: a (chart, boundary) pair the
  trimmed lane has no arm for (a torus or plane face bounded by a
  spiric, a cone face cut by a conic, a sphere face bounded by a
  spline), a ring on a curved face, a sphere face around or near a
  chart pole, a thin arc-bounded planar face.
- **Sizing.** The density each lattice emits and what proves it: the
  one-count grid axes, the arms that size at `UNPROVED_HEADROOM` for
  want of a theorem, split rims, the NURBS lattice's share, rows and
  cells, and one δ for a whole body.

A new arm lands with its theorem or with its headroom filed, so a reach
unit reads the sizing rows of the lane it extends before it is specced.

## The chart lane

The reach rows are one gap (the 2026-11-14 measurement, in `log.md`): a curved face is routed on the kinds of its outer loop's carriers, the trimmed lane constructs three arms, and both curved lanes return at a ring. Two designers weighed what a non-rectangular curved face is meshed from and reached the same final state independently; their reports are in the body of the PR that wrote this section.

**The final state.** One lane meshes every non-planar face: today's trimmed lane, with what it holds per arm factored out.

- **The face's domain is read from the body, every loop.** A loop's lift is its stored chart images placed end to end; the domain is the outer loop's lift less the rings'. Boundary points are the shared chord points, interior points are a lattice, and inside or outside is the planar lane's even-odd count.
- **A chart is one table row**: lattice steps for a target, the per-triangle certificate, the metric the triangulation runs in, its singular rows, its outward normal.
- **A boundary image carries three facts**: its value at a parameter, a box over a span, a speed bound per chart axis over a span. The chord pass has one rule for every face.
- **The sizing theorem factors the same way**: a cell bound per chart, which exists, and a containment lemma per kind of boundary run. A run on a chart iso line gets a lattice line and sizes at δ; a run that crosses lattice lines sizes at the one named `UNPROVED_HEADROOM` until its lemma is proved.

So it is charts plus images, not charts times images, and a face is never routed by what kind of curve bounds it.

**The units, in order.** Units 1 to 4 turn refusals into meshes and move nothing the mesh emits today.

| | unit | closes |
|---|---|---|
| 1 | `one-chord-rule-over-each-boundary-images-speed-bound` | |
| 2 | `one-chart-table-and-the-lane-takes-every-analytic-chart` | torus × spiric, cone × ellipse |
| 3 | `a-curved-faces-domain-is-read-from-every-loop` | the ring guards; rings on a cylinder, cone, sphere and torus, which it routes to the chart lane |
| 4 | `a-curved-face-is-routed-by-its-domain-not-its-carriers` | the notch; the carrier-kind predicates |
| 5 | `one-containment-lemma-for-an-oblique-boundary` | three missing-theorem rows; independent of 2 to 4 |
| 6 | `rectangle-faces-move-to-the-chart-lane` | the curved lane; Ev's call, with a density measurement |

**What is Ev's.** Unit 6, and with it the re-cut of `crates/mesh/README.md` `[mesh.sizing]`'s list of lanes. Until then the page's rule is unchanged and its lanes still exist; where a unit moves a mechanism the page names (the rim-chord bullet's three doors become one), the page is re-worded with that unit.

**Not this design.** A section spline with a full-multiplicity knot and the turned ball's pole entry are separate defects with their own rows. The interior-pole form Ev tabled is not needed: a loop that does not close refuses in one place, which is where it would be taught if reopened.

## Ratified ground (cited, not re-litigated)

- `docs/DESIGN.md` `[tolerance.chordal]` and `crates/mesh/README.md`
  `[mesh.sizing]`.
- D9's byte-identical replay; D2's typed refusals; structural rungs
  only, never an inferred one.
- Ev's ruling on PR 5913 (DESIGN `[topology]`): every face chart and
  edge carrier is C¹ strictly inside its cell, and no reader carries a
  crease arm. A row whose fix would teach a lane more about creases
  waits on the construction that removes the crease instead.

## Process

Review tiers and the model draws are `memories/orchestration-model.md`'s
and `docs/MODEL-COMPARE-PROTOCOL.md`'s; each unit's tier and reason are
logged at dispatch. A row filed weeks ago is measured before it is
built against (`memories/refusal-text-is-not-cause.md`).

No exit criteria are set: the track is handed back when its P0 and P1
rows are done.
