# CIW — the plan

hosted CI, workflows and scripts

## The shape of CI this plan works against

`ci.yml` is the per-PR gate, sized for latency: the default-eps suite
minus the slow set (`.config/nextest.toml`'s `ci` profile), lint, the
1e-6/1e-12 rows of the eps-sensitive crates the diff seeds, and the
direct-path rows whose subject the diff touches. `nightly.yml` holds
the rest: every eps row over the whole suite with the slow set, the
k-lint rows, the render lanes, rustdoc and wasm32. A `main` push runs
only the cache primer. `work/ciw/latency-cut.md` is the evidence for
that selection; a row that would put work back on every PR argues
against it rather than around it.

## The slate

**22 budget points** of dispatchable work against a ceiling of 30.

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

## Review posture

OPEN, for this program's first dispatch. CIW inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
