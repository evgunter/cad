# CIW — the plan

hosted CI, workflows and scripts

## The shape of CI this plan works against

`ci.yml` is the per-PR gate, sized for latency: the default-eps suite
minus the slow set (`.config/nextest.toml`'s `ci` profile), lint, the
1e-6/1e-12 rows of the eps-sensitive crates the diff seeds and of the
test files it changes, and the
direct-path rows whose subject the diff touches. `nightly.yml` holds
the rest: every eps row over the whole suite with the slow set, the
k-lint rows, the render lanes, rustdoc and wasm32. A `main` push runs
only the cache primer. `work/ciw/latency-cut.md` is the evidence for
that selection; a row that would put work back on every PR argues
against it rather than around it.

## The slate

The budget counts only the dispatchable P0, P1 and unbanded rows
(`work/README.md`, Track size); with UNREAD's half merged in (below)
that is **15 points** against 30: five unbanded rows, three of them this
half's (`a-declared-module-name-marks-unrelated-tests-new`,
`a-work-edit-can-red-pncad-py-unseen`, `demote-unearned-tests`). The
first sitting prices and bands them. The table is the workflow half's
banded slate as last written; the row files are the live state.

| pri | item | cost | title |
|---|---|---|---|
| P2 | `the-viewer-app-feature-rows-gate-one-eps-of-three` | E | the viewer's app-feature rows run at the default eps only, per-PR and nightly |
| P3 | `oracle-filter-blind-to-the-kernels-edge-onto-the-certified-crate` | E | should the oracle filter fire when the kernel's edge onto the certified crate changes |
| P3 | `red-run-whose-jobs-never-started-reads-as-a-broken-tree` | E | a run whose jobs were never acquired reads as a broken tree |
| P3 | `gating-nextest-jobs-discard-every-passing-tests-stdout` | E | the gating nextest runs discard passing tests' stdout, so announce-only idioms reach no reader |
| P3 | `doc-gate-cannot-see-a-broken-link-inside-a-cfg-test-module` | E | doc-gate.sh cannot see a broken intra-doc link inside a `#[cfg(test)]` module |
| P3 | `kernel-wasm-row-denies-no-warnings` | E | the nightly kernel/editor-core wasm32 check denies no warnings |
| P3 | `no-ci-row-runs-the-suite-at-a-non-default-k` | E | no CI row runs the suite at a non-default K |
| P3 | `tool-versions-outside-the-env-block-have-no-source-of-truth` | E | FreeCAD, the 3.12 interpreter and the render venv are pinned as literals nothing reconciles |
| P3 | `green-row-floor-has-no-watcher` | E | DESIGN.md's green-row floor counts a matrix that is gone (design; Ev's) |
| P4 | `dirty-pr-gets-no-actions-run` | E | a PR with no computable merge ref gets no Actions run: an absence, not a red |
| P4 | `inherited-red-is-not-attributed-to-its-merge` | E | a red inherited from main is not attributed to the merge that caused it |
| P4 | `a-source-attribute-can-silence-a-ci-deny-unread` | E | a source-level allow can return a denying CI row to what it was, and nothing reads for it |
| P4 | `ci-prose-and-pins-outlive-the-latency-cut` | E | doc-gate.sh, render.yml and test-fast.sh still argue in billed minutes and archive jobs; an unused sccache pin |
| P4 | `gate-reads-roster-has-two-copies-in-ciw-files` | E | the sweep script's statement of what the budget gate reads is stale |
| P4 | `no-local-script-builds-all-four-cargo-workspaces` | E | no local script builds every cargo root |
| P4 | `step-import-freecad-job-is-named-for-the-wrong-door` | E | the `step import (freecad)` job gates on the export fixtures |
| P4 | `third-party-fetches-on-the-critical-path-are-unretried` | E | two downloads carry no retry, and three pipe a retried curl into tar |
| P4 | `python-lint-row-is-locally-unverifiable-on-this-image` | E | the container's ruff is behind the pin, so the python-lint row skips locally |
| P4 | `rustdoc-gate-private-intra-doc-links` | E | should the rustdoc gate reinstate `private_intra_doc_links` |
| P4 | `rustfmt-does-not-reach-a-macro-wrapped-declaration-block` | E | rustfmt does not format a macro-wrapped declaration block |
| P4 | `shellcheck-is-not-run` | E | no shell linter runs; the `# shellcheck` markers are unverifiable |
| P4 | `wasm-only-doc-comments-are-checked-by-nothing` | E | no browser rustdoc pass runs, so a wasm-only doc comment gets neither a doc build nor a lint |

## Order

The viewer eps row first: it is the one row here about a red CI cannot
see. Then the P3 rows, which are instruments that pass while blind (Ev's
medium band: tooling that prevents a SILENT bug), and
`dirty-pr-gets-no-actions-run`,
`red-run-whose-jobs-never-started-reads-as-a-broken-tree` and
`inherited-red-is-not-attributed-to-its-merge`, the family where the CI
surface tells a reader something false about the tree. The rest are
Ev's low band by name; take them as drive-bys where you are already in
the file (`work/README.md`, "The tracker is not comprehensive").

## The guards that do not exist yet (UNREAD's half)

