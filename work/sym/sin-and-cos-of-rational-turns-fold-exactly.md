---
id: sin-and-cos-of-rational-turns-fold-exactly
kind: issue
title: The symbolic tier folds sin, cos and tan of a rational multiple of π where the value is rational, so a copy spun by turn/4 is provably parallel
status: closed
opened: 2026-10-15
closed: 2026-10-20
priority: P2
cost: M
refs: [5898]
---


From FORK-MOVE (fork log row 149, PR 5189; `docs/INTENT-STAGE3-SPEC.md`, "The symbolic tier's right angles"). `Expr` evaluates `turn` to `2π`, with π an indeterminate in `Sym`. `sin_cos` mints an opaque atom that folds only at a zero form (`unary_at_zero`, `crates/geom-core/src/sym.rs`), so a copy spun by `turn/4` has faces the tier cannot prove parallel to its target. Separately, `Mat3::rotation_about` computes `turn/4` inexactly in floats.

The fix is one fold rule beside `unary_at_zero`: `sin`, `cos` and `tan` of a rational multiple of π fold where the value is rational. A typed `90 deg` binds its nominal in radians (VR8) and is not folded unless its nominal is bound from its written unit; this unit decides that. It is a fold rule, not a representation change, and it is independent of A′.

## Closed

Closed by INTENT stage 3 D (`transform-retires-into-a-placement`), in D's PR: this file is SYM's, and D's exact-turn lane closes it there as the orchestrator directed.

- **The fold.** `trig_at_rational_pi`, beside `unary_at_zero` in `crates/geom-core/src/sym.rs`, folds `sin`, `cos` and `tan` of a form `q·π` (`q` rational, π the form's indeterminate) to its value where that value is rational: `0`, `±1/2` or `±1` for a sine or cosine, `0` or `±1` for a tangent (Niven). The tables are `geom_core::Turns::{sin_halves, cos_halves, tan_rational}`. Row: `sym::tests::trig_of_a_rational_multiple_of_pi_folds_where_it_is_rational`, where `sin(π/4)` stays an atom.
- **The rotation half** (`Mat3::rotation_about` computing `turn/4` inexactly) is closed by `geom_core::Angle`: a spin whose angle is a constant fraction of a turn takes its sine and cosine from the fraction in every lane, so `Sym` receives the exact `0` and `1` there and needs no fold. Row: `intent_s3_d_placements::a_block_placed_upright_by_a_written_quarter_turn_stands_on_the_worlds_planes`.
- **A typed `90 deg`** is a dimensioned literal, which mints a free variable (D10 `[variables.literals]`, VR6). A variable carries no form, so it is neither folded nor read exactly. Only the constant spellings are: `turn` times or over constant rationals (`Expr::constant_turns`).

