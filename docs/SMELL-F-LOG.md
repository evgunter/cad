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
   only* rule — **but a row that carries evidence the finding's record does not
   is relocated, not deleted** (F-R11). Schedule rows accumulate re-derivations,
   counts and narratives that were never folded back into the finding; striking
   such a row silently destroys the best evidence for the thing being closed.
   Before deleting a row, read it for anything the finding's own text does not
   say, and move that into the record.

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

**The counts in this ruling are withdrawn as prose, 2026-08-20** — by Track E's
own re-derivation of D23, landed after this ruling was written. *"2 of 16"* and
*"14 type-checked and never run"* are **prose counts of a set the census gate
derives every merge**, so quoting them pins a number that moves. **The ruling
does not change**: it is about *dispositions* — thirteen `--ignored` dump
harnesses are posture, the one non-`#[ignore]`d test is not — and a disposition
does not depend on the cardinality. What F-h must not do is restate the totals.

**And F-h inherits a warning that arrived with the withdrawal: the obvious floor
is unsound.** `run_dump` passes `--ignored`, and `m5_pr5_corpus_probe.rs`'s test
is **not** `#[ignore]`d — so a floor built on filter-reachability would score it
**covered while it executes nothing**. That is the exact shape of the finding,
reproduced inside its own fix, and it is now foreseen rather than discovered.
**D45 is withdrawn entirely and D85 replaces it**; F8's rows are **D84/D85**,
not D44/D45.

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

