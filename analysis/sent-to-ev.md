**Two designers weighed this and agree; their texts are below, unsigned, as A and B. One thing is yours to decide: three passages of `crates/geom-core/README.md`, "Float rounding". They recommend all three. (B) can be struck or narrowed on its own; (A) and (C) stand without it.**

**(A) A lever read off geometry is known from both sides.** In the lever bullet, replacing the sentence that sorts verdicts by what their caller acts on (agent-written; you approved it on PR 7141):

> a lever read off a consumed extent is known from both sides, each measured from the site its datum is read at: from below off points known to lie on the region and from above off its cover. A verdict is served only where it holds at every length between the two: for a reading multiplied by its lever, a definite sign at the bound from below and a Zero at the bound from above. Where the two disagree the reading is in band and is refused as one.

What it commits the kernel to: every levered decision reads both bounds, and no caller chooses one. *Rejected:* the question the fork was raised on, which bound each of about 41 callers needs. A reading multiplied by its lever grows with the lever, so a definite sign is proved at the shorter length and a Zero at the longer, whoever acts on it.

**(B) A definite sign of a datum that turns about a known line is levered from that line.** In the consumed-extent bullet, narrowing "any point is sound provided the datum and its lever are read at the same one" (agent-written; you approved it on PR 6968 after asking how a midpoint that does not look like a witness could be sound):

> Its lever is the extent's far distance from the site its datum is read at; the least over all sites is not owed. A tilt moves a point of the extent by the tilt times that point's distance from the line it turns about, which is never more than its distance from any point of that line. So a lever read from a point proves a Zero, and a definite sign of a datum that turns about a known line is levered from that line: a dihedral along a line edge, from the edge's line. A record that verdicts read from different sites holds its region, not a lever.

Your doubt on PR 6968 was right for definite signs: any pivot proves a Zero, but a definite sign needs a point that really moves that far. Today a straight edge's dihedral is levered from the edge's middle, which counts the faces' run along the edge: on plane pairs it exceeds the exact lever on 99.5% of reads. *Alternatives:* strike (B), and Zeros become sound while definite signs stay over-read, so the open row's sharp edge on a declared-tangent joint is still built; or narrow it to two planes, and cylinder and cone walls along a ruling stay at the middle, that row's extrude case among them.

**(C) What stays open is named.** Under `## Open`, replacing "Nothing on this page.":

> Which site a dihedral along a curved edge is levered from (its station's tangent line or the edge's carrier), and the site and region of the joint turn at a pole; both are read from the span's middle.

**What moves if you approve, valid bodies included (your condition on PR 7140).**
- Faces of no width refuse, as they should (20 tests): every vertex is on the edge's line, the true lever is 0, the middle read them as wide.
- Faces narrower than the band refuse (three tests, true widths 1.5 ε to 5.3 ε); they build at a finer tolerance.
- A declared-tangent joint off tangent by more than about ε over the arc's radius goes from built with a wrong sharp edge to refused, by certification's own parallelism check, whose arm (1/κ) nothing caps at the faces' reach. That arm is a separate design question, filed, and until it is settled these bodies refuse where the profile accepts them.
- One served verdict moves, toward the exact lever.
- Two classes of valid bodies refuse only because a bound from above is missing (a plane through a sphere's pole cap; a face still a placeholder mid-construction). Both are to be fixed in code before the first change lands; one that cannot be comes back to you as a newly refusing class.

**What the evidence is, and what was withdrawn.** A Zero rests on a cover being a bound from above. Counted with nothing folded in, at ε 1e-9 over three suites: on plane faces the cover's own far distance is never short of the exact lever (4.2 million reads with straight edges, 778 thousand with arcs); on cylinder and cone walls it is short past ε on 5,354 of 549,861 reads, every one examined being a door at work on a face whose surface or boundary is not yet its own (a rebased run before its move lands, an edge certified onto a seed surface, a surface set before its boundary moves, a boolean's working face); no Zero is served at any of them, and no door that reads a finished body is short past ε. Several earlier counts held by construction, because the measuring build folded one bound into the other; the designers caught it, and neither text cites them.

**Not measured:** the design's own rule run on the sweep suite (running; on core it changed 0 of 4,799 outcomes); the counts at 1e-12 and 1e-6; the cost of reading curved boundary edges on the `Interval`, `Dual` and `Sym` lanes; the exact lever on cylinder and cone walls is a convexity argument checked on paper.

