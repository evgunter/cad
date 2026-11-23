# Delta read of PR 7225 (`oneround/box-region`), reviewer r1

**Frozen head:** `1e81f303`. **Delta:** `5c3bb9b9..1e81f303`. **Main as merged:** `a3d5ce89` (the head's second parent). Hosted CI run: 45072515147.

**Verdict: NOT-MERGEABLE-AS-IS.**

The code passes every check I put to it:
- the mutants I planted on it go red, all but two harmless ones (F4);
- the near-axial torus is fixed (F5);
- the deferral is gone (F3);
- the three red CI rows are main's, reproduced on `a3d5ce89` with identical output.

What blocks the merge is the report-back to Ev (F1). It states "the class of valid bodies that now refuse is this", and that class is incomplete.

I built a valid in-box body outside it: a torus cut by a bar. It is refused as a limit 250 m and 300 m past the box, because its two ring-torus faces read their whole tube. The PR files no row for that case: it says "No row is filed for a torus face with no window … in the sample no ring torus face's cover was the cause".

The fix is text and a row, not code. The PR body's report-back is the deliverable Ev asked for before the merge, so it must be true first.

## What I executed and what I only read

**Executed** at `1e81f303`, in my own target directory, debug profile:

1. **The unit's rows, at ε = 1e-9, 1e-6 and 1e-12.** 205 tests per ε (judged against the head's own test list):
   - every `session_box` row in topo, sweep, editor-core and step-import;
   - the seamless-frustum row and `section_cert_cone_rows`;
   - all `tcost_k3_certificate` rows;
   - the three `m10_sym_profile_interval` ledger rows;
   - `geometric_cube`, `review_m2_pr3`, `review_m3_pr4`, `interval_body`, `verbs_cylsph_opening`, `nurbs_faces_refuse_typed_at_both_doors`, `tier3_tests`, `intent_e_operation_frame_of_reads`, `finish::tests` and the recourse roster.

   At each ε, 204 pass. One fails, `recourse_roster::every_decided_name_has_its_door_and_every_site_is_read`, and it is main's (below).
2. **Eleven mutants** (`analysis/oneround/box-region/r1-delta-mutants.py`), each run over 199 session-box and re-pinned rows. All 199 pass on the unmutated head.
3. **Probes** (`analysis/oneround/box-region/r1-delta-probes.rs.txt`, planted in `crates/sweep/tests` and not pushed to the PR):
   - my earlier analytic-reach oracle (240 posed tori and balls);
   - partial-revolve tori and a lofted cylinder at the box's wall;
   - the near-axial torus and the spindle football;
   - the bar-cut torus.
4. **A tier-3 probe** of what the gate says for twelve degenerate surfaces (`r1-delta-uncovered-texts-probe.rs.txt`; planted in `tier3_tests.rs`, then reverted).
5. **The three inherited reds on main** `a3d5ce89`, square-search seed `0xd9567b84aab9b1c0`.
6. **CI run 45072515147, read at step level**, including the step logs and their test lists.

**Only read:**
- the merge resolutions, as hunks against `a3d5ce89`;
- the PR body, the eight new or changed work rows, the cover and fold code, the `uncovered_face` texts and the re-pinned integration rows.

**Not done:**
- the 756-placement differential behind F1's counts: it is the other reviewer's probe, which I may not read;
- the fast-set total of 17 643 passes, the gates and clippy;
- F10's timings.

**Isolation.** The frozen head itself carries summaries of the other reviewer's findings: the PR body, the brief, and work rows "Found by the r2 review". I read those as part of the head. I did not fetch or read the other reviewer's branch.

## Findings

### MAJOR 1: the report-back's class of valid in-box bodies that newly refuse is not complete (F1)
DEMONSTRATED, `sure`.

**The body** (probe `r1d_a_cut_torus_patch_near_the_wall`): the PR's own in-box cut, `topo::intersect` of a torus (R 700, r 50, about `y` at `(0, 300, 0)`) and the bar `[100, 400] × [200, 400] × [400, 800]`. It is admitted at the origin. Its cut body lies inside the bar.

| moved by | where the body stands | verdict, at the gate and at the door |
|---|---|---|
| +500 m in `x` | x ≤ 900 | `SessionBoxUnshown(250)` |
| −550 m in `x` | x ≥ −450 | `SessionBoxUnshown(300)` |

**Which face.** Read per face (`topo::face_cover`):
- the two torus faces cover `x, z ∈ [−750, 750]`: the whole tube, so no window is proven;
- the two plane faces and all six spiric edges cover tightly, within the bar.

