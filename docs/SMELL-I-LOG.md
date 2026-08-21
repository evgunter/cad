# SMELL-SCAN Track I — orchestrator log

**Constituted 2026-08-21.** Track I is the *measuring consumers* track:
**`crates/geom-brep/src/props/`, `crates/mesh/`, `crates/topo/src/census.rs`**
— the code that measures and certifies geometry, sitting on top of Track H's
substrate. §D of `docs/SMELL-SCAN-2026-08.md` remains the schedule; this file
is the execution record — rulings, lane state, review outcomes and incidents.
**Live status is here and in §D, never in `memories/`.**

**The edge with Track H is a dependency, not a file overlap** (§D says so, and
it re-derives: the two scopes share no file). `geom-brep` and `mesh` depend on
`geom` and `geom-core`, so an H change to a public signature ripples into I's
builds. **I re-merges and re-runs rather than reasoning about it**; neither
track waits on the other to start. Track H is live as **#865**.

**This track runs entirely outside the model A/B experiment.** No Fable/Opus
pairing, no ordinal, no row in `docs/MODEL-AB-LOG.md` — **nothing on this track
reads or edits that file.** The experiment is paused on a model limit (Evan,
2026-08-21); the cheapest guarantee that the pause stays clean is that this
track never touches it. A lane that believes it needs to is wrong and should
ask.

**Branch prefix:** `smelli/` for units; the orchestrator sits on
`claude/smell-scan-track-i-n8z2iu`.

---

## Review policy for this track

Not the full orchestrator protocol. Per Evan, 2026-08-21, and identical in
shape to Tracks F and G:

- **Style review on every unit** — `docs/prompts/reviewer-style-lane.md`,
  dispatched **by path** (read it once; never paste it), with the per-lane
  emphasis a dispatch owes (`docs/REVIEW-STYLE-DISPATCH.md`). On top of the
  standing brief, every Track I style review answers two questions the brief
  does not:
  1. Is the finding's **original** stylistic problem now *completely* gone —
     not narrowed, not relocated, not half-closed in a way that reads as
     closed (§C13)?
  2. Was it closed in the **best** way available, or merely in a way that
     compiles?
- **Adversarial review only where the change carries meaningful risk.** The
  criterion is Evan's (`SMELL-C-LOG` C-R12): *complex enough that there is a
  significant chance the change introduces a regression CI will not catch.*
  That is narrower than "this code is load-bearing", and narrower than §D's
  own advance guess.

**Two of five lanes are adversarial**, and §D's advance list is narrowed with
a reason — see **I-R6**.

## What a lane does with what it finds

Three destinations, and a lane picks by the finding's kind, not by its size:

- **A new style finding** → recorded in `docs/SMELL-SCAN-2026-08.md`, in the
  lane's own PR, as a new numbered finding or as a member of an existing
  roll-up.
- **A finding about the kernel's logic** → a **GitHub issue**, signed, never a
  smell-doc row. Track I fixes prose, tests and structure; a logic defect is
  someone else's lane and needs a register that executes. **This track's
  characteristic hazard is the reverse of Track G's**: its ground is where the
  scan's *reachable wrong answers* live (#723, #862), so the temptation is to
  fix the defect while standing next to it. Don't. **A correctness defect does
  not get fixed in a style pass** (Evan, 2026-08-21, correcting §D's own
  earlier draft of I1).
- **An important design question** → a **PR asking Evan**, per
  `memories/git-workflow.md` — the doc edited to state the question, updated in
  place with the answer. Never a comment on a merged PR. **S65 is already known
  to be one of these** (§D marks it Evan-only), and **S82** is a second
  candidate the track must resolve rather than absorb.

## Recording convention

**The landing PR carries its own record**, so the concurrent orchestrators
never read a document that is behind the tree. Each unit makes two edits to
`docs/SMELL-SCAN-2026-08.md` in its own PR:

1. the finding's heading becomes `## SNN. FIXED by #NNN — …`, and its
   **original problem statement is replaced** by the record of what was done.
   Version control keeps the original; leaving it in place makes a closed
   finding read as open. A roll-up **member** gets the same treatment at its
   own bullet.
