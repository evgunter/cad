**(1)** Yes, this is enough to ask Ev now. The design run on the sweep is still running; I state it as such and treat it as a gate on the merge, not on the question.
- Every inverted read on the sweep carries a definite sign from below. Under the rule those stand, so the expected change in test outcomes is 0, as it was on core.
- Any change it does show can only add to the list of what refuses; it cannot change a sentence of the text.

**(2)** No change to (A), (B) or (C).
- All four door states share one fact: the face's recorded surface or boundary is not yet its own. Those states are a rebased run before its move lands, an edge certified onto a seed surface, a surface set before its boundary moves, and a boolean's working face.
- "The typed missing bound from above, entered from what the door knows" remains the build's answer for all four.
- None belongs in the README. The text says what a lever is; which doors are mid-construction is code.
- Item B(2) removes the 282 M figure from the evidence. The premise now rests on the arithmetic alone.

## For Ev

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

### The passages

**(A)** In the lever bullet, after "no longer than the site's own measure of it:"

> a lever read off a consumed extent is known from both sides, each measured from the site its datum is read at: from below off points known to lie on the region and from above off its cover. A verdict is served only where it holds at every length between the two: for a reading multiplied by its lever, a definite sign at the bound from below and a Zero at the bound from above. Where the two disagree the reading is in band and is refused as one.

**(B)** In the consumed-extent bullet, replacing "Its lever is the extent's far distance from that point" through "holds its region, not a lever":

> Its lever is the extent's far distance from the site its datum is read at; the least over all sites is not owed. A tilt moves a point of the extent by the tilt times that point's distance from the line it turns about, which is never more than its distance from any point of that line. So a lever read from a point proves a Zero, and a definite sign of a datum that turns about a known line is levered from that line: a dihedral along a line edge, from the edge's line. A record that verdicts read from different sites holds its region, not a lever.

**(C)** Under `## Open`, replacing "Nothing on this page.":

> Which site a dihedral along a curved edge is levered from (its station's tangent line or the edge's carrier), and the site and region of the joint turn at a pole; both are read from the span's middle.

### Why

- **Nobody needs to choose a bound (sure, by arithmetic).** A reading multiplied by its lever grows with the lever. So a definite sign proved at the bound from below holds at every longer length, and a Zero proved at the bound from above holds at every shorter one. Reading both bounds therefore serves exactly what the right single bound would, and refuses the rest. No caller has to be sorted by what it acts on.
- **Today's site over-reads the lever, so a definite sign read there from below is unsound (sure).** On edges with an exact truth, the bound from below read at the span's middle exceeded the truth on 99.5% of reads. It counted the faces' run along the edge, which a dihedral does not displace.
- **Your PR 6968 question.** You asked how the midpoint could be sound when it is no witness. The answer approved there ("any pivot is sound") holds for a Zero only, and (B) says so.

### Evidence at the line site

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

### What moves, against the truth (valid bodies included, per PR 7140)

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

### What is not yet decided

- **Curved edges and the joint turn: (C).** No truth exists yet, and they stay at the middle.
- **Still running; these gate the merge, not the question:**
  - the rule run on the sweep;
  - the recount at 1e-12 and 1e-6;
  - the symbolic lanes' cost of the curve read;
  - a machine check of the ruled-face truth against an interior maximiser.

  Any of them can only add to what refuses, or find a cover defect to fix before merge. The text stays.

### Order in the tree

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

## For the orchestrator (changes only)

- **Retire as evidence:** the "282 M paired decisions agree" figure, since the middle door folds too, and the first-run arc figures. Cite only the unfolded recount's tables.
- **The typed case must be entered from what each door knows,** at all four doors: `euler::certify_rebased_run`, `euler::certify_edge_spec`, `attach::set_face_surfaces_certifying`, and the boolean graft's working face. The inversion check stays as a backstop and counts what slips past.
  - The definite signs served at those reads (560 `tangent_second_order` Positive at the graft, 4 `classify_dihedral` Transverse at a rebased run) stand from below. Tier 3 re-asks them at rest.
  - State that in step 1's PR body, as the flow that unit now records: "decided mid-construction, re-asked at rest".
- **The 66 fewer verdicts on core** are attributed to fuzz seeds; nothing remains open there.
- **Unchanged gates on the merge:** the rule run on the sweep, the 1e-12 and 1e-6 recounts, the symbolic-lane cost, and the interior-maximiser check.
