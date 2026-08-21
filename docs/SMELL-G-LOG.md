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
| **G-R4** | **G8 and G9 both name `topo/src/chord_join.rs`.** §D carries no edge between them. | **G-f (G8) opens first; G9 sequences behind it, and they are not merged.** The two questions are different kinds: G8's is *does `chord_join::face_plane_normal`'s missing `sense_sign` matter, given it feeds `point_in_loop` for ring re-homing* — a predicate, and G8's own row already requires it be a **separate adversarial sub-unit**. G9's is *does the module header's top-level-sibling argument survive its own imports from `splitting/`* — a paragraph. Folding them puts an adversarial correctness unit inside a doc edit, which is exactly what the row forbids one level down. | orchestrator, 2026-08-20 |
| **G-R5** | **S67 quotes `face_normal.rs:26-31` as *"Three such sites exist and are NAMED (smell-scan D6: …)"*.** That sentence is not in the tree. | **The quote is a paraphrase; the substance holds, and the brief quotes the tree.** What `face_normal.rs:26-31` actually says is *"**"One door" is true of these consumers, not of the workspace.** `boolean::solid_contain::face_plane`, `chord_join`'s `face_plane_normal` and `merge_faces.rs` each still carry their own hand-multiply (smell-scan D6)."* — three sites named, `chord_join` among them, so the finding's defect is intact: `chord_join.rs:2020-2026` returns the raw chart normal with **no `sense_sign` at all** (verified), and the paragraph's *"naming them here rather than leaving the claim unqualified is the point"* is what makes an inaccurate list the whole gap. **A brief is a claim site**; three of five briefs in one session on another track carried a citation that did not resolve, and this one would have made a lane rewrite a sentence that does not exist. | orchestrator, 2026-08-20 |
| **G-R6** | **G7's gate on Track E's E-e**, which §D states as *"partly collides … sequence after it"* without naming the file or the PR. | **Confirmed, and it is a file-overlap gate, not a dependency one.** E-e is **#767**, open, in `editor-core/src/eval/`; S106's own load-bearing citation is `editor-core/src/eval/mod.rs:1565-1730` (`feed_step`, the one cross-crate copy that breaks loudly). Same file. **G7 waits for #767 to land**, and per Track E's own E-R4 a lane that later disproves the *reason* for a gate has not disproved the gate — re-read #767's head, not this sentence. | orchestrator, 2026-08-20 |
| **G-R7** | **G4's gate is recorded in another track's log.** `SMELL-F-LOG`'s sequencing note says *"F-e (F1) lands before G4/S87–S88, per Evan's S87/S88 ruling: the sentence that makes the `CertifiedBounds` conversion safe is currently false, and converting first would leave the ratification requirement unenforced at exactly the moment new code starts relying on it."* | **Read, accepted, and not restated as a mechanism of this track's own.** G4's chain is **G-j ← F1 (Track F's F-e) ← E-a (#753)**, two tracks deep, and none of the three links is Track G's to move. The row is listed as gated in the roster below with the chain written out, because a gate whose reason lives in a file this track's lanes are not told to read is a gate that gets walked through. **Track F owns the reason; Track G owns only the waiting.** | orchestrator, 2026-08-20 |

