# Review of PR #4044, frozen head 4bd8f5a8b9

Lane `reach-dual4044-r2`. Wall clock 2026-10-05 03:45 → 04:40 UTC. **Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 3 · NOTE 3.
No glimpse: read only the PR body (`get`), the brief's files, and the check runs. I fetched no `analysis/reach-dual/*` branch and read no PR comments or reviews.
Gate: check runs on head `4bd8f5a8` are green (run 37251018204: test, lint, corrupt-input, meter).
Local: nextest `-p topo -p sweep` gives 4392/4392 at ε 1e-9 and 1e-12. At 1e-6 it gives 4391/4392; the red is `pocket_ring_steep_ellipse` (known red on main).

## Exercise (probes in `probes/dual4044_r2.rs`; mount in `crates/sweep/tests/all.rs`)
Oracle: my own indicator functions per operand plus closed-form cap and lens volumes. Tiers 1–3 and volume are checked on every build, and `point_in_solid` is sampled against the indicator.
- **PR head, ε 1e-9, 342 ops, 0 wrong bodies.** Cases: lens × slab at 11 azimuths, cap h 1e-6…0.0295 (0.09° from the rim), tilts 5–12°, ×1e-3 and ×1e3 scale; the lens's lower (0.8-ball) face; an unequal lens `ball(1)∩ball(1.3,y=1.5)` at tilt 40°.
- More cases: banded ball ∩ a cube turned 45°/30° (four cut-ins, two on each half-band); two caps on one meridian (a V of slabs); results reused as operands.
- Recut and cut-in on one operand: lens ∖ (a seam-turned cavity ball under the plane) fires both, and every build matches the closed form.
- I traced which poses take the new path (stderr in `apply_cut_ins`, local only): 64 pose-op pairs cut in. Poses whose circle crosses a seam take the crossing layer, as claimed.
- On the head, every `point_in_solid` on a new body refuses (the carved-sphere lane). So I merged `reach/carved-sphere-classify` (`b793623189`) locally and re-ran everything.
  37,440 sampled points agree with the oracle and none disagree. Sphere∖box, cap-reuse and (banded∖slab)-reuse poses then build to closed form. The strut row flips to a build, as disclosed.
