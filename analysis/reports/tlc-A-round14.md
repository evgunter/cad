**(1)** Ev can be asked now, with the sweep's recount stated as still running. No result on the sweep would change a sentence of (A), (B) or (C).
- (A) says the bound from above is read off a cover. If the sweep finds a finished wall whose cover falls below the truth while its boundary is on its surface, that is a soundness defect in `topo::cover`. It would block the line-site code until fixed. The text stays.
- If the sweep instead shows refusals, they only add entries to the list of what refuses.
- Since text and code merge in one PR, the sweep gates the merge, not the question.

**(2)** No change to (A), (B) or (C).

**(3)** Yes: in the build it is the typed "no bound from above" case, and it belongs to step 1. The bounds' constructor routes an inversion past ε there; run that way, 0 of 4,799 test outcomes change.
- Certification already behaves this way in effect, since it reads from below alone and no verdict there needs the bound from above.
- The better trigger is what the door knows: the face still carries its seed surface, or the move has not landed. The door should enter the typed case from that, and keep the inversion check as a backstop.

## For Ev

**Recommendation (likely): approve three passages of `crates/geom-core/README.md` ("Float rounding").** With them, the code reads a straight edge's dihedral lever from the edge's line and decides each reading against both bounds.
- This is one `[ev]` PR: the text and the line-site code merge together.
- The change before it, which decides every levered reading against both bounds, lands first and changes no README text.

Terms:
- **Lever**: the length at which a tilt displaces the geometry it governs.
- **Bound from below**: a length the region certainly reaches, read off points on it.
- **Bound from above**: a length the region certainly does not pass, read off its cover.
- **Site**: what the lever's distance is measured from.
- **Straddle**: one reading that is Zero at one bound and definite at the other.
- **Truth**: the exact lever where one is known. That is the farthest boundary point from the line, for plane, cylinder and cone faces bounded by lines and conics: distance from a line is convex, so its maximum on such a face lies on the boundary.

### The passages

**(A)** In the lever bullet, after "no longer than the site's own measure of it:"

> a lever read off a consumed extent is known from both sides, each measured from the site its datum is read at: from below off points known to lie on the region and from above off its cover. A verdict is served only where it holds at every length between the two: for a reading multiplied by its lever, a definite sign at the bound from below and a Zero at the bound from above. Where the two disagree the reading is in band and is refused as one.

**(B)** In the consumed-extent bullet, replacing "Its lever is the extent's far distance from that point" through "holds its region, not a lever":

> Its lever is the extent's far distance from the site its datum is read at; the least over all sites is not owed. A tilt moves a point of the extent by the tilt times that point's distance from the line it turns about, which is never more than its distance from any point of that line. So a lever read from a point proves a Zero, and a definite sign of a datum that turns about a known line is levered from that line: a dihedral along a line edge, from the edge's line. A record that verdicts read from different sites holds its region, not a lever.

**(C)** Under `## Open`, replacing "Nothing on this page.":

> Which site a dihedral along a curved edge is levered from (its station's tangent line or the edge's carrier), and the site and region of the joint turn at a pole; both are read from the span's middle.

### Why

- **Choosing a bound per caller never changed a verdict.** At today's site, the span's middle, the two bounds agreed in all of about 282 M paired decisions. The defect was the site.
- **The middle over-reads the lever, so a definite sign read there from below is unsound (sure).** On edges with an exact truth, the middle's bound from below exceeded the truth on 99.5% of reads. It counted the faces' run along the edge, which a dihedral does not displace.
- **Your PR 6968 question.** You asked how the midpoint could be sound when it is no witness. The answer approved there ("any pivot is sound") holds for a Zero only, and (B) says so.

### Evidence at the line site

- **No served verdict differs from the truth's, at any ε run** (9.09 M verdicts read against a truth at 1e-9, 0 differ). This is likely right, and partly circular: on exact reads the truth is also the bound from below. What it independently shows is that every served Zero agrees with the truth.
- **The cover is a bound from above.** Checked with nothing folded into it, against the bound from below and against the truth, on topo and step-import at ε 1e-9 (likely on this suite):
  - *Plane faces, straight edges* (2.42 M reads): never below the truth by more than ε. The 160 relative-only hits are rounding, at most 2.4e-14 absolute.
  - *Plane faces, arc edges* (222,780 reads): never below the truth by more than ε. There are 10 rounding hits.
  - *Cylinder and cone walls* (10,737 reads): 108 below the truth, all past ε. In every one the face still carries a surface that is not its own: an edge is being certified onto a face that has its seed surface, or a rebased run is read before its move lands. The boundary stands 0.74–2 off that surface, so the truth does not apply. No read whose boundary lies on its surface falls below it.
  - None of those reads served a Zero, or any verdict that needs the bound from above. Both doors are certification, which reads from below alone.
