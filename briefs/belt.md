# Lane `heldrecords/belt` — P1, design first

Row: `work/heldrecords/the-junction-end-window-refuses-in-a-belt-around-a-neutral-point.md` (on origin/main; read it in full, with the P0 row `a-pipe-does-not-union-with-an-elbow-revolved-off-its-own-cap.md`'s fix-pass sections). Code: `geom_brep::curve_face::end_root` and `topo::boolean::reduce::junction_end_window` (merged on main in PRs 7463/7471). Branch `heldrecords/belt` off origin/main; PR base main.

The elbow unions, but an elbow whose axis is turned within about 2e-7 to 2e-4 rad of a neutral point (in band units: ≈200ε … 7√ε at ε 1e-9) refuses `JunctionEndUnread { Window }`: the end root's order is decided, but no halved window holds its spread against the quartic tail (the window certifies one term against the rest). A user turning an elbow by 1e-5 rad meets it — a normal verb failing on normal geometry.

**Design first, no production code**: write a dated `## Design` section on the row and send it to the orchestrator. What it must say:
- the cause, measured (which term, which bound, at which δ);
- the certificate that closes it — the row says "a joint positivity certificate over c₂..c₄"; is that the right object, or is there a simpler one (e.g. the root-count of the deflated polynomial on the window via Sturm/Descartes on the interval, a sign-definite factorisation near the neutral point)? Every bound enclosed, every decision under a named predicate, in-band refuses typed;
- what it leaves refusing, and why that remainder is in band (not a construction gap);
- the pins: a δ-scan in band units at three ε that goes from refusing to building across the belt; mutants that turn it red.

Review tier: dual (two Opus) — certified numerics. Standing rules: `briefs/common.md`. Evidence is local (Ev, 2026-11-23: merge on local evidence, skip CI). Report to `session_018u3gVKp4QiofNJ9Z1mzgD8`.
