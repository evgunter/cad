---
id: a-clearance-assertion-is-decided-by-the-sweep
kind: unit
title: A clearance is an assertion bounded below, above or both, decided by one fused E7 sweep; min_clearance and the bracket path retire
status: closed
opened: 2026-11-01
priority: P1
cost: H
branch: doors/clearance-assertion
closed: 2026-11-02
---


The kernel half of `the-gui-has-no-clearance-consumer`, whose `## The design` section is the spec. The design was ratified by Ev on PR 6261 and is written into ERROR-DESIGN E3 and E7 and DESIGN `[assertions]`. The fork record is row 231 of `docs/DESIGN-FORK-LOG.md`.

**What this unit builds:**
- an E10 assertion form whose subject is `clearance(a, b)`, bounded below, above, or both (`= c`, or a band);
- a sweep whose bound is an enclosure, deciding **both bounds in one fused subdivision**;
- the verdicts: `Holds`, `Violated` with a violating pair, an upper bound's proving pair, and a typed third state;
- the retirements: `min_clearance` as a measure, `MeasureUnavailableAt` and its payloads, `editor_core::clearance`'s second spelling folded into `drive::assertion_at`, and `min_separation` out of evaluation;
- Python's constructor;
- a `pncad` door that answers per assertion at nominal in plain numbers, which the viewer's certify seam consumes.

Claimed by DOORS because it blocks DOORS' P1 consumer and CLEARTAIL has no orchestrator. Its ground is CLEAR's and CLEARTAIL's (`crates/editor-core/src/clearance.rs`, `crates/topo/src/clearance.rs`), so both logs carry a seam note. (DOORS orchestrator)

## Closed

PR 6292. A clearance is an E10 assertion over `clearance(a, b)` — `≥ c`,
`≤ c`, `= c` or a band — decided by one fused E7 sweep at its bounds; a
lower bound's witness is a violating pair, an upper bound's a proving
pair whose separation is enclosed over the whole box, and a typed
refusal is the third state (`BoxDependent` cues the drive). The
retired `min_clearance` measure loads as a named refusal. Residue, each
its own file: DOORS
`a-meeting-scope-over-a-box-refuses-where-topology-decides-zero`;
CLEARTAIL `an-upper-bound-at-the-boxs-true-maximum-refuses-with-a-false-message`,
`a-lower-bounds-box-dependent-refusal-gives-the-drive-no-cue`, and the
rows the PR's own fix-pass report lists.
