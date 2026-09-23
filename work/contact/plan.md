# CONTACT — the plan

Touches, overlaps and declared contacts: the ordinary solids the boolean lane will not combine.

Opened 2026-09-20 by REACH's priority-seam cut (`work/README.md`,
Track size). Nothing dispatched yet.

## The slate

**23.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `area-overlap-contact-admitted-but-unmerged-refuses-at-the-next-step` | D | A declared area-overlap cap contact is admitted without a merge, and the F7 gate refuses the two coplanar rows at the next boolean |
| P0 | `axis-coincident-lap-trips-the-planar-join-invariant` | H | A box lap whose plane CONTAINS the cylinder axis reaches the all-planar join lane's conic guard through the public subtract door |
| P0 | `overlap-lane-boundary-crossing-cuts` | H | The overlap lane cuts only at coincident boundary vertices: boundary-crossing cuts (the D3 reach gap blocking the ef_bound_backed migration) |
| P0 | `partial-overlap-with-touch-only-boundaries-clears-at-the-census-gate` | H | Two half-overlapping cubes whose boundaries meet only in touches (coplanar faces, edges in faces) clear the census's instance arm at the box gate — the census has no arm for a partial overlap that produces no pierce |
| P0 | `touch-kinds-without-a-local-side-analysis-block-the-material-test` | H | VertexVertex, VertexOnEdge, EdgeEdgeOverlap and ConformalPatch touches between two solids block the census's material test because only the vertex-on-face and edge-in-face kinds have a local side analysis |
| P3 | `declared-faces-has-no-cross-solid-check` | E | Declared.faces has no cross-solid check, so a contact record naming two faces of the SAME solid would back events within it |

## Order

`touch-kinds-without-a-local-side-analysis-block-the-material-test`
first. Four touch kinds have no local side analysis and that one
absence blocks the material test for all of them, so it is the row the
other refusals in this track sit behind.

Then `partial-overlap-with-touch-only-boundaries-clears-at-the-census-gate`,
which is the same absence seen from the census gate, and
`axis-coincident-lap-trips-the-planar-join-invariant`, which is
independent and can run in parallel.
`declared-faces-has-no-cross-solid-check` is class `E` and a drive-by
for whoever opens `census.rs` first.

## Review posture

OPEN, for this program's first dispatch. REACH inherited protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. This program's units are candidates where
the failure mode is a confident WRONG answer rather than a refusal —
the first orchestrator names which of its rows those are.
