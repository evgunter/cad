---
id: one-element-grid-axes-drop-schedule
kind: issue
title: "mesh: one-element grid axes drop the other axis' computed schedule — the issue-685 class, beyond the cone"
status: open
opened: 2026-09-01
github: 1513
refs: [685, 1507, 678, S29]
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
