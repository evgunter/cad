---
id: main-red-a-cap-moved-off-a-frustum-wedge-apex-refuses-pcurve-missing-cache
kind: issue
title: main red: sweep a_plane_moved_to_a_cone_apex::a_cap_moved_off_a_frustum_wedge_apex_plans_its_wall_edge refuses Pcurve MissingCache ×3 at every ε, and its row is closed
status: closed
opened: 2026-11-25
priority: P1
cost: E
closed: 2026-11-26
pr: 7804
branch: inside/frustum-apex-main-red
---


Filed by the LATHE orchestrator: LATHE's one-band lane measured this while merging main into its stack (PR 7414).

`sweep::all a_plane_moved_to_a_cone_apex::a_cap_moved_off_a_frustum_wedge_apex_plans_its_wall_edge`
(`crates/sweep/tests/a_plane_moved_to_a_cone_apex.rs`) is red on main `5e04ac6561` at ε 1e-6, 1e-9 and 1e-12. It refuses
`Pcurve MissingCache` three times, and the same refusal shows on LATHE's merged head. LATHE's stack before the merge
(`ff4308ebb2`) is green on it, so the red came in with main.

The row the test cites, `work/inside/a-cone-wall-whose-rims-span-different-azimuths-refuses-containment.md`, is
closed, so main carries this red with no open row. Owner: INSIDE, per the cited row. EMBED's
`the-apex-census-rows-that-still-refuse-read-cells-short-of-or-off-the-apex` also names the test.

## 2026-11-26: bisected to PR 7497; the test reads tier 3 off an unfinished body

**Payload.** At default ε the panic is `solid_truth::assert_body_is`'s
`topo::validate_geometric` on the body `topo::replace_face_offset` returned
(`FaceKey(1v1) by 9e-10: tier 3: [Pcurve { finding: MissingCache { half_edge:
16v1 } }, … 10v1 …, … 11v1 …]`), raised by tier 3 check 8
(`pcurves::pcurve_pass`). The volume read just above it, on the *finished*
clone, passes.

**Bisect** (`git bisect --first-parent`, merge base `84588ee6fa` of
`ff4308ebb2` and main, to `5e04ac6561`; 296 first-parent commits, 10 steps).
First bad: `414ff4b8e2`, PR 7497 (`walks/finish-completes`). Its first parent
`53116fdf43` is green.

**Cause.** PR 7497 removed `offset_charts_staged`'s closing `mint_pcurves`
(`crates/topo/src/replace_face.rs`): the offset doors leave the rows a move
stales absent, for the finish to complete. A pure tier-3 read of the door's
output then reads the absent rows as missing, which is the contract, not a
defect. PR 7497 re-spelled its callers' raw `validate_geometric` reads as
`test_support::validate_completed`; this test is in the slow set, so 7497's
gate did not run it and it kept the old read.

**Fix.** The test finishes the offset body before the at-rest checks
(`finished(&label, offset, tol)`), the door's documented use. Green at 1e-6,
1e-9 and 1e-12. The kernel is unchanged.
