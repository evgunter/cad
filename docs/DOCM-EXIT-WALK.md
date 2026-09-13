# DOCM — exit walk

STATUS: **RATIFIED** (Ev, in-chat 2026-09-13, before the walk was cut:
"if you have an exit walk to write then please write it as ready to
merge" — so this walk merges with the exit sweep on the last unit's
merge and is DOCM's done-state of record. A program is closed when its
exit walk is ratified; this document is deleted with the tracker
directory in the sweep that carries it and is recoverable at the SHA
`docs/DOC-LEDGER.md` sweep 14 names).

DOCM = the document model (`work/docm/plan.md`; opened 2026-09-03 from
`docs/WORK-TRACKS-2026-09.md` §DOCM as the owner of the editor-core
document layer — the persisted recipe vocabulary, the `DocEdit` set,
document identity, the frames and selectors the viewer and the mate
tool consume — with the fence drawn against code-quality Track V in
the opening commit; A/B band 1800–1899). The charter's eleven
questions supply the criteria, quoted verbatim, one commitment per
row; the plan's "Exit shape" supplies four more; dispositions per the
M5–M8/ASM/S-QA/S-BLEND/S-MATE/M10 convention: MET /
MET-WITH-RECORDED-HONESTY / CARRIED (named owner).

**Nine units merged**, every one on its own green hosted head:
`DOCM-4` (#1808), `DOCM-3` (#1803), `DOCM-1` (#1829), `DOCM-2`
(#1860), `DOCM-5` (#1871), `DOCM-7` (#2028), `DOCM-6` (#2035),
`DOCM-8` (#2073), `DOCM-9` (#«PR9») — ordinals **1800–1808**, samples
**#126, #127, #128, #129, #130, #148, #149, #154, #«S9»**; three
mechanical E units beside them with no review lane and no A/B row
(#1839, #1840, #1851). Two design conversations ratified in-chat
2026-09-04 and recorded as companion docs (DM1–DM6, DI1–DI5), amended
twice by Ev's later rulings (DM4's declaration channel in member
space, 2026-09-06; the flat merged name, 2026-09-06) and bounded once
by measurement (DOCM-8's review, 2026-09-07); at exit both are README
pages beside the code (`crates/editor-core/REFERENCES.md`,
`crates/editor-core/IDENTITY.md`). Three v3 blocks drawn and FULLY
CONSUMED (DOCM-B1 byte 203, B2 byte 39, B3 byte 124 — nine slots).

## The walk

| # | Criterion (verbatim from the charter's eleven questions, then the plan's Exit shape) | Disposition | Evidence / honesty note |
|---|---|---|---|
| 1 | "**Persisted-variant compatibility** — what adding or renaming a persisted enum variant requires (forward-compat stance, migration). Unblocks `capend-top-bottom-contradicted-by-negative-extrude` and `fused-step-slot-aliases-arrival-spec` (both then E)." | MET | Needed no ruling: the Band-4 no-schema-version rule (`docs/DESIGN.md`, BOOL-13 — a format change is a corpus regeneration) already answered it, and the plan recorded that on 2026-09-04. The two E units landed as mechanical units under the orchestrator's ruling that mechanical units run no review lane and take no A/B row: **#1851** (`CapEnd::{End, Start}` name the sweep vector's own ends) and **#1840** (`StepArg::{SweepVal2, ArcLenVal2, Bulge2}`, `spec_slots`). Both items closed. |
| 2 | "**Frames** — `sketch-frame-from-face` (already `needs_ev`: frozen `Datum::Frame` vs a derived-frame datum over a `StableName`, plus a carrier-kind interrogation door vs any-face wording). Unblocks `add-profile-mints-no-frame` and `add-profile-placement-on-picked-face-frame` (CHROME builds) and `no-door-mints-mate-frame-from-face` (LIB holds the plan; the hand-off S-MATE's keep_out required is recorded here)." | MET-WITH-RECORDED-HONESTY | Ruled DERIVED, not frozen (DM1, with the sense read beside the pose DM1a and the typed non-planar refusal DM1b; ratified in-chat 2026-09-04); built as **DOCM-1** (#1829, ordinal 1802, sample #128): `Datum::FaceFrame`, the carrier-kind read as a value (DM2). **The honesty**: the unit's Phase 1 stopped at PP6's f64 sketch-plane fence — Ev ruled option A (the profile lift is on for the derived frame; PP6/DM1c amended, #1837) and the lane resumed as the same arm; the dual found one bilateral MAJOR (a key-guard row that could not fail — the feed was redundant and is gone) and two unilateral MAJORs, one per slot. The CHROME and LIB builds are theirs and open on their slates (`add-profile-mints-no-frame`, `add-profile-placement-on-picked-face-frame` — CHROME; `no-door-mints-mate-frame-from-face` parked on LIB's `LIB-B-FACE-FRAME`); `sketch-frame-from-face` closed. |
| 3 | "**Operand selectors** — `split-side-and-pattern-instance-as-operand` (a part-selecting operand vs a projection node)." | MET | Ruled a PROJECTION NODE (DM3); built as **DOCM-2** (#1860, ordinal 1803, sample #129): `Node::Part` selects a split's half or a pattern's instance as one body, its content-key tag chosen against the tree that already held 32. Two phases — a stop at the split's stamping and `topo`'s channel-less same-source assertions (ruled: the split's stamping fix in-fence; assertions read channel-less scalars as no evidence; fence widened to two sites) and the build; the dual's one bilateral MAJOR fixed. Item closed; `LIB-B-PART` filed for the Python door. |
| 4 | "**Deleting from a chain** — `no-docedit-splices-a-deleted-node` (survivor policy per node kind, typed refusals, schema bump; the build is H)." | CARRIED (EDIT) | Ruled in the negative for THIS program: DM6, "splice is not added" — a node's inputs are not rewired, and the flat operators (DM4's n-ary union with `DocEdit::SetMembers`, **DOCM-3**, #1803, ordinal 1801, sample #127) take the cases that motivated it. The H build the charter named was therefore not cut. The question the item still asks — whether a splice is a new persisted edit, with a survivor policy per node kind — stays a design conversation; the item is re-homed to EDIT's slate (item 1) at this sweep, unparked (it was parked on DOCM-7 by a mechanical re-park that DOCM-7 did not unblock; corrected at DOCM-7's close). |
| 5 | "**Document identity (LIBRARY-DESIGN A4)** — `save-a-copy-duplicate-id-bricks-store`, `memo-admission-and-resolver-state` (D→H build), `document-seam-no-in-session-change-detection` (store refresh shape, save-as warning, chooser vocabulary)." | MET-WITH-RECORDED-HONESTY | Ruled as DI1–DI4 (a held value names the world it came from; the memo is a pure function of the document and the store is the session's; an evaluation carries its document's identity; saving at a path never forks identity). Built as **DOCM-4** (#1808, ordinal 1800, sample #126): `Evaluation` carries its document's identity, A4's refusal sentence narrows to the seam; `memo-admission-and-resolver-state` closed on option 2. `save-a-copy-duplicate-id-bricks-store` closed on LIB's slate (its build was theirs); `document-seam-no-in-session-change-detection` is CHROME's, open there. **The honesty**: the dual's two bilateral MAJORs were the `pncad-py` pages contradicting the new A4 and A4's "every door" universal being false against six doors — both fixed; the adjudication filed `pair-doors-outside-the-three-do-not-check-document-identity`, which this walk carries to EDIT (item 3). |
| 6 | "**Layer-3 identity across rewinds** — `layer3-recipenodeid-aliases-across-rewinds`: one rule for every holder (generation stamp, clear on replacement, or stable names); the viewer build goes to VIEW." | MET | Ruled DI1 (a held node id is valid on the branch that minted it; a rewind is a new branch). The viewer build went to VIEW as the charter said: `layer3-recipenodeid-aliases-across-rewinds` is on VIEW's slate (parked, per VIEW's sequencing). Nothing of it is DOCM's. |
| 7 | "**Free-move commit** — `no-persistent-setplacement-session-op`, constrained by the G3 ratification." | MET | Ruled DI5 (releasing a free-move gesture IS the placement edit; no persistent session op). The build is VIEW's (`no-persistent-setplacement-session-op`, open on VIEW's slate); DI2's re-mount and DI4 went to CHROME. Nothing of it is DOCM's. |
| 8 | "**Revolve naming D1** — `revolve-pole-export-interior-on-axis-vertex` (what the pole export yields for an interior on-axis vertex now editor-reachable; one row per direction after)." | MET | Needed no ruling (the Band-4 rule again); landed as the mechanical unit **#1839**, the rule written beside the revolve roles with one row per direction. Item closed. |
| 9 | "**The instantiation seam** — `instantiation-seam-drops-mate-identity` (narrows the Q1 ruling's letter; carry `MintedDeclaration` across `PartValue`)." | MET-WITH-RECORDED-HONESTY | Put to Ev as the plan's open question 1 (whether an inner mint refusal is the outer document's error); ruled 2026-09-06: YES, refuse at the outermost gate naming the inner document and mate, no advisory channel. Built as **DOCM-6** (#2035, ordinal 1806, sample #149): `PartValue` carries `minted`/`unminted`/`carried`/`carried_unminted`, the gather re-keys them through the graft's map, `Attribution::Carried` names mate and route, `AssemblyError::CarriedMintRefusal` precedes the own head and the at-rest gate. **The honesty**: the dual found the widened `Uncertified` predicate moved the outermost gate's verdict on a grazing part with no row and no sentence (R1 MAJOR) — accepted as the contract's, rowed and said in `ASSEMBLY.md`; A1's oracle was replaced by the reviewers' name-table oracle; one ruling was unbuildable (an outer mate naming a pair inside one instance refuses `SelfMate` first) and is pinned as a row that reds the day that door admits it. Item closed; `rows-do-not-cross-a-boolean-remap` filed (unreachable today; carried to S-BOOL). |
| 10 | "**The check registry's subject** — `check-registry-gathers-product-twice` (run_checks computes the product once; the fix edits `assembly.rs` and `product.rs`'s Dual arms — announce to M10's successor)." | MET-WITH-RECORDED-HONESTY | Built as **DOCM-5** (#1871, ordinal 1804, sample #130): `Subject`, `run_checks_on` as the door with document identity checked through `ident::mispaired`, `assemble_gathered` over a gathered product, `DocSession::land` gathering ONCE, the registry's cost measured with four terms separated and registered nightly. **The honesty**: the spec's own premises were wrong twice (a "release carries no counter" claim — this workspace keeps debug assertions on in release; a fence over `product.rs` "Dual arms" the file does not have) and the dual's two unilateral MAJORs (both R2's, by red probe) were real code defects — the gather moved above the config read, and the door binding nothing to nothing — fixed under three rulings. Item closed; the scene's extra gather (VIEW), the Python doors' double gather (LIB) and the debug-only counters' missing gate filed. |
| 11 | "**A certified range query** — `certify-locally-valid-range-instead-of-sampling` (M10 residue: a slot-widening override, an indeterminate-means-subdivide verdict contract, pacing; the build reuses M10-3's driver)." | «DISP9» | Waited on M10's parameter-aware certification settling (M10 closed 2026-09-13, sweep 13). Pacing ruled by Ev 2026-09-13: the sampling probe stays the interactive answer, the certified range is an on-demand query whose result replaces the probe's. Built as **DOCM-9** (#«PR9», ordinal 1808, sample #«S9»): «EVID9» |
| 12 | Plan §Exit shape: "Every ruling above is ratified in a companion doc or a README beside the code" | MET | DM1–DM6 and DI1–DI5 ratified in-chat 2026-09-04 (the companion docs' opening commits), DM4 amended by Ev's rulings of 2026-09-06 (the declaration channel in member space; the flat merged name) and bounded at DOCM-8's review 2026-09-07; at this exit both docs become `crates/editor-core/REFERENCES.md` and `crates/editor-core/IDENTITY.md` (present tense, clause ids kept; `docs/DESIGN.md`'s companion table repointed). `ASSEMBLY.md`'s A5 carries DOCM-6's seam. |
| 13 | Plan §Exit shape: "every build it cut has merged or has a home" | MET | Nine units and three E units merged (above). Every build handed off has a home on the receiving slate: CHROME (`add-profile-mints-no-frame`, `add-profile-placement-on-picked-face-frame`, `document-seam-no-in-session-change-detection`, DI2's re-mount, DI4, the certified-range affordance filed at DOCM-9), VIEW (DI1's layer-3 rule, DI5's free-move commit), LIB (`LIB-B-FACE-FRAME`, `LIB-B-PART`, the Python doors' double gather, the certified-range door filed at DOCM-9). |
| 14 | Plan §Exit shape: "the three open questions have ratified answers" | MET | Question 1 (the seam) — Ev 2026-09-06, DOCM-6; question 2 (the registry's subject) — the orchestrator's ruling at DOCM-5 under the plan's item; question 3 (the range query) — Ev 2026-09-13, DOCM-9. A fourth question the program raised on itself — the chained-order gap in the union's declaration channel — Ev ruled 2026-09-06 (the merged name is minted flat) and DOCM-8 built it, with the measured bound recorded in DM4 and its residue filed. |
| 15 | Plan §Exit shape: "the walk convention applies"; Process: standard v6 — spec → one implementer + the cross-model dual review + union fix pass; ordinals claimed on main at review dispatch from band 1800–1899; record-at-merge; blinding discipline verbatim; mechanical units on the orchestrator's read | MET-WITH-RECORDED-HONESTY | Nine v6 duals, ordinals 1800–1808 all claimed on main at dispatch, rows at merge, briefs hashed and diff-identical modulo lane names, private target and scratch directories per lane from DOCM-3's glimpse onward; three blocks fully consumed with every arm restated in its merged row. **The honesty, five notes**: (1) DOCM-3's R1 glimpsed R2's build-script PATHS through the shared scratchpad — no content — and the hazard became a memory line and a brief rule. (2) Four stop clauses fired (DOCM-1 Phase 1, DOCM-2, DOCM-3, DOCM-8); every one was amended and resumed as the same arm, logged branch-side on the block record. (3) Two lanes were cut by account limits mid-run and resumed on their own worktrees (DOCM-7's and DOCM-6's fix passes, 2026-09-06); DOCM-8's R2 was terminated before its first action by the account's API credits running out and restarted ~19 h after R1 on the same frozen head — a wall-clock asymmetry only, disclosed in its row. (4) Two brief flaws the reviewers corrected on the record: a "D-1" clause DOCM-6's spec cited that `ASSEMBLY.md` does not have (A5), and a deviation numbering taken from a lane report rather than the PR body. (5) The two blinded orchestrator merges on unit branches (the amended-spec merge into DOCM-8's branch) carry the orchestrator's own trailers, disclosed in the row. This document is the walk. |

## Walk evidence beyond the criteria

- **The experiment's yield.** Tally candidates coded per the H4
  precedent, per dual: DOCM-4 +0, DOCM-3 +0, DOCM-1 +2 (one per slot),
  DOCM-2 +0, DOCM-5 +2 (both the R2/opus slot), DOCM-7 +2 (one per
  slot), DOCM-6 +1 (the R1/opus slot), DOCM-8 +2 (one per slot),
  DOCM-9 «T9» — «TOTAL» candidates in nine duals, coding at the blinded
  readout Ev has kept owed ("keep the duals for the moment",
  2026-09-06). Two verdict splits on converged substance (DOCM-6's
  `Uncertified` widening, DOCM-8's prose over-claim — MAJOR at one arm,
  MINOR at the other) are calibration data for the coding. The
  program's own reading, for the readout to test: every unilateral
  MAJOR that survived adjudication was a claim in the PR body or a
  sentence in the tree that a probe falsified; not one was a wrong
  bit shipped past both arms.
- **The doctrine this program leaves behind.** A recipe reference is
  a datum carrying a face name (DM1), a carrier-kind read is a value
  (DM2), a part of a multi-body value is a projection node (DM3), the
  union is n-ary with a member-space declaration channel and a flat
  merged name (DM4), a node's inputs are pairwise distinct (DM5), and
  splice is not added (DM6); a held node id is valid on its branch
  (DI1), the memo is a pure function of the document (DI2), an
  evaluation carries its document's identity (DI3), forking is its
  own act (DI4), releasing a free move is the placement edit (DI5).
  Each is a README clause beside the code; the kernel clauses carry
  the rows that pin them, and the three whose builds are the viewer's
  (DI1's layer-3 rule and DI5's free-move commit — VIEW; DI2's store
  re-mount — CHROME) say so in their record lines and are open on
  those slates, as the plan handed them off. DI4 was built by CHROME
  (2026-09-08), not by a DOCM unit.
- **The measured bounds the program recorded rather than
  overpromised.** A member-space declaration resolves through MERGES
  only — a face consumed by a split, by containment, or by a
  fragmented merge stays order-shaped (DM4's bound; filed to WIRE at
  this sweep); an inner document's mint refusal refuses the outer
  gate (no advisory channel, by ruling); the check registry's census
  costs what it costs (~11.4 s at 161 solids), registered nightly.
- **Residue at exit, every row with a home** (the sweep table in
  `docs/DOC-LEDGER.md` sweep 14 is the record): twelve rows to the
  successor **EDIT** (the edit vocabulary's missing doors, the
  persisted recipe's honesty, the resolver's doors, the two Track V
  mirrors); seven to WIRE (the names emitters and the union's
  look-through residue); four to S-BOOL (the boolean's own refusals a
  declared union reaches); four to DOOR (fixes already written); two
  to S-TINT; one each to MSOLVE, CENSUS and PORT. Closed at the sweep
  with the reason in the file: the reader-census ledger line (it
  exists on main). Nothing to `work/issues/`.
- **Open with Ev at exit** (nothing blocks this walk): the v6
  stream readout (`work/meta/ab-log-v6-stream-is-past-its-stopping-rule-unadjudicated`),
  whose candidate list this program's nine rows feed.
