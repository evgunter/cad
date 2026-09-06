---
id: TCOST-4
kind: unit
title: the topo boolean torus-oracle probe, and the cross-profile dump rows
status: closed
pr: 1608
branch: tcost/4-topo-boolean-probes
opened: 2026-09-03
closed: 2026-09-03
---

Content unit over `topo`'s in-`src` `boolean::` probe rows: the torus-oracle
row (23 cpu-s, rank 2 in the suite, never red) splits into a labelled
regime enumeration (65 rays) and a seeded generic-pose search on the fuzz
dial (a TCOST-1 gate candidate on `solid_contain.rs`); the two
assertion-free cross-profile dump rows retire with their owners named; the
NotFound dump class is swept. Hosted run 33699222447 green on the head.
Test-only: batched style review, no A/B row.

Not yet in `work/tcost/log.md` (the log holds only the opening entry at
migration); PR 1608 is the record.
