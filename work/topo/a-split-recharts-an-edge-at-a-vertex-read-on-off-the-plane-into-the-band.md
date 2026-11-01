---
id: a-split-recharts-an-edge-at-a-vertex-read-on-off-the-plane-into-the-band
kind: issue
title: A split's finish re-charts an edge at a vertex read On but off the plane, and its residual lands in the band
status: closed
opened: 2026-10-10
priority: P2
cost: M
closed: 2026-10-11
pr: 4631
---

Found by the re-review fuzz of PR 4565 (CLEAVE, the split's tree of
section corners).

## What

`topo::split` refuses
`Finish(Euler(RechartFalsifies { door: SetFaceSurfacesDescribing, .. }
| Certification { .. }))` with a `pcurve_map_residual` of about `1.1 ε`
in the band (`CertifyError::Escalated { check: ChartResidual }`). This
happens where a base vertex read On lies `0.3–0.9 ε` off the plane.
The input is valid and the refusal names a kernel defect ("there is no
way through").

Witness (now `crates/topo/tests/split_through_an_apex.rs`
`a_notch_floor_in_the_band_splits_sound`):
`apex_pyramid` over the base `(y, z)` = (-0.3709, 0.1491)
(-0.0536, -0.2641) (0.3537, -0.2767) (-0.0022, 0.0927) (-0.0422, 0.9 ε),
at `meeting::poses()`'s "turned", split by the plane through `MEET`
with normal −z (posed). It refuses at ε = 1e-9, 1e-6 and 1e-12; the
base's other nine cells split sound.

## Measured

The review's fuzz (random untangled pyramid bases and crowns, base
vertices at z ∈ {0, ±0.3 ε, ±0.6 ε, ±0.9 ε}, every pose, both normals):
- about 0.5% of cells per ε: 36 of 7,005 on seed `0x9E3779B97F4A7C15`,
  76 of 14,045 on seed `1234567891011`;
- every one is this class;
- main before PR 4565 already had 124 cells of it.

**It depends on the plane's orientation.** Before PR 4565, some of these
cells built because the split's mirrored rerun (since retired) built
them under the flipped normal. A split and its flip read the same
corners, and the finish re-charts the same edges, but only one
orientation's residual lands in the band. Up/down asymmetric cells: 158
after PR 4565, 452 before.

## Where to look (not traced)

The refusing door is `set_face_surfaces_describing`
(`crates/topo/src/attach.rs`), reached from the split finish's
section-plane move (`splitting/finish.rs`). Unmeasured guess: the
residual is the On vertex's own offset from the plane, about `1.1 ε`
against an offset of up to `0.9 ε`. Measure the residual's source
before acting on it.

## Closed 2026-10-11

**Cause: the section plane's frame, charted along an unprojected
chord.** `crates/topo/src/splitting/section_loops.rs` `chord_u_ref` took
the section loop's first chord as the plane's `u` axis. A chord to an
On corner up to `0.9 ε` off the plane leaned that axis out of the plane,
and the re-charted edge's residual inherited the lean. PR 4609 projects
the chord into the plane. With the old `chord.normalize()` restored
locally, the witness refuses again with the same `ChartResidual`; with
the projection, it builds.

**Measured (PR 4631)**, on the review's fuzz with the same harness and
the same sampled cells (7,005 per ε on seed `0x9E3779B97F4A7C15`,
14,045 on `1234567891011`), at ε = 1e-9, 1e-6 and 1e-12:
- At PR 4565's head, 188 and 388 cells per ε refused with
  `Finish(Euler(RechartFalsifies | Certification))`.
- Now none do. Every one of them builds SOUND: tier 3, corners
  disjoint, volume conserved and, for pyramids, the closed form.
- No sound cell changed.

The witness is pinned sound at every pose either way up. Its pieces
pass tier 3 but escalate the tier-3′ census when the notch floor is
`0.9 ε` off. That residue is RESTREAD's, filed as
`work/restread/a-split-whose-notch-floor-reads-on-at-0-9-eps-ships-bodies-whose-census-escalates.md`.
