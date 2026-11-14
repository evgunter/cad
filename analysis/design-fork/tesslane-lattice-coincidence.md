# Design fork: a lattice point within rounding of a boundary chord point (TESSLANE), 2026-11-14

Protocol: ddd201f411 (`git log -1 --format=%h origin/main -- docs/DESIGN-FORK-PROTOCOL.md`, after a fetch).

Dispatch draws, two /dev/urandom bytes:

- arm byte: 73 (below 86 → three designers, otherwise two) → **three designers**
- label byte: 20 (not 255; 20 mod 3 = 2) → **Fable = C; the two Opus designers are A and B**

Problem statement handed to every designer: the brief in the orchestrator's dispatch (one text), on main at `b0f4b4fc3f` and after; the row that raised it, `work/tesslane/a-lattice-point-within-rounding-of-a-trim-chord-point-cuts-a-triangle-of-no-area.md`, is on `tesslane/domain-loops` (PR 6960) and describes a withdrawn fix, so the designers were pointed at main and told not to read that branch.

## First reports, 2026-11-14, recorded before any reconciliation

Each is beside this file as delivered (`-report-A.md`, `-report-B.md`, `-report-C.md`); none names a model. All three corrected the brief on one point: the STL writer does not refuse these triangles (it refuses only an exactly zero cross product).

- **A, first report:** decide flatness where triangles are emitted: a triangle with a mesher-placed corner whose least height the tolerance band does not separate is flat, its mesher-placed corners are dropped and the face rebuilt, as a fold is today; flat triangles of boundary points only stay. The fork it puts to Ev is the scale (ε, or the certificate's rounding allowance). One line: flatness is a 3-D fact about the emitted triangle, and a flat triangle with an interior corner forms only against a constraint, so one verdict at emission covers every source and the lattice need know nothing more. `likely`. Rejected the framing in part (the defect's own line is the winding check's bare `> 0.0`). Proposes changing ratified text: under ε, a sentence of `docs/DESIGN.md` `[tolerance.chordal]`; a new promise in `crates/mesh/README.md`.
- **B, first report:** the lattice gives way to the boundary by the cell: a node is not laid if a chord point or a polygon side passes through its own cell (half a step either side). No ε and no constant; the mesh stays a function of (body, δ). One line: the cause is the lattice, the cure is by construction and is also the hypothesis the oblique-boundary lemma needs; detection at the triangle needs a scale and acts after the damage. `likely`. Rejected the framing in part (the same STL correction; rejects deciding "the same point" at all). Proposes no change to a ratified decision; states the no-degenerate-corner contract in the `Mesh` and `FacePatch` docs.
- **C, first report:** the lattice keeps `c` steps clear of every side and crease line, in its own unit; nothing decides "the same point"; and "a triangle has area" gets one definition in `mesh::validate`, read by the winding census, `check_mesh` and the STL writer. The fork it puts to Ev is `c`: it leans 1/16 (the ratified aspect cap read once more), with 1/2 (a node's own cell) the alternative, which is past today's headroom on measured faces. `sure` that the lattice-to-boundary relation is the missing piece; `unsure` on `c`. Rejected the framing in part (same corrections; a third place nodes are dropped). Proposes changing ratified text: a sentence and a clause in `crates/mesh/README.md` and its constants list.

**Agreement, judged on the recommended final state:** no. B and C agree on the mechanism (clearance in lattice units, no ε) and differ on its size; A puts the decision at emission and leaves the lattice alone.

**Isolation.** The three ran as subagents of the orchestrator's session and shared its scratch and build directories, which the brief did not separate. B reports that a probe file of its own was overwritten by another designer's and that it saw that file's probe code, no findings. C reports that it once ran another designer's probe binary by accident and saw the first lines of its output (a pose and δ with the same kind of triangle), deleted the log and used nothing from it. A reports nothing of the kind.

Harness figures, first reports: A 200,501 tokens, 1236 s; B 220,956 tokens, 1424 s; C: see below.
