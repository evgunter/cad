---
id: props-curved-carries-two-readings-of-d9-unreachable-vs-poison
kind: issue
title: props/curved.rs answers a kernel-bug-only state two ways — unreachable! at the wedge helper, a poison value at unreachable_zero "rather than panicking (D9)"
status: open
opened: 2026-09-16
refs: [2748]
priority: P1
cost: D
---

Found by BOOL-5's R2 review (PR 2748) and filed by the S-BOOL
orchestrator on PROPS's slate. One file carries two readings of D9 for
the same class of state (a branch the kernel's own invariants make
unreachable): the wedge helper panics with `unreachable!` and states
why that is D9-compliant (a kernel-bug-only state; a typed refusal
would launder a bug into a user-facing refusal, a zip would truncate),
while `unreachable_zero` returns a poison value with a comment saying
it does so "rather than panicking (D9)". Both cannot be the file's
rule. BOOL-5 stated its argument at its own site and left the
file-wide question here. The fix is one ruling for the file (and the
crate) on how a kernel-bug-only state is answered, applied to both
sites and written once. Measured, not acted on; difficulty S.

## A third site, and why it stays parked here (PROPS, props/recourse-grammar, 2026-10-01)

`props::curved::mixed_levels` is a third answer to the same
kernel-bug-only class, and it is the one that reaches a USER: it returns
a typed refusal, `PropsError::NotIsoRectangle { what: name }` — the same
variant and the same `what` (`props_rim_level`) that a valid notched
cylinder wall carries (`work/flux/a-notched-cylinder-wall-has-no-volume-measurement`).
So the state its own doc calls "kernel-bug-only (one surface builds every
rim of a face AND both its ends)" renders as a capability gap.

PROPS' recourse-grammar unit split that variant — `PropsError::OffSurface`
now carries the premises no valid body violates, chosen at the raising
site by the premise the residual checks — and deliberately did NOT route
`mixed_levels` into it, on two grounds:

1. The split's rule is the premise a RESIDUAL checks, and `mixed_levels`
   checks none: its own doc says it is "not routed through the funnel —
   the pair has no comparand, so there is nothing to decide and no
   verdict to spend". The rule does not reach it.
2. Both answers this row may pick — `unreachable!` or a poison return —
   remove the refusal entirely, so giving it a refusal VARIANT now is
   exactly the pre-emption `mixed_levels`' own doc warns against.

So the site is this row's, with the extra fact that it is the only member
of the class whose answer is currently user-visible prose.
