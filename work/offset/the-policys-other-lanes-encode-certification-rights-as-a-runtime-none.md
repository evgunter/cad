---
id: the-policys-other-lanes-encode-certification-rights-as-a-runtime-none
kind: issue
title: The at-rest policy's fitted, shell, quadrature and region lanes still encode 'a dual may not certify' as a runtime None; the edge door no longer does
status: open
opened: 2026-10-25
priority: P2
cost: M
parent: closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them
---



Filed by conic unit 1's implementer lane
(`closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them`,
branch `offset/limb2-for-every-carrier`), which built the edge door's
split on Ev's ruling on PR 5689 and was asked to move "the policy's other
optional lanes to the same shape where they encode 'a dual may not
certify'".

**What moved.** The edge door: `topo::AtRestPolicy::nurbs_lane` is gone;
the policy names the edge door as two functions every scalar implements
(`attach_edge`, `reattach_edge` in `crates/topo/src/props.rs`), a
certifying scalar's door certifying in full and a dual's minting, and
what an edge carries is `geom_core::Real::Carries` (the certificate, or
`geom_core::Uncertified` at a dual). The spline foot the offset door's
re-anchor reads moved off the certification lane to its own value,
`geom_brep::SplineFoot`, read through `AtRestPolicy::spline_foot`, an
`Option` like `section_lane`: it is a derivation written at the
certified-bracket scalars (`geom::NurbsCurve3::project_from_seed`'s
`CertifiedBounds` bound), not a certification.

**What did not, and why it is a unit of its own.** Six lanes still
answer `None` at a dual for the reason the edge lane did, certification
rights (`crates/topo/src/props.rs`, the `Dual` arm of `AtRestPolicy`):

- `fitted_lane` — the pcurve mint and the tier-3 pcurve pass refuse
  `PcurveCertifyError::FittedLaneUnsupported` at the check that asks
  (`crates/topo/src/pcurves.rs`, 7 reads; DL3 names this refusal).
- `shell_door` — the shell lowering refuses typed
  (`crates/editor-core/src/eval/wire.rs`; DL3 names it).
- `quad_lane` — not a refusal but a second reading: at `None` the
  construction-time decisions read the closed form only
  (`crates/topo/src/loop_winding.rs` `Unwound::Uncertified`,
  `crates/topo/src/boolean/solid_contain.rs` the `VolumeUncertified`
  arm that escalates to `Measure::Quadrature` only where the lane is
  held, `crates/topo/src/shell.rs` and `crates/topo/src/pieces.rs`'s
  shell roles), so a dual can refuse where `f64` escalated; see
  `a-dual-reads-the-closed-form-where-f64-escalates-to-the-quadrature`.
- `region_lane` — the census's chart-region arms refuse
  `CensusLaneUnsupported` and the crossing rung's backing consult folds
  to `false` (`crates/topo/src/census.rs`).
- `spline_face_crossings` — added by the merge of `main`'s spline ×
  face crossings door into this branch, once `nurbs_lane()` was gone
  (`crates/topo/src/props.rs:6876`, its `Dual` arm `:7614`). It is
  `Some` at the four certifying scalars and `None` at `Dual`. At
  `None` the boolean's planar lane refuses `EdgeCarrierUnsupported`
  with site `PlanarCrossing` (`crates/topo/src/boolean/reduce.rs:4042`).
- `spline_chart_meetings` — added the same way by the next merge of
  `main`, whose spline-face lane (an analytic span against a spline
  chart) read `nurbs_lane()` (`crates/topo/src/props.rs:6889`, its
  `Dual` arm `:7624`). At `None` that lane refuses
  `CurvedBooleanUnsupported` (`crates/topo/src/boolean/reduce.rs:3131`).

Each consumer has its own `None` arm with its own meaning (a typed
refusal for all but the quadrature, a fallback computation for the
quadrature), so moving them is a design pass per lane, not a mechanical
edit: the edge door could only mint at a dual because C2 and DL3 say
what a dual's edge is, and none of these six has that sentence yet.
The edge door's shape is the model: the certifying function bounded on
the certification trait with its lane a plain argument, and the scalar's
policy naming what the dual does instead.

**Two more sites of the same shape, both older than PR 5728** (the PR's
dual review, 2026-10-27):

- `geom_brep::dihedral_side::dihedral_side` and its `normal_at`
  (`crates/geom-brep/src/dihedral_side.rs:220`, `:243`) still take
  `lane: Option<NurbsLane<T>>`, and a spline face at a scalar holding
  none refuses `SideEscalation::NoLane` (`:157`, read at `:257`): a
  certification right encoded as a runtime `None`, as the edge lane
  was.
- `topo::ReplaceFaceError::NurbsLaneUnsupported`
  (`crates/topo/src/replace_face.rs:547`, minted at `:3267` from
  `section_lane`, read at `crates/topo/src/shell.rs:3795`) is the
  replace-face door's refusal for the same `None`, now covering the
  spline foot and the section lane.

PR 5728 removed `Option<NurbsLane>` from the edge door only (the
policy's `nurbs_lane`); these two keep it.
