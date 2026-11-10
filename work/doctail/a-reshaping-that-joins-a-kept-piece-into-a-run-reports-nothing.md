---
id: a-reshaping-that-joins-a-kept-piece-into-a-run-reports-nothing
kind: issue
title: A SetProgram that joins a kept piece's wall into a run (a station inserted after it) strands the single-piece wall name without reporting it
status: open
opened: 2026-11-09
priority: P0
cost: M
---


## The finding

Found by ROUND's `extensions-are-their-own-segments` unit (Ev's ruling on
PR 6542: an extension's incoming run is a segment of its own). Extrude
and revolve build one wall per run of pieces on one carrier and name it
by the run (`crates/editor-core/src/names/README.md`, N1 "Swept walls
over a run"), so `Lateral([leg])` and `Lateral([leg, next])` are
different names. `DocEdit::SetProgram`'s strand report reads only which
pieces a program DRAWS (`undrawn_kept_pieces` and `stranded_steps`,
`crates/editor-core/src/edit.rs`), never how the sweep groups them into
walls. A reshaping that keeps a leg and inserts a collinear piece after
it leaves the leg drawn, so nothing is reported, while the held
`Lateral([leg])` stops resolving: the edit took the referent silently.
N1's covers (`Lateral([p0, p1])` covers `Lateral([p0])`) make the run
wall an OFFER for the vanished name; they do not make the strand a
non-event.

Reproduced on main (`85ddde810c`): in
`edit_set_program::a_reshaping_reports_exactly_the_held_names_whose_referent_it_takes`,
a row taking `corner_at(false, ..)` to the same loop with its bottom
`line(2)` re-spelled `line(1).tangent().line(1)`, steps 0–2 and 4–6
kept, fails `report_matches_resolution`: the held
`Lateral(Piece { step: <bottom leg>, role: Leg })` resolves before,
not after, and is not reported.

**Raised to P0 (ROUND, 2026-11-09):** the class pre-exists, but
ROUND's `extensions-are-their-own-segments` unit (PR 6682) widened it to
the most ordinary reshaping there is — inserting a fillet at a corner
beside a kept `line` now strands a held wall name silently.

Since the ruling, a bare `fillet(r)` after a straight leg draws its run
in as a segment of its own after the leg, so the most ordinary
reshaping — inserting a fillet at a corner after a kept `line` — now
reaches this too: the sharp square's bottom wall `Lateral([leg])`
becomes `Lateral([leg, run in])`. The other direction is the same
gap: moving the fillet to another corner breaks the run
`Lateral([leg, run in])`, and the door REPORTS `Lateral([run in])`,
a name that never resolved, because the run in is no longer drawn.
The rows of that test that reach it are split out and pinned red on
this row:
`edit_set_program::a_fillet_inserted_after_a_kept_leg_joins_its_wall_unreported`
(insert, one pinned row per sub-case: four radii and scales on one
loop, and either loop of two) and
`edit_set_program::a_fillet_moved_off_a_corner_reports_a_run_piece_that_never_resolved`.

## Done when

A reshaping reports a held wall name whose run it changed (joined or
broken) exactly as it reports one whose piece it stopped drawing, or a
ruling says the run's offer is the whole answer and the contract in
`report_matches_resolution` changes; the pinned row is lifted either way.
The door would need the sweep's run grouping (the cosurface verdicts in
`crates/sweep/src/swept.rs`, `cosurface_pairs`/`wall_runs`) or a
profile-side reading that agrees with it.
