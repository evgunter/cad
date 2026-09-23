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
