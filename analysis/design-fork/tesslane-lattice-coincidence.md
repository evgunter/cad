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
C's first report: 318,459 tokens, 1687 s.

## Round 1, 2026-11-14

Dispatched: all three, resumed. Shown: each the other two first reports, whole. Asked: what moved it, whether its own first argument is answered, its recommendation now as a final state, and any remaining disagreement as one question. Replies are beside this file (`-round1-A.md`, `-round1-B.md`, `-round1-C.md`).

What moved:

- **A** withdrew deciding flatness at ε. B's citation moved it: commit `1240686` (2026-08-21, Ev's own) rules the mesh a function of (body, δ) and that a classification reading ε owes a guard; A had taken the sentence for an agent's (`99cc678` only condensed it). A now recommends the lattice clearance as the part that does the work. It also moved the other way on scope: after measuring that the issue-111 bodies emit no flat triangle today, it now sides with refusing a flat triangle of three boundary points.
- **B** withdrew half a cell. C's table moved it, and its own re-measurement on cone and cylinder faces (7 of 50 meshes refuse at 1/2, none at 1/16 or 1/4; its first "no new refusals" was the sphere arm only). It now recommends a small clearance as its own named constant.
- **C** withdrew "every triangle has area", the refusal of a boundary-only flat triangle and its rounding-resolved census, for A's first scope (the promise is about triangles with a lattice corner) and A's check beside `check_volume`. It also withdrew its own argument that the oblique-boundary lemma needs the clearance as hypothesis.

After round 1, all three recommend: the lattice lays no node within `c` steps of the boundary, `c` small (each names 1/16), nothing in the mesher reads ε, and the mesh's design page gains a promise about flat triangles. Positions moved and did not all meet. Still apart:

1. A flat triangle on three boundary points: kept (B, C) or refused typed (A).
2. The scale of the check outside the mesher: ε, read by the validator only (C), or rounding (A, B).
3. Whether the emission winding check's bare zero is named at rounding as a guard (A; B as optional insurance) or left with its own row (C).
4. Whether the clearance is its own named constant (B) or reads the aspect cap (A leans; C's first report).

Harness figures for the round: A 224,287 tokens (as reported for the resumed run), 181 s; B 242,565 tokens, 488 s; C: see below.
C's round 1: 369,153 tokens as reported for the resumed run, 510 s.

## Round 2, 2026-11-14

Dispatched: all three, resumed. Shown: each the other two round-1 replies, whole, and the four points still apart as the orchestrator read them. Replies are beside this file (`-round2-A.md`, `-round2-B.md`, `-round2-C.md`).

What moved:

- **Point 4 closed.** All three: the clearance is its own named quality constant, 1/16, not a read of the aspect cap (A and C moved to B).
- **Point 3 closed on the mechanism.** All three: the emission winding check's bare zero is named at the certificate's rounding allowance and answers three ways (outward, folded, unresolved). What "unresolved" does on three boundary points is point 1.
- **Point 2 crossed.** A stayed at ε in the validator and B moved to ε (C's round-1 argument: boundary points carry ε of slack). C moved to the rounding allowance, on a census it ran this round over the mesh crate's integration suite on main (45,370 faces, 40.6 million triangles): no triangle is under 1e-9 m in least height or near rounding, and 972 sound triangles, well-shaped ones of tiny features among them, lie between 1e-9 and 1e-6 m, which an ε-scale check would call flat at the ε = 1e-6 row CI sweeps. A and B have not seen that census.
- **Point 1 crossed.** A moved back to keeping a flat triangle of three boundary points (the mesher cannot remove a corner the body gave; its verdict is the body's validation's to give). B holds kept. C moved back to refusing it typed (fail-loud; the outward-winding promise stays whole; the analytic arms already refuse an exact zero). No instance exists: A, B and C each measured none (the issue-111 bodies; 50 meshes; the 40.6-million-triangle census).

Positions moved and did not all meet, so a third round follows. Still apart: point 1 (kept: A, B; refused: C) and point 2 (ε: A, B; rounding: C).

Harness figures for the round: A 235,918 tokens as reported, 48 s; B 257,592 tokens, 146 s; C: see below.
