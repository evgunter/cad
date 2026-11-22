# Blind correspondence coding: PR 6972 @ 032ac185, reports A and B

Inputs: `blind-A.md` and `blind-B.md` only. No repository access, no builds. "demo" = demonstrated by execution by that reviewer; "insp" = by inspection.

## 1. Findings

| # | Finding | Code | A: severity; demo | B: severity; demo | Class | Note |
|---|---|---|---|---|---|---|
| 1 | Committed slow-set rows red on head, green on base, never run by the gate (CLASS) | BILATERAL as a class; instances disjoint (1a, 1b) | MAJOR (M1); demo | MAJOR (M1); demo | code | Same kind of defect, zero overlapping rows. See dedup ruling below the table. |
| 1a | A's instances: `seat8_split_lowering` digests (3 documents, every eps); `index_memo` wide-candidates row, the unpinned sibling of the pinned ring row (every eps); `m10_5_r1` e2e slider receipt (1e-6, 1e-12); `coincidence_door` coaxial stage6 pin (1e-12; stale pin, a gain) | UNILATERAL-A | MAJOR; demo (red on head, green on base, three eps) | not mentioned (B: "editor-core, viewer and mesh slow sets I did not run") | code | B discusses the pinned ring row only, not its sibling. |
| 1b | B's instances: step-export `committed_fixtures_are_byte_golden` (`boss_union.step` placement point moved); `kernel_sidecar_fields_match_live_kernel` (`swept_elbow` volume pad 1.7047e-5 -> 1.6986e-5) | UNILATERAL-B | no defect raised. Claim 8 notes by inspection that the STEP placement moves and "the plane is the same"; import round-trip not run | MAJOR; demo (base 59 pass; head 57 pass, 2 fail) | code + contract-API (exported bytes change) | A saw the mechanism and passed it; A did not see the red rows. |
| 2 | Gate did not run the slow set on this head (4 rows ran; about 2,100-2,180 unrun); the body's coverage claim is false | BILATERAL | MAJOR (M2); demo (logs, 89 slow rows at three eps) | MAJOR (M2); demo (logs, 806 slow rows at default eps) | process / doc-claim | Both: eps rows ran the fast set only; whole jobs skipped (B adds topo-release). |
| 2a | No render ran on this head | BILATERAL | claim 15; logs | inside M2; logs | process | B alone adds that the cited render run concluded FAILURE (B-only sub-fact). |
| 2b | Two sweep slow rows red on the merge base too (`split_edge_loft_charts`, `m8_4_intersection_iso`) | UNILATERAL-B | not mentioned | inside M2, stated as "inherited from main, not this PR"; demo | process (not this PR) | A's nightly remark concerns A's own four rows only. |
| 3 | `GeomPred::PlaneDistance` answers differently (face read at its carrier origin, which moved); presented as a test re-baseline, filed nowhere | BILATERAL, severity diverges | MINOR (m4); insp | MAJOR (M3); mechanism insp ("severity likely") | contract-API | B adds a stale comment at `test_selectors.py:93-96`. |
| 4 | Vector arm keeps a computed, normalized wall normal; unpinned and unscheduled | BILATERAL | MINOR (m2, claim 2, Q6); probe run, body refused | MINOR 1; demo (wall normals of width 0.403) | code | The two probes disagree on the facts: contradiction C1. |
| 5 | Ratified README "no sweep divides a difference by its length" sentence is false at head (loft struts `line_between`, `mev_line`) | BILATERAL | MINOR (m2); insp | MINOR 7; insp | doc-claim (ratified clause) | One defect. B adds the hole bridge `swept.rs:911`; A adds the vector wall normal as a second falsifier; B's falsifier of "a direction the sketch holds" is `sketch_direction`. |
| 6 | `revolve_washer` registration row removed; the replacement does not pin the theorem | BILATERAL | MINOR (m3); demo on one fixture (sym/num/frozen counts on both trees) | MINOR 4; insp | test-gap | B adds: every new revolve row is on `SketchPlane::xy()`. |
| 7 | Stale prose naming the deleted Newell / CapPlane machinery | BILATERAL | NOTE (n1), Q4, Q5; insp | MINOR 10; insp | doc-claim | One defect. Shared: `lib.rs:14,131-135`, `partial.rs:9`, `mod.rs:1134`, `tube.rs:472`. B only: `mod.rs:747,759`, `topo/geometry.rs:24`, `euler.rs:283`. |
| 8 | PR 6953 overlap: 7 conflicting files, mesh probes not byte-identical, one act under two helper names | BILATERAL | NOTE (n2); demo (`merge-tree`) | MINOR 9; demo (`grep`, `diff`, `merge-tree`) | doc-claim / process | B frames it as false body statements (`register_placed_line_end` exists nowhere). See C3. |
| 9 | Ring row: why the mesh moved | BILATERAL topic, opposite diagnoses | NOTE (n3), "unsure"; insp | MINOR 3, "likely"; insp | doc-claim (filing's stated cause) | Contradiction C2. Alternate reading: two separate unilateral findings. |
| 10 | Containment rows thin; door coverage narrow | BILATERAL as a class | claim 3 PARTLY, Q3 | NOTE, claim 3 | test-gap | Missing rows (revolve angle, loft placement) are one shared defect; B adds vector and revolve off `xy()`. A's "no ceiling on enclosure width" and B's "1e-12 slack of 5e-15" are separate weaknesses (opposite directions), one per side. |
| 11 | `section_centre` re-implements the old centroid; contingent on loop cuts | BILATERAL | Style Q1, claim 8; insp | NOTE, Q7; value parity demo on 13 partials | code (design) | Same critique. |
| 12 | "Unit direction" enforced by comment only (`Vec3`, not `UnitVec3`) | BILATERAL | Style Q2, sure | Style Q2, sure | code (style) | |
| 13 | "Decided once" direction recomputed at each site | BILATERAL | Style Q1 | Style Q1 | code (style) | |
| 14 | `joints: Option` guarded by `unreachable!` | BILATERAL | Style Q7, unsure | Style Q2, likely | code (style) | |
| 15 | Survey's "not re-verified site by site" has no schedule | BILATERAL | Style Q6 | Style Q6 | process | B adds more unscheduled items (230 readers, `torus_radial`, the slow set). |
| 16 | `mate_tool_flow` rewritten assertion | BILATERAL, weak | claim 6: pins an implementation detail, "likely acceptable" | MINOR 6: does not discriminate copy from master | test-gap | Same row, different critiques. |
| 17 | `sketch_direction` re-derives what the profile records | BILATERAL (mention only on A's side) | named as a disclosed interim seam, no defect | Style Q1, claim 10 (scheduled) | code (style) | |
| 18 | PR body misstates the coincidence door ("now prove" vs still unproven, recourse changed) | UNILATERAL-A | MINOR (m1); insp | not raised (claim 5 reads the door as 21 -> 28 of 42) | doc-claim | See C5. |
| 19 | `Course::wall_normal` spells one rule two ways | UNILATERAL-A | Style Q1, unsure | - | code (style) | |
| 20 | `register_placed_carrier_end` doc proves `radial == place` only | UNILATERAL-A | Style Q2, likely | - | doc-claim | |
| 21 | A sketch origin far from its profile costs carriers their vertices; at eps 1e-12 a cell at 4.2e3 built on base and refuses on head | UNILATERAL-B | - | MINOR 2; demo (measured residuals, one build -> refuse) | code | Nothing flips inside the 1e3 m session box. |
| 22 | Refusal pins deleted without replacement (`m10_3_driver_interval`, `sym_9` `retried=`, `docm2`) | UNILATERAL-B | - | MINOR 5; insp | test-gap | |
| 23 | Other rewritten rows pin less (`mate7a` x2, both mesh probes' disjunction, `shell_cavity_embedded`, `sym11`, `torax_axial` 10x) | UNILATERAL-B | - | MINOR 6; insp | test-gap | Same class as 16; these instances are B's only. |
| 24 | Tracker rows left false or stale (five `work/` files, `m10_9_pins_interval` counts) | UNILATERAL-B | - | MINOR 8; insp | doc-claim / process | |
| 25 | Band row omits blend sites (`surgery.rs`, `junction.rs`, `open/*`) | UNILATERAL-B | - | claim 9, likely; insp | doc-claim | |
| 26 | Montage: five gaps that were 0 are now nonzero; body mentions one | UNILATERAL-B | claim 14 HELD on diff text | claim 14 "not established" | doc-claim | Verdicts diverge; the stated facts are compatible. |
| 27 | Switch-off counts differ from the body (816/1207 vs 812/1245; 2448/3115 vs 2412/3150) | UNILATERAL-B | claim 11 NOT CHECKED | claim 11; demo | doc-claim | |
| 28 | Loft with tilted top: mesh volume differs 3.3e-7 relative, unexplained | UNILATERAL-B | - | NOTE; demo | code (unexplained) | |
| 29 | Measure lever and census bound grow with origin-to-face distance | UNILATERAL-B | - | NOTE; insp | code | Sound but looser. |
| 30 | `extrusion` matched twice; `Pose` from two sources; `Placing::point` vs `carry` | UNILATERAL-B | - | Style Q1 x3 | code (style) | |

Dedup ruling on row 1. The class is bilateral: both raise "committed slow rows red on head, green on base, unrun by the gate" as MAJOR, both demonstrated. The instances are disjoint: A's four rows (editor-core x3, viewer x1) and B's two rows (step-export x2). I judge them SEPARATE defects, for three reasons. First, the factor they share (the gate skipped the slow set) is already its own bilateral finding, row 2, so what row 1 adds is the specific red rows, and none coincide. Second, the mechanisms differ per row (plane digests, an unpinned mesh row, eps receipt counts, a stale escalation pin, STEP placement bytes, a volume pad), and each needs its own fix. Third, neither report names the other's rows, and A inspected the STEP path and passed it. I am not certain. The other reading is ONE defect ("the origin move invalidated committed pins that were not re-cut", most plainly A's `seat8` digests and B's `boss_union.step`), which makes row 1 bilateral and empties the tally.

## 2. Bilateral headline

Both reports independently reach NOT-MERGEABLE-AS-IS on the same shape: the geometry is right at f64 by each one's own head-versus-base differential, but committed slow rows are red on head and the gate ran only 4 slow rows; yet the red rows each found are entirely disjoint (A: 4, B: 2). Of the 16 shared findings the one consequential severity split is the `PlaneDistance` selector change (A MINOR, B MAJOR), and the reports contradict each other on whether a boxed vector extrusion builds and on what moved the ring row's mesh.

## 3. Unilateral findings

UNILATERAL-A (4):
- MAJOR 1: row 1a (four red rows).
- MINOR 1: row 18.
- Style 2: rows 19, 20.
- Plus one instance inside bilateral row 10 (no ceiling on enclosure width).

UNILATERAL-B (11):
- MAJOR 1: row 1b (two red step-export rows).
- MINOR 4: rows 21, 22, 23, 24.
- Claim-level 3: rows 25, 26, 27.
- NOTE 2: rows 28, 29.
- Style 1 group (three items): row 30.
- Plus sub-facts inside bilateral rows: the cited render run failed (2a), two inherited reds (2b), the 1e-12 containment slack (10).

## 4. Tally

Under my ruling (separate defects), two findings enter:
- **T1 = row 1a (A).** (a) Unilateral, raised MAJOR. (b) Code: committed rows red in the tree. (c) Counted once as raised; it could be split into as many as four rows, or into three stale baselines plus one unpinned behaviour row. (d) Demonstrated: red on head and green on base at three eps. A itself calls the `coincidence_door` instance "not a regression" (a stale pin after a gain).
- **T2 = row 1b (B).** (a) Unilateral, raised MAJOR; A noted the mechanism only and raised no defect. (b) Code plus contract-API. (c) Counted once, two rows. (d) Demonstrated: 59 pass on base, 57 pass and 2 fail on head, with measured values.

Under the alternate reading (one defect) the tally is **none**. The tally is therefore 2 or 0, and turns entirely on the row 1 ruling.

Qualifier on "would have let through": both reports flag the unrun slow set as blocking, and each names the ground where the other's reds live as unrun. Neither would have passed the PR; each would have left the other's rows undiscovered unless the author ran the full slow set in response.

Nearest misses:
- Row 3 (B MAJOR): fails (a), bilateral; also fails (d), mechanism by inspection.
- Row 2 and 2a: fail (a), bilateral, and (b), process / claim.
- Row 2b: demonstrated and unilateral, but not raised as a defect of this PR; fails (a).
- Row 21: unilateral, code, demonstrated; fails (a), raised MINOR.
- Row 4: code, demonstrated by B; fails (a), bilateral and MINOR.
- Row 6: demonstrated by A; fails (a), bilateral and MINOR.

## 5. Verdicts and contradictions

| | A | B |
|---|---|---|
| Verdict | NOT-MERGEABLE-AS-IS | NOT-MERGEABLE-AS-IS ("fix those and it is an APPROVE-WITH-FIXES") |
| Idiom and structure | 4 | 4 |
| Test quality | 2 | 3 |
| Doc and comment honesty | 3 | 2 |

The test and doc scores mirror each other: each marks down hardest the area where its own unilateral findings sit.

Contradictions on fact:
- **C1, boxed vector extrusion.** A: over z in [1.95, 2.05] "the body REFUSES at check EndpointEnd (the strut) ... on head and ... on base, so there is no stored datum to measure", and "the vector arm cannot be boxed at all". B: `Extrusion::Vector((0, 0, [0.95, 1.05]))` at `Sym<Interval>` builds, "caps exact, all four walls store normals of width 0.403". The boxes differ (2 +/- 0.05 vs 1 +/- 0.05) and the lanes may differ, but A's universal statement cannot stand if B's build is accurate.
- **C2, ring row mechanism.** A (n3, claims 7 and 16): the mesh flipped "under a pure translation of the chart origin ... so the planar triangulator is translation-sensitive", and the filing "says what it measured". B (MINOR 3): `mesh/src/planar.rs` does not read the stored plane and re-derives the chart from boundary points, so the stated cause "is not there" and a far chart origin "is not a triangulation hazard". Both are by inspection.
- **C3, the 6953 merge.** A: "Nothing is silent." B: beyond the 7 conflicting files, "two more need hand work" (new `swept.rs` rows call the six-argument form; a `q_to` row "merges in open").
- **C4, reachability of the deleted cap-plane refusals on base.** A (claim 4, inspection): reached "only by rows that construct the error"; "no fuzz row, corpus row or validated input reached them". B (claim 4, run): "the one base `CapPlane` refusal I reached (Interval, revolve on a boxed-turned frame)".
- **C5, coincidence door (possibly different units; unresolved from the reports).** A: `two_blocks` and `ring_serving` stay unproven and "the one new proof is the at-rest row". B: the door "is 21 -> 28 of 42".
- **C6, soft.** B's MINOR 1 is headed "undisclosed", while A lists "the vector arm kept" as author-reported and B's own deviations list says "Reported: vector strut kept". Reconcilable: the strut is disclosed, the wall width is not. Also the thread-count figure 643 is labelled "decisions 639/643" by A and "frozen 643 -> 14" by B.

Claim verdicts that diverge without a factual clash: claim 3 (A PARTLY, B Held on B's own rows), claim 9 (A found omissions, B Held), claim 11 (A not checked, B held where run), claim 14 (A HELD, B not established).

Not coded as a PR finding: B's self-disclosure that its first review-branch commit carries a committer identity naming the tool.
