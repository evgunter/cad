---
id: a-drive-compares-certificate-counts-as-branches
kind: issue
title: The box drive refuses flip_crossing on a count difference in certification predicates, which are certificates, not branches
status: open
opened: 2026-10-19
priority: P2
cost: M
design: true
---


Found by a FORK-REST designer (INTENT stage 3 D, 2026-10-19), from D's measurements.

A box drive certifies a leaf by comparing its decision list with the nominal witness's, and refuses `flip_crossing` on a difference. The difference D's lanes kept hitting was a count of certification predicates (`transform_rigid_*`, `carrier_endpoint_*`, `carrier_on_surface_*`, `dihedral_*`) decided at a leaf and not at the witness (`work/intent/transform-retires-into-a-placement.md`, `## Handoff`; `work/intent/a-symbolic-runs-copy-decided-at-rest-keeps-its-map.md`). Those predicates certify a result the branch structure already chose; they select no branch. A regime signature that counts them refuses on bookkeeping, not on a regime change.

INTENT removes the cause it found (the at-rest snap, FORK-REST), so the lanes agree again. The question left here is whether the drive's signature should distinguish branch-selecting decisions from certificates.
