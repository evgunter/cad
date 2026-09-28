---
id: CONTACT-5
kind: unit
title: the census backstop clears a meeting pair only through the touch analysis: the box gate stops clearing partial overlaps, and a coplanar edge cross is a touch
status: closed
opened: 2026-09-26
priority: P0
cost: H
branch: contact/5-gate-and-beam
closed: 2026-09-28
---


Carries `partial-overlap-with-touch-only-boundaries-clears-at-the-census-gate` and `a-beam-across-two-supports-edges-refuses-on-coplanar-edge-crosses`. Spec: `docs/CONTACT-5-SPEC.md`.

Review tier: **dual**. The feared failure is a wrong clear on the backstop.

## Closed

Arm 2 of `sweep_cross_solid_backstop` clears a pair whose boundaries
meet only through the touch analysis.
- **The gate** is a fast path for a pair with nothing on record
  between it, and it reads each OUTER shell's hull: a void is dropped,
  and a shell whose role does not read is kept.
- **A pair that meets** has every vertex probed in both orderings, and
  `blocks` reads every finding about it. Any finding that is not a
  rest refuses with its reason, and that reason replaces the probe's
  when the probe could not decide.
- **A coplanar edge cross** is a `TouchSite`. A non-coplanar one stays
  a crossing.
- **The argument** is stated once, at arm 2's loop, with its three
  premises and what checks each.

Measured, head against base:
- the brick-grid sweep: 422 wrong clears → 0;
- the reviewers' 480 rotated prisms: 244 → 0;
- the reviewers' 135 crossed ridges: 90 → 0.
Every sweep has 0 false refusals of true rests.

Review:
- A **dual** on `3af4ebc` (DR row in `docs/DUAL-REVIEW-LOG.md`). Both
  reviewers returned APPROVE-WITH-FIXES with the same two MAJORs, both
  pre-existing and both executed: a multi-shell solid cleared wrongly
  past the whole-solid gate, and the filed declared-only row was a
  demonstrated wrong clear.
- The fix pass (`dd95fcb`) gated per shell and probed every meeting
  pair. A single delta review found that it refused ratified hollow
  and declared seats.
- The last pass (`c0b9838`) gates outer shells only and reads a
  declared-only pair's records on their word. That relaxation is
  disclosed in the PR body; the orchestrator read it.

Left open or filed:
- `declared-only-meetings-clear-at-the-census-gate-unread` (P0), the
  residue;
- `a-same-solid-self-overlap-seen-only-as-touches-is-not-a-self-crossing` (P3).
