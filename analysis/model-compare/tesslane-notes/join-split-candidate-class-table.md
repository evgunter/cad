(join-split implementer) Redirect received. (1), the class table for foot alone and for foot + pair trigger, with and without the centroid point, at the default ε.

**Switches** (local, on the head, not pushed):
- JS_MID=foot: the split point is `Surface::foot` of the straight midpoint, read back with chart_point on the branch nearest the chart midpoint (the tube angle's too, on a torus). The present rule is kept at a singular end and where there is no foot (spindle, spline).
- JS_PAIR: your pair trigger. A chord point "F and G share" is an end of a chord segment whose edge's two faces are F and G (F ≠ G). A triangle counts when all three of its corners are such points for the same G, and only if G has such a triangle toward F too. Then every inside side of those triangles is split by the existing join::split: two of F's triangles have it, and it is no chord F holds. Planar faces split at the midpoint, as today.
- JS_CENTROID: in addition, for such a triangle with no inside side on a curved face, a point at the foot of its centroid, with the triangle made three. The pieces are held as `held` holds them.

**Cells: 636.** The 625 class cells, plus the A1 bodies (snowman caps at δ 1e9 and 1, the off-axis and axial lenses at 1e9 and 1, the unit ball, the cylinder, and the ball less a box at 1e9). Base and head are from the earlier runs.

| | refused where base meshes | refused where foot-alone meshes | FLAT where base positive | negative | < ½ main | cells moved vs head |
|---|---|---|---|---|---|---|
| head | 31 | 31 | 34 | 0 | 63 | – |
| foot alone | **0** | – | 34 | 0 | 81 | 86 |
| **foot + pair** | **0** | **0** | **0** | 0 | **2** | 132 |
| foot + pair + centroid | 0 | 0 | 0 | 0 | 2 | 132: identical to foot + pair, so the centroid point never fires on the class |

- check_mesh is Ok on every meshed cell in every config.
- check_volume passes on every cell of the foot and foot + pair runs (they log it).
- So conditions **(a) and (b) hold for foot + pair on the class table**, with δ 1 and 1e9 included.

**The bored ball's table:** r 1.05 and 1.3, sign ±1, unsplit, seam split at 1 point and at 5 points, δ 1e9 plus each row's δ list. 90 cells.
- Base: 9 refuse (OverusedSide, all on r 1.05 unsplit). Head: 0 refuse.
- Foot alone: identical to the head on all 90. The walked face is a cylinder, where the foot is the chart midpoint.
- Foot + pair: moves 27 cells, all r 1.05 at δ ≥ 0.3. Every one is positive, check_mesh Ok, and closer to the body's 0.137, e.g. unsplit sign +1 at δ 1e9, 3 and 1: [42,40] +0.0893 → [44,46] +0.1185; split1 δ 0.3: +0.0798 → +0.0936.
- No refusals, nothing ≤ 0. Centroid on or off: identical.

**The two cells under ½: the three-face cap**, h 0.03 at lat ±45, az 300, δ 0.3. Ratio 0.238 against main's 0.656, and the same on the head, foot and foot + pair. I read the mesh. The cap has three faces: the sphere, the slab's plane and the box's plane x = 0.5. It has three edges, each two segments or more:
- The sphere's 6 triangles: two have all three corners on the sphere–slab chord ([5,7,4] and [6,7,5]). The other four take point 2, which is on the sphere–box edge.
- The slab plane's 6 triangles all fan from point 3, which is on the slab–box edge. So the slab has NO triangle whose corners are all sphere–slab chord points.
- The trigger therefore does not fire: G has no triangle toward F. The sphere's two inside sides between slab-chord points are ones the slab does not make.
- The volume is positive because point 2 (on the box arc, off the slab's plane) lifts the sphere sheet. It is not flat; it is coarse: main's sphere arm added three sphere points there.
- This is outside the trigger's class by its definition, not a refusal or a flat mesh, and it passes check_volume (δA/V 38).

Running now, at the default ε:
- `mesh`: foot alone, foot + pair, foot + pair + centroid.
- `sweep`'s meshing modules: head, foot alone, foot + pair (about 40 minutes each).

That gives condition (c): triangles added over both suites and the bodies the trigger fires on, plus the PR's moved list. After that, three ε on the class table for the candidate that stands.
