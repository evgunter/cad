# S-BLEND exit walk — criteria vs evidence

**STATUS: RATIFIED — S-BLEND CLOSED (Ev's approval given
directly in-session, 2026-08-31, on PR #1370's summary with the
five open items listed; the block-B3 close-short recommendation
approved and executed in the ratification commit). This document
is S-BLEND's done-state of record.**
S-BLEND = fillet/chamfer completion (`docs/S-BLEND-PLAN.md` /
`docs/S-BLEND-LOG.md`; graduated from `docs/WORK-STREAMS-2026-08.md`
2026-08-29; A/B band 600–699). The plan has no single exit-shape
paragraph, so the criteria are quoted **verbatim** from its charter
and unit slate, one commitment per row, dispositions per the
M5–M8/ASM/S-QA convention: MET / MET-WITH-RECORDED-HONESTY /
CARRIED (named owner).

## The walk

| # | Criterion (verbatim from S-BLEND-PLAN) | Disposition | Evidence / honesty note |
|---|---|---|---|
| 1 | "the fillet band/surgery gaps the ARMS program filed on its way out, and chamfer parity with the fillet. The verbs exist and refuse honestly; this stream builds the missing doors" | MET | Seven unit PRs merged, each on its own green hosted head: #1222, #1267, #1268, #1301, #1328, #1347, #1360 — issues 1022, 827, 935, 961, 917, 919, 644 all closed at their merges. Kernel-side only throughout; the recipe layer stayed LIB's per the G16 seam (S-BLEND never touched `emit_fillet.rs`). |
| 2 | BLEND-1: "the walk carries through seam vertices instead of terminating, and per-kind supports may be several FACES of one SURFACE. Closes the A3-2 recourse gap on all three lantern rims" | MET | **#1222**: `AnnulusRim` carries `crossings: Vec<SeamCrossing>`; built to the corrected A3-2 record per the carried handoff, and the promised recourse's live pin (`requesting_the_rim_whole_gets_past_the_seam`) went from red to green on the real lantern; all three rims (mouth, neck, lip — including issue 319's plane×sphere neck) exercised. Band-frontier handoffs filed at adjudication: 1244 (concave closed-rim band), 1245 (boolean-repaired pole-touching rim), 1246 (public rim-arc selector). |
| 3 | BLEND-2: "refresh only the annulus rims' seam keys between carves, keeping every decision in the plan phase — so the decide-before-mutate discipline stands. If the unit measures that shape insufficient, changing the discipline is a design fork: STOP" | MET | **#1268**: the narrow alternative sufficed — `refresh_annulus_seams` re-reads a later rim's crossing keys against the mutated body, plan-preserving; the escape hatch was never pulled and no design fork opened. Issues #1306 (CapEnd spatial lie), #1308 (inert `deny_unknown_fields` class), #1309 (.pyi stub blindness) filed at adjudication. |
| 4 | BLEND-3: "widens the two admission doors (`corner_config`'s all-convex requirement, `ConvexOpen` — renamed `AdmittedOpen` by the unit), authors the concave-corner fixture through the public API, and checks the carve walk is orientation-agnostic" | MET-WITH-RECORDED-HONESTY | **#1347**: doors take the convexity clause per verb, the classifier verb-blind with base's public signature restored, the fillet's convex-0 refusal proven byte-identical to base by the delta's front-door differential. **The honesty**: the unit minted a `CornerConfig` vocabulary tag (`ThreeConcaveEdges`) without ratification; the adjudication WITHDREW it to design-conversation **#1355** (Ev's) after the SeamVertex precedent's timeline was orchestrator-verified as ratify-then-mint. The pair also produced the experiment's **Tally Candidate #1** (R1's unilateral MAJOR standing on verified evidence). |
| 5 | BLEND-4: "ball admissibility, feet signs, octant chart orientation, arc traversals and the sense bit move as ONE change, with a concave fixture, then the three doors relax together. Verify `corner_ball`'s unexercised concave arm before building on it" | MET | **#1360**: the precondition paid FIRST (the concave arm measured CORRECT before anything moved, committed separately, exactly the S11 machinery-with-no-producer check the issue asked); ball side, feet sign and chart fold moved with the two doors as one change; the arc-traversals-and-sense half MEASURED as needing no edit (attach_contact side-blind; sense already derived per #640) — half the predicted change was already right, established by measurement rather than assumed. R2's C′ mutation exposed the chart fold's `u_ref` half pinned by nothing; the fix closed it (adopted probes red under C AND C′), and an R1-arm delta lane re-verified every count. The #1359 collision resolved LOUDLY toward BOTH guards + fold, with the concave cross-corner twin row added. |
| 6 | BLEND-5: "Names a curved-on-curved rim's support by its role in the carve or by its kind read at emit time; persisted-vocabulary change with its N-doc migration story and its own schema-seam claim" | MET-WITH-RECORDED-HONESTY | **#1301**: `RimSide::{Host, Mate}` → `RimSupport::{Host, Mate}` by ROLE, replacing the kind guess that emitted `Plane` on a cone (the misleading emission pinned pre-change); schema seam claimed at dispatch per the standing discipline. **The honesty**: the shape's reason 3 was WITHDRAWN, not softened — the reviewers' silent-retarget corollary is in the record (a kind rename fails loud, a role swap silently retargets), and the shape stands on the collision argument alone; whether a rim reference wants a resolution-time check is handed to Ev as a design conversation (open at exit, below). |
| 7 | BLEND-6: "HOW a shared refusal names the verb that raised it … is user-facing refusal prose with several viable answers — Ev's call before the ~255-reference rename executes. Must not be closed by minting a parallel enum" | MET | **#1328**: `docs/BLEND-VOCAB-DESIGN.md` ratified on Ev's #1279 sign-off FIRST, then executed — `BlendRefusal { verb, error }` minted once per door, verb-neutral inner prose with measured ball-only/chamfer-only lists, `sweep::fillet` → `sweep::blend` with thin per-verb doors; V4 held (no parallel enum — both arms verified). The rename measured 489 refs / 80 files against the issue's ~255 and was structured LAST for separable review. **Recorded at adjudication**: the zero-radius door asymmetry is a behavior change beyond the ratified scope — declined with argument and scheduled as **#1336**, cited at both doors. |
| 8 | BLEND-7: "Executes the ratified `docs/ENCLOSING-TANGENCY-DESIGN.md`: opening measurement (what the lattice serves past the ordinary branch), then the typed refusal, the pins' hedge-drop, and the `sugar.rs` purpose statement" | MET | **#1267**: measured-first per the doc's item 1 (the pre-change bracketed enclosing service quantified), then the ρ < 0 class refuses typed; the `JunctionTangent` pin's measured margin (1.6e-17) was the carried starting point per the handoff. The `crates/profile` fence exception rode the dispatch as the ruling's closing unit; `profile::structure` untouched. |
| 9 | BLEND-T: "The track taken whole per the partition rule; rows worked as style lanes under the SMELL conventions (execution record in `docs/SMELL-T-LOG.md` when it starts)" | MET-WITH-RECORDED-HONESTY | Claimed at SHELLFIX 2b's merge, constituted in `docs/SMELL-T-LOG.md`, executed whole across lanes T-a/T-b/T-c: **struck C20, C25, D90 (ADV — the defect was documented and pinned in the tree; now a total vertex-identity check), D91, D96 (enumerated nine-not-ten, then answered site by site: four type-change deletions, four costed noes, one re-scoped), D104, D124**; minted D325, D242/D243 (→ track N), D304 (→ track R); D320 stays filed-not-takeable behind D240; D322–D324 were HELD under T-R1's keep-out and **released at #1360's merge** (their §D cells name it). **The honesty, twofold**: (1) T-R7 was first recorded as a lift on a FALSE premise — the orchestrator's own dispatch brief said the slate had vacated `blend/` while the same wake dispatched BLEND-4 into it; caught at T-c's review, corrected to NO-LIFT with the provenance owned in the ruling. (2) The track ran OUTSIDE the A/B experiment on the F/G/I precedent — with the lapsed-pause caveat recorded (the pause that grounded the precedent had lapsed; revisitable by Ev). |
| 10 | "issue 987 (ruled-spine carve) is double-gated … It schedules only after a design conversation AND a named consumer; neither exists today" | CARRIED (unchanged) | Neither exists at exit either. The gate held: no unit touched it, and the OQ6 run-out taxonomy stays reserved for Ev per A3-3's parked pair. |
| 11 | "Full model A/B per `docs/MODEL-AB-LOG.md` … v3 triple blocks for implementer arms, v6 cross-model duals every row, pre-draw difficulty logging, record-at-merge, per-phase tokens/wall-clock, blinding fences" | MET-WITH-RECORDED-HONESTY | Seven rows, ordinals **600–606** claimed on main at dispatch, v6 duals every row (briefs authored+stored symmetric before R1, sequential-same-head with pre-recorded method notes where a build slot bound), samples **#46, #52, #55, #68, #69, #70, #71** — verified UNIQUE at exit. Blinding held (no trailers, no model talk in lane paths; arms branch-side until block close). **The honesty, three notes for the analysis**: (1) three samples were RENUMBERED at exit (#68, #70, #71) after a busy merge window double-booked #57, #64 and #66 across programs — earlier-merged rows keep, the mover takes the next free (the AZ-1/QA-6B procedure; repairs at #1360's state-sync and #1369). (2) The rows' idiom/tests/docs cells are the orchestrator's adjudication mapping from the reviews' style sections — the briefs asked rubric self-scores only, a brief-format drift disclosed here rather than silently backfilled. (3) BLEND-4's post-fix delta ran as a FRESH agent on R1's drawn arm (the original R1 context was lost to compaction; disclosed in the row). |

