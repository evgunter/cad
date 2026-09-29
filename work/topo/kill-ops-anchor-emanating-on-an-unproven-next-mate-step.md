---
id: kill-ops-anchor-emanating-on-an-unproven-next-mate-step
kind: issue
title: kef, kemr and kev re-anchor emanating on a next(mate(x)) step whose start vertex no plan proves: a torn next carries an off anchor through Ok
status: closed
opened: 2026-09-29
refs: [mev-fan-plan-trusts-the-orbits-start-vertices, kevs-fan-merge-needs-a-re-describing-kill-door, kill-ops-loop-anchor-on-an-unproven-next-step, revert-anchors-trust-a-torn-next-or-prev, kef-and-kev-take-a-mate-whose-own-edge-is-another]
priority: P2
cost: E
pr: 3483
branch: topo/kill-anchor-proof
closed: 2026-09-29
---

## What

Found by the receipt of `mev-fan-plan-trusts-the-orbits-start-vertices`,
which swept for plans that walk `vertex_orbit` or step
`next(mate(x))` and trust the start vertex they land on.

Three kill operators rewrite a vertex's `emanating` to a half-edge
reached by one `next` step from the killed edge's halves, and take
its start vertex on trust:

- `Body::kef` (`crates/topo/src/euler_kill.rs`, the emanating rule
  after the splice): "`next(m)` starts at `u` and `next(he)` starts
  at `w` by antiparallelism".
- `Body::kemr` (`crates/topo/src/euler_ring.rs`, the re-anchoring
  rules): `u` gets `next(he2)` and `w` gets `next(he1)`, which start
  there on tier-1-valid input.
- `Body::kev` (`euler_kill.rs`, `kev_execute`'s emanating rule):
  where the merged fan is empty, the survivor gets `next(m)`. The
  merged fan itself is proven since PR 3161 (`kev_plan`); this step
  is not.

No plan checks the start, so a torn `next` makes the kill write an
anchor that starts at another vertex (`EmanatingStartMismatch`) and
return `Ok`. With debug assertions off it never panics, because no
`unreachable!` reads the anchor; with them on (the dev profile, and
this repo's default release profile) `kef` on its counterexample below
panics at the tier-1 postcondition `debug_assert` instead. Either way
it carries the corruption past the plan instead of refusing it typed,
which is the gap D1's plan-phase contract leaves no room for.

## Measured (merge base `5ca8d2945e`, release, debug assertions off)

The probe used `review_d18`'s `FIXTURES`: 2,000 seeds of one and of
two random `NextForeign` tears on each fixture. Every kill ran at
every half-edge (`kev` falling back to `kev_describing` with chord
re-descriptions on `MergeRebasesCarriers`, as the torn sweep does).
It then checked that each surviving anchor vertex's `emanating`
starts at that vertex.

| op | calls | `Ok`, anchor off | `Ok`, anchored | `Err` | panics |
| --- | --- | --- | --- | --- | --- |
| `kef` | 400,000 | 15,646 | 274,242 | 110,112 | 0 |
| `kemr` | 400,000 | 82 | 11,534 | 388,384 | 0 |
| `kev` | 400,000 | 205 | 350,015 | 49,780 | 0 |

The first counterexamples, by position in the fixture's half-edge
arena:

- `kef`: `declined_cube`, one tear `halves[9].next = halves[8]`, kill
  at `halves[8]`.
- `kemr`: `ops_ring_bridge`, one tear `halves[48].next = halves[27]`,
  the mate pair from `halves[48]`.
- `kev`: `declined_cube`, tears `halves[19].next = halves[5]` and
  `halves[18].next = halves[19]`, kill at `halves[18]`.

## Measured: the `None` anchors (PR 3483's review, head `d9465eb31e`)

The same three plans write `None` where the `next` step lands on a
killed half, which a torn `next` fakes. The review's search planted
one and two tears of each of six kinds (seeds 1..=300) on
`declined_cube`, `ops_ring_bridge`, `ops_strut_cube`, `ops_genus2` and
`ops_holed_box`, ran each kill at every half-edge (release, debug
assertions off) and counted the `Ok` results that leave a vertex
anchored at `None` while a half-edge still starts there. Of 128,400
calls per operator and tear kind:

| tear | `kef` | `kemr` | `kev` |
| --- | --- | --- | --- |
| `NextForeign` | 0 | 2 | 0 |
| `StartForeign` | 0 | 194 | 0 |
| `EdgeBijection` | 0 | 0 | 4 |
| `PrevForeign`, `ParentLoopForeign`, `LoopAnchorForeign` | 0 | 0 | 0 |

A separate search over `EdgeBijection` tears (seeds 1..=1,000, the
kills whose merged fan is empty exactly where `next(he)` is not the
mate, or the other way round) found 6 `kev` calls that return `Ok`
with a `None` anchor on a survivor that keeps edges, the first on
`ops_strut_cube`, seed 30, one tear, kill at `halves[16]`. Four
deterministic constructions reach the arm with no search: a strut
with `next(m) = he` (`kev`); `declined_cube` with `halves[9]` and
`halves[8]` torn onto each other (`kef`); and `ops_ring_bridge` with
`next(he1) = he2`, or `next(he2) = he1` (`kemr`). In the dev profile
each panics at its operator's tier-1 postcondition. At the fix pass's
head every count above is 0.

## The shape to give

Each plan proves that the half-edge it will anchor on starts at the
vertex it anchors, and refuses typed otherwise (`OrbitBroken` naming
the step's origin, as `kev_plan` and `mev_fan_plan` do). Pin each
counterexample above as a deterministic row. If
`vertex-orbit-reads-no-start-vertex` puts the check into the walk
itself, these single steps still need their own proof, since they do
not go through the walk.

## Closed (2026-09-29, PR 3483)

`kef`, `kemr` and `kev` (both doors, and `kev_merged_members`) prove
every `emanating` write in their plans through
`Body::require_kill_anchors`:
- a `Some` anchor starts at its vertex, proved by
  `require_orbit_starts_at`;
- a `None` is written only where no half-edge outside the killed ones
  starts at the vertex.

A torn step refuses `OrbitBroken`, typed and before mutating.

The single full review found that the first head still trusted the
step in the `None` arms, and that its `kev` arm reorder rested on an
unproven `mate(m) == he`. The fix pass closed both. On the widened
probe (five fixtures, `NextForeign` and `EdgeBijection`), both vertex
columns are 0 for all three operators, and there is no over-refusal on
24k valid bodies.

What remains is filed:
- `kill-ops-loop-anchor-on-an-unproven-next-step` (the loop `first`
  and `Empty` writes);
- `revert-anchors-trust-a-torn-next-or-prev` (measured);
- `kef-and-kev-take-a-mate-whose-own-edge-is-another`.
