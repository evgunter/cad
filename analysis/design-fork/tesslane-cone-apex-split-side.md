# Design fork: a cone face through its apex with a split side (TESSLANE), 2026-11-17

Protocol: ddd201f411f (`git log -1 --format=%h origin/main -- docs/DESIGN-FORK-PROTOCOL.md`, after a fetch).

Dispatch draws, two /dev/urandom bytes:

- arm byte: 10 (below 86 → three designers, otherwise two) → **three designers**
- label byte: 110 (not 255; 110 mod 3 = 2) → **Fable = C; the two Opus designers are A and B**

Problem statement handed to every designer: one text, held as `tesslane-cone-apex-split-side-problem.md` beside this file. Each designer works in its own git worktree of `tesslane/lattice-clearance` at `ba68f8ed0a` (PR 7043's head, where the faces refuse) with its own build directory, and is told to read no pull request's thread and no branch under `analysis/`.

## First reports

Recorded as delivered, before any reconciliation. Dispatched 2026-11-17 ≈00:50Z.

- **B** (Opus): `tesslane-cone-apex-split-side-report-B.md`. Delivered ≈01:00Z; 159,025 tokens, 44 tool calls, 853 s. Recommends one home for every cone face, the development (the chart lane's plane), with the rectangle lane's strip argument restated as a rule of the cone's chart; rejects routing every cone face to the chart lane as it stands (measured ×14 to ×160 the triangles on bare cones); until then PR 7043's `UnresolvedWinding` refusal stands, with an optional temporary bridge (route only the split-side cone faces).
- **A** (Opus): `tesslane-cone-apex-split-side-report-A.md`. Delivered ≈01:06Z; 165,119 tokens, 28 tool calls, 1,037 s. Recommends meshing a ruled face (cone, cylinder) as strips between neighbouring rulings, in the chart lane, with no interior lattice and a fan-triangle bound at δ; every cone face routes there. Until then, route a cone face holding its apex with a split side to the chart lane now (not main's behaviour; a typed refusal naming the split ruling as fallback). Also measured: a cylinder face with a split straight side refuses `CertificateExceeded` at every δ in the rectangle lane; whole-cone density 26× in the chart lane as it stands.
- **C** (Fable): `tesslane-cone-apex-split-side-report-C.md`. Delivered ≈01:25Z; 286,347 tokens, 60 tool calls, 2,206 s. Widens the class to every ruled face (cylinder or cone wall) with a split straight side. Final state: the strip is held by constraint (no rows, no triangle crosses a column; the cone in its development), living in the chart lane; prototyped in the rectangle lane (every cell meshes at the unsplit count plus one triangle per vertex). Until then: route exactly the ruled faces with a split straight side to the chart lane and delete the rectangle lane's fallback lattice; PR 7043's refusal stays as the backstop.

### Agreement after the first reports (orchestrator's reading, 01:27Z)

All three name the same cause (the rectangle lane's Delaunay triangulation in the raw chart, where nothing holds triangles to one column) and the same end state in kind: a ruled face meshed as strips with no interior rows, the cone in its development, in the chart lane. All three measured the chart lane as it stands at 14 to 160 times the triangles on plain cones, against the plan's "about twice". They differ on two things, so a second round runs:

- **Scope.** B speaks of cone faces only. A and C found the cylinder wall with a split straight side refusing `CertificateExceeded` and make the rule one for ruled faces.
- **The meanwhile.** A: route cone faces holding their apex with a split side to the chart lane now. B: keep PR 7043's refusal and add nothing (the same route as an optional bridge). C: route every ruled face with a split straight side and delete the fallback lattice, the refusal staying as backstop.
