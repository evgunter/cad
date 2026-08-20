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
| **F-R4** | **F7's members live in six crates' `tests/`, and Track E's open #763 rewrites `crates/*/tests/all.rs` in nine of them.** | **F7 opens, and does not delete test files.** Editing a member's body is disjoint from `all.rs`; *removing* one is not, because the aggregation module names it. Where F7's sort concludes a member should be deleted rather than repaired, the lane records the conclusion and leaves the deletion to a follow-up row — it does not take `all.rs` out from under #763. | orchestrator, 2026-08-20 |

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

**Wave 1 — open now.** These three share no file with each other, with
Track C's open lanes, or with Track E's.

| lane | row | scope | review | state |
|---|---|---|---|---|
| **F-a** | **F5** (S92) | `topo/src/review_m1_pr5_internal.rs`, `topo/src/pcurves.rs` | style | — |
| **F-b** | **F6** (S73 parts 1 and 3) | `tools/tess-lint/` | style | — |
| **F-c** | **F7** (S110, sort first) | six crates' `tests/`, `memories/test-suite-cost.md` | style | — |

**Wave 2 — gated, and on what.**

| lane | row | gated on | why |
|---|---|---|---|
| **F-d** | **F4** (S76, S78, S84, S91) | Track C's **#734** | F-R2 — file overlap at `geom-brep/tests/m5_pr7_ssi.rs` |
| **F-e** | **F1** (S59) | Track E's **#753** | `scripts/gates/`, and the widened matcher's own conversions |
| **F-f** | **F2** (S61/S62 + D58–D60) | Track E's **#753** | the same two files, plus `ci-local.sh` |
| **F-g** | **F3** (S63) | Track E's **#753** | `scripts/gates/`, `scripts/ci-filter.py` |
| **F-h** | **F8** (D44, D45) | Track E's **#753** | F-R1 — the invocation lives in `ci.yml` |

**Sequencing inside wave 2.** F-e (F1) lands before **G4/S87–S88**, per
Evan's S87/S88 ruling: the sentence that makes the `CertifiedBounds`
conversion safe is *currently false*, and converting first would leave
the ratification requirement unenforced at exactly the moment new code
starts relying on it. Track G is not this track's, but the ordering
constraint is, and it is stated here so a Track G taker can read it.

---

## Reviews

*(none yet)*

## Landings

*(none yet)*

## Incidents

*(none yet)*