2. the unit's **row leaves §D's Track I table**, per §D's own *live rows only*
   rule — and the surrounding prose is checked too, because §D's Track I
   preamble names rows by name.

**Conflicts in that file are expected and survivable**, and there are several
live orchestrators editing it. Resolve by merging `origin/main` — **never
rebase, never force-push** — and keep both sides; the edits are to different
findings and different rows. **If the only conflict was that document and CI
was already green on the pre-merge head, merge without waiting for a second CI
run** (Evan, 2026-08-20, carried forward).

---

## Rulings made in this track

| # | The question | Ruling | By |
|---|---|---|---|
| **I-R1** | §D says *"S64 and S65 are one conversation. S65 is Evan-only and S64 should not be closed without it."* Taken literally that stalls S64 — a false sentence in a shipped crate header — behind a decision nobody has asked Evan for yet. | **S64 lands; the mesh crate's ε story does not get to read as settled while S65 is open.** The lane fixes `lib.rs`'s false sentence and the stale three-consumer count **and, in the same PR, opens the S65 question to Evan** with the two options priced. What §D is protecting against is a reader who finishes the ε ledger believing the crate's ε/watertightness story is closed; a pointer at the claim site to the open decision discharges that, and holding a false sentence in the tree to preserve a coupling does not. **S64's `FIXED by` record names S65 as open and Evan's.** | orchestrator, 2026-08-21 |
| **I-R2** | S60 has two halves with very different owners: the acceptance row that cannot go red, and the fact that `area.width()` is read nowhere in `quad.rs`. Is the second one this track's? | **No — and #472 already said so in writing.** Its deviation says *"Metering against `area.lo()` is the certified-conservative gauge and deserves its own proposal with re-measured floors — not smuggling under a guard."* That is a kernel-logic proposal, and per the routing above it gets a **GitHub issue**, not a smell row and not a patch. **What this track owes it is the measurement that makes it answerable**: the lane measures the actual `area_pad`/`surface_area` ratios across the in-tree acceptance fixtures and attaches them to the issue. **The test ceiling is style and lands here** — `area_pad` gets a row that goes red when the enclosure *degrades*, which is S26's own stated lesson and the thing eight months of passes have walked past. | orchestrator, 2026-08-21 |
| **I-R3** | S112(d) is listed under I1, and the frozen table tracks it at **C-m/C3** — a Track C row that is **struck** until #723 is fixed, on a **closed** track. | **It is I-a's, and it does not go down with C-m.** C-m is `quad.rs`'s four quadrature engines; S112(d) is a **sentence in `props/curved.rs`** (`cone_arm`'s doc on the `T::one()` fallback), which the consolidation neither owns nor touches. Leaving it pointed at a struck row on a closed track is §C3's exact failure — a deferral in a register that does not execute — and it is the third such cell this scan has found. **Routed to I-a**, whose file it already is; S112's ledger records the lane and PR that closed it, per G-R2's rule that a class row retires on the ledger, not on a member. | orchestrator, 2026-08-21 |
| **I-R4** | I4 and I5 are separate §D rows. | **One lane.** Both are claims in `boolean/boxes.rs`'s module doc about `census.rs` — I5 *is* the citation that I4's *"looseness is free"* paragraph leans on. Splitting them puts two lanes in one header, and the second would be rewriting the first's sentence. The §D rows stay two and **both leave together**. | orchestrator, 2026-08-21 |
| **I-R5** | I6 is *"roll-up members in these crates"* — a bag of three (S114(f), S115(d), S116(g)) with no shared mechanism. | **Split by mechanism into the lanes whose files they already are**, not run as a bag. **S115(d)** (`walk.rs`'s disclosed D2-addendum deviation) and **S116(g)** (`curved.rs`'s 60%-comment answer to S28) are *prose whose enforcement is missing* → the prose lane, **I-c**. **S114(f)** (`planar.rs`/`trimmed.rs`'s two #678-sibling comments resting on facts that live in another module) is a *guard that was written as a comment* → the guards lane, **I-e**. A bag row run as a bag is how the fifth instance gets found by the accident that found the first. **I6 leaves §D when all three members are recorded, not when one lane lands.** | orchestrator, 2026-08-21 |
| **I-R6** | §D's constitution paragraph names **I1, I2, I3** adversarial *"at minimum"*. Applied literally that is three of five lanes, on a track whose subject is style. | **Narrowed to two, with the criterion applied per lane rather than per §D row.** **I-a is adversarial** — S81 unifies two live certification predicates that disagree on their *lever arm by ~4×* on the torus, and S80 makes a predicate run where it currently does not; both change which faces certify, and every existing row passes either way. **I-e is adversarial** — S108's admit test and S109's accumulator decide whether a mesh is emitted or refused, and the tests that would see it are `#![cfg(feature = "budget")]`. **I-b, I-c and I-d are style-only**: I-b is test rows plus an issue (a row that reds is *visible*, which is the opposite of the criterion), I-c is prose plus a design question, I-d is a module doc plus acceptance rows. §D's list was written by Track F on closing, before any of these rows had a file set; this is the criterion applied to the file sets. | orchestrator, 2026-08-21 |

## Number reservation

**Track I holds `D160`–`D179` and `S230`–`S249`** (§D, published rather than
claimed in a lane message). Re-derived at constitution against the tree at
`5d4b88ab`: the maxima outside the reservation sentences are **`D139`** and
**`S209`**, so both blocks are clear. **Re-derive after every merge** — per
G-R13 a block cannot protect against a number arriving from another track,
only re-checking can. Track H holds `D140`–`D159` / `S210`–`S229`.

**Numbers are assigned by the orchestrator.** A lane asks; it never takes the
next visible gap. Two lanes on another track minted the same number an hour
apart doing exactly that.

| number | spent by | for |
|---|---|---|
| — | — | — |

---

## The standing lane header

**Committed, not kept in a container.** Binding on every Track I implementer
lane, alongside the unit's own brief.

**Read first, in this order:** `docs/prompts/implementer-discipline.md` in
full; this file's *Review policy*, *What a lane does with what it finds*,
*Recording convention* and *Rulings* sections; then your finding's own text in
`docs/SMELL-SCAN-2026-08.md`, and §D's Track I row for it.

**This track is outside the model A/B experiment.** No pairing, no ordinal, no
row in `docs/MODEL-AB-LOG.md`. **Never open that file.**

**Where your files go.** Your clone is `~/.local/share/cad-work/<lane>/cad`,
created with `local-scripts/new-lane.sh <lane> <branch>` — never a hand-rolled
`git clone`, which silently lacks the committed pre-push fmt hook. `export
CARGO_TARGET_DIR=~/.local/share/cad-work/<lane>/target`, never shared with
another lane — a shared one will serve you another lane's binary, and it has
already produced a green claim over ten broken assertions. Heavy cargo goes
through `local-scripts/with-build-slot.sh` (machine-wide mutex, width 1).
**PR bodies and any other to-be-published text go to
`~/.local/share/cad-work/<lane>-pr.md`** — never the session scratchpad, which
is shared between concurrently running agents. **Disk is tight** (~29 GB at
track start, and each `target/` grows to 4–8 GB): do not start a second
`target/`, and say so if you need one.

**Commit and push at every seam.** Everything pushed survives a container
reclaim; nothing else does. If your brief does not name your seams, invent them
and say what they were.

**Recording your own completion.** Your PR makes the two edits to
`docs/SMELL-SCAN-2026-08.md` described under *Recording convention* above, and
**deletes your roster row in this file**. Row and finding numbers come from the
orchestrator. Conflicts in these two files are expected: merge `origin/main`,
never rebase, never force-push, and keep both sides.

**A brief is a claim site.** If a line number, path or citation in your brief —
or in §D's Scope cell, or in the finding itself — does not resolve, **check
rather than comply**, and report what the line actually contains. Several of
this scan's briefs on other tracks carried one that did not; two of this
track's own rulings (**I-R3**, **I-R5**) correct §D cells rather than comply
with them. The orchestrator re-derived every citation in the five findings
below against `5d4b88ab` before dispatch and **three had drifted** — they are
cited by target name in your brief for that reason.

**What Track I units are especially exposed to.** This track's ground is where
the scan's *reachable wrong answers* live. Three shapes to write against:

- **Fixing the defect instead of the smell.** #723 (a wrong certified volume
  in `props/`) and #862 (the cylinder box's over-width) are open **issues**,
  and they are next to almost every row here. Your row is the style half. If
  your fix would change which faces certify or which probes refuse, that is
  not automatically wrong — but it is a thing to say out loud in the PR body,
  measure, and hand to your reviewer as the first claim to falsify.
