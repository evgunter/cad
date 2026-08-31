# SMELL-SCAN Track T — orchestrator log

**Constituted 2026-08-31, by the S-BLEND orchestrator.** Track T is
`crates/sweep/` (both `src/` and, by exception to W's fence, the
`sweep/tests/` files its own rows name), claimed whole by S-BLEND at
VERBS-SHELLFIX 2b's merge per the ratified partition
(`docs/WORK-STREAMS-2026-08.md`; row schedule in §D of
`docs/SMELL-SCAN-2026-08.md`, Track T table). This file is the
execution record — rulings, lane state, review outcomes, incidents.
Live status is here and in §D, never in `memories/`.

**Branch prefix:** `smellt/` for units; the orchestrator sits on
S-BLEND's own session branches, and cross-references to program
state live in `docs/S-BLEND-LOG.md`.

**This track runs OUTSIDE the model A/B experiment**, following the
F/G/I precedent the S-BLEND plan's "SMELL conventions" phrase names:
no pairing, no ordinal, no row in `docs/MODEL-AB-LOG.md`; nothing on
this track reads or edits that file. Stated honestly: the I-log's
recorded REASON for the exclusion (the experiment pause of
2026-08-21) has lapsed — the experiment is live again — so this is a
precedent-following ruling, not a forced one, and Evan can reverse
it for later lanes if style work should be instrumented.

## Review policy (the F/G/I shape)

- **Style review on every unit** — `docs/prompts/reviewer-style-lane.md`
  dispatched BY PATH, with the per-lane emphasis a dispatch owes
  (`docs/REVIEW-STYLE-DISPATCH.md`), plus the two standing track
  questions: (1) is the row's original problem COMPLETELY gone — not
  narrowed, not relocated; (2) was it closed the best available way,
  or merely a way that compiles.
- **Adversarial review only where the change carries meaningful
  risk** (Evan's C-R12 criterion: complex enough that there is a
  significant chance of a regression CI will not catch).

## Rulings

