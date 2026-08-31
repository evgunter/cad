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
| **T-R6** | `L5`'s rule half — `S177`'s strike rule (*a lane's row may only be struck once each rides-along it did not close has been re-homed or given its own row*) — is a **document-wide convention**, and T-b filed it as a scheduled item rather than writing it. The style reviewer flagged the same thing from the other side: a lane writing a ledger-wide rule is arguably ratification territory, which is Evan's. | **TAKE IT — write the rule into §D's *"How to read a row"* now, and leave only the WALK on `L5`.** The T-a precedent decides it: that lane landed the deletion-sweep rule in the same conventions section, in a lane PR, without ratification. The distinction that makes it a lane's to write is that this is **not new policy** — it states how §D's existing *live rows only* rule must be applied so it stops destroying what it was never meant to touch. A rule that exists only as a scheduled item does not stop the accrual it describes, and `L5`'s walk has no slot. **The reviewer's reservation is recorded rather than dismissed**, and is noted in the ledger text itself: if Evan reads the convention as policy, it is one paragraph to revert and the walk is unaffected. | orchestrator, 2026-08-31 (T-b review adjudication) |

## Lane state

| lane | rows | state |
|---|---|---|
| **T-a** | C20 (turning-path orientation pins), D104 (the two hand-run diff artefacts) | **REVIEWED (style, per T-R5)** — 7 findings, **none correctness**; the lane's mutation table was reproduced 5/5 by the reviewer's own execution, both retirement arguments judged correct, the C20 closure judged honestly scoped. Fix pass taken on `smellt/a`: `main` merged (carrying the #1330 doc-gate fix that was the CI red), three dangling `S110` citations re-aimed and the ledger's own deletion-sweep rule written down, the §D retirement footnote deleted per the delete-don't-annotate rule, the lily-spine exclusion re-argued as the PLANAR-arc class, the unreachable outer floor assert removed, the "must all fit" wording qualified to what it can catch, `C25`'s row extended to schedule the frame-recipe twin, and this log's mutation sentence corrected. Plus the orchestrator's `S390` adjudication (TAKE, by message rather than as a numbered ruling): `S390` discharged in fence by a `# Correspondence` paragraph at both public doors. **PR #1329 open, not merged.** |
| **T-b** | D124 (re-home the struck-lane findings), C25 (the six-times-built swept body + the frame-recipe twin), D91 (the swallowed `SplineError`) | **PR open, not merged.** All three closed; see the lane record below |
| (unstaffed) | D96 (ten `unreachable!` arms — file-set to be enumerated before staffing to check the `blend/` overlap), and the three rows D124 re-homed: **D322**, **D323**, **D324**, all inside `crates/sweep/src/blend/` and so held by T-R1's class | queued |
| (kept out per T-R1/T-R2) | D90, D321 (**D91 taken and closed by T-b** — T-R2's hold was spent, and the door-test collision it feared did not materialize) | **BLEND-6 merged `82a3a424` 2026-08-31**, which is the event all three waited on and which carries the V3 rename D321's conversion needed — so the T-R1/T-R2 hold is spent unless a later BLEND lane re-arms it. Whether `fillet/`-touching rows are takeable now is the orchestrator's call, not this row's; recorded here so the next dispatch starts from the fact rather than re-deriving it. Note the paths moved: the module is `sweep::blend`, so D90's and D321's own file citations read against the old spelling |
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

**The keep-out adjacency resolved itself, as predicted.** T-a's one
contact with the BLEND slate was that its `D104` retirement lived in a
`tests/` file which CALLS `fillet_edges`, so the import line was going
to move under BLEND-6's V3 rename whichever way the lane went. It did:
BLEND-6 merged at `82a3a424` (`sweep::fillet` -> `sweep::blend`,
`FilletError` -> `BlendError`), and merging `main` into `smellt/a`
auto-resolved with **no conflict** — the rename touched the import,
this lane touched the header and deleted the tail, and the two hunks
are disjoint. The merged file is main's import plus this lane's
retirement and nothing else, verified by diffing it against `main`. The
retirement made that file smaller rather than differently coupled,
which is what the PR claimed at the time and is now observed rather
than argued. Nothing else in the lane names `fillet` or `blend`: the
`turning_orientation` suite and `common/orient` ride the loft and sweep
doors, which V3 does not rename.

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

### T-b — D124, C25, D91 (branch `smellt/b`)

**D91 — the swallowed `SplineError`, closed as the row specified.**
`LoftError::SeamStructure` is now
`SeamStructure { source: SplineError }`, the site carries the payload
(`map_err(|source| …)`), and `Display` renders it after the existing
sentence — the shape `StackingEscalated { source: Indeterminate }`
already uses in this enum, so the arm was not invented. That makes
`geom_brep::nurbs_iso`'s own `# Errors` promise — *"surfaced rather
than swallowed"* — true for the first time. **The W-fence edit is one
roster entry**: `editor-core/tests/lib_doors_node_result.rs` constructs
the variant with a `ControlCountMismatch`, which is the invariant
`boundary_iso_u` can actually break. That suite asserts the arm's
render ENDS WITH its payload's render and contains no `{`, and both
hold — so the fence exception bought a real assertion, not a
recompile. Nothing else in the tree matches the variant:
`editor-core/src/eval/wire.rs`'s `LoftError` match names `Skin` and
`Profile` and forwards the rest.

**The class sweep says the fence is now clean.** `map_err(|_|` returns
**zero** hits in `crates/sweep/src/`; `D91`'s was the last. It is alive
one crate below and now rowed as `S394` (`geom-brep`'s
`pcurve_cache.rs` swallows `boundary_iso_u`/`_v` twice while
`nurbs_iso.rs` twelve hundred lines away spells the same conversion
payload-preserving). Not taken — two other tracks' files.

**C25 — the elbow homed in `sweep::test_support`, six copies to one.**
The six were `sweep/tests/{m7_skin_integral,m5_s11_concave_sense}.rs`,
`mesh/tests/{probe_review,m7_nurbs_trimmed}.rs`,
`step-export/tests/common/mod.rs` and
`step-export/examples/review_elbow_probe.rs`. Five now delegate; the
sixth **stays a copy on purpose and says so at the site** — it is the
review probe that asks whether an INDEPENDENT construction reaches the
committed STEP bytes, and pointing it at the shared home would make it
compare the fixture with itself.

**Why `test_support` and not `tests/common` or `test-utils`.**
`tests/common` is a test-target module, unreachable from another
crate; `test-utils` is a ZERO-dependency leaf by manifest comment and
could not name `Body` without inverting the layering. `test_support`
is the S52 home this class already has, and the only change it needed
was that `mesh` and `step-export` name the `test-support` feature in
their dev-deps. **Its header's claim that the feature is *"off for
every other build, including every downstream dependent"* was made
false by that and is corrected in the same diff** — the surviving
guarantee, that no non-test build of any dependent turns it on, is the
one that was doing the work.

**Bytes did not move**, which is the check that matters here:
`step-export`'s golden fixture suite is green, so the shared
constructor is bit-identical to the six it replaced.

**The frame-recipe twin, per the BLEND-2 precedent.** `demos/tour` is
outside the kernel workspace, reaches the kernel only through the
`pncad` façade, and its scenes are `src/` — so it can link neither
home. Its **two** copies (the narration cell and the twisted-cubic
cell) are folded into one tour-local helper whose doc states why it is
a copy, and the four now-stale citations naming the elbow's old homes
are re-aimed. Four hand-copies of one fixed recipe with no public door
is filed as `S393`, deliberately as a design question rather than a
defect.

**D124 — executed, and it is three rows plus a closure, not four
pointers.** Each member was re-derived against the tree BLEND-6 left,
which is not the tree `S177` read. **`S111(a)` is closed**: commit
`18fd8370` replaced its *"Likely dead in practice"* sentence with the
opposite one and the arm is a built path now; the self-declared-dead
vocabulary returns zero hits across `crates/sweep/src/`. **`S111(b)`,
`S111(d)` and `S112(a)` all stand** and are rowed as **`D322`**,
**`D323`**, **`D324`** with current citations — every one of them had
moved under the rename, and `S112(a)`'s had moved twice (the consumer
is `emit_blend` now, not `emit_fillet`).

**All three re-homes are inside `crates/sweep/src/blend/`, which is
this lane's keep-out.** That is the collision, and it is why the row's
own alternative — *"a correctly-routed ledger row with current file
citations"* — is the disposition rather than a fix pass. The files
were READ to verify, never edited; `git diff` touches nothing under
`blend/`. Both rows in `naming.rs` (`D323`, `D324`) are cheaper taken
together, and the table says so.

**One sharpening is worth the orchestrator's attention** because it is
`S177`'s own mechanism seen twice: `S112(a)`'s false paragraph is
cited BY the consuming site (`emit_blend.rs` says *"(module docs)"*) as
its authority. An untracked rides-along did not just go unfixed — it
left two files across a crate boundary each pointing at the other as
the source of a claim neither holds.

**What D124 did NOT sweep**, said plainly because the row could be read
as having done it: every OTHER track's struck rows. `D124` was written
as the sweep for that whole exposure and executed as the re-homing of
the four members `S177` had enumerated. The remainder is filed as
**`L5`** under §D's *Last, deliberately* — it audits this document's
history rather than any track's files, which is why it fits no fence
and had been riding inside a Track T row.

**`S392`, filed not taken.** `C25`'s scope is the elbow; the loft
prism is the same class and eleven copies wide across four crates and
the tour. `sweep::test_support` is now reachable cross-crate, so the
next lane pays a delegation rather than a new mechanism.

**T-b fix pass (style review: 7 findings, none correctness).** The
commissioned claims were verified by the reviewer's own execution —
golden bytes on three legs, all five constructors proven equivalent,
every citation live — so the pass is corrections, not rework. The
sharpest finding is worth keeping because it is a general trap:

**The D91 pin was vacuous and the roster row could not see it.** The
suite derived its expectation with `K::Loft(e) => e.to_string()` — the
same `Display` under test — so an arm that silently stopped rendering
its `source` would shorten both sides of the `ends_with` equally and
stay green. The pin asserted forwarding at ONE layer while reading as
if it asserted the payload. Fixed by a second row,
`a_nested_source_under_a_payload_arm_survives_into_the_message`, whose
oracle is built from the innermost payload's own type and never
through a wrapper. **Executed, not argued**: dropping `: {source}`
from `SeamStructure`'s arm reds the new row and leaves the old one
green (mutation A), and the same mutation on `StackingEscalated` reds
it too (mutation B) — two mutations, two reds, and the old row blind
to both. The suite's third source-carrying nesting, the `Split` →
`SplitFinishError` → `BandError` chain the roster already carried, was
equally vacuous and is covered by the same row.

**The other six.** `S392`'s count was wrong and the way it was wrong
is now written into the finding: the sweep behind *"eleven copies"*
was piped through `head -20` and the truncation was read as the
population, and `--include=*.rs` excluded a Python member — a claim
wearing a receipt's clothes, committed inside a finding about
duplication, which is `S131`'s warning exactly. Corrected to **18 live
constructions** across six crates, the tour and `tools/tess-meter`,
with the routing consequence stated (the population now crosses Track
J's Python fence and Track K's `tools/`). A stay-argument in
`m5_s11_concave_sense.rs` that said the fixture is *"re-typed rather
than shared"* was falsified by this lane's own delegation three lines
below it and is rewritten. `D90`'s and `D321`'s `fillet/` citations
are re-aimed to the `blend/` spelling — the discipline this lane's
`D124` chapter applies, now applied to the rows the lane did not take.
Both `Cargo.toml` dev-dep comments are rounded honestly (five of six
delegate; the sixth is deliberately independent). One over-wide
comment in the tour rewrapped.

**Filed, not fixed: the rename left stale `fillet/` citations well
past `D90` and `D321`.** Re-aiming those two exposed eight more in
other tracks' records — `docs/SMELL-SCAN-2026-08.md` lines 1252, 2796,
2797, 2945, 3882, 3977, 3978, 3996 — plus the historical
re-derivation quoted at `S177`, which this lane's own `D124` chapter
corrects in place rather than editing the quotation. They are other
tracks' rows and findings, so this lane did not re-aim them; a
document-wide citation sweep after a module rename is the shape of an
`L`-row and the orchestrator owns whether it earns one. Recorded here
so the next dispatch starts from the hit list rather than rediscovering
it.
