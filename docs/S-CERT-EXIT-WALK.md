# S-CERT exit walk — criteria vs evidence

**STATUS: PROPOSED — awaiting Ev's ratification.** Per the convention Ev
confirmed for S-MATE (2026-09-04: merging a PROPOSED walk is the
ratification), this walk rides an `[ev]` PR; merging it ratifies it, and
the closing sweep that follows (delete `work/cert/`, ledger the deletion
in `docs/DOC-LEDGER.md` with the sweep SHA) is the program's last act.
**The residue is already re-homed in this PR** — every open item that
lived under `work/cert/` now sits in the home the handoffs ledger names,
so the sweep moves nothing. Until then `work/cert/log.md`'s tail is the
program's live status.

S-CERT = the certificates-that-lie program (`work/cert/{program,plan,log}.md`,
formerly `docs/S-CERT-PLAN.md` / `docs/S-CERT-LOG.md`; graduated from
`docs/WORK-STREAMS-2026-08.md` 2026-08-29; A/B band 700–799). Criteria are
quoted **verbatim** from the plan's exit-shape paragraph, one clause per
row; dispositions per the M5–M8 / ASM / S-QA convention: MET /
MET-WITH-RECORDED-HONESTY / CARRIED (named owner). Walked against main
`37eaf5b9b` (2026-09-05) by `git show origin/main:…`, not from memory.

## The walk

