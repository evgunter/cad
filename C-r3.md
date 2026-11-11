## For Ev

**Final position.**
1. **Default side: the kept side, as main has it.** Not Out. I built a pose where they differ and Out refuses.
2. **Shared vertex: a per-pair choice of the clear side.** The one-family nesting placement is not needed by any pose that exists today.
3. **Turning: not needed as a separate step.** The per-pair choice covers it.

**Recommendation (likely).** One rule:
- A vertex pair is paired from the start whose runs lie on the side the op keeps (main's start).
- At a shared vertex, a pair whose runs on that side hold another pair's cut takes the other side, if that side is clear. For a pair with two germs "side" means which way round its one null edge runs.
- Where neither side is clear, `SharedVertexCrossings` stays, typed.
- The finish's vertex-set check stays retired.

**What moved since round 2.**
- **On 1, I hold the rule and drop my reason for it.** I said coincident regions are coded as the dropped side, so kept arcs are the real sectors. That is false for a difference: there the glued sliver is coded as the kept side (measured below). The other designer's reason is the right one: a kept arc is a stretch of one cone of the result, so a null edge along it joins two germs of one cone.
- **On 3, I move.** In round 2 I kept main's turning loop. A pair's clearance depends on where the other pairs' germs lie, not on which way their runs go. So each pair chooses once and nothing needs re-reading.

**Terms.**
- *Vertex pair, shared vertex, germ, null edge, run, start, clear, code*: as in rounds 1 and 2.
- *Kept side*: the side of a solid's boundary the op keeps. Out for both operands of a union, In for both of an intersection, Out of A and In of B for a difference.
- *Rim-own pairing*: each closed rim's two germs joined to each other. *Cross pairing*: each rim's germ joined to the other rim's germ in the same direction.

**1. Is Out equal to kept? Where do they differ?**
- They are equal for a union and for the first operand of a difference. They differ only for an intersection.
- On everything measured before this round they never differed in outcome. The family-2 file has four-germ pairs only in unions (164 plans, all `Union`). The battery's intersections built under either start (round 1).
- **A pose where they differ.** I built the mirror image of family 2:
  - A is a block with a blind pocket of radius 0.3.
  - B is a slab with a pin of radius 0.1 hanging under it. The pin's rim is tangent inside the pocket's rim, with both rims' vertices at the touch.
  - The pocket's floor and the slab's underside coincide on the crescent between the rims.

| op, one-segment rims | kept start (main) | Out start | In start |
|---|---|---|---|
| A ∩ B, B ∩ A | builds, volume right | refuses `UnpairedLooseEnds` | builds (same as kept) |
| A ∖ B, B ∖ A | refuses `UnpairedLooseEnds` | refuses (same as kept) | refuses `SingleSiteSectionLoop` |
| A ∪ B, B ∪ A | builds, sound | builds | builds |

- **Out is wrong for the intersection.** The glued crescent is coded Out there (dropped), so Out pairs across the slivers, and the join refuses. The kept side pairs each rim with itself.
- **Neither is right for the difference, and that is not the pairing's fault.** In a difference the crescent of the pocket's floor is truly on the result, with a cusp at the touch. The kept arcs are the slivers, so the kept side gives the cross pairing, which is the correct one by meaning. The join's lane for a closed one-vertex rim cannot build it. Main refuses this today. With two-arc rims the same difference builds sound.
- So the join's lane, not the pairing rule, decides these poses. Kept is the rule with a reason; it is also what main does, so it changes no pose outside shared vertices.

**2. Is the nesting placement needed today?**
- **No pose with a witness needs it.** The per-pair choice builds the battery (5 190), the cleave witness and both families, measured by two of us independently, with no run nested across pairs.
- It is needed only where no region of the other operand touches all of its cones at the point: three nested cones, or a shell beside a separate piece. Nobody has a witness.
- **It is needed by the other designer's pairing, not by the poses.** Pairing along the partner's kept arcs at a shared vertex makes runs that sit inside other pairs' runs on the battery lines themselves. That report says those lines build only once the placement exists. It is unbuilt and unmeasured.
- **What that proposal gets right.** With the per-pair choice, a null edge at a shared vertex can join germs of two cones, and the zip's cone split sorts it out. That is measured sound on 5 190 lines. It would fail at the join if a closed one-vertex rim passed through a shared vertex. No such pose exists. If one appears, the kept-arc pairing with placement is the direction to go.
- The per-pair choice does not block it.

**3. Is turning still needed?**
- For a pair with two germs, turning and "take the clear side" are the same act.
- For a pair with four or more, main can turn one run alone, which then holds the pair's other runs. I know no pose that needs this. The build branch has no turning, and the per-pair prototype on it passes the ci profiles of topo, sweep and editor-core, the battery, and both families.
- So the loop and the re-nesting it feeds go. Not run without them: the slow set outside the family-2 file. That is the condition on this answer.

**Ratified text.** None changes.

**Confidence.**
- Out refuses an intersection the kept side builds: **sure** (measured on the pose above).
- The kept side is the right default: **likely**. It has a reason and no counterexample, but the difference row above shows a pose where even it does not build.
- Nesting is not needed today: **sure** on what is measured; **likely** overall.
- Turning can go: **likely**, on the other designer's measurement plus the independence argument. I did not run it myself.

## For the orchestrator

**Measured this round** (main `1a13ef4bf9a`, debug, my round-1 probe plus a trace and one added test).
- `a_closed_conic_in_the_partners_face`, 22 tests, traced: 22 pass; 164 four-germ plans, every one `op=Union`. So that file cannot separate Out from kept.
- `svc_c_dual_pin_in_pocket` (the added test), under main's start, `SVC_C_START=out` and `=in`: the table above. Two-arc rims: unions and differences sound under main and Out; the difference under In hits the old finish check (`SVC_C_RELAX` was off).

**Two defects on main, off the question.** Both show in the added test with no probe switch set.
- **A ∩ B and B ∩ A build `OK BAD`: `t3p=false`**, volume right (1.874336294). One-segment and two-arc rims alike, under every start, so it is not the pairing. The result is the holed plate and the pin touching at one point; I expect a missing vertex-vertex record but did not look. This is a wrong result passing the door, so it wants a row at high priority. Check my operands first: they are themselves built by unions in the test.
- **A ∖ B and B ∖ A refuse `UnpairedLooseEnds { count: 4 }`** with one-segment rims; two arcs build. The join's one-site lane cannot take the cross pairing. This is the row I suggested in round 2, now with a witness on main.

**Not measured.**
- The per-pair choice without the turning loop. I rely on designer A's runs for question 3.
- The other designer's kept-arc pairing probe (`B-r2-probe-R.patch`). I did not run it; my answer to 2 rests on that report's own statement that its lines need the placement.
- The battery this round.

**Note on the lead.** With the kept side as default, the choice at a shared vertex is still best read in the shared solid's walk, as the build has it. Nothing I measured separates that from A-consecutive.

**Process.** Clone made as the setup says; every git command by `-C`. Debug builds, foreground. Clone and target deleted. Nothing pushed.

**Files.**
- `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/fork-svc-c-r3-probe.patch` — probe, trace and the added test, on main
- `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/fork-svc-c-r3-dual-main.log`
- `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/fork-svc-c-r3-dual-out.log`
- `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/fork-svc-c-r3-dual-in.log`
- `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/fork-svc-c-r3-main.log` — the 22 tests, traced
