---
id: a-generic-mirror-refuses-at-interval-a-body-that-holds-at-f64
kind: issue
title: Mirrored across a generic plane, a half rod on a block that is embedded at f64 refuses at Interval, and a near-full ring refuses at Interval on a pair its f64 refusal does not name (cause not isolated)
status: open
opened: 2026-11-23
priority: P1
cost: M
---

## Finding

Found by EMBED's lane `embed/clip-vertex` posing its body rows, traced
by PR 7464's delta review on the head `388ee3ef82`, and re-traced by the
lane on `embed/clip-vertex-main2` `de9b3eb2a2` (the landing branch
`7f85375799` with the unit), `Interval`, with the review's temporary
prints (`origin/embed/clip-vertex-review` `0e3bc380b`,
`review-probes/clip-vertex/cv-trace-instrumentation-delta.diff`). The
counts below are the lane's re-trace; where the review's differ it is
said.

`topo::mirror_body` across the plane through `(0.1, -0.2, 0.4)` with
normal `(1, 2, 3)`, then `clearance::injective` at the strict bound.
Fixtures in `crates/sweep/tests/self_intersection_proves_injectivity.rs`.

**The ring** (`near_full_ring`, a ring revolved `τ − 10⁻³`):

- whole body, `Interval`:
  `Uncertified { 3v1, 4v1, Edge(6v1), Departure }`. At `f64` the same
  mirrored body refuses on another pair
  (`Uncertified { 1v1, 3v1, Edge(9v1), Departure }`).
- that pair alone: 164 cells reached the presentations. 143 exits are
  `sheets(..)` answering `Local` in `present_chart`; 132 cells have
  both faces `Local`, 18 present one face's side with the other
  `Local`, 11 present one face's curve with the other `Local`, 2 are
  vacuous on one face, and 1 presents both (a side against a curve)
  and is not certified. No exit is at `curved_walls`.

**The block** (`half_rod_on_a_block`, a half rod united with a block
under its flat face): embedded at `f64`, on the dyadic grid and off it;
at `Interval` the whole body is `Budget { Depth 40, 3v3, 4v1 }`. The
cylinder wall `4v1` against the two block faces that share one vertex
with it and no edge:

| pair | verdict | cells | wall `Through`, face `Point` | wall `Through`, face `Local` | both `Local` | other |
|---|---|---|---|---|---|---|
| `4v1 × 3v3` | `Budget { Depth 40 }` | 100 | 67 | 27 | 5 | 1 |
| `4v1 × 7v3` | `Uncertified { Vertex(11v1), Departure }` | 99 | 67 | 25 | 5 | 2 |

So in two thirds of the cells both faces ARE presented (the wall's
side through the vertex, the face's corner point) and the pair is still
not certified: the refusal is after the presentations, in what the
plane through the ruling signs at `Interval`. Tagged exits in the
wall's presentation: 4 `sheets → Local` and 1 at
`sides_are_the_faces`, per pair. The plane face's `Local` (25 to 27
cells) is in `present_plane`, at sites the prints do not tag.

**The review's trace of the block differs** (head `388ee3ef82`, before
strip's `present_plane`): 109 of 178 traced cells of the wall against
the block face exit at `zero_of`'s second rule (`partial_zero` → `None`
→ `Local`), the rest untagged; removing that rule does not make it
hold. The lane did not reproduce that tally on that head; on the tree
above the rule's exit does not fire in either pair.

## What it is not

- Not the cause of the `f64` half
  (`work/pcert/a-cylinder-rim-on-a-turned-frame-stores-a-rounding-in-its-axial-channel-at-f64.md`:
  a rim row re-minted with a rounding in its axial channel, exiting at
  `curved_walls`). No exit here is at `curved_walls`, and the ring's
  pair is another pair.
- Not a regression of PR 7464: the landing branch without the unit
  refuses the mirrored block at `Interval` as well (`Budget`; the
  review's control).

## Unprobed, and the first thing to test

**Cause not isolated.** For the block: why a wall presenting
`Zero::Through` against a face presenting `Zero::Point` at the same
vertex is not signed at `Interval` when it is at `f64` (the plane built
by `planes_through`, `On::holds`, or the sign of the face's rectangle
over an enclosure-wide plane). For the ring: why `sheets` answers
`Local` on 143 cells of a pair that shares an edge. Whether the two are
one cause is not known.

## Valid or in band

Valid: mirror images of valid solids, each of which holds unmirrored,
turned about a generic axis, moved a hundred units, and mirrored across
an axis plane, at both scalars.

## Evidence

Pinned as it reads:
`self_intersection_proves_injectivity::a_generic_mirror_refuses_the_ring_at_f64_and_the_block_at_interval`
(the block's `Interval` refusal by class; the ring's by class at both
scalars).

## 2026-11-24: the ring's half is closed by main; the block's stands

On `embed/wave3-land-next` (the landing branch with main `17e78d9c90`),
`Interval`:

- **The mirrored ring holds**: `Holds`, 21 candidate pairs, 249 cells
  discharged, 228 splits, 82 certified, none refused. The pair that
  refused (the inner wall `3v1` against its cap `4v1`, across the rim
  `6v1`) is certified at its root cell with no split, by the carrier
  arm: the wall is presented with its side on the rim, and it departs
  the cap's plane.
- **Why it was not presented before.** `sheets` reads the wall's extent
  in the angle off the stored lift and counts the copies of it, a whole
  turn apart, that the cell meets by more than `chart_slack`
  (`8 ε · 2π = 1.1e-14`). The extent's enclosure ends past a whole turn
  by its own rounding, so the copy a turn back reaches into the root
  cell by that much:

  | tree | extent in the angle | the copy a turn back overlaps the cell by | copies met | `sheets` |
  |---|---|---|---|---|
  | `0a1e9944db` | `(0.0009999999999799, 6.283185307179604)` | `1.78e-14` | 2 | `Local`, at every cell holding the wall's start |
  | with main | `(0.0009999999999920, 6.283185307179596)` | `9.8e-15` | 1 | the wall is presented |

  The face has no points in that overlap: its extent is a milliradian
  short of a turn, and the overshoot is the enclosure's width. So the
  copy is rightly missed, and the hold is the right verdict.
- **Which of main's changes**: main through `4ab816bbfb` already reads
  so (the scratch tree `0a1e9944db` + `018b163504`), so it is not
  conic-symmetry (PR 7640). Not narrowed further: the extent's enclosure
  is `1.8×` tighter, which is what a unit direction stored exactly
  (PR 7349) would do, but that is a guess from the numbers.
- **It is a margin of `1.4e-15`**: an enclosure `15%` wider puts the
  ring back at `Local`. What decides is whether an extent short of a
  turn, enclosed to a rounding past it, is read as meeting its own
  copy; a ring a milliradian short of a turn refusing for a `2e-14`
  enclosure is the defect, and it is `sheets`' to state, not this
  row's: filed as
  `work/embed/an-extent-short-of-a-turn-enclosed-a-rounding-past-it-meets-its-own-copy.md`.
- **Negative beside it**:
  `self_intersection_proves_injectivity::a_flap_through_the_mirrored_rings_wall_never_holds`
  (a triangle through the inner wall and one through the outer, in the
  mirrored pose: `Violated`, pair and whole body, both scalars).
- **The block at `Interval` stands**: `Budget { Depth 40, 3v3, 4v1 }`,
  on the grid and off it. The title's ring half no longer holds.

