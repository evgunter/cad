# ISO leaves the tracker — 2026-10-26

ISO (the trimmed and rational lane: the iso derivation, the trimmed
quadrature's composite rounds, the rational gates and the loft seam's
exact knot compare) was opened on 2026-09-20 at CHART's priority-seam
cut. It cut its P2–P3 tail into ISOTAIL on 2026-10-23, then closed
every P0 and P1 row it kept (PRs 5629, 5630, 5634, 5635, with the reds
they left fixed by 5638, 5642, 5646 and QUAD's 5640).

What remained was four rows on ISOTAIL's one file
(`crates/geom-brep/src/edge_nurbs.rs`) and lane, so the 2026-10-26
tracker sweep merged ISO into ISOTAIL. Every row moved to
`work/isotail/` by `mv`, ids kept, closed rows included (a QUADTAIL row
still refs one of them):
- `a-loft-drive-at-interval-refuses-every-leaf-since-the-one-polynomial-weight-test` (P2);
- `a-loft-whose-arc-bulge-varies-per-section-refuses-its-arc-rim` (P2);
- `trimmed-lane-inner-rule-has-no-composite-arm` (P3);
- `the-tour-carries-two-patch-quadratures` (P4);
- nine closed rows.

Its plan set no `## Exit criteria`, so it closes without an exit walk.

Recover SHA `bb25fa4f0b5777e23a2d9c68c6b3d147bccf1cf2`.

    git show bb25fa4f0b5777e23a2d9c68c6b3d147bccf1cf2:work/iso/<FILE>
