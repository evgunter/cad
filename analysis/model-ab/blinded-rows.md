# Blinded A/B dispatch rows (arm column removed, model names redacted)

Source: `docs/MODEL-AB-LOG.md`. The `arm` column has been dropped and
any residual model name replaced with `[MODEL]`. Do not attempt to
infer the arm; label only from the substance of the row.

---

## row_id: 1

- **date**: 2026-07-25
- **task**: #93 join-stage seam-region anchors
- **difficulty**: M
- **findings**: 2/1/2 (both MAJ = claim-level; one credited a main bug the fix already fixed)
- **silent_devs**: 0
- **idiom**: 4
- **tests**: 4
- **docs**: 4
- **fix_pass**: moderate (doc corrections + 2 adopted pins + 2 minors)
- **battery**: 145 suites: 1263/1263/1408, all pin families green
- **tokens**: ~1.15M (incl. 2 crash resumes)
- **wall**: ~19h wall incl. crash gap (~5h active)

---

## row_id: 2

- **date**: 2026-07-25
- **task**: #99 tour ε-panic
- **difficulty**: S
- **findings**: 0/1/3
- **silent_devs**: 0
- **idiom**: 5
- **tests**: 4
- **docs**: 5
- **fix_pass**: 1 line (orchestrator-applied)
- **battery**: tour 3/3 ε rows; zero kernel diff
- **tokens**: 58k
- **wall**: 6.4 min

---

## row_id: 3

- **date**: 2026-07-25
- **task**: M4 PR 6 persistence
- **difficulty**: L
- **findings**: 2/2/3 + 1 delta-MAJ
- **silent_devs**: 0 (5 reported)
- **idiom**: 5
- **tests**: 4
- **docs**: 5
- **fix_pass**: substantial (token retype + strict maps + goldens + tangent_joints + save symmetry sweep)
- **battery**: 156 suites: 1327/1325/1472 + persistence rows ×3ε
- **tokens**: ~1.49M (incl. crash resume)
- **wall**: ~26h wall incl. crash gap

---

## row_id: 4

- **date**: 2026-07-25
- **task**: #101 declared tangency
- **difficulty**: M
- **findings**: 1/1/3
- **silent_devs**: 1 (falsified doc claim)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: moderate (fillet Result + fit predicate + 6 pins)
- **battery**: 147 suites: 1283/1283/1429 + eps 3/3
- **tokens**: ~880k
- **wall**: ~9h wall (incl. crash gap)

---

## row_id: 5

- **date**: 2026-07-26
- **task**: #106 depth-2 nested-island coverage
- **difficulty**: M
- **findings**: 0/0/4
- **silent_devs**: 0
- **idiom**: 4
- **tests**: 5
- **docs**: 5
- **fix_pass**: NONE (NOTEs banked for 8a latency data)
- **battery**: 1265/1265/1411; fresh probes: main refuses, branch exact 8.25
- **tokens**: 134k
- **wall**: ~1h

---

## row_id: 6

- **date**: 2026-07-26
- **task**: interval transcendentals crate
- **difficulty**: L
- **findings**: —
- **silent_devs**: —
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: —
- **battery**: —
- **tokens**: —
- **wall**: —

---

## row_id: 7

- **date**: 2026-07-26
- **task**: A×Z render scene
- **difficulty**: S
- **findings**: 1/1/2
- **silent_devs**: 0
- **idiom**: 4
- **tests**: 4
- **docs**: 4
- **fix_pass**: small (fallback fix + two-sided pin + narration)
- **battery**: tour+pins green, eps 3/3, fallback 19/19
- **tokens**: ~263k
- **wall**: ~3h

---

## row_id: 8

- **date**: 2026-07-26
- **task**: #111 CDT needle triangle
- **difficulty**: M
- **findings**: 0/2/3 (MINs report-level)
- **silent_devs**: 0
- **idiom**: 5
- **tests**: 5
- **docs**: 5
- **fix_pass**: tiny (decimal slip + comment + coordinated pin flip)
- **battery**: 158 suites 1335/0; tour+eps 3/3; admesh external gate
- **tokens**: ~356k
- **wall**: ~8h wall (incl. limit gap)

---

## row_id: 9

- **date**: 2026-07-26
- **task**: M4 PR 8a corpus+latency
- **difficulty**: L
- **findings**: 1/3/3 (MAJ = designed promotion)
- **silent_devs**: 0
- **idiom**: 5
- **tests**: 5
- **docs**: 4
- **fix_pass**: moderate (promotion + exhaustive kinds + baseline refresh; 1 finding DISPUTED w/ evidence, upheld)
- **battery**: 1333/1333/1482 + corpus/persistence/latency rows
- **tokens**: ~573k
- **wall**: ~26h wall (incl. limit gap)

---

## row_id: 10

