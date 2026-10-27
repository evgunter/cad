# PR 5810 dual review — blind coding inputs

The A/B labels were assigned by a recorded byte; the mapping is held by the orchestrator, not in this file.

Frozen head: 193b0d5d9f9230a2c27b9c15ebb8d9567f1792d4 (base 2f9e6f9e). Arm: concurrent, letter H. Both reviewers were dispatched at 00:36Z with one brief. Neither was interrupted, and both report that lane isolation held.

## Review A

ATREST dual review (concurrent arm), this reviewer: PR #5810 at frozen head 193b0d5d, base 2f9e6f9e

**Verdict: NOT-MERGEABLE-AS-IS.** There are 2 MAJOR, 3 MINOR and 4 NOTE findings. Both MAJORs are local fixes.
Probes are on `the reviewer's probe branch` @ <sha>, in `crates/editor-core/tests/<probe file>.rs`:
- p1 is red on head;
- p2 is red on head;
- p3 is green.
Lane isolation: I did not read any other `review/atrest-s5c-*` branch, any PR comment or any review scratch.

## Correctness lane

**MAJOR 1: an unlocalized overlap refuses at Error.** Claim 4 is broken (spec §4 and CHECKS DS6). Confidence: sure. Executed.
- `checks.rs` `CheckEvidence::could_not_look` (l.540) is `matches!(Self::AtRest{..})`.
- `enforce_checks` (l.2014) therefore refuses every `Interference` finding with `quiet == None`. That includes `Overlap::Unlocalized`.
- The spec rules this out in §4, *Severity and the refusing path*: "an unlocalized overlap is not among them, since `Overlap::Unlocalized` is loud and never refuses". CHECKS DS6 (CHECKS.md l.141-145) says the same: "an overlap the kernel could not intersect, a carrier pair no signed `Gap` covers … never refuses whatever the severity".
- **Probe p1:** the in-band-L document from `an_in_band_pair_is_no_overlaps_evidence`, with `at_rest: Error`. `enforce_checks` returns Err carrying the `Unlocalized(Refused{…undecided… Recourse: move the parts…})` finding. That recourse is a geometry change, the case DS6 forbids.
- The tour teapot's handle/vessel finding is also Unlocalized, so it refuses at Error too.
- **Related, by reading:** a Bounded overlap on a carrier pair `Gap` has no arm for (a sphere off-centre, a torus, a cone) is unquietable and also refuses at Error.
- **The docs disagree with each other:** CHECKS.md l.119-123 (new text: "An assertion quiets every interference the kernel can bound … Error is legal … refuses a loud interference finding and nothing else") against l.141-145.

**MAJOR 2: `opposed` reads the pairing at one point, so a convex–convex clash goes quiet depending on bearing.** Claim 3 is broken. Confidence: sure. Executed.
- `at_rest.rs` `opposed` (l.516-545) and `face_point` (l.548) evaluate both faces' implicit outward normals at ONE point of the outer face, its outer loop's start.
- `Gap` holds over parallel NON-coaxial cylinders (measured −0.5 on the r2 shafts). For that pair the second normal at p depends on where p sits relative to the other axis.
- **Probe p2:** A (r=1) and B (r=0.3), 1.2 apart, clashing 0.1 deep, with `Gap(A,B) ≤ −0.01` holding. B is swept through 8 bearings about A:
  - bearing 0: loud;
  - bearing ½ turn: **QUIET**;
  - every other bearing: the census leaves the pair undecided (could-not-look).
- The same geometry rotated becomes intended-and-silent. The base's sense test (outer concave, inner convex) refused every convex outer. So this is a regression, minted by the fix that closed `at-rest-opposed-decides-a-unitless-dot-as-a-length`.
- `review_s5b_r2_probes::r2_two_convex_shafts_side_by_side_stay_loud` passes only because its B sits at the bearing where p faces away from B. Its comment at l.393 ("Only `opposed`'s sense test … keeps it loud") is now false.
- The lever itself is sound: `Margin::levered(cos θ, folded_lever_arm(…, max reach))` is metres, and no unitless dot is compared to a length any more. For coaxial or concentric pairs, one point is sound.

