# Blind correspondence coding: PR 7203, reports A and B

Coded from `A.md`, `B.md` and rules 5–7 of `docs/DUAL-REVIEW-PROTOCOL.md` only. No code run, no history read.
Both verdicts: APPROVE-WITH-FIXES. A: "No MAJOR. 8 MINOR, 11 NOTE." B: "Nothing I found is MAJOR" (4 MINOR, 6 NOTE).
"In passing" = the other report states the underlying fact but raises no finding on it; coded BILATERAL per the strictness instruction and flagged.

## 1. Findings, one row per underlying defect

| # | Underlying defect | A | B | Code |
|---|---|---|---|---|
| 1 | PR body's and closed row's "centre-stored base" widths are not `ec8749552`'s (narrowing 5–15%, not 31–55%) | m1 MINOR, demonstrated | M2 MINOR (also C3, C12), demonstrated | BILATERAL, same severity; the six measured figures are identical |
| 2 | Form and phase in conic identity unpinned: `same_content` form check and `data()` phase mutants survive; the `:1320` assertion compares conics whose origins already differ | m2 MINOR, demonstrated (R3, R4, R5, R8) | M1 MINOR + S7, demonstrated (survive all 17,467 `ci` rows) | BILATERAL, same severity. A-only instances: R5 (`same_content` ignores phase), R17 (C9 table only) |
| 3 | step-import signature's anchor mark has no row (R14 survives `step-import` whole) | part of m2 MINOR, demonstrated | not raised: `build.rs:1753` appears only as a form-reader (M3) and as duplication (S5); B says only the mesh key's mark is pinned | UNILATERAL A (MINOR component) |
| 4 | Re-seamed anchored ellipse's readers (azimuth phase terms, `map_scalar` phase) unpinned in the unit's own rows; red only in integration rows | m3 MINOR, demonstrated (R6, R7, R9, R10; red only in `sweep`) | M1 MINOR, demonstrated (red only in pre-existing `mesh`/`sweep` rows) | BILATERAL, same severity. A-only instance: `placed` dropping the phase (R7) |
| 5 | `zero_point` on a re-seamed anchored ellipse unpinned; no production reader | C9 table only: R18 "SURVIVES (unsure)"; no numbered finding; never says it has no production reader | M1 MINOR + S11 Style, demonstrated (whole suite) | BILATERAL (A in passing); divergence: table-only/unsure vs MINOR |
| 6 | `the_anchored_offsets_bound_charges_the_squares_rounding` stays green with the square's rounding uncharged (A5, R16 survive) | m4 MINOR, demonstrated | not raised. Neighbouring: B's "sines charged half" mutant is red in that same row, and B "found no uncharged term" | UNILATERAL A (MINOR) |
| 7 | `Ellipse::placed` handed a reversal draws another ellipse (`2a` off); latent, no production caller | m5 MINOR, demonstrated | never mentioned (B's reversal probes go through re-seam then reverse; nothing on `placed`) | UNILATERAL A (MINOR) |
| 8 | Flat-cone image ceiling 48 → 104 `u` is a fitted, weaker guard; its mutant reds one pose of three | m6 MINOR, by inspection | N6 NOTE + S6 Style (+ C6) | BILATERAL; MINOR vs NOTE |
| 9 | More than three production doors rebuild a conic through a centre constructor | m7 MINOR, by inspection (about a dozen sites, incl. `pole_form.rs:58`) | N2 NOTE / C4 (one extra door, `pole_form.rs:58`) | BILATERAL; MINOR vs NOTE; A's site list far longer |
| 10 | Module doc gives two accounts of the anchor (`conic.rs:28–56`: point at θ=0, centre `anchor − u_ref·a`, true only at phase `(1, 0)`) | m8 MINOR + S1, by inspection | S2 bullet (`:39-49`), Style | BILATERAL; MINOR vs Style |
| 11 | C2 premise wrong: `RESOLUTION_ROUNDINGS` is 32 at base and head | n1 NOTE | N1 NOTE / C2 | BILATERAL, same |
| 12 | Readers outside `conic.rs` ask the form (`build.rs:1753`, `memo.rs:1098`) | n2 NOTE ("the brief's wording"; PR body says both) | M3 MINOR / C5 (adds: `Conic::anchor()` is public API) | BILATERAL; NOTE vs MINOR |
| 13 | Near-gap row's centred counts are floors (`>=`), so "unchanged" is not pinned | n3 NOTE | C2 text calls them "floors" (7/7/7/7/1); no finding | BILATERAL (B in passing) |
| 14 | Lifted `Interval` enclosure misses the stored form's exact value (frame re-minted at `Interval`; the base's) | n4 NOTE, likely (1 of 15,228; 33 on base) | N4 NOTE, unsure (48 of 5,184) | BILATERAL, same |
| 15 | `f64` arc cover does not hold the arc's points: main's defect, exposed not made by the unit; cover still read from the derived centre | C7 text + S9 Style (no m/n number) | N3 NOTE / C7 | BILATERAL; claim-text/Style vs NOTE |
| 16 | Chart image still `u·a` off its carrier near the anchor (image built from the derived centre; scheduled on pcert) | n8 NOTE + S9, measured | C6 cites the open pcert row by its title as owner of the ceiling; no measurement, no finding | BILATERAL (B in passing) |
| 17 | `point_at(0)` is not the anchor bit for bit at a `−0.0` coordinate | n5 NOTE | — | UNILATERAL A (NOTE) |
| 18 | ε 1e-12: head refuses the ring's join where base refused in the volume; not "the same payload" | n6 NOTE | never mentioned (B's 1e-12 account is `ci` pass/fail; ring rows not run at 1e-12) | UNILATERAL A (NOTE) |
| 19 | `carrier_resolution` charges an anchored conic `u·a`; module doc still says so | n7 NOTE, unsure | not raised (C2 covers the constant and its guard only) | UNILATERAL A (NOTE) |
| 20 | `signed_zero::plus_zero_curve` leaves the phase; `(−1.0, −0.0)` walks into both keys | n9 NOTE, unsure | — | UNILATERAL A (NOTE) |
| 21 | Seam turn on a straddling `Interval` hulls the phase; derived centre a semi-axis wide | n10 NOTE, unsure, not probed | not raised. Neighbouring: S10 says `map_scalar`'s `f` can widen the pair | UNILATERAL A (NOTE) |
| 22 | Bare-rounding list's seven new `conic.rs` entries carry the wrong reason `far-centre` | n11 NOTE, unsure | not raised (C12 checks counts only) | UNILATERAL A (NOTE) |
| 23 | Doc `conic.rs:104–106` (seam at the nearest vertex resolves at that part's scale) false for an anchored ellipse | S2 Style | not among B's S2 sites | UNILATERAL A (Style) |
| 24 | `quarter_turned_where` doc "Built through `Self::new`" is a struct literal | S3 Style | S2 bullet (`:959`) Style | BILATERAL, same |
| 25 | `born` and `phase` are parallel fields; phase means something only when anchored | S4 Style | not raised. Neighbouring: row 26 | UNILATERAL A (Style) |
| 26 | Phase held as a `(T, T)` pair, not a four-valued quarter-turn index | not raised. Neighbouring: row 25 | S10 Style, unsure | UNILATERAL B (Style) |
| 27 | `Circle`/`Ellipse` near-verbatim copies (`stored`, `centred_at`, `same_placement`) | S5 Style (adds `offsets`/`running_offsets`, test-local `Born`) | S3 Style (adds `anchor()`, 19 `born` match sites) | BILATERAL, same |
| 28 | Doc volume on the re-seam code out of proportion | S8 Style, mild | S8 Style, likely | BILATERAL, same |
| 29 | Typo "a ellipse" | S10 Style | S3, in passing | BILATERAL |
| 30 | Verification account says two red gates; three are red (`cover-kind-match`, red on base too) | not raised; A's C11 reports "two red … the same" as the author, a contrary observation | M4 MINOR, demonstrated | UNILATERAL B (MINOR) |
| 31 | `f64` quarter-turn re-seam moves the derived centre about 1.2 `u·a`; doc `:72-73` "derived once" | not raised (C3: no path moves the anchor; n10 concerns `Interval` only) | N5 NOTE, demonstrated + S9 Style + C3 | UNILATERAL B (NOTE) |
| 32 | Dead comment `intersect.rs:2574-2578` describes removed `tilt.centre` code | — | S1 Style | UNILATERAL B (Style) |
| 33 | Docs false of an anchored conic at `conic.rs:8`, `:120`, `:147`, `:215` | — | S2 Style | UNILATERAL B (Style) |
| 34 | Anchored circle is production code with no production caller | m7 text states the fact ("no production door mints an anchored circle") as mitigation, not as a defect | S4 Style (also C2, C4) | BILATERAL (A in passing) |
| 35 | Two hand-written anchor-mark bits (`memo.rs:1098`, `build.rs:1753`) restate `data()`'s datum | not raised; A cites the same two sites under n2 only (row 12) | S5 Style, likely | UNILATERAL B (Style) |

Not rows: A S6 is a class label over rows 2, 4, 6, 8, 13 (B's S7 is one instance of it); A S7 is a positive statement (nothing disclosed is unscheduled).

If "states the fact without raising it" is instead read as "never mentioned", rows 13 and 16 become UNILATERAL A (NOTE) and row 34 UNILATERAL B (Style). No MINOR or MAJOR changes code either way.

## 2. Rule 6 on unilateral MAJORs

Neither report raises a MAJOR at any point. A: "No MAJOR. 8 MINOR, 11 NOTE." B: "Nothing I found is MAJOR." Criterion (a) (UNILATERAL and raised as MAJOR) therefore fails for every finding, so (b)–(d) are not reached. **The tally is 0.**

For the record on (e), fair pair: neither report says it was interrupted or truncated. B records "Two disk-full events hit builds, not test runs. Every result below comes from a run that completed." Rule 4 was outside my reading, so I did not adjudicate it. Method differences visible in the reports:
- A ran `ci` at ε 1e-6 over the whole suite; B did not.
- B ran the base's whole `ci` suite; A did not ("my differential is 'the head's reds are the base's' plus the probe").
- A re-planted the author's eight mutants plus 19 of its own; B planted 12 of its own and none of the author's.
- B ran its six survivors against the whole `ci` suite; A ran no mutant against the whole workspace.

## 3. Summary

**Bilateral headline.** Both reviewers approve with fixes and find no MAJOR. Both say the anchored evaluator, running bound and anchor invariants hold against an independent oracle. They agree on the two main defects at MINOR, both demonstrated: the width table's stale base column (row 1, identical figures), and the unpinned form/phase identity and re-seamed readers (rows 2, 4), with the same core surviving mutants. They also agree on the C2 premise, the `Interval` enclosure note, the `f64` cover being main's defect, the `Self::new` doc, the duplication and the doc volume.

**Severity divergences.**
- A higher: weakened image ceiling (row 8, MINOR vs NOTE), extra rebuild doors (row 9, MINOR vs NOTE), two accounts of the anchor (row 10, MINOR vs Style).
- B higher: readers outside `conic.rs` (row 12, MINOR vs NOTE), `zero_point` (row 5, MINOR vs a table-only, unsure entry).

**Unilateral, A raised and B did not.**
- MINOR: row 6 (m4, offsets-bound row blind to the square's charge); row 7 (m5, `Ellipse::placed` under a reversal); row 3 (m2 component, step-import mark unpinned).
- NOTE: rows 17, 18, 19, 20, 21, 22 (n5, n6, n7, n9, n10, n11).
- Style: rows 23, 25 (S2, S4).

**Unilateral, B raised and A did not.**
- MINOR: row 30 (M4, third red gate omitted). A's report contradicts it on the observed count.
- NOTE: row 31 (N5 / S9, derived centre moves on an `f64` quarter turn).
- Style: rows 26, 32, 33, 35 (S10, S1, S2's four sites, S5).

**Tally: 0.** No finding qualifies, because no MAJOR was raised by either reviewer.

**Unilateral MINORs demonstrated by execution.**
- A only: row 6 (m4; surviving mutants A5 and R16). Row 7 (m5; measured wrong value, `2a` off: 60 m at a = 30, 3,000 m at 1,500). Row 3 (R14 survives `step-import` whole).
- B only: row 30 (M4; `scripts/run-gates.sh` shows `cover-kind-match` red at head and base). It is a claim-only finding about the PR's account, on a gate red that is not the unit's.

---

Orchestrator's note (2026-11-21, after the coding was returned). Mapping by the recorded byte 221 (odd): **A is r2, B is r1**. The coder was a subagent of the orchestrator's session on the session's default model, handed the two reports with the reviewer labels and branch names stripped, the protocol's rules 5 to 7, and nothing else.

- **Tally: 0**, as coded: neither review raises a MAJOR.
- **Fair pair (e): confirmed.** One brief for both, no relaxation to either, neither interrupted or truncated (one reviewer reports two disk-full events during builds, with every reported result from a completed run). The differences in what each ran (the whole suite at ε = 1e-6; the base's whole suite; whose mutants; mutants against the whole workspace) were each reviewer's own choice under the same brief.
- The coding agrees with the pre-note except where it is finer: it splits the step-import mark's missing row out as unilateral, and notes that one review's two-red-gates observation contradicts the other's three.
- Unilateral MINORs demonstrated by execution, recorded both ways as the protocol asks: three one way (a row blind to the square's charge; `Ellipse::placed` under a reversal; the step-import mark unpinned) and one the other (a third red gate omitted from the account).
