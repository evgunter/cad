**Folding the points into the bound from above is not part of the design I sign.** It is the instrument's choice, and the build should not copy it silently. The design agreed in round 3 orders the two bounds only within ε. A larger inversion, where the cover misses a point known to lie on the region, becomes the typed "no bound from above" case: it is counted and filed at its source, not hidden by a max.

## For Ev

**Recommendation (likely): approve three passages of `crates/geom-core/README.md` ("Float rounding").** With them, the code reads a straight edge's dihedral lever from the edge's line, decided against both bounds.
- This is one `[ev]` PR: the text and the line-site code merge together.
- The one change before it, deciding every levered reading against both bounds, lands first and changes no README text.

Terms:
- **Lever:** the length at which a tilt displaces the geometry it governs.
- **Bound from below:** a length the region certainly reaches, read off points on it.
- **Bound from above:** a length the region certainly does not pass, read off its cover.
- **Site:** what the lever's distance is measured from.
- **Straddle:** one reading that is Zero at one bound and definite at the other.
- **Truth:** the exact lever, where one is known. It is known for plane, cylinder and cone faces bounded by lines and conics, because distance from a line is convex and peaks on the face's boundary.

### The passages

**(A)** In the lever bullet, after "no longer than the site's own measure of it:"

> a lever read off a consumed extent is known from both sides, each measured from the site its datum is read at: from below off points known to lie on the region and from above off its cover. A verdict is served only where it holds at every length between the two: for a reading multiplied by its lever, a definite sign at the bound from below and a Zero at the bound from above. Where the two disagree the reading is in band and is refused as one.

**(B)** In the consumed-extent bullet, replacing "Its lever is the extent's far distance from that point" through "holds its region, not a lever":

> Its lever is the extent's far distance from the site its datum is read at; the least over all sites is not owed. A tilt moves a point of the extent by the tilt times that point's distance from the line it turns about, which is never more than its distance from any point of that line. So a lever read from a point proves a Zero, and a definite sign of a datum that turns about a known line is levered from that line: a dihedral along a line edge, from the edge's line. A record that verdicts read from different sites holds its region, not a lever.

**(C)** Under `## Open`, replacing "Nothing on this page.":

> Which site a dihedral along a curved edge is levered from (its station's tangent line or the edge's carrier), and the site and region of the joint turn at a pole; both are read from the span's middle.

### Why

- **Choosing a bound per caller never changed a verdict.** At today's site, the span's middle, the two bounds agreed in all of about 282 M paired decisions. The defect was the site.
- **The middle over-reads the lever, so definite signs from below are unsound there (sure).** On edges with an exact truth, the middle's bound from below exceeded the truth on 99.5% of reads. It counted the faces' run along the edge, which a dihedral does not displace.
- **Your PR 6968 question.** You asked how the midpoint could be sound when it is no witness. The answer approved there ("any pivot is sound") holds for a Zero only, and (B) says so.
- **At the line, with both bounds decided, no served verdict differs from the truth's, at any ε run (likely).**
  - On exact reads the truth equals the bound from below, so this check is partly circular.
  - What it does show: wherever a Zero is served, the bound from above agrees with the truth.
- **The cover as a bound from above is checked independently only in part.**
  - In the runs so far, the bound from above also folds in the points of the bound from below. Its ordering against the bound from below therefore holds by construction and says nothing about the covers.
  - On plane faces bounded by straight edges, the truth is the farthest vertex, which is folded in. So "never below the truth" there is by construction too.
  - **The independent evidence is the first run's arc-bounded plane faces.** Over 1.39 M reads the cover was at or above the in-plane truth except on 80. The 46 of those examined are an arc standing 1.0–2.7 m off its face's plane before a move lands, so the in-plane truth's premise does not hold there. The other 34 sit in the same tests and were not examined one by one.
  - A recount of the cover's own far distance, unfolded, against both the bound from below and the truth is queued. It covers ruled faces, where nothing independent exists yet.
