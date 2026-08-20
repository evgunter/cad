# SMELL-SCAN Track C — orchestrator log

The live state of Track C: the lane roster, the rulings made inside it,
and what has landed. §D of `docs/SMELL-SCAN-2026-08.md` remains the
schedule; this file is the *execution* record, and every finished unit
still gets its `FIXED by #NNN` lead at its own finding.

**This programme runs entirely outside the model A/B experiment.** No
Fable/Opus pairing, no ordinal, no row in `docs/MODEL-AB-LOG.md` —
nothing here touches that log. (Evan, 2026-08-20: the Fable limit is
hit, so Track C is deliberately to the side of it.)

**Branch prefix:** `smellc/` for units; the orchestrator sits on
`claude/track-c-orchestration-7b06uq`.

---

## Review policy for this track

Not the full orchestrator protocol. Per Evan, 2026-08-20:

- **Style review on every unit** — `docs/prompts/reviewer-style-lane.md`,
  dispatched by path, with the per-lane emphasis the dispatch owes
  (`docs/REVIEW-STYLE-DISPATCH.md`). On top of the standing brief, a
  Track C style review answers two questions the brief does not:
  1. Was the original stylistic problem — *as the finding states it* —
     **completely** fixed, or fixed at the reported instance only?
  2. Was it fixed **in the best way**, or merely in a way that closes
     the finding's sentence?
- **Adversarial review only where the change carries meaningful risk.**
  That is a minority of the track and is marked per row below. A unit
  that rewrites prose, adds acceptance rows, or moves code without
  changing a decision does not get one.

## Recording convention

**The landing PR carries its own record.** Evan, 2026-08-20: it is ideal
for the table update to land *with* the PR, so the concurrent
orchestrators are not reading a document that is behind the tree.

Each unit therefore makes two edits to `docs/SMELL-SCAN-2026-08.md` in
its own PR: the finding's heading becomes
`## SNN. FIXED by #NNN — …` and its **original problem statement is
replaced** by the record of what was done (version control keeps the
original — it is not preserved inline); and the unit's **row leaves §D's
Track C table**, per §D's own *live rows only* rule.

**Conflicts there are expected and survivable.** Resolve by merging
`origin/main` — never rebase, never force-push — and keep both sides;
the edits are to different findings and different rows. **If the only
conflict was that document and CI was already green on the pre-merge
head, merge without waiting for a second CI run.**

---

## Rulings made in this track

