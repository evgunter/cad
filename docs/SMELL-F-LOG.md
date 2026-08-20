# SMELL-SCAN Track F — orchestrator log

**Constituted 2026-08-20.** Track F is the second scan's *instruments*
track: the mechanisms this project uses to know things, and what those
cannot see. §D of `docs/SMELL-SCAN-2026-08.md` remains the schedule —
this file is the execution record: rulings, lane state, review outcomes
and incidents. **Live status is here and in §D, never in `memories/`.**

**This track runs entirely outside the model A/B experiment.** No
Fable/Opus pairing, no ordinal, no row in `docs/MODEL-AB-LOG.md` —
**nothing on this track reads or edits that file.** The experiment is
paused on a model limit (Evan, 2026-08-20); the cheapest guarantee that
the pause stays clean is that this track never touches it. A lane that
believes it needs to is wrong and should ask.

**Branch prefix:** `smellf/` for units; the orchestrator sits on
`smellf/orchestrator`.

---

## Review policy for this track

Not the full orchestrator protocol. Per Evan, 2026-08-20:

- **Style review on every unit** — `docs/prompts/reviewer-style-lane.md`,
  dispatched **by path** (read it once; never paste it), with the
  per-lane emphasis a dispatch owes (`docs/REVIEW-STYLE-DISPATCH.md`).
  On top of the standing brief, every Track F style review answers two
  questions the brief does not:
  1. Is the finding's **original** stylistic problem now *completely*
     gone — not narrowed, not relocated, not half-closed in a way that
     reads as closed (§C13)?
  2. Was it closed in the **best** way available, or merely in a way
     that compiles?
- **Adversarial review only where the change carries meaningful risk** —
  a minority of the track, marked per lane in the roster. The criterion
  is Evan's (`SMELL-C-LOG` C-R12): *complex enough that there is a
  significant chance the change introduces a regression CI will not
  catch*. That is narrower than "this code is load-bearing".

**Why this track needs the criterion stated twice.** Track F's subject
is guards. A unit here typically makes a guard able to fail — which
means the unit's own failure mode is that the guard now fires on
something true. That is a *correctness* exposure in exactly the rows
§D already marks, and merely a taste question everywhere else.

## What a lane does with what it finds

Three destinations, and a lane picks by the finding's kind, not by its size:

- **A new style finding** → recorded in `docs/SMELL-SCAN-2026-08.md`, in
  the lane's own PR, as a new numbered finding or as a member of an
  existing roll-up.
- **A finding about the kernel's logic** → a **GitHub issue**, signed,
  never a smell-doc row. Track F fixes instruments; a logic defect is
  someone else's lane and needs a register that executes.
- **An important design question** → a **PR asking Evan**, per
  `memories/git-workflow.md` — the doc edited to state the question,
  updated in place with the answer. Never a comment on a merged PR.

## Recording convention

**The landing PR carries its own record**, so the concurrent
orchestrators never read a document that is behind the tree. Each unit
makes two edits to `docs/SMELL-SCAN-2026-08.md` in its own PR:

1. the finding's heading becomes `## SNN. FIXED by #NNN — …`, and its
   **original problem statement is replaced** by the record of what was
   done. Version control keeps the original; leaving it in place makes a
   closed finding read as open.
2. the unit's **row leaves §D's Track F table**, per §D's own *live rows
   only* rule.

**Conflicts in that file are expected and survivable**, and there are
three live orchestrators editing it. Resolve by merging `origin/main` —
never rebase, never force-push — and keep both sides; the edits are to
different findings and different rows. **If the only conflict was that
document and CI was already green on the pre-merge head, merge without
waiting for a second CI run** (Evan, 2026-08-20).

---

## Rulings made in this track

