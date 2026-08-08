# M6 exit walk — criteria vs evidence

**STATUS: DRAFT — assembled by the orchestrator ahead of the k-lint
floor merge (#239 in review). Cells marked [VERIFY] are pending
re-verification against main's tip before this walk is presented to
Evan. Closure is Evan's call at the walk; this document becomes the
done-state of record only on his 👍.**

**How to read this.** Criteria are quoted **verbatim** from
`docs/M6-PLAN.md` — the six numbered units, the "also in scope when
reached" sentence, and the three-clause exit shape — plus the one
unit RATIFIED into M6 after the plan was assembled (the curved
sense-flip tier gate; Evan 👍 on the #184 triage, recorded in
M6-LOG's stranded-state salvage entry). Dispositions follow the
M5-EXIT-WALK convention:

- **MET** — the criterion as written is satisfied, with evidence.
- **MET-WITH-RECORDED-HONESTY** — substance satisfied; the gap
  between the words and the shipped state is named, not smoothed.
- **CARRIED** — not met; carried to a named owner.

## The walk

| # | Criterion (verbatim from M6-PLAN) | Disposition | Evidence / honesty note |
|---|---|---|---|
| 1 | Unit 1: "Composition surgery … in-place edge-blend surgery — face split along stored trimlines, ring carry-through, rim-edge→torus-band replacement. Closes walk row 8: THE COMPOSED DIE (blank + 21 pips + filleted pip rims) through the full ladder (tier-3, watertight, certified volume, STEP, FreeCAD, tour, corpus)." | MET | **#171** (M6-1, 2026-08-04, fable arm, A/B row at merge). `sweep::fillet::surgery` splits support faces along stored trimlines with rings carried through and replaces rim chains with slit-seamed torus bands — the composed die is ONE body (DESIGN.md M5-frontier (a) marked DISCHARGED). Volume confirmed 3 independent ways incl. 4e8-sample MC; FreeCAD agreement to 1e-6 mm³. Full-ladder corpus registration completed at unit 5 (#219) once the selection vocabulary made the die EXPRESSIBLE — see row 5. |
| 2 | Unit 1 rider: "Optional rider if cheap: the circle-carrier definite-miss bound (door A's honest arm)." | MET | Landed with #171: the curved-pierce door's conic arm is a genuine clearance verdict for CIRCLE carriers (`bool_circle_curved_clearance`; rider falsifier clean over 3000 pairs — M6-LOG). What remains typed there (ellipse/NURBS carriers, crossing circles, partial-sphere extent) is recorded in DESIGN.md frontier (a) — a rider delivered at its promised scope, not a gap. |
| 3 | Unit 2: "SSI generic-T lift: Box3/project/certify_branch → T: Real; unblocks Pcurve::Fitted. Acceptance owns the non-vacuous fitted-cache-at-rest row (walk row 2)." | MET | **#176** (M6-2, 2026-08-04, opus arm). `ssi::enclose`/`certify`/`certify_rung3` and `NurbsSurface::project` lifted (`T: Bounds` seam, `SsiCertificate<T>`); `Pcurve::Fitted` landed with the `PcurveFittedLane` static split. M5 walk row 2's banked obligation DISCHARGED non-vacuously: a cylinder×sphere rung-3 edge reaches a body at rest with the full C2 certificate re-derived at rest, at f64 AND at Interval; `no_body_at_rest_carries_a_nurbs_carrier_or_face` flipped to its successor law. Hosted 27/27 incl. interval shards. |
| 4 | Unit 3: "Loft/sweep body assembly: PR 10's §3 builder + both tier-3 Nurbs-face flips + certify resolve acceptance + NURBS-patch flux door + B_SPLINE_SURFACE_WITH_KNOTS export + the analytic-chart pcurve completion (sphere/cone/torus — walk row 4). Closes walk rows 4, 5, 12(a); shape (iii)'s loft body and the cut-loft row go green here." | MET-WITH-RECORDED-HONESTY | **#192** (M6-3, 2026-08-05, fable; partial by the dead M6 session's implementer, completed post-pickup — quality seam examined at review, all four honesty defects in completion scope, partial spotless). Loft/sweep bodies live (`sweep::loft_body`/`sweep_body`, IsoCurve seams, exact NURBS flux for non-rational walls); analytic-chart pcurve completion landed (ball/cone/donut + die octants carry stored pcurves at rest — M5 walk row 4 CLOSED); B_SPLINE_SURFACE_WITH_KNOTS both arms (unblocked M7-3). **The honesty (recorded in DESIGN.md frontier (c) by the #207 correction): "closes walk row 5" was overstated at merge** — the skin fit synthesized a rational weight channel, so curved-path sweeps and non-uniformly-spaced lofts refused at `nurbs_span_meter` until the SKINFIT unit (#210, M7-era) made integral input skins exactly-unit-weight; the M6-3 closure as merged covered straight-path sweeps and uniform lofts. Rational walls (arc-bearing profiles) still refuse typed at the weights gate — the flux/area lane stays banked with recourse text. MINT-side wiring of the fitted general-circle route and the cone/torus oblique classes remain open, named in DESIGN.md. |
| 5 | Unit 3 rider: "`tube_along_arc(arc, minor_radius)` — a world-coordinate direct tube/torus door storing intent parameters exactly … (no semantic fork between doors)." | MET | Rider landed in #192; lily finding 11 (silent sketch-frame placement) and the revolve minor-radius 56-ulp reconstruction drift retired by it (M7-LOG M6-3 entry). |
| 6 | Unit 4: "Census/declared-contact design doc (DESIGN-ONLY): the OQ5 deferral + ball-and-socket + interference fits + conformal contact (#161 §2, relocated out of M7 per #169). Co-designed with signed clearance as an M8 forward reference." | MET | **#178** — `docs/CONTACT-DESIGN.md` RATIFIED (C1–C8, closes CURVED OQ5), listed in DESIGN.md's companion table with implementation banked. The signed-clearance co-design is load-bearing for M8's opening (C7 join lane — the ruled M8 opener, Evan 👍 in the #223 thread). |
| 7 | Unit 5: "Edge-selection fillet vocabulary … `Node::Fillet` grows a SELECTION payload — a set of STABLE NAMES (never arena keys; the G1 boundary rule), consuming the banked N4 fillet-naming emitter as substrate … First consumer: the composed die becomes a REGISTERED corpus document (closing dev 1's inexpressibility)." | MET | **#219** (PR-1: emitter + `Vec<StableName>` node + v3 break + die registered, opus arm) + **#220** (PR-2 under Evan's Actions-outage waiver, local batteries the gate; outage debt since CLEARED — main went full-matrix green on a tip containing every waiver-era merge, M7-LOG TIE-UP). No silent name break constructible (sabotage/shuffle/duplicate/uncovered-bump probes all refuse typed); F-e measured: the single-call form works (12 open chains + 1 closed rim, one node). THE COMPOSED DIE IS A REGISTERED CORPUS DOCUMENT — M6-1 dev-1's inexpressibility closed. |
| 8 | Unit 6: "Montage curation (S, parallel anytime): the five banked items + die_pips STEP fixture + corpus/mod.rs stale comment." | MET-WITH-RECORDED-HONESTY [VERIFY: die_pips STEP fixture + corpus/mod.rs comment status] | Montage curation ran as a SERIES rather than one unit: #170 (curation), #215 (tube cell + count pins; three NURBS scenes honestly stopped at a design boundary), #218 (the mesh trimmed-NURBS lane the stop promoted — NURBS faces render, montage 22 cells), #221 (montage-v2: curated cells, twisted_duct, the conceded s_duct demotion — Evan: "these look great!"), #224 (render-guard: fallback frames structurally uncommittable). [VERIFY] whether the die_pips STEP fixture and the corpus/mod.rs stale-comment item landed in that series or need a CARRIED line with an owner. |
| 9 | Ratified addition (not in the plan text; Evan 👍 on the #184 triage): the curved sense-flip tier gate. | MET | **#223** (M6-6, fable, low-M). Check-6 curved arm + import parity rider: every previously-invisible curved sense flip now refuses `CurvedSenseInverted`; inside-out washer/cone/donut/lily certify-green CLOSED. The gate survived a byte-identical 51-row census at three ε, full truth-table re-execution, and nappe adversaries minted on both apex sides. **Recorded residuals ride #226** (conic-trimmed walls slip both gates; rimless-band half-flip; NURBS faces bit-free; arc-bounded planar caps) — each pinned in-tree with its flip condition named; carried items with owners, per row 12. |
| 10 | "Also in scope when reached: the k-lint baseline-floor refresh and the canary-gated latency refresh (banked hygiene); the internal-tangency fixture row (#161 §2c)." | k-lint: pending #239 [VERIFY at merge]; latency + internal-tangency: CARRIED | **k-lint floor**: #239 (in review at draft time) — floor 1.5e-3 → 4.0e-5 (P0 of the ε-independent population, re-argued), the ε-coupled family under its own calibrated rule, refreshed committed baseline, 0 advisory flags at all three ε rows, litmus intact. [VERIFY: merge + A/B row before presenting.] **Latency refresh** and the **internal-tangency fixture row**: CARRIED — never reached; "when reached" was conditional by the plan's own words, and the TIE-UP ruling (Evan 👍, #223 thread) closes M6 at its ratified boundary. Owners: latency refresh stays banked hygiene (opportunistic, canary-gated); internal-tangency fixture rides the C7/M8 contact implementation, whose subject it is (#161 §2c). |
| 11 | Exit shape: "The composed die replaces the two-piece row" | MET | Row 7's registration: the composed die is a registered corpus document through the full ladder. The M5 walk row 8's two-piece honesty is retired — the M5 pin flipped with its history (DESIGN.md roadmap M5 entry). |
| 12 | Exit shape: "every M5-walk carried item closed or explicitly re-banked with grounds" | MET | Closed: row 2 (#176, non-vacuous), row 4 (#192, analytic-chart completion), row 5 (#192 + the #207/#210 honesty in row 4 above), row 8 (#171 + #219), row 12(a) (#192, B_SPLINE_SURFACE_WITH_KNOTS). Re-banked with grounds (DESIGN.md M5-frontier entries, each with recourse text and a named re-open trigger): canal-surface blend (consumer-gated), cyl×sphere germ chords (the lift removed the storage half; the join lane remains), NURBS extent test (argument never derived), curved REST contact implementation (design ratified at #178; implementation = M8's C7 opener per the ruled runway), joined-path sweep lane. |
| 13 | Exit shape: "A/B experiment continues (blocked pairs, blinding, rubric unchanged)" | MET | Every M6 unit row recorded AT MERGE in docs/MODEL-AB-LOG.md (M6-1, M6-2, M6-3, M6-5, M6-6 + the M6-adjacent hygiene/demo rows); record-at-merge held as a merge blocker (post-tie-up bookkeeping verified the table complete on main, two conflict-recovery duplicates deduped). Blinding held (one waiver-class exception process-noted in the log's own cells where orchestrator-review class applied). |

## Walk evidence beyond the criteria

- **The globe lily**: the 15-row disposition table (scorecard in the
  #223 thread — [VERIFY: link the comment permalink]) is the walk's
  worked-example evidence; the lily REBUILD rides M8 per the ruled
  runway, not M6.
- **#226**: the sense-gate residual classes, each pinned with a named
  flip condition — the walk's carried-items register.
- **Hosted state**: [VERIFY at presentation: main's tip run
  full-matrix green; cite run id.]

## Tally (draft — finalize after [VERIFY] cells)

| disposition | count | rows |
|---|--:|---|
| MET | 9 | 1, 2, 3, 5, 6, 7, 9, 11, 12, 13 (10 rows — recount at finalize) |
| MET-WITH-RECORDED-HONESTY | 2 | 4, 8 |
| CARRIED | 1 | 10 (latency + internal-tangency halves) |

**Closure is Evan's call.** Presented via a docs-only PR; a 👍 on the
explicitly-affordanced closure comment ratifies M6 CLOSED.
