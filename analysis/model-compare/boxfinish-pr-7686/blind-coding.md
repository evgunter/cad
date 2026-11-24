# Correspondence coding — PR 7686, reviews A and B (blind)

Sources: `A.md` and `B.md` only (plus rules 6 and 7 of `docs/DUAL-REVIEW-PROTOCOL.md` for the instrument). No code, history or other analysis was read; nothing here is checked against the tree.

Evidence codes: **EXEC** = the reviewer executed something that demonstrates it (red row, probe, differential, mutant, measured value); **INSP** = by inspection or argued; **unstated** = the report does not say.
Classes: `code`, `test-gap`, `contract-API`, `doc-or-claim-only`.
Severity labels are the report's own: MAJOR / MINOR / NOTE / Style, or "remark" for a defect asserted inside a claim line with no severity.

## 1. Verdicts as written

| | Verdict | MAJOR | MINOR | NOTE | Style items | Rubric (idiom / test / doc) |
|---|---|---|---|---|---|---|
| A | NOT-MERGEABLE-AS-IS | 2 | 8 | 3 | 12 bullets (16 with the five stale-citation sub-items counted singly) | 4 / 3 / 3 |
| B | NOT-MERGEABLE-AS-IS (says APPROVE-WITH-FIXES once its two MAJORs are cleared) | 2 | 8 | 5 | 11 (S1–S11) | 3 / 3 / 3 |

## 2. Bilateral headline

- **A profile row green on base is red on head at ε 1e-12.** Both name the same row (`a_far_arcs_lens_reads_its_own_width::a_far_arcs_lens_past_ten_eps_wide_validates`), the same payload (`ArcBelowSceneResolution { OnCarrier, margin 6.10e-11, headroom ≈ −9.9e5 }`), the same site (`seg::check_carrier` re-posed to `off_carrier`), and the same reason the author missed it (profile differential run at 1e-9 only). Both MAJOR, both EXEC. A adds a one-line revert that greens the row and a rounding derivation; B adds default-ε instances at other doors (its m1).
- **No hosted gate on the head.** Both: the only check is `merge ref` = failure, PR conflicts with main, CI created no run; both ran the lint job's steps locally, all green. Both MAJOR. B adds the seven conflicting files.
- **`sugar.rs` "the enclosing-class gate decided it non-negative" is false**; dropping `abs` moves an in-band negative ρ by up to 2|ρ|, unlisted in the body. Both MINOR, both INSP "likely".
- **The body's receipts undercount the D-tab** (four theorems lost, not two; `circle_side`'s loss cites no row). Both MINOR.
- **Both verdicts and the two MAJORs coincide**; all four MAJORs are bilateral.

## 3. Coding table

### 3a. BILATERAL (17)