Merged in from UNREAD on 2026-11-02
(`docs/doc-ledger/unread-merges-into-ciw.md`). GUARD's rows
(`work/guard/`) are gates that cannot see what they claim, and BLIND's
(`work/blind/`) discipline gates whose matchers miss a spelling. **These
rows have no gate at all.** Each names something the tree carries with
nothing reading it, a claim (a measured number, a census, a pin, a
table's reasons) or a reference a human wrote (a citation, a tracker
path, a ledger recovery command, a public method, a merge commit's
authored lines). Each owes one of three things: a mechanical guard, a
scheduled register that re-measures it, or a written reason it can
have neither. A new gate lands under `scripts/gates/` on GUARD's harness
(`lib.sh`), with its selftest, announced on GUARD's log. A checker that
is an arm of `scripts/work.py lint` is built on META's ground by
announced seam, announced on META's log. A new CI step is this
program's own workflow ground.

| item | pri | cost | what it is |
| --- | --- | --- | --- |
| `feature-gated-lib-unit-tests-are-compiled-and-never-run` | — | — | six `probe`-gated `#[test]`s in library `src/` that no hosted job executes |
| `ledger-notes-have-no-guard-that-their-recovery-command-resolves` | — | — | nothing runs a doc-ledger note's `git show <sha>:<path>` |
| `measurements-have-no-mechanical-guard` | P3 | E | the class row: measured numbers in comments with nothing red if they go false; `Cargo.toml`'s opt-level block first |
| `viewer-disabled-control-population-has-no-gate` | P3 | E | the 30 `add_enabled` sites' dispositions, an allowlist keyed by site |
| `raw-door-compile-proof-needs-a-gate` | P3 | D | BOOL-9's witness-crate compile-fail proof needs a gate with a toolchain |
| `comment-symbol-names-outside-rustdocs-reach-have-no-gate` | P3 | E | 26 `<mod>::<name>` spans in plain `//` comments under viewer, checked by nothing |
| `door-table-reason-strings-are-unguarded-prose` | P3 | E | `review_m1_pr5_internal`'s `ALLOWED` reasons are never read against their doors |
| `vocabulary-macro-bodies-are-outside-rustfmt` | P3 | E | `vocabulary!` bodies are formatted by hand; accept and say so, or gate |
| `doc-citations-no-gate-checks-rot-silently` | P3 | D | `file.rs:NNN` citations across `work/` and test names in doc comments, neither checked |
| `no-instrument-guards-a-public-method` | P3 | D | the façade censuses guard exported names and members, never a public method |
| `authored-content-folded-into-a-merge-commit` | P3 | D | authored lines inside merge commits, invisible to `git log -p` |
| `inserting-an-item-above-another-steals-its-docs` | P4 | E | a doc block silently rebinds to an item inserted above its own |
| `S115` | P4 | E | the prelude's import census was taken once and nothing re-takes it |
| `tracker-paths-in-crate-doc-comments-rot-on-a-program-re-cut` | P4 | E | a `work/<program>/<id>` path in a crate comment rots when the row moves |
| `perf-plan-citations-name-a-path-that-moved` | P4 | E | five citations still name `docs/PERF-PLAN.md` |

**Two pairs are one finding filed from each half**, and the first lane
to touch either member of a pair reconciles the two (one closes as a
duplicate of the other, or each is narrowed to what it alone covers):

- `feature-gated-lib-unit-tests-are-compiled-and-never-run` (UNREAD's)
  and `probe-gated-lib-unit-tests-never-run-in-ci` (the workflow half's);
- `vocabulary-macro-bodies-are-outside-rustfmt` (UNREAD's) and
  `rustfmt-does-not-reach-a-macro-wrapped-declaration-block` (the
  workflow half's).

Order within this half: the two unbanded rows first, since they are what
the budget counts. `feature-gated-lib-unit-tests-…` goes together with
its pair above (six pins go unrun today, one of them a completeness
pin), and the ledger guard's body already names the shallow-clone case
it must not red on. Then the cheap E rows whose guard is already
sketched in their bodies:
`viewer-disabled-control-population-has-no-gate` (precedent:
`viewer-vocab-declared-once.sh`),
`door-table-reason-strings-are-unguarded-prose` (the cheap arm is
written), `comment-symbol-names-…`. Then the citation family, whose
three rows are one instrument seen from three sites:
`doc-citations-no-gate-checks-rot-silently` (two arms) and
`tracker-paths-in-crate-doc-comments-rot-on-a-program-re-cut` (its
third arm, and the one every program re-cut feeds), with
`perf-plan-citations-name-a-path-that-moved` as the instance a working
checker should find and the fix that lands with it.
`comment-symbol-names-…` is a sibling of the same instrument and should
be read beside it. `measurements-have-no-mechanical-guard` is a sweep,
and its own sub-list orders it. The two surface guards each open on a
decision their bodies state: how large an alphabet the public-method
census pays for, and how strict "resolution content" is for a merge
commit. `raw-door-compile-proof-needs-a-gate` needs a CI job with a
toolchain, which is this program's workflow ground. The P4s last.

## Review posture

OPEN, for this program's first dispatch. CIW inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.

UNREAD's half came with its own posture, which still holds for its
rows: one style review per unit against
`docs/prompts/reviewer-style-lane.md`, no A/B row, plus a second
correctness reviewer for a new gate or lint arm that can refuse code on
main. A check that is wrong the day it lands is either silent or red
for the wrong reason, and a false positive there blocks every program.