**Not asked of you** (sequencing, mine): the two-sided rule lands first and changes no README text; this PR's text merges together with the line-site code that makes it true, since the page is present tense, so after your answer the code joins this PR; curved edges and the joint turn follow when their exact levers exist.

The fifteen rounds, the four measured builds and the rows filed on the way are in `work/oneround/log.md`, the entries from 2026-11-21 09:01Z; the lane's notes are on `oneround/turn-lever` under `analysis/oneround/turn-lever/`.

---

## Designer A

**Recommendation (likely): approve three passages of `crates/geom-core/README.md` ("Float rounding").** With them, the code reads a straight edge's dihedral lever from the edge's line and decides each reading against both bounds.
- This is one `[ev]` PR: the text and the line-site code merge together.
- The change before it, deciding every levered reading against both bounds, lands first and changes no README text.

Terms:
- **Lever:** the length at which a tilt displaces the geometry it governs.
- **Bound from below:** a length the region certainly reaches, read off points on it.
- **Bound from above:** a length it certainly does not pass, read off its cover.
- **Site:** what the lever's distance is measured from.
- **Straddle:** one reading that is Zero at one bound and definite at the other.
- **Truth:** the exact lever, where one is known: the farthest boundary point from the line, for plane, cylinder and cone faces bounded by lines and conics. Distance from a line is convex, so on such faces its maximum lies on the boundary.

#### The passages

**(A)** In the lever bullet, after "no longer than the site's own measure of it:"

> a lever read off a consumed extent is known from both sides, each measured from the site its datum is read at: from below off points known to lie on the region and from above off its cover. A verdict is served only where it holds at every length between the two: for a reading multiplied by its lever, a definite sign at the bound from below and a Zero at the bound from above. Where the two disagree the reading is in band and is refused as one.

**(B)** In the consumed-extent bullet, replacing "Its lever is the extent's far distance from that point" through "holds its region, not a lever":

> Its lever is the extent's far distance from the site its datum is read at; the least over all sites is not owed. A tilt moves a point of the extent by the tilt times that point's distance from the line it turns about, which is never more than its distance from any point of that line. So a lever read from a point proves a Zero, and a definite sign of a datum that turns about a known line is levered from that line: a dihedral along a line edge, from the edge's line. A record that verdicts read from different sites holds its region, not a lever.

**(C)** Under `## Open`, replacing "Nothing on this page.":

> Which site a dihedral along a curved edge is levered from (its station's tangent line or the edge's carrier), and the site and region of the joint turn at a pole; both are read from the span's middle.

#### Why

- **Nobody needs to choose a bound (sure, by arithmetic).** A reading multiplied by its lever grows with the lever. So a definite sign proved at the bound from below holds at every longer length, and a Zero proved at the bound from above holds at every shorter one. Reading both bounds therefore serves exactly what the right single bound would, and refuses the rest. No caller has to be sorted by what it acts on.
- **Today's site over-reads the lever, so a definite sign read there from below is unsound (sure).** On edges with an exact truth, the bound from below read at the span's middle exceeded the truth on 99.5% of reads. It counted the faces' run along the edge, which a dihedral does not displace.
- **Your PR 6968 question.** You asked how the midpoint could be sound when it is no witness. The answer approved there ("any pivot is sound") holds for a Zero only, and (B) says so.

#### Evidence at the line site

