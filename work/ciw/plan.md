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

The row files are the slate, and `python3 scripts/work.py status`
reads it; no table here copies it. The sitting of 2026-11-22 sorted
the whole slate by one test, its effect on PR latency (`log.md`,
2026-11-22):

- **P1** goes to a row whose fix cuts fix cycles that, on the evidence,
  recur, with at least one in the last month: the gates step (the P0),
  a conflicting PR that silently gets no run, the nightly attribution of
  a newly red slow row to its merge, and the slow pins a `src` change
  moves without naming them.
- **A new check earns its place.** A row that adds a test, gate, step or
  job names the incidents it would have caught, shows that no cheaper
  check catches them, and shows that its cost per PR, times how often it
  runs, is well under the fix cycles it saves. Incidents more than a
  month old count for little: the feature that produced them is
  probably done. A row that cannot show this is closed or rewritten
  narrower, and that is how the slate lost its gate-per-finding tail.
- Anything that adds to the lint job's gates step, or to every code
  PR's shards, carries the heaviest burden of proof.

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

Its rows are on the slate like any other, under the rule above: a guard has to earn its cost, so for most of them the written reason is the answer.

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

### Review posture

OPEN, for this program's first dispatch. TINT inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
