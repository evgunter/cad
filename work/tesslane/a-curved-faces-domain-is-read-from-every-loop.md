---
id: a-curved-faces-domain-is-read-from-every-loop
kind: unit
title: a curved face's domain is its outer loop's lift less its rings', and the lane triangulates that
status: dispatched
opened: 2026-11-14
priority: P0
cost: H
branch: tesslane/domain-loops
---


Unit 3 of the chart lane (`work/tesslane/plan.md`, The chart lane). Follows unit 2.

## What it is

A curved face's domain is one value: its outer loop's lift (the stored chart images placed end to end, `topo::Body::loop_lift`) less its rings' lifts. The lane triangulates that region: boundary points are the shared chord points, interior points are the lattice, and inside or outside is the planar lane's even-odd count over all loops. Both curved lanes' ring guards go, and `TessellateError::RingOnCurvedFace` with them (its `Display` says no construction makes a ring on a curved face). A loop that does not close in its chart refuses typed, as a capability, in one place.

- `trim_polygon` reads `face.outer` only today.
- `memo::FaceInputs::gather` folds the rings' rows into its key.

## The open part (`design: true`)

Nothing in `topo` places one loop's lift relative to another's: `pcurves::Lift`'s doc says a chain's start is gauge. A ring's lift may sit whole periods from the outer loop's window (and on a sphere, on the other side of a pole row). Placing it is a decision about the lift, on `crates/topo/src/pcurves.rs`, which is not this program's ground. Whether it is needed at all, and how often, is being measured (branch `tesslane/measure-lift`); what the placement is, and where it lives, is weighed with that file's owner once the measurement is in.

## Measured (2026-11-14; `crates/mesh/tests/lift_probes.rs`) — the open part is settled

- **A ring's lift is in its outer loop's polygon as stored on 23 of 24 rings, and one whole `u` period off on the other.** The rings of the donut, the pipe walls, the cone and ball ∪ tube sit inside. Ball ∖ tube's ring, a whole circle carried as one `Projected` row, is stored at `u ∈ (−2.44, −0.70)` against an outer loop at `u ∈ [π, 2π]`; the same circle cut into two arcs (ball ∪ tube) is stored inside. `v` is never displaced, and no row of a ball ring or its outer loop carries the pole involution.
- **Nothing in `topo` places one loop against another on an analytic chart, but a placer exists for spline charts**: `Trim::with_rings` in `topo::boolean::spline_region` moves a ring by the one whole number of periods per closed channel that puts its first point in the outer loop's cover, or answers `None`. `cover::TorusChartWindow::open_loop` abandons its window on any ring, `chart_region::extract_face_uv` keeps rings only as deductions, and `props`' quadrature folds each loop on its own.
- Every ring is clockwise where `sense` is true, the reverse of its outer loop, so the even-odd count has what it needs.
- **A loop that does not close exists at rest, through the Euler doors only.** No door-built pipe, cone, ball, washer or donut has one: a full rim always sits in a loop that closes, its wall either two half faces or seamed with the seam walked twice a period apart. A seamless cylinder pipe and a seamless cone frustum built through the Euler doors finish, pass every validator, and refuse `RingOnCurvedFace` in the mesh. That is the capability refusal this unit states in one place; `work/pcert/a-seamless-band-mints-and-passes-tier-3-though-loopwraps-says-no-constructor-makes-one` is the row about whether such a body should be at rest at all.

So the placement is one integer per closed channel, and it is cut out as its own unit, `a-rings-lift-is-placed-against-its-outer-loop-in-one-place`, which this unit now also waits on. Both designers put it in `topo`, beside the lift; with a placer already there for spline charts the question left is only that the two become one, which is that unit's.

## Unit (2026-11-14, TESSLANE)

**Letter H. Tier: dual review.** It is the unit that makes the lane read a face's whole boundary. A ring placed a period away, counted on the wrong side, or dropped gives a mesh with a hole filled or a wall missing, and where its neighbour's chord points still pair up `check_mesh` can pass it; believing it takes claims to falsify and a second reading.

