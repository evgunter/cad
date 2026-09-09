---
id: recorded-program-arguments-carry-no-notation
kind: issue
title: a recorded path program holds bare f64 arguments, so a leg's written unit is gone before any Expr exists
status: open
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
