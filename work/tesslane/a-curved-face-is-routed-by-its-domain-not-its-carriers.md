---
id: a-curved-face-is-routed-by-its-domain-not-its-carriers
kind: unit
title: lane_of routes a curved face on its domain's shape, so a notch or a ring reaches the lane that meshes it
status: closed
opened: 2026-11-14
priority: P0
cost: M
branch: tesslane/route-by-domain
closed: 2026-11-17
pr: 7041
---


Unit 4 of the chart lane (`work/tesslane/plan.md`, The chart lane). Follows unit 3.

## What it is

`tessellate::lane_of` routes a curved face on its domain: a face with no ring whose outer loop is an iso-rectangle of its chart goes to the rectangle lane (`curved.rs`); every other curved face goes to the chart lane. The carrier-kind predicates (`has_trim_carrier`, `has_tilted_circle`, `takes_sphere_arm`, `face_target`) go, and `props`' rectangle predicate is read as a route where `curved::require_iso_rectangle_face` reads it as a refusal. The chord pass reads the same route, from the same door, so the two cannot disagree.

A boundary run on a chart iso line gets a lattice line through it, so a notched wall has every boundary point on the lattice and sizes at δ with no headroom, as a rectangle does.

## Closes

`a-notched-or-ringed-cylinder-wall-does-not-tessellate`: the notch meshes and passes `check_mesh`, and its `reach_probes` row is re-pinned.

## The ring half moved to unit 3 (2026-11-14)

A face that holds a ring is routed to the chart lane by unit 3, with the lane that reads it: a guard left standing between the two units would have needed a refusal text nobody could state truthfully. So unit 3 closes `valid-ringed-torus-faces-build-but-cannot-be-meshed` and the rings on a cylinder, cone, sphere and torus, and removes `TessellateError::RingOnCurvedFace`. This unit keeps the notch, the iso-line lattice rule above, and the retirement of the carrier-kind predicates.

## Unit (2026-11-16, TESSLANE)

**Letter M. Tier: dual review.** Every curved face's route moves from three kind tests to one door, so the impact is as broad as the lane; the logic is a read of one predicate and a table of its causes, which is measurement more than invention. A face that meshes on main and is sent to the other lane moves its bytes or refuses, and no row holds a face the fixtures do not build.

Branch `tesslane/route-by-domain`, cut from `tesslane/domain-loops` (unit 3, reviewed, closing): merge that branch if it moves, and main after it merges.

**The unit is cut in two, and this is the routing half.** The lattice line through an iso run, and the δ target it earns, is `a-boundary-run-on-a-chart-iso-line-gets-a-lattice-line`, parked behind the question before Ev on PR 6966 (where a lattice point may sit beside the boundary), which that rule has to be designed with. Here a notched face meshes as every other face of the chart lane does today: the uniform lattice at the lane's present target (`trimmed::face_target`, the headroom's share of δ), each triangle held by its certificate. So the sections above are right about the route and the paragraph on the lattice line is the other row's.

**Owed by the unit:**

- **Measure before building.** The shape door (`geom_brep::props::require_iso_rectangle`) and the branch door (`require_one_chart_branch`) answer `Result<(), PropsError>`, and `PropsError::NotIsoRectangle { what }` is documented as a payload that mixes classes: a face that is decided not to be a rectangle, a stored-data residual, a span outside certification. List every site in the two doors that can refuse, with the body in the crate's rows that reaches it, in the PR body, in three classes: **the face is no rectangle** (it is the chart lane's), **the door cannot say at this band**, **the data contradicts itself**. Say for each curved-lane refusal on main (`UnsupportedCurvedShape`, `MeridianFreeCurvedFace`, `SingleColumnCurvedFace`, the walk-consistency refusal) which bodies reach it. Where a section above is wrong, the measurement wins and the PR says so.
- **The route is one value from one door, read once per face per run.** `lane_of` answers the rectangle lane for a ringless face both doors pass, the chart lane for a face of the first class, and a typed refusal carrying the door's cause for the other two: a route is never a default taken because a predicate could not answer (`memories/refusal-text-is-not-cause.md`). If telling the classes apart needs the door to answer more than `()`, give it that answer where it is raised, as `OffSurface` was split, and do not census `what` strings in `mesh`. The chord pass (`chords::FaceLattice::of`, the non-circle skip beside it), `trimmed::face_target` and `tessellate_impl` take that one value; none of them asks again, and the two lanes' guards against a mis-routed face stay as router-defect refusals.
- **What goes.** `trimmed::has_trim_carrier`, `trimmed::has_tilted_circle`, `tessellate::has_ring` as a separate question if the door answers it, and every doc sentence that says a face is routed by the kind of its carriers (`lane_of`'s, `curved.rs`'s header, `walk.rs`, `types.rs`). `curved::require_iso_rectangle_face` stops being a refusal a valid face can reach: say what it still is.
- **No face that meshes on main changes lane.** Argue it by reading, per chart: a face `has_trim_carrier` or `has_tilted_circle` sends to the chart lane today must be of the first class at the door on every valid body, never of the second, and a face the rectangle lane meshes today must pass both doors, which it does by construction since the lane asks them. Then run the crate suites, sweep's and the tour with a panic armed where the old route and the new differ on a face whose body meshes on main, and say what it found. An unmoved pin is consistency, not the argument (`memories/output-stability-as-justification.md`).
- **A rectangle the walk cannot take is not this unit's.** A face both doors pass that then refuses at the walk's own questions (no meridian, one column, walk consistency) keeps its refusal and its row. Say in the PR body whether the chart lane would mesh it, measured, and file what you find; do not route it.
- **What the chart lane newly meets.** Faces whose every boundary run is an iso line and whose domain is no rectangle: the notched pipe walls of `reach_probes`, and on each other chart whatever the doors build (a cone wall cut across its seam, a sphere face of parallels and meridians with a re-entrant corner, a torus face cut across either seam). Build one per chart through the public doors where a door builds it, and for each say whether it meshes and passes `reach_probes::geometry`, or which typed refusal it reaches and whether that refusal's text is true of the body. A lattice line that lands within rounding of an iso run is the class of `a-lattice-point-within-rounding-of-a-trim-chord-point-cuts-a-triangle-of-no-area`: count the triangles of no area over these bodies at more δ than the rows use, pin what you find, and do not cure it here.
- **Cost.** The route is read once per face where three kind tests ran several times each. Give the wall time of `mesh`'s suite and of the tess meter before and after; the meter's counts do not move.
- **Done when** the three notched walls mesh at every δ the probe uses and pass `reach_probes::geometry`, their row re-pinned; a mutant that sends a notched face to the rectangle lane, and one that sends a rectangle to the chart lane, each go red, and the PR says what catches each; a door that cannot say refuses typed with its cause, pinned on a body that reaches it if one can be built and argued unreachable if not; the kind tests are gone.
- **Nothing that meshes on main moves.** D9 goldens, `docs/tess-budget-data` and the tour pins pass unedited. A byte that moves there is a stop: report it, do not re-baseline it.
- **Closes** `a-notched-or-ringed-cylinder-wall-does-not-tessellate`.
- **Run what CI runs before every push:** `bash scripts/run-gates.sh`; `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`; fmt; clippy for mesh, geom-brep, topo, `demos/tour` and the viewer job (`cargo clippy -p viewer --features app --all-targets -- -D warnings`); `cargo nextest run -p mesh` and `-p geom-brep` whole; the tests of the excluded roots that mesh (`tools/tess-meter`, `tools/tess-lint`, `tools/k-lint`: read the `tools` job in `.github/workflows/ci.yml` and run its steps); the rows you add or change at `CAD_TOLERANCE_EPS` 1e-6 and 1e-12 as well, each stating what it expects at each; sweep's rows over the bodies you touch. A test costing 0.5 s or more goes in the slow set with its measured time.

