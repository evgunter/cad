---
id: parametric-polygon-loop-certifies-nothing
kind: issue
title: A profile loop with a document parameter in a polygon vertex certifies nothing at any box width
status: open
opened: 2026-09-13
priority: P1
cost: H
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

## Widened, 2026-09-14: it is not the second loop, and it is not "no leaf at any width"

Both DOCM-9 review lanes reproduced the finding independently and the
fix pass then bracketed it, which moves the claim in two directions.

**Narrower than filed in one way**: the certification width is not
zero. A SINGLE loop with ONE parametric vertex coordinate — a unit
square whose two right-hand x coordinates are `Expr::param("x")`,
extruded by a literal — certifies over a seed of ±1e-8 and nothing at
±3e-8 (24 depth, 64 leaves, all-or-nothing at each width):

| seed | certified | refused |
| --- | --- | --- |
| ±1e-9 (`1 ε`) | 8 | 0 |
| ±3e-9 | 16 | 0 |
| ±1e-8 (`10 ε`) | 64 | 0 |
| ±3e-8 | 0 | 64 |
| ±1e-7 (`100 ε`) | 0 | 64 |

**Wider than filed in every other way**: one loop is enough (the
original evidence had two, which suggested containment), one vertex is
enough, and the width it collapses to is `~Kε`. That is the PRE-E12
state — a leaf certifying only below a fraction of the band — reached
again on a shape the symbolic tier is supposed to cancel, which is
what makes this the tier's question rather than the profile
canonicalization's. The comparison holds fixed everything else: the
same document with a LITERAL vertex certifies ±0.1 whole, and with a
parametric CIRCLE centre certifies ±0.05.

## What is not claimed

The measurements are one fixture family, dev-profile wall clock on one
machine, taken to choose test documents and then to bracket the width
— not a sweep. No other parametric vertex shape was tried (an arc's
endpoints, a chain's `Toward` director, a `CircleSplit` phase), and
nothing here says where between ±1e-8 and ±3e-8 the width actually
falls.
