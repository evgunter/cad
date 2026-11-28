---
id: a-boolean-result-carries-a-seam-meridian-certificate-a-fresh-run-does-not-reproduce
kind: issue
title: a boolean result's seam meridian carries a certificate a fresh re-certification of its own geometry does not reproduce (the snowman lens, 1.48e-16 carried against 2.22e-16 fresh)
status: open
opened: 2026-10-02
priority: P3
cost: E
---


Found by the measurement in
`the-union-fallback-graft-re-certifies-a-disjoint-operand-through-the-lane-free-door`.

## Measured

`intersect(ball(1.0, 0.0), ball(0.8, 1.4))` (`sweep`'s snowman lens,
`Tol::witness()`, `f64`). Re-certify every certified edge of the
result against its own endpoints and surfaces
(`EdgeCurve::recertify_via` with `NurbsLane::certified()`, at
`Band::linear(Tol::witness())`) and compare the certificate it
carries:
- one edge differs: the B ball's seam meridian (carrier
  `Circle { center: (0, 1.4, 0), axis: −z, radius: 0.8000000000000002 }`,
  params `(2.366399280279432, π)`, ends `(0.5598833697790122,
  0.8285714285714285, 0)` and `(0, 0.6, 0)`) carries
  `max_residual: 1.4806895100970427e-16`; a fresh run mints
  `2.220446049250313e-16`;
- in the other operand order, `intersect(ball(0.8, 1.4), ball(1.0, 0.0))`,
  two edges differ, again the B ball's seam meridians (radius 1.0),
  `1.5700924586837752e-16` carried against `2.482534153247273e-16`
  fresh, ending at `(±0.5598833697790122, 0.8285714285714283, …)`.

Both balls alone re-certify bit for bit. Every other edge of the
lenses does too.

## Cause: not established

Consistent with the seam zip keeping A's copy of a junction vertex
that B's meridian was certified against: the junction's y differs in
the last bits between the two orders (`…285` against `…283`), and the
stale edge is always B's. Not traced to the line.

## Why it matters

A certificate is documented as the attachment-time run over the
edge's geometry, and both carrying doors (`boolean::voids` and, since
the same PR, the containment fallback's assembly) say a carried
certificate is the one a fresh run would mint. That holds for every
source whose certificates match its geometry. The lens's do not, at
the ulp, so a graft carries its stale residual forward. The
difference sits far inside the band, and the at-rest gate re-derives
carriers without reading a stored certificate. `sweep`'s
`snowman::the_fallback_assembly_carries_the_kept_operands_certificates`
runs the lens as it stands (its `slab ∪ lens` row asserts the carried
certificate, not a fresh one).

## What is owed

Find the step that moves a certified edge's endpoint without
re-certifying it, and re-certify there (or show the move cannot
happen and the difference has another cause).

No pin may rely on this staleness to tell the bridges apart: which
bridge a graft ran is pinned off the graft's own record
(`topo::test_support::take_graft_bridges`), so the pins still tell
the bridges apart once this row is fixed.

## Slate triage (2026-10-26, CLEAVE)

- Moved to CLEAVETAIL (`work/cleavetail/`) by `git mv`, id and body unchanged: the track opened for CLEAVE's P3 and P4 tail: refusal stories, guards that cannot go red, latent soundness and hygiene. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## A lead from LATHE (2026-11-23, PR 7414)

On the one-band branch, a full revolve's bare ball stored
`max_residual: 1.5700924586837752e-16` on its seam meridian, where a fresh
run minted `1.2246467991473532e-16`. The cause there was that the meridian
was restated with no chart image, so the door derived the image and
certified over the derived one, and a run over a derived image differs
in the last bits from a run over the same image stated. The fix
(`sweep/src/revolve/upgrade.rs`, `upgrade_meridian_wrap`) restates it once
more from what it stores
(`work/lathe/a-one-band-balls-seam-certificate-is-not-the-fresh-runs.md`).
The carried value this row measured in one operand order is the same
number, `1.5700924586837752e-16`. But this row was measured on two-band
balls that "alone re-certify bit for bit", so the derived-image cause
does not obviously explain it. Not checked further: re-measure this row
once PR 7414 lands, and check whether the boolean's own restatement of the
B ball's meridian derives its image.
