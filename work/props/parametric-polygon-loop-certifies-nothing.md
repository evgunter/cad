---
id: parametric-polygon-loop-certifies-nothing
kind: issue
title: A profile loop with a document parameter in a polygon vertex certifies nothing at any box width
status: open
opened: 2026-09-13
---

## What

Measured by DOCM-9 while looking for a fixture whose field has a real
boundary (`crates/editor-core/tests/docm9_range.rs`'s history; the
numbers below are from the scratch runs that chose the shipped
fixtures, at branch `docm/9-certified-range`).

A two-loop profile — a 2x2 literal plate and a 0.4-wide square hole
built with `LoopProgram::polygon_expr` whose x coordinates are
`Expr::add(Expr::param("hole_x"), literal)` — certifies **no leaf at
all** over a `Band` seed of `±1e-7` on `hole_x`, at `max_depth = 24`
and 64 leaves. Every leaf comes back `Budget`. The same document with
the hole's centre a LITERAL and the extrusion distance the parameter
certifies the whole `±0.1` box in one leaf, and the same document with
the hole a `LoopProgram::Circle` whose CENTRE is the parameter
certifies `±0.05` in 1.7 s. So it is neither the second loop nor the
parameter reaching profile geometry; it is a parameter inside a
polygon vertex.

`±1e-7` on a unit-scale document is `100 ε`, and the E12 symbolic tier
exists precisely so that certification width is not bounded by ε
(`crates/editor-core/src/drive.rs`, `SymbolicDials`). A box this
narrow refusing every leaf says an identity the tier cannot discharge
is being formed per vertex, not that the box is too wide.

## Why it is PROPS'

The subject is what the certification identities look like when a
parameter enters a canonical polygon vertex, and whether the E12
normal form can cancel them — the drive's certification width, on
`crates/editor-core/src/drive.rs`'s ground, adjacent to the arc family
M10-8 measured (`docs/DOC-LEDGER.md` sweep 13). If the answer turns
out to be the profile canonicalization's rather than the tier's, the
row moves to the program that owns `crates/profile/*`.

## What is not claimed

The measurement is one fixture family and was taken to choose test
documents, not to characterise the class: no sweep was run for other
parametric vertex shapes, and the numbers are dev-profile wall clock
on one machine.
