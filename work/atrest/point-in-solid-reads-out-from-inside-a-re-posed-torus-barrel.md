---
id: point-in-solid-reads-out-from-inside-a-re-posed-torus-barrel
kind: issue
title: point_in_solid_faces answers Out for a point strictly inside a rigidly re-posed torus-walled shell (the hollowed torus barrel), where the unposed body answers In
status: dispatched
opened: 2026-09-24
priority: P0
cost: D
refs: [tier-3-does-not-check-shell-roles-per-solid]
parent: ATREST-9
---



Found by ATREST-7 (branch `atrest/7-shell-winding`, 2026-09-24) while
measuring tier 3's new check 10 (shell winding) over the corpus, and
filed here because the walk is `crates/topo/src/boolean/solid_contain.rs`,
CONTACT's ground.

## What was measured

`crates/sweep/tests/torax_axial.rs`'s
`torax_the_torus_corners_survive_a_rigid_re_pose` hollows the torus
barrel (`torus_barrel()`: a full revolution with a concave torus wall,
`topo::shell(…, T = 1/128)`) AFTER a rigid re-pose — rotation 0.7 rad
about the x axis through `(1/4, −1/2, 1/8)`. The result is one solid,
two shells: the Outer wall (6 faces) and the cavity (6 faces). The
row's own bijection pins it to the unposed hollow re-posed, to 1e-15 m.

With check 10 in the validator, the verb's own validation refuses this
body: check 10 probes a vertex of the cavity shell,
`(−0.027458739263761148, 0.05257824836935659, 0.42699783043145134)`
(unposed: `(−0.0275, 15/128, 0.0)` in the revolve frame — the offset
top cap's rim, radius ≈ 0.0275, strictly inside the wall, whose radius
at that height is ≈ 0.038, and `T` below the top cap), with
`point_in_solid_faces` over the OUTER shell's six faces
(`SolidFaces::of_shell`). The walk answers **`Ok(Out)`**. The point is
inside that shell, so the answer is wrong. On the UNPOSED hollow the
same check certifies (the row's `hollowed` helper asserts
`validate_geometric` is `Ok` before the re-pose, and that assertion
held on the same CI run), so the defect is pose-dependent.

CI evidence: run 36038053269 (the row red at every eps row —
default, 1e-6, 1e-12 — in shard 1/2), and the debug run 36046972095 on the throwaway branch
`atrest/7-debug-scratch`, whose print is quoted above.

## Why it matters

`Out` from inside is a FALSE answer, not a refusal, so no posture that
is silent on `Err` can absorb it. It reaches every consumer of the walk
— the boolean's containment fallback and the census's material test as
well as check 10. The walk's `Out` comes from either a closest crossing
read as ENTERING material or a ray that crossed nothing and read the
at-infinity side; both mean a crossing the ray should have made on the
torus wall or a cap was dropped or misread in this pose. Not narrowed
further: the torus arm's chart windows (`torus_chart_trim`,
`point_on_torus_in_face`) and the ray×torus quartic under a general
frame are the first suspects, not a finding.

## Claimed by ATREST, 2026-09-24 (ATREST orchestrator)

Filed on CONTACT's slate by ATREST-7's lane (on its branch) and
claimed onto ATREST's before it ever reached main, carried by
**ATREST-9**. `crates/topo/src/boolean/solid_contain.rs` remains
CONTACT's ground and the unit announces the seam; the claim is about
who owns the WORK, because CONTACT is `ready` with no orchestrator and
nothing dispatched, and this is a P0 false answer in a certified walk
that ATREST's check 10 (ATREST-7, parked on this row) and the
boolean's containment fallback both read. Leaving it on a slate nobody
is working would leave both waiting indefinitely.