- **A test that cannot go red.** Four of the five lanes touch an acceptance row
  whose assertions are *monotone in the degrading direction* — `pad > 0` plus
  containment, `holds(&box, sample)`, `worst_ratio ≤ 1`. Q3 is this track's
  highest-yield question. **A row you add must be able to fail**, and you must
  say what makes it fail.
- **Replacing a stale count with a fresh one.** Two rows here are enumerations
  that went stale (`"three places and no more"`, the box consumers). A right
  list leaves the next reader with the same unguarded list. Ask whether the
  claim can be **computed**, deleted, or narrowed to what its evidence
  supports — and if it can only be restated, say at the claim site why.
  **Do not mint a thirteenth hand-rolled source-text reader** to compute it
  (S117); if a guard is the answer, reuse the shared `fixtures::code_only`
  walk that #834 established, or say why it does not reach.

**Write claims you can survive having re-derived rather than re-read.** State
the qualifier that makes a claim exactly true, and scope your evidence out
loud: a green `-p onecrate` run is evidence about one crate. **A measurement is
a measurement of a tree** — name which tree each number came from.

**Do not resolve an Evan-only decision.** **S65** (the watertightness backstop's
`cfg(debug_assertions)`) and **S82** (the sphere rim predicate's
accepting-direction understatement) are Evan's, and both sit inside files this
track edits. Fix what your row names and leave those alone; where your row
makes one cheaper or harder to answer, say so.

