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

## From BLIND (merged 2026-11-20)

Opened 2026-10-26 by GUARD's split (`docs/doc-ledger/blind-guard-tint-and-vacuity-merge-into-ciw.md`, that date).
Live state is `docs/doc-ledger/blind-guard-tint-and-vacuity-merge-into-ciw.md`'s tail and the item files beside this
plan, never this file.

Branch prefix: **`blind/`** — unit branches `blind/<unit>-<slug>`.
Away-channel tag `(BLIND orchestrator)`. A/B ordinal band
**BLIND = 15600–15699**, claimed on this program for bookkeeping.

### Charter

A discipline gate is a regex or awk matcher standing in for a rule the
type system does not hold: no compound bracket bound outside the
allowlist, no interval squared by multiplying two copies, no production
bit-identity consumer, no decision on a reporting margin, no
`register_equal` outside its constructor sites, no ambient environment
read in kernel source. **Each row is a spelling of the policed thing
that the gate's matcher cannot see**, or one it refuses that is not the
policed thing. The fix is almost always the same shape: widen the
matcher or its census register, plant the escaped spelling as a
selftest case, give every new hit a disposition, and state in the gate's
KNOWN GAPS what the widened matcher still cannot match.

The shared reader and the selftest harness these gates run on
(`scripts/gates/lib.sh`) are GUARD's, not this program's. A row here that
finds the harness is the obstacle files on GUARD.

### The slate

| item | pri | cost | what it is |
| --- | --- | --- | --- |
| `reporting-margin-gate-misses-comparisons-of-two-read-readings` | P2 | E | `reporting-margin-door.sh` counts mint sites; a production `==` of two classifier-minted readings passes |
| `self-dot-has-no-gate-the-interval-square-one-cannot-see-it` | P2 | E | `interval-square-allowlist.sh` sees `x * x`, not `x.dot(x)` or `dot(a, a)` |
| `the-bit-identity-consumer-gate-cannot-see-a-to-bits-read-of-bounds` | P2 | E | `bit-identity-consumer.sh` misses `.to_bits()` on `lo()`/`hi()` |
| `G4` | P3 | M | retire `profile`'s `ArcCarrierScalar`; reds `bounds-allowlist.sh` by design |
| `D212` | P3 | D | the gate-side half of `G4` (GAP 3 text, roster entry, `arc_fillet.rs` filter); rides with `G4` |
| `S210` | P3 | D | sole `T: Bounds` / `T: Enclosure` doors have a rule and no instrument; #701 is its twin |
| `S41` | P3 | E | the poisoned-enclosure clamping idiom has three spellings and no lint |
| `census-bracket-read-gate-is-blind-to-validate-child-modules` | P3 | E | the bracket-read census counts `validate.rs` alone, not its `validate/` children |
| `register-equal-allowlist-exempts-a-whole-file-and-hides-test-calls` | P3 | E | a whole-file definition home hides thirteen test calls in `sym.rs` |
| `no-ambient-env-gate-cannot-see-temp-dir` | P3 | E | `no-ambient-env.sh` misses `env::temp_dir(`, `current_dir(`, `home_dir(`, `set_current_dir(` |
| `bounds-allowlist-fires-on-a-binding-pattern-over-an-enclosure-variant` | P4 | E | the `Enclosure` suffix matches a variant name in a match pattern |
| `tour-scalar-trait-bundles-certifiedbounds-outside-the-allowlist-scan` | P4 | E | `demos/tour`'s compound bound sits outside the gate's `crates/*/src` root |

### Order

