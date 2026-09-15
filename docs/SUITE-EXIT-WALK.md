# SUITE exit walk — PROPOSED

**Program:** SUITE — suites, fixtures and the helpers they copy.
Opened 2026-09-11, all six units merged 2026-09-15. The slate reads
**eight closed, zero open** (`work.py status --program suite`).
**This walk is PROPOSED and is
not ratified until Ev merges the `[ev]` PR carrying it** — the S-MATE
convention (Ev, 2026-09-04). It must not be read as a done-state record
before then, and `docs/DOC-LEDGER.md` carries no entry for this program
until the sweep that follows ratification.
**Charter and criteria:** `work/suite/plan.md`. **Narrative:** `work/suite/log.md`.

Criteria are quoted **verbatim** from `plan.md`'s "Exit criteria", per
`docs/DOC-LEDGER.md`'s rule that a walk quotes rather than paraphrases.
They were written on 2026-09-15 before any unit ran, and the first thing
this walk owes is that they were written at all: **the program opened
without them**, which is a defect in its own charter and is recorded as
such in the section it added.

## The slate, as it stands

| row | status | closed by |
| --- | --- | --- |
| `S391` | closed | PR 2624 |
| `D403` | closed | PR 2626 |
| `editor-core-suites-carry-eleven-part-resolver-stubs` | closed | PR 2628 |
| `D114` | closed | PR 2630 |
| `S52` | closed | PR 2639 |
| `topo-arena-census-duplicate-spellings` | closed (passenger, `rides_with: S52`) | PR 2639 |
| `S392` | closed | PR 2650 |
| `genus-rings-helper-spelled-nine-times` | closed **elsewhere**, before this program dispatched | PR 2131 |

Eight rows, eight closed. Six units.

---

## X1 — every row closed, none parked

> **X1.** Every row on the slate is `closed`, or `deferred` with its
> ratification cited in the body. No row is left `parked` on a trigger
> this program could have fired.

**Met.** `work.py status --program suite` shows eight closed, zero open,
zero parked, zero deferred. Nothing was deferred, so no ratification is
cited anywhere and none is owed.

One row closed without this program staffing it: `genus-rings-helper-…`
was closed by PR 2131's census door (`topo::readback::euler_counts`)
three days before the first dispatch. `plan.md` still carried it in the
slate and named it **second in the order**, so the board and the plan
disagreed about what the next unit was. Corrected in PR 2623.

## X2 — closed by a shared home, not a deletion

> **X2. A duplication row closes by a shared home, not by a deletion.**
> For each, the thing that was copied is spelled **once**, and the
> reviewer's standing question is answered at every call site the unit
> touched: does the shared helper still make that suite's intent
> readable, or has the suite become a call into a fixture nobody reads?
> A row that closes because its copies were deleted without a home is
> not closed against this criterion.

**Met, with one deliberate exception that strengthens it.**

| row | the home |
| --- | --- |
| `S52` + passenger | `crates/sweep/src/test_support.rs` — a scalar-generic family at `T: Decide`: `extruded` primitive, `prism`/`prism_on`/`prism_at`/`brick`/`block`/`cube`/`corners`/`pocket_die`, reached across crates by an off-by-default feature on a **dev** edge |
| `S392` | `loft_prism` / `loft_prism_at` / `loft_prism_sections` + `PRISM_*`, beside them |
| `editor-core-…-stubs` | `crates/editor-core/tests/fixture/resolver.rs` — one `PartStore`, `in_part`, `PART_BODY`, `with_resolver` |
| `S391` | no home needed: a rename. Both genuine spellings of the stack already existed in `turning_orientation.rs`, so minting a third was the wrong fix |
| `D403`, `D114` | not duplication rows |

**The standing question was answered `no` at three sites, and that is a
result, not a failure.** `editor-core/tests/{seat6_param_source,
seat7_sweep_lowering}` keep their inline bodies because the rows assert
an *equality between two constructions* that a reader can only check at
the site; the four refusal-tolerant `mesh` sites keep theirs because
their subject is a typed refusal a panicking fixture cannot hand back;
`demos/tour` keeps its own spellings by the demos rule below. Each says
so at the site.

