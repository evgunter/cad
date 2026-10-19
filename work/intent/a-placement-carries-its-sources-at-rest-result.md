---
id: a-placement-carries-its-sources-at-rest-result
kind: unit
title: A placement carries its source space's at-rest result (records and census findings) with its copies, at every placement
status: open
opened: 2026-10-19
priority: P1
cost: M
refs: [each-space-runs-its-census-between-arrivals, a-part-drops-its-census-findings-at-the-seam]
---

D10 `[placement.placement]` (PR 4666): a placement carries every pose of the source and "the source's at-rest decisions about those bodies, its contact records and its census's findings, which stay the source's".

`a-part-drops-its-census-findings-at-the-seam` builds this at the instance seam (`crates/editor-core/src/eval/parts.rs`, `PartValue`). Once each space runs its own census, every placement is such a seam. Generalise the carry from instance placements to every placement, so one type crosses every placement. With the census deciding nothing inside an arrival, the parent's re-found in-copy touches stop existing, and the solid-pair match becomes the confirm-and-match arm of `each-space-runs-its-census-between-arrivals`. `an-escalated-census-finding-names-no-pair` stays a prerequisite of matching an escalated finding.
