# Design fork: the size of a face whose worst triangle has no closed-form bound (TESSLANE), 2026-11-23

Protocol: ddd201f411f (`git log -1 --format=%h origin/main -- docs/DESIGN-FORK-PROTOCOL.md`, after a fetch).

Dispatch draws, two /dev/urandom bytes:

- arm byte: 225 (below 86 → three designers, otherwise two) → **two designers**
- label byte: 92 (even → Opus is A, odd → Fable is A) → **Opus = A, Fable = B**

Problem statement handed to both designers: one text, held as `tesslane-sizing-without-a-theorem-problem.md` beside this file. Each designer runs in its own cloud session and container on `tesslane/iso-run-density` at `387d51f5ed` (unit 4b's branch: stage 1's probes and the orchestrator's ruling on the build), and is told to read no pull request's page or thread and no branch under `analysis/`.

Where it came from: unit 4b's stage 1 (`work/tesslane/a-boundary-run-on-a-chart-iso-line-gets-a-lattice-line.md`) measured that the class meshes at the reference count when sized at δ with a refinement step on a refused certificate, and `[mesh.sizing]` keeps the headroom on a lane without a theorem. The orchestrator ruled the parts of the cure the page does not speak to and sent this part here.

## First reports

Dispatched 2026-11-23, concurrently, one identical prompt (the statement under a nine-line header), each as its own cloud session:

- **A** (Opus; session `session_01BJC3B4Wth6ZzdMHLJJRnDj`, configured `claude-opus-5-5`), created 00:13:43Z.
- **B** (Fable; session `session_01BJC3B2LUpUeS4a7VRY1b1i`, configured `claude-fable-5-2`), created 00:13:45Z.

Owed to both at once when it arrives: the implementer's row with the headroom kept and the rows off.

00:15Z: the owed row sent to both in one identical note (interrupt priority, both still reading): headroom kept, rows off on the cylinder: 210 of 210 cylinder cells at 1.00 / 1.40 / 1.50 with no refinement (it never fires); spheres as today; so the rows alone carry most of the cylinder gain, sizing at δ is worth about 1.4 on cylinders and 2 on spheres, and the refinement is reached only by a cylinder face sized at δ.

Recorded as delivered, before any reconciliation.

- **A** (Opus; `last_served_model` claude-opus-5-5): `…-report-A.md`. Delivered 00:30:44Z (17 min after creation). Usage as `get_session` reports it at 00:49Z: $1.63; input 48, output 17,743, cache read 2,869,715, cache write 87,291. **Measured** the whole `mesh` suite (546 tests, release, one ε) under five switch settings, and a scratch edit of the retry budget on four tests. **Recommends (likely):** (1) the chart lane's cylinder faces lay no rows, a defect fix under the page as it stands (sure); (2) rewrite the clause: every chart-lane face sizes its lattice and chords its rims at δ from its chart's cell bound (proved per chart); the triangulation is any division into triangles; a triangle over δ gets one interior point (the midpoint of its widest side that is no boundary side, else its centroid) and the face is triangulated again under a named work budget, refusing typed past it; `UNPROVED_HEADROOM` is deleted on EVERY analytic arm and the page's list of kinds of worst triangle goes. Lane-wide at δ with rows kept: without refinement 7 faces that mesh today refuse and 1 folds; with it and the shared budget of 6 rounds all but one wide tilted cylinder wall mesh; at 40 rounds that meshes too; one face still folds (a tube near a sphere's pole, which folds at δ and not at δ/2). Rejects sizing the notched class alone at δ (the orchestrator's interim ruling): the class is a fact about routing. Framing rejected: y in part ("no theorem" is only a missing containment bound; soundness never rested on the step; the brief's lane-wide refusals mix three causes). Ratified text challenged: y (the two bullets; provenance: the headroom precondition was designer text approved in passing; "without extra work" is Ev's own and its meaning is asked).
- **B** (Fable; model usage reported under claude-fable-5-2): `…-report-B.md`. Delivered 00:48:03Z (34 min after creation). Usage as the session's last result event reports it: $3.74; input 49, output 27,076, cache read 3,723,056, cache write 119,803. **Measured** the `mesh` suite (546 tests, about 1,120 chart-lane analytic meshings, default profile) under five switch settings with the probe log armed, faces aligned per test. **Recommends (likely):** delete the half-of-δ factor for every chart-lane face; the lane sizes at δ from the cell bound it has proved, holds every triangle to its certificate, and where one reads over δ adds one interior point and triangulates again, as it does today for spline faces, refusing when a round budget ends; `[mesh.sizing]` says the step is the lane's first guess and the certificate the judge, and a theorem is a proof that a first guess is never corrected, worth writing where cheap and no debt where not. Lane-wide at δ with rows kept: without refinement 14 meshings newly refuse (13 certificate, 1 fold); with the lane's existing centroid refinement none newly refuses on the certificate, one ringed sphere face folds, five sphere faces that refuse today mesh, at most 4 rounds, triangles 0.51 to 0.56 of today's, time in these faces 25 s to 12 s; with the widest-side rule one tilted cylinder wall refuses after the budget. Rows off everywhere as well: all 428 cylinder meshings pass at a median 0.12 of today's triangles; 16 cone meshings refuse. Rejects the class-only form the same way. Framing rejected: y (the same three points, and that the headroom guarantees nothing: 0.689 δ at the headroom is already past half). Ratified text challenged: y (the same two bullets; the same provenance; the same question on "without extra work").

### Agreement on the first reports (orchestrator's reading, 00:49Z): agreed; no reconciliation round

Both recommend the same final state, each on its own lane-wide measurement: no headroom on any chart-lane face; the step is the coarsest the chart's cell bound grants at δ; the certificate judges every triangle; a triangle over δ gets a point and the face is triangulated again under a named budget, refusing typed past it; the list of kinds of worst triangle leaves the page; the two no-theorem rows and the containment-lemma unit close as not owed; sizing the notched class alone at δ is rejected. Both ask Ev the same clarification of "without extra work".

Where they differ is what a build measures, not what Ev decides: which insertion rule (A: the widest side that is no boundary side; B measured that rule losing one tilted wall within the budget and the centroid losing none), the budget's size (A: 40 rounds for that wall under its rule), and whether "a cylinder lays no rows" is a fact of the chart or of the class (A: the chart, sure; B: leans the chart, unsure).

Acted on outside the fork: unit 4b no longer builds a δ target for its class; it measures whether the class key is needed at all (rows off on every chart-lane cylinder face at today's target) and builds the form that measurement selects.

## The `[ev]` pull request

PR 7486, opened 00:52Z on 2026-11-23 on `tesslane/ev-sizing` (from main). Diff: `crates/mesh/README.md` `[mesh.sizing]` (the theorem and headroom bullets become "the step is a first guess, and the certificate decides" and "a lane that can prove its first guess is never corrected says so"; the rim-chord and clearance bullets re-worded where they named the headroom or the lattice theorem), the unit `a-refused-certificate-adds-a-point-and-no-face-sizes-below-delta` (P1, H, spec), the log entry, and fork-log row 7486, recommendation half, labels only (`b7d1fb3c62`). Body: the decision, the alternative, the one question ("without extra work"), each designer's first-report `For Ev` section verbatim under its label. The mapping for the second commit, once Ev decides: bytes 225 (two designers), 92 (even): Opus 5.5 = A, Fable 5.2 = B.
