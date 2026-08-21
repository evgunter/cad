# SMELL-SCAN Track G — orchestrator log

**Constituted 2026-08-20.** Track G is the second scan's *unowned ground*
track: `interval-transcendentals/`, `demos/`, `profile/` and `sweep/src/`
outside `fillet/` are code the first scan explicitly excluded and no other
track is live in, plus the rows whose shared mechanism is *a consolidation
or deletion pass removed the prose that was its own evidence and left a
positive claim behind that is now false.* §D of
`docs/SMELL-SCAN-2026-08.md` remains the schedule — this file is the
execution record: rulings, lane state, review outcomes and incidents.
**Live status is here and in §D, never in `memories/`.**

**This track runs entirely outside the model A/B experiment.** No
Fable/Opus pairing, no ordinal, no row in `docs/MODEL-AB-LOG.md` —
**nothing on this track reads or edits that file.** The experiment is
paused on a model limit (Evan, 2026-08-20); the cheapest guarantee that
the pause stays clean is that this track never touches it. A lane that
believes it needs to is wrong and should ask.

**Branch prefix:** `smellg/` for units; the orchestrator sits on
`claude/track-g-smell-scan-h3xhe4`.

---

## Review policy for this track

Not the full orchestrator protocol. Per Evan, 2026-08-20, and identical
to Track F's:

- **Style review on every unit** — `docs/prompts/reviewer-style-lane.md`,
  dispatched **by path** (read it once; never paste it), with the
  per-lane emphasis a dispatch owes (`docs/REVIEW-STYLE-DISPATCH.md`).
  On top of the standing brief, every Track G style review answers two
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

**Three of ten rows are adversarial, plus one sub-unit.** G1 (the pad
constraints — the unit's own failure mode is an assertion that fires on a
sound enclosure, in the crate every certified bound in the kernel is
downstream of), G4 (the admitting set — a trait bound change that decides
which scalars instantiate a whole arc surface), G6 (the wildcard — the
unit converts a silent classification into a compile error and can get
the classification wrong), and G8's `chord_join` sub-unit, which its own
§D row already fences off from the doc edit it travels with.

**Why the criterion needs restating here.** Track G's characteristic
subject is *prose that outlived its referent*. Most of its units are
therefore doc edits, where the risk is not regression but **restating
the falsehood one level up** — writing a new sentence that is true today
and unenforced, in place of an old one that was true once. That is a
style exposure, not a correctness one, and it is what the style lane's
two extra questions are pointed at.

## What a lane does with what it finds

Three destinations, and a lane picks by the finding's kind, not by its size:

- **A new style finding** → recorded in `docs/SMELL-SCAN-2026-08.md`, in
  the lane's own PR, as a new numbered finding or as a member of an
  existing roll-up.
- **A finding about the kernel's logic** → a **GitHub issue**, signed,
  never a smell-doc row. Track G fixes prose, tests and structure; a
  logic defect is someone else's lane and needs a register that executes.
- **An important design question** → a **PR asking Evan**, per
  `memories/git-workflow.md` — the doc edited to state the question,
  updated in place with the answer. Never a comment on a merged PR.
  **G2's S114(c) is already known to be one of these** (§D: *"a design
  row, not a patch"*), and it is the only one this track can see in
  advance.

## Recording convention

**The landing PR carries its own record**, so the concurrent
orchestrators never read a document that is behind the tree. Each unit
makes two edits to `docs/SMELL-SCAN-2026-08.md` in its own PR:

1. the finding's heading becomes `## SNN. FIXED by #NNN — …`, and its
   **original problem statement is replaced** by the record of what was
   done. Version control keeps the original; leaving it in place makes a
   closed finding read as open. A roll-up **member** gets the same
   treatment at its own bullet.
2. the unit's **row leaves §D's Track G table**, per §D's own *live rows
   only* rule.

**Conflicts in that file are expected and survivable**, and there are
four live orchestrators editing it. Resolve by merging `origin/main` —
never rebase, never force-push — and keep both sides; the edits are to
different findings and different rows. **If the only conflict was that
document and CI was already green on the pre-merge head, merge without
waiting for a second CI run** (Evan, 2026-08-20).

---

## Rulings made in this track

