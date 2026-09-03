---
id: CERT-N3
kind: unit
title: Track N's surviving rows — S235, D31, D98, D244, C24
status: dispatched
opened: 2026-09-01
refs: [1558]
---

Track N's last lane: the rows left after CERT-N1 (the scalar-lift lane) and
CERT-N2 (the H2 lane) — S235, D31, D98, D244 and C24's remainder — worked
per the track's own table in `docs/SMELL-SCAN-2026-08.md` §Track N, rows
deleted in the landing PR per §D's conventions. After it Track N is empty in
§D, which is the plan's exit shape for the track.

Spec `docs/CERT-N3-SPEC.md` lives on `cert/orchestrator` (not on main at
migration). Dispatched from CERT-N2's merge (PR 1558), block CERT-B4 slot 3
(FABLE) — `work/cert/log.md`, "CERT-N2 … MERGED" (2026-09-02); first named in
the CERT-N1 entry's slate line.
