---
id: shell-cavity-gate-over-refuses-three-valid-bodies
kind: issue
title: SHELL-4's cavity gate over-refuses three valid bodies: the corpus pot at Interval (sphere/cylinder vertex Departure), the klein elbow's cavity (Departure at both scalars) and the cone frustum's cavity (pair budget)
status: dispatched
opened: 2026-10-30
priority: P1
cost: H
branch: clear/cavitydiag
---


## Finding

SHELL-4's lane, `clear/shell4` at `7d97070458`. The gate is
`cavity_embedded` in `crates/topo/src/shell.rs`: a strictly-positive
self-intersection sweep over the moved clone, with a 65 536-pair budget.
It turns three existing green rows red. Each body is valid:

1. **The corpus pot**, a spherical belly over a cylinder foot.
   - Row: editor-core
     `lib_g17_shell_node::both_documents_evaluate_green_at_the_interval_scalar`.
   - The OPERAND itself refuses at `Interval`:
     `Uncertified { 3v1 sphere, 7v1 cylinder, Vertex(3v1), Departure }`,
     `cornered = 7`.
   - At `f64` it holds, `cornered = 0`.
   - This is RIMSCALAR's class (the `Interval` reading diverges from
     `f64`) at a new site: a sphere and a cylinder meeting at a vertex.
2. **The klein elbow.**
   - Row: sweep
     `torax_interval::interval_the_klein_elbow_hollows_to_its_measured_volume`.
   - The operand holds at both scalars.
   - The shelled cavity refuses `Uncertified Departure` at both,
     immediately, after 0 splits.
3. **The cone frustum.**
   - Row: sweep
     `sf2b_interval_probe::interval_offset_charts_together_cone_frustum`.
   - The operand holds at both scalars.
   - The cavity exhausts `Budget(Pairs 65536)` at both, taking 8.5 s,
     with `outside = 41 429`.

**SHELL-4 is parked on this row.** Over-refusing legitimate shells is
ruled out by `shell-curved-wall-clearance-window`. Unit CAVITYDIAG
(`docs/CAVITYDIAG-SPEC.md`) diagnoses each of the three and fixes what
it can.
