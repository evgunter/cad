# LIB-PLACEDUNION spec — the ratified A′ group boolean: a Pattern that fuses (binding)

Mandate: implement the RATIFIED PlacedUnion design
(docs/GROUP-BOOLEAN-DESIGN.md — option A′, Evan's 👍 on the #496
thread; LIB holds the implementation home per the #510 offer).
Read the design doc IN FULL first; it is the contract. This spec
adds sequencing, fences, and acceptance only.

## 0. Discipline (absolute)

docs/LIB-PYG1-SPEC.md §0 verbatim and binding
(local-scripts/with-build-slot.sh, foreground one-at-a-time,
kill-your-own-waiter, commit+push per chunk, NO Co-Authored-By,
no model names, merge-main-before-open + re-merge, checks
STARTED, cold clippy CI scope incl. `-p pncad-py --features
python`, k-lint discipline, invariant comments). SHARD rows
>~15 min; prefer express. CONCURRENCY FENCE: RESPELL PR-2 runs
in parallel (corpus/demos/Python re-spell + the profile
test-support shim) — your territory is editor-core
node/eval/persist + the graft door + topo as the design names;
the AUDIT PAGE belongs to PR-2 this wave (your Python/audit
slice is a FOLLOW-UP unit — do not touch the audit page or
pncad-py beyond mechanical tag arms). If a collision appears,
STOP and report.

## 1. Deliverables (the design doc's shape, staged)

1. **The node**: PlacedUnion per A′ — one prototype input,
   PatternKind rule (incl. Explicit placements), ONE body out,
   Instance{i} naming unchanged from the pattern precedent.
2. **Certified disjointness** via the prototype BVH exactly as
   the design specifies (typed refusal on overlap — the #382
   adjacency noted there).
3. **Graft-door lowering** per the design's lowering section
   (reuse graft_disjoint_all — no new kernel machinery).
4. **Schema clean break** (take main's next SCHEMA_VERSION at
   your FINAL re-merge — the deterministic rule; multiple
   claims are in flight: #549 v9-shifted, M9-1 PR-2 v10 —
   verify the constant at the last merge and take next).
5. **Persist + eval content keys** append-only (the tag-29
   lesson: verify no collision, next-free numbers).
6. **The register payoffs proven in editor-core tests** (NOT
   the audit page): heatsink's fins as PlacedUnion(count-rule)
   with the memoized recompute pinned; diecomposed's 20-union
   tool as PlacedUnion(Explicit) — corpus-doc twins asserting
   the same oracles, byte-level where the design promises it.
7. Numbered findings; the Python/audit slice recorded as the
   named follow-up.

## 2. Acceptance

cargo test -p editor-core -p topo -p pncad green; schema rows
both directions; cold clippy CI scope; hosted CI green; zero
new [[test]] binaries. Pre-draw fields at dispatch: difficulty
L, task-class STRUCTURAL (the disjointness certification reuses
the existing BVH predicates — no new numeric decision; if
implementation surfaces one, note it in the report).

## 3. PR discipline

One PR. Report ≤150 lines to
~/.local/share/cad-work/lib-placedunion-report.md, per-phase
figures. Open, do NOT merge. Final message: PR number + report
path + ≤10-line summary. Forks: report, smallest faithful
reading, flag.
