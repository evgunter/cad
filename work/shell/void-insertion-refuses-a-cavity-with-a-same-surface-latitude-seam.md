---
id: void-insertion-refuses-a-cavity-with-a-same-surface-latitude-seam
kind: issue
title: shell's void insertion refuses a cavity whose chart carries a same-surface latitude seam (collinear cap plane, two-arc sphere) that the axial door already offset validly
status: open
opened: 2026-09-08
---


Measured in SHELL-7's fix pass (`crates/sweep/tests/shell7_dump.rs`,
`collinear cap` and `two-arc sphere`; pinned in
`crates/sweep/tests/shell7_seam_corner.rs`). Two door-built operands
whose one surface carries a SAME-SURFACE latitude seam — a drum whose
top cap has a collinear profile vertex at `(r/2, h)` (one plane in
four faces, a latitude ring between them), and a sphere authored as
two cocircular arcs (one sphere in four faces, a latitude seam at
`v = π/4`) — pass `topo::offset_charts_together` after the fix pass:
the cavity alone is tier-3 valid at the closed form
(`π(r−t)²(h−2t) = 5.387046002743097`; `4/3·π(r−t)³ =
3.591364001828733`). `topo::shell` on the same operands refuses
DOWNSTREAM of the door: the cap at the void-insertion door's graft
re-certification (`ShellError::Insert`, "certification:
ChartResidual residual at sample 1 definitely exceeds the tolerance
band"), the sphere at the assembled thin solid's tier 3
(`ShellError::NotValid`, `Pcurve { LoopDiscontinuity { half_edge:
HalfEdgeKey(16v1) } }`). The same class through a cylinder (a
collinear WALL vertex) and a cone (a collinear generator vertex)
shells end to end, so what differs is a chart whose latitude seam is
a `Chart`-described circle on a plane or a sphere carried into the
void's flipped orientation. Site to read first: `insert_void`'s graft
re-certification and the pcurve mint on the flipped cavity
(`crates/topo/src/shell.rs`, `crates/topo/src/pcurves.rs`). SHELL's
fence; not this unit's (the corner and the carrier are correct on
both, measured).


## Diagnosed (SHELL orchestrator, 2026-09-08)

Two defects, measured by a diagnosis lane (`shell/9-probe` @
`0cbb6593c`, `crates/sweep/tests/shell9_probe.rs`): the drum fails
inside `Body::revert` (plane chart images and rows are not mirrored
with the negated normal) — TOPO's,
`work/topo/revert-does-not-mirror-plane-chart-images`; the sphere
fails because `shell` runs no closing pcurve mint after the void
door's `Transfers` — SHELL-9. This item closes when both have landed.


## Sphere half closed (SHELL-9, 2026-09-08)

`shell` / `shell_open` now run the closing pcurve mint
(`crate::pcurves::mint_pcurves`) once on the assembled body, after
the last surgery and before `validate_geometric` — the producer's
half of the void door's `Transfers` posture, as the boolean and the
revolve already ran it. The two-arc sphere shells to its closed form
(`shell7_seam_corner::a_two_arc_sphere_shells_to_its_closed_form`:
cavity `4/3·π(r−t)³ = 3.591364001828733`, thin solid
`4/3·π(r³ − (r−t)³) = 0.5974262029576595` to `1e-12`, tier-3 valid,
tessellates); the mint's refusal is typed as `ShellError::Pcurve`.
The drum half stays open on TOPO's
`revert-does-not-mirror-plane-chart-images` — the graft's
re-certification refuses before any pcurve pass runs, so no closing
mint can reach it — and
`shell7_seam_corner::a_collinear_cap_vertex_drum_is_taken_by_the_door_and_stops_at_void_insertion`
keeps refusing `ShellError::Insert` naming that item. This item
closes when the drum half lands.
