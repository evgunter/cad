---
id: tier-3-does-not-check-shell-roles-per-solid
kind: issue
title: tier 3 does not read where one shell of a solid sits relative to another, so a second Outer shell NESTED inside that solid's own void is unchecked
status: open
opened: 2026-09-08
priority: P0
cost: H
refs: [validate-tier3-curved-boundary-containment, check-9-nesting-is-line-bounded-only, 2977]
---


Measured by the SHELL-5 lane (PR #2159, 2026-09-08) and placed here by
the SHELL orchestrator. Re-stated 2026-09-21 by ATREST-1, which
measured the count-level reading of it and refuted it.

## The finding, at its real subject

A body whose one solid holds an `Outer` shell, a `Void`, **and a second
`Outer` inside that void** — the boolean's hollow-operand subtraction,
`work/bool/subtract-of-a-hollow-operand-files-the-island-under-one-solid`
— passes `validate_geometric`. No tier reads where one shell of a solid
sits relative to another: tier 2's per-shell Euler–Poincaré and tier
3's per-face checks never ask, and `classify_shells` is a props read no
tier consumes.

**The unchecked claim is the NESTING, not the count.** That distinction
was not in the original filing and is what makes this row actionable:
an island sitting inside a cavity of the solid that owns it is material
the cavity says is not there, and it is a containment statement. The
COUNT — several `Outer` shells under one solid — is a state five kernel
doors produce deliberately, and tier 3 may not refuse it; that was
measured on 36 pinned rows and is recorded at
`one-solid-holding-two-outer-shells-is-what-five-kernel-doors-produce`.

**The band stays P0.** A solid holding a second `Outer` nested in its
own void is a body a normal verb produces — the hollow-operand
subtraction — and that tier 3 blesses: a live wrong answer. What
ATREST-1's measurement moved is this row's shape and cost, not its
severity, and `cost: H` carries the effort separately.

## Why it is not cheap

Tier 3 has no at-rest containment walk at all. This is the same family
as check 9's deferred nesting half
(`check-9-nesting-is-line-bounded-only`) and
`validate-tier3-curved-boundary-containment`: the shape that would
answer it is `shell::encloses`-shaped, not flux-shaped, and a
sign-level read of a shell's volume — which ATREST-1 built and is
available as `props::sign_certified` restricted to a shell's faces —
cannot see it. Whether the three nesting gaps want one walk between
them is the design question this row opens.

SHELL-5 pins its own grouping in its rows (`roles_by_solid` in
`crates/sweep/tests/verbs_shell.rs`) rather than adding a check.