| # | The question | Ruling | By |
|---|---|---|---|
| **G-R1** | **G2's note says S113(a)(b) collide with Track E's E-b on `demos/tour/Cargo.toml`**, and routes them to E-b to be consumed back. | **The collision does not exist, and there is nothing to consume.** E-b is **#763**, and its file set was read from the branch rather than from §D's Scope cell: sixteen files — thirteen `crates/*/tests/all.rs`, `geom-core/tests/tolerance_init.rs`, `step-import/tests/tier_gate.rs`, `topo/tests/common/mod.rs`, plus `docs/K-REPORT.md` and the scan doc. **Nothing under `demos/`.** S113(a)(b) stay with G2 and are its work. Recorded rather than silently acted on, because a §D cell is a claim site (`SMELL-C-LOG` C-R11) and this is the third such cell on this scan found describing a branch nobody opened. | orchestrator, 2026-08-20 |
| **G-R2** | **G10 says *"three of them are Track C's … the rest are free"*, and the same section's rides-along paragraph says S112(a) is Track E's E-g.** Both cannot be operative, and the row's member count decides how it retires. | **The rides-along paragraph is right; G10's sentence over-counts by one.** S112's eight members: **(a)** `sweep/src/fillet/naming.rs` → **E-g**; **(d)** and **(e)** `geom-brep/` → **Track C**; **(b)** and **(c)** `interval-transcendentals/` → **G-a**, which owns that workspace; **(f)** `profile/src/sugar.rs` → **G-d**, which is already in that file for G5; **(h)** `demos/render.py` → **G-b**, which §D already names. **(g)** `crates/pncad/src/lib.rs` is the only member with no other home. **G10 is therefore re-scoped to (g) plus the class ledger**, and assigned to **G-d**: it closes (f) and (g) in its own files and records at S112 *which lane and which PR* closed each of the eight members. **The row leaves §D when that ledger is complete, not when G-d's own member lands** — a class row retiring on its leader's member is §C13's half-fix, and this document exists partly to stop minting those. | orchestrator, 2026-08-20 |
| **G-R3** | **G1 must edit `.github/workflows/ci.yml`** — S72's overreach claim is at `:1097-1099` and S112(b)'s stale *"stays a by-hand gate"* at `:1106-1107` — and Track E's **#753** holds that file. Track F treated an overlap on `ci.yml` as a hard gate (F-R1). | **G-a opens, and declares the fence rather than assuming it.** #753's `ci.yml` diff is **two hunks**, re-derived from the branch: `:73-79` (the mold retirement comment's script path) and `:302-308` (the test-aggregation step). G1's sites are ~790 lines away, in the `oracle-*` job region. F-R1's gate was real because F8's fix *lands in* the hunk #753 rewrites; this one does not, and holding an edge-free row against a disjoint hunk in a 2000-line file is how a register stops executing (§C3). **The fence is the lane's to publish**: its PR says its `ci.yml` edits are confined to the `oracle-certify` / `oracle-inari` job comments, and that it touched neither the gate roster nor the test-aggregation step. If a conflict arrives anyway, merge `origin/main` — the recorded remedy, and cheap here. | orchestrator, 2026-08-20 |
| **G-R4** | **G8 and G9 both name `topo/src/chord_join.rs`.** §D carries no edge between them. | **The G8 lane opens first; G9 sequences behind it, and they are not merged.** *(This ruling originally said "G-f (G8)", from the planning table since deleted; G8 was dispatched as **G-g**. Rewritten to name the row rather than a letter — a ruling that outlives a roster should not depend on one.)* The two questions are different kinds: G8's is *does `chord_join::face_plane_normal`'s missing `sense_sign` matter, given it feeds `point_in_loop` for ring re-homing* — a predicate, and G8's own row already requires it be a **separate adversarial sub-unit**. G9's is *does the module header's top-level-sibling argument survive its own imports from `splitting/`* — a paragraph. Folding them puts an adversarial correctness unit inside a doc edit, which is exactly what the row forbids one level down. | orchestrator, 2026-08-20 |
| **G-R5** | **S67 quotes `face_normal.rs:26-31` as *"Three such sites exist and are NAMED (smell-scan D6: …)"*.** That sentence is not in the tree. | **The quote is a paraphrase; the substance holds, and the brief quotes the tree.** What `face_normal.rs:26-31` actually says is *"**"One door" is true of these consumers, not of the workspace.** `boolean::solid_contain::face_plane`, `chord_join`'s `face_plane_normal` and `merge_faces.rs` each still carry their own hand-multiply (smell-scan D6)."* — three sites named, `chord_join` among them, so the finding's defect is intact: `chord_join.rs:2020-2026` returns the raw chart normal with **no `sense_sign` at all** (verified), and the paragraph's *"naming them here rather than leaving the claim unqualified is the point"* is what makes an inaccurate list the whole gap. **A brief is a claim site**; three of five briefs in one session on another track carried a citation that did not resolve, and this one would have made a lane rewrite a sentence that does not exist. | **AMENDED 2026-08-20 — this ruling was half wrong, and lane G-g caught it by re-deriving rather than complying.** The sentence **is** in the tree, **verbatim**, at `face_normal.rs:93-98` — the guard test's gap-#1 bullet, which the finding itself cites as `:88-92`. So it was a **line-number misattribution, not a paraphrase**, and my correction was itself the thing it warned about: a claim about the tree made without re-deriving it. Worse for the original ruling and better for the finding: `:26-31` held a **second copy** of the same three names, and a **third** sat at `boolean/reduce.rs:248-251`, which the finding never named. The lane also found three of the finding's five "unlisted" citations off — `rest.rs:512` is the `let sign =` binding (the multiply is `:521`), `validate.rs` is `:2168` not `:2161`, and **`props.rs:264` is not a normal multiply at all** (the ±1 is an argument to `curved_face`'s closed form). **Both corrections verified from the tree by the orchestrator before acceptance.** The finding's shape held; its count was **low, not high**. | orchestrator, 2026-08-20 |
| **G-R6** | **G7's gate on Track E's E-e**, which §D states as *"partly collides … sequence after it"* without naming the file or the PR. | **Confirmed, and it is a file-overlap gate, not a dependency one.** E-e is **#767**, open, in `editor-core/src/eval/`; S106's own load-bearing citation is `editor-core/src/eval/mod.rs:1565-1730` (`feed_step`, the one cross-crate copy that breaks loudly). Same file. **G7 waits for #767 to land**, and per Track E's own E-R4 a lane that later disproves the *reason* for a gate has not disproved the gate — re-read #767's head, not this sentence. | orchestrator, 2026-08-20 |
| **G-R8** | **May `interval-transcendentals`' `DInterval::intersection` be deleted?** S111(c) reads as a deletion warrant — *"outside the crate's declared scope, in neither of `docs/inventory.md`'s lists, and **zero call sites anywhere** (every other public method has 1–1375)"* — and the orchestrator put that framing to Evan as a small take-it-unless-you-object. Evan pushed back: *"is that making it private, or deleting functionality? i also don't really see the benefit of either?"* | **No, and the finding does not survive contact with the tree.** Three corrections, all from `interval-transcendentals/` itself. **(a)** `docs/semantics-diffs.md` §D7 documents the `Trv` cap in full, and **`intersection` is load-bearing inside `hull`'s justification**: §D7's subject is that `hull()` deliberately diverges from 1788 by keeping `min(dec)`, and its stated recourse for a consumer wanting 1788-strict behaviour is that they *"can call `intersection`-style code or drop the decoration themselves."* Deleting it removes the thing a surviving argument points at — **S74's mechanism, committed on the track constituted to catch it.** **(b)** *"Zero call sites anywhere"* is false **as stated**: five, at `ops.rs:196-209`, in `intersection_trv_cap_and_taxonomy`, which pins the `Trv` cap, the empty/disjoint taxonomy and NaI propagation. **AMENDED 2026-08-20, and the amendment matters more than the original point** — #786's style reviewer ran `git grep '\.intersection('` over the whole tree and got **only that `#[cfg(test)]` block**, and the lane's own new inventory table honestly records `intersection | none today`. So the finding's diagnostic — *a `pub` function with no production caller* — was **correct about the code**; what it got wrong was only the **remedy**, because §D7 points at it. My ruling said *"the finding does not survive contact with the tree"*, which overstates it, and the lane wrote that overstatement into the register before the reviewer caught it. **The lane has withdrawn it at the finding, in the PR body and in its Landings row.** This is G-R9's shape a third time and the first instance where the unchecked claim was *mine and exculpatory* — a correction that made my own earlier error look smaller than it was. **(c)** The surviving half indicts the **document**: `inventory.md`'s exact-surface list omits `hull` too, and `hull` is unquestionably used — so the finding's own diagnostic convicts a function nobody would delete. **Disposition:** inventory both set ops (or narrow the scope sentence to what it actually claims), and treat **S116(r)** as what it now is — *a pointer problem, not a missing caveat*: §D7 has the analysis, and the reader who needs it is at the type or at `geom-core/src/interval.rs:135-143`. **Do not restate §D7 at the type**; a second home for that argument is S13's defect. | orchestrator, from Evan's question, 2026-08-20 |
| **G-R9** | **What G-R8 is an instance of.** Two of this track's first three rulings and now a third are the same shape: a finding's citation or count did not survive being checked (**G-R1** a collision that does not exist, **G-R5** a quoted sentence not in the tree, **G-R8** a call-site count off by five and a deletion warrant that inverts). | **The dispatcher checks a finding's *warrant*, not only its line numbers, before building a brief on it.** The reviewer brief already says *"the dispatch is a hypothesis"* and the dispatch notes already say *"check a lane's claim before you build a brief on it"* — G-R8 is that rule failing at the orchestrator, and it failed in the direction that costs most: an unchecked framing arrived at the lane carrying the dispatcher's authority and pointed at a deletion. **What caught it was Evan asking what the benefit was**, which is not a mechanism. So: **a member whose disposition is delete, privatize, or consolidate gets its warrant re-derived from the tree before dispatch**, and every Track G lane is told in its brief to run the same check on its own members and report any that come back overstated as findings in their own right. G-a has been sent the correction and the instruction. | orchestrator, 2026-08-20 |
| **G-R10** | **Is recording `FIXED by #NNN` in the branch that is still under review premature?** #786's style reviewer flagged it (S23): the G-a roster row is deleted and the finding marked FIXED **in the same branch two reviewers are reading**, so *"if either lane lands a MAJOR, the register already says FIXED and the row is already gone from the table people read."* | **The convention is right and the reviewer's worry is answerable, but it was worth naming and is now named.** The record lands **with the merge, not with the push**: nothing on `main` says FIXED until the PR merges, and a PR that comes back NOT CLEARED never merges — its FIXED lead dies with the branch or is rewritten by the fix pass. That is the whole reason the *Recording convention* puts the record in the landing PR: with four orchestrators editing one document, a record written *after* the merge is a second PR that races the other three. **What the reviewer is right about is that this is invisible from inside a review** — a reviewer reading a branch cannot tell a written-but-unmerged record from a landed one. So the convention gains one clause: **a review that finds a MAJOR says so against the branch's own FIXED text**, and the fix pass rewrites that text rather than leaving it. Recorded here rather than only in the reviewer's report, because the next reviewer will ask the same question. | orchestrator, from #786's style review, 2026-08-20 |
| **G-R11** | **May the S114(c) design PR open on G-b's census?** Evan asked what was waiting on him; I said the demo-manifest schema question would come to him *with the census in hand*. The census is complete and decision-ready. | **No — it waits for its verification, and this is G-R9 binding on me one turn after I wrote it.** The census **is** the warrant for the question, it is a single lane's unreviewed survey, and §D's own version of it was already wrong (it says *four readers*; the lane found **five**, the extra being an inline Python reader at `render.sh:326` nobody had counted). Putting an unverified census in front of the project owner is the exact shape G-R9 exists to stop, and *"the finding said four and I passed it through"* is how G-R1 and G-R8 happened. **The style review is verifying three specific claims** — the emitter/reader count, `transparency`'s single producer, and the one I most want independent: that `render.py:51-57` and `render_freecad.py:105-118` encode `View.up` in **opposite directions** and compose to the identity, so the two cameras agree today by coincidence rather than by construction. The PR opens when those come back. **A question to Evan is a claim site with the highest cost of being wrong on the track.** | orchestrator, 2026-08-20 |
| **G-R7** | **G4's gate is recorded in another track's log.** `SMELL-F-LOG`'s sequencing note says *"F-e (F1) lands before G4/S87–S88, per Evan's S87/S88 ruling: the sentence that makes the `CertifiedBounds` conversion safe is currently false, and converting first would leave the ratification requirement unenforced at exactly the moment new code starts relying on it."* | **Read, accepted, and not restated as a mechanism of this track's own.** G4's chain is **G-j ← F1 (Track F's F-e) ← E-a (#753)**, two tracks deep, and none of the three links is Track G's to move. The row is listed as gated in the roster below with the chain written out, because a gate whose reason lives in a file this track's lanes are not told to read is a gate that gets walked through. **Track F owns the reason; Track G owns only the waiting.** | orchestrator, 2026-08-20 |

---

## Number reservation

**`D71`–`D80` and `S127`–`S136` are Track G's**, taken the way Track F took
`D61`–`D70` / `S117`–`S126` and for the same reason: *"take the next
unassigned number from the orchestrator"* was written for lanes inside one
track and does not survive four concurrent orchestrators drawing on one
sequence from branches none of them can see. Recorded in §D so the other
three can read it rather than infer it.

Sub-blocks are allocated to **live lanes only** — an unstarted lane holding
numbers is how a block runs out while half of it is unused. A lane needing
more than its two says so in its report and takes the next free sub-block,
from the orchestrator.

| lane | §D rows | findings |
|---|---|---|
| **G-a** | D71, D72 | S127, S128 |
| ~~**G-b**~~ (landed, #787) | D73, D74 — **unused, returned**; **D79** used | S129 and **S130** used; S135, S136 free |
| ~~**G-c**~~ (landed, #781) | D75–D77 — **unused, returned** | S131, S132, S133 — **all spent** |
| unassigned | D72, D73, D74, D75, D76, D77, D80 | S128, S135, S136 |

**G-a used D71 and D78, and S127 and S134** (see *Landings*); D72 and S128 came
back. **The `unassigned` line above is a reconciliation across three landings**
— G-a's, G-b's and G-c's — assembled while resolving a merge, from each lane's
own record. It is the orchestrator's to confirm, not a lane's to assert.

**G-c's three findings are recorded without §D rows.** S133 routes to lanes
that already own the files (G-d, G-f, G-g) and says so in its own text, so it
needs no row. **S131 and S132 are unscheduled and may want one** — that is a
scheduling call, not a lane's.

---

## Track G's second number block

**`D111`–`D125` and `S167`–`S181` are Track G's**, taken 2026-08-20 when the
first block (`D71`–`D80` / `S127`–`S136`) ran out mid-wave-2. **Derived, not
guessed:** the highest numbers in use anywhere across the three tracks' logs and
the scan were `D110` and `S166`, so this block starts clear of every other
orchestrator's — including the ones in unmerged branches I cannot see, which is
the reason the per-track block exists at all (Track F's reservation note says
why).

Wave-2 allocation, spending the **returned** first-block numbers before touching
the new ones — those came back unused from lanes that declined to mint a row to
have minted one, and a returned number that is never re-spent is a slow leak:

| lane | row | §D rows | findings |
|---|---|---|---|
| **G-d** | G5 | D72 | S128, S135 |
| ~~**G-e**~~ (#833, in review) | G6 | D73, D74 — **unused, returned** | **S136, S167, S168 spent** |
| **G-f** | G7 | D75, D76 | S169, S170 |
| **G-g** | G8 | D77, D80 | S171, S172, S173 |
| **G-h** | G11 | D111, D112 | S174, S175 |
| unassigned | — | D73, D74, D113–D125 | S176–S181 |

**`S136` is spent, by G-e, and the `unassigned` line above no longer lists it.**
It was allocated to G-e in this table and *also* left in the free pool by the
wave-2 reconciliation — my error, caught by the lane, which declined to edit the
table itself on the grounds that the line says reconciliation is the
orchestrator's. That was the right call: **a lane silently correcting a number
table is how two lanes mint the same number**, which is the incident the whole
per-track block exists to prevent.

## The standing lane header

**Committed, not kept in a container.** Track C lost this text twice in one
session — once to a reclaimed container, once to a branch that was pushed and
never merged. *A register that has not landed is not a register, and a brief
that lives only in a home directory is not a brief.* Binding on every Track G
implementer lane, alongside the unit's own brief.

**Read first, in this order:** `docs/prompts/implementer-discipline.md` in
full; this file's *Review policy*, *What a lane does with what it finds*,
*Recording convention* and *Rulings* sections; then your finding's own text in
`docs/SMELL-SCAN-2026-08.md`, and §D's Track G row for it.

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

**Recording your own completion.** Your PR makes two edits to
`docs/SMELL-SCAN-2026-08.md`: the finding's heading becomes
`## SNN. FIXED by #NNN — …` with its **original problem statement replaced** by
the record of what was done (version control keeps the original), and your row
**leaves** §D's Track G table. A roll-up member gets the same treatment at its
own bullet. Check the surrounding prose as well — Track G's preamble names rows
by name, so a landing that leaves the table and stays in the paragraph makes the
paragraph false. Delete your roster row in this file too. **Row and finding
numbers are assigned by the orchestrator** — ask, never take the next visible
gap; two lanes on another track minted the same number an hour apart doing
exactly that. Conflicts in these two files are expected and survivable: resolve
by merging `origin/main`, **never rebase, never force-push**, and keep both
sides.

**A brief is a claim site.** If a line number, path or citation in your brief —
or in §D's Scope cell, or in the finding itself — does not resolve, **check
rather than comply**, and report what the line actually contains. Three of five
briefs in one session on another track carried one that did not; this track's
own rulings already correct two (**G-R1**, **G-R5**), and one of those would
have had a lane rewrite a sentence that is not in the tree.

**What Track G units are especially exposed to.** This track's subject is
largely *prose whose referent moved*, so its characteristic failure is not a
broken build — it is **a new sentence that is true today and enforced by
nothing**. Three shapes to write against:

- *Restating the falsehood one level up.* Replacing a wrong enumeration with a
  right one leaves the next reader with the same unguarded list. Ask whether
  the claim can be computed, deleted, or narrowed to what its evidence
  supports — and if it can only be restated, say at the claim site why.
- *The class re-check treated as the single fix.* **G10 is a class row whose
  deliverable is the sweep, not the instance**, and its row says so outright.
  A lane that fixes its named site and reports the class is a half-fix and
  will be labelled one. G3 was the other, and it landed as **#781**; its
  review is the worked example of the failure mode above this bullet, because
  the pass closed a false prose claim by writing two more that were already
  wrong. **A sweep is owed over the working tree, not only over history** —
  see **S131**.
- *A disclosed blind spot read as a discharge.* Your own *"my pattern could not
  match X"* is a work order, not an absolution — and it is the sentence a
  reviewer will start from.

**Write claims you can survive having re-derived rather than re-read.** State
the qualifier that makes a claim exactly true, and scope your evidence out
loud: a green `-p onecrate` run is evidence about one crate. **A measurement is
a measurement of a tree** — name which tree each number came from.

**Do not resolve an Evan-only decision.** §D's *Decisions only Evan can make*
table sits inside files this track edits — **S116(p)** (`MultipleAxisRuns`'s
permanent-refusal promise) is in `sweep/src/revolve/mod.rs`, which #781 (the
landed G3) edited without touching it, and **S107** is `pncad-py`'s, which G6
touches. Fix what your row names
and leave those alone; if your work makes one of them cheaper or harder to
answer, say so in your report.

**Your final report**, ≤150 lines, states: what you changed and why that shape;
what you swept with and **what that pattern could not match**; every claim
resting on a measurement and what guards it; which of the style brief's
questions you exercised; and anything you are holding back — you will be asked
before the merge, so answering saves a round.

---

## Lane roster

**Wave 2 — opened 2026-08-20, five lanes, after Evan restored the CI budget.**
Gates re-checked against the new `main` rather than against §D's text: Track E's
**E-e landed as #767** and Track C's **C-f as #731**, so `editor-core/` is free;
nothing live sits on `topo/chord_join.rs` or `topo/boolean/`. **G4 remains gated
on Track F's F1 (#791, still open)** — Evan's own S87/S88 sequencing ruling, and
the one constraint on this track that is not mine to lift.

| lane | row | scope | review | state |
|---|---|---|---|---|
| **G-d** | **G5** (S71) | `profile/tests/review_s2.rs`; re-read only of `profile/src/sugar.rs` | style | running |
| **G-e** | **G6** (S104) | `editor-core/src/assembly.rs`, `pncad-py/src/py/doc.rs`, **plus the two files the scan never read** (`editor-core/src/mate.rs`, `pncad-py/src/py/select.rs`) | **ADVERSARIAL** + style | running |
| **G-f** | **G7** (S106) | `profile/src/path/program.rs`, `editor-core/src/{program,persist/wire,eval/mod}.rs` | style | running |
| **G-g** | **G8** (S67) | `topo/src/face_normal.rs` (docs) + `topo/src/chord_join.rs` (the real question) | style **+ one ADVERSARIAL sub-unit** | running |
| **G-h** | **G11** (S114(c)'s residue) | `demos/render.py`, `demos/render_freecad.py`, `demos/wild/src/main.rs`, `demos/tour/src/uvdump.rs` | style | running |

**The constitution-time planning table that used to sit here is DELETED.**
It assigned different lane letters to every row than the live roster above —
G-f to G8, G-g to G9, G-h to G7 — and **three separate lanes tripped on it**:
G-g flagged the collision from inside its own lane rather than guessing which
letter it was, G-d found its own row saying something its brief did not, and
G-h found itself listed against a row it was never given. Marking it *void* was
not enough, because a void table still reads like a table. **Version control
keeps it.** The live roster is the table above and the number-reservation table
is the one under *Track G's second number block*; nothing else in this file
assigns a lane letter.

Recorded rather than quietly deleted, because it is this track's own subject
turned on its own log: **a stale artifact that is still true-looking outlives
every warning attached to it, and the third lane to trip on it was tripping on
the warning.**

**Sequenced, not gated:** **G9** waits on G-g because both edit
`topo/src/chord_join.rs` — G-g's question there is whether a missing
`sense_sign` flip is a defect, G9's is the top-level-sibling placement argument;
different questions, one file. **G10** goes last because its members are
scattered by file and would collide with whatever is open.

**Why G6 is adversarial and G7 is not**, since both are "de-duplicate a
vocabulary": G6's wildcard **decides `AssemblyError::AtRest` vs
`Uncertified`**, and every existing CI row passes whichever way a new arm is
classified — so the unit's own failure mode is invisible to the gate. G7's three
copies go *silently short* rather than wrong, which is a real defect and a
visible one. That is Evan's criterion (`SMELL-C-LOG` C-R12) applied, not row
size.

## Reviews

| lane | PR | lanes | state |
|---|---|---|---|
| **G-a** | **#786** (G1/S72 + 7 members) | **ADVERSARIAL + style**, then a targeted ADVERSARIAL re-check | **CLEARING — one sentence outstanding.** Both first-round reviews split cleanly and both were right: the adversary said **merge** (no MAJOR; it proved claim 1 by reinstating main's four bodies as `*_old` and asserting bit-identity over **12,000,020 intervals × 4 functions**, zero divergences — far harder than the lane's distribution argument, which is blind to a width-preserving endpoint shift), while the style lane supplied the verdict in one finding: the new ceiling returned on `n == 0` **before** its assert. The adversary also corrected the lane **in its favour** — `2·pad+1` counts representable steps but the ratio is on widths, and a step crossing a binade boundary is twice the oracle's ulp, so the bound is `≈4·pad+1` = 17, which is why `atan2` measures 10 against a stated 9 and `atan` hits 12 adversarially. **The fix pass rebuilt the ceiling into three asserts and earned a second adversarial pass on its own terms** — new asserts in the crate every certified bound is downstream of is the criterion exactly. All three CONFIRMED: assert 2's floor is generator *structure* (3/8 of divisors touch zero) so it cannot move on a seed; assert 3's class is empty **by construction** (`log_mag` returns `±m·2^e`, `m ∈ [1,2)`, never 0) and its allowance was measured directly at 2× the worst case the class could ever contain, so it is an entitlement rather than a constant fitted to an empty set. **One MINOR outstanding**, and it is this track's failure mode in miniature: the lane recorded the window-placement dependency correctly **against the wrong assert** — assert 1's real protection is the ~1e-14 measure of a few-ulp band around each `tan` pole, 24 binades below the `2^32` onset, so a future near-pole-but-pole-free generator class would red it on sound output and send the reader to `emax`. Sent as a one- or two-sentence fix; merges on green. **Was: NOT CLEARED**, and the fix pass is held so the lane gets one combined pass rather than two. The review's **S1 is the row failing at its own thesis**: `Tightness::report`'s new ceiling is defeated by exactly the degradation it was added to catch — `record` drops any sample with non-finite our-width or zero oracle-width, and `report` returns on `n == 0` **before** the assert, so an operation that regressed to `entire()` on every draw contributes no ratios, prints *"no finite-ratio samples"*, and passes. No floor on `n`, in a PR that gave its own new fuzz lanes exactly such floors. **S2 answers the lane's own sharpest disclosure against it**: `pad_contract.rs` uses `point(x)` on every row but one, so non-degenerate boxes are constrained only by a scale-free ratio with 6× headroom, and the two instruments are **not** equivalent cover for a two-sided `assert_contains`. **S12** finds the rewritten sentence still standing in `local-scripts/ci-local.sh:432-438` — which has no `oracle_certify` row at all, so *"nothing is left to a convention"* is false on the fallback gate. Twenty-four findings; two corrections to the dispatcher, one of them to **my** account of G-R8 (see below). Original claims: **running.** Claims handed to the falsification lane, ranked by cost: **the `sin`/`cos` and `asin`/`acos` unifications preserve behaviour** — if `cos` is anywhere a *phase shift* of `sin` that is unsound and no containment assertion catches it, and the lane's evidence (*"the tightness distribution is identical in every column"*) is **necessary, not sufficient**, being blind to a systematic endpoint shift that preserves width; **the `pad_contract` bound is derived and cannot fire on sound output**; **the ceilings 8 / 64 have real rather than accidental headroom** — the lane's own report has a measured transcendental maximum of *8–10* against a derived structural bound of *9*, and a measurement exceeding its own derivation by one means either the derivation omits a term or the measurement includes a case it does not model; **`neg_frac_pi_2` did not move** when it changed from a hand-written endpoint pair to `-frac_pi_2()`; **the four FMA-witness spellings really were one function**. The style lane has the disclosure the lane itself called sharpest — `assert_contains` is *still* one-directional for the transcendentals, which was the finding's core complaint |
| **G-b** | **#787** (G2, nine members) | style only, then a targeted re-check | **fix pass done, re-check running.** All ten must-fixes taken and **no disagreements** — because, as the lane put it, *three of the ten were defects its own first fix introduced*, and it names finding 3 as the one it is least happy about, having read G-a's `assert_contains` finding in its own brief and reproduced it anyway one tree over. **That record is why a second pass gets a look**, not ritual. Best fix of the pass: the `LEGEND` was closed **by computing it** against the emitter, and the computation immediately caught two live drifts (`#777777` appearing in no cell; `#333333` against the emitter's `#333`). The volume pin's `9.0` literal is gone — the expected volume is derived from the sections and checked *before* the kernel is asked — and `volume_pad` is now bounded above. The winding alarm became a **fatal assert**, with the lane arguing fatal-in-the-tour over a kernel issue on the ground that **nothing fires today, so an issue now would be a placeholder for a hypothetical, while the assert is the thing that would produce that issue with a witness attached**. Issues **#795** (the typed-refusal exit convention, flagged as possibly Evan's) and **#796** (the shadow tuple algebra beside `Vec3`, in the #757–#759 channel). `S130` + `D79` record the `lily.rs` roll-up. Spent `S129`, `S130`, `D79`; `D73`, `D74`, `S135`, `S136` returned free | CI green (36 success, 1 skipped). Weight put on: **a printed number is not a guarded number** — the lane's governing choice was *compute or delete, never restate*, and every closed count is now *printed by the run that produces it* or is a Rust array length; **the array length enforces, the printing does not**, and a number printed and never compared is S110(b)'s class and the exact defect G-a is fixing one tree over. Also: this unit **deleted a great deal** and a frontier arm now panics where something previously did not, which changes what a render lane sees; S114(b)'s replacement of a false *"VERBATIM"* with a computed volume pin is the right shape but **two different prisms can share a volume**; and `lily.rs` (2,446 lines, §B2's *"sampled, not read"*, the scan's highest-yield uncovered file) is the Q8 candidate |
| **G-c** | **#781** (G3/S74) | style only | **CLEARED and MERGED** (35 checks green, 2 skipped, 0 failed, `k-lint (gate)` included). The fix pass took all seven must-fixes and every judgement call, and **disagreed twice, correctly, on both** — verified independently by the orchestrator before the merge. *(i)* No visibility widening happened: `pub(super)` written in `crate::revolve` **already means** `pub(crate)`, because `revolve` is a child of the crate root — proved by compiling a sibling construction rather than by argument. The substantive worry is real, pre-existing and unchanged by the diff, and is recorded as **S131** rather than presented as fixed. *(ii)* The marker recount is **7 = 5 + 2**, not the reviewer's 4 + 2: `cap_points` and `cosurface` do carry markers. Re-derived by the orchestrator from `6b9c1236` — eight marker lines, one excluded — and the lane **published the criterion** (*a marker is a sentence declaring this item is the same code as a named item elsewhere*), which is what makes a contested count re-derivable rather than asserted. **The best thing in the pass:** S-9/S-14/S-15 were closed **in code, not prose** — the arc rule is now one body, `swept::centre_on_material_side`, called from both verbs, with the `Zero` posture decided once and the reconciling comment deleted. That is the correct answer to *"you closed a prose fence by writing prose fences"*. Spent `S131`, `S132`, `S133`; returned `D75`–`D77`. Took `revolve/axis.rs`, outside §D's Scope cell, and named it in the PR. **Was: NOT CLEARED** The reviewer's verdict on the track's own question: *"the instance is gone; the mechanism is not."* Claim 1 **confirmed two independent ways** and the code shape endorsed (*"how I would have done it"*), with the guarding row named — `m5_s11_concave_sense.rs:165` goes red on a mis-indexed bit. But claim 6 **fails** (a self-declared hand-copy of the involution at `revolve/tube.rs:216-239`, inside scope, that neither sweep surfaced) and claim 5 **fails on its arithmetic** (5+2≠7; re-derived as 4 correct + 2 standing, and two names in the list carried no deleted marker at all). **Two findings are S74's own mechanism committed by the fix for S74** — a replacement funnel count already wrong in the other direction, and S6's own copy of the false sentence left standing by the class re-check that read it. Seven must-fixes; `S131`–`S133` / `D75`–`D77` reassigned to the pass. Reviewer asked, and answered, whether this should have been adversarial: **no** | CI 22 green / 0 failed / 3 in flight at dispatch. Claims handed over for falsification: *geometry does not move* (the index algebra of the reverse arm is the one where a wrong answer ships wrong geometry), *the duplication is removed not relocated*, *loft's orientation bit is now structurally absent*, and the deleted-marker sweep's **"no third"** negative result. Emphasis: **a unification is the shape that mints duplicates**, the lane's two disclosed blind spots are work orders, and the reviewer is asked to say explicitly if the unit should have carried an adversarial review after all |

## Wave-2 review outcomes

**#834 (G-g, G8) — NOT CLEARED on style, Unit 2 CONFIRMED on adversarial.** The
split was worth it and both halves earned their posture.

*Unit 2 is the strongest single result on this track.* The reviewer confirmed
`point_in_loop`'s sign-invariance by reading the real loop (the normal enters at
exactly three places and only there), then by **320k bitwise projection/side-axis
cases, 2M FP-lemma pairs, 14k f64 probes over 5 profiles × 7 planar faces, 1.26k
probes on the certified lane, and a mutation test** — and it holds **for the
structural reason**, so reachability changes only how much is at stake.

*Two reviewers converged on one line from opposite directions*, which is the
strongest signal this process generates. The style lane flagged the pin's
`Debug`-string equality and could only mark it **`unsure`, taste**; the
adversarial lane built the witness — a vertex 3e-9 off the first ray line
escalates with the same variant, predicate and band, but the diagnostic carries
the **signed** margin, so 12 of 12 such refusals differ and the row false-reds.
**Neither lane alone would have produced a fix; together they produced a
witness.** Worth remembering when deciding whether a row gets one review or two.

*And the adversarial lane refuted a limit in the safe direction.* The pin's docs
said it would not fire on an unsigned crossing lever, *"because a closed loop
has an even number of straddles"*; the reviewer **mutated `ray_parity.rs` to
exactly that and the row failed** — the tilted block pins absolute verdicts. A
lane understating its own instrument is rarer than the reverse and still wrong:
the sentence is disproved by the file's own code. (The parity claim is also
*true but vacuous where invoked* — `sides` only ever holds `Positive`/`Negative`,
so adjacent-unequal pairs are even for any cyclic two-valued sequence.)

**#831 (G-d, G5) — NOT CLEARED on style.** The reviewer read all 1,590 lines,
**re-derived the geometry numerically** rather than taking it on report, and
cleared the substance: the new assertion is non-vacuous (a wrong crossing misses
the 1e-9 bound by ~2r), issue #827 does not quietly decide the capability
question, and both scope calls were endorsed. Then it found that **the PR
re-minted S71's shape at the site it was correcting** — `check_corner`'s doc
contradicts itself inside one paragraph, and the replacement still opens with
*"So the class is built"*, the exact phrase the finding was raised about, with a
rider appended. **A rider is a narrowing, not a removal**, and that is now the
third time this track has had to say so.

*The printed-number class, for the third time.* `n_enclosing` is accumulated,
never asserted, and printed with `eprintln!` — which `cargo test` swallows
without `--nocapture` — under a header claiming the sweep *corroborates* the
boundary. Its consequence is worse than the instance: the arm it feeds **would
pass if the door ever did emit an enclosing tangency**, so it cannot be the
tripwire its presence implies.

*A finding worth generalising:* the PR that measures **285 inert `tests/`
intra-doc links** adds **four fresh ones** in the same diff, when plain
backticks were already in use for the same targets two crates over.

*And the amnesty channel, caught by precedent rather than by argument.* The lane
declined a §D row for S135 on the grounds that its disposition is *"a policy
call"* — while **D71 sits in the same register as `ACCEPTED, unstaffed`**, which
is exactly a decision-shaped row with no patch. The channel exists and was used
one screen up. A finding whose fix is *someone should decide something* owes a
named decision-holder, and the same lane routed its other decision correctly, to
a numbered issue with an owner.

## Landings

| lane | row | PR | note |
|---|---|---|---|
| **G-a** | **G1** — S72 + S110(h), S111(c), S112(b)(c), S114(a)(d), S116(r)(t) | **#786** | Fence published per **G-R3**: `ci.yml` hunks confined to the `interval-backend` job's header comment, ~790 lines from #753's. **NOT CLEARED on first review; fix pass landed in the same PR** — the tightness ceiling had reproduced S72's own defect (a max over a sample set the degradation empties), and the structural derivation beside it was wrong in the crate's favour (`4·pad+1`, not `2·pad+1`). One member came back correcting its finding: **S114(d)**'s decoration idiom is five sites, not six. **S111(c)'s first write-up over-corrected and is withdrawn** — the diagnostic was right about the code, only the remedy was wrong; see **G-R8** as amended. New findings taken: **S127**/D71, **S134**/D78. |

### G-b — **G2**, `demos/`, #787

**Seven of the nine members closed**, S114(c) surveyed and left open as §D
required, one new finding and one issue raised.

- **S110(g)(j), S112(h), S113(a)(b), S114(b), S116(d)** — FIXED, each recorded
  at its own bullet in §D's document.
- **S114(c)** — **the census exists and the schema does not.** The lane
  produced the full producer/consumer survey (three emitters, five readers,
  field by field, each disagreement with both sides' `file:line`) and stopped.
  **The design-conversation PR asking Evan is the orchestrator's to open**, and
  the census is in the lane's report. Two halves did not wait for it, because
  they are wrong under every schema: S112(h) itself, and
  `check_render_provenance.py:104,112`'s two *"keep in sync"* claims, which are
  now read out of the scripts and compared in the selftest.
- **G-R1 discharged.** The routing to E-b was void, as ruled; S113(a)(b) were
  this lane's work and there was nothing to consume.
- **Numbers.** D73 and D74 stay free. **S129** (no runner under `demos/`) came
  from the first pass; **S130** and **D79** came from the fix pass — the
  `lily.rs` roll-up the review raised over free ground, recorded and *not*
  fixed, with D79 scheduling it. **S135 and S136 stay free.**

**Fix pass (style review: NOT CLEARED, ten must-fixes).** All ten addressed.
Three mattered: `run_body`'s `Option` return had no `None` path left and four
things still believed it did — **S112's own class, created by the PR that closed
S112(h)** — plus a doc paragraph describing the staged-stop world the code had
left; `render_freecad.py:153` was the *same* guard-against-nothing one file over,
under a new and false docstring claim that the two readers read the field the
same way; and the `loft_prism` volume pin checked a `9.0` literal against an
**unbounded** `volume_pad`, which is G-a's `assert_contains` shape reproduced in
`demos/`. The uv winding contradiction is now **fatal in the tour**, per the
orchestrator's ruling — see the lane's report for which escalation target it
argues for. Two issues filed: **#795** (should a demo surface a typed refusal as
a clean nonzero exit — S110(j)'s deferral, now scheduled) and **#796** (`Vec3`
ergonomics, a library finding in the `memories/demo-purpose.md` sense).

**What it turned up that was not on the list.** Establishing S110(j) required
knowing what runs the tour, and **nothing ran `cargo test` anywhere under
`demos/`** — `--all-targets` clippy type-checks the test targets and runs none
of them, S110(a)'s shape one tree over. Ten assertions unguarded, and **two of
them red on main**: `demos/tour/src/lily.rs`'s finding-13 tessellation pin
disagrees on both SWEPT-blade rows (1016/854 against a pinned 976/826) while
all five analytic rows are exact. That is a `mesh` question, so it is **issue
#782** and the lane did **not** re-baseline. The #99 ε pin is armed in `k-lint`;
the `--bin demo-tour` unit tests are deliberately not, and **S129 stays open**
until #782 decides them.

## Incidents

### `new-lane.sh` could not create a lane in this container

**2026-08-20, at track start.** `local-scripts/new-lane.sh` clones from a
literal `git@github.com:evgunter/cad.git`. This orchestrator's container has
**no `ssh` binary at all** and an **https `origin`**, so the one committed
door for lane creation failed on its first invocation — and it failed in a way
that reads as *"the standard way to create an agent lane is unavailable"*
rather than as a wrong URL, which is exactly the pressure that produces the
hand-rolled `git clone` the script exists to prevent (a hand-rolled clone
silently lacks `core.hooksPath`, so the committed pre-push fmt hook is off).

**Fixed in place rather than worked around**: the URL now comes from the
invoking checkout's `origin`, with the literal kept as a fallback for a
detached invocation. `local-scripts/` is non-triggering for CI by design
(`ci-filter.py`), so this carries no CI cost.

**Worth carrying:** this is the same shape the track was constituted to fix,
one level out — *a claim in prose ("the standard way") that a mechanism no
longer supports*, invisible until someone stood in an environment the author
did not have. It is local tooling and not a smell-scan row, so it is recorded
here and nowhere else.

### The container was reclaimed with five agents live, and one had 7 files uncommitted

**2026-08-21.** The container restarted (`uptime` = 1 min) with **five agents
running**: lane G-f implementing G7, lane G-d's fix pass, lane G-g's fix pass,
and three reviewers (two on #833, one on #837). **All five died. The three
reviewers had produced no output and their work is simply gone.**

**What survived, and why.** Everything pushed. Every lane clone under
`~/.local/share/cad-work/` — those are on a persistent volume, which is the
whole reason `memories/agent-lane-operations.md` forbids putting a working clone
in the session scratchpad. Four of five lanes were clean and fully pushed.

**Lane G-g was not**: **7 modified files, uncommitted**, a substantial fix pass
including the restored `#NNN` protocol template, the `geom-brep` removal, ~154
new lines of shared comment-test home in `fixtures.rs`, and ~90 in the
sign-invariance pin. **Recovered by the orchestrator** and pushed as `a63b05df`
with an explicit `RECOVERY` message stating it is **UNVERIFIED** — never
compiled, never tested, never checked against its brief — so a successor audits
it rather than inheriting it as finished work. (The committed pre-push fmt hook
rejected the first attempt, correctly, and was satisfied rather than bypassed.)

**The lesson is one the standing header already carries and this proves the cost
of:** *commit and push at every seam.* G-g committed nothing for the whole pass
and came within one non-persistent directory of losing all of it. The successor
briefs now say so **with this incident named**, because an abstract rule did not
produce the behaviour and a concrete loss might.

**What the orchestrator got right by accident and should do on purpose:** the
diagnosis started from `uptime` and per-lane `git status`/`@{u}..HEAD` rather
than from assuming, which is what separated *four lanes fine* from *one lane
nearly lost*. **On any suspected restart, that sweep comes first** — before
re-dispatching anything, because a re-dispatch into a dirty clone would have
silently destroyed the recovery.

**And a dispatch change that outlives this incident:** the two lost implementer
assignments had been delivered **only as messages** to agents that then died, so
they died too. Briefs now go to **files** under `~/.local/share/cad-work/`, and
the message points at the path. A message is not a durable channel.

### Reviewers were pointed at the orchestrator's own checkout, and one left it detached

**2026-08-20, and the cause is a brief I wrote five times.** Every wave-1 and
wave-2 **reviewer** dispatch said *"work read-only from `/home/user/cad`"* — the
orchestrator's own working checkout. Read-only was the intent and no reviewer
edited a file; but **`git checkout` is not an edit**, and one reviewer resolving
a branch left the shared checkout in **detached HEAD from
`origin/smellg/g8-face-normal-enumeration`**. The orchestrator's next
`git pull --no-rebase origin main` then merged `main` **into that detached
HEAD**, producing a commit belonging to no branch.

**Nothing was lost** — the last orchestrator commit had already pushed, the
working tree was clean, and `git checkout <branch>` restored it. **But the
failure mode is bad**: for one turn the orchestrator was reading a *lane's*
branch while believing it was reading `main`, and it very nearly filed a defect
against `main` that existed only on an unmerged branch. The tell was a
disagreement between `grep` on the working tree and `git show origin/main:`,
which is the only reason it was caught.

**The rule: reviewers never work in `/home/user/cad`.** `memories/agent-lane-operations.md`
already says working clones go under `~/.local/share/cad-work/<purpose>/`, and I
read that at session start and then wrote the opposite into five briefs, because
"read-only" felt like it made the location harmless. It does not — **a shared
checkout has one HEAD, and a reviewer needs to move it to do its job.** Future
reviewer dispatches get their own clone, or are told to read via
`git show <ref>:<path>` and `git diff <a>...<b>` without ever checking anything
out.

**And for the orchestrator specifically:** `git pull` and `git merge` in a
directory other agents can touch must be preceded by confirming the branch —
`git branch --show-current` returning empty is the whole signal, and it costs
nothing. This is the third incident on this track in the same family: **a
command whose failure or misdirection is silent** (a pipe swallowing a merge's
exit status; a marker-bearing merge landing green; a pull onto a detached HEAD).
The family, not the instances, is the thing to design against.

### The same defect reached `main`, and the orchestrator merged it there

**2026-08-20, ~one hour after the incident below, and this one is the
orchestrator's.** #787's merge commit `6fe672b3` carried **two unresolved
conflict-marker pairs** in `docs/SMELL-G-LOG.md` — the number-reservation table
and the wave-1 roster — and **I merged it to `main` without checking.** The
register was broken on `main` for roughly an hour. `docs/SMELL-SCAN-2026-08.md`
was unaffected; no code file was.

**Found by lane G-a**, resolving its own conflict against that `main` — the same
lane that had hit the identical failure a round earlier and written it up. It
repaired both pairs as part of its merge, and #786 landing is what fixed `main`.

**Why the existing guard did not catch it.** The incident below made me scan
`main`, this branch and every `docs/*.md`: all clean, and I said so. **That scan
was a snapshot, and I then merged two PRs without repeating it.** A one-time
sweep is not a guard — which is this scan's own thesis (S72, S110) applied to
the orchestrator's own process, and it is the second time on this track that a
rule I wrote for lanes bound me first (cf. **G-R9**, **G-R11**).

**The rule, and it is now mechanical:** *before every merge, and again after,*
`git grep -c -e '^<<<<<<< ' -e '^>>>>>>> ' origin/<branch>` over `docs/`,
`crates/`, `.github/`, `scripts/`, `local-scripts/` and
`interval-transcendentals/`. It costs one call. **CI cannot substitute**: a
marker pair inside a Markdown ledger breaks no build, fails no gate, and every
one of #787's 36 checks was green over it. That is the whole reason it reached
`main`.

### A pipe swallowed a merge failure, and conflict markers reached the register

**2026-08-20, lane G-a, self-reported and self-fixed** (`385c9c10`, before its
PR merged). A `git merge … | tail -2 && git commit` chain **took the pipe's exit
status, not the merge's**, so a failed merge reported success and two conflicted
ledger hunks were committed with their markers intact.

**The lane found it, resolved both properly, swept every file it had touched
(0 remaining), and flagged it unprompted** — on the reasoning that *a lane that
pushes conflict markers into the register is a worse failure than the one under
review*. That is the correct instinct and the reason this row exists rather than
a reprimand.

**The mechanism is general and this orchestrator was exposed to it too.** Every
`git pull --no-rebase origin main -q 2>&1 | tail -2` in this log's own history
has the identical defect. Swept `main`, this branch, and every `docs/*.md` on
both: **zero markers, zero unmerged paths** — the orchestrator got away with it,
which is not the same as having been careful.

**The rule, and it costs nothing:** in a chain that commits, **never pipe the
command whose exit status decides whether committing is safe.** Let `git merge`
or `git pull` stand alone, or check `git ls-files -u` before the commit. Four
concurrent orchestrators are merging one 11 000-line document all day; this will
bite someone else.

### Wave 1 was dispatched before this file was on `main`

**2026-08-20, and the mistake is the orchestrator's.** The three wave-1 lanes
were dispatched with briefs pointing at `docs/SMELL-G-LOG.md` **by path**, two
minutes before #772 merged it. G-c hit it immediately: the file was not on
`main`, not on any branch it could see, and not an open PR. It recovered by
working from `SMELL-F-LOG.md`'s equivalent sections — correctly, since #772's
own body says Track G's policy is Track F's verbatim — then merged `main`
mid-lane, read the standing header and rulings, and confirmed nothing
contradicted what it had already done.

**No damage, and the recovery was the right one**, but the near-miss is worth
the row: *point, never paste* (`docs/prompts/README.md`) makes a dispatch a
promise that the path resolves, and this dispatch made that promise against a
branch. **Merge the constitution before dispatching against it** — the ordering
is free and there is no version of this that fails loudly.

It is also the third instance this track has recorded of one shape (**G-R9**):
a claim in a brief that did not survive being checked. The first two were
citations; this one was the brief's own foundation. The lane checked rather
than complied, which is exactly what the standing header asks for — so the
mechanism that is supposed to catch this worked, on its first day, on the
orchestrator.
