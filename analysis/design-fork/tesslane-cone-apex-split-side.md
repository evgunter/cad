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

## Round two

Each designer was shown the other two first reports (sent 01:28Z; re-sent 01:34Z after the container restarted and stopped all three).

- **B**: `tesslane-cone-apex-split-side-round2-B.md`, delivered ≈01:35Z; 180,997 tokens in all (about 22,000 this round). **Moved** on scope (re-ran the cylinder wall: in the class), on the final state (the strip built by construction on every ruled chart, stated per column so a frustum with unequal rim counts is covered), and on the meanwhile (route the ruled faces with a split straight side to the chart lane, delete the fallback lattice). **Held**: main's mesh must not come back; `UnresolvedWinding` stays as backstop; every cone wholesale is not a final state.
- **A**: `tesslane-cone-apex-split-side-round2-A.md`, delivered ≈01:36Z; 191,934 tokens in all (about 27,000 this round). **Held** the final state (strips in the chart lane), with a stated lemma for the bound (two rulings of a cone lie in one plane; the argument is about corners in space, so it holds in the development as in the chart). **Moved** on scope (every ruled face with a split straight side) and on the meanwhile (route that whole class, delete the fallback lattice and `bare_sides` now). Re-ran C's strip prototype on its own bodies: 40 refusing cells all mesh at the unsplit count plus one triangle per face beside the split vertex.
- **C**: `tesslane-cone-apex-split-side-round2-C.md`, delivered ≈01:39Z; 335,199 tokens in all (about 49,000 this round). **Held** scope, home and the meanwhile. **Moved** on the rule's form: the unsplit face needs no rule in the chart lane once its rows are off (measured: 1.25 to 1.45 times the rectangle lane's count, the headroom's finer rim chords and nothing else), and each split side needs exactly one held edge (measured: without it a split cone or frustum refuses `CertificateExceeded`). Took A's lemma as the theorem. New measurements: a frustum's two rims are chorded at different counts from δ 0.03 down, so a rim point has no partner at its angle; slender cones (5°, 1°) with a split side refuse `UnresolvedWinding` at every δ today and mesh at 5 to 13 cone triangles with the held edge.

### Agreement after round two (orchestrator's reading, 01:41Z)

Everything Ev is asked now has one answer from all three: the class is every ruled face with a split straight side; the final state is that such a face meshes as the same face without the split plus one triangle per vertex, in the chart lane, with no interior rows; until then the class is routed to the chart lane, the rectangle lane's fallback lattice is deleted and PR 7043's refusal stays as the backstop; `[mesh.sizing]` gains the principle and the ruled-face theorem; unit 6 is asked after, per chart.

One difference is left, and it is the construction, not the decision: A holds an edge per column (and left the unequal-rim frustum open); B builds the strip by walking the two sorted boundary rows with no Delaunay step; C lays no rows and holds one edge per split side, with measurements that the unsplit face needs nothing more. A third round runs on that alone, each shown the other two round-two reports.

## Round three (the rule's form only)

Each designer was shown the other two round-two reports (sent 01:41Z).

- **A**: `tesslane-cone-apex-split-side-round3-A.md`, ≈01:42Z; 204,954 tokens in all (about 13,000 this round). **Moved**: withdraws its own form (an edge to each rim point's partner) on C's frustum rim counts and takes B's: walk the two boundary rows in angle order and join them, no Delaunay step. Prefers it to C's because C's leaves the unsplit face's strips to what Delaunay does, which the sentence all three propose forbids. C's form, with a test that fails if a triangle spans more than one chord in angle, is its fallback. Owes: a written proof for a triangle on three rulings within one chord's angle.
- **B**: `tesslane-cone-apex-split-side-round3-B.md`, ≈01:42Z; 195,029 tokens in all (about 14,000 this round). **Held** a construction that makes the premise true, now stated as: merge the two rows by azimuth, every join (rung) a held edge in the chart lane's constrained triangulation. States the bound for every variant: every triangle's azimuth range is at most the face's column step. **Would move** to C's single held edge on a proof that Delaunay in the development of two concentric arcs keeps every triangle within one column.
