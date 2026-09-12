---
id: coherence-findings-have-no-consumer
kind: issue
title: coherence findings have no consumer: wire examine_chart_coherence into editor-core's checks (CheckId::ChartCoherence) and step-import
status: review
opened: 2026-09-02
github: 1587
refs: [1585, 868, 723, 1571]
branch: fix/chart-coherence-check
pr: 2408
---

## From GitHub issue 1587

Opened 2026-09-02; 0 comments.

**Filed from MESH-8 (PR [#1585](https://github.com/evgunter/cad/pull/1585); issue 868's relocation) as the schedule for a disclosed forward observation.**

MESH-8 landed `topo::coherence::examine_chart_coherence(body, tol) -> CoherenceReport` — the body-side, non-gating home of the three input-quality conditions that used to be `debug_assert!`s in `mesh::walk` (loop closure; rim and meridian continuation), each a gap against a lever arm in metres against ε. On merge day the door has **zero production callers**: the mesh no longer asserts, and nobody reads the report.

Two consumers were named and not wired, by fence:
1. **editor-core checks**: a `CheckId::ChartCoherence` arm, `CheckKind::Certified` (the gap and lever are closed forms; the finding is a measurement), one `CheckFinding` per resident's rest body, ordered by `ChecksReport`'s per-resident rule. `CheckId` is a closed enum whose every arm owes a severity-configuration row, a `Display` arm, a determinism statement and Track V surface — a consumer decision, not part of relocating a condition.
2. **step-import diagnostics**: the door where defective source coordinates actually arrive (issue 723's half-cap is the recorded π-rad witness through import); step-import already depends on topo, so no new edge.

Note the shape difference MESH-8's review flagged: `CoherenceReport { findings, unexamined }` looks like `ChecksReport { findings, skipped }` but `skipped` is configuration (a check whose severity is Off) and `unexamined` is data (a loop the door could not read); the adopter must not fold them.

Refs #868, #723, #1571, MESH-8.

## Home

`work/issues/` — S-MESH names 1587 a cross-program follow-on on other programs' ground, and both named consumers (editor-core's `CheckId` lane and step-import diagnostics) fall in no open program's territory globs.

## Measured: the `CheckId` cost is far smaller than this row states (FIX orchestrator, 2026-09-11)

The row calls a new `CheckId` arm *"a consumer decision, not part of
relocating a condition"*, on the grounds that *"every arm owes a
severity-configuration row, a `Display` arm, a determinism statement
and Track V surface"*. That sentence has kept this row undispatched.
It overstates the cost. Counted on this head:

- **`CheckId` has two arms**, `Connectedness` and `Separation`
  (`crates/editor-core/src/checks.rs:52`).
- **Four non-test match arms over it, in the whole tree**
  (`grep -rn 'CheckId::[A-Za-z]* *=>' --include=*.rs crates/`, minus
  tests) — the enum's own `kind()`, `reads_subject()`, `ALL`, and
  `crates/pncad-py/src/py/checks.rs`.
- The enum is **designed** for this: its doc says *"A new check = a
  new variant; every match over this enum is a site the compiler then
  walks you to"*, and `ALL`'s doc names the row that reds when a
  variant is missing from it
  (`dsc_checks::the_registry_order_is_every_check`).

So adding an arm is a compiler-guided walk of four sites, not an
open-ended consumer design. **The row is dispatchable**, and what it
actually costs is the wiring, not the variant.

**What is genuinely hard here is the one thing the row states
precisely, and a taker should treat it as the unit's centre:**
`CoherenceReport { findings, unexamined }` and
`ChecksReport { findings, skipped }` have the same shape and different
meanings — `skipped` is **configuration** (a check whose severity is
Off) and `unexamined` is **data** (a loop the door could not read).
Folding them would report "we chose not to look" and "we could not
look" as one thing, which is the not-examined-masquerading-as-examined
shape this program has now found twice
(`validate-drops-the-material-sign-refusal-silently` is the other).
The two must reach the user distinguishably; say how in the PR.

**Scope:** the `CheckId` half only. The step-import diagnostics half is
**EXCH's** (`work/fix/plan.md` says so). `crates/editor-core/src/checks.rs`
is FIX's own glob, so the primary fence is ours; `crates/pncad-py/` is
LIB's and `crates/topo/src/coherence.rs` is read-only here — name both.
