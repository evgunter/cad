# SMELL-SCAN Track E — orchestrator log

**Constituted 2026-08-20.** Track E owns what was left of the smell-scan
register once Track A, Track B and Track D completed and Track C claimed its
own rows. The schedule itself lives in `docs/SMELL-SCAN-2026-08.md` §D, under
**Track E**; this file is the orchestrator's record — rulings, lane state,
review outcomes and incidents. **Live status is here and in §D, never in
`memories/`.**

**What this track is not.** It runs **beside** the model A/B experiment, not
inside it. No dispatch here is an A/B row, no result of it is comparable with
one, and **no lane on this track reads or edits `docs/MODEL-AB-LOG.md`**. The
experiment is paused on a model limit; the cheapest guarantee that pausing it
stays clean is that this track never touches the file. A lane that thinks it
needs to touch it is wrong and should ask.

---

## Review policy for this track

**Style review only, except at three rows.** The default is
`docs/prompts/reviewer-style-lane.md` (dispatcher notes:
`docs/REVIEW-STYLE-DISPATCH.md` — read once, then point by path, never paste).

**ADVERSARIAL additionally at D21, D25 and D27**, and at **D31** only if the
union's sort order turns out to be load-bearing. Those are the rows where a
wrong answer is *reachable*: D21 and D25 convert garbage-out into panics across
`crates/topo`'s Euler surgery surface — where #720's own per-site standard was
falsified across roughly half its sites — and D27 puts a newtype between a
refusal and a panic in the crate whose D9 rule is *never a panic*. Everywhere
else the risk is that the fix is incomplete or that a better fix existed, which
is the style lane's question and not the falsification lane's.

**The two questions this track adds to the style brief**, on every dispatch:

1. Is the finding's **original** stylistic problem now *completely* gone — not
   narrowed, not relocated, not half-closed in a way that reads as closed
   (§C13)?
2. Was it closed in the **best** way available, or merely in a way that
   compiles?

Both are phrased to require taste. Do not let them become checklist ticks —
§REVIEW-STYLE-DISPATCH §1 is the standing warning, and it binds the dispatcher
harder than the reviewer.

---

## Recording convention

**Each unit records its own completion in its own PR.** At the finding: a
bolded `FIXED by #NNN` lead, and the **original problem statement removed** —
version control keeps it, and a closed finding that still carries its problem
statement reads as open. The row is struck from §D's Track E table in the same
PR.

Every Track E PR therefore edits `docs/SMELL-SCAN-2026-08.md`, and they conflict
with each other by construction. **Merge one at a time**, and re-merge
`origin/main` into every open lane whenever one lands. A PR that goes
CONFLICTING runs *no* check runs at all — it looks like CI is absent rather than
failing (`memories/agent-lane-operations.md`).

**Row numbers are global and assigned centrally.** A lane that finds something
its own PR cannot carry does not invent a number: it asks, and gets the next
unassigned one in the D-sequence, which §D's *"No row number is reserved"*
paragraph owns. Three lanes collided on this once already.

---

## The state this track was handed, verified against the tree

Checked at `23b830e` (main, 2026-08-20), not transcribed from the records —
Track C's own §C-log lesson is that *a row minted from a review finding is
re-derived against the tree before it is written*.

- **Open PRs: five, all Track C's** — #731, #732, #734, #737, #738. Nothing on
  any other track is in flight.
- **Track A is complete.** A2 merged as #714 (residues → C11), A3's #678 landed
  as #684, A4's #667 landed as #683 and is **closed** — its remainder is #681,
  which had no owner and is now **E-l**.
- **Track D's live table had 16 rows, all unstarted.** Fourteen came here; D30
  and D32 went to Track C because each is a Track C lane's whole file.
- **Track C's row C8 has no lane** in that track's roster
  (`docs/SMELL-C-LOG.md`) — it was parked as *"fold into whoever next opens
  `step-import`"* and nobody has. Taken as **E-m**.
- **A doc defect, fixed while lifting the table**: §D's live-rows table carried
  **two `D24` rows**, the second (#748's, with `BlendArm::name` as the class's
  second witness) appended as a separate line while the first (#735's, one
  witness) was concatenated onto the end of the `D23` row — so the D23 row
  rendered with eight fields and swallowed a stale duplicate. The stale copy is
  gone; the current one stands.

---

## Container facts

This orchestrator runs in a Claude-Code-on-the-web container, not on the box
`memories/agent-lane-operations.md` was written for. What differs:

- ~~**No `ssh`**, so `local-scripts/new-lane.sh` cannot run~~ — **stale, and it steered a lane wrong.** The script derives its URL from `origin` and falls back to HTTPS, so it runs here fine, and it is the required way to make a lane (E-R6). Note `$HOME` is `/root`, so lane clones land at `/root/.local/share/cad-work/<lane>/cad`, not under `/home/user`.
  Lanes clone the orchestrator checkout locally and repoint `origin` at HTTPS,
  which is the credentialed path here. This is a container fact, not a repo one;
  the committed script is right for the environment it was written for.
- **`~/.local/share/cad-work/` starts empty**, and nothing in it survives the
  container. *A pointer at an uncommitted file in a container home directory is
  a pointer at nothing, one preemption later* — review artefacts worth surviving
  go in a commit.
- **4 CPUs, ~15 GB RAM, ~29 GB writable.** A lane's `target/` runs 4–8 GB, so
  three or four concurrent build-carrying lanes is the ceiling here, not the
  ten the wave layout would otherwise allow. Doc-only lanes are free.
- **Hosted CI is the gate and the cheap option** (`memories/local-battery-scope.md`).
  It is also the only place a timing means anything, which is why **E-n (D20)
  closes on an attribution measured there** and not in this container.

---

## Lane roster

Letters are Track E lanes; the row IDs are §D's and did not change. Gates are
file-overlap and instrument-ordering gates — the dependency structure was
discharged before this track existed.

| lane | row(s) | scope | gate | review | state |
|---|---|---|---|---|---|
| **E-a** | D22 + D34 | `scripts/gates/`, `.github/workflows/ci.yml` | none | style | **DONE — #753 merged** |
| **E-b** | D23 | `docs/` + suite headers; code set is what the re-derivation finds | none | style | **#763, green and re-merged** — awaiting its rerun |
| **E-c** | D26 | `docs/SMELL-SCAN-2026-08.md` §D and §S19 | none | style | **DONE — #752 merged**; discharged into D36–D39, plus D47/D48 from its review, all unstaffed |
| **E-d** | D33 | `docs/predicate-dimension-audit.md` | none | style | **#761, in review** |
| **E-e** | D28 + issue #693 | `editor-core/src/eval/` | — | style | **DONE — #767 merged** |
| **E-f** | D25 | `topo/src/euler.rs` and every `link_half_edges` caller | none | **ADVERSARIAL** | **#755, in review** |
| **E-g** | D27, then D29 | `sweep/src/fillet/{build,surgery,mod}.rs` | none | **ADVERSARIAL** (D27), style (D29) | **DONE — #768 and #777 merged** |
| **E-h** | D21 | `topo/src/{split,attach,movefac,revert}.rs`, `splitting/finish.rs`, `boolean/combine.rs` | **E-f, for file overlap on `split.rs`** — see E-R4 | **ADVERSARIAL** | unstarted |
| **E-i** | D24 | `Cargo.toml` workspace lints, or `.github/workflows/` | none | style | unstarted |
| **E-j** | D31 | `sweep/src/skin.rs`, `geom/src/curves/fit.rs`, home in `geom-core/src/spline/algebra.rs` | **Track C (C-l, C-g)** | style, escalates if the sort order is load-bearing | unstarted |
| **E-k** | D35 | `docs/DESIGN.md`'s D2 addendum, and whatever the answer names | ~~**E-g**, **E-h**~~ — both landed | style | **DISPATCHED** 2026-08-20, under row 0 |
| **E-l** | #681 | everything outside `crates/*/src`, **less the six surfaces F and G own** | none | style | **DISPATCHED** 2026-08-20 |
| **E-m** | #711 | `step-import/src/recognize.rs` | **BLOCKED by D86** (now on main, Track F's) | style | **#784** open, red on infrastructure only. Placed D86, D87, D93 |
| **E-n** | D20 | `topo/src/seqgen.rs` | none | style; closes on an attribution off hosted CI | unstarted |

**Not taken by Track E:** D30 and D32 (Track C's files — C-m, C-q); C11's #726
and #727 (Track A's residues, in `mesh/` and `props/`, which are C-k's and C-m's
scopes); L1, L2 and L3, which are deliberately last and stay that way.

---

## Incidents and corrections

### E-R4 — a gate whose stated reason is refuted is not a gate that has fallen (2026-08-20)