| # | Criterion (verbatim from the plan's exit shape) | Disposition | Evidence / honesty note |
|---|---|---|---|
| 1 | "the sphere polar arms take their extent from spans and their rim lever holds at the poles (VERBS-SPHSPH unblocked)" | MET-WITH-RECORDED-HONESTY | **CERT-1 (#1220)**, issues 723 + 893 closed: both arms take the v-extent from the traversed arc's span; the rim lever meters in latitude terms, red-first closed-form rows at a near-polar interior rim; the STEP half-cap twins committed as fixtures. The fix pass folded indeterminate pole decisions (refusal retired, served instead) and saturated the \|dt\| > 2π membership at a half-turn — both R2 unilateral executed MAJORs, the program's first two tally candidates. VERBS-SPHSPH's stated prerequisite is discharged (VERBS-SPHSPH DELIVERED #1290). **The honesty**: the rimless polar cap (found independently by both reviewer arms) is open with an owner — `work/cert/rimless-polar-cap-refuses-degenerateface.md` (github 1250); the K roster consequence likewise (`k-report-baseline-fold-cert1-roster.md`, github 1251). |
| 2 | "zero-angle rotations are width-preserving at Interval" | MET-WITH-RECORDED-HONESTY | **CERT-3 (#1277)**, issue 924 closed: the constructor anchors through `(I − R)` with half-angle factors; the zero-angle identity is exact at f64 and the Interval residue is subnormal and proportional-with-subnormal-slope (the fix pass corrected the "independent of anchor width" wording); RevolvedPoint start samples 4.0e-9 → 2.66e-15. **The honesty, two entries**: the residue attribution in the delivered body was FALSE (R2's unilateral executed MAJOR; corrected at every site it propagated, including issue 1143's member-5 payoff), and this is the one unit whose dual ran WITHOUT A/B logging (the handoff unit; Ev's in-chat ruling 2026-08-30) — the band's record carries no row for it by design. |
| 3 | "the period folds are honest at Interval with f64 bits unmoved and M10-3 consuming them" | MET-WITH-RECORDED-HONESTY | **CERT-4 (#1303)**, issue 1191 closed: the composed fold retired at one home (`Real::periodic_branch` / `reduce_periodic_centred`, comparison-free); every hit-list site fixed or dispositioned; the m10-p interval digest re-derived narrower on all 75 coordinates. **The honesty**: the f64-bit constraint was RESTATED SEMANTICALLY by Ev (2026-08-29) and the exact-fit structural zero survived by construction with its domain CORRECTED at the fix pass to the rounding condition `fl(fl(d/τ) + ½) < 1` after both lanes executed the boundary failure; and the spec's anticipated m10-3 driver pin flip did NOT happen — that row's widening is a different mechanism (dependency problem, box-scaled), so the doc was retargeted rather than the row forced, verified by both reviewers. "M10-3 consuming them" holds as the fold being live under M10-3's driver, not as a flipped pin. |
| 4 | "the chart-speed guard family refuses non-finite by name everywhere" | MET-WITH-RECORDED-HONESTY | **CERT-2 (#1221)**, issue 762 closed: `march.rs`'s sibling guard takes D285's exact signature red-first (`StepCollapsed` on non-finite speed), the NaN fold pinned, the poison-arm sentences made producible, plus three D285-spelling predicate siblings the implementation sweep missed and R2 found; D285/D286 left Track Q's table in the landing PR. **The honesty**: `work/cert/ssi-chart-speed-usability-boundary.md` (github 1238) records the finite-but-unusable-speed class (from both reviews' probes) — "non-finite" is refused by name; "finite but unusable" is named and owned, not closed. |
| 5 | "rational patch flux certifies through interior off-grid knots on both the native and import doors (dm1 flips)" | MET-WITH-RECORDED-HONESTY | **CERT-5 (#1314)**, issue 453 closed: knot-aligned cells at four sites retire the Θ(1/pieces) straddle floor (the straddle branch deleted, not bypassed); the `w`-uniform-in-v exact arm taken; the regression row at 6+ stations with off-grid knots; the lily flip is a GATE re-measured on the final head. **The honesty**: dm1 tightened 146× (2.7469e-4 → 1.885e-6) yet stays 1.84× above target — **honestly NOT flipped**; the dial decision is `quad2-rational-max-rounds-dial-decision.md` (github 1315) with corrected figures; issue 390 stays open annotated (route 2 unclaimed). This is the walk's one parenthetical the program did not deliver as written, and the record says so with the digits. |
| 6 | "the area enclosure is metered under the ratified #870 proposal" | MET | **CERT-6 (#1366)**, issue 870 closed: the A2 gauge as the Q1 ruling's row-5-boundary `debug_assert` — a certified chord-traversal bound under a max(chord, caller) denominator, ceiling 1.0 with margins stated against door-authored anchors (79×, 13.1×, 17.6×) rather than a corpus statistic; the calibration figures reduced to one home. R1's unilateral executed MAJOR (the delivered 16-chord schedule aliased — Nyquist collapse at k ≡ 0 mod 8) is the third tally candidate; the fix pass measured the adjudicated fix insufficient and shipped the stronger coprime + max-denominator design. The refinement door stays filed, not built (`purchasable-area-tightness-valve.md`, github 1367 — no consumer asks). |
| 7 | "the offset_fit composite certifies rational fits, recentres, and refines directionally" | MET | **CERT-7 (#1319)**, issues 1005 + 1007 + 1008 closed: the weighted composite certifies the exact rational offset at 2.837e-14 on one cell; recentring makes the certificate translation-honest with a decade ladder to 1e6 and an honest refusal endpoint at 1e10; directional refinement takes the thin patch 308 → 14 cells, the stall guard's admission set structural. Zero MAJORs from either arm; the far-origin non-monotonicity settled by measurement and the row claims only what the ladder shows. |
| 8 | "chart-stretch arms are metered from real bounds on both channels" | MET | **CERT-8 (#1398)**, issues 501 + 528 closed: `nurbs_stretch_bounds` exported through a proper layer, `azimuth_arm`'s NURBS arm and both `v_meter` fallbacks metered, the audit row OK; the chart-region positive-area lane carries inf-side arms per kind, the arm gate reading the bracket FLOOR so a folded net refuses typed in both lanes; the fix pass added the sphere arm's cos-monotone-range refusal the reviews only suspected. The mean-width contract narrowed at the claim site (R2's strip exhibit, 97× model-vs-reading). |
| 9 | "`signed_volume` is recentred" | MET | **CERT-9 (#1361)**, issue 303 closed: the fold recentres on the bbox centre (overflow-robust form); red-first digits vivid (33.3 vs the true 1e-9 at a 1e8 m offset — pre-fix far placements could sign-flip a volume); R2 proved the shipped argument understated the mechanism (equivariance gives unconditional translation invariance) and the doc now says so. |
| 10 | "tracks M and N are empty in §D" | MET-WITH-RECORDED-HONESTY | §D no longer exists — `docs/SMELL-SCAN-2026-08.md` was deleted by the tracker migration (2026-09-03) and every row became a `work/` item carrying `track:`; the criterion is walked on those items at main `37eaf5b9b`. **Track N: EMPTY** — every `track: N` item is `closed` (D240–D243 by **CERT-N1 #1536**; H2's S99/S101/S102/S103/S116(b)+(s) by **CERT-N2 #1558**; D244, D31, D98, S235, C24 by **CERT-N3 #1879**). **Track M: two open items, neither dispatchable by this program** — `work/cert/H5.md`, rewritten by **CERT-M3 #1877** into a SCHEDULE of three questions for Ev (PcurveFittedLane's representation question; ChartRegionLane's contract, `[ev]` PR #1878; the lane-keeping at-rest doors' default name, `lane-keeping-at-rest-doors-skip-the-m7-8-class.md`) plus the structural bound's honest end state; and `work/code-quality/S90-impl.md`, parked on PR 883 (lane H-g), which the program header's keep-out says is not this program's to unpark. Everything else Track M held closed: D221, H3+H4, H10+S211, D78 (**CERT-M1 #1533**), S213 (finding + row), D222, D223 (**CERT-M2 #1559**), S3's EdgeNurbsLane member and the trait itself (**CERT-M3**), D224 and S290 (closed by others after the migration). **The honesty**: "empty" is true of N and true of M's dispatchable work; M's two survivors are questions, and the program did not answer them because they are Ev's (a completeness contract; a representation decision; a default-name ruling that evicts `Body<Dual64>` from a door). |
| 11 | "Every unit merged on its own green hosted head; the walk convention applies at exit" | MET-WITH-RECORDED-HONESTY | Unit PRs #1221, #1220, #1277, #1303, #1319, #1314, #1361, #1366, #1398, #1403, #1533, #1536, #1559, #1558, #1877, #1879, each merged on a green hosted run of its merge head; ε/lane trailer-pinned from CERT-6 onward under the issue-1356 practice, until main's filter made the trailer additive-only (2026-09-04) — from then `lane=both eps=all` is the whole matrix by default. **The honesty**: CERT-10 (#1403, issues 1006 + 1322) is not named in the exit paragraph — it landed under the Q2 ruling and is walked here on its own record (the fold's cost taken BEFORE the shape was chosen; the fourth tally candidate is R1's instrumentation MAJOR that the body's "removed an assembly per face" was false). Two units (CERT-5, CERT-7) merged with ε 1e-12 unsampled on one head and no claim resting on it; four latent main reds at (interval, 1e-12) left by CERT-3/CERT-4 were found and repaired in CERT-5's PR, the class filed as issue 1356. CERT-M3's merge head carried ONE inherited red (profile's seeded fuzz miss at eps=1e-6, reproduced byte-identical on bare main, filed `work/bool/profile-fillet-radius-off-at-eps-1e-6.md`) under the inherited-red rule (Ev, 2026-08-31); CERT-N3's closing docs commit collided with CERT-M3's log entry on main and merged clean once resolved, both entries in main's merge order. |

## Walk evidence beyond the criteria

- **The A/B record**: ordinals 700 (CERT-2), 701 (CERT-1, sample #48), 702
  (CERT-4, #62), 703 (CERT-5, #66), 704 (CERT-7, #64), 705 (CERT-9, #67),
  706 (CERT-6, #70), 707 (CERT-10, #96), 708 (CERT-8, #95), 709 (CERT-M1,
  #98), 710 (CERT-N1, #99), 711 (CERT-N2, #108), 712 (CERT-M2, #107), 713
  (CERT-M3, #133), 714 (CERT-N3, #134) — claimed on main at review
  dispatch; samples assigned at merge in main's merge order (the #62 and
  #64 races with S-QA resolved by the merge-order hedge; #44 and #48 after
  the #42/#43 collision correction; #133/#134 with four other programs
  landing rows concurrently). CERT-3 has no row by Ev's handoff ruling.
  Blocks CERT-B1..B4 drawn by the v4 rule with bytes recorded on
  `cert/ab-state`; every dual sequential, blinded, cross-model, on a frozen
  head with identical stored briefs (`cert/ab-state` `ab/CERT-*-dual.txt`);
  every fix pass implementer-inherited except CERT-2's (ran cross-slot —
  orchestrator error, covariates excluded) and CERT-3's (fresh lane after
  the handoff, no A/B row anyway).
- **The v6 tally, QUEUED at exit (prior programs' convention)**: the
  unilateral-executed candidates, by unit — CERT-1 ×2 (R2 opus: the
  escalating pole predicate; the \|dt\| > 2π membership hole); CERT-6 ×1
  (R1 opus: the aliased chord schedule); CERT-10 ×1 (R1 opus: the false
  per-face assembly claim, by instrumentation); CERT-N2 ×4 (R1 opus: the
  in-fence sweep undischarged, the transposition claim false; R2 fable: the
  AABB door pruning on the masquerade, S101 swept from the wrong commit —
  the program's first fair pair with executed MAJORs on BOTH arms); CERT-M2
  ×1 (R1 fable: the closed-form +V verdict retired with the refusal);
  CERT-M3 — R1 fable ×3 (the census absence row a half-fix; the contract
  question on a false premise, overturning R2's verdict, orchestrator-
  confirmed; the certified-twin fix unmeasured) and R2 opus ×2 (the skip
  whole-edge, executed; S3's counts arithmetic not re-derived, executed),
  beside two bilateral executed MAJORs; CERT-N3 — R1 fable ×4 (S89's text
  never edited, executed; the pruning table vs the committed corpus; the
  C24 consumer count of 0, executed; S66's stale paragraph) and R2 opus ×4
  (the π shift-back mutation leaving geom green, executed; `ANGLE_SLOP`;
  the file header falsified by `edge_axial_span`; the descending-run
  coupling, executed), with S235's soundness upheld by BOTH by execution.
  Calibration data, not candidates: the severity-divergence pairs on
  converged substance in CERT-4, CERT-8, CERT-M1, CERT-M2 (H5's numbers,
  the 8/9 gating) and CERT-N2 (the roster, class 9, the citation bill);
  CERT-7, CERT-9, CERT-N1, CERT-N3 with no MAJOR either arm; CERT-M3 with
  convergent NMAI. The blinded coding (MODEL-AB-LOG item 4) is handed to
  the next tally session with `cert/ab-state` and the per-unit adjudication
  notes as the source of record; no readout was read by this orchestrator
  while a dispatch was in flight.
- **The program's signature — the two track questions plus §D rule 5 as
  the review's spine**: in every reviewed unit the dual found a claim in
  the delivered body that execution falsified — CERT-1's "declared
  deviations", CERT-3's residue attribution, CERT-5's `tier_gate`
  "corrected", CERT-10's "an assembly per face removed", CERT-8's "nothing
  is denied" about rustdoc, CERT-M2's "byte-identical passes" (true) beside
  its retired verdict, CERT-M3's "exactly two doors" and "the only arm
  reaching the trait", CERT-N3's "tens of calls per boolean" and "dead
  code". None reached main. The brief-as-claim-site rule (verify report
  claims against the artefact before briefing) came out of CERT-1 and held.
- **What the program found beyond its slate**: the (interval, 1e-12)
  matrix point never sampled (issue 1356 — the ε-trailer practice adopted
  program-wide from CERT-6, later superseded by the additive-only filter);
  `refine_dir`'s exact-equality insertion, worse on main (1358); the
  k_probe_sweep death at ε 1e-6 (1304, M10's ground); the chord_join pole
  shift (1305); the walk.rs anchor class (1362, S-MESH); the finite-but-
  unusable-speed class (1238); the rimless polar cap (1250); the normalize
  norm²-overflow (1299, PCURVE's keep-out); the two recentring centres
  (1532); profile's seeded fuzz miss at eps=1e-6 (`work/bool/`); main red
  mid-unit three times (PR 1239, PR 1257, PR 1534 — each repaired
  orchestrator-direct within the hour and disclosed in the affected unit's
  method note) plus one inherited red annotated rather than absorbed.
- **The absorbed tracks' method**: the SMELL rows were closed under §D's
  rule 3 (closed rows and findings DELETED member by member, standing
  rules relocated first) and rule 5 (no fresh instance), with the fence
  drawn by the orchestrator per unit (rule 1) and widened on the record
  where a ruling required it (CERT-M2's twenty sites; CERT-M3's two
  allowlist files and the fix pass's two consumer sites; CERT-N3's two
  seams and the D98 ripple). CERT-M2's census became CERT-M3's executable
  spec, and CERT-M3's dispatch ruling narrowed the spec to that census —
  the one time the orchestrator overrode a spec's letter, recorded in the
  PR body and the log.
- **Handoff, container and limit incidents, owned**: the predecessor
  session lost git auth after CERT-3's delivery (this walk's author is the
  successor); the container restarted mid-wave killing CERT-6's first lane
  (~50 m, fragment saved, re-dispatched on the same arm) and again at
  08:05Z 2026-09-02 across two CERT-M2/N2 reviewers (resumed from
  transcripts); a CERT-M2 lane committed its in-tree target directory and
  re-landed on a fresh branch (the discipline doc gained its
  target-outside-the-worktree bullet, main 922eb5f1); a Fable rate limit
  killed CERT-N3's implementer mid-unit (two days lost to the gap, nothing
  in-tree) and the Opus session limit killed CERT-M3's implementer
  mid-fix-pass and CERT-N3's R2 together (Ev's other sessions on the
  account) — all resumed from transcripts with no rework; the usage
  counters record only post-resume segments and the A/B rows say so. One
  orchestrator brief defect: CERT-M3's reviewer brief cited a ruling item
  that lived only on the `[ev]` branch; both arms received it identically.
- **The tracker migration under the program**: `work/` landed 2026-09-03
  with two units open; both lanes re-homed their landings into `work/`
  items without orchestrator help and `work.py lint` caught nothing they
  missed. The program's docs are now `work/cert/{program,plan,log}.md`; the
  exit walk stays at `docs/S-CERT-EXIT-WALK.md` and is deleted with the
  directory at close, the DOC-LEDGER row being the done-state of record.

## Handoffs ledger — the residue and its homes (RE-HOMED IN THIS PR)

`work/README.md`: a closed program's directory is deleted whole, and its
residue is re-homed to a live program or to `work/issues/` BEFORE the
sweep. Every open item under `work/cert/` at main `37eaf5b9b` is moved in this
PR to the home below (Ev, on the PR: "as long as all residuals are filed
appropriately"); ids are unchanged, so every reference still resolves, and
the two `work/code-quality/` filings that named CERT-N3 and S235 (D291,
D292) now cite PR 1879 instead. Re-point any of them by moving the file;
`work.py lint` is green on this head.

| Item (formerly `work/cert/`) | github | Home (moved in this PR) |
|---|---|---|
| `H5` — Track M's schedule of three questions for Ev | — | `work/code-quality/` (its origin; `parent:` cleared — no unit carries it; `refs` kept) |
| `lane-keeping-at-rest-doors-skip-the-m7-8-class` | — | `work/code-quality/` beside H5 (its third question; the fix is a ruling) |
| `chart-region-lane-contract` (ruling, `[ev]` PR #1878, needs_ev) | — | `work/code-quality/` beside H5 — moved on PR #1878's own branch, where the file lives until Ev answers |
| `rimless-polar-cap-refuses-degenerateface` | 1250 | `work/props/` |
| `two-face-sphere-split-measures-zero-volume` | 1598 | `work/props/` |
| `props-refusal-cannot-carry-measured-overshoot` | 1602 | `work/props/` |
| `props-two-eps-vocabularies-five-sites` | 699 | `work/props/` |
| `quad-face-extent-trusts-caller-perimeter` | 1368 | `work/props/` |
| `purchasable-area-tightness-valve` | 1367 | `work/props/` |
| `k-report-baseline-fold-cert1-roster` | 1251 | `work/props/` (the K roster; Track K's) |
| `budgetexhausted-conflates-three-terminations` | 1321 | `work/issues/` (offset_fit; no live program owns `geom-brep/src/offset_fit.rs` after this one) |
| `offset-fit-mignitude-floor-on-norm-e` | 1320 | `work/issues/` (offset_fit) |
| `patch-bound-offset-fit-recentring-origins` | 1532 | `work/issues/` (patch_bound + offset_fit) |
| `refine-dir-hairline-knot-insertion` | 1358 | `work/issues/` (offset_fit; refs C3, D30 stay) |
| `quad2-rational-max-rounds-dial-decision` | 1315 | `work/issues/` (a dial decision; owner Ev at his time) |
| `loft-seam-carrier-exact-knot-compare` | 1316 | `work/fillet/` (`sweep::loft`, pcurve_cache's seam compare — the program that owns `crates/sweep/src/*`; else `work/issues/`) |
| `pole-branch-pick-two-integer-shift` | 1305 | `work/bool/` (`chord_join`, topo boolean) |
| `normalize-overflow-yields-zero-axis` | 1299 | `work/issues/` (`geom-core` vec.rs; PCURVE's keep-out is closed) |
| `ssi-chart-speed-usability-boundary` | 1238 | `work/trim/` (ssi is Track Q ground; TRIM is its live program — else `work/issues/`) |
| `orthonormal-basis-poisons-vertical-planes` | 1157 | `work/issues/` (geom-core; refs 1116/1143/1146) |
| `symbolic-tier-census` | — | `work/m10/` (the symbolic tier is M10-7's) |
| `param-box-certification-of-implicit-quantities` | — | `work/m10/` (the frontier M10-7's identity layer does not reach) |
| `tess-budget-doc-finding-block-stale` | — | `work/tcost/` (TESS-BUDGET.md) |
| `nurbs-net-point-map-helper` (status review, PR 1742 from `fix/`) | — | `work/fix/` (its PR's program) |
| `unify-edge-descriptions-on-pcurves` (ruling, closed) and every closed unit/row item (CERT-M3, CERT-N3, C24, D31, D98, D244, S235, …) | 427 | deleted with the directory at the sweep (closed; recoverable at the sweep SHA) |

Program-header keep-outs that survive the close and need no re-homing:
PR 883 (lane H-g) stays parked as lane H-f on `work/code-quality/S90-impl.md`;
the bvh interval lift, `dual.rs`, the `AtRestPolicy` seam and `product.rs`'s
Dual arms are M10's; `props/quad.rs`'s consolidation (C3, D30) is Track
R's; `ssi*` and `pcurve_cache` are Track Q's. The territory globs on
`work/cert/program.md` (`crates/geom-brep/src/props/*`, `offset_fit.rs`,
`patch_bound.rs`, `crates/geom-core/src/*`, `crates/geom/src/*`,
`crates/bvh/src/*`) fall back to the tracker's default (no owner; items
file to `work/issues/`) unless Ev assigns them at ratification — `props/*`
has an obvious taker in `work/props/`.

Standing pointers outside the tracker: the four-plus tally candidates (the
blinded tally session; `cert/ab-state` is the source of record); CERT-8's
`edge_chord_len` 1 m default at two plane-gated sites
(`edge-chord-len-defaults-to-one-metre.md`, github 1529 — re-home with the
props items); issue 390 (rational flux route 2) open and annotated; the
`cert/orchestrator` branch (the binding specs `docs/CERT-*-SPEC.md`, never
on main — recoverable there; DOC-LEDGER's per-merge spec rule applies to
main only) and the probe branches `cert/*r{1,2}-probes` (kept; private
remote, cheap).

## Open with Ev at ratification

1. Row 5's parenthetical "(dm1 flips)" is walked as
   MET-WITH-RECORDED-HONESTY on the digits (146× tighter, 1.84× above
   target, dial decision `quad2-rational-max-rounds-dial-decision.md`). If
   Ev reads the parenthetical as a hard criterion the row is CARRIED to
   that item's owner instead; the code state is identical either way.
2. Row 3's "M10-3 consuming them" is walked as the fold being live under
   M10-3's driver, not as the anticipated pin flip (which did not happen
   for a verified reason).
3. Row 10's "empty" is walked as: Track N empty; Track M reduced to a
   schedule of three questions for Ev (H5 + its two companion items) and
   one parked lane the program's charter keeps out of. If Ev reads "empty"
   as zero open items, the row is CARRIED to `work/code-quality/` with the
   three questions as its owner's residue; the code state is identical.
4. The three questions themselves — ChartRegionLane's contract (`[ev]`
   PR #1878, restated on the corrected three-arm premise), PcurveFittedLane's
   representation question, and whether the certified at-rest doors
   become the DEFAULT name (evicting `Body<Dual64>` from
   `validate_pseudomanifold`) — are not this walk's to answer; they are
   listed so ratification does not read as answering them.
5. The homes in the handoffs ledger are executed in this PR; merging
   accepts them. The sweep after the merge deletes and ledgers only.
