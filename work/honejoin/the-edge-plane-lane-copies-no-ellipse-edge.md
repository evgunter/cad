---
id: the-edge-plane-lane-copies-no-ellipse-edge
kind: issue
title: The edge-plane lane copies no ellipse edge: a germ along an ellipse rim inside a curved face refuses as a section invariant
status: open
opened: 2026-10-12
priority: P1
cost: M
---


Found by `cleave/crossing-cylinders-join` once the join's kind arm
stopped reading every cylinder pair as parallel.

## What

The fixture is `crates/sweep/tests/crossing_equal_rods.rs`: two
radius-1 rods on crossing axes, `A` mitred by their bisector plane
`z = x` so that its ellipse rim lies on `B`'s wall. The germ runs along
`A`'s ellipse edge inside `B`'s wall face. `join::kind_arm` now reads
the axes apart (`face_pair_axes_parallel`), so the pair has no kind arm
and the edge-plane lane takes it (`GermLane::EdgePlane`, its plane the
ellipse's own). On the edge's side that lane's chord copies the edge
(`RingClosure::AlongEdge`, `JoinLane::AlongEdge`), and
`chord_join::along_edge_spec` copies only a line or a circle. Every op
in both orders, at spins `(0.4, 0.3)`, `(1.0, −0.7)` and `(0, 0)` and
with `B` from `x = 0.2` and `0.5`, at ε = 1e-9, 1e-6 and 1e-12, refuses

`Join(SectionInvariant { what: "a section segment along an edge whose
carrier has no chord lane (only lines and circles are copied)" })`.

That is a kernel-invariant type on valid input for a lane that is not
built, so the refusal does not tell its cause. `join`'s edge-plane
lane admits a circle or an ellipse edge (`edge_lane` in
`bool_connect`), but only the circle has a copy. The adjacency skip
can leave the edge's side with no chord to mint, and then the copy is
never asked, so refusing every ellipse at admission could refuse poses
that build today. Where the copy is asked is `chord_join`'s plan
(`JoinPlan::first` and `second`, private to that module).

## Measured past it

On the branch, uncommitted: `along_edge_spec` given an ellipse arm (the
circle arm's end-matching, the carrier reversed where the chord runs
against it, described as an image in the divided face's chart
`EdgeDescriptionSpec::chart`) moves all 24 runs to
`CurvedPairUnsupported { site: InteriorLoopGuard, operand: A, kind:
Cylinder }`. So the copy alone does not build the scene. The interior
loop guard is the next door.

## Direction

- Type the frontier where the copy is refused, so the refusal names an
  unbuilt lane and not a broken invariant. Its home is
  `chord_join::along_edge_spec`, a file other lanes are working in.
- Give an ellipse edge a copy. `EdgeCurveSpec::arc_of_circle`'s
  scaffold pushforward has no ellipse counterpart. The question is
  which description an ellipse copy carries: the chart of the face it
  divides, or the source edge's own.
- Then take the interior-loop guard measured above. The fixture flips
  to building at its closed forms (`vol A = 3π`, `vol B = (3 − x0)π`,
  and the shared volume in `shared`). Its `NO_ELLIPSE_COPY` acceptance
  comes out at the same time.


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Not duplication.** "Copies" here means copying an edge's geometry: `chord_join::along_edge_spec` is missing its Ellipse lane. DESIGN home: `[edges]`. This row belongs off the duplication track.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P1: the edge-plane lane copies a circle and not an ellipse, so crossing equal rods refuse: verb breadth beyond the everyday shapes (a split or boolean that refuses, or ships a body its own gate fails, on a legal input), P1 by `work/README.md` Priority.
- Moved to HONEJOIN (`work/honejoin/`) by `git mv`, id and body unchanged: whose charter is the curved and chord join's residue. The join's edge-plane lane admits an ellipse edge its copy then refuses as a broken invariant: an arm its gate admits and a refusal it misfiles. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## The copy, landed elsewhere (SPLITDOOR, 2026-10-28)

`splitdoor/tilted-plug-rosters`
(`a-tilted-plug-union-refuses-at-the-germ-segment-and-section-copy-rosters`)
gave `chord_join::along_edge_spec` a copy for every carrier past a
line and a circle: the edge's own spec (`EdgeCurve::restated_spec`,
its own description), run back where the chord runs against it
(`chord_join::run_back`, the reversal `SegmentCurve::running_from`
already used). That answers the description question above with the
source edge's own. Measured on that branch at ε = 1e-9, 1e-6 and
1e-12, the fixture now ends past the copy:

- spins `(0, 0)`, `B` from `x = 0.2`: every op in both orders builds,
  passing tiers 2 and 3′, the at-rest certificate and the
  legal-operand check, the closed form inside the certified enclosure,
  whose pad is over `differential`'s `1e-7` budget above ε = 1e-12
  (to `1.1e-5` m³ at ε = 1e-9, `5.7e-5` at 1e-6), so the pins judge it
  against a stated ceiling (`PAD_CEILING`, `1e-3`);
- spins `(0.4, 0.3)` and `(1.0, −0.7)` from `x = 0.2`, and `(0.4, 0.3)`
  from `x = 0.5`: every op refuses `Join(UnpairedLooseEnds { count: 4 })`.

The pins (`crossing_rods_end_past_the_ellipse_edge_copy*`) name those
endings, `NO_ELLIPSE_COPY` gone. What is left of this row is the
loose ends at the turned poses, and the enclosure's width at `(0, 0)`.
Nobody has root-caused the `UnpairedLooseEnds { count: 4 }`: it is
the same count at every turned pose, ε and op, and the poses whose seam
rulings sit at `±x` (spins `(0, 0)`) do not reach it, so the seam
rulings' placement against the copied ellipse is the first thing to
read.

## 2026-11-03: the loose ends traced and gone (sectarms 3b-ii, PR 6466)

**The cause, traced on main `27205d771f`.** At spins `(0.4, 0.3)`, B from
`x = 0.2`, `A ∪ B`, a probe dumped the join's open records:
- A's seam ruling meets A's mitre ellipse at `(0.921, 0.389, 0.921)`, a
  vertex of A on B's wall.
- The sweep's factored crossing found that pierce on the `z = x`
  bisector plane and recorded its `Positive` branch. The pierce pass then
  gave that branch to both of the vertex's germs, including the one along
  A's ellipse edge (`OnEdge` × `InFace`).
- That edge's other end, where it meets B's cap at `(0.2, 0.98, 0.2)`,
  rides `Only`, as every germ along an edge does.
- Facing pairs only germs whose branches agree, so the two ends of one
  edge never paired: `UnpairedLooseEnds { count: 4 }`.

At spins `(0, 0)` the seam sits at `±x`, off B's wall, so no pierce
records a branch.

**The fix (sectarms 3b-ii).** The recorded plane is now only the pierce's
datum. The germ's branch is read on the section value
(`join::germ_branch`), which gives an edge germ `Only`. Every op of
`crates/sweep/tests/crossing_equal_rods.rs` builds SOUND at its closed
form, at every pinned spin, both orders, at ε 1e-9. The pins are renamed
`crossing_rods_build_past_the_ellipse_edge_copy*`.

**What is left of this row** is the enclosure's width at `(0, 0)`, judged
against `PAD_CEILING`; the loose ends are gone. The ellipse copy itself
landed in SPLITDOOR's change above, and this fixture no longer witnesses
anything else of the title.
