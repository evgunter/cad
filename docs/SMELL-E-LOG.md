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

- **No `ssh`**, so `local-scripts/new-lane.sh` cannot run — it clones over SSH.
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
| **E-a** | D22 + D34 | `scripts/gates/`, `.github/workflows/ci.yml` | none | style | unstarted |
| **E-b** | D23 | `docs/` + suite headers; code set is what the re-derivation finds | none | style | unstarted |
| **E-c** | D26 | `docs/SMELL-SCAN-2026-08.md` §D and §S19 | none | style | unstarted |
| **E-d** | D33 | `docs/predicate-dimension-audit.md` | none | style | unstarted |
| **E-e** | D28 + issue #693 | `editor-core/src/eval/` | **confirm against C-f (#731)** — same crate, disjoint files | style | unstarted |
| **E-f** | D25 | `topo/src/euler.rs` and every `link_half_edges` caller | none | **ADVERSARIAL** | unstarted |
| **E-g** | D27, then D29 | `sweep/src/fillet/{build,surgery,mod}.rs` | none | **ADVERSARIAL** (D27), style (D29) | unstarted |
| **E-h** | D21 | `topo/src/{split,attach,movefac,revert}.rs`, `splitting/finish.rs`, `boolean/combine.rs` | **E-f** | **ADVERSARIAL** | unstarted |
| **E-i** | D24 | `Cargo.toml` workspace lints, or `.github/workflows/` | none | style | unstarted |
| **E-j** | D31 | `sweep/src/skin.rs`, `geom/src/curves/fit.rs`, home in `geom-core/src/spline/algebra.rs` | **Track C (C-l, C-g)** | style, escalates if the sort order is load-bearing | unstarted |
| **E-k** | D35 | `docs/DESIGN.md`'s D2 addendum, and whatever the answer names | **E-g**, **E-h** | style | unstarted |
| **E-l** | #681 | everything outside `crates/*/src` | none | style | unstarted |
| **E-m** | #711 | `step-import/src/recognize.rs`, `docs/ASM-R2A-SPEC.md` | none | style | unstarted |
| **E-n** | D20 | `topo/src/seqgen.rs` | none | style; closes on an attribution off hosted CI | unstarted |

**Not taken by Track E:** D30 and D32 (Track C's files — C-m, C-q); C11's #726
and #727 (Track A's residues, in `mesh/` and `props/`, which are C-k's and C-m's
scopes); L1, L2 and L3, which are deliberately last and stay that way.

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

**3. Your branch and your lane.** Branch `smelle/<row>` (e.g. `smelle/d25`),
off current `origin/main`. Use **your own** `CARGO_TARGET_DIR`; never one shared
with another worktree. Anything to be published — a PR body, a review findings
file — goes to a **lane-private** path, never the session scratchpad, which is
shared between concurrently running agents.

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

**7. Merge `origin/main` immediately before opening the PR, and re-merge
whenever main moves while it is open.** A PR that goes CONFLICTING runs **no**
check runs at all — it reads as CI absent, not CI failing. After any push,
confirm checks actually **started** by reading the workflow *runs* list.

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

---

## Rulings made in this track

| # | Ruling |
|---|---|
| **E-R1** | **Row numbers do not get renumbered by a change of track.** Merged PR bodies cite `D21`, `D27`, `D35`; a Track-E renumbering would fork the one register for a cosmetic gain. Track letters name *ownership*; row numbers name *placements*, and the two are not the same axis. |
| **E-R2** | **A track with no orchestrator is not a schedule.** Track D's rows were correct, placed and edge-free, and would still have been unstarted a week later. The audit that produced Track D found this once already (§C3); this track is the same finding applied one level up — a *track* that does not execute is not a register either. |
| **E-R3** | **A row whose files belong to a live lane goes to that lane, not to a second track.** D30 → C-m and D32 → C-q. The alternative is two tracks editing one file with no shared orchestrator, which is the collision the whole sequencing exists to prevent. If Track C declines, the row returns here — never to nobody. |

---

## In flight

*(none yet)*

---

## Reviews

*(none yet)*

---

## Landings

*(none yet)*