**MINOR 3: `less_carried` undercounts (claim 5).** Confidence: likely. By reading; no escalating geometry was available.
- `product.rs` l.1347-1351 collects `carried_at_rest` through `once_per_part_row`. So a part instanced twice leaves ONE carried row.
- The host census re-finds the part's pair-less escalation inside EACH copy. `less_carried` (`at_rest.rs` l.160) removes one re-finding per carried row, so the second copy's escalation is reported as the host's own `NoLane`. That is the double report deviation 1 set out to remove.
- The equal-host-escalation drop is disclosed. A sounder discard within scope: build `answered` from each source's own rows before `once_per_part_row` (one per arrival per row), not from the deduplicated list.
- **No witness.** Mutant run, `reading` with `less_carried` replaced by the identity: `cargo nextest --profile ci -p editor-core` gives 2748/2750 pass, and the only reds are my p1 and p2.
  - The rows that pinned it (`intent_part_census_seam`'s `in_band`) now yield `CensusUndecidable{ThinInBand}` after #5782, not `CensusEscalated`.
  - p3 (green) confirms this: a host's own copy of the in-band pair is reported beside the part's carried row, and `less_carried` never fires.
  - The slow set and pncad-py were not run under the mutant.

**MINOR 4: the contact half does not follow picks.** Confidence: likely. Not executed.
- Spec §4 clause 2 says a selection's body is the arrival's read "(or a pick of that piece)".
- `quiet_contact` (l.263-299) compares `select.body == input` only. So a host `Gap = 0` over a pick of an instance's piece never quiets the host row between that instance and another arrival.
- The filed `at-rest-check-rows-left-unwitnessed` asserts picks "cannot match by construction", and treats that as fine for carried rows. That is the right reading for row 17's carried rows, but not for host rows.

**MINOR 5: an empty world leaves the lint without the unread `⊔`s' rows.** Confidence: likely. Not executed.
- `checks.rs` `door_rows` l.1569: under `Subject::EmptyProduct` the lint reads NO `⊔`'s rows.
- `at_rest` (l.1887) gathers those `⊔`s itself.
- So with an empty world, a `⊔` in another space reports its interference but not its contacts. That breaks `[coincidence.door]`'s "every coincidence … a contact the at-rest census finds included".
- The lint side is pre-existing; the asymmetry is new.

**NOTE 6: claim 1 holds.** Executed.
- editor-core `perf12_census_goldens` + `perf12_census_bvh_diff` and step-import `perf12_census_goldens` pass 7/7 on head AND on base at ε 1e-6, 1e-9 and 1e-12.
- The golden diff is exactly 99 outcome lines (93 `Ok` and 6 `Err(AtRest)` becoming `at rest: N…`), and no `census:` or `rows:` line moves.
- The goldens print no interference findings, so "interference findings bit-identical" rests on the B rows, not on the goldens.

**NOTE 7: claim 2, no false quiet found.** Confidence: likely. Reading only.
- Pattern members are told apart by `RoleSeg::Instance{i}`, so the stripped `From` key cannot alias two members.
- A `Gap` reached through another node or an operation's output fails the `select.body == input` check.
- A dual or NotRun assertion fails the `Holds` test.
- `SameOriented` returns early.

**NOTE 8: claim 4 classification, two arms look misfiled.** Confidence: likely.
- `ArcEndOffCorner` → `NoLane` (`could_not_look` l.212). It is one face's kernel self-disagreement, not a pair between arrivals.
- `ContactContradicted` between arrivals also lands in `NoLane`.
- A pair-less escalation re-found inside a nested `⊔`'s arrival reads `NoLane` ("between arrivals"), though it is inside one.
- I found no real interference that is classed could-not-look apart from the census's own `Undecidable` (the teapot spout, p2's off-axis bearings), which is spec-sanctioned.

**NOTE 9: claims 6, 7 and 9.**
- Coverage reads correct:
  - the product first, then `assemble_gathers` (`census.node == id`, product var excluded);
  - the in-arrival discard prevents a nested `⊔` from doubling;
  - `once_per_part_row` runs across the whole report.
- Tour counts not executed (unsure).
- Tracker: closing the opposed row as "fixed" is unsound per MAJOR 2.
- Deviation 1's follow-up rides a parked P2 (`blocked_on d10…`), so the wrong-drop has no live schedule.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6 and Q7. For Q8, `checks.rs` (2224 lines) was read in sections, not end to end.

