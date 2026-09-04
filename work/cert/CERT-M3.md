---
id: CERT-M3
kind: unit
title: Track M's lane-trait lane — H5's three-trait census as CERT-M2 landed it
status: dispatched
opened: 2026-09-01
refs: [1559]
---

Track M lane after CERT-M2: the H5 row (C7 + S33, the lane-trait collapse)
worked from the three-trait census CERT-M2 wrote into H5 — `EdgeNurbsLane`
splits free (already a closure parameter), `PcurveFittedLane` does not split
(check 4 is the certificate's envelope), `ChartRegionLane` splits only with a
completeness contract for Ev — with the structural half's bound inheriting
`Decide + Bounds + PcurveFittedLane` plus a home for `recertify_approx`. Its
`Dual`-rewriting sub-lane is ADV per the plan's CERT-M entry.

Spec `docs/CERT-M3-SPEC.md` lives on `cert/orchestrator` (not on main at
migration). Dispatched from CERT-M2's merge (PR 1559), block CERT-B4 slot 2
(OPUS) — `work/cert/log.md`, "CERT-M2 … MERGED" (2026-09-02); first named in
the CERT-M1 entry's slate line.
