# The problem: what size the mesher gives a face whose worst triangle has no closed-form bound

This is a statement of a problem, and a hypothesis about the tree, not a finding. Check it before you build on it. It names no answers: the options are yours.

## Where you are

Branch `tesslane/iso-run-density` of `evgunter/cad`, a unit in progress (not merged). It is cut from `tesslane/ruled-density` (pull request 7096, reviewed, not yet merged), which is cut from a recent main. So this tree holds two things main does not: 7096's walk (`crates/mesh/src/ruled_walk.rs`), and this unit's measurement probes (`crates/mesh/src/iso_probe.rs`, env-gated switches in `trimmed.rs` and `tessellate.rs`, the harness `crates/mesh/tests/iso_density_probe.rs`, `#[ignore]`), which are throwaway and will not ship.

## Terms

- **δ**: the chordal tolerance a mesh is asked for. The promise (`docs/DESIGN.md` `[tolerance.chordal]`, `crates/mesh/README.md`): every point of an emitted mesh lies within δ of the true surface.
- **Certificate**: a closed-form bound on one triangle's distance from its surface (`crates/mesh/src/cert.rs`). Every triangle of every face is held to it; a face with a triangle over δ refuses, typed (`CertificateExceeded`). It knows nothing of how the triangle was made.
- **Lane**: one way of making a face's triangles. The **rectangle lane** (`mesh::curved`) meshes a face whose domain is a rectangle of its chart, on a grid. The **chart lane** (`mesh::trimmed`) meshes every other curved face: chord points on the boundary, a lattice of interior points, a constrained Delaunay triangulation (CDT). Since 7096 it also **walks** a cylinder or cone face with a point on a straight side, writing the triangles directly with no CDT.
- **Step**: the grid or lattice spacing a lane chooses before it makes any triangle. **Theorem**, in this page's sense: a closed-form bound on the worst triangle a lane's way of triangulating can make, from which the coarsest step that keeps that triangle within δ is solved (`crates/mesh/src/sizing.rs`).
- **Headroom**: `trimmed::UNPROVED_HEADROOM = 0.5`. The chart lane's analytic arms (cylinder, cone, sphere, torus, spindle) size their lattice and chord their edges at half of δ, because a CDT triangle beside a trim chord can span more than one lattice cell and nothing bounds by how much. Read its doc comment in `trimmed.rs` and `trimmed::face_target`.

## What the ratified page says

`crates/mesh/README.md` `[mesh.sizing]`; read the whole section and `[mesh.winding]` above it.

- The headline: "**Make the mesh as coarse as possible without extra work.** Each lane (curved, trimmed, NURBS) chooses its grid step from a bound on its own worst triangle, and takes the coarsest step that bound allows."
- "**Each lane has a theorem.** It names the worst triangle the lane's triangulation can produce and bounds that triangle's deviation in closed form. Depending on the lane, the worst triangle is half a grid cell, or a Delaunay sliver beside a point the grid did not place (a trim or chord point). The theorem is pinned by a test next to the lane's step function."
- "**No safety factor.** A step is never shrunk 'for margin'. Where the step can be solved for exactly, its worst triangle lands at δ."
- "**A lane without its theorem yet** keeps an explicitly named `UNPROVED_HEADROOM` factor at its call site, filed as a debt. The fix is to write the theorem, not to tune the factor."

Provenance, which you should check and weigh (`git log --all -S'<short phrase>' -- crates/mesh/README.md`): the section was written on 2026-10-25 in commits titled `[ev] S29` (`d0b1563d291`: "the rule is the coarsest step each lane's own theorem grants without extra work (Ev's clarification)"; `ff1f4265bb4`: "the mesh README rewritten plainly").

The debt is filed as three rows under `work/tesslane/`: `the-trimmed-cylinder-lane-has-no-worst-triangle-theorem`, `the-trimmed-sphere-arm-has-no-worst-triangle-theorem`, and the unit `one-containment-lemma-for-an-oblique-boundary` ("one containment lemma for a boundary run that crosses lattice lines, and UNPROVED_HEADROOM goes"). None has been built.

