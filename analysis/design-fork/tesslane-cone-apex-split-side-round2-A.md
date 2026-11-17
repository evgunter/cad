## For Ev

**Where I moved and where I did not.** I hold the first report's final state: mesh a ruled face as *strips*, in the single lane the plan is heading to, with the bound proved by a one-line lemma below. I have widened the scope to every face with a split straight side, the cylinder included. I had measured the cylinder failure but stopped short of naming the whole class. I have also widened the interim route to that same class, which means the rectangle lane's fallback lattice goes away now rather than later. I take one sentence from the other analyses for `[mesh.sizing]`. I disagree with the alternative that keeps PR 7043's refusal as the only interim answer, and with that same report's doubt that the strip bound carries over to the cone's development (both are under the options below). I re-ran the strip prototype from the third analysis on my own bodies, and it holds (see Measured).

**Terms.**
- **Ruled face**: a face on a cylinder or a cone. Each of its lines `u = const` is a straight *ruling*. On a cone every ruling passes through the apex.
- **Split side**: a straight side of a face that is made of two or more edges, so it has a vertex part-way along. Booleans produce these.
- **Rectangle lane** (`mesh::curved`): one of the two meshers of curved faces. It triangulates directly in the chart coordinates `(u, v)`, where the cone's apex is a whole row of points.
- **Chart lane** (`mesh::trimmed`): the other mesher, and the plan's one future lane. On a cone it triangulates in the *development*, the cone unrolled flat about its apex, where the apex is a single point.
- **Column**: the part of a face between the rulings through two neighbouring rim chord points.
- **Strip mesh**: a mesh of a ruled face with no interior points, in which no triangle crosses a column.
- **Headroom**: the chart lane meshes at δ/2 wherever it has no proof of its bound (`UNPROVED_HEADROOM`).

**Recommendation — final state (likely).** On a cylinder or cone chart, a face bounded by rims, rulings and the apex gets a strip mesh, built in the chart lane at δ with no headroom.
- **The rule.** Lay no lattice. Hold one edge per column wherever the boundary does not already force it: the apex to each rim point, or a corner of a split side to the nearest rim point of the next column. This is what C's prototype does.
- **The bound (sure).** Two rulings of a cone both pass through the apex, so they lie in one plane. Every triangle whose corners lie on two neighbouring rulings therefore sits inside the fan triangle (apex, rim point i, rim point i+1). It deviates from the surface by no more than that fan triangle does, and `sizing::cone_grid_step` already holds that triangle to δ. On a cylinder the two rulings are parallel, and the rectangle between them plays the same role.
- **Why the plane does not matter (sure).** That argument is about the triangle's corners in space, not about which plane the triangulation runs in. So it holds in the development exactly as in the chart. This answers the doubt raised about restating the strip argument in the development.
- **Why a split point costs nothing.** A split point lies on its side's ruling. So it lies on an edge of the unsplit mesh and divides one triangle, in that triangle's own plane.
- **What it makes true:**
  - Splitting a straight edge never makes a face refuse and never changes how dense it is.
  - A triangle with no area cannot be built. In the development, three points on one ruling are exactly collinear, and the held column edges stop any triangle spanning two columns.
  - The open row `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem` closes on a theorem that already exists.
  - The chart lane stops laying rows on ruled faces. Those rows are what make it 25–80 times denser on cones today, so the density objection to unit 6 goes away for these two kinds.
- **Still open (unsure).** A frustum (a cone with no apex on the face) with a split side whose two rims are chorded at different counts. A neighbouring face can force that. A strip there needs a held edge to a rim point that is not straight across, and I have not proved its bound. A face spanning both nappes (both halves of the double cone) refuses in the development (`BothNappes`). Nobody has found an operation that builds one.
- **Reversible**: yes. It changes the triangulation and keeps the same certificate.

**Until it is built (likely).** Route every ruled face that has a split side to the chart lane. The test is read from the face's own loop: an iso-rectangle (a face whose sides are all rims and rulings) whose straight side is more than one edge.
- **Remove at the same time:** the rectangle lane's fallback lattice and `bare_sides`. That lane then assumes bare sides and asserts it.
- **What it does:** every measured failing case meshes certified. Each lane then meshes only what its theorem covers.
- **What it costs:** these faces carry 20–80 times the triangles they need until the final state lands.
- **Keep:** PR 7043's `UnresolvedWinding` refusal stays as the backstop on every lane (sure).
- **Rejected for good:** main's behaviour, which returns triangles whose normals are rounding noise. That breaks the ratified `[mesh.winding]` rule (sure).

