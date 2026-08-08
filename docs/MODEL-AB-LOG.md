# Opus 5 vs Fable 5 implementation A/B — experiment log

**This is process data (an experiment log), not a design
reference** — nothing here binds kernel design; it moves out of
`docs/` when the experiment concludes.

Standing experiment (Evan, in-chat, 2026-07-25, Opus 5 release
day). This document is the SINGLE normative source of the protocol
(`memories/model-ab-experiment.md` is a pointer here). Protocol, as
amended:

- **Protocol v2 (Evan, 2026-07-25): blocked randomization** — the
  original per-dispatch fair coin flip (v1, rows 1–10) is
  superseded. Arms come in opus/fable PAIRS, order shuffled per
  block from /dev/urandom (block 1 forced (opus, fable) as a
  recorded balance correction after four consecutive fable draws;
  random order from block 2); every row from #11 onward records its
  "block-N draw". Design, specs, adversarial reviews, and fix-pass
  rulings stay Fable regardless. The fix pass runs as the
  implementer's agent, so it inherits the arm.
- **Protocol v3 (Evan, 2026-08-08, post-bayes-readout): TRIPLES.**
  Blocks are now the shuffled multiset {opus, opus, fable} —
  rationale: the readout shows no quality separation, a consistent
  lean toward opus on findings and cost, and modest power loss at
  2:1 (~12% contrast-variance inflation), so allocation shifts
  toward the cheaper arm while keeping a live fable stream for
  drift detection. Draw: one /dev/urandom byte, REJECT values
  ≥252 (redraw — avoids modulo bias), then byte mod 3 = fable's
  position (0/1/2). Difficulty still logged pre-draw per
  dispatch. Everything else unchanged (blinding, Fable for
  design/specs/reviews/rulings, record-at-merge, per-phase
  recording discipline). **Transition**: blocks in flight at the
  amendment complete as v2 pairs (LIB-2's opus remainder = G1;
  the M7 series' current block likewise); triples start at each
  series' next block.
- **Dual-reviewer sampling (Evan + orchestrator, 2026-08-08, from
  the readout's top recommendation): every 3rd merged
  implementation row gets a SECOND independent blinded reviewer.**
  Purpose: estimate reviewer variance so it can be divided out of
  the arm contrast (finding counts = review intensity × quality
  is the readout's binding measurement problem). Rules: the
  sampling rule is EVENLY BY RUN and arm-independent —
  deterministic every-3rd merged row across both orchestrators'
  series combined, counted from this amendment — so the
  measurement process stays identically distributed across arms
  (an arm-dependent rate would hand the arms systematically
  different review machinery, re-creating the confound). The
  second reviewer: identical brief and claims-to-falsify, own
  lane, NO access to the first review or its report; both reviews
  recorded in the row (both rubric triples, both finding counts,
  labeled R1/R2 in dispatch order); the fix pass runs off the
  orchestrator-adjudicated UNION of findings. Cost: ~one extra
  review per 3 units, accepted.
- **Record-at-merge (adopted post-M5 per the readout's
  recommendation):** the row is recorded AT MERGE, not at
  next-touch; a missing rubric or silent-deviation count is a merge
  blocker for the row.
- Reviewers are BLINDED: implementer model is never named in
  review prompts or in anything the reviewer reads (reports must
  not state it either — orchestrator checks before handing the
  implementation report to a reviewer). The arm is recorded ONLY
  here until the review + fix pass conclude.
- Every review includes a CODE QUALITY REPORT with a fixed rubric,
  identical across reviews (in addition to the usual findings):
  - counts: MAJOR / MINOR / NOTE findings; spec deviations
    (reported vs silent — silent ones counted separately and
    weighted worst)
  - ratings 1–5 with one line of evidence each: idiom/structure,
    test quality (do the tests pin the real contract?), doc/comment
    honesty
- Per-row objective companions: pre-dispatch difficulty guess
  (S/M/L, logged BEFORE the flip), fix-pass size, battery outcome,
  subagent tokens, wall-clock.
- **Recording discipline (adopted 2026-08-08 from the
  ev/ab-bayes-analysis readout's data-quality findings):**
  - **Tokens are recorded PER PHASE — impl / fix / review — as
    three separate figures, at merge, for EVERY row.** A bare lump
    sum conflates phases and cannot enter the cost models (the
    analysis could use only 24 of 51 v2 rows for impl cost; "the
    log already recommends this and then stopped doing it").
  - Wall-clock likewise per phase, with gaps ANNOTATED (crash /
    outage / usage-limit / design-round), so contamination is a
    codable covariate, not folklore.
  - The findings cell carries at least one line of PROSE per
    MAJOR, never a bare count (9 of 40 MAJORs were unclassifiable
    for severity by the blinded coder).
  - "Silent" in the silent-devs column means a silent SPEC
    DEVIATION only — never runtime-undetected corruption (row
    20's overload); disambiguate in the cell if both occur.
  - Record WHO executed the fix pass (implementer-inherited vs
    orchestrator-applied) — inconsistent execution contaminates
    the fix-pass proxies.
- Small-n caveat, stated up front: this yields a suggestive
  comparison, not significance. Read stratified by difficulty.

| # | date | task | difficulty (pre-flip) | arm | review findings (MAJ/MIN/NOTE) | silent devs | idiom | tests | docs | fix-pass size | battery | tokens | wall-clock |
|---|------|------|----------------------|-----|-------------------------------|-------------|-------|-------|------|---------------|---------|--------|------------|
| 1 | 2026-07-25 | #93 join-stage seam-region anchors | M | fable (draw 197) | 2/1/2 (both MAJ = claim-level; one credited a main bug the fix already fixed) | 0 | 4 | 4 | 4 | moderate (doc corrections + 2 adopted pins + 2 minors) | 145 suites: 1263/1263/1408, all pin families green | ~1.15M (incl. 2 crash resumes) | ~19h wall incl. crash gap (~5h active) |
| 2 | 2026-07-25 | #99 tour ε-panic | S | fable (draw 220) | 0/1/3 | 0 | 5 | 4 | 5 | 1 line (orchestrator-applied) | tour 3/3 ε rows; zero kernel diff | 58k | 6.4 min |
| 3 | 2026-07-25 | M4 PR 6 persistence | L | fable (draw 221) | 2/2/3 + 1 delta-MAJ | 0 (5 reported) | 5 | 4 | 5 | substantial (token retype + strict maps + goldens + tangent_joints + save symmetry sweep) | 156 suites: 1327/1325/1472 + persistence rows ×3ε | ~1.49M (incl. crash resume) | ~26h wall incl. crash gap |
| 4 | 2026-07-25 | #101 declared tangency | M | fable (draw 218) | 1/1/3 | 1 (falsified doc claim) | 5 | 4 | 4 | moderate (fillet Result + fit predicate + 6 pins) | 147 suites: 1283/1283/1429 + eps 3/3 | ~880k | ~9h wall (incl. crash gap) |
| 5 | 2026-07-26 | #106 depth-2 nested-island coverage | M | OPUS (block-1 forced slot) | 0/0/4 | 0 | 4 | 5 | 5 | NONE (NOTEs banked for 8a latency data) | 1265/1265/1411; fresh probes: main refuses, branch exact 8.25 | 134k | ~1h |
| 6 | 2026-07-26 | interval transcendentals crate | L | fable (block-1 remainder) | — | — | — | — | — | — | — | — | — |
| 7 | 2026-07-26 | A×Z render scene | S | fable (block 2 draw: fable,opus) | 1/1/2 | 0 | 4 | 4 | 4 | small (fallback fix + two-sided pin + narration) | tour+pins green, eps 3/3, fallback 19/19 | ~263k | ~3h |
| 8 | 2026-07-26 | #111 CDT needle triangle | M | OPUS (block-2 remainder) | 0/2/3 (MINs report-level) | 0 | 5 | 5 | 5 | tiny (decimal slip + comment + coordinated pin flip) | 158 suites 1335/0; tour+eps 3/3; admesh external gate | ~356k | ~8h wall (incl. limit gap) |
| 9 | 2026-07-26 | M4 PR 8a corpus+latency | L | OPUS (block 3 draw: opus,fable) | 1/3/3 (MAJ = designed promotion) | 0 | 5 | 5 | 4 | moderate (promotion + exhaustive kinds + baseline refresh; 1 finding DISPUTED w/ evidence, upheld) | 1333/1333/1482 + corpus/persistence/latency rows | ~573k | ~26h wall (incl. limit gap) |
| 10 | 2026-07-27 | M4 PR 8b K-lint + pickups | M | fable (block-3 remainder) | 0/2/5 | 0 | (in report) | (in report) | (in report) | light (lint_csv door + accounting + #120 golden regen) | 1343/0 + 17-row matrix green; planted-fragility catch 175 flags | ~640k | ~11h wall |
| 11 | 2026-07-27 | M5 PR 1 interval-crate adoption | M (logged pre-draw) | OPUS (block-4 draw: byte 63 → opus,fable) | 0/3/2 | 1 (stale-claims sweep left 6 live-rustdoc inari mentions) | 5 | 4 | 4 | moderate (3 MINs + CI row + computable deletion + 3 suite adoptions; in flight) | 1343/0 ×3ε + 1498/0 interval ×3ε; 17.5M-case reviewer fuzz clean | ~277k impl (+fix tbd) | ~8h impl wall |
| 12 | 2026-07-27 | M5 PR 3 NURBS substrate part 1 | L (logged pre-assignment) | fable (block-4 remainder) | 0/2/5 | 0 | 5 | 4 | 3 | light (2 doc MINs + wording NOTEs + 21 test adoptions) | 1387/0 + 1550/0 interval (post-fix); all 21 reviewer attacks held | ~465k impl + ~530k fix | ~11h impl + ~0.5h fix wall |
| 13 | 2026-07-28 | M5 PR 2 C9 interval ring | M (logged pre-draw) | OPUS (block-5 draw: byte 9 → opus,fable) | 0/2/2 | 0 | 5 | 5 | 4 | light (merged #130) | 9.7M exact fuzz 0 violations; ~3M differential max 1 step; sign clamp + zero annihilator proven as ℝ-facts | (in log) | (in log) |
| 14 | 2026-07-28 | M5 PR 4 projection+fitting+LSQ | L (logged pre-assignment) | fable (block-5 remainder) | 0/2/5 | 0 substantive | 5 | 4 | 4 | moderate, in flight (binom_row C(55,26) exactness fix, curvo hermeticity note, #126(a), 4 reviewer-test adoptions) | 1440/0 + 1615/0 interval (pre-fix); direct bound survived ~1M-sample falsification at ratio 1.0000 | (fix tbd) | (in log) |
| 15 | 2026-07-28 | M5 PR 8 BVH crate + sweep wiring | M (logged pre-draw) | fable (block-6 draw: byte 190 → fable,opus) | 2/6/4 (both MAJ = design forks, ruled by Evan) | 0 | 5 | 3 | 4 | substantial (mechanical items + ruling increment: N5 amendment, golden re-pin, L7 grep) | 1410/0 + 1586/0 interval; die −29% / corpus −21%; merged #135 | (in log) | (in log) |
| 16 | 2026-07-29 | CI dependency-closure filter (determinator/nextest eval) | S (logged pre-assignment) | OPUS (block-6 remainder) | n/a (CI infra — validated by synthetic-diff runs + hosted CI + Evan's PR review; no blinded lane) | 0 | — | — | — | none yet | filter validated on 12 synthetic diffs; -p plumbing proven live | ~94k | ~17min |
| 17 | 2026-07-29 | M5 S6 two-tolerance message sweep | S (logged pre-draw) | fable (block-7 draw: urandom coin 1 → fable,opus; S2 gets opus) | 1/2/3 (MAJ = dishonest Invalid payload at the exactly-on arm) | 0 | 4 | 4 | 5 | moderate (shape-only Display branch, Invalid-arm carrier, far-honest rephrase, 16 exactly-once pins, 3 probe suites adopted) | touched crates green both lanes; no-semantic-change proven by full-diff read; #138 gating | (in log) | interrupted twice (spend limit + 529); finisher pattern |
| 18 | 2026-07-29 | M5 S2 arc-leg fillet sugar | M (logged pre-draw at block-7 time) | OPUS (block-7 remainder) | 1/2/3 (MAJ = arc setback wrap; construction math fully verified) | 0 | 5 | 4 | 4 | moderate (signed fold bit-identical corner-side, newly-reachable refusal row, trio parity, cusp recourse, probes adopted) | 124/0 + 134/0 profile; 20k-corner review fuzz zero wrong circles; MERGED #137 (21 rows) | (in log) | interrupted once (529); finisher pattern; impl notably complete pre-review |
| 19 | 2026-07-30 | M5 PR 5 Ellipse + C5 dispatch table | L (logged pre-draw) | fable (block-8 draw: urandom coin 1 → fable,opus; PR 6 gets opus) | REJECT→APPROVE: 3/3/3 (MAJ-1 = even-crossing silent one-sided split, on the PR's own corpus geometry; MAJ-2 = D9 std trig; MAJ-3 = untested split trileans); geometry fully held (500-config fuzz ≤5e-12) | 0 | 5 | 5 | 5 | heavy (root-based crossing lane exposing+fixing 2 further latent defects; seam-cut upgraded refusal→split; re-review APPROVE 5/5/5) | shape (i) corpus e2e; M2 bit-identity independently confirmed; #141 gating | (in log) | first REJECT of the project; fix pass deepened the unit |
| 20 | 2026-07-30 | M5 S1 REST-contact join lane | M (logged pre-draw) | fable (block-9 draw: coin 1 → fable,opus; remainder to next unit) | 1/2/3 (MAJ = silent corrupt STL via hole-creating merge role inversion — pre-existing machinery, newly reachable) | 0 | 4 | 4 | 4 | moderate+ (roles corrected via Newell winding + NEW tier-3 loop-role gate filling a documented deferral; 6 probes adopted) | crosslap tripwire retired at exact volume; root cause BETTER than the wire's own story (germ-meta, confirmed at merge-base); MERGED #140 | (in log) | zip is purely structural — no new numeric predicate |
| 21 | 2026-07-30 | M5 PR 6 certified pcurve storage | M (logged at block-8 draw time) | OPUS (block-8 remainder) | 0/3/3 (best MIN = snap-to-family ε-shell falsifying the stored envelope on the attach path) | 0 (5 reported) | 5 | 4 | 4 | moderate (snap slack provably zero on minted caches + O(ε)-tightness pin + trim-window doc + max_residual split + seam probes adopted) | MERGED #144 18/18; found + independently confirmed the pre-existing PR 5 chord_spec complement-arc defect at merge-base | (in log) | (in log) |
| 22 | 2026-07-30 | M5 S7 CI/docs hygiene (ε-row retirement + cache-key audit) | S (logged pre-assignment) | OPUS (block-9 remainder) | 0/0/2 (lightweight review per spec §5) | 0 (4 reported) | — | — | — | none (one NOTE fixed on-branch by orchestrator) | MERGED #142 18/18 — its own gate demonstrated the 21→18-row battery; DEFAULT_EPS=1e-9 no-coverage-lost finding independently confirmed | (in log) | (in log) |
| 23 | 2026-07-31 | M5 S8 nearest-corner fillet selection ladder | S (logged first) | fable (block-10 draw) | 0/3/3 (MINs doc-level; math STRENGTHENED in review — mixed enclosing/non-enclosing impossibility proved) | 0 (3 reported) | 5 | 4 | 4 | light, doc-level (honest cross-lane wording + ulp-perturbed determinism rows both lanes + line×arc mirror proof + probe adoptions) | MERGED #143 18/18; 27M impl + ~160k reviewer fuzz, zero dominance violations; 3 constructor cross-checks agree | (in log) | (in log) |
| 24 | 2026-07-31 | M5 S9 chord_spec azimuth-window repair | S (logged pre-assignment) | OPUS (block-10 remainder) | 0/3/3, no re-review (MIN-1 = new definite arms missed the two-tolerance shape) | 0 | 4 | 5 | 4 | moderate (two-tolerance definite arms + true centre-reduction bound + interval belly row + short-circuit metering + reviewer probe verbatim) | MERGED #145 18/18 with MERGE PRIORITY; member-2 silent wrong body independently confirmed at merge-base 5fab705 | (in log) | (in log) |
| 25 | 2026-07-31 | M5 PR 7 SSI (march + three-limb certificate) | L (logged first) | OPUS (block-11 draw; FABLE remainder owed to the next unit) | 2/6/— (M1 = powf step rule + jet sin_cos fork; M2 ruled ACCEPT-AND-BANK → PR 7b), rubric 4/4/4 | 0 (5 reported) | 4 | 4 | 4 | substantial (9 items) + ε-fix redirect after gate RED (multi-ε battery caught test-side 1e-9 hardcoding; SSI_MAX_FIT_SAMPLES typed kernel budget) | MERGED #146 18/18; local 21/21 × (1e-6/1e-9/1e-12/interval); 8000-matrix independent SVD differential clean; core held under adversarial re-derivation | (in log) | (in log) |
| 26 | 2026-07-31 | M5 PR 7b tensor Bernstein compose + plane×NURBS retirement (EXIT-GATING) | M (logged pre-assignment) | fable (block-12 slot 1; draw byte 172 → fable,opus) | 0/4/2, rubric 5/4/4 (review REFUTED the "geometry-capped" claim with measurement; ~1.6M falsification samples zero bound-below-truth; max forced looseness 108×) | 1 (center-shift skipped, "no center to lose" ring-false — 6 orders lost at 1e6 m; review-caught, fix-pass implemented to the representation floor 1.225e-9) | 5 | 4 | 4 | moderate (center-shift impl + non-monotone rewording + recourse pin + breadth sentences + 10 probe rows adopted) | MERGED #149 18/18; shape (iii) substrate GREEN all lanes (exit gate); bound 6.5 orders tighter, within 1% of truth; one waiter-park stall (sweep-revived) + outage #9 resume en route | ~281k impl + ~323k fix | ~4.3h impl + ~4h fix wall (incl. outage gap) |
| 27 | 2026-08-01 | M5 demo unit: rocker arc-fillet stop + staged tiltedcut | S (logged pre-dispatch) | OPUS (block-12 remainder) | 0/1/2 (APPROVE, no fix pass; every narration claim survived executed check; S8 pick proven a rule via perturbation) | 0 (6 reported/assessed; render incident contained + verified exact) | 5 | 4 | 5 | one-liner (orchestrator-applied per S7 precedent: retire note names the K-sweep join) | MERGED #150 18/18; tour 3/3 ×3ε; 4783 K-probe samples ×3ε identical; one waiter-park stall (report nudge) | ~178k impl + ~123k review | ~2.5h impl wall |
| 28 | 2026-08-01 | M5 PR 9 curved booleans + tangency regime | L (logged first) | fable (block-11 remainder) | 3/6/5 vs impl (3 MAJ incl. 2 silent: union-only scope, red battery; core geometry HELD under tube-threading/zip attacks), rubric 5/3/4 | 2 silent (of 11+2 reported) | 5 | 3 | 4 | heavy: 7 items + arc-facing WENT LIVE (2-arc disc unions) + Interval root cause (infinity seeds + branch-cut-free cone) + idealized-sweep 0/0 clearance fix; then triple gate-red (2 lint rounds on adopted probes, 4th interval-square occurrence caught by the BVH Interval differential) | MERGED #152 18/18 MERGE PRIORITY — main-is-wrong du_of_rims fixed (0.6545→0.7854 silent at base, 2 public calls); review's merge-base witness led the writeup | ~742k impl+fix (in-lane) | ~1.5 days wall incl. outage #10 |
| 29 | 2026-08-01 | M5 PR 10 sweeps/lofts + schema v2 clean break | L (judged pre-draw; logged post-draw — ordering slip recorded in log) | OPUS (block-13 draw byte 82 → opus,fable) | 1/4/4, rubric 5/4/4 (MAJ = dead Sweep recipe lane misreported as a capability; math held: closed-form loft between sections, Eq 10.8 exact match, 8/8 header attacks) | 2 effectively silent (node-layer sweep total refusal; missing size note) | 5 | 4 | 4 | moderate+ (honest lane collapse + OpenClosedMixed wired/Escalated deleted + tangent-claim truth + size note + RaggedRows + 22 probe rows adopted; post-merge frontier-message truth pass unprompted) | MERGED #151 18/18; schema v2 clean break exactly per ratified mechanics; dev-2 coordination claim FALSIFIED by reviewer scratch-merge (assembly → 9c item 6) | ~325k impl + ~396k fix | ~2 days wall incl. outage #10 |
| 30 | 2026-08-01 | interval-square retirement + CI tripwire | S (logged pre-dispatch) | fable (block-13 remainder) | 0/0/3 APPROVE, no fix pass (5M-sample regroup probe: 2 ulp max, 0 flips; .sqr() tighten-only proven; allowlist audited line-by-line) | 0 (10 non-bit-identical conversions self-reported as judgment calls, all upheld) | 5 | 4 | 5 | none (PR opened by orchestrator on APPROVE) | MERGED #153 18/18; 55+6 sites converted, 2 false positives restructured, 54 allowlisted; one waiter-park + battery-scope correction (Evan live) en route | ~128k impl + ~143k review | ~6h wall |
| 31 | 2026-08-01 | M5 PR 9c banked completions (sphere doors + blocker map) | L (logged pre-draw) | OPUS (block-14 draw byte 124 → opus,fable) | 1 MAJ (proof-text scope: sphere r² parity leg refuted) / 2 MIN / 2 NOTE; group-arm design verified sound; both judgment calls ENDORSED | 0 (6 numbered, all with executed blockers) | 4 | 4 | 4 | moderate (proof scoped per-kind + option (d) pinned ×3; stale promise rewrites; both NOTE rows taken) | MERGED #154 18/18; 1 of 6 items landed by design — the five executed blockers re-planned the milestone (assembly→post-PR 11; Fitted→SSI lift; revert→sense ratification) | (in log) | (in log) |
| 32 | 2026-08-02 | M5 PR 11 tessellation + certified quadrature (demo moment) | L (logged pre-dispatch) | fable (block-14 remainder) | 0/2/5 APPROVE, rubric 4/4/5 (falsification: zero bound-below-truth; star find = the accidentally-load-bearing factor-2) | 0 (5 reported; dev 1 superseded mid-flight by Evan's static-split ruling, implemented cleanly) | 4 | 4 | 5 | moderate (factor-2 accounting + corner-scan pin; SelfTouchingTrimLoop arm; provenance field — which caught the machine-state drift; probe adoptions) + one multi-ε gate red (band-relative caps, FitSampleBudget-precedent arm) | MERGED #157 18/18 — tiltedcut renders, montage refreshed, staged machinery deleted; T6: CDT does not dominate | (in log) | (in log) |
| 33 | 2026-08-02 | M5 S10 face orientation sense | M (logged pre-draw) | OPUS (block-15 draw byte 119 → opus,fable) | 0 code-MAJ / 3 MIN / 2 NOTE, rubric 5/4/5 (A/B discipline held adversarially; spec-premise REFUTED with live-defect proof — e2e pellet-swallow found by review) | 0 (2 reported incl. the premise refutation as MAJOR-returned) | 5 | 4 | 5 | light (3 doc MINs + probe adoptions) + one gate red (PR 9c message pin caught F1-scoping erosion — fixed message, not pin) | MERGED #155 18/18; the enabling infrastructure for S11's fix | (in log) | (in log) |
| 34 | 2026-08-02 | M5 S11 concave/inward walls sense:false | M (spec-time) | fable (block-15 remainder) | 0/2/3 APPROVE, rubric 5/5/4 (criterion survived 6 adversarial constructions unmodified; nappe algebra independently confirmed) | 0 (5 reported incl. the widened revolve scope) | 5 | 5 | 4 | light (mef hazard banked + probes adopted) | MERGED #156 18/18 MERGE PRIORITY — pellet-swallow + washer-bore containment defects DEAD | (in log) | (in log) |
| 35 | 2026-08-02 | M5 S12 curved revert wiring + per-class ∖/∩ opening | M (logged pre-draw) | OPUS (block-16 draw byte 66 → opus,fable) | 0 new MAJ / 2 MIN / 3 NOTE APPROVE, rubric 5/5/4 (class boundary proven COMPLETE; reviewer implemented the rejected alternative — all pins green, rationale corrected) | 0 (2 reported incl. the fallback MAJOR-returned) | 5 | 5 | 4 | light (2 doc MINs + NURBS hazard scoping + 4 probes) | MERGED #158 18/18 — curved ∖/∩ LIVE on Plane/Cylinder; 3rd main-is-wrong found (vertex-probe fallback, ∪ sphere-class, pinned) | (in log) | (in log) |

PROTOCOL CHANGE (2026-07-25, Evan approved: "shuffled pairs makes
sense") after four consecutive fable draws (197, 220, 221, 218;
p = 1/16): remaining dispatches use BLOCKED randomization —
opus/fable pairs, order shuffled per block via /dev/urandom.
Transparency note: block 1's order is FORCED (opus, fable) as an
explicit balance correction for the 4-0 fable start — the actual
draw came out (fable, opus) and was overridden, recorded here
rather than silently; random order resumes from block 2. Blinding
unchanged (reviewers never learn arms; difficulty still logged
pre-assignment).

(Reference rows, pre-experiment, both Fable, unblinded — context
only, not comparable: M4 PR 4 impl (L): 2 MAJ / 2 MIN / 6 NOTE, 0
silent devs, fix pass P1–P4, all-green battery. Demo refresh (M):
0 MAJ / 5 MIN / 4 NOTE, 0 silent devs, light fix pass, all-green.
M4 PR 5 impl (L, first rubric-scored reference): 1 MAJ / 3 MIN /
3 NOTE, 1 SILENT deviation (skip-lane tier-3 posture unstated),
idiom 4 / tests 4 / docs 4, substantial fix pass (F1 re-describe
machinery + plumbing + door tests), battery 1256/0 ×2 + interval
1400/0, ~1.44M impl+fix tokens.)

## Rows 36–40 (added at the M5 exit sweep, PR 14 — reconstructed from the M5-LOG narrative)

These five dispatches concluded and merged but never reached the
table; the log itself flagged the debt ("A/B rows 36-39 to the table
at next touch"). They are entered here from the narrative record.
**Columns the narrative never recorded are marked `—` rather than
guessed**; that is itself a finding (see the readout's honesty
section).

| # | date | task | difficulty (pre-flip) | arm | review findings (MAJ/MIN/NOTE) | silent devs | idiom | tests | docs | fix-pass size | battery | tokens | wall-clock |
|---|------|------|----------------------|-----|-------------------------------|-------------|-------|-------|------|---------------|---------|--------|------------|
| 36 | 2026-08-02 | M5 S13 die-pips enablers (containment-fallback re-cut + plane×sphere germ arm) | M (logged pre-assignment) | fable (block-16 remainder) | 1/—/— (MAJ = new, executed: the multi-normal escape hole; dev 2 confirmed as main-is-wrong #4) | — | — | — | — | F1–F5; all nine reviewer probes adopted 9/9 | — | — | — |
| 37 | 2026-08-02 | M5 PR 13 curved STEP subset | M (logged pre-draw) | OPUS (block-17 draw byte 20 → opus,fable) | 0/2/3 APPROVE | 0 | 5 | 4.5 | 5 | light | — | — | — |
| 38 | 2026-08-03 | demo dual-montage (kernel + FreeCAD/OCC lanes) | S (logged pre-assignment) | fable (block-17 remainder) | 0/1-nit APPROVE | — | — | — | — | none (nit) | — | — | — |
| 39 | 2026-08-03 | M5 S4 save/load shared-validator consolidation | S (logged pre-draw) | fable (block-18 draw byte 131 → fable,opus) | 0/0 APPROVE | 0 | 5 | 4 | 5 | none (one note orchestrator-applied) | — | — | — |
| 40 | 2026-08-03 | M5 PR 12 constant-radius fillets + the die | L (logged pre-assignment) | OPUS (block-18 remainder) | 1/3/5 APPROVE w/ fix pass (MAJ = octant e0 pick: tier-3 lost on non-square prisms, die unaffected) | 1 (scope gap: Band-4 rows) | — | — | — | F1–F6 + two gate-red rounds | — | — | — |

| 41 | 2026-08-03 | CI build-once/shard (compile per MODE, nextest archives) | M (logged pre-draw) | fable (block-19 draw byte 227) | n/a — CI infra, no blinded lane | 0 | — | — | — | none | #167 27/27 green; wall 16m57s → 15m47s, billed ~64.5 → ~56.6 min, 4 → 2 workspace builds | — | — |
| 42 | 2026-08-03 | M5 PR 14 exit sweep (K snapshot, DESIGN/envelope, exit walk, A/B readout) | M (logged pre-assignment) | OPUS (block-19 remainder) | n/a — docs/telemetry, no blinded lane (orchestrator reviews the walk personally) | 0 | — | — | — | — | docs-only; fmt-all --check clean | — | — |

**Rows 41 and 42 are NUMBERED but EXCLUDED from every comparison in
the readout below.** Both are no-blinded-lane classes — CI
infrastructure and docs/telemetry respectively — so neither produced
a blinded review carrying the rubric the experiment compares, and
neither has a MAJ/MIN/NOTE count that means the same thing as the
other rows'. They are numbered because `docs/archive/M5-LOG.md` already
refers to the CI-shard unit as "A/B row 41", and a table that
silently disagrees with the log about what a row number denotes is
worse than a table with two clearly-marked non-comparable rows.

**So: 42 dispatches, n = 40 for the comparison.** Every statistic
below is computed over rows 11-40.

## M5-close readout (2026-08-03, PR 14)

Scope: rows 11–40 are the M5 dispatches (rows 1–10 were M4; the
reference rows in the footer are pre-experiment). Thirty M5 rows,
plus the two unnumbered no-blinded-lane units above.

**Arm balance.** M5 rows 11-40: **fable 15, opus 15.** The blocked
randomization held — every block after M4's block 1 drew its order
from `/dev/urandom`, and the pairing landed the milestone exactly
even without any further override.

**Stratified by pre-logged difficulty** (difficulty was logged before
the flip or before assignment in every M5 row; the one ordering slip,
row 29, is recorded in its own cell):

| difficulty | fable rows | opus rows | fable MAJ | opus MAJ | fable silent | opus silent |
|---|---|---|--:|--:|--:|--:|
| **L** (9) | 12, 14, 19, 28, 32 | 25, 29, 31, 40 | 6 | 5 | 2 | 3 |
| **M** (12) | 15, 20, 26, 34, 36 | 11, 13, 18, 21, 33, 35, 37 | 4 | 1 | 1 (+1 unrecorded, row 36) | 1 |
| **S** (9) | 17, 23, 30, 38, 39 | 16, 22, 24, 27 | 1 | 0 | 0 (+1 unrecorded, row 38) | 0 |
| **total** | 15 | 15 | **11** | **6** | **3 recorded, 2 unrecorded** | **4** |

Both arms are within one row of each other at every difficulty level
except M, where opus drew seven to fable's five.

**MAJOR findings — read the classifications, not the counts.** The
raw totals (fable 11, opus 6) are not a quality signal, because the
review record classifies a large share of them as something other
than implementation defects:

- **Design forks ruled by Evan, not defects**: row 15's two MAJs.
- **Ruled ACCEPT-AND-BANK** (the finding became a scheduled unit,
  PR 7b): row 25's M2.
- **Claim- or proof-text scope, not code**: row 31's MAJ; row 33's
  was a premise refutation returned as a MAJOR against the *spec*.
- **Real defects outside the unit's own acceptance target**: row
  40's octant `e0` pick (tier-3 lost on non-square prisms; the die,
  which the unit shipped, is unaffected).
- **Real, consequential, on the unit's own geometry**: row 19's
  MAJ-1 — an even-crossing silent one-sided split — the project's
  only REJECT. Its fix pass exposed and fixed two further latent
  defects and re-reviewed at APPROVE 5/5/5. Row 28's three (two
  silent) and row 20's one (a silent corrupt STL via a hole-creating
  merge role inversion) are the other members of this class.

Counting only that last class, the milestone's genuinely
consequential implementation MAJORs are rows 19, 20, 28 (fable) and
row 40 (opus) — four across thirty dispatches, and present on both
arms.

**Silent deviations** — the metric the protocol weights worst, and
the one where the arms are closest to indistinguishable. M5 total:
**fable 3** (row 26's center-shift ring-fallacy; row 28's two),
**opus 4** (row 11's stale-claims sweep leaving live rustdoc inari
mentions; row 29's two node-layer sweeps; row 40's Band-4 scope
gap). Two fable rows (36, 38) have no silent-deviation datum
recorded at all, so fable's true count is 3-5. Every other M5 row
recorded 0 silent alongside a nonzero count of *reported*
deviations — the reporting discipline itself held well on both arms,
which is the outcome the protocol most wanted.

**Fix-pass size distribution.** Rows 36, 38 and 40 were described
narratively and never classified; they are counted as unclassified
rather than folded into a bucket.

| size | fable | opus |
|---|---|---|
| none | 30, 39 | 22, 27 |
| light / tiny | 23, 34 | 24, 33, 35, 37 |
| moderate | 14, 17, 20, 26, 32 | 11, 13, 18, 21, 31 |
| substantial / heavy | 15, 19, 28 | 25, 29 |
| unclassified | 36, 38 | 40 |

Several cells carry a qualified size in the row itself
("moderate+", "light + one gate red", "moderate, in flight");
collapsing those into buckets loses information the row cells keep,
and the row cells are authoritative. Read directionally: the
distributions overlap heavily, with the heavy tail populated by both
arms and driven by unit scope rather than arm.

**What the milestone shows, honestly.**

1. **No arm-level quality difference is visible at this n.** Both
   arms produced clean rows and both produced the milestone's
   heaviest fix passes. Both arms carried silent deviations (fable 3
   recorded plus 2 rows with no datum, opus 4) — the metric the
   protocol weights worst, and it does not separate them. Both arms had a row where the review found a real,
   consequential defect that shipping would have carried (row 19
   fable, row 40 opus). The M4-close reading — "no evidence Opus
   implementation is worse at this scale; suggestive that it's
   comparable" — is unchanged by thirty more rows, and it is now
   supported by a difficulty-stratified sample rather than a skewed
   one.
2. **The confounds have NOT gone away and are not small.** Reviewer
   variance is still unmeasured — the same orchestrator-model
   reviewed both arms, and review depth demonstrably varied across
   the milestone (row 19's review found three MAJORs on geometry that
   three earlier reviews of comparable units did not probe as hard).
   Difficulty labels are one orchestrator's pre-flip guess, not a
   calibrated scale. Unit scope varied by more than an order of
   magnitude within the same difficulty letter. Fix passes were
   sometimes run by the implementer's own agent and sometimes
   orchestrator-applied.
3. **No significance is claimed, and none is available.** n = 40 with
   a binary arm, an unblinded orchestrator, a subjective outcome
   scale, and multiple uncontrolled confounds does not support a
   significance claim, and no test is reported here. The honest
   summary is the same shape as M4's: *the experiment has produced no
   evidence that either model is worse at this work, and the sample is
   now large enough that a large effect would probably have shown.* A
   small effect would not have, and this design cannot find one.
   Arm balance (15/15) and difficulty balance are the two things this
   milestone did materially improve over M4's 4-0 opening skew.

**Data-quality findings this readout is obliged to state.**

- **The table was five rows stale at milestone close** and rows
  36–40 had to be reconstructed from prose. The reconstruction is
  faithful but lossy — see the `—` cells.
- **The rubric (idiom/tests/docs) is missing for rows 36, 38, and
  40** and was never recorded. Row 40 is an L-difficulty row, so the
  most informative single rubric of the milestone's end is absent.
- **Tokens and wall-clock are absent for every row from 13 onward**
  ("(in log)" was written in place of a figure and the figure was
  never carried across). The protocol lists them as per-row objective
  companions; in practice the experiment collected them for twelve
  rows and then stopped. Any future cost comparison between arms is
  therefore not available from this log.
- **Two rows (36, 38) lack a silent-deviation count**, the
  protocol's most heavily weighted metric.
- Recommendation for the next milestone, if the experiment
  continues: record the row AT MERGE rather than at next-touch, and
  treat a missing rubric or silent-dev count as a merge blocker for
  the row — the cheap discipline that would have prevented every gap
  above.

## Post-M5 rows (M6/M7, from 2026-08-04 — recorded at merge; NOT covered by the M5-close readout above)

| # | date | task | difficulty (pre-flip) | arm | review findings (MAJ/MIN/NOTE) | silent devs | idiom | tests | docs | fix-pass size | battery | tokens | wall-clock |
|---|------|------|----------------------|-----|-------------------------------|-------------|-------|-------|------|---------------|---------|--------|------------|
| M6-1 | 2026-08-04 | composed die via in-place surgery + circle-clearance rider | M (logged pre-draw) | fable (block-20 draw byte 161 = fable,opus) | PASS 0 MAJ / 2 MIN / 3 NOTE, rubric 5/4.5/5 (volume confirmed 3 independent ways incl. 4e8 MC; rider falsifier clean over 3000 pairs) | 0 (4 numbered, all verified honest) | 5 | 4.5 | 5 | light (2 MINs + probe adoptions, via fresh finisher — transcript lost) | composed die: every verb on ONE body; FreeCAD to 1e-6 mm³; strategy divergence retired; dev 1 = corpus-inexpressibility discovery → M6 unit 5 | (in log) | (in log) |
| M6-2 | 2026-08-04 | SSI generic-T lift + Pcurve::Fitted + non-vacuous fitted cache at rest | M (logged pre-draw) | OPUS (block-21 draw byte 59 = opus,fable; fable remainder → unit 3) | APPROVE-WITH-FIXES 1/4/5, rubric 4/4/4 (MAJ = one unused import going CI-red, SKIPPING the hosted interval matrix; deep attacks all held: OnLocusHull adjudicated honest by the walk row's own letter, f64-Newton dev sound, reviewer's second-species corruption caught by loop continuity) | 0 (9 numbered; clause-by-clause spec diff; separate owner-requested design audit: 7/8 RIGHT, 1 RIGHT-BUT-MISDOCUMENTED, 1 scope gap — Evan's "felt off" instinct matched exactly the two non-RIGHT verdicts) | 5 | 4 | 4 | moderate+ (8 items + audit doc-fixes + fixture re-anchored to the kernel's own split_at carrier with natural [a,b] domains + one self-caught cross-scalar overclaim stop-and-reported and weakened honestly) | MERGED #176 27/27 incl. interval shards w/ geom-brep confirmed; walk row 2 NON-VACUOUS (full C2 at rest, planted + reviewer corruptions rejected, Interval enclosure row); UnsupportedCarrier retired (S9 flip); 3 review probes adopted | ~437k impl + ~467k fix | ~5.4h impl + ~8.7h fix wall (incl. CI waits, under CPU pin) |
| M7-1 | 2026-08-04 | step-import crate: Part-21 parser + rotation-system Euler assembly + D7 adoption ladder, own-corpus round-trip | L (logged pre-draw) | fable (block M7-1 draw byte 177 = fable,opus; opus remainder → next eligible) | APPROVE-WITH-FIXES 1/3/5, rubric 5/4/4 (MAJ = silent-unit class: CONVERSION_BASED_UNIT length context imported silently as metres; headline 14/14 first-re-export byte-identity CONFIRMED and proven un-laundered; adoption ladder held all 4 planted corruption classes with honest structured refusals; fixed point held on reordered/renumbered files) | 1 (units — the MAJ; fixed by-resolution) | 5 | 4 | 4 | moderate (by-resolution unit/uncertainty checks + inch-file test; full 7k-cut truncation sweep; 3 silent drops → typed Structure refusals; string-body refusal arm; 6.00-ulp correction; 19 review probes adopted by merge with authorship kept, incl. re-anchoring the reviewer's VACUOUS unit probe — its #93 substitution never matched cube's #155) | MERGED #183 27/27 hosted; 28/28 crate suite (9 acceptance + 19 probes); deviation 1 (sidecar kernel-census overrides) adjudicated honest; fenced findings → #184 | ~441k impl + ~181k review + ~37k fix (resumed segment) | ~1.9h impl + ~0.6h review + ~0.5h fix active |
| M7-2 | 2026-08-04 | FreeCAD foreign-corpus import: mm units, outerness inference, base cones, vertex-loop sphere, structure roots | L (logged pre-assignment) | OPUS (block M7-1 remainder) | APPROVE-WITH-FIXES 2/4/2, rubric 4/4/4 (both MAJ on adversarial inputs, not the corpus: torus normalization = SYMPTOM-FLIP laundering an inside-out torus, half of it a kernel props sense_sign gap → #184; coincident-locus rung certified surfaces not the curve; chart-inverse fuzz clean at 1.2e-15; the A3 K-landing measurement verified bit-exact) | 0 (11 reported, all check out) | 4 | 4 | 4 | moderate+ (torus winding derived from cyclic loop order — mid-quadrant chart sampling; inverted/undecidable tori refuse typed naming the kernel gap; ISO's .F./CW legal encoding imports correctly; curve certification through the shared door; ceiling-skips became sub-tolerance assertions with the declining-vs-answering-falsely distinction; probes adopted by merge) | MERGED #189 19/6-skip/0; battery green at 5 ε values; oracle 13/13; one earlier hosted gate-red (ε=1e-6 pcurve certify on the mm corpus) root-caused to scale-vs-absolute-ε, fixed by derived CORPUS_EPS_CEILING=1e-8 without widening any gate; FIRST IN-BAND K LANDING found and reported (#89) | ~422k impl + ~169k review + ~33k fix | ~2.6h impl (incl. gate-red loop) + ~0.8h review + ~0.6h fix |
| M7-4 | 2026-08-05 | wild corpus: 4-vein license-verified fixtures, unit/vector/string dialect unlocks, rigid assemblies, no-panic contract | M (logged pre-draw) | OPUS (block M7-2 draw byte 114 = opus,fable; fable remainder → next M7-eligible dispatch (M6-3 completion rides M6 block-21)) | APPROVE-WITH-FIXES 0/2/3, rubric 5/4/4 (every headline independently confirmed: oracle volumes re-derived digit-for-digit, 5 mirror-smuggling attempts defeated, 620+ mutations no-panic, ε_in floor proven NOT a widened gate, duplicate-solid latency confirmed at merge-base; MIN-1 was an INHERITED knot-multiplicity SIGABRT) | 0 (7 reported, all upheld) | 5 | 4 | 4 | light+ (false comment truthed; schema-derived knot budget — n+d+1 checked pre-allocation, probe_knot un-ignored with slowness ceiling + validity control; probes adopted by merge with the multi-ε gap self-caught and fixed as honest weaker claims) | MERGED #193 27/0/0 full matrix (widened filter ran freecad/admesh/persistence/corpus/interval rows); 7/13 wild files import first-class (oracle 1e-16..1e-13 rel), 6 refuse typed; RingOnCurvedFace refusal ruled+concurred; band-seam unit banked; 98 tests 0 ignored | ~372k impl + ~167k review + 2 fix resumes (per-segment figures unreliable) | ~1.8h impl + ~1.2h review + ~1.1h fixes |
| RIM-DIM | 2026-08-05 | du_of_rims dimensional-metering fix (RimLevel enum) + props/predicate dimensional audit (~120 rows, 8 inline comparand fixes) | M (logged pre-dispatch) | fable (block M7-2 remainder) | APPROVE-WITH-FIXES 1/2/5, rubric 5/4/4 (fix corrects real VERDICT FLIPS both directions — pre-fix silently grouped 50ε-separated rims and spuriously refused 0.5ε-coincident ones; MAJ = the unit's own twin pin not ε-row-honest, which EXECUTED deferred F4 into Band{1e-6,1e-5} on a hosted row; R6 executed the F5 linkage: the freecad ε=1e-7 cylinder refusal IS pcurve_chart_radial_moving, 2r² in-band) | 0 (7 reported, all honest) | 5 | 4 | 4 | moderate (three-outcome ε-row pin with F4's live signature; fix pass FALSIFIED the review's scale-invariance premise by measurement — the 1e-12 margins are F3's volume_backstop under a STALE predicate name, so F3 also corrupts K attribution: second executed retirement reason; probes adopted+ε-hardened; sphere pole note) | MERGED #197 all green; #89 landing RETIRED (a3 sweep delta exactly one line); banked dimensional unit grows to F3+F4+F5 + ceiling re-derivation, sequenced after M6-3 | ~290k impl + ~154k review + ~370k fix segment | ~3h impl + ~0.9h review + ~0.8h fix, all under the 20-45x pin |
| M6-3 | 2026-08-05 | loft/sweep body assembly: IsoCurve seams + iso-pcurve lane + tier-3 flips + exact NURBS flux + Leg E analytic-chart completion (walk row 4) + tube_along_arc rider | L (logged pre-assignment by the M6 orchestrator) | fable (M6 block-21 remainder; partial by the M6 session's implementer, completion + fix pass by a fresh fable implementer after the sole-orchestrator pickup) | APPROVE-WITH-FIXES 0/5/4, rubric 5/4/4 (whole-unit review: Leg E algebra RE-DERIVED BY HAND all correct; ceiling composition executed; MINORs all honesty/coverage class; quality seam between phases detectable-but-weak — all four false statements in completion scope, partial spotless) | 0 (9 reported, all executed blockers) | 5 | 4 | 4 | light (5 comment/coverage items + probe adoption by merge + one self-extended adjacent-row fix reported) | MERGED #192 27/27; WALK ROW 4 CLOSED (ball/cone/donut + die octants carry stored pcurves at rest); loft/sweep bodies live end-to-end (tour prints V=9±1e-13); F5 fixed in passing (ceiling 1e-8→1e-5); lily findings 11 + 56-ulp drift retired by the rider; six red jobs root-caused first | partial (M6 session, unrecorded) + ~642k completion + ~231k review + fix segment (figures unreliable across resumes) | ~7h completion + ~2h review + ~0.4h fix, all under the 20-45x pin |
| F3+F4 | 2026-08-05 | funnel-bypass retirement (volume backstop dual-arm) + ring-winding mean-width metering | M (logged pre-draw) | OPUS (block M7-3 draw byte 123 = opus,fable) | APPROVE-WITH-FIXES 1/3/4 (T1 silent-disable reproduced instrumented at merge-base — only 1 of 3 bound checks ran at mm scale; T3 full 23,389-sample byte-diff confirmed exactly-one-line K delta; MAJ = the metering's own weakening direction: a wrongly-kept 3mm cube on a 2m plate metered IN-BAND and passed — the assigned hide-behind-area attack landed) | 0 (2 declared; MIN-1's collision claim corrected as worse-than-unverified) | (in report) | (in report) | (in report) | moderate (dual-arm gate: sign-certain violation refuses dimension-free via the exact bit-hairline band, BOTH arms on the metered comparand so K stays dimensionally honest — verified scale-linear and RED-with-arm-removed; zero-perimeter arm declared; bypass claim scoped + editor-core F12 row added) | MERGED #200 26/0/1; the tree's audited family has no funnel bypass left; deviation 2 pinned not prose (silent skip cannot return unnoticed) | ~254k impl + ~113k review + ~312k fix | ~1h impl (post-outage) + ~25min review + ~1.2h fix |
| KERNEL | 2026-08-05 | KERNEL_* sidecar fields + live staleness row + step-import consumer swap (#184 design) | S (logged pre-draw) | fable (block M7-3 remainder) | APPROVE 0/1/4 (K2's pad-hiding-room attack answered in the unit's favor: planted +1000mm³ lie passes off-ε overlap rows but the default-ε byte pin catches it — composed hiding room ZERO; K3 corruption probe: old tolerance accepts, new catches at 1600×/8000× margins; MIN = the "4-8 orders tighter" claim measured wrong for 11/14 fixtures) | 0 (6 reported, all verified) | 5 | 4 | 4 | micro (2 prose corrections: measured-truth tolerance claim in body+comment; staleness-row doc scoped to solid sidecars; probes adopted by merge incl. loft_prism-refuses-typed) | MERGED #199 fully green; override table GONE; KERNEL_VOLUME_PAD_MM3 ε-discovery (enclosure midpoint moves with ambient ε — byte pin at declared ε + overlap rows elsewhere); fmt_real pub | ~279k impl + ~117k review + micro fix | ~2h impl + ~1.3h review |
| SKINFIT | 2026-08-05 | #207 fix: integral skin fit never synthesizes a rational wall (sweep_body's first successful caller) | S (logged pre-dispatch) | OPUS (block M7-4 remainder) | APPROVE 0/1/3, rubric 5/5/5 (every executed number reproduced exactly; the reviewer's own 17-station convergence run closed the bracket's hidden-bias headroom; stash-red 5/6 + bit-identity both ways; W2's three bitwise-conservation facts verified incl. denormal/1e300 probes) | 0 (5 reported; MIN = deviation-4 wording imprecision, orchestrator-corrected in the PR body) | 5 | 5 | 5 | none beyond the body edit | MERGED #210 27/27; source fix (ℝ³ lane — the denominator never computed, C6 bitwise structure selection); quarter-torus elbow = sweep_body's FIRST successful caller (Pappus bracket 3.8e-6 @9 stations, pad pinned separately); non-uniform lofts live (V=12 exact); uniform lane bitwise-unchanged two ways; Evan calibration note: a less-principled fix would have been acceptable | ~153k impl + ~108k review | ~2.5h impl + ~1.2h review |
| M7-3 | 2026-08-05 | NURBS-face import: both surface arms, surface_sig fix, IsoCurve rung, rim adoption, ARM B | M (logged pre-draw) | fable (block M7-4 draw byte 224 = fable,opus) | APPROVE-WITH-FIXES 3/2/3, rubric 4/4/3 (survived: sig injectivity under transposed-net/knot-swap/weights attacks, foreign-refit-seam refusal, dm1 re-anchor; MAJ-1 = Arm B's uncertified class — a different circle through the same endpoints laundered on rational walls; MAJ-2 det-0 frame; MAJ-3 the recurring vector×projection lint FP) | 0 (3 reported, all verified) | 4 | 4 | 3 | moderate (the rim residual gate with a REPORTED role inversion — wall boundary sampled against the closed-form circle distance + lever-armed angular containment killing the complement arc; honest-perpendicular line_frame with fail-loud backstop; named-binding lint fix; 3-token and payload-prose corrections; 14 probes adopted permanent) | MERGED #209 fully green; ARM B blessed by Evan then REPAIRED to verified-not-trusted (updated on-thread, blessing carried); SOLID_FIXTURES 15 with fixed point; M7 unit 3 CLOSED | ~277k impl + ~180k review + ~343k fix | ~2.5h impl + ~1.2h review + ~1h fix |
| FOLD | 2026-08-06 | corpus-widening fold: nonuniform_loft + swept_elbow (15→17), first non-default RTOL | S (logged pre-draw) | OPUS (block M7-5 draw byte 220 = fable,opus) | APPROVE-WITH-FIXES 1/2/2 (X2 settled with authority: the reviewer's own 24-pt Gauss integrator matched the KERNEL to 1.28e-12 and missed OCC by 1.9e-8 — the 1e-7 RTOL is honest, OCC is wrong; layered-row safety quantified at 71,000× over the kernel row's budget; MAJ = the builder doc stating the naive numbers the fixture refutes) | 0 (6 reported) | 4 | 5 | 3→5 post-fix | micro (doc-comment truth with the trap named; 5-ulp/5.93e-2 unit-slip corrections; whole-numeral token matching; probe adopted) | MERGED #212 26/1; elbow = sweep_body's first FIXTURE; NURBS-on-sweep import refused nowhere; trapezoid chosen over the easier prism BECAUSE the prism never exercises the #207 path | ~149k impl + ~164k review + ~180k fix | ~1.9h impl + ~1.8h review + ~0.2h fix |
| MONTAGE | 2026-08-06 | montage refresh: tube cell + count pins; 3 NURBS scenes blocked on mesh | S (logged pre-draw) | OPUS (block M7-6 draw byte 111 = opus,fable) | orchestrator-review class (demos unit): PARTIAL delivered honestly — cell 19 with an executed bit-exact intent assertion; the three NURBS scenes stopped at a genuine design boundary (placeholder cells would break the two-sheet contract) with all constructions WRITTEN and saved as a patch | 0 (the stop-and-report IS the discipline) | — | — | — | none (merged as delivered) | MERGED #215; clean-re-render verified twice; cell pin 19 with derivation; the block PROMOTED the mesh trimmed-NURBS lane (two consumers) — dispatched as the fable remainder | ~139k impl | ~0.5h |
| MIGRATE | 2026-08-06 | classify-seam migration: Length<T> by signature, ~351 door sites, the invariant lane | M (logged pre-draw) | fable (block M7-5 draw byte 220 = fable,opus) | APPROVE-WITH-FIXES 2/2/5 both MAJ silent (Y1's of-laundering attack LANDED — a unit-dot sine wrapped in of, with an executed scale-blindness probe; demos/tour broken at tip; byte identity independently reproduced 23394/23394; all 11 flagged sites verified genuinely doorless; F13/F14 discovered BY the migration) | 2 silent (the review's finds; 8 reported all accurate) | 4 | 4 | 4 | substantial+ (F15 conversion; tour twin migrated; THREE Evan design rounds absorbed mid-pass: sagitta/metered enumeration, the consistency-not-accuracy principle, then the layering fork — per_boundary DELETED, the volume backstops on the new permanent invariant lane firing Corrupt-voiced ResultVolumeImplausible, census re-proven byte-identical after the restructure; #214 debt tracking with a count assertion) | MERGED #213 27/27; the seam's public doors are exactly the approved set; margins are lengths BY SIGNATURE workspace-wide; judgment call (positive_volume stays in-seam) adjudicated sound | ~327k impl + ~147k review + ~438k fix | ~4.5h impl + ~0.7h review + ~6h fix (incl. the design rounds) |
| M6-5 | 2026-08-06 | edge-selection fillet vocabulary PR-1: emitter + Vec<StableName> node + v3 break + the die REGISTERED | M (logged pre-draw) | OPUS (block M7-7 draw byte 19 = opus,fable) | APPROVE 0/2/3 (sabotaged birth row caught loud typed; shuffled/duplicated wire selections refuse NotCanonical incl. the save-side symmetric gate; the uncovered bump refuses Vanished — no silent name break constructible; 114 table rows counted with exact histogram; MINORs = PR-2 riders: all_edges untested, the ⊆ direction unasserted) | 0 (5 reported, all verified — incl. deviation 1: the orchestrator's own substrate-inventory deletion, honestly re-derived) | 5 | 4 | 5 | none (MINORs ride PR-2) | MERGED #219 26/0/1; F-e MEASURED: the untested single-call form WORKS (12 open chains + 1 closed rim, one node) — the spec's fallback never taken; THE COMPOSED DIE IS A REGISTERED CORPUS DOCUMENT (M6-1 dev-1 inexpressibility CLOSED); PR-2 MERGED #220 under Evan's Actions-outage waiver (local batteries the gate: 346/0+274/0+14/0; review APPROVE 0/1/3, the doc contradiction fixed, the trapdoor made a designed guard, drift claim honestly bounded; door-symmetry bijection executed; the boolean-over-octants kernel frontier pinned flip-when-fixed) | ~294k impl + ~162k review | ~4.6h impl + ~1.6h review |
| MESH | 2026-08-06 | trimmed-NURBS tessellation lane: hull-derived Hessian certificate + both consumers + montage completion | M (logged pre-dispatch) | fable (block M7-6 remainder) | APPROVE 0/2/3 scoped to the code head (Z1: 5.2M per-triangle samples, worst ratio EXACTLY 0.5000 — the Q/4-vs-Q/8 conservatism attained, never violated; planted cert bugs: 2 of 3 caught, the third only by a tautological mirror → MIN-1) | 0 (7 reported) | (in report) 5 | 4 | 4 | folded in-unit (the empirical per-triangle falsifier became the unconditional CI guard; Fitted arms executed with genuinely certified caches; probes by merge) | MERGED #218 27/27; NURBS faces RENDER (walk-frontier retired, S9); montage 22 cells incl. the s_duct (opposed curvature — unrevolvable, edge-on); one CONFLICTING silent-CI incident → the standing merge-main norm; 3 checkpoint nudges + 1 escalation (recovered); Evan merge-with-visual-followup-banked | ~444k impl (incl. conflict fix + visual rework) + ~163k review | ~8.3h impl wall + ~2.8h review |
| MV2 | 2026-08-06 | montage-v2: cell curation, twisted_duct (measured path vocabulary), true minimal loft pair | S (logged pre-dispatch) | fable (block M7-7 remainder) | Evan-eyeball class ("these look great!"): item 2 CONCEDED his read (s_duct = two glued revolves, demoted honestly) and answered by measurement — twisted_duct with nowhere-zero τ beyond any revolve gluing; profile twist verified unsupported; the ≥0.5-turn helix refusal found and now FILED; item 3 measured (the old pair was the prism rescaled) and re-spaced silhouette-obvious | 0 | — | — | — | none | MERGED #221 (Actions-outage waiver; eyeball = the gate); stray-fallback-frame corruption in the committed montage found and repaired; two follow-ups filed per Evan (long-turn frontier; the fallback pathway question) | ~304k impl | ~5.7h wall (render-heavy) |
| GUARD | 2026-08-07 | render-guard: the matplotlib fallback structurally uncommittable (preview dir + provenance guard) | S (logged pre-dispatch) | OPUS (block M7-8 remainder) | orchestrator-review class: all three scenarios + a bonus arm executed (planted #221-victim frame → named typed fail; absent-FreeCAD → loud preview routing, committed tree bit-untouched; present → 34/34 byte-stable); sheet exemption as a POSITIVE assertion; guard self-tests ahead of every scan | 0 | — | — | — | none | MERGED #224 27/27 — and the run itself confirmed ACTIONS RECOVERED; FreeCAD warm-session deadlock confirmed recurring (3/3 full-pass attempts, different scenes) → per-scene-timeout follow-up filed | ~136k impl | ~4.6h wall (render-verification-heavy) |
| M6-6 | 2026-08-07 | the curved sense-flip tier gate (check-6 curved arm + import parity rider) | low-M (logged pre-draw) | fable (block M7-8 draw byte 194 = fable,opus) | NOT-MERGEABLE-AS-IS → fixed: 1 MAJ (missing test lint header — both clippy jobs red; trivial) + 1 MINOR-high (conic-trimmed cylinders slip BOTH gates whole-body — executed on cut_cylinder, unrecorded); the GATE HELD everything: census byte-identical 51/51 at three ε, full truth-table re-execution, nappe adversaries minted both apex sides (no correction needed — confirmed), three-rim layering probe, 11/11 pins | 0 (3 reported) | (in report) | (in report) | (in report) | light (header; residual 4 recorded+pinned-as-residual with the ellipse-rim flip condition; residual 3 + rider claims scoped to the circle-rimmed class; probes adopted) | MERGED #223 fresh-run 0-failed; EVERY previously-invisible curved sense flip now refuses CurvedSenseInverted; inside-out washer/cone/donut/lily certify-green CLOSED; M6's ratified content is DONE — the k-lint floor is the last hygiene before its exit walk | ~241k impl + ~175k review + ~259k fix | ~3.1h impl + ~2.7h review + ~0.3h fix |
| KLINT | 2026-08-07 | k-lint baseline-floor refresh: ε-independent P0 floor 4.0e-5 + ε-coupled rule 4 + rule-2 cap (M7-F1, ruled) + m7 committed baseline | M (logged pre-draw) | OPUS (block M7-9 draw byte 47 = opus,fable; fable remainder → next M7-eligible dispatch) | APPROVE-WITH-FIXES 0/1/4, rubric 5/4/4 (every load-bearing number independently recomputed from the committed baseline; 5/5 mutation probes killed; cold sweep byte-identical to the committed rows; MIN-1 = a false "no intermediate case" classification claim — `props_quad_face_extent` is ε-dependent-not-proportional; M7-F1 adjudicated with the blind window MEASURED and pinned by adopted probes: [4e-5, 1e-3) at 1e-6 only, empty at tight rows) | 0 (1 reported — the rule-2 cap, self-flagged as M7-F1 with the counter-posture argued; RULED: cap stands, both arms concurring) | 5 | 4 | 4 | light, IMPLEMENTER-INHERITED (MIN-1 carve-out both places + NOTE-1/2/3 adopted + reviewer probes merged fast-forward with authorship; snapshot-contract scoping after main's tour grew path_junction_turn mid-flight) | MERGED #239 fully green (27 checks); hosted advisory row 0 flags at all three ε rows (fresh sweep AND committed baseline); litmus fires at every row, now ASSERTING margin < floor; 14 k-lint tests (was 10); the M4-era 102-flag noise retired; lint stays ADVISORY (gating readiness = walk material) | ~184k impl + ~89k review + ~222k fix (resumed segment) | ~2.3h impl + ~0.6h review + ~0.3h fix (no gaps) |
| KLINT-GATE | 2026-08-08 | k-lint gate flip: findings fail the row (exit 2 + the interpretation-discipline message, Evan's design #243/5224869607); three exit voices pinned | S (logged pre-draw) | OPUS (block M7-10 slot 0; draw byte 205 = opus,fable,opus) | n/a — CI infra, orchestrator-reviewed (three voices demoed executed: flagged CSV exit 2 + discipline text; malformed exit 1 distinct; committed baseline exit 0 at all three ε rows), no blinded lane — EXCLUDED from comparison and from the dual-review count | 0 | — | — | — | none; fix-pass executor: n/a | MERGED #253 27/27 incl. the renamed "k-lint (gate)" row green on its own fresh sweep; no branch protection existed (protection API 404), no watcher references the old row name; "k-lint (advisory)" survives only in historical records | impl ~103k / review n/a (orchestrator) / fix 0 (per-phase) | impl ~1.4h / fix 0 (no gaps) |
| M7-5 | 2026-08-08 | band-seam re-mint: seamless periodic cylinder/torus bands import first-class (seam mint at u_ref, shared-rim splits, D2 mint-side winding, both wild fixtures flipped + FreeCAD oracles derived) | M (logged pre-assignment from the substrate inventory) | fable (block M7-9 v2-pair remainder; post-amendment dual-review row 2 → single review, G1 unmerged at merge) | NOT-MERGEABLE-AS-IS → re-review APPROVE. 3/3/2, rubric 4 / 2→4 / 3→4. MAJ prose: (1) an inside-out torus band imported CERTIFY-GREEN at V=−3.61e-7 m³ — the tier-3 backstop the reported D2 torus carve-out relied on never ran on the import path; (2) `chart_direction` misread winding in an ~18° azimuth window — a VALID washer imported green with the silent COMPLEMENT (895.36 vs 1684.93 mm³), worse than the refusal it replaced; (3) the ruled inverted-cylinder refusal was deletable with the suite green (unpinned). C3 adjudication upheld the implementer's geometry (winding×sense provably selects the torus v-region; A/B fixtures at rel<1e-12) — the miss was the unwired backstop, not the analysis | 0 silent (3 reported deviations: torus D2 carve-out, ε-floor re-pin 1e-10→1e-9, row-2 volume to 4 ulp — both relaxations reproduced PRE-EXISTING at merge base) | 4 | 2→4 | 3→4 | substantial, IMPLEMENTER-INHERITED (band_backstop wired inside import_step with unknown-escalation-kinds refusing; structural winding read sign(rim axis·surface axis)∘use — window GONE not moved, 30-azimuth dense re-sweep; refusal pins mutation-verified; 9 review fixtures adopted → 7 permanent rows, authorship kept; all MIN/NOTE applied incl. Seam-only adoption + ε_in coaxiality gate). Delta re-review probes preserved on m7/band-seam-review-probes (verification-grade, not adopted) | MERGED #252 27/27; suite 132/132 committed (135/135 with lane probes) × ε default/1e-6/1e-12; census + oracle volumes re-derived from raw entities EXACTLY (2.5e-16/5.3e-16 rel); one mid-flight hosted red (clippy type_complexity, both rows one root cause) fixed same-day; second NOT-MERGEABLE-AS-IS of the project — the assigned inside-out-band attack executed the gap | impl ~300k + ~1k clippy fix / review ~236k + ~44k delta / fix ~73k (per-phase) | impl ~1.0h + fix ~1.1h / review ~2.4h + ~0.7h delta (no gaps) |

Block M7-9 (2026-08-07): difficulty logged PRE-DRAW: k-lint
baseline-floor refresh (re-derive `BASELINE_FLOOR_MARGIN` from the
current distribution + settle the ε-coupled-family rule; the 102
advisory flags/run are the symptom) = **M**. Draw follows this log
entry. DRAWN: byte 47 → (opus, fable) — the k-lint floor refresh
gets OPUS; fable remainder → the next M7-eligible dispatch.
REMAINDER ASSIGNED (2026-08-07): the band-seam re-mint unit
(ftc_11 + cq_red_cube flip back to imports); difficulty logged
PRE-ASSIGNMENT from the substrate inventory = **M**.

Block M7-10 (2026-08-08) — FIRST M7 TRIPLE under v3: difficulty
logged PRE-DRAW for slot 1: the k-lint gate flip (advisory→failing
row with Evan's interpretation-discipline message, #243 comment
5224869607; CI-infra class, no blinded lane, excluded from
comparison per the row-16/41 precedent — and therefore NOT counted
in the dual-review every-3rd, which counts blinded-lane
implementation rows only) = **S**. Draw follows this entry. DRAWN: byte 205 (no
rejection), 205 mod 3 = 1 → block M7-10 arm order (opus, fable,
opus) — the gate flip (slot 0) gets OPUS; fable rides slot 1
(stage-1 recognition, next dispatch); opus remainder in slot 2.

## LIB program rows (usable-as-a-library, docs/LIBRARY-DESIGN.md; own block series per the §L8 ruling — no draw collision with the concurrent M6/M7 orchestrator)

Block LIB-1 draw (2026-08-06): byte 13 → (opus, fable).
Difficulties logged pre-draw: U1 façade = S, U2 PATHS algebra = L.
U1 dispatched first → OPUS; U2 = fable remainder.

| # | date | task | difficulty (pre-flip) | arm | review findings (MAJ/MIN/NOTE) | silent devs | idiom | tests | docs | fix-pass size | battery | tokens | wall-clock |
|---|------|------|----------------------|-----|-------------------------------|-------------|-------|-------|------|---------------|---------|--------|------------|
| U2 | 2026-08-08 | PATHS algebra: typestate lattice, lowering to ProfileLoop, differential/property/compile-fail suites (PR-1 of 2; PR-2 tour rework rides the same unit) | L (logged pre-draw) | fable (block LIB-1 remainder) | APPROVE-WITH-FIXES 1/1/3, rubric 5/4/5 (extraction verbatim-confirmed at b1781c2; bit-identity HELD on every new adversarial differential — reflex polygon, r=1e-7/1.999, rotated frames, tangent-seam close — with the one-ulp divergence exactly the documented finding-10 boundary; 8 further illegal states all E0599, no runtime mint door; e2e extrude+revolve volumes bit-equal vs LoopBuilder twins; MAJ-1 = sign domain ungated — negative line(len) after a fillet stranded the authored anchor ~0.5 off-path THROUGH validate, a §4-item-3 ratified-text breach) | 2 silent (MAJ-1/MIN-1, holes in the PR's own universal property claims; 11 reported findings, the load-bearing three audited doc-faithful) | 5 | 4 | 5 | moderate, crash-fragmented (funnel gates path_leg_length/path_fillet_radius, typed NonpositiveLeg/NonpositiveFilletRadius; R6/R7 + len=0 regression rows; #131 pin; Clone-fork doc; PathNoCornerReason rename; TWO fix agents lost to the WSL crash — the surviving lane diff was verified correct and adopted unchanged by a finisher) | MERGED #233 26/1-skip; PR-2 MERGED #238 (review APPROVE 0/0/3 rubric 5/4/5, zero silent devs — census, lowering bit-identity, bracket ulp drift, and k-probe ALL independently re-executed; +~157k impl / ~105k review tokens); 12 loop sites algebra-authored, 14 gap-named raw; stadium gap + NURBS legs + ε_input plumbing + the PR-2 seven-item wall list banked as v2-conversation evidence | ~364k impl + ~166k review + ~49k finisher (2 dead fix segments unrecorded) | ~4.1h impl + ~1.1h review + fixes fragmented across the WSL crash |
| G1 | 2026-08-08 | PATHS vocabulary growth cheap set: circle/arc_via/arc_center/far-end anchor/exact directors + §2a addendum + corpus migration (raw census → 3) | M (logged pre-dispatch) | OPUS (block LIB-2 remainder) | **DUAL REVIEW (sample #1)** — R1: APPROVE-WITH-FIXES 1/1/3, rubric 5/4/4; R2: APPROVE-WITH-FIXES 1/1/2, rubric 5/4/5. CONVERGED on the identical MAJOR (Zero-fit far-end anchor inherited resolve_fillet's unconditional outgoing tangency declaration → executed spurious TangencyContradicted on §2a-legal sharp continuation; declaration-without-construction, §4 item 2); tails disjoint (R1: §3 table order, footprint prose; R2: t2-vs-anchor vertexhood pin, PQ4-phrasing clause, in-band gate rows). Headlines both verified independently: byte-identity 3 ε rows, disc canonicalization mechanism established, .angle bit-preservation, bracket exact via .toward | 0 silent (R1: 3 reported all honest; R2: 4 reported 0 silent) | 5/5 | 4/4 | 4/5 | moderate (ArrivalKind enum replaces the seam bool — declares only when something tangent follows, red-checked both directions; t2 absorb rule stated + pinned (emitting the anchor would break two-doors bit-identity — measured call); §2a sentence + PQ4 clause + table order; in-band gate rows; byte-diff re-run clean though not logically required — declared joints are exported); fix-pass executor: implementer-inherited | MERGED #254 27/27; the BRACKET moves bit-identically (VQ4 proven); raw census: boss (measured 3-arc topology), rocker (G2), bowtie (permanent); CI caught interval-square poison (dx*dx→powi(2)) pre-review | impl ~544k (parked+resumed) / R1 ~188k / R2 ~180k / fix ~335k | impl ~5.3h (incl. slot waits at load ~20) / R1 ~1.9h / R2 ~1.2h / fix ~0.8h (no gaps) |
| U3 | 2026-08-08 | SectionSegments retirement: loft/sweep speak ProfileLoop; door validation for ALL sections; split-brain closed structurally | M (logged pre-draw) | fable (block LIB-2 draw byte 134 = fable,opus) | APPROVE 0/0/3, rubric (in report) (all ten claims independently re-executed: byte-identity 3 ε rows on the reviewer's own base build; differential pin red-checked at one ulp; door delta probed both directions — base silently BUILT an invalid interior section, branch refuses SectionProfile typed; no silent open-chain reinterpretation path — the type change forces conscious ports; false tangency declarations refused at the new seam; novel loft volume exact to 5e-16; NOTE-1 = error-precedence flip unstated, NOTE-2 = per-call loop clone, NOTE-3 = probe-count wording) | 0 (3 reported, all verified; NOTE-1 sub-deviation-threshold) | (in report) | (in report) | (in report) | none required (NOTEs banked as G-series riders); fix-pass executor: n/a | MERGED #245 checks green; SectionSegments DELETED (grep pin), OpenClosedMixed retired, 42 sites migrated, 9 quad() clones collapsed per-crate; NOTE-3 differential row in-repo | impl ~246k / review ~114k / fix 0 (per-phase per v3 discipline) | impl ~0.96h / review ~0.41h / fix 0 (no gaps) |
| U1 | 2026-08-07 | pncad façade crate + prelude; tour on ONE kernel dependency | S (logged pre-draw) | OPUS (block LIB-1 draw byte 13 = opus,fable) | APPROVE-WITH-FIXES 0/2/3, rubric 5/3/3 (byte-identity of all 89 tour exports independently rebuilt+reproduced at merge-base; closure property HELD under independent sweep but its advertised "no-dev-deps ⇒ physically incapable" proof mechanism executed-FALSE — reviewer's `use topo as _;` probe compiled clean; novel washer authored end-to-end on one dependency, 3 prelude exits, none to a second crate) | 0 (4 reported, all verified) | 5 | 3 | 3 | moderate (self-scanning guard pin PROVEN by executed falsification; audit +9 rows + 2 honest closure exceptions — serde_json::Value flagged for U9, unnameable DuplicateName a filed kernel wart; Band into prelude per orchestrator ruling; ladder test now tier-3-XOR-3′ with a real-union row; doctests 1→8) | MERGED #232 27/27; P8 dead (six p2 + four validated deleted); SurfaceKind leak closed STRUCTURALLY (zero kernel micro-edits — the §1 permission unused) | ~158k impl + ~152k review + ~242k fix | ~1h impl + ~1.5h review + ~2.3h fix |
Block LIB-2 draw (2026-08-08): byte 134 → (fable, opus).
Difficulty logged pre-draw: U3 SectionSegments retirement = M.
U3 dispatched first → fable; opus = remainder for the next unit.
G1 rides block LIB-2's remainder (2026-08-08): U3 took the fable
first slot; G1 = OPUS remainder. Difficulty logged pre-dispatch:
G1 vocabulary-growth cheap set = M.
