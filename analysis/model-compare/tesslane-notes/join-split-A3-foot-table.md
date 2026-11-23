(join-split implementer) A3: the potential-side table with the split point at the foot.

**Method.** A local instrument on the head (JS_POT, not pushed):
- Placement is untouched; the instrument only counts.
- Every placed curved patch is counted, over the `mesh` suite at the default ε, seed 1. A side is every side between two shared ids that two of the patch's triangles hold at the same chart ends.
- Each side is split two ways:
  - PRESENT: today's join::midpoint (chart midpoint; the development's on a cone; the other end's u at a singular end).
  - FOOT: `Surface::foot` of the straight midpoint, read back with `chart_point` and evaluated, on the u branch nearest the chart midpoint (and the tube angle's on a torus).
- Each piece is read for a fold against the analytic outward normal at its centroid, as the filed row read it, in every lane. Each piece is also read for winding resolution (`validate::resolved_normal`).
- Three rows red under the instrument, both artifacts of the probe itself: the two K-stream pins (chart_point records band reads) and the ε inventory (the instrument's Run in tessellate.rs).

| surface, lane | sides | a piece folds: present | a piece folds: foot |
|---|---|---|---|
| cone, a lane that reads folds (walk + trimmed), no apex end | 37,968 | 687 | **0** |
| cone, the same, an apex end | 3,426 | 0 | 0 |
| cone, rectangle lane, no apex end | 136,812 | 10 | **0** |
| cone, rectangle lane, an apex end | 23,067 | 0 | 0 |
| sphere, fold-reading (trimmed) lane, no pole end | 95 | 75 | **0** |
| sphere, rectangle lane, no pole end | 3,494 | 8 | **0** |
| sphere, a pole end (both lanes) | 1,589 | 0 | 0 |
| torus, rectangle lane | 13,622 | 46 | 16 |
| torus, fold-reading lane | 717 | 0 | 0 |
| cylinder, rectangle lane | 961,197 | 6 | 6 (unchanged) |
| cylinder, fold-reading lane | 12,423 | 0 | 0 |
| other (no foot in closed form: the spindle) | 44 | 0 | 44 with no foot |

- **No piece falls under winding resolution** under either rule, on any curved surface. Planar is not measured: a plane's foot is the straight midpoint, which planar splits at already.
- **Not one foot read back to a singular chart point.** The only sides with no foot are the spindle's 44.
- **On the cylinder the foot IS the chart midpoint.** The azimuth bisects the chord, so the 6 are the same sides.

**Against the designer's expectations, and the filed row.** The filed table was taken on the stage-1 tree, before the retirements; this is the head, so the sphere rows have changed shape. The designer expected trimmed sphere 297 → 0. Here it is 75 → 0 on the trimmed lane and 8 → 0 on the rectangle lane. Cylinder is unchanged, as expected. The cone goes 687 → 0 (walk and trimmed together; the filed 685 + 0) and 10 → 0 on the rectangle lane. The torus's unknown 32 is 46 → 16.

**Does the foot serve the cone and its apex as the development and the apex rule do? Yes, and better off the apex.**
- At an apex end (cone) or a pole end (sphere), the foot IS the present point, analytically. A side from the apex runs along a straight generator, so its straight midpoint is on the surface and is its own foot, and the apex rule (other end's u, mean slant) names the same point. A side from a pole runs along a meridian, a great circle, so the radial push of its midpoint is the meridian point at mean latitude, which is what the pole rule names.
- Away from the apex, the foot folds nothing where the development folds 687 + 10.
- So the foot would make **one rule of three** (chart midpoint, the development, and the singular-end rule). It leaves one exception, the spindle sheet (no closed-form foot; 44 sides here, none folding under the present rule), which would keep the present rule.
- On the sphere the foot is exactly my JS_MID=geo. On the 625-cell class table, a+geo was 0 flat, **0 refused** and 2 under half (both outside the one-neighbour class). geo alone removed all 31 of the head's refusals, and left the flat cells flat.

Next, unless you redirect: the per-patch differentials over `mesh` and `sweep` for a+geo and a, which I paused for A1 to A3. Since the foot is the general form of geo, I will rerun them with the cure's split point at the foot on every analytic surface (spindle excepted) rather than the sphere-only geo. Say if you want the sphere-only version kept as the measured one.
