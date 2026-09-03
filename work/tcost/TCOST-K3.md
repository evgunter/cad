---
id: TCOST-K3
kind: unit
title: validate_geometric recomputes the enclosure its caller holds: one certificate per body
status: review
opened: 2026-09-03
refs: [TCOST-K1, TCOST-5]
branch: tcost/k3-unit
---

Third kernel finding for the A/B track, from TCOST-5 (PR 1621):
`topo::validate_geometric` recomputes the enclosure its caller just
computed and hands nothing back, so three rows in the rational family
— and the real import path — pay two rational certificates per body. An
API that lets the gate consume or return the mass properties removes
one certificate per body. Spec after TCOST-K1 lands.

Spec: `docs/TCOST-K3-SPEC.md` (ratified 2026-09-03; pre-draw fields
difficulty M, task-class STRUCTURAL). Correction read from the code:
the import path's redundant pair sits behind `validate_pseudomanifold`
(the tier-3′ door), not `validate_geometric`, on single-solid imports;
the lever covers both doors. Dispatches after TCOST-K1 lands (block
TCOST-KB1, next slot).

**Fix pass (2026-09-03).** The dual review's union, implemented on the
same branch. The kernel change was found sound by both reviewers; the
findings were about the unit's EVIDENCE and its COST, and both moved:

- the certifying fixture is ε-SCALED, so it certifies at every ε row
  instead of refusing at 1e-12 — where the suite had been comparing
  refusal counts and three mutants lived through it. Every row now
  asserts the arm it takes;
- the suite is GATED (`test_utils::gated_to!`) to the four source paths
  it is specific to, and its cost is flat across the ε draw;
- a second row counts certificates through `import_step`, which is the
  only witness a field documented as *not a second computation* can
  have;
- the lane dispatch behind the identity claim is pinned structurally
  (`topo`'s `quad_lane_is_the_certified_lane`), so the claim holds at
  the scalars no local row builds;
- hosted before/after at the DEFAULT ε on both lanes, the ε where the
  saving is not zero, bought with a probe PR on the merge base.

The digest's one unexplained observation is explained and was not what
it looked like: the instrument's shared append log TEARS lines under
nextest's parallel processes, so a body measured exactly once can drop
out of a `sort -u` roster. Both "surplus" bodies are physically present
in the run that was said to lack them.