**Amendment returned by a lane — G-R2, from #831's ledger walk.** G-R2's
conclusions all hold: the rides-along paragraph was right, S112(a) is not this
row's, G10's sentence is wrong by one, and the row does not retire on its
leader's member. **Two things the tree says differently.** (1) The over-count
is in *"three of them are Track C's"*, not in *"the rest are free"* — the free
five ((b)(c)(f)(g)(h)) are exactly right and all Track G's; Track C holds two,
(d) and (e), and the third body in that parenthetical is (e)'s **second file**,
`ssi.rs`, counted as a member. (2) **(a) has no owner at all, not E-g.** E-g
landed as #768, its own §D row records S112(a) as not in it, and that row is
struck — so the ruling's *"→ E-g"* points at a retired lane. That is **S177**
and **D114**, and it also means **G10 cannot retire on this ledger**: (a) and
(e) are named by no live row but G10's, so the row stays, re-scoped to the
residue.

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
| **G-d** (G5 + G10, #831) | **D72**, **D113**, **D114** used | **S128**, **S135**, **S136**, **S176**, **S177** used |
| ~~**G-b**~~ (landed, #787) | D73, D74 — **unused, returned**; **D79** used | S129 and **S130** used; S135 and S136 later taken by G-d |
| ~~**G-c**~~ (landed, #781) | D75–D77 — **unused, returned** | S131, S132, S133 — **all spent** |
| ~~**G-g**~~ (G8, landed #834) | **D77** and **D80** used | **S171**, **S172** and **S173** used |
| unassigned | D73, D74, D75, D76 — **D77 and D80 went to G-g** | **none — the S127–S136 block is spent** (`D72`, `S128`, `S135` and `S136` are G-d's and are used, not returned — #831); `D113`–`D125` / `S176`–`S181` opened for the fix pass, `D113`, `D114`, `S176` and `S177` taken |

**G-a used D71 and D78, and S127 and S134** (see *Landings*); D72 and S128 came
back. **The `unassigned` line above is a reconciliation across three landings**
— G-a's, G-b's and G-c's — assembled while resolving a merge, from each lane's
own record. It is the orchestrator's to confirm, not a lane's to assert.

**G-c's three findings are recorded without §D rows.** S133 routes to lanes
that already own the files (G-d, G-f, G-g) and says so in its own text, so it
needs no row. **S131 and S132 are unscheduled and may want one** — that is a
scheduling call, not a lane's.

---

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

**Wave 1 — open now.** These lanes share no file with each other, with Track
C's open lanes (#732, `stl/`), with Track E's (#753 `scripts/`+`ci.yml`, #763
`crates/*/tests/all.rs`, #767 `editor-core/src/eval/`, #768
`sweep/src/fillet/`), or with Track F's. **All three have landed and left this
roster** — G-c as #781, G-b as #787, G-a as #786 (see *Landings*). Wave 1 is
complete; wave 2 is the live one.


**Wave 2 — opens as wave 1 lanes free up; edge-free today.**

| lane | row | scope | review |
|---|---|---|---|
| **G-d** | **G10** (S112, re-scoped by **G-R2**) and **G5/S71** — **both in #831**, open, G5 with its fix pass. **G10's row does NOT leave §D on this PR**: its ledger says three members are open and two are tracked nowhere else (see the G-R2 amendment above) | `profile/src/sugar.rs` (S112(f)), `crates/pncad/src/lib.rs` (S112(g)), plus the eight-member ledger at S112 | style |
| **G-e** | **G6** (S104) | `editor-core/src/assembly.rs`, `pncad-py/src/py/doc.rs`, plus `editor-core/src/mate.rs` and `pncad-py/src/py/select.rs`, which the scan did not read | **ADVERSARIAL** |

**Lane-name collision, flagged not resolved.** G8's lane was dispatched as
**G-g**, which is also this roster's name for G9's lane. Names are the
orchestrator's; recorded here so a reader of two PRs does not read them as one
lane.

**Wave 3 — gated, and on what.**

| lane | row | gated on | why |
|---|---|---|---|
| **G-g** | **G9** (S95, S96) | ~~G-f~~ **LIFTED** (G8 landed as #834); Track C still gates S96 | **G-R4** — the `chord_join.rs` overlap is gone. S96's imports still reach `splitting/rules.rs`, which §D says to confirm with Track C before touching. G9 also inherits **S171** and the `chord_join.rs` half of **S133**, both already discharged by G8's PR |
| **G-h** | **G7** (S106) | Track E's **#767** | **G-R6** — file overlap at `editor-core/src/eval/mod.rs` |
| **G-j** | **G4** (S87, S88's `profile` half) | Track F's **F1** ← Track E's **#753** | **G-R7** — Evan's S87/S88 ruling, recorded in `SMELL-F-LOG`; two tracks deep and none of it Track G's to move |

---

## Reviews

*(none yet)*

## Landings

| lane | row | PR | note |
|---|---|---|---|
| **G-g** (dispatched under that name; this roster called the G8 lane **G-f**) | **G8** — S67, plus its adversarial sub-unit | **#834** | The three-name list is **computed** by a new `topo` row rather than restated; `reduce.rs`'s third copy points at the one home. **The sub-unit is NOT a defect**: `point_in_loop` is exactly invariant under `n̂ ↦ −n̂`, derived at the site and pinned over four fixtures. **Two corrections to the register**: G-R5's *paraphrase* was a line-number misattribution (the sentence is at `:93-98`), and the finding's own five-site list is off in three places. Raised **S171** / **D77**; discharged S133's `chord_join.rs` obligation (37 marker hits, 36 false positives, one stale marker fixed). **The fix pass then spent the rest of the block**: **S172** / **D80** (five copies of `starts_with("//")` as the answer to *is this line code*, the lane's own instance lifted into `fixtures::code_only` and the class recorded) and **S173** (the curved half of the one door hosted in `boolean/`, recorded not moved — no §D row, scheduling is the orchestrator's). |
| **G-a** | **G1** — S72 + S110(h), S111(c), S112(b)(c), S114(a)(d), S116(r)(t) | **#786** | Fence published per **G-R3**: `ci.yml` hunks confined to the `interval-backend` job's header comment, ~790 lines from #753's. **NOT CLEARED on first review; fix pass landed in the same PR** — the tightness ceiling had reproduced S72's own defect (a max over a sample set the degradation empties), and the structural derivation beside it was wrong in the crate's favour (`4·pad+1`, not `2·pad+1`). One member came back correcting its finding: **S114(d)**'s decoration idiom is five sites, not six. **S111(c)'s first write-up over-corrected and is withdrawn** — the diagnostic was right about the code, only the remedy was wrong; see **G-R8** as amended. New findings taken: **S127**/D71, **S134**/D78. |

### G-d — **G5**, `profile/tests/review_s2.rs`, #831

**S71's prose half only; G10 stays open on this lane's roster row.** The header
now states the boundary the deleting commit established and cites the shipped
pins; `sugar.rs` was re-read and not edited. Three things beyond the doc edit:

- **The parenthetical is issue #827.** The unreachable r > R class was called
  *"a design question"* with no number and no unit — Q6's shape. Filed with
  what would close it either way; the decision stays Evan's.
- **`S128`/`D72`.** Making `check_corner`'s own doc claim true (rather than
  narrowing it) turned a row red: the ε-keyed conditioning pin builds on a
  hairline lens's TWIN crossing, so the collapsed lever it was mined for is
  never exercised on the building bands. Not a kernel defect — the file's fuzz
  skips exactly these draws — and now asserted rather than described. D72
  schedules the re-mine.
- **`S135` + `D113`.** Every intra-doc link under `tests/` is inert, because
  `cargo doc` builds no test targets — the doc gate's own header says so.
  **At least nine** are already broken (nine is a floor: the check resolves a
  link's root, never its member path); three fixed here — one in
  `geom-core/tests/`, outside the Scope cell and named in the PR — six
  dispositioned. **`D113` holds the decision**, Evan's, after the review
  pointed out that D71 is the same shape and no row was an amnesty rather than
  a scoping call. **The census is not restated here**: it has one home, at
  S135, with the sweep's definition beside it, because this record is where
  S176(b)'s drift happened and it happened a second time in the fix that
  recorded it.
- **S133's `profile/` half discharged** and recorded at the finding: 9 hits in
  the lane's scope with no duplication, 14 in `profile/src/path/` with three
  candidates left for the lanes that own those files.
- **`S136`.** The ε-keyed row's NAME asserts the arm the default band does not
  take, and its own doc header says so instead of fixing it. Not renamed —
  `sugar.rs`'s `LEVER_ULPS` doc cites it by name as its *"what goes red"* — so it
  is recorded, with two more members found in `step-import/tests/`.
- **`S176`.** Raised BY the style review, out of this lane's own register
  entries: a landing PR's record drifts from the change it documents, two ways
  — stale `file:line` written inside the diff that renumbers the file, and one
  census restated in log + scan + §D row + PR body that a re-sweep updates in
  one place. Both recorded as classes; neither swept. **(b) recurred inside its
  own fix** — the corrected pair did not re-derive either — so the census now
  has one home and a written-down definition rather than a better number.

**Style review: NOT CLEARED, and the fix pass is in the same PR.** The review's
verdict was that the PR **re-minted S71 at the site it was correcting** — the
header still opened on the flagged sentence with a rider, `check_corner`'s doc
contradicted itself inside one paragraph, and the new prose promised a
two-answer pin against code that panics on one. All eight must-fixes and every
judgement call taken; nothing disputed. The three that mattered: `n_enclosing`
is **asserted** rather than printed (0 at 24 000 corners on all three bands
locally); `check_corner`'s enclosing arm was **inverted onto the boundary**,
having failed on the legitimate build and passed on the boundary-moving one;
and the two enclosing pins — one `Ok` arm written twice, neither site
naming the other — got one home, with the non-swallowing arithmetic
under it getting another that `check_corner` shares.

**G10 is in the same PR**, as its second unit and its own commits: one lane,
one branch, and both halves edit `SMELL-SCAN-2026-08.md`, so splitting them
would have this lane resolving conflicts against itself in that file for no
reviewer's benefit. Split on request — nothing in the G10 commits touches
`review_s2.rs`.

- **S112(f)** — `sugar.rs`'s `arc_fillet_trims` header asserted a *current*
  two-consumer property with no twin and no raw builder in the tree; it now
  names the one consumer it has (`path::arc_fillet`, checked workspace-wide)
  and puts the extraction in the past tense. **G5's re-read fence lifted for
  this member only**, per G-R2; the enclosing-candidate machinery is unread and
  unedited.
- **S112(g)** — `pncad`'s *"What the façade itself contains"* now names
  `tolerance`, and says the thing that made the omission worth fixing:
  `report()` and `eps_source()` **commit the ambient ε as a side effect of
  being asked**, which is the one place in a façade claiming *"no numeric
  behavior"* where calling a wrapper changes the run. `doc-gate.sh` exit 0,
  `cargo test -p pncad --doc` 34 passing.
- **The ledger, which is the row's actual deliverable** — all eight members at
  S112, each with the lane, the PR and the merge commit, **re-derived from the
  tree**. Five closed: (b)(c) #786, (h) #787, (f)(g) #831. Three open.
- **Two corrections the walk returned, both against the ruling that assigned
  it** — see the amendment note under **G-R2** below.
- **`S177` + `D114`.** Track E's **E-g** row is struck while it is the only
  live tracker for S111(a)(b)(d), S112(a) and S75, all of which its own row
  records as *not in #768*. Three re-derived from the tree and standing. The
  class is *a lane's row is struck while it is the only index for the findings
  routed to it after dispatch*; **D114** re-homes the five and asks whether any
  other struck row did the same. **Not taken by this lane**: `sweep/src/fillet/`
  is outside Track G's constitution, and re-dispatching another track's
  findings is not a lane's act.

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
  fixed, with D79 scheduling it. **S135 and S136 stayed free** at that landing;
  both were taken by G-d in #831.

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
