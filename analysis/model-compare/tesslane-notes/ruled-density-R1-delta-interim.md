INTERIM (delta re-review, PR 7096 @ 7a6f5bfe6e; base = merge base 54938d6a95) — about 45 min in, continuing.

Verdict so far: APPROVE-WITH-FIXES; no MAJOR found yet.

- MAJOR-1 cured (claim 1 held so far). My whole first-review oracle re-run on the fix head (edge census, polylines, analytic winding, sampled deviation, volume, count relation): cones 14,635 / 16,320 / 9,450 meshed at ε 1e-9 / 1e-12 / 1e-6 with zero failures; the 1°, 5°, 30°, 45°, 0.1°, 0.001° one-chord cells and the bicone mesh at scale 1 (ε 1e-12) and scale 1000 (default) and hold the count relation. Frusta/walls 31,115 / 35,672 / 19,551, zero new failures.
- MAJOR-2 cured (claim 2, the regression part). Bored ball R 1.05 / 1.3 / 3, both signs, full / half / 0.5 rad, plus bands and D-prisms: 2,868 head cells, none non-manifold; 48 cells non-manifold on base are manifold on head; no cell manifold on base is non-manifold on head.
- MINOR so far (DEMONSTRATED): the cut costs density above base where neither tree was non-manifold. Bore R 1.05, torus neighbour, seam split at 0.5: δ 0.1 base 160 triangles (bore 72), head 220 (bore 132); δ 0.06 base 208 (92), head 290 (174). 201 cells mesh above base in all, every one at δ at or past the shell thickness, which is where the rule cannot exclude the neighbour.
- NOTE (DEMONSTRATED; also true at the frozen head, I missed it then): 36 D-prism cells (0.5 rad at δ 0.1, 0.2 rad at δ 0.03) mesh on base and refuse on head, exactly as the UNSPLIT body refuses on both trees (its caps are two coincident chords); base meshed them only because the headroom chorded the rim twice.
- BY-INSPECTION: `walk_face`'s doc (trimmed.rs ~:3520-3545) still describes the removed centroid form ("replaced by the three joining its sides to the surface point at its chart centroid") and lists `MissingEntity` where the code returns `RouterDefect`.
- Hosted CI on 7a6f5bfe6e (run 44216222633) at 02:00Z: `test (app)` success (was red); lint, tools, step import, corrupt input, viewer wasm success; `test (kernel)`, python, demos, budget meter in progress.

Still to run: per-patch digest differential, mutants, claims 3, 5–9 by reading, the description.
