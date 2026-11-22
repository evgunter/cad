# Design fork: a side two faces can both make (TESSLANE), 2026-11-22

Protocol: ddd201f411f (`git log -1 --format=%h origin/main -- docs/DESIGN-FORK-PROTOCOL.md`, after a fetch).

Dispatch draws, two /dev/urandom bytes:

- arm byte: 82 (below 86 → three designers, otherwise two) → **three designers**
- label byte: 152 (not 255; 152 mod 3 = 2) → **Fable = C; the two Opus designers are A and B**

Problem statement handed to every designer: one text, held as `tesslane-a-side-two-faces-can-both-make-problem.md` beside this file. Each designer works in its own git worktree of `tesslane/ruled-density` at `a020a3edca` (PR 7096's closing head, not yet merged: its hosted run was queued behind about 90 others) and is told to read no pull request's page or thread and no branch under `analysis/`. The three share one build directory (11 GB of disk free at dispatch).

The row this fork weighs: `work/tesslane/the-mesh-cures-a-side-two-faces-can-both-make-three-ways.md` (P1, design), filed at PR 7096's closing.

## First reports

Recorded as delivered, before any reconciliation. Dispatched 2026-11-22 23:10Z, concurrently, as subagents of the orchestrator's session.

- **A** (Opus): `tesslane-a-side-two-faces-can-both-make-report-A.md`. Delivered 23:15Z; 150,820 tokens, 19 tool calls, 270 s. Read the code; measured nothing. **Recommends (likely): split at the join.** Each face meshes reading nothing about the faces across its edges; the census finds each side two patches both use as an inside edge; a second pass has EVERY such patch split that side at a point of its own surface over the side's midpoint (two triangles become four, certified as any triangle of that face), with no rebuild, one pass. The three present answers retire; the memo key folds the split set; `OverusedSide` becomes a panic. Rejected the framing: the question names the wrong fact (which stand-in to pick; the fact is whether the neighbour DOES make the side, known only at the join). Ratified text: would change `lib.rs`'s purity sentence (agent-written, PR 7241); no README clause. Off the question: the walked lane reads the neighbour's surface and the memo key does not fold it.
- **B** (Opus): `tesslane-a-side-two-faces-can-both-make-report-B.md`. Delivered 23:17Z; 141,548 tokens, 22 tool calls, 372 s. Read the code; measured nothing. **Recommends (likely): avoid a side only where it is actually made, decided at the join.** Lanes read nothing of neighbours; the census is the detector; every face using an overused side as an INTERIOR side is meshed again with that side on an avoid list (the triangulating arms, the planar and the rectangle lane insert a point and rebuild; the walk cuts), in rounds until nothing is overused; the memo key folds the avoid list. Unsure whether both faces or one give way; leans both. Rejected the framing the same way (each lane predicts its neighbour; stop predicting). Corrected the brief: the memo-key row is closed (PR 7241 folds the face across); the rectangle lane does triangulate. Ratified text: the same `lib.rs` sentence. Off the question: the same unfolded-surface defect.
- **C** (Fable): `tesslane-a-side-two-faces-can-both-make-report-C.md`. Delivered 23:52Z; 269,861 tokens, 50 tool calls, 2,487 s. **Measured** with throwaway switches over the mesh crate's default suite at the default ε. **Recommends (likely): the join divides the side.** No face reads its neighbours; at the join, a side more patches make as an interior side than the mesh can hold is split at its 3-D midpoint in all but one of them, with no certificate (the new triangles are pieces of triangles already certified) and no lane re-entered. Measured: lane rules off and this cure on, 302 of 307 rows pass (the failures pin today's cut counts, plus two needle-cone cells that need a fold given its own inserted point); the walk rows take 406 cuts where today's rule makes 1,211; the split bored ball at δ 0.1 is 134 triangles against 220. A lesser alternative it names and would not build unless Ev wants the sheets parted geometrically: the join detects and the face's own lane cures on its surface (in substance B's and A's). Two things it says the answer needs beside it: a chord floor (two edges joining one vertex pair never chord to one segment: the seven refusing bored-ball cells hold that second defect, which the census passes and `check_mesh` refuses) and the fold insert. Corrections to the premise: that second defect; the memo-key row is closed; the walk's reading makes the mesh depend on ε against `[tolerance.chordal]` and `[mesh.winding]`; the walk's reading reads a surface the memo key does not hold; today's rule on the triangulating arms inserts 997 points in 103 rebuilds where about 502 sides are actually both made. Ratified text: the same `lib.rs` sentence; no README clause.

### Agreement after the first reports (orchestrator's reading, 23:55Z)

All three reject the framing in the same way and name the same final state in kind: no lane reads anything about the faces across its edges; the fact that two faces both made a side exists only where the patches join, and that is where it is acted on; the three present rules and the neighbour entries in the memo key retire. None proposes a README clause; all three would rewrite the same agent-written sentence in `lib.rs`.

They differ on the cure, so a second round runs:

- **What the cure is.** A: a local split at a point of the face's own surface, certified, in a second pass, no rebuild. B: the face meshed again by its own lane with an avoid list (insert and rebuild, or the walk's cut), in rounds. C: a split at the side's chord midpoint done at the join itself, no certificate, no lane re-entered.
- **Which patches give way.** A: every patch that has the side as an inside edge. B: leans every such patch, unsure. C: all but one.
- **What it needs beside it.** Only C found the chord floor (measured) and the fold insert.

One finding of C's was acted on at once outside the fork: PR 7096's `may_make` read ε to choose between two meshes, against `[tolerance.chordal]`; the orchestrator held that pull request's merge and had the read removed.

## Round two

