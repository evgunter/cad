**(a)** (B) stands as signed. Under config A, with both bounds truthful, no served verdict anywhere differs from the truth's verdict: 9.09 M at 1e-9, 0 at every ε. No refusal traces to the site on a curved face. Config B tests a tighter bound from above, which affects completeness for Zeros, not the site, so (B) does not wait for it.

**(b)** Nothing changes in (A), (B) or (C) as signed in round 11. Each sentence says what a lever is. "Both are read from the span's middle" in (C) is still true of the tree.

**(c)** It is part of the build that follows (step 2), not part of what Ev is asked. It is what (A)'s "points known to lie on the region" means at a line. It stays sound on the lanes where it falls back to 17 samples, because every sample lies on the edge.

**(d)** It is a consequence Ev is told is unmeasured, and it does not change the text. A NaN is not a bound from above, so (A) already says what follows: no Zero is served there. If every covered face refuses on `Dual`, (A), (B) and (C) stand. What changes is step 1's landing: the cover's far-distance read must carry a usable derivative on `Dual`, or `Dual` must take only the value of a bound. It is a defect in the cover read, and it gates step 1 as a newly refusing class.

**(e)** It is a row beside this fork, on certification's slate. Its arm is a curvature arm, 1/|κ_rel|, which the reach never caps, so it can be longer than "the site's own measure". It is the same principle on a different lever, and it is now what keeps the open extrude row from closing.

## For Ev

**Recommendation (likely): approve three passages of `crates/geom-core/README.md` ("Float rounding"). With them, the code reads a straight edge's dihedral lever from the edge's line, decided against both bounds.** This is one `[ev]` PR: the text and the line-site code merge together. The one change before it, deciding every levered reading against both bounds, lands first and changes no README text.

Terms:
- **Lever**: the length at which a tilt displaces the geometry it governs.
- **Bound from below / from above**: a length the region certainly reaches, read off points on it / certainly does not pass, read off its cover.
- **Site**: what the lever's distance is measured from.
- **Straddle**: one reading that is Zero at one bound and definite at the other.
- **Truth**: the exact lever, where one is known. It is known for plane, cylinder and cone faces whose boundary edges are lines or conics, because distance from a line is convex and peaks on the face's boundary.

### The passages

**(A)** In the lever bullet, after "no longer than the site's own measure of it:"

> a lever read off a consumed extent is known from both sides, each measured from the site its datum is read at: from below off points known to lie on the region and from above off its cover. A verdict is served only where it holds at every length between the two: for a reading multiplied by its lever, a definite sign at the bound from below and a Zero at the bound from above. Where the two disagree the reading is in band and is refused as one.

**(B)** In the consumed-extent bullet, replacing "Its lever is the extent's far distance from that point" through "holds its region, not a lever":

> Its lever is the extent's far distance from the site its datum is read at; the least over all sites is not owed. A tilt moves a point of the extent by the tilt times that point's distance from the line it turns about, which is never more than its distance from any point of that line. So a lever read from a point proves a Zero, and a definite sign of a datum that turns about a known line is levered from that line: a dihedral along a line edge, from the edge's line. A record that verdicts read from different sites holds its region, not a lever.

**(C)** Under `## Open`, replacing "Nothing on this page.":

> Which site a dihedral along a curved edge is levered from (its station's tangent line or the edge's carrier), and the site and region of the joint turn at a pole; both are read from the span's middle.

### Why

- **Choosing a bound per caller never changed a verdict.** On today's site (the span's middle) the two bounds agreed in all of about 282 M paired decisions. The defect was the site.
- **The middle over-reads the lever, so definite signs from below are unsound there.** On edges with an exact truth, the middle's bound from below exceeded the truth on 99.5% of reads. It counted the faces' run along the edge, which a dihedral does not displace. (sure)
- **Your PR 6968 question.** You asked how the midpoint could be sound when it is no witness. The answer approved there ("any pivot is sound") holds for a Zero only. (B) says so.
- **At the line, both bounds decided, nothing served is wrong.**
  - At every ε run, no served verdict differs from the truth's.
  - No bound from above is below its bound from below, and no definite sign from below meets the opposite sign from above.
  - The truth is the bound from below on exact reads, so the equal count alone is partly circular. The two zero counts in the bullet above do not depend on it. (likely)
- **What the line site needs, and gets in its code.** The bound from below reads each boundary edge's farthest point from the line: the ends of a line, the critical points of a circle or ellipse, 17 samples otherwise. With vertices alone, a face whose corners sit on the line read a lever of about 0, and many valid bodies refused.