## Closed (2026-11-17, PR 7041)

Built: `tessellate::route_of` makes one route value per curved face, once per run (`Routes::of`), from the first of four facts: a ring; an outer loop that stores an image of a form no iso line takes; the two doors' answer; and, of a rectangle the doors certify on a ruled chart, a straight side that holds a point (PR 7049's interim route, merged into main while this unit was in review and folded in here as the fourth fact). The rectangle lane takes a `curved::Rectangle` that only the doors' answer mints. The carrier-kind predicates are gone. The notched pipe walls and the notched sphere faces mesh at every δ the probes use; no door builds a notch on a cone or a torus.

What the sections above had wrong, each measured by the unit or its reviews:

- **"One value from one door" is not enough.** A sphere face bounded by a section tilted within the band meshes on main through its stored projected image, and a route that asks the door first refuses it `Escalated` (the first review's MAJOR, eight cells). The route reads a stored image's form, a decided fact, ahead of any banded question.
- **`NotIsoRectangle` mixed three classes at more sites than the spec listed.** It is now only the decided verdict; data that lies on no such surface answers `OffSurface`, a kernel state `Defect`, a span outside certification `SpanUncertified`, and an incidence the band cannot decide escalates. The branch door's `Ok` now means decided too.
- **"D9 passes unedited" did not hold, and the unit did not stop.** Four hand-built witnesses (`keyway`, `oblique_lens`, `pole_crossing_half_cap`, `apex_crossing_bowtie`) store no chart image, so the chart lane's domain read refuses them `DomainUnread` where main gave the doors' verdict. The unit re-baselined them and reported it; its first fix pass then swapped them for minted bodies in place, which hid the move, and the second review found that. Ruled by the orchestrator: the unminted witnesses stay in D9 with their constants moved and the move stated in the PR body, and their minted forms are rows of their own. Two of the moved rows (the lens and the half cap) now digest alike.
- `takes_sphere_arm` does not exist; `face_target` was no kind test; the doors read no ring.

**The K stream** (the doors' recorded decisions): on a ruled body with no split side it is main's before PR 7049 plus one split read per face; on a body with a split side it is main's less main's repeated reads, reordered; a memo hit records what it recorded before. A memo hit is 1.0 to 1.6 times slower than main's, since the route now asks the doors where the lane used to skip them: `a-memo-hit-pays-for-the-route-the-lane-used-to-skip`, P3.

**What the lattice-line paragraph above asked for is not built** and is unit 4b's (`a-boundary-run-on-a-chart-iso-line-gets-a-lattice-line`): a notched face meshes on the chart lane's uniform lattice at the lane's target, each triangle held by its certificate.

Filed by the unit and its reviews: `a-seam-cornered-cylinder-notchs-rectangle-neighbour-exceeds-its-certificate`; `a-rectangle-the-walk-refuses-has-no-witness-the-chart-lane-can-read`; `a-memo-hit-pays-for-the-route-the-lane-used-to-skip`; `three-readings-of-a-non-iso-boundary-disagree-on-a-line-on-a-sphere`; `work/ciw/a-chart-branch-guard-row-is-red-locally-and-green-on-the-hosted-kernel-job`; a note on `work/tesslie/apex-crossing-branch-premise-has-no-body-level-witness`. `work/conearm/the-branch-door-reads-an-escalated-apex-or-pole-decision-as-one-branch` was filed by the first fix pass and closed by the second.