**E-f (#755) refuted the reason I gave for E-h's gate, and then removed the
gate.** The register said D21's half-edge sites *"inherit D25 for free"*, so
sequencing D21 after D25 would discharge them structurally rather than site by
site. E-f checked it against the tree and it is **false**: not one of D21's 14
sites is a half-edge key handed to a splice — three in `split_edge` are an edge
and two vertices, `attach.rs`'s are a face and an edge, `movefac.rs`'s a shell,
a face and a solid, and the one half-edge-arena lookup on the list
(`revert.rs:206`) is a field write into a clone of the arena it iterates, not a
splice. That correction is right, it is exactly what a lane is for, and it
stands.

**But the gate survives its reason.** Track E's gates are **file-overlap and
instrument-ordering** gates, not dependency gates — §D's edge list says so in
terms — and #755 rewrites `crates/topo/src/split.rs`, where D21's file set
opens. Two lanes rewriting one function collide whether or not either inherits
anything from the other. The lane was asked to record **both** claims: that D21
inherits nothing (its finding, in its words) and that E-h still sequences after
E-f for file overlap, which is a different and weaker claim than the one it
removed.

*Generalisable, and it is the dispatcher's error as much as the lane's:* **I
wrote a gate down with one reason attached, and the reason was load-bearing for
the gate's survival in a way I did not intend.** A gate stated as *"X because
Y"* invites a lane that disproves Y to delete X. State the mechanism a gate
protects, not the story you happen to have for it — and when a lane refutes the
story, the dispatcher re-derives the gate rather than accepting its removal.

**Corrected in `147e3a4`**, and the lane's own account of how it happened is the
sharper half: *"I read 'the gates are not dependency gates' in the roster, found
the dependency was fictional, and deleted the gate rather than checking whether
the gate rested on the dependency at all. The file overlap was visible in my own
diff — `split.rs` is on both scopes — and I reported the line numbers moving
inside `split_edge` as a **courtesy to E-h** in the same breath as cutting the
edge that would have kept E-h from colliding with it."* The evidence against the
deletion was in the deleting lane's hands, in the same paragraph. That is not
carelessness; it is what happens when a refutation and its consequence are
reasoned about separately, and it is why a gate removal is the dispatcher's call
and not a lane's.

**One byproduct worth carrying, for E-a.** This unit's `doc-gate.sh` red — a
deletion orphaning a rustdoc intra-doc link, invisible to both `cargo build` and
`clippy -p topo --lib` — is a **third** instance of the class D34's third
decision turns on, after #740 and #744, and the first whose shape is *a lane
creating the defect precisely by doing the right thing*. Forwarded to E-a as
evidence, explicitly not as an instruction: the row closes on a verdict either
way, and the cost side is that lane's to measure.

### A stale-claims sweep can write the false sentence: E-d's argument for re-deriving rather than striking (2026-08-20)

E-d found `docs/DESIGN.md:31` describing the predicate-dimension audit's open
findings as *"F2, F6's residue, F7–F15"* when the audit has **F6 and F7 both
RETIRED** — retired *together*, being one quantity three sites had handled three
ways. Asked whether the fix is a one-line strike or a re-derivation, it argued
**re-derive**, and the argument is worth keeping because it turns S39 on itself:

> S39's single non-benign row was a false sentence written **by a previous
> stale-claims sweep**, which replaced two honest sentences with one wrong one.

Striking `"F6's residue, F7"` makes the sentence true this week and leaves it
the same hand-copied enumeration with the same expiry. The honest form is a
**pointer plus a shape** — *"open findings live in that document's disposition
list; F2 and the arm-policy family F8–F11 are the standing ones"* — rather than
a range that must be retyped on every retirement.

It is also sharper than a plain miss: F6's residue did not vanish, the audit
**re-homed** it to issue #501 and says explicitly it is *"NOT this family"*. So
a reader following the index lands on a disposition that redirects them. Placed
as **D51**, with the verdict recorded and the taker left free to disagree.

### A second smell scan landed under three live lanes (2026-08-20)

The second scan — **S59–S116**, scan base `0714d540`, written by
another session and merged mid-wave. **It has since been folded into
`docs/SMELL-SCAN-2026-08.md`**; the separate file is deleted and recorded in
`docs/DOC-LEDGER.md`. REPORT ONLY, no verdicts, **no schedule**.
Its ID space continues the first scan's on purpose, so a citation never means
two things; `S45`–`S48` stay reserved in the first document.

**Its higher-yield half is a fix audit** — code rewritten *in response to* the
first scan, read as a diff against `4258584`, looking for what the fixes
introduced. That is Track D's and Track C's output, and now Track E's. Its
recurring outcome, in its own words: *a fix pass touching a file is a fix pass
with the file open, and it swept the reported instance and left the sibling.*

**Where it lands on Track E, and what was done about it:**

| finding | lands on | action |
|---|---|---|
| **S61** `[verified]` | **E-a**, both rows. The whole `discipline` job is gated on `run_build`, and `_is_docs` is true for any `.md` **or `local-scripts/`** path — so all 14 gates skip on a docs-tier change. `probe-suite-census.sh`'s citation half (D22's file) *cannot fire on the only change class that can break it*; `gate-roster.sh` (D34's file) argues it need not read `local-scripts/` **because** of that hole | forwarded to E-a mid-unit |
| **S62** | **E-a / D34.** Five gate-by-every-criterion checks outside `scripts/gates/`, hand-named in both halves — including the two D34 asks about, plus two `demos/` scripts D34's `ci.yml` enumeration never surfaced. And the **executable bit is the registration mechanism**: a gate at mode `0644` is invisible to both halves | forwarded |
| **S63** | possibly **E-a**. Three of six grep gates pass the spellings they forbid | forwarded to judge |
| **S68** `[verified]` | **E-h / D21**, independently. *The W2c discard sweep stopped inside the function it was editing* — `split_edge`'s mutation phase took two `unreachable!` conversions in the same diff, ten lines above three silent discards. Its framing is the better one: **`split_edge` is not a sibling, it is the same call** | fold into E-h's brief when dispatched |

**S60** reports S26 was never fixed; that is Track C's C5, not Track E's.

**The instruction to E-a is the part worth keeping.** Three fresh findings are
**not a licence to widen a unit** — D22 still closes on its decision and D34 on
a scope verdict plus at most one move. What they change is whether the *premise*
of a verdict still holds: a census whose citation half cannot fire on the change
class that breaks it is a different object than the row describes, and a
partition of *"which scripts can be roster gates"* reasons about a gate surface
that is itself conditionally inert. So: re-derive where the premise moved, say
in the PR body which of S61–S63 the verdict answers / leaves / changes, and ask
for a row for anything touched and not closed.

**And the general rule this makes concrete:** a Track E verdict that silently
overlaps an unverdicted finding in another document is the same
claim-travels-between-documents failure #752's review caught, one document
further out. The second scan has no schedule; that does not make its findings
absent, it makes them unowned.

### Tracks F and G exist, and F took part of Track E's territory (2026-08-20)

**#766 folded the second smell scan into the first document and carved Tracks F
and G out of it.** Three things follow for this track:

- **Track F has an orchestrator.** It owns the instrument surface — gates,
  `ci-filter.py`, the guards that cannot fire, the tests that cannot go red —
  and it is **gated on E-a (#753)**, which holds `scripts/gates/` and `ci.yml`
  until it lands. **D58, D59 and D60 were placed for Track F, not for here**,
  out of E-a's own re-derivation of S61 and S62. **D50** (the `Live`
  unforgeability gate) is `scripts/gates/` work and belongs to F for the same
  reason; it stays listed here only until F picks it up.
- **Track G is unstarted and edge-free.** `interval-transcendentals/`,
  `demos/`, `profile/` and `sweep/src/` outside `fillet/` — no track is live in
  any of them.
- **Every open Track E PR went CONFLICTING**, because #766 grew
  `docs/SMELL-SCAN-2026-08.md` by ~2,400 lines and every lane edits it. That is
  the cost of the merge, it was foreseeable, and it is worth stating plainly
  rather than letting four lanes each rediscover it: **a register that every
  lane edits makes a structural change to that register expensive in exactly
  proportion to how many lanes are live.** The alternative — leaving two
  documents — cost a silently-wrong C-number in both. The merge was still right.

**What Track E keeps:** the Euler-surgery rows the second scan added land here
rather than in F or G, because `topo`'s mutation surface is fenced under one
ADVERSARIAL policy and a second track on those files is the collision the
sequencing exists to prevent. S68 is D21 (E-h). S67, S69 and S70 are unplaced
and want rows after E-h lands — S70 is S14 and is Evan's.

### The dispatcher wrote a stale index inside the section warning about stale indexes (2026-08-20)

**#766 placed D49, D50 and D58–D60 and left §D's *"D48 is the highest one
placed"* sentence untouched.** E-d found it while re-merging and corrected it to
**D60**. That sentence is the placement register's own index — the one artefact
whose entire job is to say what the highest number is — and I broke it in the
same PR that added three new §D sections about findings travelling between
documents without being re-derived.

*Why it is worth a paragraph rather than a shrug:* it is **not** the same
mechanism as E-R4 or as #752's inherited-gate finding. Those were claims copied
from one place to another without checking. This was a claim I **did not touch
at all** — the defect is that a number-carrying sentence sits far from the table
whose contents determine it, so an edit to the table is not an edit to the
sentence and nothing connects them. That is C14's shape (*pins guard the
invariant as it was reachable then*) applied to prose, and D23's class exactly:
**an enumeration stated in prose, already drifted.**

The honest reading is that the register has now produced this defect twice in
one day, in two different sentences, and both were caught by a lane rather than
by the orchestrator who wrote them. **The instrument that works here is a lane
re-deriving on its way past** — which is what the standing header's rule 5
asks for, and it has now paid for itself four times on this track.

**E-d also re-measured a third time** at the new base after #755 gave `topo` a
`live.rs` and rewrote `split.rs`; all ten figures reproduced, and the file names
all three commits. That is the discipline D23 asks for, applied without being
asked, on a measurement nobody would have re-checked.

---

## The standing lane header

**Every Track E dispatch carries this by reference** — *"read
`docs/SMELL-E-LOG.md` § The standing lane header in full before you start"*. It
is committed rather than kept in a container home directory on purpose: a
pointer at an uncommitted file is a pointer at nothing, one preemption later.

**1. Read these, by path, before touching anything.**
`docs/prompts/implementer-discipline.md` in full; your row in
`docs/SMELL-SCAN-2026-08.md` §D Track E, and the **finding** it came from
(follow the `Was` column); `CLAUDE.md`; and `memories/MEMORY.md`'s index,
following the pointers your row actually touches.

**2. The A/B fence.** This track runs **beside** the model A/B experiment. Do
not read, cite, or edit `docs/MODEL-AB-LOG.md`, and do not describe your unit
as an A/B row anywhere. If something seems to require it, stop and ask the
orchestrator.

**A repo-wide grep is a read of every file in the repo.** A lane here had
`MODEL-AB-LOG.md` lines enter its context from an unexcluded `grep -rn`, and
handled it correctly: stopped, cited nothing, excluded it from every subsequent
search, and **reported it**. That is the whole obligation — the fence is against
reading and editing, not a tripwire, and an honest report costs nothing.
Exclude it in your search commands rather than relying on not looking.

**And check that your exclusion is the right syntax for the tool you are
running.** A second lane put the exclusion in the command, as instructed, and
still read the file: it used git-pathspec syntax — `:!path` — in a `grep -rn`.
**`grep` does not know that syntax, does not error on it, and does not warn.**
It silently excludes nothing, so the command *looks* fenced and is not. Use
`--exclude=MODEL-AB-LOG.md` (or `--exclude-dir`) for `grep`, and `:!path` only
where `git` is doing the matching. **An exclusion that fails open gives no
feedback**, which makes it strictly worse than no exclusion — with none you
know to be careful. Both lanes handled the aftermath correctly and neither
breach cost anything; the syntax is what cost the second one.

**3. Your branch and your lane.** Branch `smelle/<row>` (e.g. `smelle/d25`),
off current `origin/main`. Use **your own** `CARGO_TARGET_DIR` — never one
shared with another worktree, **and never one inside the session scratchpad.**
Put it in your own worktree. The scratchpad is shared between concurrently
running agents and it is on the same fixed per-session disk allowance as every
lane's build: six lanes each parking a 1–7 GB target directory there took this
container from 24 GB free to **6.4 GB** in under two hours, which is past the
point where a build can fail mid-link and leave torn binaries behind. Anything
to be **published** — a PR body, a review findings file — goes to a
**lane-private** path for the same reason plus a second one: another agent can
read and overwrite it.

**4. Commit and push at every coherent seam**, not at the end. Two lanes have
been lost mid-unit with everything uncommitted; a pushed commit is the only
state that survives.

**5. Re-derive; do not transcribe.** Your row was written from another lane's
record. Records are accurate about the moment they describe, and a fix pass may
have moved the tree since. **Check every citation in your brief and in your row
against the tree before you build on it** — five briefs on the sibling track
carried a citation that did not resolve, and one carried a claim that was simply
false on main. Checking rather than complying is the behaviour that is wanted;
say in your report what did not resolve.

**6. State what your sweep cannot match.** If your unit closes an instance of a
class, name the pattern and its blind spot. A path glob is part of the pattern.
A count is not a deliverable; a sweep with a stated blind spot is.

**And never read a truncated result as a negative one.** A lane here swept for
the strings its change would break, **found the breaking site as hit #2 of 37**,
piped the grep through `head -10`, and read the truncated list as complete —
shipping a change that reddened three CI shards on an assertion its own sweep had
located. The pattern was fine. *Truncation is worse than a weak pattern, because
it looks like a finished sweep*: a weak pattern's blind spot gets disclosed, and
a truncated one's does not exist to disclose. If you cap output, say so and say
the total.

**7. Merge `origin/main` immediately before opening the PR, and re-merge
whenever main moves while it is open.** A PR that goes CONFLICTING runs **no**
check runs at all — it reads as CI absent, not CI failing. After any push,
confirm checks actually **started** by reading the workflow *runs* list.

**And report a run by how many of its jobs FINISHED, not by how many were
green.** *"22 of 26 green"* is true and unfalsifiable at once: nothing in it
distinguishes *four still pending* from *all done*. One lane reported exactly
that and three shards failed eight minutes later; on another run the last job
finished **fourteen minutes** after the first. **Three instrument traps, all
observed here:** a jobs listing saying `unfinished: []` can mean *not yet
spawned* rather than *done*; the listing endpoint returned **30 rows while
reporting `total_count: 37` in the same response** — the two disagreed, and only
`get_check_runs` returned all 37; and, the sharpest, **the `test (eps = …)`
shards do not exist until `build + archive` finishes.** One run's job count went
**26 → 32 → 37**, the shards spawning eleven minutes in. So an early snapshot is
not merely incomplete — **it is a count of a list that does not yet contain the
jobs which catch test failures**, which is precisely the class of failure a
green count is being offered as evidence against.

**The rule those traps all reduce to: a job count is a denominator only once
the graph is fully expanded, and on this workflow the fan-out is
artifact-gated — so `M` is not final until the `build + archive` pair is
green.** Any *"N of M"* taken before that is measured against a denominator
that has not stopped moving, and the check-runs endpoint reports the partial
board without flagging it as partial. Reporting *"N of M finished"* is
therefore not sufficient discipline here: **say whether `M` is final.** The
lane that wrote this paragraph reported *"25 of 26 finished, 0 failed"* while
holding the rule above — its tally was not wrong about what it counted, it was
wrong about what there was to count; the board went 26 → 36 the moment the
builds landed. **The one report that needs no denominator is the run's own
`conclusion`** — take that, and no tally is load-bearing.

**And the re-merge is not only conflict avoidance — it re-derives the fence
set.** E-l was fenced out of `mesh/tests/revolves.rs` because Track C's #803
held it. #803 merged mid-lane, the fence lifted, and the lane found that out by
re-merging `main` and re-deriving its own scope rather than by trusting the
scope sentence it had been given. **A fence is a fact about a moment**, and a
long-running lane's fence list rots exactly like the enumerations D23 is about.
Re-derive it when you re-merge; a lane that skips a leg on a fence that has
since lifted has under-swept and will report a clean negative for it.

**8. Record your completion at the finding, in your own PR.** A bolded
`FIXED by #NNN` lead at the finding in `docs/SMELL-SCAN-2026-08.md`, the
**original problem statement removed** (version control keeps it), and your row
struck from §D's Track E table. Every Track E PR edits that file, so expect to
re-merge; merges are serialized by the orchestrator.

**9. If your unit finds something it cannot carry, it is a row, not a
footnote.** Ask the orchestrator for a number — do not invent one, and do not
leave it in the PR body. *A verdict is not a placement* (§D ordering rule 4).

**10. Your report is a claim site too.** ≤150 lines. Name the questions you
actually exercised, what you could not check, and anything in this header or in
your brief that turned out to be wrong. Reviewers correcting the dispatcher is a
working lane, not a malfunction.

### Evan's ruling on #777: representability becomes **row 0** (2026-08-20)

E-g's D27 dissolved `FilletError::EmptyChain` structurally rather than
classifying it, which retired the D2 addendum's *"one state this taxonomy does
not contain"* paragraph. I split the factual retraction (kept in #768) from the
generalisation it suggested (lifted into **#777**, a design-conversation PR left
open for Evan, which argued the three strongest cases **against** itself).

**Evan ruled bigger than the PR asked:**