At `lib_u3_sections` delegation **did** cost the suite its subject. It
was repaired rather than reverted: the row now reads the sections back
and asserts three one-loop quads with every bulge exactly `0.0`, and
that assertion was falsified before commit (a `0.01` bulge reddens it).

**A ruling this program made and applied twice:** `demos/tour` and
`demos/wild` never reach into a crate's `test_support`. They are
evidence about the public API from an outside consumer's seat, and the
module is `#[doc(hidden)]` and dev-only, so a demo naming it would lean
on a private path — `memories/demo-purpose.md` and implementer-discipline
§3. The tour's `boxy` and `skinned.rs`'s prism, and `docs/GUIDE.md`'s
and the Python suite's copies downstream of them, are therefore
**deliberately separate spellings and not unconverted duplication.**
The S52 lane found this is already the tree's settled practice, twice
documented at the copy sites; this program restated it rather than
ruling it.

## X3 — every unit states what its sweep could not match

> **X3.** Each unit states what its sweep pattern could **not** match.
> A sweep whose blind spot is unstated is an unverified claim, not a
> negative result, and this program's own `S392` is the receipt for what
> that costs — its first count was a truncated grep read as a
> population.

**Met in the letter, and the criterion is too narrow.** Every unit
states its blind spots; several were then defeated by a blind spot they
had stated, and two were defeated by one the criterion does not ask for.

**The census that moved most:** `S392`'s loft prism, **11 → 18 → 21 →
23 → 24**. The row opens by dissecting its own first miss (a grep piped
through `head -20`, the truncation read as the population) and was then
missed again by the orchestrator, the lane and the reviewer in turn.
`S52`'s two open sub-rows turned out to sit on ~70 copies of the box
fixture across seven crates. The `editor-core` row's eleven suites were
seventeen, across two crates.

**The finding this criterion does not reach.** Of the misses, some were
the search **pattern** — a copy that renamed (`boxy`), an `impl` spelled
path-qualified, a trapezoid computed as `-1.0 - d` — and some were the
search **scope**: one crate instead of the workspace (the seventeenth
`PartStore`, in `crates/viewer`), and `crates/ demos/ tools/ scripts/`
with `docs/` never in the set (`docs/GUIDE.md`, which lofts the prism
and *executes* through the python suite). **A wrong scope is worse than
a wrong pattern because it looks clean**: a bad regex returns odd
results, a bad path list returns tidy ones and a confident count. In
both scope misses the list was *inherited*, never re-derived — by the
row, by the dispatch and by the lane alike.

