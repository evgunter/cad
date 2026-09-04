# DOCM-4 — an evaluation carries its document's identity; A4's refusal sentence narrows to the seam (spec)

**Program:** DOCM (`work/docm/plan.md`), unit `DOCM-4`
(`work/docm/DOCM-4.md`). **Ratified design:**
`docs/DOCM-IDENTITY-DESIGN.md` DI2 and DI3 — read them first; this spec
binds the build and does not re-open them.
**Track:** kernel change — the standard v6 unit (binding spec, drawn
implementer arm, cross-model dual review, union fix pass,
record-at-merge; §Review below).
**Pre-draw fields, logged before the draw:** difficulty **M**, task-class
**STRUCTURAL**.

- **M** — one field on one struct, stamped at one site, checked at four
  doors, plus two documents' sentences and the pinned tests that state
  the old contract. Bounded, but the doors are the assembly gate and the
  product gather, where a wrong refusal is loud.
- **STRUCTURAL** — identity comparison only; no numeric decision.

## What the unit builds

**1. `Evaluation<T>` gains `document: DocumentId`** (`eval/mod.rs:55`),
stamped from `doc.id()` at the three sites that construct an
`Evaluation` (`eval/mod.rs:1654`, `:1849`, `:1949` — verify by grep; a
fourth constructor anywhere is a finding). The field is the document
identity only (A4: "which part"); no pin, no resolver identity — DI3
says why, and the spec does not reopen it.

**2. The memo lookup checks it** (`eval/mod.rs:1955`, the
`prior.and_then(|p| p.nodes.get(&id))` site): a `prior` whose `document`
differs from `doc.id()` is refused before any node is looked up. Refusal
shape: `evaluate` does not return `Result`, so the honest door is
`EvalOptions`/`evaluate`'s contract — choose ONE of: (a) `evaluate`
treats a foreign prior as no prior and records the fact on the
`Evaluation` as a typed field the caller can read (`prior_ignored:
Option<PriorIgnored { expected, found }>`), or (b) `evaluate` grows a
typed refusal arm in `EvalOutcome`. Prefer (b) if `EvalOutcome` already
carries refusal-shaped arms; prefer (a) if it is purely
completed/cancelled. State the choice and its reason in the PR; either
is in spec. Silent acceptance is out of spec.

**3. Every door that takes a (document, evaluation) pair refuses a
mismatch typed:**
- `product::product_recorded` / `product` / `product_named`
  (`product.rs:322`, `:406`, `:433`) — `ProductError::EvaluationOfAnotherDocument
  { expected, found }`, raised before the first root is read.
- `assembly::assemble` (`assembly.rs:506`) — inherits it through
  `AssemblyError::Product`; verify no second path reaches the gate
  without the product.
- `mate::solve::SolvedPoses::placement` (`mate/solve.rs:102`) and any
  other `SolvedPoses` door that takes a `Doc` — a typed refusal in that
  module's own error vocabulary, or a documented obligation if the
  door has no `Result` to carry it (say which, and why, at the site).
- `checks::run_checks` (`checks.rs:544`) — through the product gather's
  refusal; do not add a second check there.
Sweep: grep every `pub fn` in `crates/editor-core/src` taking both a
`&Doc<_>` and an `&Evaluation<_>`; the hit list with dispositions goes
in the PR (discipline doc §5).

**4. A4's sentence narrows** (`crates/editor-core/ASSEMBLY.md`, clause A4):
"Edits to a referenced document never retarget a reference: the resolver
returns a document only when its bytes hash to the pin, else
`ResolveFault::PinMismatch`" stays; add the DI2 sentence in the present
tense — an evaluation that crosses the seam refuses a moved pin; an
evaluation served from a prior serves what the document pins, and store
freshness is the mounting session's. The `pncad-py` wording at
`pncad.pyi:2827` and `py/value.rs:1474` already says the qualified form
(from LIB-G18a); check it against the new A4 text and change nothing
there unless it now contradicts it (a contradiction is a one-line
finding for LIB, not an edit in this unit).

