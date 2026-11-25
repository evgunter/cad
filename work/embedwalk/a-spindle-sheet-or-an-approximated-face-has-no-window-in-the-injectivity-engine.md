---
id: a-spindle-sheet-or-an-approximated-face-has-no-window-in-the-injectivity-engine
kind: issue
title: The injectivity engine opens no window on a spindle sheet's face or an approximated face, so every finished body holding one refuses Unsupported at the at-rest gate
status: review
opened: 2026-11-21
priority: P0
cost: H
branch: embedwalk/spindle-window
pr: 7769
---


## Finding

From EMBED's per-solid census trace (lane `embed/trace-cone-torus`,
2026-11-21), on main `c2a0c41f40` with the at-rest gate
(`embed/gate-measure`).

`topo::clearance`'s window builder (the `match &charted` over the
lifted carrier, `crates/topo/src/clearance.rs`) returns
`unsupported("a spindle sheet's face")` for `Surface::Spindle` and
`unsupported(APPROXIMATED)` for `Surface::Approx`, twice each (the
stored and the lifted match). `injective::by_kind` says the same of a
spindle sheet ("has no injectivity lane yet"). No cell of such a face
is ever asked.

Both kinds are first-class carriers a finished body holds:
- `docs/DESIGN.md` `[surfaces]`: "Plane / cylinder / cone / sphere /
  ring torus / spindle sheet are first-class variants", and `Approx`
  is "the certified approximating class (OFFSET-DESIGN O2)".
- `crates/topo/README.md` (the full revolve): an arc whose circle's
  centre is off the axis by no more than the minor radius picks "one
  sheet of a spindle or horn torus, whose points on the axis are apex
  vertices in the pole form".

No clause says a finished body may not hold either, so each refusal is
a valid body refused at its door: P0.

## Count and reproducers

- **Spindle, 20 tests**, all f64. Cheapest:
  `sweep::all the_football_revolves::a_horn_reached_at_its_cusp_from_one_side_is_a_needle_tip`
  (0.09 s): `Refused(Unsupported { carrier: "a spindle sheet's face", face: FaceKey(3v1), side: First })`.
- **Approx, 7 tests**, all f64, all `sweep::all`. Cheapest:
  `a_moved_body_keeps_its_verdict::an_offset_cap_keeps_its_verdict_under_a_move`
  (0.14 s): `Refused(Unsupported { carrier: "an approximated face", face: FaceKey(1v1), side: First })`.

Checked: both reproducers refuse at the window with exactly that text,
which no other site raises.

## What is needed

Two units, not one:
- **The spindle sheet:** a window on its chart (the torus's two turns,
  with the apex vertices on the axis as collapsed sides, the reading
  PR 7115 gives a cone's apex and a sphere's pole), and `by_kind`'s
  injectivity on one sheet.
- **The approximated face:** a window on its fit's knot domain, the
  NURBS face's iso-bounded window (`iso_bounded_domain`), and a reading
  of the face through its fit and its distance certificate.

## Evidence: the spindle half (lane `embedwalk-spindle`, 2026-11-25)

The 20 spindle tests of the census, each solid asked at the gate's
budgets (depth 40, 65 536 pairs) after tier 3, on current main and on
the branch giving the sheet its window and by-kind discharge (the
gate's ask ported as a logging harness, f64 throughout):

| | sheet asks | `Embedded` | `Budget` | `Uncertified` | `Unsupported` | tests whose every ask holds | gate seconds |
|---|---|---|---|---|---|---|---|
| main | 143 | 0 | 0 | 0 | 143 | 0 | 7.4 |
| branch | 143 | 101 | 28 | 14 | 0 | 7 | 140.3 |

The other asks (non-sheet solids in the same tests: 60 `Embedded`, one
ring-torus `Budget`) are unchanged. The residue, each filed:
- `Budget(Pairs)` on two π-bands sharing meridians to rim corners, 26
  asks, ~3.5 s each, and the ring torus refuses the same:
  `two-bands-of-a-revolved-arc-sharing-their-meridians-spend-the-pair-budget`.
- Horn bodies refused departure at a shared edge, 8 asks, and a horn's
  part-way caps at wedge 0, 4: `a-horns-cusp-is-a-zero-angle-apex-no-certificate-parts` (EMBED).
- A part-way football's cap against its sheet, 2 departure and 2 depth:
  `a-part-way-revolved-sheets-cap-refuses-at-the-meridian-through-its-apexes` (EMBED).
- No station lands inside a sheet face, only on its boundary, so a
  crossing through one is confirmed only where boundary stations meet it:
  `the-point-door-places-no-point-on-a-spindle-sheet-face` (INSIDE).
- A full-revolve football does not finish at `Interval`:
  `a-full-revolve-football-does-not-finish-at-interval` (SPIRICARM).
