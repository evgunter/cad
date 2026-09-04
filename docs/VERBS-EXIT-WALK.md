# VERBS — exit walk (DRAFT — not ratified, not committed)

**Status: DRAFT, updated 2026-09-04.** First walked against main
`69640aaba` (2026-09-02); re-walked against main `a2bb84386`
(2026-09-04) by `git show origin/main:...` and REST PR state, not
from memory. NOTE since the first walk: **GitHub issues are now
DISABLED repo-wide** (the #1619 tracker migration) — every
issue-numbered disposition below is a `work/` tracker disposition;
the numbers survive only as `github:` fields on item files. The program closes when this walk
is ratified as `docs/VERBS-EXIT-WALK.md`; until then
`docs/VERBS-LOG.md`'s tail is the live status. One structural note
up front: `docs/KERNEL-VERBS.md` is a **reference register** by its
own charter ("the register never schedules anything itself") — it
does NOT close with this program. This walk closes the PROGRAM: the
waves, the claims taken at #1200, and the A/B instrument rows VERBS
recorded. The register survives as the standing home for
missing-verb rows, with VERBS' updates already folded in.

## 1. Program units — the done-state per unit

### Wave 1 (all DELIVERED, pre-window; PRs per the plan)
| unit | state |
|---|---|
| VERBS-RIM | DELIVERED #910 |
| VERBS-CHAMFER | DELIVERED #920 |
| VERBS-ARMS-1/2/3 | DELIVERED #932 / #962 / #1028 |
| VERBS-TUBEWALL | DELIVERED #960 |
| VERBS-RING | DELIVERED #933 |

### Wave 2
| unit | state |
|---|---|
| VERBS-GATE | DELIVERED #1001 |
| VERBS-CYLCYL A/B | DELIVERED #1021 + #1044 |
| VERBS-SPHSPH | DELIVERED #1290, ordinal 108, **sample #66** (promoted 9→before 8 per the CYLCYL survey; the germ class landed, the union honestly did not flip — the pierce door is a layer above any join arm, measured) |
| VERBS-CYLSPH | **DELIVERED #1604, ordinal 112, sample #116** (post-merge correction; three-way #114 draw). The survey found the historical name stale (the route arm implemented since M5 PR 7); what shipped: the exact DECLARED-coaxial classification (Evan's declared-only ruling), the `(Cylinder, Sphere)` germ-frame arm, #974's blocker re-measured and superseded (now structural: `TangentLocus` carries a Line only), the fitted join window a deliberate scope cut. Opening measurement was a TABLE of three doors with the unit's own two guesses refuted by the run and recorded |
| VERBS-CONE (row 10) | OPEN as a plan row, never cut; its C5-section half was EXECUTED by this window (TORAX + C5ARMS PR-1); the cone/torus OPERAND lanes remain unscheduled register territory |

### Wave 3 (all DELIVERED, pre-window)
VERBS-OFF-A #994; OFF-B/C #1003/#1012; OFF-D #1043 + shell #1048;
the teapot demo #1078.

### Wave 4 + the post-survey window
| unit | state |
|---|---|
| VERBS-TESSFOLD | DELIVERED #1045 (sample 81-era row; #1038's class half was then still open — the issue is now CLOSED on main, verify at ratification whose fix closed it) |
| VERBS-DEMO2 | DELIVERED #1054 |
| VERBS-PIERCE | DELIVERED #1068, sample #25 |
| VERBS-TEAPOT | DELIVERED #1078→#26 (sample #100-era numbering: #26) |
| VERBS-SHELLFIX 1 / 2a / 2b | DELIVERED #1099 (#34) / #1126 (#36) / #1180 (#42) — #1081 and #1082 both CLOSED by the arc; the teapot ships whole |
| VERBS-LILYWELD PR-1 | DELIVERED #1109 (#35); PR-2 closed as a measurement that dissolved its own premise |
| VERBS-F7POLE | DELIVERED #1131 (#37) — #1031's pole half; #1031 stays OPEN for half B (below) |
| VERBS-GERMARMS PR-1 | DELIVERED #1229, ordinal 106, **sample #51** (ring lane; sagitta charge in at the union) |
| VERBS-AZIMUTH | DELIVERED #1256, ordinal 107, **sample #60** — **#1077 CLOSED** (verified). The fix pass also caught and fixed a real order-dependence regression against main's composition rows (seam re-anchor) |
| VERBS-GERMARMS PR-2 | DELIVERED #1353, ordinal 109, **sample #86** — the re-cut spec's TYPED-REFUSAL branch: `GermFrameCylinderPinch`, pinch theorem proven family-wide; the chord-lane STOP fired and became the #1377 sequencing; the Placement unification with S-MATE #1417 landed here |
| VERBS-TORAX | DELIVERED #1494, ordinal 110, **sample #102** (after three renumbers — process note in §3). The elbow-split STOP fired: barrel/teapot half shipped; the klein-elbow half waits on the rim capability (below) |
| VERBS-C5ARMS PR-1 | **DELIVERED #1577, ordinal 111, sample #111** — the full arc closed: STOP at first dispatch → TORAX specced/delivered as the enabler → resume → clean dual (no tally candidates; principal = the ring guard's missing `r > 0` half, bilateral) → fix pass (`pt_tube_guard` red-measured-first) → merged. `(Plane, Torus)` is Closed+implemented |
| VERBS-C5ARMS PR-2 (cone×cylinder) | OPEN — specced in the ratified C5ARMS spec, not dispatched; its consumers (rows 5/6/7/7b) verified unmoved by PR-1 |
| VERBS-RIMCAP PR-1 | **DELIVERED #1674, ordinal 113, sample #123** (no collision). The sphere rim capability: the meridian-pair + carried-datum arms, the inline off-axis-circle mint, the mechanism hypothesis instrumented CONFIRMED by both reviewers independently; ONE declared deviation with a forcing red-the-day row (the operand lune is tier-3 `VolumeUncomputable` today — `props_band_coplanar`; hollow shipped at the public direct door; schedule filed `work/issues/sphere-flux-arm-refuses-partial-bands.md`). The TORUS half (spiric carrier) stays a design conversation inside the spec; it owns the klein elbow and C5ARMS rows 3/4/8 |
| VERBS-1031B | **DELIVERED #1671, ordinal 114, sample #125** (post-merge correction; M10-7 held #124). Evan's option-(1) ruling executed as a PORT of join's `run_term` (the spec's own discovery — three sites, not four; reported not silently fixed). THE TEAPOT CUP MERGES; the register's gate-admission deferral CLOSED BY CITATION (verified in `docs/KERNEL-VERBS.md`'s breadth row on main); the honestly-declared assigner/checker divergence filed as `work/verbs/verbs-1031b-assigner-checker-divergence.md` with the refusal-surface measurement as the follow-up's opening step. **#1031 (github field) is closable at ratification** — pole half #1131, coplanar half here |

## 2. Claims taken at #1200 — disposition

| claim | state |
|---|---|
| #347's remaining half (germ arms) | MEASURED-AND-REFUSED + narrowed: GERMARMS PR-1/PR-2 delivered the ring lane and the typed pinch refusal; **#347 remains OPEN** (verified) — its union demand narrows to the circle×wall residue (design-gated, unscheduled) and the pinch family (Evan's must-support ruling → #1377) |
| #1031 half B (ordinary coplanar pair, full-valence edge) | **DELIVERED.** The opening measurement was taken 2026-09-03 (split verdict: the lily class DISSOLVED to the curved-pierce substrate; the cup class STOOD on one missing arc-bounded winding arm), Evan ruled option (1), and VERBS-1031B delivered it (#1671, above). The seam-straightness question is answered on main's own register text (a latitude annulus; the meridian gloss was wrong) |
| #1076 | **OPEN (verified) — never dispatched.** Ratification needs a close/transfer/execute ruling. (It rode the #1200 claim with #1077; #1077 closed via AZIMUTH, #1076 did not get a unit) |
| #1077 | DELIVERED — CLOSED at AZIMUTH's merge (verified) |

## 3. The A/B program instrument — state of record

- **v6 dual ordinals recorded by VERBS**: 85 (PIERCE, the first v6
  pair) and the unbroken run 100–114 — sixteen duals; ordinals
  79–84 predate the dual instrument (single-review era rows).
- **Tally: 1/8** (F7's R1 output-stage MAJ). **Unilateral candidates
  pending the blinded coding session — the log of record says
  SEVEN; the enumerable set from the adjudication entries is SIX**
  (a count discrepancy the coding session's materials prep must
  reconcile against the ledger rows before the readout; flagged
  here rather than papered over):
  1. ordinal 107, R1: the f64 "bit-identical everywhere" claim
     falsified at the parameter level (claim/test-gap; AZ-1 row);
  2. ordinal 107, R2: the |δ|=π endpoint-anchored 2π disagreement
     invisible to the point-comparison guard (test-gap/code; AZ-1);
  3. ordinal 108, R2: the `bool_sphere_trim*` acceptance rows in
     zero test files, undeclared (test-gap, grep-proven; SS-1);
  4. ordinal 110, R2: the "planted red" exercising none of the
     unit's code (test-gap, mutation-proven; TX-1);
  5. ordinal 112, R2: the opening row's sphere-face half pinned
     vacuously by static Display literals (test-gap, demonstrated;
     CS-1);
  6. ordinal 113, R1: `TogetherEdgeDisagreement`'s public doc and
     Display asserting a cause its three raising sites do not share
     (doc/contract-API — 3b weighed at the coding; RC-1).
  Bilateral principals (NOT candidates, recorded for calibration):
  106 (sagitta charge), 109 (the certified-scalar "both arms" pin),
  111 (the ring guard), 112 MAJ-2 (the unpinned factored form),
  114 (the assigner/checker schedule artifact). **The stopping rule
  fires at 8 confirmed unilaterals; the coding session — the
  analysis agent's, structurally not the orchestrator's — decides.**
- **Fable 5.1 era boundary (2026-09-01, ledger note of record)**:
  fable rows near the mark need era assignment before any readout —
  the enumerated near-boundary rows are in the ledger note
  (ordinal-107 redispatch and 108-R2 before-near; 110-R1 and the
  C5ARMS implementer SPANNING; 111-R1 and later after). Rows 112–114
  fable arms are cleanly post-5.1.
- **Ratio-change question PENDING with Evan** (recorded in the same
  ledger note): switching the implementer arms from {opus×3, fable}
  (1:3) back to fable:opus 1:2 — takes effect at the next block draw
  if ratified; VERBS-8's drawn slots executed as drawn.
- **Pairs**: clean pairs 3 (TEAPOT, F7, SHELLFIX-2B); counting-but-
  flagged from this window: 106; 107 (R1 process death + fresh
  redispatch + zombie-injection quarantined; R2 limit death, rubric
  recovered); 108 (R2 resumed); 109 (the mid-review method
  correction — R2's battery killed under Evan's suites-via-CI
  ruling, an evidence-source asymmetry); 110–114 (112's R2, 113's
  both arms, and 114's R2 each carried an outage/limit
  death-and-resume, lanes verified pristine). Exclusions on record:
  PIERCE (3e), SHELLFIX-1/2a (asymmetric allowances), LILYWELD-1
  (3e).
- **Block-draw deviations**: VERBS-4 and VERBS-6 both executed
  opus×4 against their draws (ledger-recorded; remedy hardened to
  per-slot arms read at dispatch — VERBS-7 slot 4's fable and all
  four VERBS-8 slots executed correctly under it).
- **Process note worth Evan's eye — the sample-number collision
  cascade.** The assign-at-merge rule met two merge-heavy weeks and
  produced: AZ-1 one pre-merge renumber (#52→#60); GA-2 two
  post-merge corrections (#81→#85→#86) PLUS a duplicate-row dedup
  (#1594 — a union resolve had kept both the stale and corrected
  rows on main); TX-1 three renumbers (#97→#100→#102); CS-1 one
  post-merge correction (#114→#116, a three-way concurrent draw)
  and one false-alarm dropped-row scare; WB-1 one post-merge
  correction (#124→#125). The rule held — main's merge order stayed
  authoritative every time — but each collision costs a correction
  PR and a gate cycle. **Recommendation for the protocol
  conversation: recorders stop pre-drawing numbers entirely; rows
  land sample-less and a single post-merge sweep (or the analysis
  agent's materials prep) assigns numbers in main's commit order.**
- ~~INCONSISTENCY: two GA-2 rows on main~~ — **FIXED** (#1594
  merged; re-verified at `a2bb84386`: one GA-2 row, sample #86).

## 4. Transfers and standing handoffs (verified state)

- **S-BLEND**: fillet residue ceded with handoff records on #1200
  (#1022→A3-2's measured record; #827 from LILYWELD's
  JunctionTangent payload). #1244 (concave closed-rim band) and
  #1245 (repaired pole-touching body served by neither door) OPEN,
  S-BLEND-territory.
- **S-CERT**: #723/#893 ceded; #723 fixed by S-CERT pre-SPHSPH;
  #893's lever half is now PINNED by SPHSPH's fix pass
  (mutation-red row) — the cession stands for the props remainder
  (the rimless-lune Δu=π arm, banked as a props-unit finding in
  SPHSPH's record).
- **S-MATE**: #968 ceded (the #966 record + LILYWELD's killed-rung
  context); the vertex_on_curved_face Placement unification is DONE
  (settled on the #1353 thread, landed with GERMARMS PR-2's fix
  pass).
- **Design conversations, Evan-gated, filed by VERBS this window**:
  **#1372** (parameter identity — Evan's same-parameter direction
  recorded; the SEAT orchestrator has since posted a concrete
  mechanism proposal riding PR #1388; OPEN, verified) and **#1377**
  (pinch machinery — Evan's must-support ruling; sequenced
  #1372 → chord-lane widening → pinch; OPEN, verified). Both are
  TRANSFERRED-to-design-track, not VERBS execution items.
- **The rim-construction capability**: the SPHERE half DELIVERED
  (VERBS-RIMCAP PR-1, #1674); the TORUS half (the spiric carrier)
  is a design conversation carried inside `docs/VERBS-RIMCAP-SPEC.md`
  — it owns C5ARMS rows 3/4/8 (the klein elbow) and the klein-elbow
  half of TORAX. The former filing obligation is discharged by the
  spec + the tracker item; what remains is Evan-gated design.
- **Cross-program filings by VERBS, open and owned elsewhere**:
  #1291 (ring-join NeitherContained, parked with evidence), #1342
  (E6 k-probe red isolated to main, E6 unit owns), #1449 (demos
  dead-const, d485124ca's lane owns), #1288 (k-lint census — OPEN
  still, verified; the census half was fixed on this window's heads
  but the issue remains open), #986 (hollow-ring STEP export — OPEN,
  three probes retire together when the classifier widens), #1055 /
  #1056 (shell residue with Evan's rulings recorded: planar
  clearance gate + curved window; refuse hollow operands now,
  thicken-every-boundary eventually), #1058, #1018/#1019/#1020
  (Approx-face lanes), #795 (demo exit-code question, still open).

## 5. Register rows — net movement this window (the register keeps them)

- Curved boolean breadth row: sphere×sphere germ class landed
  (SPHSPH); the C5 SECTION table gained plane×torus
  (C5ARMS PR-1, pending merge) after TORAX retired the corner door
  for axial bodies; the pinch family refuses typed
  (GERMARMS PR-2); cyl×cyl ring lane landed (PR-1). Remaining:
  cyl×sphere (CYLSPH, undispatched), cone×cylinder (C5ARMS PR-2,
  specced), the operand lanes, the chord-lane widening (#1377 step 2).
- Shell rows: #1081/#1082 closed; TORAX widened the axial door to
  the torus kind; the honest boundary moved from "a torus never
  reaches the door" to "the partial revolve's circle-profile rim".
- The scope-limits section absorbed the teapot wall-1 retirement and
  the klein-elbow rim record (C5ARMS PR-1's fix pass is carrying the
  final text corrections at this writing).

## 6. OPEN list — must close or transfer before ratification

1. **The blinded coding session** for the tally candidates runs
   (the analysis agent's, with the seven-vs-six count reconciled
   from the ledger during materials prep), or is explicitly handed
   to Evan's A/B analysis with the ledger pointers. The stopping
   rule (8 confirmed) is live at 1 confirmed + up to 6-7 candidates.
2. **Tracker sync to main**: the item closes for VERBS-CYLSPH,
   VERBS-RIMCAP and VERBS-1031B live on `mngr/kernel-verbs` only —
   main still shows all three `status: review` (verified at
   `a2bb84386`). One docs PR.
3. **Design conversations, Evan-gated** (transfers, not execution):
   #1372 parameter identity (SEAT's mechanism proposal pending
   Evan's ruling), #1377 pinch machinery (sequenced
   #1372 → chord-lane widening → pinch), the RIMCAP torus half /
   spiric carrier (owns the klein elbow + C5ARMS rows 3/4/8).
4. **Scheduled follow-ups with committed artifacts** (verify owners
   at ratification): `work/verbs/verbs-1031b-assigner-checker-
   divergence.md` (the check-6 port, refusal-surface measurement
   first), `work/issues/sphere-flux-arm-refuses-partial-bands.md`
   (the props inventory; RIMCAP's red-the-day row forces the flip).
5. **#1076 (github field)**: never dispatched — execute, close, or
   transfer with a handoff record.
6. **C5ARMS PR-2 (cone×cylinder)**: execute (specced and small) or
   transfer with the spec.
7. **The ratio-change question** (fable:opus 1:2) — Evan rules or
   declines; recorded pending in the ledger.
8. **The sample-numbering process note** (§3) is raised as a
   protocol conversation or explicitly declined.
9. Retroactive 👍s Evan owes (F7 arc in #1131's body; A3-2 in
   #1042) — nudge or drop, but the walk should state which.
10. Transfers per the #1200 map re-verified at ratification
    (S-BLEND / S-CERT / S-MATE rows in §4 held at this walk).
