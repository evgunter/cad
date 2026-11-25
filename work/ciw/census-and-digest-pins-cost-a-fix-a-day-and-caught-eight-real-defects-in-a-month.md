---
id: census-and-digest-pins-cost-a-fix-a-day-and-caught-eight-real-defects-in-a-month
kind: issue
title: Censuses, hand-kept rosters and digest/count pins went red 81 times in a month and caught 8 real defects; restructure each to its claim, or delete it
status: parked
opened: 2026-11-22
priority: P1
cost: H
refs: [a-stored-data-change-runs-none-of-the-slow-rows-that-read-it, dependents-slow-sets-skip-a-kernel-pr]
blocked_on: [an-interval-endpoint-can-be-read-without-asking-whether-it-certified]
---

Filed by the CIW orchestrator, 2026-11-22, on Ev's question in chat:
are the slow rows that keep reddening main catching defects, or making
work? "if these tests are mostly making work for no reason then the much
bigger win is to get rid of them."

**Measured.** Every main red filed from 2026-10-22 to 11-22 (202 rows;
20 rustdoc, build or fuzz rows dropped; 24 duplicate filings merged; 158
distinct incidents) was classified by what its fix changed. REAL: a
production defect fixed in `crates/*/src`. REAL-P: judged a regression and
pinned, still open. TEST: the production change was intended and only the
test moved. CONF: production code reshaped only to satisfy a census, with
behaviour unchanged. AMB: no ruling, or healed with the cause untraced.

| family | n | REAL | REAL-P | TEST | CONF | AMB |
|---|---|---|---|---|---|---|
| censuses and rosters | 51 | 4 | 0 | 39 | 7 | 1 |
| digest, id, ledger and count pins | 30 | 3 | 1 | 20 | 2 | 4 |
| rows at 1e-6 and 1e-12 | 40 | 6 | 3 | 9 | 0 | 22 |
| other slow behaviour rows | 37 | 11 | 5 | 16 | 0 | 5 |

Behaviour rows at the default ε earn their place: 43% of their reds were
real. **Censuses, rosters and digest pins do not:** 81 reds, 8 real (all
fixed before any wrong answer was seen), 61 fixes that only moved the test
or reshaped code to satisfy it. Each of those 61 cost a red main, a
diagnosis, often a bisect, and a fixing PR.

**What to do with each family.** Restructure each test to the claim it
exists for, so it reds when the claim breaks and not when an intended
change moves a number. Where no claim survives that, delete the test.

- **Exact counts become the claim.** `geom-core/tests/certified_endpoint_census.rs`
  had 15 reds and 3 real catches. All three were reads that skip
  `is_certified()`; the other twelve were per-file count re-pins, one a
  line reflow. Gate only on an unasked, undispositioned read and drop the
  counts. Better still, remove the need for it: SCALAR's
  `an-interval-endpoint-can-be-read-without-asking-whether-it-certified`
  (P1, design) makes the forgotten ask a compile error, and this census
  then shrinks to the named escape's call sites. Do the cheap
  restructure now only if that design is far off.
- **Hand-kept lists go**, or become derived: `reader_census`,
  `node_standing` `READERS`, the binding census, the tier-1 `ALLOWED`
  roster, the box-door `PINNED` inventory, `raw_door_census`,
  `deny_unknown_fields` and `hand_written_impl`, `flagged_census`,
  `live`'s `loop_keys`, `recourse_roster` (topo and geom-brep) and
  `predicate_name_coverage`. Together they had 19 reds and no real
  catch; every fix was a line added to a list.
- **Text censuses with no real catch are deleted or narrowed to one
  claim:**
  - `pncad-py/src/prose_census.rs`, three rows: 11 reds, fixed by
    `UNDECIDED` lines and Display arms reworded to the same text;
  - `topo/src/face_normal.rs`'s `no_source_file_folds_the_bit_by_hand`
    and its planar-flip row: 8 reds, five of them answered by
    bit-identical rewrites through `OutwardNormal::from_chart`;
  - the refusal-text and refusal-shape pins: `refusal_concision_chains`'
    word cap, `m5_pr12_die_body`, `germ_tilted_rod`'s Debug shape, the
    python north-star teapot and tour wall 3.
- **Digest and count goldens keep only what a reviewer cannot see in a
  diff.** These had 25 reds and 3 real catches:
  - `m10_sym_profile_interval.rs`'s `PLATE_LEDGER`, which moved 2 or 3
    times while a single PR was open;
  - `mass_props_are_thread_count_invariant`'s serial golden. Its claim is
    that threads agree, so compare the threads with each other, not with
    a stored number;
  - the seat4, seat7 and seat8 lowering digests, `m4_pr3_names_ci` and
    the boolean corpus name tables, which moved on key-only and mint-id
    changes;
  - the symbolic counts in `sym11`, `sym_9` and `m10_9`;
  - `perf12_census_goldens`, the `m10_p_fence` digest and the fixed pad
    budgets at 1e-6.

  The three real catches were a rim certified twice (a Zero count 10 →
  18), a k-probe sample count 2409 against 2408 (tier 3′ skipping the
  session-box gate), and `segment_gap` (5542). Each is a count whose
  claim can be stated directly: one certification per rim, the sample
  stream's length from its formula, a gap bound. Each row keeps its
  claim and drops the golden.

**Scope and seams.** Most of these tests live in other programs' crates,
and many enforce a rule a crate README states. Ev, in chat on 2026-11-22:
the wording of crate READMEs was mostly not reviewed, so a silly rule
there goes, and only a rule that might have a non-obvious reason goes to
an `[ev]` PR first. A restructure that keeps the claim and drops the
counts needs no ruling. A deletion whose rule carries a reason in its
README, DESIGN.md or ratifying commit (`git log -S`) goes to `[ev]`.
Otherwise it lands with its reason. Each PR tells the owning program on
its log. Work family by family, one PR each, starting
with `certified_endpoint_census` (the biggest churner, whose own fix row
already sketches the claim) and the hand-kept lists.

The ε rows' 22 AMB reds are a different problem, and not this row's:
pinned 1e-12 kernel-limit refusals waiting for rulings.

What would close it: every test named above either asserts its claim
without an exact count, list or golden that an intended change moves, or
is deleted with the reason; and the next month's census and pin reds are
counted against this one's 81.

## Progress (2026-11-23)

Three families are done (PRs 7494, 7496, 7501): the text censuses and refusal-text pins, the hand-kept lists, and the digest and count goldens. Each test now asserts its claim, with counts as ceilings, lists derived, and digests fed by geometry with certificate residuals read only as finite. Witness pins stay only where an open defect row needs one.

What remains is `geom-core/tests/certified_endpoint_census.rs` (15 reds, 3 real). It waits on SCALAR's `an-interval-endpoint-can-be-read-without-asking-whether-it-certified` (P1, design), which makes the census unnecessary. If that design is still far off, the cheap restructure in this row (red only on an unasked, undispositioned read) can go first.

Count the next month's census and pin reds against this row's 81.

## Appendix: the classification

# Main-red rows, 2026-10-22 to 2026-11-22: what the fix touched

Population: every `work/*/` item whose id starts `main-red-` or whose title says "main red", opened 2026-10-22..2026-11-22, collected from origin/main history (added files plus files since deleted or moved by the `work/issues/` retirement), deduplicated by id: **202 rows**. 20 are excluded (rustdoc, build breaks, fuzz seeds, meta/aggregate rows) and 24 are duplicate filings of another row's red. That leaves **158 distinct incidents**. Every incident was classified (no sampling). The evidence was the row's Closed/Resolution/pin sections and, where a PR is named, `git diff --numstat` of its merge (src vs tests). Spot checks with `git show` covered 6522, 6525, 6550, 7181 and 5814.

Classes:
- **REAL**: a production defect, fixed in `crates/*/src`.
- **REAL-P**: a production regression that the owner's row or a reviewer judged a defect. It is pinned and the fix is still open.
- **TEST**: the production change was intended. Only the test moved (re-pin, re-bless, list entry, tolerance or fixture).
- **CONF**: production code was reshaped only to satisfy a source census. It is bit-identical or the same text, so no behaviour changed.
- **AMB**: unresolved or self-healed with the cause untraced. Most of these are still-open pins at 1e-6/1e-12 whose owner has not ruled.
- **DUP**: another row's red.

Effort is days from opened to closed, a proxy (most fixes are same-day or next-day).

