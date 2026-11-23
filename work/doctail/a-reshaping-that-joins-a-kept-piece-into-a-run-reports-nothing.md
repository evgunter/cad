---
id: a-reshaping-that-joins-a-kept-piece-into-a-run-reports-nothing
kind: issue
title: A SetProgram that joins a kept piece's wall into a run (a station inserted after it) strands the single-piece wall name without reporting it
status: closed
opened: 2026-11-09
priority: P0
cost: M
closed: 2026-11-20
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
loop, and either loop of two).

**The moved-fillet row is lifted (#7075's value-free door).** The strand
report compares what each program's structure can draw, so the moved
fillet, whose structure still draws a run in, no longer reports
`Lateral([run in])`; and the broken run's name rests on the dropped
leg, so it is reported. The row passes at a rise where the moved run
in fits `Zero` and at one where it fits `Positive`:
`edit_set_program::a_fillet_moved_off_a_corner_reports_exactly_what_it_takes`.

## Done when

A reshaping reports a held wall name whose run it changed (joined or
broken) exactly as it reports one whose piece it stopped drawing, or a
ruling says the run's offer is the whole answer and the contract in
`report_matches_resolution` changes; the pinned row is lifted either way.
Where the door reads a run's grouping from is #7082's ruling (an
`[ev]` PR, unruled), not this row's. The sweep's own grouping
(`crates/sweep/src/swept.rs` `cosurface_pairs`/`wall_runs`) is not
it: it is crate-private, and it decides a join from values within a
band, which would bring ε back into DM7's structural comparison
against `[recipe.doors]`. #7082 proposes validation's per-joint carrier
verdict as the one home, read at generic values for the structural
strand report.

## Unparked (2026-11-20)

The rulings it waited on are merged: #7075 and #7116 (the value-free
door, which compares what each program's structure draws) and Ev's
`[ev]` PR #7082 (`959ecfe`), whose DM7 text in
`crates/editor-core/REFERENCES.md` ("What a reshaping strands") is the
source: the door compares the partition the old and the new program's
structure draws at generic values, its pieces, runs and corners, read
off the program's shape with no value, and judges each held name whole.

## Closed

The shape walk (`profile::chain_shape`, `crates/profile/src/path/shape.rs`)
answers, per piece, whether the step order constructs it onto the
carrier of the piece before (`ChainShape::continues`): a straight
segment departing along a straight segment's own ray (a line off a leg
end, `.tangent()` then a line, `continue_to`, a fillet's run in off its
leg), every station of a `circle` or a `circle_split`, and both joints
of a lone arc closed by a `tangent_arc_to` off its own tangent, which
is that arc's circle again. Every other joint is a corner: no other
verb reads an incoming arc's circle, so any other arc-to-arc
continuation on one circle is a value coincidence. `chain_shape_random`
holds the answer against the replay's constructed joints in three
ways. A static row covers each construction. A counterexample search
over chains built around one continuation of each kind asserts it
reached every kind. The random chains of the table's verbs are steered
toward continuations; for arcs that search is two-way, and a
coincidence is told from a miss by redrawing the chain's values.

`ProfileProgram::partition` (`ProfilePartition`) reads the pieces, the
walls a sweep builds over each run (spelled as a wall name spells it;
a split circle keeps its stations) and the corners off that shape, and
replaces `ProfilePayload::drawable_pieces`. `StableName::referents`
reads each locator segment whole: `Lateral`, `RimEdge`, `Band`,
`BandPi`, `Meridian` and `AxisEdge` as their run; `LateralEdge`,
`CapVertex`, `BandRim`, `BandRimPi`, `MeridianVertex` and `Pole` as a
corner; a loft's pairings as pieces. The `SetProgram` door reports a
held name with a referent on kept steps that the old partition holds and
the new one does not, as `Took::Piece` where a piece went undrawn, else
`Took::Run` or `Took::Corner`. `MaintenanceNet` keeps such a strand only
where the action's start document draws what the name denotes and its
end document does not.

The pinned rows are lifted as
`edit_set_program::a_fillet_inserted_after_a_kept_leg_reports_the_legs_wall`
(plus a clockwise loop and a revolve), beside
`edit_set_program::a_reshaping_that_joins_or_breaks_a_run_reports_it_whole`;
`edit_set_program::a_fillet_moved_off_a_corner_reports_exactly_what_it_takes`
stays green. The axis edge and pole cases the harness cannot hold end
to end are
`work/doctail/an-axis-edge-or-pole-a-reshaping-takes-is-reported-nowhere-end-to-end.md`.