So a ring-torus face a boolean cuts reads its whole tube: an overshoot up to the tube's diameter, as for the sphere.

**What the PR claims.** Its class names only a sphere, spindle or spline face. It files no torus row. Its evidence for leaving the torus out is one sample of 14 revolved profiles, where the only ring-torus face places its candidates Out.

**What I did not find.** My partial-revolve tori (quarter, half and three-quarter turns, R 300, r 100, the absent tube crossing the box) read their windows: no wrong verdict (`r1d_a_partial_torus_reads_its_window`).

So the missing class is torus pieces whose pcurves the window walk does not read, which boolean results produce.

**Possibly incomplete in other ways** (by reading, `unsure`):
- a cone face whose lift or nappe is undecided keeps a far apex in its hull;
- a ruled face bounded by an edge whose own cover is loose.

My lofted-cylinder probe showed no case of the second.

**F1's counts** (289 / 17 / 85 in the box; 329 / 36 past it; 82 of 365 at the base) I could not reproduce, because they come from the other reviewer's probe.

### MINOR 1: F3 gives a person less, and in one case the wrong recourse, for faults that are not placeholders
DEMONSTRATED by probe, `sure`.

**What each fault now reads:**

| fault | check 1 said, before | the box says now |
|---|---|---|
| **a poisoned described net**, in a file or a corrupt import (`interval_body`'s fixture) | "a face's surface description holds invalid numbers, so it describes no shape", with the defect ending | "… has a control net that bounds no region: its surface may not be described yet. Recourse: describe the surface first, then ask again" |
| **a NaN sphere radius** (probe) | "stores a radius that is not a finite number" (it named the datum), plus the edge-description findings | one finding, naming no datum: "a datum of a face's sphere surface is not a finite number, so it describes no locus, so the kernel cannot show the body inside…" (the "so … so" is in the text) |
| **a NaN vertex** (the row's brick) | (not shown) | "a face's plane surface is unbounded and its boundary is unread: an end vertex of the edge is not a finite point". This blames the face's surface for a vertex fault. |

The poisoned net's surface *is* described, so the new recourse is wrong for it.

**What was lost, across the re-pinned rows:**
- every uncovered face named, not just the first: `boolean::ops::nurbs_faces_refuse_typed_at_both_doors` now asserts "one of its faces named, alone";
- the four `DescriptionNotAdjacent` edges (`review_m3_pr4`);
- the distinction "a placeholder's verdict is a different state's answer", which `interval_body` asserted;
- check 1's described-poison arm at `Interval`, which no end-to-end row now covers.

**What is filed.** The filed P2 row (`a-face-covers-refusal-names-no-placeholder-net`) covers only the placeholder-versus-poison split and which datum. Two lost subjects are unfiled: naming every face, and the `Interval` arm.

**Should the refusal name every uncovered face?** Yes. Check 1 did. The fold stops at the first face (`faces_reach`), so a file with several bad faces is now fixed one face per round trip.

### MINOR 2: the delta mints two small fresh instances of what it fixes
BY-INSPECTION, `sure`.

- **A second statement of the face rule** (F7). The cone arm of `face_cover_in` writes the boundary's convex hull itself, `and_trim(surface.cover()…, boundary.map(Cover::convex))`, beside `face_region`'s plane arm. Its "a hull of fewer of the base's points" is an argument, not a construction.
  - The cylinder arm keeps its own `is_direction` guard after `face_region` gained one. The merge note calls it "redundant but kept to match main's cover arm".
  - The `Plane | Cylinder | Cone if !bounded => base()` arm repeats `face_cover_unproven`'s own lone-vertex return.
  - So "the rule is stated once, by construction" holds for the region's start, not for the cone arm.
- **A row that pins a refusal where it used to pin a subject, the subject unfiled** (MINOR 1): `interval_body` and `nurbs_faces_refuse_typed_at_both_doors`.

No refusal defers behind a check any longer.

### NOTE 1: the mutants, re-planted on the head (F4)
DEMONSTRATED. Each red row is judged against the 199 rows that pass unmutated.

| mutant | red rows |
|---|---|
| stage 2 never admits | `a_zone_whose_pole_alone…`, `a_seamless_frustum_band…` |
| placement filter keeps every candidate | `a_ball_wedge…`, `a_zone_whose_pole_alone…` |
| stage 3 reads no edge points | `a_torus_tilted…`, `an_arc_whose_middle_alone…` |
| an uncovered face skipped in the fold | 6 rows, including `a_face_with_no_cover…` and the five re-pinned integration rows |
| a Zero read as past | `a_body_at_the_boxs_edge_is_in_it` |
| the cone's apex dropped from `face_region` | `a_cone_cap_whose_apex_alone…` |
| a single winding loop read as not holding the apex | `a_cone_cap_whose_apex_alone…` |
| the old `e − a(a·e)` spelling | `a_torus_tilted…` |
| a placed carrier left unmapped | 8 rows |
| two winding loops, the nappe not read | **survives** |
| the cylinder zero-axis guard dropped in `face_region` | **survives** |

**Why the two survivors are harmless** (by reading):
- **The nappe guard.** Two loops that each wind the axis, on opposite nappes, each meet the axis inside their own hull, one above the apex and one below. So the apex lies in the boundary's hull anyway, and the nappe guard does not change the cover. It is safe and has no row.
- **The cylinder guard.** It only chooses which refusal a zero-axis cylinder gets (the box's uncovered refusal or check 1's). No row pins it.

### NOTE 2: F5, F6 and the oracle re-run on the head
DEMONSTRATED, `sure`.

- **Near-axial torus.** The θ sweep now refuses past the box at every θ from 1e-3 down to 1e-9. At 1e-12 the overshoot is 1.5e-10 m, within the band, and the body is admitted.
- **Spindle football, 30 m past.** It now reads the new sentence, which is true of it: "the body may lie inside the box or past it".
- **The oracle (240 poses).** No wrong admission, no in-box limit and no door-versus-gate disagreement.

### NOTE 3: F2's two-winding rule is sound as built
BY-INSPECTION, `likely`.

A bounded face whose only winding loops are two on one nappe is the band between them. A face that holds its apex keeps one winding loop and so still reads "may hold" (the cap row, red under my mutant). The nappe test is redundant for the hull (NOTE 1).

### NOTE 4: the re-pinned ledger and hosted CI (F9 and the merge)
DEMONSTRATED.

**The ledger.** I ran `the_forms_the_plates_walks_build_are_pinned`, `the_plains_ledger_lines_are_the_same_under_every_dial_set` and the slab row at 1e-9, 1e-6 and 1e-12: all green.

Against main's pin, the diff is `Plain/Assertion`, `Early/Assertion` and `Door/Decision`, each +12 forms, each with a new digest. Calls and frozen counts are unchanged, and `Plain/Decision` and `Early/Decision` are untouched. So "+12 forms, no call moved" is the whole change, the three digests aside.

**Hosted CI, step "slow-set tests this diff adds or edits":**
- **Kernel job:** 53 rows, including the four `tcost_k3` rows and the four `verbs_cylsph_opening` rows.
- **App job:** 7 rows, including both plate-ledger rows and the intent-E corpus row.

All of those ran at the default ε only. Their 1e-6 and 1e-12 runs exist only locally (mine, above). The interval job was skipped.

### NOTE 5: the inherited reds are main's
DEMONSTRATED.

On `a3d5ce89` all three fail with the CI step's own output:
- **offer rows:** the same false offers;
- **square search:** three mismatches at the same seed;
- **recourse roster:** exactly `Const(NEST)`, `Const(REACH)`, `chain_parity.rs` and `spline_trim.rs`.

On the head, the roster prints the same four lines, so nothing of the unit's is among them.

### NOTE 6: stale text
`sure`.

- `validate.rs`'s doc on `SESSION_BOX` still calls it "`FindingFamily::SessionBox`'s one recourse", and `recourse()` is now `None`.
- The PR body cites `work/boxfinish/a-finished-spindle-body-turned-rigidly-stops-being-finished`. The file is `…-turned-off-its-axis-stops-being-finished.md`, and the brief repeats the PR body's name.

### NOTE 7: the ball-wedge move keeps its subject
BY-INSPECTION, `sure`.

At radius 30, with its axis at x = 990, the wedge still stands in the box with its ball 20 m past. It still reds under the placement-filter mutant.

## Style

**Questions exercised:**
- Q1: MINOR 2, the cone arm's own hull and the duplicated guards.
- Q3: NOTE 1, two surviving mutants.
- Q4: NOTE 6.
- Q5: the module doc and `uncovered_face`'s texts against what they say (MINOR 1).
- Q6: MINOR 1's unfiled lost subjects.
- The brief's fresh-instance question: MINOR 2.

**Ratings:**
- **Idiom and structure, 4/5.** One fold over faces; covers start from one region; the refusal is first and alone. The cone arm still builds its own hull.
- **Test quality, 4/5.** Every mutant that matters goes red; two harmless ones survive. The re-pinned integration rows lost subjects.
- **Doc and comment honesty, 3/5.** The report-back to Ev overstates its class (MAJOR 1); the rest is accurate.