- **The design's rule, run.** A bound from above below the bound from below by more than ε is treated as missing: definite signs stand, Zeros refuse. Run that way on the same suite:
  - 0 of 4,799 test outcomes change;
  - the refusals (90), straddles (7) and moves against the middle (34) are identical.
- **What the line site needs, and gets in its code.** The bound from below reads each boundary edge's farthest point from the line: a line's ends, a circle's or ellipse's critical points, 17 samples otherwise. Read off vertices alone, a face whose corners sit on the line reads a lever of about 0, and many valid bodies refused.
- **The `Dual` lane does not refuse.** The not-a-number seen there is in the derivative of a cover's far distance only: a square root at zero in the cover's frame. No decision reads a derivative. In the second run, 116,359 `Dual` verdicts at 1e-9 were refused 0 times, straddled 0 times and moved 0 times. What the derivative costs a sensitivity read is filed as its own row.

### What moves, against the truth (valid bodies included, per PR 7140)

- **Faces of no width now refuse, as they should** (20 tests at 1e-9). Every vertex lies on the edge's line, so the true lever is 0, and the middle read them as wide. The tests pinned the old refusal and are rewritten.
- **Faces narrower than the band now refuse.**
  - Two bisector fixtures, true widths 5.0e-9 and 5.3e-9 at ε 1e-9, which the middle read as 1e-4.
  - A tilted-sketch sliver of true width 1.5e-9.
  - The truth confirms each, and they build at 1e-12. These are verdicts moving toward refusing because the lever is read honestly.
- **One served verdict moves.** A keel near-tangent to a bore's wall reads Zero instead of Negative, on a lever of 0.0671 the truth confirms. The middle read 1.002. The test still passes.
- **Two classes of valid bodies refuse only because a bound from above is missing.** Neither is a site question.
  - A plane cutting a sphere's pole cap (7 turns): the site mint returns no bound from above, by construction.
  - The contact-edge corpus (24 must-carry readings): a face still a placeholder seed mid-construction.

  Both are fixed in code before the first change lands. The site mint's cover is read off the geometry the door holds, and those doors decide once the face has a surface. If either cannot be done, it comes back to you as a newly refusing class.
- **The open row on a face's run along its edge** (an extrude profile: a line into an arc, φ off tangent).
  - The wall join now reads the true lever, 0.05, and the declared tangency reads smooth. The sharp edge that tier 3 passed at 15ε and 19ε is gone.
  - What now refuses it is certification's normal-parallelism check. That check is levered at the arc's 1 m curvature arm, which nothing caps at the region's reach.
  - That is a separate row, so the open row does not close with this PR.

### What is not yet decided

- **Curved edges and the joint turn: (C).** No truth exists yet, and both stay at the middle.
- **Still running; these gate the merge, not the question:**
  - the same recount and design run on the sweep suite, where most curved walls of finished bodies are;
  - the same at 1e-12 and 1e-6;
  - the symbolic lanes' cost of the curve read;
  - a machine check of the ruled-face truth against an interior maximiser.

  A finished wall whose cover falls below the truth would be a defect in the cover read, fixed before merge. The text stays either way.

### Order in the tree

1. Step 1, landing first:
   - every levered reading decided against both bounds;
   - a typed "no bound from above" case, entered where the face does not yet carry its own surface, with an inversion past ε as a backstop;
   - covers read lazily, once per body state.

   No README change, and no verdict moves at finite bounds at today's site.
2. This PR: (A), (B) and (C), with the line site and its curve-reading bound from below, merged together.
3. Curved edges and the joint turn, once their truths exist.

Confidence:
- the recommendation: likely;
- (B)'s inequality: sure;
- "no served verdict past the truth": likely;
- the cover as a bound from above: likely on core, unmeasured on the sweep;
- the moved classes: sure as counted;
- `Dual`: sure that it does not refuse.

## For the orchestrator (changes only)

- **What gates the merge, not the `[ev]` PR:**
  - the sweep recount and design run;
  - the 1e-12 and 1e-6 recounts;
  - the symbolic-lane cost of the curve read;
  - the interior-maximiser check of the ruled truth.

  Any finished-body cover below the truth while its boundary is on its surface gets filed against `topo::cover` and blocks the line-site code.
- **The design run made 66 fewer verdicts than the folded run** (out of 1.8 M). The difference is not attributed. Attribute it before step 1 lands: a path that changes with a cover's fold is worth one look for determinism.
- **Retire the first-run arc figures.** Cite only the recount's table: the "1.39 M with 80 below" figure folded vertices into the bound from above, and "402,282" counted vertex under-reads, not covers.
- **Make the door's knowledge the trigger for the typed case.** Step 1 should enter it from what the door knows (a seed surface, a move that has not landed), not only from an inversion. The 129 reads show the door already knows.
