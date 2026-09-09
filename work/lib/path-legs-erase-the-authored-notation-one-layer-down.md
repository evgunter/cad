---
id: path-legs-erase-the-authored-notation-one-layer-down
kind: issue
title: a path leg's 25 mm is recorded as a bare f64 before any Expr exists
status: parked
opened: 2026-09-09
refs: [node-slot-literals-erase-the-authored-notation]
blocked_on: [recorded-program-arguments-carry-no-notation]
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
