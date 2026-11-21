## For Ev

**What changed since you last read.** A conic stored at its centre is now read exactly as main reads it today, from its centre. The only new thing in the kernel is a second placement: a conic may instead be stored about a point of its arc. Which one it uses is fixed by how it was built. No size decides it, and a wrongly chosen one is a kernel bug that is closed in code.

**What is asked:**
- approve the placement family;
- approve the centre-held conic staying as main has it;
- approve the sentence below.

A possible later refinement for large centre-held conics is not part of this decision. I expect it not to pay (below).

**Why the reading went back to main's.** A conic stored at its centre is the curve about that centre and those directions. Every reading is judged against that curve.
- On a large conic its numbers fix a point of the arc only to about u·a, where u is one rounding (about 1.1e-16) and a the semi-axis: about 2e-12 m at 10 km.
- That limit comes from two places:
  - the rounding of the centre's distance;
  - the rounding of the stored directions.
- The faster "chord" reading answered for a nearby, slightly different circle, and gave wrong answers. Main's centre sum gave none: it refuses typed where the conic cannot answer.

**The refinement, and why I expect it not to pay.** It would sum the seam vertex exactly. That removes the rounding of the centre's distance, but not that of the directions:
- `Interval` reads a frame as the nearest exact rotation, about one rounding per entry from the stored numbers (PR 7131);
- on a tilted frame that difference, times the semi-axis, is again u·a near the arc;
- so on a large tilted centre-held conic, the stored numbers and that rotation are two candidate truths u·a apart, and an exact vertex is exact for only one of them.

It could earn a place only if a probe on tilted frames, judged against both truths, shows no wrong answer and recovers the near-graze poses. Only an arc-held conic is free of this, because its directions are multiplied only by the short distance from its own point.

**What a person who authors a large circle by its centre gets:**
- the curve about the centre they wrote;
- never a wrong answer in anything measured;
- typed refusals near nearly tangent crossings, more of them as the circle grows, starting between 500 and 900 m;
- the recourse: write the arc by points on it, or loosen the tolerance.

**Sentence** for "Float rounding", the proposed wording with three words added:
> "A stored form that loses digits is re-minted in a well-conditioned form: a conic is one of a family of placements behind the one `Conic` reading, held about its centre or about a point of the arc it serves with its parameter zero there; its construction fixes which, the same at every size, and a construction whose centre runs off at a pose it can reach holds the arc. A conic is the curve its stored point states, and no reading replaces it with a rounded copy: one held at its centre is read from its centre, and so fixes a point of its arc only to the rounding of the centre's distance and directions. Every reading of where a conic is goes through the conic."

**One correction to "main's, unchanged".** That holds for the evaluator. What changes:
- readers that rebuild the curve from its centre now go through the conic;
- main's cover on the 1e7 ellipse misses the stored curve's true arc by 4.2e-10 m and has to be fixed against that curve;
- scaling (`homothety`) maps the stored point.

**Confidence:**
- the decision as asked: likely;
- that the exact vertex cannot beat u·a on a tilted large frame against the polar truth: likely, by arithmetic and not measured.

## For the orchestrator

**(a) I sign (1)–(3), with two amendments.**
- Add "and directions" to (3). Without it the sentence claims the stored point is the only limit, and the frame argument shows it is not.
- Qualify (1)'s "unchanged" as above, so Ev is not surprised by cover and reader rows landing under the same decision.

Neither amendment changes the decision.

**(b) My answer to the frame objection, as one who proposed the exact vertex.** The objection holds, and I concede it.
- **What is the f64 truth?** `[simple.conditioned]` reads f64 values "as if exact", which makes the stored entries the f64 truth.
  - PR 7131's decision 1 (open) says entries such as (0.6, 0.8, 0) "would claim a rigid motion that does not exist" and makes the polar factor the truth ("the true shape of a stored body is then its carrier on that frame").
  - So the two lanes judge two curves.
  - For a conic of box size they differ by at most u·box, under the admission floor, and nothing notices.
  - For a large centre-held conic they differ by u·a near its arc, the same size as the centre's rounding.
- **Does the exact vertex survive a tilted frame?** Only against the entries.
  - Against the polar factor it still carries w·a, where w is one ulp per frame entry, and its meter does not charge that.
  - So it could certify for one truth while being wrong for the other. Main's centre sum charges u·a-sized rounding per sample, which happens to cover both.
- **What the probe must be judged against:** both truths on tilted frames, and the lane must say which oracle it used.
  - If the probe's doors are all axis-aligned, the probe cannot tell the two readings apart. My round-5 claim was made without that check.
  - Arc-held conics are immune: the directions multiply only the distance from the stored point.
- **What follows:** the "possible later improvement" should be framed as conditional on Ev ruling that the f64 truth is the stored entries, or on the tilted probe being clean against the polar factor. The second I expect to fail.

**(c) The other two reports, one line each**
- **First report:** "the curve read is then the stored centre's own, to the rounding of in-box numbers" is true only against the stored entries. On a tilted frame it misses the u·a frame term, as the second report says.
- **First report:** it rejects charged chords as against "re-posing, never charging". But the centre sum it falls back to certifies through a running meter, which is a charge. The real argument against charged chords is the measured lever of about 1000, not that sentence.
- **Second report:** right on the frame, on phase 1 (a) needing re-judging (its 1.28e-13 bound suggests the twin oracle), and on the panic needing no run-time number. Its "if Ev still wants one" edge-certificate site, which needs provenance, I would not add either.
- **Second report:** the size-table column says "from about 500 m", but the first loss is at 900 m (33 of 40), so "between 500 and 900 m". I have used that.

**Still owed (unchanged):**
- the nine unmetered far-term readers, on bodies stored through real doors;
- a profile row for point-written sketch arcs. `Arc2` holds only a centre, so a flat `Bulge` arc cannot reach the arc-held placement. Not filed, and it is the main way a person gets a large conic that certifies.

**Housekeeping:** I read only the round-6 message and the two reports; nothing else in that directory, nothing edited, nothing run.
