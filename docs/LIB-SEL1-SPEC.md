# LIB-SEL1 spec — geometric selectors PR-1 (binding)

Mandate: docs/SELECT-DESIGN.md §§1-2 (RATIFIED #286) — the
predicate vocabulary's v1 set and the `select_where` algebra
extension. PR-2 (detect/declare/menu, §3) is a LATER dispatch, not
yours. The design doc is BINDING design; this spec adds only
operational discipline. Where the doc under-specifies, that is a
finding-back, never a silent fix.

## 0. Discipline (absolute)

≤~150 lines per tool call; chunked reads; skeleton-first writes;
report ≤150 lines. Slot rules: `local-scripts/with-build-slot.sh --
cargo ...`; `--express SECS` for ≤10-min rows; long rows default
mutex, BLOCKING foreground waits (timeout 590000, re-issue on
timeout; setsid + foreground-poll past the harness cap); NEVER
park. Cold clippy (cargo clean -p touched crates) default AND
--features interval + discipline greps BEFORE opening. Commit AND
push per chunk. NO Co-Authored-By, no model names. Merge
origin/main before opening; re-merge if main moves (SWITCH-E is
in flight in editor-core — coordinate ONLY by merging main, never
by touching its surfaces; if its PR-B merges mid-unit, re-merge
and re-run your battery); confirm checks STARTED.

## 1. Fence

In scope: `crates/editor-core` (names/select extension +
the new predicate module), `crates/pncad` (select door widening),
`demos/tour` (the migration below), new funnel predicate
registrations per the doc. OUT: §3's detectors/declare sugar
(PR-2), `Node`/schema/persist, contact machinery, CI,
crates/profile.

## 2. Deliverables (the doc's §§1-2, operationalized)

1. **Exact kind predicates** (§1's exact half): carrier-kind and
   adjacent-surface-kind-pair patterns as TAG READS (post-#256
   always-promote makes them exact — no funnel, no margins);
   composed into the selector vocabulary per §2.
2. **The decided position predicate** (§1's decided half):
   datum-relative position (GS-Q6) through the `decide` funnel
   with `sel_*` site naming; K-census participation per GS-Q1;
   in-band → typed refusal (never a silent drop from the result
   set). Convexity: the enum slot reserved, NOT built (GS-Q2).
3. **`select_where`**: the new materializer combining the
   structural Selector with a GeomPred conjunction filter, per
   §2 — same contract (evaluate → resolve → store
   Vec<StableName>, sorted-deduped, no serde on any query type);
   mixed-Tied refuses typed (GS-Q4).
4. **Migration**: diefillet.rs's two geometric filters
   (:203-234) move to select_where + name-fed fillet_edges —
   THE acceptance (P10 dies at its origin site); byte-identity
   of exports (names in, same geometry out). bossplate/curvedcut
   position-based finds migrate where datum-relative position
   expresses them; report per-site where it does not.
5. Doctests per pncad convention; the sel_* predicates' in-band
   rows (the G1 NOTE-2 lesson — escalation paths tested).

## 3. Acceptance

- Byte-identity: tour exports at 3 ε rows vs your own base build.
- The migrated diefillet selections resolve to the SAME edge sets
  (pin: name-set equality against the old key-set route, bridged
  via the U7 inversion doors).
- Full batteries on touched crates; zero new [[test]] binaries;
  the K-census gains sel_* rows without flagging (run the k-lint
  gate locally; a fired lint = re-derive per runbook, never
  geometry tweaks).

## 4. PR discipline

One PR. Report ≤150 lines to
`~/.local/share/cad-work/lib-sel1-report.md`, per-phase figures.
Open, do NOT merge. Final message: PR number + report path only.
