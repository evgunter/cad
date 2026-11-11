## For Ev

**Recommendation (likely).** Keep the pairing start main has. Change the pairing only where main refuses: at a shared vertex, a pair whose runs cannot go either way round clear of the other pairs' cuts is paired from the other start. Keep the run turning. Retire the finish's vertex-set check, as agreed in round 1.

This builds everything main builds, by construction, and the 982 lines and the cleave witness besides.

| what ran | main | built rule (PR 6832) | this recommendation |
|---|---|---|---|
| battery, 5 190 lines | 4 208 | 5 190 | 5 190 |
| cleave witness, 6 ops | 3 | 6 | 6 |
| family 1, 4 editor-core tests | 4 | 2 | 4 |
| family 2, 22 sweep tests | 22 | 18 | 22 |

**What moved since round 1.** I withdraw two things I said.
- **"Pair along the arcs inside the partner piece."** It rested on a premise that is false in general, and on a reading of the codes that is false at coincident faces.
- **"The op leaves the insertion."** The codes the insertion reads already carry the op. Taking the op out of the start did not take it out of the insertion.

What stands: the refusals are a pairing fault, the finish check is not needed, and nesting across pairs should not be built yet.

**Terms.**
- *Vertex pair*, *shared vertex*, *link*, *germ*, *null edge*, *run*, *pairing*: as in round 1.
- *Start*: which germ the consecutive pairing begins at. A pair with four germs has two.
- *Clear*: a run that holds no other pair's cut. *Turning* a run takes the other way round between the same two germs.
- *Code*: In or Out, what the insertion reads for each arc of a link.

**Family 1: why the rule failed.** The premise was "one operand's pieces at a point are disjoint". `y` is a pyramid less a nested pyramid at the same apex. Its two vertices there bound nested cones, and its material is the shell between them.
- Walking x's link, the germs are o1, i1, i2, o2 (outer cone, inner, inner, outer).
- The arcs inside `y`'s material are o1–i1 and i2–o2. Each joins germs of *different* vertex pairs, so no null edge can run along them.
- The clear runs are the two arcs *outside* the material: o2–o1 beyond the outer cone, and i1–i2 in the hollow. x's vertex keeps the part inside `y`.
- So here the runs are Out and the vertex keeps In. In the battery it is the reverse. "Clear" is the property that holds in both; "In" is not.
- Main finds this by turning. The build removed turning and asserted In.
- The op should produce what main produces.

