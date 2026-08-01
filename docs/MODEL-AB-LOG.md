# Opus 5 vs Fable 5 implementation A/B — experiment log

Standing experiment (Evan, in-chat, 2026-07-25, Opus 5 release
day). Protocol:

- Every IMPLEMENTATION dispatch gets a fair coin flip from
  /dev/urandom: heads = Opus 5 (`model: "opus"`), tails = Fable 5
  (session default). Design, specs, adversarial reviews, and
  fix-pass rulings stay Fable regardless. The fix pass runs as the
  implementer's agent, so it inherits the arm.
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
