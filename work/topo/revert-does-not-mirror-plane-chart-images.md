---
id: revert-does-not-mirror-plane-chart-images
kind: unit
title: Body::revert negates a plane's normal but leaves its Chart images and cache rows unmirrored, so a same-plane Chart edge fails certification on the reverted body
status: dispatched
opened: 2026-09-08
branch: topo/revert-mirrors-chart-images
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

## Brief (TOPO, 2026-09-14) — block TOPO-B2 slot 2, dual at review

**The answer to give.** `Body::revert` keeps its contract at
`revert.rs`'s header — "every certification survives the map" — on a
plane carrying `Chart` edges with a non-zero `v` channel: after the
reversal, every `Chart` image on a reverted plane and every pcurve
cache row of that plane's faces describe the same locus under the
reversed frame, so `validate_geometric(&body.revert())` certifies what
`validate_geometric(&body)` did, and `revert ∘ revert` is the identity
on those descriptions bit for bit.

**Mechanism (hypothesis — verify in phase 1).** The plane's frame is
`(origin, normal, u_ref)` with `v_ref = normal × u_ref` derived at
`eval`, so negating `normal` maps the chart by `(u, v) ↦ (u, −v)`;
the row's measurement says the re-derived image is the stored one
with `v` negated at all nine samples. So the transform is: for every
`Chart` edge description whose surface is a reverted plane, negate the
image's `v` components (for `Harmonic { p0, pa, pb }` that is the `v`
of each point; say what it is for every other image kind the crate
stores — enumerate them from the type, not from prose); for every
pcurve cache row of a face on a reverted plane, the same map on the
stored pcurve (an `IsoLine`/`IsoArc`/`Harmonic` row's chart
coordinates), re-certified against the reverted plane in the same
pass, or dropped and re-minted by `mint_pcurves_of` if the map is
not exact for some kind — phase 1 decides per kind and says why. The
M5 S12 curved arm (`Face::sense` flip on non-plane surfaces) is
untouched: a non-plane chart is not mirrored by the reversal.

**Red-first rows.** SHELL's diagnosis rows on branch `shell/9-probe`
(`crates/sweep/tests/shell9_probe.rs` at `0cbb6593c`, rows 1–2) —
the drum whose top cap carries a collinear profile vertex: the door's
cavity is tier-3 valid, `validate_geometric` of its `revert()` fails
`EdgeCertification { ChartResidual, sample 1 }` on the ring's two
half-circles with residual `0.3827` at `t = π/8`, while the six
radial line edges pass. Rebuild it as a `topo`-side row where the
fixture can be built without `shell` (a plane face with a `Chart`
circle edge whose image has a `v` channel), plus the drum row in
`sweep/tests` as the e2e (announced S-TCOST seam). Assert the
merge-base refusal, the head's `Ok`, the bitwise involution, and
that `insert_voids` no longer refuses `Recertify` on that drum (the
consequence the row names for `shell`).

**Class receipt.** Every reader of a plane's frame that would see the
mirror: `Surface::Plane::eval`, the chart-image certifiers in
`geom-brep`'s `certify.rs`, the pcurve rows, `Chart` images, any
cached `v`-dependent datum (`chart.rs`'s cache rows, `chart_iso.rs`);
say for each whether `revert` transforms it, leaves it invariant, or
leaves it stale — and file what is stale and outside this unit on the
owner's slate.

**Seams.** `crates/topo/src/pcurves.rs` (TRIM's) — the row transform,
if it lands there, is one function by announced seam; `chart.rs` and
`chart_iso.rs` are UNOWNED in the program's `keep_out` sense — a row
landing there draws the fence in the PR; `geom-brep`'s `certify.rs`
is read, not edited, unless the certifier itself is the bug — say so
if it is and stop for the orchestrator.

Branch `topo/revert-mirrors-chart-images`. PR title: "TOPO: revert
mirrors the plane chart's images with its frame". Do not close the
item; the dual runs at review.