| # | The question | Ruling | By |
|---|---|---|---|
| **F-R1** | **Is F8 gated on E-a (#753)?** §D's Track F preamble says *"Nothing in F1–F3 or F8 may open until that lands"* and calls the gate a **file-overlap** gate; F8's own scope cell says its file is *"neither of E-a's two files"*. The two sentences cannot both be operative. | **F8 stays gated, and the preamble is right for a reason it does not give.** #753's actual file set was read from the PR, not from the schedule: it is ten files, not two, and it includes `.github/workflows/ci.yml` and `local-scripts/ci-local.sh`. D44's defect is that `k_probe_sweep.sh` filters CI's probe run to 2 of 16 suites — a fix that makes CI run the other fourteen is an edit to the *invocation*, which lives in `ci.yml`. So the overlap is real; the scope cell simply counted E-a's files from §D's Scope column instead of from the branch. **Recorded as a finding, because the schedule cell is a claim site** (`SMELL-C-LOG` C-R11). | orchestrator, 2026-08-20 |
| **F-R2** | **F4's S84 half is `crates/geom-brep/tests/m5_pr7_ssi.rs`, which is the single code file Track C's open #734 edits.** §D's Track F table names no edge here. | **F4 waits for #734.** Not split: S84 is one of four members of *one missing idiom*, and a lane that closes three and reports the fourth is the half-fix this document already records as §C13. The whole row sequences behind #734 rather than fragmenting the class. | orchestrator, 2026-08-20 |
| **F-R3** | **F6 and issue #746 are the same file** — `tools/tess-lint/src/lib.rs`. #746 is Track C's **C15**, the positional-ordinal join, and F6's row explicitly excludes it as part 2 of S73. | **F6 opens, and declares the boundary rather than assuming it.** C15 is *unstaffed* — a row and an issue, no `C-` lane letter — so there is nothing to collide with today, and holding an edge-free row against an unstaffed one is how a register stops executing (§C3). The lane's brief fences it off `compare`'s key and off the `else { continue }` arm, and its PR says so, so that whoever takes #746 can see the boundary from the tree. | orchestrator, 2026-08-20 |
| **F-R5** | **F8 / D44 — is 14-of-16 probe suites type-checked-but-never-run a cost posture or a defect?** Re-derived before asking: there are **16** probe-gated suites under `crates/*/tests`, and `k_probe_sweep.sh:91,94` executes exactly two module filters (`m4_pr8_k_probe::`, `k_report::`). Separately, `editor-core/tests/m5_pr5_corpus_probe.rs:21` is a plain `#[test] fn cut_cylinder_replays_at_probe()` with **no `#[ignore]`**, registered at `all.rs:153`, which has never executed in CI — the only thing that runs that crate's probe feature passes `--ignored m4_pr8_k_probe::`. | **SPLIT** (Evan, 2026-08-20). The thirteen `--ignored` dump harnesses are **posture**: they are opt-in instruments by design, and the deliverable there is documentary plus a **floor pinning the executed set** so it cannot silently shrink further — S61's ruling one file over, *a gate must be sited where it can fire on its own inputs*. The plain non-`#[ignore]`d test is **not** posture and gets run — **conditional on Evan's caveat: *"ensure the accidental-looking skipped test isn't, like, super compute intensive."*** See the note below; the measurement is F-h's, and it is a gate on the ruling, not a footnote to it. | Evan, 2026-08-20 |
| **F-R6** | **F1 / S59 — widening the compound-`Bounds` matcher reds every site spelled `Decide + CertifiedBounds`, which `real.rs:787` says *"still needs ratification"*. The gate has been blind to that spelling, so none of them was ever ratified.** What happens to the pre-existing population? | **Convert what should exclude a dual; grandfather only the residue** (Evan, 2026-08-20) — the mechanical fix the S87/S88 ruling already describes, with an allowlist entry carrying a one-line reason for whatever genuinely remains, and only new sites coming to Evan. **With the caveat that is the operative half:** *"be careful with grandfathering; if it's semantically wrong or suggests that the relevant code should be moved into a different layer then that should be done instead."* So an allowlist entry is a **last** resort, not a default landing place, and the lane owes a per-site reason of a kind that survives being read back. See the note below. | Evan, 2026-08-20 |
| **F-R4** | **F7's members live in six crates' `tests/`, and Track E's open #763 rewrites `crates/*/tests/all.rs` in nine of them.** | **F7 opens, and does not delete test files.** Editing a member's body is disjoint from `all.rs`; *removing* one is not, because the aggregation module names it. Where F7's sort concludes a member should be deleted rather than repaired, the lane records the conclusion and leaves the deletion to a follow-up row — it does not take `all.rs` out from under #763. | orchestrator, 2026-08-20 |

### F-R5's caveat, and what F-h owes against it

Evan's condition on running `cut_cylinder_replays_at_probe` is that it not be
*"super compute intensive"*. Two costs, and they are not the same:

- **Compile cost is already paid.** `--features probe` monomorphizes every
  generic-over-`Real` body at `Probe` — the file's own header says so, and that
  is why it is feature-gated at all. But `k_probe_sweep.sh` already builds
  `-p editor-core --features probe --test all`, and `all.rs:153` already
  registers this module, so the binary containing this test **is built on every
  building merge today**. Running it adds no compilation.
- **Runtime cost is one corpus document replayed at `Probe`** — build
  `corpus::cut_cylinder::document()`, `corpus::eval::<Probe>`, compare
  failures. One document, one scalar. That is the number F-h must **measure**,
  not estimate, and measure on **hosted CI** rather than in whatever container
  the lane runs in (`memories/perf-measurement-lane.md`: a timing is worth
  nothing without its box).

**Amended by Evan, 2026-08-20, and the amendment is the better condition:**
*"test cost seems probably fine then, would just want to check that it isn't,
like, the longest thing in some cluster so it increases total CI latency."* So
the question is not *is it expensive* but **does it move the critical path**.
That is checkable, and it was checked — from run **32388258102**, the most
recent green *building* run on `main` (the nine runs after it are all docs-tier
and every job skipped, which is S61's posture doing exactly what it is supposed
to):

| job | duration | `needs:` |
|---|---|---|
| `build + archive (default)` | 9.7 min | `filter` |
| `build + archive (interval)` | 8.8 min | `filter` |
| **`k-lint (gate)`** | **8.3 min** | **`filter`** |
| `render lanes / kernel montage` | 2.6 min | `filter` |

**`k-lint` is a leaf off `filter`** — it does not `needs:` the build, so it runs
in parallel with it. The critical chain is
`filter (0.2) → build (9.7) → test (0.8) → cleanup-archives (0.1)` ≈ **10.6
min**; k-lint's is `filter (0.2) → k-lint (8.3)` ≈ **8.5 min**. **So the budget
is ~2.1 minutes** before this job becomes the thing everything waits on.

**And there is a 3× multiplier waiting in the obvious placement.**
`k_probe_sweep.sh:80` is `for eps in 1e-6 1e-9 1e-12`, and both existing
`run_dump` calls sit inside it. A test added there costs three runs, not one, so
the per-ε budget is ~40 s. **The placement is therefore part of the fix, not an
implementation detail**: `cut_cylinder_replays_at_probe` asserts a *bit-identical
replay*, not a margin distribution — it has no reason to be swept per ε — so
running it **once, outside the loop** is both cheaper and a truer statement of
what it checks. A lane that drops it into the loop because that is where the
other invocations are has paid 3× for a claim that is not per-ε.

**DISCHARGED by Evan, 2026-08-20**, on the measurement above: *"ok yeah k lint
is consistently shorter, sounds like there's no worry about test time then."*
So **the cost condition on F-R5 is settled and is no longer a gate on F-h.**
The lane does not owe a re-measurement, and it does not owe the `#[ignore]`
fallback: `cut_cylinder_replays_at_probe` gets run.

**What survives the discharge, and why it is not a cost argument.** Run it
**once, outside the ε loop.** That is now a *truthfulness* point rather than a
budget one: the test asserts a bit-identical replay, not a margin distribution,
so sweeping it per ε would state a per-ε claim the test does not make. It would
also have been 3× the cost, but that is no longer the reason.

**Two honesty conditions on those numbers.** They are **one sample**, and job
durations move with runner and cache state — F-h states which run each number
came from (`memories/perf-measurement-lane.md`) and re-takes them if `main` has
moved materially. And the ~2.1 min slack is a property of *today's* graph: it is
an argument for placing the test, not a licence anyone else may spend.

**The fallback is part of the ruling, not a concession.** If the measurement
comes back expensive, the answer is *not* to quietly leave the test unreachable
— that is the state the finding is about. It is to give the test the same
disposition as its thirteen siblings **explicitly**: an `#[ignore]` plus a
sentence at the site saying it is an opt-in instrument and why, so the next
reader learns from the file rather than from the invocation. What D44 actually
found is a test whose disposition was decided by a filter nobody read; either
disposition is defensible, being decided by accident is not.

### F-R6's caveat, and why it changes the lane's default

The ruling reads as *convert, then grandfather the residue*, but the caveat
inverts which of those is the resting place. **An allowlist entry is a
confession, not a disposition.** Before writing one, the lane asks two questions
Evan named:

1. **Is the bound semantically wrong here?** If the site should exclude a dual,
   change the bound. That is the whole of the mechanical fix and needs no
   allowlist.
2. **Does the site's needing this spelling say the code is in the wrong layer?**
   If a body needs `Decide` *and* certified brackets, and it sits where neither
   is natural, the finding is the placement — and moving it is the fix, with the
   allowlist entry the thing that would have hidden it. `real.rs` names
   `probe_tube_chart` (`geom-brep/src/ssi/certify.rs`) as *exactly the shape the
   rule targets*; a lane that allowlists it has closed the gate hole and kept
   the thing the gate exists to find.

Only what survives both questions gets an entry, and its one-line reason has to
survive being read back by a reviewer who did not write it. **If the residue is
large, that is itself the finding** — report the count before writing the
entries, not after.

---

## Number reservation, and why this track takes a block

**`D61`–`D70` and `S117`–`S126` are Track F's.** The register's standing rule is
*take the next unassigned number from the orchestrator, never the next gap you
can see* — a rule written after two lanes on one track minted `C11`
independently, an hour apart, in two unmerged branches (C-R20). **That rule was
written for lanes inside one track and does not survive three concurrent
orchestrators**: Track E's D-numbers and Track F's come from one sequence, the
orchestrators cannot see each other's unmerged branches either, and asking is a
round trip through a document that is behind the tree. A per-track block is the
smallest thing that closes it, and it is recorded in §D so the other two can
read it rather than infer it.

Sub-blocks, so a lane does not have to ask for the common case. A lane needing
more than its two or three says so in its report and takes the next free
sub-block, from the orchestrator.

| lane | §D rows | findings |
|---|---|---|
| **F-a** | D61, D62 | S117, S118 |
| **F-b** | D63, D64 | S119, S120 |
| **F-c** | D65, D66, D67 | S121, S122, S123 |
| unassigned | D68–D70 | S124–S126 |

---

## The standing lane header

**Committed, not kept in a container.** Track C lost this text twice in one
session — once to a reclaimed container, once to a branch that was pushed and
never merged. *A register that has not landed is not a register, and a brief
that lives only in a home directory is not a brief.* Binding on every Track F
implementer lane, alongside the unit's own brief.

**Read first, in this order:** `docs/prompts/implementer-discipline.md` in
full; this file's *Review policy*, *What a lane does with what it finds*,
*Recording convention* and *Rulings* sections; then your finding's own text in
`docs/SMELL-SCAN-2026-08.md`, and §D's Track F row for it.

**This track is outside the model A/B experiment.** No pairing, no ordinal, no
row in `docs/MODEL-AB-LOG.md`. **Never open that file.**

**Where your files go.** Your clone is `~/.local/share/cad-work/<lane>/cad`;
`export CARGO_TARGET_DIR=~/.local/share/cad-work/<lane>/target`, never shared
with another lane — a shared one will serve you another lane's binary, and it
has already produced a green claim over ten broken assertions. Heavy cargo goes
through `local-scripts/with-build-slot.sh` (machine-wide mutex, width 1).
**PR bodies and any other to-be-published text go to
`~/.local/share/cad-work/<lane>-pr.md`** — never the session scratchpad, which
is shared between concurrently running agents. **Disk is tight** (~20 GB): do
not start a second `target/`, and say so if you need one.

**Commit and push at every seam.** Everything pushed survives a container
reclaim; nothing else does. If your brief does not name your seams, invent them
and say what they were.

**Recording your own completion.** Your PR makes two edits to
`docs/SMELL-SCAN-2026-08.md`: the finding's heading becomes
`## SNN. FIXED by #NNN — …` with its **original problem statement replaced** by
the record of what was done (version control keeps the original), and your row
**leaves** §D's Track F table. Check the surrounding prose as well — Track F's
preamble names rows by name, so a landing that leaves the table and stays in
the paragraph makes the paragraph false. Delete your roster row in this file
too. **Row and finding numbers are assigned by the orchestrator** — ask, never
take the next visible gap; two lanes on another track minted the same number an
hour apart doing exactly that. Conflicts in these two files are expected and
survivable: resolve by merging `origin/main`, **never rebase, never
force-push**, and keep both sides.

**A brief is a claim site.** If a line number, path or citation in your brief —
or in §D's Scope cell, or in the finding itself — does not resolve, **check
rather than comply**, and report what the line actually contains. Three of five
briefs in one session on another track carried one that did not, and this
track's own table was already found stating a file set nobody had read (F-R1).
The second scan's own instruction on F3 is *"its line numbers are fiction —
re-derive, do not transcribe"*; treat that as the default everywhere here.

**What Track F units are especially exposed to.** This track's subject is
guards, so its characteristic failure is not a broken build — it is **a guard
that now passes for a new reason**. Two shapes to write against:

- *The fix reproducing the defect it closes.* A unit that makes an assertion
  able to fail can mint a new assertion that cannot, one line down.
- *A disclosed blind spot read as a discharge.* Your own "my pattern could not
  match X" is a work order, not an absolution — and it is the sentence a
  reviewer will start from.

**Write claims you can survive having re-derived rather than re-read.** State
the qualifier that makes a claim exactly true, and scope your evidence out
loud: a green `-p onecrate` run is evidence about one crate. **A measurement is
a measurement of a tree** — name which tree each number came from.

**Your final report**, ≤150 lines, states: what you changed and why that shape;
what you swept with and **what that pattern could not match**; every claim
resting on a measurement and what guards it; which of the style brief's
questions you exercised; and anything you are holding back — you will be asked
before the merge, so answering saves a round.

---

## Lane roster

**Wave 1 — open now.** These lanes share no file with each other, with
Track C's open lanes, or with Track E's. **F-a is done and its PR is open**
(#788, style review pending) — see *Landings*.

| lane | row | branch | scope | review | state |
|---|---|---|---|---|---|
| **F-b** | **F6** (S73 parts 1 and 3) | `smellf/f6-tess-lint` | `tools/tess-lint/` | style | **dispatched** 2026-08-20 |
| **F-c** | **F7** (S110, sort first) | `smellf/f7-cannot-go-red` | six crates' `tests/`, `memories/test-suite-cost.md` | style | **dispatched** 2026-08-20 |

Lane clones are `~/.local/share/cad-work/smellf-{a,b,c}/cad`. They are
**reused** stale lanes from finished work, renamed and reset to `origin/main`
rather than freshly cloned: the box was at 22 GB free with three concurrent
tracks running, and a fresh clone plus a cold `target/` is several GB a lane.
`core.hooksPath` was verified on each before reuse, since that is the one thing
a hand-rolled clone silently lacks.

**Wave 2 — the gates fell together.** #753 (E-a) and #734 (C-d) both merged
2026-08-20, so every wave-2 row unblocked in one step. Dispatch is staggered by
**disk and by `scripts/gates/` overlap**, not by dependency: F1, F2 and F3 all
live in that directory and two of them share `scripts/ci-filter.py`.

| lane | row | branch | scope | review | state |
|---|---|---|---|---|---|
| **F-d** | **F4** (S76, S78, S84, S91) | `smellf/f4-guards-that-pass` | `topo/src/review_d18.rs`, `sweep/tests/review_d2_adv_probes.rs`, `geom-brep/tests/`, `geom-core/src/spline/knots.rs` | **ADVERSARIAL** (S76, S78) + style | **dispatched** |
| **F-e** | **F1** (S59) | `smellf/f1-certifiedbounds-gate` | `scripts/gates/bounds-allowlist.sh`, `geom-core/src/real.rs` | style; **ADVERSARIAL** for forced conversions | **dispatched** |
| **F-f** | **F2** (S61/S62 + D58–D60) | — | `ci.yml`, `ci-filter.py`, `probe-suite-census.sh`, `gate-roster.sh`, `ci-local.sh` | style | queued behind F-e |
| **F-g** | **F3** (S63) | — | `scripts/gates/{no-extra-real-bounds,bit-identity-debug-only,interval-square-allowlist,lib.sh}`, `ci-filter.py` | style; **ADVERSARIAL** for the `x*x → powi(2)` conversions | queued — owns `lib.sh` |
| **F-h** | **F8** (D44, D45) | — | `scripts/k_probe_sweep.sh`, `ci.yml`, `docs/` | style | queued behind F-f |

**F-e is first because Track G's G4 is blocked on it** — per Evan's S87/S88
ruling, the sentence that makes the `CertifiedBounds` conversion safe is
currently false, and converting before the gate can see the spelling would leave
the ratification requirement unenforced at exactly the moment new code starts
relying on it. **F-g owns `scripts/gates/lib.sh`**; F-e's brief says to stop and
report rather than take it.

**Superseded gate table, kept only as the record of what was gated on what:**

| lane | row | gated on | why |
|---|---|---|---|
| **F-d** | **F4** (S76, S78, S84, S91) | Track C's **#734** | F-R2 — file overlap at `geom-brep/tests/m5_pr7_ssi.rs` |
| **F-e** | **F1** (S59) | Track E's **#753** | `scripts/gates/`, and the widened matcher's own conversions |
| **F-f** | **F2** (S61/S62 + D58–D60) | Track E's **#753** | the same two files, plus `ci-local.sh` |
| **F-g** | **F3** (S63) | Track E's **#753** | `scripts/gates/`, `scripts/ci-filter.py` |
| **F-h** | **F8** (D44, D45) | Track E's **#753** | F-R1 — the invocation lives in `ci.yml` |

**Two more edges, recorded when they appeared rather than when they bite.**

- **`.github/workflows/ci.yml` now has three tracks in it.** #753 (E-a) holds
  the gate-roster and test-aggregation hunks; Track G's **G-a** holds the
  `oracle-*` job comments and publishes a fence saying so (their G-R3); **F-h**
  holds the probe **invocation**. Three disjoint regions of one file, and the
  only reason that is safe is that all three said which region out loud. F-h's
  brief must carry the same fence.
- **`scripts/gates/probe-suite-census.sh` makes `docs/SMELL-SCAN-2026-08.md` a
  hard CI dependency**: it asserts the literal string
  `type-check every probe-gated test target` still appears there (and in
  `docs/K-REPORT.md`, `topo/tests/probe_s5_sectors.rs`,
  `sweep/tests/k_report.rs`). Every Track F lane edits that document. The two
  live occurrences are at S4's record and at Track D/E's, so no wave-1 lane is
  near them — but **F-h is**, since D44 is about that very step, and a lane that
  rewrites the sentence reds the build for a reason its diff will not explain.
  This is S61's own *"a dated historical scan is a hard CI dependency"* residue,
  hitting the track that has to touch the scan most.

**Sequencing inside wave 2.** F-e (F1) lands before **G4/S87–S88**, per
Evan's S87/S88 ruling: the sentence that makes the `CertifiedBounds`
conversion safe is *currently false*, and converting first would leave
the ratification requirement unenforced at exactly the moment new code
starts relying on it. Track G is not this track's, but the ordering
constraint is, and it is stated here so a Track G taker can read it.

---

## Reviews

### #783 (F-b / S73 parts 1 and 3) — style lane, 2026-08-20: **NOT CLEARED**

**What it confirmed.** Part one's *shape* is right — per-column admission at
the parse boundary, the one real absence carried in the type, exit in the
harness voice — and the **#746 fence was verified by diff rather than by
prose**: `compare`'s key, `fresh_faces`, the `else { continue }` arm and its
comment are unchanged, and `Row::recoverable`'s signature still offers #746 the
same seam. Every measured interval the unit reported reproduced on both trees,
including the non-monotonicity that refuted S73's own *"pinned by nothing at
all"*.

**What it found, and it is the row's own defect with the sign reversed.**

| # | Ruling |
|---|---|
| **F-R7** | **The `tess-meter` box reds on a refinement that improves the answer.** Holding `SPLIT_SCAN_DECADES = 8.0`, **S=1000 fails while returning 4844 cells — strictly better than the shipped 4911**; S=322 fails `cells <= 4911` at 4987. The green step counts are exactly the lattices containing exponent `-3.7`; the reds are the ones that do not. So the row pins the constants to **a sample lattice, not a resolution**, and its failure message — *"the range is too narrow or the step is too coarse"* — is wrong in the surprising cases, because those step counts are **finer**. The unit's own defence, *"refinement cannot red it, deliberately"*, is **false**: the superset argument holds only when `S-1` is a multiple of 320. **Direction ruled, shape left to the lane: pin the RELATION, not the answer** — a test that computes its own reference refinement and asserts the shipped pair is within a stated tolerance is monotone-safe by construction. **"No honest box exists" is a passing answer**, recorded at the claim site per Q6; *making it worse than the non-monotone pin it replaced* is the one outcome the row cannot ship. Riding with it: the *"within 2.0%"* figure is **2.04%** on the reviewer's measurement and rests on an unstated choice of denominator. |

**The lesson, which outlives the row.** S73's whole subject is *an instrument
whose failure mode is its pass condition*. The fix for it minted **an instrument
whose pass condition is a sample lattice** — a guard that goes red on an
improvement. The lane header's first named shape is *the fix reproducing the
defect it closes*; this is that shape arriving inside the very row written to
demonstrate a box, in a unit whose part-one work was otherwise careful and
correct. **Boxing a constant by pinning the answer it currently produces is not
boxing it** — it freezes the sample grid that produced the answer, and every
property of that grid becomes load-bearing by accident.

**A second class, worth as much as F-R7.** Three of the review's findings are
one shape: **the fix's own new mechanism is unobserved by its own tests.** The
`>= 1.0` cell-count floor — *the load-bearing premise of the whole part-one
argument* — is pinned by nothing, because the four-value bad-input matrix
cannot separate `CellCount` from `Positive`. `delta`'s new admission is
unobservable and is defended by a reason that does not apply to it (it is a
denominator, not a numerator). And `Some(0.0)` is folded straight back into
`None` by the first call that uses the type introduced to separate absent from
small. **A unit that adds refusals owes a test that can see each one**, and the
admission matrix had its hole exactly where its argument was.

**A convention clarified rather than changed** (the review read the sequencing
as wrong): a lane records its landing **in its own PR**, so the record lands
*atomically with the code*. A landing recorded in a PR that never merges never
lands either. The `## Reviews` section here is the orchestrator's and is written
when the review lands, which is why it trails.



## Landings

**F-a — F5 (S92) — PR #788, opened 2026-08-20, review pending.** One home for
the mutation-door set (`topo/src/fixtures.rs`'s `mutation_doors()`), and a
classifier that reads code rather than prose. Measured at merge base
`4f959cb4`: the walk finds **37** doors; the duplicated `&mut self` /
`&mut Body` predicate was **byte-identical** at the two sites, not merely
near-identical; the two tables (23 and 36 entries, **22 names in both**) were
consistent with each other and **deliberately did not merge** — they are two
properties of one set, and a merged table would let an edit about pcurve
staleness red the tier-1 guard. Both sites now carry that reason.

The string-match hole was **demonstrated before it was closed**: a planted
door whose body held only two comments naming the two literals left both
guards green and counted compliant (38 doors: 15 asserting / 23 allowlisted;
2 re-minting / 36 declared). After the change the same plant reds both.
`MutationDoor` hands out a body with comments, string and char literals
blanked, so a consumer is never given a raw body to `contains` on, and
`fixtures::source_reader` pins the mechanism in both directions — six
spellings of the plant that must not read as calls, seven real calls that
must. Door counts are unchanged by the fix, so nothing was over-stripped.

**Not cleared on first pass (F-R8, F-R9), fixed in the same PR.** The style
review reproduced the finding's own defect one layer beneath the fix: the
delimiter matcher that carved bodies for the blanker knew none of the three
constructs the blanker was written for, so a door carrying `'"'` was dropped
from the walk and the next one's body corrupted. `CodeOnly` is now the crate's
only lexer and the item scan is a method on it, so nothing can run it over
un-blanked text. Demonstrated on named trees: the pre-fix scanner extracted
from `6a2d237a` and run standalone loses the `'"'`-carrying door and finds only
the door after it; the current one finds both. The review also found two
over-strip defects (byte raw strings, char-literal escapes), that
*"over-stripping is loud"* is **false of the pcurve guard**, and that the
argument for staying textual rather than parsing existed nowhere — all now
fixed or written at the site.

**Residue: S117 / D61**, **eleven** further source-text guards, not the seven
this lane first wrote — the count moved 7 → 9 → 11 inside one review cycle,
each step a differently-shaped sweep (`include_str!` was the spelling all six
of the lane's patterns missed). The sharper framing came with the third sweep:
**four hand-rolled Rust readers exist in this workspace and no two lex the same
language**, and `pncad/tests/all.rs`'s `code_without_comments` carries the same
`'"'` defect this lane's review found, worked around in a comment rather than
fixed. Of the eleven, **seven** are served by `CodeOnly` as shipped, **three**
need a comments-only variant and one needs the inverse; five are outside `topo`,
so the row's real question is whether this warrants a test-support crate.
`topo/src/{face_normal,chord_join}.rs` are Track G's G8/G9 and
`pncad/tests/all.rs` is Track E's #763 — flagged there, not taken. **S118 and D62 were reserved to F-a and are unused.**

## Incidents

### F-a claimed its record edits and shipped without them (2026-08-20)

**#788's body carried a complete *Record edits* section** — S92 re-headed and
its problem statement replaced, F5 struck from §D, S117/D61 placed, the F-a
roster row removed, S118/D62 returned unused. **The PR's diff was three
files**, all under `crates/topo/src`. Neither document was in it.

Caught by reading the PR's **file list** against its body before dispatching the
review, which cost one `gh pr view` and is now a standing step here.

**Why this is worth an incident rather than a nudge.** The code in that PR is
good and its central claim is *demonstrated rather than asserted* — a door was
planted, both guards were shown green, the fix was shown to red them. The lane
that held itself to that standard on the hard claim did not apply it to the easy
one. That is the exact asymmetry Track C's log reports for every unit it
reviewed: **the failure is never a shipped wrong answer, it is a claim wider
than its evidence**, and it lands on whichever claim nobody expected to have to
check.

**The rule, now in every Track F brief:** *do not write the Record-edits section
until the record edits are in your diff.* It is cheap, it is mechanical, and the
generalisation is the part worth keeping — **a section describing work is not
evidence the work happened, and the sections least likely to be checked are the
procedural ones.**

**Corrected by the lane, 2026-08-20**, on being sent back: the doc edits landed
in a later commit and the PR body now says so plainly rather than being quietly
retitled. That second part matters more than the first — a silently repaired
body is what teaches the next reader to trust a section they should have
checked.

**Second-order cost, which is the reason this track cares.** The recording
convention exists so that three concurrent orchestrators are never reading a
document that is behind the tree. A PR that merges with the code and without the
record does not just leave a stale finding: it leaves S92 reading as **open** to
Tracks C, E and G while its fix is in `main` — inviting exactly the duplicate
lane the schedule exists to prevent.