**An amendment to §5 was proposed here and is withdrawn** (Ev, on the
`[ev]` PR; tested rather than defended). §5 already says what this walk
was about to say: *"what that pattern could not match"*, *"before you
write the scope sentence, grep for the shape — not the symbol"*, and a
paragraph headed *"Scope sentences read as completeness even when the
claim above them does not share their scope."* Tested against the six
misses, the proposed wording would have caught **none** — the four
pattern misses are already covered by prose that three lanes violated
anyway (one while quoting it), and a lane writing *"I searched `crates/
demos/ tools/ scripts/`"* has written something true that does not
prompt anyone to check `docs/`.

**The finding survives the amendment's withdrawal, and it is about
defaults rather than disclosure.** A disclosure rule asks a lane to
notice its own blind spot, which is what the six misses are evidence it
cannot do. What worked, twice, was mechanical: sweep with `git grep`
over every tracked file, **no path argument**, which is the only form of
the sweep that makes no path claim, and narrow only with a stated reason
why the excluded paths cannot hold the class. Both of `S392`'s final
sweeps are that form. Whether it is worth a line in `docs/prompts/` is
Ev's, as its own conversation — the evidence that more §5 prose does not
work is this program's own.

## X4 — no unit minted a fresh instance of the defect it closed

> **X4. No unit minted a fresh instance of the defect it closed.** This
> is the trap `docs/prompts/reviewer-style-lane.md` records for exactly
> this shape of work, and naming it in a PR body has never prevented it;
> only a reader who did not write the fix has caught it. The walk names
> who checked, per unit.

**Met only after review, and the pattern is the program's main result.**
**Every duplication unit minted or nearly minted the defect it was
closing, and in every case a reader who did not write the fix caught
it.** Naming the trap in the PR body prevented nothing — twice a unit
committed the defect *inside the paragraph that named it*.

| unit | what was minted | who caught it |
| --- | --- | --- |
| `S391` | the rename undone one line later at both live call sites (`let axis = first_wall_chord(…)`) | style review |
| `editor-core` | (clean) — but it walked past 12 `fn run`, 12 `opts`, 15 open-coded literals it had just made collapsible | full review, then X4 re-check: clean |
| `S52` | `boxy`: deleted `verbs_shell::brick`, left `verbs_shell::boxy` **twenty lines above it**, in a PR whose sweep section explains that name-based greps miss renamed copies | style review |
| `D114` | `feed_props`: moved the body feed to a shared home and wrote a **new local** mass-properties feed one file over — the 13th spelling in the tree | full review |
| `S392` | `stacked_at`: a **seventh** spelling of "translate by z", added public into the shared home, in the commit whose X4 section credited it for not minting a second `quad`; plus a hand-written numeric census thirty lines below the paragraph explaining why the module keeps none | full review |

All resolved: seven `stacked` bodies became one; `props_digest` moved to
the shared feed with its 31-file class filed; `boxy` collapsed onto a
new `block` view; both hand-written censuses deleted in favour of
describing the shape.

**Reviews run:** style-only on `S391`, `S52` (plus two gating
correctness claims, since it changed three cargo manifests); full on
`editor-core`, `D114` and `S392`. Ev set the posture — style by default,
full for the hardest — and three of six qualifying is the honest count.
Two units got a second, narrow re-check after their fix pass
(`editor-core` for X4 on the new diff; `D114` after its lane disclosed
that a `git checkout` had destroyed uncommitted edits it then rebuilt).

## X5 — every disclosed residue has its own file

> **X5.** Every residue a lane disclosed has its **own file**, on this
> slate or on the owning program's, minted at the moment of disclosure.
> A residue disclosed only in a `## Closed` section or a PR body is
> invisible to the re-homing sweep and dies with this directory.

**Met. Eighteen rows filed on four other programs' live slates, one
closed on another program's slate, two amended.** Nothing is left on
SUITE's own slate, so the re-homing sweep has nothing to do.

**S-TINT (12 new, 1 re-scoped)** — `crates/*/tests/*` is its territory:
`orient-module-prose-accumulation`;
`mate6r1-shared-has-eleven-tests-and-no-assertions` (widened to its
class); `three-part-resolver-stub-residues-resist-the-shared-fixture`;
`run-with-a-prior-evaluation-has-seven-private-copies`;
`topo-tests-brick-copies` (24 `fn brick` in 22 suites);
`tests-common-body-fixtures-triplicated` (three already drifted, so it
reconciles rather than merges); `sweep-per-step-differencing-helper-unhomed`;
`sweep-test-support-two-wrapper-conventions`;
`editor-core-raw-twin-planes-unreconciled`;
`mass-properties-bit-comparison-has-thirty-spellings` (31 files, ~110
occurrences, and the spellings **disagree about whether the pads are in
the comparison**); `corpus-result-node-loops-skip-silently`;
`value-channel-digest-tag-24-collides`. Re-scoped:
`sweep-boolean-suite-brick-and-prism-copies`.

**CURVED (2 new, 1 closed)** — `kernel-verbs-teapot-paragraph-predates-the-canal`
(three false claims, one a wrong mental model the paragraph *reasons
from*); `r2-union-wall-probe-only-prints`. And
`teapot-walls-have-no-suite-row` **closed** — see the collisions below.

