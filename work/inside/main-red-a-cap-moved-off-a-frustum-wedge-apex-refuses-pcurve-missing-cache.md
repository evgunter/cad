---
id: main-red-a-cap-moved-off-a-frustum-wedge-apex-refuses-pcurve-missing-cache
kind: issue
title: main red: sweep a_plane_moved_to_a_cone_apex::a_cap_moved_off_a_frustum_wedge_apex_plans_its_wall_edge refuses Pcurve MissingCache ×3 at every ε, and its row is closed
status: open
opened: 2026-11-25
priority: P1
---


Filed by the LATHE orchestrator: LATHE's one-band lane measured this while merging main into its stack (PR 7414).

`sweep::all a_plane_moved_to_a_cone_apex::a_cap_moved_off_a_frustum_wedge_apex_plans_its_wall_edge`
(`crates/sweep/tests/a_plane_moved_to_a_cone_apex.rs`) is red on main `5e04ac6561` at ε 1e-6, 1e-9 and 1e-12. It refuses
`Pcurve MissingCache` three times, and the same refusal shows on LATHE's merged head. LATHE's stack before the merge
(`ff4308ebb2`) is green on it, so the red came in with main.

The row the test cites, `work/inside/a-cone-wall-whose-rims-span-different-azimuths-refuses-containment.md`, is
closed, so main carries this red with no open row. Owner: INSIDE, per the cited row. EMBED's
`the-apex-census-rows-that-still-refuse-read-cells-short-of-or-off-the-apex` also names the test.
