## For Ev

**Revised recommendation (likely).**
1. **The reader.** A shell's role (outer boundary or cavity wall) is read by a ray aimed at one of its faces from outside the shell. The ray meets the face at a fixed oblique angle, about 60° from its normal, and the role is read off the nearest crossing.
2. **The volume.** The certified volume sign is read only where no ray can be cast: a shell made entirely of faces the ray walk cannot cross yet (splines). It is never read after a ray has read in band.
3. **Curved faces.** Aims extend to them, which needs no new certification.
4. **Settled in round 2.** D10's in-band-shell clause is deleted. A solid's only shell is outer by arity, with no read. Check 7 moving onto the same reader remains your choice, because it restates D1.

**Terms.** A *ray reading in band* has a margin inside the band at this ε (the tolerance), so a smaller ε might decide it. The *order gap* is the distance along the ray between its two nearest crossings. A *pair* is two faces whose separation the census, the check that measures every pair of cells, reads.

**Where volume-first and ray-first differ (sure).** The question underneath is which bodies come out differently.
- **Thick shells:** both orders give the same verdict. Only two things differ: which certificate the verdict log records, and cost. Neither cost is measured, so I can't call either one cheaper.
- **Thin shells whose pairs are all definite:** only the ray decides. This is the whole point of the change.
- **Shells whose volume read refuses** (out-of-inventory boundaries, rational walls, quadrature budget) but whose faces a ray crosses: only the ray decides.
- **Shells made entirely of spline faces:** only the volume can decide, because the ray walk has no spline arm. Curved aims do not change this. "Ray alone" would turn today's thick spline-walled cavity into a capability refusal. That is a regression if such bodies can be built; I have not checked whether they can.

**Two choices for you on the volume:**
- **Volume first vs ray first.** They differ only in which certificate the log shows and in cost, so this is your preference. I lean ray first: every shell a ray can reach gets one reader and one refusal shape.
- **Ray first with a spline fallback vs ray alone.** This is a matter of argument, and it turns on one fact: whether a multi-shell solid with a spline-only shell is reachable today. If it is, keep the fallback until the ray walk gains a spline arm. If not, use the ray alone.

**The aim, after the probe.** Measured at ε 1e-9 and 1e-6, slabs 10–20ε thick:
- **V/A:** refused every case.
- **Rays from the operand gate's far point** (about 60° to the normal, order gap about 2t): decided every case.
- **Rays along the normal from about 3 m out:** read in band at exactly t = 10ε = Kε (the top of the band), where the census read the same pair as definite. The gap came out as 9.99999994e-9 against 1e-8. That is rounding in the two long advances along the ray, not a wider band.

So aiming along the normal is wrong, which reverses my round-2 position. What made the oblique rays work was the incidence angle, and the far point only supplies that by accident. A slab lying in the plane x = y puts the hull's diagonal far point inside its own plane, so every ray to its big faces grazes them. That case is argued, not measured. Fix the angle instead:
- **Aim at each face from beyond the shell's box, at a set incidence (about 60°).**
- **Glancing-angle margin.** At the aimed face, d·n = cos 60° = 0.5 (d the ray direction, n the face normal), so no ray grazes by construction.
- **Order margin.** Between two parallel faces t apart, the gap along the ray is t/cos θ = 2t, and in general it is at least the pair's own separation.
- **Why the reduction to pairs holds at the band edge.** A pair the census reads as definite (t ≥ Kε) gives a ray gap of at least 2Kε. That is a full Kε of slack against rounding of a few ulp (units in the last place), so no special allowance is needed at the threshold.
- **The converse is harmless.** The ray reads in band only below about Kε/2, so it decides some roles whose pair is still in band. The pair check refuses those bodies anyway.
- **Unreachable face.** If a face's 60° ray is shadowed by another face, a nearer crossing simply decides, which is still correct. If it grazes elsewhere, try the next face.

Confidence: argued, likely. One cheap probe settles it: the probe branch's slab, rotated into the plane x = y and read both ways.

**Curved aims are feasible now (likely).** The code has no certified interior point for a curved face: only `planar_interior_point`, and `point_in_face` is planar too. But an aim point does not need certifying. The ray walk already decides every crossing's membership in a trimmed curved face, cone and torus windows and sphere regions included. The side is read only off a decided nearest crossing, so an aim point only steers the ray. A ray that misses everything or grazes is set aside for the next aim, and nothing wrong can be read.
- **Candidate aim point:** evaluate the surface at a parameter point inside the face's chart, for example the mean of its boundary vertices' chart parameters, or an edge midpoint nudged inward.
- **Effect:** hollow curved bodies keep their role reads with no regression, and a thin lens becomes readable.

**What a refusal names.** Either the in-band pair the ray found (its two faces, the aim point, and the gap), or a limit (no face the ray can aim at, or a spline face). A "too thin shell" refusal no longer exists.

**Confidence:**
- Recommendation: likely.
- Thick shells read the same in both orders: sure.
- The 60° aim keeps the reduction to pairs with slack: likely (planar pairs sure, curved likely).
- Curved aims without certification are sound: likely.

## For the orchestrator

1. **What moved me, and whether it's answered.**
   - I moved to volume-first for the other report's round-1 NURBS point.
   - Its round-2 "one certificate" argument answers why the volume should not come first, but not the spline-only coverage. Its cost claim ("the role read already walks every face") is unmeasured: curved-face quadrature and ray root-finding are both linear in faces, with unknown constants.
   - My NURBS concern is answered only if spline-only shells in multi-shell solids are unreachable. **Check:** can `shell`, a boolean, or any constructor produce such a solid today? If not, ray alone.
2. **Probe to settle the aim** (the one disputed point):
   - Rotate the probe branch's slab into the plane x = y (or extend `cube_into` with a placement).
   - Read it with (a) the gate's far point and (c) a fixed 60° per-face aim, at t ∈ {10, 15}ε and ε 1e-9.
   - Expected: (a) grazes or refuses, (c) decides with gap ≈ 2t.
   - **Also rerun (c) at exactly t = Kε**, where the along-normal ray failed.
3. **Implementation notes.**
   - The tangent component of a 60° aim must point into the face's own region side. Any choice is sound, because a decided nearest crossing is correct whichever face it lands on.
   - Start beyond the shell's box, not 2·reach + 1, to keep rounding on the order of the coordinates' own ulp.
   - `aimed_side` drops `aim_points`' certification requirement; the planar ladder becomes just one candidate source.
4. **Rows.** As in round 2:
   - **EXPORT row:** routing step-export through `classify_shells_of` closes it, and then inherits this reader.
   - **TALLY "locates no piece":** closes if the refusal carries the aim point and the two faces.
   - **TALLY "f64 margin":** closes for roles. It survives only on the volume fallback, and on check 7 if check 7 stays on V/A.
5. **Plumbing.** `Crossings::closest` should hand back both hits on an in-band order, so the refusal can name the pair.
6. **Doc drift** in `shell_winding_errors` (says silent, pushes `ShellRoleUndecided`) still stands.