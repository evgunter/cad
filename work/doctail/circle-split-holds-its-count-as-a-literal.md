---
id: circle-split-holds-its-count-as-a-literal
kind: issue
title: LoopProgram::CircleSplit holds its count as a u32 literal, not a Count expression
status: open
opened: 2026-11-22
priority: P2
cost: M
---


Filed by the cut of `work/repeat/a-loop-cannot-repeat-inside-a-profile.md`
(both designers of fork log row 296 asked for it).

## What

`LoopProgram::CircleSplit { centre, radius, n: u32, phase }`
(`crates/editor-core/src/program.rs`) holds its subdivision count as a
`u32` literal. The variant's doc says "`n` STRUCTURAL", citing "corpus
ruling (a)". Every other count in the document is a `Count` expression
(`docs/DESIGN.md` `[variables]`; `k = index(N)` with `N` any `Count`
expression, `[repetition]`). A parameter can set a gear's tooth count
through `each k` but cannot set a `circle_split`'s arc count.

## Check first

Trace "corpus ruling (a)" (`git log -S'n STRUCTURAL' --all`) and read
what it decided. If Ev ruled the literal deliberately, this row is a
question for an `[ev]` PR, not a fix. The arc count changes the
structure (how many pieces are named), which is exactly what a `Count`
expression changes in an index's family, so `[repetition]` may already
answer it.
