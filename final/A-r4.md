## For Ev

**I sign `joint.md` (J1 to J6) as it stands.** It asks you two things:
- **(a) The ask ends at its first finding.** The embedding ask should end at its first finding, a verified crossing or a refusal, and report that one finding. I recommend yes (likely).
- **(b) Rule the later rule now, conditionally.** I recommend ruling J3 now, conditional on its measurement. Once the engine learns cheaply that a place cannot be read (J2), and if a refusing ask then costs about what the same body costs when it holds, the ask stops ending at a refusal. It ends at its first crossing; with no crossing, it refuses and lists every refusal it met, once each, by place. Ruling it now means the later change to the text is the consequence of a decision you already made, not a second decision. If you would rather see the numbers first, asking you again then also works.

**Terms**
- **Ask:** the embedding question put to one solid: is its boundary embedded?
- **Cell pair:** a patch of one face against a patch of another. The engine halves cells (a **split**) until each pair is decided.
- **Touching cell pairs** share a point of the boundary. They are decided by a **local certificate**: how the two faces leave the edge or vertex they share. The rest are decided by distance.
- **Refusal:** a cell pair the engine could not decide. It names its faces and its place: the shared edge or vertex, and which certificate failed.
- **Crossing:** a verified pair of points where the boundary meets itself.
- **Depth floor:** 40 halvings, where `f64` stops carrying information.

**Why (a) now.** Today a refusing ask keeps the first refusal it met and walks on to the end, which is the cost of every refusal after it, and then reports one. It pays for many findings and keeps one. Stopping at the first finding keeps everything the ask found: "every finding kept, none folded" (your words, PR 6326). The answer stays one of three verdicts per ask: embedded, crosses, or refused. ERROR-DESIGN E7 is unchanged. The stop applies to the embedding question alone; the editor's clearance assertion keeps today's rule.

**Why J3 is the right rule later.** "Embedded" means "every pair of faces is apart". Read with three values (apart, crosses, unread), one crossing settles that "every" as no, whatever else is unread. A refusal settles nothing, because another pair may still cross. So the logic of the question is: stop at a crossing, never at a refusal (sure). Stopping at a refusal is the price of refusals being slow to reach today, not the right rule. What the person loses under (a) is a crossing reported as "the kernel cannot read this". A crossing has a lever the person can pull; an unread place often does not. Some refusals are the model's to move: a separation inside the band, a torus that is not a ring, a parameter box too wide at `Interval`. The next finish reports those.

**Worked example.** A fillet T (a torus face) meets a plate P along an edge whose corner no certificate reads. Separately, a rib R passes through a wall W.
- **Today:** the cells around the corner are halved again and again, each failing as its parent did, for 150 to 564 s. The walk then reaches the rib, and the person is told "R and W cross here". Without the rib the walk would end, after that time, in "T and P: unread at the corner".
- **Under (a):** "T and P: unread at the corner", in about a third of a second, if the walk meets the corner before the rib. Nothing about the rib.
- **Under J3, once J2 is built:** the corner cells refuse a few splits in, the walk goes on, and the person is told "R and W cross here". Without the rib: "T and P: unread at the corner".

**The price, measured.** The gate lane measured 372 refusing asks: the 13 behind the twelve dearest tests, 153 sampled by refusal kind (cheapest first), and 206 from the 22 tests whose asks found a crossing after a refusal.
- **The dearest:** 13 asks, 4,434 s. Each refuses first within 3 splits, and then 4 to 21 other face pairs refuse in turn. 3,250 s go by before each of those reaches its own first refusal. Stopping the ask leaves 4.6 s; settling each face pair separately would leave about 3,280 s.
- **The cheap asks:** the first refusal comes at the depth floor, 71 to 93% of the time comes before it, and stopping saves 6 to 37%.
- **Whole suite:** of 11,434 s spent on refusing asks, stopping should remove roughly 5,000 to 5,600 s. That is extrapolated from this sample, not measured over the suite.
- **What (a) loses:** 38 asks, in 22 tests, found a crossing after a refusal. Under (a) all 38 are reported as refusals. The suite has several thousand asks that cross (8,727 in the first census).
- **J2's saving:** not measured yet. It aims at the time spent before refusals, which no stopping rule reaches.

**The engine defect (J2; likely; a unit to file, not a ruling).** When a local certificate fails, it answers "split, a smaller cell may certify" with no ground for that claim. So the engine learns that no cell of any size will certify only by reaching the depth floor or the budget. In the final state:
- A split carries its ground: what a smaller cell would reduce.
- A failure with no ground refuses that cell at once, typed.

This is `[one-way.readings]` ("an unlisted arm refuses typed, naming what it lacks") and `[topology.tiers]` ("a reading inside the band refuses as ambiguous"), reached without exhausting the walk. A budget refusal also gains the faces of the cell pair it was refused on.

**Dropped by both reports:**
- one finding per face pair as the shape of the answer;
- settling a face pair at its first finding: in 19 of the 38 the crossing lay in the pair that refused first;
- any change to E7;
- a second, smaller budget;
- a probe-only search after a refusal: it has no natural end near a shared edge.

**Ratified text.**
- **Now,** in `[topology.tiers]` after "one fact per pair of faces": "The embedding question ends at its first finding, a verified crossing or a refusal, and that is its finding."
- **Under J3, once measured:** "The embedding question ends at its first verified crossing; without one, any refusal refuses it, and every refusal it met is listed, each once, by its place." If you rule (b) now, that change lands with the unit that meets the condition.

**Confidence**
- (a): sure on the measured numbers, likely as the rule for now.
- J3 as the rule the logic asks for: sure. That J2 makes it affordable: unsure until measured.
- J2 as the cause of the 3,250 s: likely. It fits the code and the tracker's rows, and nobody has traced it cell by cell.

## For the orchestrator

**New since round 3**
- I sign J1 to J6 unamended. The question my round 3 put to Ev (after a crossing, stop or read on and list) no longer has anyone on its other side; it is J4, and I have dropped it from the Ev section.
- I accept the other report's correction that not every refusal has the kernel's recourse: `Sliver`, `TorusNotRing`, a parameter box too wide at `Interval`, and some `Budget(Depth)` refusals are the model's. I have reflected it above.
- I accept the other report's correction of the "28%". The population is named above, and no percentage of crossing asks is quoted.

**My lean on (b):** rule J3 now, conditionally. The stop rule J3 states follows from the three-valued logic of the question, so only its affordability waits on a measurement. Per CLAUDE.md, wording that follows from an approved change lands with that change, so a conditional ruling avoids a second `[ev]` round on the text. The condition must stay out of `DESIGN.md` ("no status scaffolding"). It lives in the PR body and in the J2 unit's row.

**Measurements:** unchanged from J2 and J3. M5/M6 (the grounds for each `Split` on the 13 dearest asks and a sample from the depth floor), then, on a J2 prototype, the cost of not breaking at a refusal and whether the 38 are still found.

**Not read:** `draw.md`.
