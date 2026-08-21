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
| **I-R7** | **S116(g)'s residue** — I-c narrowed the finding at `curved.rs`'s module header, and the bulk that remains is in `entries_off_bbox`, `require_swept_rectangle` and `pole_columns`, which are **I-e's** function bodies. Whose is it? | **I-e's**, and the row stays *narrowed, not FIXED*. Flagged by I-c rather than acted on, which is the right instinct — a lane does not move a row into a sibling's scope on its own. It goes to I-e because a reviewer reading those bodies has the behaviour question and the doc-bulk question in front of them at once, and splitting them puts two lanes in one function. **I-c's re-derivation corrects the finding's numbers and my own brief**: the file's production half is **681 lines / 404 comment (59%) / 259 code**, not 712/429; the guard functions carry **146 doc over 44 code**, not ~180/~55; the sharpest ratio in the file is one the finding missed, **`pole_columns` at 82 doc lines over a 3-line body**; and the **1,630** I quoted in I-c's brief was the *test* half, not the argument. The corrections go in the document, not in a report only I read. | orchestrator, 2026-08-21 |
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
| **S233** | **I-a** (#877) | `geom-brep/tests/rim_dim_scale_twins.rs` states `Band { zero: 1e-7, escalate: 1e-6 }` in a comment while the tree's `DEFAULT_EPS` is **`1e-9`** — two decades off, in a file whose subject is the dimensional scaling of predicate margins. **The mechanism is the finding**: a lane took the comment for the constant and built a row on it that then **passed for the wrong reason at the default ε**. The file is off CI's roster and `probe-suite-census.sh` registers that fact — **but the registration covers the file's non-execution, not the constants inside it**, which is the gap |
| **S232** | **I-d** (#876) | `geom/src/surfaces/boxes.rs` — S66(b)'s defect one crate down, on `nurbs_surface_aabb`, whose box `face_box` hands to four doors, three of which do not prune. **Routed to Track H**, whose scope is `geom-core/` and `geom/`; the lane had read it as unowned. **The contrast with S230/S231 is the point** — this row has an owner and says so |
| **S231** | **I-c** (#872) | `mesh/src/chords.rs`'s *"These tightenings are the only places adjacent surfaces enter chord counts"* — S64's shape, in a file inside Track I's crate scope but in **none** of its five lanes' file sets. **Recorded unowned**, and deliberately not routed to I-e: widening a running lane's brief by writing a row at it is how a lane discovers its scope grew after dispatch |
| **S230** | **I-b** (#873) | S60's class, two live members outside every live track's ground: a containment-only `volume_pad` row on the same tilted-cut fixture that never reads `area_pad`, and a python row bounding `volume_pad` at 1e-6 while saying nothing about an `area_pad` measured at **0.199 m²** on the same loft. **Recorded as unrouted, and it says so** |

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

**Wave 1 — opened 2026-08-21, four lanes; I-e sequenced behind I-c.** No file
overlap between any two concurrent lanes. *(The table below shows five rows
because I-e is listed with its gate; four were opened.)*
`crates/mesh/` carries two of this track's five lanes and they are
**sequenced, not concurrent**: I-e waits on I-c because both read
`mesh/src/curved.rs`, from opposite ends (I-c its module header and the ε
ledger, I-e `entries_off_bbox` and the guards).

| lane | rows | scope | review | state |
|---|---|---|---|---|
| **I-a** | **I1** minus S60 — **S77, S80, S81, S112(d)** | `geom-brep/src/props/{mod.rs,curved.rs}` | **ADVERSARIAL** + style | dispatching |
| **I-b** | **I1**'s **S60** | `geom-brep/src/props/quad.rs`, `sweep/tests/m5_pr11_quad_props.rs`, `sweep/tests/m6_loft_body.rs` | style | dispatching |
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
| **I-c** | **#872** (S64, S115(d), S116(g); issue **#868**; `S231`) | style only, per **I-R6** | **running.** Handed over as the load-bearing claim: **is the two-kind taxonomy exhaustive** — is there an ε read in `mesh` that is neither a refuse/report bar nor a classification picking an emitted `f64`? If a third kind exists the unit swapped a falsifiable wrong list for an **unfalsifiable wrong taxonomy**, which is worse than what it replaced. Also handed over: whether the `fixtures::code_only` unreachability that justifies not computing is real; whether *"for every body this build can mint"* is weaker than the evidence supports; and the lane's own disclosed blind spot, **three sites restating the #653 per-edge/per-side argument**, which the lane moved out of its report and into the PR body as falsification claim 6 so the reviewer meets it as a disclosure rather than never meeting it |
| **I-b** | **#873** (S60; issue **#870** for the metering half; `S230` for the residue) | style only, per **I-R6** | **running.** The sharpest question is handed over in the lane's own words and unanswered by me: the m5 ceiling **bites on CI's ε = 1e-6 leg and not on the default leg**, where it absorbs ~1500× before firing — *is declining an ε-aware ceiling discipline, or the easier thing?* The lane's argument for declining is that a per-ε table in a test file is how a deferred metering rule gets smuggled past its deferral; the argument against is that a row which cannot fire on the leg a developer actually runs is uncomfortably near the defect S60 is about. Also handed over: whether `3e-4` is anchored on a **structural** maximum or on the one knob the lane turned, and whether the 3× / 1.5× headroom asymmetry is derived or fitted |

## Landings

### I-c — **S64**, the mesh ε ledger, #872

**The ε roster was DELETED, not corrected and not computed** — and the reason
is written at the claim site rather than in the PR body. `Tol` no longer says
*"ε reaches three places from here and no more"*; it says what an ε read may
**do**, in two kinds: bars that refuse or report, and bars that **classify**,
whose answer picks which `f64` an emitted entry carries. A list can go stale; a
taxonomy of what a read can be cannot go stale the same way.

**Computing it was rejected on a checked reachability fact**, which is what
makes the rejection a finding rather than a preference: `topo`'s shared
`fixtures::code_only` walk is `pub(crate)` and does not reach `mesh`, so the
only computed form available was a private copy — **S117's thirteenth
hand-rolled source reader**, in a PR whose subject is unmechanized claims.

**The residue is disclosed and is real**: a new ε read still has to be
classified by hand. What changed is that **no claim in the crate can be
falsified by a list going stale** — which is weaker than a mechanism and was
the best available.

**The diagnosis of the headline sentence is the sharpest thing in the unit.**
`lib.rs`'s false claim was not a wrong fact; it was **the em-dash**. *"ε is
never read for sizing — mesh structure is a function of (body, δ) alone"* reads
the second clause as a restatement of the first, and only the first is true.
Two claims now, with the second scoped to *"every body this build can mint"* —
empirical, and labelled as such.

**S65 was equipped rather than taken (I-R1), and the question was CORRECTED
before it was priced.** The finding framed release enforcement as *"an
O(triangles) re-derivation against D9's never a panic"* — but **D9 forbids that
panic outright**, so the real option is a **typed refusal**, which is a
behaviour change rather than a cost: bodies that today return `Ok` with a
silently non-manifold mesh would start refusing. The original question was
unanswerable as posed. Priced by **making the guard real in release** rather
than by modelling it: **+10–30% of tessellation time on bodies whose curved
faces all carry a pole, and exactly 0 on a pole-free body** (the block is inside
`if has_pole`) — and most of that is two heap allocations per pole patch, so
**the table is an upper bound on the option, not its floor.**

**S116(g) was narrowed, not fixed, and the lane said so** rather than writing
`FIXED` over a residue — §C13 resisted at the one moment it is tempting.
Residue routed to I-e by **I-R7**.

**Corrections it published, including to me.** The finding's line counts had
drifted *downward* (**681/404/259**, not 712/429); the guard functions carry
**146 doc over 44 code**, not ~180/~55; the sharpest ratio in the file is one
the finding never named, **`pole_columns` at 82 doc lines over a 3-line body**;
and **the 1,630 in my own dispatch brief was the test half, not the argument**.
All four went into the document rather than into a report only I read.

**Raised on the way: `S231`** (`chords.rs`, recorded unowned) and issue
**#868** (the missing typed warning channel, which S115(d) had disclosed and
nobody had filed) — plus **a second copy of that same disclosure** at
`closing_column` that no finding had noticed.

### I-b — **S60**, the area enclosure's acceptance rows, #873

**Two test files, two documents, no kernel change** — `quad.rs` is not in the
diff, which is **I-R2** executed rather than merely obeyed. `area_pad` gets a
ceiling at both of its tightness-relevant sites, and the in-kernel metering
went to **issue #870** with the measurement attached, because #472 had already
deferred it in writing.

**The measurement is the deliverable, and it is what #870 was missing.** On the
patch lane (`m6_loft_body`'s `shape_iii_sections`) the area bracket is
**7.8e-3 relative while the same body's volume bracket is 1.2e-14** — eleven
orders apart, on one body. That number is S26 stated as a fact rather than as a
worry, and it is the first time this scan has had one. The area pad there is
**bit-identical at all three ε legs** and exactly O(h) in `QUAD2_AREA_PIECES`
(64/32/16/8 → 0.1986 / 0.3971 / 0.7943 / 1.5885), so the ceiling is set below
what one halving would produce.

**Every new row was demonstrated red**, by degrading `quad.rs` locally and
reverting: `QUAD2_AREA_PIECES` 64→32 reds m6; `QUAD_INIT_PIECES` 16→4 at
ε = 1e-6 reds m5; a hand-widened `area` reds m5 at default ε. In all three
`area_pad > 0.0` and containment stayed green — which is the finding
reproduced as a demonstration, and the right way to prove a row can fail.

**Two facts found while measuring, both in #870:** `cylinder_cut_face` does not
read `QUAD2_AREA_PIECES` at all, and on that lane the area and flux pads are
rigidly coupled through the shared `a_s` (ratio exactly 6.000 at every ε, the
body's `r = 0.5`). So the unmetered risk is concentrated in the **patch** lanes
— which is a sharper work order than S26's own text ever produced.

## Incidents

### The same defect, committed by the orchestrator, in the PR documenting it

**2026-08-21, two hours after the entry below.** I reopened #723, wrote the
diagnosis, and **quoted the responsible sentence verbatim into the body of
#879** — the orchestrator-sync PR whose whole purpose was to land the record of
that incident. Merging #879 **closed #723 again.** GitHub now lists two merged
PRs as having closed it; neither changed a line of arithmetic.

In the interval I had told lane I-c to sweep its own PR body for exactly this
pattern before merging. I did not sweep mine.

**Caught by lane I-a's style reviewer**, which checked the issue's live state
instead of taking the brief's word for it. The brief in question was mine, and
the rule it broke is this file's own: *a brief is a claim site*.

**What it changes about the finding, and this is why the entry is worth its
space.** The entry below concludes that the hazard lives in how PR bodies
narrate parked lanes. True, and too narrow. The sharper statement is that
**there is no way to write about this failure mode in a PR body without
triggering it**: quoting the sentence fires it, describing it accurately fires
it, and any of the seven keywords adjacent to the reference fires it under any
grammar or negation. **A postmortem of an accidental close is itself an
accidental close.** The only safe forms break the token adjacency — dropping
the `#`, or putting a word between keyword and reference — which is a
by-hand workaround applied by whoever remembers, and nothing checks it.

Two independent authors hit this on one issue inside three hours. That is the
argument for a mechanical guard, and it is recorded here rather than filed as
work on #723, which is about a wrong certified volume and should not accumulate
process rows.

**Standing rule for this track, effective now: every PR body written by this
orchestrator or its lanes is scanned for `(close|closes|closed|fix|fixes|fixed|
resolve|resolves|resolved)` immediately followed by an issue reference, before
the PR is opened or updated.** I-c ran exactly that scan on its own body when
told to, and reported zero hits — so the check works when it is run. The
failure was that I exempted myself from an instruction I had just given.

### #723 was closed by a document describing it, and three schedules were parked behind it

**Found by lane I-b while measuring; verified and reopened by the orchestrator,
2026-08-21.** GitHub closed **#723** — *a wrong certified volume where a sphere
meridian arc crosses a pole* — as `completed`, attributed to **#863**, the
track-definition PR that created Tracks H and I. #863 edits
`docs/SMELL-SCAN-2026-08.md` **and nothing else**: no arithmetic, no predicate,
no test. It picked up the close from a keyword in **#863's PR body** — not
from its diff, and not from any of its four commit messages. The operative
sentence, verbatim:

> Whoever **closes #723** finds the lane waiting there.

**An English sentence *about* somebody closing the issue, parsed as an
instruction to close it.** `closes #723` is the keyword form regardless of the
grammar around it — no negation, tense or subject analysis — so *"whoever
closes"*, *"nobody has closed"* and *"do not close"* all fire identically.
**#863 was a PR whose entire purpose was to say "do not touch this until #723
is fixed", and merging it marked #723 fixed.**

**Three schedules were parked behind that register while it read `completed`:**
Track C's **C-m** (struck *until #723 is fixed*), Track I's **I1** (the style
half, which stands on its own *because* the correctness defect is #723's), and
**S82**, which is in front of Evan partly as *"is this a #723 sibling?"*.

This is **§C3** — *deferrals must land in a register that executes* — for at
least the fourth time on this scan, and the first time where **the register was
retired by a document describing it**. The three earlier instances were
pointers that resolved to nothing; this one is a pointer that resolved to
*success*, which is strictly worse: a reader checking C-m's gate would have
found it green.

**The generalisation, and note where it does NOT bite.** The diff is
irrelevant: editing the document would not have prevented this and will not
prevent the next one. What carries the hazard is the **PR body**, and this
scan's PR bodies narrate parked lanes constantly — *"whoever closes #N
finds…"*, *"stays open until #N is fixed"*, *"resolves once #N lands"*. Every
one of those is one word from retiring the register its own lane is waiting on.
Nothing in the repo guards it, and the failure is silent in both directions:
the issue closes without a fix, and the PR that closed it says nothing about
having done so.

**My own first write-up of this got it wrong in exactly the instructive way** —
I put the keyword in §D's prose, because that is where I had read the sentence,
and both documents contain a variant of it. The published correction is on
#723. A wrong diagnosis here is worse than none: it aims the fix at the file
instead of at the workflow.