- **date**: 2026-07-27
- **task**: M4 PR 8b K-lint + pickups
- **difficulty**: M
- **findings**: 0/2/5
- **silent_devs**: 0
- **idiom**: (in report)
- **tests**: (in report)
- **docs**: (in report)
- **fix_pass**: light (lint_csv door + accounting + #120 golden regen)
- **battery**: 1343/0 + 17-row matrix green; planted-fragility catch 175 flags
- **tokens**: ~640k
- **wall**: ~11h wall

---

## row_id: 11

- **date**: 2026-07-27
- **task**: M5 PR 1 interval-crate adoption
- **difficulty**: M (logged pre-draw)
- **findings**: 0/3/2
- **silent_devs**: 1 (stale-claims sweep left 6 live-rustdoc inari mentions)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: moderate (3 MINs + CI row + computable deletion + 3 suite adoptions; in flight)
- **battery**: 1343/0 ×3ε + 1498/0 interval ×3ε; 17.5M-case reviewer fuzz clean
- **tokens**: ~277k impl (+fix tbd)
- **wall**: ~8h impl wall

---

## row_id: 12

- **date**: 2026-07-27
- **task**: M5 PR 3 NURBS substrate part 1
- **difficulty**: L (logged pre-assignment)
- **findings**: 0/2/5
- **silent_devs**: 0
- **idiom**: 5
- **tests**: 4
- **docs**: 3
- **fix_pass**: light (2 doc MINs + wording NOTEs + 21 test adoptions)
- **battery**: 1387/0 + 1550/0 interval (post-fix); all 21 reviewer attacks held
- **tokens**: ~465k impl + ~530k fix
- **wall**: ~11h impl + ~0.5h fix wall

---

## row_id: 13

- **date**: 2026-07-28
- **task**: M5 PR 2 C9 interval ring
- **difficulty**: M (logged pre-draw)
- **findings**: 0/2/2
- **silent_devs**: 0
- **idiom**: 5
- **tests**: 5
- **docs**: 4
- **fix_pass**: light (merged #130)
- **battery**: 9.7M exact fuzz 0 violations; ~3M differential max 1 step; sign clamp + zero annihilator proven as ℝ-facts
- **tokens**: (in log)
- **wall**: (in log)

---

## row_id: 14

- **date**: 2026-07-28
- **task**: M5 PR 4 projection+fitting+LSQ
- **difficulty**: L (logged pre-assignment)
- **findings**: 0/2/5
- **silent_devs**: 0 substantive
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: moderate, in flight (binom_row C(55,26) exactness fix, curvo hermeticity note, #126(a), 4 reviewer-test adoptions)
- **battery**: 1440/0 + 1615/0 interval (pre-fix); direct bound survived ~1M-sample falsification at ratio 1.0000
- **tokens**: (fix tbd)
- **wall**: (in log)

---

## row_id: 15

- **date**: 2026-07-28
- **task**: M5 PR 8 BVH crate + sweep wiring
- **difficulty**: M (logged pre-draw)
- **findings**: 2/6/4 (both MAJ = design forks, ruled by Evan)
- **silent_devs**: 0
- **idiom**: 5
- **tests**: 3
- **docs**: 4
- **fix_pass**: substantial (mechanical items + ruling increment: N5 amendment, golden re-pin, L7 grep)
- **battery**: 1410/0 + 1586/0 interval; die −29% / corpus −21%; merged #135
- **tokens**: (in log)
- **wall**: (in log)

---

## row_id: 16

- **date**: 2026-07-29
- **task**: CI dependency-closure filter (determinator/nextest eval)
- **difficulty**: S (logged pre-assignment)
- **findings**: n/a (CI infra — validated by synthetic-diff runs + hosted CI + Evan's PR review; no blinded lane)
- **silent_devs**: 0
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: none yet
- **battery**: filter validated on 12 synthetic diffs; -p plumbing proven live
- **tokens**: ~94k
- **wall**: ~17min

---

## row_id: 17

- **date**: 2026-07-29
- **task**: M5 S6 two-tolerance message sweep
- **difficulty**: S (logged pre-draw)
- **findings**: 1/2/3 (MAJ = dishonest Invalid payload at the exactly-on arm)
- **silent_devs**: 0
- **idiom**: 4
- **tests**: 4
- **docs**: 5
- **fix_pass**: moderate (shape-only Display branch, Invalid-arm carrier, far-honest rephrase, 16 exactly-once pins, 3 probe suites adopted)
- **battery**: touched crates green both lanes; no-semantic-change proven by full-diff read; #138 gating
- **tokens**: (in log)
- **wall**: interrupted twice (spend limit + 529); finisher pattern

---

## row_id: 18

- **date**: 2026-07-29
- **task**: M5 S2 arc-leg fillet sugar
- **difficulty**: M (logged pre-draw at block-7 time)
- **findings**: 1/2/3 (MAJ = arc setback wrap; construction math fully verified)
- **silent_devs**: 0
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: moderate (signed fold bit-identical corner-side, newly-reachable refusal row, trio parity, cusp recourse, probes adopted)
- **battery**: 124/0 + 134/0 profile; 20k-corner review fuzz zero wrong circles; MERGED #137 (21 rows)
- **tokens**: (in log)
- **wall**: interrupted once (529); finisher pattern; impl notably complete pre-review

---

## row_id: 19

- **date**: 2026-07-30
- **task**: M5 PR 5 Ellipse + C5 dispatch table
- **difficulty**: L (logged pre-draw)
- **findings**: REJECT→APPROVE: 3/3/3 (MAJ-1 = even-crossing silent one-sided split, on the PR's own corpus geometry; MAJ-2 = D9 std trig; MAJ-3 = untested split trileans); geometry fully held (500-config fuzz ≤5e-12)
- **silent_devs**: 0
- **idiom**: 5
- **tests**: 5
- **docs**: 5
- **fix_pass**: heavy (root-based crossing lane exposing+fixing 2 further latent defects; seam-cut upgraded refusal→split; re-review APPROVE 5/5/5)
- **battery**: shape (i) corpus e2e; M2 bit-identity independently confirmed; #141 gating
- **tokens**: (in log)
- **wall**: first REJECT of the project; fix pass deepened the unit

---

## row_id: 20

- **date**: 2026-07-30
- **task**: M5 S1 REST-contact join lane
- **difficulty**: M (logged pre-draw)
- **findings**: 1/2/3 (MAJ = silent corrupt STL via hole-creating merge role inversion — pre-existing machinery, newly reachable)
- **silent_devs**: 0
- **idiom**: 4
- **tests**: 4
- **docs**: 4
- **fix_pass**: moderate+ (roles corrected via Newell winding + NEW tier-3 loop-role gate filling a documented deferral; 6 probes adopted)
- **battery**: crosslap tripwire retired at exact volume; root cause BETTER than the wire's own story (germ-meta, confirmed at merge-base); MERGED #140
- **tokens**: (in log)
- **wall**: zip is purely structural — no new numeric predicate

---

## row_id: 21

- **date**: 2026-07-30
- **task**: M5 PR 6 certified pcurve storage
- **difficulty**: M (logged at block-8 draw time)
- **findings**: 0/3/3 (best MIN = snap-to-family ε-shell falsifying the stored envelope on the attach path)
- **silent_devs**: 0 (5 reported)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: moderate (snap slack provably zero on minted caches + O(ε)-tightness pin + trim-window doc + max_residual split + seam probes adopted)
- **battery**: MERGED #144 18/18; found + independently confirmed the pre-existing PR 5 chord_spec complement-arc defect at merge-base
- **tokens**: (in log)
- **wall**: (in log)

---

## row_id: 22

- **date**: 2026-07-30
- **task**: M5 S7 CI/docs hygiene (ε-row retirement + cache-key audit)
- **difficulty**: S (logged pre-assignment)
- **findings**: 0/0/2 (lightweight review per spec §5)
- **silent_devs**: 0 (4 reported)
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: none (one NOTE fixed on-branch by orchestrator)
- **battery**: MERGED #142 18/18 — its own gate demonstrated the 21→18-row battery; DEFAULT_EPS=1e-9 no-coverage-lost finding independently confirmed
- **tokens**: (in log)
- **wall**: (in log)

---

## row_id: 23

- **date**: 2026-07-31
- **task**: M5 S8 nearest-corner fillet selection ladder
- **difficulty**: S (logged first)
- **findings**: 0/3/3 (MINs doc-level; math STRENGTHENED in review — mixed enclosing/non-enclosing impossibility proved)
- **silent_devs**: 0 (3 reported)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: light, doc-level (honest cross-lane wording + ulp-perturbed determinism rows both lanes + line×arc mirror proof + probe adoptions)
- **battery**: MERGED #143 18/18; 27M impl + ~160k reviewer fuzz, zero dominance violations; 3 constructor cross-checks agree
- **tokens**: (in log)
- **wall**: (in log)

---

## row_id: 24

- **date**: 2026-07-31
- **task**: M5 S9 chord_spec azimuth-window repair
- **difficulty**: S (logged pre-assignment)
- **findings**: 0/3/3, no re-review (MIN-1 = new definite arms missed the two-tolerance shape)
- **silent_devs**: 0
- **idiom**: 4
- **tests**: 5
- **docs**: 4
- **fix_pass**: moderate (two-tolerance definite arms + true centre-reduction bound + interval belly row + short-circuit metering + reviewer probe verbatim)
- **battery**: MERGED #145 18/18 with MERGE PRIORITY; member-2 silent wrong body independently confirmed at merge-base 5fab705
- **tokens**: (in log)
- **wall**: (in log)

---

## row_id: 25

- **date**: 2026-07-31
- **task**: M5 PR 7 SSI (march + three-limb certificate)
- **difficulty**: L (logged first)
- **findings**: 2/6/— (M1 = powf step rule + jet sin_cos fork; M2 ruled ACCEPT-AND-BANK → PR 7b), rubric 4/4/4
- **silent_devs**: 0 (5 reported)
- **idiom**: 4
- **tests**: 4
- **docs**: 4
- **fix_pass**: substantial (9 items) + ε-fix redirect after gate RED (multi-ε battery caught test-side 1e-9 hardcoding; SSI_MAX_FIT_SAMPLES typed kernel budget)
- **battery**: MERGED #146 18/18; local 21/21 × (1e-6/1e-9/1e-12/interval); 8000-matrix independent SVD differential clean; core held under adversarial re-derivation
- **tokens**: (in log)
- **wall**: (in log)

---

## row_id: 26

- **date**: 2026-07-31
- **task**: M5 PR 7b tensor Bernstein compose + plane×NURBS retirement (EXIT-GATING)
- **difficulty**: M (logged pre-assignment)
- **findings**: 0/4/2, rubric 5/4/4 (review REFUTED the "geometry-capped" claim with measurement; ~1.6M falsification samples zero bound-below-truth; max forced looseness 108×)
- **silent_devs**: 1 (center-shift skipped, "no center to lose" ring-false — 6 orders lost at 1e6 m; review-caught, fix-pass implemented to the representation floor 1.225e-9)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: moderate (center-shift impl + non-monotone rewording + recourse pin + breadth sentences + 10 probe rows adopted)
- **battery**: MERGED #149 18/18; shape (iii) substrate GREEN all lanes (exit gate); bound 6.5 orders tighter, within 1% of truth; one waiter-park stall (sweep-revived) + outage #9 resume en route
- **tokens**: ~281k impl + ~323k fix
- **wall**: ~4.3h impl + ~4h fix wall (incl. outage gap)

---

## row_id: 27

- **date**: 2026-08-01
- **task**: M5 demo unit: rocker arc-fillet stop + staged tiltedcut
- **difficulty**: S (logged pre-dispatch)
- **findings**: 0/1/2 (APPROVE, no fix pass; every narration claim survived executed check; S8 pick proven a rule via perturbation)
- **silent_devs**: 0 (6 reported/assessed; render incident contained + verified exact)
- **idiom**: 5
- **tests**: 4
- **docs**: 5
- **fix_pass**: one-liner (orchestrator-applied per S7 precedent: retire note names the K-sweep join)
- **battery**: MERGED #150 18/18; tour 3/3 ×3ε; 4783 K-probe samples ×3ε identical; one waiter-park stall (report nudge)
- **tokens**: ~178k impl + ~123k review
- **wall**: ~2.5h impl wall

---

## row_id: 28

- **date**: 2026-08-01
- **task**: M5 PR 9 curved booleans + tangency regime
- **difficulty**: L (logged first)
- **findings**: 3/6/5 vs impl (3 MAJ incl. 2 silent: union-only scope, red battery; core geometry HELD under tube-threading/zip attacks), rubric 5/3/4
- **silent_devs**: 2 silent (of 11+2 reported)
- **idiom**: 5
- **tests**: 3
- **docs**: 4
- **fix_pass**: heavy: 7 items + arc-facing WENT LIVE (2-arc disc unions) + Interval root cause (infinity seeds + branch-cut-free cone) + idealized-sweep 0/0 clearance fix; then triple gate-red (2 lint rounds on adopted probes, 4th interval-square occurrence caught by the BVH Interval differential)
- **battery**: MERGED #152 18/18 MERGE PRIORITY — main-is-wrong du_of_rims fixed (0.6545→0.7854 silent at base, 2 public calls); review's merge-base witness led the writeup
- **tokens**: ~742k impl+fix (in-lane)
- **wall**: ~1.5 days wall incl. outage #10

---

## row_id: 29

- **date**: 2026-08-01
- **task**: M5 PR 10 sweeps/lofts + schema v2 clean break
- **difficulty**: L (judged pre-draw; logged post-draw — ordering slip recorded in log)
- **findings**: 1/4/4, rubric 5/4/4 (MAJ = dead Sweep recipe lane misreported as a capability; math held: closed-form loft between sections, Eq 10.8 exact match, 8/8 header attacks)
- **silent_devs**: 2 effectively silent (node-layer sweep total refusal; missing size note)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: moderate+ (honest lane collapse + OpenClosedMixed wired/Escalated deleted + tangent-claim truth + size note + RaggedRows + 22 probe rows adopted; post-merge frontier-message truth pass unprompted)
- **battery**: MERGED #151 18/18; schema v2 clean break exactly per ratified mechanics; dev-2 coordination claim FALSIFIED by reviewer scratch-merge (assembly → 9c item 6)
- **tokens**: ~325k impl + ~396k fix
- **wall**: ~2 days wall incl. outage #10

---

## row_id: 30

- **date**: 2026-08-01
- **task**: interval-square retirement + CI tripwire
- **difficulty**: S (logged pre-dispatch)
- **findings**: 0/0/3 APPROVE, no fix pass (5M-sample regroup probe: 2 ulp max, 0 flips; .sqr() tighten-only proven; allowlist audited line-by-line)
- **silent_devs**: 0 (10 non-bit-identical conversions self-reported as judgment calls, all upheld)
- **idiom**: 5
- **tests**: 4
- **docs**: 5
- **fix_pass**: none (PR opened by orchestrator on APPROVE)
- **battery**: MERGED #153 18/18; 55+6 sites converted, 2 false positives restructured, 54 allowlisted; one waiter-park + battery-scope correction (Evan live) en route
- **tokens**: ~128k impl + ~143k review
- **wall**: ~6h wall

---

## row_id: 31

- **date**: 2026-08-01
- **task**: M5 PR 9c banked completions (sphere doors + blocker map)
- **difficulty**: L (logged pre-draw)
- **findings**: 1 MAJ (proof-text scope: sphere r² parity leg refuted) / 2 MIN / 2 NOTE; group-arm design verified sound; both judgment calls ENDORSED
- **silent_devs**: 0 (6 numbered, all with executed blockers)
- **idiom**: 4
- **tests**: 4
- **docs**: 4
- **fix_pass**: moderate (proof scoped per-kind + option (d) pinned ×3; stale promise rewrites; both NOTE rows taken)
- **battery**: MERGED #154 18/18; 1 of 6 items landed by design — the five executed blockers re-planned the milestone (assembly→post-PR 11; Fitted→SSI lift; revert→sense ratification)
- **tokens**: (in log)
- **wall**: (in log)

---

## row_id: 32

- **date**: 2026-08-02
- **task**: M5 PR 11 tessellation + certified quadrature (demo moment)
- **difficulty**: L (logged pre-dispatch)
- **findings**: 0/2/5 APPROVE, rubric 4/4/5 (falsification: zero bound-below-truth; star find = the accidentally-load-bearing factor-2)
- **silent_devs**: 0 (5 reported; dev 1 superseded mid-flight by Evan's static-split ruling, implemented cleanly)
- **idiom**: 4
- **tests**: 4
- **docs**: 5
- **fix_pass**: moderate (factor-2 accounting + corner-scan pin; SelfTouchingTrimLoop arm; provenance field — which caught the machine-state drift; probe adoptions) + one multi-ε gate red (band-relative caps, FitSampleBudget-precedent arm)
- **battery**: MERGED #157 18/18 — tiltedcut renders, montage refreshed, staged machinery deleted; T6: CDT does not dominate
- **tokens**: (in log)
- **wall**: (in log)

---

## row_id: 33

- **date**: 2026-08-02
- **task**: M5 S10 face orientation sense
- **difficulty**: M (logged pre-draw)
- **findings**: 0 code-MAJ / 3 MIN / 2 NOTE, rubric 5/4/5 (A/B discipline held adversarially; spec-premise REFUTED with live-defect proof — e2e pellet-swallow found by review)
- **silent_devs**: 0 (2 reported incl. the premise refutation as MAJOR-returned)
- **idiom**: 5
- **tests**: 4
- **docs**: 5
- **fix_pass**: light (3 doc MINs + probe adoptions) + one gate red (PR 9c message pin caught F1-scoping erosion — fixed message, not pin)
- **battery**: MERGED #155 18/18; the enabling infrastructure for S11's fix
- **tokens**: (in log)
- **wall**: (in log)

---

## row_id: 34

- **date**: 2026-08-02
- **task**: M5 S11 concave/inward walls sense:false
- **difficulty**: M (spec-time)
- **findings**: 0/2/3 APPROVE, rubric 5/5/4 (criterion survived 6 adversarial constructions unmodified; nappe algebra independently confirmed)
- **silent_devs**: 0 (5 reported incl. the widened revolve scope)
- **idiom**: 5
- **tests**: 5
- **docs**: 4
- **fix_pass**: light (mef hazard banked + probes adopted)
- **battery**: MERGED #156 18/18 MERGE PRIORITY — pellet-swallow + washer-bore containment defects DEAD
- **tokens**: (in log)
- **wall**: (in log)

---

## row_id: 35

- **date**: 2026-08-02
- **task**: M5 S12 curved revert wiring + per-class ∖/∩ opening
- **difficulty**: M (logged pre-draw)
- **findings**: 0 new MAJ / 2 MIN / 3 NOTE APPROVE, rubric 5/5/4 (class boundary proven COMPLETE; reviewer implemented the rejected alternative — all pins green, rationale corrected)
- **silent_devs**: 0 (2 reported incl. the fallback MAJOR-returned)
- **idiom**: 5
- **tests**: 5
- **docs**: 4
- **fix_pass**: light (2 doc MINs + NURBS hazard scoping + 4 probes)
- **battery**: MERGED #158 18/18 — curved ∖/∩ LIVE on Plane/Cylinder; 3rd main-is-wrong found (vertex-probe fallback, ∪ sphere-class, pinned)
- **tokens**: (in log)
- **wall**: (in log)

---

## row_id: 36

- **date**: 2026-08-02
- **task**: M5 S13 die-pips enablers (containment-fallback re-cut + plane×sphere germ arm)
- **difficulty**: M (logged pre-assignment)
- **findings**: 1/—/— (MAJ = new, executed: the multi-normal escape hole; dev 2 confirmed as main-is-wrong #4)
- **silent_devs**: —
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: F1–F5; all nine reviewer probes adopted 9/9
- **battery**: —
- **tokens**: —
- **wall**: —

---

## row_id: 37

- **date**: 2026-08-02
- **task**: M5 PR 13 curved STEP subset
- **difficulty**: M (logged pre-draw)
- **findings**: 0/2/3 APPROVE
- **silent_devs**: 0
- **idiom**: 5
- **tests**: 4.5
- **docs**: 5
- **fix_pass**: light
- **battery**: —
- **tokens**: —
- **wall**: —

---

## row_id: 38

- **date**: 2026-08-03
- **task**: demo dual-montage (kernel + FreeCAD/OCC lanes)
- **difficulty**: S (logged pre-assignment)
- **findings**: 0/1-nit APPROVE
- **silent_devs**: —
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: none (nit)
- **battery**: —
- **tokens**: —
- **wall**: —

---

## row_id: 39

- **date**: 2026-08-03
- **task**: M5 S4 save/load shared-validator consolidation
- **difficulty**: S (logged pre-draw)
- **findings**: 0/0 APPROVE
- **silent_devs**: 0
- **idiom**: 5
- **tests**: 4
- **docs**: 5
- **fix_pass**: none (one note orchestrator-applied)
- **battery**: —
- **tokens**: —
- **wall**: —

---

## row_id: 40

- **date**: 2026-08-03
- **task**: M5 PR 12 constant-radius fillets + the die
- **difficulty**: L (logged pre-assignment)
- **findings**: 1/3/5 APPROVE w/ fix pass (MAJ = octant e0 pick: tier-3 lost on non-square prisms, die unaffected)
- **silent_devs**: 1 (scope gap: Band-4 rows)
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: F1–F6 + two gate-red rounds
- **battery**: —
- **tokens**: —
- **wall**: —

---

## row_id: 41

- **date**: 2026-08-03
- **task**: CI build-once/shard (compile per MODE, nextest archives)
- **difficulty**: M (logged pre-draw)
- **findings**: n/a — CI infra, no blinded lane
- **silent_devs**: 0
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: none
- **battery**: #167 27/27 green; wall 16m57s → 15m47s, billed ~64.5 → ~56.6 min, 4 → 2 workspace builds
- **tokens**: —
- **wall**: —

---

## row_id: 42

- **date**: 2026-08-03
- **task**: M5 PR 14 exit sweep (K snapshot, DESIGN/envelope, exit walk, A/B readout)
- **difficulty**: M (logged pre-assignment)
- **findings**: n/a — docs/telemetry, no blinded lane (orchestrator reviews the walk personally)
- **silent_devs**: 0
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: —
- **battery**: docs-only; fmt-all --check clean
- **tokens**: —
- **wall**: —

---

## row_id: M6-1

- **date**: 2026-08-04
- **task**: composed die via in-place surgery + circle-clearance rider
- **difficulty**: M (logged pre-draw)
- **findings**: PASS 0 MAJ / 2 MIN / 3 NOTE, rubric 5/4.5/5 (volume confirmed 3 independent ways incl. 4e8 MC; rider falsifier clean over 3000 pairs)
- **silent_devs**: 0 (4 numbered, all verified honest)
- **idiom**: 5
- **tests**: 4.5
- **docs**: 5
- **fix_pass**: light (2 MINs + probe adoptions, via fresh finisher — transcript lost)
- **battery**: composed die: every verb on ONE body; FreeCAD to 1e-6 mm³; strategy divergence retired; dev 1 = corpus-inexpressibility discovery → M6 unit 5
- **tokens**: (in log)
- **wall**: (in log)

---

## row_id: M6-2

- **date**: 2026-08-04
- **task**: SSI generic-T lift + Pcurve::Fitted + non-vacuous fitted cache at rest
- **difficulty**: M (logged pre-draw)
- **findings**: APPROVE-WITH-FIXES 1/4/5, rubric 4/4/4 (MAJ = one unused import going CI-red, SKIPPING the hosted interval matrix; deep attacks all held: OnLocusHull adjudicated honest by the walk row's own letter, f64-Newton dev sound, reviewer's second-species corruption caught by loop continuity)
- **silent_devs**: 0 (9 numbered; clause-by-clause spec diff; separate owner-requested design audit: 7/8 RIGHT, 1 RIGHT-BUT-MISDOCUMENTED, 1 scope gap — Evan's "felt off" instinct matched exactly the two non-RIGHT verdicts)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: moderate+ (8 items + audit doc-fixes + fixture re-anchored to the kernel's own split_at carrier with natural [a,b] domains + one self-caught cross-scalar overclaim stop-and-reported and weakened honestly)
- **battery**: MERGED #176 27/27 incl. interval shards w/ geom-brep confirmed; walk row 2 NON-VACUOUS (full C2 at rest, planted + reviewer corruptions rejected, Interval enclosure row); UnsupportedCarrier retired (S9 flip); 3 review probes adopted
- **tokens**: ~437k impl + ~467k fix
- **wall**: ~5.4h impl + ~8.7h fix wall (incl. CI waits, under CPU pin)

---

## row_id: M7-1

- **date**: 2026-08-04
- **task**: step-import crate: Part-21 parser + rotation-system Euler assembly + D7 adoption ladder, own-corpus round-trip
- **difficulty**: L (logged pre-draw)
- **findings**: APPROVE-WITH-FIXES 1/3/5, rubric 5/4/4 (MAJ = silent-unit class: CONVERSION_BASED_UNIT length context imported silently as metres; headline 14/14 first-re-export byte-identity CONFIRMED and proven un-laundered; adoption ladder held all 4 planted corruption classes with honest structured refusals; fixed point held on reordered/renumbered files)
- **silent_devs**: 1 (units — the MAJ; fixed by-resolution)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: moderate (by-resolution unit/uncertainty checks + inch-file test; full 7k-cut truncation sweep; 3 silent drops → typed Structure refusals; string-body refusal arm; 6.00-ulp correction; 19 review probes adopted by merge with authorship kept, incl. re-anchoring the reviewer's VACUOUS unit probe — its #93 substitution never matched cube's #155)
- **battery**: MERGED #183 27/27 hosted; 28/28 crate suite (9 acceptance + 19 probes); deviation 1 (sidecar kernel-census overrides) adjudicated honest; fenced findings → #184
- **tokens**: ~441k impl + ~181k review + ~37k fix (resumed segment)
- **wall**: ~1.9h impl + ~0.6h review + ~0.5h fix active

---

## row_id: M7-2

- **date**: 2026-08-04
- **task**: FreeCAD foreign-corpus import: mm units, outerness inference, base cones, vertex-loop sphere, structure roots
- **difficulty**: L (logged pre-assignment)
- **findings**: APPROVE-WITH-FIXES 2/4/2, rubric 4/4/4 (both MAJ on adversarial inputs, not the corpus: torus normalization = SYMPTOM-FLIP laundering an inside-out torus, half of it a kernel props sense_sign gap → #184; coincident-locus rung certified surfaces not the curve; chart-inverse fuzz clean at 1.2e-15; the A3 K-landing measurement verified bit-exact)
- **silent_devs**: 0 (11 reported, all check out)
- **idiom**: 4
- **tests**: 4
- **docs**: 4
- **fix_pass**: moderate+ (torus winding derived from cyclic loop order — mid-quadrant chart sampling; inverted/undecidable tori refuse typed naming the kernel gap; ISO's .F./CW legal encoding imports correctly; curve certification through the shared door; ceiling-skips became sub-tolerance assertions with the declining-vs-answering-falsely distinction; probes adopted by merge)
- **battery**: MERGED #189 19/6-skip/0; battery green at 5 ε values; oracle 13/13; one earlier hosted gate-red (ε=1e-6 pcurve certify on the mm corpus) root-caused to scale-vs-absolute-ε, fixed by derived CORPUS_EPS_CEILING=1e-8 without widening any gate; FIRST IN-BAND K LANDING found and reported (#89)
- **tokens**: ~422k impl + ~169k review + ~33k fix
- **wall**: ~2.6h impl (incl. gate-red loop) + ~0.8h review + ~0.6h fix

---

## row_id: M7-4

- **date**: 2026-08-05
- **task**: wild corpus: 4-vein license-verified fixtures, unit/vector/string dialect unlocks, rigid assemblies, no-panic contract
- **difficulty**: M (logged pre-draw)
- **findings**: APPROVE-WITH-FIXES 0/2/3, rubric 5/4/4 (every headline independently confirmed: oracle volumes re-derived digit-for-digit, 5 mirror-smuggling attempts defeated, 620+ mutations no-panic, ε_in floor proven NOT a widened gate, duplicate-solid latency confirmed at merge-base; MIN-1 was an INHERITED knot-multiplicity SIGABRT)
- **silent_devs**: 0 (7 reported, all upheld)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: light+ (false comment truthed; schema-derived knot budget — n+d+1 checked pre-allocation, probe_knot un-ignored with slowness ceiling + validity control; probes adopted by merge with the multi-ε gap self-caught and fixed as honest weaker claims)
- **battery**: MERGED #193 27/0/0 full matrix (widened filter ran freecad/admesh/persistence/corpus/interval rows); 7/13 wild files import first-class (oracle 1e-16..1e-13 rel), 6 refuse typed; RingOnCurvedFace refusal ruled+concurred; band-seam unit banked; 98 tests 0 ignored
- **tokens**: ~372k impl + ~167k review + 2 fix resumes (per-segment figures unreliable)
- **wall**: ~1.8h impl + ~1.2h review + ~1.1h fixes

---

## row_id: RIM-DIM

- **date**: 2026-08-05
- **task**: du_of_rims dimensional-metering fix (RimLevel enum) + props/predicate dimensional audit (~120 rows, 8 inline comparand fixes)
- **difficulty**: M (logged pre-dispatch)
- **findings**: APPROVE-WITH-FIXES 1/2/5, rubric 5/4/4 (fix corrects real VERDICT FLIPS both directions — pre-fix silently grouped 50ε-separated rims and spuriously refused 0.5ε-coincident ones; MAJ = the unit's own twin pin not ε-row-honest, which EXECUTED deferred F4 into Band{1e-6,1e-5} on a hosted row; R6 executed the F5 linkage: the freecad ε=1e-7 cylinder refusal IS pcurve_chart_radial_moving, 2r² in-band)
- **silent_devs**: 0 (7 reported, all honest)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: moderate (three-outcome ε-row pin with F4's live signature; fix pass FALSIFIED the review's scale-invariance premise by measurement — the 1e-12 margins are F3's volume_backstop under a STALE predicate name, so F3 also corrupts K attribution: second executed retirement reason; probes adopted+ε-hardened; sphere pole note)
- **battery**: MERGED #197 all green; #89 landing RETIRED (a3 sweep delta exactly one line); banked dimensional unit grows to F3+F4+F5 + ceiling re-derivation, sequenced after M6-3
- **tokens**: ~290k impl + ~154k review + ~370k fix segment
- **wall**: ~3h impl + ~0.9h review + ~0.8h fix, all under the 20-45x pin

---

## row_id: M6-3

- **date**: 2026-08-05
- **task**: loft/sweep body assembly: IsoCurve seams + iso-pcurve lane + tier-3 flips + exact NURBS flux + Leg E analytic-chart completion (walk row 4) + tube_along_arc rider
- **difficulty**: L (logged pre-assignment by the M6 orchestrator)
- **findings**: APPROVE-WITH-FIXES 0/5/4, rubric 5/4/4 (whole-unit review: Leg E algebra RE-DERIVED BY HAND all correct; ceiling composition executed; MINORs all honesty/coverage class; quality seam between phases detectable-but-weak — all four false statements in completion scope, partial spotless)
- **silent_devs**: 0 (9 reported, all executed blockers)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: light (5 comment/coverage items + probe adoption by merge + one self-extended adjacent-row fix reported)
- **battery**: MERGED #192 27/27; WALK ROW 4 CLOSED (ball/cone/donut + die octants carry stored pcurves at rest); loft/sweep bodies live end-to-end (tour prints V=9±1e-13); F5 fixed in passing (ceiling 1e-8→1e-5); lily findings 11 + 56-ulp drift retired by the rider; six red jobs root-caused first
- **tokens**: partial (M6 session, unrecorded) + ~642k completion + ~231k review + fix segment (figures unreliable across resumes)
- **wall**: ~7h completion + ~2h review + ~0.4h fix, all under the 20-45x pin

---

## row_id: F3+F4

- **date**: 2026-08-05
- **task**: funnel-bypass retirement (volume backstop dual-arm) + ring-winding mean-width metering
- **difficulty**: M (logged pre-draw)
- **findings**: APPROVE-WITH-FIXES 1/3/4 (T1 silent-disable reproduced instrumented at merge-base — only 1 of 3 bound checks ran at mm scale; T3 full 23,389-sample byte-diff confirmed exactly-one-line K delta; MAJ = the metering's own weakening direction: a wrongly-kept 3mm cube on a 2m plate metered IN-BAND and passed — the assigned hide-behind-area attack landed)
- **silent_devs**: 0 (2 declared; MIN-1's collision claim corrected as worse-than-unverified)
- **idiom**: (in report)
- **tests**: (in report)
- **docs**: (in report)
- **fix_pass**: moderate (dual-arm gate: sign-certain violation refuses dimension-free via the exact bit-hairline band, BOTH arms on the metered comparand so K stays dimensionally honest — verified scale-linear and RED-with-arm-removed; zero-perimeter arm declared; bypass claim scoped + editor-core F12 row added)
- **battery**: MERGED #200 26/0/1; the tree's audited family has no funnel bypass left; deviation 2 pinned not prose (silent skip cannot return unnoticed)
- **tokens**: ~254k impl + ~113k review + ~312k fix
- **wall**: ~1h impl (post-outage) + ~25min review + ~1.2h fix

---

## row_id: KERNEL

- **date**: 2026-08-05
- **task**: KERNEL_* sidecar fields + live staleness row + step-import consumer swap (#184 design)
- **difficulty**: S (logged pre-draw)
- **findings**: APPROVE 0/1/4 (K2's pad-hiding-room attack answered in the unit's favor: planted +1000mm³ lie passes off-ε overlap rows but the default-ε byte pin catches it — composed hiding room ZERO; K3 corruption probe: old tolerance accepts, new catches at 1600×/8000× margins; MIN = the "4-8 orders tighter" claim measured wrong for 11/14 fixtures)
- **silent_devs**: 0 (6 reported, all verified)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: micro (2 prose corrections: measured-truth tolerance claim in body+comment; staleness-row doc scoped to solid sidecars; probes adopted by merge incl. loft_prism-refuses-typed)
- **battery**: MERGED #199 fully green; override table GONE; KERNEL_VOLUME_PAD_MM3 ε-discovery (enclosure midpoint moves with ambient ε — byte pin at declared ε + overlap rows elsewhere); fmt_real pub
- **tokens**: ~279k impl + ~117k review + micro fix
- **wall**: ~2h impl + ~1.3h review

---

## row_id: SKINFIT

- **date**: 2026-08-05
- **task**: #207 fix: integral skin fit never synthesizes a rational wall (sweep_body's first successful caller)
- **difficulty**: S (logged pre-dispatch)
- **findings**: APPROVE 0/1/3, rubric 5/5/5 (every executed number reproduced exactly; the reviewer's own 17-station convergence run closed the bracket's hidden-bias headroom; stash-red 5/6 + bit-identity both ways; W2's three bitwise-conservation facts verified incl. denormal/1e300 probes)
- **silent_devs**: 0 (5 reported; MIN = deviation-4 wording imprecision, orchestrator-corrected in the PR body)
- **idiom**: 5
- **tests**: 5
- **docs**: 5
- **fix_pass**: none beyond the body edit
- **battery**: MERGED #210 27/27; source fix (ℝ³ lane — the denominator never computed, C6 bitwise structure selection); quarter-torus elbow = sweep_body's FIRST successful caller (Pappus bracket 3.8e-6 @9 stations, pad pinned separately); non-uniform lofts live (V=12 exact); uniform lane bitwise-unchanged two ways; Evan calibration note: a less-principled fix would have been acceptable
- **tokens**: ~153k impl + ~108k review
- **wall**: ~2.5h impl + ~1.2h review

---

## row_id: M7-3

- **date**: 2026-08-05
- **task**: NURBS-face import: both surface arms, surface_sig fix, IsoCurve rung, rim adoption, ARM B
- **difficulty**: M (logged pre-draw)
- **findings**: APPROVE-WITH-FIXES 3/2/3, rubric 4/4/3 (survived: sig injectivity under transposed-net/knot-swap/weights attacks, foreign-refit-seam refusal, dm1 re-anchor; MAJ-1 = Arm B's uncertified class — a different circle through the same endpoints laundered on rational walls; MAJ-2 det-0 frame; MAJ-3 the recurring vector×projection lint FP)
- **silent_devs**: 0 (3 reported, all verified)
- **idiom**: 4
- **tests**: 4
- **docs**: 3
- **fix_pass**: moderate (the rim residual gate with a REPORTED role inversion — wall boundary sampled against the closed-form circle distance + lever-armed angular containment killing the complement arc; honest-perpendicular line_frame with fail-loud backstop; named-binding lint fix; 3-token and payload-prose corrections; 14 probes adopted permanent)
- **battery**: MERGED #209 fully green; ARM B blessed by Evan then REPAIRED to verified-not-trusted (updated on-thread, blessing carried); SOLID_FIXTURES 15 with fixed point; M7 unit 3 CLOSED
- **tokens**: ~277k impl + ~180k review + ~343k fix
- **wall**: ~2.5h impl + ~1.2h review + ~1h fix

---

## row_id: FOLD

- **date**: 2026-08-06
- **task**: corpus-widening fold: nonuniform_loft + swept_elbow (15→17), first non-default RTOL
- **difficulty**: S (logged pre-draw)
- **findings**: APPROVE-WITH-FIXES 1/2/2 (X2 settled with authority: the reviewer's own 24-pt Gauss integrator matched the KERNEL to 1.28e-12 and missed OCC by 1.9e-8 — the 1e-7 RTOL is honest, OCC is wrong; layered-row safety quantified at 71,000× over the kernel row's budget; MAJ = the builder doc stating the naive numbers the fixture refutes)
- **silent_devs**: 0 (6 reported)
- **idiom**: 4
- **tests**: 5
- **docs**: 3→5 post-fix
- **fix_pass**: micro (doc-comment truth with the trap named; 5-ulp/5.93e-2 unit-slip corrections; whole-numeral token matching; probe adopted)
- **battery**: MERGED #212 26/1; elbow = sweep_body's first FIXTURE; NURBS-on-sweep import refused nowhere; trapezoid chosen over the easier prism BECAUSE the prism never exercises the #207 path
- **tokens**: ~149k impl + ~164k review + ~180k fix
- **wall**: ~1.9h impl + ~1.8h review + ~0.2h fix

---

## row_id: MONTAGE

- **date**: 2026-08-06
- **task**: montage refresh: tube cell + count pins; 3 NURBS scenes blocked on mesh
- **difficulty**: S (logged pre-draw)
- **findings**: orchestrator-review class (demos unit): PARTIAL delivered honestly — cell 19 with an executed bit-exact intent assertion; the three NURBS scenes stopped at a genuine design boundary (placeholder cells would break the two-sheet contract) with all constructions WRITTEN and saved as a patch
- **silent_devs**: 0 (the stop-and-report IS the discipline)
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: none (merged as delivered)
- **battery**: MERGED #215; clean-re-render verified twice; cell pin 19 with derivation; the block PROMOTED the mesh trimmed-NURBS lane (two consumers) — dispatched as the [MODEL] remainder
- **tokens**: ~139k impl
- **wall**: ~0.5h

---

## row_id: MIGRATE

- **date**: 2026-08-06
- **task**: classify-seam migration: Length<T> by signature, ~351 door sites, the invariant lane
- **difficulty**: M (logged pre-draw)
- **findings**: APPROVE-WITH-FIXES 2/2/5 both MAJ silent (Y1's of-laundering attack LANDED — a unit-dot sine wrapped in of, with an executed scale-blindness probe; demos/tour broken at tip; byte identity independently reproduced 23394/23394; all 11 flagged sites verified genuinely doorless; F13/F14 discovered BY the migration)
- **silent_devs**: 2 silent (the review's finds; 8 reported all accurate)
- **idiom**: 4
- **tests**: 4
- **docs**: 4
- **fix_pass**: substantial+ (F15 conversion; tour twin migrated; THREE Evan design rounds absorbed mid-pass: sagitta/metered enumeration, the consistency-not-accuracy principle, then the layering fork — per_boundary DELETED, the volume backstops on the new permanent invariant lane firing Corrupt-voiced ResultVolumeImplausible, census re-proven byte-identical after the restructure; #214 debt tracking with a count assertion)
- **battery**: MERGED #213 27/27; the seam's public doors are exactly the approved set; margins are lengths BY SIGNATURE workspace-wide; judgment call (positive_volume stays in-seam) adjudicated sound
- **tokens**: ~327k impl + ~147k review + ~438k fix
- **wall**: ~4.5h impl + ~0.7h review + ~6h fix (incl. the design rounds)

---

## row_id: M6-5

- **date**: 2026-08-06
- **task**: edge-selection fillet vocabulary PR-1: emitter + Vec<StableName> node + v3 break + the die REGISTERED
- **difficulty**: M (logged pre-draw)
- **findings**: APPROVE 0/2/3 (sabotaged birth row caught loud typed; shuffled/duplicated wire selections refuse NotCanonical incl. the save-side symmetric gate; the uncovered bump refuses Vanished — no silent name break constructible; 114 table rows counted with exact histogram; MINORs = PR-2 riders: all_edges untested, the ⊆ direction unasserted)
- **silent_devs**: 0 (5 reported, all verified — incl. deviation 1: the orchestrator's own substrate-inventory deletion, honestly re-derived)
- **idiom**: 5
- **tests**: 4
- **docs**: 5
- **fix_pass**: none (MINORs ride PR-2)
- **battery**: MERGED #219 26/0/1; F-e MEASURED: the untested single-call form WORKS (12 open chains + 1 closed rim, one node) — the spec's fallback never taken; THE COMPOSED DIE IS A REGISTERED CORPUS DOCUMENT (M6-1 dev-1 inexpressibility CLOSED); PR-2 MERGED #220 under Evan's Actions-outage waiver (local batteries the gate: 346/0+274/0+14/0; review APPROVE 0/1/3, the doc contradiction fixed, the trapdoor made a designed guard, drift claim honestly bounded; door-symmetry bijection executed; the boolean-over-octants kernel frontier pinned flip-when-fixed)
- **tokens**: ~294k impl + ~162k review
- **wall**: ~4.6h impl + ~1.6h review

---

## row_id: MESH

- **date**: 2026-08-06
- **task**: trimmed-NURBS tessellation lane: hull-derived Hessian certificate + both consumers + montage completion
- **difficulty**: M (logged pre-dispatch)
- **findings**: APPROVE 0/2/3 scoped to the code head (Z1: 5.2M per-triangle samples, worst ratio EXACTLY 0.5000 — the Q/4-vs-Q/8 conservatism attained, never violated; planted cert bugs: 2 of 3 caught, the third only by a tautological mirror → MIN-1)
- **silent_devs**: 0 (7 reported)
- **idiom**: (in report) 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: folded in-unit (the empirical per-triangle falsifier became the unconditional CI guard; Fitted arms executed with genuinely certified caches; probes by merge)
- **battery**: MERGED #218 27/27; NURBS faces RENDER (walk-frontier retired, S9); montage 22 cells incl. the s_duct (opposed curvature — unrevolvable, edge-on); one CONFLICTING silent-CI incident → the standing merge-main norm; 3 checkpoint nudges + 1 escalation (recovered); Evan merge-with-visual-followup-banked
- **tokens**: ~444k impl (incl. conflict fix + visual rework) + ~163k review
- **wall**: ~8.3h impl wall + ~2.8h review

---

## row_id: MV2

- **date**: 2026-08-06
- **task**: montage-v2: cell curation, twisted_duct (measured path vocabulary), true minimal loft pair
- **difficulty**: S (logged pre-dispatch)
- **findings**: Evan-eyeball class ("these look great!"): item 2 CONCEDED his read (s_duct = two glued revolves, demoted honestly) and answered by measurement — twisted_duct with nowhere-zero τ beyond any revolve gluing; profile twist verified unsupported; the ≥0.5-turn helix refusal found and now FILED; item 3 measured (the old pair was the prism rescaled) and re-spaced silhouette-obvious
- **silent_devs**: 0
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: none
- **battery**: MERGED #221 (Actions-outage waiver; eyeball = the gate); stray-fallback-frame corruption in the committed montage found and repaired; two follow-ups filed per Evan (long-turn frontier; the fallback pathway question)
- **tokens**: ~304k impl
- **wall**: ~5.7h wall (render-heavy)

---

## row_id: GUARD

- **date**: 2026-08-07
- **task**: render-guard: the matplotlib fallback structurally uncommittable (preview dir + provenance guard)
- **difficulty**: S (logged pre-dispatch)
- **findings**: orchestrator-review class: all three scenarios + a bonus arm executed (planted #221-victim frame → named typed fail; absent-FreeCAD → loud preview routing, committed tree bit-untouched; present → 34/34 byte-stable); sheet exemption as a POSITIVE assertion; guard self-tests ahead of every scan
- **silent_devs**: 0
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: none
- **battery**: MERGED #224 27/27 — and the run itself confirmed ACTIONS RECOVERED; FreeCAD warm-session deadlock confirmed recurring (3/3 full-pass attempts, different scenes) → per-scene-timeout follow-up filed
- **tokens**: ~136k impl
- **wall**: ~4.6h wall (render-verification-heavy)

---

## row_id: M6-6

- **date**: 2026-08-07
- **task**: the curved sense-flip tier gate (check-6 curved arm + import parity rider)
- **difficulty**: low-M (logged pre-draw)
- **findings**: NOT-MERGEABLE-AS-IS → fixed: 1 MAJ (missing test lint header — both clippy jobs red; trivial) + 1 MINOR-high (conic-trimmed cylinders slip BOTH gates whole-body — executed on cut_cylinder, unrecorded); the GATE HELD everything: census byte-identical 51/51 at three ε, full truth-table re-execution, nappe adversaries minted both apex sides (no correction needed — confirmed), three-rim layering probe, 11/11 pins
- **silent_devs**: 0 (3 reported)
- **idiom**: (in report)
- **tests**: (in report)
- **docs**: (in report)
- **fix_pass**: light (header; residual 4 recorded+pinned-as-residual with the ellipse-rim flip condition; residual 3 + rider claims scoped to the circle-rimmed class; probes adopted)
- **battery**: MERGED #223 fresh-run 0-failed; EVERY previously-invisible curved sense flip now refuses CurvedSenseInverted; inside-out washer/cone/donut/lily certify-green CLOSED; M6's ratified content is DONE — the k-lint floor is the last hygiene before its exit walk
- **tokens**: ~241k impl + ~175k review + ~259k fix
- **wall**: ~3.1h impl + ~2.7h review + ~0.3h fix

---

## row_id: KLINT

- **date**: 2026-08-07
- **task**: k-lint baseline-floor refresh: ε-independent P0 floor 4.0e-5 + ε-coupled rule 4 + rule-2 cap (M7-F1, ruled) + m7 committed baseline
- **difficulty**: M (logged pre-draw)
- **findings**: APPROVE-WITH-FIXES 0/1/4, rubric 5/4/4 (every load-bearing number independently recomputed from the committed baseline; 5/5 mutation probes killed; cold sweep byte-identical to the committed rows; MIN-1 = a false "no intermediate case" classification claim — `props_quad_face_extent` is ε-dependent-not-proportional; M7-F1 adjudicated with the blind window MEASURED and pinned by adopted probes: [4e-5, 1e-3) at 1e-6 only, empty at tight rows)
- **silent_devs**: 0 (1 reported — the rule-2 cap, self-flagged as M7-F1 with the counter-posture argued; RULED: cap stands, both arms concurring)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: light, IMPLEMENTER-INHERITED (MIN-1 carve-out both places + NOTE-1/2/3 adopted + reviewer probes merged fast-forward with authorship; snapshot-contract scoping after main's tour grew path_junction_turn mid-flight)
- **battery**: MERGED #239 fully green (27 checks); hosted advisory row 0 flags at all three ε rows (fresh sweep AND committed baseline); litmus fires at every row, now ASSERTING margin < floor; 14 k-lint tests (was 10); the M4-era 102-flag noise retired; lint stays ADVISORY (gating readiness = walk material)
- **tokens**: ~184k impl + ~89k review + ~222k fix (resumed segment)
- **wall**: ~2.3h impl + ~0.6h review + ~0.3h fix (no gaps)

---

## row_id: KLINT-GATE

- **date**: 2026-08-08
- **task**: k-lint gate flip: findings fail the row (exit 2 + the interpretation-discipline message, Evan's design #243/5224869607); three exit voices pinned
- **difficulty**: S (logged pre-draw)
- **findings**: n/a — CI infra, orchestrator-reviewed (three voices demoed executed: flagged CSV exit 2 + discipline text; malformed exit 1 distinct; committed baseline exit 0 at all three ε rows), no blinded lane — EXCLUDED from comparison and from the dual-review count
- **silent_devs**: 0
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: none; fix-pass executor: n/a
- **battery**: MERGED #253 27/27 incl. the renamed "k-lint (gate)" row green on its own fresh sweep; no branch protection existed (protection API 404), no watcher references the old row name; "k-lint (advisory)" survives only in historical records
- **tokens**: impl ~103k / review n/a (orchestrator) / fix 0 (per-phase)
- **wall**: impl ~1.4h / fix 0 (no gaps)

---

## row_id: RTIMEOUT

- **date**: 2026-08-08
- **task**: per-scene FreeCAD process isolation + timeout with kill-and-retry (#224 follow-up; the warm-session deadlock structurally contained)
- **difficulty**: S (logged pre-dispatch)
- **findings**: orchestrator-review class (render infra, GUARD precedent): wedge mechanism DEMONSTRATED executed — SIGTERM-ignoring fixture with orphan-bait child: transient → tree-kill → retry → rendered attempt 2; persistent → RENDER WEDGED naming scene+budget, exit 1, committed tree untouched, no orphans; #224 guard intact (selftest 5/5, planted frame refused); byte-stable double passes both lanes
- **silent_devs**: 0 (3 STOP-class infra findings reported: the width-1 mutex has no hold budget and was starved 90+ min by a workspace battery; stale holders misreport (#235); renders degrade 25× under cargo contention)
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: none; fix-pass executor: n/a
- **battery**: MERGED #266 27/27 MERGEABLE/CLEAN; kernel 34/34 scenes (median 4s quiet / 56s contended), STEP 19/19 (7s / 64s); silence-aware per-scene budget FREECAD_SCENE_TIMEOUT=300s with session-group tree-kill; new memory freecad-render-lane.md indexed
- **tokens**: impl ~202k / review n/a (orchestrator) / fix 0 (per-phase)
- **wall**: impl ~3.7h (render-verification-heavy, incl. the 90-min mutex starvation gap, ANNOTATED)

---

## row_id: WMONTAGE

- **date**: 2026-08-09
- **task**: wild-corpus montage: 6 license-cleared cells through the kernel's own import + tessellation (FreeCAD-free lane)
- **difficulty**: S (logged pre-dispatch)
- **findings**: orchestrator-review class (demo, MONTAGE/MV2 precedent — EXCLUDED from comparison + dual-review count): 6 cells byte-stable ×2 clean re-renders, guard selftest 9/9 with new per-lane wild rules, attribution block VERBATIM per the license audit, ci.yml k-lint-row comment extended for demos/wild; eligible-set honesty: b123d refuses import (SURFACE_CURVE, as audited), 1982_MPR121 + 328_battery import first-class but refuse TESSELLATION — a NEW mesh-lane finding (translator-noise plane axes → ~1e-67 chart coords, below spade's 2^-142 floor), reported not fudged; ftc_11 + cq_red_cube joined post-audit via the M7-5 flips
- **silent_devs**: 0
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: none; fix-pass executor: n/a
- **battery**: MERGED #283 CLEAN; post-M7-7-merge re-verify owed (one re-render check, orchestrator)
- **tokens**: impl ~208k / review n/a / fix 0 (per-phase)
- **wall**: impl ~1.2h (no gaps)

---

## row_id: M7-6

- **date**: 2026-08-09
- **task**: stage-1 NURBS recognition: always-promote (#256), whole-patch certification envelope, QUASI_UNIFORM vocabulary, exact Gauss planar flux for spline boundaries
- **difficulty**: M (logged pre-dispatch)
- **findings**: **⚠ NOT VALID DUAL-REVIEW DATA — STRUCK FROM THE VARIANCE SAMPLE.** This row landed on the every-3rd slot (row 6), but R1's primary review ran on the PRE-fix head and R2 on the POST-fix head (the trigger became knowable only after later merges) — sequential reviews of different code, recorded R1/R2 for completeness only; the reviewer-variance estimator must EXCLUDE this pair (see the #268 same-head amendment). R1: NOT-MERGEABLE-AS-IS → re-review APPROVE, 1/4/3, rubric 4 / 2→5 / 3→4. MAJ prose: the sampled certification track shipped GRID-ONLY — no between-samples envelope (spec D-c required one) — and R1's executed falsifiers promoted a 0.25 m-off "plane" and a 0.148 m-off "cylinder" with ~0 certified residuals, fully silently. R2 (independent, blinded to R1): APPROVE 0/1/3, rubric 5/4/4, with its own soundness adjudication of the fixed envelope (L=ρ_max/r hull-valid, denominator variation covered) and the one find R1's cycle missed — `chart_flipped` stubbed false survived the suite (sense-compose arm vacuously verified; closed by the direct P8 pin, red-then-green). Union fix pass consumed both. **Same-head caveat (Evan's catch, recorded for the variance analysis): R1's primary review ran on the PRE-envelope head, R2 on the post-fix head — the trigger became knowable only after later merges made this row 6. The pair is sequential-review data, NOT same-head variance data; do not difference their finding counts as reviewer variance.**
- **silent_devs**: 0 silent (3 STOP-class reported: dm1 is a 7-instance ASSEMBLY — the substrate inventory was false; the mixed promoted/stays-NURBS accept-with-pin class (ruled, the plane×NURBS lane retires it); promoted-cylinder t3 decline)
- **idiom**: 4
- **tests**: 5
- **docs**: 4
- **fix_pass**: substantial ×2, IMPLEMENTER-INHERITED (envelope: plane track collapsed to a whole-patch hull certificate — stronger than sampled; cylinder track span-aware grid + first-order rational-safe envelope, under which dm1's 7 cylinders HONESTLY fail to certify at ε_in — reported regression, nothing widened; plus the triple-clippy saga whose false-negative local runs yielded the standing COLD-LINT protocol, and the union merge that caught MAIN's own #274-class red and fixed it)
- **battery**: MERGED #264 fully green; 146/146 ×3ε + geom-brep 213/213; own-corpus plane promotions 0.0 / ~2.2e-16 / ≤1.7e-16 with one-cycle BYTE-IDENTICAL promoted fixed point; dm1: 17 planes promote, 7 cylinders stay NURBS, assembly gate #186 next; 11 Gauss literals bit-verified correctly rounded
- **tokens**: impl ~396k + fix segments ~500k (multi-resume, per-segment attribution unreliable — annotated) / R1 ~205k + ~44k delta / R2 ~152k
- **wall**: impl ~2.5h + fixes ~4h (slot-starvation gaps annotated) / R1 ~4.6h + delta ~0.6h / R2 ~0.6h

---

## row_id: REBASELINE

- **date**: 2026-08-10
- **task**: full render re-baseline to the hosted canonical producer (Evan's #338 ruling executed): all 35 kernel + 20 freecad frames + both sheets re-committed from the hosted lane; mechanism SPLIT and measured (42/43 scene STLs drifted since #301 with STEP/scenes.json byte-identical — the tessellation-only signature; ~20–22% wholesale pixel drift incl. the mesh-unchanged diefillet control = the GL-stack re-baseline); #316's mid-flight merge reconciled (exactly its four lily files moved)
- **difficulty**: S (demo/infra, logged pre-dispatch)
- **findings**: orchestrator review: montage eyeballed post-re-baseline (all 19 cells incl. the new pulchellus lily); SECOND hosted render on the committed head left git status EMPTY (the new contract's proof, runner-vs-runner artifacts byte-identical); provenance guard green
- **silent_devs**: 0 (the #316 mid-flight reconciliation reported precisely)
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: none; executor n/a
- **battery**: MERGED #354 fully green
- **tokens**: impl ~115k cumulative (multi-segment incl. two nudge recoveries)
- **wall**: ~5h wall (dominated by the ref-build compile + two hosted runs; two parked windows nudged, annotated)

---

## row_id: RENDER-CLI

- **date**: 2026-08-10
- **task**: hosted-render one-command wrapper (scripts/render-hosted.sh: push-check, workflow dispatch, silence-aware polling, byte-exact artifact pull-back) + local entry points refuse without the explicit CAD_RENDER_LOCAL_OVERRIDE sentence (Evan's ask: hosted = default, local = deliberate preview only)
- **difficulty**: S (infra, logged pre-dispatch)
- **findings**: orchestrator review: round-trip byte-exactness PROVEN on real hosted runs (9 wild PNGs + the UV SVG byte-identical through upload-artifact→download, tEXt stamps intact, git status 0, provenance guard green on the pulled tree); the guard demoed three ways (unset rc 1; wrong value rc 1 naming the sentence; correct → PREVIEW ONLY); override wiring STRUCTURAL (explicit env per workflow job, no CI sniffing); scope addition argued sound (the wild lane added to render.yml — the only PNG byte-reproducible lane, hence the only possible stamp-survival proof, and the guard's pointer would otherwise lie)
- **silent_devs**: 0
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: none; executor n/a
- **battery**: MERGED #338 fully green
- **tokens**: impl ~101k
- **wall**: ~0.4h (no gaps)

---

## row_id: M8-C1

- **date**: 2026-08-10
- **task**: assembly instancing (#317): rep→map association, per-instance materialization with fresh topology ids, topo::graft_disjoint + RemapKeys bridge, per-instance rigid re-certification, nested-assembly COMPOSITION (representation graph, one instance per path, outermost-last), A7 record shape (StepImport::Solid.instances)
- **difficulty**: M (logged pre-dispatch)
- **findings**: R1 at ordinal 23 = SINGLE review (head a2f6d116): NOT-MERGEABLE-AS-IS 1/2/3 — the project's FOURTH. MAJ prose: nested-assembly outer transforms were SILENTLY DROPPED (a contentless intermediate SHAPE_REPRESENTATION contributed no instance and no refusal — sub-assembly imported at +10 instead of +110; pre-PR this refused typed: a refuse→silently-wrong REGRESSION, the worst class). Fix chose COMPOSE over refuse (argued from the ratified ASSEMBLY-DESIGN A2/R1), with three typed refusals closing the class (cycle, unreachable placement, dedup). Delta re-review MERGE-READY: fresh 4-level two-rotation hand oracle to 1e-9 mm, reversed composition demonstrably rejected, diamond graph → TWO instances in entity-id order (no dedupe), A7 records verified against shipped geometry, RemapKeys mutation reds the new kernel-side tests
- **silent_devs**: 0 silent — and the unit's HONEST HEADLINE INVERSION reported loudly: the scope premise was wrong (placement refused pre-assembly, so dm1's edges had never reached the ladder); instancing is retired + the one-wall IsoCurve rung closed edge #668, but dm1 STILL refuses at edge #685 (rational-quadratic rim → #327, stage-1 CURVE recognition) — dm1's THIRD advance, S9 pattern; process note: the first fix-pass push preceded the agent's own lint row (orchestrator's early takeover push — one expect_used red cycle)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: substantial ×2, IMPLEMENTER-INHERITED (composition rework beyond the reviewer's one-guard suggestion + A7 rider + kernel-side graft tests + R1 probes adopted with the red probe flipped green and tightened positive-only)
- **battery**: MERGED #325 28/28; step-import 170-176/0 ×3ε (implementer + reviewer independently); wild corpus UNCHANGED at 9/13 (the honest non-flip); montage untouched (license law)
- **tokens**: impl+fix ~363k cumulative / R1 ~135k + delta ~21k
- **wall**: impl+fix ~6h cumulative wall (multi-segment, slot waits annotated) / R1 ~2.7h + delta ~0.9h

---

## row_id: M8-5

- **date**: 2026-08-10
- **task**: mesh rational deviation certificate: quotient-rule Hessian sup bound + rational sagitta/chord bound over the homogeneous nets (cell-centroid recentring, w_min divisor argued as M8-2's mirror, RATIONAL_CERT_SPLITS=16 fixed schedule, ring end-to-end); both mesh gates opened for rational faces/carriers
- **difficulty**: M (logged pre-dispatch)
- **findings**: R1 at ordinal 22 = SINGLE review (head eee5af71): APPROVE-WITH-FIXES 1/2/4. MAJ prose: TEST STRENGTH, not code — dropping the v0·w11 cross-term from suv survived the entire shipped suite despite genuine unsoundness; the reviewer's seeded 1500-patch random domination sweep falsified the mutant at trial 323 (true 108.8 > mutated 106.2) — adopted with authorship, mutation re-run RED then GREEN by the implementer, sweep verified D9-clean. MINORs: frontier pin matched prose not variant (→ UnsupportedCurve variant pin); process-global probe_stats contaminated armed z1 evidence (→ thread-local). R1 hand-re-derived the recurrences + w_min direction; worst bound-vs-truth margin 1.000000 (equality attained at tight constant curvature, NEVER exceeded); 0.9753 Möbius; integral z1 rows bit-identical to merge-base
- **silent_devs**: 0 silent (the full-body frontier honestly pinned: rational tessellation from real bodies waits on the M8-3 pcurve half — the two units meet at a variant-pinned boundary)
- **idiom**: 5
- **tests**: 4→5 post-fix
- **docs**: 5
- **fix_pass**: moderate, IMPLEMENTER-INHERITED (sweep adoption + red/green; variant pin; thread-local stats; doc truth)
- **battery**: MERGED #322 fully green (post-#332 re-merge); falsifier worst rational per-triangle ratio 0.1543; dust re-derived for the rational divisor (2.18e-13 vs 1e-11 pin); lily.rs untouched (Evan's canvas — fence held)
- **tokens**: impl ~254k + fix ~14k / R1 ~158k
- **wall**: impl ~0.9h + fix ~0.2h / R1 ~1.0h (no gaps)

---

## row_id: M8-2

- **date**: 2026-08-10
- **task**: rational-carrier speed_lower_bound: per-span quotient-rule chord-projection bound (numerator min-hull over w_max), retiring the span-meter half of the rational bank; two conscious pin flips; crescent parked at the discovered THIRD bank (→ M8-5)
- **difficulty**: M (logged pre-dispatch)
- **findings**: R1 at ordinal 16 = SINGLE review (head 51105045): APPROVE-WITH-FIXES 0/2/2, rubric 5/4/5. Soundness held R1's full adversarial set (weights 1e-6..1e6, near-cusp/turn-around families, 1e-12 spans, 1e8 offsets, deg 7, 200-case fuzz, ≥4001 samples each — ZERO bound>truth, worst ratio 0.9987); denominator algebra re-derived by hand (w_max direction confirmed); both certifying-path mutations killed; blast-radius of the ruled deviation verified clean (both consumers arc-length-only; a 359° returning regular arc certifies and splits honestly). MINORs: the "cusp" fixture wasn't a genuine collapse (true min 0.0856 — re-derived to an anti-parallel-legs cubic with each row SELF-ASSERTING its collapse before consulting the meter); "certifies kernel-side" overclaim narrowed
- **silent_devs**: 1 REPORTED deviation, RULED SOUND (per-span directions drop the doubled-back global-chord conservatism — the contract is arc-length metering, never injectivity, now stated as the invariant; cusp+turn-around pinned on the actual trigger) + the crescent stop-and-report (mesh/nurbs_cert.rs:151 rational Hessian bank → the M8-5 plan amendment; working restoration parked as cad-work/span-meter-crescent.patch)
- **idiom**: 5
- **tests**: 4
- **docs**: 5
- **fix_pass**: light-moderate, IMPLEMENTER-INHERITED (fixture re-derivation; wording; rounding-posture + conservatism docs; R1's interval-bracket probe adopted with authorship); executor: implementer
- **battery**: MERGED #306 fully green; m5_pr7 rows 7/7 f64 + 8/8 interval; integral arm BYTE-IDENTICAL (only the poison return changed); no corpus disposition moved; bound/truth conservatism band 0.86–0.97 pinned by frontier rows
- **tokens**: impl ~264k cumulative (incl. ruling + fix segments) / R1 ~126k
- **wall**: impl ~4.3h + fix ~0.5h / R1 ~1.0h (no gaps)

---

## row_id: M8-1

- **date**: 2026-08-10
- **task**: #284 Newell chart-frame re-anchor: mesh planar lane derives its chart frame from the boundary (anchor = first walk point, anchor-translated Newell normal, farthest-point u); the two wild tessellation refusals flip, montage 6→8
- **difficulty**: S (logged pre-dispatch — Evan's own sizing on #284)
- **findings**: R1 at ordinal 14 = SINGLE review (head 5515d48d): APPROVE-WITH-FIXES 2/1/3, rubric 5/3→5/4→5. MAJ prose: (1) "well-conditioned by construction" FALSIFIED by execution for off-plane POSITION noise (~ν² residue — head refused typed a ν=1e-22…1e-60 sweep merge-base tessellated; judged acceptable as fail-loud on a synthetic class, doc narrowed to the axis-noise class actually closed + posture pinned typed at 5 ν values); (2) the load-bearing anchor judgment (input-point over centroid, chosen when centroid manufactured ~1e-51 sub-floor coords) had ZERO CI-run coverage — centroid mutation survived everything CI executes, guarded only by the never-run wild generator. R1 independently: 15+80 suites, debug↔release hashes, centroid-patch reproduction of the OLED refusal exact, STL-independent re-derivation of both cells (204 tri genus-2 Euler-exact; 12 tri box), byte-stable re-render, license law intact
- **silent_devs**: 0 silent (anchor judgment + latent stored-axis exposure in walk.rs/chart.rs/step-import both REPORTED in the PR; NOTE-3 became #303 filed register-class)
- **idiom**: 5
- **tests**: 3→5 post-fix
- **docs**: 4→5
- **fix_pass**: moderate, IMPLEMENTER-INHERITED (R1 probes adopted by merge; in-suite anchor falsifier — centroid mutation RED via a 1-ulp anchor assert, GREEN on revert, red/green executed; doc narrowing + pinned typed position-noise row; closing the position-noise class judged non-cheap, banked without a ruling request). SECOND ruled blinding no-action: the reviewer's probe commit 32a95363 carries the trailer (protocol-[MODEL], no blinded party remained)
- **battery**: MERGED #301 fully green; mesh 16+88; tier_gate censuses unmoved; wild montage 8 cells byte-stable ×2 (implementer) + ×1 (reviewer)
- **tokens**: impl ~150k + fix ~50k / R1 ~136k
- **wall**: impl ~1.3h / fix ~0.7h / R1 ~2.0h (no gaps)

---

## row_id: M7-8

- **date**: 2026-08-09
- **task**: plane×NURBS intersection certification (declare-and-check, Evan's #264 ruling): geom-brep edge lane + injected NurbsLane door, additive attach door, seam-orphan pin flip, ε-row postures, ruled #276-union re-fixture
- **difficulty**: M (logged pre-dispatch)
- **findings**: R1 at ordinal 10 = SINGLE review (head 5a18740): APPROVE-WITH-FIXES 1/2/3, rubric 5/3/5. MAJ prose: the between-samples envelope — the unit's headline obligation — had ZERO shipped-test coverage: with the chart-sup decision mutated away every shipped row stayed green incl. the 1e-12 Escalated posture; the envelope ITSELF held under R1's attack (32π wiggle falsifiers refused with the true displacement both operand sides; certified sup dominated dense-sampled truth). MINORs: wall-side falsification cross-crate only; TubeStraddles payload printed the clamped 0.0 as if measured. R1 reproduced the seam numbers bit-for-bit and ran its own 3-ε batteries 147/0 ×3
- **silent_devs**: 0 silent (6 reported devs at R1 all verified; the #276 union collision REPORTED red not smoothed; the second (pcurve) gap REPORTED; acceptance row 1's body-level claim recorded as a reported spec deviation)
- **idiom**: 5
- **tests**: 3→5 post-fix
- **docs**: 5
- **fix_pass**: substantial, IMPLEMENTER-INHERITED (R1 probes adopted by fast-forward with authorship — the reviewer's own commit carries its model trailer, ruled no-action (reviews are protocol-[MODEL], no blinded party remained); mutation (a) proven 12/2 RED → 14/0 GREEN; wall-side falsifier; certified_clearance rename; NaN poison typed refusal) + the RULED re-fixture (option (c): integral twin offset_square_prism tier-valid at rest; arc prism re-pinned as advanced waypoint refusing the banked quadrature verbatim with no-adoption-refusal-survives assertion; both flip conditions named in-code); executor: fresh finisher (also ran the whole endgame after BOTH predecessor implementers died)
- **battery**: MERGED #288 27/27; step-import 160/0 ×3ε; geom-brep 227/0; unit delivered by FOUR agents across the orchestrator handoff (impl died mid-unit → continuation died at handoff → finisher → same finisher ran the fix pass)
- **tokens**: impl segments UNRECORDED (both died with the predecessor session); finisher ~275k cumulative across 3 segments; R1 ~206k
- **wall**: finisher ~6.8h across segments (slot-contention gaps annotated); R1 ~6.1h wall (incl. stale-loop tail); impl wall unrecorded

---

## row_id: CALOCH

- **date**: 2026-08-09
- **task**: calochortus partial refresh: stems through tube_along_arc world-coordinate intent (56-ulp note retired), leaves sweep out of plane via curved-path sweep_body, kite section (rational-section frontier), sweep-crate refusal pin
- **difficulty**: S (logged pre-draw)
- **findings**: orchestrator-review class (demo, MONTAGE/MV2/WMONTAGE precedent — EXCLUDED from comparison + dual-review count): report verified against diff; both render lanes eyeballed (kernel + FreeCAD agree); orchestrator's own clean re-render byte-stable (RC 0, git status 0, guard 35/35); all 7 wall probes fire verbatim (no wall moved); 5 analytic tessellation rows reproduce EXACTLY + 3 blade rows + Pappus check (1.4–2.3e-5 in a two-sided band); k-lint gate green
- **silent_devs**: 0 silent (1 REPORTED deviation, ruled: the crescent section CANNOT sweep — rational sections refuse at nurbs_span_meter (no speed_lower_bound on rational carriers, #207 was integral-only); kite shipped, 8th wall probe REFUSED by the implementer on k-census-pollution grounds (correct), typed-refusal pin adopted in crates/sweep's suite with flip-when-fixed naming the banked rational-wall unit; this finding seeded the pre-M8 demo-hardening stretch Evan ruled)
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: ruling-application by implementer (kite framing, sweep pin, PR-body row-3 bound); executor: implementer
- **battery**: MERGED #294 27/27 incl. k-lint gate; exactly 4 PNGs moved; fences honored; lantern pins unchanged (honest: nothing to re-derive)
- **tokens**: impl ~248k (incl. ruling application)
- **wall**: ~7.6h wall — ~50 min parked (nudged) + ~4h slot starvation by the SEL1 battery, ANNOTATED; active work the remainder

---

## row_id: M7-7

- **date**: 2026-08-09
- **task**: tier-at-import: every imported solid through the SHARED at-rest gate (#260 ruling (a)); band_backstop dissolved; per-SOLID gating; ε-row-honest corpus pins (477-cell sweep)
- **difficulty**: M (logged pre-draw)
- **findings**: R1 at ordinal 8 = SINGLE review (pre-fix head c78ad11): APPROVE-WITH-FINDINGS 1/2/4, rubric 5/4/5. MAJ prose: the per-solid guarantee was FALSE on multi-solid files — a fully-inverted cube refusing alone SHIPPED as the second MANIFOLD_SOLID_BREP beside a normal cube (whole-body flux sum +1−1=0, Zero-exempt; executed probe), and the PR/D7-step-4 claim was falsified; MINORs = suite blind to solid-count loss (take(1) mutation survived tier_gate), 3′≡3 equivalence overclaim. Delta re-review on final head (fix-verification, NOT a dual-review sample): APPROVE-WITH-FIXES 0/1/3 — R1's smuggle probe now refuses naming the guilty solid; 4 NEW attack variants held incl. a 1 cm inverted cube beside a unit cube (aggregate flux POSITIVE — invisible to any whole-body gate); pin machinery mutation-verified both directions; MINOR = stale PR body (orchestrator-rewrote) + TAIL_TURBINE bank comment
- **silent_devs**: 2 silent-ish, both R1's finds (the unnamed multi-solid blindness behind the headline claim; the 3′ overclaim) — everything else reported
- **idiom**: 5
- **tests**: 4→5 post-fix
- **docs**: 5
- **fix_pass**: substantial ×2, SPLIT EXECUTORS: original fix agent (died at the orchestrator handoff; per-solid gating + census teeth + 3′ scope, commit 82a4974, recovered and pushed by the successor orchestrator) + FRESH FINISHER (the 3 red hosted rows were probe-file lints fail-fasting AHEAD of the real defect; swept all 477 cells, found 3 moving files, pinned 18 EpsSensitive cells by live sub-reason signature; judged ftc11@1e-12 pass + nist@1e-12 refusal HONEST, nothing widened, no geometry touched) + orchestrator (delta's doc fixes)
- **battery**: MERGED #276 fully green (19 checks, then 16 on the final comment-only head); tier_gate 5/5 + adopted probes 7/7 ×3ε; full step-import 163/0; R1 probes adopted authorship-kept, ε-hardened from the ambient band; TierInvalid names the guilty solid; touching-assemblies residue named in D7 step 4 + kiss-pin banked to M8/C7
- **tokens**: impl + R1 + first fix segment: UNRECORDED (all three died with the predecessor session — the handoff gap, annotated); finisher ~156k; delta review ~141k; orchestrator ~2k
- **wall**: finisher ~4.6h (slot-starvation gaps annotated); delta review ~3.0h; impl/R1 wall unrecorded

---

## row_id: M7-5

- **date**: 2026-08-08
- **task**: band-seam re-mint: seamless periodic cylinder/torus bands import first-class (seam mint at u_ref, shared-rim splits, D2 mint-side winding, both wild fixtures flipped + FreeCAD oracles derived)
- **difficulty**: M (logged pre-assignment from the substrate inventory)
- **findings**: NOT-MERGEABLE-AS-IS → re-review APPROVE. 3/3/2, rubric 4 / 2→4 / 3→4. MAJ prose: (1) an inside-out torus band imported CERTIFY-GREEN at V=−3.61e-7 m³ — the tier-3 backstop the reported D2 torus carve-out relied on never ran on the import path; (2) `chart_direction` misread winding in an ~18° azimuth window — a VALID washer imported green with the silent COMPLEMENT (895.36 vs 1684.93 mm³), worse than the refusal it replaced; (3) the ruled inverted-cylinder refusal was deletable with the suite green (unpinned). C3 adjudication upheld the implementer's geometry (winding×sense provably selects the torus v-region; A/B fixtures at rel<1e-12) — the miss was the unwired backstop, not the analysis
- **silent_devs**: 0 silent (3 reported deviations: torus D2 carve-out, ε-floor re-pin 1e-10→1e-9, row-2 volume to 4 ulp — both relaxations reproduced PRE-EXISTING at merge base)
- **idiom**: 4
- **tests**: 2→4
- **docs**: 3→4
- **fix_pass**: substantial, IMPLEMENTER-INHERITED (band_backstop wired inside import_step with unknown-escalation-kinds refusing; structural winding read sign(rim axis·surface axis)∘use — window GONE not moved, 30-azimuth dense re-sweep; refusal pins mutation-verified; 9 review fixtures adopted → 7 permanent rows, authorship kept; all MIN/NOTE applied incl. Seam-only adoption + ε_in coaxiality gate). Delta re-review probes preserved on m7/band-seam-review-probes (verification-grade, not adopted)
- **battery**: MERGED #252 27/27; suite 132/132 committed (135/135 with lane probes) × ε default/1e-6/1e-12; census + oracle volumes re-derived from raw entities EXACTLY (2.5e-16/5.3e-16 rel); one mid-flight hosted red (clippy type_complexity, both rows one root cause) fixed same-day; second NOT-MERGEABLE-AS-IS of the project — the assigned inside-out-band attack executed the gap
- **tokens**: impl ~300k + ~1k clippy fix / review ~236k + ~44k delta / fix ~73k (per-phase)
- **wall**: impl ~1.0h + fix ~1.1h / review ~2.4h + ~0.7h delta (no gaps)

---

## row_id: U2

- **date**: 2026-08-08
- **task**: PATHS algebra: typestate lattice, lowering to ProfileLoop, differential/property/compile-fail suites (PR-1 of 2; PR-2 tour rework rides the same unit)
- **difficulty**: L (logged pre-draw)
- **findings**: APPROVE-WITH-FIXES 1/1/3, rubric 5/4/5 (extraction verbatim-confirmed at b1781c2; bit-identity HELD on every new adversarial differential — reflex polygon, r=1e-7/1.999, rotated frames, tangent-seam close — with the one-ulp divergence exactly the documented finding-10 boundary; 8 further illegal states all E0599, no runtime mint door; e2e extrude+revolve volumes bit-equal vs LoopBuilder twins; MAJ-1 = sign domain ungated — negative line(len) after a fillet stranded the authored anchor ~0.5 off-path THROUGH validate, a §4-item-3 ratified-text breach)
- **silent_devs**: 2 silent (MAJ-1/MIN-1, holes in the PR's own universal property claims; 11 reported findings, the load-bearing three audited doc-faithful)
- **idiom**: 5
- **tests**: 4
- **docs**: 5
- **fix_pass**: moderate, crash-fragmented (funnel gates path_leg_length/path_fillet_radius, typed NonpositiveLeg/NonpositiveFilletRadius; R6/R7 + len=0 regression rows; #131 pin; Clone-fork doc; PathNoCornerReason rename; TWO fix agents lost to the WSL crash — the surviving lane diff was verified correct and adopted unchanged by a finisher)
- **battery**: MERGED #233 26/1-skip; PR-2 MERGED #238 (review APPROVE 0/0/3 rubric 5/4/5, zero silent devs — census, lowering bit-identity, bracket ulp drift, and k-probe ALL independently re-executed; +~157k impl / ~105k review tokens); 12 loop sites algebra-authored, 14 gap-named raw; stadium gap + NURBS legs + ε_input plumbing + the PR-2 seven-item wall list banked as v2-conversation evidence
- **tokens**: ~364k impl + ~166k review + ~49k finisher (2 dead fix segments unrecorded)
- **wall**: ~4.1h impl + ~1.1h review + fixes fragmented across the WSL crash

---

## row_id: SWITCH-P

- **date**: 2026-08-09
- **task**: profiles-as-programs PR-A: Step vocabulary (17 variants), record-as-you-lower (26 binder sites), DynTip replay driver (7 states, 33 arms), differential pin over corpus+generator
- **difficulty**: L (logged pre-draw)
- **findings**: APPROVE-WITH-FIXES 0/4/2 (core safety held under attack: wrong-state binder calls UNCOMPILABLE — the drift-proofing construction is real; pin sensitive 3/4 mutations pre-fix, 4/4 post; byte-identity 3 ε rows on reviewer's own base; F1 red-main reproduced first-hand; MINORs = Turn pin hole, 9-vs-13 count, doc overclaim, tripwire follow-up)
- **silent_devs**: 0 silent (findings F1-F6 all reported; 3 flagged as forks for the reviewer — all adjudicated in the unit's favor with F5's ArcCarrierScalar alias endorsed + #279 filed)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: light-moderate (Turn rows verified to kill the reviewer's mutation; count recounted to the reviewer's 13; drift-proofing claim aligned to what the attacks established; #279 filed not implemented — fence); executor: implementer-inherited
- **battery**: MERGED #273 27/27; the program IS recordable+replayable bit-identically; F1 exposed the red-main union gap (#274/#275); SWITCH-E unblocked
- **tokens**: impl ~222k / review ~135k / fix ~235k
- **wall**: impl ~0.9h / review ~0.5h / fix ~0.4h

---

## row_id: U7

- **date**: 2026-08-09
- **task**: structural selectors + name doors: Selector/NamePat/SegTag matcher (data, no serde), all_faces/vertices/bodies materializers, pncad name doors, die_composed 14-name migration pin
- **difficulty**: M (logged pre-dispatch)
- **findings**: **DUAL REVIEW (sample #2, row 6)** — R1: APPROVE 0/1/3, rubric (in report); R2: APPROVE-WITH-FIXES 0/1/3, rubric (in report). CONVERGED on zero MAJORs and the D1-deviation-faithful judgment; disjoint doc-only MINORs (R1: report diff-accounting measured against the wrong base; R2: D1 footprint omitted the lib.rs re-export line). Both independently executed: 14-name pin sensitivity-mutated, SegTag tripwire compile-probed, LB7 fence swept, byte-identity 89 files, prelude doors external-probed
- **silent_devs**: 0 silent (1 reported deviation D1, both reviewers verified faithful)
- **idiom**: 5/5
- **tests**: 4/5
- **docs**: 4/4
- **fix_pass**: none required — both MINORs doc-only, ORCHESTRATOR-applied (PR-body D1 amendment; report figure corrected); executor: orchestrator
- **battery**: MERGED #265; StableName no longer write-only at the façade; P10's structural case closed (geometric predicates deferred per LB7 to a designed follow-up + GQ7 re-homing)
- **tokens**: impl ~172k / R1 ~159k(incl. limit-kill resume) / R2 ~126k / fix 0
- **wall**: impl ~1.9h / R1 ~5h wall (limit gap annotated) / R2 ~0.6h / fix ~0.1h

---

## row_id: G2

- **date**: 2026-08-09
- **task**: arc-carrier fillet modes: extraction seam (#259) + fillet_select module (#261) + the §3 surface/eye/§2b (#268)
- **difficulty**: L (logged pre-draw)
- **findings**: #259/#261 orchestrator-review class (bitwise-pinned extractions); #268 blinded APPROVE-WITH-FIXES 0/3/2 (byte-identity independently rebuilt 3×89; eye √¾-exact corner mutation-verified; masking-bug fix reconstructed; LB10 wall verified typed+unreachable; MINORs = stale docs, missing advance_arc in-band row, unpinned wall)
- **silent_devs**: 0 silent (the LB10 mechanism wall and the anchor-lottery/seam-at-fillet findings were all REPORTED-back mid-unit and ruled LB4/LB5/LB10)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: light-moderate (docs; the in-band row + wall pin both added WITH source.predicate assertions so they cannot be green for the wrong reason); executor: finisher-inherited
- **battery**: MERGED #268 27/27; UNIT G2 CLOSED — raw census = boss(→circle_split at the switch)/outline(LB5 wall)/bowtie(permanent); squared-radius rule ratified in §2b; process note: original implementer lost ~250 lines to an rm overreach, fresh finisher rebuilt from the recorded design (the record-everything discipline priced in)
- **tokens**: impl ~543k(orig, fragmented incl. 2 stale-waiter wakeups) + ~366k(finisher) / review ~176k / fix ~374k(cumulative-resumed, unreliable)
- **wall**: impl ~2.1h+~5.1h / review ~3.7h / fix ~0.7h — slot-wait dominated throughout, annotated

---

## row_id: G1

- **date**: 2026-08-08
- **task**: PATHS vocabulary growth cheap set: circle/arc_via/arc_center/far-end anchor/exact directors + §2a addendum + corpus migration (raw census → 3)
- **difficulty**: M (logged pre-dispatch)
- **findings**: **DUAL REVIEW (sample #1)** — R1: APPROVE-WITH-FIXES 1/1/3, rubric 5/4/4; R2: APPROVE-WITH-FIXES 1/1/2, rubric 5/4/5. CONVERGED on the identical MAJOR (Zero-fit far-end anchor inherited resolve_fillet's unconditional outgoing tangency declaration → executed spurious TangencyContradicted on §2a-legal sharp continuation; declaration-without-construction, §4 item 2); tails disjoint (R1: §3 table order, footprint prose; R2: t2-vs-anchor vertexhood pin, PQ4-phrasing clause, in-band gate rows). Headlines both verified independently: byte-identity 3 ε rows, disc canonicalization mechanism established, .angle bit-preservation, bracket exact via .toward
- **silent_devs**: 0 silent (R1: 3 reported all honest; R2: 4 reported 0 silent)
- **idiom**: 5/5
- **tests**: 4/4
- **docs**: 4/5
- **fix_pass**: moderate (ArrivalKind enum replaces the seam bool — declares only when something tangent follows, red-checked both directions; t2 absorb rule stated + pinned (emitting the anchor would break two-doors bit-identity — measured call); §2a sentence + PQ4 clause + table order; in-band gate rows; byte-diff re-run clean though not logically required — declared joints are exported); fix-pass executor: implementer-inherited
- **battery**: MERGED #254 27/27; the BRACKET moves bit-identically (VQ4 proven); raw census: boss (measured 3-arc topology), rocker (G2), bowtie (permanent); CI caught interval-square poison (dx*dx→powi(2)) pre-review
- **tokens**: impl ~544k (parked+resumed) / R1 ~188k / R2 ~180k / fix ~335k
- **wall**: impl ~5.3h (incl. slot waits at load ~20) / R1 ~1.9h / R2 ~1.2h / fix ~0.8h (no gaps)

---

## row_id: U3

- **date**: 2026-08-08
- **task**: SectionSegments retirement: loft/sweep speak ProfileLoop; door validation for ALL sections; split-brain closed structurally
- **difficulty**: M (logged pre-draw)
- **findings**: APPROVE 0/0/3, rubric (in report) (all ten claims independently re-executed: byte-identity 3 ε rows on the reviewer's own base build; differential pin red-checked at one ulp; door delta probed both directions — base silently BUILT an invalid interior section, branch refuses SectionProfile typed; no silent open-chain reinterpretation path — the type change forces conscious ports; false tangency declarations refused at the new seam; novel loft volume exact to 5e-16; NOTE-1 = error-precedence flip unstated, NOTE-2 = per-call loop clone, NOTE-3 = probe-count wording)
- **silent_devs**: 0 (3 reported, all verified; NOTE-1 sub-deviation-threshold)
- **idiom**: (in report)
- **tests**: (in report)
- **docs**: (in report)
- **fix_pass**: none required (NOTEs banked as G-series riders); fix-pass executor: n/a
- **battery**: MERGED #245 checks green; SectionSegments DELETED (grep pin), OpenClosedMixed retired, 42 sites migrated, 9 quad() clones collapsed per-crate; NOTE-3 differential row in-repo
- **tokens**: impl ~246k / review ~114k / fix 0 (per-phase per v3 discipline)
- **wall**: impl ~0.96h / review ~0.41h / fix 0 (no gaps)

---

## row_id: U1

- **date**: 2026-08-07
- **task**: pncad façade crate + prelude; tour on ONE kernel dependency
- **difficulty**: S (logged pre-draw)
- **findings**: APPROVE-WITH-FIXES 0/2/3, rubric 5/3/3 (byte-identity of all 89 tour exports independently rebuilt+reproduced at merge-base; closure property HELD under independent sweep but its advertised "no-dev-deps ⇒ physically incapable" proof mechanism executed-FALSE — reviewer's `use topo as _;` probe compiled clean; novel washer authored end-to-end on one dependency, 3 prelude exits, none to a second crate)
- **silent_devs**: 0 (4 reported, all verified)
- **idiom**: 5
- **tests**: 3
- **docs**: 3
- **fix_pass**: moderate (self-scanning guard pin PROVEN by executed falsification; audit +9 rows + 2 honest closure exceptions — serde_json::Value flagged for U9, unnameable DuplicateName a filed kernel wart; Band into prelude per orchestrator ruling; ladder test now tier-3-XOR-3′ with a real-union row; doctests 1→8)
- **battery**: MERGED #232 27/27; P8 dead (six p2 + four validated deleted); SurfaceKind leak closed STRUCTURALLY (zero kernel micro-edits — the §1 permission unused)
- **tokens**: ~158k impl + ~152k review + ~242k fix
- **wall**: ~1h impl + ~1.5h review + ~2.3h fix

---

## row_id: U5

- **date**: 2026-08-09
- **task**: read-back doors: loft_parameters/section_params, name→geometry (face/edge/vertex + denotation), cap doors + Pose, blend_arcs, LB12 seal
- **difficulty**: M (logged pre-dispatch)
- **findings**: **DUAL (sample #3, ordinal 9 fixed at dispatch — first same-head-by-construction pair)** — R1 APPROVE-WITH-FIXES 0/2/5 rubric 5/3/4; R2 APPROVE-WITH-FIXES 0/2/2 rubric 5/4/4. CONVERGED: 0 MAJORs; the SAME doc falsity found independently (blend_arcs keys canonical order not authored — R1 proved the hole-reversal flip); disjoint tails (R1: the module-path key residue = pre-echo of LB13; R2: edge_frame coverage)
- **silent_devs**: 0 silent (6 reported devs, all verified; NURBS-frame refusal fork upheld by both)
- **idiom**: 5/5
- **tests**: 3/4
- **docs**: 4/4
- **fix_pass**: moderate (converged doc fix + fork recorded; InterrogateError ladder tests; LB13 landed IN the pass: curated pncad::document, editor_core re-export dropped, boundary guard falsified-before-trusted — rustdoc-JSON out of fence, source-level fallback with documented blind spots; byte-identity re-confirmed post-LB13; CI caught a ClosedLoop doctest staleness — SWITCH-P had merged mid-pass); executor: implementer-inherited
- **battery**: MERGED #280 27/27; P3's loudest sites are queries+pins; the G1 key boundary is now a TEST
- **tokens**: impl ~235k / R1 ~170k / R2 ~154k / fix ~307k
- **wall**: impl ~0.9h / R1 ~0.7h / R2 ~0.5h / fix ~2.6h

---

## row_id: U9S

- **date**: 2026-08-09
- **task**: Python bindings scaffold: pncad-py (PyO3/maturin abi3), typed quantities, curated document surface, .pyi stubs, D9 pin seed, bracket.py
- **difficulty**: M (logged pre-dispatch)
- **findings**: APPROVE-WITH-FIXES 0/3/4 (wheel rebuilt + venv-installed + D9 pin reproduced by the reviewer; feature-gating verified 0-default/5-gated pyo3 crates; planted-variant tag-match falsification held; MINORs = 4 dropped clippy lints found by manifest diff, the lint drift-check absent (Evan's ask, made a formal claim mid-review), FORK-2 justification misstated)
- **silent_devs**: 0 silent (F1-F6 curated-surface gaps all honestly reported not worked around — the fence held)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: light (lints restored + claim corrected; drift-check test added and FALSIFIED once — string-scan, no new dep; FORK-2 re-grounded on LQ4/SEL1; pyo3 count corrected 4→5); executor: implementer-inherited
- **battery**: MERGED #290 27/27; PYTHON EXISTS: 25*mm → Length, docs evaluate, D9 volume pinned at 0x1.8p+1; F1/F2/F3 feed the curated-doors unit; the demos-via-python goal recorded as the U9/U10 north star
- **tokens**: impl ~244k / review ~262k(incl. limit-kill resume) / fix ~274k
- **wall**: impl ~5h / review ~1.5h active / fix ~0.3h

---

## row_id: SEL1

- **date**: 2026-08-09
- **task**: geometric selectors PR-1: GeomPred exact/decided split, select_where, sel_* census rows, die_composed three-way name-agreement acceptance, tour filters on ratified vocabulary
- **difficulty**: M (logged pre-dispatch)
- **findings**: APPROVE-WITH-FIXES 0/1/2 (all nine claims executed: byte-identity 3×89 sha256, acceptance mutation-killed, k-lint clean, exact paths verified funnel-free; MINOR = GS-Q4 mixed-Tied refusal implemented but untested — reviewer wrote the probe)
- **silent_devs**: 0 silent (D1-D4 reported incl. D3's acceptance relocation to die_composed, orchestrator-accepted mid-unit — a STRONGER acceptance than spec'd)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: light (the tied-trilean row adopted from the reviewer's probe + an exact-only-stays-total row beyond it; 2 doc corrections); executor: implementer-inherited
- **battery**: MERGED #289 27/27; P10 dead at the recipe layer (three independent selector descriptions agree name-for-name); LB7's deferral fully discharged
- **tokens**: impl ~483k (incl. limit-kill resume) / review ~354k (incl. limit-kill resume) / fix ~288k
- **wall**: impl ~2.2h active / review ~1.5h active / fix ~0.4h — limit gaps annotated

---

## row_id: SWITCH-E

- **date**: 2026-08-09
- **task**: profiles-as-programs PR-B: ProfileProgram + Expr steps, schema v4 + display units (U8b), program-anchored naming, VQ3 loop-coord slots, corpus clean-break (circle_split; half-disc declared-subdivision per the load-bearing measurement)
- **difficulty**: XL (logged pre-dispatch)
- **findings**: **DUAL (sample #4, ordinal 12, frozen head)** — R1 APPROVE-WITH-FIXES 2/2/6; R2 NOT-MERGEABLE-AS-IS 2 MAJOR. FULL CONVERGENCE on both MAJORs independently executed (hole-circle naming parity: n=2 position-only matching ambiguous under canonical reversal — the semicircle faces silently swapped program names; clippy-red head = hosted interval battery never ran); verdict LABELS differed on identical findings — calibration data
- **silent_devs**: 0 silent (findings F1-F8 + D1 + §5-1 all reported; F1 exposed the #274 red-main union gap)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: substantial (bulge-bit+joint-set parity matching, both reviewers' probes adopted, corpus names UNCHANGED by the fix; Lit unit-code shrink clears large_enum_variant without API change; binding-sensitive names dump measured 13/15 with the rename class stated; 3 mechanical pncad-py tag arms = the tag tripwire working as designed); executor: implementer-inherited, crash/limit-fragmented
- **battery**: MERGED #291 full matrix green; THE PROGRAM IS THE REPRESENTATION (schema v4, v3 refuses typed, empty migration table); PR-C = honest partial, next dispatch
- **tokens**: impl ~605k+ / R1 ~275k / R2 ~217k / fix ~673k — all limit-fragmented, annotated
- **wall**: wall figures unreliable across 3 limit gaps + the CONFLICTING window

---

## row_id: SEL2

- **date**: 2026-08-10
- **task**: detect/declare protocol: find_flush_candidates (C4 verifier in candidate-gen mode), declare/declare_all sugar (no-fusion boundary), corner-table migration, refusal-menu follow-up reported
- **difficulty**: M (logged pre-dispatch)
- **findings**: APPROVE-WITH-FIXES 0/1/3 (the anti-twin attack LANDED: the detector's verify-arm was hand-mirrored from rest.rs — a planted 2m drift passed the suite undetected; C4's verify-at-use backstop kept it MINOR)
- **silent_devs**: 0 silent (the refusal-menu fence call verified honest)
- **idiom**: (in report)
- **tests**: (in report)
- **docs**: (in report)
- **fix_pass**: moderate (the arm SHARED BY CONSTRUCTION — topo::flush_pair_relation, both callers; the planted-drift shape structurally impossible now; two-sided tilted falsifier row; v4 adaptation post-SWITCH merge); executor: implementer-inherited
- **battery**: MERGED #304 fully green; the P9 class closes at the document layer with detect/declare/menu as ratified; the extraction pattern's FOURTH use
- **tokens**: impl ~284k / review ~128k / fix ~349k — limit-fragmented, annotated
- **wall**: wall fragmented across 2 limit gaps

---

## row_id: DOORS

- **date**: 2026-08-10
- **task**: curated-surface gaps F1-F6: persist doors (v4), document-layer export door, typed node failures, re-export set, Display prose (F6 reopened by review)
- **difficulty**: M (logged pre-dispatch)
- **findings**: **DUAL (sample #5, ordinal 15, frozen head)** — R1 APPROVE-WITH-FIXES 0/3/4; R2 APPROVE-WITH-FIXES 0/2/3. CONVERGED on the wheel-layout stub brittleness AND the Debug-dump message problem — the dual pair's convergence OVERTURNED the implementer's F6 no-change disposition (first sample where dual review changed a design call, not just found defects); R1-unique: the silent LiteralError payload drop
- **silent_devs**: 0 silent (5 reported devs verified; both ran the full Python journey incl. tampered-v4 typed-refusal probes)
- **idiom**: 5/5
- **tests**: 4/4
- **docs**: 4/4
- **fix_pass**: moderate (layout-invariant stub check proven on both layouts; Display in editor-core — 67 concise no-guts arms, prose pinned; LiteralError payload restored; superset conflict resolution vs SEL2); executor: implementer-inherited
- **battery**: MERGED #308 26/1-skip; bracket.py completes the FULL §L3 journey (build → measure → export STEP, step-import as oracle); the bindings' error story is human-readable + machine-typed
- **tokens**: impl ~228k / R1 ~143k / R2 ~258k(incl. parking resumes) / fix ~287k
- **wall**: impl ~0.7h / R1 ~0.6h / R2 fragmented / fix ~0.7h

---

## row_id: PR-C

- **date**: 2026-08-10
- **task**: the v1→program lift tool (refusal-driven, re-implements no predicate; census 8 bit-identical / 2 value-equal / 3 named walls / 0 defects) + plate_param (the parametric acceptance scene — all four §V8 rows)
- **difficulty**: M (logged pre-dispatch)
- **findings**: APPROVE-WITH-FIXES 0/2/3 (census reproduced row-for-row with 3 independent bit-diffs; never-at-load call-graph clean; chord preference probed; the 3-value parametric sweep with stable names executed; MINORs = an untyped refusal assertion + a loose value-equal bound)
- **silent_devs**: 0 silent
- **idiom**: 5
- **tests**: 4
- **docs**: 5
- **fix_pass**: light (typed NonSimple{Crossing} match naming both hole loops — made the engineered separation load-bearing; VALUE_EQUAL bound tightened 2^20→2^12 + the honesty-backstop comment; 3 NOTEs)
- **battery**: MERGED #311 26/26; THE PROFILES-AS-PROGRAMS ARC CLOSES (PR-A #273, PR-B #291, PR-C #311); plate_param demonstrates the program's whole payoff: edit → re-eval → new geometry, refusals name loop+step
- **tokens**: impl ~476k (2 segments) / review ~128k / fix ~266k
- **wall**: impl ~3h / review ~0.5h / fix ~2.3h

---

## row_id: PYG1

- **date**: 2026-08-10
- **task**: Python PATHS lattice (audit G1): 7 typestate classes + Node.profile from the recorded program, stubs w/ first @overload, ty 0.0.39 wired in CI (9/9 illegal-line biconditional), bracket/vase/sheave/bossplate flipped w/ oracles ≤4e-16, guide Python mirror, bracket.py through the lattice
- **difficulty**: M-L (register-sized, logged pre-dispatch)
- **findings**: single (ordinal 20, reviewed head 6046e8b4) — APPROVE 0/3/3, rubric 5/5/4 (reviewer executed a complete Rust impl-block census: both structural rulings — entry-Open/PathOpen split, single PathDirected — proven FORCED by the Rust code; tag-arm mutation E0004-red executed; all four oracles independently re-derived, vase by exact π∫x²dy and sheave by independent numeric integration to 1.6e-14; the kernel Boolean carrier-refusal finding reproduced exactly at the r=4/r=5 crossover → #347; audit arithmetic + two PRE-EXISTING tally errors verified at merge-base; MINORs: bracket.py lacked the in-code carrier invariant, the report's second clippy lane was a non-CI substitution presented as the acceptance row, sheave docstring half-truth)
- **silent_devs**: 2 silent, both cosmetic (`outline` param name vs spec's `loop`; `arc_continue` unlisted delta — both ACCEPTED as-is at adjudication; 6 reported)
- **idiom**: 5
- **tests**: 5
- **docs**: 4
- **fix_pass**: moderate (both implementer-flagged forks RULED ADOPTED and landed in the pass: LoopProgram::from_recorded promoted to editor-core beside its literal siblings — moved not rewritten, new RecordedProgramError, bit-for-bit replay contract test, the door had no test because it had no door; prelude gains ClosedLoop/circle_split; m1 carrier comment citing #347, m3 sheave honesty line, n1 ty date; report corrections m2/n2/s1/s2); executor: implementer-inherited (one waiter-parking incident, nudged, redundant local battery cancelled — hosted is the gate)
- **battery**: MERGED #346 28/28 green; G1 CLOSED — authorable 7→11, NO rows 27→23, python suite 48→83, the ty static gate is live; findings 3/1/2 fed #347 + the G2 unit-cut ruling + the from_recorded door
- **tokens**: impl ~333k / review ~165k / fix ~70k
- **wall**: impl ~1.4h active / review ~3.5h (~2h slot contention, annotated) / fix ~45min active (cancelled slot tail excluded)

---

## row_id: U10

- **date**: 2026-08-10
- **task**: docs/tutorials/corpus-as-examples (the FINAL unit): GUIDE both languages via include_str! doctests + test_guide.py, corpus index, fail-loud tour, executable north-star audit (G1-G11), crate fronts, 6 doc-rot fixes
- **difficulty**: M (logged pre-dispatch)
- **findings**: **DUAL (sample #6, ordinal 18, frozen head)** — R1 APPROVE-WITH-FIXES 0/1/4; R2 APPROVE-WITH-FIXES 0/2/4. CONVERGED on the shipped corpus-count factual error (17→16, both counted the registry independently); R2-unique: the undischarged per-gap pointer clause; the no-rot machinery mutation-proven by both in both languages; projectbox oracle independently re-derived by both
- **silent_devs**: 0 silent (the mid-unit LB13-guard trip honestly reported; the fix-pass count partition self-caught a defect in the audit's own G-counts)
- **idiom**: 5/5
- **tests**: 5/4
- **docs**: 4/4
- **fix_pass**: light (count 16 everywhere; per-gap register pointers incl. G5→R3, G10→R1; NOTEs; report accuracy) — AND the pass CAUGHT the PY-CI red-main venv defect, hotfixed as #332; executor: implementer-inherited
- **battery**: MERGED #318 25/3-skip/0; the guide+audit enforced hosted (46 tests; guide blocks execute from Markdown in CI); R4 discharged
- **tokens**: impl ~270k / R1 ~272k(2 resumes) / R2 ~142k / fix ~567k(2 segments)
- **wall**: slot-wait-dominated, annotated

---

## row_id: R1-PARAMS

- **date**: 2026-08-10
- **task**: named-parameter curation (residual R1/G10): ParamName/DocParam through pncad::document, set_doc_param in Python, guide §3.2 compile_fail flipped to passing, G10 audit row flipped with executed oracle
- **difficulty**: S (logged pre-draw)
- **findings**: single (ordinal 19, RETROACTIVE on frozen head 9bb1916 post-merge) — APPROVE 0/1/4, rubric 5/5/5 (all headline claims independently re-executed at the frozen head: pin green ×3ε and RED under 3 tampers; §3.2 doctest RED under oracle + counter mutations; 48/48 on the reviewer-rebuilt cdylib incl. both TestPlateParam rows; non-finite DocParams construct freely and refuse typed at Doc.apply with NO binding pre-check, located edit.rs:879; LB13 guard RED under an injected EntityKey re-export; audit partition arithmetic + #318 pointer survival verified; MINOR-1 = the pin's ε filter drops EVERY ε-prefixed line vs the doc-comment's "ONE", so a duplicated/corrupted ε line passed the Rust pin — mitigated end-to-end by Python load's typed refusals, CI-run)
- **silent_devs**: 0 silent (3 reported, all disclosed: persistence-door strategy, fixture+pin mechanism, Closed-gaps table; full 9-file diff swept hunk-by-hunk)
- **idiom**: 5
- **tests**: 5
- **docs**: 5
- **fix_pass**: tiny (sans_epsilon now asserts exactly one excluded ε line per side, mutation-verified RED "found 2" on a duplicated line; NOTEs banked: DocParam __eq__/__hash__ asymmetry → bindings rider; LB13 guard's fn-signature blind spot → register note); executor: ORCHESTRATOR-applied (implementer lane retired at handoff)
- **battery**: MERGED #329 all green pre-review (recorded pre-authorization); review re-ran everything at the frozen head; fix pass landed post-merge
- **tokens**: impl ~165k / review ~82k / fix orchestrator-direct (negligible)
- **wall**: impl ~1.1h / review ~0.7h (15:01–15:42Z, one 2-min slot wait, no gaps) / fix ~0.2h

---

## row_id: PY-CI

- **date**: 2026-08-10
- **task**: the python-suite hosted job (wheel + venv + discovery)
- **difficulty**: infra-class (no blinded lane, KLINT-GATE precedent)
- **findings**: validated by the hosted run; POST-MERGE DEFECT: the venv lived in rust-cache's target/ — green on the PR's cold cache, red on main's restored cache; caught by U10's fix pass, hotfixed #332 (venv → RUNNER_TEMP --clear), job green 58-59s since
- **silent_devs**: the defect class recorded: cold-cache-green ≠ warm-cache-green for jobs writing inside cached trees
- **idiom**: —
- **tests**: —
- **docs**: —
- **fix_pass**: n/a
- **battery**: MERGED #326 + #332; the no-rot gate is structural
- **tokens**: impl ~203k + hotfix orchestrator-direct
- **wall**: —

---

## row_id: PYG23A

- **date**: 2026-08-11
- **task**: audit G3 + G2's loft half: SketchPlane::yz()/zx() additive + Python plane values, plane= on both profile doors, Node.loft (Expr::count), 7 audit rows flipped w/ exact oracles, ty fixtures +3, guide blocks
- **difficulty**: M (spec-sized, logged pre-dispatch)
- **findings**: single (ordinal 22, reviewed head 6dc9ec1d; the concurrent-dispatch tiebreak entry above) — APPROVE 0/1/4, rubric 5/4/4 (all six oracles independently re-derived — silhouettes by exact integer grid-count, nonuniform's t from chord geometry, loft 9 analytically; finding 1 CONFIRMED genuine: wire_loft drops section_params, no Section/Affine3 vocabulary; zero-new-test-binaries verified; MINOR = PR-body "12 marked lines" vs 13 in-file; NOTE-2 = the V=8+16d/3 derivation typo, PRE-EXISTING in skinned.rs and copied faithfully — value 9 correct; NOTE-3 = "asserted" oversold a by-construction sharing)
- **silent_devs**: 0 silent substantive (9 reported findings all verified genuine)
- **idiom**: 5
- **tests**: 4
- **docs**: 4
- **fix_pass**: tiny, text-level (count 13; the derivation typo fixed at FIVE sites — the review named 3, repo-grep found demos/README + the corpus twin; sharing reworded by-construction; touched rows re-run 95 OK); executor: implementer-inherited
- **battery**: MERGED #365 (one interval shard red in the #366 billing-outage window, green on re-run — not a code failure); G3 CLOSED + G2's loft half: authorable 11→18, NO 16, suite 83→95; findings feed: loft read-back residue (row 14), origin-less named planes, SketchPlane eq/accessors rider, Count-exception doc note
- **tokens**: impl ~135k + ~55k surveys / review ~126k / fix ~15k
- **wall**: impl ~4.8h wall (verification slot-contended, annotated) / review ~3.0h / fix ~0.3h

---

## row_id: ASM-1

- **date**: 2026-08-11
- **task**: document identity + content pins: DocumentId/ContentPin/DocRef, include-by-default canonical bytes (D-3 amended mid-flight #348), schema v5 clean break + id header line, read-side workspace store with typed DuplicateId/PinMismatch, mechanical-only Python surface
- **difficulty**: M (logged pre-draw)
- **findings**: **DUAL (sample #7, ordinal 21, frozen head f04d08e8)** — R1 MERGEABLE 0/1/4 rubric 5/5/4; R2 APPROVE-WITH-FIXES 0/3/4 rubric 5/4/3. CONVERGED on the one headline gap (doc-metadata preimage inclusion unfalsified) — verdict LABELS differed on identical 0-MAJOR substance (calibration data, SWITCH-E precedent). Disjoint tails: R2 replayed-pin discipline + next_id/undone-insert consequence; R1 skipped-replay assert + Pin error-mapping. Blinding caveat disclosed: R2 glimpsed R1 probe TOOLING (shared-scratchpad script investigation; no findings read)
- **silent_devs**: 0 silent (5 deviations reported in the PR incl. the cross-crate ctor relocation + header-agreement completion)
- **idiom**: 5/5
- **tests**: 5/4
- **docs**: 4/3
- **fix_pass**: moderate (all 5 union items: crafted-save metadata falsifier, logged-fixture replayed-pin falsifier both-directions, next_id consequence into spec D-3 + documenting test, skipped-replay assert, WorkspaceError::Pin arm); executor: implementer-inherited
- **battery**: MERGED #364 28/28 (spanning the #366 billing outage); PINS EXIST: id answers which-part, pin answers which-version, the workspace resolves and refuses typed
- **tokens**: impl ~275k + fix ~266k / R1 ~155k / R2 ~166k+2 lost-wake resumes
- **wall**: impl ~5.9h / R1 ~7.3h / R2 wake-fragmented, annotated

---

## row_id: PYBUNDLE

- **date**: 2026-08-11
- **task**: audit G4/G6/G7/G9 close + riders: Node.fillet/split/transform/datum_plane, multi-loop profile, boolean declare=, all_* materializers, SketchPlane accessors/==, DocParam eq/hash; 5 flips + crosslap YES*→YES; 3 NEW gap ids minted from measured walls (G12/G13/G14); G8 measured-unbound
- **difficulty**: M (logged pre-dispatch)
- **findings**: single (ordinal 28, reviewed head cafa8608) — APPROVE-WITH-FIXES 1/2/3, rubric 5/4/3 (the MAJOR was the round's substance: reviewer EXECUTED full diecomposed from Python by parsing serde name-text provenance markers, falsifying G13's "cannot be said" prose — the strongest audit-honesty catch of the series; all 6 flips reproduced against re-derived oracles; G12 wall verbatim+executed, G14 reproduced; adversarial args refused typed on every new door; MINOR-2 byte-for-byte claim false (save pretty-prints), MINOR-3 diepips chart attribution silently re-attributed a corpus workaround)
- **silent_devs**: 3 silent prose-level (one audit-material — the diepips attribution), 6 reported (incl. the datum_plane + materializers additions, justified: no name SOURCE existed and the spec named only the sink — reviewer verified smallest-faithful)
- **idiom**: 5
- **tests**: 4
- **docs**: 3
- **fix_pass**: moderate (MAJOR RULED both-arms: diecomposed→YES* AND name-text OPAQUE BY CONTRACT with G13 re-scoped to the unbound selector surface — the ruling confirms the no-representation-dependence doctrine; value-equality wording; honest diepips attribution naming the revolve-naming refusal it dodges; __eq__ stub rows; interval clippy re-run observed exit=0); executor: implementer-inherited
- **battery**: MERGED #376 28/28; audit 18→24 of 34 (20 YES + 4 YES*), 10 NO (G2:6 G5:2 G12:1 G14:1); suite 95→118; issues filed: #377 (LoopBuilder/G12 retirement conversation), #380 (NamingError diagnostic swallowed)
- **tokens**: impl ~300k / review ~142k / fix ~318k
- **wall**: impl ~1.5h / review ~2.3h (~70min slot contention annotated) / fix ~1.7h

---

## row_id: ASM-ROOTS

- **date**: 2026-08-11
- **task**: A10 product roots: ordered roots list + coverage/ancestor-freedom invariants (one shared checker at both doors), automatic maintenance incl. replay re-derivation, schema v6 clean break, the product gather (editor-core product.rs) + export_document_step
- **difficulty**: M (logged pre-dispatch)
- **findings**: single (ordinal 25, frozen head 5b5850b2) — MERGEABLE 0/0/5 rubric 5/5/4; every roots::check clause mutation-proven red; deviation 3 (MultiSolidRoot unreachable) survived direct falsification; both fixtures re-blessed byte-identical in the reviewer's own process; e2e STEP round-trip exact
- **silent_devs**: 0 silent (4 deviations reported; the sink-set equivalence observation written into module docs)
- **idiom**: 5
- **tests**: 5
- **docs**: 4
- **fix_pass**: none required (0 MAJ / 0 MIN; 5 NOTEs recorded in the review report, none blocking)
- **battery**: MERGED #383 25/25 (one mid-flight hosted red: the pncad-py python-feature tag-map gap — caught by CI, fixed, and the lesson now rides every future brief: clippy must include -p pncad-py --features python when error surfaces move)
- **tokens**: impl ~280k / review ~249k
- **wall**: impl ~5.3h / review ~4.8h

---

## row_id: ASM-2K

- **date**: 2026-08-11
- **task**: multi-solid instancing kernel door: graft_disjoint_all (equivalence-tested vs sequential single grafts), uniform Instance(i) wrapping pinned for multi-solid masters, step-import zero-diff
- **difficulty**: M (logged pre-dispatch)
- **findings**: **DUAL (sample #8, ordinal 24, frozen head ada35468)** — R1 MERGEABLE 0/2/3 rubric 5/4/4; R2 MERGEABLE 0/1/4 rubric 5/4/5. FULL convergence incl. LABELS (first sample); converged MINOR = the single-solid wording at the N-solid refusal; both independently verified the D-2 deviation TRUE on unmodified main (the wall is output-body-indexed)
- **silent_devs**: 0 silent (4 deviations reported incl. the spec-premise falsification; overlap-validation gap filed as #382; GraftMap name bridge banked to ASM-2b)
- **idiom**: 5/5
- **tests**: 4/4
- **docs**: 4/5
- **fix_pass**: light (invariant-stating refusal split, R7 retirement pointer at the row, partial-write parity doc); executor: implementer-inherited
- **battery**: MERGED #381 (run green; a GitHub status-propagation wedge held the last job in_progress ~35min post-completion — reconciled on poke, nothing ours)
- **tokens**: impl ~211k + fix ~219k / R1 ~131k+resumes / R2 ~108k
- **wall**: impl ~3.4h / R1 wake-fragmented / R2 ~5.3h