Branch `tesslane/domain-loops`, cut from `tesslane/chart-table` (unit 2, in review): merge that branch as its fix passes land, and main after it merges.

**What changed since the sections above were written.** Both ring units are on main: `topo::Body::face_lift` answers a face's outer lift and each ring's lift moved into the outer loop's sheet, on analytic and spline charts, by one rule (`pcurves::place_ring`), with every miss typed (`pcurves::RingPlacementError`). So nothing about placement is this unit's: the lane reads the door.

**Owed by the unit, beyond the sections above:**

- **Measure before building.** For every body in `reach_probes` and `lift_probes` that holds a ring on a curved face, say in the PR body which lane takes the face today, where it refuses, and what `face_lift` answers for it. Where a section above is wrong, the measurement wins and the PR says so.
- **The domain is one value, read once.** The lane takes the outer lift and the placed rings from `Body::face_lift` and builds its polygon from them. It does not place a ring, shift a lift or re-read a loop on its own. A miss of the door (`OuterOpen`, `Undecided`, `SplineTrimUnread` and the rest) surfaces as one typed refusal of the mesh that carries the door's cause; it is the capability refusal for a loop that does not close, stated in that one place.
- **Inside or outside is one rule with one home.** The planar lane's even-odd reading over all loops is what the lane uses; it is read, not copied. Each ring's polygon is oriented against its outer loop (`geom_brep::PolygonOrientation`), and a ring that is not is refused typed, not flood-filled on the wrong side.
- **A ring's edges are boundary like any other.** Their chord points are the shared ones, and the chord pass sizes a ring's edges against this face's lattice as it sizes the outer loop's (`chords::FaceLattice`, the row's speed bounds). A ring vertex on a singular point is decided where an outer vertex is (`AnalyticRow::is_singular`). A singular point strictly inside the face, on no loop, is the interior-pole form that was tabled: it refuses typed, in the same one place, and the PR says which bodies reach it.
- **On a cone, rings ride the development** with the outer loop. Say what a ring round the axis is (a loop that winds is not a ring of a disc: the door answers `OuterOpen` first), and pin a ring that holds the apex in its hole.
- **Routing: a face with a ring goes to the chart lane.** `tessellate::lane_of` sends a curved face that holds a ring to the trimmed lane, and the rectangle lane's ring guard and `TessellateError::RingOnCurvedFace` go. This is the ring half of unit 4, taken here so that no face is left with a guard whose text has to be invented; unit 4 keeps the notch and the retirement of the carrier-kind predicates. No face that meshes on main is re-routed: every ringed curved face refuses today.
- **The memo key.** `memo::FaceInputs::gather` folds the rings' rows into its key. Say whether a ring's placement is a function of what the key already holds, and add to the key only if it is not.
- **Done when** rings on a cylinder, a cone, a sphere and a torus mesh at every δ the probes use and pass `reach_probes::geometry` (the mesh check, the volume bracket, winding, sampled deviation, no triangle of no area); the ball less the tube, whose ring is stored a period away, is among them; a filled hole and a ring meshed where it is stored each go red (mutate the lane to produce both and say what catches each); the ring rows of `reach_probes` are re-pinned; the seamless pipe and frustum built through the Euler doors refuse typed with the door's `OuterOpen`; and `lift_probes` still holds.
- **Nothing that meshes on main moves.** D9 goldens, `docs/tess-budget-data` and the tour pins pass unedited. A byte that moves there is a stop: report it, do not re-baseline it.
- **Closes** `valid-ringed-torus-faces-build-but-cannot-be-meshed` and the ring half of `a-notched-or-ringed-cylinder-wall-does-not-tessellate` (the notch stays unit 4's; say so on that row).
- **Run what CI runs before every push:** `bash scripts/run-gates.sh`; `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`; fmt; clippy for mesh, geom-brep, topo and `demos/tour`; `cargo nextest run -p mesh` whole; the rows you add or change at `CAD_TOLERANCE_EPS` 1e-6 and 1e-12 as well (a row that builds bodies across scales or probes near a boundary states what it expects at each); sweep's rows over the bodies you touch. A test costing 0.5 s or more goes in the slow set with its measured time.
