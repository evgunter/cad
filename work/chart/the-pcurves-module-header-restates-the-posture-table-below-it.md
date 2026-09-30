---
id: the-pcurves-module-header-restates-the-posture-table-below-it
kind: issue
title: pcurves.rs opens with 264 doc lines that restate the posture table twenty lines below them, one paragraph per unit
status: open
opened: 2026-09-14
refs: [pcurve-posture-guard-is-blind-to-body-producing-doors, 2594]
---


Filed by the fix pass of TOPO's
`set-face-surface-leaves-a-complete-face-certified-against-the-chart-it-left`
(PR 2594), on R1's style finding (Q8), on this slate because
`crates/topo/src/pcurves.rs` is TRIM's.

`pcurves.rs` opens with **264 doc lines before its first `use`**. The
accumulation is per-unit: each door class that changed its row posture
added a titled paragraph (revert; the loop-re-parenting doors; the
surface setter, in PR 2594), each individually reasonable. Below them,
`staleness_posture::DECLARED` states the same dispositions again as a
table with a note per door — which is the version a guard walks.

So one rule has two descriptions in one file, and they grow together:
PR 2594 edited both, in the same shapes, for one behaviour change. The
cost is not the length but the pairing — a future lane that edits one
and not the other leaves the file self-contradicting, and nothing
measures the agreement.

What would close it: decide which of the two is the record. The table
is the one with a mechanical reader
(`every_mutation_door_declares_its_pcurve_posture`) and the one a
per-door note belongs in; the header's job is then the MODEL (what a
row is, what a chart is, why absence is not a claim) and the pointer,
with the per-door paragraphs deleted rather than summarised. Sibling
rows on the same table: `pcurve-posture-guard-is-blind-to-body-producing-doors`.
