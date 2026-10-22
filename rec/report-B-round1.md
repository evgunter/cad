# Designer B — first report (quad rounds dial), as delivered

## For Ev

**Recommendation (likely): stop refining a face on work and on proof, not on a round count.** Replace the four per-lane round caps (`QUAD_MAX_ROUNDS`, `QUAD2_MAX_ROUNDS`, `QUAD2_RATIONAL_MAX_ROUNDS`, `TRIM_MAX_ROUNDS`) and `LAST_ROUND_EVERY_LANE_RUNS` with the crate's existing stop rule, `crate::descent`. A face refines until its enclosure meets the reporting target (`1024·ε` as a mean boundary displacement). It stops short of that for one of two reasons only, and each reason gets its own typed refusal:
- **Floor:** no amount of refinement can reach the target. Either the lane proves this (the trimmed lane's pads, which no round removes), or `descent` sees a stall (two rounds in a row that do not narrow the enclosure, which happens at the arithmetic floor).
- **Wall:** the next round would cost more than one per-face work cap shared by every lane. Work is counted deterministically in each lane's own evaluation units, calibrated so that one unit is roughly the same cost in every lane. The after-round-0 early exit stays, rephrased: "the certified lower bound on the width at the last round the wall admits misses the target."

Keep the target, the fixed cut points and the round-window API as they are. The window addresses rounds; it never needed a cap. The sign-only caller (check 7 since PR #5568) and the reporting caller stop at the same wall. This also settles `quadrature-budget-conflates-its-lanes-and-budgets`: the one variant `QuadratureBudget` with its `rounds: 0/1/N+1` payload becomes Floor and Wall, and the Wall refusal can say which round would have been needed.

**Where I would set the cap (your dial; I lean to the first):**
- **The cost of today's most expensive face**, a composite-arm round 7 (about 10 s on this box). No face then costs more than it can today, so the change only adds admissions and never adds latency. The two-span half cylinder certifies. The quarter torus still refuses at the default ε, because it needs about 40 s.
- **A composite round 8** (about 40 s) admits the torus too, at 4× today's worst-case latency.

### Premise check
1. **The figures are not stale in substance (sure; re-measured on this branch at ε = 1e-9, release build).** The width traces still quarter cleanly, ratio 3.99–4.00. Width at the round each lane stops at today, against the 1.024e-6 target:
   - two-span half cylinder: **1.3035e-6** at round 7. Round 8 gives 3.26e-7 and certifies.
   - quarter torus: **1.1461e-6** at round 7. Round 8 gives 2.88e-7.
   - sphere octant: 9.68e-7. Certifies at round 7, as before.
2. **dm1 no longer reaches the quadrature (likely; I read the pinned test and did not run it).** `step-import/tests/wild.rs` pins dm1 as refusing at the pcurve mint with `MapResidual`. Check 7 settles its sign at round 0. So "one more round admits dm1" no longer holds.
3. **The row-by-row refusal argument the August row asks for is not owed (likely).** Every round recomputes from scratch and gives a sound enclosure, checked against the same target. So any admission that more work buys is correct by construction.
   - The "D2 addendum" posture the row cites is not in current DESIGN.md, the crate READMEs or `memories/`, as far as I could find.
   - Your 2026-08-29 ruling (a flip is fine when it is semantically correct) covers these admissions.
   - What is owed is mechanical: re-pinning the floor tests and the doc tables.
4. **The real question is cost, and a round is not a unit of cost (sure, measured).** What the next round costs depends on the face's arm, not on the round number:

   | arm | round 7 | round 8 | round 9 | growth per round |
   |---|---|---|---|---|
   | composite (quarter torus, sphere octant) | 10 s | 39 s | 157 s | ×4 |
   | `w`-uniform-in-v (half cylinder) | 0.15 s | 0.5 s | 0.6 s | ×2 |

   - The half cylinder reaches **ε = 1e-12** at round 13 in 21 s. It starts to hit the arithmetic floor at round 14 (ratio drops to 1.87), which is exactly the stall `descent` detects.
   - The teapot's own docs record that raising the rational cap to 8 makes a spout face run for over half an hour. I did not re-run that.
   - So turning one dial by one round costs very different amounts on different faces, and no round count bounds the wait.
5. **Since #5568 the caps also govern the sign walk.** A face refused after round 0 now refines to the cap whenever its sign is still undecided. So any rise in a cap also raises check 7's worst-case latency, and the latency of the boolean volume backstop's refusals.
6. **One refusal currently covers two different findings.** The trimmed lane's after-round-0 exit means "no round can ever reach the target" (a floor). The patch lanes' exit means "the last scheduled round cannot reach it" (a wall). Both refuse with `QuadratureBudget { rounds: 1 }`.

### Ratified text
Nothing ratified needs to change (likely). The caps and the `1024·ε` factor exist only as code constants: they were introduced with the lane's first commit (`b54f9af64d`), and nothing in DESIGN.md, the crate READMEs or `memories/` mentions them. The code's "the schedule is fixed (D9)" refers to the cut points, which stay fixed, and a work count is deterministic. C12(7) still holds as written: exhaustion stays a typed refusal, never a silent Gaussian.

### The options as final states
**A. One stop rule (recommended).**
- *What it makes true:*
  - one rule and one unit for every lane;
  - a refusal's type says whether the limit is the mathematics or the budget;
  - the worst-case cost of a face is bounded by construction;
  - the sign walk and the measurement share one bound;
  - a future higher-order rule only changes its lane's cost model and leaves the rule alone.
- *Consequences:*
  - Faces on the `w`-uniform-in-v arm (cylinders, extruded walls) get many more rounds for the same cost. At the 10 s cap the half cylinder certifies at ε = 1e-9. At ε = 1e-12 it narrows to 1.3e-9 against the 1.024e-9 target, so it still refuses there.
  - Composite faces keep today's reach.
  - A face like the teapot spout stops at the wall rather than running for half an hour.
- *Still possible that should not be:* the per-unit calibration is approximate, so a "unit" is only roughly equal across lanes.
- *Reversible:* yes. The cap is one constant.

**B. Turn the dial (`QUAD2_RATIONAL_MAX_ROUNDS` 7→8).**
- *For:* one line; it admits the half cylinder and the torus.
- *Against:* composite faces cost 4× (10 s → 40 s), and the teapot face is documented at over 30 minutes; the sign walk now pays this too; the four caps stay in incommensurable units. It repeats the August framing.
- *Reversible:* yes.

**C. Per-arm round caps** (give the `w`-uniform arm more rounds). Gets most of A's admissions, but adds dials where A removes them. I do not recommend it.

**D. Status quo.** `measure()` already hands back the certified bracket on a budget refusal, so nothing is unsound today. But an ordinary metre-scale part keeps refusing a number at the default ε, for a reason that costs 0.5 s to remove.

**Worked example (A, 10 s cap, default ε):**
- *Half cylinder:* the wall admits rounds up to about 11 on its arm. The bound at that round is about 5e-9, under the 1.024e-6 target, so the lane runs and certifies at round 8 in 1.5 s. Today it refuses after round 0.
- *Quarter torus:* the wall admits composite round 7 only. Its bound there, 1.146e-6, misses the target, so it refuses after round 0 exactly as today, now as a Wall refusal naming round 8.

**The lever this does not pull (sure):** the composite rule converges as O(h²) and the meter is a length. Reaching 1e-12 on a composite face needs about round 12, roughly 1000× round 7. No sensible cap gets there; a higher-order rule (one the lane's docs already list) would.