**Family 2: why the rule failed.** The vertex is not shared. It is one vertex pair with four germs: a boss's rim tangent inside a hole's rim.
- Two germs lie along the boss's rim and two along the hole's, in the same two directions.
- The floor between the rims is glued to the plate's top. For a union that glued sliver is coded In.
- That code is not a reading of the pose. Where faces coincide, the reclassification (`tables::eq15_3_lump`, the book's Eq. 15.3) codes the coincident region as the side the op drops.
- So "In arcs" here are two zero-width slivers. Pairing along them joins each rim's germ to the other rim's germ in the same direction.
- The join then refuses `UnpairedLooseEnds`. It fails only where a rim is a single closed edge with its vertex at the touch. The two-arc form builds under the same pairing.
- Main's start pairs each rim's two germs with each other, and builds.
- I ran the Out start for every op on `ellipse_rims_tangent_inside_build_every_op`: all 60 rows build. The In start fails the 12 union rows.

**The rule, stated.**
1. A vertex pair is paired consecutively in A's walk, from the start where A's runs lie on the side the op keeps of A (main).
2. At a shared vertex each run goes the way round that is clear (main's turning).
3. Where a run is clear neither way, its pair takes the other start, and step 2 runs again. Where that is not clear either, `SharedVertexCrossings` stays, typed.
4. The finish takes the seam correspondence the pair records give. Its shape check is retired.

**What the op contributes.** The default start, and nothing at a shared vertex beyond it: clearance there is read from the cuts. I can now give a reason for the kept side as the default, which round 1 lacked: coincident regions are coded as the dropped side, so only the kept side's arcs are sure to be real sectors. That reason is mine, not the code's. If you want it in the module doc, it should be confirmed first.

**Nested cones.** No separate rule. Steps 2 and 3 cover a vertex where some region of the other operand touches every one of its cones there: the outside for disjoint pieces, the shell for two nested cones. Where no region does (three nested cones, or a shell beside a separate piece), the runs must nest across pairs. That refuses typed today and would still. I would not build for it until a pose reaches it.

**Alternatives.**
- *Keep the built rule and patch it* (restore turning; Out start for unshared pairs). Unmeasured as a whole. It keeps a lead concept and an In premise that family 1 contradicts, with exceptions on top.
- *One search bit per shared vertex* (all runs In, or all Out). Tidier to state, and enough for every pose here. It is stricter than turning per run, which main already does, so it could refuse something main builds. I did not measure it.

**Ratified text.** None changes. The contract is in `insert.rs`'s module docs.

**Confidence.**
- Both families build under the recommendation: **sure** (measured).
- It cannot break a pose main builds: **sure** by construction. It acts only where main refuses `SharedVertexCrossings`, and the retired check only ever refused.
- Battery and cleave: **sure** on the round-1 base; not re-run on today's main.
- The family 2 failure is the join's handling of a closed one-vertex edge: **likely**. I located it by which forms fail, not by reading the join.
- The kept side is the right default everywhere, not just where measured: **unsure**.

## For the orchestrator

**The recommendation is my round-1 probe.** `GAUGE_SVC_RETRY` (re-plan a refusing pair from the other start) plus `SVC_C_RELAX` (finish check off), on main's rule. Round 1 measured it: battery 5 190, cleave 6 of 6, topo suite. I recommended a different rule from that evidence, which was the error: the probe never tested "In arcs", only "main, then the other start on refusal".

**Measured this round.**
- Build branch `0fd699740c`, debug, `ellipse_rims_tangent_inside_build_every_op`, asserts softened to print all 60 rows:
  - as built: 12 union rows refuse `UnpairedLooseEnds` (count 4 with both vertices at the touch, count 2 with one). "Two arcs" and "neither there" build.
  - `GAUGE_MUT=start` (the implementer's mutant): 60 build.
  - Out start for every unshared pair, every op (my added mutant): 60 build.
  - Trace: every four-germ plan in that test is a union, `shared=false`, `lead=A`. Intersections and differences plan no four-germ pair there.
- Main `a0f53219e41` with the round-1 probe on, debug: `touch_reread_rows` 4 passed; `a_closed_conic_in_the_partners_face` 22 passed.

**Not measured.**
- Battery and cleave on today's main under the probe (round-1 base only).
- Family 1 in the model. I took the implementer's reading and checked it against the geometry on paper. I did not trace its plan.
- The one-bit alternative, and the patched built rule.
- Whether an intersection with coincident faces and four germs exists that needs the In start. If one does, "kept side" and "Out" differ, and only main's rule is known safe.
- The rest of the slow set and the app profile under the probe.

**For the implementer.**
- The probe re-plans in the reduction's loop. The real form is the reconcile's to own: it knows which pair is not clear.
- The dead-arm count (0 turns in 372 readings) covered neither failing family. Turning and `hang_at_shared` are live; keep them.
- A-consecutive pairing suffices on everything measured, including the 40 six-crossing lines. The lead concept is not needed.

**Worth a row.** The join's one-site lane (`one_site_segments`) builds a closed one-vertex rim only when its two germs share a null edge. That is a dependence on the pairing that nothing states.

**Process.** Debug builds only this round, all in the foreground. Clone and target deleted. Nothing pushed.

**Files** (in `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/`):
- `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/fork-svc-c-probe.patch` — the round-1 probe, which is the recommendation
- `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/fork-svc-c-r2-branch-probe.patch` — trace and mutants on the build branch
- `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/fork-svc-c-r2-ell-branch.log`, `fork-svc-c-r2-ell-start.log`, `fork-svc-c-r2-ell-allout.log` in the same directory — the 60 rows under each start
