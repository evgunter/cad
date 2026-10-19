---
id: sin-and-cos-of-rational-turns-fold-exactly
kind: issue
title: The symbolic tier folds sin, cos and tan of a rational multiple of π where the value is rational, so a copy spun by turn/4 is provably parallel
status: open
opened: 2026-10-15
priority: P2
cost: M
refs: [a-pose-is-moved-by-lengths-and-angles]
---


From FORK-MOVE (fork log row 149, PR 5189; `docs/INTENT-STAGE3-SPEC.md`, "The symbolic tier's right angles"). `Expr` evaluates `turn` to `2π`, with π an indeterminate in `Sym`. `sin_cos` mints an opaque atom that folds only at a zero form (`unary_at_zero`, `crates/geom-core/src/sym.rs`), so a copy spun by `turn/4` has faces the tier cannot prove parallel to its target. Separately, `Mat3::rotation_about` computes `turn/4` inexactly in floats.

The fix is one fold rule beside `unary_at_zero`: `sin`, `cos` and `tan` of a rational multiple of π fold where the value is rational. A typed `90 deg` binds its nominal in radians (VR8) and is not folded unless its nominal is bound from its written unit; this unit decides that. It is a fold rule, not a representation change, and it is independent of A′.
