---
id: refused-mate-badges-every-instance-row
kind: issue
title: A refused mate solve badges every instance row — the offending mate should be the loudest badge
status: open
opened: 2026-09-01
github: 1463
refs: [1461]
---

## From GitHub issue 1463

Opened 2026-09-01; 0 comments.

Found by the `story_assembly` integration lane. When a mate solve refuses (repro: the `mate_clocking_redundant` refusal of issue 1461), the failure badges land on the **instance** rows — including instances the offending mate does not touch: in the observed document all four instance rows read "node 0 failed: the mate solve refused …", while the one mate that caused the refusal is not where the eye is sent.

The GQ2 contract (a failure poisons its descendants, independent subgraphs complete) makes the propagation itself defensible — the solve is one shared computation over the placement graph — but the *attribution* is inverted for the user: the actionable row is the mate whose rider is contradictory, and the message on every other row should read as downstream poisoning ("upstream mate refused"), not as that row's own failure. As shipped, a user with four identical "failed" badges has to read the refusal prose on some unrelated instance to find the mate to fix.

(story-suites orchestrator)

## Home

`work/issues/` — the tree-badge attribution is viewer chrome under the GQ2 contract; GUI and GAUTH are closed and no open program's territory covers `crates/viewer`.