## Walk evidence beyond the criteria

- **Block accounting — one question for Ev**: BLEND-B1 and B2
  drawn, consumed and closed on main (B2's close at #1357). **B3 is
  OPEN**: drawn branch-side at BLEND-3's merge, slot 1 consumed by
  BLEND-4 (arm still branch-side per the redaction shape), slots
  2–3 unconsumed — and this was the slate's last unit.
  Recommendation: close B3 SHORT at this walk's ratification
  (restate slot 1's arm, land the draw record on main in the
  ratification commit); no successor program consumes the band and
  the analysis needs the arm. **Approved and executed at
  ratification**: the CLOSED SHORT record and BLEND-4's arm
  restatement land in this walk's ratification commit.
- **The experiment's yield**: Tally Candidate #1 (ordinal 605,
  BLEND-3) — the v6 unilateral-MAJOR tally moved 0 → 1, coded in
  the row; no other candidates in seven duals (BLEND-4's verdict
  split carried NO MAJOR on either arm — the arms laddered on the
  same question instead, R2's invented mutation finding the gap
  R1's dispatched one could not).
- **Dispatch discipline, calibrated**: the program's one
  dispatch-premise error that reached a ruling (T-R7's false
  premise) is owned in `docs/SMELL-T-LOG.md` with its provenance
  stated; it was caught by a reviewer treating the dispatch as a
  hypothesis, which is the review shape working as designed. The
  BLEND-5 reader-census needle (a probe assertion ending in a
  `.rs` filename) tripped main's census after merge — fixed by
  #1323 (Ev's pointer), and the needle rule propagated into
  every subsequent brief.
- **What the program found beyond its slate**: the #1330 doc-gate
  main red (theme.rs links under `--skip-viewer-toolkit`), found
  at a state-sync and fixed orchestrator-direct the same day
  (#1332); the {interval, 1e-12} measured-constants red flagged as
  #1338 (the tree has since healed by a re-measure on main; the
  issue stays open); the standing dev-probe k-lint red cited as
  #1296 where drawn (BLEND-2/BLEND-6 standing-down comments, per
  the pin-to-proven-green precedent).
- **Handoffs ledger at exit**: issues 1244, 1245, 1246 (BLEND-1's
  band frontier), #1294 (the annulus-rim emitter gap, LIB's seam),
  #1306, #1308, #1309 (BLEND-2's adjudication), #1336 (the
  zero-radius door asymmetry), #1364 (the shared test-support
  fixture/oracle home), #987 (double-gated, untouched), D320
  (behind D240), D322–D324 (released, takeable), D325, D242/D243
  (track N), D304 (track R).
- **Open with Ev at exit** (none block ratification; all have
  homes): **#1355** — the honest corner tag, ratify-then-mint (the
  options mapped in the issue); the **rim-reference
  resolution-time check** (BLEND-5's silent-retarget corollary) —
  a design conversation nobody has opened; the **per-site row-0
  ratification question** left in PR #1359's body by track T's
  last lane; the **block-B3 close-short** recommendation above;
  and the **lapsed-pause caveat** on track T's A/B exclusion, if
  the F/G/I precedent is to be re-grounded for future tracks.