**DISCHARGED on an empty residue, 2026-08-20 (#791).** F-e widened the matcher
and re-ran it: the file set is **identical** to the pre-widening one, and at
line granularity the whole workspace gains exactly one hit — a doc comment at
`real.rs:787` that the existing comment filter strips. **Every `CertifiedBounds`
use in the tree is a sole bound**, so there was nothing to convert and **no
allowlist entry was written**. The ruling's two questions were never reached.
What the sweep *did* find is a compound bound reached through an alias NAME
(`ArcCarrierScalar`, 49 use sites) — the ruling's second question, one level up,
and it is left to **G4** because the answer depends on what the alias is bound
to. That is the shape the caveat wanted: the entry that would have hidden it was
not written.

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
| **F-e** | D68, D69 — **both used** | S124, S125 — **both used** |
| **F-d** | D70 | S126 |
| **F-f** | **D101** | **S157** |
| **F-e** (2nd) | D102, D103, **D106** — all used | S158, S159 — **both used** |
| unassigned (2nd block) | D107–D110 | S161–S166 |

**Second block claimed 2026-08-20: `D101`–`D110` and `S157`–`S166`.** The first
block is spent. Taken beyond Track E's `D81`–`D100` / `S137`–`S156` and Track
G's `D71`–`D80` / `S127`–`S136` rather than into any gap — **and the reason is
now demonstrated rather than argued**: between Track F claiming D61–D70 and
that claim landing, **Track E's orchestrator issued D61–D70 to five of its own
lanes**, and had to reissue them as D82–D89 once both branches were visible.
Two orchestrators, one sequence, neither able to see the other's unmerged
work — the exact failure the block convention exists to prevent, caught only
because both blocks were published. A block that is not landed is not a
reservation.

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

**Wave 1 — open now.** These three share no file with each other, with
Track C's open lanes, or with Track E's.

| lane | row | branch | scope | review | state |
|---|---|---|---|---|---|
| **F-a** | **F5** (S92) | `smellf/f5-door-registries` | `topo/src/review_m1_pr5_internal.rs`, `topo/src/pcurves.rs` | style | **dispatched** 2026-08-20 |
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
| **F-f** | **F2** (S61/S62 + D58–D60) | — | `ci.yml`, `ci-filter.py`, `probe-suite-census.sh`, `gate-roster.sh`, `ci-local.sh` | style | queued behind F-e |
| **F-g** | **F3** (S63) | — | `scripts/gates/{no-extra-real-bounds,bit-identity-debug-only,interval-square-allowlist,lib.sh}`, `ci-filter.py` | style; **ADVERSARIAL** for the `x*x → powi(2)` conversions | queued — owns `lib.sh` |
| **F-h** | **F8** (D44, D45) | — | `scripts/k_probe_sweep.sh`, `ci.yml`, `docs/` | style | queued behind F-f |

**F-e went first because Track G's G4 is blocked on it** — per Evan's S87/S88
ruling, the sentence that makes the `CertifiedBounds` conversion safe was false,
and converting before the gate could see the spelling would leave the
ratification requirement unenforced at exactly the moment new code starts
relying on it. **F-e is out of this table: #791 is open**, and the landing is
recorded below. **F-g owns `scripts/gates/lib.sh`**; F-e stopped short of it and
reported the one helper it wanted there (`selftest_passes`), which is now F-g's
to place or decline.

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
**Discharged: F-e opened #791, and G4 is unblocked once it merges.**

---

## Incidents (orchestrator's own)

### The register's over-claim, 2026-08-20 — S157 as filed was wider than its evidence

**S157 was recorded, escalated and merged with a claim one observation could
not carry**: *"all fifteen gates currently fail on the hosted half without
saying why."* The evidence was a single run showing only
`Process completed with exit code 1`. **The lane that raised the finding
refuted it** on run `32413754011`, where `ERROR:` and `##[error]` both appear
with the full diagnosis. The `::error::` plumbing works; a gate that dies under
`errexit` simply never reaches it.

The finding survives, **narrowed to the mechanism** — which is where it was
always strongest, and which the style review had already stated precisely. What
did not survive is the generalisation the orchestrator wrapped around it.

**Why this one is worth an incident when a lane's would be worth a fix.**
This track has ruled on *a claim wider than its evidence* four times today —
F-a's record edits, F-b's box twice, F-f's *"stated as total and is not"*. Every
one of those was caught because **a lane's claims get a reviewer**. The
register's do not. An orchestrator writing a finding is the one author on this
track with no adversary, and the failure mode is identical.

**Two things follow.** *A finding is not exempt from the standard it enforces* —
S157 was written from one run and escalated on a review's diagnosis without
re-deriving the escalated part. And **the sharpest reviewer of a finding is the
lane that raised it**: F-f had the run open, knew what its own gate printed, and
said so against a document that had just credited it. That is worth more than
the finding was.

### Two more register defects, found by a verifier reading the log itself (2026-08-20)

`docs/SMELL-F-LOG.md` **quoted the pre-fix matcher in F-R10 while the fix-pass
entry two paragraphs below said that group was gone** — one entry contradicting
itself — and said *"Eight self-test cases"* immediately before enumerating ten.
Found by the F1 verifier, which read the record as a claim site rather than as
background. Both corrected.

**That is the second orchestrator over-claim in one session** (the first: S157
as filed). The pattern is now clear enough to name: **the register accumulates
quoted fragments — a regex, a count, a file list — and quoted fragments go
stale exactly like the code comments this scan exists to find.** A finding's
record is prose that argues, and §S38 is the class it belongs to.

**The mitigation that costs nothing:** *do not quote a mutable artefact in the
register unless the entry is about that artefact's text.* Say what the matcher
does, not what it says. Every one of these three defects was a verbatim quote
of something that then changed.

## Standing rules this track derived

### A verification is valid for the PATHS it verified, not for the SHA it ran on

Two correct rules pulled against each other: **a PR must never sit CONFLICTING**
(it runs *no* checks at all and reads as CI being absent rather than failing),
and **a head must not move under a running verification**. Freezing the branch
resolves it in the wrong direction — it trades a real risk for a bookkeeping
convenience.

The resolution costs one command. A verification names **the SHA it measured**
and **the path set it is about**; the lane, after any merge, reports the new
SHA **paired with**

    git diff <verified-sha> <new-head> -- <that path set>

**Empty → the verification still holds, whatever else moved.** Non-empty → it
is re-run rather than reasoned about. Neither party has to wait for the other,
and each half is checkable by the other.

**Two refinements the lane added, both better than the rule as issued.** *Merge
while still `MERGEABLE` rather than waiting to conflict* — same move, strictly
cheaper, and it never passes through the state where CI is silent. And *widen
the path-diff past the paths under verification to the whole subtree the unit
owns*, so the answer covers the rest of the unit rather than only the row being
checked.

**Report the merge and the diff together.** A merge reported without its
path-diff is a claim the orchestrator then has to go and check; the diff is the
thing that makes the merge harmless, so it travels with it.

*(Second instance today of two correct rules colliding. The first: a unit records
its completion in its own PR, yet cannot cite a PR number before the PR exists —
resolved by opening with an honest placeholder rather than a claim.)*

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

### #783 F-R7 — independent verification, 2026-08-20: **the fix FAILED, second instance**

**`SPLIT_SCAN_STEPS = 321 → 323` reds the row** — two steps above shipped,
5.2369% against a 5% pin. The verifier scanned **every** S in 322..2000 with a
replica validated to reproduce the real build to the digit and found **exactly
one red: S=323.** Rarer than the defect it replaced, same species — *the
docstring's thirteen rows were a sample that missed the counterexample sitting
next door.*

**Every figure the lane reported reproduced exactly.** The verifier's own words:
*"the lane measured honestly — the defect is in the claim's shape, not its
arithmetic."*

| # | Ruling |
|---|---|
| **F-R14** | **Second instance, so no third patch — the row goes to first principles, and the first principle is now measured.** *The worst excess moves ~3 percentage points between **adjacent** step counts; the headroom is 0.54.* **The quantity being pinned is discontinuous in the parameter it is pinned against** — so no tolerance on the excess can simultaneously admit every refinement and exclude every degradation. Wide enough to survive the jumps is too weak to catch a degradation; tight enough to catch one is a lottery on the lattice. **Both failed versions failed for this reason wearing different clothes.** The expected answer is the one F-R7 pre-authorised: **not boxable by a tolerance on the excess, recorded at the claim site with this measurement as the reason** — Q6's *"a written reason it can have neither"*, now evidenced rather than shrugged. **And the real result is the question it forces: if the excess is discontinuous in `S`, what resolution guarantee do these constants provide at all?** Possibly none — which is a bigger finding than the box that was asked for. |

**Independent of the boxing question: the oracle guard is green by
construction.** The shipped lattice is a **strict subset** of the reference's
(exponents at multiples of 0.05 in [-8,8] against 1e-4 in [-12,12], ratio exactly
500; **0 of 321 shipped samples lie off the reference lattice**), and both seed
with the same lane fallback — so `reference <= shipped` holds by construction.
**Replacing the reference with the subject itself leaves the row GREEN at
0.0000% on all five members.** A guard that passes when its reference is replaced
by the thing it judges. Two more, both `sure`: **tolerance and family are not
independent** (a plausible sixth member scores 5.8824%, and the worst shipped
member is *mildly anisotropic*, not the ruled wall the row is about), and the
`D = 3/4` *"genuinely better"* argument is outcome-correct with a **failed
mechanism**.

**What this says about the verification lane, which is why it was run.** The
lane had *itself* identified an oracle gap — under-convergence — and asked
whether to close it. **It was deliberately held**, so that an independent lane's
result would be interpretable rather than a re-check of a known answer. The
verifier found **a strictly worse instance of the same gap**: not an imprecise
oracle, an oracle that can *be* the subject. So the gap was findable from
outside, the hold cost one round and bought a calibration, and the author's
restraint is what made the answer mean anything.

**The rule this hardens.** *A guard verified only by its author's chosen
samples is a sample, not a guard* — and when the quantity is discontinuous, no
number of author-chosen samples converges on the truth. Both versions of this row
were green on thirteen honestly-measured points and false as stated.

### #798 (F-f / S61, S62, D58–D60) — style lane, 2026-08-20: **NOT CLEARED**, five MAJORs

**What held, and it is substantial.** All three isolated demonstration commits
verified as descendants of the review head, all TIER=docs, with `discipline` and
every build/lint/test job **skipped** in all three while `mirror` red for the
plant the lane named. **D58 and D59 are completely closed** — mode 0644
re-planted by the reviewer and it reds correctly. **S62/D60's class is genuinely
mechanised**, and the population was *derived rather than listed*: twelve
non-`scripts/gates/` executables, eleven already mirrored, including the member
two prior enumerations missed. **Q6 came out in the unit's favour**: 8/9/12 s,
in parallel with `filter`, **zero added critical-path latency** — one extra
billed job-minute per docs run, which honours *"cheap docs CI stays"*.

| # | Ruling |
|---|---|
| **F-R12** | **S157's mechanism, and it is much larger than S157.** `set -euo pipefail` **aborts the gate before its own `gate_error`** whenever the diagnostic path runs a pipeline or command substitution — and `gate_selftest_case` runs each gate inside `if out=$(…)`, **where bash suppresses errexit**, so the self-test is *structurally incapable* of seeing it. **Every gate can die before its own error message, and every gate's self-test is blind to that by construction.** S157 is escalated to carry this. **F-f owns only the two instances in its own new code; the harness and the sweep are F-g's** — and *fifteen self-tests passing is not evidence here, because the harness is what hides it.* |
| **F-R13** | **The re-siting invariant is unguarded, and the row's own defect is reproducible after the fix.** The reviewer hollowed `mirror` to checkout+echo and moved all three gate steps back into `discipline`: **all three gates stayed green.** The exact S61 state can be restored in one commit with nothing firing. Evan's ruling is a *rule* — *a gate must be sited where it can fire on its own inputs* — currently held as **prose in three headers**. A rule this repo has now paid for twice deserves a check. |

**Three more that each break a stated claim.** Claim 2 is *stated as total and is
not* — the job regex misses uppercase names, and a planted `buildXtra:` reading
`ci-local.sh` after a pruning job returned **OK, exit 0**. The **local half still
`exit 0`s at TIER=docs**, so the ruled rule holds hosted-side only — S61's
deliverable half-closed and reading as closed. And `--citations` is now
**hosted-only**, moved out of one blind spot into another.

**The shape verdict, worth more than the individual defects.** The reviewer's
answer to *best available way* is **no**, and the reason generalises: **grep/awk
over YAML, in a repo that already ships a parser.** Four of the five MAJORs are
text-processing failures — errexit inside a pipeline, a case-sensitive regex, a
blind exclusion. That is not bad luck, it is the tool.

### #791 (F-e / S59) — style lane, 2026-08-20: **NOT CLEARED**, and **G4 is cleared to proceed**

**The headline number survived a hostile re-derivation.** The reviewer re-derived
the zero **independently on `origin/main`** — 122 raw hits old, 123 new, the one
added line a **leading** `///`, so the leading-only stripper is *correct* here
rather than a second bug carrying the claim — and post-strip file sets are
byte-identical at 19 files. The enumeration of `CertifiedBounds` uses is
exhaustive and all are sole bounds. **The zero is honest.** The reviewer also
reverted the matcher and ran each planter **alone**: all three new positives fail
separately, so the self-test cases are individually load-bearing rather than
load-bearing as a set.

**Track G's G4 may proceed on the strength of this PR** — the ruling's
precondition, that `real.rs`'s sentence be true, is met in both operand orders.
**With one correction G4 must carry: the widened matcher fires on nothing G4
actually writes, so a green gate there is not ratification evidence.**

| # | Ruling |
|---|---|
| **F-R10** | **The gate is blind to the two edits that would defeat it.** *(a)* The `trait CertifiedBounds:` definition skip is anchored on the **name**, so `pub trait CertifiedBounds: Decide + Bounds + CertifiedEnclosure {}` is **silently skipped** — planted, exit 0. That is the single edit that would turn every sole-bound site in the tree into a decide-and-bracket parameter at a stroke, and it is **undisclosed**. *(b)* The direct S59 successor — `trait Bracket: Bounds + CertifiedEnclosure` used as `Decide + Bracket` — is invisible, and neither it nor its mitigation is in the gap list. The unit's own argument was that *"an enumerating matcher is blind to the next alias the day it is written"*; **a name-shaped matcher is blind to the next alias that does not carry the name.** Not a reason to return to a list — a reason the gap list must say it. *(c)* **A hole in the self-test itself:** deleting the path-prefix group from **one** side of `+` leaves `--selftest` **green** while `Decide + geom_core::CertifiedBounds` goes blind — *a spelling the tree already uses*; the group on the **other** side is dead code, later confirmed by mutation and removed rather than planted around. **(Corrected 2026-08-20: this entry originally quoted the pre-fix matcher and named the wrong side, while the fix-pass entry below says the group is gone — one entry contradicting itself two paragraphs apart.)** The mutation check covered the matcher wholesale and missed a mutation **inside** it. |
| **F-R11** | **Striking a row deleted the evidence, and the defect is in this track's convention rather than in the lane's judgement.** Removing the F1 row from §D removed the E-g `admit.rs` narrative — the gate's strongest single piece of evidence — with no relocation. The *Recording convention* says a row leaves §D when it lands and says nothing about a row that **carries evidence the finding's record does not**. **Amended below.** Version control keeping something is not the same as a reader finding it. |

**A third thing, small and sharp.** The gate header grew **131 → 168 lines** — and
**S116(m), live in this same document**, measures a 130-line header as *"past the
point where the rule is findable."* The unit grew a header past a threshold the
scan records, in the file the scan records it about. That is not a lapse of care;
it is what happens when a finding's own document is 12,000 lines and its
neighbours are unreadable from inside a lane.

### #788 (F-a / S92) — style lane, 2026-08-20: **NOT CLEARED**

**What it confirmed, by running rather than reasoning.** The reviewer re-planted
`plant_s92_door` on the PR head and got **both guards red**; `code_only` flips
exactly that door's two needles and no live door's; with `code_only` bypassed,
all six `a_mention_is_not_a_call` spellings go red. Every count re-derived. The
unit's correction of its own record-edit gap was read as honest rather than
retitled-around.

| # | Ruling |
|---|---|
| **F-R8** | **The defect is reproduced one layer down, in the same file, by code the fix did not touch.** `public_fns`/`matching` carves the body that `code_only` then blanks — and **its own lexing is strictly weaker**: `'"'` reads as a string opener, block comments do not nest, raw strings are unknown. *Those are the exact three constructs `code_only` was written for.* Demonstrated: a door body containing `'"'` makes `public_fns` **lose the door entirely and corrupt the next one**. The row that should catch it, `a_call_is_still_a_call`, tests `code_only` **in isolation and never through the pipeline**. Not live today (161 = 161 over `topo/src`) — luck, not a guard. **Direction ruled, shape left to the lane: one home for *read Rust source past comments and literals*.** The unit argued exactly that for the door set — *"a guard against duplication should not be the next copy of its own walk"* — then left two lexers in one file with different competence. Riding with it: `br"x\"` **over-strips and erases a real call**; `char_literal_len`'s escape loop starts one byte past the escape (`'\''` short, `'\\'` returns `None`); and *"over-stripping is loud"* is true of the tier-1 guard and **false of the pcurve one**, where a door that stops reading as minting passes **silently**. |
| **F-R9** | **The argument for staying textual does not exist in the tree.** A string-match classifier was replaced by a better string-match classifier, and three of the four disclosed blind spots are artefacts of reading text that a parser dissolves rather than documents. **The deliverable is the argument, not a particular answer:** if textual is right — build cost, `memories/review-and-dependency-policy.md`'s dependency-age rule, a guard that must run without compiling the crate — it goes **at the site**, because a reader today cannot learn why. If a parser is right, that is a **dependency decision and a design question** → a PR asking Evan, not an addition. **What the row cannot ship is the choice made silently for a third time.** |

**Two lessons, and the second is about this document.**

**A disclosed blind spot list is a claim about a population.** The unit disclosed
its blind spots honestly and dispositioned seven class members. The reviewer's
**differently-shaped** sweep found **two more that none of the unit's six
patterns could reach**, because `include_str!` is the blind spelling
(`profile/tests/seal.rs`, `editor-core/tests/schema_ledger.rs`). So **S117's
"seven" was a floor presented as an enumeration** — the same shape as D45, one
track over, and the reason the style brief asks not merely for a sweep but for
*what the pattern could not match*, run **shaped differently** by someone else.

**One wrong number reached four documents before anyone recomputed it.** The
class disposition says "four" in the PR body while naming three, "four" at S92,
and "three" at D61 and in this log — and `face_normal.rs` was excluded *for a
reason untrue of it* (it greps **code fragments**, not string literals, so the
blanker serves it as shipped). A count copied between records is not four
independent statements; it is one statement with three echoes, and this
document's own §C13 is what it becomes.

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

- **F-e — F1 / S59**, PR **#791**, opened 2026-08-20; **CLEARED 2026-08-20**
  after a style review (NOT CLEARED → F-R10, F-R11) and a targeted
  verification pass (F-R15), both addressed in the same PR; **held for the
  merge queue behind F6**, not merged by the lane. `bounds-allowlist.sh`'s matcher is now shaped by the
  trait **name** — `(\+\s*(\w+::)*\w*Bounds\b)|(\b(\w+::)*\w*Bounds\s*\+)`
  — rather than by a list of names, so `Decide + CertifiedBounds` fires in both
  operand orders and so does the alias after it. **Eight self-test cases**:
  both `Decide + Bounds` orders, both `Decide + CertifiedBounds` orders, a
  path-qualified alias after the `+`, an alias name not in the tree, a
  non-`Bounds`-named alias *declaration* in all **three** spellings (pair,
  sole supertrait, `where Self:` — what the declaration alternative catches,
  which is NOT a mitigation for GAP 4; see F-R15 below), real.rs beside its skipped definition lines, real.rs with
  the alias **redefined to carry `Decide`** — plus a **negative** case (sole
  bracket bounds must not fire). An exact-text skip is brittle where a name
  anchor is not, so `gate_definition_skip_subject` proves the two skipped
  lines are still verbatim in `real.rs` *before* the scan and names the repair
  that is meant; a rustfmt-style wrap reds with that message rather than as a
  confusing compound hit. **Mutation battery re-run on the final head**: four
  mutations red exactly one case each, and **two survive, which is reported
  rather than buried** — re-adding the left-hand path group (dead by
  construction; the qualified-left spelling is covered by a positive case
  instead), and reverting the skip to a name anchor (the subject check now
  refuses that edit one step earlier). Three prose sites corrected — the gate header, two
  `real.rs` paragraphs — plus S56's own record, which asserted the same false
  thing. **Red count before allowlisting: zero; no allowlist entry written**
  (F-R6 discharged empty, above). Sweep raised **S124/D68** (`ArcCarrierScalar`
  invisible at 49 use sites — handed to **G4**, which owns the alias's bound)
  and **S125/D69** (`no-extra-real-bounds.sh` is order-sensitive, S56's own
  defect un-swept to a third gate — handed to **F-g**, whose scope holds the
  file). **D68 is a visibility row and G4 does NOT discharge it**; the review
  caught the first draft handing it to G4 as if it did.
- **The style review (F-R10, F-R11) found three further evasions of this gate,
  all now planted.** The alias-definition skip was anchored on the *name*, so
  redefining `CertifiedBounds` to carry `Decide` was silently skipped — the
  one edit that turns every sole-bound site in the tree into a
  decide-and-bracket parameter; the skip is now exact text. The path prefix
  after `+` was untested while a spelling the tree already uses depended on
  it. The same group on the left of `+` was dead and is gone. The header's
  gap list gained **GAP 4** (an alias not named `…Bounds`, and see F-R15 — it
  is disclosed OPEN, with no mitigation) and **GAP 5** (the leading-only comment strip, F-g's to close).
  **The header is 204 lines against 131 at open** — S116(m) measures this very
  file at 130 and is re-measured in place rather than restored; five lines of
  comment archaeology were cut and the lane's own additions compressed twice.
  **The argument that came out of that row is worth more than the row**, and
  it is written into S116(m) rather than left in a transcript: a gate whose
  gaps are honest is longer than one whose gaps are silent, so **this
  directory wants the ratification ledger split out of the script**. The
  per-seam justifications are a document that happens to live in a comment
  block, and they are what makes a 20-line function carry a 204-line header.
- **The lane minted a fresh instance of the defect it closed.** GAP 4's
  mitigation was published as *"the declaration writes the pair literally and
  therefore fires"* — true only of `trait Bracket: Bounds +
  CertifiedEnclosure`. **`trait Bracket: CertifiedBounds` carries both bracket
  doors with no `+` on the line**, so neither it nor `Decide + Bracket` fired:
  S59 exactly, one turn later, in the change that closes S59. The lane caught
  that one itself, by attacking its own sentence rather than measuring it, and
  added a third matcher alternative for single-line trait declarations naming
  a `…Bounds` supertrait or `where` bound. **It then republished the
  mitigation on that alternative, and F-R15 refuted it too** — see below.
  **This is the track's fix-passes-minting-their-own-defect record, and the
  datum is that self-measurement settled neither round.**
- **A helper this lane needed and did not put in `lib.sh`.** `selftest_passes`
  — the negative twin of `gate_selftest_case` — is local to
  `bounds-allowlist.sh` and is named `bounds_selftest_passes`, deliberately
  gate-specific: a generically-named local definition is sourced *after*
  `lib.sh` and would silently shadow a promoted one. Every gate in the
  directory could use it — today the only fixture any of them proves *passes*
  is the empty clean tree, which says nothing about a spelling that must not
  fire. **RULED (orchestrator, 2026-08-20): F-g takes it, F-e keeps the
  gate-specific name.** F-g owns `lib.sh` for F3 and now also for S157/D101
  (the `errexit`-before-`gate_error` class, same harness), so it will have the
  file open; two lanes editing it for different reasons is the collision the
  sequencing exists to prevent. **The gate-specific name is what makes the
  handoff safe**, and it is in the tree at `bounds-allowlist.sh` beside the
  function so F-g inherits it from the code rather than from a message.
- **F-R15: GAP 4's mitigation was REFUTED by the verification pass and is out
  of the tree.** `rustfmt --edition 2021` rewrites the single-line
  `trait Bracket: CertifiedEnclosure where Self: Bounds` — which the lane's
  third matcher alternative *does* catch — into a multi-line `where` block
  that is **silent**, so the silent form is the formatter-stable one. GAP 4 is
  now disclosed as **open with no mitigation**, with the reason written at the
  gap: no line-based matcher reaches it, and a colon-free widening
  false-positives on `trait ArrivalSpec<T: CertifiedBounds>`, a sole bracket
  bound outside the class. **The lane published a false mitigation twice and
  had it caught by attack rather than by measurement** — the second time after
  it had already self-caught the first. Recorded because that is the datum:
  self-measurement did not settle a claim of this shape either time, and the
  thing that settled it was a compiled counterexample plus a formatter run.
- **S158/D102 and S159/D103 recorded, not closed**, per the ruling. S158
  subsumes S59 — the gate anchors on `+`, and `+` is one of several ways Rust
  writes a compound bound; `where T: Decide, T: Bounds` is silent with no
  alias in sight. S159 is the allowlist's file granularity against its
  per-seam justifications. Neither has a live instance in an unratified file
  today. **A taker of D102 should expect F-R6's grandfathering caveat to be
  live on a real residue**, unlike this lane's empty one.
- **The dead `\b` in `(\b\w*Bounds\s*\+)` removed**, symmetric with the dead
  path group: the tree-wide hit set is identical with and without it.
- **Header now 204 lines against 131 at open** (131 → 157 → 195 → 204), and the
  growth is the argument recorded at S116(m): every line past the fix is a
  blind spot named, a false claim retracted, or a repair the next reader is
  told not to make. **Placed as D106** — *split the ratification ledger out of
  `scripts/gates/`'s scripts* — with the progression and the reasoning written
  into the row so its taker inherits both. **Not F-e's to execute** (one row,
  one review, one verification, two fix passes) and **sequenced after F-g**,
  which owns `lib.sh` and will have the harness open. The property D106 must
  preserve is the one that made #791 recoverable: **the argument and the
  enforcement have to fail together**, so a ledger entry with no matching
  allowlist line — or the reverse — is itself a red.
- **The third matcher alternative stays, on the orchestrator's condition**:
  it catches a real single-line spelling and reds nothing, and the rustfmt
  fact sits **adjacent to it** in the header, so a reader who sees it fire
  learns in the same breath that the neighbouring multi-line form is silent.
  That adjacency is the difference between a partial catch and false comfort.

### F6 (S73 parts 1 and 3) — **CLEARED 2026-08-20**, the track's first

**It cleared on a deletion**, which F-R14 had pre-authorised. The row that failed
twice is gone; what replaces it is Q6's written reason living on the constants —
the adjacent-sample table (321: 5.88%, 322: 3.64%, **323: 5.24%**, 324: 1.79%,
325: 3.94%), the non-convergence at 2,000, and `323` named as the witness. **It
carries the fact rather than the conclusion.** S73 part 3's actual subject,
`GROWTH_TOLERANCE`, is boxed to `[1.04, 1.06)` from 1.889× wide and was never in
question; what died was the class-check member, with its reason written where the
constants live.

**Route: two failed instruments, one deletion, one row placed.** That is not a
failure of the lane — every figure it reported reproduced under adversarial
re-derivation, twice. It is what happens when a finding asks for a guard on a
quantity that cannot carry one, and the honest end state took two attempts to
reach because *"no honest box exists"* is the answer nobody reaches first.

### D105 / S160 — the proposal that came out of it

**Pin the continuous objective, not the cell count.** The discontinuity is
*entirely* the two `ceil`s: the unceiled worst excess moves by **hundredths** of
a point across the same neighbours (321: 0.017%, 322: 0.083%, 323: 0.011%, 324:
0.030%) against ~4 points for the ceiled quantity, and falls smoothly with
resolution (65: 1.82%, 200: 0.096%, 400: 0.0069%, 1,000: 0.0034%). It is
continuous because it is a sampled minimum of a smooth function of `log t` with
no rounding, so it depends only on the sampling step and the range — *exactly
what these two constants set*. It reds on both failure modes with one number.

**The validation is the part that convinces, and it is not an argument:**
**`D = 40` → 1.82% is identical to `S = 65`, which has the same sampling step.**
That equality is evidence the quantity measures **resolution** rather than
lattice luck.

Placed rather than landed: no third instrument in a row that has shipped two, it
needs a parameterization of code the brief marked read-only, and it deserves its
own review rather than riding a clearance. **The evidence is written into the
row, which is the thing that was at risk.**

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
