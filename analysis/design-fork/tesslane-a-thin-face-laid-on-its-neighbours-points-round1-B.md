**Reconciliation, designer B**

My recommendation has changed on two of its three parts.

**(1) What moved me**

- **Another designer's scale-probe point moved where the sign is checked.** The viewer tessellates every body at δ = 1e9 (`scene::SCALE_PROBE_DELTA`), and at that δ every body is thinner than δ.
  - A thin cap there has only a few rim points. Its triangles are made entirely of shared points and have no inside side, so no split rule can reach them.
  - If `tessellate` refused every mesh without positive volume, as I proposed, it would refuse the viewer's own probe. `combine::measured_at_scale` would then pass that refusal on.
  - So I now put the sign check at the call that needs a solid (export), plus a public check function. `tessellate` no longer refuses on sign.
- **The other designer's narrower census trigger moved what starts a split.** Under it, the join also finds two faces whose patches each hold a "bare triangle" toward the other: a triangle whose three corners all lie on the other face's boundary. It then splits those triangles' inside sides.
  - It reaches the −60 cap, as my rule did.
  - Unlike my rule, it does not split every strip between two rims, which would roughly double a plain cylinder band's triangles.
  - Unlike the topology rule, it does not put the faces across a face's edges back into the memo key.
  - The patch stays a function of (face, δ). I drop my rule, which split every inside side joining two boundary points, in favour of this one.
- **The foot is now proved on spheres.** The radial-projection argument, given in another report, settles the split point there: each piece winds as its parent did and lies no farther from the sphere. I am now sure on the sphere and cylinder, and still likely on the cone and torus.

**(2) Who answers whom**

- **Against my own-point rule:** the doubling of rim-to-rim strips and the conflict with `[mesh.sizing]` are fair. I had flagged the overhead myself, and the narrower trigger avoids it. Answered.
- **Against a refusal inside `tessellate`:** the scale probe, and bare triangles with no inside side, answer my placement of the check. Answered.
- **Against the narrower trigger:** the remark that two faces closer than δ that share no point (a hollow ball's two spheres) get no split applies to every trigger. It is why some check still has to exist, and it is not a reason to choose a different trigger. It does not refute the trigger.
- **The point I still hold against the flat-is-fine position:** "a flat mesh is correct" is true under the promise. But that view's own premise says thin-body volume then depends on how the two patches happen to choose their diagonals (0.59 of the volume at +60, flat at −60). The same body, rotated, should not flip between solid and flat at one δ. The narrower trigger removes that dependence, and it costs splits only where two patches overlap.
- That position answers this point only by "no volume is promised". That is true, and it is also why the ruling against losing bodies is still met. It does not make returning flat at −60 good when +60 returns 0.59, and Ev has said zero volume is annoying.
- **Per-shell or whole-mesh sign:** I accept that the sign must be read per shell. A void's shell is correctly negative, and a whole-mesh sign misses an inverted void. That needs the body, which the export call has.

**(3) Recommendation now (likely)**

The final state, landing in the unit:
- **Split point:** the foot, on the face's own surface, of the side's straight midpoint, for every analytic surface kind. Spline and spindle faces keep the chart midpoint.
- **Census:** trigger (i), a side two patches have inside, as now; and trigger (ii), the bare-triangle pairs. Both apply the existing split.
- **What a bare triangle with no inside side gets:** the face adds the surface point nearest the triangle's centroid, if it holds. Otherwise the face is a known, filed gap.
- **Merge condition:** the unit merges when no body the base meshes is refused, and no cell is flat where the base had positive volume.

Landing in the parked row:
- a public per-shell sign check (resolved above rounding, reading no ε);
- the export call refusing typed through that check, naming δ;
- a README sentence saying a mesh within δ may enclose nothing once δ is past the body's thickness.

Confidence on the sign-check placement: likely. On the narrower trigger clearing every flat cell: unsure until measured.

**(4) The measurement that would most change my mind**

Run the foot split with trigger (ii) on the 435 cells and the plain-ball cap, and add δ = 1e9 for the snowman cap and one lens.
- **If I am right:** it shows zero refusals and zero flat cells where the base was positive, at a negligible triangle cost.
- **If a flat cell or a refusal remains:** if one remains at a δ the viewer would actually ask for, other than the probe, that points back to splitting every inside side joining two boundary points.
- **If (ii) leaves δ = 1e9 positive on every body:** then the refusal could live in `tessellate` after all, and the third designer's placement wins.

## For Ev

**Recommendation (likely).** The split lands, with two changes. A separate, later piece adds a check that the mesh is a solid.

1. **Split point.** Put the split point at the nearest point of the face's own surface to the side's straight midpoint, not over the side's midpoint in the chart.
   - On a sphere this provably cannot fold and cannot leave δ (sure). That clears the 29 bodies the unit now loses.
   - It does not depend on where the sphere's frame puts its pole.
2. **A wider check where the patches join.** Besides a side two patches both have inside, the join also finds two faces whose patches each hold a triangle built only from points the two faces share. Both faces split those triangles' inside sides at their own surface.
   - This is what makes the thin cap at −60 come out as a solid instead of the plane disc twice.
   - The patch still depends only on its own face and δ. Nothing is meshed again, and no lane reads its neighbours.
3. **The solid check lives at export, not in `tessellate`.** It is per shell (a void is correctly negative), and the export call refuses typed, naming δ.
   - It does not live in `tessellate`, because the viewer meshes every body at δ = 1e9 to size it. At that δ, a refusal on volume would break the viewer for thin bodies.

**Rejected:**
- **Accepting flat meshes as they come.** A body's volume would then depend on which diagonals two patches happen to pick, so the same cap is solid at +60 and flat at −60.
- **Splitting every inside side between two boundary points.** This doubles plain cylinder bands. Another designer showed it conflicts with `[mesh.sizing]`.
- **The base's rule of reading the faces across an edge.** It brings the neighbours back into the patch cache key.

**Text to ratify:**
- Add to `crates/mesh/README.md`: once δ is past a body's thickness, a mesh within δ may enclose nothing; `tessellate` returns it, and a reader that needs a solid checks it.
- Clarify `[mesh.winding]`: "outward" is per triangle and says nothing about the whole body.
- `[tolerance.chordal]` is unchanged.

**Open:** a triangle whose three sides are all boundary chords has no inside side to split, for example a cap with three rim points. The proposal is to add a surface point inside it. If that point cannot hold, the gap is filed and the export check catches it.