| # | Question | Ruling | Who, when |
|---|---|---|---|
| **T-R1** | Serialization vs the S-BLEND implementation slate: BLEND-6 (and later BLEND-3/-4) edit `crates/sweep/src/fillet/`, and BLEND-6's ratified V3 renames the whole module path. | **`fillet/`-touching rows are KEEP-OUT while a BLEND implementation lane is live**: D90 (`fillet/build.rs` + `fillet/surgery.rs`) and D321 (`fillet/admit.rs`) wait, and D321 additionally waits for the V3 rename so its test-utils conversion lands against the final path. Non-fillet rows run in parallel with the BLEND lanes — the partition's own premise (different files). | orchestrator, 2026-08-31 |
| **T-R2** | D91 spans this track and Track W (`LoftError::SeamStructure`'s shape change reaches `editor-core/tests/lib_doors_node_result.rs`). BLEND-6 is simultaneously reworking the kernel-door refusal surface and will plausibly touch the same door-test file. | **D91 DEFERRED until BLEND-6 merges** — the collision risk is in exactly the file the fence exception names. Not staffed into T-a. | orchestrator, 2026-08-31 |
| **T-R3** | C-e/H13 carries §D's own instruction: "Verify against #779 before staffing." | Verification dispatched (read-only) 2026-08-31, before T-a's brief was cut; T-a takes the row ONLY if the verdict is OPEN, and otherwise records the verified-closed evidence here and in §D. **VERDICT: CLOSED** (2026-08-31) — the coverage is on `main`, landed by **#779** (merge `db241875`): the containment oracle at `sweep/tests/common/orient.rs`, three helix orientation rows in the long-turn sweep suite (the `min_roll_turn` anti-vacuity floor, the not-orientable-against-the-stacking-chord guard, then walls and caps against the continuity index including the face `sense` bit), and the rational circle-section elbow row in the skin-integrality suite — all aggregated into the default target with no ignore and no `cfg` gate. **The row was not staffed**; it is retired VERIFIED-CLOSED on the §D ledger in T-a's PR, with its own meta-claim corrected: the two contradicting statements were NOT both "in this document" — Track C's FIXED record is in `docs/SMELL-C-LOG.md`, so the contradiction was cross-document, which is why it survived two partitions. | orchestrator, 2026-08-31 |
| **T-R5** | T-a review mode. | **T-a review mode: STYLE-ONLY per C-R12** (test rows + retirements; a row that reds is visible). Ruled at dispatch 2026-08-31, recorded here at close. | orchestrator, 2026-08-31 |
| **T-R4** | D320 | Filed-not-takeable ahead of D240, per the row itself. Nothing to decide; recorded so the track's ledger is complete. | orchestrator, 2026-08-31 |

## Lane state

| lane | rows | state |
|---|---|---|
| **T-a** | C20 (turning-path orientation pins), D104 (the two hand-run diff artefacts) | **REVIEWED (style, per T-R5)** — 7 findings, **none correctness**; the lane's mutation table was reproduced 5/5 by the reviewer's own execution, both retirement arguments judged correct, the C20 closure judged honestly scoped. Fix pass taken on `smellt/a`: `main` merged (carrying the #1330 doc-gate fix that was the CI red), three dangling `S110` citations re-aimed and the ledger's own deletion-sweep rule written down, the §D retirement footnote deleted per the delete-don't-annotate rule, the lily-spine exclusion re-argued as the PLANAR-arc class, the unreachable outer floor assert removed, the "must all fit" wording qualified to what it can catch, `C25`'s row extended to schedule the frame-recipe twin, and this log's mutation sentence corrected. Plus the orchestrator's `S390` adjudication (TAKE, by message rather than as a numbered ruling): `S390` discharged in fence by a `# Correspondence` paragraph at both public doors. **PR #1329 open, not merged.** |
| (unstaffed) | D124 (re-home the struck-lane findings), C25 (the six-times-built swept body — cross-crate homing, fence note owed at dispatch), D96 (ten `unreachable!` arms — file-set to be enumerated before staffing to check the fillet overlap) | queued |
| (kept out per T-R1/T-R2) | D90, D321, D91 | wait on BLEND-6 (D321 also on V3) |
| (not takeable) | D320 | waits on D240 |

## Lane records

### T-a — C20, D104 (branch `smellt/a`)

**C20 — turning-path orientation pins.** The corpus pinned two turning
families and no others: the quarter-turn arc elbow (integral, and
rational one suite over) and the constant-pitch helix at ½, 1 and 2
turns. The complement it enumerated and closed is three shapes — a
lofted chart carrying an AUTHORED roll, a swept path that reverses its
curvature at an inflection, and a swept path with nonzero torsion —
each pinned walls-and-caps against `common::orient`'s containment
oracles, each with an anti-vacuity condition on the shape and a
HANDEDNESS pin on positions. Red-capability executed, not argued: a
production sense flip in the loft assembly reddens all three on the
material-side assertion, and mirroring each fixture reddens it on the
positions half while leaving containment green — but *which* assertion
in that half fires differs and the summary owes the distinction: the
mirrored S duct and the mirrored cubic red on their HANDEDNESS pins,
while the mirrored roll fixture reds one assertion EARLIER, on the
anti-vacuity turn bar, because mirroring the authored angle also moves
which vertex is lex-min and the body comes back barely rolled at all.
The roll row's hand pin was shown red-capable on its own by a separate
mutation that swaps the two sections — same roll magnitude, opposite
hand — which reaches the hand assertion. Five mutations, five reds,
reproduced by the style reviewer.

**D104 — the two hand-run diff artefacts.** Both retired rather than
promoted, with the reason recorded in the tree: a `Debug`-string hash
printed for a comparison that happened once cannot become an assertion
(a `DefaultHasher` digest is not stable across toolchains and a
whole-body `Debug` dump is not a claim about geometry), and the
consumer differential's printed digest was licensing a pinned seed for
a cross-build comparison nobody schedules — so the digest went and the
seeds became a real search.

**Incident — the doc gate is red for every sweep-only PR, and not
because of this lane.** T-a's run 3802 came back 20 of 21 jobs green,
the failure being `rustdoc (gate)` on `crates/viewer/src/theme.rs`'s two
app-feature intra-doc links. They are byte-identical on `main` and this
lane opens no viewer file: what the lane did was DRAW the path that
exposes them — the gate documents the viewer at DEFAULT features
whenever the change filter says the viewer is not in the closure, which
a sweep-only diff always is, while a viewer-touching PR takes the other
path and resolves the links. So `main` is green over a break that reds
this whole track. Filed as **#1330**, not fixed: `crates/viewer/` is
outside the fence and the fix is a judgement about that module's prose.
**Every Track T lane should expect this red until #1330 lands**, and
should check the failing job is that one before believing it.

**Second incident — `main` is red at the interval / eps=1e-12 point,
and Track T lanes will keep drawing it.** T-a's post-merge run 3821
drew `interval, eps = 1e-12` — a point its earlier runs never drew —
and both shards failed. The lane's own rows are green there; the whole
`-p sweep` suite at that point has three failures
(`m5_s12_curved_ops_interval`, `review_arceval_r1_probes` and
`m5_s13_pips_interval`, all `certified::` rows), and **all three fail
identically on a clean detached `origin/main`**. One root cause: a
measured constant `1.1414768974413613e-12` pinned in two files while
the value is now `1.1362773333939659e-12` — *"the arc chain moved;
re-measure and re-state"* — with the third row a consequence of the
same narrowing. Filed as **#1338**, the same class as the closed #921.
Not fixed: the rows are outside this lane's two and the fix is a
re-measurement someone must adjudicate, since a moved baseline is
evidence rather than a number to restore. **Two of these three rows are
in `crates/sweep/`, so this is Track T ground** and wants a row if it
outlives #1338.

**What the lane found and did not fix.** The `twisted_lofted` fixture's
`theta` is not the body's roll: validation re-anchors each loop to its
lex-min vertex and the loft pairs CANONICAL loops by index, so for
`theta` in `(0, pi/2)` the body rolls by `theta - pi/2`. Both halves are
documented kernel behaviour (`loft_geometry`'s "correspondence is BY
INDEX … the canonical loops are what get skinned", and the profile
crate's canonical-start rule), so this is not a logic defect and takes
no issue; the fixture's doc said otherwise and is corrected, and the
composition is unstated at the `loft_body` door itself. Filed as ledger finding
**`S390`** at first push, then **discharged inside the same PR** on the
orchestrator's adjudication of the reviewer's pushback: the finding's
own "what is missing" sentence named a doc gap at the door a caller
reads, the door is `crates/sweep/src/loft.rs` and therefore in fence,
and the fix is one paragraph. `loft_body` now carries a
`# Correspondence` section stating that loops are paired canonically by
index, that a rotated section can be re-anchored, and that the built
body's roll is the angle between canonical loops rather than the
authored one — with this lane's own `theta - pi/2` fixture as the
worked example; `sweep_body` carries the short form (its sections are
one profile repeated, so the pairing is the identity and only the wall
order is decided). **`S390` is therefore not in the ledger**: a finding
whose whole content is discharged by the PR that raised it would be a
note saying work completed, which §D deletes. The number is spent, not
reusable. What is deliberately NOT carried forward is the expensive
alternative the finding mentioned — a door that takes the
correspondence explicitly — because it was named as an option, not as a
defect; anyone who wants it is opening a design question for Evan, not
re-raising this one.
