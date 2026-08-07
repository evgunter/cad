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