The three P2 rows first: each is cheap, has its widened pattern written
in its body, and closes a hole through which a wrong answer can land
unseen. Then the cheap P3 widenings (`no-ambient-env`, `register-equal`,
the bracket-read census, `S41`'s clamping lint). `G4` with `D212` riding
it is one unit that crosses into `profile` by announced seam. `S210` is
the tail: its own body says the real instrument may be a rustc or
rust-analyzer driver, which is a design weighing before it is a unit.

### Review posture

Infra-only, as GUARD: one style review per unit against
`docs/prompts/reviewer-style-lane.md`, no A/B row. A unit that changes
what a gate ACCEPTS takes a second correctness reviewer, because a
widened gate that is wrong is silent — and most units here change what
a gate accepts.

## From GUARD (merged 2026-11-20)

Opened 2026-09-11 in the tracker cut of that date
(`docs/WORK-TRACKS-2026-09.md` addendum 3) on `scripts/gates/*` whole;
re-chartered on 2026-10-26, when GUARD measured 43 points against 30 and
split three ways (`docs/doc-ledger/blind-guard-tint-and-vacuity-merge-into-ciw.md`, that date). Live state is
`docs/doc-ledger/blind-guard-tint-and-vacuity-merge-into-ciw.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`guard/`** — unit branches
`guard/<unit>-<slug>`. The retired prefix `gates/` names the closed
GATES program's branches and is not reused. Away-channel tag
`(GUARD orchestrator)`. A/B ordinal band **GUARD = 3600–3699**, claimed
in `docs/MODEL-AB-LOG.md` in the opening commit.

### Charter

Every gate under `scripts/gates/` runs on `lib.sh`: one Rust reader,
one test-only skip, one selftest harness. **A defect there is a defect
in every gate at once, and it fails in the quiet direction** — a reader
that cuts a record wrongly or reads nothing, and a gate that prints OK
over it. The rows here are that harness: the reader that splits an
array type at its `;`, the `--skip-cfg-test` cut that drops a
production item sharing a test item's line, the selftest that can
assert a gate fired but not on which identity, the selftests no CI job
runs, the pipeline stages with no guard, and the readers gates copy
instead of share (the embedded Python TOML readers, the three
bound-list walkers).

The neighbours since the split:

- **BLIND** (`work/blind/`) holds the rows where one discipline gate's
  matcher misses a spelling. Its gate scripts are shared ground; what a
  matcher sees is BLIND's, the reader it runs on is GUARD's.
- **CIW** (`work/ciw/`) holds the rows with no gate at all (UNREAD's,
  merged into CIW on 2026-11-02). Its new gates land in this directory
  on this harness, announced here.

### Territory and the seams

Territory: **`scripts/gates/*` whole**. Stated seams, each in
`program.md`'s `keep_out`:

- **The Rust side of the bound-list reader** — `crates/geom-core/tests/bounds_census.rs`
  and `crates/test-utils/src/source.rs` are S-TINT's and S-TCOST's. The
  row lands its shell half here and announces the Rust half.
- **CI wiring is CIW's.** `gate-selftests-run-in-no-ci-job` adds a step
  to `.github/workflows/*` by announced seam; the embedded-Python row's
  lint half reaches `scripts/check-python-lint.py`, also CIW's.

### The slate

| item | pri | cost | what it is |
| --- | --- | --- | --- |
| `gate-rust-reader-splits-an-array-type-at-its-semicolon` | P3 | D | `gate_rust_code --statements` cuts `[T; N]` at its `;` and tracks no `<>`; `viewer-vocab-declared-once.sh` carries a workaround reader |
| `gate-selftest-cannot-observe-the-identity-a-gate-names` | P3 | D | `gate_selftest_case` matches one substring; a `want` that asserts file:line and identifier is owed |
| `gate-selftests-run-in-no-ci-job` | P3 | E | no CI job runs any gate's `--selftest`; one sat red on main for three weeks |
| `gate-roster-and-probe-census-have-no-reader-guards` | P3 | D | `probe-suite-census.sh`'s process substitutions have no stage guard (the roster half is moot) |
| `embedded-python-readers-are-copied-between-gates-and-unlinted` | P3 | D | the heredoc TOML readers are copied between gates and outside `ruff.toml`'s population |
| `bound-list-readers-have-three-homes` | P3 | D | three bracket-depth bound-list walkers in two languages, none citing the others |
| `skip-cfg-test-drops-a-production-item-sharing-a-test-items-line` | P4 | E | `--skip-cfg-test` drops a production item on a test item's closing line |

### Order

The shared reader first (`gate-rust-reader-…`), because most gates call
`gate_rust_code` and every later row reads better against a reader that
parses items correctly; `skip-cfg-test-…` is the same function and can
ride it. Then the selftest affordance, which is what makes the rest of
the slate — and BLIND's widened matchers — checkable, and
`gate-selftests-run-in-no-ci-job`, which keeps the selftests honest
after. Then the stage guards and the two shared-reader consolidations;
`bound-list-readers-…`'s full fix is a shell-vs-Rust design call, and
its cheap half (each reader cites the others) can land first.

### Review posture

Infra-only, the CIW/S-TCOST posture: one style review per unit against
`docs/prompts/reviewer-style-lane.md`, no A/B row, the band claimed for
bookkeeping. A unit that changes what a gate ACCEPTS (as against what it
reports) takes a second correctness reviewer, because a widened gate that
is wrong is silent — and a change to `lib.sh`'s reader changes what every
gate accepts, so every reader unit takes one.

## From TINT (merged 2026-11-20)

test-suite integrity

Re-scoped 2026-09-20 by TINT's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched: the two P1 slow-set reds closed with PR 6818 (2026-11-11).

### The slate

**34 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P1 | `main-red-steeper-tilts-union-stops-by-identity-reads-the-wrong-refusal-shape` | E | closed (PR 6818): the steeper tilts' union pinned a refusal's Debug text |
| P1 | `bounds-census-stale-roster-lines` | E | closed (PR 6818): bounds_census's roster lagged a new sole-bracket door |
| P3 | `test-utils-is-production-source-to-every-narrowing-gate` | M | test-utils documents itself as dev-only but is production source to every gate that narrows by test-only mounts |
| P3 | `C18` | D | Close the three residues of H12's own enumeration left open by #734, all coverage or prose |
| P3 | `D380` | E | Rename the band-keyed test rows whose names assert the arm the default epsilon does not take |
| P3 | `D381` | E | Pin RecipeEditRef::ForeignNode at both mid-evaluation doors of editor-core's shared name ladder |
| P3 | `D66` | E | Fix the NURBS re-gate comment that credits sweep's s16_box_soundness with both blockers, or write the missing row |
| P3 | `D72` | E | Re-mine the epsilon-keyed conditioning pin in profile/tests/review_s2.rs so its building bands exercise the collapse |
| P3 | `H12` | D | Move, convert or delete the eleven never-collected compile_fail doctests in geom-core/tests/ |
| P3 | `S216` | D | Make the repo's ~39 compile_fail rows verify the reason they claim, with machinery rather than a sweep |
| P3 | `S230` | D | Put a measured ceiling on the certified widths in editor-core, pncad-py and sweep test rows, on the tightness home |
| P3 | `census-verdict-golden-hashes-an-anchor-ordered-stream` | E | the shell census golden hashes voided_rod's verdicts in decision order, so a pure re-anchoring of a loop moves it and a sign change is indistinguishable from a move |
| P3 | `decoration-seam-header-names-no-pin-for-enclose` | E | decoration_seam.rs's header says the ssi::enclose crossing is pinned by no row; enclose.rs's own decoration_seam module pins it |
| P3 | `dump-ban-lists-spelled-guts-are-a-fourth-copy-and-two-are-dead` | E | editor-core's `guts` dump ban lists are the shape TINT-1's sweep could not see, and the resolve/placement list bans two of the nine identifiers it renders |
| P3 | `interrogate-ladder-header-claims-every-rung-and-pins-five` | E | lib_u5_interrogate's header claims every rung of InterrogateError is pinned; five of ten have no row |
| P3 | `item-body-takes-a-const-generic-brace-for-an-item-body` | E | item_body reads a const-generic argument's brace as the item's body, so an impl with one silently stops being a scope in two censuses |
| P3 | `r1-dual-digest-ladder-collides-and-has-no-keeper` | E | The R1 dual digest's hand-numbered tag ladder collides at 24, beside a comment claiming the opposite |
| P3 | `sentinel-markers-with-no-reader-are-grep-only` | E | mesh7r1_probes' R1-DOOR-ONLY markers are grep-only: sentinels with no programmatic reader |
| P3 | `source-scanning-censuses-are-a-tripwire-on-ordinary-rust` | D | the source-scanning censuses hand-parse Rust and fail loud, so an ordinary-but-unusual signature reds another program's test with a byte offset for a message |
| P3 | `test-fixtures-hand-spell-the-knot-rescale-with-computed-ends` | E | Six test fixtures hand-spell the knot rescale with computed ends instead of the on_domain door |
| P3 | `tests-common-body-fixtures-triplicated` | E | Three crates' tests/common each carry the same six body fixtures; three pairs are byte-identical and three have drifted |
| P3 | `topo-display-contract-rosters-could-be-derived-beside-the-enum` | E | topo's two display-contract enums could carry a compiler-derived variant list beside the enum, as its own src rows already do |
| P3 | `topo-prism-z-construction-is-written-out-again-across-the-tree` | E | prism_z's construction is written out again elsewhere in the tree, including once inside its own file |
| P3 | `topo-tests-has-two-vocabulary-homes-with-no-stated-boundary` | E | topo/tests has two tests-only vocabulary homes and neither names the other |
| P3 | `topo-tests-scaled-probe-box-five-spellings` | E | topo's suites spell the box wrapper n ways, of which the scaled probe box is five |
| P3 | `topo-the-ops-cube-has-four-spellings-and-no-shared-home` | E | The bare-topology ops cube (mev_line/mef_chord) is written out four times in topo, with no shared door |
| P3 | `value-channel-digest-tag-24-collides` | E | the value-channel digest's discriminator is not injective: tag 24 is claimed twice |
| None | `D113` | None | Decide what an intra-doc link in a tests/ file is, and land the mechanism the decision needs |
| None | `D382` | None | Collapse the copies of every_suite_file_is_aggregated onto one test-utils macro (fifteen at the time of the fix; test-utils is already opted in) |
| None | `D70` | None | Decide whether the 13 silent whole-row stand-downs in three test files should be epsilon-conditional at all |
| None | `assert-f6-dump-lists-are-hand-written-mirrors-of-error-enums` | None | assert_f6's dumps lists mirror whole error enums by hand, with nothing to say the enum grew |
| None | `loud-skip-marker-is-a-hand-kept-idiom` | None | The loud-skip marker is a hand-kept idiom in eight files, each copy naming its rows by hand and admitting it goes stale silently |
| None | `loud-stand-down-announcements-are-discarded-by-the-gate` | None | test_utils::vacuity::stood_down prints to a stdout the twelve gating nextest jobs discard: 22 announcements nobody can hear |
| None | `m10-4-bore-pin-row-red-at-interval-1e-6` | None | M10-4 the_bore_pin_fit_as_a_consumer_reads_it is red at interval / eps 1e-6 on main |
| None | `r2-m10-6-header-roster-omits-the-suites-heaviest-row` | None | A test file's own roster lists five of seven rows, omits the heaviest, and names one in the opposite sense |
| None | `sibling-display-contract-suites-hand-mirror-their-enums-too` | None | mesh and topo display-contract suites hand-mirror their enums too, and mesh's list is already one variant short |
| None | `test-headers-name-fns-that-exist-nowhere` | None | Two test file headers name retired fns that exist nowhere in the tree |

### Order

By instrument, and start with the ones whose failure mode is a SILENT
pass: `source-scanning-censuses-are-a-tripwire-on-ordinary-rust` (they
hand-parse Rust and fail loud, so an ordinary construct reds the gate),
`sentinel-markers-with-no-reader-are-grep-only`, and
`the-per-impl-sight-anchor-is-a-suppression-list-that-shrinks`.

`test-utils-is-production-source-to-every-narrowing-gate` is P1 and is
the structural row under several others — `test-utils` documenting
itself as dev-only while being production source to every narrowing
gate is a fact the gates' own arguments rest on. `H12`, `S216` and
`C18` are the compile_fail family and are one unit.

### Review posture

OPEN, for this program's first dispatch. TINT inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.

## From VACUITY (merged 2026-11-20)

rows that cannot fail: assertions on their own codomain, probes that print, floors that count what they discard

Opened 2026-09-20 by TINT's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

### The slate

**22 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P1 | `random-integer-rays-search-trips-at-eps-1e-6-on-one-run` | H | review_gui1_r1::random_integer_rays_match_the_exact_oracle failed once at eps = 1e-6 on a commit that passed it before and after |
| P3 | `anti-vacuity-floor-cannot-go-red-on-degradation` | E | The GUI0-R1 walk's anti-vacuity floor survives deleting either refusal arm: a guard 16x looser than it reads |
| P3 | `corpus-result-node-loops-skip-silently` | E | m4_pr8_corpus.rs's result-node loop can skip every document with nothing counting it |
| P3 | `e2-recut-probe-has-no-residual-independence-left` | E | review_arceval's E2 now re-executes the shipped row's fixture, constant and predicate, and can no longer disagree with it |
| P3 | `fuzz-rows-discard-trials-against-a-floor-that-counts-them` | E | A fuzz row's continue shrinks its own sample while its coverage floor is written against the attempted trial count: twelve candidate files |
| P3 | `m5-s13-pips-union-escalation-arm-runs-at-no-gated-eps` | E | m5_s13_pips_interval's union escalation arm executes at none of the three gated eps rows |
| P3 | `mate6r1-shared-has-eleven-tests-and-no-assertions` | E | Three mate/fix suites print unasserted probe answers; mate6r1_shared's eleven assert nothing at all and its in_part named the wrong node for its whole life |
| P3 | `phantom-turn-row-stands-down-outside-the-tree-s-door` | E | m10_5_r1 phantom-turn row asserts nothing in three of four arms and announces through a bare println!, outside the stand-down door |
| P3 | `pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1` | E | review_gui1_r1's random-ray oracle sweep asserts "some draw hit the cube" over 120 integer rays at effort 1 — a probabilistic guard that reds CI on a bad seed |
| P3 | `rounded-plate-bulges-are-asserted-by-nothing` | E | The conic corpus's rounded plate can lose both its bulges and no row reds |
| P3 | `sign-asserts-on-structurally-zero-measurements` | E | Test-integrity class - sign/positivity asserts riding cancellation noise on structurally-zero measurements (the washer donut instance) |
| P3 | `topo-cylinder-sheet-geomsources-are-asserted-by-nothing` | E | mate5's cross-instance GeomSource fingerprint — node, minted index and pairwise distinctness — is documented at three suites and pinned by no row |
| P3 | `torus-tangency-shell-floor-does-not-scale-with-k` | E | the_clamp_floor_clears_the_torus_tangency_shell asserts a fixed floor against a shell that grows as K^(1/3) — red at CAD_AMBIGUITY_K=30 on main |
| P3 | `unexplained-literal-seeds-do-not-say-which-shape-they-are` | E | Fourteen test-side PRNGs take an unexplained literal seed: none says in-file which shape licenses it |
| P3 | `verbs-f7-r2-probes-brick-rows-print-without-asserting` | E | verbs_f7_r2_probes' two brick-operand rows print their boolean outcome and assert nothing |
| P4 | `cert10-strict-gap-floor-gates-on-a-varying-seed` | E | nurbs_cert_fuzz's cert10 strict-gap floor gates hosted CI over a per-run varying seed and fails at some seeds |
| P4 | `folded-slab-strands-a-hand-derived-volume-and-a-silent-refusal-arm` | E | review_s12_adv's log probe restates the slab's volume by hand and swallows a refusal, so no fixture change can red it |
| P4 | `malformed-ambient-eps-reds-review-m2-pr7-k` | E | local cargo test — review_m2_pr7_k reds under a malformed ambient CAD_TOLERANCE_EPS |

### Order

`random-integer-rays-search-trips-at-eps-1e-6-on-one-run` FIRST, and
not because it is the biggest: it is a test that actually failed, and
it may be reporting a real pick defect rather than its own weakness.
Until that is settled it is not safe to treat the rest of this slate as
test hygiene.

Then the rows that assert nothing at all
(`mate6r1-shared-has-eleven-tests-and-no-assertions`,
`verbs-f7-r2-probes-brick-rows-print-without-asserting`,
`phantom-turn-row-stands-down-outside-the-tree-s-door`) — one unit,
not three. The floors and the seeds behind them.

### Review posture

OPEN, for this program's first dispatch. TINT inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