> the sentiment is great — so great that i think it should be promoted from an
> addendum to **row 0** — *can this error state be made unrepresentable?* —
> **better than all other resolutions if possible**

So it is not a paragraph beside the classification. **It is the first question
asked of any state, ahead of rows 1–5, and when the answer is yes that is the
answer** — the preferred resolution rather than one of six.

**Why "row 0" and not "row 6" is the whole ruling.** Rows 1–5 classify a state
that exists; row 0 asks whether it should exist at all. It is answered *before*
the classification begins, so it renumbers nothing and is not a sixth bucket. A
lane that reaches row 1 has already answered row 0, and a lane filing a state
under any row owes the reason row 0 did not apply.

**Three consequences, and the third is the long one.**

*The counter-argument the PR made against itself is now more load-bearing, not
less.* **"Only if it cannot" is a judgement with no cost threshold** — readable
as licensing type churn to dodge a classification. That mattered when the rule
sat beside the alternatives; it matters more now it outranks them. The rule owes
a statement of what *"if possible"* excludes, and the calibration already exists
at both ends: `EmptyChain` dissolved for a private field and a constructor
signature, while **#755 rejected a generative brand for `Live` because a `Body`
lifetime would infect every public signature in the workspace.**

*That rejection is row 0 being answered "no" in the field, before row 0
existed.* It is the best available evidence that the rule describes what careful
lanes already do rather than inventing an obligation.

***S14's first question changes.*** It has been *"which row does the graft's
partially-written destination fall under, and does the no-panic principle need
amending"*. Under row 0 it becomes **"can `graft_disjoint_all_keyed` be
restructured so a partially-written destination is unrepresentable?"** — staging
into a fresh body and committing on success, which is the shape
`merge_faces.rs:468` already uses and the shape D27 itself used. **That reframes
S14 and does not answer it**; the cost of the restructuring is exactly the
*"if possible"* judgement above, and S14 stays Evan's. It has reach: **#740 left
46 lookup sites typed rather than converted because S14 is open**, so anything
that moves S14 moves them.

**#777 is now a ratified-decision PR rather than a proposal** and self-merges
once written and green.

### A repo-wide CI outage looks exactly like a per-branch failure (2026-08-20)

**The CI budget ran out and every run failed within seconds**, unbroken across
ten branches on four tracks. The signature: the first job, `change filter`,
completes in 2–4 seconds with `steps: []`, `runner_id: 0`, `runner_name: ""`,
and everything else skips behind it — a job that was never assigned a runner,
not a job that ran and failed.

**`main` is the discriminator, and it is cheap.** Main's own push run carried
the identical signature, which settles that no branch's diff is responsible
without reading a single log. Two other checks cost nothing and were worth
doing anyway: the failing job's `runner_id`, and re-running the failed step
locally — `scripts/ci-filter.py --base origin/main` returned `TIER=docs`, exit
0, on exactly the diff CI could not classify.

**The mode also progressed**, which is the part that would have misled a lane
diagnosing one branch: earlier in the outage `change filter` still ran and the
*test shards* died 1–2 seconds in, which reads as a test failure. Only later
did the first job stop getting a runner at all. **The same outage presents as
two different defects depending on when you look at it.**

**There was no lever.** `rerun-failed-jobs` and `rerun` are both 403 for this
token, and the discipline forbids an empty commit to kick CI. So the sequence
is: diagnose against `main`, then wait for a real reason to push — a re-merge
when `main` next moves, which a long-running branch owes anyway. Waiting is not
inaction here; it is the only honest trigger.

### E-R9 — the standing lane header was not on `main` (2026-08-20)

