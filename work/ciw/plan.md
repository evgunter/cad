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

**11 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P2 | `the-viewer-app-feature-rows-gate-one-eps-of-three` | E | the viewer's app-feature rows run at the default eps only, per-PR and nightly |
| P3 | `oracle-filter-blind-to-the-kernels-edge-onto-the-certified-crate` | E | should the oracle filter fire when the kernel's edge onto the certified crate changes |
| P4 | `dirty-pr-gets-no-actions-run` | E | a PR with no computable merge ref gets no Actions run: an absence, not a red |
| P4 | `inherited-red-is-not-attributed-to-its-merge` | E | a red inherited from main is not attributed to the merge that caused it |
| P4 | `a-source-attribute-can-silence-a-ci-deny-unread` | E | a source-level allow can return a denying CI row to what it was, and nothing reads for it |
| P4 | `ci-prose-and-pins-outlive-the-latency-cut` | E | doc-gate.sh, render.yml and test-fast.sh still argue in billed minutes and archive jobs; an unused sccache pin |
| P4 | `gate-reads-roster-has-two-copies-in-ciw-files` | E | the sweep script's statement of what the budget gate reads is stale |
| P4 | `no-local-script-builds-all-four-cargo-workspaces` | E | no local script builds every cargo root |
| P4 | `step-import-freecad-job-is-named-for-the-wrong-door` | E | the `step import (freecad)` job gates on the export fixtures |
| P4 | `third-party-fetches-on-the-critical-path-are-unretried` | E | two downloads carry no retry, and three pipe a retried curl into tar |
| P4 | `wasm-only-doc-comments-are-checked-by-nothing` | E | no browser rustdoc pass runs, so a wasm-only doc comment gets neither a doc build nor a lint |

## Order

The viewer eps row first: it is the one row here about a red CI cannot
see. Then `dirty-pr-gets-no-actions-run` and
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