- **(Q1, sure)** The `⊔`'s `From{read, of}` un-keying is now written three times: `at_rest.rs` l.251 (`quiet_contact`, new), l.1047 (`localize`) and l.442-448 (`gap_assertions`, inverse direction). The PR added a fresh copy rather than one helper.
- **(Q1, likely)** Python `InterferenceFinding.quiet_by` (pyi l.6256) sits beside the new `CheckFinding.quiet` (pyi l.6778). Rust spells both `quiet`.
- **(Q7, sure)** Whether a finding refuses is keyed on its evidence's TYPE (`could_not_look() == AtRest{..}`). `Unlocalized` lives under `Interference`, so the type split misfiles it: this is the root of MAJOR 1.
- **(Q3, sure)**
  - `r2_two_convex_shafts_side_by_side_stay_loud`'s premise (one bearing) excludes the failing mode.
  - `less_carried` has no witness in the ci set (mutant above).
  - The row 13 test never puts an `Unlocalized` finding at Error.
- **(Q4, sure)** Stale prose naming the retired gate:
  - `src/finding.rs` l.77 `[crate::AtRestFinding]` is a broken intra-doc link;
  - `tests/msolve1_transform_aware.rs` l.392 "the at-rest gate refused a seat";
  - `tests/asm_r2b_assembly.rs` l.93 "this unit is about the at-rest gate";
  - `tests/msolve13_read_at_operand.rs` l.4.
  - The PR's blind-spot sweep for the phrase missed these in tests.
- **(Q2/Q4, sure)** CHECKS.md l.119-123 and l.141-145 state opposite refusal rules within one section; see MAJOR 1. I read this as the code having drifted from DS6, not as DS6 rotting.
- **(Q5, likely)** The viewer badge (`frame.rs` `at_rest_badge` l.2276-2284):
  - it reads "at rest: clear; N pair(s) the census could not look at", where "clear" sits beside could-not-look;
  - "pair(s)" also counts `NotRun`, `NotGathered` and `Malformed`, which are not pairs.
- **(Q6, likely)** Deviation 1 is scheduled only behind a parked row, and the undercount in MINOR 3 is not disclosed.
- **(Q6, sure)** The trap of a fix minting a fresh defect held here: the opposed fix (deviation 4, "No row moved") introduced the bearing dependence in MAJOR 2.

## Review B

FULL review (concurrent arm), PR #5810 @ 193b0d5d vs merge-base 2f9e6f9e. 

**Verdict: APPROVE-WITH-FIXES.** MAJOR 1 · MINOR 4 · NOTE 8.
Probes are on `the reviewer's probe branch` (<sha>):
- `crates/editor-core/tests/<probe file>.rs`: p1 RED; p1b, p2, p3, p4 green; p5 RED.
- `demos/tour/src/gallery.rs` `<probe fn>` (green; prints its findings).

Lane isolation: I read no other `review/atrest-s5c-*` branch, no PR comments and no review scratch, and glimpsed none.

## MAJOR
**M1. `enforce_checks` refuses an overlap the kernel could not intersect.** Breaks claim 4 and is the fix to make first. sure.
- Where: `checks.rs` `enforce_checks` and `CheckEvidence::could_not_look`. The filter is severity Error, `quiet == None` and `!could_not_look()`, and `could_not_look()` is only `CheckEvidence::AtRest`. So an `Interference` finding with `Overlap::Unlocalized(_)` refuses at Error.
- What it contradicts:
  - spec §4, "Severity and the refusing path": "an unlocalized overlap is not among them, since `Overlap::Unlocalized` is loud and never refuses";
  - CHECKS DS6, on main at line 155: "a finding outside what the document can yet say (… an overlap the kernel could not intersect, a carrier pair no signed `Gap` covers) is reported loud, never refuses whatever the severity, and is not quietable".
- Executed:
  - p5 takes row 16's `A ⊔ A` report, sets its one interference finding's overlap to `Unlocalized::Empty`, and calls `enforce_checks` at Error: it is `Err`.
  - The tour probe: the gallery teapot refuses at `at_rest: Error` on its one interference. The PR's own gallery text calls that finding unlocalised, and nothing can quiet it. So under Error the teapot has no recourse but moving geometry.
