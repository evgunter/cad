Merged origin/main 184e484ed2 (PR 7052 and later) as edef0cf9bd. Committed locally, not pushed: it goes up with fix pass 2.

**What PR 7052 changed, and what I kept:**
- props/curved.rs: main adds `Curve3::Periodic` beside `Nurbs` in the cylinder, cone and sphere arms, refused `Unimplemented`.
  - Kept my side, where those arms go through the one kind table.
  - The table (`IsoChart::kind_refusal`) and the parses' non-iso arms now name `Periodic` too, refused `Unimplemented` on every chart, torus included.
- chords.rs, memo.rs, sizing.rs: main's `Periodic` arms only (a whole-loop periodic carrier refused `UnsupportedCurve`, its memo tag and key, the sizing kinds). They auto-merged, and all of main's side is kept.
- reach_probes: main re-documents and re-asserts the spline-section row for the periodic fitted loop, and adds `quadric_pair_sections.rs`. Kept whole.

**Next:** geom-brep, mesh and topo compile. Starting fix pass 2, items A to J. The armed tour on 650e2ce14e is still running detached from its built binary; result to follow.
