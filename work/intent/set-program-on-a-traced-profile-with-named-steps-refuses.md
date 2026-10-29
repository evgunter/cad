---
id: set-program-on-a-traced-profile-with-named-steps-refuses
kind: issue
title: SetProgram on a traced profile whose kept steps carry names refuses, because drawn_pieces resolves with no traces
status: open
opened: 2026-10-29
---


Found by INTENT stage 6 PR C (`work/intent/a-sketch-continues-tangent-to-a-surfaces-trace.md`).

A `SetProgram` on a profile drawn on a frame works out which named
pieces of its kept steps the new program stops drawing. It does that in
`undrawn_kept_pieces` (`crates/editor-core/src/edit.rs:5229`), through
`ProfilePayload::drawn_pieces` (`crates/editor-core/src/program.rs:2743`).

`drawn_pieces` resolves the program on its own, through
`ProfileProgram::pieces` → `validated` → `replay_records`
(`program.rs:2323`, `:2380`), and passes no traces. A step that runs from the pen (a trace, or a written coordinate)
therefore resolves `TraceStepFault::Unevaluated`. The new program's refusal
is mapped to `EditError::ProfileProgramRefused` (`edit.rs:5263`), so the
edit refuses. The old program's refusal is dropped (`.ok()`, `:5260`).

This happens only when some carried name spells a piece of a kept step
(`edit.rs:5252`). A `SetProgram` on a traced profile with no names held
on it goes through.

**Done when** the stranding check reads the pieces a traced program
draws. Either it reads them from the evaluation's own pieces, as
`ProfilePieces` publishes them for the node, or `drawn_pieces` is handed
the traces the evaluation read. A test in
`crates/editor-core/tests/intent_s6_c_trace.rs` then writes a
`SetProgram` that keeps a named step of a traced boss and is not
refused.