**5. The pinned tests move with the contract.** LIB-G18a pinned the
prior-serves-old-body behaviour as contract tests
(`crates/pncad-py/tests/test_assembly_author.py` and the Rust twins it
adopted — find them by the `prior` keyword). Under DI2 that behaviour is
BY DESIGN, so those rows stay green and their doc-strings say so; do not
weaken or delete them. Add the new rows below.

## Acceptance

- **A1 — the stamp.** Every `Evaluation` produced by `evaluate`,
  `evaluate_nested` and the bounds probe carries the evaluating
  document's id; a row per constructor site.
- **A2 — a foreign prior is refused, not mined.** Evaluate document A,
  then evaluate document B with A's evaluation as `prior`: the outcome is
  the typed refusal or the typed `prior_ignored` record chosen in item 2,
  `reused == 0`, and B's nodes all recompute. A second row where A and B
  share node ids and content keys by construction (same recipe, different
  ids) shows no coincidental hit is served.
- **A3 — the doors refuse.** `product_recorded`, `assemble` and
  `SolvedPoses::placement` each refuse a mismatched pair typed, naming
  both ids; the same pair matched succeeds unchanged (bit-identical
  product on the tour's die and on one assembly-shaped corpus document).
- **A4 — the memo still works.** The viewer's `evalseam::run_once`
  priming path and `probe_bounds`' prior path are untouched and their
  existing rows stay green; `reused` on a same-document re-evaluation is
  unchanged from the merge base on the tour's bench corpus (state the
  number before and after).
- **A5 — the sentences.** `ASSEMBLY.md` A4 carries the DI2 sentence; the
  LIB-G18a contract tests' doc-strings name DI2 as the reason the
  behaviour is by design.

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted CI
  is the verification of record; poll it in the foreground; never end a
  turn with background work active.
- **Blinding: NO `Co-Authored-By` or `Claude-Session` trailer in lane
  commits** (the A/B experiment's rule overrides the harness convention;
  if one lands in a pushed commit, note it in the PR body and carry on —
  never rewrite history).
- Merge-only: no rebase, no force-push, no squash. Push early and often.
- Private `CARGO_TARGET_DIR` outside the worktree. Read `git status`
  before every `git add`; never `git add -A`.
- The `Evaluation` wire: evaluations are NOT persisted (`persist/mod.rs`
  "What persists"), so no format change; confirm and say so.
- Do not touch `resolve/vdiff.rs`, `crates/profile/*`, the analysis lane
  (`analysis.rs`, `distribution.rs`, `drive.rs`, `measure.rs`), the
  `product.rs` Dual arms, or `crates/pncad-py` source — `.pyi`/binding
  consequences are LIB's, one line each in the PR body.
- Another lane (DOCM-3) is concurrently editing `eval/wire.rs`,
  `node.rs`, `edit.rs`, `names/`, `persist/`, `refactor.rs` and the
  viewer's `combine.rs`/`tree.rs`. Stay out of those files; merge `main`
  into your branch (never rebase) if it moves under you.

## Out of scope

The session's `Reevaluate` re-mount (DI2's viewer half, CHROME); the
resolver door's Python signature (LIB); anything about pins on the
evaluation (DI3 rules it out).

## Review

v6 dual on the frozen head, claims to falsify (the reviewers get these
verbatim plus `docs/prompts/reviewer-style-lane.md` by path):

- **C1** No `Evaluation` can be constructed without a document id (the
  reviewer greps for a fourth constructor and for `Default`/struct-update
  paths).
- **C2** A foreign prior yields zero reuse under A2's second row, the one
  built to collide on node ids and content keys.
- **C3** Every (document, evaluation) door refuses a mismatch and none
  refuses a match; the sweep's hit list is complete (the reviewer re-runs
  the grep).
- **C4** The LIB-G18a contract tests were neither weakened nor deleted,
  and `reused` on the bench corpus is unchanged from the merge base.
- **C5** `ASSEMBLY.md` A4 states the present contract in the present
  tense with no history narration, and the `pncad-py` wording does not
  contradict it.