**Your final report**, ≤150 lines, states: what you changed and why that shape;
what you swept with and **what that pattern could not match**; every claim
resting on a measurement and what guards it; which of the style brief's
questions you exercised; and anything you are holding back — you will be asked
before the merge, so answering saves a round.

---

## Lane roster

**Wave 1 — opened 2026-08-21, four lanes, no file overlap between any two.**
`crates/mesh/` carries two of this track's five lanes and they are
**sequenced, not concurrent**: I-e waits on I-c because both read
`mesh/src/curved.rs`, from opposite ends (I-c its module header and the ε
ledger, I-e `entries_off_bbox` and the guards).

| lane | rows | scope | review | state |
|---|---|---|---|---|
| **I-a** | **I1** minus S60 — **S77, S80, S81, S112(d)** | `geom-brep/src/props/{mod.rs,curved.rs}` | **ADVERSARIAL** + style | dispatching |
| **I-c** | **I2** (**S64**; **S65** to Evan) + **I6**'s **S115(d)**, **S116(g)** | `mesh/src/{lib.rs,sizing.rs,walk.rs}`, `mesh/src/curved.rs` **header only** | style | dispatching |
| **I-d** | **I4** + **I5** — **S66**'s style halves, **S97** | `topo/src/boolean/boxes.rs`, `topo/src/census.rs` (**doc only**), `sweep/tests/s16_box_soundness.rs` | style | dispatching |
| **I-e** | **I3** (**S108**, **S109**) + **I6**'s **S114(f)** | `mesh/src/{curved.rs,trimmed.rs,planar.rs,budget.rs}`, `mesh/tests/budget_meter.rs` | **ADVERSARIAL** + style | **sequenced behind I-c** |

**Struck from this track's schedule, with a pointer rather than a deletion:**
**C-m** (S27, `props/quad.rs`'s four quadrature engines) — **not scheduled
here**, because #723 reports a wrong certified volume in the very file it
consolidates and the fix comes first. The lane is described in a comment on
**#723**. I-b works in that file and **must not consolidate anything**.

---

## Reviews

| lane | PR | lanes | state |
|---|---|---|---|
| — | — | — | — |

## Landings

*(none yet)*

## Incidents

*(none yet)*
