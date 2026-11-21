Round 5 on the conic fork. A measurement all three of you built on in rounds 3 and 4 was judged against the wrong curve, and the lane has corrected it. Same form as before: `## For Ev` (forwarded verbatim, unsigned; Ev reads on a phone; lead with what changed and what you now recommend), `## For the orchestrator`, at most 150 lines, read-only. All three of you get this same message and not each other's answers.

WHAT YOU AGREED IN ROUND 4 (and what went to Ev): a placement inside `Circle`/`Ellipse` (`Centre(point)` or `OnArc(point)`), fixed by the construction, no size picking; a construction whose centre runs off at a reachable pose holds the arc (a kernel bug otherwise); a centre-held conic read at f64 by chords from its seam vertex `fl(centre + u·a)` and about its centre at the enclosure scalars; no length rule (two of you) or a rule at the rounding reach `R` (one), to be decided by a probe. The chord reading rested on the size table's "reader-chord" column ("certifies 40 of 40 at every size, digit for digit as on-arc").

THE LANE'S CORRECTION, verbatim (07:03Z):
"What Probe 1's chord cells were judged against: the exact oracle run on the CHORD conic itself (an anchored twin at `fl(centre + u·a)`; the oracle reads that twin's derived centre). So "0 wrong" there means: right for the circle through `fl(centre + u·a)`. It was NOT judged against the circle about the stored centre. The centre-sum cells are unaffected (judged against the stored centre's circle).
Why it matters, measured just now in Probe 2 (unit branch + temporary f64 chord switch, eps 1e-9, K 10): the row `in_band_sweep::a_circle_of_radius_1500_places_its_roots_within_the_zero_band` fails with 50 answers. Three of its poses, taken apart:
- the stored-centre circle's exact roots and the `fl(centre + u·a)` circle's exact roots lie 1.2e-9, 1.2e-9 and up to 2.0e-9 m apart along the arc (graze depths -2.2e-8, -3.4e-8, +4.3e-7 m; radius 1500): one rounding of the seam vertex (ulp(1500) = 2.3e-13 m) moved a near-tangent root by more than eps;
- with the switch, the door on the centre-held conic certifies the `fl(centre + u·a)` circle's roots (equal to the anchored twin's to 1e-22 rad): within 4e-14 m of that circle's truth, 1.2e-9 to 2.0e-9 m from the stored centre's;
- without the switch the door refuses those three poses on the centre-held conic (`Unsettled::Meter { bool_conic_quadric_sub_root_slack }`) and certifies the same roots on the anchored twin.
So in the chord reading a definite answer is right or wrong by more than eps according to which circle is the conic: the one about the stored centre, or the one through the rounded seam vertex."

THE RE-TALLY OF PROBE 1, verbatim (07:07Z). Probe 1 is: circles with centres D = 3e3, 1e4, 1e5 m out and their arcs in the box, stored through three real doors (a nearly flat bulge arc extruded; a unit square revolved about a far axis; a plane cut of the wall); 48 near-graze poses per edge against a wall of radius 1 at depths ±m·K·ε, 384 per D; the conic-against-quadric root door; an exact-rational oracle.
"The chord reading's definite answers, judged two ways:
(1) against the exact roots of the circle through `fl(centre + u·a)` (the chord conic's own; what the first message reported): 0 wrong in all cells. Unchanged.
(2) against the exact roots of the circle about the STORED CENTRE. Right / WRONG / refused of 384:
eps, K      | D   | chord f64      | chord Interval
1e-9, 10    | 3e3 | 192 / 0 / 192  | 72 / 0 / 312
1e-9, 10    | 1e4 | 93 / 3 / 288   | 48 / 0 / 336
1e-9, 1.002 | 3e3 | 96 / 0 / 288   | 48 / 0 / 336
1e-9, 1.002 | 1e4 | 39 / 9 / 336   | 0 / 0 / 384
every other cell (D 1e5; all of eps 1e-12) | 0 / 0 / 384 | 0 / 0 / 384
The twelve:
- doors: the sketch-arc prism's edges and its plane section's, radius 1e4 (none on the revolve's edges, none at 3e3);
- depths -1e-6 m (K 10, three answers) and -5.01e-7 m (K 1.002, nine);
- each has the right COUNT of roots; the roots sit 1.4e-9 to 1.9e-9 m (K 10) and 1.4e-9 to 2.7e-9 m (K 1.002) along the arc from the stored centre's true roots. Past eps = 1e-9; at K = 10 inside K·eps = 1e-8; at K = 1.002 past K·eps too.
- the same twelve are within eps of the roots of the circle through `fl(centre + u·a)`.
- The two circles differ by the rounding of the seam vertex, at most ulp(1e4) = 1.8e-12 m a coordinate; at these near-tangent crossings that moves a root by more than eps.
- f64 lane only. The Interval lane (chord conic and wall lifted to point enclosures) has no such answer in any cell.
The centre-sum reading: unchanged, 0 wrong (it is judged against the stored centre's circle and refuses these poses).
Run constants as before: (1e-9, 10) R 5.07e6 m, (K-1)eps/u 8.11e7 m; (1e-9, 1.002) R 1.13e3 m, 1.80e4 m."
For reference, the centre-sum f64 cells of that probe (judged against the stored centre): (1e-9, 10) 96/0/288 at 3e3, 48/0/336 at 1e4; (1e-9, 1.002) 48/0/336 at 3e3, 24/0/360 at 1e4; 0/0/384 elsewhere.

NOT YET IN, and asked for: the size table's reader-chord column re-tallied against the stored centre's curve (10 m to 10 km); the same cells with the seam vertex's rounding charged into the root door's meter; the root's displacement per metre of rigid shift at the three poses taken apart. Treat the size table's reader-chord column as unverified until then: its "as good as on-arc" was most likely judged the same way. Its centre-form and on-arc columns stand (each judged against its own stored datum's curve; an arc-held conic's stored point is exact, so its own conic is the truth). The tree and the probes are where they were (`origin/oneround/anchored-conic`; `origin/analysis/oneround/conic-vertex-probes`, `analysis/oneround/conic-vertex/far-centre-through-real-doors/` and `size-table/`); read the READMEs.

WHAT I NEED FROM YOU.
(a) What a centre-held conic IS, and so what its f64 reading must be judged against. Then the reading in the final state: the centre sum as main has it; chords with something charged (say exactly what is charged, where, and what is gained once it is); or something else. Note that three of the twelve wrong roots are at K = 10, D = 1e4, far inside `R`.
(b) Whether which point is stored matters after all for a large conic (the centre-held one refuses near-graze poses from about 500 m by the size table's centre column; the arc-held one certifies them), and what follows for the final state under Ev's two constraints: no cutoff chooses the form, and a wrong choice by the kernel may be a panic. Say plainly what a person gets who authors a large centre-held conic.
(c) The length rule, in the light of (a): the one probe you all named has now returned "wrong definite verdicts in the chord reading, none in the centre sum".
(d) Which of round 4's agreed points you still hold, one line each.
(e) The README sentence under your answer.
(f) Any measurement still needed before this goes back to Ev, with which result means which answer; and say what each measurement must be judged against.