- ε 1e-6 and 1e-12 give the same picture. The only reds are fixture or oracle limits: the ×1e3 ball is not finished at 1e-12, and a ×1e-3 sample lies inside the 1e-6 band.
- Claim 4 (cosurface merge): every ∩ cap result is F2 E2 (the cap's halves merged). ∪ and ∖ keep one extra face and two extra edges against the seam-crossing poses (the cut kept, as claimed). Tier 3 holds everywhere.
- Claim 5: `CutIn` sits in `ending()`, in the test table, and in `offer_rows` SITES (2+2). See NOTE 1.

## MINOR
1. **The second cut-in on one face depends on face order.** DEMONSTRATED (`probe_two_caps_one_meridian`).
   Banded ∩ cube turned 135° (the same point set as 45°, a cube being 90°-symmetric) refuses every op: "the cut's meridian leaves the sphere face through no boundary". Turns −45°, 100° and 160° refuse the same way, while 45° and 30° build.
   `apply_cut_ins` reads `cut.face` (ops.rs:3655), recorded by the scan before an earlier `mef` split that face. Whether the later circle still sits on that key depends on which piece `mef` leaves it on.
   This is a typed refusal, not a wrong body, but it is a liveness gap that the PR does not disclose and no item schedules.
2. **A pole-to-pole cut is a rounding coin flip.** DEMONSTRATED (`probe_sphere_less_box`, with #4046 merged).
   `span = atan2(≈0, −1)` (ops.rs:3924) comes out ±π by rounding when both ends are poles. For ball∖box, lat 20 az 130 builds; lat 0 az 270 refuses every op with `Euler(Certification{IntervalNotForward, margin −π})`.
   That error comes from the Euler layer, not the `CutIn` site, so it carries neither the cut's reason nor its lever.
3. **Claim 6 is partly false.** DEMONSTRATED by mutants on the head:
   - M1, the wrong half-plane (`h = axis×k`, ops.rs:3674): the lens and banded rows go red.
   - M3, any verdict cuts (ops.rs:3210): only the strut row goes red, and only through its refusal text.
   - M2, the farther hit (ops.rs:3827/3836): survives every PR row, on the head and on the #4046 merge. Every PR fixture has one hit on each side, so nearest-hit selection is unguarded.
     My sphere∖box probe (arc hit plus a pole beyond it) catches M2 only as build→`VolumeUncertified` on the merge.
   - With #4046 merged, M3 leaves every row as it was without M3. So once the strut row flips to a build, no row guards "only R-loop cuts" (claim 2).

## NOTE
1. refusal_routes.rs:964–968: `CUT_LEVER` was inserted between `EXTENT_LEVER` and its doc comment. `CUT_LEVER` now carries the extent lever's doc, and `EXTENT_LEVER` has none. By inspection.
2. On the head, `point_in_solid` refuses on every new result. The rows' only oracle there is volume plus tiers. Disclosed through #4046 and the tess P0, and fine as a follow-up.
3. The pole-strut row is a pin that flips with #4046, as disclosed. Measured: on the merge both strut poses build.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q6, Q7; Q8 partly — read `apply_cut_ins`, `sphere_extent_scan` and `apply_recuts` end to end, not all 5.7k lines of ops.rs)
- Q1, **sure**: the arc × meridian-plane roots in `apply_cut_ins` (ops.rs:3725–3760) are the same derivation, line for line, as `sphere_region::ray_roots` on the in-flight sibling `reach/carved-sphere-classify` (its `sphere_region.rs`, which I read on my local merge).
  Both build offset/d/cos_part/sin_part/amplitude/`rounding_charge` into a `FirstHarmonic` for the arc against a plane through the sphere's centre, and both pick the nearest hit along a meridian.
  Whichever lands second makes two copies with no shared home. Also look at the other `first_harmonic_roots` callers (`circle_sphere.rs`, `circle_cylinder.rs`, `ellipse_roots.rs`).
- Q1, **likely**: `apply_cut_ins` walks the face's loops twice with the same `linked`/`Cycle`/`loop_walk` scaffold (ops.rs:3708, 3886). One returns on a lone-vertex loop, the other `continue`s, and the gate register needed two audit entries to cover that.
- Q2, **likely**: predicate names cover different questions. `bool_sphere_cut_span` names arc direction, root-in-span and along-arc order. `bool_sphere_cut_half` names two different half-plane tests, and `bool_sphere_cut_meridian` names two different placements. Escalation diagnostics cannot tell them apart.
- Q4, **likely**: `apply_cut_ins` runs on `apply_recuts`'s output with face keys the scan recorded. That holds only because `carve` keeps kept entities' keys (splitting/finish.rs:984–987), and nothing at ops.rs:775 says so. My cavity probe shows it works today.
- Q4, **unsure**: the `faces` closure comment (ops.rs:3102) still says "else the first pair's refusal"; it now returns the holding face too.
- Q7, **likely**: `apply_cut_ins` is about 350 lines in one function, with a local `Hit` struct and an `end` closure that mutates the body. The ordering `below_first` dance for two ends on one arc is the kind of invariant a type would hold better.
- Q6, **unsure**: lever soundness. `CUT_LEVER` says "move clear of edges and poles". The `section_meridian` refusal (reachable only for a great circle via the `u_ref` fallback) is not answered by moving, only by turning the plane. That refusal is a decided refuse, not an escalation, so it may never meet the lever.
- Q6, **sure (fine)**: dropping the mesh from the new rows is scheduled (tess P0 item).

## Probe sources
`probes/dual4044_r2.rs` covers: lens caps, lower face and unequal lens, sphere∖box, two caps on one face and reuse, two caps on one meridian, and recut plus cut-in on one operand. Mutants were applied by hand with `sed` at the ops.rs lines cited above.