- Also covered: a Bounded finding on a carrier pair `Gap` has no arm for (DS6's third clause) refuses too. That one is by reading, not executed.
- Row 13 (`a_could_not_look_finding_never_refuses`) cannot catch this: its premise has no unlocalized overlap.
- The `ChecksConfig::at_rest` doc ("Error is legal because an assertion quiets every interference the kernel can bound") and the CHECKS.md resident paragraph both say "refuses a loud interference finding", and so cover this wrongly.

## MINOR
**m1. With no world product, an unread `⊔`'s contact is dropped silently.** Breaks claim 6, `[intent.checked]` and `[coincidence.door]` ("every coincidence … a contact the at-rest census finds included"). sure.
- Where: `checks.rs` `door_rows`. Its `Subject::EmptyProduct` arm reads no at-rest row, while the new `at_rest` pass gathers the `⊔`s itself under `EmptyProduct` (`product::assemble_gathers(doc, ev, tol, None)`).
- Executed:
  - p1: two blocks resting face to face, the world `⊔` unread after `SetProduct{None}`. The report has no findings at all. RED.
  - p1b: the same document with the blocks overlapping reports its interference. Green.
- The arm predates this PR. But the gate that used to refuse such a document is gone, and the at-rest pass now covers the case, so the two residents disagree on one document.

**m2. Deviation 1 (`less_carried`) has no witness on this head, and its discard looks wrong for a part placed twice.** Claim 5. Execution and mutation are sure; the two-copy reasoning is likely.
- Witness:
  - #5782 turned the in-band pairs into `ThinInBand`, which names its pair. So neither `intent_part_census_seam::in_band` nor a Rust port of pncad-py's `sliver` (the "12 + 12" case) produces a `CensusEscalated` any more (p2, p3).
  - With `reading` calling the census directly (`less_carried` bypassed), the full default editor-core run reddens no at-rest, seam, s5b or s5c row. The only reds are 50 slow interval/sym/revalue rows: none touches the at-rest code, but I did not re-run them on the unmutated head.
  - The contacthold row's "pinned by `intent_part_census_seam`'s escalation rows" is therefore not true.
- Design, by reading:
  - `gather_sources` keeps the part's carried rows once (`once_per_part_row`), but the census re-finds a pair-less escalation once per copy. With two instances, the second copy's escalation finds no carried row left to cancel it and is reported as the host's `NoLane`.
  - A genuine host escalation equal to a part's would be dropped, and the count stays right only while the re-find exists.
  - I see no sound by-value discard in scope. Either count answers per copy of the part, or keep `less_carried` out until a fixture escalates.

**m3. `ArcEndOffCorner` is classified `NoLane`.** Breaks claim 4's classification. sure, by reading.
- Where: `checks/at_rest.rs` `could_not_look`, in its matches! list.
- `topo` census.rs (~3150) calls it "a defect, not a reading": it is about one face's own boundary. That makes it `Malformed` (or `InsideArrival`), not "a pair the census had no lane for".
- The fn's own doc says a finding about the body's own structure is `Malformed`, so the code contradicts it.

**m4. A `Gap` over a pick of the arrival's piece never quiets.** Claim 2, should-quiet-and-doesn't. likely, not executed.
- Spec §4 rule 2: "each selection's body is the read the cell's arrival entered the `⊔` through (or a pick of that piece)".
- `quiet_contact` admits only `select.body == input` and has no pick path. It is not filed either: `at-rest-check-rows-left-unwitnessed` covers only row 17's host/part case.

## Claims confirmed (NOTE)
- **N1, claim 1: confirmed.**
  - At `CAD_TOLERANCE_EPS` 1e-6, 1e-9 and 1e-12, editor-core `perf12_census_goldens` + `perf12_census_bvh_diff` and step-import `perf12_census_goldens` pass 7/7 at each.
  - The golden diffs touch only the outcome line: 31 Ok → `0/0` or `5`/`9` interference, and 2 Err → `29` / `14` could not look.
  - No `census:` or `rows:` line moved. The corpus holds neither stated exception. sure.
- **N2, claim 3: holds.** sure on execution; unsure on the lever.
  - p4 tilts B's press fit by an off-axis turn plus a shift. It is loud unasserted and quiet under `Gap(bore, pin) ≤ −0.001`, at the identity and tilted.
  - `Gap(pin, bore)` measures +0.005 and is `Violated`. That role order is the measure's own, not `opposed`'s.
  - One point is sound for the pairs `Gap` admits: planes, and coaxial or parallel cylinders, where the pairing is constant.
  - The lever (`opposed`) is now metres. It folds `r_outer.max(r_inner)` where the fold's doc reads "the faces' extent". The larger reach overstates the arm, which errs toward deciding, not toward a wrong sign.
- **N3, claim 7: the counts reproduce, and nothing repeats.** Run via the tour probe; no (subject, evidence) repeats.
  - Teapot (42): 32 `CensusUndecidable` face pairs, 1 solid pair, 8 `EdgeEdgeCross`, 1 interference.
  - Chain (196): 104 face-pair and 10 solid-pair `CensusUndecidable`, and 82 `UnrecordedContact`: 36 `EdgeEdgeCross`, 18 `VertexOnFace`, 18 `EdgeFaceOverlap`, 6 `EdgeEdgeOverlap`, 4 `VertexVertex`.
  - The chain's contact touches lie on pairs with an uncleared curved `MayOverlap`, where `topo` records no row. So they are `NoLane` by spec §4's own definition, not misfiled contacts.
  - The teapot's spout/vessel interpenetration is could-not-look only.
- **N4, claim 2: nothing quiets wrongly.** By reading against each candidate: a stripped `From` key, an intermediate node, an operation output, a part inside an instance, and a dual or not-run verdict. The match is the tuple `(input var, own name)`, a `Gap` read directly, `Holds`, a bound decided zero, and never `SameOriented`.
- **N5, `A ⊔ A`.** Its finding has `a == b` (the same `CopyRef`), and the face list after sort+dedup holds 6 faces for two arrivals. So it cannot say which arrival a face is on. Seen in p5's premise output. likely.
- **N6, claim 8: the retirement sweep left stale prose.**
  - `editor-core/src/finding.rs:77` links `[crate::AtRestFinding]`, which is retired.
  - The tour's printed captions still say "the A5 at-rest gate CERTIFIES" and "at-rest gate passes outright" (`demos/tour/src/assembly.rs` ~2294 and ~2322), and comments say "the gate certifies" (~2082, ~2091).
  - `work/intent/assemble-is-an-operator-and-the-product-is-a-slot.md:87` and `work/intent/a-placement-carries-its-sources-at-rest-result.md:31` say `less_carried` retired; this PR reintroduces it. sure.
- **N7, viewer badge.** `viewer/src/frame.rs` `at_rest_badge` reads "at rest: clear; N pair(s) the census could not look at". "clear" sits beside could-not-look (DS6: not a clean report), and N counts `NotRun`, `NotGathered` and `Malformed`, which are not pairs. likely.
- **N8, claim 9: the tracker edits are sound except one stale line.** The rows closed, opened and pruned match the code. But the body of `an-assertion-quiets-an-operations-unproven-row` still says "Cells below a face wait on gap-reads-vertex-and-edge-cells", which landed in #5746. Opening `a-boolean-drops-…` inside CONTACTHOLD may not fit a hold program. unsure.
- **Claim 4's rule.** Apart from M1 and m3, the never-refuse rule for `AtRestEvidence` is what spec §11 and DS6 decided.
- **Claim 6's rule.** `once_per_part_row` across gathers is correct. A nested `⊔` is reported once: the inner `⊔` sits in `assembles`, and its findings are discarded inside the outer arrival.

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8.
- **Q1, sure.** The `From { read, of }` un-keying is written three times: `checks/at_rest.rs` `quiet_contact` (~252), `gap_assertions`' `face_of` (~445) and `localize` (~1048).
- **Q1, sure.** The assertion preamble (Assertion → value → Gap measure → Holds → decide bound) is copied between `quiet_contact` and `gap_assertions`.
- **Q1, sure.** `Reading` (at_rest.rs:117) and `SeamVerdict` (carried.rs:311) are the same struct twice.
- **Q1, sure.** `copy_ref` (:579) and `copy_rows`' `by` closure (:929) build the same `CopyRef`. `partition`'s `copy_of` map duplicates `copy_groups`.
- **Q1, sure.** The two `CarriedRow` impls are identical (carried.rs ~274-284).
- **Q2, sure.** `could_not_look`'s doc says structure → `Malformed`, while its code lists `ArcEndOffCorner` under `NoLane` (m3).
- **Q3, sure.** Row 13's premise excludes `Unlocalized` (M1). The `less_carried` rows cannot go red because nothing escalates (m2).
- **Q4, sure.** DS6's unquietable-never-refuses sentence is the invariant `enforce_checks` dropped: code drift, not a stale doc. The work rows still cite `less_carried` as retired.
- **Q5, likely.** `ChecksConfig::at_rest`'s doc promises Error is safe because assertions quiet every boundable interference, but the refusal reaches ones that cannot be bounded.
- **Q6, likely.** Deviation 5 keeps `FitSubject::Ungathered` public, and no landing produces it any more. No row schedules its retirement.
- **Q7, unsure.** `quiet_contact` scans `doc.ids()` once per row, which is O(rows × nodes).
- **Q8.** I read `checks/at_rest.rs` (1339 lines) end to end. Beyond the above, nothing.

Not run: the Python suite, and the viewer tests.
