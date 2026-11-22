# Problem: a side two faces can both make (TESSLANE, 2026-11-22)

You are a designer. Read `docs/prompts/designer.md` in full before anything else; it binds you. This file states a problem, not solutions: the options are yours to find.

## The tree

Your worktree is `tesslane/ruled-density` at `a020a3edca` (pull request 7096's closing head: reviewed and approved, waiting only on a hosted run to merge; treat it as main). Read no pull request's page or thread, and no branch under `analysis/`. The code, `docs/`, the crate README pages and the `work/` rows are yours to read.

## What happens

`mesh::tessellate` meshes each face on its own, a patch per face, and joins the patches by the ids of the boundary vertices they share. Two faces that share two boundary chains can each make, as an INTERIOR edge of its own triangulation, the same straight side between a point of one chain and a point of the other. The joined mesh then has an edge of four triangles. Until 2026-11-21 `tessellate` returned such a mesh as `Ok`.

## What exists today

Three answers, in three places, and a backstop:

1. **The triangulating arms of the trimmed lane** (`TrimmedFace::emit` in `crates/mesh/src/trimmed.rs`). A kept triangle may not have a side joining two polygon points that are not loop neighbours and that one face across the loop's edges holds both of (`TrimPolygon::faces_holding_side`). Where it has one, a point is inserted and the face rebuilt; past the retry budget the face refuses `UnclearedSharedSide`.
2. **The walk** (`trimmed::walk_face`, `crate::ruled_walk`: a ruled face with a split straight side is meshed row against row with no triangulation step). The same test, then a second reading, `trimmed::may_make`: the side is cut at its chart midpoint only where the face across could reach it (the midpoint is not decided farther than `δ·(1 + cert::ALLOWANCE_SHARE)` from that face's surface).
3. **The rectangle lane** (`crates/mesh/src/curved.rs`). No rule.
4. **The backstop:** a census where the patches join (`tessellate::joined`, `side_uses`, `overused_side`) refuses any mesh with a side used past twice per chord: `TessellateError::OverusedSide`.

## What was measured

- **The rectangle lane refuses a valid body.** A ball of radius 1.05 bored through at radius 1, with no split: the bore is the rectangle lane's one strip, the outer face (a sphere or a torus face, by the profile's sign) holds both of its rims, and `tessellate` answers `OverusedSide` on seven cells (δ 3, 1 and 0.3; and 0.1 with the sphere). It meshes at finer δ; a ball of 1.3 meshes at every δ run. `work/tesslane/a-strip-against-a-neighbour-holding-both-rims-makes-an-edge-of-four-triangles.md`.
- **The walk's reading cuts where no cut is needed.** With its cut switched off, the census refuses on exactly fourteen bored-ball cells. The reading also cuts at finer δ on that ball, on every cell of a 1.3 ball, a bored torus, a cone-bored ball and a bead that it cuts at all, and on a one-chord band face beside a wider face of the same cylinder: in those cells the other face makes no such side. Two independent reviews counted 256 of about 2,640 and 440 of 19,426 cells denser than the same body on main, by 2 to 16 triangles. The number of cuts is pinned per cell (`crates/mesh/tests/split_rulings.rs`, `holds_the_relation`).
- **The first answer was not counted** for how often it inserts a point for a side the other face does not make (`work/tesslane/a-walked-sides-reach-exclusion-is-read-only-by-the-walk.md`, which also states the premise the walk's reading rests on and where it is not shown).
- **A patch already depends on which faces lie across its edges**, and the patch memo's key does not hold that: `work/tesslie/the-patch-memo-key-misses-the-faces-across-a-trimmed-face.md` (it cites the per-face purity invariant in `crates/mesh/src/lib.rs`).

## What needs deciding

What the mesh's answer to such a side is, as a final state of the code, for every lane that makes triangles (planar, rectangle, the trimmed lane's triangulating arms, its walk, the spline arm): what a face reads about the faces across its edges, what is done where a lane has no triangulation to rebuild, what meshes and what refuses, and what that does to the per-face purity the memo rests on. The measured costs above are evidence of where the code is flawed, not the edge of what may change.

## Where to look

- `crates/mesh/src/trimmed.rs` (`faces_across`, `faces_holding_side`, `TrimmedFace::emit` / `attempt`, `walk_face`, `may_make`), `crates/mesh/src/ruled_walk.rs`, `crates/mesh/src/curved.rs`, `crates/mesh/src/planar.rs`, `crates/mesh/src/tessellate.rs` (`route_of`, `joined`, the census), `crates/mesh/src/memo.rs`, `crates/mesh/src/lib.rs`.
- `crates/mesh/README.md` (`[mesh.sizing]`, `[mesh.winding]`), `docs/DESIGN.md` (`[failure.taxonomy]`, `[loud-failure]`, `[unrepresentable]`).
- Rows: the four named above; `work/tesslane/the-mesh-cures-a-side-two-faces-can-both-make-three-ways.md` (this question's row); `work/tesslane/a-ruled-face-with-a-split-side-meshes-at-the-density-of-the-face-without-it.md` ("Closed"); `work/tesslane/rectangle-faces-move-to-the-chart-lane.md`; `work/tesslie/the-shared-side-rule-on-the-spline-arm-has-no-witness.md`; `work/tesslie/a-cone-face-near-its-apex-meshes-an-edge-of-four-triangles.md` and `work/tesslie/two-faces-cutting-one-ear-of-a-shared-chord-chain-leave-a-four-use-edge.md` (closed; their history).
- Rows that pin the behaviour: `crates/mesh/tests/split_rulings.rs` (`a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge`, `no_walked_triangle_has_an_edge_a_neighbour_makes`, `a_side_whose_reach_the_band_cannot_decide_is_cut`), `crates/mesh/tests/reach_probes.rs` (the `…keeps_no_side…` rows), `tessellate`'s `side_census` unit rows.

## Working rules

- The box has little disk and is shared with two other designers and a build. Reading is expected to carry most of this. If you must run something, use the one shared build directory and nothing else: `CARGO_TARGET_DIR=/home/user/cad/target cargo test -p mesh --test all <filter>` from your worktree (cargo serializes concurrent builds; expect to wait). Create no other target directory.
- Write scratch files only under your own scratch directory (named in your dispatch). Commit and push nothing. Edit nothing outside your worktree and scratch directory.
- Your report follows `docs/prompts/designer.md` §5 (`## For Ev`, then `## For the orchestrator`), at most 150 lines, as your final message. Name no model.
