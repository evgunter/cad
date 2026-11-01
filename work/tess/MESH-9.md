---
id: MESH-9
kind: unit
title: issue 950 — rim chords versus the rim-adjacent band, built on first demand
status: closed
blocked_on: [950]
opened: 2026-08-31
priority: P1
cost: H
closed: 2026-10-21
pr: 5551
---

Parked with a typed trigger, in the issue's own words: neither fix is needed
until a body presents the configuration, and the failure is a
`CertificateExceeded` refusal that names it loudly. The two candidate fixes
(rim chords snap up to a multiple of `patch_nuc`; the rim-adjacent band
raised to the rim's chord count) stay recorded on issue 950. Unparks when a
body reaches the refusal.

`work/mesh/log.md`: "MESH-9 stays parked on its trigger" (the MESH-8,
MESH-10 and MESH-11 entries).

## Re-homed at S-MESH's exit (2026-09-16)

Moved from `work/mesh/` to TESS (opened at this exit as S-MESH's successor for the tessellation kernel) when S-MESH closed (`docs/S-MESH-EXIT-WALK.md`); the item's content, id and history are unchanged.

## Superseded (2026-10-21)

Both candidate fixes align rim chords and bands on `patch_nuc`, the column
family the malign-band snap made. #5551 retires the snap, so that family no
longer exists, and issue 950 (`rim-chords-exceed-snapped-column-count`)
closes as superseded. The refusal this unit was parked on is still the
backstop: a face that reaches it refuses typed `CertificateExceeded`. That
refusal names the face, and any fix would start from the lattice metric,
not from these two.
