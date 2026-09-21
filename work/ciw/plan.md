# CIW — the plan

hosted CI, workflows and scripts

Re-scoped 2026-09-20 by CIW's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**31 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P4 | `a-source-attribute-can-silence-a-ci-deny-unread` | E | a source-level allow can return a denying CI row to what it was, and nothing reads for it |
| P4 | `an-unmergeable-pr-is-silently-ungated-not-visibly-red` | E | A PR whose merge ref cannot be computed gets ZERO check runs, which reads as green unless you count jobs |
| P4 | `billed-minute-arguments-survive-across-ci-yml` | E | ci.yml still argues live configuration decisions in billed minutes, including one that decides the shard matrix |
| P4 | `critical-path-citations-name-a-job-that-is-not-the-pole` | E | Five sites name the wrong last job on the critical path, and the run's shape has changed under all of them |
| P4 | `dirty-pr-gets-no-actions-run` | E | A PR that goes mergeable_state dirty against a moved main gets NO Actions run on its next push — an absence, not a red — and the lane cannot tell it from a queue |
| P4 | `eps-klint-and-shard-counts-are-prose` | E | the eps rows, k-lint unifications and test shards are counted in prose in eight places |
| P4 | `fmt-cache-carries-the-toolkit-codegen` | E | The app-feature test row makes fmt's rust-cache carry toolkit codegen on every run, including runs that skip it |
| P4 | `gate-ok-summarised-a-run-with-a-k-lint-row-still-in-progress` | E | gate ok ran while k-lint (gate, release-default) was still in_progress although needs: names k-lint — the roll-up reported red on a run whose every job concluded green |
| P4 | `gate-reads-roster-has-two-copies-in-ciw-files` | E | Two CIW files state what the budget gate reads: the sweep script's copy is stale, and ci.yml's says the roster lives nowhere else in the file that carries it |
| P4 | `guard-size-was-never-argued` | E | the apt guard is 1770 lines against an item that estimated a few, and only its siting was argued |
| P4 | `inherited-red-is-not-attributed-to-its-merge` | E | A red inherited from main is not attributed to the merge that caused it - the diagnosis is re-derived by every lane that trips over it |
| P4 | `interval-cfg-gate-names-the-wrong-cause-for-an-attribute-order` | E | check-interval-cfg-additive's tests-half message names block-gating for what is really an attribute order |
| P4 | `interval-only-selection-premise-restored` | E | The interval-only selection's original premise holds again; hosted keeps the whole suite |
| P4 | `loud-skip-marker-text-is-unchecked-against-its-own-file` | E | Nothing checks a loud-skip marker's text against its own file; three markers shipped saying their file's rows are the gated ones when most were not |
| P4 | `loud-skip-row-did-not-stop-a-lane-verifying-the-wrong-build` | E | The loud-skip row makes the app-feature gap visible to a CI log and not to a lane, and a lane verified an entirely app-gated diff without it |
| P4 | `nightly-rows-cannot-be-dispatched-by-a-lane` | E | no agent here can workflow_dispatch, so a nightly-only row lands unverified |
| P4 | `no-local-script-builds-all-four-cargo-workspaces` | E | The repo has four Cargo workspaces plus tools/tess-meter and no local script builds them all, so a signature change sweeps crates/ and reaches hosted CI red from demos/ |
| P4 | `perf-history-writers-are-guarded-three-different-ways` | E | the four docs/perf-data histories are written by three kinds of emitter with three different guard sitings |
| P4 | `population-layer-duplicated-across-two-checkers` | E | two checkers carry the same population reader nine functions deep, and scripts/ already imports siblings by path |
| P4 | `prose-digits-are-records-nothing-reconciles` | E | ci.yml's env block argues against itself about restating a pin in prose |
| P4 | `python-suite-axis-skips-only-two-members` | E | the python-suite axis now skips only viewer and test-utils — does the exception still earn its machinery |
| P4 | `reach-cannot-follow-every-ascent` | E | The read reach's chain resolver does not follow every ascent, and the fail-closed sweep is not total |
| P4 | `step-import-freecad-job-is-named-for-the-wrong-door` | E | The 'step import (freecad)' job is named for the import door and gates on the export fixtures |
| P4 | `third-party-fetches-on-the-critical-path-are-unretried` | E | seven hosted downloads from a third-party host carry no retry, beside three callers that do |
| P4 | `three-shell-splitters-nothing-compares` | E | three hand-written shell command splitters in scripts/, with independent break sets and nothing holding them against each other |
| P4 | `wasm-only-doc-comments-are-checked-by-nothing` | E | no browser rustdoc pass runs, so a doc comment on a wasm-only item gets neither a doc build nor a lint |
| None | `apt-preamble-bypass-is-unguarded` | None | nothing stops a new workflow step spelling its own apt preamble again |
| None | `apt-update-fails-on-the-runner-image-google-chrome-repo` | None | apt-get update fails repo-wide on the runner image's google-chrome list, and it reds four steps in two workflows |
| None | `ci-draw-can-hide-a-compile-break-on-main` | None | The ci.yml filter draw can hide a hard compile break on main for an unbounded number of merges |
| None | `ci-draw-rows-tree-rs-citation-does-not-locate-the-unleverable-arm` | None | The closed ci-draw row's tree.rs citation names a different arm of the same match |
| None | `closure-reaches-tree-wide-guards` | None | The change closure reaches tree-wide guards, derived from what a suite reads |
| None | `closure-tier-scope-hides-whole-tree-census-tests` | None | A closure-tier run seeded outside geom-core never executes geom-core's whole-tree census tests, so a door added elsewhere can land with the census red |
| None | `closure-tier-skips-python-suite-on-geom-core-changes` | None | TIER=closure on a geom-core/geom public-signature change runs RUN_PNCAD_PY=false — the python wheel is never built although it compiles against those crates |
| None | `committed-conflict-markers-reach-main` | None | Committed conflict markers keep reaching main — three instances in two days; CI owes a tree-wide marker/delimiter guard |
| None | `configuration-sampling-outlives-its-premise` | None | The lane/eps draw was bought with billed minutes: price un-sampling, and say what the red record can and cannot attribute to it |
| None | `criterion-selftest-nightly-only` | None | criterion-emit.py --selftest is invoked only from nightly.yml - a guard exercised only on a schedule |
| None | `cut-regex-unanchored-admits-a-line-the-lint-refuses` | None | The cut script's CUT_RE is anchored only at the start, so it admits a stamp line tess-lint refuses |
| None | `cut-script-header-claims-no-cross-language-gate-exists` | None | The cut script's header says its format is pinned by nothing; tools/tess-lint now pins it in two clauses |
| None | `debug-only-gate-step-name-understates-its-subjects` | None | ci.yml's 'bit-identity debug-only guard (topo/source.rs)' step title names one subject of a gate that now scans a list |
| None | `delete-config-trailer` | None | Delete the CI-Config commit-trailer configuration path |
| None | `detached-demo-workspaces-are-gated-only-by-a-sampled-row` | None | demos/tour and demos/wild are detached workspaces, so the clippy a lane runs before pushing cannot see them and CI is the first thing that tells them |
| None | `doc-gate-two-unread-axes` | None | The two axes the rustdoc gate still cannot read after pass 3: an in-half broken link, and the not(debug_assertions) profile axis |
| None | `f3-recosting-on-a-public-repo` | None | F3 and the nightly demotions rest on an Actions allowance this repo no longer has: re-cost them on a public repo |
| None | `facade-guards-defer-to-rustdoc-json` | None | Three facade guards defer to a rustdoc-JSON check that is not scheduled |
| None | `geom-brep-test-unused-edgedescription-import` | None | geom-brep test binary carries an unused EdgeDescription import visible only under --all-features |
| None | `gui-log-citations-do-not-resolve` | None | Twelve live citations of docs/GUI-LOG.md, and the ledger's recovery recipe does not resolve any of them |
| None | `gui-wasm-build-is-not-gated-at-all` | None | the GUI's wasm32 build is gated by nothing: ci.yml's wasm row excludes viewer, and default features exclude the app feature where the wasm code lives |
| None | `hosted-renderer-announces-itself-preview-only` | None | hosted-render-guard — the canonical renderer announces itself as PREVIEW ONLY, do NOT commit what this pass draws |
| None | `keep-out-names-a-track-that-owns-nothing` | None | CIW's keep_out gives tools/* to code-quality Track K; INSTR has owned it since 2026-09-08 |
| None | `klint-memory-false-after-unsampling` | None | memories/agent-lane-operations.md says the k-lint row is sampled; PR 1850 makes that false |
| None | `klint-row-still-sampled` | None | The k-lint unification row is still drawn 1-in-5 after the lane/eps un-sampling |
| None | `local-half-restates-ci-pins-as-literals` | None | the local half restates ci.yml's tool pins as literals in five places and nothing reconciles them |
| None | `main-latently-red-at-tier-all` | None | Main is latently red at TIER=all: the pncad-py wheel does not compile (pyo3 create_exception) and doc-gate rejects the workspace pass |
| None | `merge-order-semantic-break-reaches-main` | None | Two green PRs merged 22 minutes apart left main non-compiling: no run ever gates the union |
| None | `merge-queue-trial` | None | Merge queue trial: designed and costed, then found unavailable — GitHub offers merge queues only to organization-owned repositories |
| None | `mirror-pairs-context-beyond-env` | None | a mirrored pair's working directory and action inputs are still compared by nothing |
| None | `mirror-pairs-env-divergence-unchecked` | None | no check compares the env a mirrored CI pair runs under, so a deliberate divergence and a dropped variable look the same |
| None | `mirror-parity-never-compares-flags` | None | check-ci-mirror-parity compares which checks each half names, never their flags, so the two halves can drift on what a red run reports |
| None | `nightly-demotions-have-never-run` | None | A row demoted to the nightly is not verified at the demotion - the three from 2026-09-03 first ran two nights later, unwatched |
| None | `nightly-pin-reading-idiom-four-copies` | None | nightly.yml reads ci.yml's tool pins with a sed idiom that is now in four places and breaks silently on a second match |
| None | `no-ci-run-on-a-conflicting-pr` | None | A push to a PR that conflicts with main gets no CI run at all — GitHub creates no refs/pull/N/merge, so a silent 'no run' after a push means a conflict, not a stalled queue |
| None | `opt-level-selftest-runs-nowhere` | None | opt-level-calibrate.py --selftest is invoked by nothing in the tree - a guard that has never been shown to fire |
| None | `perf-history-cannot-identify-its-host` | None | perf histories cannot identify the box that produced a sample - the environment block records nproc/mem/toolchain and nothing that distinguishes two ubuntu-latest hosts |
| None | `pinned-version-named-in-present-tense-prose` | None | prose across both halves asserts what is pinned NOW by restating the value, and goes false on a bump |
| None | `pipestatus-after-assignment-in-ci-yml` | None | a status capture that cannot fail: PIPESTATUS read after the assignment that clobbers it |
| None | `probe-interval-lane-has-no-clippy-row` | None | the probe+interval feature combination has no clippy row anywhere, and four unused imports have accumulated in it |
| None | `python-suite-zero-test-guard-three-copies` | None | The python suite's zero-test guard exists in three places because no shared runner does |
| None | `reinstate-full-configuration-runs` | None | Reinstate full configuration runs in place of the lane/eps sampling draw |
| None | `render-hosted-knows-four-lanes-and-there-are-six` | None | render-hosted.sh knows four lanes and the repo has six |
| None | `render-lanes-red-at-missing-merge-ref` | None | render lanes: ~100 hosted reds are couldn't find remote ref refs/pull/N/merge at checkout |
| None | `retire-render-automatic-matplotlib-fallback` | None | render.sh — retire the automatic matplotlib fallback; a crashed scene must fail loudly, not become a green preview |
| None | `ruff-pin-read-shares-the-first-match-shape` | None | check-python-lint.py reads ci.yml's ruff pin with the same first-match-at-any-indentation shape ci-pin.py replaced |
| None | `rustdoc-d-warnings-breakages-outside-the-doc-gate` | None | Pre-existing rustdoc -D warnings breakages the doc gate does not render |
| None | `rustdoc-gate-disagrees-with-workspace-doc` | None | Workspace `cargo doc -D warnings` and the hosted rustdoc gate disagree about topo: a broken intra-doc link fails the workspace pass |
| None | `sccache-trial-verdict-to-read` | None | sccache on trial - check in a few days whether it actually helped |
| None | `seal-oracle-toolchain-read-first-match` | None | seal-oracle.sh reads the toolchain with a first-match sed against Cargo.toml, not the toolchain file |
| None | `session-start-hook-restates-ci-pins` | None | the agent-container hook restates three ci.yml pins as literals, out of every gate's reach |
| None | `tree-wide-guards-outside-the-change-closure` | None | Tree-wide guards are unreachable from the change closure - two main breaks in one night, and Ev's ruling: the closure reaches them |
| None | `view-made-the-skip-mode-viewer-doc-pass-lint-inert` | None | NOTICE, not a request: VIEW changed scripts/doc-gate.sh's skip-mode viewer pass to RUSTDOC_LINTS_INERT on Ev's ruling — read and close |
| None | `wasm-row-warning-debt-comment-names-a-closed-item-and-a-deleted-symbol` | None | ci.yml's wasm-row warning-debt comment describes a state that has been resolved, and names a symbol that no longer exists |

## Order

By what a reader of a red run needs, then by cost. `dirty-pr-gets-no-actions-run`
and `an-unmergeable-pr-is-silently-ungated-not-visibly-red` are one
family — a PR that is ungated rather than red — and
`inherited-red-is-not-attributed-to-its-merge` is the third.

Everything else is class `E` and is Ev's low band by name: the payoff
is main being red less often or costing less. Take them as drive-bys
where you are already in `ci.yml` (`work/README.md`, "The tracker is
not comprehensive") rather than dispatching fifty PRs.

## Review posture

OPEN, for this program's first dispatch. CIW inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