**Every Track E brief says *"read `docs/SMELL-E-LOG.md` § The standing lane
header in full before you start."* Lanes clone `main`. This file was last on
`main` at 18:56 (#794).** So every rule added after that — rule 7's denominator
rule, rule 2's repo-wide-grep clause, the corrected container facts, E-R6, E-R7,
E-R8, and the allocator bump — existed only on the orchestrator's branch, and
every lane dispatched since read an eight-hour-old header while being told it
was binding.

**It surfaced through the allocator.** Lane E-q reported the reservations table
reading *"Next unassigned: **D94**"* against a brief that said **D97**, flagged
the discrepancy, and did not edit the file because it is the orchestrator's.
Had a lane instead resolved the conflict the way the register tells it to —
take the log's number — it would have minted a row three below the true
frontier, which is the four-orchestrator collision again with one allocator
instead of four.

**This is E-R5 turned on its author.** That rule says a row is not placed until
it is on `main`; E-R8 adds that being on `main` is not being received. This is
the third face: **the document that carries the rules was itself subject to the
rules, and the orchestrator was the one party never checking.** The header's
own preamble says it is committed rather than kept in a container home
directory *"on purpose: a pointer at an uncommitted file is a pointer at
nothing, one preemption later."* Committed to a branch is not published, and
the sentence explaining why did not save the file it was written in.

**Standing correction: the orchestrator log lands on `main` at every pipeline
seam** — after each merge it records — not when the session ends. A lane cannot
read a branch it does not have.

### E-R8 — a row on another track's TABLE is not a handoff (2026-08-20)

**D86 blocked Track E's #784 for hours while both tracks believed the other
owned it.** Track E found it, could not fix it (`scripts/` is Track F's), and
placed it on **Track F's table** in the shared register as BLOCKING — #794,
merged. Track F's lane F-f then read #794 and wrote into #798's body that *"the
script itself is Track E's (#794 open)"*, concluding the opposite ownership.
Nobody scheduled it.

**The mechanism: `docs/SMELL-F-LOG.md` has zero mentions of D86.** Each track's
orchestrator works from its own log; the register is where rows *live*, not
where a track *looks* for its work. So a row written onto another track's table
lands in a document that track reads for reference and not for assignment —
**visible, durable, correct, and unread.**

**This is E-R5 one level over.** E-R5: a row is not placed until it is on
`main`. E-R8: **being on `main` is not being received.** Placement makes a row
true; a handoff needs an *addressee*, and the addressee is the other track's
log or its orchestrator, never the register alone. Where the receiving track's
session is unreachable — as Track F's is from here — the handoff is not
complete until a human routes it, and **the honest state until then is
"unrouted", not "handed off"**.

**And it compounded**: F-f's lane read the placement PR and derived ownership
from it, which was reasonable — #794 *is* Track E's PR, and a docs-only PR that
places a row looks exactly like a docs-only PR that fixes one. **A placement and
a fix are indistinguishable from the outside of a merged docs PR**, so a
placement should say, in the row itself, who is expected to act.

Evan ruled Track E takes D86 (lane **E-o**), crossing into `scripts/` with the
crossing stated, after both `SMELL-F-LOG.md` and `SMELL-G-LOG.md` were checked
for a claim on the file and neither had one.

### E-R7 — a green PR does not absorb new work at merge time (2026-08-20)

**E-l found twelve copies of one build-cost measurement across the
`crates/*/tests/all.rs` headers and filed it as #808 rather than fixing it,
because all thirteen files were in #763's head** — and #763 was already deleting
a restated *suite count* from the same paragraphs, on reasoning that transfers
verbatim. The lane put the merge-order question to me rather than answering it,
which was right.

**Ruling: #808 does not go into #763.** #763 had by then been through a full
review, a withdrawn row, two register conflict resolutions and three CI runs.
Folding thirteen unreviewed file edits into a green head at merge time buys a
tidier diff and spends the one thing that made the head trustworthy. **The
cheapest place to land a change is not the same as the right place**, and
"cheapest" was the only argument for the fold. #808 stands free the moment #763
merges, and its fence disappears with it.

The general form: **a PR that is green and reviewed is a finished artifact, not
an open branch.** Work discovered against it goes beside it.

### E-R6 — "work in your own lane" is not a dispatch instruction (2026-08-20)

**E-k and E-l were dispatched in the same turn and both checked out a branch in
`/home/user/cad`, the shared main checkout.** E-l's `smelle/681` switched E-k's
`smelle/d35` out from under it; an orchestrator commit written between the two
checkouts landed on `smelle/d35` instead of the orchestrator branch, and was
recovered from the reflog. Nothing was lost, and only because the window was
minutes wide.

**The brief is what failed, not the lanes.** Both briefs said *"work in your own
lane"* and gave a private `CARGO_TARGET_DIR` — which reads as a rule about
**disk**, and both lanes obeyed exactly that rule. Neither said how a lane is
made. `memories/agent-lane-operations.md` has the answer
(`local-scripts/new-lane.sh <lane> <branch>`, which also sets `core.hooksPath`
so the committed pre-push fmt hook is live), and the standing header's rule 1
points at `memories/MEMORY.md`'s index — but a pointer two hops from the
instruction is not an instruction. **Every dispatch now names the clone command
and its path, in the brief.**

**The orchestrator was the third party in the same tree**, which is why this cost
a commit rather than merely a branch switch: the orchestrator now works from its
own worktree too. *A working tree has one branch, so it has one occupant* — the
shared checkout is for reading `main`, and nothing else.

**Note the shape.** The private-target-dir rule exists because six lanes filled
the disk; it was written as the fix to *that* incident and inherited none of the
lane-creation rule it sits beside. **A rule extracted from an incident carries
that incident's scope, not the scope of the thing it is a rule about** — which is
S15's finding, arriving in the dispatch process rather than in the kernel.

### E-R5 — a row is not placed until it is on `main` (2026-08-20)

**Twice in one session the orchestrator wrote a row, told a lane it existed, and
left it on an unmerged branch.** First the `D81`–`D100` block, which lane E-h
caught by reading `main` and finding the sentence absent. Then **D86**, the
blocking CI defect, which was handed to Track F in a message and in a §D edit
that Track F could not see — while the lane whose PR it blocks was told to stop
waiting on it.

**This is the same defect as the number collision it was recording.** A fact
asserted about a branch nobody else can read is not a fact about the register.
The orchestrator wrote that sentence about four allocators and then made the
error twice more in its own voice.

**The rule: a placement is a `main` commit, not a branch commit.** Concretely —
when a row is minted for another track, or when a lane is told a number is
reserved, the state-sync PR goes out **in the same turn**, not at the next
convenient seam. `memories/orchestration-model.md` already says the orchestrator
branch must not accumulate a large unmerged delta and to open a docs-only PR *at
every pipeline seam*; **placing a row for someone else is a seam**, and that is
the part that was not obvious enough to stop it happening twice.

*Why it kept happening:* the log and the register live in the same branch as the
running commentary, so a row and a paragraph of session narrative are the same
kind of edit to make and only one of them is urgent. **The urgency is not
visible in the diff** — which is exactly the property that makes the failure
repeatable, and why it needs a rule rather than more care.

---

## Rulings made in this track

| # | Ruling |
|---|---|
| **E-R1** | **Row numbers do not get renumbered by a change of track.** Merged PR bodies cite `D21`, `D27`, `D35`; a Track-E renumbering would fork the one register for a cosmetic gain. Track letters name *ownership*; row numbers name *placements*, and the two are not the same axis. |
| **E-R2** | **A track with no orchestrator is not a schedule.** Track D's rows were correct, placed and edge-free, and would still have been unstarted a week later. The audit that produced Track D found this once already (§C3); this track is the same finding applied one level up — a *track* that does not execute is not a register either. |
| **E-R3** | **A row whose files belong to a live lane goes to that lane, not to a second track.** D30 → C-m and D32 → C-q. The alternative is two tracks editing one file with no shared orchestrator, which is the collision the whole sequencing exists to prevent. If Track C declines, the row returns here — never to nobody. |

---

## Row-number reservations

Blocks handed to lanes so they do not round-trip for a number mid-unit. A lane
uses only what it needs and hands the rest back; anything past its block comes
from the orchestrator. **D35 was the highest number placed when Track E was
constituted.**

| block | lane | state |
|---|---|---|
| D36–D39 | E-c (D26) | all four used |
| D40–D41 | E-a (D22 + D34) | used |
| D42–D43 | E-f (D25) | returned unused |
| D44–D45 | E-b (D23) | used |
| D46, D51, D57 | E-d (D33) | used; **D56 handed back** with a measurement as the reason |
| D47–D48 | E-c's fix pass | used |
| D49–D50 | E-f's fix pass | used; **D50 is Track F's** by file |
| D52–D53 | E-g (D27 + D29) | returned unused — the work landed instead |
| D54 | E-e (D28) | used; **D55** returned |
| D58–D60 | **Track F**, via E-a | used |
| **D81–D100** | **Track E's own block**, claimed 2026-08-20 | see the reissue below |

**Reissued out of Track F's block, 2026-08-20.** Track E issued D61–D70 to five
lanes while `D61`–`D70` were **already reserved to Track F** and `D71`–`D80` to
Track G. Nothing was overwritten — neither new track had placed into its block
yet — and the double-allocation was caught by **lane E-h**, which read
`SMELL-F-LOG.md` before writing a row and withdrew the D61 it had already
placed. Reissued:

| was | now | lane |
|---|---|---|
| D67 (**placed in #767**) | **D81** | E-e — the `Debug`-at-a-composing-layer class |
| D63–D64 | **D82–D83** | E-a's fix pass |
| D65–D66 | **D84–D85** | E-b's fix pass |
| D69–D70 | **D86–D87** | E-m |
| D61–D62 | **D88–D89** | E-h — D88 is `merge_faces.rs:766`'s `unwrap_or_default` discard |

Next unassigned in Track E's block: **D100**, which is the block's LAST. (**D96** is E-k's — the thirteen row-0 candidates out of D35. **D97**, **D98** and **D99** are E-p's, out of S14: `from_algebra`'s do-nothing debug arm, `unit_segment`'s clamp and its false caller claim, and the `indexing_slicing` deferral that lost its revisit condition — **placed on `main` by #839**, deliberately split out of E-p's design PR because all three are true however S14 is decided and **E-R5 says a row is not placed until it is on `main`**. Leaving them on a branch that waits for Evan would have left three numbers reading as assigned here and free in the register.) **Anything past D100 comes to the orchestrator before it is written** — the block is exhausted, not extensible by whoever notices first, which is the whole point of E-R3.
re-issued — a number that has appeared in a lane's report, even as *unused*, is
cheaper to skip than to explain.

**D36–D39 were assigned by the orchestrator**, in E-c's dispatch, not minted by
the lane. #752's style review could not tell from outside and asked; recording
it here is the answer, and the fact that it was not visible from the PR is
itself worth knowing — a lane that follows the rule should be able to *show*
that it did.

**D36–D39 are placed and unstaffed**, and they are Track E's to schedule.
E-c's report says D37(a) and D39 want **one** lane, not two: both need a
fieldless `Copy + Eq` discriminant on `PathError`, which `pncad-py/src/tags.rs`
has already hand-written once. D37 is gated on **D28** (E-e). Schedule them
once #752 lands and the review has had its say about whether these four are
rows a taker can act on — which is one of the two things that review owes.

---

## In flight

**Wave 1 opened 2026-08-20**, three lanes, no shared file except
`docs/SMELL-SCAN-2026-08.md` — which every Track E PR edits, so merges are
serialized here and each lane re-merges `origin/main` when one lands.

| lane | row(s) | branch | PR | state |
|---|---|---|---|---|
| **E-c** | D26 | `smelle/d26` | **#752** | **MERGED 2026-08-20** — Track E's first landing |
| **E-a** | D22 + D34 | `smelle/d22-d34` | **#753** | **MERGED 2026-08-20** — 37/37 jobs finished, 36 success, 1 skipped, 0 failures. **Track F is unblocked** |
| **E-f** | D25 | `smelle/d25` | **#755** | **CLEARED by both lanes**; combined fix pass running (3 must-fix, 2 → rows D49/D50). Merges after #752 |
| **E-b** | D23 | `smelle/d23` | **#763** | fix pass complete — **D45 withdrawn**, a guard taken, final count 59/17/9. Awaiting its run |
| **E-d** | D33 | `smelle/d33` | **#761** | **MERGED 2026-08-20.** Placed D46, D51, D57; handed D56 back |
| **E-e** | D28 + #693 | `smelle/d28` | **#767** | **MERGED 2026-08-20** — 37/37 finished, 0 failed. Census 12 arms not 8; placed D54, D81 |
| **E-h** | D21 | `smelle/d21` | **#773** | **style lane NOT CLEARED** — 8 MAJOR; adversarial lane running. Placed D88, D89 |
| **E-k** | D35 | `smelle/d35` | **#809** | **MERGED 2026-08-20.** D35 closes on **(d)**; 103 sites re-derived, 76 one state, 13 row-0 candidates → **D96**, 3 messages fixed. Found **#777 never reached `main`** → **#817** |
| **E-l** | #681 | `smelle/681` | **#810** | **MERGED 2026-08-20.** 7 of 9 surfaces swept, 2 declared; 24 claims → 7 guarded, 2 scheduled, 13 unguardable-with-reason, **1 unguarded (#807)**. #808 stands free now that #763 is in; `memories/` raised as a tenth surface |
| **E-o** | D86 | `smelle/d86` | — | **dispatched** 2026-08-20. Crosses into `scripts/` with Evan's ruling, after F's and G's logs were checked for a claim on the file and neither had one (E-R8) |
| **E-p** | S14 | `smelle/s14`, `smelle/d97-d99` | **#823** (conversation, open for Evan); **#839** (D97–D99) | **dispatched** 2026-08-20. A design-conversation PR, not a fix; **waits for Evan** and never self-merges. Its three residues were **split into #839 off `main`** under E-R5 — they do not depend on how S14 is decided, so coupling them to it would have made them invisible until it was |
| **E-q** | `memories/` | `smelle/memories` | **#826** | **MERGED 2026-08-20.** 21 blocks → 17 keeps, 4 repointed; two drifted second-copies resolved; #681's `.md` instrument corrected |

**E-g dispatched 2026-08-20** (`smelle/d27-d29`), D27 then D29 — one lane
because both edit `sweep/src/fillet/`, and D27 first because its newtype may
dissolve part of D29's surface. Its brief carries the four things #755 learned
at cost about shipping an unforgeable token, so it does not re-learn them.

**E-e dispatched 2026-08-20** (`smelle/d28`), fenced off `sweep/` while E-g holds it and told to read C-f's head rather than trust my claim that `eval/` and `resolve/` are disjoint. E-h
waits on E-f (file overlap on `split.rs`, see E-R4), E-k on E-g and E-h, E-j on
a Track C confirmation. E-i, E-l, E-m are wave 2; E-n is last.

**Six rows are placed and unstaffed** and want lanes once capacity frees:
D36–D39, D47, D48 (all E-c's), D46 and D51 (E-d's), D49 and D50 (E-f's). Two
cheap groupings are already known: **D37(a) with D39** (one `PathError`
discriminant serves both) and **D50 after E-a** (it needs `scripts/gates/`).

---

## Reviews

### #752 (E-c / D26) — style lane, 2026-08-20: **CLEARED**, fix pass running

All seven claims held. The two questions this track adds both came back
positive, and the second is the one worth recording: **D36–D39 do not reproduce
the defect they close.** Each carries a deliverable, a closing condition, a
scope and a named constraint the taker hits first; D36 names its *decision*
rather than its edit; D37 and D39 each admit a written scope verdict as a pass.
None reads as *"someone should look at this"*, which is what D26 exists to stop.
The `tags.rs` refutation was judged the **best** available answer rather than a
compiling one, and its residue was preserved rather than dissolved.

**Eight style findings, and the shape they share is the review's real result.**
Three of the eight are *the finding recurring inside its own fix*:

- **A gate inherited as prose and re-published as verified.** D37's gate on D28
  said the payload is discarded *before any tag is computed*, so nothing in
  `tags.rs` can recover it. `node_error_tag` takes `&NodeErrorKind` — the typed
  value — and `Display` renders at a different call site. The sentence came
  verbatim from D28's row, and the PR presented repeating it as *"the
  re-derivation confirms it from the other side"*. **A restatement is not a
  check**, and this is the one thing a re-derivation lane was specifically there
  to catch.
- **A stated negative result that was false.** The lane declared it had checked
  for closure factories bound to another name and found none; `pcurve_cache.rs`
  has two, one of them fifteen lines from the sibling D36's whole asymmetry
  argument rests on, with eight call sites. It cost D36 a count.
- **Citations pointing at the enclosing construct** — in the PR whose deliverable
  was correcting exactly that defect in S19.

Two findings became rows (**D47**, **D48**); the rest are fixes in place.

*Generalisable:* **the class to watch is a claim that travels between documents
without being re-derived at each stop.** All three above are one mechanism —
prose is cheap to move and expensive to check, so it moves. §D's own rows are
now a place this happens, which is new: the register was built to stop findings
being lost, and it can lose their *accuracy* instead.

### #755 (E-f / D25) — style lane, 2026-08-20: **CLEARED**, one must-fix; adversarial lane still running

**Question 1 came back split, and the split is the useful part.** The
*precondition* is genuinely no longer prose — the compiler carries it at all 44
sites, and that half is completely gone, not narrowed. But **prose reappeared
wearing a module-doc hat, three times bigger**: across `crates/topo/src` the
diff is **+152 comment lines against −51**, so ~78% of the unit's net growth is
comment. `live.rs` retired ~22 doc lines on `link_half_edges` and shipped ~82.
Two pieces of it argue rather than describe — a **rejected alternative** (the
GhostCell brand) and an anticipated objection — which is §S38's pattern
exactly, and S38 is ACCEPTED with Evan's *"should definitely be trimmed"*. L2
inherits it.

The lane judged the residue an **honest, correctly-scoped narrowing** rather
than the defect re-minted, and the reasoning is worth keeping: the obligation
went from *seven callers must each prove a key resolves* to *one crate-wide
ordering rule whose violation is loud*, and the failure mode is unchanged
because slotmaps version their keys. What shrank was the obligation; what grew
was the word count. Those are different axes and the review is the first thing
on this track to separate them.

**MAJOR (must fix before merge): `docs/DESIGN.md` still says this cannot be
done.** The D2 addendum reads *"…carry a precondition rather than a per-site
proof, **and cannot carry one** … Retiring that asymmetry — a `Live` key type
that makes the discharge structural — **is** `SMELL-SCAN-2026-08.md`'s D25."*
The unit removed the identical sentence from `link_half_edges`' own doc and did
not sweep for it. It is in the **ratified contract**, and it is precisely what
this track's recording convention exists to prevent: a closed finding that still
reads as open. **The trap is worth more than the instance** — a citation sweep
that filters out lines mentioning `SMELL-SCAN-2026-08.md` hides this line,
*because the line contains that string*. The reviewer's own first grep missed it
for that reason.

**Question 2 found a better option available and not taken: the name.**
`certify` was already this codebase's word for **geometric** certification —
`geom-brep/src/certify.rs`, `certify_edge_spec`, `CertifiedEnclosure`, 279
occurrences of *certified* in `topo` alone, essentially all geometric. In one
function, `split.rs:162` (`certify_half_edge`, liveness) and `:224`
(`certify_edge_spec`, geometry) sit sixty lines apart with comments at `:158`
and `:222` both saying *certify* about different things. The type is already
called `Live`; `require_live` / `Live::of` / *proven* were free. **This is §S4
inverted — one spelling, two concepts** — and it is the clearest thing on this
track so far where the best answer was available and a working one was taken.

**Two findings feed the adversarial lane and were forwarded mid-review**, per
the precedent: *"one constructor"* is false (four mint doors, and
`resolve_half_edge_live` mints raw rather than through `Live::certify`, at ten
call sites), and the unforgeability claim is guarded by nothing, with the
crate's usual `compile_fail` instrument unable to reach a `pub(crate)` type.

**The class the review would look at next** (orchestrator's call whether it is a
row): the literal block D25 named survives ~9 times for the **other five
arenas** — shells, faces, vertices, loops, solids, surfaces. The *token* half
correctly does not transfer, and the PR says so; the **token-free** half does —
`certify_half_edge` is now a one-line refusal helper and the other five arenas
still spell four lines out longhand.

### #755 (E-f / D25) — adversarial lane, 2026-08-20: **CLEARED**, nothing broken

**All nine claims survive, and none of them survived by assertion.** What makes
this report worth keeping is its instrument discipline, not its verdict:

- **Unforgeability was compiled, not read.** Six forge attempts from a scratch
  module inside the crate, every one a hard error — `Live(k)` E0423, struct
  literal and functional-update E0451, destructuring E0532, `default()` E0599,
  `into()` E0277, and `mem::zeroed` refused by the workspace's
  `unsafe_code = "forbid"`. Field privacy is module-scoped, which the E0451 case
  demonstrates rather than asserts.
- **The loud-failure claim was planted.** A residue violation (`remove` between
  certify and splice) run in **release**, where the debug postcondition is
  compiled out: the `unreachable!` fires, `#[track_caller]` names the splice site
  rather than the helper, 14 tests red. Plus a null-key probe that matters
  because `mint_halves` leaves `next`/`prev` null, and a stale-key probe over
  200 000 insert/remove cycles on one slot.
- **The two arguments the style lane declined to check were made executable.**
  `kef`'s `remnant.first()` ⟺ `next(he) == he` and `split_edge`'s two-case
  exhaustiveness became assertions run over the lib *and* the 339-test
  integration suite, **each proved live by inverting it** (25 tests red, 16 red,
  8 red respectively). An assertion that has not been inverted is not evidence.

**The sharpest negative result: the cross-body hole is real, reachable and
pre-existing.** `merge_faces.rs:468` stages `let mut work = self.clone()` and
commits at `:540`; `Body: Clone` preserves slotmap keys, so a token certified on
`self` certifies equally on `work` and would write into a body that has since
diverged. `revert.rs`, `transform.rs` and two `body.rs` doors are the same
shape. **No token reaches any of them today** — `Live` and its four doors live
in five files, none holding two bodies — and on the merge base the identical
misuse was equally available through raw keys. So: no regression, and a named
shape for whoever widens `Live`'s reach.

**Two corrections to the dispatcher, both mine:** hosted run 32387970145 was
**cancelled** (superseded when the lane pushed the E-R4 correction), and the
green run is 32388486531; and the release-profile selection is **25** tests, not
17. I cited a run number from a report rather than from the runs list — the same
class as E-R4 and §C13, one level down.

*Worth carrying:* **the two lanes found different things and neither would have
found the other's.** The style lane found the ratified contract still saying the
thing had been proved impossible; the adversarial lane found the cross-body
shape and proved the arms. The only overlap was *"one constructor"* is false —
which both found independently, from opposite directions, which is why it is the
one finding recorded as certain.

### #753 (E-a / D22 + D34) — style lane, 2026-08-20: **not cleared**, and the review is the strongest instrument work on this track

Most of the unit held under a genuinely adversarial re-derivation: D22's premise
being false on main reproduced **verbatim at rustc 1.97.0**, the narrower-silence
argument confirmed by construction, the move and its fixture timed independently,
the `.py` refusal confirmed, and **six guards mutated out of the census one at a
time, 6/6 red**. Both rows are credited as *decided and defended, not deferred
behind work*, which is what they close on.

**Four MAJORs, and the first is a live wrong answer in a gate this PR wrote.**

- **A refusal that states a falsehood and refuses gates CI actually builds.** The
  census refuses a second-feature gate on the ground that *"no CI row builds
  `probe` with another one"*. `topo` and `sweep` carry **self dev-dependencies**
  that turn `test-support` / `sweep-testing` on for every one of that crate's own
  test builds — the two crates owning 6 of the 16 censused suites. Demonstrated
  by planting a `compile_error!` in such a suite and watching **the census's own
  type-check loop** fail with it: the file compiles, and the gate refuses it
  anyway, with a wrong explanation. Two further false positives on the same
  predicate, one of them contradicting the header two screens above it.
- **The widening opened a hole it is presented as leaving.** For a *re-gated
  existing* suite, main reds (the crate drops below its floor) and this head does
  not. That may be the right trade — a textual predicate cannot tell `not(miri)`
  from `not(target_os="linux")` — but it is a **cost of the change** written up as
  a standing gap.
- **The number carrying D34's rejection is n=1 per arm and inside its own
  variance.** Two runs give 219 s vs 196 s; **a third, same-config, in this PR's
  own run list gives 212 s** — a 16 s move between identical configurations
  against a claimed 23 s delta. And *"196 s today"* was the PR's **first commit**.
  The pair-sum unit was checked and is right; the basis is not.
- **The structural half of the rejection is falsified by `ci.yml:1873`**, which
  already invokes this script from the `k-lint` job — the one with the cargo
  build. So the assertion *could* live as a second mode with `--root` and a
  `--selftest`, and what actually remains is fixture cost — **which the same PR
  rules affordable two rows apart**, at D40.

**Two corrections to the dispatcher, both accepted.** The two defects I recorded
as *"found by hosted CI and not reachable locally"* were **minted by this PR** —
the stderr fold arrived with the rewrite, and the SIGPIPE check is new in the same
commit. That is REVIEW-STYLE-DISPATCH §2's *"a unit that adds a guard can leave it
failing open"*, not a discovery about hosted CI, and recording it the way I did
would have flattered the unit. Also three positive controls, not two.

*Worth carrying:* **the review's instrument was mutation, and it is the third time
on this track that mutating a guard beat reading it.** Six guards mutated, 6/6
red; two fixtures reverted to their pre-fix form, both red; two of
`test-aggregation.sh`'s own guards mutated out and the selftest stayed **green**,
which is how the vacuous pair was found. C22's *"executing the mutation beats
reading the code, and it was rare"* is no longer rare here.

### The fence I gave E-e was false, and the lane checked it because I told it not to trust me (2026-08-20)

**I wrote, in E-e's brief and in §D's lane table, that Track C's C-f (#731) is on
`editor-core/src/{resolve/, select.rs, refactor.rs}` and therefore disjoint from
`eval/`.** #731's actual file set includes **`crates/editor-core/src/eval/mod.rs`**
and `eval/anchor.rs`, plus six more files. It touches E-e's file.

They are disjoint **by item, not by file**: #731's `eval/mod.rs` hunks are the
content-key hasher, ~700 lines from the `Display` impl E-e rewrote. So the
exposure is a merge conflict, not a semantic collision, and the standing rule
covers it — *whichever is second re-merges rather than assumes*. **E-e continued
rather than stopping, and was right to**: stopping bought nothing the serialized
merge does not already provide, and flagging it loudly is the useful half.

*Where the claim came from, which is the point:* **§D's Track C table**, written
by that track's orchestrator, transcribed into my brief without being re-derived
against #731's actual diff. That is the same mechanism as #752's inherited gate
and as my own stale index — **a claim that travels between documents without
being checked at each stop** — and it is the third instance on this track in one
day. The register is where this happens, because a register is exactly a place
where claims are written to be copied.

**What stopped it costing anything was one sentence in the brief:** *"Disjoint by
inspection — so read C-f's head and confirm it, do not trust this sentence."*
That sentence has now paid for itself twice (E-f's line numbers were the other),
and it is worth keeping in every dispatch whose fence I did not personally
verify. **The honest generalisation is not "check your citations" — it is that a
dispatcher who cannot verify a fence must say which fences those are**, because a
brief states them all in the same voice otherwise.

Corrected in the roster above, and in §D's E-e row by the lane.

### #763 (E-b / D23) — style lane, 2026-08-20: **not cleared**, and the finding turned on its own author

Claims 1–4 held under independent re-derivation, and the **code half is judged
close to best available**: eleven prose counts removed behind an existing
directory-derived guard, with two of the replacements improving on their
originals. The verdict half did not survive.

**A live wrong answer, inside the row the unit placed.** D45 says
`PERF-SCAN`'s *"367 `tests/*.rs` files exist"* does not re-derive at its stated
base. **At `870c7a9` the literal single-level glob the sentence writes is exactly
367.** Its sibling leg is the same shape: *"all 14 crates with tests carry the
aggregator"* is **true at the stated base** — 15 `tests/` directories, one of them
Python — and the lane's "13" is the count at a *different* commit. **The lane
measured a scan-base claim at the wrong commit, in the row whose entire thesis is
measure at the stated base.** D23's *"none of which re-derive at their own stated
scan bases"* is false for two of four, and the verdict's third leg drops from
three-of-six to two-of-six.

**Three more of the same shape, all self-inflicted:** the survey's own headline
instance counts are **off by one, all three** (and one is the post-merge number
inside a row declaring its survey commit); a four-criterion list **switches
criterion mid-list**, which is the exact defect the unit corrected two paragraphs
earlier in the same file; and the unit **minted a fresh drifted enumeration in
`K-REPORT.md`** — *"2 of the 16 probe-gated suites"* — at the one site where the
tree already derives that number every merge.

**The verdict needs re-arguing, not editing.** *"No mechanism; this class is
found by reading"* was concluded after defeating only the weakest candidate. The
tree already carries the shape the row says does not exist:
`probe-suite-census.sh` pairs a derived count with a `CITING_FILES` list and
**fails when the prose stops naming its step** — a doc-prose gate keyed to a
derived quantity, in the same subsystem that produced the row's sharpest
instance. And the largest sub-class, eleven near-verbatim aggregator headers, has
an exact cheap guard nobody costed.

**A correction to the dispatcher, and it is the named exposure.** I repeated the
lane's PERF-SCAN error back to it **with my authority**, as claim 5 of the review
brief. `REVIEW-STYLE-DISPATCH` §3 says exactly this: *a lane's unverified
observation, repeated back to it as an instruction, arrives carrying the
dispatcher's authority and is one commit from a ratified doc.* It was one commit
from a ratified doc. The reviewer caught it because the brief also told it to
correct my framing — which is the second time today that instruction has been the
thing that worked.

*The shape worth keeping:* **a unit whose subject is unverified enumerations is
the hardest kind to write, because every number it publishes is an instance of
its own class.** This one published ~25 and got at least four wrong. That is not
an argument against the unit — it is the strongest available argument *for* the
finding, and the fix pass should say so rather than quietly correcting the
numbers.

### #767 (E-e / D28 + #693) — style lane, 2026-08-20: **CLEARED**

The census was re-derived independently and the lane's number and reading both
hold: **twelve** arms, not the row's eight, because the row counted *op* arms
while the class is *a payload with a `Display`, discarded*. Two payload types had
no `Display` at all, and the second is the find — `EditError::MetaUnversioned`
was rendering *"lacks the integer `v` version field"* for all three arms of a
dropped error, **two of which are not that**, and the test covering it matched
the variant and so could not see the message. A live wrong message, pre-existing,
found by a lane whose row was about thin messages rather than wrong ones.

**Eight style findings, and the first is a consequence of the fix itself.**
Forwarding now ships `Debug` struct guts into the Python exception string:
`splitting/finish.rs` renders a `BandError` as `{e:?}` and an `Indeterminate` as
`{diag:?}` — **both types have good `Display`s, and this PR starts forwarding one
of them fifteen lines away.** So the same `BandError` now has two spellings and
the `Debug` one became FFI-reachable *because of this change*, while
`refusals_render_as_prose_not_debug_guts` — asserting a rendered refusal contains
no `{` — runs on one fixture and cannot see it. **A fix that invalidates the
premise a neighbouring test states as the rule.**

The second finding is the same class **outside the crate the existing row
covers**: five more `Debug`-dumps of typed payloads in `editor-core/src/persist/`
and `product.rs`, including one that bypasses the very `Display` this PR
corrected, one layer up in the same crate. D47 is scoped to `pncad-py`, so these
had no home. Placed as **D67**.

*Worth carrying:* **the lane's disclosed blind spot was where the class lived.**
It said its `Self::V(_)` pattern could not match a `Debug`-derived rendering and
routed that to D47 — correctly identifying the shape and mis-identifying its
extent by a whole crate. A disclosure that names the right pattern and the wrong
scope reads exactly like a discharge, which is why the standing rule is that a
disclosure is a work order.

### Six lanes filled the disk with target directories in the shared scratchpad (2026-08-20)

**24 GB free to 6.4 GB in under two hours**, with four lanes building. Cause:
every lane obeyed *"use your own `CARGO_TARGET_DIR`"* — which is the rule, and
which each of them followed correctly — and put it under
`/tmp/.../scratchpad/`. Six distinct directories, 13 GB, one of them 6.6 GB.

**The rule was right and incomplete.** `memories/agent-lane-operations.md` says
*never share a `CARGO_TARGET_DIR`* and separately says *the scratchpad is shared,
so lane-private files go elsewhere* — but the second rule is written about
**published** artefacts (PR bodies, findings files), and its stated reason is
*another agent can overwrite it*. A target directory is neither published nor at
risk of being overwritten, so nothing in the rule as written excluded the
scratchpad, and six independent lanes all read it the same way. **When six
careful agents make the same call, the brief is what is wrong.**

Reclaimed 11 GB from four finished lanes' target directories; the standing lane
header now says *put it in your own worktree*, with the number, because the
reason is a disk allowance and not a correctness argument and the number is what
makes it stick.

*The reason this is worth a paragraph:* the container's writable space is a
**fixed per-session allowance**, so a disk-full does not degrade gracefully —
it produces torn binaries and test results that must be treated as suspect for
the whole pressure window (`memories/agent-lane-operations.md`). This came
within a couple of gigabytes of costing every in-flight lane's verification, and
the only reason it did not is that a routine `df` was run before dispatching the
next lane.

### "22 of 26 green" is not a CI result (2026-08-20)

**#767's fix pass reported *"22 of 26 green, 0 failures"* and was accurate when
written.** Three `test (eps = …)` shards started eight minutes later and all
three failed — on `sweep::all m5_pr6_pcurves::a_seam_closed_tube_split_is_typed_either_way`,
a test in the crate the lane had deliberately fenced itself out of and did not
touch.

**The cause is the fix pass's own F7.** Asked to read a composed message end to
end, the lane found `SplitError` prefixing `"split: "` onto three stages that
already say `split_reduce` / `split join` / `split finish`, dropped it on those
three, kept it on the one stage shared with non-split callers, and wrote: *"no
test asserts on them."* One does, three shards over. **The grep was for the old
literal strings**, and a test that builds its expectation by concatenation, or
matches a prefix, or `expect`s with a formatted message, matches no search for a
literal.

**Two things here, and they should not be collapsed into one.**

*The message-assertion sweep is a known-hard class and the lane's pattern was
the wrong shape* — that is an ordinary finding and the fix pass will answer it.

*The reporting shape is the transferable one.* **A count of green jobs is only a
claim about the jobs that have finished**, and nothing in the number distinguishes
"26 jobs, 22 green, 4 pending" from "26 jobs, all done, 22 green". The lane's
sentence was true and unfalsifiable at once. The rule now in every dispatch:
**report how many of the run's jobs had completed, not only how many were
green** — two numbers, one of which is checkable later.

*And the orchestrator's half:* I nearly merged on that sentence. What stopped it
was reading the run's own check list rather than the lane's summary of it —
which is the same instrument that caught #755's cancelled-run citation, and the
same reason `memories/agent-lane-operations.md` says to read the workflow **runs**
list rather than the PR's checks list. **A lane's CI summary is a record of a
moment; the run is the fact.**

### Four orchestrators, one number sequence, and nobody's register noticed (2026-08-20)

**Track E issued `D61`–`D70` to five of its own lanes while those numbers were
already reserved to Track F**, and `D71`–`D80` to Track G. Both blocks were
claimed the same day, both are recorded in §D, and neither register caught the
collision. **Lane E-h did** — it read `SMELL-F-LOG.md` before writing a row,
found `D61` was not Track E's to give, and **withdrew a row it had already
placed** rather than shipping it.

Nothing was overwritten, and the only reason is timing: both new tracks were
hours old and neither had placed into its block yet. **One day later this would
have been two rows with one number in a register three tracks cite.**

*The mechanism, and it is not carelessness:* the standing rule is **take the
next unassigned number from the orchestrator, never the next gap you can
see** — which is exactly right inside one track, and silently wrong across
four. *"Unassigned"* was a fact about a branch the other three orchestrators
could not see. Four of us read `main`, each **correctly** computed the next free
number, and each got the same one. **A sequence with one allocator per branch is
not a sequence**, and no amount of care at any single allocator fixes that.

Track F's own block paragraph says as much and tells Tracks C and E to claim
blocks — it was right, and Track E owed itself one before it issued a single
number. Track E now holds **`D81`–`D100`**; the ten issued out of F's block are
reissued and each holding lane was told individually.

*What generalises past the register:* this is the third defect on this track
whose shape is **a correct local derivation of a quantity that is not local** —
the others being my stale *"highest one placed"* index and the C-f fence I
transcribed from Track C's table. All three were caught by a lane looking at
something the orchestrator had not thought to check. **The instrument that
works across tracks is a lane reading the other track's log**, and it is worth
saying in a dispatch rather than hoping for.

### #777 — a design-conversation PR that argues against itself, and is waiting on Evan

E-g's D27 dissolved `FilletError::EmptyChain` structurally, which retired the D2
addendum's *"one state this taxonomy does not contain"* paragraph. The lane's
first draft replaced it with **both** the factual retraction **and** a new
general rule — *ask whether the type can stop representing the state before
asking the taxonomy to grow.* I ruled those two apart:

- **The retraction stays in #768.** The state no longer exists, so leaving the
  sentence would be a stale claim **in the ratified contract** — exactly what
  #755 was made to fix one paragraph over.
- **The general rule goes to Evan in its own PR**, because it reaches into
  **S14**, which sits in *Open decisions — Evan only* and is currently a bound
  on other lanes (#740 left 46 lookup sites typed because it is open). The
  lane's own framing made the connection — *S14's graft class stays open because
  a public door genuinely produces it* — and an argument about S14 is not mine
  to ratify.

**#777 is what came back, and its shape is worth copying.** One paragraph of
proposal, the argument, both worked cases as a table, an explicit *"S14 stays
open and stays yours"* — and **three arguments against itself on the record**:
that it can be read as licensing type churn to dodge a classification, that
*"only if it cannot"* is a judgement with no cost threshold, and that it is a
rule minted from **one positive instance**. The body states that *"no — record
what #768 did and leave the procedure alone"* is a **passing answer**, and that
#768 does not depend on it either way.

*Why that matters beyond this PR:* §C2 of this document records that
**disclosure functions as immunity** here — a disclosed deviation scores as a
positive with no counter-metric. A design PR that lists the strongest arguments
against its own proposal, and names the answer that rejects it as passing, is
the one shape that cannot be read that way. It is also the first thing on this
track that made a design question *cheaper* for Evan to answer rather than
larger.

### The rule the block scheme was missing: placed numbers keep their identity (2026-08-20)

Lane E-a asked the question the double-allocation should have prompted and I had
not answered: **its own rows D40 and D41 sit outside every block** — issued under
the old single-sequence rule, unspent on `main`, and therefore *"in exactly the
kind of gap a fourth orchestrator reading `main` would compute as free."* It did
**not** renumber them, on the grounds that moving assigned numbers unilaterally
is the same failure in reverse. That instinct is right and worth more than the
two rows.

**The ruling: numbers already placed keep their identity; blocks govern new
allocations only.** Renumbering a placed row breaks every PR body and finding
that cites it — the argument that kept Track D's numbers when Track E took the
register this morning.

**But the gap E-a named is real, and the fix is legibility, not renumbering.**
*Placed* and *assigned-but-unspent* look identical from outside a branch, which
is the whole mechanism of the collision. §D now records **D1–D60 as the closed
pre-block sequence — allocated, none available, gaps included** — so a fifth
orchestrator reading `main` sees them as taken rather than recomputing them as
free. One paragraph closes it for everyone instead of moving two rows for one
track.

### #768 (E-g / D27 + D29) — style lane, 2026-08-20: **D29 cleared, D27 not**, and the correction is to my framing

**The headline I reported was true and I drew the wrong conclusion from it.** The
lane's result was *"none of the three refusal arms had to become row 4"*, and I
carried that as *the refusals did not become panics, so nothing was lost.* The
style lane's correction is exact:

> **"None had to become row 4" is true and is not the same as "no refusal was
> lost."** Two of the three sites lost their branch because a value carries the
> fact. The third lost half its branch because a *different* fact — one nothing
> carries — was assumed.

**`octant_chart` is the third.** The old code filtered incident links and
skipped one whose two supports were **not among the corner's three faces**,
refusing if none contributed. The incidence half is genuinely gone and is a real
improvement. The **membership** half is not: `CornerFaces::third(a, b)` is
*total*, so a link whose supports are not the corner's three now **scores a
candidate off an arbitrary face** instead of being skipped — and the only
statement that this is acceptable is `third`'s own doc, in another module.

**That is D27's own defect, reproduced by D27's fix**: *a fact held in prose one
or two frames from where it is needed.* `octant_chart`'s new sentence — *"there
is no 'no candidate here' state left to refuse"* — is true and is not the
property that was lost. **Totality is not meaningfulness.** And the assumption
holds only on a manifold-consistent body, which `surgery.rs`'s own header
doctrine refuses to assume anywhere else: *"No site inherits its proof from
whole-body validity."*

The style lane deliberately did **not** claim a live wrong answer: reaching it
needs a body where the vertex orbit and the edge's two faces disagree **while
both walks succeed**. Forwarded to the adversarial lane, whose claim 3 is
exactly `third`'s totality, as the highest-value thing left on the PR.

**Three more worth keeping.** D29's sweep **did not run the pattern it
disclosed** — `loft.rs:518,520` discard a typed error through `map_err(|_| …)`,
the shape the PR body names as its blind spot, in the crate the unit was already
editing. The **guard test's blind spot was demonstrated rather than described**:
a forged constructor written `-> Self { Self { faces } }` reds, the same one
written `-> CornerFaces { CornerFaces { faces } }` stays green. And `admit.rs`
added **3.6× the prose it retired** (comments +313/−86 against code +514/−379),
nine sites of it narrating what the code *used to* do, one narrating **another
PR's** incident — which is §4's rule and #755's finding, at a larger multiple.

*The generalisable half:* **a unit that replaces a refusal with a type has two
ways to succeed and they look identical in a diff** — the value carries the
fact, or the fact stops being checked. Only the first is what the row asked for,
and the PR body cannot tell them apart because in both cases the branch is
simply gone.

### #768's two reviewers disagreed, and 3,125 cases settled it (2026-08-20)

The style lane returned **D27 NOT CLEARED** on its lead finding: that
`octant_chart`'s deleted `continue` refused *a link whose two supports are not
among the corner's three faces*, and that `CornerFaces::third`'s totality
swallowed the refusal. I forwarded it to the adversarial lane as the highest-value
thing left on the PR. **It refuted it.**

The old lookup was `faces.iter().find(|f| **f != l.face_a && **f != l.face_b)`.
When both supports are strangers, every orbit face differs from both, so `find`
returns `faces[0]` — **exactly** what `third(a, b)` returns. `third` *is* that
`find`, unrolled, with `f2` as the total fallback. An exhaustive differential over
a 5-key alphabet, 3 orbit slots × 2 query keys, **3,125 cases**: 2960/2960
agreements wherever the old `find` answered; **zero `None` cases over three
distinct faces**, so the `continue` was **dead** for the same structural reason
`third` is total; and all 165 `None` cases have a **duplicated orbit**, which
`CornerFaces::admit` now refuses up front where the old code merely stepped past
it. **Detection went up.**

**What survives of the style finding is a real, pre-existing gap** — `octant_chart`
never verifies `faces.contains(l.face_a)` — unchanged by #768, taken as **D90**,
and made *cheaper* by it. The style lane's guess that a bad chart would surface
downstream as a late `Op`/`Certify` is probably wrong: the sphere case is a
reparameterization, same point set.

**The adversarial lane then found four things nobody had**, three of them in the
half the style lane had verified rather than attacked: `CornerLinks::seed` takes
its `vertex` **on faith**, contradicting the module's own *"no constructor that
takes the underlying data on faith"* — demonstrated by planting a token seeded
with a link touching **neither end** of the vertex and watching `corner_plan`
plan a corner from it. The guard has a **fourth escape**, a **child module**
`admit/inner.rs`, which is *inside* the privacy boundary and *invisible* to the
`include_str!` scan — planted, guard stayed green. `CornerFaces::admit`'s
distinctness check is **dead and untested**: planted `if false`, all ten lib tests
pass. And the `unreachable!` count is **22 → 23 code sites**, not "unchanged at
23", because the base's `build.rs` token was **prose inside a doc comment** — a
number E-k's decision row will read.

*Two things worth keeping about how this resolved.*

**A disagreement between reviewers is not a tie to be split.** The style lane's
reading was careful, plausible, and explicitly hedged on reachability; the
adversarial lane's was decisive because it **ran the two functions against each
other over the whole input space** rather than reasoning about them. Where a
claim is decidable by execution, the lane that executes wins and the other's
uncertainty is not evidence against it.

**The style lane's other findings were unaffected by being wrong about the lead**
— D29's sweep not running its own disclosed pattern inside its own crate, the
3.6× prose growth, three present-tense citations of a deleted symbol. A refuted
headline does not discredit a report, and treating it that way would have cost
three real findings.

### The unfalsifiable claim, falsified by execution (2026-08-20)

E-m's row offered two options for `step-import`'s recognizer: the tighter
cylinder certificate, or an encoding change. **It took neither, because the
premise was false** — and it is the second lane today to answer a row by
refuting it rather than choosing from its menu.

`recognize.rs` claimed three things: the first-order envelope refuses *every*
cylinder certificate; the `Plane > Cylinder` preference order is *"unfalsifiable
by execution"*; and *"no authorable patch double-certifies"*. **All three are
false on main**, shown with one fixture — an exact unit rational cylinder — that
double-certifies at ε = 0.99 and promotes to `Cylinder` at ε = 0.9.

**The claim that no execution could falsify the order was falsified by
execution.** Two mutations: stubbing the promoting arm → **0 red before**, two
new rows red after; **inverting the preference order → 0 red before**, one red
after. The test cited as pinning the order stays green under the inverted order,
so it never could pin what it was cited for.

*Why it was believed, which is the transferable half:* the certificate is
**scale-covariant** — its slack is a per-knot-span extent, not an ε — so what
decides it is per-span extent **against** `eps_in`. Fixing one side of that ratio
at the wild corpus's scale makes a local reading look universal. **A claim about
a ratio, tested at one value of the denominator, is a claim about that value.**

And the travel is the register's own shape: *"unfalsifiable by execution"* passed
through **four documents** — #711 → C8 → Track E's roster → the lane brief —
with every reader treating it as a property of the world rather than a claim to
check.

*Also settled:* the row's second item, `docs/ASM-R2A-SPEC.md:21`, **does not
exist** — deleted in DOC-LEDGER Sweep 1. A clarifier is impossible and the
general rule should not be minted, because `DOC-LEDGER` already rules that merged
per-unit specs are deleted and were never normative — **stronger** than *"landed
specs read as of their own date"*, since a merged spec is not read with a caveat,
it is not kept. The class is empty by construction. Reported rather than enacted,
which was the right call; **D87** carries what it leaves behind — the ledger's
*"no file was deleted that a live pointer depends on for its content"* misses a
live pointer depending on a deleted file **as a target**, which is what silently
voided half of C8's option set.

### The same number, wrong three times, and the third correction is the right one (2026-08-20)

`sweep`'s `unreachable!` population has now been counted three times by three
parties and every count was wrong in a different way:

- **E-g's first report: "unchanged at 23 tokens."**
- **The adversarial lane's correction: "22 → 23 code sites"** — because the
  base's `build.rs` hit was **prose inside `corner_convexity`'s doc comment**.
- **E-g's fix pass: 18 → 19.** Both prior figures counted `unreachable!`
  *tokens*, which include the macro's own definition sites and mentions. Actual
  **call sites**: `main` 18 (all `surgery.rs`); #768 19 (`surgery.rs` 17,
  `admit.rs` 1, `battery.rs` 1). The **+1 direction** of the correction was right
  every time; the magnitude was wrong twice.

**E-k's row carries the derivation and both retractions**, because that row is a
decision about bounding the row-4 population and is the one place the number will
be read rather than restated. *A count corrected without its derivation is a
fourth number waiting to happen* — which is D21's own lesson, applied to a
different quantity by a different lane on the same day.

### D91, and a stated blind spot that was a claim

E-g ran D29's disclosed pattern rather than leaving it disclosed:
`map_err(|_| …)` has **exactly two hits in `crates/sweep/src`**, both
`loft.rs:518/520`, and **zero** in `revolve/`, `skin.rs`, `extrude.rs` or
`fillet/`. And it is **not** the two-line fix the finding implied — both variants
are *also* constructed payload-free at `:273`/`:302`, so carrying a payload is a
**variant-meaning decision**, not a field addition.

The row records the method lesson in the lane's own words: **a stated blind spot
is a claim, and publishing one unrun is an unverified negative.** That is the
third time today this track has found a disclosure standing in for a discharge.

### D92 declined, and the reason is the register's own rule

Offered a row for row 0's S14 reframing, E-g **declined it**: the reframing is a
paragraph, not a row, because *its premise is unratified* — placing
implementation work against an unanswered question would be a register entry that
**cannot execute**, which is §C3's failure exactly. S14 is already an entry in
*Open decisions — Evan only*, and the reframing is recorded there with its
precedent (`merge_faces.rs:468`'s `let mut work = self.clone();` under its own
*"Never a partial commit: each sub-stage is tier-2-gated before adoption"* —
checked rather than carried). **If Evan answers "yes, restructure it", that is
the moment a row is worth minting**, and #777 says so in those words.

*Worth keeping:* a lane declining a number it was offered, on the grounds that
the row could not be executed, is the opposite failure mode from the one this
whole register exists to prevent — and it is the right call. **A placement whose
premise is open is not a placement; it is a deferral wearing a row number.**

### The counting bug that counted the line which derives the count (2026-08-20)

E-b's fix pass found why its three headline instance counts were each **+1**: the
grep was unanchored, so it matched **each guard's own
`format!("#[path = …]")`** — *the line that derives the set, counted as a member
of it.* Anchored: `topo` 53, `editor-core` 90, `sweep` 82.

That is D23's own class turned on the instrument measuring it, and it is the
second self-referential defect this row has produced — the first being the head
sentences falsified by the section the same change added. **A survey of
enumerations is written with the tools it is auditing**, and there is no version
of this row that escapes that; what there is, is a re-derivation recipe committed
to the tree so the next reader does not have to trust the number.

**D45 is withdrawn as a row rather than repaired.** Two of its four legs were
false — `PERF-SCAN`'s 367 and its 14-of-14 both re-derive **exactly** at the
stated base — and what survives is annotated in place. Withdrawing a row whose
evidence collapsed is the right disposition and the harder one; the lane carried
both consequences rather than softening them, including that the verdict's third
leg drops from *three of six* to *two of five*.

**The verdict was re-argued per sub-class and the guard was taken.** *"No
mechanism"* is withdrawn as a blanket. The twelve aggregator headers get a real
guard — one spelling asserted verbatim, the retired phrasing forbidden by name,
an empty walk refused, **falsified both ways before landing**, and deliberately a
**test rather than a gate**, so there is no `ci.yml` wiring to drift and nothing
to unwire. The `CITING_FILES` mechanism I pointed at became **D84**.

*And a citation rotted inside the fix pass that wrote it:* **D85 cited
`scripts/check-test-aggregation.sh`, which #753 moved to
`scripts/gates/test-aggregation.sh` hours later.** Re-pointed, with the move
recorded in the row. A register this active produces stale pointers faster than
a lane can write them, which is the argument for citing by **symbol and reason**
rather than by path wherever a row can.

### #773 (E-h / D21) — style lane, 2026-08-20: **not cleared**, and the reading is the finding

The conversions are good work and the arithmetic is honest — the reviewer
**re-derived 14 + 3 independently and it holds**, every one a genuine discard,
none wrong. What did not survive is the unit's own deliverable: **the reading**.

**Its boundary is a scope sentence, which is the defect D21 exists to close.**
Pass 1 reads *"over `crates/topo/src`"*; **pass 2 went workspace-wide over
`crates/*/src`** — so the crate clause is a convenience for one spelling and the
class for another. And there is a receipt on the other side of it:
`step-import/src/normalize.rs:737` silently discards a mutation-phase write **on
a key resolved by an infallible panicking index five lines above**, nine lines
apart, two dispositions, one loop — with a fifteen-line comment immediately above
defending that very write as load-bearing for bit-identical round-trip.

*That is D21's own thesis executed against D21.* The row's complaint was that a
scope sentence, not the class, drew W2c's boundary. The fix drew a new one.

**Seven more, and three are the same shape one level in:**

- **`euler.rs:24`'s replacement asserts a universal the PR knowingly falsifies.**
  It widened *"at every write"* to **crate-wide**, naming the non-operator
  structural mutators — while placing `merge_faces.rs`'s discard as **D88**
  rather than converting it, and `merge_coplanar_faces` is one of those mutators
  by the PR's own argument. The old sentence asserted a universal on three
  modules' evidence; the fix widened the universal and made it false.
- **The instrument's justification, corrected once, is still false.** It claims
  `set_face_surface` has *"no error-path row at all"* and there is *"no stale-key
  row anywhere"*. `topo/tests/review_m2_pr3.rs:332-337` **is** a
  `set_face_surface` error-path row on a stale key, asserting the typed refusal
  **and** a body-untouched snapshot — the exact pair the new rows call
  unprecedented. *It moved the claim without running the grep that would have
  settled it.*
- **Six structurally identical siblings sit at row 1 in a file with two converted
  sites.** `boolean/combine.rs` has 48 `.ok_or_else(corrupt)?`, six keyed by a
  `dk` minted by the same call's mint pass — all provable under #720's standard.
  The reading's classification clause makes them invisible, so **the scope hazard
  migrated from a path boundary to a classification boundary.**

**And one is row 0, four lines away in the same function.** `revert.rs:215`
iterates `self.edges` to harvest keys and looks each one up again in `out`; the
unrepresentable form is `out.edges.iter_mut()`, **which `revert.rs:227` and
`:234` already use** for the surface and face arenas. The PR added an
`unreachable!` and then a probe row defending a fact the restructure would make
unstatable. The per-arena-token rejection is a good argument about *tokens* and
does not reach *restructuring*, which is what Evan's row 0 asks.

**Also: `8d80e2bb` deletes five checked-in proptest regression seeds**, three
carrying hand-written diagnoses, unmentioned in the PR body — in a PR about not
discarding things silently.

**Two corrections to the dispatcher, both accepted.** *"14 + 3"* is honest and I
was right to carry it. And **S68's problem statement must be left alone**: the
`S`-series `Verdict:` line is Evan's by convention and S-findings keep their
bodies — unlike D-rows, whose recording convention demands the problem statement
be removed. I had the two conventions blurred; leaving S68 untouched was correct.

### #773's adversarial lane broke four claims by building, and the census survived (2026-08-20)

**What survived is worth stating first, because it is most of the unit.** The
count was re-derived **twice independently** — direct enumeration of the diff,
and a 180-hit sweep of the base tree with all 120 non-W2c hits classified by hand
— and there is **no 18th site inside `crates/topo/src` under the stated
reading**. Candidates were chased and rejected with reasons, including one that
looked like a D88 sibling and is not: `merge_faces.rs:416`'s
`SecondaryMap::entry` returns `None` only for a null key or an older version, so
on a freshly built map a stale non-null key yields `Some(Vacant)` and the write
happens — **a null-key guard, not a stale-key discard**, established by reading
slotmap's source. Per-site soundness survived too, verified rather than read off
the row. And **all five probe rows inverted red**, including the movefac walk row
retargeted to `EntityId::Solid`, which proves it reaches the guard it claims
rather than passing on an earlier refusal.

**Four claims broke, and the method is the point: the reviewer built rather than
argued.**

- **"No two share a consumer" is false.** Eight of the 17 sit on three shared
  doors — `get_edge_mut`, `get_vertex_mut`, `get_face_mut` — and **D25's
  "consumer" was `link_half_edges`, a function that takes the key**, whose exact
  analogue here is `get_*_mut` at 22/26/21 call sites. The conclusion may still
  hold **on cost**; the stated reason does not.
- **One site should have been row 0, demonstrated by replacing it and running the
  suite.** `revert.rs:217` became `out.edges.iter_mut()` — *the idiom the same
  function already uses two loops down* — and topo went **441/342/6/4 green**.
  The `unreachable!` and a third of a probe row's purpose become unstatable. The
  deliverable is **16 conversions + 1 restructure**.
- **Two of the seventeen messages are byte-identical**, and the lane's own
  poisoning rounds named them `graft-remap` / `graft-recert` — *the distinct text
  existed and did not land in the code.*
- **One proof is one frame up, not in the same call.** `merge_faces.rs:955` is
  sound — verified by establishing that `kemr` contains no `faces.remove` and the
  only face removals in the euler modules are in `kef` and `kfmrh` — but #720's
  standard as the addendum states it is *never on a proof borrowed from one frame
  up*, and the row-0 fix is to hand the resolved face down.

**And it flipped the style lane's framing on the six siblings.** `combine.rs`'s
six `.ok_or_else(corrupt)?` are keyed by a `dk` minted by the same call's mint
pass — **so under the addendum a minted-in-call key that fails to resolve is a
kernel bug, row 4, and if anything it is the six that are misfiled at row 1**,
not the two that were converted. Same facts, opposite conclusion from the same
document. The file answers one proof two ways and the unit owes the reason.

*A reviewer's artefact worth keeping:* **`carve`'s arm fires only in
`tests/all.rs`**, so a `-p topo` run without `--no-fail-fast` never reaches it
once an earlier lib test panics — the reviewer's first inversion scored it 0 for
exactly that reason and it caught its own instrument before reporting.

### Row 0 is ratified — and it was NOT on `main` until #817 (corrected 2026-08-20)

> **This heading was wrong for eight hours and I repeated it to Evan and into a
> dispatch brief.** #777 merged, but its base was `smelle/d27-d29`, not `main`.
> #768 merged that branch into `main` at `12:09:59`; #777 merged **into the
> branch** at `12:10:18` — nineteen seconds later, so GitHub's retarget never
> fired and `c4d284aa` never became an ancestor of `main`. Row 0's text sat on
> an already-merged branch, reachable from nothing.
>
> **`main` was left self-contradictory in its ratified contract**: D21's
> paragraph cites *"16 row 4 + 1 row 0"* while the taxonomy's own closing
> sentence twelve lines down reads *"The five rows stand unamended"*, and the
> superseded *"open for Evan's sign-off in its own PR"* line was still there.
> Recovered as **#817**, `docs/DESIGN.md`'s added lines verified identical to
> #777's.
>
> **Found by lane E-k, and only because its brief told it to read the addendum
> as it stands today rather than as the brief described it.** I had written
> *"#777 merged and promoted row 0"* into that brief as a fact. The lane read
> the file.
>
> **E-R5 said a row is not placed until it is on `main`. This is the same rule
> one level up: a PR is not landed because it says "merged".** *Merged* names an
> event, not a destination — and the destination is the whole content of the
> claim. What I checked was #777's state; what I needed was its base. **A
> green checkmark on a PR whose base is a lane branch looks exactly like a green
> checkmark on a PR whose base is `main`**, which is why nobody caught it for
> eight hours and why the check is now `merge-base --is-ancestor`, not the
> word on the PR.


*Can this error state be made unrepresentable?* — asked of every state **before**
rows 1–5, and **preferred over every row below whenever it is available.** It
adds no bucket and renumbers nothing; what it adds is a procedure step: **a lane
filing a state under any row owes the reason row 0 did not apply.**

Without that step the procedure had no place for *"this state should not exist"*
to be an answer, so a state fitting no row read as a **gap in the taxonomy** —
which is exactly how `FilletError::EmptyChain` came to sit under a row whose
definition it failed, and how a sixth row came to look like the fix.

**"If possible" is bounded at both ends, from the tree rather than from
principle.** Yes: `EmptyChain`, a private field and a constructor signature, no
public API change. No: `Live`'s generative brand, which needs a lifetime on
`Body` that infects every signature naming a body — **and #755 weighed exactly
that and rejected it, before row 0 existed.** So the rule is *yes when the change
is local to the type and its constructors; no when it propagates into signatures
that do not otherwise care* — and **a "no" is a complete answer**, recorded as
the reason a row below applies. A preferred disposition that cannot be declined
is not a rule.

**S14 is reframed and not answered.** Its first question is now *can
`graft_disjoint_all_keyed` stage into a fresh body and commit on success* —
a shape already in the same crate at `merge_faces.rs:468`, checked rather than
transcribed, under its own *"Never a partial commit."* **No row was minted for
it, deliberately**: the premise is unratified, so a placement would be a register
entry that cannot execute. If Evan answers yes, that is the moment.

*The two things worth carrying past this ruling:* **#755 is the rule's best
evidence** — a careful lane asked row 0's question, answered it no on
propagation cost, and documented the residue its token does not carry, with no
rule telling it to. Row 0 describes what careful lanes already do. And the PR
that proposed it **argued the three strongest cases against itself and named the
rejecting answer as passing**, which is the one shape §C2's *disclosure-as-immunity*
finding cannot absorb.

---

## Landings

### #809 — D35 + D96 (E-k), merged 2026-08-20

**D35 closes on (d): no gate**, with the reason written into the D2 addendum
beside row 0 rather than left in a PR body — a row that closes on *"no, and here
is why"* has to put the why where a reader of row 4 will meet it.

**Population re-derived: 103 kernel call sites across seven crates**, not the
row's 101/102 over nine. The row's figure *reproduces*, and the lane could say
exactly how: its census counted `crates/**` including `tests/`, which is where
its `geom-brep` 5 and `profile` 1 came from — **both crates have zero call sites
in `src`**. The file list in the row was wrong the same way and is corrected.

**76 of the 103 are one state**, an arena key proven live earlier in the same
call, written by three conversion passes under one ruling. **Their row 0 was
already answered *no* by #755, before row 0 existed.** So for 74% of the
population neither D35's question nor row 0's has live work in it — which is the
measurement that made (d) the answer rather than a shrug.

**(a) was falsified by the tree in both directions**, not argued against. False
negatives: `topo` already contains a source walk over these exact messages, and
it works *because* it forbids one spelling — *"does this message state why the
state cannot occur"* is not decidable by grep. False positives: `quantity`'s
`row_index` is message-**less** on purpose and a required-message rule **cannot
be satisfied there** — `unreachable!` routes even a literal through
`format_args!`, which is not const-callable, so `unreachable!("literal")` in a
`const fn` is `E0015`. The lane doubted the site's doc comment, compiled it, and
found the comment right and its own first reading wrong.

**13 row-0 candidates → D96**, written as its own finding. Its evidence is
`battery.rs:796`: **D35's own roster line had already named it** — *"the third
non-empty-by-construction sequence in that file and the only one still a
`Vec`"* — published in the register, and still a `Vec` when the lane opened the
file. *Findable from the register and unfixed* is the register failing at the
one thing it is for.

**Two durable method findings, both from the lane's own errors.**

*The fence's file list is stable; its head set is not.* D96's Track C fence was
measured three times in one session and was stale twice by the time it was read
— **six SHAs had moved within the hour**, three PRs merged, two opened. The lane
then widened from Track C to **all** open heads, correctly: file overlap is the
mechanism and it does not care who owns the branch. The row now tells its taker
to re-derive rather than cite, and says why.

*And it transcribed a claim in the same breath as insisting on re-derivation.*
Its merge commit message named three roster rows **from the dispatch instead of
from the tree**; the tree disagreed thirty seconds later, during the
verification step it had already scheduled. It reported this plainly rather than
amending it away. **That is an argument for the step, not for care** — the lane
that spent a day on re-derivation still copied one sentence, and what caught it
was procedure, not vigilance.

### #826 — `memories/` (E-q), merged 2026-08-20

Evan's ruling: *"most of the stuff in memories that cites a specific
measurement should just be deleted. memories is definitely not the place for
historical anecdotes, but it's also not really the place for live data."* So
**§Q6's menu was the wrong frame** — it classifies a measured claim as
guarded / scheduled / unguardable-with-a-written-reason, and all three assume
the claim stays. Here it mostly should not. **21 blocks → 17 keeps, 4
repointed, the rest deleted, every rule surviving.**

**Four "live data" sites became pointers, and two of them had already drifted —
in the file whose own criteria forbid second copies.** `tessellation-budget.md`
said the safe aspect was `= 5`; `docs/ASM-LOG.md` said `≤ ~4`; the real owner,
`crates/mesh/src/nurbs_cert.rs`, carries the derivation and a measured
`(3.87, SAFE_ASPECT]` gap. Same shape in `agent-lane-operations.md`: *"4–8 GB
`target/`"* against `disk-watchdog.sh`'s own `5-8G`. **The repointing was not
tidying — it resolved two live disagreements**, and the `ASM-LOG.md` one sits
on a dispatch path.

**The instrument correction is the part that reaches backwards.** #681's `.md`
row prescribes `--marker ''`, and `line.find("")` returns `0` on every line, so
nothing terminates a block and the script prints **one block per file**. The
issue's prose names *"paragraph-blocking"* as the replacement; **paragraph
blocking is not in the script.** E-q added it as a `--paragraphs` flag,
reproduced E-l's 21 exactly, and thereby established that **#810's `.md` legs
ran a variant the issue does not contain, inferred from prose** — which is
§C15's failure mode occurring inside the lane whose brief was written to
prevent it. Nothing in #810's dispositions is wrong; its *method statement*
named a flag that cannot produce its numbers.

**Two holes, one new.** #681's carried hole (the bare-number arm is time units
only, so bytes/percent/counts reach only through the vocabulary arm) bit
hardest here: **in five of the eight edited files at least one site was reached
by reading, not by the instrument** — including `tessellation-budget.md`'s
densest numbers. And the time-unit arm omits **`min`**: `git-workflow.md`
scored **zero** blocks while carrying `~5-7 min`, `35-70 min` and `30G cache`
in one sentence. **21 is a floor, and the lane said so.**

The rule went into `memories/cad-working-style.md`'s criteria as one bullet
extending *No live counters*, so it does not recur.

### #810 — #681 (E-l), reported 2026-08-20, awaiting #763

The measured-claim sweep outside `crates/*/src`. **Seven of #681's nine surfaces
swept, the other two declared in writing** — a declaration discharges that
issue's done-condition and silence does not, which is why the fences are written
into the PR rather than merely obeyed.

**The instrument was re-run, not transcribed**, and that is the row's own
finding about itself: #667's *197 blocks over `crates/\*/src`* is **217** at this
base. Nothing depends on the number; everything depends on it having been
re-taken.

Per leg, each with **its own** blind spot rather than #667's: `crates/*/tests`
189 blocks → 10 claims; manifests 8 → 4; the guide's rustdoc-included `.md`
files 11 → 1; `pncad-py` **6 with docstrings against 1 without** — the docstring
half is the surface, and a `#`-only pass would have reported a clean negative;
`crates/*/examples` 2 → 0; `docs/` prose 171 → 6; `local-scripts/` 21 → 3.

**§Q6: 24 real claims — 7 guarded, 2 scheduled, 13 unguardable-with-written-reason
(11 newly written at the site), 1 unguarded.** The one unguarded row is the
interesting one precisely because it is *not* unguardable: **#807**, the claim
that the kernel plus `editor-core` compiles clean to `wasm32-unknown-unknown`.
Nothing in CI builds a wasm target, so a dependency bump falsifies it with every
check green — and two `cargo check --target` lines would fix that. **It is
Track F's surface, and Track E cannot place it there** (E-R5: a row is not
placed until it is on `main`, and cross-track placement is that track's).

**Two corrections owed to my dispatch, one in each direction.** My brief said
`guide.rs` pulls four files; it pulls **five**. And the Track C fence I flagged
as unverified had *lifted* — #803 merged mid-lane and unfenced
`mesh/tests/revolves.rs`, which the lane then carried. Flagging the fence as
unverified is what made both recoverable; see rule 7's new clause.

**A tenth surface #681's list does not contain: `memories/`, 21 blocks.**
Reported, not swept — adding a surface is the issue owner's call. It is
plausibly the highest-consequence one: `CLAUDE.md` makes `memories/MEMORY.md` a
session-start read whose pointers get followed, and the two dense files are
`tessellation-budget.md` and `test-suite-cost.md`, whose subjects *are*
measurements — one of which `docs/ASM-LOG.md` routes a live dispatch decision
through. **This goes to Evan.**

### #773 — D21 (E-h), merged 2026-08-20. **Track E's one adversarial review.**

The discard idiom outside W2c's three modules, censused and disposed:
**17 sites — 16 row 4, 1 row 0** — with the reading stated because three
readings of *"the discard idiom"* exist and they give different censuses. The
src/test cut is the `#[cfg(test)] mod` **declaration**, wherever it stands, not
the file name.

**The adversarial lane broke four of its claims by building them**, which is
the review this row was given an adversarial lane to get: a census defended in
prose is a claim, and a census a reviewer can re-run is a receipt. All eleven
items (B1–B5, M1, M3, M6, M8, N1–N4, N6) closed in one fix pass.

Placed **D88** (`merge_faces.rs:766` — `absorb` drops every ring of the
absorbed face on a silent `None`, **inside the mutation loop**: the one site the
census found that cannot meet #720's standard), **D89** (the same fourth
spelling in `editor-core/src/edit.rs:995`, eleven lines after the handler has
already proved the node live — so the discard is unreachable and the proof is
not carried), **D94** (D21's *"`crates/topo`"* clause was **a scope of work, not
a finding about the class** — its own pass 2 went workspace-wide for one
spelling, so the row used the crate as a boundary for one spelling and as no
boundary for another) and **D95** (`boolean/combine.rs` now answers one proof
two ways, and D21 is why).

**Its own reporting is the incident above** — a *"25 of 26 finished"* taken
against a denominator the artifact-gated fan-out had not finished growing. Rule
7's closing paragraph is that lane's correction, in its own words.

### #761 — D33 (E-d), merged 2026-08-20

The audit's coverage of its own two crates, measured, and its bound stated.
**246 funnel-reaching predicate names; the document dimensions 121, reaches 223
under the most generous reading, and misses 23** — a ceiling on reach, a floor
on the hole, and a verdict count, three numbers the file had conflated into one.
The two-crate bound is **deliberate and binds the TABLE, not the document**,
which is sharper than the yes/no the row asked for: four of the dispositions are
`editor-core` rows, so *"the document is bounded"* would have been false.

**Its review returned four MAJORs and every one was on the axis the row exists
to close** — the number is the deliverable, and a published number was wrong, an
omitted spelling was named in the document's own scope line, and the sentence
that made the floor safe to build on was falsified by the section the same
change added. All four closed, two of them better than asked:

- **The site ledger is published and demoted** — 322 raw → 315 funnel calls →
  302 name-fixing sites, every subtraction named, rather than a headline that
  contradicted the one piece of arithmetic the paragraph offered.
- **D56 was handed back with a measurement as the reason.** Asked to check
  whether "a helper that fixes a name" is a class, the lane re-derived it at
  **~30 instances** and found every one benign — *a helper that fixes a name
  calls the funnel with that name as a literal, so all of them are already in
  the 210.* The class costs a **site** count its canonicity and costs the roster
  nothing. A row declined on evidence is worth more than a row taken on
  suspicion.
- **The published re-derivation recipe now strips the file's two
  self-describing sections**, so the next re-deriver does not have to exclude
  one by hand as the reviewer did.
- **The overclaim was retired in place, not footnoted:** *"The word* every *was
  this document's for a year and it was never true of the second bound."*

Placed **D46** (audit the 23), **D51** (`DESIGN.md:31`'s F-range, where two
findings the audit itself RETIRED are still listed as open — and the residue was
*re-homed*, so the pointer redirects rather than merely missing) and **D57** (a
class measured at seven names by one spelling and **nine** by two, over two
crates out of a dozen — an unknown population read by a known-incomplete
instrument, which is S113/D23's shape caught inside the review looking for it).

### #752 — D26 (E-c), merged 2026-08-20. **Track E's first.**

S19's four unplaced rows are placed: **D36** (`UnsupportedCarrier`, three
meanings in 22 sites, and the overload is load-bearing), **D38**
(`SkippedMerge`, a door handing one of two incompatible contracts depending on
a property of the input), **D39** (`ProgramRefusal::Geometry`, whose constraint
has *not* moved). The fourth, `pncad-py/tags.rs`, was **refuted as stated** — a
discriminant tag map is the right FFI shape and does not drop the payload — with
**D37** placed for what survived it. Its review added **D47** and **D48**.

**Three things this landing is the record of.**

*A restatement is not a check.* D37's gate on D28 was inherited verbatim from
D28's row and re-published as verified (*"the re-derivation confirms it from the
other side"*). It was false. Now cut — and per E-R4 the row records the three
mechanisms **checked and found absent** (no data dependency, no file overlap, no
visibility problem), so the residual note cannot be deleted by disproving one
story. What remains is an *answer* dependency, not a gate: what D28 makes the
message say is an input to *"is the tag plus the message the whole surface?"*

*A stated negative result is a claim.* The unit declared it had checked for
closure factories bound to another name and found none. `pcurve_cache.rs` has
two, fifteen lines from the sibling D36's whole argument rests on, with eight
call sites. It cost D36 a count, now stated with both readings named.

*The class check came back negative, and that is a result.* Asked to look for
other "gated on" cells minted from another lane's prose, the lane read all 22
live rows: 17 are "nothing", two are Track C handoffs, one is a pure file-overlap
gate verifiable from the Scope cells, and exactly one other asserts a mechanism
(D21→D25) — which it sampled and found re-derivable. **One instance, sound: not
a class, and not worth a row.** A negative result reported in three sentences is
worth more than a row nobody would have closed.
