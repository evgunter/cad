# Delta read r2 of PR 7225, `oneround/box-region`, frozen at `1e81f303`

Read: the delta `5c3bb9b9..1e81f303` (the unit's own commits, and the
hand resolutions of the merges of main by `git show --remerge-diff`;
only `6af4f344` has any). Hosted CI: run 45072515147. Everything of mine
ran in a detached worktree of `1e81f303` with my own target dir, at
ε = 1e-9 unless a line says otherwise. I did not fetch or read the other
reviewer's branch.

## Verdict: APPROVE-WITH-FIXES

The fix pass did what was adjudicated, and the predicate still holds
under the differential. Three things are owed before the report-back
goes to Ev, none of them in the fold:

1. The report-back's class of valid bodies that newly refuse leaves out
   ring-torus faces. Its own fixture shows one (D1).
2. The two-winding-loop cone rule's one-nappe guard is pinned by no row
   (D2). It is the unsound direction F2 asks about.
3. A corrupt described net is now told to "describe the surface first"
   (D3).

## Findings

### D1 (F1). MINOR. The class in the report-back omits ring-torus faces; "no ring torus face's cover was the cause" is true of the sample and not of the kernel. DEMONSTRATED, sure

The PR body: "a body with a sphere, spindle or spline face refuses as a
limit wherever that face's whole carrier … reaches past the box", and
"No row is filed for 'a torus face with no window'".

The PR's own row fixture, `a_bar_cutting_a_large_torus_inside_the_box_builds`:
a ring torus `R = 700`, `r = 50` about `y` at `(0, 300, 0)` cut by the
bar `[100, 400] × [200, 400] × [400, 800]`. The piece is finished at
home. Moved 300 m along `x` it lies in `x ≤ 700`, `y ≤ 400`, `z ≤ 800`
(its vertices reach 743.3 m), and the gate says: "the kernel's bound on
the body's faces reaches 50 m past the 1e3 m session box … the body may
lie inside the box or past it". 50 m is the whole tube, `750 + 300`.
Moved 200 m, the tube at 950, it is admitted
(`probes/sweep_tests_zz_review_r2_probe.rs`,
`review_r2_delta_a_bar_cut_ring_torus_piece_near_the_wall`).

So a ring torus face that reads no window (here one bounded by spiric
edges; by `TorusChartWindow`'s guards also one with a ring, or a loop
that wraps a channel) is in the class, with the tube's diameter as its
overshoot, and has no row. I did not separate the torus face's cover
from the spiric edges' own (each is the tube met with a rectangle); the
second would put any face bounded by a spiric edge in the class too.

The counts are reproducible: my 756 placements on this head give 289
admitted, 17 refused by tier 3, 85 a limit inside the box; 329 past and
36 a limit past it. That is the PR body's table exactly, judged against
my own run at `5c3bb9b9` (the same numbers) and at the old merge base
(82 of 365 not refused by the box).

