---
id: revert-does-not-mirror-plane-chart-images
kind: issue
title: Body::revert negates a plane's normal but leaves its Chart images and cache rows unmirrored, so a same-plane Chart edge fails certification on the reverted body
status: open
opened: 2026-09-08
---



Measured by the SHELL program's diagnosis lane (2026-09-08; probe rows
on branch `shell/9-probe` @ `0cbb6593c`, `crates/sweep/tests/shell9_probe.rs`,
rows 1–2) and placed here by the SHELL orchestrator. `Body::revert`
(`crates/topo/src/revert.rs:230`) negates every `Plane`'s `normal`
with `u_ref` and `origin` unchanged; the module doc says the frame
"stays right-handed with `v_ref` flipping alongside"
(`revert.rs:23`). `Surface::Plane::eval` derives `v_ref = normal ×
u_ref` (`crates/geom/src/surfaces.rs:340`), so the reversal MIRRORS
the plane's chart, `(u, v) ↦ (u, −v)` — and neither the `Chart` edge
descriptions on that plane nor the plane faces' pcurve cache rows are
transformed with it. The contract at `revert.rs:79` ("every
certification survives the map") is therefore false for a `Chart`
image on a plane with a non-zero `v` channel. Measured on the cavity
`shell` builds for a drum whose top cap carries a collinear profile
vertex (one plane in four faces with a latitude ring between them):
the door's cavity is tier-3 valid; `validate_geometric` of its
`revert()` alone fails `EdgeCertification { ChartResidual, sample 1 }`
on the ring's two half-circles (carrier `Circle { center (0, 1.95, 0),
axis +y, r 0.5 }`, image `Harmonic { p0 (0,0), pa (0.5,0), pb
(0,0.5) }` on the reverted plane) with residual `0.3827 = 2·0.5·sin(π/8)`
at `t = π/8`, while the six radial `Chart` LINE edges (zero `v`
channel) pass — which is why samples 0 and 8 on the `u_ref` axis read
zero. Re-deriving the image against the reverted plane certifies, and
the re-derived image is the stored one with `v` negated at all nine
samples. The graft re-runs the same meter with the image verbatim
(`crates/topo/src/boolean/combine.rs:405` → `:440` →
`crates/geom-brep/src/certify.rs:1859`), so `insert_voids` refuses
`Recertify` — `shell`'s pinned `ShellError::Insert` on that drum
(`shell7_seam_corner::a_collinear_cap_vertex_drum_is_taken_by_the_door_and_stops_at_void_insertion`).
Cylinder, cone and sphere charts carry the reversal on `Face::sense`
and their images stay right, which is why the same seam class through
a cylinder wall or a cone generator shells end to end.

**Class, not instance**: any reverted body with a same-plane `Chart`
edge hits this — boolean subtract's `revert(B)` on an operand with a
split planar face, not only `shell`'s cavity; box operands escape
because their edges are `Intersection`-described. Fix shape the lane
proposed (not implemented): in the map phase, for every plane in
`plane_surfaces`, negate the `v` components of every `Chart`
description image on that plane and of every cache row on that
plane's faces (`Harmonic`: `p0.y, pa.y, pb.y, pl.y`; `IsoLine`: `p0.y,
pl.y`) — a sign flip is a bitwise involution, so `revert ∘ revert =
id` holds. Pin: the reverted drum cavity's tier 3 reports exactly
`NegativeVolume`, and `shell` of the collinear-cap drum reaches
`π(r−t)²(h−2t)`. `work/shell/void-insertion-refuses-a-cavity-with-a-same-surface-latitude-seam`
keeps its drum half open on this item; its sphere half is SHELL-9's
(the missing closing mint in `shell`). Signed (SHELL orchestrator).
