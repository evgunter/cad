---
id: a-convex-graze-of-a-cone-refuses-at-some-azimuths
kind: issue
title: a plane holding a cone's apex refuses at the apex vertex, grazing or cutting
status: open
opened: 2026-10-02
priority: P0
cost: M
design: true
---


## What

A full cone (base r = 1, apex at y = 1, revolved about y), cut by a
plane that holds its apex, refuses `Reduce(SliverSector)` on
`sector_straight` at the apex vertex, with margin 0, under either
normal. This happens both when the plane grazes the cone along a ruling
and when it cuts through it:

- grazing along a ruling at every azimuth of `THETAS`
  (`split_tangent_edge_curved.rs`), at ε 1e-6, 1e-9 and 1e-12
  (`cleave/inband-graze`, 2026-10-06);
- turned off tangency by t ∈ {1e-3, 0.05, 0.4, 1, π/2, 2, 3, −0.4, −1.2}
  at θ ∈ {0, 0.3, 2}, at the same three ε (`cleave/seam-ruling-split`'s
  sweep, main at 78bee3ac68).

Each such plane holds the apex, so each side is a cone over a base
segment, and its volume has a closed form. None of these poses is
pinned.

## Where to look

`sector_straight` at a cone's apex vertex (`topo/src/sector_shape.rs`).
Not measured: whether the plane×cone apex lane's tangent line plays any
part. That lane reads a residue generator on its `Zero` arm
(`work/germ/a-plane-cone-apex-tangent-reads-a-residue-generator-off-the-tangency.md`,
the same mechanism the crossing lane had). The secant poses, which never
reach that arm, refuse the same way, so this door comes first.

## Resolved elsewhere

The row first held the frusta poses as well (narrowing θ = 0.3; widening
θ ∈ {1.1, 2.9}; `SliverSector` on `split_conic_departure`, and
`UnpairedLooseEnds` at 1.1) and the filleted slab at φ = 1.2. Their
concave twins were the filleted hole at φ = 1.2 and DR-4098's L-bracket
cove at φ = 4.2 (margin 5.3e-9). All of them stopped because the crossing
lane put its graze root on the residue's crossing rather than the
extremum. PR 4179
(`an-in-band-concave-graze-refuses-at-certification-on-one-side-of-tangency`)
fixed that. The frusta and the slab now answer at their closed forms, at
all three ε, and `CONE_GRAZES_REFUSED` and the slab's φ = 1.2 allowance
are gone. The filleted hole and a cove
(`a_concave_graze_of_a_cove_refuses`) refuse the knife edge.

Measured again on branch `cleave/frustum-apex`, which pairs a two-ruling section's
crossings along each ruling: the full cone through a ruling at a = 0.3, t = 0.05 still refuses
`Reduce(SliverSector { predicate: "sector_straight" })` at the apex vertex, margin exactly 0.0, in
the reduction. That is before any pairing, so it is not the frustum's top↔top pairing defect
(`a-frustum-split-through-a-ruling-off-its-seam-refuses-a-degenerate-section`). Once the
reduction passes, a null-edge half at the apex lies on both rulings, so `ruling_pairs`
(`splitting/join.rs`) will refuse it undecided on `split_join_ruling_side` wherever the wall
face holds more than two halves. That is unmeasured, because nothing reaches it yet.
At the apex itself the margin is exactly 0, so no in-band recourse (a finer ε, a sized
rereading) can settle which ruling an apex half lies on; it has to be read off the half's
direction of departure, not its position.

