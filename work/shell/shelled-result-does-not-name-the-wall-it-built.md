---
id: shelled-result-does-not-name-the-wall-it-built
kind: issue
title: Shelled names operand shells to result solids, not the wall it built: the second shell_open cannot choose its t or name the inner wall
status: open
opened: 2026-09-08
---



Both SHELL-8 reviewers (PR #2207, 2026-09-08) hit the same two
frictions in their end-to-end exercise of `shell` and `shell_open`
on a multi-solid body, and the orchestrator files them as one item
because they share a cause: `Shelled<T>` carries a body and a
`ShellNaming` whose `thickened` map names operand SHELLS to result
solids, and nothing in it names the wall the verb just built.
(i) Thickness does not compose and the verb cannot say so usefully:
a second `shell_open` at the same `t` on the hollowed result refuses
`WallClearance { gap: 0.05, needed: 0.1 }` — correct, but the caller
has to already know the wall the previous call built to choose the
next `t`; the record it holds does not surface it. (ii) There is no
vocabulary for "the inner wall of the second part": after two
hollowings of a two-part body the result has 2 solids, 4 shells, 24
faces and 8 z-normal candidate faces distinguishable only by walking
geometry and matching a plane's offset (R1's
`r1_naming_the_inner_wall_after_two_hollowings`); the `RimNaming` and
`ShellNaming::outer` rows answer "where did operand face f go", not
"which face is the cavity ceiling". The gap is inherited from the
one-solid verb (LIB's `shell-mouth-chart-designated-in-full` is the
same friction one coat earlier) and SHELL-8 multiplies it by the
solid count; it is now the dominant cost of using the verb twice.
Candidate shape: a per-solid wall record on `Shelled` (thickness, the
inner twin of every outer face, `RimShell` side) that a second call
and LIB's `Node::Shell` can both read. Rides the naming-record
readers' seam (LIB-G17 consumes `ShellNaming`). Signed (SHELL
orchestrator).
