---
id: cone-ring-rows-sheets-do-not-hold-their-islands
kind: issue
title: cone_ring_rows' corner-escalation sheet does not hold its island, so the winding's references disagree (read first as a path_parity miscount)
status: closed
opened: 2026-11-09
priority: P3
cost: M
closed: 2026-11-09
---


## Finding

Filed by `onecopy/pinch-premise`. A scratch check that reads every rung of
`chord_join::IslandReading::read_paths` (the winding references, an
`Answering::Any` ladder) found the fixture of
`chord_join::cone_ring_rows::a_path_escalating_by_a_corner_hands_the_winding_to_the_next`
reading the references [Negative, Negative, Negative, **Positive**,
Negative]. Production takes the first, which matches the oracle, so the
test is green.

The first path's lean is Negative at every reference (|m| ≈ 1). The
parities differ. Three of the references lie on one outer-loop ruling, at
azimuth +0.6:

| reference | point | parity |
|---|---|---|
| vertex | (0.0238, 0.0163, 0.05) | odd |
| vertex | (1.4295, 0.978, 3.0) | odd |
| the edge's midpoint | (0.7267, 0.4971, 1.525) | **even** |

The ruling is an edge of the outer loop, which the island does not cross,
so the three paths' ruling legs cross nothing and their parities must
agree. One reading is a definite miscount in `ring_path::path_parity`:
either the midpoint's leg counts a crossing, or the endpoints' legs miss
one.

The fix: log the per-piece crossings, find the miscounted piece and fix it
in `path_parity`, then pin it with the three same-ruling references
agreeing.

## Resolution (`onecopy/path-parity`)

**Not a miscount.** The per-piece crossings show every reading is right:

- From h = 0.05, the ruling leg crosses island arc 0 at param −0.43, inside
  its span [−0.658, 0.431].
- From h = 3.0, it crosses arc 2 at param 0.600, inside its span
  [−0.6, 0.9].
- From the midpoint, it crosses nothing.

The fixture's sheet, `sheet_to`, spanned azimuths ±t with t ≈ 0.6, so
that its −t ruling passes a hair outside the sector's corner. The
sector's island runs to azimuth 0.9, so it crossed the sheet's +t ruling
twice, and that ruling's midpoint lay inside the island. The face was
invalid: its ring crossed its outer loop. `read_paths`'s premise, that
every outer-loop point is on one side of the island, failed on the input
and not in the reader.

**The fix:**

- `sheet_to` and `sheet_up_to` take the sheet's azimuth range, and the
  corner row's sheet runs from just past the corner to 1.2.
- The sense-`false` sheets of `cone_ring_rows`, `torus_ring_rows` and
  `sphere_island_rows` were each built with the sense-`true` loop. At
  sense `false` the face was the surface minus the rectangle, so the
  island sat in its hole. Each now walks its loop from the other ruling
  or meridian, so the face is the rectangle at either sense (PR 6607's
  review).
- `run_from` asserts, sampled, that the island lies in the sheet before
  it builds the ring (`assert_island_in_face`): no corner, and no point
  of nine stations on each edge, is placed outside the face by the face's
  own region door. The torus and sphere rows' `empty_ring` assert the
  same of every ring they plant. The old narrow sheet, and the old
  sense-`false` loops, turn them red.
- The old sense-`false` sheets were not passed at rest:
  `validate_geometric` reports `CurvedSenseInverted` on all twelve cone
  sheets, so check 6 refuses them.
- `read_paths` states the premise and names what establishes it
  mid-operation: the construction, with nothing re-checking it there
  (`the-ring-lanes-island-is-not-checked-in-its-face`).

With the scratch every-rung check over `chord_join::` and the cone rows,
the references agree.