**The other options, each as a final state.**
- **Only PR 7043's refusal until the final state lands.** This is honest, and it adds no code. But the 2026-11-17 brief measured 84 refusals on plain cones from δ 0.01 down, and plain cylinder walls refuse `CertificateExceeded` too. Both are faces the chart lane already meshes certified. The charter says a body is drawn, or refused with a reason that names what is not built. Here it is built.
  - *The objection*: the route goes by a property of the boundary. My answer: the number of edges along a side is a fact about the face's domain, the same basis unit 4 routes by.
  - *What would change my mind*: a face the route sends to the chart lane that then refuses or meshes worse. C's suite showed none beyond pins its over-wide test predicate moved.
- **The strip mesh first in the rectangle lane** (the third analysis's ~40-line prototype). It reaches the final triangle counts at once. But it puts new rules for the apex and for side corners into the raw chart, in a lane the plan retires. This is defensible, and the choice between it and the route is only about order. I lean to the route.
- **Send every cone face to the chart lane as it stands.** Rejected (sure). Plain cones become 25–80 times denser, and split cylinders still refuse.
- **Fix only faces through an apex** (my first scope). Rejected. It leaves the cylinder refusing and the frustum on a lattice no theorem sizes.

**Measured.** This is PR 7043's head, debug build, default ε. Bodies: a full 45° cone, a 1.3-radian cone wedge, a full 80° cone, and a diamond revolved through 1.3 rad (two frustums and a cylinder wall). Each edge was split at its midpoint, or at two points, then the body was placed obliquely and meshed at δ 0.1, 0.01 and 0.001.
- **Today:** 40 cells refuse. Cones through the apex refuse `UnresolvedWinding`. The cylinder wall refuses `CertificateExceeded` at every δ, with a bound stuck between 0.09 and 0.20. Frustums refuse at δ 0.1, and at δ 0.001 they mesh with 424 triangles where 116 would do.
- **Under the strip prototype:** all 40 mesh and pass `check_mesh` and `check_winding`. Each comes in at the unsplit count plus one triangle per face beside the split vertex: full cone at δ 0.001, 144 against 142; cylinder wall, 118 against 116.
- I take as measured but did not re-run: the 18 T1 cells, the unit-cylinder table, and the exact chart-lane density ratios. My own chart-lane counts (26× on a full cone at δ 0.003) agree with them in size.

**What needs your word.**
1. One sentence for `[mesh.sizing]`, which you reviewed in the S29 rewrite: *a lane's triangulation is constrained to what its theorem assumes; a premise is never left to what Delaunay does on a grid.* With it goes the strip lemma, as the cylinder and cone arms' theorem.
2. Unit 6 (rectangle faces move to the chart lane) stays yours. Ask it after strips land in the chart lane. Re-measure its density premise then, per chart: "about twice the triangles" is wrong for ruled faces today.

The interim route changes no ratified sentence. The arm it removes came from an agent's fix pass (PR 5717), not from you.

## For the orchestrator

- **Re-run.** I applied C's `probe.diff` to my tree and added my own probe test. Raw results: `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/fork2/A/probe2.txt`. Each line is tagged BASE or STRIP. The first-round run is `probe1.txt` beside it. The tree is reverted and the target directory deleted.
- **Agreement.** All three reports name the same defect: the rectangle lane's raw-chart Delaunay, whose strip premise nothing holds. All three put the final home in the chart lane. B frames it as a chart rule for when rows are needed. A and C both say no rows plus held column edges, and I think B's "rows when needed" is subsumed by that.
- **Where B differs:**
  - B's interim is the refusal only. That is the real fork to put to Ev.
  - B scopes to cones only. That misses the cylinder, which both A and C measured.
  - B rates the strip argument in the development as "unsure". The plane-independence lemma above settles it.
- **Where C differs:** my lemma gives C's "theorem that already exists" a stated proof.
- **C's off-question rows** (cylinder split refuses, `TOTAL_REFUSED`'s comment blames the wrong face, reopen `curved-lane-triangulates-in-raw-chart-coordinates`) match my first-round finding. File them once.
- **B's `chords::tighten` straight-edge skip** is moot under strips. A straight edge is its own chord.
- **Not checked by me:** a face truly spanning both nappes; the misaligned-rim frustum case; strips implemented in the development itself (C prototyped in the raw chart only); needle-cone thinness under strips (`a-needle-cone-face-meshes-thin-triangles-beside-its-rulings`).
