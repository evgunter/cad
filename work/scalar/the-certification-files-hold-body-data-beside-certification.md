---
id: the-certification-files-hold-body-data-beside-certification
kind: issue
title: pcurve_cache.rs, certify.rs, projected.rs and ssi/certify.rs hold a body's data beside its certification, so the Certifies gate cannot list them
status: open
opened: 2026-11-20
priority: P2
cost: H
parent: from-computed-claims-a-zero-tangent-at-a-dual
---


Found building `certification-interiors-are-closed-to-a-dual-by-a-marker-bound`, 2026-11-20. Measured by type-check, not by instantiation: the bound was written and `cargo check -p geom-brep` read.

## What

`scripts/gates/certification-interiors.sh` holds a listed file's every generic item to `Certifies`. The unit opened its list with five files. Four of them cannot be listed as they stand, because each declares types a dual body holds or names:

- `crates/geom-brep/src/pcurve_cache.rs`: `Pcurve`, `PcurveCache`, `ChartImage`, `Channel`, `ChartWindow`, `PcurveCertificate`, `FocalImage`, `SpiricImage`, `HeldLine` and their evaluation methods, beside the four checks and the incidence readers.
- `crates/geom-brep/src/certify.rs`: `EdgeCurve`, `EdgeCurveSpec`, `Certificate` and the minting doors, beside `run_checks` and the lane types.
- `crates/geom-brep/src/pcurve_cache/projected.rs`: `ProjectedImage` (a `Pcurve::Projected` payload) and its frame methods, beside `run_projected_checks` and the envelope.
- `crates/geom-brep/src/ssi/certify.rs`: `SsiTube`, `SsiCertificate`, beside the tube limbs. It also declares items generic over a parameter that is no scalar (`Hull<P>`, `subdivide<P, E>`), which the gate reads as unbounded (its KNOWN GAP 2).

With `Certifies` written on every generic item of the five files (types included), the first wave of refusals in `geom-brep` alone is 421 errors over `edge_nurbs.rs`, `pcurve_cache/closed_bound.rs`, `pcurve_cache/stated.rs`, `description.rs`, `row_contact.rs`, `dihedral_side.rs`, `dihedral.rs`, `pcurve_cache/patch.rs`, `pair_section.rs`, `fitted_lane.rs`, `props/curved.rs`, `props/quad/projected_edge.rs`, `ssi.rs`, `conic_bend.rs`: construction code reading `Pcurve::eval`, `PcurveCache::pcurve`, `EdgeCurve::carrier`, `EdgeCurve::params`, `Channel::winding`, `ChartImage::into_pcurve`, `ProjectedImage::chart`.

With `Certifies` written on the functions only of `projected.rs` and `ssi/certify.rs` (types left alone), the callers that do not already certify are, in `geom-brep`: `pcurve_cache::chart_image`, `chart_iso_family`, `PcurveCache::certify`, `PcurveCache::certify_projected`, `PcurveCache::recertify`, the `over` constructor beside them, `edge_nurbs::tube_image` and `edge_nurbs::edge_piece`, and `ProjectedImage`'s own methods (`piece_span`, `frame_box_on`, `tube_box`, `window`, `over`). The callers that already carry a certifying bracket door and would only gain the name: `residual_nurbs_limbs`, `plane_nurbs_incidence`, `nurbs_pair_limbs`, `analytic_rung3`, `twin_span_hulls`, `limb2_offset_sup`, `fitted_lane`, `ssi::certify_rung3`. `topo` was not read in this pass.

## What it asks

Each file apart into the data a body holds (bounded by what the data means, a dual included) and the certification that reads it (bounded `Certifies`, listed in the gate). The four are other programs' ground (`work.py territory`: `pcurve_cache.rs` PCERT, `certify.rs` SELECT, `ssi/certify.rs` SSIARITH and SSIEDGE), so the split is theirs to schedule or to hand over.

Not checked: which of the first-wave callers a dual instantiates today, as opposed to admits by bound.
