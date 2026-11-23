ISO-RUN-DENSITY fix-pass progress. Local only, not pushed. main is merged in.
Done:
- **A**: the correction, plus the unit-file section (the two-hole class, main's two cells, your 07:28Z ruling, the measurements).
- **B1–B7**: the four relation rows are replaced by one identity. Every face of a constrained triangulation with n polygon points, h holes and p inserted points holds T = n − 2 + 2h + 2p (Euler, derived in the file header; on this lane seams count twice and no two-copy triangle is dropped on a cylinder). It is asserted as an equality on every cylinder face, with p read off the patch, and **p = 0** on every cut wall, tilt and split wall. That makes "p = 0" mean "no rows and nothing refused".
  - Bodies: comb of six notches, bars of half-width 0.3/0.6/0.9 r, tilts 0.05/0.3/0.8/1.25/1.35 at δ 1e-1/1e-2/1e-3, the split walls, and the two-hole cells (p = 4, one round, on each of the 8; main's 2 have p = 0).
  - **Exact equality held on all 84 + faces probed. Mutants: rows laid again → all three rows red; correction off → the two-hole row red.**
  - The plain-wall ratios are now prose (measured, with the review's numbers for the bars, comb and tilts).
  - Row 4 is gone: notched_wall and holed_wall joined d9_mesh_goldens' corpus, green at all three ε. The private FNV is deleted.
  - TARGET_SHARE is dropped: the two-hole builder reads the column off a chart-lane wall meshed at δ, so the lane's target sets it.
  - reach_probes' 3.25n cap is replaced by the same identity, with the cure's numbers marked measured.
  - The shared helpers (walls, tilts, two holes, `Count`) are in tests/common/rowless.rs, so reach_probes no longer imports rowless_cylinders.
- **D**: every listed site fixed.
  - Class sweep. Pattern: `grep -rnI -i cylinder` over crates/mesh/src, the mesh README, docs/TESS-BUDGET.md and the two tesslane items, filtered for lattice|rows?|grid|lever|strip. Hits and disposition:
    - fixed: chords.rs:22, chart_table.rs:590 and :15, trimmed.rs:283/369/1544, curved.rs:1096;
    - dated notes appended to `the-trimmed-cylinder-lane-has-no-worst-triangle-theorem.md` and `the-trimmed-cylinder-arm-chords-its-boundary-to-no-lattice.md`;
    - plan.md's 4b row updated;
    - left alone, as they are about the curved lane: TESS-BUDGET.md:727 and curved.rs:920/1082/1106.
- **E**: filed and `work.py lint` green:
  - `tesslane/a-cone-and-a-spline-cylinder-keep-the-rows-a-cylinder-face-no-longer-lays` (P3);
  - `tesslane/two-rods-a-hair-apart-crossing-at-a-shallow-angle-refuse-triangulation` (P3, marked as found by review, not re-measured by me);
  - `green/issue555s-skewed-prism-reads-its-skew-through-powi` (P4; territory gives ciw, green and tesslane, so I chose green).
Running: **C**, the metric table. The current metric has finished the 900-cell sweep (0 refuse) and the mesh suite (552/552). Still to run: current on sweep's modules, then the development (k = 1/r) and main's pre-unit metric on all three. The table follows; it decides whether I change the metric.
After C: F (PR body) and G (the full pre-push on the merged head, including the pick tally re-measured beside a base run on merged main).
