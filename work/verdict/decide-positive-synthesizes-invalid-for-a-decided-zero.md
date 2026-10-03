---
id: decide-positive-synthesizes-invalid-for-a-decided-zero
kind: issue
title: geom-core: k_stats::decide_positive reports a decided Zero as MarginDiag::INVALID, the poisoned-margin encoding, at every gate whose Zero is a size the user may intend
status: review
opened: 2026-09-30
pr: 3979
branch: cleave/mints-doors
---


(TOPO, the fix pass of PR 3513: the review's m-4, the helper.)

## What

`geom_core::k_stats::decide_positive` (`crates/geom-core/src/k_stats.rs`,
through `classify_gated`) escalates a definite `Zero` and a definite
`Negative` alike as `MarginDiag::INVALID`, the poisoned-margin encoding.
Where the gated quantity is a size the user may intend, a decided `Zero`
is band-decided (D4 ¶1 (i)): a smaller tolerance decides a zero-band
margin positive, so its refusal should offer that tolerance, valued. An
`INVALID` margin instead renders the unreadable-margin note ("… may
indicate a kernel bug worth reporting") on a clean short length.

PR 3513's fix pass added `k_stats::decide_positive_reported` (a decided
`Zero` carries its margin; a `Negative` stays `INVALID`; both recorded as
`decide_positive` records them) and moved only the three lever-arm gates
whose Boolean decision is sized (`geom_brep::enters`'s
`enters_material_arm` and `tangent_sector_order2_arm`, and `dihedral`'s
`dihedral_arm`). The other shipped callers still synthesize `INVALID`
for a decided zero:

- `geom::surfaces::require_ring_torus` (`crates/geom/src/surfaces.rs`):
  `torus_tube_positive` and `ring_torus_convention`, whose Boolean
  decision is sized (`geom_brep::TorusConvention::sized`);
- `topo::boolean::solid_contain` (`bool_torus_frame_radius`);
- `geom_brep::pcurve_cache` (`pcurve_interval_meter`);
- `geom_brep::certify` (`nurbs_span_meter`);
- `geom_brep::ssi::march` (`ssi_transversality_arm`).

## Repair shape

Per caller, ask whether its `Zero` is a size the user may intend (then
`decide_positive_reported`, and a row that its zero-band refusal offers
the tolerance its margin gives) or a question never validly posed (then
`INVALID` is the honest encoding and a comment says so). If every
caller lands on the first answer, `decide_positive` itself carries the
margin and the second door goes.