**CIW (2 new, 1 amended)** — `k-probe-sweep-says-no-test-compares-probe-against-f64`
(extended with the `ci.yml` site rather than opening a second row);
`red-run-whose-jobs-never-started-reads-as-a-broken-tree`; and a fourth
measured occurrence added to the existing `dirty-pr-gets-no-actions-run`.

**GUARD (1)** — `probe-run-floor-does-not-hold-the-probe-f64-differential`,
with its stated cause corrected (below).

**INSTR (1)** — `k-report-bit-identity-claim-has-no-citation`.

## X6 — the program claimed no paths, start to finish

> **X6. The program claimed no paths, start to finish.** `paths` stays
> empty in `program.md`, every unit announced to S-TINT (and `D403` to
> SHELL, whose scenes it touches by courtesy), and no `keep_out` clause
> anywhere in the tree had to be written to accommodate a claim this
> program made.

**Met.** `paths: []` in `program.md`, unchanged. Every unit announced by
seam — S-TINT and S-TCOST throughout, plus BLEND (`sweep/src/`), EXCH
(`stl`, `step-export`, `step-import`), MESH, DOCM and MSOLVE (whose
suites moved), INSTR (`tools/tess-meter`), SHELL (`D403`). No `keep_out`
clause anywhere was written to accommodate this program. The one
`keep_out` edit was a **correction**: the clause announcing `D403` to
"code-quality Track X, which that program still dispatches" named a
program deleted from the tracker on 2026-09-11, the day this one opened.

`work.py lint` reports 0 problems; its 18 warnings are the pre-existing
`*/tests/*` double-claim family that `work/README.md` documents and no
single program may fix. None names `suite`.

---

## What this program actually found

The eight rows named a duplication class each. **In every case the row
was a sample and not the population, and the population was larger by
roughly an order of magnitude.** That is the program's result, more than
any individual fix: two rows on ~70 copies, eleven suites that were
seventeen, eighteen constructions that were twenty-four, a
mass-properties comparison with thirty-odd spellings that disagree about
what they compare.

**A mutation is a sweep instrument.** The `editor-core` unit's strongest
evidence was not a grep: planting `PART_BODY = RecipeNodeId(1)` reddened
73 rows across 11 suites and **zero** in three of them, which is how a
third unguarded consumer was found that no grep had named. With the
guard the same mutation reds 224 rows, 222 carrying the guard's own
message.

