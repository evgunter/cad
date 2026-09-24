---
id: tier-3-does-not-check-shell-roles-per-solid
kind: issue
title: tier 3 does not check that a solid's shells bound a winding number of 0 or 1 everywhere, so a void outside every outer shell (or two overlapping outer shells) certifies when the solid's total volume is positive
status: parked
opened: 2026-09-08
priority: P0
cost: H
refs: [validate-tier3-curved-boundary-containment, check-9-nesting-is-line-bounded-only, 2977]
parent: ATREST-7
blocked_on: [point-in-solid-reads-out-from-inside-a-re-posed-torus-barrel]
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

## Re-stated again, 2026-09-24 (ATREST orchestrator) — the nested island is VALID

The 2026-09-21 re-statement said the unchecked claim is "a second
`Outer` NESTED inside that solid's own void". **That body is valid, and
tier 3 must not refuse it.** Read by winding number, which is what a
closed oriented boundary means: inside the island the shells contribute
`+1` (the wall) `-1` (the cavity) `+1` (the island) `= 1`, so the island
is material, and `work/zip/subtract-of-a-hollow-operand-files-the-island-under-one-solid`
itself records the total volume as *"the correct number"*. What that
ZIP row objects to is the GROUPING — the island filed under the wall's
solid rather than a solid of its own — and the kernel deliberately
files disconnected components under one solid elsewhere
(`graft_disjoint`'s onto door; the boolean coplanar split's three
prisms; `editor-core`'s placed union, counted against an authored
`expected_components` under `CheckId::Connectedness`). That is the
evidence that retracted check 10 (`work/atrest/log.md`, 2026-09-21), and
it retracts this reading for the same reason. The grouping stays ZIP's
output convention to pursue; it is not an at-rest invalidity.

**The real invariant this family is missing**: a solid's shells bound a
region whose winding number is `0` or `1` everywhere. Violations tier 3
does not see today, all with a positive per-solid total (so check 7,
now per solid, passes them):

- a `Void` shell lying OUTSIDE every `Outer` of its solid (winding `-1`
  inside the cavity — negative material);
- an `Outer` shell inside another `Outer` of the same solid and not
  inside a `Void` between them (winding `2` — doubly counted material);
- a `Void` inside another `Void` with no `Outer` between them (winding
  `-1`).

Under the no-crossing premise the rest of tier 3 already assumes
(global self-intersection is an explicit not-yet-checked item), one
witness point per shell decides it: the winding of the OTHER shells of
the solid at a point of shell `s` must be `0` if `s` is `Outer` and `1`
if `s` is `Void`. `boolean::solid_contain::point_in_solid_faces` answers
per shell selection, and refuses `KindUnsupported` on a curved face it
cannot certify — where the check is silent and names the residue, the
same posture as check 9's nesting arm (the false-refusal direction is
the one this program must not add to).