The rows filed are true of what they measured. Two small things: the
sphere row's "Measured" cites the radius-300 wedge and names the row,
which now holds the radius-30 one; and one "Why P2" paragraph ("refuses
a valid body that main admits") is pasted onto the two inside-read rows,
whose bodies are past the box.

### D2 (F2). MINOR. The one-nappe guard of the two-winding-loop rule can be removed and no row goes red. DEMONSTRATED on 313 rows, sure

`cone_may_hold_apex` (`cover.rs`) excludes the apex for exactly two
loops winding once only where `face_nappe_reading` decides one nappe. A
face with a ring on each nappe holds its apex; with the guard gone its
cover drops the apex and a body whose apex alone is past the box is
admitted. Mutant 18 (two winding loops never hold the apex, nappe
unasked) passes all 313 rows I ran: every session-box row, the cone
rows, the STEP band row, the re-pinned gate rows, `tcost_k3`. The whole
fast set was not run under it.

By reading, the rule as written is sound: on one nappe the chart is an
annulus about the apex, a connected region with two boundary curves that
each wind it is the band between them, every cycle has a corner, and the
nappe is read off the extreme corner stations of all of them. Likely,
not sure: I did not build the hourglass.

Mutants 19 (two winding loops always may hold it), 20 (one winding loop
never holds it) and 21 (the apex candidate dropped) are each red.

### D3 (F3). MINOR. What a person is no longer told, concretely. BY-INSPECTION, sure

Each sentence is true of its cause as far as the cover can name it. What
changed for the person at the gate:

- **A corrupt described net.** Before: "a face's surface description
  holds invalid numbers, so it describes no shape", a defect. Now: "a
  face's nurbs surface has a control net that bounds no region: its
  surface may not be described yet … Recourse: describe the surface
  first, then ask again". The sentence is hedged true; the recourse is
  wrong for a surface that is described. The PR body says the two read
  alike and files the split (P2); it does not say that one of the two
  now gets the other's recourse.
- **Which datum.** Before, check 1 named the field (a torus's
  `MinorRadius`). Now "a datum of a face's sphere surface is not a
  finite number". Disclosed, filed.
- **Every face after the first.** Disclosed.
- **Every other finding on the body.** `review_m3_pr4`'s four
  `DescriptionNotAdjacent` are gone from the report until the first
  uncovered face is cured. Disclosed.
- **The family.** A seed face mid-construction, at the origin, is now
  reported under "the body is past the session box, or not shown inside
  it", where it was a description finding. Not said in the PR body.
- For a sphere, torus, spindle or spline face,
  `UncertifiableSurface`, `PoisonedSurfaceDescription` and
  `PoisonedSurfaceDatum` can no longer leave the gate at all. They stay
  in the enum, the tags and the binding census.

Should it name every uncovered face: yes, I think. The refusal is alone
by ruling, so a body with six seed faces is six round trips, and
`faces_reach` already walks them; stopping at the first is a choice, not
a cost. Likely.

One more by reading, unsure and probably unreachable since main's frames
hold unit axes: geom's cylinder guard refuses a zero axis as
`Unknown::NonFinite`, which `uncovered_face` would say as "is not a
finite number".

### D4 (F3, the brief's last question). MINOR. A lost subject is unfiled. BY-INSPECTION, sure

`interval_body`'s row was renamed and now pins the box's refusal. Its
old subject, check 1's described-poison arm at `Interval` and that a
poisoned net does not draw the placeholder's verdict, has no end-to-end
row. The PR body says so; no row under `work/` does
(`a-face-covers-refusal-names-no-placeholder-net.md` is about the
cover's vocabulary and does not mention it).

### D5 (F10). MINOR. "Up to a tenth of the gate" is not what I measure against the structural gate. DEMONSTRATED, likely

On this head, debug, two runs: the ask where stage 1 does not admit is
748–833 µs (the wedge at the wall), 559–655 µs (the dome), 393–1040 µs
(the tapered ring, apex 10 km to 1000 km off). The structural gate on
the bodies I timed is 2.2–3.4 ms. That is a sixth to a third, judged
against `validate_geometric_structural`. The PR body does not say which
gate its tenth is of; against the certified gate it may hold. The 10 to
30 times the old ask stands (my base figures were 18–50 µs).

### D6 (F9). NOTE. Which re-pinned slow rows ran on hosted CI. DEMONSTRATED, sure

The kernel job's slow-set step ran 53 tests, the app job's 7 (60
selected "in a file the diff edits", 0 by name).
- Ran on hosted CI, green, at ε = 1e-9 only: all four
  `tcost_k3_certificate` rows, the two re-stood ones among them;
  `m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned`
  and `::the_plains_ledger_lines_are_the_same_under_every_dial_set`;
  `verbs_cylsph_opening` (4 rows).
- Ran nowhere but locally: the plate ledger at ε = 1e-6 and 1e-12 (the
  slow-set step has no ε loop). I ran the three ledger tests at all
  three ε: 3 of 3 pass at each.
- "+12 forms, no call moved" is the whole of the change against main's
  pin, by the merge's remerge-diff: three lines' form counts and
  digests, every call and frozen count as main's, the other two lines
  main's. Before the merge the same read was +2, +14, +14, +20 on four
  lines; the row's note records only the merged figure.
- The fast rows re-pinned for F3 ran in the default-ε step.
- Skipped whole again: `interval`, `tools (k-lint…)`, `demos`,
  `step import (freecad)`, `viewer`.

### D7. NOTE. The three reds are main's. DEMONSTRATED, sure

At `1e81f303` and at main `a3d5ce89` (the merged-in parent), locally,
all three fail with the messages hosted CI printed:
`offer_rows::every_offered_tolerance_passes_just_below_it` ("false
offers"), `square_search::the_square_wall_arm_agrees_with_the_traced_section`
(3 mismatches at seed `0xd9567b84aab9b1c0`), and `recourse_roster`
(`Const(NEST)`, `Const(REACH)`, `chain_parity.rs`, `spline_trim.rs`: the
same four lines on main, none naming this unit). Nothing of this unit's
is among the step's 3 failures of 53.

## The items

- **F1.** Counts reproduce. Class incomplete: D1.
- **F2.** The sentence is corrected. Slender rings at the origin are
  admitted (measured, apex to 1000 km). Soundness: D2.
- **F3.** The deferral is gone: `structural_via` and
  `pseudomanifold_certificate_via` ask the box first, and mutant 8 (the
  gate dropping the uncovered refusal) is red on 7 rows. Texts: D3, D4.
- **F4.** Re-planted. Of 19 mutants, 18 are red on the 313 rows; the
  three that survived the whole fast set before are among them (12 the
  placement filter: the wedge and the zone; 8; 6: the zone, the seamless
  band, and the kink row). The survivor is D2's.
- **F5.** `a × (e × a)` is in. The old spelling (mutant 17) is red on
  the θ row. That the 13 were spindles checks by arithmetic on my own
  profiles (the D bulge: centre at 193 m, radius 227 m).
- **F6.** The sentence is as claimed (read off the gate). True of a
  body inside and of one past.
- **F7.** Mostly. `face_cover_unproven` is the base of every arm and
  calls geom's `face_region`; one `faces_reach`, one `outer_bounds`; the
  header and the readers ledger are right and the gate is green against
  the new base. Two leftovers, Style 1.
- **F8.** Both rows ran their subject at 1e-9 on hosted CI and here.
- **F11, F12, F13.** The module doc says what is decided before the box
  admits. The rows exist and read true.
- **The wedge.** Moving it to radius 30 kept the subject: the ball's
  `+x` pole at 1020 m is still a candidate off the face, and mutant 12
  is red on it. What it dropped is the radius-300 wedge refusing to
  revolve at 1e-12, which the PR body calls a kernel limit and for which
  I found no row.
- **The merge.** The cover's arms on main's frames read as the pre-merge
  arms with `frame.origin()` and `frame.axis()` for the old fields; the
  differential and every row agree across it.

## Is anything a fresh instance of what it fixes

- A second statement of the face rule: a small one, Style 1.
- A refusal that defers behind a check: no. The reverse now holds, and
  is the ruling: check 1's naming waits behind the box.
- A row that pins a refusal where it pinned a subject, the subject
  unfiled: D4.

## Style

Exercised: Q1, Q3, Q4, Q5, Q6.

1. Q1, sure. `face_cover_in`'s cone arm still spells a ruled rule in
   topo: where the face does not hold its apex it builds
   `and_trim(surface.cover()…, boundary.map(Cover::convex))`, the
   plane's and cylinder's rule, beside geom's `face_region`. And the
   plane has two arms that both return `base()`.
2. Q4, likely. `census.rs` (near 7635 and 10315) still says a
   placeholder surface "is also reported on its own, as the kernel
   defect it is (`UncertifiableSurface`)" and leans on check 1 pushing
   it for every placeholder face. The second still holds inside tier 3;
   the first is no longer what the gate reports.
3. Q3, sure. D2.
4. Q6, sure. D4, and the cost sentence with no named denominator (D5).
5. Q5, likely. `face_extreme_points`'s doc now argues that no torus
   face needs its top circle read ("every finished torus face that does
   has a meridian seam"), a claim about three doors that nothing checks.

Ratings, against the first read: idiom and structure 4 (the rule has one
home now; the cone arm's leftover). Test quality 4, up from 3 (18 of 19
mutants die; the nappe guard is bare). Doc and comment honesty 4 (the
report-back is far better; D1, D3 and D5 are what is left of it).
