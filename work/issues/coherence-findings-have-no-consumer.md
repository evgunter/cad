---
id: coherence-findings-have-no-consumer
kind: issue
title: "coherence findings have no consumer: wire examine_chart_coherence into editor-core's checks (CheckId::ChartCoherence) and step-import"
status: open
opened: 2026-09-02
github: 1587
refs: [1585, 868, 723, 1571]
---

## From GitHub issue 1587

opened 2026-09-02, 0 comments.

**Filed from MESH-8 (PR [#1585](https://github.com/evgunter/cad/pull/1585); issue 868's relocation) as the schedule for a disclosed forward observation.**

MESH-8 landed `topo::coherence::examine_chart_coherence(body, tol) -> CoherenceReport` — the body-side, non-gating home of the three input-quality conditions that used to be `debug_assert!`s in `mesh::walk` (loop closure; rim and meridian continuation), each a gap against a lever arm in metres against ε. On merge day the door has **zero production callers**: the mesh no longer asserts, and nobody reads the report.

Two consumers were named and not wired, by fence:
1. **editor-core checks**: a `CheckId::ChartCoherence` arm, `CheckKind::Certified` (the gap and lever are closed forms; the finding is a measurement), one `CheckFinding` per resident's rest body, ordered by `ChecksReport`'s per-resident rule. `CheckId` is a closed enum whose every arm owes a severity-configuration row, a `Display` arm, a determinism statement and Track V surface — a consumer decision, not part of relocating a condition.
2. **step-import diagnostics**: the door where defective source coordinates actually arrive (issue 723's half-cap is the recorded π-rad witness through import); step-import already depends on topo, so no new edge.

Note the shape difference MESH-8's review flagged: `CoherenceReport { findings, unexamined }` looks like `ChecksReport { findings, skipped }` but `skipped` is configuration (a check whose severity is Off) and `unexamined` is data (a loop the door could not read); the adopter must not fold them.

Refs #868, #723, #1571, MESH-8.

## Home

`work/issues/` — S-MESH names 1587 a cross-program follow-on on other programs' ground, and both named consumers (editor-core's `CheckId` lane and step-import diagnostics) fall in no open program's territory globs.
