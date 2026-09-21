# MIRROR — the plan

the checkers and local scripts beside CI: the parity reader, the render lanes, the calibrators and their pins

Opened 2026-09-20 by CIW's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**24 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P4 | `cache-rendered-cells-on-input-hash` | E | Cache rendered cells on their input hash, to close the window where main's committed renders are stale |
| P4 | `calibrator-cpuinfo-parser-selftest-cannot-see-a-broken-parse` | E | opt-level-calibrate.py's cpuinfo selftest asserts a subset an empty list satisfies and never drives the parse |
| P4 | `calibrator-record-writes-without-its-selftest` | E | nightly.yml runs opt-level-calibrate.py record with no selftest before it, on refs the merge gate never saw |
| P4 | `ci-local-topo-release-guard-cannot-pass` | E | ci-local.sh's topo_release guard greps for a row that cannot exist locally: the local gate row is structurally red |
| P4 | `ciw-rows-and-ci-local-prose-rotted-by-the-c1-c3-restore` | E | The C1-C3 restore left three CIW premises stale: a guard count, a nightly tabulation, and ci-local's dispatch comment |
| P4 | `criterion-lane-asymmetry-argued-in-four-prose-homes` | E | the criterion lane's hosted-only argument now lives in four prose copies and no machine-read one |
| P4 | `criterion-selftest-fixture-is-one-scalar-in-five-fields` | E | criterion-emit.py's selftest passes 11 of 20 single-token mutations - its fixture writes one scalar into five collected fields |
| P4 | `doc-gate-error-sites-outside-the-gate-population` | E | doc-gate.sh carries 14 gate_error sites outside the gate population D109 reads |
| P4 | `k-probe-eps-const-read-first-match` | E | rundump-guard-selftest.sh reads k_probe_sweep.sh's PLAIN_EPS with a first-match grep |
| P4 | `mirror-job-is-the-whole-wall-for-tracker-prs` | E | CI half parity is ~90% of the wall on docs/tracker PRs: the 61s step is fixed, the remaining 121s is not measured |
| P4 | `mirror-parity-checker-growth` | E | check-ci-mirror-parity.py has grown 64% in three days and nothing measures it |
| P4 | `mirror-readers-blind-through-bash-c` | E | claim 10's flag and env arms both read nothing inside a bash -c command string |
| P4 | `mirror-step-keys-still-discarded` | E | shell: and a step-level continue-on-error: still decide a mirrored pair''s semantics and are read as text |
| P4 | `mirror-three-copy-reader-preamble` | E | claim 10's three readers open with the same five-step preamble, written three times |
| P4 | `opt-level-sample-reader-written-twice` | E | nightly.yml reads the written opt-level sample path twice, with the explanation only at the first |
| P4 | `provenance-lane-dirs-is-not-the-lane-roster` | E | check_render_provenance.py's LANE_DIRS calls itself the lanes and is the PNG trees |
| P4 | `python-lint-row-is-locally-unverifiable-on-this-image` | E | the container ships ruff 0.15.8 against a 0.16.1 pin, so the python-lint row skips for every lane |
| P4 | `renderer-free-cross-crate-links-are-ungated-off-the-seed-set` | E | a cross-crate link from viewer's renderer-free half is gated only while its target crate is a toolkit seed, and the nightly re-take that is supposed to backstop it cannot red |
| P4 | `rustdoc-gate-private-intra-doc-links` | E | Revisit — should the rustdoc gate reinstate private_intra_doc_links? |
| P4 | `rustfmt-does-not-reach-a-macro-wrapped-declaration-block` | E | rustfmt does not format a macro invocation's body, so a macro-wrapped declaration block has no formatting gate |
| P4 | `seal-oracle-refuses-toml-spellings-the-msrv-gate-blesses` | E | seal-oracle.sh refuses rust-toolchain.toml spellings the MSRV gate certifies as valid |
| P4 | `session-start-hook-is-exercised-by-nothing` | E | no gate, test or script in the tree runs a line of .claude/hooks/session-start.sh |
| P4 | `shellcheck-is-not-run` | E | no shell linter runs anywhere: 20+ `# shellcheck` markers are unverifiable claims |
| P4 | `two-anchored-pin-readers-two-homes` | E | the tree now has two anchored pin readers, in two languages, and only one can go red |
| P4 | `verify-lane-set-is-a-property-nothing-declares` | E | render-hosted.sh's --verify lane set is a property render.yml states only in prose |

## Order

`mirror-parity-checker-growth` first, with the three reader rows
(`mirror-readers-blind-through-bash-c`, `mirror-three-copy-reader-preamble`,
`mirror-step-keys-still-discarded`) specified as part of it rather than
after it: they are what the growth is made of, and fixing the readers
is what makes the checker smaller.

It is also ~90% of the wall on a docs or tracker PR, which is the only
cost argument on this slate that reaches a person waiting.

## Review posture

OPEN, for this program's first dispatch. CIW inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
