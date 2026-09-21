---
id: path-legs-erase-the-authored-notation-one-layer-down
kind: issue
title: a path leg's 25 mm is recorded as a bare f64 before any Expr exists
status: open
opened: 2026-09-09
refs: [node-slot-literals-erase-the-authored-notation]
priority: P3
cost: H
---



The sibling `node-slot-literals-erase-the-authored-notation`'s own
sweep predicted: "a door that constructed an `Expr` some other way
... would not appear". This is that door, found while closing it at
LIB-SEATS and not fixable there.

## What happens

The path vocabulary takes typed quantities —
`crates/pncad-py/src/py/path.rs:92` lowers a `(Length, Length)` to a
`Point2<f64>`, and every leg's radius, angle and target crosses the
same way — and a closed loop reaches the document through
`crates/pncad-py/src/py/path.rs:1343`'s `loop_program`, which calls
`LoopProgram::from_recorded`. The binding builds no `Expr` at all:
the program's literals are minted kernel-side out of the recorded
`f64`s, canonical and unitless, so
`Open.at((0 * mm, 0 * mm)).line_to(...)` records `0.0` and the `mm`
is gone before an expression exists.

The consequence is visible in one document: a polygon vertex authored
through `Node.polygon` now reads back `25 mm` (LIB-SEATS), and the
same vertex authored through the paths vocabulary reads back
`0.025 m`. Two spellings of one authoring disagree — the exact
sentence the parent item opened with, one vocabulary over.

## Why LIB-SEATS could not take it

Not a binding fault. Rust's own path API is `f64` throughout and its
`RecordedProgram` holds bare numbers, so the Python surface MIRRORS
Rust exactly here; converting the seat would make Python wider than
the kernel, which is the thing the `(H)` ruling's mirror argument
refuses. Recording notation through a path leg is a decision about
`RecordedProgram` — a kernel shape — and the Python door follows it
rather than leading it.

## What closing it would have to decide

Whether a recorded program's arguments carry the notation they were
authored in (a `WrittenLength` per leg argument, or a display unit
beside each recorded `f64`), and what that costs the recorder's
replay identity — the program is compared and re-run, and a unit is
presentation metadata under D7 that must stay out of `bit_eq` exactly
as a literal's does.

## Unparked (2026-09-16, EDIT orchestrator)

The trigger fired: EDIT's `recorded-program-arguments-carry-no-notation`
merged as PR #2779 with the kernel shape this row follows —
`RecordedNotation` beside the recording, keyed by `(step, StepArg)`,
applied at `LoopProgram::from_recorded_with_notation`; the binding
census charter `B-PATH-NOTATION` names the Python half. This header was
edited from outside LIB's fence only to keep the tracker true (a row
parked on a closed trigger is a lint error); nothing else here is
LIB's decision made for it.

## Widened (2026-09-19, PR #2876)

EDIT's `recorded-notation-makes-a-rust-author-count-step-indices`
landed two doors this row's Python builder threads through, so the
work it names is now "call these", not "design these". Recorded here
because LIB reads this file, not EDIT's tracker; LIB's row, by
announcement, and nothing about what LIB decides is changed.

- `profile::PartialPath::recorded(&self) -> &[Step<T>]` — the steps
  recorded so far, in program order, the prefix of the program the
  chain publishes. Answered by every builder state that holds the
  core (the partial path and the arrival builders a verb hands back),
  so it is reachable at the moment a step exists rather than one
  state later; after the closer the same slice is
  `ClosedLoop::program`.
- `editor_core::RecordedNotation::set_after(&mut self, recorded,
  arg, unit)` — writes the notation of the LAST recorded step, the
  index being `recorded.len() - 1` and never a count. A binding that
  records an entry per typed quantity it lowers writes it at the leg
  it just lowered, and never spells a step number.

What that means for the charter, which is LIB's own text and is cited
rather than edited: `B-PATH-NOTATION`
(`crates/pncad-py/tests/test_binding_census.py`) asks for "the path
builder to record a `(step, StepArg)` entry for every typed quantity
it lowers" and for "`loop_program` to hand the notation to the
lifting door instead of dropping it". The first half is `set_after`
called at each lowering site in `py/path.rs`; the second is
`py::path::loop_program` calling `LoopProgram::from_recorded_with_notation`
where it calls `from_recorded` today.

Until that happens, both notation words in
`crates/pncad-py/src/tags.rs`'s `RecordedProgramError` map —
`notation_off_program` and `notation_before_any_step` — are
vocabulary no Python caller can reach; that map's doc says so and
points back here.