**A guard beats a sentence, and the tree proves it.** The defect the
`editor-core` unit found had been **disclosed in prose at a copy site
and never read** (`docm6_seam_declarations`' header: *"the suites'
`in_part` spellings have already diverged once by a node index"*). The
fix is therefore `assert_part_body`, not a paragraph. Two exemption
edges are stated at the site rather than papered over.

**Two suites were measuring their own broken helper.** `mate1_r1_probes`
and `mate6r1_shared` named the profile node where every sibling names
the extrude, so their "good" member names named a face that does not
exist — eight of fourteen printed probe lines change once corrected. It
never gated: `mate6r1_shared` has eleven tests and **zero assertions**.

**A finding's sentence is not evidence, even when the finding is
right.** `D114`/`S168` claimed no test in this tree compares `Probe`
against f64; `crates/profile/tests/review_m2_pr2_probe.rs` already did,
rostered and running. The gap was real but narrower (editor-core's
*evaluation* lane), and a row already on GUARD's slate had inherited the
false sentence as its stated cause.

**Instruments go green without checking.** `D114`'s differential passed
its first review with three anti-vacuity guards and was still made to
pass vacuously: the one arm ε actually reaches had no counter. Guards
that count **feeding** rather than presence are the fix.

## Cross-program collisions — four in one day

Worth Ev's attention because all four are one mechanism: **`work/README.md`
tells a lane to grep the owning program's directory before filing, and
four lanes did not.**

1. `work/curved/teapot-walls-have-no-suite-row` and `D403` are **one
   defect**, filed two days apart on two slates. CURVED could have
   dispatched a lane to build what `D403` built. Closed against PR 2626.
2. `work/tint/sweep-boolean-suite-brick-and-prism-copies` was open,
   named `S52`'s exact class, listed all six sites it converted, and
   nominated a home that is now deleted. Its own declared blind spot —
   *"a builder doing the same job under a third name"* — **is `boxy`**.
3. `main` landed another lane's full re-derivation of the `cavity.rs`
   row mid-unit; merged and reconciled rather than overwritten.
4. PORT-DOORS-1 renamed three test rows in three suites the
   `editor-core` unit was migrating. Resolved by keeping PORT's names
   and this unit's store, verified by compiling: clippy named exactly
   the thirteen imports the deleted stubs had needed and neither of
   PORT's additions.

The one that went right: `S392` added its occurrence to CIW's existing
`dirty-pr-gets-no-actions-run` instead of filing a fifth row.

## The CI surface reports things that are not about the tree

Two rows on CIW, from this program, one mechanism:

- **Six jobs reported `failure` having never been acquired by a runner.**
  Every surface an agent reads — the rollup, the job list, the log 404,
  the empty `output.text` — looked ordinary, and six failures named for
  the six points of one lane look exactly like a real lane-specific
  defect. The discriminator is free and undocumented: **a job that never
  started reports zero steps**, and the sentence lives only at the
  check-run annotations endpoint. This cost this orchestrator about an
  hour and a full lane re-provision.
- **A dirty PR fires no `pull_request` run at all**, so "no run appeared"
  is a merge-state fact, not a CI outage. It compounds with stacking,
  which on a merge-only repo is routine. That cost the `S392` lane ~45
  minutes, and the stacking was this orchestrator's choice.

## Open with Ev

1. **The successor question.** S-TINT now carries **twelve** rows from
   this program alone, several naming classes an order of magnitude
   larger than the row that surfaced them (~70 box copies over seven
   crates; ~110 mass-properties comparisons over 31 files; 24 `fn brick`
   in `topo` alone, blocked on whether `topo` may dev-depend on `sweep`).
   `work/README.md` says a dozen items on one territory is a successor's
   opening slate. **Recommendation: open one**, on the duplication-class
   territory rather than on S-TINT's test-integrity charter — they are
   different questions, which is why SUITE existed. Ev's call.
2. **The §5 amendment is WITHDRAWN** (Ev, on this PR; the test is in X3).
   §5 already carries the sentence it would have added, and the proposed
   wording would have caught none of the six misses. What survives is a
   *default* rather than a disclosure — `git grep` over every tracked
   file, no path argument — offered as its own `docs/prompts/`
   conversation if Ev wants it, not carried by this walk.
3. **The `examples/` coupling** — Ev asked whether it should auto-rebaseline
   as the renders do; **no**, and the difference is load-bearing.
   `crates/step-export/tests/fixtures/loft_prism.step` is read by
   **`step-import`'s** tests, so the corpus is cross-crate input rather
   than one crate's snapshot: auto-rebaselining would move the yardstick
   the round-trip is measured against and the round-trip would keep
   passing. A render is a *view* whose drift is environmental and
   expected (`memories/freecad-render-lane.md`: re-baselining on a mesa
   bump "is the lane working"); STEP bytes move only when the kernel or
   the fixture does. It is accepted deliberately and stated:
   `step-export`'s committed STEP corpus is now regenerated from a kernel
   test fixture. Guarded by `committed_fixtures_are_byte_golden` on every
   PR, so an edit reddens its own branch. Flagged because it is a real
   change in what re-authors a committed corpus.
4. **The A/B protocol was not used** this session, per Ev's instruction.
   `ab_band: 4200-4299` goes unclaimed; no `docs/MODEL-AB-LOG.md` row was
   written.

## On ratification

Per the S-MATE convention (Ev, 2026-09-04): **merging this PR is the
ratification.** The sweep then deletes `work/suite/` and this file, and
records them in `docs/DOC-LEDGER.md` with the SHA they are recoverable
at. Nothing else waits on it — every unit is merged and every residue is
on a live slate already.
