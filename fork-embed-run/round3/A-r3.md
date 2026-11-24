## For Ev

**My recommendation is unchanged in substance and sharpened in form.** Read one number per pair: the wedge's sine (never more than one) times the narrower face's **near run, capped at its far run**. That single reading carries both facts the other designer names, so the question needs no second reading.

**Terms.**
- *Band*: K·ε.
- *Pair*: two faces that share an edge or vertex.
- *Far run*: the farthest any point of a face stands from the carrier of what the two faces share. This is what is built today.
- *Near run*: how far a face runs out from what it shares before any part of its boundary that does not meet the shared edge or vertex (judged by key, never by distance) cuts in. Where nothing cuts in, it is the far run.
- *Stranger boundary*: the part of a face's boundary that does not meet what the pair shares, by key.

**Where the two designers agree (sure).**
- **The promise:** a certified solid's boundary comes within the band of itself only beside a cell its faces share, and only between the faces and edges that meet there.
- **The rate:** the sine of the wedge, never more than one, replacing the summed rates.
- **Verdicts:**
  - case 1 (flat strip) and case 2 (strip with an ear) refuse;
  - case 3 (sector sliver) is certified, at 10 m and at 1 m;
  - case 4 (0.6·K·ε strip at 2.6 rad, 0.8·K·ε strip at a right angle) refuses.
- **The tree:**
  - a certified cell is discharged whole;
  - a third face that shares nothing with the neighbour is read by distance on every surface kind;
  - the census reads vertex against face only on planes.

**One reading, not two (sure, given the cap).**
- With the lever capped at the far run, near ≤ far. So whenever the near-run reading passes, the far-run reading ("the faces are distinct") passes too.
- The other designer's lens, two faces sharing two edges with nothing cutting in, reads its far run under the fallback and refuses exactly as their first fact would.
- The cap matters. Measured from the edge's span, a stranger vertex past the edge's end can stand farther away than the far run. Without the cap the near run could read longer than the face's real reach.
- So no body is refused by the second reading that the capped near run certifies. I know of none, and the inequality rules it out for any body where the cap holds.

**The second fact as a final state: a kept length or a rule on discharge.**
- **(i) Kept near run** (recommended, likely).
  - Tier 3 keeps one length per face and edge (and per vertex shared alone), in the slots that exist today. Only the recipe changes: the least distance to each stranger edge and vertex, read from below over the whole cover.
  - A refusal tells its reader: this face extends only *d* from the shared edge before its boundary cuts in; at this wedge, that puts it within the band. The recourse is to open the wedge or move that boundary away.
  - It leaves possible a rare refusal of a valid body, where the nearest stranger part stands beside the shared edge but past the end of the other face. That is the safe direction.
- **(ii) Far run kept, plus a rule that a certified cell may not discharge a stranger boundary part, which is then read by distance in the walk.**
  - It is exact where (i) is a lower bound. Its refusal names the edge and its distance: "the strip's far edge stands 5e-12 from the plate".
  - Its costs: two readings, a new edge-against-face distance arm inside touching pairs, and the question re-reading tier 3's angle fact.
- **The curved edge that leaves the shared edge's end vertex and dips back beside it does not separate them (likely, against the other designer's claim).**
  - Under (ii), "part" must be judged by key, like the near run. If it were judged per cell, it would refuse the sector sliver.
  - Its side edges lie within the band of the other face for about 20 mm. The sliver certifies today only because the cell holding the tip edge is discharged by departure before any cell clear of the tip edge is made. A cell clear of it would be read by distance and refuse.
  - Judged by key, the dip-back edge meets the shared vertex and is excluded under both forms.
  - It is caught, if at all, by the pair the dip-back edge's other face makes with the neighbour, away from their shared vertex, read by distance.
- **One near-touch is never reported twice.** The injectivity question ends at its first finding (`clearance.rs` module docs), so that cost does not exist under either form.

**Is the wedge's reality the question's to read?**
- No. It is tier 3's class, and "decides none of it" stands (likely).
- Under (i), the question never decides it: it reads a stronger condition that implies it.
- Under (ii), the first fact *is* check 4's reading, decided a second time.
- Check 4 should read the capped sine at the lesser far run. Today it levers at the farther face's run, which is why the flat strip passes tier 3. That is the other program's row; under (i) the question is right either way.

**The rule under both readings.** "Passes by more than the band" means: every body whose geometry moves by less than the band, keeping its topology, stays embedded. Both readings derive from it:
- the tilt uncertainty (band ÷ far run) gives the first fact;
- the hinge's sideways uncertainty (band ÷ sine) set against the near run gives the second.

