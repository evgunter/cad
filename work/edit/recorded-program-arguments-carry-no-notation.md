---
id: recorded-program-arguments-carry-no-notation
kind: issue
title: a recorded path program holds bare f64 arguments, so a leg's written unit is gone before any Expr exists
status: review
pr: 2779
branch: edit/recorded-argument-notation
opened: 2026-09-09
---


Handed off by LIB (2026-09-09) from
`work/lib/path-legs-erase-the-authored-notation-one-layer-down.md`,
which carries the measurement; this file is the DOCM-side home
because the shape it asks about is `RecordedProgram`'s
(`crates/editor-core/src/program.rs`, DOCM's paths), not the
binding's.

## What LIB measured

After LIB-SEATS (#2264) a polygon vertex authored through
`Node.polygon` reads back `25 mm`, and the same vertex authored
through the paths vocabulary reads back `0.025 m`: the Python path
door lowers `(Length, Length)` to `Point2<f64>` and reaches the
document through `LoopProgram::from_recorded`, whose literals are
minted kernel-side out of the recorded `f64`s, canonical and
unitless. Rust's own path API is `f64` throughout, so Python mirrors
Rust here exactly; the binding cannot lead.

## What closing it decides

Whether a recorded program's arguments carry the notation they were
authored in — a `WrittenLength` per leg argument, or a display unit
beside each recorded `f64` — and what that costs the recorder's
replay identity: the program is compared and re-run, and a unit is
presentation metadata under D7 that must stay out of `bit_eq`
exactly as a literal's `display_unit` does. LIB's Python half (the
path door recording the written form) follows whatever shape lands
here; the LIB item is parked on this one.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/edit/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): EDIT is DOCM's successor on the document-model ground (persist, the edit vocabulary, the node and resolver doors). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Spec (2026-09-16, EDIT orchestrator) — middle tier: one opus style review with a correctness arm, no A/B row

Branch `edit/recorded-argument-notation`. **Ruled by precedent, not
by a new decision:** D7 makes a unit presentation metadata outside
`bit_eq`, and the document-parameter family already carries one
(`DocParam::display_unit`, `Expr::literal_with_unit`); a recorded
program's argument carries the same thing the same way. The PR body
says where the orchestrator looked (`docs/DESIGN.md` D7; the doc-param
unit's PR #2732 body; `git log -S` finds no clause deciding the
recorded program's notation).

1. **The shape.** A recorded argument is a value with the notation it
   was authored in: beside each recorded length or angle `f64` in
   `Step<f64>` (`crates/editor-core/src/program.rs`,
   `LoopProgram::from_recorded`'s input), an optional `UnitSym` — the
   doc-param shape, not a `WrittenLength` per leg (a `WrittenLength`
   re-types the number; the unit beside it does not). `from_recorded`
   mints `Expr::literal_with_unit` where a unit is present and
   `Expr::literal` where it is not, so a program recorded without
   notation is bit-identical to today's. Scalars (bulge, director
   components) carry none.
2. **Identity.** `bit_eq` is unchanged: two recordings of one leg, `25
   mm` and `0.025 m`, are `bit_eq` and evaluate to one geometry; a row
   pins it. The recorder's replay (`RecordedProgram` compared and
   re-run) is identical for a recording with no notation; a row pins
   the round trip of a recording with notation through save and load
   (the literal's unit persists as every literal's does).
3. **The seam.** `LoopProgram::from_recorded` is the one door; the Rust
   path API that records the `f64`s (`Step<f64>`'s producers in
   `crates/profile`? measure where `Step<f64>` is minted) gains the
   notation only where a caller wrote one. The Python half
   (`work/lib/path-legs-erase-the-authored-notation-one-layer-down`,
   parked on this row) follows the shape; do not build it — the PR
   body announces the shape and that row unparks at merge.
4. **Rows**: a leg authored in millimetres reads back millimetres
   through the document (RED today: it reads back metres — write it
   first); `bit_eq` across notations; the round trip; a recording with
   no notation unchanged byte-for-byte in the corpus goldens (none may
   move — say so). The sweep: every producer of `Step<f64>` and every
   consumer of a recorded argument, with what the pattern cannot match.

## Built (2026-09-16)

`RecordedNotation` in `crates/editor-core/src/program.rs` and
`LoopProgram::from_recorded_with_notation` beside `from_recorded`, with
`crates/editor-core/tests/edit_recorded_notation.rs` as the rows.

**The spec's shape moved, and the spec's own clause 3 asked for the
measurement.** Clause 1 put an `Option<UnitSym>` beside each `f64` in
`Step<f64>`; `Step<T>` is `profile`'s, `profile` depends on `geom-core`
alone, and D6's first paragraph is that kernel-internal code carries no
dimensional types (G1 layering keeps `quantity` and `UnitSym` above
`profile`). So the notation travels BESIDE the recording and meets it at
the crossing D6's second paragraph names: a value entering a document
carries the unit it was written in. Same precedent, same shape as
`DocParam::display_unit`, one door.

The notation is keyed by `(step, StepArg)` — the pair
`SlotId::Profile` addresses an expression by — so the lift applies it
through the same `expr_mut` the slot doors read, and there is no second
table of which argument is which. A unit whose quantity is not the
role's dimension refuses at `set`; an entry addressing an argument the
recording has none of refuses at the lift
(`RecordedProgramError::NotationOffProgram`), never silently dropped.

`bit_eq` is untouched: `Expr::bit_eq` already excludes the display unit,
so `25 mm` and `0.025 m` are one program and replay to the same
vertices. No golden moved. `from_recorded` is unchanged and an empty
notation returns its answer bit for bit.

Not built: the Python half (`work/lib/path-legs-erase-the-authored-notation-one-layer-down`,
parked on this row, unparks at merge).

Mechanical follow-throughs on LIB's ground, each forced by one of the
two new names: `crates/pncad/src/document.rs` carries the type (the
facade completeness guard refuses an uncarried root export);
`crates/pncad-py/tests/test_binding_census.py` dispositions it as
`gap: B-PATH-NOTATION`, on the `B-EDGE-KIND` precedent — a kernel door
reaching the facade while the binding stays where it is; and
`crates/pncad-py/src/tags.rs` plus its inventory in
`crates/pncad-py/src/tests.rs` gain the `notation_off_program` arm,
because the binding's tag map is an exhaustive match over
`RecordedProgramError`.

CI: run `35133793584` on `388213ed5`, SUCCESS — 38 jobs, twelve
`test (...)`, five `k-lint (gate, ...)`, the python suite's own steps
read. One red round before it (`35132552404`), from the two
follow-throughs above being invisible to a `-p editor-core` check; the
PR body carries that record.
