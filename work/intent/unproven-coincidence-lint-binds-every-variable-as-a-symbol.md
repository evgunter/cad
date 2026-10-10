---
id: unproven-coincidence-lint-binds-every-variable-as-a-symbol
kind: issue
title: Stage 4's unproven-coincidence lint binds every variable as a symbol in its own pass
status: open
opened: 2026-10-06
---


INTENT-LITERALS PR C revised VR8 (Ev, via the orchestrator; spec §11,
"Q3 and Q4, revised at C"): a variable is an analysis axis only if it
carries a tolerance, and a free variable without one is a constant in
every analysis lane. In the symbolic tier that means
`var_env_over` (`crates/editor-core/src/analysis.rs:1052`) binds an
untoleranced variable as its exact nominal, not as a symbol
(`analysis.rs:1068`, the rule's one home `is_axis` at
`analysis.rs:329`).

That is a numeric convenience for the box question, and it is not a
structural claim. Two separately typed `5 mm` are two variables with
distinct tokens, yet in this lane `x − x'` over them decides Zero,
because both bind the constant 5 mm.

Stage 4's structural rung, the `unproven-coincidence` lint
(`docs/DESIGN.md:263`, `:344`), asks exactly the opposite question:
whether two quantities coincide **by structure**. It must therefore
bind **every** variable as a symbol keyed by its id, toleranced or not,
in its own pass. It must not reuse `var_env_over`'s environment, where
an untoleranced pair is equal by number. Otherwise every coincidence of
two equal typed values reads as proven, which is the coincidence D10
says the kernel must not trust.

Done when the lint's symbolic binding is its own door, and a row shows
two separately typed equal values flagged by the lint while the
analysis lane decides their difference Zero.