A frustum's convex graze at its seam ruling refuses where the same graze off the seam lands
whole (branch `cleave/frustum-apex` on main's #4179). Frusta of radii 1 → ½ and ½ → 1 about y,
cut through the ruling at azimuth a turned t ∈ {1e-3, 1e-4, 1e-5} off tangency, both normals, at
ε 1e-6. The sliver's depth is 0.559·t², within ε at every tilt. At a ∈ {0.3, 1, 2, 3, 4, 5.5}
every pose lands whole on its material side. At the seam ruling, a = 0, all 12 refuse in the
reduction at the seam vertex:

- `ConsecutiveOnSectors` (frustum t ∈ {1e-3, 1e-4}, flared t = 1e-4);
- `SliverSector` (flared t = 1e-3);
- `CrossingEscalated` / `CrossingInsertion` (t = 1e-5).

At ε 1e-9 the four a = 0, t = 1e-5 poses refuse `ConsecutiveOnSectors` while the off-seam ones
land whole. Probe: `zz_probe_near` in the lane's scratch (not committed); the poses are those of
`a_near_tangent_cut_through_a_frustum_ruling_never_answers_wrongly`.

## Found by

CLEAVE DR-51's review of PR 3892, `review-tests/dr51` (74e151b6).

Measured on `germ/pn-apex-tangent-generator`, which makes the apex lane
return the grazed generator itself. A cylinder with a conical pit
(α = π/6, apex vertex), split through the apex along the grazed ruling
(τ ∈ {−0.5, −0.1, 0, 0.1, 0.5}·ε at four spins, ε 1e-9), still refuses
`SliverSector` on `sector_straight` at the apex vertex. The tangent
line plays no part in this refusal.


## The partial revolve's two end rulings differ (review r5152, PR 5152)

A frustum revolved `Partial(4 rad)` and grazed along its START ruling
refuses `SliverSector` at `|τ| ≥ 0.5ε`. Grazed along its END ruling
(`a = 4`), the same pose lands whole. Both results are identical on main
and on the apex fix, at three ε. This asymmetry is not yet explained.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P0 (CLEAVE orchestrator's ruling on PR 5765): a plane through a full cone's apex refuses the split at every pose measured, at all three ε, which is a normal verb broken on normal geometry, P0 by `work/README.md` Priority.
- Moved to SPLITDOOR (`work/splitdoor/`) by `git mv`, id and body unchanged: the P1 track opened for split and boolean shapes that refuse, or ship a side their own gate fails, where a uniform reading would build: the split door's tier-3′ bar, tangent and pinch poses, and the curved-carrier lanes that read lines and circles only. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Measured on main (2026-10-28, SPLITDOOR)

Probe branch `splitdoor/cone-apex-probe` (95b1843b1d, measurement only, not for merge):
`crates/sweep/tests/a_plane_through_a_cones_apex.rs` `zz_probe_cone_apex`; a full cone,
profile (0,0),(1,0),(0,1) revolved `Full` about y. 74 poses through the apex (the 20 grazes of
`THETAS` × both normals; 54 secants θ ∈ {0, 0.3, 2} × the row's nine t × both normals): all 74 refuse
identically at ε 1e-6, 1e-9 and 1e-12, `Reduce(SliverSector { vertex, face,
Escalated(margin 0.0, "sector_straight") })`, raised by `sector_shape` from
`neighborhood::wide_entry`.

**The cause is a class, not the rung.** `sector_face::resolve`'s cone arm takes
`implicit_gradient` at the apex, which is 0/0, so the face normal there is NaN; the reflex rung
reads NaN and the straight rung reads the angle between the two bounding rulings, which is
exactly 90° at this half-angle (the 0 margin is a coincidence of α = 45°: below it the rung
decides straight with a NaN bisector, above it refuses as a spike). Every apex reader in the
split takes the face normal at the vertex: rule (a)'s coplanarity, graze, osculation
(`implicit_max_normal_curvature`) and bend reads, rule (b)'s edge wedge, the join's `leave_on`
(`implicit_outward_normal`) and `chord_join::Leave`. And an apex sector's interior directions
are rulings, not positive combinations of its bounds, so a plane through the apex can flip side
twice inside one sector at any width: the planar convex-sector argument does not hold there.

An f64-only hack on the probe branch (`ZZ_HACK=1`: subdivide each apex sector at the plane's
extremal ruling, read rule (a)'s normal along the bounding ruling) lands 21 of 74 at every ε,
each side at its closed form to 3.3e-16; 27 then refuse `Join(Escalated
"split_join_ruling_side")` at margin 0 (the row's predicted `ruling_pairs` refusal); the grazes
refuse `chord_join` `SectionInvariant` (20/16/16) or `tangent_sector_osculation` (4 at θ ∈ {0, π});
t = 1e-3 refuses per ε (`ConsecutiveOnSectors`, `pn_apex_pair_reach` in band, and
`OpenConicChord { Hyperbola }`, likely
`work/germ/a-plane-cone-apex-tangent-reads-a-residue-generator-off-the-tangency.md`).
`sector_shape` is shared with the Boolean, which refuses at an apex by design
(`BooleanError::NormalAtConeApex`).

What is open: one coherent reading of a cone apex for the split, across all those readers, and
where it lives. Weighed by the designer pair before any build.
