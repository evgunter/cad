INTERIM, delta review of PR 7096 at 7a6f5bfe6e. Still running; no MAJOR so far. Leaning APPROVE-WITH-FIXES.

- MAJOR-1 cured. Base was the merge base 54938d6a. Bodies: cones of 0.1/1/5/30/45° at 0.05, 0.3 and π rad; a 5° cone ×1000; the bicone, full and 0.05 rad. Splits: every line and each line alone, at 1, 2 and 5 points; placed and unplaced. δ 10 to 0.01.
  - About 1,700 comparable cells. All mesh manifold on head (my own edge census). Head is never denser than base.
  - The same at ε 1e-6 and 1e-12 on head.
- MAJOR-2 is cured but over-cut. With the cut off (my env-hooked mutant), the walk is non-manifold only on the bored ball at δ ≥ 0.1. At δ 0.04 to 0.05 the PR's own row asserts a cut that is not needed.
  - Head still cuts where no cut is needed: the bored torus (bore x=1.6 in a torus of R 2, r 0.5) at δ 0.1–3, a cone-bored ball at δ 0.1–3, and a bead (r 1.2) at δ 0.3–3.
  - Result: 208 of ~1,100 cells mesh denser than base. Example: the bored torus at δ3, walked face 48 triangles, against 26 on base and 16 for the unsplit strip.
  - Not a correctness issue; a density one, against the unit's goal. Details in the final.
- Mutants, all red on named rows: zip off, zip order, route walked-fact off, cut never, cut always, cut threshold at δ alone, headroom kept, and developed_runs with no runs (now 5 rows red, against 2 at the frozen head).

Continuing with the differential, CI and the description.