It holds of (i) to first order, with K as the margin (likely). I would put it in the PR body as the argument, **not** into ratified text. It is a first-order statement, and the band's meaning is already ratified in `[tolerance.recourse]`. Writing a looser second statement of it beside the first is the thing `[tolerance.recourse]` exists to prevent. Whether you want it written down anyway is your call.

**Which door owns "pieces that do not meet are more than K·ε apart" for a face's own boundary against a face it shares a cell with.**
- It is part of this answer: the question owns it, for every surface kind, through the lever.
- The census also reads it for planar cells, as a coincidence census. Whether that pass is then redundant is housekeeping for a work row, not a question for you (likely).

**The sentence I would put to you.** In `[topology.tiers]`, replace "Over each cell it reads the rate … passes by more than the band" with:

> Over each cell beside what two faces share it reads the sine of the wedge between them, never more than one, levered at how far the narrower of them runs out from what they share before any part of its boundary that does not meet what they share cuts in, and never farther than it stands from that at all ([tolerance.epsilon]), never at a cell of the question's own subdivision, and that passes by more than the band. So the two faces are distinct there, and a solid's boundary comes within the band of itself only beside a cell its faces share, between the faces and edges that meet there.

Provenance: the current sentence was written by designers (commit `b130a7bbd5`). You approved it by comment on PR 7456, with a stated doubt on exactly this lever.

**The other designer can likely sign this** (likely):
- it keeps their promise and their case verdicts;
- it is their own "one kept number that gives both at once";
- the cap answers their lens.

If a difference remains, it is one question for you: **should the question keep one lower-bound length per face (one reading, may rarely refuse a valid corner), or read stranger boundary by distance in the walk (exact, two readings, a new arm)?** I lean to the first.

**What each case reads** (ε = 1e-9, K = 10):

| case | reading | verdict |
|---|---|---|
| 1 flat strip | 5e-5 × 1e-7 = 5e-12 | refuses |
| 2 strip with an ear | the far edge cuts in at 1e-7; the ear is not reached | **refuses** (certified today) |
| 3 sector sliver, 10 m / 1 m | about 480ε / 50ε | certified |
| 3 sliver, 20ε at its far end | about 2 bands | should certify; today's budget refusal comes from the walk, not the reading |
| 4 strip 0.6·K·ε at 2.6 rad, 0.8·K·ε at a right angle | 0.6 / 0.8 of the band | **refuse** |
| 5 L-shaped face, hole beside the edge | the arm's width, or the distance to the hole's rim, × sine | by that |
| 5 face touching the edge's line again far away | not read by the lever; read by distance in cells clear of the shared edge | by that |
| 6 spline shared edge | near run from the edge's own span; cap from the edge continued past its ends | unmeasured |

**What the measurement lane could overturn.**
- A valid ordinary body (a small chamfer at a moderate wedge, a hexagonal face, a notched shallow wedge) refused under (i): that costs (i) its "rare" claim.
- A body where the near run passes but some stranger part lies in the band: that shows the lower bound fails for curved faces.
- Neither result would revive the far run alone.

## For the orchestrator

**Checked at `273f765de5`.**
- Discharge: `injective.rs` `certify_pair_in` → `departure`; the other designer's reading of `clearance.rs` `pair` (Certified → discharge, return) agrees with mine.
- Non-touching cell pairs are read by distance: `clearance.rs`, `Sweep::run` doc.
- The walk stops at the first finding: the `clearance.rs` module docs, "The injectivity question ends at its first FINDING".
- The fixture geometry is as reported in round 2.

**Not checked.**
- That the sector sliver's 20 mm stretch would refuse if split clear of the tip edge. I reasoned this from the touch rule (a cell whose ball is clear of the shared cell goes to distance), not from a run. The 20ε sliver's `Budget Depth 40` refusal may be exactly this walk effect.
- `sign_on`'s body.
- PR 7456's thread.

**For the implementer under (i).**
- Make the cap explicit: lever = min(near, far).
- "Does not meet" is judged by key: an edge or vertex sharing a vertex with the shared edge, or lying on it.
- Stranger covers are read from below over the whole edge; three stations bound from the unsafe side.

**Brief.** Your summary of my round 2 was right. "Two pairs report one near-touch" is not a cost, because of the first-finding stop.

**Still owed under either answer.** Off the question:
- `parts`' doc comment;
- the unit test pinning the summed rate;
- the fixture doc's "right verdict at the wrong door".