- **Served verdicts against the truth.** At ε 1e-9, 9.09 M served verdicts were read against a truth and 0 differ (likely). This check is partly circular: on exact reads the truth is also the bound from below. What it shows independently is that every Zero it served agrees with the truth.
- **The cover is a bound from above (likely).** Counted with nothing folded in, at ε 1e-9, over topo, step-import and sweep in full.

  | class | reads | below the truth by more than ε |
  |---|---|---|
  | plane, straight edges | 2.42 M core + 1.77 M sweep | 0 (core's 160 hits within ε are rounding, ≤ 2.4e-14) |
  | plane, arc edges | 222,780 core + 554,854 sweep | 0 |
  | cylinder and cone walls | 10,737 core + 539,124 sweep | 108 core + 5,246 sweep, all at mid-construction doors (see the next bullet) |

  - On a plane, the truth was computed independently of the covers.
  - Every at-rest door (tier 3, the boolean's result, attach's at-rest read) stays within ε. Its largest shortfall is 5e-10, which equals that boundary's own distance off its surface.
- **Mid-construction faces whose surface is not yet their own.** Every read past ε comes from one of four door states:
  - a rebased run read before its move lands;
  - an edge certified onto a face that still carries its seed surface (blends, chamfers and fillets reach the kernel this way);
  - an offset's surface set before its boundary moves (the boundary sits exactly the offset distance off it);
  - a boolean's working face.

  Of 725 sampled wall reads past ε, every one has a wall face 0.02 to 2 off its surface. There the truth does not apply, and no Zero or other verdict needing the bound from above is served. The verdicts served there are definite signs from below: 560 `tangent_second_order` Positive at the graft, and 4 `classify_dihedral` Transverse at a rebased run. Those stand under the rule and are re-asked at rest, where the finished bodies pass.
- **The rule, run (core).** With an inversion past ε treated as no bound from above, 0 of 4,799 test outcomes change. The refusals (90), straddles (7) and moves against the middle (34) are identical. The design run on the sweep is still running.
- **What the line site needs, and gets in its code.** The bound from below reads each boundary edge's farthest point from the line: a line's ends, a conic's critical points, 17 samples otherwise. Read off vertices alone, a face whose corners sit on the line reads a lever of about 0, and many valid bodies refused.
- **The `Dual` lane does not refuse (sure).** Its not-a-number is in the derivative of a cover's far distance only, a square root at zero in the cover's frame, and no decision reads a derivative. In the second run, 116,359 `Dual` verdicts at 1e-9 were refused 0 times, straddled 0 times and moved 0 times. The cost to a sensitivity read is filed as its own row.

#### What moves, against the truth (valid bodies included, per PR 7140)

- **Faces of no width now refuse, as they should (20 tests at 1e-9).** Every vertex lies on the edge's line, so the true lever is 0, and the middle read them as wide. The tests pinned the old refusal and are rewritten.
- **Faces narrower than the band now refuse.**
  - Two bisector fixtures, true widths 5.0e-9 and 5.3e-9 at ε 1e-9, read by the middle as 1e-4.
  - A tilted-sketch sliver of true width 1.5e-9.
  - The truth confirms each, and each builds at 1e-12. These verdicts move toward refusing because the lever is read honestly.
- **One served verdict moves.** A keel near-tangent to a bore's wall reads Zero instead of Negative, on a lever of 0.0671 the truth confirms; the middle read 1.002. The test still passes.
- **Two classes of valid bodies refuse only because a bound from above is missing.** Neither is a question about the site.
  - A plane cutting a sphere's pole cap (7 turns): the site mint returns no bound from above, by construction.
  - The contact-edge corpus (24 must-carry readings): a face still a placeholder seed.
  - Both are fixed in code before the first change lands: the site mint's cover is read off the geometry the door holds, and those doors decide once the face has a surface. Whichever cannot be done comes back to you as a newly refusing class.
- **The open row on a face's run along its edge** (an extrude profile: a line into an arc, φ off tangent).
  - The wall join now reads the true lever, 0.05, and the declared tangency reads smooth. The sharp edge tier 3 passed at 15ε and 19ε is gone.
  - What now refuses is certification's normal-parallelism check. It is levered at the arc's 1 m curvature arm, which nothing caps at the region's reach.
  - That is a separate row, so the open row does not close with this PR.

#### What is not yet decided

- **Curved edges and the joint turn: (C).** No truth exists yet, and they stay at the middle.
- **Still running; these gate the merge, not the question:**
  - the rule run on the sweep;
  - the recount at 1e-12 and 1e-6;
  - the symbolic lanes' cost of the curve read;
  - a machine check of the ruled-face truth against an interior maximiser.

  Any of them can only add to what refuses, or find a cover defect to fix before merge. The text stays.

#### Order in the tree

1. **The first change, no README text.**
   - Every levered reading is decided against both bounds.
   - A typed "no bound from above" case covers a face that does not yet carry its own surface or boundary. It is entered from what the door knows, with an inversion past ε as a backstop.
   - Covers are read lazily, once per body state.
   - No verdict moves at finite bounds at today's site.
2. **This PR:** (A), (B) and (C), with the line site and its curve-reading bound from below, merged together.
3. **Curved edges and the joint turn,** once their truths exist.

Confidence:
- the recommendation: likely;
- the arithmetic premise and (B)'s inequality: sure;
- "no served verdict past the truth": likely;
- the cover as a bound from above: likely, on both suites at 1e-9;
- the moved classes: sure as counted;
- `Dual`: sure.

---

## Designer B

**Decision asked: approve or strike three passages in `crates/geom-core/README.md`, "Float rounding".** Together they say a lever read off geometry is known from both sides, and that a dihedral along a straight edge is levered from the edge's line. I recommend all three (likely). I ran nothing; every figure is the lane's.

Terms:
- *Lever*: the length a tilt is multiplied by to get a displacement.
- *Bound from below / from above*: a length the region certainly reaches / certainly does not pass.
- *Site*: the point or line a lever's distance is measured from.
- *Dihedral*: the angle between an edge's two faces.
- *Straddle*: one decision whose two bounds read differently.
- *Cover*: a region known to contain a face.
- *Core* and *sweep*: the two test suites measured (topo with step-import; the sweep crate).

#### The passages

**(A)** Lever bullet. After "no longer than the site's own measure of it:", replacing the sentence that sorts verdicts into three kinds:

> a lever read off a consumed extent is known from both sides, each measured from the site its datum is read at: from below off points known to lie on the region and from above off its cover. A verdict is served only where it holds at every length between the two: for a reading multiplied by its lever, a definite sign at the bound from below and a Zero at the bound from above. Where the two disagree the reading is in band and is refused as one.

**(B)** Consumed-extent bullet. Replacing "Its lever is the extent's far distance from that point … holds its region, not a lever.":

> Its lever is the extent's far distance from the site its datum is read at; the least over all sites is not owed. A tilt moves a point of the extent by the tilt times that point's distance from the line it turns about, which is never more than its distance from any point of that line. So a lever read from a point proves a Zero, and a definite sign of a datum that turns about a known line is levered from that line: a dihedral along a line edge, from the edge's line. A record that verdicts read from different sites holds its region, not a lever.

**(C)** Under `## Open`, replacing "Nothing on this page.":

> Which site a dihedral along a curved edge is levered from (its station's tangent line or the edge's carrier), and the site and region of the joint turn at a pole; both are read from the span's middle.

#### Premise, and the text this replaces

- **The fork was opened on "which bound does each of about 41 callers need". That is the wrong question.** A reading multiplied by its lever grows with the lever. So a definite sign is proved at the bound from below and a Zero at the bound from above, whoever acts on it. This is an argument, not a count.
- **The measured defect is the site.** On straight edges between plane faces with straight boundaries, the exact lever is the farthest vertex's distance from the edge's line. The points' reach from the edge's middle exceeds it on 99.5% of 8.78 M reads. It counts the faces' run along the edge.
- **(B) narrows "any point is sound provided the datum and its lever are read at the same one".** That clause is agent-written. You approved it on PR 6968 after asking how a midpoint that "doesn't look like a witness" could be sound.
  - The answer given then holds for a bound from above, which needs no witness.
  - A definite sign does need one: a point that really moves that far. Your question was the right one for it.
- The sentence (A) replaces is also agent-written, approved on PR 7141.

#### Final states

1. **(A), (B) and (C): recommended.**
   - Every levered decision reads both bounds.
   - Wedges on straight edges read from the edge's line.
   - Curved edges and the joint turn stay at the middle, named as open.
   - Left possible: a door building its region from too little geometry. Review holds that, as today.
2. **(A) and (C), with (B) struck.** Zeros become sound. Definite signs stay over-read as today. The open row `the-wedge-reach-counts-a-faces-run-along-its-edge` stays as it is: a sharp edge built on a declared-tangent joint, passing tier 3.
3. **(B) narrowed to two planes.** Cylinder and cone walls along a ruling stay at the middle. That row's extrude case is one of them, so it stays wrongly built.
4. **Nothing.** Today.

Each is reversible: the site is one read in the consumer, and the text is two sentences.

**Worked example.** A strip 4 long and 0.05 wide, at 15ε to its neighbour along its long edge.
- From the middle the lever is 2, so the reading is 30ε: a corner.
- From the edge's line the lever is 0.05, so the reading is 0.75ε: the tangent joint that was declared.

A second case: a half-disc along its diameter. Both vertices are on the line, so the bound from below has to read the arc; the lever is the radius.

#### The evidence

**Definite signs need no measurement.** The bound from below is read off points of the boundary, so the region reaches at least that far.

**Zeros rest on the cover being a bound from above. That was counted with nothing folded in:** the covers' own far distance from the line, against the exact lever. ε = 1e-9, every test in both suites.

| face class | reads, core | reads, sweep | cover short by more than ε, core / sweep |
|---|---|---|---|
| plane, straight edges | 2,422,531 | 1,768,879 | 0 / 0 |
| plane, arcs | 222,780 | 554,854 | 0 / 0 |
| cylinder or cone wall | 10,737 | 539,124 | 108 / 5,246 |

- **Every read past ε is at a door working on a face that is not finished.** There are four such states:
  - a run of edges re-based before its move lands;
  - an edge certified onto a face still carrying its seed surface;
  - a surface set before its boundary moves (offsets and shells; the boundary stands exactly the offset distance off);
  - a boolean's working face.
- **Of 725 such wall reads examined, each has a wall face 0.02 to 2 off its own surface.** That is a sample: at most five per door and class per test, with every one of the 179 affected tests represented.
- **No door that reads a finished body has a cover short past ε.** The largest shortfall there is 5e-10, and it equals the boundary's own distance off its surface.
- **No Zero stands at any such read.** The 564 verdicts served at them are all definite signs, proved from below.
- **The design's rule** treats a cover short by more than ε as a missing bound from above. Run on core, it changed 0 of 4,799 test outcomes against the run that folded the two bounds. The same run on the sweep is in progress. Since no Zero stands at such a read, it is expected to change nothing.

**The exact lever for cylinder and cone walls** is the boundary's farthest point from the line, by a convexity argument along rulings. It is checked on paper only; a check against an interior maximiser is not in. One hole, a cone's apex inside its face, is guarded.

**One body built to a known answer on a curved wall:** the open row's extrude case. A plane wall meets a cylinder wall along a ruling, declared tangent. The line site reads the lever 0.05 and the join reads smooth.

**The suites, line site against middle site at 1e-9:** no test changes outcome except the classes below.

#### Classes that move

- **Declared-tangent joints off tangent by more than about ε over the arc's radius** (the open row; here 15ε and 19ε on a 1 m arc). Before: built with a sharp edge. Now: the join reads smooth and certification refuses it.
  - What refuses is certification's normal-parallelism check, levered at 1/κ = 1 m and never capped by the face's 0.05 reach.
  - Whether that arm is right is a separate row. Until it is settled, these bodies refuse where the profile accepts them.
- **Faces whose true width is inside the band** (1.5ε to 5.3ε wide, three tests). They built because the lever counted their length. They now refuse in band, and the exact lever agrees.
- **Faces of no width** (20 tests). They refuse at the arm check, as intended. The tests pin the old refusal and get rewritten.
- **One near-tangent keel wall** reads Zero where the middle read Negative. The lever is 0.067; the middle read 1.002. Its test still passes.

#### Unmeasured, and told as such

- **The design's rule on the sweep:** running, as above.
- **The counts above at 1e-12 and 1e-6.**
- **The curve-reading bound from below on lanes other than f64.** On f64 it is a numerical maximum. On `Interval`, `Dual` and `Sym` it is 17 samples, so those lanes read a shorter lever. Its cost there is not timed.
- **One corpus test newly refuses** (24 reads): must-carry is asked over a face that is still a placeholder mid-construction. Whether production asks it there is not attributed.
- **Curved edges and the joint turn:** no exact lever built, as (C) says.

Settled since earlier drafts: the `Dual` lane does not refuse. Its NaN is in a derivative that no decision reads, and none of its 116,359 verdicts is refused, straddled or moved.

#### Order in the tree

1. The two-sided rule lands first, with a missing bound from above as a typed case that an unfinished face's door enters, and covers read lazily. It changes no README text.
2. The line site and the curve-reading bound from below join this PR after your answer. Text and code merge together.
3. Curved edges and the joint turn follow once their exact levers exist.

Confidence:
- The recommendation: likely.
- The inequality in (B): sure.
- The line as the site for two planes: sure.
- For cylinder and cone walls along a ruling: likely (argument on paper, one body built to a known answer, no counter-case in 550,000 reads).
- That covers bound from above: sure on plane faces; likely on cylinder and cone walls (none short at any finished body; the unfinished ones examined by sample).
- The two-sided rule's soundness: sure. Its completeness on lanes other than f64: unsure.
- (C): sure that it is open.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01EdpJs4dRmhgaKbdqQzfNmw