| # | row id | opened→closed | breaking PR | fixing PR/commit | class | family | test(s) | why |
|---|---|---|---|---|---|---|---|---|
| 0 | `a-cylinder-circle-and-a-chart-junction-escalate-in-band-at-1e-12` | 2026-11-14→open | 6505/6466 | open (pinned) | AMB | non-default-eps behaviour | sweep far-origin rods + slanted bores @1e-12 | VolumeUncomputable in-band at 1e-12; kernel-limit refusal, owner row open, no wrong answer shown |
| 1 | `a-glued-rod-escalates-a-section-straddle-at-1e-12-since-the-cylinder-node-listing` | 2026-11-14→open | 6466 | open (pinned) | AMB | non-default-eps behaviour | sweep a_rod_glued_in_its_bore @1e-12 | Coincidence(Section,Moot) escalation at 1e-12; owner open, undecided |
| 2 | `a-node-end-row-certifies-a-coarse-span-onto-the-node-at-1e-6` | 2026-11-17→2026-11-17 | 6897 | 7057 (src pair_section, ssi/certify) | REAL | non-default-eps behaviour | topo node_end_rows @1e-6 | coarse span certified onto node where TubeStraddles owed; kernel fixed |
| 3 | `g-moves-four-main-reds-messages` | 2026-10-29→2026-10-30 | 5954 (INTENT G) | INTENT 10-30 re-pin + SYM row | REAL-P | digest/id/ledger/count pin | m10_9 R2 pad symbolic count; m10_sym_profile digest; sweep thread-count | 1 of 3 moves judged a defect (rule H loses 2 pad decisions, filed on SYM); 2 correct, re-pinned |
| 4 | `idealised-sweep-closed-forms-held-inside-certified-enclosures` | 2026-10-24→2026-10-24 | 5640 (QUAD) | row closed 10-24 | TEST | other slow behaviour | demos tour projectbox wall-1 pin | wall stopped refusing (improvement); pin retired, check strengthened |
| 5 | `lofted-concave-prism-pad-pin-moved-at-1e-6` | 2026-10-31→2026-11-14 | 6092 | 6934 (re-pin) | TEST | non-default-eps behaviour | LOFTED_CONCAVE_PAD_AT_1E6 pad pin | pad narrowed by deliberate schedule change; 6092 re-pinned the default-ε siblings only |
| 6 | `main-red-5734-moves-tilted-cut-upper-and-two-reach-backstop-rows` | 2026-10-27→2026-10-27 | 5734 | 5818 (QUAD re-cut) | TEST | digest/id/ledger/count pin | mass_props_are_thread_count_invariant tilted_cut_upper | digest moved with exact sliver bracket; re-cut, holds closed form |
| 7 | `main-red-a-ball-r-0-1-across-the-cylinder-rim-refuses-b-minus-a-at-edge-in-face` | 2026-11-20→2026-11-22 | 7052 | 7392 (pin) -> SECTARMS P0 open | REAL-P | other slow behaviour | cylinder_sphere_frame ball r0.1 rim B-A | build turned into CarrierUnread refusal; owner P0 regression, design open |
| 8 | `main-red-a-carrier-whose-origin-stands-far-off-leaves-the-union-unevaluated` | 2026-11-09→2026-11-14 | 6505 | 6967 (pin) | AMB | non-default-eps behaviour | union_fold_face_face_touch far-origin @1e-12 | same in-band volume refusal as #0 |
| 9 | `main-red-a-cone-stated-image-fuzz-seed-fails-at-1e-12` | 2026-11-12→2026-11-12 | - | 6857 | - | excluded | fuzz seed | excluded: fuzz seed |
| 10 | `main-red-a-fan-mev-plane-x-nurbs-row-escalates-its-general-lane-at-1e-12` | 2026-11-11→open | 6532 | 6967 (pin) | AMB | non-default-eps behaviour | topo fan_mev plane x nurbs @1e-12 | finding order differs at 1e-12; row may just be eps-specific |
| 11 | `main-red-a-lofted-arc-walls-rim-zeros-count-eighteen-not-ten` | 2026-11-04→2026-11-09 | 6484 | 6692 | DUP | digest/id/ledger/count pin | a_loft_computes_its_walls_in_t rim Zero count | duplicate of #12 |
| 12 | `main-red-a-lofted-arc-walls-rim-zeros-count-eighteen` | 2026-11-04→2026-11-09 | 6484 | 6692 (src topo pcurves.rs) | REAL | digest/id/ledger/count pin | a_lofted_arc_wall rim Zeros 10->18 | rim certified twice; certificate now handed on, pin kept at 10 |
| 13 | `main-red-a-lofted-operand-and-an-approx-cap-refuse-differently` | 2026-10-27→2026-10-27 | 5789 | 5814 (src boolean) + 5837 | REAL | other slow behaviour | m5_s11 lofted operand; operand_gate_pose Approx cap | Approx operand built a body against a roster decision; kernel fixed |
| 14 | `main-red-a-near-node-cap-at-1e-8-refuses-on-quadrature` | 2026-11-03→2026-11-14 | ? | none (healed) | AMB | other slow behaviour | bisector_nodes near-node poses | passed on a later nightly; cause untraced |
| 15 | `main-red-a-near-node-cap-loses-its-volume-to-a-projected-ruling-image` | 2026-11-04→2026-11-04 | 6470 | 6478 (feature) | AMB | other slow behaviour | bisector_nodes near-node cap | greened by a new rotational volume lane, not a fix of 6470 |
| 16 | `main-red-a-nearest-tie-in-band-on-a-bar-across-the-hole-no-longer-escalates-at-1e-6` | 2026-11-11→open | 6752 | 6934 (pin), open | AMB | non-default-eps behaviour | germ_plane_torus_frame nearest tie @1e-6 | escalation no longer fires; undecided whether lost |
| 17 | `main-red-a-needle-cavity-12eps-channel-escalates-at-bool-contact-vertex` | 2026-11-01→2026-11-11 | 6225 | 6812 (src reduce.rs) | REAL | other slow behaviour | needle cavity 12eps channel | 'The kernel was wrong': settle judged thinness by a probe, fixed |
| 18 | `main-red-a-partner-tipped-off-the-face-refuses-line-wedges-overlap` | 2026-11-01→2026-11-01 | - | 6256 | DUP | other slow behaviour | line wedges partner tipped | duplicate of an already-fixed row |
| 19 | `main-red-a-prism-tilted-within-the-zero-escalates-at-the-sector-reach-since-arm-vs-reach` | 2026-10-30→2026-11-01 | 6053 | 6212 (src sectors/insert/vtxfac) | REAL | other slow behaviour | topo three_solids_on_one_line prism tilt | normalize divided tilt by sin10deg; line now read unnormalized |
| 20 | `main-red-a-ray-about-a-partial-torus-rim-counts-its-ends-with-no-material-beyond` | 2026-10-29→2026-11-01 | 5697 | 6168 | - | excluded | fuzz seed | excluded: fuzz (was real) |
| 21 | `main-red-a-rod-tipped-past-the-band-builds-at-1e-6-since-the-span-foot` | 2026-11-22→open | 7112 | 7383 (pin), ONEHOME P1 open | REAL-P | non-default-eps behaviour | parallel_cylinder_join tipped rod @1e-6 | decided refusal became a body tier 3' cannot certify |
| 22 | `main-red-a-round-keel-in-the-plane-reads-off-annuli-at-1e-6` | 2026-11-09→2026-11-14 | 6365 | 6934 (pin) | AMB | non-default-eps behaviour | a_round_keel_in_the_plane... @1e-6 | witness of an already-open defect drifts 1.5e-6 at 1e-6 |
| 23 | `main-red-a-seamless-pipes-curved-junction-row-is-red-at-eps-1e-12` | 2026-11-22→open | ? | open | AMB | non-default-eps behaviour | topo seamless pipe curved junction @1e-12 | unbisected, payload unknown |
| 24 | `main-red-a-section-ends-own-rounding-escalates-at-1e-12-after-angle-reads` | 2026-11-04→2026-11-04 | 6482 | 6522 (test re-pose, 6 lines) | TEST | non-default-eps behaviour | section_end_poses own-rounding @1e-12 | new angle read is closer; pose re-posed |
| 25 | `main-red-a-sheared-fillet-witness-escalates-its-pcurve-envelope-at-1e-12` | 2026-11-04→2026-11-04 | 6510/6513 | 6533 (src blend/arms) | REAL | non-default-eps behaviour | band_planar_cut_off sheared fillet @1e-12 | lossy u_ref read; admission premise false; fixed |
| 26 | `main-red-a-tall-thin-cylinder-pairs-section-reads-limb-2-in-band-at-1e-12` | 2026-11-14→open | ? | 6934 (pin), open | AMB | non-default-eps behaviour | verbs_cylcyl_r1 tall thin cylinder @1e-12 | limb-2 in band after step budget; unbisected |
| 27 | `main-red-a-torus-carriers-limb-two-residual-exceeds-its-bound-by-rounding` | 2026-11-06→2026-11-06 | - | 6573 | - | excluded | fuzz seed | excluded: fuzz (test slack fix) |
| 28 | `main-red-aimed-ray-across-a-pair-in-band-at-1e-6` | 2026-11-02→open | 6266? | open | AMB | non-default-eps behaviour | offer_rows aimed_ray_across_a_pair_in_band @1e-6 | fixture gap does not scale with eps (likely test) |
| 29 | `main-red-an-approx-cap-clear-of-a-bar-builds-past-the-approx-door` | 2026-10-27→open | 5789 | 5814 | DUP | other slow behaviour | operand_gate_pose approx cap | same red as #13 |
| 30 | `main-red-axis-normal-snap-refuses-nist-ftc-09` | 2026-11-20→open | 7171 | open (GERM) | REAL-P | other slow behaviour | step-import nist_ftc_09 + 3 slow rows + render lane | NIST file import now refuses at a snap relation; regression owed by GERM |
| 31 | `main-red-band-turn-overrun-needle-refuses-its-census-at-3-degrees-at-eps-1e-12` | 2026-11-04→open | 6500? | open | AMB | non-default-eps behaviour | band_turn_overrun needle 3deg @1e-12 | census cannot settle count at 1e-12; unbisected |
| 32 | `main-red-blend-refusal-rewording-breaks-the-concision-cap-and-the-die-body-row` | 2026-11-01→2026-11-01 | 6200 | 6227 (src blend text) | CONF | digest/id/ledger/count pin | refusal_concision_chains 75-word cap; m5_pr12_die_body text | reworded refusal over word cap; recourse text shortened, text pins followed |
| 33 | `main-red-bounds-census-misses-reach-rs-crossing-and-sure-below` | 2026-11-20→2026-11-22 | 7114 | roster line | TEST | census/roster | geom-core bounds_census | roster line added (Selection); other site on BAND row |
| 34 | `main-red-bounds-roster-misses-the-four-along-side-door` | 2026-10-29→2026-10-29 | 5938 | 6040 (src four.rs) | REAL | census/roster | geom-core bounds roster | along_side moved a refusal on an uncertified bracket; now asks certified_bracket |
| 35 | `main-red-boxfinishs-merges-left-slow-rows-red-through-the-gates-slow-set-gap` | 2026-11-15→2026-11-15 | 6963/6959/6946 | 6990 (src profile mint) + 6984/6987 re-pins | REAL | other slow behaviour | revalue_corpus x5; sweep iso-image x2; thread-count golden | axis mint stored constant where a chord READ zero: real; iso/golden rows re-pinned |
| 36 | `main-red-certified-endpoint-census-drifted-in-geom-brep-after-6171` | 2026-11-01→2026-11-03 | 6171,6412 | 6437 (re-pin) | TEST | census/roster | certified_endpoint_census | 16 files re-pinned with dispositions |
| 37 | `main-red-certified-endpoint-census-moved-in-six-files` | 2026-10-28→2026-10-29 | 5855..5927 | 6040 (src: 7 sites now ask) | REAL | census/roster | certified_endpoint_census | 27 unasked reads; 7 fixed to ask certified_bracket (latent soundness), rest argued |
| 38 | `main-red-certified-endpoint-census-pins-pair-section-at-four-where-the-tree-reads-six` | 2026-11-20→2026-11-20 | 7112 | 7218 (re-pin) | TEST | census/roster | certified_endpoint_census | 2 reads already asking; count 4->6 |
| 39 | `main-red-coincident-wedges-overlap-refuses-flush-copies` | 2026-10-26→2026-10-26 | 5745/5704 | 5779 (census.rs pass 5) | REAL | other slow behaviour | topo+editor-core coincident wedges (7 rows) | flush copies refused as double overlap; kernel fixed |
| 40 | `main-red-cylinder-pair-the-region-cuts-unmarched-at-1e-6` | 2026-11-02→open | ? | open | AMB | non-default-eps behaviour | section_cert cylinder pair unmarched @1e-6 | march gives up at 1e-6; unknown |
| 41 | `main-red-cylinder-sphere-section-runs-out-of-cells-near-the-node-at-1e-6` | 2026-10-28→2026-10-28 | 5872 | sect/near-node-at-1e-6 (src ssi refine, foot) | REAL | non-default-eps behaviour | pair_section + cylinder_sphere_frame @1e-6 | two kernel defects (cell budget, foot across seam) |
| 42 | `main-red-docm2-a7-ladder-escalates-at-eps-over-32` | 2026-11-08→2026-11-09 | 6399 | boxes re-pin | TEST | other slow behaviour | docm2_part_interval a7 width ladder | sound cover widened enclosure; ladder floor moved one rung, re-pinned |
| 43 | `main-red-dual-corpus-reshaped-rod-census-disagrees-at-1e-12` | 2026-10-28→2026-11-09 | ? | healed by 6482 | AMB | non-default-eps behaviour | m10_di_dual_corpus reshaped_rod @1e-12 | pin removed; cause untraced |
| 44 | `main-red-edit-rs-folds-a-face-sense-by-hand` | 2026-10-27→2026-10-28 | 5742 | 5849 | DUP | census/roster | topo face_normal fold census | duplicate of #106 |
| 45 | `main-red-eighteen-sweep-analytic-rows-red-on-main-at-the-default-eps` | 2026-10-31→2026-11-02 | 6013 (Ev ruling) | 6343 (pins) + 5793 | TEST | other slow behaviour | 18 sweep analytic rows (rulings, cylinder_sphere_frame, torus_doors_join) | refusal moved one door later per ruling; pinned at new stop |
| 46 | `main-red-endpoint-census-has-no-pins-for-step-imports-on-built-reads` | 2026-11-22→open | EXCH 651aed96.. | open | TEST | census/roster | certified_endpoint_census | new file unrostered; dispositions owed |
| 47 | `main-red-endpoint-census-moves-interval-rs-on-less-sin` | 2026-11-20→2026-11-20 | 7146 | 7223 (re-pin, no code) | TEST | census/roster | certified_endpoint_census | 'not a certification read and no code changed' |
| 48 | `main-red-endpoint-census-moves-interval-rs-on-the-told-reading` | 2026-11-20→open | 7159 | open | TEST | census/roster | certified_endpoint_census | two told-reading reads unrostered |
| 49 | `main-red-endpoint-census-reads-41-in-speed-rs-since-pole-section` | 2026-11-20→open | 7180 | open | TEST | census/roster | certified_endpoint_census | speed.rs 38->41 unrostered |
| 50 | `main-red-five-slow-rows-refuse-at-limb-2s-kernel-limit-since-5728` | 2026-10-28→open | 5728 | open | AMB | non-default-eps behaviour | five rows limb-2 KernelLimit @1e-12 | one row's stop was the oracle's 2 km slab (test); rest undecided |
| 51 | `main-red-five-torus-section-rows-refuse-at-1e-6-since-components-by-foot` | 2026-10-30→2026-10-31 | 6011 | sect/torus-rows-at-1e-6 (src winds) | REAL | non-default-eps behaviour | five torus section/germ rows @1e-6 | winding pad grew to 0.11 m; reading fixed |
| 52 | `main-red-flagged-census-reads-a-lift-site-as-an-expression-citation` | 2026-11-10→open | ac5d2e44e2 | open | TEST | census/roster | geom-core flagged_census | census cannot read a non-literal citation |
| 53 | `a-planar-faces-volume-term-reads-its-unweighted-mean-vertex-so-in-band-vertices-lever-over-the-face` | 2026-11-14→open | 6505 | 6934 (pin) -> ONEDOOR P0 | REAL-P | non-default-eps behaviour | four sweep rows volume @1e-6 | volumes outside their own enclosures (review-verified); P0 open |
| 54 | `main-red-four-sweep-slow-rows-refuse-outside-session-box` | 2026-11-09→2026-11-10 | 8a97c7fab5 | 6760 (fixtures moved) | TEST | other slow behaviour | five sweep rows OutsideSessionBox | refusals true; fixtures past 1 km box moved |
| 55 | `main-red-inert-deny-on-the-qualifier-enums` | 2026-10-26→open | D'3,5871 | re-bless | TEST | census/roster | deny_unknown_fields_census; hand_written_impl_census | table counts re-blessed |
| 56 | `main-red-injective-rs-folds-the-face-sense-by-hand` | 2026-10-29→2026-10-29 | ? | 6028 (bit-identical) | CONF | census/roster | topo face_normal fold census | routed through OutwardNormal::from_chart; receipts unchanged |
| 57 | `main-red-loop-keys-guard-flags-spline-region-rows` | 2026-11-03→2026-11-12 | 6435x6460 | overtaken | AMB | census/roster | topo live loop_keys census | semantic merge cross; code gone before fix |
| 58 | `main-red-m10-bulge-d-tab-radial-moving-pcurve-chart-moved` | 2026-11-09→2026-11-19 | 6988,7088 | lanes re-pin | TEST | digest/id/ledger/count pin | m10_bulge d_tab pcurve_chart_radial_moving | re-pinned with causes |
| 59 | `main-red-m10-p-fence-interval-corpus-digest-moved` | 2026-11-03→2026-11-14 | 6482 etc | lanes re-take | TEST | digest/id/ledger/count pin | m10_p_fence Interval corpus digest | re-taken by moving lanes |
| 60 | `main-red-m10-sym-profiles-two-pinned-rows` | 2026-11-05→2026-11-06 | 6504,6527,6562 | 6560 (re-pin) | TEST | digest/id/ledger/count pin | m10_sym_profile walk ledger | re-pinned; ledger moved 3x during the PR |
| 61 | `main-red-near-tangent-pierce-poses-w60-a4-census-undecidable-at-1e-6` | 2026-11-01→open | 6212 | 6934 (pin), open | AMB | non-default-eps behaviour | near_tangent_pierce_poses w60 @1e-6 | CensusUndecidable; body not shown wrong |
| 62 | `main-red-node-standing-readers-census-trails-the-dprime2-mate-split` | 2026-10-25→2026-10-26 | 5613 | 5636 | TEST | census/roster | editor-core node_standing READERS | list update |
| 63 | `main-red-offer-rows-false-offers-since-the-told-quantity-type` | 2026-11-20→open | 7159 | open (SCALAR) | REAL-P | other slow behaviour | offer_rows every_offered_tolerance_passes_just_below_it | 16 false offers: rendered number cut below its own value |
| 64 | `main-red-overlap-passes-a-bare-margin-where-a-lever-is-wanted` | 2026-11-02→2026-11-03 | - | 6410 | - | excluded | compile break | excluded: build break |
| 65 | `main-red-partners-along-the-face-reads-an-edge-inside-the-along-face-at-1e-6` | 2026-11-02→2026-11-14 | ? | healed | AMB | non-default-eps behaviour | partners_along_the_face @1e-6 | passed later; untraced |
| 66 | `main-red-pcurve-cache-gained-two-endpoint-reads-in-limb2-offset-sup` | 2026-11-10→2026-11-14 | 6521 | 6831 (dispositions) | TEST | census/roster | certified_endpoint_census | reads dispositioned, roster re-pinned |
| 67 | `main-red-perf12-census-golden-kitchen-sink-loses-a-curved-pair-at-5817` | 2026-10-27→2026-11-02 | 5817 | re-bless | TEST | digest/id/ledger/count pin | perf12_census_goldens kitchen_sink | lost finding was spurious (0.236 apart); golden re-blessed |
| 68 | `main-red-perf12-kitchen-sink-census-drops-a-curved-pair-after-restread` | 2026-10-27→2026-10-28 | 5817 | - | DUP | digest/id/ledger/count pin | perf12 census goldens | duplicate of #67 |
| 69 | `main-red-predicate-name-coverage-reads-three-topo-test-literals-no-crate-decides` | 2026-11-16→2026-11-17 | ONECOPY test literal | 7060 (2 lines in a test) | TEST | census/roster | test-utils predicate_name_coverage | test literal respelled |
| 70 | `main-red-predicate-name-coverage-step-import-decides-names-and-runs-no-roster` | 2026-11-21→2026-11-22 | 7091 | 7371 (KNOWN_MISSING) | TEST | census/roster | predicate_name_coverage | allowlist entry citing an importgate row |
| 71 | `main-red-prose-census-cannot-decide-certify-errors-check` | 2026-10-30→open | 5959 | UNDECIDED line | TEST | census/roster | pncad-py prose_census | nested pattern; census cannot read; listed |
| 72 | `main-red-prose-census-cannot-decide-the-glue-record-broken-surfaces` | 2026-10-30→2026-11-12 | 6810 | lines struck | TEST | census/roster | pncad-py prose_census | variant retired, stale lines removed |
| 73 | `main-red-prose-census-cannot-decide-the-selection-refusal-face-arm` | 2026-10-30→open | 5977 | UNDECIDED lines | TEST | census/roster | pncad-py prose_census | listed with reason |
| 74 | `main-red-prose-census-cannot-read-a-guarded-edge-in-face-arm` | 2026-10-26→2026-10-27 | 5546 | 5830 (Display rewritten, byte-identical) | CONF | census/roster | pncad-py prose_census | guarded arm unreadable by census; source reshaped |
| 75 | `main-red-prose-census-misses-two-boolean-error-fields` | 2026-10-27→2026-10-27 | 5772 | healed | TEST | census/roster | pncad-py prose_census | gone on main |
| 76 | `main-red-prose-census-reads-no-write-renderer` | 2026-11-09→2026-11-09 | 6430 | 6685 (census rewrite) | TEST | census/roster | pncad-py prose_census | census found renderers by trait name; taught to read write fns; roster re-pinned |
| 77 | `main-red-quad-lanes-endpoint-census-reads-27-against-a-pin-of-25` | 2026-11-16→2026-11-17 | 6982 | GREEN re-pin | TEST | census/roster | certified_endpoint_census | 27 reads all asking; 25->27 |
| 78 | `main-red-reach-volume-backstop-midpoint-premise-and-planted-subtraction` | 2026-10-27→2026-10-27 | 5734 | - | DUP | other slow behaviour | reach_volume_backstop | duplicate of #79 |
| 79 | `main-red-reach-volume-backstop-rows-after-the-exact-sliver-bracket` | 2026-10-26→2026-11-11 | 5734 | 6817 (test) | TEST | other slow behaviour | reach_volume_backstop x2 | 5734's readings right; planted rows moved to oracle file |
| 80 | `main-red-reach-volume-backstop-rows-lose-their-premise` | 2026-10-27→open | 5734 | - | DUP | other slow behaviour | reach_volume_backstop | same two rows as #79 |
| 81 | `main-red-reader-census-lists-the-deleted-given-pair-test` | 2026-10-27→2026-10-27 | 5772 | ATREST | TEST | census/roster | test-utils reader_census | deleted file still listed |
| 82 | `main-red-reader-census-misses-topo-link-rs` | 2026-10-27→2026-10-27 | 5773 | ATREST | TEST | census/roster | test-utils reader_census | new file unlisted |
| 83 | `main-red-recharted-row-unstated-renders-its-relation-through-debug` | 2026-10-27→2026-10-27 | 5795 | wire/recharted-relation-words | CONF | census/roster | pncad-py prose_census (Debug payload) | Display renders relation in words (user text changed) |
| 84 | `main-red-recourse-roster-misses-chain-parity-and-spline-trim-decide-sites` | 2026-11-21→open | 7193? | open | TEST | census/roster | topo recourse_roster | two indirect sites unlisted |
| 85 | `main-red-recourse-roster-misses-intersect-rs-closed-form-shown-and-shown` | 2026-11-20→open | 7171 | open | TEST | census/roster | geom-brep recourse_roster | two indirect sites unlisted |
| 86 | `main-red-recourse-roster-misses-two-ring-path-indirect-sites` | 2026-11-13→2026-11-14 | 6891 | 6921 (roster) | TEST | census/roster | topo recourse_roster | three .travel() calls listed |
| 87 | `main-red-revalue-boss-union-answers-two-period-near-mark-decisions-on-either-side-since-6482` | 2026-11-14→open | 6482 | 6934 (pin), open | AMB | non-default-eps behaviour | revalue_boss_union @1e-6/1e-12 | two decisions flip on a margin of exactly 0; undecided |
| 88 | `main-red-revalue-boss-union-reads-two-period-near-mark-flips-at-1e-6` | 2026-11-11→2026-11-14 | 6482 | 6934 | DUP | non-default-eps behaviour | revalue_boss_union | duplicate of #87 |
| 89 | `main-red-ringed-curved-doors-reds-four-sweep-rows` | 2026-10-26→2026-11-20 | 5664 | triage (rows elsewhere) | AMB | non-default-eps behaviour | bisector_nodes, review_4781_r1, germarms2, ray-wall twins @1e-12 | umbrella; scattered to owners |
| 90 | `main-red-rows-two-lanes-found-red-on-their-merge-base-are-held-by-closed-rows-or-none` | 2026-11-22→open | - | - | - | excluded | meta | excluded: meta row about rows |
| 91 | `main-red-rustdoc-borders-links-to-obstacles-split` | 2026-11-09→2026-11-10 | - | - | - | excluded | rustdoc | excluded |
| 92 | `main-red-rustdoc-eval-mod-links-to-measure-observed` | 2026-11-09→open | - | - | - | excluded | rustdoc | excluded |
| 93 | `main-red-rustdoc-ops-link-and-demo-tour-since-the-finished-body` | 2026-11-09→2026-11-12 | - | - | - | excluded | rustdoc/build | excluded |
| 94 | `main-red-rustdoc-redundant-eps-min-links-in-predicate-rs` | 2026-11-09→2026-11-19 | - | - | - | excluded | rustdoc | excluded |
| 95 | `main-red-rustdoc-redundant-link-in-quad-patch-rs` | 2026-11-09→open | - | - | - | excluded | rustdoc | excluded |
| 96 | `main-red-rustdoc-step-import-links-to-topology` | 2026-11-09→2026-11-17 | - | - | - | excluded | rustdoc | excluded |
| 97 | `main-red-rustdoc-take-over-is-both-a-function-and-a-module` | 2026-11-09→2026-11-17 | - | - | - | excluded | rustdoc | excluded |
| 98 | `main-red-rustdoc-validate-links-to-surface-boundary-cycle-winding` | 2026-11-09→open | - | - | - | excluded | rustdoc | excluded |
| 99 | `main-red-seams-lying-on-the-walls-read-the-axes-at-1e-12` | 2026-11-06→2026-11-06 | 6adc5fdd | 6550 (sweep tests/common/differential.rs) | TEST | non-default-eps behaviour | parallel_cylinder_join seams @1e-12 | work wall at 1e-12 is cost frontier; guard re-written |
| 100 | `main-red-seat7-corner-table-moved-at-6832-and-kitchen-sink-after-it` | 2026-11-13→2026-11-14 | 6832,6820,5848 | 6956 (re-bless/pin) | TEST | digest/id/ledger/count pin | seat7_sweep_lowering digests | kitchen_sink: copy of a pin 6820 already re-blessed elsewhere; corner_table pinned unjudged |
| 101 | `main-red-section-end-poses-own-rounding-escalates-its-map-residual-at-1e-12` | 2026-11-04→open | 6482 | - | DUP | non-default-eps behaviour | section_end_poses @1e-12 | same red as #24 |
| 102 | `main-red-select-no-member-raise-composes-a-debug-rendering` | 2026-11-10→2026-11-12 | 6555 | reads/no-member-prose | CONF | census/roster | pncad-py prose_census (no Debug raise) | message rebuilt without Debug, same text |
| 103 | `main-red-seven-sweep-and-topo-rows-red-at-1e-6-since-11-17` | 2026-11-21→2026-11-22 | 7052/7112 | 7383 (pins/filters) | REAL-P | non-default-eps behaviour | 7 sweep+topo rows @1e-6 | regressions pinned; residue P1 rows on ONEHOME, SECTARMS |
| 104 | `main-red-since-pr-7052-the-recourse-roster-and-a-close-rings-pin` | 2026-11-17→2026-11-17 | 7052 | 7057 | TEST | census/roster | geom-brep recourse_roster + close_rings box count 206->208 | roster and count follow |
| 105 | `main-red-six-editor-core-interval-rows-red-behind-the-cancelled-full-suite` | 2026-10-31→2026-11-02 | 6105 | 6355 (re-bless 4, pin 2) | TEST | digest/id/ledger/count pin | sym11/sym_9/m10_9 counts; m10_3 drive | 4 counts = new decision; m10_3 drive refuses all leaves pinned (PCERT P2) |
| 106 | `main-red-sketch-on-face-folds-the-face-normal-by-hand` | 2026-10-27→2026-10-27 | 5742 | PR (OutwardNormal door) | CONF | census/roster | topo face_normal fold census | hand fold routed through door |
| 107 | `main-red-slanted-bores-a-union-b-census-ray-exhausted-at-1e-12` | 2026-11-02→open | 5664/6466 | open | AMB | non-default-eps behaviour | bisector_nodes slanted bores A u B @1e-12 | behind #0's volume refusal |
| 108 | `main-red-slanted-bores-b-minus-a-exhausts-its-probe-ray-at-1e-12` | 2026-10-26→open | 5664/6466 | open | AMB | non-default-eps behaviour | bisector_nodes slanted bores B - A @1e-12 | behind #0's volume refusal |
| 109 | `main-red-slow-rows-uncovered-at-b2337e9e` | 2026-11-03→2026-11-14 | - | - | - | excluded | meta | excluded: aggregate |
| 110 | `main-red-slow-set-receipts-moved-at-5542` | 2026-10-28→open | 5542 | 5952 (src seg.rs fix) + re-pins | REAL | digest/id/ledger/count pin | m10_6, m10_sym_profile, sym_9 receipts | 5542's two segment_gap defects fixed; residual drift re-pinned |
| 111 | `main-red-steeper-tilts-union-stops-by-identity-reads-the-wrong-refusal-shape` | 2026-11-09→2026-11-11 | 6371 | 6818 | TEST | other slow behaviour | germ_tilted_rod steeper tilts refusal shape | error retyped; test now matches variants |
| 112 | `main-red-swept-elbows-step-fixture-drifted-in-five-cap-directions` | 2026-11-16→2026-11-17 | 6988 | regen fixture | TEST | digest/id/ledger/count pin | step-export committed_fixtures_are_byte_golden | 3-ulp rounding; regenerated |
| 113 | `main-red-sym11s-numeric-column-fell-on-five-documents-after-the-least-turn-mint` | 2026-11-15→2026-11-16 | 6988 | 7016 | TEST | digest/id/ledger/count pin | sym11 numeric column | identity charts log no pole verdict; re-pinned |
| 114 | `main-red-tess-lint-release-budget-gate-fires-eight-findings` | 2026-11-09→2026-11-10 | 6138,6465 | 6799 (baseline re-cut) | TEST | digest/id/ledger/count pin | k-lint tess-lint release budget | deliberate frame changes; 'Nothing here is a defect' |
| 115 | `main-red-the-aimed-ray-seam-split-census-misses-its-settle-floor` | 2026-11-04→2026-11-14 | 6478 | 6931 (pin) -> SPIRICARM | REAL-P | other slow behaviour | an_aimed_ray seam_split census | sliver measure stops at floor 2600 volumes wide |
| 116 | `main-red-the-arc-rim-pins-read-the-retired-refusal` | 2026-11-04→2026-11-04 | 6484 | PR (IsoArc end weights) | REAL | other slow behaviour | dm1 arc-rim pins | pick missed a rim; 'neither new reading right' |
| 117 | `main-red-the-axis-edit-mates-the-copy-coincidence-door-row` | 2026-11-01→2026-11-02 | 6047 | 6355 (pin) -> 7181 (src) | REAL | other slow behaviour | coincidence_door axis edit mates copy | union refused RingHomingAmbiguous; path reading fixed |
| 118 | `main-red-the-blend-run-out-reword-fails-four-refusal-text-tests` | 2026-11-01→2026-11-01 | 6200 | 6227 | DUP | digest/id/ledger/count pin | refusal text tests | duplicate of #32 |
| 119 | `main-red-the-box-door-inventory-misses-three-edge-cover-readers` | 2026-11-03→2026-11-06 | 928e6189 | 6553 | TEST | census/roster | topo box-door inventory | three files added to PINNED |
| 120 | `main-red-the-brace-payload-census-flags-recharted-row-unstated` | 2026-10-27→2026-10-27 | 5795 | - | DUP | census/roster | prose_census | duplicate of #83 |
| 121 | `main-red-the-certified-endpoint-census-counts-one-arcs-split-midpoint-lines` | 2026-11-10→2026-11-10 | 4566f51520 (reflow) | 6737 | TEST | census/roster | certified_endpoint_census | line reflow moved count 18->20 |
| 122 | `main-red-the-chord-arc-rung-count-reads-zero-through-a-const` | 2026-11-10→2026-11-10 | e57e5d868e | 6746 (count reads consts) | TEST | census/roster | chord_join the_chord_arc_rung_is_decided_in_one_place | literal moved into const; census taught consts |
| 123 | `main-red-the-circle-lane-coefficient-charge-row-fails-on-some-seeds` | 2026-11-09→2026-11-10 | - | - | - | excluded | fuzz | excluded: fuzz seed (was real) |
| 124 | `main-red-the-collinear-walls-probe-expects-the-edit-door-to-refuse-a-tangent-junction` | 2026-11-19→2026-11-20 | 7116 | 7148 | TEST | other slow behaviour | collinear walls probe (all-features) | refusal moved to evaluation by design |
| 125 | `main-red-the-composed-die-door-proves-two-joints-where-one-is-pinned` | 2026-11-10→2026-11-10 | 6533/6482 | 6771 | TEST | digest/id/ledger/count pin | coincidence_door composed die joints | 2 proved where 1 pinned (improvement) |
| 126 | `main-red-the-copysign-census-names-a-walk-rs-site-since-the-horn-cusp` | 2026-11-11→2026-11-11 | 6806 | 6838 | TEST | census/roster | sym_rule_f_rows copysign census | f64-only site; census counts sites a Sym never reaches |
| 127 | `main-red-the-copysign-census-since-the-offset-conic-arc` | 2026-10-26→2026-10-27 | 5692 | 5874 | TEST | census/roster | copysign census | 'None is a defect'; registered |
| 128 | `main-red-the-corner-boss-log-test-panics-on-an-unlabelled-node` | 2026-10-27→2026-10-27 | 5753/5636 | 5820 | TEST | other slow behaviour | emit_union_borders corner boss | test helper at fault |
| 129 | `main-red-the-corner-boss-row-labels-a-node-no-member-is` | 2026-10-26→open | 5753 | - | DUP | other slow behaviour | emit_union_borders corner boss | duplicate of #128 |
| 130 | `main-red-the-cylinder-sphere-ruling-sweep-refuses-tube-straddles-near-the-node-at-1e-6` | 2026-11-11→2026-11-11 | 6518 | sectarms/ruling-sweep-straddle (src) | REAL | non-default-eps behaviour | 7 cylinder x sphere rows @1e-6 | sweep handed march a bad seed; fixed |
| 131 | `main-red-the-doc-gates-workspace-pass-masks-35-broken-links` | 2026-11-09→2026-11-10 | - | - | - | excluded | doc gate | excluded |
| 132 | `main-red-the-endpoint-census-counts-six-unasked-reads-in-critical-rs-since-6811` | 2026-11-11→2026-11-11 | 6811 | sectarms census disposition (src node_frame) | REAL | census/roster | certified_endpoint_census | node_frame width gated NODE_BOX unasked; now asks |
| 133 | `main-red-the-endpoint-census-misses-four-files-moved-on-main` | 2026-11-17→open | SECTARMS,6982 | re-pins | TEST | census/roster | certified_endpoint_census | four files moved |
| 134 | `main-red-the-endpoint-census-moved-in-four-props-files-since-6808` | 2026-11-11→2026-11-11 | 6808 | 6846 | TEST | census/roster | certified_endpoint_census | 21 reads sound; one site made to refuse uncertified radii |
| 135 | `main-red-the-endpoint-census-moved-in-spline-implicit-rs-since-6862` | 2026-11-12→2026-11-12 | 6862 | re-pin | TEST | census/roster | certified_endpoint_census | four reads already asking |
| 136 | `main-red-the-endpoint-census-since-flux-rational-rim-and-shell-pad` | 2026-10-26→2026-10-26 | 5696/5699 | 5809 | REAL | census/roster | certified_endpoint_census | one unsafe read in deciding_walk fixed |
| 137 | `main-red-the-filleted-die-moved-in-seat4-and-the-step-export-fixture` | 2026-11-09→2026-11-14 | 6533,6482 | 6956 (pinned unjudged) | AMB | digest/id/ledger/count pin | seat4 die_fillet digest; step-export filleted_die.step | pinned at main unjudged; judgement owed ONECOPY |
| 138 | `main-red-the-four-thread-walk-misses-tilted-cut-uppers-serial-golden` | 2026-11-04→2026-11-14 | ? | 6956 | TEST | digest/id/ledger/count pin | mass_props thread-count tilted_cut_upper | judged on closed forms; re-blessed |
| 139 | `main-red-the-gallery-chain-pins-196-at-rest-findings-after-pr-5853` | 2026-10-27→2026-10-28 | 5853 | 5888 | TEST | digest/id/ledger/count pin | demos gallery chain at-rest count | 10 findings true; 196->10 |
| 140 | `main-red-the-gather-records-one-sample-tier-3-prime-does-not` | 2026-11-09→2026-11-16 | 6380 | 7017 (src validate.rs) | REAL | digest/id/ledger/count pin | k-probe assemble_one_local_battery sample count | tier 3' skipped the session-box gate tier 3 asks |
| 141 | `main-red-the-k-probe-sweep-cannot-compile-geom-brep-under-probe-alone` | 2026-11-05→2026-11-09 | - | 6674 | - | excluded | probe build | excluded: build break |
| 142 | `main-red-the-lattice-discard-pin-moved-where-pred-5724-met-flush-5706` | 2026-10-26→open | 5724x5706 | pin (26292,2004) | AMB | digest/id/ledger/count pin | lattice corpus discard pin | pinned unjudged, open |
| 143 | `main-red-the-lofted-cylinders-far-union-builds-the-wrong-volume-at-1e-6` | 2026-10-28→2026-11-03 | ? | 6429 | TEST | non-default-eps behaviour | spline_spiric_operands far union @1e-6 | 'build right, reading wrong'; quadrature midpoint |
| 144 | `main-red-the-loop-keys-census-flags-a-splice-row` | 2026-11-03→open | 6435x6460 | - | DUP | census/roster | live loop_keys census | duplicate of #57 |
| 145 | `main-red-the-m10-p-fence-interval-digest-moved` | 2026-11-03→2026-11-19 | - | - | DUP | digest/id/ledger/count pin | m10_p_fence | duplicate of #59 |
| 146 | `main-red-the-m7-8-plain-names-probe-fails-after-the-faceview-fix-pass` | 2026-11-04→2026-11-05 | 6498 | 6535 | TEST | other slow behaviour | cert_m3r1_probes m7_8 plain names | legitimate move; re-pinned |
| 147 | `main-red-the-meeting-axes-pose-refuses-on-a-ruled-region-span-since-5664` | 2026-10-28→2026-11-20 | 5664 | 6482/6946 (src) | REAL | other slow behaviour | verbs_germarms2 meeting-axes pose | refusal since 5664 cured by kernel work |
| 148 | `main-red-the-needle-refuses-its-meet-at-1e-12-after-meet-pose-order` | 2026-11-04→2026-11-04 | 6492 | 6525 (src curve_pair/pose) | REAL | non-default-eps behaviour | band_turn_overrun needle @1e-12 | meeting reading could not settle a count; fixed |
| 149 | `main-red-the-perf12-at-rest-census-golden-moved-with-the-backstop-abort` | 2026-11-11→open | 6757 | 6956 (pinned unjudged) | AMB | digest/id/ledger/count pin | perf12_census_goldens line 75 | judgement owed |
| 150 | `main-red-the-plates-sym-forms-ledger-moved-at-1e-12-since-the-ellipse-speed-magnitudes` | 2026-11-21→2026-11-22 | 7183/7193 | 7389 | TEST | digest/id/ledger/count pin | m10_sym_profile PLATE_LEDGER @1e-12 | same forms, other order |
| 151 | `main-red-the-plates-sym-walk-ledger-reads-12-more-forms` | 2026-11-02→2026-11-09 | 6568,6594,6588,6628,6639,6631 | 6663 | TEST | digest/id/ledger/count pin | m10_sym_profile plate ledger | re-pinned; 2 more moves landed while PR open |
| 152 | `main-red-the-plates-sym-walk-ledger-reads-2-to-8-more-forms` | 2026-11-21→open | 7183 | 7389 | DUP | digest/id/ledger/count pin | plate ledger | duplicate of #150 |
| 153 | `main-red-the-probe-station-counts-double-and-the-gather-pays-one-more` | 2026-11-09→2026-11-14 | - | - | DUP | digest/id/ledger/count pin | probe station counts | healed + #140 |
| 154 | `main-red-the-profile-raw-door-census-names-two-lines-of-mesh-trimmed-rs` | 2026-11-14→2026-11-15 | 6950 | GREEN (census fix) | TEST | census/roster | profile raw_door_census | census misread pub(super) mod tests |
| 155 | `main-red-the-prose-census-reads-a-debug-rendering-in-selects-message` | 2026-11-11→2026-11-12 | 6555 | - | DUP | census/roster | prose_census | duplicate of #102 |
| 156 | `main-red-the-pseudomanifold-gate-door-is-off-the-tier1-mutation-roster` | 2026-10-26→2026-10-27 | 5741 | 5824 | TEST | census/roster | topo tier-1 mutation roster | ALLOWED entry |
| 157 | `main-red-the-quotient-statement-widens-two-rational-wall-volume-pads-past-their-pins` | 2026-10-22→2026-10-23 | b0962d6af1 | 9c390fac00 | TEST | digest/id/ledger/count pin | cert5 blade-8 / m8_3 volume pad pins | pad pins re-pinned |
| 158 | `main-red-the-ringless-cone-face-probe-builds-at-1e-12-where-it-pins-a-refusal` | 2026-11-20→2026-11-22 | 7184 | 7198 (drop pins) | TEST | non-default-eps behaviour | mesh reach_probes ringless cone @1e-12 | builds now (improvement) |
| 159 | `main-red-the-rook-story-transform-is-not-offered` | 2026-11-14→2026-11-14 | 6856 | 6947 (test helper + 6 id pins) | TEST | other slow behaviour | viewer rook story | test helper answered wrong name ask; id pins re-pinned |
| 160 | `main-red-the-rounded-pad-reads-24-more-symbolic-decisions-since-planar-foot` | 2026-11-09→2026-11-10 | 6594,6682 | 6714 | TEST | digest/id/ledger/count pin | m10_9 rounded pad symbolic 1660->1704 | 'real and correct' |
| 161 | `main-red-the-seam-region-door-row-runs-15x-slower-since-the-cylinder-sphere-chord` | 2026-10-29→2026-10-30 | 5894 | sect/seam-region-speed (src) | REAL | other slow behaviour | seam-region door row timeout | 8 s -> 127 s perf regression; recomputation removed |
| 162 | `main-red-the-seat-and-name-table-goldens-moved-at-dprime2` | 2026-10-25→2026-10-26 | 5613 | 5636 | TEST | digest/id/ledger/count pin | seat4/7/8, m4_pr3 name-table digests | re-pinned; body vs name move not separated |
| 163 | `main-red-the-sense-fold-census-flags-knot-split-a` | 2026-11-18→2026-11-19 | 87801ff8 | 7133 | TEST | census/roster | face_normal fold census | test helper area routed through door |
| 164 | `main-red-the-side-readers-aims-take-a-circle-foot-since-point-walk-floors` | 2026-11-09→2026-11-10 | 6645 | 6666 (src, planned refactor) | CONF | digest/id/ledger/count pin | sweep side_reader_aims_ask_nothing (probe) | aims took a circle foot; read reshaped |
| 165 | `main-red-the-stated-wrong-general-rows-refuse-chart-image-unavailable-in-the-slow-set` | 2026-11-11→2026-11-11 | 6815 | 6833 | TEST | other slow behaviour | stated-wrong-general rows | 'nothing in src changed'; rows re-written |
| 166 | `main-red-the-teapot-north-star-row-still-expects-the-handle-join-to-refuse` | 2026-11-11→open | f1f3a9a8dc | open | TEST | other slow behaviour | python north-star teapot handle | union now builds (improvement) |
| 167 | `main-red-the-teapot-north-star-test-expects-the-old-spline-face-refusal` | 2026-10-26→2026-10-26 | 5768? | INTENT re-pin | TEST | other slow behaviour | python north-star teapot | re-pinned by refusal kind |
| 168 | `main-red-the-teapot-spout-join-refuses-at-the-point-split-since-splice-5768` | 2026-10-26→open | 5768 | open (stale) | AMB | other slow behaviour | python teapot spout | superseded by #171 |
| 169 | `main-red-the-teapot-spout-join-refuses-join-on-the-hosted-runner` | 2026-11-12→open | 6872? | open | AMB | other slow behaviour | python teapot spout (hosted only) | machine-dependent threshold |
| 170 | `main-red-the-teapots-handle-join-builds-where-the-north-star-pins-a-refusal` | 2026-11-11→open | 6808 | - | DUP | other slow behaviour | python north-star handle | duplicate of #166 |
| 171 | `main-red-the-teapots-spout-join-refuses-in-the-join-since-6862` | 2026-11-12→2026-11-12 | 6862 | 6879 | TEST | other slow behaviour | tour wall3 + north-star spout refusal | re-pinned per eps |
| 172 | `main-red-the-tilted-rods-other-spins-row-asks-a-periodic-view-past-its-window` | 2026-11-17→open | 7052 | open | REAL-P | other slow behaviour | germ_tilted_rod other spins | PeriodicView asked past window; defect named |
| 173 | `main-red-the-torus-charge-family-row-draws-an-ellipse-refused-at-1e-6` | 2026-11-10→open | 6761 | open | TEST | non-default-eps behaviour | circle_lane_rows torus charge @1e-6 | test draw premise wrong at 1e-6 (unreachable!) |
| 174 | `main-red-the-torus-charge-row-draws-an-unbuildable-ellipse-at-1e-6` | 2026-11-10→open | 6761 | - | DUP | non-default-eps behaviour | circle_lane_rows | duplicate of #173 |
| 175 | `main-red-the-tours-probe-build-destructures-a-circles-private-radius` | 2026-11-21→2026-11-22 | - | - | - | excluded | build | excluded: probe build break |
| 176 | `main-red-the-wrap-strip-folds-its-period-by-the-sense-bit` | 2026-11-11→2026-11-14 | ? | 6930 | TEST | census/roster | face_normal fold census | sanctioned (chart u period) |
| 177 | `main-red-thin-nested-rings-build-meshes-the-rings-its-pin-expects-refused` | 2026-10-26→2026-10-28 | 5717 | 5911 | TEST | other slow behaviour | tess thin_nested_rings_build | meshes right; pin expected refusal |
| 178 | `main-red-three-curated-member-names-are-unbound-in-python` | 2026-11-01→2026-11-01 | 6208 | clear/binding-census-red | TEST | census/roster | pncad-py binding census | names added to BOUND_AS/NOT_BOUND |
| 179 | `main-red-three-cylinder-pair-volume-pads-pass-the-fixed-budget-at-1e-6` | 2026-11-14→open | ? | 6934 (pin), open | AMB | non-default-eps behaviour | three cylinder pair pad budget @1e-6 | budget vs eps-scaled pad (cf. #187 test-only) |
| 180 | `main-red-three-editor-core-rows-red-at-1e-12-since-before-11-19` | 2026-11-21→2026-11-22 | 7114 | 7389 | TEST | non-default-eps behaviour | e2e slider + revalue apart floors @1e-12 | floors per eps |
| 181 | `main-red-three-m10-interval-pins-moved-at-5728` | 2026-10-28→open | 5728 | open | AMB | digest/id/ledger/count pin | m10 interval pins | pins moved, re-pin owed |
| 182 | `main-red-three-source-censuses-red-since-the-told-quantity-type` | 2026-11-21→open | 7159 | 7267 (open) | TEST | census/roster | hand_written_impl_census x2, prose_census | lists follow new file |
| 183 | `main-red-tilted-cut-upper-moves-in-the-thread-count-digest-at-default-eps` | 2026-10-27→open | ? | 6956 | DUP | digest/id/ledger/count pin | thread-count tilted_cut_upper | same as #138 |
| 184 | `main-red-trace-rs-reads-as-a-planar-sense-flip-re-fork` | 2026-10-30→2026-10-30 | ? | PR (new door fn) | CONF | census/roster | face_normal planar-flip census | routed through new door |
| 185 | `main-red-trimmed-and-injective-fold-a-face-sense-by-hand` | 2026-10-29→2026-10-29 | - | - | DUP | census/roster | face_normal | duplicate of #56/#186 |
| 186 | `main-red-trimmed-rs-folds-the-face-sense-by-hand` | 2026-10-29→2026-11-14 | ? | 6930 | CONF | census/roster | face_normal fold census | routed through implicit_outward_normal |
| 187 | `main-red-two-cylinder-sphere-volume-pads-pass-their-ceiling-at-1e-6-since-the-sphere-lane` | 2026-11-11→2026-11-11 | 6509 | 6840 | TEST | non-default-eps behaviour | cylinder x sphere pad ceilings @1e-6 | 'ceiling was the wrong side' |
| 188 | `main-red-two-door-at-rest-rows-on-the-2026-11-11-nightly` | 2026-11-12→open | 6710 | 6931 | TEST | digest/id/ledger/count pin | viewer door_at_rest DIE_UNPROVEN | hand-kept copy of a count |
| 189 | `main-red-two-dual-interval-corpus-rows-overflow-the-stack` | 2026-11-10→2026-11-10 | 6735? | 6785 (src cover.rs) | REAL | other slow behaviour | Dual<Interval> corpus rows stack overflow | deep Cover chain overflowed stack |
| 190 | `main-red-two-injectivity-rows-fail-at-eps-1e-12` | 2026-10-31→2026-10-31 | 6138 healed / tee | - | AMB | non-default-eps behaviour | two injectivity rows @1e-12 | helix healed by 6138; tee re-pinned to typed refusal |
| 191 | `main-red-two-offer-rows-quote-a-positive-margin-against-a-negative-case` | 2026-11-10→2026-11-14 | 6687 | 6931 (pin) -> ONECOPY | AMB | other slow behaviour | offer_rows margin sign | which end changes is open |
| 192 | `main-red-two-offered-tolerances-fail-just-below-their-margin` | 2026-11-10→2026-11-14 | 6687 | - | DUP | other slow behaviour | offer_rows | duplicate of #191 |
| 193 | `main-red-two-review-4781-rows-refuse-at-limb-2s-kernel-limit-since-5728` | 2026-10-28→open | 5728 | open | AMB | non-default-eps behaviour | review_4781 r1/r2 @1e-12 | limb-2 kernel limit; pins follow fixtures |
| 194 | `main-red-two-slow-digest-rows-at-8ae2b9b` | 2026-11-03→2026-11-14 | - | - | DUP | digest/id/ledger/count pin | seat7/m10_sym | covered by #100/#151 |
| 195 | `main-red-two-sweep-rows-at-sym-and-the-certified-scalar-since-limb-2` | 2026-10-27→2026-10-28 | 5728 | 5893 | TEST | other slow behaviour | sym11 far placement TABLE cell | honest move, re-baselined |
| 196 | `main-red-two-tori-subtract-refuses-on-tangent-carriers-not-on-no-section-classification` | 2026-10-28→2026-10-28 | 5869 | 5925 (src section_cert) | REAL | other slow behaviour | germ_torus_doors two tori | exact tie in own construction; kernel fixed |
| 197 | `main-red-validate-pseudomanifold-is-an-undeclared-public-mutation-path` | 2026-10-27→2026-10-30 | 5741 | 5824 | DUP | census/roster | tier-1 roster | duplicate of #156 |
| 198 | `main-reds-without-an-item-met-by-pcert-e1b` | 2026-10-27→2026-11-14 | - | - | - | excluded | meta | excluded: aggregate |
| 199 | `the-boolean-corpus-name-digests-are-red-on-main` | 2026-11-12→2026-11-13 | 6832 | 6910 | TEST | digest/id/ledger/count pin | boolean corpus name digests | key-only move |
| 200 | `the-cone-fuzzs-one-material-check-steps-past-a-sliver-piece` | 2026-11-16→2026-11-16 | - | - | - | excluded | fuzz | excluded |
| 201 | `the-plain-sphere-witness-escalates-its-pcurve-envelope-at-1e-12-since-the-rigid-motion` | 2026-11-14→open | 6988 | GREEN 11-17 pin text | TEST | non-default-eps behaviour | spheres_crossing plain witness @1e-12 | same refusal, different check; pin text follows |

## Counts by family (distinct incidents; DUP and excluded rows not counted)

| family | n | REAL | REAL-P | TEST | CONF | AMB | median days open (closed rows) |
|---|---|---|---|---|---|---|---|
| census/roster | 51 | 4 | 0 | 39 | 7 | 1 | TEST 1, CONF 0, REAL 0 |
| digest/id/ledger/count pin | 30 | 3 | 1 | 20 | 2 | 4 | TEST 1, REAL 5 |
| non-default-eps behaviour | 40 | 6 | 3 | 9 | 0 | 22 | TEST 1, REAL 0, AMB 8.5 (16 AMB still open) |
| other slow behaviour | 37 | 11 | 5 | 16 | 0 | 5 | TEST 1, REAL 1 |

Reading the counts:
- **Censuses/rosters**: 4 REAL in 51 (8%), and all 4 are latent soundness reads. 37 (#34) made `four::along_side` ask `certified_bracket` before moving a refusal to `AlongSide`. 37 also fixed 7 unasked endpoint reads. 132 made `node_frame` ask before gating `NODE_BOX`. 136 fixed one unsafe read in `deciding_walk`. None was a wrong answer anyone had observed. 46 of 51 were list or count updates, or code reshaped to satisfy the census.
- **Digest/id/ledger pins**: 3 REAL in 30. #12 shows the rim certified twice: the rim Zero count went from 10 to 18. In #140, a k-probe sample count of 2409 against 2408 exposed tier 3′ skipping the session-box gate that tier 3 runs. In #110, the 5542 receipt moves came with two `segment_gap` defects, fixed by PR 5952. #3 is a count-only loss (rule H) that was filed on SYM. 4 more are pinned unjudged: seat4 `die_fillet`, `corner_table`, the perf12 line-75 golden and the lattice discard. Everything else was re-blessed as an intended move. Two kinds of rebless were pure noise: key-only digest moves (PRs 6832 and 6856, each of which moved every id-keyed digest at once) and one line reflow.
- **ε rows (1e-6/1e-12)**: 6 REAL + 3 REAL-P in 40. Every REAL was a real kernel fix: #2, #25, #41, #51, #130 and #148. #53 is a P0: four 1e-6 volumes fall outside their own enclosures. But 22 are AMB, mostly pins at 1e-12 on kernel-limit refusals that still wait for the owner: VolumeUncomputable or escalation in band. Of the 9 TEST rows, 4 had the test wrong about ε: a fixed `1e-7` pad budget against an ε-scaled pad (#187), a fixture gap that does not scale (#28), a draw premise (#173), and a midpoint read at a coarse resolution (#143).
- **Other slow behaviour rows**: the best ratio, 11 REAL + 5 REAL-P in 37 (43%). Examples: an Approx operand building a body (#13), flush copies refused (#39), a needle channel (#17), a tilted prism (#19), two tori (#196), RingHomingAmbiguous (#117), a stack overflow (#189), a 15× slowdown (#161), and the revalue axis mint (#35). The 16 TEST rows are mostly pinned refusals that moved one door later on purpose (#45: 18 rows, after Ev's crease ruling) or that started building (#4, #166, #177).

## Tests re-baselined repeatedly in the month that caught nothing real

Counts are incidents in this population, plus re-pins the green log records outside a row (marked +log).

| test (file::fn) | reds | REAL | what each fix was |
|---|---|---|---|
| `crates/topo/src/face_normal.rs` `tests::no_source_file_folds_the_bit_by_hand` (+ the planar-flip row) | 8 (#44 #56 #106 #163 #176 #184 #185 #186) | 0 | 5 reshaped to call `OutwardNormal::from_chart` (bit-identical), 2 added to the sanctioned list, 1 test helper |
| `crates/pncad-py/src/prose_census.rs` (3 rows: undecided list, brace payload through Debug, Debug raise) | 11 (#71–76 #83 #102 #120 #155 #182) | 0 | `UNDECIDED` lines added or struck, the census taught a new renderer shape, 3 Display arms reworded to the same text |
| `crates/editor-core/tests/m10_sym_profile_interval.rs` plate/slab walk ledger (`PLATE_LEDGER`) | 6 (#60 #151 #150/#152 #194 #110 #3) +log | 0 | re-pinned. Main moved it 2–3 times while one PR was open (6663, 6560). One move was build order only (7193) |
| `crates/sweep/tests/…mass_props_are_thread_count_invariant` (serial golden, 1 and 4 threads) | 5 (#6 #183/#138 #35 #3) +log 11-15, 11-17 | 0 | re-cut: ulp-level volume/area/pad moves, `frozen` counts 14→16 |
| fixture-digest goldens: `seat4_verb_lowering`, `seat7_sweep_lowering` (`the_sweep_documents_evaluate_to_their_committed_digests`), `seat8_split_lowering`, `m4_pr3_names_ci`, boolean corpus name tables | 5 (#162 #100 #194 #199 #137) +log (6856 "moved every id-keyed digest again") | 0 | key-only or mint-id moves. 2 pinned unjudged. One red was a second copy of a pin another PR had already re-blessed (#100) |
| `sym11_exact_channel_rows`, `sym_9_retry_interval`, `m10_9_pins_interval` (symbolic/numeric/frozen counts) | 6 (#105 #113 #160 #181 #110 #3) +log (11-15, 6963) | 0 (+1 count loss filed) | every move but one was "real and correct" |
| `topo/tests/recourse_roster.rs`, `geom-brep/tests/recourse_roster.rs`, `test-utils/tests/predicate_name_coverage.rs` | 6 (#84 #85 #86 #104 #69 #70) | 0 | roster lines or a `KNOWN_MISSING` entry added. One test literal respelled |
| hand-kept file lists: `reader_census`, `node_standing` `READERS`, binding census `BOUND_AS`/`NOT_BOUND`, tier-1 mutation roster `ALLOWED`, box-door inventory `PINNED`, `raw_door_census`, `deny_unknown_fields`/`hand_written_impl_census`, `flagged_census`, `live` `loop_keys` | 13 (#81 #82 #62 #178 #156/#197 #119 #154 #55 #182 #52 #57/#144) | 0 | add or remove a path. `raw_door_census` misread `pub(super) mod tests` |
| `sym_rule_f_rows` copysign census + `sym/manifest.rs` | 2 (#126 #127) +log 11-17 | 0 | f64-only sites registered. "None is a defect" |
| `perf12_census_goldens` + `m10_p_fence::the_corpus_evaluation_is_bit_identical_at_interval` | 4 (#67/#68 #149 #59/#145) | 0 | re-blessed (the lost finding was spurious) or pinned unjudged |
| refusal-text and refusal-shape pins: `refusal_concision_chains` 75-word cap, `m5_pr12_die_body`/`m6_surgery` text, `germ_tilted_rod` Debug shape, python north-star teapot, tour wall 3 | 7 (#32/#118 #111 #166/#170 #167 #171 #124) | 0 | wording or shape followed |
| fixed pad/volume budgets at 1e-6: `LOFTED_CONCAVE_PAD_AT_1E6`, cert5/m8_3 pads, cyl×sph and cyl×cyl `1e-7` ceilings | 4 (#5 #157 #187 #179) | 0 | re-pinned. #187: "the ceiling was the wrong side" |

**High-churn but did catch something.** `crates/geom-core/tests/certified_endpoint_census.rs` `every_production_endpoint_read_is_counted_and_dispositioned` went red **15 times** (#36–38 #46–49 #66 #77 #121 #132–136). 12 were re-pins or open re-pins. One was a pure line reflow (#121: "counts lines, not reads"; the fix row says a read-count would move 42 of its 51 pins). 3 found unasked reads that were then fixed (#37 #132 #136). Its value lies in what it asks; its cost lies in pinning exact per-file counts. Counting only unasked, undispositioned reads would have kept the 3 and dropped most of the 12.

**Tests that caught the real bugs** (mostly slow behaviour rows):
- **sweep and topo slow behaviour rows at the default ε**: `m5_s11` and `operand_gate_pose` (Approx built a body), `coincidence_door` axis edit, `germ_torus_doors`, `three_solids_on_one_line`, the needle cavity row, `emit`/`dm1` arc rim, `verbs_germarms2` meeting axes, `revalue_corpus` ×5, the seam-region timeout, the `Dual<Interval>` stack overflow, the coincident-wedges rows.
- **ε rows at 1e-6/1e-12**: `node_end_rows`, the `pair_section`/`cylinder_sphere_frame` near-node rows, the torus section rows, the cyl×sph ruling sweep, the band-turn needle, the sheared fillet witness, and the ONEDOOR P0 volume rows.
- **one count, one census**: the k-probe gather sample count (#140) and the bounds roster's `along_side` (#34).

Caveats:
- AMB is large among the ε rows (22) because those rows are still pinned at a kernel-limit refusal with no owner ruling. They are neither shown real nor shown to be tax.
- "REAL-P" relies on the owner's or reviewer's judgement; 5 of the 8 are still open.
- The breaking and fixing PR numbers come from the rows' own bisects. Where a row did not bisect, the column says `?`.
