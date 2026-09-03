---
id: TCOST-K3
kind: unit
title: validate_geometric recomputes the enclosure its caller holds: one certificate per body
status: spec
opened: 2026-09-03
refs: [TCOST-K1, TCOST-5]
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
