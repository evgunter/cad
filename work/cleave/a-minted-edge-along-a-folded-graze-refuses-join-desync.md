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
