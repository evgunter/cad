---
id: verbs-1031b-assigner-checker-divergence
kind: issue
title: the winding assigner/checker divergence: merge_faces assigns arc-bounded roles validate check 6 cannot check
status: open
opened: 2026-09-03
refs: [VERBS-1031B, m6-sense-gate-recorded-residuals]
---

## The divergence

VERBS-1031B ported `boolean::join::ring_run_ccw`'s arc machinery into
`merge_faces::loop_winding`, so the `bool_ring_run_winding` predicate
now decides Line-, Circle- and Ellipse-bounded cycles at that site.
`validate.rs`'s tier-3 **check 6** — the third site of the same
predicate — still guards on `all_lines` and `continue`s past every
conic-bounded loop (`crates/topo/src/validate.rs:3549`, the guard;
`:3594`, the shared `decide` call).

The three sites no longer cover the same carrier set, and the split is
not in a harmless direction:

> **a merged face's outer/ring roles are now ASSIGNED by a functional
> the validator cannot CHECK.**

Nothing regressed — check 6 skipped those loops before this unit too,
and it skipped them back when the merge simply refused
`MergedFaceRoleAmbiguous` instead of assigning anything. What changed is
that a PRODUCER now feeds the class: before VERBS-1031B no arm minted a
merged planar face whose loops ride arcs, because the role pass refused
the whole call first. `merge_coplanar_faces` now mints them routinely
(four per teapot cup), so the uncovered class has live inhabitants for
the first time.

## The evidence: MUT-2

Measured, not argued. VERBS-1031B's mutation battery includes **MUT-2**
— `newell = newell + bulge` → `newell = newell - bulge`, the arc
correction applied backwards, a mutation strictly inside the guarded
block. Under it, on the teapot cup:

- `merge_coplanar_faces` **SUCCEEDS**. The sign flip does not make the
  roles undecidable; it makes them WRONG. Exactly one loop is still
  positively wound — it is the wrong one — so `merged_outline_ring`
  finds its unique positive cycle and swaps the ANNULUS into the outer
  slot and the OUTLINE into the ring slot.
- `topo::validate_geometric(&cup, tol)` is **`Ok(())`** on that body.
  Tier 3 is green on a body whose every merged annulus is inside out.

The inversion is caught only by the acceptance rows' own
`outer[0] > ring[1]` assertion
(`crates/sweep/tests/verbs_1031b_arcwind.rs`). That is a fixture
assertion about one shape, not a validator gate: any other producer of
an arc-bounded merged face would get no gate at all. Role inversion
passes every volume gate by construction (they are role-invariant) and
corrupts tessellation and export silently — the exact class check 6
exists to close.

## Flip condition

**Port the winding arm into `validate.rs`'s check-6 site** — widen the
`all_lines` guard at `validate.rs:3549` the way
`merge_faces::loop_winding` was widened, adding the same per-conic
bulge `axis · sa·sb · (Δ − sin Δ)` and the same arc-length re-metering,
leaving NURBS as the honest remainder. The arithmetic is already
written identically at three sites; this is a fourth statement of it,
not a new derivation.

**Its cost, which is why VERBS-1031B's fence deferred it rather than
taking it as a rider:** check 6 is a REFUSAL surface and the other two
sites are not. `join` and `merge_faces` ask the predicate a question
they need an answer to; check 6 asks it in order to FAIL a body. Its
posture today is that only a definite wrong sign refuses — Zero and
escalated windings are exempt, the check-7 posture. Widening the
carrier set therefore widens the set of bodies tier 3 can REJECT, and
does it on a margin (`2A/P` over an arc-metered perimeter) whose
in-band behaviour on real revolve output has never been measured: a
merged annulus whose mean width lands in the ambiguity band would newly
escalate where it is silently exempt today. That is a refusal-surface
change, and it deserves its own opening measurement rather than a rider
on a merge-op unit whose spec fenced it to `merge_faces.rs`.

## Schedule

A VERBS unit of its own, sized on that measurement: run the widened
check 6 over the existing revolve/shell/merge fixtures first, count what
newly refuses and what newly escalates, then decide the posture.
Sequencing is open — it is not blocking: the class has exactly one
producer (`merge_coplanar_faces`) and that producer's own output is
pinned by name in `crates/sweep/tests/verbs_1031b_arcwind.rs` in the
meantime.

Recorded against the fourth residual in
`work/props/m6-sense-gate-recorded-residuals.md:20`, which owns the flip
condition for the arc-bounded planar class.
