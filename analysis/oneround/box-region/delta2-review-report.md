# Second delta read of PR 7225 (`oneround/box-region`), reviewer r1

**Frozen head:** `2e9fa0b6`. **Delta:** `1e81f303..2e9fa0b6`. **Main as merged:** `dd8e7019`, the head's second parent. **Hosted run:** 45165602738.

**Verdict: APPROVE-WITH-FIXES.**

The code survives everything I put to it:
- Every mutant I planted on the changed code goes red, but one whose branch cannot be reached.
- Main's four reds reproduce on `dd8e7019` with identical output.
- The plate ledger's re-pin is main's move plus the unit's own +12.
- The merge resolutions drop nothing of main's.
- The gates pass.

My previous MAJOR is fixed: the torus line and its row are in the table. What remains is two wrong sentences in the report-back table that goes to Ev, both shown by building a body. They need text fixes and one row's scope widened.

## What I executed and what I only read

**Executed** at `2e9fa0b6`, debug profile, my own target directory:

1. **The unit's rows at ε = 1e-9, 1e-6 and 1e-12.** These are every `session_box` row (topo, sweep, editor-core, step-import), the frustum and STEP-seam rows, `tier3_tests`, the re-pinned integration rows, `tcost_k3_certificate`, the three `m10_sym_profile_interval` ledger rows, `intent_e`, `verbs_cylsph_opening`, `finish::tests`, geom-brep's `edge_extent_bounds` row and the recourse roster.
   - 217 tests per ε; one of them was my own print-only probe.
   - At each ε, 216 pass. The one failure is `recourse_roster::every_decided_name_has_its_door_and_every_site_is_read`, which is main's (NOTE 4).
2. **Mutants** (`r1-delta2-mutants.py`): eight plants, each run over 207 rows. All 207 pass unmutated.
3. **Probes** (`r1-delta2-probes.rs.txt`, `r1-delta2-topo-probe.rs.txt`): planted in sweep and topo, then removed.
4. **The gates**, on this head: `GATE_BASE=dd8e7019 scripts/run-gates.sh` exits 0 with 67 OK lines. `cover-kind-match` reports 401 sites pinned, 106 reads of 30 doors, and no pin above the base.
5. **`payload-rung-sweep.py --check`** on the head and on main.
6. **`cargo fmt --all --check`** on the head: clean.
7. **On main `dd8e7019`:** the plate-ledger row, `square_search` at seed `0x9c64e8fd5d2bf505`, and the recourse roster.
8. **The hosted run**, read at step level.

**Only read:** `git show --remerge-diff` of the three merges; the PR body's table against `topo::cover`, `geom::cover` and `TorusChartWindow`; the new rows.

**Not done:**
- **Claim 5's 289 / 17 / 85 and 329 / 36:** r2's differential, which I may not read.
- **Clippy:** I did not run it. The hosted lint job **skipped rustfmt, clippy and rustdoc** after its gates step failed, so clippy and rustdoc have not run hosted on this head. I ran rustfmt only.
- **The full fast set.**

**Isolation.** The frozen head and the brief carry summaries of the other reviewer's findings (the PR body, and work rows "found by the r2 review"). I read them as part of the head. I did not read the other reviewer's branch.

## Findings

### MINOR 1: the table's sphere line understates which sphere faces read the whole ball
DEMONSTRATED, `sure`. Claim 1.

**The body.** A ball (r = 1.5, poled on `z`) intersected with the brick `[−1, 1]² × [0, 3]` by `topo::intersect`. Probe `r1d2_a_cut_ball_at_the_wall`.

**What each face reads.** Both of the cut's sphere faces cover the whole ball, `x, z ∈ [−1.5, 1.5]`, even though the cut is z ≥ 0 and x ≤ 1. Its plane faces read tightly.

**At the wall.** Moved +998.7 m in `x`, the body reaches 999.7 m. It is refused `SessionBoxUnshown(0.2)` at the gate and at the door; at +998.9 m, `Unshown(0.4)`. The subtract body, which truly reaches past, is refused `Outside`, correctly.

**What the PR says.** The table's sphere line reads "windowed where its boundary says which side is the face (domes, caps and partial wedges probed all do)", and its row is the pole-to-pole wedge. The sample's 15 sphere cases are pole-to-pole. The filed row, `work/boxes/a-sphere-face-through-both-poles-reads-its-whole-ball.md`, is scoped to faces running pole to pole.

**Why it matters.** These sphere faces come from a plain boolean, a ball cut by a box. They read the whole ball too, and no row or line names them. The "otherwise" branch is stated, so the table is not wrong in form. But its parenthesis and the filed row tell Ev the branch is wedge-only, as the torus line said before this pass.

### MINOR 2: "of the doors I found, only an import leaves [a spline edge] on a plane, cylinder or cone face" is false
DEMONSTRATED, `sure`. Claim 1.

