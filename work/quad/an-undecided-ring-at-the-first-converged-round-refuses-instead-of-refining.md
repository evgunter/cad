---
id: an-undecided-ring-at-the-first-converged-round-refuses-instead-of-refining
kind: issue
title: An undecided ring at the first converged round refuses instead of refining
status: closed
opened: 2026-10-10
priority: P1
cost: M
closed: 2026-10-22
pr: 5580
---


## What

`geom_brep::props::quad::cylinder_cut_face_rounds` decides each ring's
winding against its face (`require_holes_wound_against`, the closed
forms' `2·A/P` meter at the interval scalar) at whatever round it
returns. At the FIRST round that converges it returns, so a ring whose
mean-width enclosure still straddles the band there escalates
(`props_ring_winding`), and no further round is tried, although the
face's flux has converged and a later round would decide the ring.

Witness (the PR 4557 verifier, `verify_probe_thin_rings_round0_vs_whole`
on `review/4557-verify-XczE1i7v`, `crates/geom-brep/src/props/quad.rs`):
on the chart fixture `probe_face`, a 2 × 1 face with a circular hole of
radius 1e-4 down to 2e-6 refuses over the whole schedule, while windows
starting at rounds 1 to 3 converge and decide the ring. The head before
PR 4557's fix pass (60597c6d, winding at convergence only) refuses the
same cases, so this predates that pass. It is the same mechanism as
`quadrature-convergence-test-escalates-instead-of-refining`: an in-band
decision ends the schedule where a refinement would answer.

## The shape of a fix

When the face has converged and a ring's winding is still undecided,
refine on while rounds remain, as the escalation row asks for the
convergence test itself, and escalate only at the schedule's end.

## Closed

PR 5580. The row's cited witness did not reproduce on main: its hole sat at `level_anchor`'s level, where the ring's enclosure is tight. The defect was live off that level. A ring that straddled the band at the face's first converged round escalated, though the face had converged and a later round decides the ring.

Now:
- `quad::Run` (`Schedule::run(window)`) holds a lane's last round and its window. `Run::exit` is the one round-exit ladder for all five lanes (it was `round_exit`). `Run::ringed` decides rings at convergence, in the cylinder and spiric lanes alike.
- A ring whose reading straddles the band refines while the window has rounds. A ring wholly in the band escalates at once: `refinable` reads the escalation's own margin kind and `terminal_sliver`.

Reviewed in a single FULL review and a focused re-review of the fix pass (no MAJOR). Tests:
- the cylinder rows discriminate hole from island at every ε, and assert round-0 convergence;
- the spiric witness locates its own straddling window;
- each reverted lane, and `refinable` forced either way, turns a row red.

Residue, each with its own row:
- `a-ring-undecided-at-a-windows-last-round-refuses-a-sign-walk` (P2; the reviewer could not reach it through the public doors);
- `early-round-decisions-past-convergence-the-extent-gate-and-the-meter-lever` (P3).
