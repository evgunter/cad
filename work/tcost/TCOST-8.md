---
id: TCOST-8
kind: unit
title: geom-brep test helpers: band x47, edge/great x16, p3/v3/pt x38
status: review
opened: 2026-09-03
refs: [TCOST-7]
branch: tcost/8-geom-brep-helpers-2
pr: 1659
---

Deferred by TCOST-7 (PR 1635): the helper families it left as-is in
`crates/geom-brep/tests/` — `band` (47 spellings), `edge`/`great`
(16), `p3`/`v3`/`pt` (38). Same shared-module shape; one home per
concept, deliberately independent spellings stated at the copy.