**The bodies.** Two crossed revolved rods (r = 1 about `y`, and r = 0.8 turned onto `x`), under union, subtract and intersect. Each result has `Cylinder ← Nurbs` edges, the boolean's section curves (probe `r1d2_edge_kinds_on_ruled_faces`, through `readback::edge_carrier_kind`).

**Consequence.** Booleans of cylinders leave spline edges on cylinder faces, so the spline-edge line's overshoot reaches ordinary boolean results, not imports only. The overshoot is small: at +997 and +998.5 m every verdict was right.

**An edge kind the table omits.** `Curve3::Periodic`. Its cover arm in `geom::cover::readings` (the whole loop's net, met with a clamped view) is not listed. Whether any door leaves one on a ruled face I did not establish (`unsure`).

### NOTE 1: the mutants (claims 2, 3, 4)
DEMONSTRATED. Each red row is judged against 207 rows that pass unmutated.

| mutant | red rows |
|---|---|
| stage 2 never admits | 4: the zone, the slender-cone and STEP seamed-frustum rows, the seamless band |
| only the first uncovered face named | `nurbs_faces_refuse_typed_at_both_doors`, `a_face_with_no_cover…` |
| uncovered faces skipped | 6 |
| placement filter keeps every candidate | the wedge, the zone, the cut-torus row |
| geom's cylinder axis guard dropped | `a_face_with_no_cover…` (G3's pin holds) |
| one winding loop read as missing no apex | the cone-cap row |
| 3 or 4 winding loops read as missing no apex | **survives** |

**Why the survivor is harmless** (by reading, `likely`). A connected face on one nappe has at most two winding boundary loops, and a face across both nappes has one on each. So three or more is never a valid face, and the branch is unreachable.

### NOTE 2: the refusal sentences (claim 2)
DEMONSTRATED.

**True of each body I built:**
- an infinite sphere radius and a NaN sphere centre name their datum, as check 1 does;
- a brick with two NaN corners gives four findings, one per face, each "an end vertex of the edge is not a finite point…".

**One false of the body** (`sure`; pre-existing by reading, `likely`). A brick with one *infinite* corner is told "the body reaches inf m past the 1e3 m session box … Recourse: place the geometry nearer the origin". The point is not a finite one, so moving the geometry does nothing.
- Cause: stage 3's inside read takes the vertex as a point past the box before the face's uncovered cause is reached.
- Main reads vertices the same way, so I expect the same sentence there.
- It is outside G2's list of four causes, and no row covers it.

### NOTE 3: the cone argument and the one home (claims 3, 4)
BY-INSPECTION, `likely`. The site argument holds:
- each winding loop's hull meets the axis on its own nappe's side;
- two winding loops on one nappe bound only the band, whose rulings run between boundary points.

`face_region(boundary, Apex)` is the one hull statement. The other `.convex()` sites are tests, and `ruled_region` is a chart region, not a hull. The deleted cylinder guard is equivalent to geom's.

### NOTE 4: main's reds, checked on main (claim 7's list)
DEMONSTRATED.

| red | on main `dd8e7019` |
|---|---|
| plate ledger | reads 12370 / 12820 / 12512, a 31fb… digest, and 1051 / 1051 / 554 calls. The head's pin is exactly that +12 forms on the three Assertion/Door lines. So the re-pin is main's move plus the unit's own (claim 5) |
| `square_search`, seed `0x9c64e8fd5d2bf505` | 4 mismatches |
| recourse roster | the same five lines the head prints: `chain_parity`, `collapsed_side_rows` ×2, `pcurves/decision.rs`, `spline_trim` |
| `payload-rung-sweep --check` | prints the same `NodeErrorClass` line on main and on the head; nothing of `UncoveredCause` |
| dead slow-set entry | `e6_drive_over_a_loft_at_interval::an_arc_loft_bisects_to_its_budget_under_a_box_elsewhere` is on main's `nextest.toml`, and its test was deleted by main's `85a85f0f` |

### NOTE 5: the merge (claim 6)
BY-INSPECTION, `sure`. The remerge diffs adopt main's `EdgeView` refactor in `session_box.rs` and `cover.rs`, and main's `NoRegion` arm in step-import's `reach.rs`. Nothing of main's is dropped.

### NOTE 6: the plate re-pin absorbs main's unexplained move
`sure`. The four new calls PR 7434 added are not explained anywhere. The unit pins them, so main's red turns green when this merges, while the cause stays on its DOCTAIL row. That is disclosed, and P2 is right for it.

## Style

**Questions exercised:**
- Q1: one home (NOTE 3).
- Q3: mutants (NOTE 1).
- Q4: the table against the code (MINOR 1 and 2).
- Q5: sentences against bodies (NOTE 2).
- Q6: the re-pin (NOTE 6).
- The fresh-instance question: no second rule statement and no deferral. Rows that lost a subject now restore it on the checks.

**Ratings:**
- **Idiom and structure, 4/5.**
- **Tests, 4/5.** Every mutant that can matter goes red.
- **Doc honesty, 4/5.** Two table sentences are too narrow (MINOR 1 and 2).