| # | Underlying defect | A: where / severity | B: where / severity | Class | Evidence A / B | Severity divergence / note |
|---|---|---|---|---|---|---|
| B1 | Rationalized gap at far radius moves an `f64` decision the body does not list; far-arc lens row red at ε 1e-12 | MAJOR-1 / MAJOR | M2 / MAJOR, plus m1 / MINOR | code | EXEC / EXEC | Red row: none. Spread to other re-posed readers: A inside MAJOR-1, INSP "likely"; B as m1 MINOR, EXEC (5040 documents, 71 stage changes). Folded as one defect (B: "M2 is its ε = 1e-12 instance") |
| B2 | No hosted CI run on the head; `merge ref` failed, PR conflicts with main | MAJOR-2 / MAJOR | M1 / MAJOR | test-gap (gate; fits no class cleanly) | EXEC / EXEC | None. B adds the 7 conflicting files (4 re-cut pin files) |
| B3 | `sugar.rs` gate comment false; in-band ρ < 0 moves by up to 2|ρ| at `f64`, unlisted, no row | MINOR-3 (+ Style Q2/Q4) / MINOR | m3 / MINOR | code | INSP / INSP | None. A adds an 80-case fillet sweep that moved no decision (EXEC) |
| B4 | Body's receipts: D-tab loses four theorems, not two (`arc_span` [6,4]→[4,6]); `circle_side` loss cites no row | MINOR-1 / MINOR | m4 (bullets 2, 3) / MINOR | doc-or-claim-only | INSP / INSP | None. A alone adds the `pcurve_*_chart_incident` gains ("gains eight"); B's other m4 bullet is 3c-C2 |
| B5 | Negative table radius: `InconsistentArc { OnCarrier }` → `Escalated` (invalid margin), unlisted, test-only door | MINOR-8 / MINOR | n1 / NOTE | code | EXEC / EXEC | MINOR vs NOTE |
| B6 | Tie arm (`BuiltArc::lowered`, `turn: None`) is reached by no row; the design said a tie refuses | MINOR-4 (claim 4 "broken as a schedule"; Style Q3, Q6) / MINOR | claim 4 remark / none (claim "Held") | test-gap | EXEC / EXEC | MINOR vs unranked remark under a held claim |
| B7 | Reader sweep incomplete: a raw `radius − rim` circle-pair depth in `seg.rs` is unlisted | MINOR-5 (first site) / MINOR | claim 2 remark (`seg.rs:2497`, "right in value") / none | code | INSP / INSP | MINOR vs unranked remark; claim 2 "broken in part" vs "Mostly held". A's second site is 3b-A2 |
| B8 | Flagged-and-kept fillet sites: `copysign(rho)` and setback `half_tan.abs()` | MINOR-6 ("have no row") / MINOR | n4 ("pre-existing"; second held by a `work/` row) / NOTE | code | INSP / INSP | MINOR vs NOTE, and a fact contradiction (§5.2) |
| B9 | Stale doc text in facts this change moved; shared sites: `validate.rs` "`Seg::radius` is `signed_radius.abs()`", `sym11_exact_channel_rows.rs` "link replays whole" over a refusing pin | Style Q4 + NOTE 2 / Style, NOTE | m8 + S11 / MINOR | doc-or-claim-only | INSP / INSP | Style/NOTE vs MINOR. Unshared sites: 3b-A11, 3c-C6 |
| B10 | Radial mint in two spellings: sweep test `arc_from` divides by the radius, `seg::mint_radial` by the rim | Style Q1 (+ Q6 "names no row"; claim 7 "test copy drifted") / Style | S3 + m8 bullet 3 / Style, MINOR | code | INSP / INSP | Style vs Style-and-MINOR |
| B11 | `Arc2::carrier_end` duplicates `landing` / `on_turned_radial` | Style Q1 ("unsure") / Style | S2 ("likely") / Style | code | INSP / INSP | None. B adds: four of `register_endpoints`' eight facts restate the others |
| B12 | `sketch_vector`'s Arc arm is dead by its own doc | Style Q7 ("sure") / Style | S9 ("likely") / Style | code | INSP / INSP | None |
| B13 | `Arc2::radius` "positive in value" is a promise by doc alone; no door or type checks it | Style Q5 / Style | S10 / Style | contract-API | INSP / INSP | None |
| B14 | `ValidatedSegment::lift` doc has no sentence for a point scalar (`f64` / `Dual` re-round) | claim 18 remark / none | claim 8 remark ("Held, with a gap") / none | contract-API | INSP / INSP | None |
| B15 | The ±1 sign is a bare `T`, a literal by convention only | claim 18 remark (`from_chord`'s `sign`) / none | S1 / Style | contract-API | INSP / INSP | A's is a strict part of B's (B: three spellings across five functions) |
| B16 | The author's stretched-radial row is `f64`-only; its claim is untested at enclosure scalars | Style Q3 / Style | claim 6 remark / none | test-gap | INSP / INSP | Both ran their own enclosure-scalar probe (EXEC) around it; see §5.5 |
| B17 | Registration justification prose is oversized / restated | Style Q8 (four places) / Style | S4 (four times its body, `swept.rs:795`) / Style | doc-or-claim-only | INSP / INSP | None |

### 3b. UNILATERAL, A only (11)

| # | Finding | Severity | Class | Evidence |
|---|---|---|---|---|
| A1 | Body's verb table, base column, wrong in three cells (bulge, tangent, via stop at `ArcStartOnCarrier`, not `SegmentDirection`) | MINOR-2 | doc-or-claim-only | EXEC; contradicted by B (§5.1) |
| A2 | Reader sweep missed `topo/src/boolean/reduce.rs` `junction_end_held` | MINOR-5 (second site) | code | INSP (effect "unsure") |
| A3 | `register_placed_carrier_ends` doc "hold for every carrier" is false for a table radial: `Interval` builds a wall with a non-unit `u_ref`; `Sym<f64>` registers a fact false by the squaring | MINOR-7 | contract-API (doc claim; test-only door) | EXEC (`Interval`) + INSP (`Sym<f64>`); B's claim 6 is "Held" (§5.5) |
| A4 | Read-back overflow escalation at `f64` offers a refinement that cannot help | NOTE 3 ("unsure") | code | unstated |
| A5 | Bodies' bits moved in 487 of 960 cases: the frame class is wider than the body's corpus count | claim 15 remark | doc-or-claim-only | EXEC |
| A6 | Restated "needs no door" row was green under `|r|` on base, so it never pinned the sign | claim 12 remark | test-gap | unstated |
| A7 | `carrier_line_circle_margin` hand-spells the rationalized gap instead of `radius_gap` | Style Q1 | code | INSP |
| A8 | Two frame policies: line wall on a normalized chord, arc on an un-normalized radial | Style Q1 | code | INSP |
| A9 | `ValidatedSegment::lift`'s normalization survives for one test door | Style Q7 | code | INSP |
| A10 | `lift`'s point-scalar branch has no row that can fail | Style Q3 | test-gap | INSP |
| A11 | Stale sites only A names: `copied_carriers_at_interval.rs` header (two gone functions), `radius_gap` sibling doc, `lift_onto` doc, E12 "folds as a fact about atan2" (unsure) | Style Q4, claim 14 remark | doc-or-claim-only | INSP |

### 3c. UNILATERAL, B only (15)

| # | Finding | Severity | Class | Evidence |
|---|---|---|---|---|
| C1 | Two kept registrations have no `work/` row; the unit's own row still says they retire | m2 MINOR | doc-or-claim-only | INSP; A records the deviation as "an improvement, disclosed" (held, not a defect) |
| C2 | Body's "`revalue` plate floor 1802 → 1783" is false; `boss_union` `at_1e9` 1462 → 1406 is absent from the body | m4 (bullet 1) MINOR | doc-or-claim-only | INSP |
| C3 | Three of six removed `abs` sites have no row their mutant reddens; `ARC_VERBS` lacks four legs | m5 MINOR | test-gap | INSP |
| C4 | `u_ref` "a unit through the tier" asserted only at the ε/64 box, a width that does not distinguish | m6 MINOR | test-gap | INSP |
| C5 | `m10_9_the_rim_registrant…` misnamed, pins only `open > shut`; "32 decisions" not asserted; `Door::LoadBearing` dead | m7 MINOR | test-gap | INSP |
| C6 | Stale sites only B names: `path/verbs.rs` ×3, `path/family.rs`, `path.rs:4604`, `m10_9_pins_interval.rs:128`, `nextest.toml` lists a deleted row, E12 same-OBJECT paragraph (unsure); S11 pointers | m8 MINOR, S11 | doc-or-claim-only | INSP |
| C7 | `Arc2::carrier_end` doc does not except `0/0` at radius 0 | n2 NOTE | doc-or-claim-only | INSP; A mentions the `0/0` as "removable, as before" (held) |
| C8 | `sym11` pad ceiling 1963 → 2863 with no comment; `m10_9` link bracket leaves [303, 439]·ε unwatched | n3 NOTE | test-gap | INSP |
| C9 | Body's grep names a nonexistent `sweep/src/revolve.rs`; `revolve/` was not swept | n5 NOTE | doc-or-claim-only | INSP |
| C10 | Disc row's only side guard switches off above 1e-3; loft row has no tightness bound | S5 Style | test-gap | INSP |
| C11 | `m10_sym_profile_interval.rs` `blocked` column is dead | S6 Style | test-gap | INSP |
| C12 | `PLATE_MAX_TERMS` 76 → 222 rests on a measured 111 nothing derives | S7 Style | code | INSP |
| C13 | Tour row hand-copies `4096` / `128` instead of `DriveConfig::default()` | S8 Style | test-gap | INSP |
| C14 | The tour headline row asserts nothing about the body, and is nightly-only | claim 9 remark | test-gap | INSP |
| C15 | Body says four sweep reds, five exist; two `work/green` rows naming them read `closed` | claim 16 remark | doc-or-claim-only | EXEC (suite on head and base) |

## 4. Severity divergences on shared findings

| Row | A | B |
|---|---|---|
| B1 (spread beyond the red row) | inside MAJOR-1, INSP | m1 MINOR, EXEC |
| B5 negative table radius | MINOR | NOTE |
| B6 tie arm | MINOR; claim 4 "broken as a schedule" | unranked; claim 4 "Held" |
| B7 unlisted `seg.rs` reader | MINOR; claim 2 "broken in part" | unranked; claim 2 "Mostly held" |
| B8 flagged fillet sites | MINOR | NOTE |
| B9 stale text | Style + NOTE | MINOR |
| B10 radial mint spellings | Style | Style + a MINOR bullet |
| B15 sign literal | claim remark | Style |
| B16 stretched-radial row | Style | claim remark |

No shared finding is MAJOR in one report and below MAJOR in the other, except the B1 sub-part noted.

## 5. Contradictions on a fact

1. **Body's verb table, base column.** A: "The body's verb table, base column, is wrong in three cells on `f2815a232a`. At ±1e-6, door shut, base stops at `ArcStartOnCarrier` for bulge, tangent and via, not `SegmentDirection`." B: "Held at the base. 80 cells … the body's table reproduces, including bulge 0.5 ± 0.02." Incompatible as written.
2. **Whether a row holds the fillet setback site.** A: "The two flagged-and-kept fillet sites have no row … The 'fillet row' (`fillet-side-and-centre-arcs-…`) names neither." B: "the second is held by `work/profile/the-fillets-registered-tangency-says-it-holds-where-the-doors-decisions-hold.md`." Incompatible; the two cite different rows.
3. **Whether `f64` decisions move at default ε.** A: "960 verb cases … Validated forms are identical in all 960 and no decision moved. Bodies' bits moved in 487." B: "5040 generated documents … 93 differ; 22 are frame bits only, 71 change stage." Different corpora (B's stage changes are all radius ≳ 4e6 m), so not strictly incompatible; the frame-bit rates (487/960 vs 22/5040) also differ widely.
4. **Which rows hold the five sweep reds.** A: "Each is red on the base with the same payload, held by open rows" (six named). B: "Two of the `work/green` rows naming them read `closed`, so which open row holds each is unverified."
5. **Claim 6, the stretched table radial.** A: "broken for the table door … At `Interval`, a table radial off by 1e-15 builds a wall whose stored `u_ref.x` is the point [1.000000000000001], an enclosure holding no unit (DEMONSTRATED)." B: "Held. … `Interval` and `Sym<Interval>` refuse typed at `EndpointStart` (poisoned frame), no panic, no false registration." They agree on the 0.1·ε and 0.4·ε stretches; A's 1e-15 input is one B did not report.
6. **Check-run identifier.** A: "Check run 138266480756 `merge ref` = failure." B: "`merge ref`, **failure** (run 45341655092)." Possibly two id kinds for one run.
7. **Sweep totals at default ε.** A: "sweep slow set at default ε: head 3820/3825, 5 red." B: "Sweep whole suite, default ε: head 3901/3906, base 3897/3902, the same five reds." Different scope labels; the five reds are the same rows.
8. **`boss_union` and the body.** A: "the body's frame class, wider than its corpus count (boss_union only)." B: "What moved is `boss_union` `at_1e9` 1462 → 1406, absent from the body." Possibly two different passages of the body; unresolved from the reports.
9. **Reaching validation on a tied bulge.** A: "the doc's 'for validation to decide' describes a path I could not reach." B: "the fixture door stores radius `[2, inf]` and validation escalates." Different doors; not incompatible.

Claim-verdict divergences (judgement, not fact): claim 2 (A broken in part / B mostly held), claim 4 (A broken as a schedule / B held), claim 6 (A broken / B held), claim 12 (A held for all three / B `m10_9` faulted), the kept registrations (A an improvement / B owes a row).

## 6. Tally candidates

Criteria applied (rule 6): UNILATERAL and raised as MAJOR; class `code` / `test-gap` / `contract-API`; deduplicated to one underlying defect; demonstrated by execution; fair pair.

**None.** Each report raises two MAJORs and all four are bilateral (B1, B2). No unilateral finding is raised as MAJOR in either report. Fair-pair status (e) cannot be judged from the reports: neither says it was interrupted or truncated (B reports one OOM-killed build and lists what it did not run).

## 7. Counts

| Bilateral | A-only | B-only | Tally candidates |
|---|---|---|---|
| 17 | 11 | 15 | 0 |
