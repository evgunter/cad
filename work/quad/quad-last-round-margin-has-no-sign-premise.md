---
id: quad-last-round-margin-has-no-sign-premise
kind: issue
title: nothing states that props_quad_last_round's recorded margin cannot exceed target_len, and a k-lint guard rests on it
status: open
opened: 2026-09-16
priority: P1
cost: D
---



## What

`crates/geom-brep/src/props/quad.rs`'s budget exit
(`last_round_refuses`) mints `props_quad_last_round` with
`Margin::of(target_len - last_round_len)`, where `last_round_len` is
`displacement_len(last_round_width_lo, area)?` — a length. Nothing
states that it is non-negative, so nothing states that the recorded
margin cannot exceed `target_len`.

## Finding

**A `tools/k-lint` guard rests on that premise and cannot see it.**
`tools/k-lint/tests/predicate_roster.rs`'s
`an_unruled_eps_coupled_margins_positive_side_is_loud_under_rule_3_at_the_tight_rows`
derives, from `QUAD_TARGET_LEN_FACTOR` read out of this file, that an
ε-coupled margin's positive side lies in `(0, QUAD_TARGET_LEN_FACTOR·ε]`
and therefore below `BASELINE_FLOOR_MARGIN` at the 1e-9 and 1e-12 rows
— so every positive row the family can record flags under rule (3)
there. **The interval's upper end is the unstated premise.** A kernel
that recorded a margin above the target would leave that test green
and rule (3) quiet on the rows it claims to catch, and no row anywhere
would notice.

**It is the same gap on the rostered side.** `props_quad_converged`'s
mint is `Margin::of(target_len - width_len)`, and
`EPS_COUPLED_FLOOR_RATIO`'s doc describes its statistic as *"bounded
above by 1024·ε"* — the identical premise, about the identical shape,
also unstated. That description is load-bearing: it is the half of the
constant's argument that says why a lower-tail P0 is the informative
cut for this family.

**The cheap repair is kernel-side and is PROPS' to make**: a
`debug_assert!` at each mint that the subtracted length is finite and
non-negative, or a typed length that cannot be negative. Either states
at the mint what two `tools/k-lint` documents already assume about it.
A tooling-side pin is not available — the property is about a runtime
value, and `predicate_roster.rs` reads source text.

**What is NOT at risk, so the priority is not urgency.** The ruling
that keeps `props_quad_last_round` off rule (4)
(`docs/K-REPORT.md`, 2026-09-16) does not rest on this: its guard fires
on the NAME appearing in a scanned file, whatever a row's sign or
size. What is at risk is the narrower claim about how loud the metre
rules are, and `EPS_COUPLED_FLOOR_RATIO`'s stated rationale.

**Confidence:** sure that neither mint asserts the sign and that no
test in `tools/k-lint` or `crates/geom-brep` states it — grepped both
mints' enclosing functions and `displacement_len`. Unsure whether
`displacement_len` can return a negative value in practice; the
finding is that nothing says it cannot, not that it does.

**Sweep and its blind spot.** Read both `classify_len` mint sites in
`props/quad.rs` whose margin derives from `target_len`, and their
enclosing functions, for an assertion on the subtracted length. The
pattern was the mint sites themselves, so it cannot match an
invariant stated somewhere else about `displacement_len`'s range — in
its own doc, or in a caller's — which would satisfy the premise
without being visible at the mint.

## Was

Raised by the falsification review of INSTR unit 12 (C1's caveat),
which re-derived the guard from the kernel and upheld it while
observing that its upper bound is unpinned.