- **What the line site needs, and gets in its code.** The bound from below reads each boundary edge's farthest point from the line: a line's ends, a circle's or ellipse's critical points, 17 samples otherwise. With vertices alone, a face whose corners sit on the line read a lever of about 0, and many valid bodies refused.

### What moves, against the truth (valid bodies included, as you asked on PR 7140)

- **Faces of no width now refuse, as they should** (20 tests at 1e-9).
  - Every vertex lies on the edge's line, so the true lever is 0. The middle read them as wide.
  - The tests pin the old refusal and are rewritten.
- **Faces narrower than the band now refuse.**
  - Two bisector fixtures with a true width of 5.0e-9 and 5.3e-9 at ε 1e-9, where the middle read 1e-4.
  - A tilted-sketch sliver with a true width of 1.5e-9.
  - The truth confirms each, and they build at 1e-12. These verdicts move toward refusing because the lever is now read honestly.
- **One served verdict moves.** A keel near-tangent to a bore's wall now reads Zero instead of Negative, on a lever of 0.0671 that the truth confirms; the middle read 1.002. The test still passes.
- **Two classes of valid bodies refuse only because a bound from above is missing.** Neither is a site question.
  - A plane cutting a sphere's pole cap (7 turns): the site mint returns no bound from above, by construction.
  - The contact-edge corpus (24 must-carry readings): a face still a placeholder seed mid-construction.
  - Both are fixed in code before the first change lands. The site mint's cover is read off the geometry the door holds, and those doors decide once the face has a surface. Either one that cannot be done comes back to you as a newly refusing class.
- **The open row on a face's run along its edge** (an extrude profile: a line into an arc, φ off tangent).
  - The wall join now reads the true lever, 0.05, and the declared tangency reads smooth. The sharp edge tier 3 passed at 15ε and 19ε is gone.
  - What now refuses is certification's normal-parallelism check, levered at the arc's 1 m curvature arm, which nothing caps at the region's reach. That is a separate row, so the open row does not close with this PR.

### What is not yet decided

- **Curved edges and the joint turn:** (C). There is no truth yet, and they stay at the middle.
- **Not yet measured:**
  - the cover's own bound from above, unfolded, against the truth (queued);
  - a closed-form bound from above (completeness for Zeros only);
  - the symbolic lanes' cost of the curve read;
  - a NaN derivative on the `Dual` lane's bound from above on covered faces. If that is real, Zeros on `Dual` refuse until the cover read is fixed. The text is unchanged either way.

### Order in the tree

1. Every levered reading decided against both bounds, a typed "face not built yet" case, and covers read lazily once per body state. The bounds are ordered within ε, and a larger inversion is the typed case, counted. No README change, and no verdict moves at finite bounds at today's site.
2. This PR: (A), (B), (C) with the line site and its curve-reading bound from below, merged together.
3. Curved edges and the joint turn, once their truths exist.

Confidence:
- the recommendation: likely;
- (B)'s inequality: sure;
- "no served verdict past the truth": likely (the ruled-face truth is proved on paper; the apex hole is guarded from 1e-6 on);
- the cover as a bound from above: likely on arc-bounded planes, unmeasured on ruled faces until the recount;
- the moved classes: sure as counted;
- the `Dual` question: unsure.

## For the orchestrator (changes only)

- **New gate before the `[ev]` PR goes up: the queued unfolded recount** of the cover's far distance against the bound from below and against the truth, on plane, arc and ruled classes.
  - An unexplained cover below the truth is a soundness defect in `topo::cover`, and it gates the line-site code.
  - An inversion against the bound from below that is larger than ε belongs in the typed case's count.
- **The build must not copy the instrument's `max(lo)` fold.** The constructor orders the bounds within ε and routes a larger inversion to the typed case; this was agreed in round 3.
- **m-e's and g1's straight-plane "bound from above never below the truth" also folded the points**, so on that class it holds by construction. Do not cite it as evidence on covers.