### What moves, against the truth (valid bodies included, as you asked on PR 7140)

- **Faces of no width now refuse, as they should** (20 tests at 1e-9). Every vertex lies on the edge's line, so the true lever is 0. The middle read them as wide. The tests pin the old refusal and are rewritten.
- **Faces narrower than the band now refuse.** Two bisector fixtures have a true width of 5.0e-9 and 5.3e-9 at ε = 1e-9, where the middle read 1e-4. A tilted-sketch sliver has a true width of 1.5e-9. The truth confirms each width, and they build at 1e-12. These are verdicts moving toward refusing because the lever is now read honestly.
- **One served verdict moves.** A keel near-tangent to a bore's wall reads Zero instead of Negative, on a lever of 0.0671 that the truth confirms. The middle read 1.002. The test still passes.
- **Two classes of valid bodies refuse only where the bound from above is missing.** Neither is a site question.
  - A plane cutting a sphere's pole cap (7 turns). The site mint returns no bound from above by construction.
  - The contact-edge corpus (24 must-carry readings): a face still a placeholder seed mid-construction.

  Both are fixed in code before the first change lands. The site mint's cover is read off the geometry the door holds, and those doors decide once the face has a surface. If either cannot be done, it comes back to you as a newly refusing class.
- **The open row on a face's run along its edge** (an extrude profile: a line into an arc, φ off tangent).
  - The wall join now reads the true lever, 0.05, and the declared tangency reads smooth.
  - The sharp edge that tier 3 used to pass at 15ε and 19ε is gone.
  - What now refuses it is certification's normal-parallelism check. That check is levered at the arc's curvature arm, 1 m, which nothing caps at the region's reach. That is a separate row, so the open row does not close with this PR.

### What is not yet decided

- **Curved edges and the joint turn**: (C). There is no truth yet, and they stay at the middle.
- **Not yet measured:**
  - a closed-form bound from above (completeness for Zeros only);
  - the symbolic lanes' cost of the curve read;
  - a NaN derivative on the `Dual` lane's bound from above on covered faces. If that is real, Zeros on `Dual` refuse until the cover read is fixed. The text is unchanged either way.

### Order in the tree

1. Every levered reading decided against both bounds; a typed "face not built yet" case; covers read lazily, once per body state. No README change, and no verdict moves at finite bounds on today's site.
2. This PR: (A), (B), (C) with the line site and its curve-reading bound from below, merged together.
3. Curved edges and the joint turn, once their truths exist.

Confidence:
- recommendation: likely;
- (B)'s inequality: sure;
- "no served verdict past the truth": likely (the truth on ruled faces is proved on paper and is not machine-checked; a hole at a cone's apex is guarded from the 1e-6 runs on);
- the moved classes: sure as counted;
- the `Dual` question: unsure.

## For the orchestrator

- **Before step 1 lands (PR 7140's condition):**
  - The site mint's bound from above must be read off held geometry; the pole-cap turns refuse until it is.
  - The contact-edge corpus's must-carry doors must decide once a face is described. The 24 readings come from a placeholder seed, Zero at the bound from below.
  - The `Dual` NaN must be attributed (188 of 202 sampled covered reads).

  Any of these left unfixed goes to Ev as a newly refusing class.
- **Before the `[ev]` PR goes up:**
  - the full middle-site baseline for the 1e-12 sweep (its columns compare 1,350 of 3,825 tests);
  - config B, for the record, not as a gate on (B).
- **File, as a row beside this fork:** `tangent_normal_parallel` reads `Margin::levered_inv(sin θ, |κ_rel|)`, an arm of 1/|κ_rel| the reach never caps (per the g2 note; only `named_by_parallelism` uses the folded arm). Put it on certification's slate, and record the extrude-table finding on the open honeone row.
- **The 80 reads where the bound from above fell below the truth:** the g1 note attributes the 46 it sampled to `certify_rebased_run` reading an arc 1.0–2.7 m off its face's plane before the move lands, which puts the in-plane truth outside its premise. Nothing to file. The 34 unsampled reads sit in the same 16 tests and should be spot-checked once.
- **Unchecked by me:**
  - The ruled-face convexity argument, beyond the apex hole the lane found. A machine check against an interior maximiser on a few fixtures would make "no served verdict past the truth" independent.
  - Whether the turns that the 1e-9 and 1e-12 runs carried as "truth" before the apex guard include any apex-interior cone read with a served verdict. The g2 note says none depended on it, because config A's bound from above is the cover's.
- **Instrument artefacts to drop before any PR** (they are measurement code):
  - the probe suite missing from `all.rs`;
  - the `join.rs` shadow edits.
