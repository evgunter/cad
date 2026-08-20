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
| **C-R4** | **S29 is not blocked on a design conversation.** §D routes it to `docs/TESS-SPLIT-SPEC.md` / PR #568. | **Correction, and it stands.** Checked by #684's reviewer: both #568 and TESS-SPLIT-SPEC are scoped entirely to the NURBS per-cell schedule in `nurbs_cert`. **No open conversation covers `curved::grid_steps`**, so S29's analytic-chart half was never waiting on a venue — it does not have one. §D's C3 row is wrong on this point and is corrected when C-j lands. | the parallel orchestrator, 2026-08-20 |
| **C-R11** | **The roster's own scope citations are stale, and a lane checks them rather than complying.** `C-h`'s `splitting/rules.rs:268` lands on `face_extent`, a lever-arm helper, not on anything the `bridged` skip reads. | **Every scope cell is a claim site.** A lane that finds a citation unresolvable says so in its report and states what the line actually contains; it does not silently work around it or silently obey it. This is the same failure as the brief that fenced a lane against a memory deleted from main two days earlier — recorded there as an incident, promoted here to a standing rule, because the roster is now the third document on this track to carry a citation that outlived what it described. | orchestrator, 2026-08-20 |
| **C-R12** | **How many lanes get an adversarial review.** The inherited roster marked 7 of the 12 remaining lanes adversarial, which is more than *"a minority of the track"* claims. | **Retrimmed to five: C-e, C-g, C-h, C-k, C-m** — plus **C-q**, which is new. The criterion Evan gave is *"complex enough that there's a significant chance the change will introduce a regression not caught by CI"*, and it is a narrower test than "this code is load-bearing". **C-j drops to style-only** because its mechanical half is a re-spelling whose correctness is provable by byte-identity of mesh output — a wrong answer is visible without an adversary, and the lane owes that proof. **C-l drops provisionally**, because a lane-trait collapse is type-level and the compiler is the adversary; the sub-lane that *rewrites* `Dual` arithmetic rather than re-spelling it is promoted back. | Evan (criterion) + orchestrator (application), 2026-08-20 |

---

## Lane roster

Gates are the *live* ones as of 2026-08-20; §D's own edge list is
superseded for Track C by this table.

| lane | finding | scope | gate | review |
|---|---|---|---|---|
| **C-d** | **H12** — the SSI sweeps' other never-silence doors have no acceptance row | `geom-brep/tests/` | **discharged** (#692 merged) | style |
| **C-e** | **H13** — `sweep_body`'s helix rows have no orientation coverage | `sweep/tests/{m8_14_long_turn_sweep,m7_skin_integral}.rs`, `step-export/tests/common/mod.rs` | none | **adversarial** + style — #636's level-plane oracle trips its own precondition here (`cos ≈ 0.011`), so this needs a *new* oracle, and the oracle carries the soundness |
| **C-f** | **H11** — #632's two residues | `editor-core/src/{resolve/,select.rs,refactor.rs}` | none | style |
| **C-g** | **S32** — `Surface`'s one-partial-per-call API and the shadow SSI enum | `geom/` (the merged crate), `geom-brep/src/ssi/system.rs` | **discharged** (#705, #692 merged) | **adversarial** + style |
| **C-h** | **H14** — the census's `bridged` skip | `topo/src/census.rs`, `splitting/rules.rs:268` (the line citation does **not** resolve to the skip on today's main — see C-R11) | **discharged** (#690 merged) | **adversarial** + style — a live soundness hole of S49's exact shape |
| **C-i** | **H15** — #635's unclassified siblings | `mesh/src/planar.rs:63`, `topo/src/validate.rs:426`, `topo/src/splitting/mod.rs:194` | **discharged** (#690 merged) | style |
| **C-j** | **S29** — the sizing vocabulary across five modules | `mesh/src/{nurbs_cert,curved,chords,trimmed,budget}.rs` | **discharged** (#684 merged) | **style only** on the mechanical half — retrimmed, see C-R12; the policy half is a design PR (**C-R2**) |
| **C-k** | **S28's duplication half** — three tessellation lanes, three pipelines | `mesh/` | **discharged** (#684 merged) | **adversarial** + style |
| **C-l** | **C7 + S33** — the lane-trait collapse, `RingInterval`, the scalar ladders | `geom-core/`, and W2b's 535 refs across 15 files | **discharged** (#682 merged) | **style only, provisionally** — see C-R12; expect to split into 2–3 lanes, and the sub-lane that rewrites `Dual` arithmetic rather than re-spelling it gets promoted to adversarial |
| **C-m** | **S27** — `props/quad.rs`'s four quadrature engines | `geom-brep/src/props/quad.rs` | **STILL GATED** — A2 / #649, open as **#714** | **adversarial** + style |
| **C-n** | **H17** — the rustdoc spec-code remainder, ~1115 lines / 130 files | per crate: `topo` 300, `editor-core` 267, `geom-brep` 192, `geom-core` 107, `sweep` 64, rest < 70 | **deliberately last** — it touches 130 files and would conflict with every open lane | style, per crate batch |
| **C-o** | **H16** — the STL header is not caller-settable | `stl/` | none | style; design PR, waits for sign-off (**C-R1**) |
| **C-p** | **C9** — the `tess-meter` CSV's `agreement` column measures nothing | `tools/tess-meter/`, `tools/tess-lint/`, the committed baseline, `docs/TESS-BUDGET.md` | none | style — and the lane must first decide whether a realisation check is worth having at all, or whether the honest fix is to delete the column |
| **C-q** | **C10** — `geom_core::k_stats`, S30's class one crate over | `geom-core/src/k_stats.rs`, and `profile::k_stats`'s shim (S40) | none | **adversarial** + style — the recording sits *inside* three load-bearing kernel predicate doors, so #709's split does not transfer mechanically |

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

- **D7 is gated on #702** (this track's C-a) — its `PairSolve` deletion waits on
  `mate.rs`, `mate/solve.rs` and the `lib.rs` re-export block C-a is editing.
  Landing #702 promptly is therefore a Track D unblock, not only a Track C one.
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

| lane | finding | branch | review |
|---|---|---|---|
| **C-d** | H12 — the SSI sweeps' other never-silence doors | `smellc/h12-ssi-never-silence` | style |
| **C-f** | H11 — #632's two residues | `smellc/h11-632-residues` | style |
| **C-h** | H14 — the census's `bridged` skip | `smellc/h14-census-bridged` | **adversarial** + style |
