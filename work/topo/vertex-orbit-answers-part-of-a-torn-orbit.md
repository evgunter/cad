---
id: vertex-orbit-answers-part-of-a-torn-orbit
kind: issue
title: Body::vertex_orbit, the half-edge-keyed door, answers part of an orbit a torn next split or closed past some of its members
status: open
opened: 2026-10-03
priority: P4
cost: E
refs: [a-fan-split-at-a-vertex-another-vertexs-torn-walk-merges-into]
---


## What

`Body::orbit_inverts` (`crates/topo/src/body.rs`) proves a closed vertex
walk is the vertex's whole orbit on a body whose `prev` links are
untorn: each member's inverse step `mate(prev(·))` is the member before
it. The fan
split's plan (`Body::mev_fan_plan`, `euler.rs`) and every vertex-keyed
read (`Body::vertex_orbit_of`, so `edges_of_vertex`, `faces_of_vertex`
and their callers in `boolean`, `splitting`, `offset_axial` and
`shell`) refuse a walk that fails it.

`Body::vertex_orbit`, the public door keyed by a half-edge, does not.
Its docs say so: on a torn body its `Some` lists half-edges that all
start at the vertex, but not necessarily all of them. Its readers today:

- `kev_plan` (`euler_kill.rs`) reads the dying vertex's walk as a set
  and proves the set whole with `require_vertex_unnamed`, a whole-arena
  pass whose panic names the first stranded half-edge. Putting the
  inversion proof inside `vertex_orbit` makes `kev_plan` panic earlier,
  at the walk, and lose that name (a refusal, `OrbitBroken { he: m }`,
  when this was filed): tried in the PR that
  filed this row, six kev rows in `euler_kill` and `review_d18` red on
  the refusal they pin.
- `boolean::sectors` (`along`, the sector edge reader) reads one step
  `orbit[1]` of an operand at rest, not the orbit as a set.
- `MergeFaces::strut_tip` (`merge_faces.rs`) reads
  `vertex_orbit(toward).len() == 1` as "valence one", the walk taken as
  the whole orbit. On a cube corner (valence 3), the single tear
  `next(mate(he)) = he` gives the walk `[he]` and `strut_tip(he) ==
  Ok(true)`, while `vertex_orbit_of` refuses on the same body (executed
  in review of the PR that filed this row, uncommitted).

## A paired `next` + `prev` tear passes the inversion proof

The proof reads `prev`, so a `prev` tear matching the `next` tear
defeats it. On the declined cube, with seed corner `v` and orbit
`[a, b, c]`: `next(mate(a)) = c` together with `prev(c) = mate(a)`
makes the walk `[a, c]` invert, and `b` is stranded.
`vertex_orbit_of(v)` and `edges_of_vertex(v)` answer the two-member
part of a valence-3 orbit, and every fan site at `v` splits `Ok`. The
validator reports `SplitVertexOrbit { v, orbit: 2, incident: 3 }`, and
the minted halves in `UnreachableHalfEdge`; no minted key lands in an
orbit error.
`euler::tests::a_fan_split_past_a_paired_tear_leaves_no_minted_key_in_an_orbit_error`
pins that limit. Closing it needs a proof that reads neither link
alone, e.g. a count of the half-edges starting at `v` (O(arena), or a
per-vertex incidence count the body does not keep).

## `chord_join::null_site` reads a refusal as "no null edges"

`null_site` (`chord_join.rs`) spells
`body.edges_of_vertex(v).unwrap_or_default()`, so a vertex whose walk
`edges_of_vertex` refuses drops out of the site with no error. Since
`vertex_orbit_of` refuses a walk that does not invert, a torn walk at a
site vertex gives no ties where it used to give part of them.

Not fixed in the PR that filed this: `null_site` returns `Vec`, so
mapping `None` to a refusal changes its signature and its eight
call sites (`boolean::join`, `boolean::sectors` ×6, `chord_join`'s own
circle chord). The probe: `null_site` was instrumented to panic on
`None`, and the whole `cargo nextest run -p topo` suite (2232 rows,
the torn-body and corrupt-input rows among them) never reached it.
That is a row census, not a door proof. The argument that no door
reaches it from at-rest input: null edges exist only mid-operation (the
validator refuses one at rest, `ScaffoldAtRest`), so on an at-rest
operand `None` and `Some` give the same site `[v]`; a null edge is
minted by a fan split, whose plan now refuses a walk that does not
invert. A paired tear (above) still passes that plan.

## The shape to give

Either put the proof inside `vertex_orbit` and move `kev_plan` onto a
walk whose panic keeps naming the stranded half-edge (D2 row 4: a
torn orbit panics naming the record), or keep the door as
documented and say why a half-edge-keyed read may answer part of an
orbit. No measured fault rides this today, so it is P4.