## What the owner said on a neighbouring question

On pull request 7046 (2026-11-17), where designers had proposed a rule constraining how a ruled face is triangulated so that a bound could be had, Ev wrote:

> why impose the triangulation constraint rather than improve the method for getting the bound? it's not a total nonstarter to impose additional constraints on triangulation, but i'd really prefer if everything would work properly with triangulation just at the level of abstraction of "any division into triangles so that it never deviates by more than delta [and maybe a couple other universal conditions]"

and, of a proposed page sentence saying a lane's step must cover the triangles its triangulation can make: "i still don't really see how [it] means anything stronger than 'triangulation is sound', which doesn't need to be stated". That sentence was not added.

## What was measured, and what it puts in question

Unit 4b (`work/tesslane/a-boundary-run-on-a-chart-iso-line-gets-a-lattice-line.md`: read its last sections, the table and the ruling) measured one class of chart-lane faces: a cylinder wall or sphere face bounded by lines of its own chart alone that is no rectangle (a pipe wall with a notch, a ball less a wedge). Over 210 cylinder and 60 sphere cells, δ 0.1 to 0.001, three ε with identical results, against the count the rectangle lane's proved theorems give the same shape cut into rectangles:

- Today (headroom, uniform lattice): cylinder walls at 1.2 / 22 / 223 times that count (min / median / max); sphere faces at 1.65 / 1.96 / 4.4.
- Sized at δ instead of δ/2, the lattice's rows left off on the cylinder (it is straight that way), nothing else changed: 194 of 210 cylinder cells mesh, at 1.00 / 1.02 / 1.17; the other 16 refuse on the certificate. All 60 sphere cells mesh, at 0.75 / 0.95 / 3.0.
- The same, and where a triangle's certificate refuses, one point inserted (the midpoint of that triangle's widest side that is no constraint) and the face triangulated again: 210 of 210, at 1.00 / 1.02 / 1.20, one round on every cell that needed any, worst certificate reading 0.998 δ.
- Applied to every chart-lane analytic face rather than to the class, the same switches make faces refuse that mesh today (cone faces cut by a section, m5_pr11's tilted cylinder wall).

On the analytic arms nothing repairs a refused certificate today: the face refuses. The lane does refine on a refusal for spline rows (`refine_lane` in `trimmed.rs`).

The implementer is measuring one more row, which I will send to every designer when it arrives: the same faces with the headroom KEPT and the rows off, with and without that refinement.

So there is a class of faces for which no theorem exists, the half-of-δ factor costs a factor of two to two hundred in triangles, and a mesh at δ is reached by sizing at δ and letting the certificate say where a point is missing. The page's answer to a face with no theorem is the headroom until the theorem is written. Separately, since 7096 the page's sentence listing the kinds of worst triangle ("half a grid cell, or a Delaunay sliver…") no longer lists every way a lane makes triangles: a walked triangle is neither.

## What needs deciding

What the mesher does, and what `[mesh.sizing]` says, about the size of a face whose worst triangle has no closed-form bound: for this class, and for the chart lane's other faces that read the headroom. What "without extra work" and "each lane has a theorem" should mean once that is settled, and what becomes of the three debt rows. The scope is yours to set: the question may be about this class, about the headroom, about the role of the certificate against the role of the step, or about something this statement has not seen.

## Rules for this lane

- Read `docs/prompts/designer.md` IN FULL first; it binds you. Your report follows its §5: `## For Ev` and `## For the orchestrator`, at most 150 lines in total, naming no model, lane or vendor.
- Read no pull request's page, review thread or comments, and fetch no branch under `analysis/`. Commit nothing, push nothing, post nothing on GitHub.
- You may measure: the probes are there for it (`ISO_*` environment switches; read `iso_probe.rs` and the harness). One build chain at a time, built narrowly (`-p mesh`); a job that could outlive a 600 s foreground call is launched `setsid`-detached with its output to a file and polled in the foreground. Say in the report what you measured and what you only read.
