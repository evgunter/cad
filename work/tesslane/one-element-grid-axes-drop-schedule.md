---
id: one-element-grid-axes-drop-schedule
kind: issue
title: mesh: one-element grid axes drop the other axis' computed schedule — the issue-685 class, beyond the cone
status: open
opened: 2026-09-01
refs: [685, 1507, 678]
priority: P3
cost: M
---

## From GitHub issue 1513

Opened 2026-09-01; 0 comments.

Filed at MESH-5's state-sync (PR [#1507](https://github.com/evgunter/cad/pull/1507)); both blinded reviews flagged the unscheduled siblings independently.

Issue 685 was one member of a class: every interior-grid emitter in `crates/mesh` runs `for j in 1..nv { for i in 1..nu }` (or its transpose), so a count of 1 on either axis empties the ranges and the other axis' computed schedule is silently dropped. PR #1507 decided the cone's `nu == 1` case (a ruling argument — `cert_cone` row-invariance — makes one strip provably right) and stopped computing the discarded schedule there; the remaining members have no such local argument and stay computed-and-dropped:

- the sphere arm at `nu == 1` (measured watertight, max_dev 0.596·δ on `sphere_wedge(0.3)`);
- the torus arm at `nu == 1` (no in-tree body reaches it; a 0.05-rad wedge measures watertight, 0.011·δ);
- every curved arm at `nv == 1` with `nu ≥ 2` (the mirror; 0.341·δ);
- `trimmed.rs:737` `uniform_candidates` (the second instance of exactly 685's shape);
- next-look: `trimmed.rs:829` per-band `1..b.nuc`, `nurbs_cert.rs:744-745`.

In every measured member the per-triangle certificate is the backstop, so nothing here is a correctness bug — it is 685's sizing-intent silence, N more times. Wanted per member: state at the site why the emitted density is right and stop computing what is dropped (685's shape), or honour the schedule, or route to S29's sizing-policy conversation if no local argument exists — the sphere/torus members look policy-shaped. Site pointers live at `curved::grid_counts`' doc (the class record) and `trimmed::uniform_candidates`; measurements rerunnable via `tools/tess-meter/tests/mesh5_probe.rs`.

Refs: issue 685, PR #1507, S29, issue 678 (adjacent pole floor, distinct).

## Home

`work/mesh/` — every cited site is in `crates/mesh/*`, S-MESH's territory glob, and sizing intent versus budget is its charter line.

## Re-homed at S-MESH's exit (2026-09-16)

Moved from `work/mesh/` to TESS (opened at this exit as S-MESH's successor for the tessellation kernel) when S-MESH closed (`docs/S-MESH-EXIT-WALK.md`); the item's content, id and history are unchanged.

## Moved to TESSSIZE (the curved and trimmed lanes' sizing: which density each lattice emits, and the theorem behind it) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.

- 2026-11-14 — moved from TESSSIZE to TESSLANE by `git mv` when TESS and TESSSIZE merged into their successor TESSLANE (`docs/doc-ledger/tess-and-tesssize-leave-the-tracker.md`), id and body unchanged.

## Re-read against main (2026-11-14, TESSLANE) — re-banded P3, M

A read-only lane re-read every member at `d6e1eb1f`. Nothing was run. No member is a live wrong answer or a refusal anyone has measured: on every path each emitted triangle passes `cert::verdict` (curved) or `TrimmedFace::emit`'s check (trimmed) before `Ok`, so the worst outcome is a typed refusal. That is not P0's definition, and the row's own text said as much. What is left, member by member:

- **Curved sphere, torus, spindle** (`curved::grid_counts` → the interior loop in `curved::tessellate_curved`). The interior loop is empty at a count of 1, but the "dropped" axis' schedule now reaches the mesh through the boundary: `chords::compute_chords` chords every circle side at `chords::tighten`. What is emitted is a boundary-only strip. Left: the lanes' theorem text does not state that shape. `sizing::sphere_grid_steps`' confinement argues from lattice points, its premise "the `ceil` can shrink `h_v` by up to half" is false at `nv == 1`, and where the two opposite chains carry unequal counts the argument needed is the cylinder's strip lemma, which neither doc cites. The spindle is a member the row did not list. The class record at `grid_counts`' doc is stale: it omits the `nv == 1` mirror, the spindle and the per-band loop, and does not say the boundary carries the schedule.
- **Cylinder and bare-sided cone**: no drop. Both return `(nu, 1)` by their theorems (`cylinder_rim_triangle_lands_at_delta`, `cone_grid_step_lands_the_worst_triangle_at_delta`).
- **Cone with a split side** at `nv == 1`, `nu ≥ 2`: still computed and dropped; it has no theorem, which is `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`.
- **Trimmed cylinder and trimmed sphere** (`trimmed::uniform_candidates`; the sphere arm is a new caller through `pole_side_columns`): still computed and dropped on both axes, and the counts still feed `lattice_metric`. Neither arm has a theorem, which is the two `…-has-no-worst-triangle-theorem` rows at P2.
- **NURBS** (`NurbsCellGrid::band_schedule` → `trimmed::per_cell_candidates`; the row's `nurbs_cert.rs` site and its per-band loop are this one member): a band with `nuc == 1` emits no interior point, so its rows, raised ones included, do not reach the mesh, and the `LATTICE_SHARE` theorem's argument and its CDT pin assume lattice points on the neighbouring rows. **Unmeasured**: the settling run is a NURBS face narrower than one band's column step, added to the reach measurement lane's probes.
- `tools/tess-meter/tests/mesh5_probe.rs` still exists, but prints a refusal rather than asserting on it, and covers none of the trimmed or NURBS members.
- The three lattice emitters (`tessellate_curved`, `uniform_candidates`, `per_cell_candidates`) each spell an interior lattice.

Cost drops to M: what this row owes is the strip shape stated and pinned in the sphere, torus and spindle theorems, the class record made true, and the probe made able to fail. The NURBS member is re-banded on its own once measured.

## The NURBS one-column band, measured (2026-11-14)

Run at `d6e1eb1f` (probes `reach_probes::a_one_column_wall_meshes_on_its_boundary_chords_alone` and `a_one_column_band_with_raised_rows_adds_no_lattice_point` on `tesslane/measure-reach`). An ordinary thin blade reaches `nuc == 1`: a 2 × 0.01 rectangle lofted through `sweep::loft_body`. Its narrow walls are one band with one column and no lattice point at every δ tried; raised rows add nothing to the mesh. Every run meshes and passes `check_mesh`: across degree 2 and 3, even and uneven stations, twist 0.01–0.1 and bow 0.02–0.05 at six δ, the largest certificate on a one-column wall was 6.7 % of δ. The mesh's rows are the chord pass's boundary schedule (`chords::tighten`, whole-patch steps), not the band's `nvc`: 458 triangles against 232 scheduled rows on one wall.

So this member is not a refusal and not near δ. What it leaves is the reverse: a one-column wall is meshed by a schedule the band did not choose, far under δ, and the `LATTICE_SHARE` theorem's text does not describe it. No face that refuses or exceeds δ was constructed.
