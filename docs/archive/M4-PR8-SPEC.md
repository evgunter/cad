# M4 PR 8 binding spec — Band 4 corpus, K-telemetry + large-K lint, exit sweep

Status: DRAFT until PR 6 merges. **SPLIT (Evan, 2026-07-26) into
three sequenced units**:
- **PR 8a** (implementer dispatch; gate: PR 6 + #101 merged) =
  D1 + D2 below. Difficulty L.
- **PR 8b** (implementer dispatch; gate: 8a merged — the lint
  baseline computes against the MERGED corpus) = D3 below.
  Difficulty M.
- **PR 8c** (ORCHESTRATOR work + Evan sign-off, per the
  design-conversation-PR convention; gate: 8b merged; the exit
  walk runs LAST against merged deliverables, never
  self-attesting) = D4 + D5 + D6 below. Not an A/B row.
D7/D8 apply to all three. Deviations via the REPORT mechanism.

## D1 — Band 4 model corpus

Recipe documents (authored through DocEdits, persisted, loaded)
spanning the FULL F4 vocabulary: every node type incl. Declare,
every edit type incl. Rebind/ReWitness/SetTolerance, appearance +
metadata, declared tangency (#101 flags + fillet constructor),
boolean-of-boolean chains (R13 shapes + the #105/#106 nested-island
chains), the die-as-recipe (77 nodes), corner table, heat sink.
Each corpus document: evaluates green at ε ∈ {1e-6, 1e-9, 1e-12} +
Interval, round-trips bit-identically (PR 6 rows), and pins exact
mass properties where dyadic. The corpus is a CRATE-LEVEL test
asset (editor-core tests or a corpus dir they read), not demo code.

## D2 — Rebuild-latency tracking (measured, not gated)

Per F8: wire per-document full-rebuild and incremental-recompute
timings into CI REPORTING (a printed table row per corpus doc in a
dedicated CI job; a committed baseline JSON refreshed by the PR) —
NO threshold gate (architectural instrumentation, not a contract;
PERF-PLAN stays advisory). REPORT the shape you land (job name,
artifact vs log).

## D3 — K-telemetry Probe run + the large-K lint (Evan's ask)

The M3 addendum's K-telemetry Probe runs over the Band 4 corpus AND
the demo scenes (the corpus was the missing harness). Statistic per
K-REPORT's normalization: |margin|/band_zero per predicate site.
Deliverables: (a) the distribution report regenerated into
docs/k-report-data (same format as M2's); (b) **the large-K lint**:
a CI row that FLAGS (advisory — prints, never fails, first
iteration) any demo/corpus predicate landing in a bottom percentile
of the baseline distribution or within 10^2 of the ambient band at
ANY supported ε row — the #99 case (margin 2.3ε at 1e-6) is the
motivating catch and must light up when replayed against the OLD
bracket data (test the lint by resurrecting that datum in a lint
fixture, not in the demos). Threshold constants live in the lint
tool with the baseline provenance documented — tooling-level, no
kernel ε. REPORT the percentile choice with the baseline histogram.

## D4 — DESIGN.md exit sweep (all banked ratification texts land)

1. F1–F8 outcomes ratified into DESIGN.md (each fork: decision,
   where it landed, deviations).
2. N6 retirement recorded DONE (bit_identity debug-only, empty
   allowlist; the R2 narrowing stated as the designed consequence).
3. The M3 operand-internal-declaration envelope entry RETIRED
   (closure corpus certifies declared — PR 5 D6.1); the REST-contact
   join gap and #106 residue (post-#106: whatever remains) become
   the new envelope entries, honestly scoped.
4. F5 verified-at-use semantics wording (PR 5 review F5): a false
   declaration that never meets geometry is a silent no-op;
   contradiction fires where the lie meets an edge.
5. #101 declared-tangency discipline synced into the profile
   section (R6): UndeclaredTangency/TangencyContradicted/
   FilletDoesNotFit doors, same-carrier-is-identity rule.
6. Roadmap M4 line updated to done-state; M5 line gains the
   banked openers (curved STEP subset, arc-leg fillet sugar, #89
   K-revisit at exit, PR 6 two-ε machinery notes).

## D5 — State-doc trim (standing convention)

M4-LOG gains its final CURRENT-STATE snapshot; superseded interim
snapshots marked historical (never deleted). memories updated:
cad-project-state's M4 line → complete; model-ab-experiment gains
the final table + a one-paragraph readout (n, arms, caveats — no
overclaiming at this n).

## D6 — M4 exit walk

Walk the M4-PLAN exit criteria list item by item with evidence
links (test names / PR numbers / CI rows) in the PR body. Any
criterion not met: STOP and REPORT — no waiving from the
implementation lane; the orchestrator rules.

## D7 — Out of scope

Q9 (Evan's call; #107 shortlist noted); GUI; M5 anything; #104
(concept); the A/B experiment continues past M4 (do not conclude
it here beyond D5's honest readout).

## D8 — Process (standing)

One implementer + one adversarial reviewer + one fix pass; OUTPUT
DISCIPLINE header; persistent clone, push per unit; cwd rule
(prefix every command with the clone cd); RAM discipline; every
build/battery row synchronous FOREGROUND, one at a time, no
waiters/monitors/background chains ever; fail loud; Actions gates
the merge. Model per MODEL-AB-LOG protocol v2 (block 1 remainder:
fable) with pre-assignment difficulty logging; reviewer blinded,
fixed rubric required.
