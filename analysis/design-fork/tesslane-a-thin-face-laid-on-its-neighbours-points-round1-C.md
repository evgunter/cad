**(1) What moved me.** One point from another designer changes my position: where the sign is checked.
- The viewer tessellates every body at δ 1e9 to measure its scale (`scene::SCALE_PROBE_DELTA`, `combine::measured_at_scale`). At that δ, "thinner than δ" is the normal case, not an edge case.
- A body whose two faces are closer than δ but share no point (a hollow ball's two spheres) gives the join nothing to split. Its sheets can cross, and no rule inside the mesher keeps their sign.
- So a sign refusal inside `tessellate` loses bodies the base meshes. It also breaks the scale probe, which passes a refusal straight through.

I now put the sign check at the door that needs a solid: STL export. `tessellate` should not refuse on it.

I also take that designer's proof for the sphere. With the split point at the radial projection of the midpoint, each piece is wound like its parent and lies no farther from the sphere than its parent did. So on a sphere the split cannot refuse.

**(2) Point by point.**
- *My premise (the base rule was not wrong on this body).* Neither report refutes it. The other two put it as "the retired rule also kept the faces apart" and as "the base met it by accident". These agree in substance. Each of us says the census looks for a combinatorial fact (one side made twice) while the defect is geometric (two patches covering one region with the same points).
- *My sign check inside `tessellate`.* Answered by the scale probe and the hollow ball (above). I concede it.
- *"A flat mesh is a correct answer."* I agree it keeps the distance promise. But it leaves two things that this state does not answer:
  - The thin cap's volume then depends on which diagonals two lanes happen to lay (+60 gives 0.59 of the true volume, −60 gives zero).
  - A cap the base exported at δ 0.04 is refused at export on the head. That is a body lost at the door where loss counts.

  Widening the census, as I proposed, removes both at the cost of one more census rule. Nothing the flat-mesh state offers addresses them.
- *Splitting every inside side between two boundary points, in every curved face.* That rule is local to the patch and could be memoised, which is a real advantage. My answer stands, and the report that proposes it concedes it: the rule splits rim-to-rim diagonals on faces no neighbour lies near (every cylinder band), against `[mesh.sizing]`. My census rule splits only where two patches both have triangles built wholly from points they share, which is the defect itself. The deciding factor is the extra triangles, which is a measurement.

**(3) Recommendation now (likely).**
- **Split point (sure on sphere and cylinder, likely elsewhere):** the foot of the side's straight midpoint on the face's own surface. Spline faces keep the chart midpoint.
- **Census, in the unit:** keep today's rule (a side two patches have inside) and add a second: faces F and G whose patches both have a triangle whose three corners all lie on both faces' boundaries. Both patches then split the inside sides of those triangles. A triangle with no inside side to split gets a point at the foot of its centroid.
- **What `tessellate` owes:** distance and a closed manifold, stated in `crates/mesh/README.md`. It returns any mesh that keeps those, flat ones included.
- **The sign:** a typed refusal at the STL door that names δ. Per shell where that door holds the body, since a void's shell is rightly negative. This is the parked row's work.
- **Merge:** the unit merges once the foot and the wider census clear the 435-cell class with no refusal and no flat mesh where the base meshed.

**(4) The measurement that would most change my mind.** Head + foot + wider census over the 435 cells, the plain-ball cap at h 0.1, and the `mesh` and `sweep` suites.
- If more than a handful of cells stay flat, the wider census is a mechanism that does not do its job. I would drop it and take the flat-mesh state whole.
- If the extra triangles from the every-side rule come in under about 2%, I would take that rule instead of mine, because it is local to the patch.

## For Ev

**Recommendation (likely).**
1. **Split at the foot.** Split at the point of the face's own surface nearest the side's 3-D midpoint, not over its chart midpoint. On a sphere, a split there provably cannot refuse. This recovers the bodies the unit loses near poles.
2. **Widen what the join looks for.** Besides a side two patches both make, the join also acts where two faces' patches both contain a triangle built wholly from points the two faces share. Each such patch splits that triangle's inside sides at points of its own surface. Without this, the thin cap at latitude −60 comes back as one disc counted twice. With it, the cap keeps volume whichever diagonals the lanes lay.
3. **`tessellate` promises distance and a closed manifold, not volume.** A flat mesh of a body thinner than δ stays a correct `Ok`. STL export checks the sign, per shell, and refuses with a typed error naming δ. That is where zero volume would actually hurt.

Terms:
- **Inside side:** a triangle side that is not a chord of the face's boundary.
- **Foot:** the nearest point on the surface.

**Why the check sits at export and not in `tessellate`.**
- The viewer meshes every body at δ 1e9 to measure its size, so thin-against-δ is routine.
- Two faces closer than δ that share no point (a hollow ball) give the mesher nothing to split, and their sheets can cross.
- A sign refusal in `tessellate` would lose those bodies and break the viewer's measurement.
- Your "zero volume sounds annoying" is met where it bites: no zero-volume mesh reaches a file.

**The alternatives.**
- *Return the flat mesh and skip item 2.* Another designer recommends this. It is the simplest, but volume at coarse δ then turns on which diagonals two lanes happen to lay, and caps the base exported are refused at export.
- *Every curved face splits every inside side between two of its own boundary points.* Another designer proposes this. It is local to each face, but it adds points to faces no neighbour lies near, such as every cylinder band. Which of the two widenings costs less is being measured.

**Ratified text.**
- Add to `crates/mesh/README.md`: a mesh within δ may enclose no volume once δ is at least the body's thickness, and a reader that needs a solid checks the sign.
- Clarify that `[mesh.winding]` reads each triangle against its own face and does not say the whole body is oriented.
- `[tolerance.chordal]` is unchanged.
