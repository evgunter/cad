# BLEND exit walk — criteria vs evidence

**STATUS: PROPOSED (design conversation — awaiting Ev's
ratification; on sign-off this document is BLEND's done-state of
record, the program closes, and the successor CARVE opened in the cut
of 2026-09-17 (PR #2810) stands).**
BLEND = the blend kernel and the profile fillet door
(`work/blend/plan.md` / `work/blend/log.md`; opened 2026-09-06 as
FILLET's successor in the tracker-wide cut of that day; A/B band
2900–2999). The plan's "Exit shape" paragraph, as restated by the cut,
supplies the criteria, quoted verbatim, one commitment per row,
dispositions per the S-MATE/S-CERT convention: MET /
MET-WITH-RECORDED-HONESTY / CARRIED (named owner).

Fifteen units and one ruling merged. E units 1–5 under single style
reviews with no A/B row (PRs #2123, #2122, #2141, #2129, #2155, all
2026-09-08); unit K, the K-floor ruling, PR #2149 (2026-09-08); H
units 6–12, 14 and 15 under the full v6 dual: ordinals **2900–2908**,
samples **#176, #177, #181, #182, #183, #184, #185, #218, #219**
(units 6, 7, 9, 11, 10, 12, 8, 14, 15 in merge order).

## The walk

| # | Criterion (verbatim from `work/blend/plan.md` §Exit shape) | Disposition | Evidence / honesty note |
|---|---|---|---|
| 1 | "Units 14 and 15 land beside the twelve" — the twelve | MET-WITH-RECORDED-HONESTY | **E units**: 1 (`escalated-recourse-dispatch-has-no-coaxiality-arm`, PR #2123), 2 (`sweep-top-field-docs-make-the-spatial-claim-capend-shed`, PR #2122), 3 (`blend-recourses-under-describe-their-doors` §1, PR #2141), 4 (`rim-seed-finders-disagree-on-at-this-radius`, PR #2129), 5 (`sweep-doc-comments-cite-tests-unenforced`, PR #2155) — each one PR, a single style review, the fix in the item. **H units**: 6 (PR #2215, ordinal 2900, sample #176 — the ring-clearance forms, the hostless rim's closed-form clearance), 7 (PR #2483, 2901, #177 — `battery::Junction`, the closed chain's junctions paired with the link that touches them), 8 (PR #2505, 2905, #185 — `split_fragment`/`retire_fragment`, the debug postcondition over both `Retired` arenas), 9 (PR #2491, 2902, #181 — `geom_brep::must_carry_over_edge` returning one verdict, the in-band policy typed at both smooth arms), 10 (PR #2497, 2904, #183 — the path fillet door asking the validator's own classifier of its stored form, two `PathError` arms with their recourses), 11 (PR #2495, 2903, #182 — arm-collects, door-picks through `fillet_select::nearest_candidate`), 12 (PR #2508, 2906, #184 — the fillet arm on `PathError::Escalated` through one map, `EscalationSite::Fillet` retired). **The honesty**: the orchestrator's specs were corrected by measurement in every H unit — the dome's convexity and oracle (6), the concave twin (7), a false "unmeasured" premise and an unreachable door (8), the folded lever-arm count (9), the loss mechanism and a backwards recourse (10), the pick's stated reason false at every grid entry (11), two "pre-empted" gates driveable in band (12) — each recorded in the spec's ledger entry at its deletion and in the unit's log entry; none was presented as the spec's win. |
| 2 | "Units 14 and 15 land beside the twelve" — 14 and 15, "promoted from the slate's residues on 2026-09-13" | MET-WITH-RECORDED-HONESTY | **14** (PR #2509, ordinal 2907, sample #218): the blend's contact edges routed through the must-carry rule with `FILLET3_CONTACT_RECOURSE` conditioned by the site after both reviewers falsified its lever by execution; the reached `UnderDetermined → Chart` arm pinned. **15** (PR #2514, ordinal 2908, sample #219): one fall-through — `geom_core::MissingRecourse` at both `Escalated` Displays, one reader in `test_utils::source::predicate_census` after both reviewers defeated the per-crate copies by mutation, the deliberately-unrouted list in `src`, pairing pinned on both doors. **The honesty**: unit 14's state-sync merged four days after its delta (the orchestrator's session was idle from 2026-09-13 ~14:45Z to 2026-09-17), across a main that had rewritten the rod helper the unit generalised — the helper was rebuilt on main's extrusion family and type-checked before the gate; unit 15's brief carried a deviation miscount (eight against the body's seven) and its spec's constant became a newtype, its per-crate census one homed reader, its cross-crate "two sentences" list an intra-crate one — recorded at the spec's ledger entry. |
| 3 | "13 stays blocked on PROPS' H5 and is walked as such" | CARRIED (CARVE; blocked on PROPS' H5) | `S90-impl` — tighten the blend seam's three doors to `CertifiedBounds` — could not run: the lane-trait split PROPS' `H5` names has not landed. The plan's commitment that this program "owes the per-read classification of the nineteen bracket reads so the day `H5` lands the tightening is one PR" is NOT discharged: no unit classified the reads. The row moves to CARVE's slate (the seam doors are `crates/sweep/src/blend/*`, CARVE's ground) by `git mv` in this walk's PR, blocked on PROPS as before, with that owed classification named in its body as the successor's first step on it. |
| 4 | "the ruling is answered" | MET | `ambiguity-k-below-the-cap-rim-crossover`: Ev ruled in-chat on PR #2119 (2026-09-08) that the K floor deserves no special case — nothing bad happens without one — and unit K (PR #2149) removed the special case and `SmoothCapRim`, leaving no constraint on K; the item closed 2026-09-08; the plan's D section was left stale until the cut restated it. |
| 5 | "Track T is empty" | MET-WITH-RECORDED-HONESTY | Code-quality Track T's remainder on `crates/sweep/src` was claimed whole at opening: `D320`–`D324` were already closed; the two claimed rows were `sweep-doc-comments-cite-tests-unenforced` (unit 5, PR #2155, closed) and `S90-impl` (row 3 above — carried, not empty). `S116p` stayed a ruling in `work/code-quality/` as the plan said. So Track T is empty of everything this program could run. |
| 6 | "the residue slate is empty by the cut of 2026-09-17 (twenty-one rows to CARVE, eight to PATHS, one each to SYM, TOPO and META — the table is in `work/blend/log.md`'s cut entry)" | MET | PR #2810 (merged at 947056714 on Ev's in-chat go of 2026-09-17): thirty-two moves by `git mv`, bodies untouched, on VIEW's 2026-09-17 precedent; CARVE opened on BLEND's ground with band 5600–5699 and a charter that is true of its twenty-one rows and false of the others'; the eight profile-door rows to PATHS (S-BOOL's successor for `crates/profile`); the SYM-door row, the `rim_of` row and the suite-naming rule to their owners. Four rows landed in `work/blend/` after the cut branch was drawn — unit 14's `contact-edge-arm-is-picked-from-the-carrier-kind-not-the-dihedral` and `ruled-band-keys-a-d-hole-rim-on-the-caps-outer-cycle`, and unit 15's `corner-config-recourse-and-policy-assert-a-default-for-any-tag` (its `flush-invents-a-predicate-name-for-a-nameless-escalation` went straight to `work/issues/`) — and move to CARVE with this walk, alongside unit 13. |
| 7 | "the two docs rows closed in place" | MET | PR #2811: `kernel-verbs-cap-pair-ulp-claim-stale` (the register now says what the rows pin, cited as `<module>::<row>`) and `assembly-recourse-omits-the-transverse-cap-open-chain` (already widened by PR #2141; its status had been left at `review`). Ev's instruction of 2026-09-17: finish them here rather than open a docs successor. |
| 8 | "the walk convention applies" | MET | Fifteen unit merges and one ruling, each on a hosted green run verified at the run level (39 jobs, 12 `test (…)` and 5 `k-lint (gate, …)`, skips standing) before the merge, the A/B row riding the unit branch as its last commit under the docs-only-over-a-green-head convention — and where the landing merge of main carried code into the unit's crates (units 8's, 14's and 15's did), CI was re-run on the merged head before the merge. This document is the walk. |
| 9 | Process (§Review posture, verbatim in substance): "full v6 dual with Fable specs for the H units; E units take a single style review … no A/B row"; blocks drawn branch-side; ordinals claimed on main at review dispatch from band 2900–2999; blinding; record-at-merge | MET-WITH-RECORDED-HONESTY | Nine v6 duals, ordinals 2900–2908 all claimed on main at dispatch, rows at merge with per-phase tokens and wall-clock, briefs symmetric and stored with digests on the block branch before either arm ran, blocks BLEND-B1–B3 drawn branch-side from a private byte and folded into main as CONCLUDED records at each block's close. **Tally**: zero counted candidates in nine duals; three recorded and excluded — unit 9's (a receipt finding, 3(d)), unit 14's (a file header's invariant, doc-class by reading, 3(b)/(d)) and unit 15's (a third name-keyed table, by reading, the other arm adjudicating it the opposite way, 3(d)); every headline bilateral by execution (at MAJOR on 11, 12, 14, 15; at differing severity on 6, 7, 8, 9, 10). **The honesty, six notes**: (1) two usage-limit outages (2026-09-08; 2026-09-13 ~14:45Z, from which the orchestrator did not wake until 2026-09-17) and one container restart (2026-09-13 ~13:58Z) killed implementer lanes and one delta mid-run — every lane resumed from its transcript, no review ARM was interrupted, every pair counts; (2) the orchestrator's specs were wrong in a load-bearing sentence in every H unit (row 1) and the units are the record of that, not the specs; (3) unit 11's lane STOPPED at its fence once and was re-scoped by the orchestrator to arm-collects + door-picks with the lane's own measurement added to the spec; (4) the S-BOOL seam announcements that governed units 10–12 and 15 lived in `work/bool/log.md` and left the tracker with S-BOOL's directory on 2026-09-16 — recoverable at the ledger's sweep-15 SHA, and the class (an announced seam recorded only on the crossing program's side) is filed in `work/meta/`; (5) the suite-naming ruling (no unit number in a test file name; suites by subject) was made in-program when `blend6_` collided with S-BLEND's and is now META's row; (6) unit 8's union-merge resolution of a code file by the orchestrator was wrong once (both sides kept) and caught by its own type-check before the gate — the fix rebuilt the helper on main's version. |

## Walk evidence beyond the criteria

- **Unit K** (PR #2149) executed Ev's ruling with no floor and no special
  case — the plan's one `[ev]` question, answered in the program's
  first week.
- **The description rule as doctrine**: units 9 and 14 left
  `geom_brep::must_carry_over_edge` as the one rule three verbs' arms
  compose (extrude, revolve, the blend's contact edges), each arm
  storing its own conventional description; the remaining structural
  decision in `attach_contact` (the carrier kind picking the arm) is
  CARVE's row `contact-edge-arm-is-picked-from-the-carrier-kind-not-the-dihedral`.
- **The recourse table as doctrine**: units 10, 12 and 15 left the path
  door rendering, for every in-band verdict, a sentence true at every
  site its predicate fires (README A3-2), keyed through one map with one
  gap sentence — and left on PATHS' slate the five sentences the duals
  found false or under-specified at a site.
- **The experiment's yield**: nine duals, zero counted candidates, three
  recorded exclusions, nine bilateral headlines by execution — and two
  genuine adjudication splits (unit 12's "pre-empted" gates driven by
  both arms in different geometries; unit 15's third table read one
  way by each arm) — calibration data for the blinded coding.
- **Residue re-homed before this exit, by file**: by the cut
  (row 6) and by this walk's PR: `S90-impl`, the `CornerConfig` row,
  the carrier-kind arm pick and the D-hole rim keying to CARVE. Nothing
  is left in `work/blend/` but the program files, the closed rows and
  this walk.
- **What opens with the sweep** (per `work/README.md`): `work/blend/`
  is deleted at ratification with the walk, recorded in
  `docs/DOC-LEDGER.md` with the SHA; band 2900–2999 stays claimed and
  closed at 2908; CARVE's `keep_out` drops the "BLEND stays open" clause
  and its `program.md` reads present tense.