| # | Question | Ruling | By |
|---|---|---|---|
| **C-R1** | **H16 — the STL header is not caller-settable.** New public API, which #639 deliberately left as a residue for Evan rather than closing. | **Take it**, as a design PR that **waits for sign-off**. `StlOptions` mirrors `StepOptions` (solid name + 80-byte header; defaults leaking nothing). Preserve `ascii.rs`'s *"constant in this build"* wording — phrased so a caller-settable header does not falsify it. The only pins are `export.rs`'s `NAME:` row and `review_m2_pr7.rs:172`'s `HEADER:` row; both move with the change. | Evan, 2026-08-20 |
| **C-R2** | **S29 — nothing states what the mesh sizing *policy* is.** Writing one down is a design act, not a cleanup. | **Split the lane.** The mechanical vocabulary unification self-merges; the **policy statement** goes out as its own design-conversation PR and waits for sign-off. The question has been routed around twice already. | Evan, 2026-08-20 |
| **C-R3** | **S31 — the `geom-curves`/`geom-surfaces` boundary.** The duplicated constants and helpers need a home, and where that home is *is* the crate-boundary call. | **Merge the two crates.** Named `geom`, above `geom-core` and `bvh` (the name is the orchestrator's call). Authorises editing `DESIGN.md`'s crate-table row and its pointer at the crate docs as authoritative text — and nothing else ratified. | Evan, 2026-08-20 (name: orchestrator) |
| **C-R4** | **S29 is not blocked on a design conversation.** §D routes it to `docs/TESS-SPLIT-SPEC.md` / PR #568. | **Correction, and it stands.** Checked by #684's reviewer: both #568 and TESS-SPLIT-SPEC are scoped entirely to the NURBS per-cell schedule in `nurbs_cert`. **No open conversation covers `curved::grid_steps`**, so S29's analytic-chart half was never waiting on a venue — it does not have one. §D's C3 row is wrong on this point and is corrected when C-j lands. **DISCHARGED, and not by C-j**: the row already carried the correction, dated 2026-08-19, before C-j opened. #803 removed S29 from C3 altogether and preserved the correction inside §S29's own record, which is where it now lives — the policy half still has no venue. | the parallel orchestrator, 2026-08-20 |
| **C-R11** | **The roster's own scope citations are stale, and a lane checks them rather than complying.** `C-h`'s `splitting/rules.rs:268` lands on `face_extent`, a lever-arm helper, not on anything the `bridged` skip reads. | **Every scope cell is a claim site.** A lane that finds a citation unresolvable says so in its report and states what the line actually contains; it does not silently work around it or silently obey it. This is the same failure as the brief that fenced a lane against a memory deleted from main two days earlier — recorded there as an incident, promoted here to a standing rule, because the roster is now the third document on this track to carry a citation that outlived what it described. | orchestrator, 2026-08-20 |
| **C-R12** | **How many lanes get an adversarial review.** The inherited roster marked 7 of the 12 remaining lanes adversarial, which is more than *"a minority of the track"* claims. | **Retrimmed to five: C-e, C-g, C-h, C-k, C-m** — plus **C-q**, which is new. The criterion Evan gave is *"complex enough that there's a significant chance the change will introduce a regression not caught by CI"*, and it is a narrower test than "this code is load-bearing". **C-j drops to style-only** because its mechanical half is a re-spelling whose correctness is provable by byte-identity of mesh output — a wrong answer is visible without an adversary, and the lane owes that proof. **C-l drops provisionally**, because a lane-trait collapse is type-level and the compiler is the adversary; the sub-lane that *rewrites* `Dual` arithmetic rather than re-spelling it is promoted back. | Evan (criterion) + orchestrator (application), 2026-08-20 |
| **C-R13** | **H16 — what the STL `solid` name should default to.** The lane defaulted `solid_name` to the producer (`cad-kernel`, the Q9 placeholder) and flagged that `StepOptions::product_name` defaults to `part` — a *part* — so the mirror C-R1 asked for is not exact. | **Make it semantically correct; symmetry with STEP is not the criterion.** Evan, 2026-08-20: *"i don't care about the symmetry with STEP, it should just match what that field is supposed to be."* STL's `solid <name>` names **the solid being described**, so a producer string is the wrong kind of thing there; the binary 80-byte header is free text conventionally carrying a producer, so it is right as it stands and does not move. **This moves the default output bytes**, which is permitted and is not a reason to hesitate (`memories/output-stability-as-justification.md`) — but the PR's *"26/26 byte-identical"* evidence must be restated as what it then proves, not kept as written. Bonus the change buys: today's exported bytes carry the **Q9 placeholder project name**, so a part-shaped default decouples STL output from the naming decision entirely. | Evan, 2026-08-20 |
| **C-R14** | **H16 — `StlOptions` derives `Eq` and `StepOptions` cannot** (its `Option<f64>`). The lane offered keep-as-a-strengthening or drop-to-match. Evan, 2026-08-20: *"this feels off, is there a cleaner third option?"* | **Both offered options answer the wrong question, and that is why it read as off.** `Eq` is not a feature — it is a marker asserting the existing `PartialEq` is reflexive. Underneath sit two real questions. **(a) Does anything need it?** No: every `StlOptions` consumer in the workspace constructs one or passes `&StlOptions::default()`; none uses it as a set element, map key, or anywhere total equality is required. So drop it — on the ground that **nothing needs it**, never on the ground that `StepOptions` lacks it. **(b) Why can `StepOptions` not have it?** `uncertainty_m: Option<f64>` — a bare `f64` tolerance in a public options struct, in a codebase whose ratified thesis is that bare `f64` tolerances get types. **The `Eq` asymmetry is not a fact about STL; it is a symptom of S25's shape in the other struct**, one crate over from where #692 fixed an instance of it with `MarchTol(f64)`. Typing it would move the *"finite and > 0"* check from write time to construction time and make `Eq` honest on both (a NaN-rejecting newtype has reflexive equality, so `impl Eq {}` is sound where `derive` cannot reach). That half is a `step-export` public API change → **C-R7: an issue and a §D row, not a widening of #732.** Half (a) is taken now. **Half (b), CORRECTED by Evan, 2026-08-20** — *"it sounds similar to what epsilon already is"*, and he is right, which makes the finding **smaller and better**: `uncertainty_m` **is** an ε — same quantity, same units, same validity rule that `geom_core::Tolerance::init` already enforces (*finite and strictly positive*). So this is not a missing newtype, it is **S4's dominant shape — one vocabulary, N hand-synced copies** — with `StepOptions` restating ε's rule by hand as the Nth. `step-import/src/entities.rs` carries the same bare `f64` twice, so import and export each restate it independently. One real distinction survives: `Tolerance` is `{eps, k}`, the **run configuration**, whereas this wants **ε alone**, which has no type while ε-plus-K does. Filed rather than done, per Evan's standing scope rule for this track. | orchestrator, 2026-08-20 (from Evan's question) |
| **C-R15** | **H16's two remaining flags.** Empty strings accepted for both fields (`solid_name: ""` writes `solid ` with a trailing space; `header: ""` writes 80 zero bytes), and the stale pre-#639 artifacts under `crates/stl/target/m3pr6-stl/`. | **Both stay as they are** (Evan, 2026-08-20: *"no opinion"* / *"sounds good"*), with one addition: the empty-string behaviour is **documented at the field**, not only in the PR body. A reader of the type currently cannot learn it, and a PR body is not where a type's contract lives. | Evan, 2026-08-20 |
| **C-R16** | **#731's E0004 mutation table was taken on an intermediate working state and never re-run.** Two of four rows are wrong, and both omit `eval/anchor.rs` — the PR's own headline fix. The tell is a cited line (`resolve/mod.rs:977`) matching **neither** tree (main 975, head 1007), under the claim *"identical to #632's post-state"* — true only because it literally **is** #632's post-state. | **Re-derive every row on the shipped head and name the tree each number came from.** Not re-read — re-run. A table taken during development is not evidence about what merges. | orchestrator, 2026-08-20 |
| **C-R17** | **Three more in-crate members of H11's class**, declined or unfound: `eval/mod.rs:1372` (`content_key`'s payload wildcard under an **exhaustive** tag match — the identical split the PR closed elsewhere), `node.rs:955`/`:1004`, and `expr.rs:686/696` (a wildcard nested in a tuple, inside the PR's own declared blind spot). | **Fix all three. C-R6 is explicit that in-crate residues are fixed, not reported.** `eval/mod.rs:1372` carries the most: **S4's own record documents this bug having already happened** — *"`Step::AtToward`'s memo content-key tag 28 COLLIDED with `ArcContinue`'s existing 28 — a hit would serve wrong geometry"*, caught by a reviewer rather than a type. Routing `node.rs` to "S4's payload-lists row, marked FIXED by #618" is C-R7's named failure verbatim: a live problem parked inside a record labelled FIXED. | orchestrator, 2026-08-20 |
| **C-R18** | **#731's prose-hygiene pass manufactured the defect it was fixing.** Its correction to **§C15** replaced a false clause with *"the conclusion survived in two of three"* — a reading C15's own bullets do not support (under the reading that makes #632 fail, all three failed). It also corrected the **meta**-record while leaving the **object-level** sentences that carry #632's wrong population (`SMELL-SCAN:627`, `:728`). | **State a reading the bullets actually support and say which it is, or leave C15 alone and record the #632 fact elsewhere — not the split difference.** And sweep the object-level sentences: correcting the record *about* a claim while leaving the claim is the same half-fix one level up. **This is §C16 committed inside a correction to §C15**, which is the sharpest instance of that section the scan has produced. | orchestrator, 2026-08-20 |
| **C-R19** | **What this track may take, restated.** The working rule had drifted to *style fixes only; anything cross-crate or public-API gets filed and left*, which is what sent H16's ε item out as a parked issue. | **Cross-crate and public-API changes are within scope** (Evan, 2026-08-20): *"cross crate and public api is potentially within scope for these style fixes, though it may have a design element that means the plan should go by me before implementation."* So the discriminator is **not** where the change lands — it is whether a **design element** is present. Three tiers: a style or encoding fix is taken by the lane; a change with a design element is written up as a **plan that goes to Evan before implementation**; **implementing a new feature or fixing a logic bug is never this track's** and gets a GitHub issue. The middle tier is new and is where the ε item, the two Part-21 user-assigned STEP header fields, and anything like them now sit — **takeable Track C rows asking for a plan, not parked issues.** A filed row must say which tier it is in, or the next reader re-derives the judgement. | Evan, 2026-08-20 |
| **C-R20** | **Two lanes minted §D row `C11` independently**, an hour apart, in two unmerged branches (#732's `pncad-py` option surface and #731's `editor-core` residues). Neither could see the other's number. | **Row numbers are assigned by the orchestrator, never taken from the next visible gap.** Assigned: **C11** C-o/#732 (issue #730), **C12** C-f/#731, **C13** and **C14** C-o's ε and Part-21 rows, **C15** reserved for C-p's per-face slack gate hole. A lane needing a row asks for the number. **This rule already existed one track over** — Track D hit the identical collision when three lanes minted in parallel and two landed on D10/D11 — and its handoff states it explicitly. I read that handoff at session start and did not apply it, which is the whole failure: *a rule recorded in another track's operational file is not carried by reading it once.* The cost was small only because both PRs were still unmerged. | orchestrator, 2026-08-20 |
| **C-R21** | **C-R20 is not sufficient, and its third instance proved it.** I assigned §D row numbers centrally *within Track C*. Track A's #714 fix pass then landed its own **C11** on `main` — roughly ten minutes after I assigned C11 to #732, and cited from three other places in the document. Neither track could see the other's unmerged branch. | **A row number is not owned until it is on `main`, and a lane mints against the merged tree, never against an assignment.** Central assignment prevents *intra*-track collisions and cannot prevent cross-track ones, because **§D is shared by every track**. So: the orchestrator still assigns, but the assignment is a *reservation against a tree that may move*, and a lane re-verifies against `origin/main` immediately before it writes the number — exactly what C-o did, which is why this cost a relabel instead of a silent duplicate. **Holes in the sequence are named in the gating paragraph** so the next reader does not tidy them away. Settled state: C11 Track A; C12 #731; C13/C14/C16 #732; C15/C17 #738. | orchestrator, 2026-08-20 |
| **C-R22** | **C10 asks for work the ratified design forbids, and nothing in the row knows it.** The row instructs its taker to lift the instrument out of `geom-core`, S30-style. C-q's compile oracle — a scratch crate against `geom-core --features probe` — walked the three escapes and hit an escalating wall: `impl Decide for P` → E0277 (`P: SpanLocate` unsatisfied); `impl SpanLocate for P` → E0277 (`P: locate::sealed::Sealed` unsatisfied); `impl …sealed::Sealed for P` → **E0603, module `sealed` is private**. The seal is what closes **Q1's ratified instantiation set**. | **Refuse the split, and record the refusal as the unit's answer** — C10's own text asks the taker to *"decide whether the instrument can leave a door that certifies, and record that decision"*, so a reasoned no **is** the deliverable, not a declined lane. Take instead: consolidate the three doors' identical bodies into one private `classify<T: Decide>` (in-crate, no public API, no behaviour change); **D32 option (b)** — make `ledger_row` load-bearing by extending `flagged_census.rs`; and **retire `profile::k_stats`** (S40's shim, whose own docs say to stop using it — tier 1.5, executing a decision recorded at M2 PR 7, not making one). Also correct **two wrong premises in C10 itself**: the instrument is **263 lines, not ~96** (the row's figure omits `Probe`'s own trait impls, and a type's impls travel with the type), and *"S30's class one crate over"* is wrong **at the level of the diagnosis** — the volume question S30 found unasked **was** asked here, in `geom-core/Cargo.toml`'s `probe` feature comment, measurement and placement argument attached. | orchestrator, 2026-08-20 (from C-q's oracle) |
| **C-R23** | **S32's causal sentence is false, and two more of its clauses with it.** The finding says *"the natural query being unavailable at the enum is what created the second enum"* — i.e. give `Surface` a jet door and `geom-brep`'s `Chart` dissolves. C-g's archaeology found `Chart` born at `ffc0b0fa` carrying today's doc verbatim, and `ssi/system.rs:150-235` on main confirms it. | **`Chart` stays; S32 lands as `FIXED IN PART`** (#714/S58's precedent for this exact shape). The API half is real and is fixed — one `jet` door, the five partials and `normal` become projections of it, `normal` and `metric_floor` each drop from two passes to one. The causal half is **corrected, not closed**, on three independent grounds any one of which is sufficient: `Chart` is **third order** (`jet3`, `k+l ≤ 3`) where `SurfaceJet` is second; it **carries a domain** (`u_range`/`v_range`, `domain()`) because a plane is unbounded and `Surface::Plane` has nowhere to put a window; and a `Surface`-wide constructor is **deliberately refused by a named rule**, C12.1's per-arm retirement rule — so the narrowness is a ratified property, not a workaround. **A third clause is also false:** S32 reads the *"hand-filled `SurfaceJet3` of zeros"* as placeholder filler, but a plane's second and third partials **are** exactly zero — the finding mistook a right answer for a smell. The fourth clause, *"re-implementing plane evaluation"*, is the weak one and is to be stated at the notch it can hold: a hoisted cross product plus a window, not an independent implementation. | orchestrator, 2026-08-20 (from C-g's archaeology, verified against main) |
| **C-R24** | **Where C-R6 stops and C-R19 tier two starts.** C-g found the identical class on the curve side — `NurbsCurve::deriv_in_span`/`deriv2_in_span` discard 2 of 3, and `topo/src/splitting/neighborhood.rs:180,184` runs two full order-2 passes at the same `t` **under a comment calling the result *"the base-endpoint jet"***. It is the **same crate**, which normally puts it under C-R6 (in-crate residues are fixed, not reported). | **File it (row C24), and the row states the discriminator.** The surface side has a **ratified type to project onto**: `SurfaceJet` already exists and is already publicly exported — which is half of S32's own complaint. The curve side has **no `CurveJet` at all**, so the work is not *apply the same fix one type over*, it is **mint a new public type** — a design element, C-R19 tier two, a plan rather than a lane's discretion. That is the line: **using a decision already made is tier one; making one is tier two, in-crate or not.** C-R6 is about *ownership* (do not hand an in-crate residue to someone else), not about *authority* (a lane may not ratify a new public type because it happens to be nearby). | orchestrator, 2026-08-20 |

---

## The standing lane header — committed, because the last copy was not

**2026-08-20, second container loss.** This text lived in
`~/.local/share/cad-work/trackc-lane-header.md` and every lane was dispatched
against it by path. The container was reclaimed and it went with the clones,
the five review-findings files, and the container-local `new-lane.sh`. Nothing
*pushed* was lost — all five lanes' fix-pass work survived, because the
commit-and-push-at-every-seam rule held. The brief they were working from did
not, because it was never committed.

*That is this session's second instance of one lesson.* The first was
**#745**: eight rulings written, committed and pushed to the orchestrator
branch, cited by number in briefs, and unreachable because they had not
**merged**. This one is a step worse — never committed at all. **A register
that has not landed is not a register, and a brief that lives only in a
container's home directory is not a brief.** It is now here, where it is read
from the tree.

---

Binding on every Track C implementer lane, alongside the unit's own brief.

**Read first, in this order:** `docs/prompts/implementer-discipline.md` in
full; this file's *Review policy*, *Recording convention* and *Rulings*
sections; then your finding's own text in `docs/SMELL-SCAN-2026-08.md`.

**This track is outside the model A/B experiment.** No pairing, no ordinal, no
row in `docs/MODEL-AB-LOG.md`. **Never edit that file.**

**Where your files go.** Your clone is `~/.local/share/cad-work/<lane>/cad`;
`export CARGO_TARGET_DIR=~/.local/share/cad-work/<lane>/target`, never shared
with another lane. Heavy cargo goes through `local-scripts/with-build-slot.sh`
(machine-wide mutex, width 1). **PR bodies and any other to-be-published text
go to `~/.local/share/cad-work/<lane>-pr.md`** — never the session scratchpad,
which is shared between concurrent agents.

**Commit and push at every seam.** Three container losses have now killed lanes
on this track. Every lane that had pushed lost nothing; the one that had not
came within a disk reap of losing a day. Your brief names your seams; if it
does not, invent them and say what they were.

**Recording your own completion.** Your PR makes two edits to
`docs/SMELL-SCAN-2026-08.md`: the finding's heading becomes
`## SNN. FIXED by #NNN — …` with its **original problem statement replaced**
by the record of what was done (version control keeps the original), and your
row **leaves** §D's table. Check §D's surrounding prose too — the gating
paragraph names findings by name, so a landing that leaves the table but stays
in the paragraph makes the paragraph false. Delete your roster row here.
**Row numbers are assigned by the orchestrator** (C-R20) — ask, never take the
next visible gap. Conflicts in these files are expected: resolve by merging
`origin/main`, **never rebase, never force-push**.

**What this track's reviews actually catch.** Every unit so far was *not
cleared* on first review, and in every case the finding was **a claim wider
than its evidence**, not a shipped wrong answer: a mutation table that reported
widenings leaving the suite green; *"bitwise the old code"* on a diff that
changed behaviour at non-finite intermediates; *"preserved exactly, and
strengthened"* on a falsifier whose population had silently narrowed;
*"the last is a real hole"* where three existed. **Write claims you can survive
having re-derived rather than re-read** — this track re-derives any claim once
found overstated. State the qualifier that makes a claim exactly true
("finite *arithmetic*", not "finite *inputs*"), and scope your evidence out
loud: a green `-p onecrate` run is evidence about one crate.

**A measurement is a measurement of a tree.** Name which tree each number came
from. A cited line number matching neither base nor head is a receipt for the
tree the measurement actually ran on.

**Two obligations beyond the standing discipline.** *A claim about a shared
helper is a claim about every caller* — collapsing N copies into one converts N
local facts into one shared contract, and the guard obligation moves with it.
And *a brief is a claim site too*: if a line number, path or citation in your
brief does not resolve, **check rather than comply**, and say so (C-R11 — three
of five briefs in one session carried one that did not).

**Your final report**, ≤150 lines, states what you swept with and **what that
pattern could not match**; every claim resting on a measurement and what guards
it; and anything you are holding back — you will be asked before the merge, so
answering saves a round.

## Lane roster

Gates are the *live* ones as of 2026-08-20; §D's own edge list is
superseded for Track C by this table.

| lane | finding | scope | gate | review |
|---|---|---|---|---|
| **C-e** | **H13** — `sweep_body`'s helix rows have no orientation coverage | `sweep/tests/{m8_14_long_turn_sweep,m7_skin_integral}.rs`, `step-export/tests/common/mod.rs` — **two of these three citations do not contain a member of the finding's own class, and the mechanism is worth keeping (C-R11: the scope cell is a claim site, so the correction lands here and not only in a PR body).** `m7_skin_integral.rs` and `step-export/tests/common/mod.rs` contain **no helix at all**; both build the SQUARE ELBOW, byte-identical to the body #636's own orientation row already pins (`R = 3`, `h = 0.25`, `bulge = tan(π/8)`, 9 stations, `v_degree` 3), and `step-export`'s own docstring says *"constant for constant"*. The cell was minted from **#636's list of uncovered `sweep_body` callers** — a wider class than *"the helix rows"* — and the two were conflated. The direction of the error is **over-scope**, not stale location: the cell described coverage that already existed. #636's own citation `step-export/tests/common/mod.rs:482` is line 4 of a six-line comment, not a call. **The one real gap the cell obscured** is `m7_skin_integral.rs:381`, the RATIONAL circle-section elbow, which had no orientation coverage and is closed by #779 alongside the helices. | **discharged** (#779) | **adversarial** + style — #636's level-plane oracle trips its own precondition here (`cos ≈ 0.011`), so this needs a *new* oracle, and the oracle carries the soundness |
| **C-f** | **H11** — #632's residues: two as recorded, **ten** as they existed | `editor-core/src/{resolve/,names/select.rs,refactor.rs,eval/{mod,anchor}.rs,node.rs,expr.rs,persist/check.rs,doc.rs}` — this cell read `editor-core/src/select.rs` until #731, and no such file exists (**C-R11**: the scope cell is a claim site, so the correction lands here rather than only in a PR body) | **discharged** (#731 merged) | style — returned NOT CLEARED twice (C-R16/C-R17/C-R18, then the verification pass's probe-D re-derivation and a tenth class member at `doc.rs`) |
| **C-g** | **S32** — `Surface`'s one-partial-per-call API and the shadow SSI enum | `geom/` (the merged crate), `geom-brep/src/ssi/system.rs` | **discharged** (#705, #692 merged) | **adversarial** + style |
| **C-j** — **discharged** | **S29** — the sizing vocabulary across five modules | `mesh/src/{nurbs_cert,curved,chords,trimmed}.rs` + the new `mesh/src/sizing.rs` and `tools/tess-meter/src/lib.rs`; the cell read `budget.rs` until #803, and that file lost its sizing content to `tools/tess-meter` when #709 landed (**C-R11**: a scope cell is a claim site, so the correction lands here) | **discharged** (#684 merged) | **style only** on the mechanical half — retrimmed, see C-R12. **Mechanical half merged as #803**; the policy half is a design PR (**C-R2**) and is **NOT** discharged by it |
| **C-k** | **S28's duplication half** — three tessellation lanes, three pipelines | `mesh/` | **discharged** (#684 merged) | **adversarial** + style |
| **C-l** | **C7 + S33** — the lane-trait collapse, `RingInterval`, the scalar ladders | `geom-core/`, and W2b's 535 refs across 15 files | **discharged** (#682 merged) | **style only, provisionally** — see C-R12; expect to split into 2–3 lanes, and the sub-lane that rewrites `Dual` arithmetic rather than re-spelling it gets promoted to adversarial |
| **C-m** | **S27** — `props/quad.rs`'s four quadrature engines | `geom-brep/src/props/quad.rs` | **STILL GATED, and the gate changed identity** — #714 **merged** (`08:42Z`), so A2 is discharged; C3's row now names **#723** as the second gate, and it is **open**. See the note below. | **adversarial** + style |
| **C-n** | **H17** — the rustdoc spec-code remainder, ~1115 lines / 130 files | per crate: `topo` 300, `editor-core` 267, `geom-brep` 192, `geom-core` 107, `sweep` 64, rest < 70 | **deliberately last** — it touches 130 files and would conflict with every open lane | style, per crate batch |
| **C-q** | **C10** — `geom_core::k_stats`, S30's class one crate over | `geom-core/src/k_stats.rs`, and `profile::k_stats`'s shim (S40) | none | **adversarial** + style — the recording sits *inside* three load-bearing kernel predicate doors, so #709's split does not transfer mechanically |

**C-m's gate is now a live kernel defect, not a queued lane.** #723 is a
sphere meridian arc crossing a pole: `min_max` folds only edge *endpoint*
levels, so the latitude extreme in the arc's interior is never seen, and
`area = r·Δu·(hi − lo)` integrates against a false `(lo, hi)`. Executed at rest
through `import_step`: **tier 3 green, `pad = 0.0`, volume −47.19%** — more than
twice #649's 19%, on the same failure *shape* and a different premise. Adding
one vertex to a solid that is otherwise **refused** turns the refusal into a
wrong certified number.

Three things follow for this track.

1. **It is not ours.** Under **C-R19** that is tier three — a logic bug, not a
   style or encoding fix and not a design element. It is already filed. Track C
   does not take it and does not widen a lane to reach it.
2. **So C-m does not unblock by waiting on us**, and should not be read as
   queued behind C-q/C-j/C-g. Its gate is somebody else's fix landing.
3. **The gate's identity changed under the roster**, which is the same failure
   C-R11 names one level up: my roster cell said *"STILL GATED — A2 / #649, open
   as #714"* hours after #714 merged. A gate cell is a claim site too, and a
   *discharged* gate reads identical to a live one until someone checks. What
   saved it here is that the row it points at, C3, records **two** gates and the
   second is still real — so the cell was stale rather than wrong, by luck.

*Worth Evan's attention on its own terms*, separately from Track C: #723 has
sat open since `05:48Z` with an executed at-rest reproduction and two named
candidate fixes, and it is the second wrong-certified-volume-at-`pad = 0.0`
found in one day.

**Not taken by Track C:** C6's rows (blocked on other programmes — OnArc
+ RESPELL-TABLE, the workspace's first proc-macro crate, a persisted
format), and **S26**, which #472 deferred *in writing* as needing its own
proposal with re-measured floors — a proposal, not a patch.

**H17's measurement is not to be re-run.** #639 walked each crate's
`pub mod` tree from `lib.rs` and measured ~1189 against an estimate of
~124; its reviewer independently parsed 1188. Start where the density
is. The pattern must cover **bare** clause letters (`F5`, `G1`, `U7`,
`R3`, `C4`, `S13`) as well as prefixed codes, and must follow
`\`-continued multi-line literals.

### The distinction C-c derived while re-anchoring, which generalises

Re-anchoring a renamed crate across a 6,800-line document forced a rule that
did not exist, and it is worth keeping:

> **A pointer that tells someone where to go is re-anchored; a record of what
> was observed at a place and time keeps the name it was observed under.**

Twelve live sites moved under it; twenty-two survivors are accounted for by it
— closed findings, records of what was merged, and one case worth naming:
**S41's `Trv`-crossing table and its `cargo test -p geom-curves --features
interval` line are measurements, not directions.** Renaming a measurement's
command would make a record of what was *run* look re-runnable when it is not.
That line keeps its command and gains a bracket saying the crate is now `geom`.

This is the discriminator the project's invalidation discipline has been
missing. That discipline is symbol-scoped, and the repeated failure — S39, Q4's
second sub-case, `nurbs_iso.rs` in this very PR — is prose that outlived what
it described. But the fix is not "rename every mention": half of them are
history, and renaming history is its own corruption. The question is what the
sentence is *for*.

---

## Coordination with Track D

**Track D was constituted 2026-08-20** while this track's lanes were running,
and it overlaps Track C at three points that must not be rediscovered:

- **D7 was gated on #702** (this track's C-a) — its `PairSolve` deletion waited
  on `mate.rs`, `mate/solve.rs` and the `lib.rs` re-export block C-a was
  editing. Both have since merged: #702 on 2026-08-20, and the deletion as
  **#735**.
- **Track D owns `sweep/` for the duration** (D1/D2 in `extrude`, `revolve/`,
  `fillet/`; D8 in `skin.rs`). **C-e / H13** edits `sweep/tests/` only, which is
  disjoint from all four, but H13's oracle is *about* skinning orientation and
  D8 is inside `skin.rs` — sequence C-e after D8, or confirm with Track D first.
- **D9 and C-h both wait on #690**, in `topo/`. Their file sets are disjoint
  (D9: `chart_region.rs`, `splitting/containment.rs`; C-h: `census.rs`,
  `splitting/rules.rs`) — but both land in the same crate within hours of each
  other, so whichever is second re-merges rather than assumes.

**Track D handed a row to Track C**: S18's `step-export/volume.rs` row. Its
immediate cause is that `topo::props` exposes only body-scoped
`mass_properties` while the exporter needs *per-shell* volume, so closing it
means a new door in `props/` — which is A2's file set. It joins C3's `props/`
work and inherits **C-m**'s gate on A2 / #649.

---

## Incidents

**2026-08-20, second preemption — nothing lost.** The container went down
again roughly two hours later, killing two implementer lanes and both of
#705's reviewers. **Every lane clone had a clean tree and every branch was
pushed**: the loss was zero, against a whole uncommitted unit the first time.
The difference is entirely that the lanes had been told, after the first
incident, where their seams were.

Two further adjustments came out of it, both about *what survives a kill*:

- **Reviewers now report incrementally**, appending each settled finding to a
  lane-private file as they go, rather than holding a verdict in context until
  the end. A killed reviewer's reasoning is not recoverable; a file is. Both
  #705 reviewers were resumed with a priority order attached, so a third
  preemption costs the least valuable findings rather than an arbitrary
  suffix.
- **Resumption beat restart again.** Four agents were resumed from transcript
  across the two incidents and none needed re-briefing on its own work.

**2026-08-20 — container restart, three lanes lost.** The session's container
went down with C-a, C-b and C-c all live. In-process subagents do not survive a
restart; the lane clones under `~/.local/share/cad-work/` and the subagent
transcripts under `~/.claude/projects/` both did.

What that cost, per lane, is the useful part:

- **C-a** had finished and pushed, and #702 was already open and green. Nothing
  lost but its final report — recoverable from the PR body, which was complete.
- **C-c** had five commits **pushed** and had not yet opened its PR. Nothing
  lost.
- **C-b** had **21 modified files and a new `tools/tess-meter/` crate in the
  working tree with nothing committed** — the whole unit, one `rm -rf` from
  gone. The orchestrator committed it verbatim as a WIP commit and pushed it
  before doing anything else.

*Lesson, and it is the implementer discipline's existing rule earning its
keep:* **commit and push after every coherent unit**, not at the end. The two
lanes that had done so lost nothing; the one that had not came within a disk
reap of losing a day. C-b's brief did point at
`docs/prompts/implementer-discipline.md`, which says exactly this — so the
gap is not the rule, it is that a lane deep in a large refactor has no natural
seam and will not invent one. Long units should be told where their seams are.

Both surviving lanes were resumed from transcript rather than restarted fresh:
their accumulated design state (C-b's placement decision, C-c's merge-conflict
resolutions across 11 dependents) is exactly the kind the death-recovery rule
says is worth a replay.
| **C-R8** | **`docs/DESIGN.md:1132`'s stale crate name**, inside the **ratified D2 addendum's** rationale. C-c left it deliberately: the merge ruling authorised the crate-table row and the line-369 pointer *and nothing else*, and it would not edit ratified text without cover. | **Authorise the rename in place.** It is the same mechanical consequence as the crate-table row — `geom-curves` becomes the merged crate's name, the D2 argument itself is untouched. The sentence's claim is unaffected; only the name it uses is stale. Folded into #705's fix pass rather than pushed under the running reviewers. | Evan, 2026-08-20 |

---

## Reviews

### #702 (C-a / S24) — style lane, 2026-08-20: **not cleared**

The first test of this track's *style-review-only* policy on a low-risk row,
and it argues for the policy rather than against it: **a style lane with no
adversarial partner found a MAJOR anyway**, because it treated the PR's
mutation table as a claim to run rather than as evidence to read.

**The MAJOR.** The PR reported *"widen `is_declared_frontier` and this row goes
red"*. Three genuine widenings leave the whole 567-row suite green —
`all(…)` → `any(…)`, dropping the `mate.is_some()` conjunct, and relaxing
`CensusUnsupported { entity: Face(_) }` to `{ .. }`. Only constant-true on any
non-empty `AtRest` reddens `row4_a`, whose fixture contains no
`CensusUnsupported` at all, and **no row in the suite mixes a declined finding
with a refuted one** — the exact case the `all` exists to exclude. Q3's shape
#2, and the overstatement was on its way into this document's permanent record.

**Rulings on the fix pass** (orchestrator, 2026-08-20):

| # | Ruling |
|---|---|
| **C-R5** | **Take the variant encoding.** The reviewer produced the fact that decides it: `editor_core::assemble` has **no consumer in the workspace** outside its own test file, so a distinct `AssemblyError` variant costs nothing and never will be cheaper. A predicate a caller may forget to call leaves them exactly where S24 found them; the finding's thesis was the compiler, and the PR's own title conceded *"a compiler **or a test**"*. Direction ruled, shape left to the lane. |
| **C-R6** | **Fix the two in-crate residues rather than reporting them.** `node.rs:700` was declined on a merge-conflict argument (#683 live) that has expired — #683 merged — and was never a scope argument, in a PR that already reached outside its scope column. `persist/mod.rs:285` carries the identical false advertisement in the same crate and **the sweep did not find it**, which the PR owes an account of. |
| **C-R7** | **Out-of-crate residues get an issue number and a §D row, not a sentence.** The PR marks S24 FIXED and deletes its Track C row, which would leave `step-import/recognize.rs:126` alive only inside the body of a finding labelled FIXED. That is the failure `REVIEW-STYLE-DISPATCH.md` names: the style lane becoming where known problems go to be recorded and forgotten. |

*What this row teaches the track:* the two questions Evan added to the style
brief did the work. Both came back **No** — not completely fixed, and not fixed
in the best way — and neither answer was reachable from the diff alone. The
first needed a sweep for siblings the PR had not named; the second needed the
reviewer to go and check whether the stronger encoding was affordable, which is
not a question the standing brief asks.

### #705 (C-c / S31) — both lanes, 2026-08-20: **not cleared**

This is the row that justifies the track's review policy in both directions at
once: the **adversarial** lane found the defect, and the **style** lane found
that the PR contradicted its own headline claim. Neither would have found the
other's.

**MAJOR-1 (adversarial).** The PR asserted, in its body *and* at
`SMELL-SCAN:3520`, that the curve-projection lift is *"bitwise the old code at
`T = f64` — the diff is only the wrapping."* It is not. `mid` is
`lo + 0.5*(hi − lo)`, which at `f64` is `x + 0.5*(x − x)` — **NaN for any
non-finite `x`** — so an overflowed residual now exits to
`ProjectionInconclusive` where it used to return `Ok` with an infinite
distance. The reviewer reproduced it **with all-finite inputs**: control points
at `1e200`, where `d.dot(d)` overflows, and `NurbsCurve3::new` validates
weights and counts but never coordinate magnitude. A semantic change riding
inside a diff whose text said "only the wrapping" is the exact class an
adversarial lane exists to catch, and no style reading would have found it.

The other six claims survived, and the method is worth recording: claim 1 was
verified by **whole-multiset numeric-literal extraction** across both old trees
against the new one — no value appears or disappears, every count drop matching
a named dedup — rather than by reading the diff. Claim 2 was tested literally
(158 lines each, `diff` empty after the constant renames). Claim 5 was tested by
**planting three un-aggregated suite files** and confirming the recursive guard
fired on each.

**RULED (C-R9):** keep the new refusal — an overflowed residual is not an honest
answer, and refusing is the fail-loud posture — but **retract the bitwise claim
in the durable record**, add a row using the finite-input fixture (none of the
224 covers it), and correct `projection.rs:43`'s `mid` doc, which claims
"bitwise the identity … no overflow at the representable extremes" and is false
at ±∞. That sentence is inherited from the surface half on main, so it predates
the PR; the curve half newly depends on it, which is what makes it this lane's.

**What the style lane found that the adversarial lane could not.** The header
merge's expensive failure did **not** occur — no normative sentence vanished or
became two contradictory ones, verified paragraph by paragraph. The damage was
in the *pointers*: type-level docs still say "see the crate docs" for material
that stayed in the module docs, so a reader following `DESIGN.md:369`'s
authoritative-text chain lands in the wrong file. And `geom::projection`
promises the shared policy is *"declared here, once, and neither half may hold
a private copy"* while **four paragraphs of that policy remain duplicated
word-for-word** — in the PR that named header-merging as its own headline risk.

Three findings reach past the PR:

- **`geom-brep/src/nurbs_iso.rs`** — Q4's second sub-case, and the sharpest
  finding of the night. The deleted acyclicity sentence recorded *where
  iso-curve extraction belongs*, not merely a crate-graph fact; that file says
  nothing about why it lives in `geom-brep`, and post-merge nothing structural
  stops it moving into `geom`. Deleting the sentence erased the only record of
  an intended invariant.
- **`SMELL-SCAN:5362` is Track D's D8 row**, and its scope column names a
  deleted path in a unit that has not run yet. Cross-track breakage, caught by a
  reviewer rather than by either orchestrator.
- **`docs/LIB-U1-SPEC.md:44` was misclassified as historical.** It is a
  *binding* spec enumerating the pncad façade's re-exports, which this PR
  changed.

*And a measured lesson about the sweeps themselves.* The PR disclosed that
`removal_pass_bound` — two line-for-line identical bodies — matched **none** of
Q1's self-declaration vocabulary, and then did not compensate with a
differently-shaped sweep of its own. The style reviewer ran two n-gram scans
and found two more unconfessed twins: `azimuth_frame`, open-coded bit-identically
at three sites plus three scaled variants, on a convention `DESIGN.md:1366`
already ratifies as **one**; and four byte-identical test converters under two
spellings. Q1's caveat that *the question is the instrument, not the pattern*
now has three worked examples in one PR.

### #702's corrected evidence — independently verified, 2026-08-20

The first round's MAJOR was an overstated mutation table. The fix pass restated
it, and **the restatement was not taken on trust** — a verification pass re-ran
every claim and then went beyond the lane's own list. That is now the rule for
this track: *a claim that has once been found overstated is re-derived, not
re-read.*

**It held.** Five of six claimed mutations reproduce exactly, three down to the
**named row identities** rather than merely the counts; the claimed zero is
real and was disclosed rather than hidden; ten of the verifier's twelve
independent mutations were caught, including every widening that would let a
*contradicted* declaration reach the frontier arm. The mixed-verdict fixture
the lane added is the sole or joint guard in four of those.

**What it found anyway, and it is the interesting one.** The
`StaleContactDeclaration` arm is **never executed by any row** — proved by
replacing the arm's body with a `panic!` and observing 584/584 still green. So
its `Attribution::Refuted` label is unpinned: a refuted declaration can be
relabelled a decline, promoted into `AssemblyError::Uncertified`, and reported
as an *unrefuted, uncertified frontier*, with nothing going red. That is the
dangerous direction, in the same dispatch the PR rewrote, and it is exactly the
confusion S24 was raised about.

The record carried the irony without noticing it: it quotes *"unfalsifiable by
execution"* as the diagnosis for which it schedules `step-import/recognize.rs:126`
under **#711**, while shipping a second instance of that inside the function
the PR rewrote.

Two smaller corrections, both about *how a claim is worded rather than whether
it is true*: one table row names a mutation and reports a neighbouring one's
count (the faithful reading of "hard-coded instead of table-sourced" reddens
nothing, because the assertion pins the string *value*, not its sourcing); and
the scope sentence says *"the last is a real hole"* where three exist, which
claims a complete search rather than reporting one.

*Generalisable:* **"a hole" and "the hole" are different claims**, and the
second is the one a reader acts on. The style brief's Q6 already asks whether a
disclosure owes a schedule; it does not ask whether the disclosure is
*exhaustive*. On this row that gap was the difference between a disclosed
limitation and an undisclosed live one.

### #705's MAJOR-1 fix — targeted re-read, 2026-08-20: **cleared**

Same rule as #702: *a claim once found overstated is re-derived, not re-read.*
The lane had now written a claim about this arithmetic **twice**, and the first
was false, so the adversarial lane was asked back for four checks and nothing
else. All four cleared.

- **The new row goes red for the right reason.** Reverting the refusal (making
  `mid` the identity at ±∞ again) fails it at the refusal assertion, reproducing
  the original Case B output exactly. And shrinking the fixture from `1e200` to
  `1e2` **also** fails it — at *"the fixture must overflow"* — so the guard is
  real: if the fixture stops overflowing the row goes red rather than passing
  for the wrong reason. That second mutation is the one worth noting; a refusal
  row that no longer reaches its own precondition is the classic silent pass.
- **The restated claim survives on a distinction worth keeping.** The qualifier
  is **"finite *arithmetic*", not "finite *inputs*"** — Case B has finite inputs
  and a non-finite intermediate, so it falls outside the claim rather than
  contradicting it. And the entry does not leave that to the reader: three
  sentences on it states outright that the case *"is reachable with all inputs
  finite"* and names the mechanism.
- **`mid`'s doc is true at ±∞ now**, and volunteers something the reviewer had
  found and not reported: `mid(-0.0) = +0.0`, the only finite-value departure
  from the identity, with the argument for why it is unobservable in
  `project`'s outputs.
- **MINOR-1 fixed**, verified by re-running the `pub`-declaration extraction
  *and* by a compile probe importing all 24 names both old crate roots exported
  through the new paths.

*Generalisable:* the useful discriminator was **"finite arithmetic" vs "finite
inputs"**, and it is a distinction a reviewer only reaches by constructing the
case where they come apart. The first claim was not sloppy — it was precise
about the wrong quantity.

### The finding a lane volunteered about itself, 2026-08-20

After both reviews cleared #705 and the merge was staged, the lane was asked
whether it was holding anything back. It was, and it is the sharpest self-report
of the night:

> **The overflow row exists on the curve half only. The surface half has the
> identical exposure and no guard.** … I fixed it at the reported instance and
> left the class — with the aggravating detail that **I am the one who made it a
> class.**

`mid`'s NaN-at-±∞ is, after this PR, a **stated contract on a shared helper**
pinned by exactly one of its callers. Someone who reads `mid`'s own doc — which
flags the asymmetry, because this PR added that — and makes it total gets a red
curve row and a **silent behaviour change in the surface half's certified
path**.

**RULED (C-R10): hold the merge, add the surface row.** The lane recommended
landing it as a follow-up. Overridden, on its own reasoning: this is the
standing failure the whole scan was raised to catch, stated in the first person,
on a live PR, by the lane that caused it — landing it as a follow-up would be
the report documenting its own thesis and then doing the thing anyway. And a
guard that tells the next person *half* the truth is worse than no guard,
because they will trust it. That trap is created by this PR and should not
outlive it by one merge.

*Generalisable, and it is MAJOR-1 one level up:* **a claim about a shared helper
is a claim about every caller, not the one whose diff you are looking at.** The
lane named its own cause exactly — *"I was reasoning about my diff rather than
about the mechanism I had just created."* Collapsing N copies into one helper
converts N independent local facts into one shared contract, and the guard
obligation moves with it. Neither review lane caught this; the lane did, after
the reviews had cleared it, when asked a direct question.

*Operationally:* **ask the lane whether it is holding anything back, after the
reviews clear and before the merge.** Both reviewers had finished. The question
cost one message.

### A monitor that failed silent

The lane's CI monitor used unauthenticated `curl` against a **private** repo:
every poll 404s, its filter read that as "no data", and it would have run its
full window and reported *"still running"* — silence indistinguishable from a
green wait. The lane noticed and fell back to hand-polling.

This is `memories/agent-lane-operations.md`'s **waiter self-test** rule earning
its keep: run a background waiter's detection expression *once in the
foreground* before arming it, because a catch-all arm converts a permanent error
into silent eternal waiting. Recorded as an instance rather than a new rule —
the rule already exists and is correct.

### A fence in a brief that pointed at a file that did not exist

C-b's brief carried a hard fence: *"`memories/telemetry-gating.md` names
`mesh::budget` as the worked example of the gating rule. If the module moves,
**the memory moves with it in the same PR**, or the memory's pointer is dead the
moment you land."*

**The memory had been deleted from main two days earlier** — `dd6d199`,
2026-08-18, *"cut unnecessary and harmful prescriptions from the orchestrator
reading path"*, verified. The orchestrator wrote the fence by copying a citation
out of S30's own body without checking it resolved; the smell scan's citation
was stale as of that commit, not as of the PR.

The lane checked instead of complying, said so, and made the right call
downstream of it: the *rule* is not homeless (`scripts/gates/no-ambient-env.sh`
carries it, enforced), so it updated that gate's worked-example pointer and
wrote **no new memory** — per `memories/cad-working-style.md`'s criteria, "the
meter moved to `tools/tess-meter`" is a fact git and two crates' module docs
already state. What earned a place was the placement *rule*, which went into the
existing `memories/tessellation-budget.md`.

*Generalisable, and it is this track's own failure mode pointed at itself:* **a
brief is a claim site too.** The fence was written with the same confidence as
the rest of the brief and was false on the day it was written. Two further
stale citations of the deleted memory survive at `SMELL-SCAN:3225` and in this
log's own roster row (now fixed) — the class, not the instance.

---

## Where this stands — session handoff, 2026-08-20

**All three units in flight landed.** Nothing new was started after the
wind-down (Evan, on the approaching usage limit); the three below finished
completely — reviews, fix passes and self-merges — and everything else on the
roster is untouched and unclaimed, exactly as it was.

| unit | state | who finishes it |
|---|---|---|
| **#702** C-a / S24 | **MERGED** `f382c4a` | — |
| **#705** C-c / S31 | **MERGED** `2e861932` | — |
| **#709** C-b / S30 | **MERGED** `33fff6f9` | — |

**Review artefacts survive the session** even if the reports do not reach the
orchestrator: `~/.local/share/cad-work/rev-709-findings.md` (adversarial) and
`rev-709-style-findings.md` (style), both written incrementally for exactly
this reason.

**Owed and unwritten — the loose ends this session leaves, all stated in
#709's body rather than left to be rediscovered.** #709 found that
the CSV's `agreement` column **measures nothing**: both sides of the ratio are
the same `Σ nuc·nvc` from the same `band_schedule`, so it is `≡ 1.0` by
arithmetic, the `≤ 1%` assertion on it was vacuous, and the module docs claimed
it *"verifies the lane's REALISATION of the schedule"*. `tess-lint`'s own report
legend already printed *"1.00 by construction"* — the tool knew and the docs
disagreed. Fixing it properly is a CSV schema change and a re-cut committed
baseline, correctly out of that unit's scope. **It has no §D row.** That is this
session's own instance of §C3 — *deferrals must land in a register that
executes* — and the next orchestrator should write the row before doing
anything else.

**Also unscheduled**, from the same source: `geom_core::k_stats` (598 lines,
~96 separable) is S30's class one crate over, reported and deliberately
untouched because its recording sits *inside* `decide`/`decide_flagged`/
`decide_invariant`, which are load-bearing kernel predicate doors — so the
`mesh::budget` split does not transfer mechanically.

### #709 (C-b / S30) — both lanes, 2026-08-20: **not cleared**

Twenty findings across the two lanes on a unit whose CI was green and whose PR
body was the most rigorous of the three. Neither lane found a shipped wrong
answer; both found **claims wider than their evidence**, which is now this
track's characteristic result.

**The adversarial break (claim 2), and it is the one worth carrying.** The PR
claimed the deleted per-triangle `assert!`'s falsification was *"preserved
exactly, and strengthened"* by reducing to `worst_ratio`. The floating-point
half is sound — verified across binades, a one-ULP violation still caught, and
genuinely stronger at `d = B = +inf`. **The population is not the same.** The
accumulators are local to the tessellation attempt and `note_face` runs only on
the *accepted* one, while the old assert fired inside **every** attempt,
discarded retries included. A certificate violation on a discarded attempt's
triangle was caught before and is silently dropped now. The lane's own comment
argues the reset is right *for the reported numbers* — true — and does not
notice it also narrows the falsifier.

Exposure is **zero today**: the reviewer instrumented the retry path and
measured 0 retries across the full tour sweep and the 152-test budget build. So
it is a real, unstated narrowing that nothing currently exercises.

*Generalisable:* **"the reported numbers are per accepted attempt" and "the
falsifier sees every triangle we tessellated" are different obligations that
happened to share an accumulator.** Scoping the accumulator correctly for one
silently rescoped the other. A falsifier that quietly stopped watching a case
is precisely the shape S30's own postmortem describes — a compliance check that
became the whole review.

**The style lane's counterpart:** the PR whose headline finding was *a vacuous
assertion* shipped one — `opt_cells <= patch_cells`, where `patch_cells` is
exactly the value the optimizer seeds its running minimum with. And its
correction of the false `agreement` sentence landed at **two of four sites**,
leaving it verbatim in the lint tool's docs and in `TESS-BUDGET.md`.

**Two findings reach past the PR**, both from the style lane: `docs/MESH-PROBEGATE-SPEC.md`
is a **binding spec whose entire subject no longer exists** after this PR,
untouched and unmarked; and ~1,050 lines of prose left the workspace rustdoc
gate, whose own header claims "no exclusions" and argues it exists because
*"prose that quietly stops rendering is a real loss"*.

*And the successor risk, stated by the style lane and worth keeping:* **the
danger is that "is it in `tools/`?" becomes the new compliance check** — S30's
lesson one level up, applied to S30's own fix.

---

## Landings

| PR | unit | state |
|---|---|---|
| **#702** | C-a / S24 | **MERGED 2026-08-20** (`f382c4a`). 37 checks terminal, zero failures. Two review rounds plus an independent verification pass. **Unblocks Track D's D7.** |
| **#705** | C-c / S31 | **MERGED 2026-08-20** (`2e861932`). 37 checks, zero failures; the three carrying rows pulled from the job logs individually rather than read off green ticks. Two crates → one `geom`. **Unblocks S32.** |
| **#709** | C-b / S30 | **MERGED 2026-08-20** (`33fff6f9`). 37 checks, zero failures. `mesh::budget` 898 → ~275 lines, eleven recording sites → one; the schema and the optimizer to `tools/tess-meter`; `probe_stats` deleted. |

### What #702 cost, and what that says about the policy

Two review rounds and a verification pass on a row the track had classed
**low-risk, style-review-only**. The style lane found a MAJOR anyway; the
verification pass found a second dead arm the corrected evidence had not
mentioned. Neither was a soundness bug — nothing shipped was ever wrong — and
both were about **evidence claiming more than it had**.

That is the calibration `REVIEW-STYLE-DISPATCH.md` predicted (*"expect findings
counts to rise… that is the instrument changing, not implementation quality"*),
and it is the argument for the two questions Evan added: neither MAJOR was
reachable from the diff, and both were reachable from *"is this completely
fixed, and is this the best way"*.

The lane's own closing disclosure is the one to keep: every local run it made
was `-p editor-core`, so its "575 passed" was evidence about **one crate**, and
`pncad`'s façade-parity gate caught three new exports on CI. It said so in the
PR rather than leaving it implicit. A green local count is scoped to the crates
it names.

## Follow-ups this track raised about itself

- **The gating paragraph is landing state too.** #698's *"six of these are
  edge-free and could start today"* list names findings by name, so a landing
  that leaves the §D table but stays in the paragraph makes the paragraph
  false. This is now part of the recording convention rather than something the
  orchestrator patches afterwards.
- **A long unit has no natural seam and will not invent one.** C-b lost nothing
  to the restart only because the orchestrator committed its tree for it. The
  implementer discipline already says *commit and push after every coherent
  unit*, and C-b's brief pointed at it — so the gap is not the rule. Briefs for
  large units should name where the seams are.
- **The confessed-copy grep is known-blind, and now measurably so.** C-c found
  `removal_pass_bound` — two line-for-line identical bodies — matching **none**
  of Q1's self-declaration vocabulary, because neither copy confessed. It was
  found by reading. Q1's own instrument caveat (*the question is the
  instrument*) has its worked example now.

---

## Session 2 — 2026-08-20, resumed

The handoff above was accurate and the tree matched it: `origin/main` at
`63e7770`, three units merged, nothing else on the roster claimed or dirty.
**Every gate the handoff listed as live has since fallen** — #682, #684, #690,
#692 and #705 are all merged — so ten of the twelve remaining lanes are
edge-free. The two that are not: **C-m** waits on A2/#649 (open as **#714**,
and stalled — Track A has no orchestrator driving it), and **C-n** is held
deliberately last.

### The row that was owed, and is now written

#709's body said the `agreement`-column finding *"has no §D row"* and told the
next orchestrator to write it **before doing anything else**. Done: **C9** and
**C10** are now rows in §D's Track C table, and **C-p** and **C-q** are lanes
here. C10 (`geom_core::k_stats`) was the other unscheduled residue from the
same source.

*What that cost is the argument for §C3.* Both were stated clearly, in the
right place, by a lane that knew exactly what it had found — and neither would
have executed, because a register that does not execute is not a register. The
gap between "disclosed in a merged PR body" and "has a row" is the whole of
§C3, and this track produced an instance of it within hours of quoting it.

### Two conditions this container imposes that the previous one did not

- **There is no `ssh` binary on this box**, so `local-scripts/new-lane.sh`
  cannot run: it clones `git@github.com:evgunter/cad.git`. Lanes are created
  instead by cloning the orchestrator checkout locally (hardlinked objects,
  ~4 s, ~38 MB) and repointing `origin` at HTTPS, which is the credentialed
  path here. The stand-in lives at `~/.local/share/cad-work/new-lane.sh` and
  sets `core.hooksPath` exactly as the committed script does — verified that
  the pre-push fmt hook fires and that a push authenticates. **This is a
  container fact, not a repo one**; the committed script is right for the
  environment it was written for and is left alone.
- **`~/.local/share/cad-work/` starts empty.** Nothing survived from the
  previous session — no lane clones, and the two #709 review-findings files
  the handoff points at (`rev-709-findings.md`,
  `rev-709-style-findings.md`) **are gone**. That handoff sentence is now
  false, and the lesson generalises: *a pointer at an uncommitted file in a
  container's home directory is a pointer at nothing, one preemption later.*
  Review artefacts worth surviving belong in a commit.

### The standing lane header

Every Track C lane is now dispatched against
`~/.local/share/cad-work/trackc-lane-header.md`, which carries what every
brief on this track was repeating by hand: the A/B fence, target-dir and
lane-private-path rules, the commit-and-push-at-seams rule with the two
incidents that earned it, the recording convention, **the four overstated
claims this track's reviews actually caught**, and the two obligations that
are this track's own (a claim about a shared helper is a claim about every
caller; a brief is a claim site too).

### In flight

Refreshed 2026-08-20 after the container reclaim. Two lanes flagged the
previous version as stale within minutes of each other and both correctly
declined to edit it — it is the orchestrator's table and concurrent lanes
cannot know who else is in it.

| lane | finding | PR | state |
|---|---|---|---|
| **C-d** | H12 — the SSI sweeps' other never-silence doors | **#734** | **MERGED** `c58a45ba` |
| **C-f** | H11 — #632's residues (**ten**, not the two the finding states) | **#731** | **MERGED** `96c109ec` |
| **C-h** | H14 — the census's record-keyed deferrals | **#737** | **MERGED** `ec12b7ce` |
| **C-o** | H16 — `StlOptions` → validated newtypes + per-format option structs | **#732** | **MERGED** `1948e2a5` — Evan signed off after ruling all seven §5 choices |
| **C-p** | C9 — the `agreement` column | **#738** | **MERGED** `a0a6e1a5` |
| **C-e** | H13 — `sweep_body`'s helix orientation coverage | **#779** | fix pass in, CI **35/2/0 green on `0494295e`** (verified at source, not taken); cleared to merge after writing row **C25** |
| **C-i** | H15 — #635's unclassified siblings (**23 rows**, not the three the finding names) | **#775** | **MERGED** `3fa135fa` |
| **C-q** | C10 — `geom_core::k_stats`, S30's class one crate over | **#801** | complete, **CI green on `9fb929d2`** (full matrix, TIER=all); adversarial + style reviews dispatched. Answer to C10 is **no, with the compiler as witness** (C-R22). Row **C22 released unused** — the lane declined to mint a row to have minted one. |
| **C-j** | S29 — the mesh sizing vocabulary (mechanical half) | **#803** | **MERGED** `d6b5ae01` — **before its style review, which is my dispatch gap, not the lane's**: the brief said a style review would happen and never said it was a merge gate. Post-hoc style review dispatched; its output is a follow-up PR. Row C23 landed. Policy half is a plan, unimplemented, waiting on Evan (C-R2). |
| **C-g** | S32 — `Surface`'s one-partial-per-call API and SSI's shadow enum | — | dispatched **plan-first** (C-R19 tier 2); **adversarial** + style; row **C24** reserved |

**Three reservations, and what they are reservations against.** C22/C23/C24
are the next free numbers *as of `1a94204d`* — C9 and C20 are holes (C9 landed
and left; C20 is #779's, unmerged). Per **C-R21** each lane re-verifies against
`origin/main` immediately before writing, because §D is shared and another
track can land a number in the ten minutes between assignment and write. That
is not a hypothetical: it is exactly how Track A's C11 collided with mine.

**Why C-g routes a plan before a diff.** S32 adds public API to a kernel
geometry enum (`Surface` gains a jet accessor) and deletes a downstream one
(`geom-brep`'s `Chart`). C-R19's discriminator is *whether a design element is
present*, not where the change lands, and "what should the enum's natural query
be" is a design element. The lane measures first and sends a ≤60-line plan; the
orchestrator decides whether it needs Evan. It is told explicitly not to idle
during the wait — oracle-building is available to it the whole time.

### #731 (C-f / H11) — style lane, 2026-08-20: **not cleared**

The second test of the style-review-only policy on a row classed low-risk, and
the second time a style lane alone returned a MAJOR. Both of the track's added
questions came back **No**.

**The finding said "two residues". There were four** — and the two the lane
added were each ruled out *in writing* by #632's own body: one dismissed as
"different enum (`DocParam`/`DocEdit`)" when it matches `DocEdit`, and one
under the flat assertion *"no fail-quiet wildcard in any `RoleSeg` or
`Qualifier` match in the workspace."* The review then found **two more**, so
the class is at least six. The sharpest is `eval/anchor.rs`'s `remap_seg`,
which ends `other => other` — a **binding** catch-all inside a match written
through `use RoleSeg as R`, so the literal `RoleSeg::` never appears in the
window #632's corrected scan required. Missed twice over, by two differently
shaped sweeps, and it is the one with a *wrong value* behind it rather than a
missed check: it rewrites profile locators on re-anchor, and a thirteenth
carrier variant would have crossed a re-anchor with a stale locator, silently.

**MAJOR-1, and it is this track's characteristic result arriving for the
second time.** The mutation table was measured on an intermediate working
state and never re-run: two of four rows are wrong, and **both omit the PR's
own headline fix**. Ruled at **C-R16**.

*Generalisable, and it now has two instances rather than one:* **a mutation
table is a measurement of a tree, and the tree it measures is the one that
was checked out when it was run.** #702's table overstated what its mutations
reddened; #731's was accurate about a tree that no longer existed. Both
passed a reader who checked whether the numbers were plausible rather than
whether they were current. The cheap discriminator is the one that caught
this: **a cited line number that matches neither the base nor the head is a
receipt for the tree the measurement actually ran on.**

**MAJOR-2 is a bug this repo has already had once.** `content_key`'s payload
wildcard sits under an *exhaustive* tag match, so the compiler forces a
decision at one site and silently defaults at the other — and S4's record
carries the realised failure: *"`Step::AtToward`'s memo content-key tag 28
COLLIDED with `ArcContinue`'s existing 28 … a hit would serve wrong
geometry."* Caught then by a reviewer, not by a type; still not caught by a
type. Ruled at **C-R17**.

**And the prose finding is §C16 in its purest form.** The PR's correction to
**§C15** repaired a false clause and left the paragraph asserting something
its own bullets do not support — *a prose-hygiene pass manufacturing the
defect it exists to remove*, committed inside a correction to the section
about half-fixes. It also corrected the record *about* #632's population while
leaving the two sentences that state it. Ruled at **C-R18**.

*What the reviewer's own instrument adds, and it is worth keeping.* It swept
by **match-arm content** rather than by scrutinee type — arms mentioning the
target variants, plus any catch-all, ignoring what is being matched on — which
sees through the `Option`/`Result`/tuple nesting that defeats
`clippy::wildcard_enum_match_arm`. That reproduced all five of the lane's hand
re-reads and returned eight more sites. It also stated its own blind spot
without being asked: **it is keyed to a fixed enum list, so `ExprKind`, `Entry`
and any enum not named were invisible until read by hand.** Three differently
shaped instruments have now been run at this one class and each found what the
previous two could not — which is Q1's *the question is the instrument* with a
third worked example.

### #732 (C-o / H16) — style lane, 2026-08-20: no MAJOR, twenty findings

The design PR, and the first row where the style lane's answer to track
question 1 turned on **an argument rather than a sweep**. Both questions came
back qualified rather than No.

**Where the asymmetry is not gone.** H16 is a finding *about an asymmetry
between two export doors*, so "is STL fixed" was never the question. The
reviewer found that `step-export/src/writer.rs:913-930` still hardcodes two
**caller-facing free-text** fields — `FILE_NAME`'s seventh argument, which is
Part 21's `authorisation` and is assigned by the standard to the **user**, and
the description list in `FILE_DESCRIPTION((''), '2;1')`. The PR's §7 names only
`'2;1'` and the schema name and then promotes them to the whole entity.

That is the load-bearing gap, because **§7's Part-21 argument is exactly what
makes STL's 80 bytes "the caller's"**. The argument that decides the finding
was incomplete in the direction that hurts: STEP keeps free text the caller
cannot reach, while STL's becomes settable in the same PR.

**Two guards blind to their own weakening.** Q3's shape, at the property the
PR itself says moved **from structural to checked**: mutating the sniff from
`starts_with(b"solid")` to `starts_with(b"solid ")` leaves the suite green,
because the fixture `"solid widget"` still trips it — the row sees the guard
*deleted* but not *narrowed*. Likewise dropping the name range's upper bound
admits DEL, `é`, emoji and U+2028 while the doc promises `0x20..=0x7E`, with
only `'\n'` actually pinned. `HeaderTooLong` is the counter-example done right.

And the predicate is narrower than the sentence above it: `"Solid widget"`,
`" solid widget"`, `"\tsolid widget"` and `"SOLID widget"` are all accepted,
so a reader that lowercases or trims before sniffing misreads those files.

**The sentence one level up was never re-read.** C-R1 named `ascii.rs`'s
*"constant in this build"* wording and the lane preserved it correctly — and
`crates/stl/src/lib.rs:10-15`, the crate header a reader meets **first**, still
says byte-identical output for *"identical inputs"* and never names
`StlOptions`. The mirror target does it right, naming `StepOptions` in its own
crate header.

*Generalisable:* **a ruling that names a sentence protects that sentence, and
a lane that satisfies it exactly has still only checked the sentence it was
handed.** C-R1's instruction was precise and was followed precisely; the defect
moved one level up, into prose nobody had been told to look at.

**The field the PR is built on is exercised by nothing but its own tests.**
`solid_name` is constructed at exactly two sites in the repo, both in
`crates/stl/tests/export.rs`; **every consumer-seat site sets `header`** — both
demos and `GUIDE.md`. So the headline correspondence (`product_name` ↔
`solid_name`) is between the field nobody uses and its STEP twin, while the
field everyone uses has no STEP counterpart at all. §5.3's argument for one
struct — *"a caller exporting both formats states its identity once"* — is
contradicted by every call site it has: both demos are binary-only and their
`solid_name` silently stays the default.

**A demo finding that must not be hidden.** Both demos pipe an arbitrary body
label into `header` under `unwrap_or_else(panic!)`, so a body named
`solid-block` — or any label over 80 bytes — hard-panics the demo. Per
`memories/demo-purpose.md` that is a library finding the demos surfaced, and
it is filed rather than smoothed away.

**A sweep whose blind spot was the glob, not the regex.** Both of the lane's
sweeps excluded `demos/` by path, which contains `uvdump::emit` — a door its
own verb list would have caught had it looked there. The lane declared regex
blindness and not path blindness. The conclusion still holds (an
independently-shaped third sweep reproduced the door set exactly and found no
seventh in-crate door); it is the **disclosure** that was incomplete, which is
§C15's question one turn further in: *a sweep's result is worth nothing
without a statement of what its pattern cannot match* — and a path glob is
part of the pattern.

### The C9 row I wrote this morning carried two stale claims — and the mechanism generalises

C-p checked its brief rather than complying with it (**C-R11**) and found that
**two of the claims in §D's C9 row are false against the tree**, not merely
imprecise:

- *"#709 corrected the sentence at two of four sites, leaving it in the lint
  docs and `TESS-BUDGET.md`"* — **true at review time, false on main.** #709's
  own fix-pass commit `46b44fd` is literally titled *"sweep the agreement
  correction to all four sites"*, and did.
- *"its `≤ 1%` assertion in `budget_meter` was vacuous"* — **that assertion no
  longer exists.** It went with the schema when #709 moved the columns;
  `budget_meter.rs` has carried no agreement assertion since.

Both came from the same place: **I wrote C9 out of #709's review record rather
than out of the tree.** The review record was accurate about the moment it
described. The fix pass then changed the tree, and nothing updated the record —
because a review record is not supposed to be updated. It is a record.

*Generalisable, and it is **C-c's pointer-versus-record discriminator turned on
this log itself**:* a review finding is **a record of what was observed at a
place and time**; a schedule row is **a pointer telling someone where to go**.
Writing the first into the second silently converts a dated observation into a
present-tense instruction, and it will be wrong exactly as often as the fix
pass that followed it did its job. **A row minted from a review finding is
re-derived against the tree before it is written**, not transcribed.

That is now the third of five briefs this session to carry a citation that did
not resolve (C-h's line number, C-f's scope path, and this). The rate is no
longer anecdotal, and C-R11's instruction — *check rather than comply* — has
paid for itself three times. What is new here is the **direction**: the earlier
two were stale *locations*, this one is a stale *claim*, and a stale claim is
the more expensive kind because a lane cannot discover it by failing to find
the file.

What C-p found live instead, once it stopped trusting the row: the column
itself, `TESS-BUDGET.md`'s *"A REAL agreement check is owed and unscheduled"*,
and **seven item-doc and report-legend sites** #709's module-doc sweep missed —
including `main.rs:181`'s legend, still reading *"agree = the lane's realised
cell count vs the same schedule's sum"* with *"(1.00 by construction)"*
appended after it. The tool's own legend disagreed with itself in one sentence.

### #737 (C-h / H14) — style lane, 2026-08-20: **not cleared** (adversarial lane still running)

The unit that found a **live wrong answer** — a 1 m cube inside a 4 m cube with
four bottom corners declared v-on-f, every record geometrically true, returning
`Ok(())` at the merge base while the same body undeclared returns
`CensusUndecidable`. **Declaring a true contact switched the containment
examination off.** The one pre-existing guard bridges its pair with a *bogus*
record and measures the staleness refusal, so it stays green under the reverted
fix.

Both track questions came back qualified: fixed **at the instance**, and *"mostly
yes on the deletion, no on its neighbours."*

**The finding of the review is that the PR's own honest-cost fixtures are not
exercising the arm it fixed.** The style lane instrumented `conformal_pair()`
and found it carries two `mvfs` seed faces that are **placeholder `Nurbs`**;
`face_box_rule` sends those down `ControlNet`, a placeholder net is four
`poison_point()`s, and `Real::min`/`max` propagate NaN **by documented
contract**. So each solid's `solid_reach` box is entirely NaN and **all six
containment margins are uncallable**. Causation was proved rather than argued:
excluding placeholder faces from the fold flips the verdict text from *"not
definitely separable … in band"* to *"one instance's extent box inside
another's"*.

*The consequence, and it is the sharp one:* **two sheets a kilometre apart
would produce the identical refusal.** On the two fixtures the PR offers as the
honest cost of deleting the skip, the arm it fixed is not examining anything.

Three riders came with it, each a claim the PR makes elsewhere:
`census.rs:1207-1211`'s *"An unboxable kind is `None` here"* is false for this
case; the typed *"unclaimable extent"* guard at `:1590` is dead for it; and
**the placeholder exclusion the PR expanded a paragraph to justify is applied
in arm 1 only**, while arm 2's two folds walk `body.faces.iter()` raw — the PR
argued for an exclusion and did not apply it in the arm it rewrote.

**Two deferrals pointing at each other.** Backstop `:1486` skips same-key curved
pairs as *"the conformal arm's pair"*; the conformal arm `:972` skips same-sense
pairs with a bare `continue`. A cross-solid same-key same-sense pair is decided
by **neither**. This PR re-audited that exact skip and wrote a **new paragraph
defending it**, whose test stops at *"pairs the conformal arm never walks"* —
walking, not deciding — three paragraphs after the PR itself raises the bar to
*"asks the SAME question"*.

**Residue 2's defect is inside the function the PR rewrote.** `census.rs:1146`
carries the identical empty-loop `continue`, and `:1423`'s
`if pts.is_empty() { continue; }` drops an unbounded face out of the backstop
**with no error and no comment** — in the function whose own header says the
census must never silently not-examine. `boundary_reach:1293` gets it right, so
both handlings live in the same file and the PR touched neither.

**And the question this row was asked to answer came back No.** Three
instruments have now found this defect in this function and none of them was a
sweep: S49 found arm 1's, #637 left two residues, this PR found a third
instance in arm 1. The PR's instrument against a fourth is **a paragraph**. A
differently-shaped sweep of residue 2's class returns **29 sites in 18 files**,
and `scripts/gates/` already holds three allowlist-shaped gates — which is the
shape a deferral register would take if anyone built one.

*Operationally, and it worked:* the style lane finished first and flagged two
findings as correctness-lane candidates. **They were forwarded to the
adversarial lane mid-review** rather than held for the fix pass. #705 is the
precedent — there the two lanes found things neither would have found alone;
here one lane's measurement reframes the other's central question before it
finishes.

## Incident — container RECLAIM, 2026-08-20: a different failure from the two restarts

**Nothing pushed was lost. Nothing unpushed survived.** Five lanes were live —
three fix passes and two reviewers. All five PRs are intact **with their
fix-pass work on them**, because every lane had committed and pushed at its
seams. The commit-and-push rule is the entire reason this cost a re-dispatch
rather than a day.

**But this is not the failure the death-recovery rule describes, and the
difference matters.** `memories/agent-lane-operations.md` says a dead
subagent's **transcript and worktree survive**, so `SendMessage` resumes it —
and that was true twice this session, where four agents were resumed from
transcript and none needed re-briefing. Those were container **restarts inside
a live session**.

This was a **reclaim after ~5.5 hours idle**, and it took three things
together:

- the lane clones under `~/.local/share/cad-work/`;
- the **subagent transcripts** under the session's `tasks/` tree — checked, not
  assumed: `ListAgents` returns *"No reachable agents"* and the `.output` files
  are gone;
- the session scratchpad.

**So resume-from-transcript was not available.** The recovery is re-dispatch
from the pushed heads, with the outstanding work re-derived from the commits
and from the orchestrator's own record. That is exactly why the review records
in this file are written per unit as they land rather than at the end: the
three completed reviews (#731, #732, #737) survived in full, and the one that
had not yet reported — **#734's style review — was lost entirely and is being
re-run from scratch.**

*The rule that needs the amendment:* **"resume is cheaper than restart" holds
only while the transcript exists, and a reclaim is not a restart.** Prefer
resume when an agent dies inside a live session; assume re-dispatch when the
session itself has been idle long enough to be reaped. The cheap insurance is
unchanged and is what worked: **push at every seam, and write reviewer findings
to a file as they are settled rather than holding a verdict in context.** Both
of those are already rules here; both paid.

**What the reclaim also took, and what that cost:** the standing lane header,
which lived at `~/.local/share/cad-work/trackc-lane-header.md` and which every
lane was dispatched against **by path**. It is now committed above, in this
file. That is the **second instance of one lesson in one session** — #745
landed eight rulings that were committed and pushed to the orchestrator branch,
cited by number in briefs, and unreachable because they had not **merged**.
This one was never committed at all.

> **A register that has not landed is not a register**, and a brief that lives
> only in a container's home directory is not a brief. The test is not "is it
> saved" — it is *"can the person who needs it read it from the tree."*

### #738 (C-p / C9) — style lane + fix pass, 2026-08-20: **MERGED** `a0a6e1a5`

The decision unit. C9 asked whether a realisation check was worth having at
all; the lane answered **no** and deleted the column rather than re-deriving it,
with the argument recorded in `docs/TESS-BUDGET.md` under *"Why there is no
realisation column"* — where the deferral it discharges lived, not only in a PR
body. All four legs survived the review's re-derivation.

**The finding of the review, and the lane's own account of it is better than
mine.** The PR whose subject is a vacuous ratio **shipped a vacuous assertion**:
`span_opt_cells <= grid_cells`, defended in the body as *"it can fail: the two
sides are different derivations."* They are not, on the only fixture the crate
has — every NURBS face of `loft_prism` has `cells = 1`, so one knot-span cell,
one band, no max across a band, and the inequality holds **by construction of
the optimizer's loop**. That is the verbatim mechanism quoted six lines below it
as the reason for deleting its sibling.

*Generalisable, and it is the sharpest methodological result of the session:*

> The sweep was a read of `assert!`/`assert_eq!` **text**, so it could see an
> assertion vacuous by its own **form** but not one vacuous because **its
> fixture cannot reach its precondition**. The lane wrote that blind spot down
> and then shipped an instance of it *inside* the blind spot it had just
> declared. **The instrument that eventually found it was not a better sweep —
> it was running the fixture and printing the values**, which is what the sweep
> owed every assertion whose truth depends on data.

**A declared blind spot is not a discharged one.** This track has now seen the
shape three times — Q1's confessed-copy grep, the arm-content sweep, this — and
each time the fix was **a differently-shaped instrument, not a wider pattern**.

**The question the review could not answer, the baseline answered.** Asked
whether a multi-band fixture was worth building, the lane went to the committed
baseline instead of building one: **56 of its 64 NURBS rows are already
multi-cell**, so multi-band faces are the tour's norm rather than something
reachable only from `crates/mesh` — which its own replacement comment had
claimed. On all 56, `span_opt_cells / grid_cells` runs **0.16 to 0.73**: the
sides do separate once `cells > 1`, and the ordering never comes within a
quarter of inverting. So a multi-band fixture buys **a guard, not a detector**,
and deletion is honest rather than a fallback. *Answering "is this fixture worth
building" from data already committed is cheaper than building it, and is
available more often than anyone reaches for it.*

**Two live holes left with executing homes rather than sentences** (C-R7):
issue **#746** + row **C15** for the per-face slack key — two directions on one
key, since a permuted ordinal compares two unrelated faces and, worse, a
permutation landing a baseline NURBS ordinal on a fresh non-NURBS face **drops
the face from the gate with no finding at all**. And row **C17** for the
doc-gate copies, middle tier under C-R19 because the question is whether the
gate stops being workspace-scoped, not whether to add three more copies.

**A conflict resolved on C-c's discriminator, which keeps earning its keep.**
Main deleted `TESS-SPAN-SPEC.md` with 108 other closed-unit artifacts into the
new `docs/DOC-LEDGER.md` while this lane held a modify. The lane **took the
deletion** — a spec the repo deliberately retired is not resurrected to carry a
bracket — and split the survivors: `TESS-BUDGET.md:3` is a **pointer** and was
repointed at #594 and the ledger; `MODEL-AB-LOG.md:1058` and `:1153` are
**records** of what was measured at spec time and keep the name they were
observed under.

*Worth carrying past this unit:* **nothing gates a doc pointer against the file
it names.** `scripts/gates/` holds fourteen gates and none checks that a `docs/`
path in prose still resolves — so that pointer was set to survive indefinitely,
and 108 files were just deleted. Not this track's to close and not minted here;
recorded because the ledger records *what was deleted* and nothing checks *who
still points at it*.

### #731's verification pass — the same error, twice, in the same PR

**C-R16 exists because #731's first mutation table was measured on an
intermediate working state and never re-run.** The fix pass re-derived it. The
verification pass then re-derived the re-derivation, and **one row of five is
wrong the same way**: probe D's head figure is 6 where it measures **7**, and
the missing span is `expr.rs:806` — `with_replaced`, **the PR's own newest
fix**. The row reproduces exactly at `0f902553`, *the commit before*
`571c34c4` closed that wildcard.

So the defect recurred **inside the correction to itself**, and both times the
omitted site was the lane's most recent fix. That makes the mechanism visible,
and it is not carelessness:

> **A mutation table always lags the diff by exactly the last fix** — you run
> the probes, then you fix one more thing, and nothing re-runs the table. The
> lagging row is therefore the *newest* one, which is the row most likely to be
> the finding's own headline.

The cheap tells, both present twice: a cited line matching neither base nor
head, and **the body's own prose contradicting the table beside it** (here, two
paragraphs below, naming `expr.rs:804` as found and closed while the table
omits it). *Re-run the probes after the last code change, not before* is now
the operative sentence — an ordering rule, not an exhortation to be careful.

**A tenth class member, found by a third instrument.** `doc.rs:65`'s
`DocParam::bit_eq` — `match (self, other) { … _ => false }` with arms spelled
`Self::Continuous` / `Self::Count`. Same shape, same crate, **one of the PR's
own four target enums**, and the **same alias tell** as the `use RoleSeg as R`
miss the style lane caught, which is why neither earlier instrument saw it. A
future `DocParam` variant compares unequal to itself, and it feeds `Doc::bit_eq`
and `diff.rs` — D7 replay identity and change detection.

*The instrument that found it is the generalisable part:* a whole-file
brace/string/comment-accurate scan with **no window**, covering `match`,
`if let`, `while let` and `matches!`, keyed on **every enum-variant name in
`crates/`** rather than a hand-picked vocabulary, with guarded arms excluded per
rustc's own rule. Three instruments have now been run at this one class, each
found what the previous could not, and **the progression was never a wider
pattern — it was a differently-shaped one**: prose vocabulary → arm content →
whole-file, vocabulary-free.

**And one methodological correction worth keeping.** The PR claims its
instrument is self-tested — run against `main` it rediscovers `remap_seg`
unaided. **The PR ships no script**, so that claim cannot be re-executed; the
verification confirmed the *property* on an independently written instrument of
the same family. That is weaker evidence than a runnable check and the body
must not imply otherwise. **A claim about an instrument owes the instrument.**

### #732 (C-o / H16) — complete, green, **waiting on Evan's sign-off**

The design PR. All four of Evan's rulings (C-R13/C-R14/C-R15) landed, the style
review's twenty findings are addressed, and the byte-identity probe was re-run
on the merged head rather than carried forward.

**The re-run earned more than the number it confirmed.** The pre-merge figure
reproduced exactly — 13/13 binary byte-identical, 13/13 ASCII differing in
exactly the two `solid`/`endsolid` lines C-R13 moves, four diff lines per file
and nothing else. But the reason for asking got its own measurement: `crates/mesh`
moved in three files across the 106 commits, so the lane exported the same
thirteen fixtures at **both** main SHAs and compared — **13/13 binary
byte-identical**. None of the three mesh changes reaches any fixture in the set.

*That is C-R16's point stated positively:* **the stale measurement happened to
still be true, and only the run could establish that.** "It probably still holds"
and "it holds, and here is what would have made it not" are different claims, and
the second is the one a reader can act on. The probe is deliberately **not
committed** — it is a measurement, not a fixture.

### An operational finding about §D itself, and it is not this track's alone

The lane's closing disclosure:

> This branch went `dirty` **twice within minutes of going green**, both times on
> §D's table, and both times the resolution needed a judgement about someone
> else's landed row.

**The window between "green and clean" and "CONFLICTING" on `docs/SMELL-SCAN-2026-08.md`
is now shorter than one CI run.** Five programmes are live and every unit of
every one of them edits the same table by construction — the recording
convention requires it. Nothing has been lost: *resolve by merging, keep both
sides* held on all three of this lane's merges, but only because each side's
edits were to different rows, which is luck holding rather than a mechanism
working.

Two consequences already visible, both recorded rather than fixed here:

- **C-R21's rule is doing real work** — three of this session's row numbers moved
  under a lane between assignment and write, one of them across tracks.
- **A green PR on this document is a perishable result.** Expect to re-merge
  before merging; do not read "clean an hour ago" as clean.

This is a cross-programme question (a per-track table? a landing lock? a
generated index?) and belongs to whoever owns §D's shape, not to Track C. Named
here because Track C produced the measurement.

### #731 (C-f / H11) — **MERGED** `96c109ec`, after three rounds

**The count is the finding.** H11's whole statement was the clause *"#632's two
residues"*. Two were recorded; **ten existed**, all in `editor-core`, all the
same shape. Three of the eight extra had been ruled out **in writing** by #632's
own body — including the flat *"no fail-quiet wildcard in any `RoleSeg` or
`Qualifier` match in the workspace"*, which was false.

*The mechanism by which the count moved is the transferable part:* **two → four
→ nine → ten, and every move came from a differently shaped instrument, never
from looking harder with the same one.** Prose vocabulary; then type-aware
clippy attribution; then arm-content-directed (scrutinee type never consulted,
so nesting and aliasing are invisible by construction); then whole-file,
window-free, keyed on every enum-variant name in `crates/`.

**Row 10 is the argument for that in one site.** `DocParam::bit_eq` was missed
by instrument 1 because its scrutinee is a **tuple** — the blind spot round 1
declared — and missed by instrument 2 because its arms are spelled `Self::` and
name no enum at all, **the same alias tell as the `use RoleSeg as R` miss that
started the unit**, defeating the instrument built to be immune to exactly that.
Two instruments, two different reasons, one site.

**Row 6 is the one with a realised precedent.** `content_key`'s tag match was
exhaustive and its payload match was `_ => {}`, so the compiler forced a
decision at one half of one memo key and defaulted at the other — and S4 already
records the realised failure: *"`Step::AtToward`'s memo content-key tag 28
COLLIDED with `ArcContinue`'s existing 28 … a hit would serve wrong geometry"*,
caught by a reviewer, not by a type. Same key, same mechanism, second time.

### The probe table failed twice, and round 3 fixed the *class* rather than the cell

Round 1's table was measured on an intermediate tree; **round 2's re-derivation
of it was too**, one row over — probe D said 6, measured 7, and the missing span
was `expr.rs:804`, the PR's own newest fix. So C-R16's defect recurred inside
the correction to C-R16's defect.

The fix that matters is not the corrected number. **Round 3 changed the line-number
convention: every cited line is now a line of the file *as it ships*, probe
reverted**, so `sed -n 804p` lands on the cited `match`. The earlier tables gave
**mutated-tree** numbers, whose offset depends on how many lines the inserted
probe variant occupies — which is why round 1's `resolve/mod.rs:977` matched
neither tree and why probe C's `node.rs` cells sat exactly two lines high.

*Generalisable:* **a mutation table cites the tree the reader has, not the tree
the measurer had.** Under the old convention the diagnostic tell (a line
matching neither base nor head) was indistinguishable from ordinary probe
offset, so the error was invisible by construction; under the new one every cell
is checkable with `sed` in one second. That is the difference between correcting
an instance and removing the class — and it is what a fourth round would
otherwise have been spent on.

The ordering rule stands beside it: **run the probes after the last code change,
not before**, because the table otherwise lags the diff by exactly the last fix
— the one most likely to be the finding's headline.

**Two cross-cutting notes recorded rather than acted on.** #731 went CONFLICTING
**a second time mid-fix-pass** as main took five merges — the §D contention
window again. And Track E's E-e row calls C-f *"disjoint by inspection"* from
`eval/`, which C-f edits; both sentences already say to read C-f's head instead
of themselves, which is what saves it. The lane reported it rather than editing
another track's live table, which is right.

### #734 (C-d / H12) — **MERGED** `c58a45ba`

Tests only, no source file touched, and the strongest self-correcting pass of
the session. Three results, all of the same kind:

**The grid was nine and is thirteen, and probing found that reading had not.**
The first enumeration counted refusal *sites*. Instrumenting both `exhaust`
entry points on every row showed the budget check and both poison arms are
reached under **both** `SweepDuty` values, on separate calls with separate
floors. **Duty is precisely the axis S23's postmortem names as the one the
original bug lived on** (`sweep` read its duty off `tubes.is_empty()`), so
omitting it was the same mistake one level up. Three cells had rows, five more
do, **five remain** — recorded as row **C18** with the negative result written
in, rather than dropped to make the count look complete.

**The guard the lane wrote to satisfy the review was not a guard, and its own
mutation said so.** The natural chart FLOOR-TIE is a *ratio* between the two
runs' reported floors — and **any scale shared by both translations cancels out
of a ratio**. Dividing the accounting floor by `speed²`, a second scale entering
the very expression the assertion exists to guard, left the whole suite green.
Found by mutating a five-minute-old assertion, not by re-reading it.

> *A guard is not a guard until a mutation says so — including a guard written
> five minutes ago to satisfy a review.*

**And the cost claim was corrected before it shipped**: the first draft said the
new block ran in *"well under a second"*; measured, it is **3.0 s against 2.9 s
for the file's other nineteen rows** — it roughly doubles the file, and two rows
are all of it.

**The headline measurement was comparing unlike things.** `UvRect::contained_in`
has one consumer; the ℝ³ twin's *inherent* method has three. Restated like for
like at the `SweepCell` forwarder — **1 red against 3** — with the 6 kept and
decomposed (the extra three are the predicate's own unit test plus two
adversarial rows that redden only when a second, unrelated consumer breaks at
the same time).

**H12 ruled discharged** on the finding's own terms: it is about doors lacking
acceptance rows, and every reachable cell got one. The four Account-duty cells
are unreachable by the naive road — measured, `floor_scale` ∈ {0, 1e-12, 1e-9}
still terminates `Ok` on both fixtures — so what is needed is a new fixture, not
a new assertion. The live `+∞` guard hole is **issue #762**, deliberately with
no §D row: kernel work, C-R19's third tier.

### The row-number namespace is wider than one document, and my check was not

Assigning C18 I checked `SMELL-SCAN-2026-08.md` for a colliding `## C18.`
section and reported *"no §C18 exists to collide with it."* The lane checked
further and corrected me on two counts:

- **Inside that file**, §C's lessons run **C1–C17**, so `§C18` is the *next free
  lesson number in the same document* — the collision is not absent, it is
  merely not minted yet. The file already tolerates exactly this at **C15** and
  **C17**, where a §D row and a §C lesson share a string.
- **`docs/SMELL-SCAN-2-2026-08.md` already has `## C18.`** — a different
  document's namespace, so not a collision, but it defeats anyone grepping
  `C18` across `docs/`.

*My claim was scoped to one file and stated unscoped* — the same defect this
track keeps finding in lanes, arriving in an orchestrator's verification of a
row number. **C-R11 applies to the orchestrator too**, and the lane applying it
to me is the instrument working as intended.

The mitigation already in use is #731's `§CNN` convention (bare `CNN` is a §D
schedule row, `§CNN` a §C observation), now stated once for all three colliding
numbers rather than re-derived per row. It does not survive a second document
adopting the same letter, which is now the live situation.

### The `C<n>` namespace is saturated, and it always was

Assigning C19 I mapped the whole namespace rather than checking one number, after
being corrected twice on exactly this. The result:

**§C's process observations in `docs/SMELL-SCAN-2026-08.md` run C1 through C25.**
§D's Track C schedule rows currently run C1–C8, C10, C11, C12, C15, C17, C18.

So **every Track C row number from C1 up has a §C twin in the same document, and
has had from the first row.** C15 and C17 were noticed as collisions; C18 was
noticed as an impending one; C19 turned out to be already taken. None of those
was an accident — the two sequences share a letter and a document and have
always overlapped completely.

*What this corrects, and it is mine:* I checked C18 by grepping one file for one
number and reported *"no §C18 exists to collide with it"*. That was wrong twice
over — §C18 was minted by the time I said it, and the correct question was never
"is this number taken" but **"do these two sequences share a namespace at all"**,
which is answered once and for good rather than per row. **A per-instance check
cannot discover a structural collision**; it can only keep failing to notice one.

The mitigation in use is #731's convention — bare `CNN` is a §D schedule row,
`§CNN` a §C observation — applied per citation. It works, and it is a convention
rather than a mechanism: nothing enforces it, and it does not survive a reader
grepping `C19` across `docs/`.

**Not fixed here.** Renaming one sequence touches every citation in a
6,800-line document that five live programmes are editing concurrently, and
which sequence should move is a §D-shape question belonging to whoever owns that
document — the same owner as the conflict-window problem recorded above. Track C
produced both measurements and neither ruling.

### A companion clause to §C15, earned by C-i and worth having

§C15 says a sweep's result is worth nothing without a statement of what its
pattern cannot match. C-i's closing disclosure found the other half:

> sites 7, 11 and 12 were **in** I2's and I4's output and I missed them by
> reading a 390-line list from its tail; a literal `rg 'until SSI'` after the
> first fix pass recovered them.

The instrument matched correctly. **The reading of its output failed.** And the
cheapest of four instruments — a bare `rg` for a literal string — is what
finally proved the class closed, after three elaborate ones had produced the
material and had it misread.

The clause, as the lane stated it:

> §C15 asks a sweep to declare what its pattern cannot match; this asks it to
> declare **how its output was actually read** — because an instrument that
> matched correctly and a reader who skimmed its tail produce the identical
> artefact, **a confident negative result**, and only the second is invisible in
> the record.

*Why this is not a restatement of §C15.* Every previous lesson on this track has
been about an instrument's blind spot — the pattern could not reach the site, so
the negative result was honestly wrong. This one is about a **reading** blind
spot, where the negative result is dishonestly right: the evidence was in hand
and the conclusion was drawn past it. Nothing in the existing discipline asks
after that, and a 390-line hit list is exactly the artefact that invites it.

*Operationally:* a sweep reporting more hits than a person will read owes either
a mechanical filter down to a readable set, or a statement that its tail was
sampled rather than read.

**CORRECTION, and it is the orchestrator's — 2026-08-20.** This entry first
ended by recommending C-i's own recovery as the cheap check: *"after the fix
pass, re-run the narrowest literal query the class admits and confirm it returns
zero."* **That is wrong, and #775's style review proved it on this very unit.**
A fixed-string grep for the wording you have just retired **cannot see a class,
only a phrasing** — it proves your edits landed and says nothing about members
surviving under different words. #775 reported an unqualified zero on that basis
and **three live members of the class stood on the shipped head**, one of them a
public enum variant doc.

Worse, all four of the unit's instruments had *seen* those three and dropped them
**by selection rule, not by regex** — one required the live-scope marker before
the milestone token where it followed, another discarded any clause carrying a
milestone token at all. So the population was bounded by the instruments' own
rules, and the closing grep could not have discovered that because it was
narrower than any of them.

*The rule that survives:* **a closure check must be wider than the instrument
that found the population, not narrower.** The narrow literal re-grep is a
useful check that your edits landed — it is not a closure argument, and calling
it one converts a self-test into a false negative result. I repeated the lane's
version of this without testing it, which is the same failure one level up.

### I nudged a finished lane, on two bad liveness signals — both are in my own check-in instructions

C-o was done, pushed and green. I read it as an hour with no work, and both
signals I used were wrong:

- **`target/` directory mtime is not a build-activity signal.** I read
  `c-o/target` at 15:09 and concluded nothing had been built in two and a half
  hours. Measured after the fact: **2,655 files under it are newer than 16:00.**
  A directory's mtime moves only when its own entries are added or removed;
  cargo rewrites fingerprints and artefacts *deeper in the tree*, so a busy
  `target/` can keep a stale mtime indefinitely. The correct probe is
  `find <target> -type f -newermt <time> | head`, not `stat` on the directory.
- **`git log --oneline -6` is too shallow when `main` merges hourly.** The
  lane's two work commits were four and five deep behind `origin/main` merges
  it had taken on top of them. The window has to be sized against main's merge
  rate, or the query has to exclude merges (`--no-merges`) — which is what I
  had used earlier in the session and stopped using.

**My own check-in prompt tells a successor to use both**, which is how this
would have recurred. Corrected there.

*What it cost, and what it did not.* One round of the lane's attention and a
status message that read as doubt of a lane that had done the work. Cheap, and
the lane corrected me flatly and with evidence, which is the behaviour the
briefs ask for. **What it nearly cost is the interesting part**: the escalation
order in `memories/agent-lane-operations.md` is nudge → check the lane → TaskStop
after a full battery-length window. Had I trusted these two signals one step
further I would have killed a finished, green unit — and the memory's own rule
that *"transcript-mtime is NOT a liveness signal"* is the same lesson about a
different clock, already written down, already generalising further than I
applied it.

*The rule underneath all three:* **a liveness signal must be something the work
necessarily touches.** A transcript, a directory's own mtime and the tip of a
merge-heavy log are all things a working lane can leave untouched for hours. A
new commit reachable with `--no-merges`, or a file written under `target/`, are
not.

### #732 (C-o / H16) — **MERGED** `1948e2a5`, the track's only design PR

Seven §5 choices, all ruled by Evan; two of them changed the API's shape and
were implemented as **one** design rather than two refactors, because the
intermediate tree would have had validation in one place and inert fields in
the other — and a mutation table measured on a tree nobody ships.

**Evan's pushback on 5.3 found a defect, not a better framing.** The inherited
argument was *"two structs would remove the inert field at the cost of making
the pair invisible."* His reply — *"I'd be surprised if there weren't a way to
have two structs where the pair is not invisible"* — is right, and the
dichotomy was the orchestrator's, not his. **"Invisible pair" is not a cost of
two structs; it is a cost of two structs and no other structure.**

And two structs bought something one could not: with `write_binary` taking only
what the binary writer reads, **the inert-field class became unrepresentable**
— including its live instance, which the style review had already found (both
demos are binary-only, their `solid_name` silently stayed default, so an ASCII
export added later would ship the default name for a body already named). A
hazard that had to be *disclosed* under one struct is *unstateable* under two.

**The three rejections are the useful part of 5.3's answer**, and each cites
something already in this scan: a convenience struct holding both *recreates
the inert field one level up*; a trait would be one method, two impls, no
generic consumer — **S55's shape, live in this same scan**; a `From` pair would
convert a part name into a producer header, **the exact category error C-R13
ruled against**.

**5.7's newtypes moved three arms out of `StlError`**, and the reason is this
track's own: after the move the writers cannot produce them, and *a public enum
arm no door can return is a claim nothing can falsify* — #702's dead
`StaleContactDeclaration`, proved dead by replacing its body with `panic!`.
Two error types rather than one shared `OptionsError`, for the same reason one
level down: a name cannot be too long, a header cannot be unrepresentable.

*And the property that stopped being a test:* the refusal rows used to prove
*"refused before any byte is written, so a refusal never leaves a partial
file."* Under construction-time validation that is no longer testable — it is
**unrepresentable**, and the rows now call the constructors. Admissible rows
still export through `write_ascii`, so the newtype's idea of admissible and the
writer's cannot drift.

### A red CI run that was neither this diff's nor a flake

One run failed at `profile::path_property::sharp_polygons_differential_and_verified`
(eps 1e-6) on a head whose diff touches no `profile` code, and **the next run
was green because the seed is random**. The lane derived the cause instead of
re-rolling: from the four printed coordinates alone, segments 1 and 3 do cross —
exactly the pair the verifier named — because `convex_polygon()` rescales its
gaps by `TAU/total`, one scaled gap reaches **4.322 > π**, the origin falls
outside the polygon, and increasing-angle order stops implying simplicity. The
generator's docstring claims *"convex"* and *"gaps ≥ 0.15 rad"*; **neither
survives the rescale.** The verifier was right and the generator wrong. Filed
as **#774** with the seed, the minimal input and three candidate fixes.

*This is the "flake is not a root cause" rule paying off in the direction that
is easy to miss.* The rule usually stops someone re-running a red until it goes
green. Here the re-run went green **on its own**, which is the case where
nothing forces the question — and the finding was a real defect in another
crate's test generator, reachable by any seed.

### A third category for §S39, found by C-i: the prediction that came true the other way

§S39 asks one question of a stale-looking claim: **benign rot, or a latent
defect wearing a documentation costume?** #775 found a member that is neither.

`geom-brep/src/edge_geometry.rs:285` — a **public enum variant doc** — said the
witness would double as *"the marching seed when numerical SSI arrives (M3+)"*.
SSI arrived. The sentence is still false, and **not because it went stale**:
`ssi::certify` states that nothing about the witness contract moves at rung 3,
and `exhaust::seed_r3` seeds from surviving cell centres instead. The prediction
**came true the other way** — the thing it was waiting for shipped, and shipped
differently.

*Why it is a third category and not a wording of the first two.* Benign rot means
the subject moved and the sentence did not; the repair is a corrected sentence.
A latent defect means something **was** meant to hold and the code drifted; the
repair is the code, and deleting the sentence would erase the only record of the
intent. Here the subject arrived **on schedule**, and **nothing was ever meant to
hold** — so there is no date to correct and no invariant to restore. The repair
is a **statement of what actually happened instead**, which is what the fix
wrote: the witness selects the component and nothing else, and here is where the
marcher gets its own seed.

*Why this one hid.* All four of the unit's instruments passed over it, and it sat
inside the 102-hit bucket the lane had named as a population rather than read —
which is now row **C21**, and is the argument for that row being worth minting:
**a population that has already hidden one live member is not a backlog, it is an
unread result.**

### "A measurement is a measurement of a tree" has a second failure mode

The rule has fired four times today on **stale** numbers — a table measured on a
tree that no longer exists. #775's round produced both ends of it at once, and
C-i's framing is the one to keep:

> the reviewer's S4 measured a tree that had moved, and my S1 reasoned about a
> line number without saying which tree it belonged to. Both are *"a measurement
> is a measurement of a tree"* — from opposite ends.

The second is the **unlabelled** case: the number may be perfectly correct and is
still unusable, because no reader can tell which tree it describes. It produced a
worse artefact than staleness here — the lane performed C-R11's check on a
citation, compared it against a tree it had not named, and **reported a defect
that does not exist**. `:193` is `},` on every tree; the claim starts at `:194`,
exactly where H15 cited it.

*The repair is cheaper than re-measuring:* **name the tree each table is numbered
against**, and name a merge commit as a merge commit rather than as a base. And
the lesson underneath, which is C-R11 sharpened:

> **C-R11 asks you to check a citation, not to find something wrong with one.** A
> check that reports a defect it did not find is worse than an unchecked
> citation, because the next reader stops checking.

*And one nuance from the same round, in the other direction:* S10 flagged
`face_normal.rs`'s *"five consumers"* as wrong. It is **right** — it counts
lanes, and three of the four remaining reach the door through
`reduce::face_plane`, so two direct call sites do not falsify four. The defect
was that it never said **what it counted**. **A reviewer flagging an unclear
claim is not the same as the claim being false**, and rewriting a true number to
satisfy a review would have been the worse outcome.

### §S39's own named instrument was unavailable to every lane, and one command restored it

C-i reported, holding back rather than claiming: it could not settle whether
`split_other_at_point`'s retired *"post-gate"* precondition was a sentence that
rotted or a precondition **silently widened out from under the helper** —
because `git log -S` *"bottoms out at the graft commit `541d9d83` in this shallow
clone for both sides."* And it named the consequence exactly: **§S39's method
note says `git log -S` is the instrument; in this clone it is not one.**

Checked, and it was true of every lane this session. The container's checkout is
shallow — **1,006 commits reachable, 15 graft points**. `git fetch --unshallow`
takes seconds and yields **4,539**. Run against a fresh clone, `git log -S` then
answers C-i's question on the first try, returning the two commits that touch the
sentence and the gate.

**Fixed rather than recorded.** The orchestrator checkout is unshallowed, so
every lane cloned from it now inherits full history; the four live lane clones
were unshallowed in place; and `new-lane.sh` re-asserts it, so a re-shallowed
source cannot quietly take it away again.

*Generalisable, and it is a shape this track has not had before.* Every other
instrument failure here has been **a lane's instrument being too narrow**. This
is **the repository's own ratified instrument being absent from the environment
the work happens in** — §S39 names `git log -S` as the way to tell benign rot
from a latent defect, and no lane could run it. The finding was reachable only
because C-i disclosed a question it could not answer *and said why*, rather than
picking the likelier of the two readings.

**What that costs, retroactively:** every S39-class classification made this
session was made without the instrument the finding names. None is thereby
wrong — the lanes classified by reading code, which is what they had — but
**"benign rot" and "a precondition widened out from under a caller" were
distinguished by argument where a command was supposed to decide it.** Worth
re-checking if a classification is ever load-bearing.

### The orchestrator overstated #636, and the lane corrected it one notch

Relaying C-e's derivation I wrote that **"#636's index was already broken on a
case #636 itself named."** C-e pushed back, and its weaker version is the true
one:

> **#636 never ran its index on a helix** — it named the helices as out of reach
> and declined them. So no #636 row was ever wrong.

What is wider than #636 stated is the **reason**, in two places:

1. **Orientability fails everywhere on a whole turn, not only at the ends.** The
   level-plane normal is the path tangent `T(a) ∝ (−R sin a, R cos a, k)`, and
   the whole-turn stacking chord `c(2π) − c(0) = (0, 0, 2πk)` is purely axial —
   so the *model* gives `cos = k/√(R²+k²) = 0.0635` **independent of `a`**,
   measured `0.06351710028158192` at level 0. **Corrected below: that is the
   right model and a sentence wider than its evidence** — the shipped body's
   level planes are the Newell fit of the placed ring, not the exact tangent,
   so the measured cosine runs `0.057–0.129` across the 513 levels and
   *exceeds* the `0.1` guard at some of them. The conclusion is unharmed (the
   guard is load-bearing everywhere, and by more than the model says); the
   quantifier was not measured. The half turn differs
   only because its chord is *not* axial (`c(π) − c(0) = (−2R, 0, πk)`), giving
   `≈ 0.0063` analytically against `0.011081` measured — and **`0.011` is #636's
   own figure to five digits**, which is what corroborates that both units
   measured the same object.
2. **Monotonicity fails on the half turn too**, which #636's note did not reach
   at all: `dh/da ∝ −R² cos a − k²` turns positive at `a ≈ π/2`, well inside a
   half turn, and mutation O2 shows it in execution at step 273/512. #636 gave
   orientability as its reason, and orientability is the **weaker** of the two
   failures.

*So the correction is not cosmetic:* "#636's index was broken on a case it named"
and "#636 declined a case for a reason narrower than the real one" are different
claims about another unit's competence, and only the second is supported. **I
made the error the track exists to catch, in a message instructing a lane to
expect its claims to be re-derived.** The lane re-derived mine.

*And it named where its own two numbers legitimately differ*, unprompted: the
derivation uses the exact tangent, the measurement uses the Newell fit of the
placed ring, and the gap between `0.0063` and `0.011` **is** that difference. A
lane that says which of its numbers is analytic and which is measured has already
answered the question a reviewer would spend an hour on.

**A self-correction in the same report, worth keeping:** C-e's first report
listed `klein.rs`'s U-turn as a fourth uncovered demo body. It is **not one** —
that `sweep_body` call is a documented refusal wall (`LoftError::ReversedStacking`),
so it builds nothing and there is nothing there to orient. Row C20 says so
explicitly, *so a taker does not go looking for a body that does not exist* —
the extreme case is closed by refusal rather than by a bit. Found while writing
the row, and stated rather than silently patched.

### The orchestrator propagated a lane's number instead of checking it

C-e's derivation ended in a sentence I liked: `cos = k/√(R²+k²)` is
**independent of `a`**, so the whole turn's stacking chord is unorientable
*at every level*, not just at the ends. I relayed it to Evan and wrote it into
this log. #779's adversarial lane measured it: over the 513 levels the cosine
runs **`0.057–0.129`**, and at some levels it is **above the `0.1` guard** the
sentence was explaining. The model is exact; the shipped body is not the model.
Its level planes come from the Newell fit of the placed ring, which deviates
from the exact tangent by up to ~3.8°, and 3.8° is worth a factor of two here
because the quantity is a near-zero cosine.

**Three things are worth separating.**

- *The conclusion survived.* The guard is needed at every level — by more than
  the derivation claimed, since some levels breach it outright. A wrong sentence
  can sit on top of a right conclusion, which is exactly why "does the fix hold?"
  does not answer "is the sentence true?".
- *The measurement was taken, and taken at one level.* `0.06351710028158192` is
  a real number from a real run. What made it wrong was the word **every**,
  which no run produced. A quantifier is a claim, and it needs its own evidence;
  a spot measurement that agrees with a derivation corroborates the derivation
  **at that spot** and nowhere else.
- *I was the amplifier.* The lane derived it once. I repeated it to Evan and
  committed it, and each repetition made it look better-attested than the single
  level it rested on. **An orchestrator relaying a lane's number is the last
  place it can still be cheap to check** — after that it is in the record and in
  a human's head.

*And it did not stop at the log.* The same sentence shipped in
`crates/sweep/tests/common/orient.rs`, where it would have outlived every
document in this scan — which is the pattern §S39 is about, seeded live. The
fix pass corrects the source file, not only the PR body.

**Rule taken:** when a lane hands over an analytic result with a measurement
attached, ask **at how many points** before relaying it. If the answer is one
and the claim says *every*, the claim is the derivation's, not the
measurement's — relay it that way or make the lane widen the run.

### A guarantee's own rustdoc claimed it enforced itself, and it does not

C-q, extending `flagged_census.rs` under D32, measured the census's matcher: it
greps the literal `k_stats::decide_flagged(`, so a site spelled **bare** after a
`use` is invisible to it. Three such sites exist today — `topo/tests/common/mod.rs`,
`geom/tests/curves/review_m5_pr4_adversarial.rs`, `demos/tour/src/booleans.rs` —
none under `crates/*/src`, so **the shipped count is honest today**.

That is where a lane would normally stop, and it is one clause short. The
shipped rustdoc at `decide_flagged` says:

> *"The census count assertion … pins the shipped-site count to the ledger's
> inventory, so adding a site without updating both the ledger and the assertion
> fails the suite — **the rule enforces itself**."*

**That sentence is false for one spelling**, and the sentence is what a reader
consults instead of reading the matcher. So this is not a fragility note about a
test; it is a **false guarantee shipping in rustdoc** — §S39's class, found
inside the machinery a lane was sent to extend rather than by looking for it.

*Three things generalise.*

- **"Honest today" and "self-enforcing" are different claims, and the gap
  between them is invisible from the doc.** The count is right; the mechanism
  that is supposed to keep it right has a hole. A reader who trusts the sentence
  will add a bare-spelled site and get a green suite.
- **The remedy is not "note the limitation".** Ruled: either widen the pattern
  to match every spelling, or correct the sentence to say which spellings
  self-enforce — and the limitation goes **at the pattern**, not in a report.
  §C15 says a sweep's result is worth nothing without a statement of what its
  pattern cannot match; the corollary earned here is that **the statement has to
  live where the pattern lives**, because a limitation recorded in a report dies
  with the report. This track has now paid for that lesson three times in one
  session.
- **A lane sent to extend a guarantee is the best-placed instrument for
  falsifying it**, and worse-placed for noticing it should. C-q found this while
  reading the matcher to reuse it. Nobody auditing rustdoc for false sentences
  would have gone looking in a test's grep pattern.

### A true model is the best camouflage an unmeasured quantifier can have

The #779 correction is worth one more paragraph than it already has, because
C-e's own account of how it happened is the generalisable part:

> O1's table measured level 0 on three fixtures, found the same number twice,
> and I generalised it to "every level" without measuring a second level. **The
> model happened to be right, which is what made it survive re-reading.**

That is the mechanism. A claim backed by a *correct derivation* passes exactly
the check a careful reader applies — "does the maths say this?" — and the maths
did. What the maths did not say is that **the shipped object is the model**. The
level planes come from a Newell fit of the placed ring, deviating up to ~3.8°
from the exact tangent, and 3.8° is worth a factor of two when the quantity is a
near-zero cosine.

**The measured table is now in `orient.rs`, and it pays for itself immediately:**

| fixture | level 0 | min | max | levels above the `0.1` guard |
|---|---|---|---|---|
| half turn | 0.011081 | 0.011081 | **0.993462** | **486 / 513** |
| whole turn | 0.063517 | 0.057463 | 0.129178 | 18 / 513 |
| two turns | 0.063517 | 0.057492 | 0.129178 | 18 / 513 |

Two things nobody had before it. **The half turn is *mostly* orientable** and
fails only near its ends — which is exactly what #636's note claimed for it, so
#636 was right about the half turn and merely gave the weaker of its two
reasons. And **the conclusion rests on the minimum**, not on a typical value:
the fixed-chord index needs *every* level orientable and refuses on the first
that is not, so `0.0575` and `0.0111` are the load-bearing numbers and the row
asserts on the minimum. The assertion had been right all along; the prose around
it was not.

*Rule, stated where a lane will read it:* **a quantifier is a claim and needs its
own evidence.** A spot measurement agreeing with a derivation corroborates the
derivation *at that spot*. When the two agree, that is the moment to measure a
second point, not the moment to stop.

### A capability the lanes do not have, discovered by a lane working around it

#779 hit a red `render lanes / freecad montage` reporting a montage cell **not
produced** — which that job itself classifies as a failed or partial pass rather
than a re-baseline. C-e established it was not the diff's: the job's own
classification, a diff touching only `crates/sweep/tests/` and docs, an empty
`git diff` over `demos/`, and the lane green on `main`. That is the legitimate
*not this PR's* case, established rather than assumed, and exactly the check the
drive-to-green rules ask for.

**Then it could not act on it: re-running a failed job returns 403 for a lane.**
It pushed an **empty commit** to trigger a fresh run, which came back green.

*Two things to keep.* The workaround was the only lever available and it worked,
so this is not a reprimand — but an empty commit is a permanent record of no
work in a repo whose stated convention is that **commits are the record of
actual work done**, and it is the one CI-kicking move the standing rules name as
forbidden. The real finding is upstream of the choice: **the orchestrator can
trigger workflow runs and a lane cannot, and nothing had written that down**, so
the lane could not know that handing the re-run up was available. Now told, and
recorded here rather than only in one lane's message.

*And the lane's own flag is the useful half*: if that scene is intermittently
failing to produce a cell on the runner, it will hit the next lane too. Three are
live.

### A register that has not landed is not a register — the fourth instance, and it is mine again

**2026-08-20, late.** I cited **C-R22** by number in a reviewer's brief and told
the reviewer to read it in `docs/SMELL-C-LOG.md`. It is not on `main`. Nor are
C-R23, C-R24, or **thirty-three commits** of this session's rulings, incidents
and corrections — everything since #754.

*This is the same lesson for the fourth time in one session*, and the pattern of
the four is the point:

1. **#745** — eight rulings committed and pushed to the orchestrator branch,
   cited by number in briefs, **never merged**. Caught by a lane applying C-R11.
2. **The standing lane header** — the section *describing* it committed, the
   header itself not. Caught by two lanes independently.
3. **The `0.0635` sentence** — corrected in the PR body, still shipping in
   `orient.rs`. Caught by an adversarial lane.
4. **This** — rulings pushed to a branch, cited in a brief, not on `main`.

Every one is *"the thing was written"* mistaken for *"the thing is where it is
read from"*, and I have now recorded the lesson three times **while committing
it a fourth**. Recording a lesson is not the same act as installing it.

**What actually saved this instance is luck, and worth naming:** the lane helper
clones from the orchestrator's working copy rather than from the remote, so
review lanes inherit whatever branch that copy is on and *would* have found
C-R22. It happened to be checked out on the branch that has it. Had I been on
`main`, the reviewer would have read a brief citing a ruling that does not exist
— which is C-R11's exact subject, from the other side.

**The mechanical fix**, which is what the previous three write-ups all lacked:
the orchestrator branch merges to `main` **at every ruling**, not at every
session. A ruling is dispatch-visible the moment it is cited, so the merge has to
precede the citation, not follow the batch.

### C-R21 is a rule about reliance, and a lane read it as a rule about truth

#803 re-derived §D's numbering paragraph — correctly, it is the orchestrator's
paragraph and a landing leaves it — and wrote:

> **C20** has never been assigned on `main` at all, **so it is a gap and not a
> reservation.**

The premise is true and the inference is false. **C20 is assigned**, to #779,
which was unmerged when that sentence was written. C-R21 says *a number is not
owned until it is on `main`* — that tells a lane what it may not **rely** on, not
what is **true**. A hole on `main` is perfectly consistent with an unmerged
reservation, and here it was one.

*The generalisation is worth more than the correction.* This track has now
written several rules of the form *"do not trust X"*, and every one of them is
about **what a reader may act on**. None of them licenses the converse — that
what cannot be relied on is therefore absent. **A rule that bounds your
knowledge is not a source of knowledge**, and the tell is a sentence that turns
an epistemic limit into an ontological claim with a *"so"*.

Corrected in §D, and the reading rule is left standing with its real reason: a
taker asks the orchestrator for a number **not because the hole is unreserved,
but because from `main` alone you cannot tell** — which is exactly why the
asking exists.

### Reviewers age out of their own tree, twice now — and it is not their error

Both style reviews today reported a finding that was **already false when they
reported it**, because the lane pushed while they were reading:

- **#775's S4** — *"§D row C19 is not written"*. It was, on the head, pushed
  fourteen minutes before the report.
- **#779's S2** — *"the assigned §D row `C20` is not written on this head"*, and
  half of its S13 (`klein.rs` mis-described). Both were fixed on the head twenty
  minutes before the report.

Each time I verified before relaying and dropped the stale finding, which cost
one command. **Relaying it would have cost a lane a round arguing with a
correction it had already made** — and worse, would have taught it that the
review is not worth reading closely.

*This is the same rule that has fired six times today, arriving from the third
direction:* **a measurement is a measurement of a tree**, and a **review is a
measurement**. Stale numbers, unlabelled numbers, and now stale *findings*.

It is not the reviewers' error. A review takes 20–40 minutes; lanes push fix
passes and row numbers inside that window; nothing tells a reviewer the ground
moved. The structural cause is the same one as §D's conflict window — **the tree
now changes faster than a careful read of it takes.**

**Operationally, and cheap:**

- **A reviewer records the head SHA it reviewed**, at the top of its report. Both
  of these would then have been self-evident rather than needing an orchestrator
  check.
- **Any finding of the form "X is missing / not written / not done" is
  re-checked against the current head immediately before reporting.** Only
  absence claims need it — a finding about what the code *says* stays true of the
  tree it was read on, but a finding about what is *absent* is exactly the one a
  push invalidates.
- **The orchestrator verifies absence findings before relaying**, which is what
  happened both times and should be the rule rather than the reflex.
