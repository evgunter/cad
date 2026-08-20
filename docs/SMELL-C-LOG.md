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

---

## Lane roster

Gates are the *live* ones as of 2026-08-20; §D's own edge list is
superseded for Track C by this table.

| lane | finding | scope | gate | review |
|---|---|---|---|---|
| **C-a** | **S24** — the assembly gate's success path is documented unreachable | `editor-core/src/{assembly,mate}.rs` | none | style |
| **C-b** | **S30** — ~1,050 lines of instrument in the mesh hot loop | `mesh/src/{budget,probe_stats,trimmed,chords}.rs`, the feature matrix, the CI row, `memories/telemetry-gating.md` | none (disjoint from #684's `curved.rs`) | **adversarial** + style |
| **C-c** | **S31** — the crate split that buys nothing | `geom-curves/`, `geom-surfaces/`, new `geom/`, 11 dependents, `step-export/src/writer.rs` | none | **adversarial** + style |
| **C-d** | **H12** — the SSI sweeps' other never-silence doors have no acceptance row | `geom-brep/tests/` | #692 | style |
| **C-e** | **H13** — `sweep_body`'s helix rows have no orientation coverage | `sweep/tests/{m8_14_long_turn_sweep,m7_skin_integral}.rs`, `step-export/tests/common/mod.rs` | none | **adversarial** + style — #636's level-plane oracle trips its own precondition here (`cos ≈ 0.011`), so this needs a *new* oracle, and the oracle carries the soundness |
| **C-f** | **H11** — #632's two residues | `editor-core/src/{resolve/,select.rs,refactor.rs}` | none | style |
| **C-g** | **S32** — `Surface`'s one-partial-per-call API and the shadow SSI enum | `geom-surfaces/` (→ `geom/`), `geom-brep/src/ssi/system.rs` | **C-c**, #692 | **adversarial** + style |
| **C-h** | **H14** — the census's `bridged` skip | `topo/src/census.rs`, `splitting/rules.rs:268` | #690 | **adversarial** + style — a live soundness hole of S49's exact shape |
| **C-i** | **H15** — #635's unclassified siblings | `mesh/src/planar.rs:63`, `topo/src/validate.rs:426`, `topo/src/splitting/mod.rs:194` | #690 | style |
| **C-j** | **S29** — the sizing vocabulary across five modules | `mesh/src/{nurbs_cert,curved,chords,trimmed,budget}.rs` | #684 | **adversarial** + style on the mechanical half; the policy half is a design PR (**C-R2**) |
| **C-k** | **S28's duplication half** — three tessellation lanes, three pipelines | `mesh/` | #684 | **adversarial** + style |
| **C-l** | **C7 + S33** — the lane-trait collapse, `RingInterval`, the scalar ladders | `geom-core/`, and W2b's 535 refs across 15 files | **#682** | **adversarial** + style; expect to split into 2–3 lanes |
| **C-m** | **S27** — `props/quad.rs`'s four quadrature engines | `geom-brep/src/props/quad.rs` | A2 / S56 / **#649** | **adversarial** + style |
| **C-n** | **H17** — the rustdoc spec-code remainder, ~1115 lines / 130 files | per crate: `topo` 300, `editor-core` 267, `geom-brep` 192, `geom-core` 107, `sweep` 64, rest < 70 | **deliberately last** — it touches 130 files and would conflict with every open lane | style, per crate batch |
| **C-o** | **H16** — the STL header is not caller-settable | `stl/` | none | style; design PR, waits for sign-off (**C-R1**) |

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

---

## Landings

*(none yet — #702 is green and in style review)*
