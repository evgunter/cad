---
id: a-minted-edge-along-a-folded-graze-refuses-join-desync
kind: issue
title: A pierce run minted along a bound read On at the band's edge fails its edge's certification and refuses JoinDesync (12 near-tangent runs at 1e-9)
status: open
opened: 2026-10-10
priority: P1
cost: M
---


## What

Found by `near-tangent-pierce-poses-reach-three-classification-invariants`
(branch `cleave/near-tangent-invariants`), re-taking JOIN's near-tangent
census (`crates/sweep/examples/near_tangent_census_probe.rs`) after its
fix.

At ε = 1e-9, 12 runs refuse `JoinDesync { "minted-edge description
failed certification" }`, a kernel-defect refusal on valid input: three
ops each of `Ltop nt e2 a13 d-1e-9`, `Lbot nt e2 a9 d1e-9`,
`Lmirror nt e2 a9 d-1e-9` and `asym nt e2 a13 d-1e-9` (the cube's near
face 1e-9 rad off the corner's vertical edge). None at 1e-6 or 1e-12.

Repro: `NT_ONLY=Ltop NT_POSE="nt e2 a13 " NT_D=-1e-9 cargo run -p sweep
--release --example near_tangent_census_probe`.

On main these runs refused in band at `bool_sector_within` in
`boolean/vtxfac.rs` `pierce_germ_dir`, the membership re-reading that
branch removed (the germ is now the transition's own ray). The germs
those runs read are the ones the codes name: each lies along the bound
read On, inside the sector or a graze outside it (the run's eight germs
checked against `sectors::within` by hand). So the in-band refusal was
shielding the describe pass, not reading the germ.

The refusal is raised in `boolean/ops.rs` `describe_edges`: the edge
reads `DihedralClass::Transverse`, and `set_edge_curve` refuses the
`Intersection` draft; `of_describe` maps that to `JoinDesync`. The
likely edge is the section edge minted along the prism's vertical edge:
the side code read that edge On against the cube face (departure 1e-9 m
at its 1 m reach, inside the zero band), but the two planes' line
leaves the edge's far vertex by about the same 1e-9 m, the certification
limit. Unmeasured: which edge, check and sample (the payload is dropped;
`work/issues/joindesync-swallows-the-certification-payload.md`).

## Owed

Name the edge and the residual, then decide which reading is wrong: the
On that lets the edge be minted as lying in the face, or a describe pass
that types a reachable residual miss as a desync. A reachable miss is a
typed in-band refusal, not `JoinDesync` (D9 row 1).

## Built (branch cleave/folded-graze-join-desync)

**Measured on main (0a700cf7ac), the census probe with the dropped
payload printed.** ε = 1e-9: the 12 runs, every one
`Certification { Escalated { SurfaceResidual, sample 8 (the carrier's
end), Indeterminate { Value(±1.0000000518e-9 or ±1.0000000241e-9),
band (1e-9, 1e-8) } } }`. ε = 1e-6 and 1e-12: none.

**The edge** is not the one this row guessed. It is the section edge
the cube's near face cuts across the prism's bottom (top, for `Lbot`)
face, e.g. `Ltop`'s (0.4655, 0, 0) → (1, 1, 0): a `line_between`
chord from the y = 0 edge's pierce, on the cube's plane to 4e-17, to
the far vertex of the prism's vertical edge, which reads on the face.

**The residual, exactly** (rationals over the printed `f64`s):

| | vertex `p1` | `carrier(t₁)` |
|---|---|---|
| `Ltop`, `asym` | 9.999999994736444e-10 | 1.0000000518091955e-9 |
| `Lbot`, `Lmirror` | 9.999999717180687e-10 | 1.0000000240536198e-9 |

`carrier(t₁) = origin + |p1 − p0|·(p1 − p0)/|p1 − p0|` lands one ulp
off the vertex, at (1, 0.9999999999999999, 0), and that ulp moves the
residual across the zero band's edge: 5e-19 or 3e-17 inside it at the
vertex, 5e-17 or 2e-17 past it at the sample. Both readings are of one
point ε off the face, split by less than the residual's own rounding
(~1e-16 on unit coordinates). The geometry is at the band's edge.

**Ruling: in band, mistyped as a defect.** Building them would trust
the vertex's on-face read over the certification, a coincidence the
D10 hold forbids; the certification is not loosened. D9 row 1: an
`Indeterminate` with a `Value` margin is the input's.

**What landed.**
- `describe_edges`' four attachment arms (Intersection, Tangency,
  Arc, Line) no longer discard the gate's refusal (`of_attachment`).
  A `SurfaceResidual` or `WitnessSurfaceResidual` escalation is
  `DescribeRefusal::OffFace`, which the Boolean refuses as
  `Escalated { BooleanDecision::EdgeOnFaces { edge, surface } }`
  ending in its lever, and the merge door as
  `MergeCoplanarError::KeptBoundaryOffFace`.
- Every other failure is `BooleanError::EdgeUndescribed { edge,
  failure, cause }`, in the defect voice, its text carrying the
  check, the sample, the surface and the miss.
- The pin `near_tangent_pierce_poses` holds the four poses at every ε
  row and now counts a `JoinDesync` as a defect (it let one pass);
  `a_section_edge_read_in_band_of_its_face_refuses_typed` holds the 12
  runs refusing `EdgeOnFaces` at 1e-9 and none at 1e-6 or 1e-12.

Filed: `an-in-band-certification-of-a-minted-edge-other-than-its-face-residual-reads-as-a-defect`,
`boolean-structural-refusals-drop-their-payload`.
