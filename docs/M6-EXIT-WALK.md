# M6 exit walk — criteria vs evidence

**STATUS: RATIFIED — M6 IS CLOSED (Ev, 2026-08-08, PR #243
comment 5224869607: "lgtm!"). This document is M6's done-state of
record. Every criterion was dispositioned against main's tip at
presentation (79db554, the #239 merge — full-matrix run
31242416671 GREEN). Follow-ups from the same ruling: the M6
carried-items register issue, and the k-lint gate flip with the
interpretation-discipline message (Ev's design, same comment).**

**How to read this.** Criteria are quoted **verbatim** from
`docs/M6-PLAN.md` — the six numbered units, the "also in scope when
reached" sentence, and the three-clause exit shape — plus the one
unit RATIFIED into M6 after the plan was assembled (the curved
sense-flip tier gate; Ev 👍 on the #184 triage, recorded in
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
| 6 | Unit 4: "Census/declared-contact design doc (DESIGN-ONLY): the OQ5 deferral + ball-and-socket + interference fits + conformal contact (#161 §2, relocated out of M7 per #169). Co-designed with signed clearance as an M8 forward reference." | MET | **#178** — `docs/CONTACT-DESIGN.md` RATIFIED (C1–C8, closes CURVED OQ5), listed in DESIGN.md's companion table with implementation banked. The signed-clearance co-design is load-bearing for M8's opening (C7 join lane — the ruled M8 opener, Ev 👍 in the #223 thread). |
| 7 | Unit 5: "Edge-selection fillet vocabulary … `Node::Fillet` grows a SELECTION payload — a set of STABLE NAMES (never arena keys; the G1 boundary rule), consuming the banked N4 fillet-naming emitter as substrate … First consumer: the composed die becomes a REGISTERED corpus document (closing dev 1's inexpressibility)." | MET | **#219** (PR-1: emitter + `Vec<StableName>` node + v3 break + die registered, opus arm) + **#220** (PR-2 under Ev's Actions-outage waiver, local batteries the gate; outage debt since CLEARED — main went full-matrix green on a tip containing every waiver-era merge, M7-LOG TIE-UP). No silent name break constructible (sabotage/shuffle/duplicate/uncovered-bump probes all refuse typed); F-e measured: the single-call form works (12 open chains + 1 closed rim, one node). THE COMPOSED DIE IS A REGISTERED CORPUS DOCUMENT — M6-1 dev-1's inexpressibility closed. |
| 8 | Unit 6: "Montage curation (S, parallel anytime): the five banked items + die_pips STEP fixture + corpus/mod.rs stale comment." | MET | Montage curation ran as a SERIES rather than one unit: #170 (curation), #215 (tube cell + count pins; three NURBS scenes honestly stopped at a design boundary), #218 (the mesh trimmed-NURBS lane the stop promoted — NURBS faces render, montage 22 cells), #221 (montage-v2: curated cells, twisted_duct, the conceded s_duct demotion — Ev: "these look great!"), #224 (render-guard: fallback frames structurally uncommittable). **die_pips STEP fixture: CONFIRMED in the CI-gated corpus** (`crates/step-export/tests/common/mod.rs` — its in-tree comment credits the M6 curation unit closing M5 exit-walk row 12's by-hand-only honesty). Corpus comment hygiene flipped with the loft_prism registration (`NODE_KINDS` comment, commit e680e81) and the composed-die registration (37e7faf). | |
| 9 | Ratified addition (not in the plan text; Ev 👍 on the #184 triage): the curved sense-flip tier gate. | MET | **#223** (M6-6, fable, low-M). Check-6 curved arm + import parity rider: every previously-invisible curved sense flip now refuses `CurvedSenseInverted`; inside-out washer/cone/donut/lily certify-green CLOSED. The gate survived a byte-identical 51-row census at three ε, full truth-table re-execution, and nappe adversaries minted on both apex sides. **Recorded residuals ride #226** (conic-trimmed walls slip both gates; rimless-band half-flip; NURBS faces bit-free; arc-bounded planar caps) — each pinned in-tree with its flip condition named; carried items with owners, per row 12. |
| 10 | "Also in scope when reached: the k-lint baseline-floor refresh and the canary-gated latency refresh (banked hygiene); the internal-tangency fixture row (#161 §2c)." | k-lint: MET; latency + internal-tangency: CARRIED | **k-lint floor: MERGED #239** (2026-08-07, fully green, A/B row at merge): floor 1.5e-3 → 4.0e-5 (P0 of the ε-independent ambient definite population, 1.35M samples/row, percentile choice re-argued; the binding family is `volume_backstop` on the composed die's pips — the M5-2 predictions rose out of the way); `props_quad_converged` under its own calibrated rule 4 (|m| < 150·ε); rule 2's definite arm capped at the floor (M7-F1, ruled with the review concurring — the 1e-6 blind window [4e-5, 1e-3) is MEASURED, pinned by adopted probes, and recorded as ε-policy material); refreshed committed baseline byte-reproduced by a cold sweep; hosted advisory row 0 flags at all three ε rows; litmus asserts and fires at every row. The advisory→gating flip is Ev's call — the refreshed baseline would support it (both the fresh sweep and the committed baseline lint to exactly zero). **Latency refresh** and the **internal-tangency fixture row**: CARRIED — never reached; "when reached" was conditional by the plan's own words, and the TIE-UP ruling (Ev 👍, #223 thread) closes M6 at its ratified boundary. Owners: latency refresh stays banked hygiene (opportunistic, canary-gated); internal-tangency fixture rides the C7/M8 contact implementation, whose subject it is (#161 §2c). |
| 11 | Exit shape: "The composed die replaces the two-piece row" | MET | Row 7's registration: the composed die is a registered corpus document through the full ladder. The M5 walk row 8's two-piece honesty is retired — the M5 pin flipped with its history (DESIGN.md roadmap M5 entry). |
| 12 | Exit shape: "every M5-walk carried item closed or explicitly re-banked with grounds" | MET | Closed: row 2 (#176, non-vacuous), row 4 (#192, analytic-chart completion), row 5 (#192 + the #207/#210 honesty in row 4 above), row 8 (#171 + #219), row 12(a) (#192, B_SPLINE_SURFACE_WITH_KNOTS). Re-banked with grounds (DESIGN.md M5-frontier entries, each with recourse text and a named re-open trigger): canal-surface blend (consumer-gated), cyl×sphere germ chords (the lift removed the storage half; the join lane remains), NURBS extent test (argument never derived), curved REST contact implementation (design ratified at #178; implementation = M8's C7 opener per the ruled runway), joined-path sweep lane. |
| 13 | Exit shape: "A/B experiment continues (blocked pairs, blinding, rubric unchanged)" | MET | Every M6 unit row recorded AT MERGE in docs/MODEL-AB-LOG.md (M6-1, M6-2, M6-3, M6-5, M6-6 + the M6-adjacent hygiene/demo rows); record-at-merge held as a merge blocker (post-tie-up bookkeeping verified the table complete on main, two conflict-recovery duplicates deduped). Blinding held (one waiver-class exception process-noted in the log's own cells where orchestrator-review class applied). |

## Walk evidence beyond the criteria

- **The globe lily**: the 15-row disposition table is assembled as
  this walk's appendix (below) — the worked-example evidence. The
  runway ruling is
  https://github.com/evgunter/cad/pull/223#issuecomment-5211450936
  ("close M6 at its ratified boundary", Ev 👍); the lily REBUILD
  rides M8 per that ruling, not M6.
- **#226**: the sense-gate residual classes, each pinned with a named
  flip condition — the walk's carried-items register.
- **Hosted state**: main's tip carries #239 (merge 79db554);
  full-matrix run 31242416671 on it completed GREEN (and the prior
  tip run, fbae459, was green too — no waiver-era debt anywhere
  behind this walk).

## Appendix: the globe-lily disposition table (15 rows)

The lily (#175) was built as a deliberate stress catalogue — eight
closed analytic solids plus fifteen recorded walls. It is the walk's
worked-example evidence: each wall dispositioned against what M6
actually shipped. "CLOSED" = a shipped unit retired it; "CARRIED" =
named owner; "TRIAGE" = recorded API-breadth item with no milestone
obligation (Band 3 / LONGTERM material).

| # | Finding (#175) | Disposition |
|---|---|---|
| 1 | A stem cannot be ONE stem — G1 tube unions refuse (tangent curved contact) | CARRIED → M8's C7 join lane (CONTACT-DESIGN #178 ratified the declared conformal class; the co-design with signed clearance is why C7 opens M8 — Ev's 👍 on the #223 ruling). The lily REBUILD rides there. |
| 2 | A flower cannot grow out of its stem (flower∪stem tangent contact) | CARRIED → same owner as row 1 (corroborated unit 4's design scope at the time — M6-LOG). |
| 3 | Leaves cannot sweep out of their own plane | PARTIALLY CLOSED: curved-path `sweep_body` went live at #210 (first successful caller; #212 made the elbow a corpus fixture). Remaining: the joined-path sweep lane (banked past M6) and the ≥0.5-turn frontier (#222). |
| 4 | A bud cannot be an ovoid (no spheroid surface kind) | TRIAGE: a D3 closed-enum extension question (new analytic kind or NURBS route); no M6 obligation, recorded for feature-breadth planning. |
| 5 | Leaves cannot be mirrored | TRIAGE: reflection instancing is Band-3 breadth (patterns/mirror at the D8 structural level); the equivariance convention (D9 conv. 4) is the design frame when it lands. |
| 6 | A revolved body cannot be filleted at all (seam meridian tangency) | CORROBORATED unit 5's need at the time (M6-LOG); the seam-tangent fillet itself remains a fillet-frontier item — CARRIED with the fillet family's named refusals (DESIGN.md frontier (a) residue). Verified at walk assembly: `FilletError::TangentialEdge` still fires on the Sign::Zero dihedral (`sweep/src/fillet/battery.rs:344`) — the selection vocabulary changed how edges are NAMED, not the tangent-seam refusal. |
| 7 | Tepal seams cannot be carved (tangent subtract) | CARRIED → the tangent-contact boolean class: same C7/M8 family as rows 1-2 (a declared-tangency boolean is contact machinery). |
| 8 | No general-path sweep body | CLOSED: #192 (M6-3 loft/sweep assembly) + #207/#210 (integral skin fit — curved paths live). |
| 9 | No tapering sweep / variable-radius tube | TRIAGE: Band-3 breadth (variable-radius family; canal-surface-adjacent, consumer-gated by the same rule as the canal blend). |
| 10 | No lofted membrane, so no petal | SPLIT: the loft BODY door is CLOSED (#192); a petal MEMBRANE is a sheet body — out of scope by D1 (manifold solids first), recorded, not a gap M6 owed. |
| 11 | `revolve` has no placement (silent sketch-frame landing) | CLOSED: the `tube_along_arc` rider (#192) — world-coordinate intent parameters stored exactly; the 56-ulp reconstruction drift retired with it. |
| 12 | Frame construction unassisted, errors deferred | CARRIED → the F10 arm-policy design (transform rigidity residuals, #214 dimensional-debt family) plus API-ergonomics triage. |
| 13 | One tessellation δ per body, not per feature | TRIAGE: display-mesh-lane material by the ratified distance-only tessellation ruling (D4) — the certified export promise is not the place to fix visual refinement; recorded with measured numbers (#175). |
| 14 | Near-tangency margins untooled (no distance query) | CARRIED → M8 signed clearance (min_clearance is C7's co-designed twin — the design already names this consumer). |
| 15 | `topo::BooleanError` payload not nameable from `topo` | CLOSED by the LIB program's U1 façade (#232): `BooleanError` is exported through the `pncad` prelude (`crates/pncad/src/prelude.rs:62`) — nameable by any consumer; the editor-core sibling wart is separately filed as #234. |

Scorecard: 5 CLOSED (8, 10's body half, 11, 15, and 3's live half),
5 CARRIED with named owners (1, 2, 6, 7, 12, 14 — six rows, five
owners), 4 TRIAGE (4, 5, 9, 13). No wall is unowned; the CARRIED
set concentrates exactly where the ruled runway already points
(C7/M8 contact + clearance).

## Tally

| disposition | count | rows |
|---|--:|---|
| MET | 11 | 1, 2, 3, 5, 6, 7, 8, 9, 11, 12, 13 |
| MET-WITH-RECORDED-HONESTY | 1 | 4 |
| CARRIED (halves of row 10) | 1 | 10 — latency refresh + internal-tangency fixture, owners named; k-lint half pending #239 |

**Closure is Ev's call.** Presented via a docs-only PR; a 👍 on the
explicitly-affordanced closure comment ratifies M6 CLOSED.
