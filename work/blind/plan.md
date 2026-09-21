# BLIND — the plan

CI instruments that cannot see what they name: the denials, the rosters and the runs that read as something else

Opened 2026-09-20 by CIW's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**12 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `doc-gate-cannot-see-a-broken-link-inside-a-cfg-test-module` | E | doc-gate.sh cannot see a broken intra-doc link inside a #[cfg(test)] module |
| P3 | `gate-ok-has-no-expected-job-roster` | E | check-run-jobs.py holds no expected-job roster: a job absent from the API reads as green on the one required check |
| P3 | `gating-nextest-jobs-discard-every-passing-tests-stdout` | E | The twelve gating nextest jobs discard every passing test's stdout, so both of the tree's announce-only idioms reach no reader on the gate |
| P3 | `green-row-floor-has-no-watcher` | E | DESIGN.md's green-row floor (engineering convention 3) has no watcher in the repo that asserts it |
| P3 | `k-probe-sweep-says-no-test-compares-probe-against-f64` | E | k_probe_sweep.sh says no test in this tree compares Probe against f64; one now does |
| P3 | `kernel-wasm-row-denies-no-warnings` | E | the kernel/editor-core wasm32 row denies no warnings, and is green under a deny today |
| P3 | `klint-gate-has-no-interval-row-so-interval-clippy-is-ungated` | E | k-lint (gate)'s five rows are all default-lane, so a clippy warning on the interval lane is ungated by construction |
| P3 | `no-ci-row-runs-the-suite-at-a-non-default-k` | E | No CI row runs the suite at a non-default K — the eps axis is gated at three values and the K axis at one |
| P3 | `red-run-whose-jobs-never-started-reads-as-a-broken-tree` | E | A run whose jobs were never acquired is indistinguishable from a broken tree, and the tell is undocumented |
| P3 | `semantic-env-is-fail-open-where-pin-free-is-fail-closed` | E | SEMANTIC_ENV admits an unlisted variable silently where PIN_FREE refuses an undeclared literal |
| P3 | `tier-blind-rationale-has-five-prose-spellings` | E | the tier-blind siting rule is argued in five places and declared in one |
| P3 | `tool-versions-outside-the-env-block-have-no-source-of-truth` | E | FreeCAD, the 3.12 interpreter and the demo render venv are pinned as literals nothing reconciles |

## Order

`gate-ok-has-no-expected-job-roster` and
`red-run-whose-jobs-never-started-reads-as-a-broken-tree` FIRST, and
together. On 2026-09-20 a billing lapse stopped jobs being acquired on
PR #2939 and the three unguarded jobs read as failures with no logs —
the exact confusion both rows describe, met in the wild the day they
were scored. A roster that says which jobs were EXPECTED turns that
from a diagnosis into a line of output.

Then the gates that pass while blind:
`a-source-attribute-can-silence-a-ci-deny-unread`,
`doc-gate-cannot-see-a-broken-link-inside-a-cfg-test-module`,
`ci-local-topo-release-guard-cannot-pass`.

## Review posture

OPEN, for this program's first dispatch. CIW inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
