# LIB-DOORS spec — curated-surface gaps F1-F6 (binding)

Mandate: close the U9S findings (the report's F1-F6,
~/.local/share/cad-work/lib-u9s-report.md) so the bracket.py
journey completes and the demos-via-python north star (LIB-LOG)
is reachable. Post-switch: persist is now the v4 schema — F1's
doors expose THAT, no legacy surface. Deviations numbered and
REPORTED.

## 0. Discipline (absolute)

≤~150 lines per tool call; chunked reads; skeleton-first.
Slot rules: `scripts/with-build-slot.sh -- cargo ...`;
`--express SECS` ≤10-min rows; long rows default mutex, BLOCKING
foreground waits (timeout 590000, re-issue; setsid+poll past the
cap); NEVER park. Cold clippy both lanes + greps BEFORE opening.
Commit AND push per chunk. NO Co-Authored-By, no model names.
Merge origin/main before opening; confirm checks STARTED.

## 1. Fence

In scope: `crates/pncad` (curated re-exports/doors),
`crates/editor-core` (ADDITIVE accessors only — the NodeResult
accessor F3 names; no eval semantics changes), `crates/pncad-py`
(binding the new doors + the bracket.py completion), .pyi stubs.
OUT: schema/persist internals (v4 ships as-is — you re-export
its doors), kernel crates, CI, renders.

## 2. Deliverables (each an F-item)

1. **F1**: curate the persist surface — save/load/Loaded/
   PersistError (the v4 forms) through pncad::document; bind to
   Python (round-trip test: build → save → load → evaluate →
   same D9-pinned volume).
2. **F2**: a document-layer export door — the curated surface
   gains a step/stl export path accepting an EVALUATED body
   (measured call on shape: a pncad function taking the
   Evaluation+node vs a method on the bound body handle;
   report). bracket.py completes its §L3 journey (build →
   measure → export STEP) and asserts the file exists +
   parses (step-import as the oracle).
3. **F3**: the NodeResult accessor (editor-core, additive):
   failed/poisoned nodes reachable as typed data from an
   Evaluation; pncad-py's EvaluationError gains the real
   NodeError payload (the waiting tags::node_error_tag consumes
   it); the gap-demonstrating test flips to asserting the
   payload.
4. **F4**: re-export the usable-but-unnameable set (Applied,
   EditRecord, NodeValue, EvalOutcome) or state per-type why not
   (measured; the #234 residue class).
5. **F5**: re-export Expr::literal's error type; drop the
   pre-check workaround in pncad-py errors.rs.
6. **F6**: MEASURED disposition on Display for EditError/
   NodeErrorKind — if a Display impl is the right fix it is
   editor-core additive and in-fence; if the tag approach
   suffices for Python, say so and close F6 as no-change with
   the reasoning.

## 3. Acceptance

- bracket.py runs the full journey (venv, per U9S tooling);
  pytest suite green incl. the new round-trip + export tests.
- Hosted CI green WITHOUT Python (the U9S gating holds).
- Stubs updated, name-for-name check green; lint drift-check
  still green.
- Batteries on touched crates; cold clippy both lanes; zero new
  [[test]] binaries.

## 4. PR discipline

One PR. Report ≤150 lines to
`~/.local/share/cad-work/lib-doors-report.md`, per-phase
figures. Open, do NOT merge. Final message: PR number + report
path only. Forks: report, smallest faithful reading, flag.
